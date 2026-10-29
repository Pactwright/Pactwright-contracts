//! Test vectors (ADR-015). This crate is the source of truth for every hash.
//!
//! - `cargo test vectors` checks that `test-vectors/*.json` match the code.
//! - `PACTWRIGHT_WRITE_VECTORS=1 cargo test vectors` (scripts/gen_vectors.sh) rewrites them.
//!
//! `condition_hash.json` is produced by Pactwright-Kit (canonical JSON, Witness
//! `conditionHash()`), so it is not generated here.

use std::{format, fs, string::String, vec, vec::Vec};

use ed25519_dalek::Signer;
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env};

use super::*;
use crate::{hashing, UsageVoucher, Verdict, VerdictAttestation};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn bytes_hex(b: &Bytes) -> String {
    let mut out = vec![0u8; b.len() as usize];
    b.copy_into_slice(&mut out);
    hex(&out)
}

fn g_address(env: &Env, seed: u8) -> (Address, String) {
    let pk = signing_key(seed).verifying_key().to_bytes();
    let strkey = format!("{}", stellar_strkey::ed25519::PublicKey(pk));
    (Address::from_str(env, &strkey), strkey)
}

fn c_address(env: &Env, fill: u8) -> (Address, String) {
    let strkey = format!("{}", stellar_strkey::Contract([fill; 32]));
    (Address::from_str(env, &strkey), strkey)
}

fn b32(env: &Env, fill: u8) -> BytesN<32> {
    BytesN::from_array(env, &[fill; 32])
}

/// Renders one vector file. `cases` are pre-rendered JSON objects.
fn render(formula: &str, notes: &str, cases: Vec<String>) -> String {
    let body = cases
        .iter()
        .map(|c| format!("    {c}"))
        .collect::<Vec<_>>()
        .join(",\n");
    format!(
        "{{\n  \"version\": 1,\n  \"formula\": \"{}\",\n  \"notes\": \"{}\",\n  \"cases\": [\n{}\n  ]\n}}\n",
        formula.replace('"', "\\\""),
        notes.replace('"', "\\\""),
        body
    )
}

fn check_or_write(name: &str, content: String) {
    let path = format!(
        "{}/../../test-vectors/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    if std::env::var("PACTWRIGHT_WRITE_VECTORS").as_deref() == Ok("1") {
        fs::write(&path, &content).unwrap();
        return;
    }
    let committed = fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(
        committed, content,
        "test-vectors/{name}.json is stale; run scripts/gen_vectors.sh"
    );
}

const ENCODING: &str = "xdr(x) = XDR of the value's ScVal; integers big-endian; hex is lowercase";

#[test]
fn nonce_vectors() {
    let env = Env::default();
    let cases = ["", "order-1", "req-7f3a9c"]
        .iter()
        .map(|key| {
            let nonce = hashing::nonce_from_key(&env, &Bytes::from_slice(&env, key.as_bytes()));
            let escaped = format!("{key:?}").replace('"', "\\\"");
            format!(
                "{{\"name\": \"key {escaped}\", \"input\": {{\"idempotency_key\": \"{key}\"}}, \"output\": \"{nonce}\"}}"
            )
        })
        .collect();
    check_or_write(
        "nonce",
        render(
            "u64_be(sha256(utf8(idempotency_key))[0..8])",
            "output is the u64 as a decimal string",
            cases,
        ),
    );
}

#[test]
fn pact_id_vectors() {
    let env = Env::default();
    let (g1, g1s) = g_address(&env, 1);
    let (g2, g2s) = g_address(&env, 2);
    let (c3, c3s) = c_address(&env, 3);
    let inputs = [
        (
            "G buyer, G seller",
            (g1.clone(), g1s.clone()),
            (g2, g2s),
            0u64,
            0x11u8,
        ),
        ("G buyer, C seller", (g1, g1s), (c3, c3s), u64::MAX, 0xabu8),
    ];
    let cases = inputs
        .iter()
        .map(|(name, (buyer, buyer_s), (seller, seller_s), nonce, fill)| {
            let condition = b32(&env, *fill);
            let preimage = hashing::pact_id_preimage(&env, buyer, seller, *nonce, &condition);
            let id = hashing::pact_id(&env, buyer, seller, *nonce, &condition);
            format!(
                "{{\"name\": \"{name}\", \"input\": {{\"buyer\": \"{buyer_s}\", \"seller\": \"{seller_s}\", \"nonce\": \"{nonce}\", \"condition_hash\": \"{}\"}}, \"preimage\": \"{}\", \"output\": \"{}\"}}",
                hex(&condition.to_array()),
                bytes_hex(&preimage),
                hex(&id.to_array())
            )
        })
        .collect();
    check_or_write(
        "pact_id",
        render(
            "sha256(xdr(buyer) || xdr(seller) || nonce_u64_be || condition_hash)",
            ENCODING,
            cases,
        ),
    );
}

