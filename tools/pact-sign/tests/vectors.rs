//! pact-sign must reproduce test-vectors/ byte for byte (ADR-015).

use std::process::Command;

fn vectors(name: &str) -> Vec<serde_like::Case> {
    serde_like::cases(
        &std::fs::read_to_string(format!(
            "{}/../../test-vectors/{name}.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap(),
    )
}

/// Seed strkey (`S…`) for a raw 32-byte seed, as the vectors' signers use `[seed; 32]`.
fn secret(seed: u8) -> String {
    stellar_strkey::ed25519::PrivateKey([seed; 32])
        .to_string()
        .as_str()
        .to_owned()
}

fn run(args: &[&str], secret: Option<String>) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pact-sign"));
    cmd.args(args);
    if let Some(s) = secret {
        cmd.env("PACT_SIGN_SECRET", s);
    }
    let out = cmd.output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

#[test]
fn pact_id_matches_vectors() {
    for c in vectors("pact_id") {
        let out = run(
            &[
                "pact-id",
                &c.input("buyer"),
                &c.input("seller"),
                &c.input("nonce"),
                &c.input("condition_hash"),
            ],
            None,
        );
        assert_eq!(out, c.get("output"), "{}", c.get("name"));
    }
}

#[test]
fn verdict_signatures_match_vectors() {
    for c in vectors("verdict_attestation") {
        let verdict = c.input("verdict").to_lowercase();
        let out = run(
            &[
                "verdict",
                &c.input("pact_id"),
                &c.input("condition_hash"),
                &c.input("envelope_hash"),
                &verdict,
                &c.input("amount"),
                &c.input("issued_ledger"),
            ],
            Some(secret(7)),
        );
        assert_eq!(out, c.get("signature"), "{}", c.get("name"));
    }
}

#[test]
fn usage_signatures_match_vectors() {
    for c in vectors("usage_voucher") {
        let out = run(
            &["usage", &c.input("pact_id"), &c.input("units")],
            Some(secret(9)),
        );
        assert_eq!(out, c.get("signature"), "{}", c.get("name"));
    }
}

#[test]
fn pubkey_matches_vector_signer() {
    let c = &vectors("verdict_attestation")[0];
    assert_eq!(
        run(&["pubkey"], Some(secret(7))),
        c.get("signer_public_key")
    );
}

/// Minimal reader for the vector files (one case object per line, flat string/number fields),
/// so the tool needs no JSON dependency.
mod serde_like {
    pub struct Case(String);

    pub fn cases(file: &str) -> Vec<Case> {
        file.lines()
            .map(str::trim)
            .filter(|l| l.starts_with("{\"name\""))
            .map(|l| Case(l.to_string()))
            .collect()
    }

    impl Case {
        fn field(text: &str, key: &str) -> String {
            let needle = format!("\"{key}\": ");
            let start = text
                .find(&needle)
                .unwrap_or_else(|| panic!("missing {key}"))
                + needle.len();
            let rest = &text[start..];
            if let Some(quoted) = rest.strip_prefix('"') {
                quoted[..quoted.find('"').unwrap()].to_string()
            } else {
                rest[..rest.find([',', '}']).unwrap()].trim().to_string()
            }
        }
        pub fn get(&self, key: &str) -> String {
            Self::field(&self.0, key)
        }
        pub fn input(&self, key: &str) -> String {
            let start = self.0.find("\"input\": {").unwrap();
            let end = start + self.0[start..].find('}').unwrap();
            Self::field(&self.0[start..=end], key)
        }
    }
}
