//! Data model of the `pact` contract. Source of truth: ARCHITECTURE.md §4.2.
//! Mirrored in TypeScript by `@pactwright/core` (schemas.ts); keep both in sync.

use soroban_sdk::{contracttype, Address, BytesN};

/// Lifecycle state. Terminal states are `Settled` and `Released` (invariant 1).
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PactState {
    Funded = 1,
    Allowed = 2,
    Denied = 3,
    Settled = 4,
    Released = 5,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Verdict {
    Allow = 1,
    Deny = 2,
}

/// What happens if no verdict arrives by `verdict_deadline`. Agreed at open (ADR-011).
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Silence {
    /// Anyone may `release` the escrow to the buyer.
    Refund = 1,
    /// The seller may `settle` up to `cap`.
    Pay = 2,
}

/// Present only on metered pacts. Metered pacts never take a verdict (invariant 8).
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeterTerms {
    /// Stroops of `Config.token` per unit.
    pub unit_price: i128,
    /// Per-pact ed25519 public key held by the buyer's SDK; signs `UsageVoucher`s.
    pub meter_key: BytesN<32>,
}

/// Whether a pact is metered. A contract enum rather than `Option<MeterTerms>`, which the
/// SDK cannot convert inside a `#[contracttype]` struct.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Metering {
    /// Verdict-gated pact.
    None,
    /// Metered pact; settled with signed usage vouchers, never a verdict (invariant 8).
    Units(MeterTerms),
}

/// Arguments of `open`, bundled in one struct.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenParams {
    pub buyer: Address,
    pub seller: Address,
    pub cap: i128,
    /// `u64_be(sha256(idempotency_key)[0..8])` — retries reuse the same pact.
    pub nonce: u64,
    pub condition_hash: BytesN<32>,
    pub meter: Metering,
    pub silence: Silence,
    pub verdict_deadline: u32,
    pub settle_deadline: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pact {
    pub buyer: Address,
    pub seller: Address,
    pub cap: i128,
    pub condition_hash: BytesN<32>,
    /// Snapshot of `Config.verifier` at open (ADR-014).
    pub verifier: BytesN<32>,
    pub meter: Metering,
    pub silence: Silence,
    pub opened_ledger: u32,
    pub verdict_deadline: u32,
    pub settle_deadline: u32,
    pub state: PactState,
    /// From the ALLOW attestation, or `cap` under `Silence::Pay`.
    pub allowed_amount: i128,
    pub envelope_hash: Option<BytesN<32>>,
    pub paid_to_seller: i128,
    pub returned_to_buyer: i128,
}

/// Compact, Soroban-verifiable form of a Witness verdict-v1 envelope (ADR-003).
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerdictAttestation {
    pub pact_id: BytesN<32>,
    pub condition_hash: BytesN<32>,
    /// sha256 of the canonical verdict-v1 envelope.
    pub envelope_hash: BytesN<32>,
    pub verdict: Verdict,
    /// Amount owed to the seller, `<= cap` (partial delivery).
    pub amount: i128,
    pub issued_ledger: u32,
}

/// Cumulative usage, signed by the buyer's `meter_key` (ADR-010).
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageVoucher {
    pub pact_id: BytesN<32>,
    pub units: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub admin: Address,
    /// The only accepted token (USDC SAC).
    pub token: Address,
    pub verifier: BytesN<32>,
    /// Upper bound on `settle_deadline - now` at open.
    pub max_pact_ledgers: u32,
}
