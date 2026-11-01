# Research Report: Re-evaluating M1 (`upto`) vs Accensa ADR-002

This report records the research evaluation of the proposed **M1 (`upto`)** mode in comparison with Accensa's ADR-002 and formalizes the design outcome per **ADR-017** (ROADMAP milestone **P9-06**).

---

## 1. Background: The M1 Proposal

The M1 (`upto`) mode was originally proposed as an alternative to M2 (fixed escrow cap) and M0 (seller exact-refund). Under M1, a buyer would authorize an upstream escrow or account allowance *up to* a specified ceiling, with funds drawn incrementally as consumption occurred without pre-funding the entire cap into contract escrow.

## 2. Evaluation of Accensa ADR-002

Accensa drafted proposal `docs/ADR-002-upto-scheme.md` exploring an `upto` scheme. A technical audit of the specification revealed several structural impediments on Stellar/Soroban:

1. **Sequence Contention & Concurrency**: In Soroban, concurrent transactions drawing against a single variable authorization line frequently collide on account sequence numbers or entry footings, causing high transaction failure rates under concurrent agent workloads.
2. **Unlimited Agent Liability**: Under an un-escrowed `upto` authorization, an agent cannot mathematically guarantee its maximum risk exposure if an API seller streams unexpected calls. By contrast, M2 places strict, unalterable escrow bounds at the moment of `open`.
3. **Double-Spend & Insolvent Buyer Risk**: Without locking funds in escrow, a buyer could deplete their USDC balance between the time the purchase was agreed and the time the seller called `settle`, creating insolvency risk for the seller.
4. **Draft Status in Upstream Accensa**: Accensa's own ADR-002 remains explicitly marked as `Status: DRAFT` ("Do not cite this document as a design that works"), with unanswered architectural questions regarding wire formats and multi-authorization semantics.

## 3. Decision & Architecture Standard (ADR-017)

Following Accensa's draft findings and Pactwright's core architecture principles:

- **Two Modes Only**: Pactwright supports exactly two modes:
  - **M0**: Exact-refund (seller-cooperative, using `RefundVault`).
  - **M2**: Escrow (trustless on-chain escrow where funds wait in the contract until verified delivery).
- **M1 is Dropped**: M1 (`upto`) is formally dropped and excluded from the smart contracts. Variable/metered payments are fully handled within **M2** via `MeterTerms` and signed cumulative usage vouchers (`close_meter`), which maintain strict escrow bounds while supporting fine-grained pay-per-unit consumption.
- **Reopen Trigger**: M1 will only be reconsidered if Soroban consensus introduces native multi-party atomic allowance primitives that eliminate sequence contention and solvency risks, which would require a new formal ADR.
