//! record_verdict: attestation binding + ed25519 check. ARCHITECTURE.md §4.4, ADR-003.
//! Core-team-only path (CODEOWNERS).

use soroban_sdk::{BytesN, Env};

use crate::errors::Error;
use crate::events::VerdictRecorded;
use crate::hashing;
use crate::storage;
use crate::types::{Metering, PactState, Verdict, VerdictAttestation};

/// No auth: the signature is the authority. Requires `Funded`, not metered,
/// `now <= verdict_deadline`, matching `pact_id` and `condition_hash`,
/// `opened_ledger <= issued_ledger <= now`, `0 < amount <= cap` for ALLOW, and a valid
/// signature by the pact's snapshotted verifier key (`ed25519_verify` traps otherwise).
/// A second call always fails because the state is no longer `Funded` (invariant 5).
pub(crate) fn record_verdict(
    env: &Env,
    pact_id: BytesN<32>,
    attestation: VerdictAttestation,
    signature: BytesN<64>,
) -> Result<(), Error> {
    let mut pact = storage::read_pact(env, &pact_id)?;
    if pact.state != PactState::Funded {
        return Err(Error::InvalidState);
    }
    if matches!(pact.meter, Metering::Units(_)) {
        return Err(Error::IsMetered);
    }
    let now = env.ledger().sequence();
    if now > pact.verdict_deadline {
        return Err(Error::VerdictWindowClosed);
    }
    if attestation.pact_id != pact_id
        || attestation.condition_hash != pact.condition_hash
        || attestation.issued_ledger < pact.opened_ledger
        || attestation.issued_ledger > now
    {
        return Err(Error::AttestationMismatch);
    }
    let amount_ok = match attestation.verdict {
        Verdict::Allow => attestation.amount > 0 && attestation.amount <= pact.cap,
        Verdict::Deny => attestation.amount == 0,
    };
    if !amount_ok {
        return Err(Error::InvalidAmount);
    }

    let digest = hashing::verdict_message(env, &attestation);
    env.crypto().ed25519_verify(
        &pact.verifier,
        &hashing::message_bytes(env, &digest),
        &signature,
    );

    match attestation.verdict {
        Verdict::Allow => {
            pact.state = PactState::Allowed;
            pact.allowed_amount = attestation.amount;
        }
        Verdict::Deny => pact.state = PactState::Denied,
    }
    pact.envelope_hash = Some(attestation.envelope_hash.clone());
    storage::write_pact(env, &pact_id, &pact);
    storage::bump_instance(env);

    VerdictRecorded {
        pact_id,
        verdict: attestation.verdict,
        amount: attestation.amount,
        envelope_hash: attestation.envelope_hash,
    }
    .publish(env);
    Ok(())
}
