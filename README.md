# pactwright-contracts

**Soroban contracts for Pactwright — accountable AI-agent purchases on Stellar.**

> Status: scaffold. Nothing is deployed yet. See [`ROADMAP.md`](ROADMAP.md).

## The problem
An AI agent that pays for a service has no recourse: money moves before delivery and
"delivered" is whatever the seller says.

## What this repo contains
The `pact` escrow contract (M2): the buyer's cap is held until a signed verdict says the
delivery met a condition frozen before payment, or until signed usage proves what was
consumed. Deadlines and a silence policy guarantee funds are never stranded.

## Why Stellar
Sub-cent fees make per-request escrow viable; USDC is native through the Stellar Asset
Contract; Soroban's predictable fees let us bound every entrypoint's cost with Tollcraft.

## Layout
- `contracts/pact/` — the contract ([design](docs/ARCHITECTURE.md#4-on-chain-design))
- `test-vectors/` — the source of truth for every hash
- `vendor/wasm/` — pinned Accensa `RefundVault` and `ReceiptAnchor`

## Quickstart
```bash
cargo test            # placeholder tests are ignored until their ROADMAP task lands
stellar contract build
```

## Contributing
Read [`CONTRIBUTING.md`](CONTRIBUTING.md). **Every PR updates [`ROADMAP.md`](ROADMAP.md).**
