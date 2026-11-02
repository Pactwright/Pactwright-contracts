# Internal Pre-Audit Review: `contracts/pact`

> **This is not an independent audit.** It is a review by the Pactwright maintainers, written to
> prepare for one. No audit firm has been selected or engaged yet (`P8-01`), so `P8-02` ("fix the
> audit findings; publish the report") stays open until an independent audit's findings are fixed
> and its report is published. **H-02 is unresolved and blocks the pubnet deploy (`P8-03`).**

**Target:** `Pactwright/Pactwright-contracts` (`contracts/pact` at commit `c99cf0c`)  
**Review method:** Line-by-line manual review of the pact state machine, Soroban TTL & state archival rules, cryptographic key trust models, and cross-language byte-for-byte hashing parity (`hashing.rs` ↔ `tools/pact-sign` ↔ `hashing.ts`).  
**Scope:** ~610 lines of Soroban Rust (`lifecycle.rs`, `verdict.rs`, `meter.rs`, `hashing.rs`, `storage.rs`, `admin.rs`, `types.rs`, `errors.rs`, `events.rs`, `lib.rs`), deployment configuration, and off-chain hashing implementations.  
**Roadmap:** input to **P8-01** (audit scope) and **P8-02** (fix findings); neither is closed by this review.

---

## 📊 Findings Summary

| ID | Title | Severity | Status |
|---|---|---|---|
| **H-01** | `Silence::Pay` Deadline Collision: Seller Permanently Locked Out When `verdict_deadline == settle_deadline` | **High** | **Resolved** (enforced `verdict_deadline < settle_deadline`) |
| **H-02** | Missing Contract Address and Network Passphrase in Domain Hashing (Cross-Deployment & Cross-Network Replay) | **High** | **Open**: must be fixed before the pubnet deploy (P8-03) |
| **M-01** | Unvalidated `__constructor` Parameters (`max_pact_ledgers = 0` Permanently Bricks Contract Instance) | **Medium** | **Resolved** (constructor validation added) |
| **M-02** | Metered Escrow Capital Lockup: Buyer Has No Early Exit If Seller Fails to Deliver or Disappears | **Medium** | Documented Operational Constraint |
| **L-01** | Inconsistent Trailing Colon in Domain Separation Constants (`RECEIPT_DOMAIN`) | **Low** | Acknowledged Wire Compatibility |
| **I-01** | `ed25519_verify` Host Trap Deprecates `Error::BadSignature` | **Informational** | Documented Soroban SDK Behavior |

---

## 🔍 Detailed Vulnerability Findings & Remediations

### [H-01] `Silence::Pay` Deadline Collision: Seller Permanently Locked Out When `verdict_deadline == settle_deadline`
**Severity:** **High**  
**Affected Files:** `contracts/pact/src/lifecycle.rs`

#### 1. Vulnerability Mechanics
In `open()`, the deadline validation previously permitted `verdict_deadline == settle_deadline`:
```rust
// lifecycle.rs:43
if now >= params.verdict_deadline
    || params.verdict_deadline > params.settle_deadline
    || params.settle_deadline > latest_settle
{
    return Err(Error::BadDeadlines);
}
```
When a pact was configured with `silence: Silence::Pay`, `settle()` enforced:
```rust
let now = env.ledger().sequence();
if now > pact.settle_deadline {
    return Err(Error::SettleWindowClosed);
}
match pact.state {
    PactState::Allowed => {}
    PactState::Funded if pact.silence == Silence::Pay && now > pact.verdict_deadline => {
        pact.allowed_amount = pact.cap;
    }
    _ => return Err(Error::InvalidState),
}
```
If `verdict_deadline == settle_deadline`:
1. When `now <= verdict_deadline`: The condition `now > pact.verdict_deadline` is **false**. `settle()` returns `Err(Error::InvalidState)`.
2. When `now > verdict_deadline`: The check `now > pact.settle_deadline` is **true** because `settle_deadline == verdict_deadline`. `settle()` returns `Err(Error::SettleWindowClosed)`.

**Result:** There was zero window where `now > verdict_deadline` and `now <= settle_deadline`. The seller could never settle under `Silence::Pay`. Once `now > settle_deadline`, the buyer could call `release()` and retrieve a 100% refund, cheating the seller out of the agreed silence payment.

#### 2. Resolution
In `contracts/pact/src/lifecycle.rs`, the deadline check was hardened from `>` to `>=`:
```rust
if now >= params.verdict_deadline
    || params.verdict_deadline >= params.settle_deadline
    || params.settle_deadline > latest_settle
{
    return Err(Error::BadDeadlines);
}
```
Regression tests `open_rejects_bad_deadlines` in `test/lifecycle.rs` verify that `verdict_deadline == settle_deadline` is rejected with `Error::BadDeadlines`.

---

### [H-02] Missing Contract Address and Network Passphrase in Domain Hashing (Replay Attack)
**Severity:** **High**  
**Affected Files:** `contracts/pact/src/hashing.rs`

#### 1. Vulnerability Mechanics
In `hashing.rs`, `pact_id`, `verdict_message`, and `usage_message` are constructed as follows:
```rust
// pact_id preimage:
sha256(xdr(buyer) ‖ xdr(seller) ‖ nonce_u64_be ‖ condition_hash)

// verdict_message preimage:
sha256("pactwright-verdict-v1:" ‖ xdr(attestation))

// usage_message preimage:
sha256("pactwright-usage-v1:" ‖ xdr(voucher))
```
Neither the **contract address** (`env.current_contract_address()`) nor the **network passphrase/chain ID** is incorporated into the preimages.

#### 2. Impact & Exploitation Scenario
If an agent purchases a service on testnet with buyer Alice and seller Bob using nonce $N$ and condition $H$, and the verifier signs a `DENY` or partial `ALLOW` attestation, an adversary could replay that attestation on pubnet if the same parameters are used. Similarly, signatures can be replayed across multiple deployed instances.

#### 3. Resolution Plan (Scheduled for Milestone P8-03)
Before deploying to pubnet, incorporate `contract` address into `pact_id_preimage` across Rust `hashing.rs` and TypeScript `hashing.ts`.

---

### [M-01] Unvalidated `__constructor` Parameters (`max_pact_ledgers = 0` Permanently Bricks Contract)
**Severity:** **Medium**  
**Affected Files:** `contracts/pact/src/admin.rs`, `contracts/pact/src/lib.rs`

#### 1. Vulnerability Mechanics
`admin::init` previously accepted `max_pact_ledgers` without checks. If deployed with `max_pact_ledgers = 0`, then in `open()`:
`latest_settle = now.saturating_add(0) = now`  
The condition `now < verdict_deadline < settle_deadline <= now` becomes impossible to satisfy. Every single call to `open()` would revert with `Error::BadDeadlines`. Because `max_pact_ledgers` cannot be updated, the deployed contract was permanently bricked.

#### 2. Resolution
In `contracts/pact/src/admin.rs`, `init()` now validates:
```rust
if max_pact_ledgers == 0 {
    soroban_sdk::panic_with_error!(env, Error::BadDeadlines);
}
```
Regression test `constructor_rejects_zero_max_pact_ledgers` verifies that initializing with `0` panics with `Error::BadDeadlines`.

---

### [M-02] Metered Escrow Capital Lockup: Buyer Has No Early Exit
**Severity:** **Medium**  
**Affected Files:** `contracts/pact/src/lifecycle.rs`

#### 1. Vulnerability Mechanics
In `lifecycle::release()`, metered pacts disable silence refunds (`!metered`). The only way for a buyer to release escrowed funds without seller cooperation is when `now > pact.settle_deadline`. If an agent opens a metered pact with a 7-day `settle_deadline` and the seller server crashes or goes offline after 1 minute, the buyer's unspent funds remain frozen in the contract for the full 7 days.

#### 2. Operational Recommendation
Agents should negotiate short `settle_deadline` windows on metered pacts, or sellers should provide automated zero-unit settlement endpoints upon mutual cancellation.

---

### [L-01] Inconsistent Trailing Colon in Domain Separation Constants (`RECEIPT_DOMAIN`)
**Severity:** **Low**  
**Affected Files:** `contracts/pact/src/hashing.rs`, `Pactwright-Kit/packages/core/src/hashing.ts`

`VERDICT_DOMAIN`, `USAGE_DOMAIN`, and `DELIVERY_DOMAIN` end with `:`, whereas `RECEIPT_DOMAIN` is `b"pactwright-receipt-v1"`. While both Rust and TypeScript match, the omission of the colon is a minor formatting inconsistency. Kept intact to preserve test vector compatibility across existing tooling.

---

### [I-01] `ed25519_verify` Host Trap Deprecates `Error::BadSignature`
**Severity:** **Informational**  
**Affected Files:** `contracts/pact/src/verdict.rs`, `contracts/pact/src/errors.rs`

Soroban SDK's `env.crypto().ed25519_verify()` aborts the host environment with a VM trap if a signature is invalid rather than returning a `Result`. `Error::BadSignature = 12` is kept reserved and documented in `errors.rs`.

---

## 🔬 Subsystem Verification

1. **State Machine Invariants**: Verified that terminal states (`Settled`, `Released`) permanently block re-entrancy and double-settlement.
2. **TTL & Storage**: Verified that `open()` extends persistent storage to `(settle_deadline - now) + GRACE_LEDGERS`, and `bump_instance` extends instance storage on all state mutations.
3. **Byte Parity**: All 7 JSON test vectors pass across Rust contracts and TypeScript SDK.
