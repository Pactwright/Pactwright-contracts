# Fuzzing (ROADMAP C-16)

Property tests use [bolero](https://github.com/camshaft/bolero) and live with the contract's
other tests in `contracts/pact/src/test/fuzz.rs`, where they can reach the test fixture and the
crate-internal hashing used to sign attestations.

| Target | Property |
|---|---|
| `fuzz_amounts_always_sum_to_cap` | Random caps, verdict amounts, settle amounts and paths always end with `paid + returned == cap`, an empty escrow and no lost tokens (invariant 2). |
| `fuzz_verdict_requires_valid_signature` | Random signatures, and valid signatures over a changed attestation, never record a verdict. |
| `fuzz_deadlines_never_trap` | Random deadlines and ledger jumps never trap: every call returns a value or a contract error. |

## Running

```bash
cargo test fuzz                     # bounded: 200 cases per target, part of CI (`ci.yml`)
cargo install cargo-bolero          # longer, coverage-guided runs
cargo bolero test --package pactwright-pact test::fuzz::fuzz_amounts_always_sum_to_cap
```
