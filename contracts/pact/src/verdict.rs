//! record_verdict: attestation binding + ed25519 check. ARCHITECTURE.md §4.4, ADR-003.
//! Core-team-only path (CODEOWNERS).

use soroban_sdk::{BytesN, Env};

use crate::errors::Error;
use crate::types::VerdictAttestation;

/// Requires `Funded`, not metered, `now <= verdict_deadline`, matching `pact_id` and
/// `condition_hash`, `amount <= cap`, and a valid signature by the pact's snapshotted
/// verifier key. A second call always fails because the state is no longer `Funded`.
pub(crate) fn record_verdict(
    _env: &Env,
    _pact_id: BytesN<32>,
    _attestation: VerdictAttestation,
    _signature: BytesN<64>,
) -> Result<(), Error> {
    unimplemented!("ROADMAP C-05")
}
