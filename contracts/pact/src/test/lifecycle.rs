//! Every entrypoint × every reachable error (ROADMAP C-09).

use soroban_sdk::BytesN;

use super::*;
use crate::{Error, PactState, Verdict};

// ---------------------------------------------------------------- open

#[test]
fn open_moves_cap_into_escrow() {
    let f = Fixture::new();
    let pact_id = f.open(1);

    assert_eq!(f.escrow_balance(), CAP);
    assert_eq!(f.balance(&f.buyer), FUNDS - CAP);
    let pact = f.pact.get_pact(&pact_id);
    assert_eq!(pact.state, PactState::Funded);
    assert_eq!(pact.cap, CAP);
    assert_eq!(pact.opened_ledger, START_LEDGER);
    assert_eq!(pact.verifier, public_key(&f.env, &f.verifier));
}

#[test]
fn open_requires_buyer_auth() {
    let f = Fixture::new();
    f.open(1);
    let auths = f.env.auths();
    assert_eq!(auths[0].0, f.buyer);
}

#[test]
fn open_returns_deterministic_pact_id() {
    let f = Fixture::new();
    let params = f.params(42);
    let expected = hashing::pact_id(
        &f.env,
        &params.buyer,
        &params.seller,
        params.nonce,
        &params.condition_hash,
    );
    assert_eq!(f.pact.open(&params), expected);
}

#[test]
fn open_rejects_zero_cap() {
    let f = Fixture::new();
    let mut params = f.params(1);
    params.cap = 0;
    assert_eq!(f.pact.try_open(&params), Err(Ok(Error::InvalidAmount)));
}

#[test]
fn open_rejects_non_positive_unit_price() {
    let f = Fixture::new();
    let mut params = f.metered_params(1);
    if let Metering::Units(terms) = &mut params.meter {
        terms.unit_price = 0;
    }
    assert_eq!(f.pact.try_open(&params), Err(Ok(Error::InvalidAmount)));
}

#[test]
fn open_rejects_metered_until_p7_01() {
    let f = Fixture::new();
    assert_eq!(
        f.pact.try_open(&f.metered_params(1)),
        Err(Ok(Error::IsMetered))
    );
    assert_eq!(f.escrow_balance(), 0);
}

#[test]
fn open_rejects_self_dealing() {
    let f = Fixture::new();
    let mut params = f.params(1);
    params.seller = params.buyer.clone();
    assert_eq!(f.pact.try_open(&params), Err(Ok(Error::SelfDealing)));
}

#[test]
fn open_rejects_bad_deadlines() {
    let f = Fixture::new();
    let now = f.now();

    let mut past = f.params(1);
    past.verdict_deadline = now;
    assert_eq!(f.pact.try_open(&past), Err(Ok(Error::BadDeadlines)));

    let mut inverted = f.params(2);
    inverted.verdict_deadline = now + 100;
    inverted.settle_deadline = now + 99;
    assert_eq!(f.pact.try_open(&inverted), Err(Ok(Error::BadDeadlines)));

    let mut too_long = f.params(3);
    too_long.settle_deadline = now + MAX_PACT_LEDGERS + 1;
    assert_eq!(f.pact.try_open(&too_long), Err(Ok(Error::BadDeadlines)));
}

#[test]
fn open_rejects_reused_pact_id() {
    let f = Fixture::new();
    f.open(1);
    assert_eq!(f.pact.try_open(&f.params(1)), Err(Ok(Error::AlreadyExists)));
}

#[test]
fn open_fails_when_buyer_cannot_fund() {
    let f = Fixture::new();
    let mut params = f.params(1);
    params.cap = FUNDS + 1;
    assert!(f.pact.try_open(&params).is_err());
    assert_eq!(f.escrow_balance(), 0);
}

// ---------------------------------------------------------------- record_verdict

#[test]
fn record_verdict_allow_sets_allowed_amount() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP / 2);

    let pact = f.pact.get_pact(&pact_id);
    assert_eq!(pact.state, PactState::Allowed);
    assert_eq!(pact.allowed_amount, CAP / 2);
    assert_eq!(
        pact.envelope_hash,
        Some(BytesN::from_array(&f.env, &[0xee; 32]))
    );
}

#[test]
fn record_verdict_deny_sets_denied() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Deny, 0);
    assert_eq!(f.pact.get_pact(&pact_id).state, PactState::Denied);
}

