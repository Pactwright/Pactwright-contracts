//! One named test per invariant, ARCHITECTURE.md §4.5 (ROADMAP C-11).

use soroban_sdk::{testutils::MockAuth, testutils::MockAuthInvoke, IntoVal};

use super::*;
use crate::{Error, PactState, Verdict};

/// 1. A pact reaches exactly one terminal state.
#[test]
fn invariant_1_exactly_one_terminal_state() {
    let f = Fixture::new();
    let settled = f.open(1);
    f.verdict(&settled, Verdict::Allow, CAP);
    f.pact.settle(&settled, &CAP);
    assert_eq!(
        f.pact.try_settle(&settled, &CAP),
        Err(Ok(Error::InvalidState))
    );
    assert_eq!(f.pact.try_release(&settled), Err(Ok(Error::InvalidState)));

    let released = f.open(2);
    f.verdict(&released, Verdict::Deny, 0);
    f.pact.release(&released);
    assert_eq!(
        f.pact.try_settle(&released, &CAP),
        Err(Ok(Error::InvalidState))
    );
    assert_eq!(f.pact.try_release(&released), Err(Ok(Error::InvalidState)));
    let attestation = f.attestation(&released, Verdict::Allow, CAP);
    let signature = f.sign(&attestation);
    assert_eq!(
        f.pact
            .try_record_verdict(&released, &attestation, &signature),
        Err(Ok(Error::InvalidState))
    );
}

/// 2. `paid_to_seller + returned_to_buyer == cap` at terminal state.
#[test]
fn invariant_2_amounts_sum_to_cap() {
    let f = Fixture::new();
    for (nonce, actual) in [(1u64, 1i128), (2, CAP / 3), (3, CAP)] {
        let pact_id = f.open(nonce);
        f.verdict(&pact_id, Verdict::Allow, CAP);
        f.pact.settle(&pact_id, &actual);
        let pact = f.pact.get_pact(&pact_id);
        assert_eq!(pact.paid_to_seller + pact.returned_to_buyer, pact.cap);
    }
    let denied = f.open(4);
    f.verdict(&denied, Verdict::Deny, 0);
    f.pact.release(&denied);
    let pact = f.pact.get_pact(&denied);
    assert_eq!(pact.paid_to_seller + pact.returned_to_buyer, pact.cap);
}

/// 3. The contract's balance equals the sum of `cap` over non-terminal pacts.
#[test]
fn invariant_3_balance_equals_open_caps() {
    let f = Fixture::new();
    let a = f.open(1);
    let b = f.open(2);
    let _c = f.open(3);
    assert_eq!(f.escrow_balance(), 3 * CAP);

    f.verdict(&a, Verdict::Allow, CAP);
    f.pact.settle(&a, &(CAP / 2));
    assert_eq!(f.escrow_balance(), 2 * CAP);

    f.verdict(&b, Verdict::Deny, 0);
    f.pact.release(&b);
    assert_eq!(f.escrow_balance(), CAP);
}

/// 4. No entrypoint lets the admin move funds or change an open pact.
#[test]
fn invariant_4_admin_cannot_move_funds() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);

    // Only the admin's signature is available: settle must fail.
    f.env.mock_auths(&[MockAuth {
        address: &f.admin,
        invoke: &MockAuthInvoke {
            contract: &f.pact.address,
            fn_name: "settle",
            args: (pact_id.clone(), CAP).into_val(&f.env),
            sub_invokes: &[],
        },
    }]);
    assert!(f.pact.try_settle(&pact_id, &CAP).is_err());

    // Rotating the verifier does not touch the open pact.
    f.env.mock_all_auths();
    let before = f.pact.get_pact(&pact_id);
    f.pact.set_verifier(&public_key(&f.env, &signing_key(8)));
    assert_eq!(f.pact.get_pact(&pact_id), before);
    assert_eq!(f.balance(&f.admin), 0);
    assert_eq!(f.escrow_balance(), CAP);
}

/// 5. An attestation is bound to one pact and one condition; a second verdict always fails.
#[test]
fn invariant_5_attestation_bound_and_single() {
    let f = Fixture::new();
    let a = f.open(1);
    let b = f.open(2);

    let for_b = f.attestation(&b, Verdict::Allow, CAP);
    let signature = f.sign(&for_b);
    assert_eq!(
        f.pact.try_record_verdict(&a, &for_b, &signature),
        Err(Ok(Error::AttestationMismatch))
    );

    f.verdict(&a, Verdict::Deny, 0);
    let second = f.attestation(&a, Verdict::Allow, CAP);
    let signature = f.sign(&second);
    assert_eq!(
        f.pact.try_record_verdict(&a, &second, &signature),
        Err(Ok(Error::InvalidState))
    );
    assert_eq!(f.pact.get_pact(&a).state, PactState::Denied);
}

/// 6. `set_verifier` never affects an already-open pact.
#[test]
fn invariant_6_set_verifier_spares_open_pacts() {
    let f = Fixture::new();
    let old_pact = f.open(1);
    let new_key = signing_key(8);
    f.pact.set_verifier(&public_key(&f.env, &new_key));
    let new_pact = f.open(2);

    // The open pact still trusts the old key, not the new one.
    let attestation = f.attestation(&old_pact, Verdict::Allow, CAP);
    let by_new = f.sign_with(&new_key, &attestation);
    assert!(f
        .pact
        .try_record_verdict(&old_pact, &attestation, &by_new)
        .is_err());
    f.verdict(&old_pact, Verdict::Allow, CAP);

    // The new pact trusts only the new key.
    let attestation = f.attestation(&new_pact, Verdict::Allow, CAP);
    let by_old = f.sign(&attestation);
    assert!(f
        .pact
        .try_record_verdict(&new_pact, &attestation, &by_old)
        .is_err());
    let by_new = f.sign_with(&new_key, &attestation);
    f.pact.record_verdict(&new_pact, &attestation, &by_new);
    assert_eq!(f.pact.get_pact(&new_pact).state, PactState::Allowed);
}

/// 7. settle never succeeds after `settle_deadline`; release never succeeds for an
///    `Allowed` pact before it.
#[test]
fn invariant_7_deadlines_enforced() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    for ledger in [START_LEDGER + 1, START_LEDGER + SETTLE_LEDGERS] {
        f.advance_to(ledger);
        assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::NotReleasable)));
    }
    f.advance_to(START_LEDGER + SETTLE_LEDGERS + 1);
    assert_eq!(
        f.pact.try_settle(&pact_id, &CAP),
        Err(Ok(Error::SettleWindowClosed))
    );
    f.pact.release(&pact_id);
}

/// 8. Metered pacts never accept a verdict; verdict pacts never accept `close_meter`.
#[test]
fn invariant_8_metered_and_verdict_paths_disjoint() {
    let f = Fixture::new();
    let verdict_pact = f.open(2);
    let zero = BytesN::from_array(&f.env, &[0; 64]);
    assert_eq!(
        f.pact.try_close_meter(&verdict_pact, &1, &zero),
        Err(Ok(Error::NotMetered))
    );
}

/// 8 (metered side).
#[test]
fn invariant_8_metered_pact_rejects_verdict_and_settle() {
    let f = Fixture::new();
    let metered = f.pact.open(&f.metered_params(1));
    let attestation = f.attestation(&metered, Verdict::Allow, CAP);
    let signature = f.sign(&attestation);
    assert_eq!(
        f.pact
            .try_record_verdict(&metered, &attestation, &signature),
        Err(Ok(Error::IsMetered))
    );
    assert_eq!(f.pact.try_settle(&metered, &CAP), Err(Ok(Error::IsMetered)));
}
