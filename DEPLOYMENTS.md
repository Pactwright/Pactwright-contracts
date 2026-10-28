# Deployments

Every Pactwright deployment, with enough provenance to verify it independently.
Machine-readable values live in [`deployments/<network>.env`](deployments/), written by
[`scripts/deploy.sh`](scripts/deploy.sh).

## Testnet

Deployed 2026-10-28 with `scripts/deploy.sh testnet all` (stellar-cli 28.1.0, rustc 1.98.1,
`wasm32v1-none`). The `pact` WASM was built from the source in the PR that added this file
(branch `feat/metering-testnet-deploy-release`); rebuild with `scripts/build.sh` and compare
the hash.

| Contract | ID | WASM sha256 | Notes |
|---|---|---|---|
| `pact` | [`CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6`](https://stellar.expert/explorer/testnet/contract/CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6) | `748b5dbcc800d8cd3bdaf4b24555bfa8e3659cdefb60888791fb9a086b35d1a4` | Includes `close_meter` (P7-01) |
| `ReceiptAnchor` (Accensa v0.2.0) | [`CA2VU3DRTV7V2A5CL3WVFK7FIXEMTJSLVI2ERKZHR6MEWDZY3ZGZ3ITX`](https://stellar.expert/explorer/testnet/contract/CA2VU3DRTV7V2A5CL3WVFK7FIXEMTJSLVI2ERKZHR6MEWDZY3ZGZ3ITX) | `d7ec929b1f678d1f26f98dd803a566a44502ce7ddb890613612007166061f31e` | Vendored WASM (`vendor/wasm/`); hash matches `SHA256SUMS` |

**`pact` constructor arguments**

| Argument | Value |
|---|---|
| `admin` | `GD4BTAHYO7JBRTUVIB7DY7OLOTZZSDAE26YLRWPJ7OVQ3KLINK237UOT` (`pactwright-deployer`) |
| `token` | `CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA` (Circle testnet USDC SAC, issuer `GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5`) |
| `verifier` | `4efdaeb6c34ef1babbc844909afe576b288a36a2afe43a8bf6fcff82c9ffe4ff` (`pactwright-verifier`, `GBHP3LVWYNHPDOV3ZBCJBGX6K5VSRCRWUKX6IOUL636P7AWJ77SP7TDT`) |
| `max_pact_ledgers` | `120960` (~7 days) |

**`ReceiptAnchor`**: `initialize(merchant = GDBPV6OMFX5EJZFARLQ4DVYHQGI2M3S7TFSDKN4AD4OHB6PPNWMBZGIR)`
(`pactwright-anchorer`). Verified from on-chain instance storage: `Admin` is the anchorer.

**Testnet identities** (F-11; secrets stay in the stellar CLI keystore of the maintainer who
created them, never in git):

| Identity | Address | Role |
|---|---|---|
| `pactwright-deployer` | `GD4BTAHYO7JBRTUVIB7DY7OLOTZZSDAE26YLRWPJ7OVQ3KLINK237UOT` | Deploys contracts; `pact` admin |
| `pactwright-sponsor` | `GAVTUPPEMLXAE2TFFPMPG43QTEV5UPKPFV55YQ4OUYVTE4NML7W6F2LU` | Sponsors agent account reserves (K-10) |
| `pactwright-verifier` | `GBHP3LVWYNHPDOV3ZBCJBGX6K5VSRCRWUKX6IOUL636P7AWJ77SP7TDT` | Signs verdict attestations |
| `pactwright-anchorer` | `GDBPV6OMFX5EJZFARLQ4DVYHQGI2M3S7TFSDKN4AD4OHB6PPNWMBZGIR` | `ReceiptAnchor` admin (S-04) |

Testnet is reset periodically; after a reset, re-fund the identities and re-run
`scripts/deploy.sh testnet all` (see [`docs/RUNBOOK-network-upgrade.md`](docs/RUNBOOK-network-upgrade.md)).

## Pubnet

None. Pubnet deployment follows the audit (ROADMAP P8-03); `scripts/deploy.sh pubnet` refuses
to run with placeholder inputs or without `PACTWRIGHT_CONFIRM_PUBNET=yes`.
