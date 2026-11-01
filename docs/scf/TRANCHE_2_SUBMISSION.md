# Stellar Community Fund (SCF) — Tranche 2 Milestone Submission

> **Project:** Pactwright  
> **Milestone:** Tranche 2 (v1 Release — Metering, Disputes, Dashboards & Add-ons)  
> **Roadmap Task:** `P7-18` (Phase 7 — v1)  
> **Associated Deliverables:** Metered usage vouchers (`close_meter`), Vespi & Pollar adapters, evidence privacy, KMS seller key isolation

---

## 1. Executive Summary & Delivered Objectives

Pactwright v1 expands core escrow into dynamic real-time agentic resource settlement:
1. **Metered Purchases (`UsageVoucher`)**:
   - Implemented `close_meter` entrypoint verifying non-decreasing units and signed nonces, settling exact consumed compute while releasing excess escrow back to buyer (PRD §6.2, ADR-010).
2. **Partner Adapters**:
   - `gate-vespi` budget approval adapter for human-in-the-loop agent authorization (P7-12).
   - `wallet-pollar` seedless principal funding adapter using `@pollar/sdk` (P7-13).
3. **Evidence Privacy & Compliance**:
   - Configurable redaction hooks, short retention windows, and privacy disclosure guarantees preventing sensitive LLM inference leakage (P7-09).
4. **Seller Key Isolation**:
   - Enterprise KMS signing for refund and delivery attestations via HashiCorp Vault transit secrets engine (P7-08).

---

## 2. Verification Artifacts & Evidence

- **Contract Tests**: 86 unit and property tests passing with zero warnings.
- **Budget Report**: Verified against Soroban resource limits (`#[budget_*]` passing within 1.15x threshold).
- **Merchant & Principal Views**: Integrated in Studio via `002_accounts.sql` migrations.
