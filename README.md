<p align="center">
  <img src="brand/pactwright-mark.png" alt="Pactwright" width="128">
</p>

<p align="center">
  <b>Escrow for AI-agent purchases on Stellar.</b><br>
  The buyer's money moves only when delivery is proven. Otherwise it comes back.
</p>

<p align="center">
  <a href="https://github.com/Pactwright/Pactwright-contracts/actions/workflows/ci.yml"><img src="https://github.com/Pactwright/Pactwright-contracts/actions/workflows/ci.yml/badge.svg" alt="CI status"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT license"></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/Pactwright/Pactwright-contracts"><img src="https://api.scorecard.dev/projects/github.com/Pactwright/Pactwright-contracts/badge" alt="OpenSSF Scorecard"></a>
</p>

<p align="center">
  <a href="https://pactwright.github.io/docs/"><strong>Documentation</strong></a> ·
  <a href="#-quick-start"><strong>Quick start</strong></a> ·
  <a href="#-benchmarks"><strong>Evidence</strong></a> ·
  <a href="https://github.com/Pactwright/Pactwright-studio/releases/download/v0.1.0-rc.1/brag.mp4"><strong>Watch Film (1080p)</strong></a>
</p>

<p align="center">
  <a href="https://github.com/Pactwright/Pactwright-studio/releases/download/v0.1.0-rc.1/brag.mp4">
    <img src="https://github.com/Pactwright/Pactwright-studio/releases/download/v0.1.0-rc.1/film-preview.webp" width="100%" alt="Pactwright Launch Film: one agent, four purchases, four endings, each enforced on Stellar testnet">
  </a>
</p>

