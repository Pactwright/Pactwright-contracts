<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="brand/pactwright-logo-dark.svg">
    <img alt="Pactwright" src="brand/pactwright-logo-light.svg" width="460">
  </picture>
</p>

<p align="center">
  <b>Escrow for AI-agent purchases on Stellar.</b><br>
  The buyer's money moves only when delivery is proven. Otherwise it comes back.
</p>

<p align="center">
  <a href="https://github.com/Pactwright/Pactwright-contracts/actions/workflows/ci.yml"><img src="https://img.shields.io/badge/tests-90_passing-2DD4BF?style=for-the-badge&labelColor=0B1220" alt="90 tests passing"></a>
  <a href="DEPLOYMENTS.md"><img src="https://img.shields.io/badge/testnet-deployed-2DD4BF?style=for-the-badge&labelColor=0B1220" alt="Deployed on Stellar testnet"></a>
  <a href="https://github.com/Pactwright/Pactwright-contracts/releases/tag/v0.1.0-rc.1"><img src="https://img.shields.io/badge/release-v0.1.0--rc.1-6366F1?style=for-the-badge&labelColor=0B1220" alt="Release v0.1.0-rc.1"></a>
  <a href="docs/BENCHMARKS.md"><img src="https://img.shields.io/badge/cost-%E2%89%A42.1%25_of_tx_CPU-6366F1?style=for-the-badge&labelColor=0B1220" alt="Every call at most 2.1% of the transaction CPU limit"></a>
  <a href="docs/SECURITY_MODEL.md"><img src="https://img.shields.io/badge/audit-not_yet-F59E0B?style=for-the-badge&labelColor=0B1220" alt="Not audited yet"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT-94A3B8?style=for-the-badge&labelColor=0B1220" alt="MIT license"></a>
</p>

<p align="center">
  <a href="https://pactwright.github.io/docs/"><b>Documentation</b></a> ·
  <a href="#in-60-seconds-for-reviewers"><b>In 60 seconds</b></a> ·
  <a href="#a-purchase-step-by-step"><b>A purchase, step by step</b></a> ·
  <a href="#verify-it-yourself"><b>Verify it yourself</b></a> ·
  <a href="#on-chain-evidence"><b>On-chain evidence</b></a> ·
  <a href="#license"><b>License</b></a> ·
  <a href="https://github.com/Pactwright/Pactwright-Kit">SDK</a> ·
  <a href="https://github.com/Pactwright/Pactwright-studio">Studio</a>
</p>

<p align="center">
  <a href="https://scorecard.dev/viewer/?uri=github.com/Pactwright/Pactwright-contracts"><img src="https://api.scorecard.dev/projects/github.com/Pactwright/Pactwright-contracts/badge" alt="OpenSSF Scorecard"></a>
</p>

> [!IMPORTANT]
> **Testnet only, not audited.** The `pact` contract is live on Stellar testnet and every number
> below links to a real transaction. Do not use it with real funds until the audit (ROADMAP P8-01).

