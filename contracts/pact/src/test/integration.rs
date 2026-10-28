//! `pact` together with the vendored Accensa contracts (ROADMAP C-15).
//!
//! - A settled pact's receipt leaf, anchored in `ReceiptAnchor` v0.2.0, verifies; a tampered
//!   leaf does not.
//! - An M0 refund through `RefundVault` v0.2.0 pays the buyer once per `pact_id`.

use std::vec::Vec;

use soroban_sdk::{
    testutils::Address as _, token::StellarAssetClient, vec, Address, Bytes, BytesN, Env,
};

use super::*;
use crate::{hashing, Verdict};

mod receipt_anchor {
    soroban_sdk::contractimport!(file = "../../vendor/wasm/receipt_anchor.wasm");
}

mod refund_vault {
    soroban_sdk::contractimport!(file = "../../vendor/wasm/refund_vault.wasm");
}

/// Receipt transition codes (ARCHITECTURE §4.3).
const SETTLED: u32 = 2;

fn hash_pair(env: &Env, a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut data = Bytes::from_array(env, lo);
    data.extend_from_array(hi);
    env.crypto().sha256(&data).to_array()
}

/// Root and the proof for `leaves[0]` of a 4-leaf sorted-pair tree.
fn root_and_proof(env: &Env, leaves: &[[u8; 32]; 4]) -> ([u8; 32], Vec<[u8; 32]>) {
    let left = hash_pair(env, &leaves[0], &leaves[1]);
    let right = hash_pair(env, &leaves[2], &leaves[3]);
    let root = hash_pair(env, &left, &right);
    (root, std::vec![leaves[1], right])
}

/// Settles a pact and returns its receipt leaf.
fn settled_leaf(f: &Fixture) -> BytesN<32> {
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    let actual = CAP * 3 / 4;
    f.pact.settle(&pact_id, &actual);
    let pact = f.pact.get_pact(&pact_id);
    hashing::receipt_leaf(
        &f.env,
        &pact_id,
        SETTLED,
        pact.paid_to_seller,
        pact.returned_to_buyer,
        pact.envelope_hash,
        f.now(),
    )
}

fn anchor(f: &Fixture, leaf: &BytesN<32>) -> (receipt_anchor::Client<'static>, u64, Vec<[u8; 32]>) {
    let anchorer = Address::generate(&f.env);
    let id = f.env.register(receipt_anchor::WASM, ());
    let client = receipt_anchor::Client::new(&f.env, &id);
    client.initialize(&anchorer);

    let leaves = [leaf.to_array(), [0xa1; 32], [0xb2; 32], [0xc3; 32]];
    let (root, proof) = root_and_proof(&f.env, &leaves);
    let batch_id = client.anchor_batch(&BytesN::from_array(&f.env, &root), &4, &0, &1);
    (client, batch_id, proof)
}

fn to_proof(env: &Env, proof: &[[u8; 32]]) -> soroban_sdk::Vec<BytesN<32>> {
    let mut out = vec![env];
    for node in proof {
        out.push_back(BytesN::from_array(env, node));
    }
    out
}

#[test]
fn settled_pact_receipt_verifies_in_receipt_anchor() {
    let f = Fixture::new();
    let leaf = settled_leaf(&f);
    let (anchor, batch_id, proof) = anchor(&f, &leaf);

    assert!(anchor.verify_receipt(&batch_id, &leaf, &to_proof(&f.env, &proof)));
}

#[test]
fn tampered_receipt_leaf_fails_verification() {
    let f = Fixture::new();
    let leaf = settled_leaf(&f);
    let (anchor, batch_id, proof) = anchor(&f, &leaf);

    let mut tampered = leaf.to_array();
    tampered[0] ^= 0x01;
    let tampered = BytesN::from_array(&f.env, &tampered);
    assert!(!anchor.verify_receipt(&batch_id, &tampered, &to_proof(&f.env, &proof)));
}

#[test]
fn m0_refund_pays_buyer_once_per_pact_id() {
    let f = Fixture::new();
    // The seller runs a RefundVault holding float for M0 refunds.
    StellarAssetClient::new(&f.env, &f.token.address).mint(&f.seller, &CAP);
    let id = f.env.register(refund_vault::WASM, ());
    let vault = refund_vault::Client::new(&f.env, &id);
    vault.initialize(&f.seller, &f.token.address, &17_280);
    vault.deposit(&f.seller, &CAP);

    // An M0 pact is off-chain; its id is still the deterministic pact_id.
    let params = f.params(1);
    let pact_id = hashing::pact_id(
        &f.env,
        &params.buyer,
        &params.seller,
        params.nonce,
        &params.condition_hash,
    );
    let buyer_before = f.balance(&f.buyer);
    vault.refund(&pact_id, &f.buyer, &(CAP / 2), &f.now());
    assert_eq!(f.balance(&f.buyer), buyer_before + CAP / 2);

    assert_eq!(
        vault.try_refund(&pact_id, &f.buyer, &(CAP / 2), &f.now()),
        Err(Ok(refund_vault::Error::AlreadyRefunded))
    );
}