#[test]
fn record_verdict_rejects_unknown_pact() {
    let f = Fixture::new();
    let pact_id = BytesN::from_array(&f.env, &[9; 32]);
    let attestation = f.attestation(&pact_id, Verdict::Allow, CAP);
    let signature = f.sign(&attestation);
    assert_eq!(
        f.pact
            .try_record_verdict(&pact_id, &attestation, &signature),
        Err(Ok(Error::PactNotFound))
    );
}

#[test]
fn record_verdict_rejects_wrong_pact_id() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    let other = f.open(2);
    let attestation = f.attestation(&other, Verdict::Allow, CAP);
    let signature = f.sign(&attestation);
    assert_eq!(
        f.pact
            .try_record_verdict(&pact_id, &attestation, &signature),
        Err(Ok(Error::AttestationMismatch))
    );
}

#[test]
fn record_verdict_rejects_wrong_condition_hash() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    let mut attestation = f.attestation(&pact_id, Verdict::Allow, CAP);
    attestation.condition_hash = f.condition_hash(2);
    let signature = f.sign(&attestation);
    assert_eq!(
        f.pact
            .try_record_verdict(&pact_id, &attestation, &signature),
        Err(Ok(Error::AttestationMismatch))
    );
}

#[test]
fn record_verdict_rejects_issued_ledger_outside_pact_life() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    for issued in [START_LEDGER - 1, START_LEDGER + 1] {
        let mut attestation = f.attestation(&pact_id, Verdict::Allow, CAP);
        attestation.issued_ledger = issued;
        let signature = f.sign(&attestation);
        assert_eq!(
            f.pact
                .try_record_verdict(&pact_id, &attestation, &signature),
            Err(Ok(Error::AttestationMismatch))
        );
    }
}

#[test]
fn record_verdict_rejects_bad_amounts() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    for (verdict, amount) in [
        (Verdict::Allow, 0),
        (Verdict::Allow, CAP + 1),
        (Verdict::Deny, 1),
    ] {
        let attestation = f.attestation(&pact_id, verdict, amount);
        let signature = f.sign(&attestation);
        assert_eq!(
            f.pact
                .try_record_verdict(&pact_id, &attestation, &signature),
            Err(Ok(Error::InvalidAmount))
        );
    }
}

#[test]
fn record_verdict_rejects_bad_signature() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    let attestation = f.attestation(&pact_id, Verdict::Allow, CAP);
    let forged = f.sign_with(&signing_key(8), &attestation);
    assert!(f
        .pact
        .try_record_verdict(&pact_id, &attestation, &forged)
        .is_err());
    assert_eq!(f.pact.get_pact(&pact_id).state, PactState::Funded);
}

#[test]
fn record_verdict_rejects_tampered_attestation() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    let attestation = f.attestation(&pact_id, Verdict::Allow, CAP / 2);
    let signature = f.sign(&attestation);
    let mut tampered = attestation.clone();
    tampered.amount = CAP;
    assert!(f
        .pact
        .try_record_verdict(&pact_id, &tampered, &signature)
        .is_err());
}

#[test]
#[ignore = "ROADMAP P7-01: metering"]
fn record_verdict_rejects_metered_pact() {
    let f = Fixture::new();
    let pact_id = f.pact.open(&f.metered_params(1));
    let attestation = f.attestation(&pact_id, Verdict::Allow, CAP);
    let signature = f.sign(&attestation);
    assert_eq!(
        f.pact
            .try_record_verdict(&pact_id, &attestation, &signature),
        Err(Ok(Error::IsMetered))
    );
}

// ---------------------------------------------------------------- settle

#[test]
fn settle_pays_seller_and_returns_rest() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    f.pact.settle(&pact_id, &(CAP * 3 / 4));

    assert_eq!(f.balance(&f.seller), CAP * 3 / 4);
    assert_eq!(f.balance(&f.buyer), FUNDS - CAP * 3 / 4);
    assert_eq!(f.escrow_balance(), 0);
    let pact = f.pact.get_pact(&pact_id);
    assert_eq!(pact.state, PactState::Settled);
    assert_eq!(pact.paid_to_seller, CAP * 3 / 4);
    assert_eq!(pact.returned_to_buyer, CAP / 4);
}

#[test]
fn settle_requires_seller_auth() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    f.pact.settle(&pact_id, &CAP);
    assert_eq!(f.env.auths()[0].0, f.seller);
}

