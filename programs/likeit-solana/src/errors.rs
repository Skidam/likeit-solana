use anchor_lang::prelude::*;

#[error_code]
pub enum AppError {
    #[msg("Arithmetic overflow occurred")]
    Overflow,
    #[msg("Invalid creator for project")]
    InvalidCreator,
    #[msg("Invalid name for project")]
    InvalidName,
}