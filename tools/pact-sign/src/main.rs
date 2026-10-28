//! Off-chain helper for scripts (ROADMAP C-21). Uses `pactwright_pact::hashing`, so digests are
//! byte-identical to what the contract verifies.
//!
//! ```text
//! pact-sign pact-id <buyer> <seller> <nonce> <condition_hash>
//! pact-sign verdict <pact_id> <condition_hash> <envelope_hash> <allow|deny> <amount> <issued_ledger>
//! pact-sign usage <pact_id> <units>
//! pact-sign pubkey
//! ```
//! Hashes and keys are lowercase hex. Signing commands read the signer's Stellar secret seed
//! (`S…`) from `PACT_SIGN_SECRET`, never from arguments, and print the signature as hex.

use std::{env, process::exit};

use ed25519_dalek::{Signer, SigningKey};
use pactwright_pact::{hashing, UsageVoucher, Verdict, VerdictAttestation};
use soroban_sdk::{Address, BytesN, Env};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn bytes32(env: &Env, value: &str) -> BytesN<32> {
    let raw: Vec<u8> = (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(value.get(i..i + 2).unwrap_or("x"), 16))
        .collect::<Result<_, _>>()
        .unwrap_or_else(|_| fail(&format!("not hex: {value}")));
    let arr: [u8; 32] = raw
        .try_into()
        .unwrap_or_else(|_| fail(&format!("expected 32 bytes: {value}")));
    BytesN::from_array(env, &arr)
}

fn number<T: std::str::FromStr>(value: &str) -> T {
    value
        .parse()
        .unwrap_or_else(|_| fail(&format!("not a number: {value}")))
}

fn signer() -> SigningKey {
    let secret =
        env::var("PACT_SIGN_SECRET").unwrap_or_else(|_| fail("PACT_SIGN_SECRET is not set"));
    let key = stellar_strkey::ed25519::PrivateKey::from_string(secret.trim())
        .unwrap_or_else(|_| fail("PACT_SIGN_SECRET is not a Stellar secret seed"));
    SigningKey::from_bytes(&key.0)
}

fn fail(message: &str) -> ! {
    eprintln!("pact-sign: {message}");
    exit(2)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let a = |i: usize| {
        args.get(i)
            .map(String::as_str)
            .unwrap_or_else(|| fail("missing argument"))
    };
    let env = Env::default();
    match a(0) {
        "pact-id" => {
            let id = hashing::pact_id(
                &env,
                &Address::from_str(&env, a(1)),
                &Address::from_str(&env, a(2)),
                number(a(3)),
                &bytes32(&env, a(4)),
            );
            println!("{}", hex(&id.to_array()));
        }
        "verdict" => {
            let verdict = match a(4) {
                "allow" => Verdict::Allow,
                "deny" => Verdict::Deny,
                other => fail(&format!("verdict must be allow or deny, got {other}")),
            };
            let attestation = VerdictAttestation {
                pact_id: bytes32(&env, a(1)),
                condition_hash: bytes32(&env, a(2)),
                envelope_hash: bytes32(&env, a(3)),
                verdict,
                amount: number(a(5)),
                issued_ledger: number(a(6)),
            };
            let digest = hashing::verdict_message(&env, &attestation).to_array();
            println!("{}", hex(&signer().sign(&digest).to_bytes()));
        }
        "usage" => {
            let voucher = UsageVoucher {
                pact_id: bytes32(&env, a(1)),
                units: number(a(2)),
            };
            let digest = hashing::usage_message(&env, &voucher).to_array();
            println!("{}", hex(&signer().sign(&digest).to_bytes()));
        }
        "pubkey" => println!("{}", hex(&signer().verifying_key().to_bytes())),
        other => fail(&format!("unknown command {other}")),
    }
}
