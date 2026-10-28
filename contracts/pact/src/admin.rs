//! Configuration. The admin can never move funds or change an open pact (invariant 4).

use soroban_sdk::{Address, BytesN, Env};

use crate::errors::Error;

pub(crate) fn init(
    _env: &Env,
    _admin: Address,
    _token: Address,
    _verifier: BytesN<32>,
    _max_pact_ledgers: u32,
) {
    unimplemented!("ROADMAP C-08")
}

/// Admin auth. Affects pacts opened afterwards only (invariant 6). Emits `verifier_set`.
pub(crate) fn set_verifier(_env: &Env, _key: BytesN<32>) -> Result<(), Error> {
    unimplemented!("ROADMAP C-08")
}
