//! open / settle / release and the deadline + silence rules.
//! ARCHITECTURE.md §4.4, §5.4. Core-team-only path (CODEOWNERS).

use soroban_sdk::{token::TokenClient, Address, BytesN, Env};

use crate::errors::Error;
use crate::events::{Opened, ReleaseReason, Released, Settled};
use crate::hashing;
use crate::storage;
use crate::types::{Metering, OpenParams, Pact, PactState, Silence};

/// Moves `amount` of the configured token out of escrow. Skips zero amounts.
pub(crate) fn pay_out(env: &Env, token: &Address, to: &Address, amount: i128) {
    if amount > 0 {
        TokenClient::new(env, token).transfer(&env.current_contract_address(), to, &amount);
    }
}

/// Buyer auth. Checks: `cap > 0`; `buyer != seller`; `now < verdict_deadline <=
/// settle_deadline <= now + max_pact_ledgers`; TTL fits; `pact_id` unused. Then: escrow
/// transfer, verifier snapshot, TTL past `settle_deadline + GRACE_LEDGERS`, state `Funded`.
pub(crate) fn open(env: &Env, params: OpenParams) -> Result<BytesN<32>, Error> {
    let config = storage::read_config(env)?;
    params.buyer.require_auth();

    if params.cap <= 0 {
        return Err(Error::InvalidAmount);
    }
    if params.buyer == params.seller {
        return Err(Error::SelfDealing);
    }
    if let Metering::Units(terms) = &params.meter {
        if terms.unit_price <= 0 {
            return Err(Error::InvalidAmount);
        }
        // Metered pacts have no settlement path until close_meter ships (ROADMAP P7-01).
        // Remove this guard in P7-01 and re-enable the tests marked "ROADMAP P7-01".
        return Err(Error::IsMetered);
    }

    let now = env.ledger().sequence();
    let latest_settle = now.saturating_add(config.max_pact_ledgers);
    if now >= params.verdict_deadline
        || params.verdict_deadline > params.settle_deadline
        || params.settle_deadline > latest_settle
    {
        return Err(Error::BadDeadlines);
    }
    let ttl = (params.settle_deadline - now).saturating_add(storage::GRACE_LEDGERS);
    if ttl > env.storage().max_ttl() {
        return Err(Error::BadDeadlines);
    }

    let pact_id = hashing::pact_id(
        env,
        &params.buyer,
        &params.seller,
        params.nonce,
        &params.condition_hash,
    );
    if storage::has_pact(env, &pact_id) {
        return Err(Error::AlreadyExists);
    }

    TokenClient::new(env, &config.token).transfer(
        &params.buyer,
        env.current_contract_address(),
        &params.cap,
    );

    let metered = matches!(params.meter, Metering::Units(_));
    let pact = Pact {
        buyer: params.buyer.clone(),
        seller: params.seller.clone(),
        cap: params.cap,
        condition_hash: params.condition_hash.clone(),
        verifier: config.verifier,
        meter: params.meter,
        silence: params.silence,
        opened_ledger: now,
        verdict_deadline: params.verdict_deadline,
        settle_deadline: params.settle_deadline,
        state: PactState::Funded,
        allowed_amount: 0,
        envelope_hash: None,
        paid_to_seller: 0,
        returned_to_buyer: 0,
    };
    storage::write_pact(env, &pact_id, &pact);
    storage::extend_pact_to(env, &pact_id, ttl);
    storage::bump_instance(env);

    Opened {
        pact_id: pact_id.clone(),
        buyer: params.buyer,
        seller: params.seller,
        cap: params.cap,
        condition_hash: params.condition_hash,
        metered,
        silence: params.silence,
        verdict_deadline: params.verdict_deadline,
        settle_deadline: params.settle_deadline,
    }
    .publish(env);

    Ok(pact_id)
}

/// Seller auth. `0 < actual <= allowed_amount`, `now <= settle_deadline`, and either
/// `Allowed`, or `Funded` + not metered + `Silence::Pay` + past `verdict_deadline`
/// (then `allowed_amount = cap`).
pub(crate) fn settle(env: &Env, pact_id: BytesN<32>, actual: i128) -> Result<(), Error> {
    let config = storage::read_config(env)?;
    let mut pact = storage::read_pact(env, &pact_id)?;
    pact.seller.require_auth();

    if matches!(pact.meter, Metering::Units(_)) {
        return Err(Error::IsMetered);
    }
    let now = env.ledger().sequence();
    if now > pact.settle_deadline {
        return Err(Error::SettleWindowClosed);
    }
    match pact.state {
        PactState::Allowed => {}
        PactState::Funded if pact.silence == Silence::Pay && now > pact.verdict_deadline => {
            pact.allowed_amount = pact.cap;
        }
        _ => return Err(Error::InvalidState),
    }
    if actual <= 0 || actual > pact.allowed_amount {
        return Err(Error::InvalidAmount);
    }

    let to_buyer = pact.cap - actual;
    pay_out(env, &config.token, &pact.seller, actual);
    pay_out(env, &config.token, &pact.buyer, to_buyer);

    pact.state = PactState::Settled;
    pact.paid_to_seller = actual;
    pact.returned_to_buyer = to_buyer;
    storage::write_pact(env, &pact_id, &pact);
    storage::bump_instance(env);

    Settled {
        pact_id,
        to_seller: actual,
        to_buyer,
    }
    .publish(env);
    Ok(())
}

/// No auth; funds only ever go back to the buyer. Allowed when: `Denied`; or
/// (`Funded` | `Allowed`) + past `settle_deadline`; or `Funded` + not metered +
/// `Silence::Refund` + past `verdict_deadline`.
pub(crate) fn release(env: &Env, pact_id: BytesN<32>) -> Result<(), Error> {
    let config = storage::read_config(env)?;
    let mut pact = storage::read_pact(env, &pact_id)?;
    if pact.state.is_terminal() {
        return Err(Error::InvalidState);
    }

    let now = env.ledger().sequence();
    let metered = matches!(pact.meter, Metering::Units(_));
    let reason = match pact.state {
        PactState::Denied => ReleaseReason::Denied,
        PactState::Funded | PactState::Allowed if now > pact.settle_deadline => {
            ReleaseReason::Deadline
        }
        PactState::Funded
            if !metered && pact.silence == Silence::Refund && now > pact.verdict_deadline =>
        {
            ReleaseReason::Silence
        }
        _ => return Err(Error::NotReleasable),
    };

    pay_out(env, &config.token, &pact.buyer, pact.cap);

    pact.state = PactState::Released;
    pact.returned_to_buyer = pact.cap;
    storage::write_pact(env, &pact_id, &pact);
    storage::bump_instance(env);

    Released {
        pact_id,
        reason,
        to_buyer: pact.cap,
    }
    .publish(env);
    Ok(())
}
