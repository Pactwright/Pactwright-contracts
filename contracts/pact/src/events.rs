//! Events. Topics are `("pact", <name>)` with `pact_id` first in the data.
//! ARCHITECTURE.md §4.6 · docs/EVENTS.md.

// Used once the entrypoints land (ROADMAP C-04…C-08).
#![allow(dead_code)]

use soroban_sdk::{contracttype, Address, BytesN, Env, IntoVal, Symbol, Val};

use crate::types::{Silence, Verdict};

pub const OPENED: &str = "opened";
pub const VERDICT: &str = "verdict";
pub const SETTLED: &str = "settled";
pub const METER_CLOSED: &str = "meter_closed";
pub const RELEASED: &str = "released";
pub const VERIFIER_SET: &str = "verifier_set";

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ReleaseReason {
    Denied = 1,
    Silence = 2,
    Deadline = 3,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Opened {
    pub pact_id: BytesN<32>,
    pub buyer: Address,
    pub seller: Address,
    pub cap: i128,
    pub condition_hash: BytesN<32>,
    pub metered: bool,
    pub silence: Silence,
    pub verdict_deadline: u32,
    pub settle_deadline: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerdictRecorded {
    pub pact_id: BytesN<32>,
    pub verdict: Verdict,
    pub amount: i128,
    pub envelope_hash: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settled {
    pub pact_id: BytesN<32>,
    pub to_seller: i128,
    pub to_buyer: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeterClosed {
    pub pact_id: BytesN<32>,
    pub units: u32,
    pub to_seller: i128,
    pub to_buyer: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Released {
    pub pact_id: BytesN<32>,
    pub reason: ReleaseReason,
    pub to_buyer: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifierSet {
    pub key: BytesN<32>,
}

/// Publishes `data` under the topics `("pact", name)`.
#[allow(dead_code)]
#[allow(deprecated)] // TODO(C-02): move to #[contractevent] once the event format is final.
pub(crate) fn publish<D: IntoVal<Env, Val>>(env: &Env, name: &str, data: D) {
    env.events()
        .publish((Symbol::new(env, "pact"), Symbol::new(env, name)), data);
}
