import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { CounterProgram } from "../target/types/counter_program";

describe("counter_program", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;

  const user = anchor.web3.Keypair.generate();

  const [pda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Initialize the Counter Account", async () => {
    const airdropSignature = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // Airdrop 2 SOL to cover the transaction cost
    );

    // Wait for the airdrop transaction to be confirmed
    await provider.connection.confirmTransaction(airdropSignature);

    const txSig = await program.methods
      .initialize()
      .accounts({
        counter: pda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const accountData = await program.account.counter.fetch(pda);
    console.log(`Transaction Signature: ${txSig}`);
    console.log(`Initialized Counter Account. Count is: ${accountData.count}`);
  });

  it("Increment", async () => {
    const transactionSignature = await program.methods
      .increment()
      .accounts({
        counter: pda,
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    const accountData = await program.account.counter.fetch(pda);

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
});
