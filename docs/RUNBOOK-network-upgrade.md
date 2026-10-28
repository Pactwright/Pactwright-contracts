# Runbook: Stellar network upgrades and testnet resets (ROADMAP XC-04)

Protocol upgrades change host behaviour, cost models and network limits. Testnet resets wipe
every account and contract. This runbook keeps Pactwright's contracts, budgets and deployments
correct through both.

## 1. Before the upgrade (when SDF announces a protocol vote)

1. Read the protocol release notes and the matching `soroban-sdk` / `stellar-cli` releases.
2. Note any change to per-transaction limits (CPU, memory, read/write bytes, entry counts) or
   to the cost model. The current ceilings are listed in `budget.toml`'s header.
3. Open a tracking issue: "Protocol <N> upgrade", labelled `area: contracts`, `area: ops`.

## 2. Update toolchains (on a branch)

1. Bump `soroban-sdk` in `Cargo.toml` and `stellar-cli` in `.github/workflows/release.yml`
   (`STELLAR_CLI_VERSION`) only if the new protocol requires it.
2. Bump `rust-toolchain.toml` only if the new SDK requires it.
3. If the Tollcraft cost-lint Action needs a new nightly, update `RUSTUP_TOOLCHAIN` and the
   `toolchain` input in `.github/workflows/cost-lint.yml` together, and pin the new Action commit.

## 3. Prove nothing changed that shouldn't

1. `cargo test` (including `test::vectors`): any hash drift is a bug, not an upgrade effect.
   Never regenerate vectors to make an upgrade pass.
2. `scripts/check_vendor_hashes.sh`: vendored Accensa WASMs must still match.
3. `scripts/build.sh`: record the new WASM hash. A different hash after an SDK bump is expected;
   with an unchanged SDK it is not.
4. `cargo clippy --all-targets -- -D warnings` and the cost-lint Action.

## 4. Re-baseline budgets

1. Re-run the Tier A budget tests; failures show which entrypoints got more expensive.
2. On testnet after the upgrade: `cargo budget-report --derive-limits`, review the diff, and
   commit new limits at measured × 1.15. Never round down; explain every increase in the PR.
3. Update `docs/BENCHMARKS.md` with before/after numbers and the protocol version.
4. If `max_ttl` changed, check that `max_pact_ledgers + GRACE_LEDGERS` still fits (`open` rejects
   deadlines that need more than `max_ttl`).

## 5. Verify the live deployment

1. `stellar contract invoke --id $PACT_CONTRACT_ID --network testnet --source pactwright-deployer --send=no -- get_config`
   must still return the recorded configuration.
2. Run the Pactwright-Kit nightly testnet e2e (K-16) manually and link the run in the issue.
3. Contracts are not redeployed for a protocol upgrade unless the SDK change requires it. A
   redeploy mints new contract IDs: update `deployments/<network>.env`, DEPLOYMENTS.md, and
   every consumer (Kit `.env`, studio, facilitator config).

## 6. Testnet reset

1. Re-fund the identities: `stellar keys fund pactwright-deployer --network testnet` (and
   `pactwright-sponsor`, `pactwright-verifier`, `pactwright-anchorer`).
2. `scripts/deploy.sh testnet all`, then update DEPLOYMENTS.md with the new IDs.
3. Announce the new IDs to Pactwright-Kit and studio maintainers.

## 7. Close out

- CHANGELOG entry under `[Unreleased]`, ROADMAP Update log line, and close the tracking issue
  with links to the PR, the budget diff and the e2e run.
