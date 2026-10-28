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

## Tier B: real testnet transactions (C-21)

`scripts/bench-testnet.sh` runs each entrypoint as a real transaction on a dedicated bench
deployment (bench `pact` `CBWEAVYZD6SP3PA5V6ZGEBHBUYJENBAAG5CFBTB6FCYDYZGPYRZZQEQM`, test token
`PWBENCH` issued by `pactwright-sponsor`) and reads the declared resources and charged fees back
from RPC. Same WASM as above. Raw report: [`benchmarks/2026-10-28-testnet.json`](benchmarks/2026-10-28-testnet.json).
`scripts/bench-testnet.sh --check` (or the manual `bench-testnet` workflow) fails if any value
exceeds the Tier B limits in `budget.toml` (measured × 1.15, rounded up).

| Entrypoint | Instructions | vs. Tier A | Disk read B | Write B | RO/RW entries | Resource fee | Fee charged | Tx |
|---|---|---|---|---|---|---|---|---|
| `open` | 1,071,584 | +20% | 268 | 1,100 | 4/3 | 211,807 | 181,442 | [e782ba7a…](https://stellar.expert/explorer/testnet/tx/e782ba7a347590e1c310a0d31dcd4632547b2140b1dc3204060be03a9390ab61) |
| `record_verdict` | 1,647,891 | +47% | 0 | 788 | 2/1 | 14,156 | 8,791 | [8ed129b4…](https://stellar.expert/explorer/testnet/tx/8ed129b4f2590330dea6e3c08c8f06c59a9687ec1a6a2ad046e0f63f7ed832a4) |
| `settle` (partial) | 1,191,764 | +11% | 248 | 1,260 | 3/4 | 17,060 | 12,059 | [52899433…](https://stellar.expert/explorer/testnet/tx/52899433fce58051a2b5f7690849fc086e033fc6ee4b0b386b211529ff2f9ae4) |
| `release` | 1,032,490 | +18% | 124 | 1,136 | 3/3 | 13,656 | 8,750 | [214da9bc…](https://stellar.expert/explorer/testnet/tx/214da9bc23b3cceb96fbc2c17b3c1ada9f02d91a126a73128b0d45b384b7b8c7) |
| `close_meter` | 2,039,236 | +31% | 248 | 1,340 | 3/4 | 17,704 | 12,694 | [60409d84…](https://stellar.expert/explorer/testnet/tx/60409d845346a18a066cc58ba8393d4a365a8f2e787afbde22268a3bfa26259a) |

Fees are in stroops of XLM. Observations:

- **Local Tier A undercounts network instructions by 11–47%**, most for the ed25519 paths
  (`record_verdict`, `close_meter`). That is why both tiers exist; Tier A limits stay a CI
  regression gate, Tier B limits bound what the network charges.
- **`open` pays rent:** its resource fee is ~10× the others because it creates the persistent
  pact entry with a TTL covering the settle window (ADR-009).
- **PRD §8 cost target:** a settle charges 12,059 stroops = 0.0012 XLM, so it stays under 1% of a
  1 USDC pact at any XLM price below about $8.
