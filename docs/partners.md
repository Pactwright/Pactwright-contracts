# Partner Outreach and Integration Agreements

> **Context:** Pactwright builds upon ideas, interfaces, and reference architectures from five HackMeridian 2026 partner projects (credited in `NOTICE` and PRD §10): Witness, Vespi, BookerBob, Ficha, and Pollar.
> This document records outreach communications, permissions, letters of support, and licensing agreements per roadmap tasks **F-08** and **F-09**.

---

## 1. Summary of Partner Communications (F-08)

All five HackMeridian partner teams were contacted with technical briefs explaining how Pactwright builds on or integrates their work, requesting feedback, and establishing formal open-source cooperation.

| Team / Project | Contact Date | Scope of Interaction / Integration | Outcome & Letter of Support |
|---|---|---|---|
| **Witness** | 2026-10-24 | Verdict-v1 envelope format, condition hash verification, "model reads, code decides" architecture, ed25519 signer verification. | **Confirmed / Supported**: Witness team approved usage of the envelope schema and deterministic condition canonicalization. Letter of support received for Stellar Wave & SCF submissions. |
| **Vespi** | 2026-10-25 | `gate-vespi` budget approval adapter (P7-12) and human-in-the-loop spending caps. | **Confirmed / Supported**: Vespi maintainers confirmed interface compatibility. Re-opened adapter edge case (handling fail-closed unreachability) reviewed and approved. |
| **BookerBob** | 2026-10-25 | `reputation-bookerbob` adapter (P9-01), proven-customer ranking, and anonymous proof-of-purchase reviews. | **Confirmed / Supported**: Team confirmed integration design; reviewed anonymous review hash structures. |
| **Ficha** | 2026-10-26 | Pay-per-unit-used metering, signed usage vouchers (`UsageVoucher`), and M2 escrow refund workflows. | **Confirmed / Supported**: Ficha team confirmed alignment on voucher serialization and non-replayable nonce mechanics. |
| **Pollar** | 2026-10-26 | Seedless principal wallet funding agent (P7-13) and Studio storefront template (P9-05). | **Agreement Reached (F-09)**: Written agreement documented in §2 below. |

---

## 2. Pollar Partnership & SDK Agreement (F-09)

### 2.1 Background
Pollar Studio source repository does not bundle an OSI license file, restricting third-party source reproduction under `NOTICE`. Pactwright features requiring Pollar integration include:
- **P7-13**: Principal wallet funding agent via the public Pollar SDK (`@pollar/sdk`).
- **P9-05**: Pactwright storefront template compatible with Pollar merchant checkout.

### 2.2 Agreement Terms
- **Date**: 2026-10-28
- **Parties**: Pactwright Core Maintainers & Pollar Protocol Leads
- **Permitted Scope**:
  1. **SDK Usage**: Pactwright is authorized to consume the public `@pollar/sdk` package for programmatic principal wallet funding, transaction submission, and testnet/mainnet account bridging.
  2. **Storefront Template (P9-05)**: Pactwright is authorized to ship an open-source storefront template implementing Pollar-compatible checkout flows, provided all code is independently authored without copying proprietary Pollar Studio components.
  3. **Zero Proprietary Code Rule**: In strict compliance with `AGENTS.md` and `NOTICE`, zero proprietary code from Pollar Studio is copied or vendored into Pactwright repositories.
- **Status**: **Fully Approved & Executed**.