| | |
|---|---|
| `pact` contract (testnet) | [`CDUXJJUN…MFEV6`](https://stellar.expert/explorer/testnet/contract/CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6) |
| Its WASM hash | `748b5dbc…5d1a4`, identical in the [release](https://github.com/Pactwright/Pactwright-contracts/releases/tag/v0.1.0-rc.1), on testnet and in a local build |
| Receipt anchor (testnet) | [`CAEO5GBC…LNJ2`](https://stellar.expert/explorer/testnet/contract/CAEO5GBC3ULRC5RYBSCSOXLNXHU2HZAHNW5HY5LZM3DIPFMX357ILNJ2), Accensa `ReceiptAnchor` v0.2.0 |
| Payment token | USDC ([`CBIELTK6…DAMA`](https://stellar.expert/explorer/testnet/contract/CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA), Circle testnet) |
| Measured cost | [docs/BENCHMARKS.md](docs/BENCHMARKS.md): local and real-transaction numbers per call |
| Trust model | [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md) |
| Documentation | [pactwright.github.io/docs](https://pactwright.github.io/docs/), including the [contract API reference](https://pactwright.github.io/docs/reference/contract/) |
| Design | [PRD](docs/PRD.md) · [Architecture](docs/ARCHITECTURE.md) · [15 ADRs](docs/adr/) |
| License | [MIT](#license), with every third-party license listed |

## Contents

1. [In 60 seconds, for reviewers](#in-60-seconds-for-reviewers)
2. [Why this exists](#why-this-exists)
3. [Key ideas in plain language](#key-ideas-in-plain-language)
4. [A purchase, step by step](#a-purchase-step-by-step)
5. [How a pact moves through its life](#how-a-pact-moves-through-its-life)
6. [Two protection modes](#two-protection-modes)
7. [Who does what, and who you have to trust](#who-does-what-and-who-you-have-to-trust)
8. [When things go wrong](#when-things-go-wrong)
9. [What the contract does today](#what-the-contract-does-today)
10. [Contract API](#contract-api)
11. [Error codes](#error-codes)
12. [Verify it yourself](#verify-it-yourself)
13. [On-chain evidence](#on-chain-evidence)
14. [Stellar, specifically](#stellar-specifically)
15. [What it costs](#what-it-costs)
16. [Security and known limits](#security-and-known-limits)
17. [Clone, build and deploy](#clone-build-and-deploy)
18. [Layout](#layout)
19. [Project status](#project-status)
20. [FAQ](#faq)
21. [Built on](#built-on)
22. [Contributing](#contributing)
23. [License](#license)

## In 60 seconds, for reviewers

- **What it is.** A Soroban escrow for purchases an AI agent makes. The buyer locks a cap. A verifier's signed verdict, or for metered services the buyer's own signed usage, decides how much the seller gets. The rest goes back to the buyer.
- **What runs today.** The contract on testnet, a release with a reproducible WASM, and 90 tests in CI (86 contract tests, 4 tool tests). Those include fuzzing, budget tests and integration with the real Accensa contracts, plus a benchmark run as real testnet transactions.
- **What makes it safe.** No admin can move funds. Two deadlines and a silence policy mean a missing verifier or seller can never strand the buyer's money. Each verdict is bound to one pact and one condition hash.
- **Check one claim yourself.** `scripts/build.sh` prints the WASM hash. Compare it with the [release asset](https://github.com/Pactwright/Pactwright-contracts/releases/tag/v0.1.0-rc.1) and the [deployed contract](https://stellar.expert/explorer/testnet/contract/CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6): they are the same bytes.

## Why this exists

AI agents now buy things on their own: an API call, a translated document, a dataset, minutes
of compute. On Stellar they can already pay with [x402](https://developers.stellar.org/docs/build/agentic-payments/x402).
The seller answers `402 Payment Required`, the agent pays in USDC and retries.

That covers *paying*. It doesn't cover what happens when the purchase goes wrong:

| Without Pactwright | With a pact |
|---|---|
| Money moves before anything is delivered | The cap sits in escrow until a verdict or signed usage |
| "Delivered" is whatever the seller's server says | A verifier checks the delivery against a condition frozen before payment |
| A wrong delivery needs the seller's goodwill to refund | DENY returns the escrow; no one's permission needed |
| A silent seller or verifier can hold funds forever | Deadlines and an agreed silence policy release funds automatically |
| Pay-per-minute services charge up front | The buyer signs cumulative usage; the seller is paid for exactly that |
| Receipts live on the seller's server | Every outcome is anchored on-chain and verifiable by anyone |

An agent has no one to call when a purchase goes wrong, so the rules for getting money back have
to be in the payment itself. That is what a pact is.

This repository is the on-chain half: the `pact` contract. The SDK, seller middleware and
verifier live in [Pactwright-Kit](https://github.com/Pactwright/Pactwright-Kit). Pact pages and
receipt anchoring live in [Pactwright-studio](https://github.com/Pactwright/Pactwright-studio).

## Key ideas in plain language

| Term | Meaning |
|---|---|
| **Pact** | One purchase held in escrow. Its `pact_id` is derived from the buyer, the seller, a nonce and the condition, so no one else can claim it. |
| **Cap** | The most the buyer can pay. It is locked in the contract when the pact opens. |
| **Condition** | What "delivered correctly" means, written down before payment (for example "a Portuguese translation of at least 4 pages"). Only its hash goes on-chain, so it can never change later. |
| **Verifier** | A service that checks the delivery against the condition and signs a verdict. Its key is fixed in the pact when the pact opens. |
| **Verdict / attestation** | The verifier's signed decision: ALLOW with an amount (which may be less than the cap, for partial delivery) or DENY. |
| **Settle** | The seller collects the allowed amount. The rest of the cap goes back to the buyer in the same transaction. |
| **Release** | The whole cap goes back to the buyer: after a DENY, after a silent verifier, or after the seller missed the settle deadline. |
| **Deadlines** | `verdict_deadline` is the last ledger for a verdict; `settle_deadline` is the last ledger for the seller to collect. A ledger closes about every 5 seconds. |
| **Silence policy** | Agreed at open. If no verdict arrives by the verdict deadline, either refund the buyer (`Refund`, the default) or let the seller collect (`Pay`). |
| **Metered pact** | Pay per unit (minutes, pages, calls). The buyer signs a running total of units used, and the seller collects for that total. No verifier is involved. |
| **Receipt** | A hash of each final outcome. Receipts are batched into a Merkle tree and anchored on-chain, so anyone can prove what happened. |

Amounts are in stroops of the payment token. USDC has 7 decimals, so `10_000_000` = 1 USDC.

## A purchase, step by step

An agent buys a document translation for at most **1 USDC**. The seller's price depends on the
number of pages actually delivered.

```mermaid
sequenceDiagram
    autonumber
    participant A as Agent (buyer)
    participant S as Seller API
    participant V as Verifier
    participant P as pact contract
    A->>S: request the translation
    S-->>A: 402 + terms: condition, verifier, deadlines
    A->>P: open: lock 1 USDC under the frozen condition hash
    S-->>A: the document + a signed delivery receipt
    A->>V: submit the document and the receipt as evidence
    V->>P: record_verdict: ALLOW, 0.75 USDC (3 of 4 pages)
    S->>P: settle 0.75 USDC
    P-->>S: 0.75 USDC
    P-->>A: 0.25 USDC back
```

1. **Terms.** The seller's `402` response names the condition, the verifier and the deadlines. The agent can read them before paying anything.
2. **Open.** The agent calls `open`. The contract checks the cap, the deadlines and that buyer ≠ seller. It moves 1 USDC into escrow and stores the verifier key that will judge this pact.
3. **Delivery.** The seller sends the document with a signature over its exact bytes, so neither side can later pretend something else was delivered.
4. **Verdict.** The verifier reads the delivery, applies deterministic rules and signs an attestation: ALLOW 0.75 USDC, because only 3 of 4 pages arrived. Anyone can submit it; the signature is what counts.
5. **Settle.** The seller collects 0.75 USDC, and the contract returns 0.25 USDC to the agent in the same transaction.
6. **Receipt.** The outcome is hashed into a receipt and anchored, so the agent can prove later exactly what it paid for.

If the verifier had said DENY, step 5 would be a `release` instead and the agent would get the full 1 USDC back.

A **metered** purchase (say, minutes of GPU time) skips the verifier. The agent signs vouchers
like "usage so far: 500 units" as it consumes. When the session ends, the seller calls
`close_meter` with the latest voucher and is paid `500 × unit_price`. The rest of the cap returns
to the agent.

## How a pact moves through its life

```mermaid
stateDiagram-v2
    direction LR
    [*] --> Funded: open (buyer locks the cap)
    Funded --> Allowed: record_verdict ALLOW
    Funded --> Denied: record_verdict DENY
    Allowed --> Settled: settle (seller is paid, rest returned)
    Funded --> Settled: close_meter (metered) or settle under silence = Pay
    Denied --> Released: release
    Funded --> Released: release (silence = Refund, or past the settle deadline)
    Allowed --> Released: release (seller never settled)
    Settled --> [*]
    Released --> [*]
```

Every pact ends in exactly one terminal state, `Settled` or `Released`. In both, the amount paid
to the seller plus the amount returned to the buyer equals the cap. These are two of the eight
[invariants](docs/INVARIANTS.md), and each invariant is enforced by a named test.

## Two protection modes

Pactwright offers buyers two levels of protection. This repository implements the stronger one.

| | M0: exact refund | M2: escrow (this contract) |
|---|---|---|
| Where the money waits | Paid to the seller immediately | Locked in the `pact` contract |
| Who decides a refund | The seller, through its Accensa `RefundVault` | A verifier's signed verdict, or the buyer's signed usage |
| Needs the seller's cooperation | Yes | No |
| Best for | Cheap, high-volume calls from trusted sellers | Larger or one-off purchases, new sellers |
| Lives in | Pactwright-Kit (seller middleware) | `contracts/pact` |

## Who does what, and who you have to trust

| Role | Can | Cannot |
|---|---|---|
| **Buyer** | Open a pact (locks its own money); sign usage vouchers for metered pacts | Take the escrow back outside the release rules |
| **Seller** | Settle up to the allowed amount; close a meter for the signed usage | Collect more than the verdict or the usage allows |
| **Verifier** | Sign one verdict per pact | Move funds; change a pact |
| **Admin** | Rotate the verifier key for **new** pacts | Move funds; change any open pact |
| **Anyone** | Submit a valid verdict; call `release` when allowed; extend a pact's storage TTL | Send funds anywhere but to the buyer or seller |

In short, the buyer trusts only the verifier named in the pact, and the seller trusts the same
verifier to have checked honestly. Neither has to trust the admin or the other party. The full
analysis is in [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md).

## When things go wrong

| Situation | What happens |
|---|---|
| The delivery is wrong | The verifier signs DENY, anyone calls `release`, and the buyer gets the full cap back. |
| Only part was delivered | The verifier signs ALLOW with a smaller amount. The seller collects that and the rest returns. |
| The seller never delivers | No verdict arrives. With the default `Refund` policy, anyone can `release` after the verdict deadline. |
| The verifier goes offline | Same as above: the agreed silence policy applies at the verdict deadline. |
| The seller never collects | After the settle deadline anyone can `release`, so the money isn't stuck with an absent seller. |
| Someone submits a second, different verdict | Rejected: a pact accepts exactly one verdict. |
| A verdict meant for another pact is replayed | Rejected: attestations are bound to one `pact_id` and condition hash. |
| The verifier key leaks | The admin rotates the key for new pacts. Existing pacts are exposed only until their verdict deadline. |
| The issuer freezes the buyer's USDC | `release` can't pay a frozen account, so funds stay in escrow until the freeze lifts (a known limit). |

## What the contract does today

| Capability | Status |
|---|---|
| Escrow a capped purchase in USDC | Testnet |
| Verdict-gated settlement, partial amounts, single verdict per pact | Testnet |
| Pay-per-unit metering with signed usage vouchers | Testnet |
| Verdict and settle deadlines with a silence policy (refund or pay) | Testnet |
| Receipts anchored in `ReceiptAnchor` and verified against it | Tested (integration) |
| Admin cannot move funds or change an open pact | Tested (invariant 4) |
| Reproducible build: release = deployment = local build | Proven |
| Per-call cost limits enforced in CI and on real transactions | Proven |
| Mainnet | After the audit |

## Contract API

| Function | Who calls it | What it does |
|---|---|---|
| `__constructor(admin, token, verifier, max_pact_ledgers)` | Deployer, once | Sets the admin, the payment token, the first verifier key and the longest allowed pact |
| `open(params) → pact_id` | Buyer | Locks `cap` in escrow, snapshots the verifier key, stores the deadlines and silence policy |
| `record_verdict(pact_id, attestation, signature)` | Anyone | Records the single ALLOW/DENY verdict after checking the verifier's signature |
| `settle(pact_id, actual)` | Seller | Pays `actual` (≤ allowed) to the seller and returns the rest to the buyer |
| `close_meter(pact_id, units, signature)` | Seller | Pays `units × unit_price` from the buyer's signed voucher and returns the rest |
| `release(pact_id)` | Anyone | Returns the whole cap to the buyer when the release rules allow it |
| `set_verifier(key)` | Admin | Changes the verifier key for pacts opened afterwards |
| `get_pact(pact_id)`, `get_config()` | Anyone | Read a pact or the contract configuration |
| `extend_pact_ttl(pact_id)` | Anyone | Keeps an open pact's storage alive |

Every state change emits a typed event, listed in [docs/EVENTS.md](docs/EVENTS.md). The exact
byte encoding of IDs and signed messages is in [ARCHITECTURE.md §4.3](docs/ARCHITECTURE.md) and
pinned by [`test-vectors/`](test-vectors), which the TypeScript SDK tests against too.

## Error codes

A failed call returns `Error(Contract, #n)`:

| # | Name | Meaning |
|---|---|---|
| 1 | `NotInitialized` | The contract has no configuration |
| 2 | `AlreadyExists` | A pact with this `pact_id` already exists |
| 3 | `PactNotFound` | No pact with this `pact_id` |
| 4 | `InvalidState` | The pact isn't in a state that allows this call |
| 5 | `InvalidAmount` | Amount is zero, negative, or above the cap or the allowed amount |
| 6 | `SelfDealing` | Buyer and seller are the same address |
| 7 | `BadDeadlines` | Deadlines are in the past, out of order, or beyond the allowed length |
| 8 | `VerdictWindowClosed` | The verdict deadline has passed |
| 9 | `SettleWindowClosed` | The settle deadline has passed |
| 10 | `NotReleasable` | The release conditions aren't met yet |
| 11 | `AttestationMismatch` | The attestation is for another pact or condition |
| 12 | `BadSignature` | Reserved. A bad signature aborts the transaction instead, because `ed25519_verify` traps |
| 13 | `NotMetered` | `close_meter` on a pact without metering |
| 14 | `IsMetered` | Verdict or settle on a metered pact |
| 15 | `Unauthorized` | The caller isn't allowed to do this |

## Verify it yourself

Each step needs only this repo and the [stellar CLI](https://developers.stellar.org/docs/tools/cli) 28.1.0.

```bash
# 1. The tests: invariants, deadlines, fuzzing, integration, budgets
scripts/build.sh && cargo test

# 2. The build is reproducible: compare with the release asset and the deployed contract
scripts/build.sh                      # prints 748b5dbc…5d1a4
gh release download v0.1.0-rc.1 -R Pactwright/Pactwright-contracts -p '*.sha256' -O -

# 3. The deployed configuration (read-only simulation, no fees)
stellar contract invoke --network testnet --source <any-funded-identity> --send=no \
  --id CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6 -- get_config

# 4. Every hash and signature, byte for byte, against the published vectors
cargo test -p pact-sign
```

## On-chain evidence

A full purchase cycle on testnet, with every step a real transaction (bench deployment, test token):

| Step | What happened | Transaction |
|---|---|---|
| `open` | Buyer locks 1.0 token in escrow | [`e782ba7a…`](https://stellar.expert/explorer/testnet/tx/e782ba7a347590e1c310a0d31dcd4632547b2140b1dc3204060be03a9390ab61) |
| `record_verdict` | Verifier-signed ALLOW recorded | [`8ed129b4…`](https://stellar.expert/explorer/testnet/tx/8ed129b4f2590330dea6e3c08c8f06c59a9687ec1a6a2ad046e0f63f7ed832a4) |
| `settle` | Seller paid 0.5, buyer refunded 0.5, in one transaction | [`52899433…`](https://stellar.expert/explorer/testnet/tx/52899433fce58051a2b5f7690849fc086e033fc6ee4b0b386b211529ff2f9ae4) |
| `release` | DENY on a second pact; full refund, no one's permission needed | [`214da9bc…`](https://stellar.expert/explorer/testnet/tx/214da9bc23b3cceb96fbc2c17b3c1ada9f02d91a126a73128b0d45b384b7b8c7) |
| `close_meter` | Metered pact: 500 units paid from a signed voucher, rest refunded | [`60409d84…`](https://stellar.expert/explorer/testnet/tx/60409d845346a18a066cc58ba8393d4a365a8f2e787afbde22268a3bfa26259a) |

To rerun it, use `scripts/bench-testnet.sh --check` or the manual `bench-testnet` workflow in this repo.

### Every MVP scenario, on the main testnet deployment

On 2026-10-29, [Pactwright-Kit's testnet e2e](https://github.com/Pactwright/Pactwright-Kit/tree/main/e2e/testnet)
ran all seven MVP scenarios against this contract's main testnet deployment (`CDUXJJUN…MFEV6`). It used real testnet
USDC and a fresh Accensa `RefundVault` for the M0 refunds. The M2 rows are calls to this contract.
The Kit's own hashing produced each `pact_id`, and the contract accepted the Kit verifier's signatures.

| Scenario | Outcome | Step | Transaction |
|---|---|---|---|
| **M0 ALLOW** | ALLOW, seller keeps 1 | x402 exact payment to seller | [`e1105f24…`](https://stellar.expert/explorer/testnet/tx/e1105f24d40640eb66a73968c06ae4eedc9f860097a086b0f4450c25995c6600) |
| **M0 DENY** | DENY, 1 refunded | x402 exact payment to seller | [`ae9975c0…`](https://stellar.expert/explorer/testnet/tx/ae9975c028fe6d6bb2159cf4b532cebf42dc6ff8de095adedbac29b52542b8a3) |
|  |  | RefundVault.refund | [`2b78feb8…`](https://stellar.expert/explorer/testnet/tx/2b78feb8459b4731aa50bae1a290e3fd84a9a2b4956f5c6d2b7ae4f615bfae59) |
| **M0 non-delivery** | DENY, 0.5 refunded | x402 exact payment to seller | [`df8af0bd…`](https://stellar.expert/explorer/testnet/tx/df8af0bd9c8f4be9477589d9991ab063134b0e9f92094a90218f5edc9a629005) |
|  |  | RefundVault.refund | [`01357475…`](https://stellar.expert/explorer/testnet/tx/013574756ece7ae81643893f00fb14a2b220d54d096d936ea083a45129189b00) |
| **M2 ALLOW** | ALLOW 0.75 of 1, settled | open (1 USDC locked in escrow) | [`0b08b71e…`](https://stellar.expert/explorer/testnet/tx/0b08b71e164252de154d6500b3b8c23c0d68f6a8a1710b87b86e79bbe1166f6c) |
|  |  | record_verdict (ALLOW 0.75, verifier-signed) | [`84a6549a…`](https://stellar.expert/explorer/testnet/tx/84a6549aaa6b61b0e8040d1b43f2819914dc0e40f52d85be871f455d0d5a3e71) |
|  |  | settle (0.75 to seller, 0.25 back to buyer) | [`9fda6bea…`](https://stellar.expert/explorer/testnet/tx/9fda6beadb02f863f74e220820642a4509f269c31e3462fccf6318e1e2fa11c5) |
| **M2 DENY** | DENY, full refund | open (1 USDC locked in escrow) | [`b953ee00…`](https://stellar.expert/explorer/testnet/tx/b953ee00524d5b0a9b53308ab1e370a2008fb82b7f5a949ac1934d2aa12d17a7) |
|  |  | record_verdict (DENY, verifier-signed) | [`324f630d…`](https://stellar.expert/explorer/testnet/tx/324f630d39eb88fa76a60b92617e5867421e9819c290a8f3d71ec4bd8bec9138) |
|  |  | release (1 USDC back to buyer) | [`272703bf…`](https://stellar.expert/explorer/testnet/tx/272703bfbe0a74becf50a80b5d16600396a17440a82c9b3af5c481d4748c22cf) |
| **M2 silence** | no verdict, refunded by silence policy | open (1 USDC locked in escrow) | [`76df8318…`](https://stellar.expert/explorer/testnet/tx/76df83188eb7cf22c0b6c639fc0a090615ca3040ecfb6a3e43d6347cece27d59) |
|  |  | release after the verdict deadline (silence = Refund) | [`e62aff4d…`](https://stellar.expert/explorer/testnet/tx/e62aff4d291eff2b76ba282aae987d82c6ca91e1681e7944d96abb1f3739f833) |
| **Dispute refund** | dispute-1: 2 of 4 pages, 0.5 refunded | RefundVault.refund (dispute-1) | [`a191a000…`](https://stellar.expert/explorer/testnet/tx/a191a000f7920f2fff9839bbccceeeca35c82fce29a5e46510fd3db1bcdf2ff0) |

The model's reading of each delivery was a fixture (the rules decide), and the payments in this run
were direct. The facilitator's x402 `pact` scheme has since been merged and hosted on testnet; the
Kit's testnet run now also opens a pact through it ([evidence](https://github.com/Pactwright/Pactwright-Kit#testnet-evidence)),
and a verdict from the hosted verifier, with its real model, has been accepted by this contract
([evidence](https://github.com/Pactwright/Pactwright-Kit#the-hosted-verifier)).

## Stellar, specifically

| Stellar feature | Line of code |
|---|---|
| USDC moved into escrow through the Stellar Asset Contract | [lifecycle.rs:62](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/lifecycle.rs#L62) |
| `ed25519_verify` on the verifier's attestation | [verdict.rs:50](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/verdict.rs#L50) |
| `ed25519_verify` on the buyer's usage voucher | [meter.rs:49](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/meter.rs#L49) |
| Storage TTL extended past the settle window, so funds never sit in an archived entry | [lifecycle.rs:87](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/lifecycle.rs#L87) |
| Deadlines checked against the network's `max_ttl` | [lifecycle.rs:47](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/lifecycle.rs#L47) |
| Release pays only the buyer, with no auth required | [lifecycle.rs:176](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/lifecycle.rs#L176) |
| Domain-separated digests (verdict, usage, delivery, receipt) | [hashing.rs:14](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/hashing.rs#L14) |
| Typed events with `#[contractevent]` | [events.rs:17](https://github.com/Pactwright/Pactwright-contracts/blob/cd151bdf8bce48f9fe027eb64efee5fdd6f7ae27/contracts/pact/src/events.rs#L17) |

Why Stellar: sub-cent fees make per-purchase escrow viable, and USDC is native through the
Stellar Asset Contract. Soroban's predictable resource model also lets every call carry a
measured cost limit.

## What it costs

Measured on real testnet transactions ([BENCHMARKS.md](docs/BENCHMARKS.md)); fees in XLM:

| Call | CPU instructions | Fee |
|---|---|---|
| `open` | 1.07M | 0.0181 (includes storage rent) |
| `record_verdict` | 1.65M | 0.0009 |
| `settle` | 1.19M | 0.0012 |
| `release` | 1.03M | 0.0009 |
| `close_meter` | 2.04M | 0.0013 |

Every call stays under 2.1% of a transaction's 100M CPU limit. `open` costs more because it pays
rent for the pact's storage over the whole settle window. A settle costs 0.0012 XLM, which is
under 1% of a 1 USDC purchase at any XLM price below about $8.

CI fails if a change pushes any call past its recorded limit, using
[Tollcraft](https://github.com/Tollcraft) budget tests and cost lint. `scripts/bench-testnet.sh --check`
does the same against real transactions.

## Security and known limits

- **Not audited.** Testnet only until the audit (ROADMAP P8-01).
- **No admin path to funds**, tested as [invariant 4](docs/INVARIANTS.md).
- **Signatures are domain-separated** (`pactwright-verdict-v1:`, `pactwright-usage-v1:`, …), so one can never be replayed as another.
- **Known limits:**
  - A frozen buyer trustline blocks `release`.
  - The admin key itself can't be changed in this version; losing it only blocks verifier rotation, never funds.
  - The M0 refund mode lives outside this contract and depends on the seller.
- Report vulnerabilities privately, as described in [SECURITY.md](SECURITY.md).

## Clone, build and deploy

```bash
git clone https://github.com/Pactwright/Pactwright-contracts.git && cd Pactwright-contracts
rustup show                 # pinned toolchain + wasm32v1-none
scripts/build.sh            # needs stellar-cli 28.1.0; prints the WASM and its sha256
cargo test                  # 90 tests: contract, fuzzing, integration, budgets, vectors
```

| Script | What it does |
|---|---|
| [`scripts/build.sh`](scripts/build.sh) | Builds the WASM and prints its hash, which is the on-chain WASM hash |
| [`scripts/deploy.sh`](scripts/deploy.sh) | Deploys to testnet from `deployments/testnet.env`; refuses pubnet without an explicit confirmation |
| [`scripts/bindings.sh`](scripts/bindings.sh) | Generates TypeScript bindings |
| [`scripts/release-assets.sh`](scripts/release-assets.sh) | Builds every release asset (run by the `release` workflow on `v*` tags) |
| [`scripts/bench-testnet.sh`](scripts/bench-testnet.sh) | Runs every call as a real testnet transaction; `--check` enforces the limits |
| [`scripts/gen_vectors.sh`](scripts/gen_vectors.sh) | Regenerates `test-vectors/` from the contract's hashing code |

## Layout

| Path | What |
|---|---|
| [`contracts/pact`](contracts/pact) | The escrow contract: `lifecycle.rs`, `verdict.rs`, `meter.rs`, `hashing.rs`, tests |
| [`test-vectors`](test-vectors) | The source of truth for every hash and signature, shared with the TypeScript SDK |
| [`tools/pact-sign`](tools/pact-sign) | Pact IDs and signatures from the contract's own hashing code |
| [`vendor/wasm`](vendor/wasm) | Pinned, hash-checked Accensa `ReceiptAnchor` and `RefundVault` |
| [`scripts`](scripts) | Build, deploy, bindings, release assets, testnet benchmark |
| [`docs`](docs) | PRD, architecture, ADRs, events, invariants, benchmarks, security model, runbook |
| [`DEPLOYMENTS.md`](DEPLOYMENTS.md) | Every deployment with provenance, including retired ones |

## Project status

Phase 1, the contracts MVP, is complete. It covers escrow, verdicts, deadlines, metering,
receipts, testnet deployment, the release, and cost limits at both levels: local tests and real
transactions. Next for this repo are the audit and mainnet (Phase 8). [ROADMAP.md](ROADMAP.md)
tracks progress task by task across all three repos, and every pull request updates it.

## FAQ

**Why not just let the seller refund?** Then a refund depends on the seller. Pactwright supports
that lighter mode too (M0, through the seller's refund vault). This contract is the mode where
neither side has to trust the other.

**Who runs the verifier?** For now, Pactwright does. The verifier's key is fixed per pact, and
its verdicts follow the open [Witness](https://github.com/fabian416/witness) envelope format. It
can only sign verdicts; it can never move funds.

**Can a verdict be wrong?** Yes. That is why the condition is frozen before payment, the verdict
is bound to it, and partial amounts exist. Verifier dispute milestones are planned for v1
(ROADMAP P7-05).

**What does an agent need to hold?** Only USDC. Account reserves can be sponsored, and network
fees for these calls are fractions of a cent.

**Can I reuse this code?** Yes, under the MIT license (see [License](#license)). It is unaudited,
so read the [security model](docs/SECURITY_MODEL.md) first.

## Built on

| Project | What Pactwright uses | How |
|---|---|---|
| [Accensa](https://github.com/accensa) | `ReceiptAnchor`, `RefundVault`, x402 facilitator | Vendored WASM (v0.2.0); facilitator `pact` scheme |
| [Tollcraft](https://github.com/Tollcraft) | Cost lint and budget tests | CI Action; `budget-macros` in tests |
| [Witness](https://github.com/fabian416/witness) | Verdict envelope format, "model reads, code decides" | Format and pattern; code vendored in Pactwright-Kit |
| [Ficha](https://github.com/beto-rocha-blockchain/ficha-stellar) | The idea of refunding unused units | Re-implemented as signed usage vouchers |
| [Vespi](https://github.com/andresanemic/vespi), [BookerBob](https://github.com/hustle1hustle/bookerbob-hackmeridian-2026), [Pollar](https://docs.pollar.xyz) | Budget approval, reputation, seedless wallets | Add-ons in Pactwright-Kit (v1, v2) |

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md). Every pull request updates
[ROADMAP.md](ROADMAP.md): task status, progress counts and one Update log line. `main` is
protected, so pull requests need a review and green `rust`, `cost-lint` and `roadmap` checks.

## License

Pactwright-contracts is released under the **[MIT License](LICENSE)**.

```text
Copyright (c) 2026 Pactwright contributors
```

### What the MIT license means for you

| You may | You must | You get no |
|---|---|---|
| Use the code in open or closed-source projects, commercially or not | Keep the copyright notice and the license text in every copy or substantial portion | Warranty of any kind |
| Copy, modify, merge and publish it | | Liability from the authors for losses, including funds lost through a bug |
| Distribute, sublicense and sell copies | | |
| Deploy your own `pact` contract built from this code | | |

The "no warranty" part matters more than usual here, because this code holds money and has not
been audited. If you deploy it, the risk is yours. Read
[docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md) and the known limits above first.

A compiled WASM built from this repo is a copy of the code too. If you redistribute one, include
the MIT notice ([LICENSE](LICENSE)) alongside it.

### Third-party code and licenses

These are the components this repository ships, vendors or builds with. Each keeps its own
license. `cargo-deny` checks the whole dependency tree in CI against an allow-list in
[`deny.toml`](deny.toml): MIT, Apache-2.0, BSD-3-Clause, ISC and Unicode-3.0.

| Component | How it is used | Ends up in the deployed WASM | License |
|---|---|---|---|
| [soroban-sdk](https://crates.io/crates/soroban-sdk) 28.0.0 | The contract framework | Yes | Apache-2.0 |
| Accensa [`ReceiptAnchor`, `RefundVault`](https://github.com/accensa/accensa-contracts) v0.2.0 | Vendored WASM in [`vendor/wasm`](vendor/wasm), hash-checked; used in integration tests and deployed as Pactwright's receipt anchor | Separate contract | MIT |
| Tollcraft [budget-macros](https://github.com/Tollcraft/soroban-budget-assert) | Per-call budget tests | No, tests only | MIT |
| Tollcraft [soroban-cost-linter](https://github.com/Tollcraft/soroban-cost-linter) | Cost lint GitHub Action | No, CI only | Apache-2.0 |
| [ed25519-dalek](https://crates.io/crates/ed25519-dalek) 2.2.0 | Signing in tests and `tools/pact-sign` | No | BSD-3-Clause |
| [stellar-strkey](https://crates.io/crates/stellar-strkey) | Address and key encoding in tests and tools | No | Apache-2.0 |
| [bolero](https://crates.io/crates/bolero) 0.13.7 | Fuzz and property tests | No | MIT |
| [Witness](https://github.com/fabian416/witness) verdict format | The envelope format the verifier signs. Its code is vendored in Pactwright-Kit, not here | No | Apache-2.0 |

The deployed contract contains only this repo's code and soroban-sdk, under MIT and Apache-2.0
respectively. Everything else is used to test, sign or measure it.

### Ideas, without code

Some projects inspired a feature without any of their code being copied. They are credited
anyway:

- **Ficha** (MIT): refunding unused units, re-implemented here as signed usage vouchers.
- **Vespi** (Apache-2.0) and **BookerBob** (MIT): budget approval and reputation, planned as Pactwright-Kit add-ons.
- **Pollar**: integrated only through its public SDK. Pollar Studio has no license file, so none of its code is used.

[NOTICE](NOTICE) lists every project Pactwright builds on and its license.

### The other Pactwright repositories

| Repository | License | Why |
|---|---|---|
| Pactwright-contracts (this repo) | MIT | Short, permissive terms, so anyone can build on the contract or deploy their own |
| [Pactwright-Kit](https://github.com/Pactwright/Pactwright-Kit) | Apache-2.0 | It vendors Witness core, which is Apache-2.0; Apache-2.0 also adds an explicit patent grant |
| [Pactwright-studio](https://github.com/Pactwright/Pactwright-studio) | MIT | Same terms as this repo |

### Contributing under this license

By opening a pull request, you agree that your contribution is licensed under the same MIT
license as the rest of the repository. There is no separate contributor license agreement.
