# Solana Token Distribution Protocol (TDP)

A non-custodial, on-chain SPL-token vesting and distribution program built with [Anchor](https://www.anchor-lang.com/) on Solana.

---

## Monorepo Structure

```
Solana-TDP-Program/
├── apps/
│   ├── solana-tdp-anchor/        # Anchor program (Rust + tests)
│   └── web/                      # Future: React frontend
├── packages/
│   └── solana-tdp-sdk/           # Future: TypeScript SDK
├── Cargo.toml                    # Root Rust workspace
├── package.json                  # pnpm 10.33.0 workspace root
└── pnpm-workspace.yaml
```

---

## Quick Start (from Root)

Standard commands are proxied to the Anchor app directory:

```bash
# Install dependencies
pnpm install

# Build the program
pnpm build

# Run tests
pnpm test
```

If you prefer to work directly with the Anchor CLI:
```bash
cd apps/solana-tdp-anchor
anchor build
anchor test
```

## Program Instructions

| Instruction | Description |
|---|---|
| `create_stream` | Lock tokens in a PDA vault, record vesting schedule |
| `withdraw` | Recipient claims all vested tokens at any time |
| `cancel` | Sender cancels; vested → recipient, unvested → sender |

## Account Structure

```
StreamAccount (PDA)  [seeds: b"stream" + sender + recipient]
├── sender             Pubkey
├── recipient          Pubkey
├── mint               Pubkey
├── vault              Pubkey ──▶ VaultTokenAccount (PDA)
│                              [seeds: b"vault" + stream]
├── amount             u64
├── amount_withdrawn   u64
├── start_time         i64
├── end_time           i64
├── cliff_time         i64
├── cancelled          bool
└── bump               u8
```

### Prerequisites

| Tool | Version |
|---|---|
| Rust | stable ≥ 1.75 |
| Solana CLI | ≥ 1.18 |
| Anchor CLI | 0.30.1 |
| Node.js | ≥ 20 |
| pnpm | **10.33.0** |

## License

MIT
