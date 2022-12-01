import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { P2pTradeFacilitatorProgram } from "../target/types/p2p_trade_facilitator_program";

describe("p2p-trade-facilitator-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.P2pTradeFacilitatorProgram as Program<P2pTradeFacilitatorProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
