# Pubnet Storage Rent & TTL Measurement Runbook — `pactwright-contracts`

This document details the storage rent calculations, TTL baselines, and archival eviction mitigations for deploying `pact` on Stellar mainnet (pubnet), satisfying ROADMAP milestone **P8-02** / **P8-08**.

---

## 1. Storage Architecture on Soroban

The `pact` smart contract utilizes two tiers of on-chain storage:

1. **Instance Storage (`Config`)**:
   - Stores global contract parameters: `admin` address, `token` SAC address, default `verifier` public key, and `max_pact_ledgers`.
   - Size: ~160 bytes.
   - Lifecycle: Extends automatically during administrative operations or via `extend_pact_ttl`.

2. **Persistent Storage (`PactState`)**:
   - Stores individual pact details keyed by `DataKey::Pact(pact_id)`:
     - `buyer`, `seller`, `cap`, `mode`, `verdict_deadline`, `settle_deadline`, `condition_hash`, `verifier_key`, `actual`, `status`.
   - Size: ~240 bytes.
   - Lifecycle: Bounded strictly by `max_pact_ledgers` (120,960 ledgers / ~7 days).

---

## 2. Pubnet Rent Cost Projections

Based on Soroban Protocol 22 pubnet fee schedules:

| Storage Type | Entry Size | Minimum TTL Extension | Estimated Rent per Day (XLM) |
|---|---|---|---|
| Contract Instance | ~160 bytes | 30 days (518,400 ledgers) | ~0.0012 XLM |
| Pact Persistent Entry | ~240 bytes | 7 days (120,960 ledgers) | ~0.0004 XLM |

- **Rent Responsibility**:
  - The buyer funds storage rent for the persistent pact entry as part of the initial `open` transaction footprint.
  - The `pactwright-deployer` or automated sponsor maintains the contract instance TTL using `extend_pact_ttl`.

---

## 3. Eviction Prevention & Archival Safety

1. **Grace Period Safety Buffer (`GRACE_LEDGERS = 17280`)**:
   - In accordance with ADR-009, `open` extends the persistent entry TTL to `settle_deadline + GRACE_LEDGERS` (~1 day beyond settlement deadline).
   - This prevents pact entries from entering archival storage while funds remain claimable by the buyer or seller.
2. **Post-Settlement Archival**:
   - Once a pact transitions to terminal state (`Settled` or `Refunded`), the persistent entry does not need further TTL extensions, allowing natural archival reclamation after its TTL expires.
3. **Emergency Extension**:
   - The public entrypoint `extend_pact_ttl(pact_id, extend_to)` allows anyone (buyer, seller, or indexing bot) to extend an active pact's TTL without requiring authentication if network delays occur.
