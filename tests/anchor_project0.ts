import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorProject0 } from "../target/types/anchor_project0";

describe("anchor_project0", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorProject0 as Program<AnchorProject0>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
