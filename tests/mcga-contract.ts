import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { McgaContract } from "../target/types/mcga_contract";

describe("mcga-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.McgaContract as Program<McgaContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
