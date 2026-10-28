//! Bolero property tests (ROADMAP C-16). They run under `cargo test` with a bounded number of
//! iterations; see fuzz/README.md for longer runs with `cargo bolero test`.

use soroban_sdk::BytesN;

use super::*;
use crate::{storage::GRACE_LEDGERS, Error, PactState, Verdict};

const ITERATIONS: usize = 200;
/// Keeps ledger jumps inside the TTL of every entry the test env created.
const MAX_JUMP: u32 = 20_000;

/// Invariant 2 under random amounts and paths: at the terminal state
/// `paid_to_seller + returned_to_buyer == cap`, the escrow is empty and no token is lost.
#[test]
fn fuzz_amounts_always_sum_to_cap() {
    bolero::check!()
        .with_type::<(u64, u64, u64, i64, bool, bool)>()
        .with_iterations(ITERATIONS)
        .for_each(
            |&(cap_raw, amount_raw, actual_raw, wild_actual, deny, settle)| {
                let f = Fixture::new();
                let cap = 1 + (cap_raw % FUNDS as u64) as i128;
                let mut params = f.params(1);
                params.cap = cap;
                let pact_id = f.pact.open(&params);

                if deny {
                    f.verdict(&pact_id, Verdict::Deny, 0);
                    f.pact.release(&pact_id);
                } else {
                    let amount = 1 + (amount_raw % cap as u64) as i128;
                    f.verdict(&pact_id, Verdict::Allow, amount);
                    // An arbitrary amount either settles legitimately or is rejected.
                    let _ = f.pact.try_settle(&pact_id, &(wild_actual as i128));
                    if f.pact.get_pact(&pact_id).state != PactState::Settled {
                        if settle {
                            let actual = 1 + (actual_raw % amount as u64) as i128;
                            f.pact.settle(&pact_id, &actual);
                        } else {
                            f.advance_to(params.settle_deadline + 1);
                            f.pact.release(&pact_id);
                        }
                    }
                }

                let pact = f.pact.get_pact(&pact_id);
                assert!(pact.state.is_terminal());
                assert_eq!(pact.paid_to_seller + pact.returned_to_buyer, cap);
                assert!(pact.paid_to_seller <= pact.allowed_amount || deny);
                assert_eq!(f.escrow_balance(), 0);
                assert_eq!(f.balance(&f.buyer) + f.balance(&f.seller), FUNDS);
            },
        );
}

/// A verdict is never recorded without the verifier's signature over the exact attestation.
#[test]
fn fuzz_verdict_requires_valid_signature() {
    bolero::check!()
        .with_type::<([u8; 32], [u8; 32], u8, u64)>()
        .with_iterations(ITERATIONS)
        .for_each(|&(sig_lo, sig_hi, field, tweak)| {
            let f = Fixture::new();
            let pact_id = f.open(1);
            let attestation = f.attestation(&pact_id, Verdict::Allow, CAP);

            // Random signature.
            let mut random = [0u8; 64];
            random[..32].copy_from_slice(&sig_lo);
            random[32..].copy_from_slice(&sig_hi);
            let random = BytesN::from_array(&f.env, &random);
            assert!(f
                .pact
                .try_record_verdict(&pact_id, &attestation, &random)
                .is_err());

            // Valid signature, one field changed afterwards.
            let signature = f.sign(&attestation);
            let mut changed = attestation.clone();
            match field % 4 {
                0 => changed.amount = 1 + (tweak % CAP as u64) as i128,
                1 => {
                    let mut h = changed.envelope_hash.to_array();
                    h[(tweak % 32) as usize] ^= 1 + (tweak % 255) as u8;
                    changed.envelope_hash = BytesN::from_array(&f.env, &h);
                }
                2 => changed.verdict = Verdict::Deny,
                _ => changed.issued_ledger = START_LEDGER - 1 - (tweak % 100) as u32,
            }
            if changed != attestation {
                assert!(f
                    .pact
                    .try_record_verdict(&pact_id, &changed, &signature)
                    .is_err());
            }
            assert_eq!(f.pact.get_pact(&pact_id).state, PactState::Funded);
        });
}

/// Deadline arithmetic never traps: every call returns a value or a contract error.
#[test]
fn fuzz_deadlines_never_trap() {
    bolero::check!()
        .with_type::<(u32, u32, u32, bool)>()
        .with_iterations(ITERATIONS)
        .for_each(|&(verdict_raw, settle_raw, jump, raw)| {
            let f = Fixture::new();
            let now = f.now();
            let mut params = f.params(1);
            if raw {
                params.verdict_deadline = verdict_raw;
                params.settle_deadline = settle_raw;
            } else {
                params.verdict_deadline = now.saturating_add(1 + verdict_raw % MAX_JUMP);
                params.settle_deadline = params
                    .verdict_deadline
                    .saturating_add(settle_raw % MAX_JUMP);
            }

            let opened = f.pact.try_open(&params);
            assert!(!matches!(opened, Err(Err(_))), "open trapped: {:?}", opened);
            let Ok(Ok(pact_id)) = opened else {
                assert_eq!(opened, Err(Ok(Error::BadDeadlines)));
                return;
            };

            let pact = f.pact.get_pact(&pact_id);
            let horizon = (pact.settle_deadline - now + GRACE_LEDGERS - 1).min(MAX_JUMP);
            f.advance_to(now + jump % (horizon + 1));

            let settled = f.pact.try_settle(&pact_id, &CAP);
            assert!(!matches!(settled, Err(Err(_))), "settle trapped");
            let released = f.pact.try_release(&pact_id);
            assert!(!matches!(released, Err(Err(_))), "release trapped");
            let extended = f.pact.try_extend_pact_ttl(&pact_id);
            assert!(!matches!(extended, Err(Err(_))), "extend trapped");
        });
}
