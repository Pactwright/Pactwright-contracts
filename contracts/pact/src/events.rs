//! Events. Topics are `("pact", <name>)` with `pact_id` first in the data.
//! ARCHITECTURE.md §4.6 · docs/EVENTS.md.

use soroban_sdk::{contractevent, contracttype, Address, BytesN};

use crate::types::{Silence, Verdict};

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ReleaseReason {
    Denied = 1,
    Silence = 2,
    Deadline = 3,
}

#[contractevent(topics = ["pact", "opened"])]
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

#[contractevent(topics = ["pact", "verdict"])]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerdictRecorded {
    pub pact_id: BytesN<32>,
    pub verdict: Verdict,
    pub amount: i128,
    pub envelope_hash: BytesN<32>,
}

#[contractevent(topics = ["pact", "settled"])]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settled {
    pub pact_id: BytesN<32>,
    pub to_seller: i128,
    pub to_buyer: i128,
}

/// Emitted by `close_meter` (release v1, ROADMAP P7-01).
#[allow(dead_code)]
#[contractevent(topics = ["pact", "meter_closed"])]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeterClosed {
    pub pact_id: BytesN<32>,
    pub units: u32,
    pub to_seller: i128,
    pub to_buyer: i128,
}

#[contractevent(topics = ["pact", "released"])]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Released {
    pub pact_id: BytesN<32>,
    pub reason: ReleaseReason,
    pub to_buyer: i128,
}

#[contractevent(topics = ["pact", "verifier_set"])]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifierSet {
    pub key: BytesN<32>,
}
