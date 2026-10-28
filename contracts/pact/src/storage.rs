//! Storage keys and TTL helpers. ARCHITECTURE.md §4.8.
//! An open pact's TTL must outlive `settle_deadline + GRACE_LEDGERS` (set at open).

use soroban_sdk::{contracttype, BytesN, Env};

use crate::errors::Error;
use crate::types::{Config, Pact};

/// Ledgers kept after `settle_deadline` so `release` can always run. TODO(C-04): confirm value.
#[allow(dead_code)]
pub const GRACE_LEDGERS: u32 = 17_280;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Instance storage.
    Config,
    /// Persistent storage, one entry per pact.
    Pact(BytesN<32>),
}

pub(crate) fn read_config(_env: &Env) -> Result<Config, Error> {
    unimplemented!("ROADMAP C-08")
}

#[allow(dead_code)]
pub(crate) fn write_config(_env: &Env, _config: &Config) {
    unimplemented!("ROADMAP C-08")
}

pub(crate) fn read_pact(_env: &Env, _pact_id: &BytesN<32>) -> Result<Pact, Error> {
    unimplemented!("ROADMAP C-04")
}

#[allow(dead_code)]
pub(crate) fn write_pact(_env: &Env, _pact_id: &BytesN<32>, _pact: &Pact) {
    unimplemented!("ROADMAP C-04")
}

pub(crate) fn extend_pact_ttl(_env: &Env, _pact_id: &BytesN<32>) -> Result<(), Error> {
    unimplemented!("ROADMAP C-08")
}
