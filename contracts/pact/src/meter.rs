//! close_meter: signed cumulative usage (ADR-010). O(1): one signature check.
//! Release v1 (ROADMAP P7-01). Core-team-only path (CODEOWNERS).

use soroban_sdk::{BytesN, Env};

use crate::errors::Error;

/// Seller auth. Requires `Funded`, metered, `now <= settle_deadline`, a valid signature by
/// `meter_key` over `UsageVoucher { pact_id, units }`, and `units * unit_price <= cap`.
pub(crate) fn close_meter(
    _env: &Env,
    _pact_id: BytesN<32>,
    _units: u32,
    _signature: BytesN<64>,
) -> Result<(), Error> {
    unimplemented!("ROADMAP P7-01")
}