> 🔊 **Turn sound on.** [Download or watch the full film (1080p, 60 fps, with music & sound)](https://github.com/Pactwright/Pactwright-studio/releases/download/v0.1.0-rc.1/brag.mp4)  
> *One agent, four purchases, four different endings on Stellar testnet: paid pro rata for 3 of 4 pages, refunded for a tortilla recipe, refunded when nobody answered, and half refunded after a dispute.*

> [!IMPORTANT]
> **Testnet only, not audited.** The `pact` contract is live on Stellar testnet and every number
> below links to a real transaction or to the file that recorded it. Nothing here is on mainnet, and
> no external audit has been performed; do not use it with real funds until the audit (ROADMAP
> P8-01). The safest first read is [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md).

| | |
|---|---|
| `pact` contract (testnet) | [`CDUXJJUN…MFEV6`](https://stellar.expert/explorer/testnet/contract/CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6) |
| Its WASM hash | [`748b5dbcc800…35d1a4`](DEPLOYMENTS.md), identical in the [release](https://github.com/Pactwright/Pactwright-contracts/releases/tag/v0.1.0-rc.1), on testnet and in a local build |
| Receipt anchor (testnet) | [`CCLPJIYC…EDXZ`](https://stellar.expert/explorer/testnet/contract/CCLPJIYCSZXAK5HQNVOBO634YGATAL7ASE6HZRFQLKB3TJHMK7NCEDXZ), Accensa `ReceiptAnchor` v0.2.0 |
| Payment token | USDC ([`CBIELTK6…DAMA`](https://stellar.expert/explorer/testnet/contract/CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA), Circle testnet) |
| Measured cost | [docs/BENCHMARKS.md](docs/BENCHMARKS.md): local and real-transaction numbers per call |
| Trust model | [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md) |
| Documentation | [pactwright.github.io/docs](https://pactwright.github.io/docs/), including the [contract API reference](https://pactwright.github.io/docs/reference/contract/) |
| Design | [PRD](docs/PRD.md) · [Architecture](docs/ARCHITECTURE.md) · [15 ADRs](docs/adr/) |
| License | [MIT](LICENSE), with every third-party license listed |

### 🎯 Hackathon Scale-Path Goals (PRD §9)

Pre-declared targets for the Stellar Hackathon Scale path:
1. **First M2 pact settled on mainnet**: Settle the first verifiable AI agent purchase on Stellar mainnet post-audit.
2. **Autonomous buyer dispute & escrow recovery**: An AI agent buys compute/data through MCP, receives a cryptographic DENY, and recovers its locked escrow on stage without human or seller intervention.
3. **Metered pay-per-minute demo**: Live dynamic streaming settlement using signed `UsageVoucher`s with Pollar principal funding and Vespi budget approval.

## 📖 Overview

This repository is the on-chain half of Pactwright: the `pact` Soroban escrow contract for
purchases an AI agent makes. The buyer locks a cap. A verifier's signed verdict — or, for metered
services, the buyer's own signed usage — decides how much the seller gets. The rest returns to the
buyer. No admin can move those funds, and no counterparty's cooperation is required to get money
back once a verdict says DENY.

AI agents already buy things on their own: an API call, a translated document, a dataset, minutes of
compute. On Stellar they can already pay with [x402](https://developers.stellar.org/docs/build/agentic-payments/x402):
the seller answers `402 Payment Required`, the agent pays in USDC and retries. That covers *paying*.
It does not cover what happens when the purchase goes wrong, and an agent has no one to call. So the
rules for getting money back have to be inside the payment itself. That is what a pact is.

This README and [docs/OVERVIEW.md](docs/OVERVIEW.md) describe what this repo actually contains and actually proves: the contract and its API,
the invariants and test vectors that pin its behaviour, the cost limits enforced in CI and measured
on real transactions, and the testnet evidence. **What is not real here:** there is no mainnet
deployment, no audit report, and no M1 `upto` mode — the lighter M0 refund mode is real, but it
lives in the seller's middleware, not in this contract. Full documentation lives at
[pactwright.github.io/docs](https://pactwright.github.io/docs/).

The SDK, seller middleware and verifier live in
[Pactwright-Kit](https://github.com/Pactwright/Pactwright-Kit); pact pages and receipt anchoring live
in [Pactwright-studio](https://github.com/Pactwright/Pactwright-studio).

### 🏗️ Architecture

The workspace has three crates, a vendored dependency and the scripts that bind them together, all
in one repo:

1. **`contracts/pact` — the escrow contract** ([manifest](contracts/pact/Cargo.toml), `publish = false`)
   - Nine entrypoints: `open`, `record_verdict`, `settle`, `close_meter`, `release`, `set_verifier`,
     `get_pact`, `get_config`, `extend_pact_ttl`, plus the deploy-time `__constructor`.
   - `lifecycle.rs` moves USDC through the Stellar Asset Contract and enforces the two deadlines;
     `verdict.rs` and `meter.rs` verify ed25519 signatures; `hashing.rs` is the only place a hash is
     computed.
   - Amounts are `i128` stroops; every failure aborts with a named `panic_with_error!` code.
2. **`test-vectors/` — the source of truth for every hash** ([README](test-vectors/README.md))
   - Eight vector files, generated from the contract's own hashing code by `scripts/gen_vectors.sh`
     and never edited by hand; `cargo test` fails when a committed file drifts from the code.
     The one documented exception is `condition_hash.json`, which comes from the Kit's canonical-JSON
     hashing (ADR-015) and is treated as opaque bytes here.
   - The same files are consumed by the TypeScript SDK in Pactwright-Kit, so both implementations are
     pinned to one encoding.
3. **`tools/pact-sign` — off-chain signing** ([manifest](tools/pact-sign/Cargo.toml))
   - Produces pact IDs, verdict attestations and usage vouchers using the contract crate as a library,
     so a test signature and an on-chain-verifiable signature come from the same code path.
4. **`contracts/testutils` and `vendor/wasm` — real counterparties in tests** ([`SHA256SUMS`](vendor/wasm/SHA256SUMS))
   - Pinned, hash-checked Accensa `ReceiptAnchor` and `RefundVault` v0.2.0 WASMs, used in integration
     tests so receipts are verified against the contract that will check them in production.
5. **`scripts/` and `.github/workflows/` — the two cost tiers**
   - `scripts/build.sh` prints the WASM hash — the same sha256 the on-chain deployment carries, which
     [DEPLOYMENTS.md](DEPLOYMENTS.md) records; `scripts/bench-testnet.sh` runs every entrypoint as a
     real testnet transaction, and `--check` fails the run when a recorded limit in
     [budget.toml](budget.toml) is exceeded.

## 📊 Benchmarks

Measured cost per call, locally and in real testnet transactions: [docs/BENCHMARKS.md](docs/BENCHMARKS.md).

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

These fourteen hashes are recorded in the Kit repository's
[testnet evidence](https://github.com/Pactwright/Pactwright-Kit#-testnet-evidence), not in this repo;
this table links them. The model's reading of each delivery in that run was a fixture (the rules
decide), and the payments were direct. The facilitator's x402 `pact` scheme has since been
[merged into Accensa's facilitator](https://github.com/accensa/x402-facilitator-stellar/pull/651) and
hosted on testnet, and the Kit's testnet run now also opens a pact through it; a verdict from the
hosted verifier, with its real model, has been accepted by this contract
([evidence](https://github.com/Pactwright/Pactwright-Kit#-testnet-evidence)).

## ⚙️ Supported Versions & Compatibility

Testnet only, pre-1.0. What "supported" means for this repo, and the version policy, is in [docs/OVERVIEW.md](docs/OVERVIEW.md#what-supported-means-here).

### Compatibility matrix

| Component / target | Version | Status | Notes |
|---|---|---|---|
| Rust toolchain | 1.98.1, `wasm32v1-none` | Supported | Pinned in `rust-toolchain.toml`; `ci.yml` installs it via `rustup show` on every push and PR. |
| `soroban-sdk` | 28.0.0 (manifest) | Supported | Builds, tests and produces the deployed WASM under this pin in `ci.yml`. |
| `soroban-env-host` | 28.0.2 (resolved) | Supported | What the WASM-mode budget tests execute against. |
| `stellar-cli` | 28.1.0 | Supported | `ci.yml` installs this exact release; `DEPLOYMENTS.md` records the deploy. |
| Cost-lint build toolchain | `nightly-2026-04-16` | Supported | Forced by `cost-lint.yml` because the Action builds its lint library from this repo's root. |
| Tollcraft `budget-macros` | git `rev b211441…` | Supported | Pinned in `contracts/pact/Cargo.toml`; the budget tests run in `ci.yml`. |
| Older `soroban-sdk` / older protocol | < 28 | Untested | No pin, no CI job, no measurement recorded in this repo. |
| Future `soroban-sdk` / protocol upgrades | > 28 | Untested | Budget baselines must be re-measured on upgrade (XC-04, runbook). |
| Stellar protocol number | not recorded here | Untested | This repo states no protocol version; the SDK pin is the only compatibility statement. |
| Stellar testnet | `CDUXJJUN…MFEV6` | Supported | Deployed 2026-10-28; every entrypoint exercised by real transactions. Verified by hand and by the manual `bench-testnet.yml`, not by a push gate. |
| Stellar pubnet / mainnet | — | Not implemented | Follows the audit (P8-03); `deployments/pubnet.env` holds empty inputs only. |
| M2 escrow mode (this contract) | — | Supported | Verdict, metered, deadline and release paths tested in `ci.yml` and on-chain. |
| M0 exact-refund mode | — | Not implemented | Lives in Pactwright-Kit seller middleware and the Accensa `RefundVault`. |
| M1 `upto` partial-payment mode | — | Not implemented | Deliberately excluded; Pactwright ships two modes only (AGENTS.md §3). |
| Facilitator x402 `pact` scheme | hosted on testnet | Untested | Merged upstream and exercised in Pactwright-Kit's CI, not this repo's. |
| TypeScript SDK parity (`@pactwright/core`) | via `test-vectors/` | Untested | The vectors live here and `cargo test` asserts them; the SDK's matching assertions run in Pactwright-Kit's CI. |

## 🚀 Quick Start

### 1. Installation

The crates are not published, so this is a source build:

```bash
git clone https://github.com/Pactwright/Pactwright-contracts.git
cd Pactwright-contracts
rustup show              # installs the pinned 1.98.1 toolchain + wasm32v1-none
cargo test --workspace   # 90 tests, measured 2026-10-30: contract, fuzzing, integration, budgets, vectors
```

Building the deployed artifact needs [stellar CLI](https://developers.stellar.org/docs/tools/cli)
28.1.0 — the same release CI installs:

```bash
scripts/build.sh         # builds the WASM and prints its sha256
cargo --version          # the rust 1.98.1 pin comes from rust-toolchain.toml
```

> [!NOTE]
> No crate in this workspace is published to crates.io — `publish = false` in all three manifests.
> Install from source as above, or take the WASM, TypeScript bindings and test vectors from the
> [v0.1.0-rc.1 release](https://github.com/Pactwright/Pactwright-contracts/releases/tag/v0.1.0-rc.1),
> which CI builds from this repo's [`release.yml`](.github/workflows/release.yml).

### 2. Configuration

Three files carry the settings a reader actually needs to know about:

| File | What it configures |
|---|---|
| [`rust-toolchain.toml`](rust-toolchain.toml) | The Rust version and `wasm32v1-none` target every build uses. |
| [`budget.toml`](budget.toml) | Shared Tollcraft config: target `network`, signing `source` identity, and the Tier B instruction/byte limits each entrypoint may not exceed. Limits are measured × 1.15, rounded up. |
| [`deployments/testnet.env`](deployments/testnet.env) | Testnet contract IDs, WASM hashes, token, verifier key and `max_pact_ledgers`. Written by `scripts/deploy.sh`; public values only — secrets stay in the stellar CLI keystore. |

The `[profile.release]` block in [Cargo.toml](Cargo.toml) is a measurement input, not cosmetics:
`opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"` and `overflow-checks = true`
change the WASM that gets deployed and measured. Figures produced under a different profile are a
different build and are not comparable to the numbers in this README.

Deploying a testnet instance of your own:

```bash
stellar network add testnet   # if the network is not configured yet
scripts/deploy.sh testnet pact
```

### 3. Usage

**Build, test, and check the cost limits, as CI does:**

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
scripts/build.sh && cargo test
cargo test invariant          # the eight invariants by name
cargo test fuzz               # bolero property and fuzz targets
cargo test -p pact-sign       # every hash and signature against the published vectors
```

**Verify the deployment is the code you just built.** Each step needs only this repo and stellar CLI
28.1.0:

```bash
# 1. The build is reproducible: compare with the release asset and the deployed contract
scripts/build.sh                      # prints 748b5dbcc800…35d1a4
gh release download v0.1.0-rc.1 -R Pactwright/Pactwright-contracts -p '*.sha256' -O -

# 2. The deployed configuration (read-only simulation, no fees)
stellar contract invoke --network testnet --source <any-funded-identity> --send=no \
  --id CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6 -- get_config

# 3. The vendored Accensa WASMs still match their recorded hashes
scripts/check_vendor_hashes.sh
```

More scripts (benchmarks, bindings, release assets, vectors) are listed in [docs/OVERVIEW.md](docs/OVERVIEW.md#scripts).

## 🤝 Community & Maintainers

Join the discussion and get support:

- **Discord**: [discord.gg/4WFU6fwPe](https://discord.gg/4WFU6fwPe)
- **Telegram**: [t.me/Pactwright](https://t.me/Pactwright)
- Issues and task work: [Pactwright-contracts issues](https://github.com/Pactwright/Pactwright-contracts/issues)
  (pick one labelled `Stellar Wave`; `core-team-only` issues are not open to outside contributors).
- Documentation hub: [pactwright.github.io/docs](https://pactwright.github.io/docs/)
- The other repositories: [Pactwright-Kit](https://github.com/Pactwright/Pactwright-Kit) (SDK, verifier,
  seller middleware) and [Pactwright-studio](https://github.com/Pactwright/Pactwright-studio) (pact pages
  and receipt anchoring).
- Design discussion: [docs/adr/](docs/adr/) holds ADR-001…ADR-015, and
  [docs/ARCHITECTURE_ESSENTIALS.md](docs/ARCHITECTURE_ESSENTIALS.md) states the rules a change must not
  break.
<!-- demo: fill in — screencast or dashboard link -->

| Maintainer | Role | Scope |
|---|---|---|
| Allison ([@mallison031](https://github.com/mallison031)) | Lead maintainer; reach out on [Telegram](https://t.me/Pactwright) or [Discord](https://discord.gg/4WFU6fwPe) | Everything |
| [`@Pactwright/maintainers`](https://github.com/orgs/Pactwright/teams/maintainers) | Repository maintainers | Everything ([CODEOWNERS](.github/CODEOWNERS)) |
| [`@Pactwright/core`](https://github.com/orgs/Pactwright/teams/core) | Core team | `lifecycle.rs`, `verdict.rs`, `meter.rs`, `hashing.rs`, `test-vectors/`, `tools/pact-sign/`, `vendor/wasm/`, `deployments/` |

## 🛠️ Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md) and
[docs/ARCHITECTURE_ESSENTIALS.md](docs/ARCHITECTURE_ESSENTIALS.md) before you build. Report
vulnerabilities privately as described in [SECURITY.md](SECURITY.md) — not as a public issue.

`main` is protected: changes land through pull requests, and the `rust`, `cost-lint`, `roadmap`
and malware-guard checks must pass. Core-team-only paths need a review from
`@Pactwright/core`. Use conventional commits (`feat:`, `fix:`, `docs:`, `test:`, `chore:`), add tests
for every behavior change, and write an ADR in `docs/adr/` before changing documented behavior.

**Every pull request updates [ROADMAP.md](ROADMAP.md)** — task status, progress counts, and one
Update log line, even for a docs or dependency change. The `roadmap-check` job fails a PR that
doesn't. Phase 1, the contracts MVP, is complete: escrow, verdicts, deadlines, metering, receipts,
testnet deployment, the release, and cost limits at both tiers. What is left for this repo is the
audit and mainnet (Phase 8).

### 🧑‍💻 Contributors

<p align="center">
  <a href="https://github.com/Pactwright/Pactwright-contracts/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=Pactwright/Pactwright-contracts" alt="Contributors">
  </a>
</p>

## License

Pactwright-contracts is released under the **[MIT License](LICENSE)**.

```text
Copyright (c) 2026 Pactwright contributors
```
