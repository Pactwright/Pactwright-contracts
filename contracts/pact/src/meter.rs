//! close_meter: signed cumulative usage (ADR-010). O(1): one signature check.
//! Core-team-only path (CODEOWNERS).

use soroban_sdk::{BytesN, Env};

use crate::errors::Error;
use crate::events::MeterClosed;
use crate::hashing;
use crate::lifecycle::pay_out;
use crate::storage;
use crate::types::{Metering, PactState, UsageVoucher};

/// Seller auth. Requires a metered pact in `Funded`, `now <= settle_deadline`,
/// `units * unit_price <= cap`, and the buyer's signature (with the pact's `meter_key`) over
/// `UsageVoucher { pact_id, units }` (`ed25519_verify` traps otherwise). Pays
/// `units * unit_price` to the seller and returns the rest to the buyer.
pub(crate) fn close_meter(
    env: &Env,
    pact_id: BytesN<32>,
    units: u32,
    signature: BytesN<64>,
) -> Result<(), Error> {
    let config = storage::read_config(env)?;
    let mut pact = storage::read_pact(env, &pact_id)?;
    pact.seller.require_auth();

    let terms = match &pact.meter {
        Metering::Units(terms) => terms.clone(),
        Metering::None => return Err(Error::NotMetered),
    };
    if pact.state != PactState::Funded {
        return Err(Error::InvalidState);
    }
    if env.ledger().sequence() > pact.settle_deadline {
        return Err(Error::SettleWindowClosed);
    }
    let actual = i128::from(units)
        .checked_mul(terms.unit_price)
        .ok_or(Error::InvalidAmount)?;
    if actual > pact.cap {
        return Err(Error::InvalidAmount);
    }

    let voucher = UsageVoucher {
        pact_id: pact_id.clone(),
        units,
    };
    let digest = hashing::usage_message(env, &voucher);
    env.crypto().ed25519_verify(
        &terms.meter_key,
        &hashing::message_bytes(env, &digest),
        &signature,
    );

    let to_buyer = pact.cap - actual;
    pay_out(env, &config.token, &pact.seller, actual);
    pay_out(env, &config.token, &pact.buyer, to_buyer);

    pact.state = PactState::Settled;
    pact.allowed_amount = actual;
    pact.paid_to_seller = actual;
    pact.returned_to_buyer = to_buyer;
    storage::write_pact(env, &pact_id, &pact);
    storage::bump_instance(env);

    MeterClosed {
        pact_id,
        units,
        to_seller: actual,
        to_buyer,
    }
    .publish(env);
    Ok(())
}
