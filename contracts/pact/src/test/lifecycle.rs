//! Every entrypoint x every error (ROADMAP C-09).

#[test]
#[ignore = "ROADMAP C-09"]
fn open_rejects_zero_cap() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn open_rejects_self_dealing() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn open_rejects_bad_deadlines() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn open_rejects_reused_pact_id() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn open_moves_cap_into_escrow() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn record_verdict_rejects_wrong_pact_id() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn record_verdict_rejects_amount_above_cap() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn record_verdict_rejects_metered_pact() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn settle_requires_seller_auth() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn settle_rejects_actual_above_allowed() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn release_rejects_allowed_before_deadline() {}

#[test]
#[ignore = "ROADMAP C-09"]
fn set_verifier_requires_admin() {}