#[test]
fn settle_rejects_actual_above_allowed() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP / 2);
    assert_eq!(
        f.pact.try_settle(&pact_id, &(CAP / 2 + 1)),
        Err(Ok(Error::InvalidAmount))
    );
    assert_eq!(
        f.pact.try_settle(&pact_id, &0),
        Err(Ok(Error::InvalidAmount))
    );
}

#[test]
fn settle_rejects_funded_without_verdict() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    assert_eq!(
        f.pact.try_settle(&pact_id, &CAP),
        Err(Ok(Error::InvalidState))
    );
}

#[test]
fn settle_rejects_denied_pact() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Deny, 0);
    assert_eq!(
        f.pact.try_settle(&pact_id, &CAP),
        Err(Ok(Error::InvalidState))
    );
}

#[test]
#[ignore = "ROADMAP P7-01: metering"]
fn settle_rejects_metered_pact() {
    let f = Fixture::new();
    let pact_id = f.pact.open(&f.metered_params(1));
    assert_eq!(f.pact.try_settle(&pact_id, &CAP), Err(Ok(Error::IsMetered)));
}

#[test]
fn settle_rejects_unknown_pact() {
    let f = Fixture::new();
    let pact_id = BytesN::from_array(&f.env, &[9; 32]);
    assert_eq!(
        f.pact.try_settle(&pact_id, &CAP),
        Err(Ok(Error::PactNotFound))
    );
}

// ---------------------------------------------------------------- release

#[test]
fn release_returns_escrow_after_deny() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Deny, 0);
    f.pact.release(&pact_id);

    assert_eq!(f.balance(&f.buyer), FUNDS);
    assert_eq!(f.escrow_balance(), 0);
    let pact = f.pact.get_pact(&pact_id);
    assert_eq!(pact.state, PactState::Released);
    assert_eq!(pact.returned_to_buyer, CAP);
}

#[test]
fn release_needs_no_auth() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Deny, 0);
    f.pact.release(&pact_id);
    assert!(f.env.auths().is_empty());
}

#[test]
fn release_rejects_allowed_before_deadline() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::NotReleasable)));
}

#[test]
fn release_rejects_funded_inside_verdict_window() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::NotReleasable)));
}

#[test]
fn release_rejects_terminal_pact() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Deny, 0);
    f.pact.release(&pact_id);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::InvalidState)));
}

#[test]
fn release_rejects_unknown_pact() {
    let f = Fixture::new();
    let pact_id = BytesN::from_array(&f.env, &[9; 32]);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::PactNotFound)));
}

// ---------------------------------------------------------------- admin and reads

#[test]
fn set_verifier_requires_admin() {
    let f = Fixture::new();
    let key = public_key(&f.env, &signing_key(8));
    f.pact.set_verifier(&key);
    assert_eq!(f.env.auths()[0].0, f.admin);
    assert_eq!(f.pact.get_config().verifier, key);
}

#[test]
fn set_verifier_fails_without_admin_auth() {
    let f = Fixture::new();
    f.env.set_auths(&[]);
    let key = public_key(&f.env, &signing_key(8));
    assert!(f.pact.try_set_verifier(&key).is_err());
}

#[test]
fn get_config_returns_constructor_values() {
    let f = Fixture::new();
    let config = f.pact.get_config();
    assert_eq!(config.admin, f.admin);
    assert_eq!(config.token, f.token.address);
    assert_eq!(config.max_pact_ledgers, MAX_PACT_LEDGERS);
}

#[test]
fn get_pact_rejects_unknown_pact() {
    let f = Fixture::new();
    let pact_id = BytesN::from_array(&f.env, &[9; 32]);
    assert_eq!(f.pact.try_get_pact(&pact_id), Err(Ok(Error::PactNotFound)));
}

#[test]
fn extend_pact_ttl_works_until_terminal() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.pact.extend_pact_ttl(&pact_id);
    f.verdict(&pact_id, Verdict::Deny, 0);
    f.pact.release(&pact_id);
    assert_eq!(
        f.pact.try_extend_pact_ttl(&pact_id),
        Err(Ok(Error::InvalidState))
    );
}

#[test]
#[ignore = "ROADMAP P7-01: metering"]
fn close_meter_is_not_available_yet() {
    // Metering lands in release v1 (ROADMAP P7-01); until then close_meter must not settle.
    let f = Fixture::new();
    let pact_id = f.pact.open(&f.metered_params(1));
    let signature = BytesN::from_array(&f.env, &[0; 64]);
    assert!(f.pact.try_close_meter(&pact_id, &1, &signature).is_err());
    assert_eq!(f.escrow_balance(), CAP);
}
