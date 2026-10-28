//! Pactwright `pact` contract — M2 escrow.
//!
//! Entrypoints only; logic lives in the modules below. Design: ARCHITECTURE.md §4.
//! Rules you must not break: ARCHITECTURE_ESSENTIALS.md §4.
#![no_std]

mod admin;
mod errors;
mod events;
mod hashing;
mod lifecycle;
mod meter;
mod storage;
mod types;
mod verdict;

#[cfg(test)]
mod test;

pub use errors::Error;
pub use events::ReleaseReason;
pub use types::{
    Config, MeterTerms, Metering, OpenParams, Pact, PactState, Silence, UsageVoucher, Verdict,
    VerdictAttestation,
};

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

#[contract]
pub struct PactContract;

#[contractimpl]
impl PactContract {
    /// Stores `Config`. ARCHITECTURE §4.4.
    pub fn __constructor(
        env: Env,
        admin: Address,
        token: Address,
        verifier: BytesN<32>,
        max_pact_ledgers: u32,
    ) {
        admin::init(&env, admin, token, verifier, max_pact_ledgers)
    }

    /// Rotates the verifier key for **new** pacts only (ADR-014). Admin auth.
    pub fn set_verifier(env: Env, key: BytesN<32>) -> Result<(), Error> {
        admin::set_verifier(&env, key)
    }

    /// Opens a pact and moves `cap` into escrow. Buyer auth. Returns `pact_id`.
    pub fn open(env: Env, params: OpenParams) -> Result<BytesN<32>, Error> {
        lifecycle::open(&env, params)
    }

    /// Records the single verdict for a non-metered pact. No auth: the signature is the authority.
    pub fn record_verdict(
        env: Env,
        pact_id: BytesN<32>,
        attestation: VerdictAttestation,
        signature: BytesN<64>,
    ) -> Result<(), Error> {
        verdict::record_verdict(&env, pact_id, attestation, signature)
    }

    /// Pays `actual` to the seller and returns the rest to the buyer. Seller auth.
    pub fn settle(env: Env, pact_id: BytesN<32>, actual: i128) -> Result<(), Error> {
        lifecycle::settle(&env, pact_id, actual)
    }

    /// Settles a metered pact from the buyer's latest signed usage voucher. Seller auth.
    pub fn close_meter(
        env: Env,
        pact_id: BytesN<32>,
        units: u32,
        signature: BytesN<64>,
    ) -> Result<(), Error> {
        meter::close_meter(&env, pact_id, units, signature)
    }

    /// Returns the full escrow to the buyer when the release rules allow it. No auth.
    pub fn release(env: Env, pact_id: BytesN<32>) -> Result<(), Error> {
        lifecycle::release(&env, pact_id)
    }

    pub fn get_pact(env: Env, pact_id: BytesN<32>) -> Result<Pact, Error> {
        storage::read_pact(&env, &pact_id)
    }

    pub fn get_config(env: Env) -> Result<Config, Error> {
        storage::read_config(&env)
    }

    /// Bumps the TTL of a non-terminal pact. No auth.
    pub fn extend_pact_ttl(env: Env, pact_id: BytesN<32>) -> Result<(), Error> {
        storage::extend_pact_ttl(&env, &pact_id)
    }
}
