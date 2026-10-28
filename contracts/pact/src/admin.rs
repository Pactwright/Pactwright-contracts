//! Configuration. The admin can never move funds or change an open pact (invariant 4).

use soroban_sdk::{Address, BytesN, Env};

use crate::errors::Error;
use crate::events::VerifierSet;
use crate::storage;
use crate::types::Config;

pub(crate) fn init(
    env: &Env,
    admin: Address,
    token: Address,
    verifier: BytesN<32>,
    max_pact_ledgers: u32,
) {
    storage::write_config(
        env,
        &Config {
            admin,
            token,
            verifier,
            max_pact_ledgers,
        },
    );
}

/// Admin auth. Affects pacts opened afterwards only: open pacts keep the key snapshotted
/// at open (invariant 6, ADR-014).
pub(crate) fn set_verifier(env: &Env, key: BytesN<32>) -> Result<(), Error> {
    let mut config = storage::read_config(env)?;
    config.admin.require_auth();
    config.verifier = key.clone();
    storage::write_config(env, &config);
    VerifierSet { key }.publish(env);
    Ok(())
}
