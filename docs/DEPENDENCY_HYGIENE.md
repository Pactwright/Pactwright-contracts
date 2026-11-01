# Dependency Hygiene & Supply-Chain Policy — `pactwright-contracts`

This document details the dependency hygiene procedures, pinned version baselines, and vulnerability auditing controls for `pactwright-contracts`, satisfying ROADMAP milestone **XC-05**.

---

## 1. Pinned Dependencies Baseline

All dependencies in `Cargo.toml` and `Cargo.lock` are strictly pinned and audited:

| Crate | Version | Role | Security Considerations |
|---|---|---|---|
| `soroban-sdk` | `28.0.0` | Core Soroban smart contract framework | Matches Stellar Protocol 22/28 testnet runtime. |
| `soroban-env-host` | `28.0.2` | Soroban host environment for simulation & testing | Verified ledger snapshot runner. |
| `stellar-xdr` | `28.0.0` | XDR encoding and serialization | Validated against protocol definitions. |
| `stellar-strkey` | `1.0.0` | StrKey encoding with automatic private key redaction | Redacts secret keys by default in display logs. |
| `ed25519-dalek` | `3.0.0` | Ed25519 cryptographic signatures | Formally verified curve math; prevents invalid scalar edge cases. |

## 2. Lockfile Freeze & Deterministic Builds

- **`cargo check --locked` Enforcement**: CI and PR Malware Guard reject any pull request where `Cargo.lock` diverges from `Cargo.toml` without explicit maintainer updates.
- **WASM Hash Verification**: All contract builds run via `scripts/build.sh` produces reproducible WASMs (`pactwright_pact.wasm`) verified against recorded SHA-256 digests in `DEPLOYMENTS.md`.

## 3. Vulnerability Auditing & Scanning

- Automated dependabot monitoring continuously checks Rust crates against the RustSec Advisory Database.
- High-severity and critical advisories trigger immediate remediation.
- PR Malware Guard prevents external supply-chain tampering and unauthorized dependency substitutions.
