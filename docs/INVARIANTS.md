# Invariants → tests

The invariants of ARCHITECTURE.md §4.5, each with the tests that enforce it.

| # | Invariant | Named test (`test/invariants.rs`) | Also covered by |
|---|---|---|---|
| 1 | A pact reaches exactly one terminal state (`Settled` or `Released`). | `invariant_1_exactly_one_terminal_state` | `lifecycle::release_rejects_terminal_pact`, `close_meter_rejects_terminal_pact` |
| 2 | `paid_to_seller + returned_to_buyer == cap` at the terminal state. | `invariant_2_amounts_sum_to_cap` | `fuzz::fuzz_amounts_always_sum_to_cap` |
| 3 | The contract's token balance equals the sum of `cap` over non-terminal pacts. | `invariant_3_balance_equals_open_caps` | `fuzz::fuzz_amounts_always_sum_to_cap` (escrow empties) |
| 4 | No entrypoint lets the admin move funds or change an open pact. | `invariant_4_admin_cannot_move_funds` | — |
| 5 | An attestation is bound to one `pact_id` and `condition_hash`; a second `record_verdict` fails. | `invariant_5_attestation_bound_and_single` | `fuzz::fuzz_verdict_requires_valid_signature` |
| 6 | `set_verifier` never affects an already-open pact. | `invariant_6_set_verifier_spares_open_pacts` | — |
| 7 | `settle`/`close_meter` never succeed after `settle_deadline`; `release` never succeeds for an `Allowed` pact before it. | `invariant_7_deadlines_enforced` | `deadlines.rs`, `fuzz::fuzz_deadlines_never_trap` |
| 8 | Metered pacts never accept a verdict; verdict pacts never accept `close_meter`. | `invariant_8_metered_and_verdict_paths_disjoint`, `invariant_8_metered_pact_rejects_verdict_and_settle` | `lifecycle::close_meter_rejects_verdict_pact` |

Run them with `cargo test invariant`. Fuzz targets: `cargo test fuzz` (see `fuzz/README.md`).
