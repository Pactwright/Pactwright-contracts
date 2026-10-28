//! Fake-clock tests for every deadline boundary and both silence policies (ROADMAP C-10).

use super::*;
use crate::{Error, PactState, Silence, Verdict};

#[test]
fn verdict_accepted_at_verdict_deadline() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.advance_to(START_LEDGER + VERDICT_LEDGERS);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    assert_eq!(f.pact.get_pact(&pact_id).state, PactState::Allowed);
}

#[test]
fn verdict_rejected_after_verdict_deadline() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.advance_to(START_LEDGER + VERDICT_LEDGERS + 1);
    let attestation = f.attestation(&pact_id, Verdict::Allow, CAP);
    let signature = f.sign(&attestation);
    assert_eq!(
        f.pact
            .try_record_verdict(&pact_id, &attestation, &signature),
        Err(Ok(Error::VerdictWindowClosed))
    );
}

#[test]
fn silence_refund_allows_release_after_verdict_deadline() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.advance_to(START_LEDGER + VERDICT_LEDGERS);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::NotReleasable)));
    f.advance_to(START_LEDGER + VERDICT_LEDGERS + 1);
    f.pact.release(&pact_id);
    assert_eq!(f.balance(&f.buyer), FUNDS);
}

#[test]
fn silence_refund_does_not_let_seller_settle() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.advance_to(START_LEDGER + VERDICT_LEDGERS + 1);
    assert_eq!(
        f.pact.try_settle(&pact_id, &CAP),
        Err(Ok(Error::InvalidState))
    );
}

#[test]
fn silence_pay_allows_settle_after_verdict_deadline() {
    let f = Fixture::new();
    let mut params = f.params(1);
    params.silence = Silence::Pay;
    let pact_id = f.pact.open(&params);

    assert_eq!(
        f.pact.try_settle(&pact_id, &CAP),
        Err(Ok(Error::InvalidState))
    );
    f.advance_to(START_LEDGER + VERDICT_LEDGERS + 1);
    f.pact.settle(&pact_id, &CAP);
    assert_eq!(f.balance(&f.seller), CAP);
    assert_eq!(f.pact.get_pact(&pact_id).allowed_amount, CAP);
}

#[test]
fn silence_pay_does_not_allow_early_release() {
    let f = Fixture::new();
    let mut params = f.params(1);
    params.silence = Silence::Pay;
    let pact_id = f.pact.open(&params);
    f.advance_to(START_LEDGER + VERDICT_LEDGERS + 1);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::NotReleasable)));
}

#[test]
fn settle_accepted_at_settle_deadline() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    f.advance_to(START_LEDGER + SETTLE_LEDGERS);
    f.pact.settle(&pact_id, &CAP);
    assert_eq!(f.pact.get_pact(&pact_id).state, PactState::Settled);
}

#[test]
fn settle_rejected_after_settle_deadline() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    f.advance_to(START_LEDGER + SETTLE_LEDGERS + 1);
    assert_eq!(
        f.pact.try_settle(&pact_id, &CAP),
        Err(Ok(Error::SettleWindowClosed))
    );
}

#[test]
fn allowed_pact_releasable_after_settle_deadline() {
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    f.advance_to(START_LEDGER + SETTLE_LEDGERS);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::NotReleasable)));
    f.advance_to(START_LEDGER + SETTLE_LEDGERS + 1);
    f.pact.release(&pact_id);
    assert_eq!(f.balance(&f.buyer), FUNDS);
}

#[test]
#[ignore = "ROADMAP P7-01: metering"]
fn metered_pact_releasable_only_after_settle_deadline() {
    let f = Fixture::new();
    let pact_id = f.pact.open(&f.metered_params(1));
    f.advance_to(START_LEDGER + VERDICT_LEDGERS + 1);
    assert_eq!(f.pact.try_release(&pact_id), Err(Ok(Error::NotReleasable)));
    f.advance_to(START_LEDGER + SETTLE_LEDGERS + 1);
    f.pact.release(&pact_id);
    assert_eq!(f.balance(&f.buyer), FUNDS);
}

#[test]
fn open_pact_survives_until_release_window() {
    // The pact entry's TTL must cover settle_deadline + grace, so release still works.
    let f = Fixture::new();
    let pact_id = f.open(1);
    f.advance_to(START_LEDGER + SETTLE_LEDGERS + crate::storage::GRACE_LEDGERS - 1);
    f.pact.release(&pact_id);
    assert_eq!(f.balance(&f.buyer), FUNDS);
}
