# MVP Launch Metrics (L-05)

> **Roadmap task:** `L-05` (Phase 6, MVP launch on testnet) · **Requirement:** PRD §8
> **Contract:** `pact` `CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6` (Stellar testnet)
> **Measured:** 2026-11-02, up to ledger 5523057

**Status: not met.** Two of the three PRD §8 MVP targets are met; testnet pact volume is short, and
no independent seller has been onboarded yet ([`TESTNET_SELLERS.md`](TESTNET_SELLERS.md)).
Every number below is measured and says where it comes from. Anything not measured says so.

| Metric | PRD §8 MVP target | Measured | Status |
|---|---|---|---|
| Testnet pacts | ≥ 100 | **49** M2 pacts opened on the `pact` contract | Not met |
| Verdict accuracy (labelled set) | ≥ 90% | **29/29 (100%)**, real model, 29 cases | Met (small set) |
| `settle` cost | < 1% of 1 USDC | **12,059 stroops** (0.0012 XLM) | Met below ~$8/XLM |
| Independent sellers (L-04) | ≥ 3 | **0** | Not met |
| Median open → terminal | — | not measured | — |

## How each number was measured

### Testnet pacts
`getEvents` on `https://soroban-testnet.stellar.org`, filtered to the `pact` contract and paged
from the RPC's oldest retained ledger (5402098) to the latest (5523057). The contract's first
event is at ledger 5447617, so its whole history is inside retention.

| Event | Count |
|---|---|
| `opened` | 49 |
| `verdict` | 33 |
| `settled` | 21 |
| `released` | 24 |

45 of the 49 pacts have reached a terminal state. All were opened by the maintainers' own e2e and
benchmark runs; none came from an outside seller or agent. M0 purchases are plain x402 payments
and never touch the `pact` contract, so they aren't in this count.

### Verdict accuracy
Kit `verifier-eval` workflow, `real-model` job, run
[`40897633766`](https://github.com/Pactwright/Pactwright-Kit/actions/runs/40897633766)
(2026-10-29): `document.translation` **29/29** through the full pipeline (delivery receipt,
evidence hash, model, rules, signing) with Google `gemini-3.5-flash-lite`. The scheduled weekly
run also passes. The set has 29 cases, all of which the model gets right, so it doesn't yet
contain cases the model fails; it should grow before this number is relied on.

### `settle` cost
[`BENCHMARKS.md`](BENCHMARKS.md), Tier B testnet benchmark: a partial `settle` (two transfers,
the worst case) charged 12,059 stroops, tx
[`52899433…`](https://stellar.expert/explorer/testnet/tx/52899433fce58051a2b5f7690849fc086e033fc6ee4b0b386b211529ff2f9ae4).
That stays under 1% of a 1 USDC pact at any XLM price below about $8.

## To close L-05
- 51 more testnet pacts, ideally from outside agents through the hosted facilitator.
- At least three independent sellers ([`TESTNET_SELLERS.md`](TESTNET_SELLERS.md)).
- Re-run the event count above and update this file with the new ledger range.
