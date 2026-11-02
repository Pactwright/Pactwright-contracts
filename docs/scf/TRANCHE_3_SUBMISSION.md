# Stellar Community Fund (SCF) — Tranche 3 Milestone Submission (draft)

> **Draft, not submitted.** Nothing in this file has been sent to the Stellar Community Fund.
> It depends on `P8-08` (first M2 pact settled on mainnet). There is no independent audit and no pubnet deployment yet.

> **Project:** Pactwright  
> **Milestone:** Tranche 3 (v1.1 Release — Security Audit, Mainnet Guard & Hardened Ops)  
> **Roadmap Task:** `P8-10` (Phase 8 — v1.1 audit and mainnet)  
> **Associated Deliverables:** Formal audit scope (`docs/AUDIT_SCOPE.md`), Pubnet storage rent & TTL baseline (`docs/PUBNET_RENT_TTL.md`), Mainnet deployment harness

---

## 1. Executive Summary & Delivered Objectives

Pactwright v1.1 targets production readiness for mainnet agent transactions. Status:
1. **Security audit scope** (auditor not yet selected):
   - Audit perimeter covering invariants 1–8, instance storage TTLs, and cryptographic attestation boundaries in `docs/AUDIT_SCOPE.md` (P8-01). An internal pre-audit review is in `docs/AUDIT_REPORT_PRE_PUBNET.md`; its finding H-02 is open.
2. **Pubnet Storage Rent & Archival Protection**:
   - Documented exact Stroop rent allocations and 7-day minimum TTL baselines to prevent archival eviction under continuous operation in `docs/PUBNET_RENT_TTL.md` (P8-06).
3. **M1 Research Resolution**:
   - Evaluated upstream Accensa ADR-002 draft and confirmed formal exclusion of `upto` mode under ADR-017, maintaining fail-closed two-mode security (P9-06).
4. **Supply-Chain & PR Security Guard**:
   - Deployed automated scanners (`scripts/pr-malware-scan.py`) in CI blocking Trojan Source (CVE-2021-42576) and unauthorized binary evaluation.

---

## 2. Mainnet Verification Plan

- Mainnet deployment scripts configured with fail-closed pubnet guards (`scripts/deploy.sh`). Not deployed: `deployments/pubnet.env` is empty (P8-03).
- Auditor findings verification and report publication protocol established in `docs/SECURITY_MODEL.md`.
