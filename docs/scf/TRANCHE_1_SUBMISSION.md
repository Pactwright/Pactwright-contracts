# Stellar Community Fund (SCF) — Tranche 1 Milestone Submission (draft)

> **Draft, not submitted.** Nothing in this file has been sent to the Stellar Community Fund.
> It can be submitted once `L-05` (MVP metrics) is met; see [`LAUNCH_METRICS.md`](../LAUNCH_METRICS.md).

> **Project:** Pactwright — Escrow and settlement infrastructure for AI agentic commerce on Stellar & Soroban  
> **Milestone:** Tranche 1 (MVP Launch on Testnet)  
> **Roadmap Task:** `L-06` (Phase 6 — MVP launch on testnet)  
> **Associated Deliverables:** Contracts MVP, Kit SDK/CLI/MCP, Studio Dashboard, Testnet Deployments

---

## 1. Executive Summary & Milestone Deliverables

Status of the SCF Tranche 1 objectives:
1. **Pact Core Escrow Contract deployed on Stellar Testnet**:
   - Contract ID: `CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6`
   - Verified functionality: M0 exact payment refunds and M2 multi-party escrow with cryptographic verdict evaluation.
2. **ReceiptAnchor Contract deployed on Stellar Testnet**:
   - Contract ID: `CAEO5GBC3ULRC5RYBSCSOXLNXHU2HZAHNW5HY5LZM3DIPFMX357ILNJ2`
   - Merkle root notarization for delivery receipts.
3. **Pactwright-Kit SDK & Tooling**:
   - Developer kit with TypeScript bindings, a Model Context Protocol (MCP) server for agents, and Express/Hono seller middleware. **Not yet published to npm** (`K-17`).
4. **Pactwright Studio**:
   - Live web application hosted on Render (`https://pactwright-studio-testnet.onrender.com`) providing merchant dashboard, pact ledger explorer, and receipt verification.
5. **Phase 6 MVP metrics: not met yet** ([`LAUNCH_METRICS.md`](../LAUNCH_METRICS.md)):
   - 49 testnet pacts (target 100), 0 independent sellers (target 3), real-model verdict accuracy 29/29 (target 90%), `settle` 12,059 stroops (under 1% of 1 USDC).

---

## 2. On-Chain Verification & Evidence Links

- **Pact Contract WASM Hash**: `748b5dbcc800d8cd3bdaf4b24555bfa8e3659cdefb60888791fb9a086b35d1a4` (`deployments/testnet.env`)
- **ReceiptAnchor WASM Hash**: `d7ec929b1f678d1f26f98dd803a566a44502ce7ddb890613612007166061f31e` (`deployments/testnet.env`)
- **Pre-Release Tag**: `v0.1.0-rc.1` across all three repositories (`Pactwright-contracts`, `Pactwright-Kit`, `Pactwright-studio`).
- **Partner letters of support**: none yet; outreach has not been sent ([`partners.md`](../partners.md)).
