import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { LikeitSolana } from "../target/types/likeit_solana";

describe("likeit-solana", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.LikeitSolana as Program<LikeitSolana>;
  const authority = provider.wallet.publicKey;
  const name = "TestProject";
  const url = "https://example.com";
  let projectPda: PublicKey;

  before(async () => {
    [projectPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("project"), authority.toBuffer(), Buffer.from(name)],
      program.programId
    );

    await program.methods
      .initializeProject(name, url)
      .accounts({
        project: projectPda,
        authority: authority,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();
  });

  it("Initializes a project", async () => {
    const project = await program.account.projectAccount.fetch(projectPda);
    assert.equal(project.name, name);
    assert.equal(project.url, url);
    assert.equal(project.likes.toNumber(), 0);
    assert.equal(project.dislikes.toNumber(), 0);
    assert.equal(project.authority.toString(), authority.toString());
  });

  it("Likes a project", async () => {
    await program.methods
      .likeProject(name)
      .accounts({
        project: projectPda,
        signer: authority,
        creator: authority,
      } as any)
      .rpc();

    const project = await program.account.projectAccount.fetch(projectPda);
    assert.equal(project.likes.toNumber(), 1);
    assert.equal(project.dislikes.toNumber(), 0);
  });

  it("Dislikes a project", async () => {
    await program.methods
      .dislikeProject(name)
      .accounts({
        project: projectPda,
        signer: authority,
        creator: authority,
      } as any)
      .rpc();

    const project = await program.account.projectAccount.fetch(projectPda);
    assert.equal(project.likes.toNumber(), 1);
    assert.equal(project.dislikes.toNumber(), 1);
  });
});