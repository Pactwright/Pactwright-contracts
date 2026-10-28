//! open / settle / release and the deadline + silence rules.
//! ARCHITECTURE.md §4.4, §5.4. Core-team-only path (CODEOWNERS).

use soroban_sdk::{BytesN, Env};

use crate::errors::Error;
use crate::types::OpenParams;

/// Checks: `cap > 0`; `buyer != seller`; `now < verdict_deadline <= settle_deadline
/// <= now + max_pact_ledgers`; `pact_id` unused. Then: escrow transfer, verifier snapshot,
/// TTL past `settle_deadline + GRACE_LEDGERS`, state `Funded`, event `opened`.
pub(crate) fn open(_env: &Env, _params: OpenParams) -> Result<BytesN<32>, Error> {
    unimplemented!("ROADMAP C-04")
}

/// Seller auth. `0 < actual <= allowed_amount`, `now <= settle_deadline`, and either
/// `Allowed`, or `Funded` + not metered + `Silence::Pay` + past `verdict_deadline`.
pub(crate) fn settle(_env: &Env, _pact_id: BytesN<32>, _actual: i128) -> Result<(), Error> {
    unimplemented!("ROADMAP C-06")
}

/// No auth. Allowed when: `Denied`; or `Funded` + not metered + `Silence::Refund` + past
/// `verdict_deadline`; or (`Funded` | `Allowed`) + past `settle_deadline`.
pub(crate) fn release(_env: &Env, _pact_id: BytesN<32>) -> Result<(), Error> {
    unimplemented!("ROADMAP C-07")
}