#[test]
fn verdict_attestation_vectors() {
    let env = Env::default();
    let key = signing_key(7);
    let cases = [(Verdict::Allow, 7_500_000i128), (Verdict::Deny, 0i128)]
        .iter()
        .map(|(verdict, amount)| {
            let attestation = VerdictAttestation {
                pact_id: b32(&env, 0x01),
                condition_hash: b32(&env, 0x02),
                envelope_hash: b32(&env, 0x03),
                verdict: *verdict,
                amount: *amount,
                issued_ledger: 123_456,
            };
            let digest = hashing::verdict_message(&env, &attestation).to_array();
            let signature = key.sign(&digest).to_bytes();
            format!(
                "{{\"name\": \"{verdict:?}\", \"input\": {{\"pact_id\": \"{}\", \"condition_hash\": \"{}\", \"envelope_hash\": \"{}\", \"verdict\": \"{verdict:?}\", \"amount\": \"{amount}\", \"issued_ledger\": 123456}}, \"xdr\": \"{}\", \"output\": \"{}\", \"signer_seed\": \"{}\", \"signer_public_key\": \"{}\", \"signature\": \"{}\"}}",
                hex(&[0x01; 32]),
                hex(&[0x02; 32]),
                hex(&[0x03; 32]),
                bytes_hex(&attestation.clone().to_xdr(&env)),
                hex(&digest),
                hex(&[7u8; 32]),
                hex(&key.verifying_key().to_bytes()),
                hex(&signature)
            )
        })
        .collect();
    check_or_write(
        "verdict_attestation",
        render(
            "sha256(\"pactwright-verdict-v1:\" || xdr(VerdictAttestation)); ed25519 signs the 32-byte digest",
            ENCODING,
            cases,
        ),
    );
}

#[test]
fn usage_voucher_vectors() {
    let env = Env::default();
    let key = signing_key(9);
    let cases = [0u32, 12, u32::MAX]
        .iter()
        .map(|units| {
            let voucher = UsageVoucher {
                pact_id: b32(&env, 0x01),
                units: *units,
            };
            let digest = hashing::usage_message(&env, &voucher).to_array();
            let signature = key.sign(&digest).to_bytes();
            format!(
                "{{\"name\": \"units {units}\", \"input\": {{\"pact_id\": \"{}\", \"units\": {units}}}, \"xdr\": \"{}\", \"output\": \"{}\", \"signer_seed\": \"{}\", \"signer_public_key\": \"{}\", \"signature\": \"{}\"}}",
                hex(&[0x01; 32]),
                bytes_hex(&voucher.clone().to_xdr(&env)),
                hex(&digest),
                hex(&[9u8; 32]),
                hex(&key.verifying_key().to_bytes()),
                hex(&signature)
            )
        })
        .collect();
    check_or_write(
        "usage_voucher",
        render(
            "sha256(\"pactwright-usage-v1:\" || xdr(UsageVoucher)); ed25519 signs the 32-byte digest",
            ENCODING,
            cases,
        ),
    );
}

#[test]
fn delivery_receipt_vectors() {
    let env = Env::default();
    let body = b"{\"translated\":\"ola mundo\"}";
    let body_sha256 = env
        .crypto()
        .sha256(&Bytes::from_slice(&env, body))
        .to_bytes();
    let pact_id = b32(&env, 0x01);
    let digest = hashing::delivery_message(&env, &pact_id, &body_sha256).to_array();
    let signature = signing_key(5).sign(&digest).to_bytes();
    let case = format!(
        "{{\"name\": \"json body\", \"input\": {{\"pact_id\": \"{}\", \"body_utf8\": \"{}\", \"body_sha256\": \"{}\"}}, \"output\": \"{}\", \"signer_seed\": \"{}\", \"signer_public_key\": \"{}\", \"signature\": \"{}\"}}",
        hex(&[0x01; 32]),
        std::str::from_utf8(body).unwrap().replace('"', "\\\""),
        hex(&body_sha256.to_array()),
        hex(&digest),
        hex(&[5u8; 32]),
        hex(&signing_key(5).verifying_key().to_bytes()),
        hex(&signature)
    );
    check_or_write(
        "delivery_receipt",
        render(
            "sha256(\"pactwright-delivery-v1:\" || pact_id || sha256(body)); header X-Pactwright-Delivery: sha256=<body hash>; sig=<base64 signature>",
            ENCODING,
            vec![case],
        ),
    );
}

