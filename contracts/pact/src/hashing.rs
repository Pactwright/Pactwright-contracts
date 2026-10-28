//! Identifiers and signed-message digests. ARCHITECTURE.md §4.3, ADR-015.
//!
//! **This file and `pactwright-kit/packages/core/src/hashing.ts` must produce identical
//! bytes.** Both are tested against `test-vectors/` (generated from this crate).

#![allow(dead_code)]

use soroban_sdk::{Address, BytesN, Env};

use crate::types::{UsageVoucher, VerdictAttestation};

pub const VERDICT_DOMAIN: &[u8] = b"pactwright-verdict-v1:";
pub const USAGE_DOMAIN: &[u8] = b"pactwright-usage-v1:";
pub const DELIVERY_DOMAIN: &[u8] = b"pactwright-delivery-v1:";
pub const RECEIPT_DOMAIN: &[u8] = b"pactwright-receipt-v1";

/// `sha256(xdr(buyer) ‖ xdr(seller) ‖ nonce_u64_be ‖ condition_hash)`
pub(crate) fn pact_id(
    _env: &Env,
    _buyer: &Address,
    _seller: &Address,
    _nonce: u64,
    _condition_hash: &BytesN<32>,
) -> BytesN<32> {
    unimplemented!("ROADMAP C-03")
}

/// `sha256(VERDICT_DOMAIN ‖ xdr(attestation))` — the bytes the verifier signs.
pub(crate) fn verdict_message(_env: &Env, _attestation: &VerdictAttestation) -> BytesN<32> {
    unimplemented!("ROADMAP C-03")
}

/// `sha256(USAGE_DOMAIN ‖ xdr(voucher))` — the bytes the buyer's meter key signs.
pub(crate) fn usage_message(_env: &Env, _voucher: &UsageVoucher) -> BytesN<32> {
    unimplemented!("ROADMAP C-03")
}

/// `sha256(RECEIPT_DOMAIN ‖ pact_id ‖ transition_u32 ‖ to_seller_i128 ‖ to_buyer_i128 ‖
/// envelope_hash_or_zero ‖ ledger_u32)` — computed off-chain by the anchorer; defined here
/// so the test vectors have one source. Transitions: 1 Paid, 2 Settled, 3 Released, 4 Refunded.
pub(crate) fn receipt_leaf(
    _env: &Env,
    _pact_id: &BytesN<32>,
    _transition: u32,
    _to_seller: i128,
    _to_buyer: i128,
    _envelope_hash: Option<BytesN<32>>,
    _ledger: u32,
) -> BytesN<32> {
    unimplemented!("ROADMAP C-03")
}
