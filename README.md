# Solana Token Distribution Protocol (TDP)

A non-custodial, on-chain token vesting and distribution program built with
[Anchor](https://www.anchor-lang.com/) on Solana.

---

## Overview

The TDP program lets any project stream SPL tokens to a recipient over time.
Tokens are locked inside a **Program Derived Address (PDA) vault** and become
claimable according to a linear vesting schedule with an optional cliff.

## Program Instructions

| Instruction | Description |
|---|---|
| `create_stream` | Initialise a vesting stream and deposit tokens into the PDA vault |
| `withdraw` | Recipient claims all vested (unlocked) tokens at any point |
| `cancel` | Sender cancels the stream; vested portion goes to recipient, remainder returned |

## Account Structure

```
GlobalConfig (PDA)
└── StreamAccount (PDA)  [seeds: "stream" + sender + recipient]
    ├── sender       : Pubkey
    ├── recipient    : Pubkey
    ├── mint         : Pubkey
    ├── vault        : Pubkey  ──► VaultTokenAccount (PDA)
    ├── amount       : u64          [seeds: "vault" + stream]
    ├── amount_withdrawn : u64
    ├── start_time   : i64
    ├── end_time     : i64
    ├── cliff_time   : i64
    ├── cancelled    : bool
    └── bump         : u8
```

## Data Flow

```
Sender creates stream
  │
  ▼
Tokens locked in PDA Vault  ──────────────────────────────────┐
  │                                                            │
  │  time passes …                                             │
  ▼                                                            │
Recipient calls withdraw                                       │
  │  (amount = (elapsed / duration) × total − already_claimed)│
  ▼                                                            │
Vested tokens transferred to recipient token account          │
                                                               │
Sender may call cancel at any time                            │
  │  vested → recipient                                        │
  └─ unvested → returned to sender ◄─────────────────────────┘
```

## Prerequisites

| Tool | Version |
|---|---|
| Rust | stable (≥ 1.75) |
| Solana CLI | ≥ 1.18 |
| Anchor CLI | 0.30.1 |
| Node.js | ≥ 20 |
| pnpm | **10.33.0** |

## Quick Start

```bash
# 1. Install JS dependencies
pnpm install

# 2. Build the program
anchor build

# 3. Run placeholder tests (local validator)
anchor test
```

## Project Structure

```
Solana-TDP-Program/
├── Anchor.toml
├── Cargo.toml
├── package.json               # pnpm 10.33.0
├── pnpm-workspace.yaml
├── tsconfig.json
├── programs/
│   └── solana-tdp/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs         # create_stream, withdraw, cancel handlers
├── tests/
│   └── solana-tdp.ts
└── migrations/
    └── deploy.ts
```

## License

MIT
