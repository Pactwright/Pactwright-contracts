//! Storage keys and TTL helpers. ARCHITECTURE.md §4.8.
//! An open pact's TTL must outlive `settle_deadline + GRACE_LEDGERS` (set at open), so funds
//! can never sit in an archived entry.

use soroban_sdk::{contracttype, BytesN, Env};

use crate::errors::Error;
use crate::types::{Config, Pact};

/// Ledgers per day at ~5 s per ledger.
pub const DAY_LEDGERS: u32 = 17_280;
/// Ledgers kept after `settle_deadline` so `release` can always run.
pub const GRACE_LEDGERS: u32 = DAY_LEDGERS;
const INSTANCE_BUMP_THRESHOLD: u32 = 7 * DAY_LEDGERS;
const INSTANCE_BUMP_TO: u32 = 30 * DAY_LEDGERS;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Instance storage.
    Config,
    /// Persistent storage, one entry per pact.
    Pact(BytesN<32>),
}

pub(crate) fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_THRESHOLD, INSTANCE_BUMP_TO);
}

pub(crate) fn read_config(env: &Env) -> Result<Config, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Config)
        .ok_or(Error::NotInitialized)
}

pub(crate) fn write_config(env: &Env, config: &Config) {
    env.storage().instance().set(&DataKey::Config, config);
    bump_instance(env);
}

pub(crate) fn has_pact(env: &Env, pact_id: &BytesN<32>) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Pact(pact_id.clone()))
}

pub(crate) fn read_pact(env: &Env, pact_id: &BytesN<32>) -> Result<Pact, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::Pact(pact_id.clone()))
        .ok_or(Error::PactNotFound)
}

pub(crate) fn write_pact(env: &Env, pact_id: &BytesN<32>, pact: &Pact) {
    env.storage()
        .persistent()
        .set(&DataKey::Pact(pact_id.clone()), pact);
}

/// Ledgers of TTL an open pact needs from now: until `settle_deadline`, plus the grace period.
pub(crate) fn ttl_needed(env: &Env, pact: &Pact) -> u32 {
    pact.settle_deadline
        .saturating_sub(env.ledger().sequence())
        .saturating_add(GRACE_LEDGERS)
}

pub(crate) fn extend_pact_to(env: &Env, pact_id: &BytesN<32>, extend_to: u32) {
    env.storage()
        .persistent()
        .extend_ttl(&DataKey::Pact(pact_id.clone()), extend_to, extend_to);
}

/// No auth. Only non-terminal pacts can be extended.
pub(crate) fn extend_pact_ttl(env: &Env, pact_id: &BytesN<32>) -> Result<(), Error> {
    let pact = read_pact(env, pact_id)?;
    if pact.state.is_terminal() {
        return Err(Error::InvalidState);
    }
    let extend_to = ttl_needed(env, &pact).min(env.storage().max_ttl());
    extend_pact_to(env, pact_id, extend_to);
    bump_instance(env);
    Ok(())
}
