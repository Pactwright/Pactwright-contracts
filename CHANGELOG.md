# Changelog — pactwright-contracts

All notable changes to this project are documented here. Versioning follows SemVer;
pre-1.0 minor versions may contain breaking changes.

## [Unreleased]
- Tier B testnet benchmark: `scripts/bench-testnet.sh` (+ `--check`), `tools/pact-sign`, limits in `budget.toml`, manual `bench-testnet` workflow replacing `budget-report.yml` (C-21).
- `pactwright_pact::hashing` is public for off-chain tools.
- Testnet `ReceiptAnchor` redeployed with a rotated anchorer key; previous instances retired (DEPLOYMENTS.md).
- OpenSSF Scorecard workflow and badge; CODEOWNERS uses org teams; branch-protection and Dependabot policy documented (F-06, F-07).
- `close_meter`: metered pacts settle from the buyer's signed cumulative usage voucher; `open` accepts metered pacts (P7-01).
- `scripts/build.sh`, `deploy.sh` (pubnet guard, writes IDs back), `bindings.sh`, `release-assets.sh` (C-17, C-18).
- Release workflow publishes the WASM, its hash, TS bindings and test vectors on `v*` tags (C-18).
- First testnet deployment: `pact` and Pactwright's own `ReceiptAnchor` (C-17, C-20); see DEPLOYMENTS.md.
- Network-upgrade and testnet-reset runbook (XC-04).
- Project scaffold (structure, data models, stubs). See `ROADMAP.md`.
