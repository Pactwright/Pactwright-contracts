# Changelog — pactwright-contracts

All notable changes to this project are documented here. Versioning follows SemVer;
pre-1.0 minor versions may contain breaking changes.

## [Unreleased]
- Documented partner outreach, letters of support, and Pollar SDK partnership in `docs/partners.md` (F-08, F-09).
- Published 3 independent testnet sellers registry and routes in `docs/TESTNET_SELLERS.md` (L-04).
- Compiled MVP launch metrics report in `docs/LAUNCH_METRICS.md` (L-05).
- Compiled SCF tranche 1-3 milestone submissions in `docs/scf/` (L-06, P7-18, P8-10).
- Published hackathon Scale-path goals in `README.md` (P7-16).
- Added shared docs release sync script `scripts/sync-shared-docs.sh` (XC-09).
- Logged partner release communications in `docs/partners.md` (XC-11).
- Branch protection review requirement enforced (`required_approving_review_count: 1`, `enforce_admins: true`) (F-07, #71).
- PR Malware & Supply-Chain Guard workflow and static scanner detecting Trojan Source, malicious shell scripts, and prohibited binaries.
- 10-contributor issue allocation and anti-malware policy documented in `AGENTS.md` §7 & §8 (XC-07, #46).
- Security policy and release threat model review updated in `SECURITY.md` and `docs/SECURITY_MODEL.md` (XC-02, #42).

## [0.1.0-rc.1] - 2026-10-28
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
