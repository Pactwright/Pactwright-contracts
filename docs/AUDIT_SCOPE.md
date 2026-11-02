# Formal Security Audit Scope — `pactwright-contracts`

This document defines the formal security audit scope, perimeter, security invariants, and auditor selection criteria for the Pactwright Soroban smart contracts, satisfying ROADMAP milestone **P8-01**.

---

## 1. Audit Perimeter

The audit perimeter covers the on-chain Soroban escrow smart contract codebase. Test fixtures, off-chain tooling, and client wrappers are excluded from the core cryptographic perimeter.

### In-Scope Files

| File | Primary Responsibility |
|---|---|
| `contracts/pact/src/lib.rs` | Main contract entrypoints (`open`, `record_verdict`, `settle`, `close_meter`, `release`, `set_verifier`, reads, TTL extension) |
| `contracts/pact/src/storage.rs` | Storage keys (`DataKey`), instance and persistent storage operations, TTL extensions (`GRACE_LEDGERS`) |
| `contracts/pact/src/lifecycle.rs` | Core state transition state machine (`Funded`, `Allowed`, `Denied`, `Settled`, `Released`) |
| `contracts/pact/src/verdict.rs` | Ed25519 cryptographic signature verification and attestation parsing |
| `contracts/pact/src/meter.rs` | Usage voucher signature validation and cumulative metered payment calculations |
| `contracts/pact/src/hashing.rs` | Domain-separated SHA-256 hashing for pact IDs, attestations, vouchers, deliveries, and receipts |
| `contracts/pact/src/types.rs` | Data structures, enums (`PactState`, `Verdict`, `Silence`, `Metering`), and constants |
| `contracts/pact/src/errors.rs` | Fail-closed contract error codes (`Error`) |
| `contracts/pact/src/events.rs` | Soroban contract events (`Opened`, `VerdictRecorded`, `Settled`, `MeterClosed`, `Released`, `VerifierSet`) |

### Out-of-Scope Files

- `contracts/testutils/`: Test mocks and testing harnesses.
- `tools/pact-sign/`: Developer CLI tool for signing test vouchers and attestations.
- `vendor/`: Vendored upstream contract WASMs (`ReceiptAnchor`).
- Repository CI workflows, build scripts, and documentation files.

---

## 2. Security Invariants Under Audit

Auditors must specifically verify that the implementation adheres strictly to the 8 core security invariants:

1. **Zero Admin Backdoors**: The contract admin has no path to withdraw, redirect, or alter held pact funds. Admin permissions are restricted exclusively to rotating the default verifier key for *future* pacts via `set_verifier`.
2. **Fail-Closed Escrow**: Escrow funds are released to a seller if and only if a valid, non-expired cryptographic attestation signed by the pact's pinned verifier key is recorded (or signed cumulative usage vouchers on metered pacts).
3. **Strict Amount Caps**: The amount paid out to a seller cannot exceed the attested actual amount, nor can it exceed the original escrowed cap. Any remainder is automatically refunded to the buyer.
4. **Single Attestation Execution**: A pact accepts at most one valid verdict. Once settled (`Settled`) or released to the buyer (`Released`), state transitions are terminal and immutable.
5. **Two Independent Deadlines**: A verdict must arrive before `verdict_deadline`. Settlement must occur before `settle_deadline`. If `verdict_deadline` elapses without a verdict under `Silence::Refund`, or `settle_deadline` elapses without settlement, `release` guarantees an exact full refund to the buyer. Metered pacts can only be released after `settle_deadline`.
6. **Domain Separation**: All hashed cryptographic payloads must use strict unique domain separation prefixes (`pactwright-verdict-v1:`, `pactwright-usage-v1:`, `pactwright-delivery-v1:`, `pactwright-receipt-v1`).
7. **Storage TTL Bounds**: No pact can be initialized with a TTL exceeding `max_pact_ledgers` (120,960 ledgers / ~7 days), and persistent entries must extend TTL beyond the settlement window plus grace periods (`settle_deadline + GRACE_LEDGERS`).
8. **Integer Overflow & Underflow Resistance**: All balance calculations, voucher unit multiplications, and currency splits must use checked or saturating integer math (`checked_mul`, `saturating_add`) and strict precondition bounds checks (`actual <= cap`, `actual <= allowed_amount`) before subtraction (`cap - actual`) without risk of truncation, wrapping, or underflow.

---

## 3. Threat Model & Key Attack Vectors

Auditors should evaluate:
- **Cross-Deployment & Cross-Network Signature Replay**: Assessing domain separation in `hashing.rs` to determine if `pact_id`, `verdict_message`, and `usage_message` should bind the contract address (`env.current_contract_address()`) or network ID to prevent cross-deployment or testnet-to-pubnet replays.
- **Front-running & Replay Attacks**: Ensuring `pact_id` cannot be hijacked or replayed across ledger resets or different token contracts.
- **Meter Key Trust & Metered Path Release**: Evaluating the security model of per-pact `meter_key` held by the buyer, capping vouchers, and ensuring release semantics on metered pacts adhere to `settle_deadline`.
- **Cross-Contract Re-entrancy**: Verification that token transfers (`soroban_sdk::token::Client`) occur only after internal state transitions are committed.
- **Resource Exhaustion & Gas Limits**: Profiling CPU and memory limits on `record_verdict`, `settle`, and `close_meter` to ensure transactions fit comfortably within Soroban pubnet limits.
- **Attestation Forgery**: Verification that ed25519 Dalek verification on Soroban correctly rejects malformed signatures, scalar manipulations, and invalid public keys.

---

## 4. Auditor Selection Criteria

To qualify for conducting the formal audit, candidate audit firms must meet the following criteria:
1. Demonstrated track record auditing production Soroban smart contracts on Stellar or Rust cryptographic protocols.
2. Expertise in Rust memory safety, cryptographic signature verification (Ed25519/Dalek), and DeFi escrow mechanics.
3. Availability to deliver a comprehensive public report including severity classifications, remediation verification, and final sign-off.