/// (name, transition, to_seller, to_buyer, envelope fill byte, ledger)
type LeafInput = (&'static str, u32, i128, i128, Option<u8>, u32);

#[test]
fn receipt_leaf_vectors() {
    let env = Env::default();
    let inputs: [LeafInput; 4] = [
        ("paid (M0)", 1, 10_000_000, 0, None, 500),
        ("settled, partial", 2, 7_500_000, 2_500_000, Some(0x03), 600),
        ("released", 3, 0, 10_000_000, Some(0x04), 700),
        ("refunded", 4, 0, 10_000_000, None, 800),
    ];
    let cases = inputs
        .iter()
        .map(|(name, transition, to_seller, to_buyer, envelope, ledger)| {
            let envelope_hash = envelope.map(|f| b32(&env, f));
            let leaf = hashing::receipt_leaf(
                &env,
                &b32(&env, 0x01),
                *transition,
                *to_seller,
                *to_buyer,
                envelope_hash,
                *ledger,
            );
            let envelope_json = match envelope {
                Some(f) => format!("\"{}\"", hex(&[*f; 32])),
                None => String::from("null"),
            };
            format!(
                "{{\"name\": \"{name}\", \"input\": {{\"pact_id\": \"{}\", \"transition\": {transition}, \"to_seller\": \"{to_seller}\", \"to_buyer\": \"{to_buyer}\", \"envelope_hash\": {envelope_json}, \"ledger\": {ledger}}}, \"output\": \"{}\"}}",
                hex(&[0x01; 32]),
                hex(&leaf.to_array())
            )
        })
        .collect();
    check_or_write(
        "receipt_leaf",
        render(
            "sha256(\"pactwright-receipt-v1\" || pact_id || transition_u32 || to_seller_i128 || to_buyer_i128 || (envelope_hash or 32 zero bytes) || ledger_u32)",
            "transitions: 1 paid, 2 settled, 3 released, 4 refunded; integers big-endian (i128 = 16 bytes, two's complement)",
            cases,
        ),
    );
}

fn pair(env: &Env, a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut data = Bytes::from_array(env, lo);
    data.extend_from_array(hi);
    env.crypto().sha256(&data).to_array()
}

#[test]
fn merkle_vectors() {
    let env = Env::default();
    let leaves: Vec<[u8; 32]> = (1u8..=4).map(|f| [f; 32]).collect();
    let n12 = pair(&env, &leaves[0], &leaves[1]);
    let n34 = pair(&env, &leaves[2], &leaves[3]);
    let root = pair(&env, &n12, &n34);
    let pair_case = format!(
        "{{\"name\": \"pair (order-independent)\", \"input\": {{\"a\": \"{}\", \"b\": \"{}\"}}, \"output\": \"{}\"}}",
        hex(&leaves[1]),
        hex(&leaves[0]),
        hex(&n12)
    );
    let tree_case = format!(
        "{{\"name\": \"4-leaf tree\", \"input\": {{\"leaves\": [\"{}\", \"{}\", \"{}\", \"{}\"]}}, \"output\": \"{}\", \"proof_for_leaf_0\": [\"{}\", \"{}\"]}}",
        hex(&leaves[0]),
        hex(&leaves[1]),
        hex(&leaves[2]),
        hex(&leaves[3]),
        hex(&root),
        hex(&leaves[1]),
        hex(&n34)
    );
    check_or_write(
        "merkle",
        render(
            "node = sha256(min(a, b) || max(a, b)), bytewise comparison (Accensa ReceiptAnchor ADR-001)",
            "odd-count trees: rule to be confirmed against Accensa's off-chain SDK in ROADMAP K-04",
            vec![pair_case, tree_case],
        ),
    );
}
