import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InstructionsReflection } from "../target/types/instructions_reflection";

describe("instructions-reflection", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.InstructionsReflection as Program<InstructionsReflection>;

  const MEMO_PROGRAM_ID = new anchor.web3.PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

  it("call parent", async () => {
    // Add your test here.
    const tx = await program.methods.parentCall().accounts({
      memoProgram: MEMO_PROGRAM_ID,
      childProgram: new anchor.web3.PublicKey("ExQUucxSnQbAuLoTQxKo4m5woiMmh1YiLgZ2UW11zhEN"),
    }).preInstructions([
      { programId: MEMO_PROGRAM_ID, keys: [], data: Buffer.from("memo#t.1") },
      { programId: MEMO_PROGRAM_ID, keys: [], data: Buffer.from("memo#t.2") },
    ]).postInstructions([
      await program.methods.parentCall().accounts({
      memoProgram: MEMO_PROGRAM_ID,
      childProgram: new anchor.web3.PublicKey("ExQUucxSnQbAuLoTQxKo4m5woiMmh1YiLgZ2UW11zhEN"),
    }).instruction()
    ])
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
