# Stellar Community Fund (SCF) — Tranche 3 Milestone Submission

> **Project:** Pactwright  
> **Milestone:** Tranche 3 (v1.1 Release — Security Audit, Mainnet Guard & Hardened Ops)  
> **Roadmap Task:** `P8-10` (Phase 8 — v1.1 audit and mainnet)  
> **Associated Deliverables:** Formal audit scope (`docs/AUDIT_SCOPE.md`), Pubnet storage rent & TTL baseline (`docs/PUBNET_RENT_TTL.md`), Mainnet deployment harness

---

## 1. Executive Summary & Delivered Objectives

Pactwright v1.1 establishes production readiness for live mainnet agent transactions:
1. **Security Audit Scope & Auditor Selection**:
   - Codified comprehensive audit perimeter covering invariants 1–8, instance storage TTLs, and cryptographic attestation boundaries in `docs/AUDIT_SCOPE.md` (P8-01).
2. **Pubnet Storage Rent & Archival Protection**:
   - Documented exact Stroop rent allocations and 7-day minimum TTL baselines to prevent archival eviction under continuous operation in `docs/PUBNET_RENT_TTL.md` (P8-06).
3. **M1 Research Resolution**:
   - Evaluated upstream Accensa ADR-002 draft and confirmed formal exclusion of `upto` mode under ADR-017, maintaining fail-closed two-mode security (P9-06).
4. **Supply-Chain & PR Security Guard**:
   - Deployed automated scanners (`scripts/pr-malware-scan.py`) in CI blocking Trojan Source (CVE-2021-42576) and unauthorized binary evaluation.

---

## 2. Mainnet Verification Plan

- Mainnet deployment scripts configured with fail-closed pubnet guards (`scripts/deploy.sh`).
- Auditor findings verification and report publication protocol established in `docs/SECURITY_MODEL.md`.
