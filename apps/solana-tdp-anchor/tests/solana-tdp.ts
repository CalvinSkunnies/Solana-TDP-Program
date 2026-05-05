import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTdp } from "../target/types/solana_tdp";
import { expect } from "chai";

describe("solana-tdp", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaTdp as Program<SolanaTdp>;

  it("create_stream: handler compiles and returns Ok(())", async () => {
    // TODO: create mint, token accounts, call program.methods.createStream(...)
    expect(program.programId).to.not.be.undefined;
  });

  it("withdraw: handler compiles and returns Ok(())", async () => {
    // TODO: advance clock, call program.methods.withdraw()
    expect(program.programId).to.not.be.undefined;
  });

  it("cancel: handler compiles and returns Ok(())", async () => {
    // TODO: call program.methods.cancel(), verify final balances
    expect(program.programId).to.not.be.undefined;
  });
});
