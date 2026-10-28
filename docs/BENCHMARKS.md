# Benchmarks

Resource cost of each `pact` entrypoint, measured with the Tollcraft Tier A macros
(`contracts/pact/src/test/budget.rs`, ROADMAP C-12). Each test runs the **compiled WASM**,
resets the budget right before the call, and asserts the call alone.

- WASM: `748b5dbcc800d8cd3bdaf4b24555bfa8e3659cdefb60888791fb9a086b35d1a4` (= testnet deployment and release `v0.1.0-rc.1`)
- soroban-sdk 28.0.0, stellar-cli 28.1.0, measured 2026-10-28
- Limits = measured × 1.15, rounded up. A change that pushes a call over its limit fails CI.

| Entrypoint | Path measured | CPU instructions | Memory bytes | CPU limit | Memory limit | CPU vs. 100M ceiling |
|---|---|---|---|---|---|---|
| `open` | escrow transfer in | 889,745 | 1,436,284 | 1,030,000 | 1,660,000 | 0.89% |
| `record_verdict` | ed25519 check + state write | 1,122,084 | 1,362,925 | 1,300,000 | 1,570,000 | 1.12% |
| `settle` | partial: two transfers (worst case) | 1,073,058 | 1,475,617 | 1,240,000 | 1,700,000 | 1.07% |
| `settle` | full: one transfer | 889,671 | 1,438,667 | — | — | 0.89% |
| `release` | full refund, one transfer | 872,907 | 1,423,341 | 1,010,000 | 1,640,000 | 0.87% |
| `close_meter` | ed25519 check + two transfers | 1,560,061 | 1,484,463 | 1,800,000 | 1,710,000 | 1.56% |

Per-transaction network ceilings (budget.toml): CPU 100,000,000 · memory 40,000,000 bytes.
Every entrypoint uses under 2% of the CPU ceiling and under 4% of the memory ceiling.

**Not yet measured here:** network-side read/write bytes, ledger-entry counts and fees from a real
testnet run (Tier B, `cargo budget-report`). That needs a scripted stateful flow (funded test
token, signed attestations), tracked as ROADMAP C-21.
