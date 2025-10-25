use anchor_lang::prelude::*;

mod errors;
use errors::AppError;

declare_id!("GwdN65ra3767d8ciDTsXvaw5RccdDrzWBxkp2nvuuiJe");

#[program]
pub mod likeit_solana {
    use super::*;

    pub fn initialize_project(ctx: Context<InitializeProject>, name: String, url: String) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.name = name;
        project.url = url;
        project.likes = 0;
        project.dislikes = 0;
        project.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn like_project(ctx: Context<LikeProject>, name: String) -> Result<()> {
        let project = &mut ctx.accounts.project;
        require_eq!(project.name.clone(), name, AppError::InvalidName);
        require_keys_eq!(project.authority, ctx.accounts.creator.key(), AppError::InvalidCreator);
        project.likes = project.likes.checked_add(1).ok_or(AppError::Overflow)?;
        Ok(())
    }

    pub fn dislike_project(ctx: Context<DislikeProject>, name: String) -> Result<()> {
        let project = &mut ctx.accounts.project;
        require_eq!(project.name.clone(), name, AppError::InvalidName);
        require_keys_eq!(project.authority, ctx.accounts.creator.key(), AppError::InvalidCreator);
        project.dislikes = project.dislikes.checked_add(1).ok_or(AppError::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeProject<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 100 + 4 + 100 + 8 + 8 + 32,
        seeds = [b"project", authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub project: Account<'info, ProjectAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct LikeProject<'info> {
    #[account(
        mut,
        seeds = [b"project", creator.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub project: Account<'info, ProjectAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Used for PDA derivation only
    pub creator: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DislikeProject<'info> {
    #[account(
        mut,
        seeds = [b"project", creator.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub project: Account<'info, ProjectAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Used for PDA derivation only
    pub creator: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct ProjectAccount {
    #[max_len(100)]
    pub name: String,
    #[max_len(100)]
    pub url: String,
    pub likes: u64,
    pub dislikes: u64,
    pub authority: Pubkey,
}