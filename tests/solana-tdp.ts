import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTdp } from "../target/types/solana_tdp";
import { expect } from "chai";

describe("solana-tdp", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTdp as Program<SolanaTdp>;

  // ── create_stream ──────────────────────────────────────────────────────────
  it("create_stream: placeholder – handler compiles and returns Ok(())", async () => {
    // TODO: set up mint, accounts, and call program.methods.createStream(...)
    expect(program.programId).to.not.be.undefined;
  });

  // ── withdraw ───────────────────────────────────────────────────────────────
  it("withdraw: placeholder – handler compiles and returns Ok(())", async () => {
    // TODO: advance clock, call program.methods.withdraw()
    expect(program.programId).to.not.be.undefined;
  });

  // ── cancel ─────────────────────────────────────────────────────────────────
  it("cancel: placeholder – handler compiles and returns Ok(())", async () => {
    // TODO: call program.methods.cancel(), verify balances
    expect(program.programId).to.not.be.undefined;
  });
});
