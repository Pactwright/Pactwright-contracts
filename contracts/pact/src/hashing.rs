//! Identifiers and signed-message digests. ARCHITECTURE.md §4.3, ADR-015.
//!
//! **This file and `Pactwright-Kit/packages/core/src/hashing.ts` must produce identical
//! bytes.** Both are tested against `test-vectors/`, which this crate generates
//! (`scripts/gen_vectors.sh`).
//!
//! Encoding rules: `xdr(x)` is the XDR of the value's `ScVal`; integers are big-endian;
//! `‖` is byte concatenation.

use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env};

use crate::types::{UsageVoucher, VerdictAttestation};

pub const VERDICT_DOMAIN: &[u8] = b"pactwright-verdict-v1:";
pub const USAGE_DOMAIN: &[u8] = b"pactwright-usage-v1:";
pub const DELIVERY_DOMAIN: &[u8] = b"pactwright-delivery-v1:";
pub const RECEIPT_DOMAIN: &[u8] = b"pactwright-receipt-v1";

fn sha256(env: &Env, data: &Bytes) -> BytesN<32> {
    env.crypto().sha256(data).to_bytes()
}

fn as_bytes(env: &Env, value: &BytesN<32>) -> Bytes {
    Bytes::from_array(env, &value.to_array())
}

/// `sha256(domain ‖ payload)`
fn domain_digest(env: &Env, domain: &[u8], payload: &Bytes) -> BytesN<32> {
    let mut data = Bytes::from_slice(env, domain);
    data.append(payload);
    sha256(env, &data)
}

/// `u64_be(sha256(idempotency_key)[0..8])`. Off-chain; defined here for the test vectors.
#[allow(dead_code)]
pub(crate) fn nonce_from_key(env: &Env, idempotency_key: &Bytes) -> u64 {
    let digest = sha256(env, idempotency_key).to_array();
    let mut first = [0u8; 8];
    first.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(first)
}

/// Preimage of `pact_id`: `xdr(buyer) ‖ xdr(seller) ‖ nonce_u64_be ‖ condition_hash`.
pub(crate) fn pact_id_preimage(
    env: &Env,
    buyer: &Address,
    seller: &Address,
    nonce: u64,
    condition_hash: &BytesN<32>,
) -> Bytes {
    let mut data = buyer.clone().to_xdr(env);
    data.append(&seller.clone().to_xdr(env));
    data.extend_from_array(&nonce.to_be_bytes());
    data.append(&as_bytes(env, condition_hash));
    data
}

/// `sha256(xdr(buyer) ‖ xdr(seller) ‖ nonce_u64_be ‖ condition_hash)`
pub(crate) fn pact_id(
    env: &Env,
    buyer: &Address,
    seller: &Address,
    nonce: u64,
    condition_hash: &BytesN<32>,
) -> BytesN<32> {
    sha256(
        env,
        &pact_id_preimage(env, buyer, seller, nonce, condition_hash),
    )
}

/// `sha256("pactwright-verdict-v1:" ‖ xdr(attestation))` — the bytes the verifier signs.
pub(crate) fn verdict_message(env: &Env, attestation: &VerdictAttestation) -> BytesN<32> {
    domain_digest(env, VERDICT_DOMAIN, &attestation.clone().to_xdr(env))
}

/// `sha256("pactwright-usage-v1:" ‖ xdr(voucher))` — the bytes the buyer's meter key signs.
pub(crate) fn usage_message(env: &Env, voucher: &UsageVoucher) -> BytesN<32> {
    domain_digest(env, USAGE_DOMAIN, &voucher.clone().to_xdr(env))
}

/// `sha256("pactwright-delivery-v1:" ‖ pact_id ‖ body_sha256)`. Off-chain (seller signs);
/// defined here for the test vectors.
#[allow(dead_code)]
pub(crate) fn delivery_message(
    env: &Env,
    pact_id: &BytesN<32>,
    body_sha256: &BytesN<32>,
) -> BytesN<32> {
    let mut payload = as_bytes(env, pact_id);
    payload.append(&as_bytes(env, body_sha256));
    domain_digest(env, DELIVERY_DOMAIN, &payload)
}

/// `sha256("pactwright-receipt-v1" ‖ pact_id ‖ transition_u32 ‖ to_seller_i128 ‖
/// to_buyer_i128 ‖ envelope_hash_or_32_zero_bytes ‖ ledger_u32)`. Computed off-chain by the
/// anchorer; defined here so the test vectors have one source.
/// Transitions: 1 Paid, 2 Settled, 3 Released, 4 Refunded.
#[allow(dead_code)]
pub(crate) fn receipt_leaf(
    env: &Env,
    pact_id: &BytesN<32>,
    transition: u32,
    to_seller: i128,
    to_buyer: i128,
    envelope_hash: Option<BytesN<32>>,
    ledger: u32,
) -> BytesN<32> {
    let mut payload = as_bytes(env, pact_id);
    payload.extend_from_array(&transition.to_be_bytes());
    payload.extend_from_array(&to_seller.to_be_bytes());
    payload.extend_from_array(&to_buyer.to_be_bytes());
    let envelope = envelope_hash.map(|h| h.to_array()).unwrap_or([0u8; 32]);
    payload.extend_from_array(&envelope);
    payload.extend_from_array(&ledger.to_be_bytes());
    domain_digest(env, RECEIPT_DOMAIN, &payload)
}

/// Digest as `Bytes`, the message type `ed25519_verify` takes.
pub(crate) fn message_bytes(env: &Env, digest: &BytesN<32>) -> Bytes {
    as_bytes(env, digest)
}
