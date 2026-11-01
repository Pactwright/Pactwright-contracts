# Formal Security Audit Scope — `pactwright-contracts`

This document defines the formal security audit scope, perimeter, security invariants, and auditor selection criteria for the Pactwright Soroban smart contracts, satisfying ROADMAP milestone **P8-01**.

---

## 1. Audit Perimeter

The audit perimeter covers the on-chain Soroban escrow smart contract codebase. Test fixtures, off-chain tooling, and client wrappers are excluded from the core cryptographic perimeter.

### In-Scope Files

| File | Primary Responsibility |
|---|---|
| `contracts/pact/src/lib.rs` | Main contract entrypoints (`open`, `record_verdict`, `settle`, `close_meter`, `release`, `set_verifier`, reads, TTL extension) |
| `contracts/pact/src/storage.rs` | Storage keys (`DataKey`), instance and persistent storage operations, TTL extensions |
| `contracts/pact/src/lifecycle.rs` | Core state transition state machine (Funded, Settled, Refunded, Expired) |
| `contracts/pact/src/verdict.rs` | Ed25519 cryptographic signature verification and attestation parsing |
| `contracts/pact/src/meter.rs` | Usage voucher signature validation and cumulative metered payment calculations |
| `contracts/pact/src/hashing.rs` | Domain-separated SHA-256 hashing for pact IDs, attestations, and condition hashes |
| `contracts/pact/src/types.rs` | Data structures, enums (`PactState`, `Mode`, `Outcome`), and constants |
| `contracts/pact/src/errors.rs` | Fail-closed error codes (`ContractError`) |
| `contracts/pact/src/events.rs` | Soroban contract events (`pact_opened`, `pact_settled`, `pact_refunded`, etc.) |

### Out-of-Scope Files

- `contracts/testutils/`: Test mocks and testing harnesses.
- `tools/pact-sign/`: Developer CLI tool for signing test vouchers.
- `vendor/`: Vendored upstream contract WASMs (`ReceiptAnchor`).
- Repository CI workflows, build scripts, and documentation files.

---

## 2. Security Invariants Under Audit

Auditors must specifically verify that the implementation adheres strictly to the 8 core security invariants:

1. **Zero Admin Backdoors**: The contract admin has no path to withdraw, redirect, or alter held pact funds. Admin permissions are restricted exclusively to rotating the default verifier key for *future* pacts via `set_verifier`.
2. **Fail-Closed Escrow**: Escrow funds are released to a seller if and only if a valid, non-expired cryptographic attestation signed by the pact's pinned verifier key is recorded.
3. **Strict Amount Caps**: The amount paid out to a seller cannot exceed the attested actual amount, nor can it exceed the original escrowed cap. Any remainder is automatically refunded to the buyer.
4. **Single Attestation Execution**: A pact accepts at most one valid verdict. Once settled or refunded, state transitions are terminal and immutable.
5. **Two Independent Deadlines**: A verdict must arrive before `verdict_deadline`. Settlement must occur before `settle_deadline`. If `verdict_deadline` elapses without a verdict, `release` guarantees an exact full refund to the buyer.
6. **Domain Separation**: All hashed cryptographic payloads must use strict unique domain separation prefixes (`pactwright-verdict-v1:`, `pactwright-usage-v1:`).
7. **Storage TTL Bounds**: No pact can be initialized with a TTL exceeding `max_pact_ledgers` (120,960 ledgers / ~7 days), and persistent entries must extend TTL beyond the settlement window plus grace periods.
8. **Integer Overflow Resistance**: All balance calculations, voucher unit multiplications, and currency conversions must use checked integer math (`checked_mul`, `checked_sub`) without risk of truncation or wrapping.

---

## 3. Threat Model & Key Attack Vectors

Auditors should evaluate:
- **Front-running & Replay Attacks**: Ensuring `pact_id` cannot be hijacked or replayed across ledger resets or different token contracts.
- **Cross-Contract Re-entrancy**: Verification that token transfers (`soroban_sdk::token::Client`) occur only after internal state transitions are committed.
- **Resource Exhaustion & Gas Limits**: Profiling CPU and memory limits on `record_verdict` and `settle` to ensure transactions fit comfortably within Soroban pubnet limits.
- **Attestation Forgery**: Verification that ed25519 Dalek verification on Soroban correctly rejects malformed signatures, scalar manipulations, and invalid public keys.

---

## 4. Auditor Selection Criteria

To qualify for conducting the formal audit, candidate audit firms must meet the following criteria:
1. Demonstrated track record auditing production Soroban smart contracts on Stellar.
2. Expertise in Rust memory safety, cryptographic signature verification (Ed25519/Dalek), and DeFi escrow mechanics.
3. Availability to deliver a comprehensive public report including severity classifications, remediation verification, and final sign-off.
