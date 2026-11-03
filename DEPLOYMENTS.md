# Deployments

Every Pactwright deployment, with enough provenance to verify it independently.
Machine-readable values live in [`deployments/<network>.env`](deployments/), written by
[`scripts/deploy.sh`](scripts/deploy.sh).

## Testnet

Deployed 2026-10-28 with `scripts/deploy.sh testnet all` (stellar-cli 28.1.0, rustc 1.98.1,
`wasm32v1-none`). The `pact` WASM was built from the source in the PR that added this file
(PR #55, branch `feat/metering-testnet-deploy-release`); rebuild with `scripts/build.sh` and compare
the hash.

| Contract | ID | WASM sha256 | Notes |
|---|---|---|---|
| `pact` | [`CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6`](https://stellar.expert/explorer/testnet/contract/CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6) | `748b5dbcc800d8cd3bdaf4b24555bfa8e3659cdefb60888791fb9a086b35d1a4` | Includes `close_meter` (P7-01) |
| `ReceiptAnchor` (Accensa v0.2.0) | [`CCLPJIYCSZXAK5HQNVOBO634YGATAL7ASE6HZRFQLKB3TJHMK7NCEDXZ`](https://stellar.expert/explorer/testnet/contract/CCLPJIYCSZXAK5HQNVOBO634YGATAL7ASE6HZRFQLKB3TJHMK7NCEDXZ) | `d7ec929b1f678d1f26f98dd803a566a44502ce7ddb890613612007166061f31e` | Vendored WASM (`vendor/wasm/`); hash matches `SHA256SUMS` |

**`pact` constructor arguments**

| Argument | Value |
|---|---|
| `admin` | `GD4BTAHYO7JBRTUVIB7DY7OLOTZZSDAE26YLRWPJ7OVQ3KLINK237UOT` (`pactwright-deployer`) |
| `token` | `CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA` (Circle testnet USDC SAC, issuer `GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5`) |
| `verifier` | `4efdaeb6c34ef1babbc844909afe576b288a36a2afe43a8bf6fcff82c9ffe4ff` (`pactwright-verifier`, `GBHP3LVWYNHPDOV3ZBCJBGX6K5VSRCRWUKX6IOUL636P7AWJ77SP7TDT`) |
| `max_pact_ledgers` | `120960` (~7 days) |

**`ReceiptAnchor`**: redeployed 2026-11-03 to rotate the anchorer key (the contract has no admin transfer).
Same vendored WASM (`d7ec929b…`, checked with `scripts/check_vendor_hashes.sh`), deployed by
`pactwright-anchorer-2` in [`ff49c410…`](https://stellar.expert/explorer/testnet/tx/ff49c410fc734d2da3286c4fb61967ebd55aac996a865c62bbcda167d6d24822)
and initialized in [`379cd299…`](https://stellar.expert/explorer/testnet/tx/379cd2995e4ebf9b60b6a076944d453f494ffce1b2a5ac68fb95b7e2bd99dae3):
`initialize(merchant = GBWCL7DDYV755SVX7PAAFZ66DZM3ODJRIAO5GWE42VXFP6FEUAPR5Y5F)`. Studio re-anchored all 42
receipts into its batch 1 ([`72017e67…`](https://stellar.expert/explorer/testnet/tx/72017e676a689905b48416a26d361445be7f9f15865e4efa0a1bdddf4c46054a)),
and a certificate verifies on-chain against it.

**Retired testnet deployments**

| Contract | ID | Why |
|---|---|---|
| `ReceiptAnchor` | `CAEO5GBC3ULRC5RYBSCSOXLNXHU2HZAHNW5HY5LZM3DIPFMX357ILNJ2` | 2026-11-03: the secret of its admin, `pactwright-anchorer` (`GAKQTGNQ…M4KZ`), was found stored in the wrong setting on the Studio host and printed in a maintainer session. Its 7 batches were re-anchored on the replacement; certificates citing them no longer verify. Do not use. |
| `ReceiptAnchor` | `CA2VU3DRTV7V2A5CL3WVFK7FIXEMTJSLVI2ERKZHR6MEWDZY3ZGZ3ITX` | 2026-10-28: the anchorer seed phrase was exposed in a maintainer session log. Admin key rotated; do not use. |
| `ReceiptAnchor` | `CCSST64R3DDWDTMK7ZJQY7KNC6ZJW6665B2L2HOW7LVIWZMKHGJFQ3K7` | 2026-10-28: redeployed during the rotation before the new key existed (same exposed admin). Do not use. |

The exposed anchorer account `GDBPV6OMFX5EJZFARLQ4DVYHQGI2M3S7TFSDKN4AD4OHB6PPNWMBZGIR` holds only testnet XLM and has no role in the current
deployment.

**Bench deployment** (C-21, `scripts/bench-testnet.sh`): bench `pact`
`CBWEAVYZD6SP3PA5V6ZGEBHBUYJENBAAG5CFBTB6FCYDYZGPYRZZQEQM` using the `PWBENCH` test token issued by
`pactwright-sponsor`. Each run deploys a fresh bench `pact`; never use the bench instances for
anything but measurement.

**Testnet identities** (F-11; secrets stay in the stellar CLI keystore of the maintainer who
created them, never in git):

| Identity | Address | Role |
|---|---|---|
| `pactwright-deployer` | `GD4BTAHYO7JBRTUVIB7DY7OLOTZZSDAE26YLRWPJ7OVQ3KLINK237UOT` | Deploys contracts; `pact` admin |
| `pactwright-sponsor` | `GAVTUPPEMLXAE2TFFPMPG43QTEV5UPKPFV55YQ4OUYVTE4NML7W6F2LU` | Sponsors agent account reserves (K-10) |
| `pactwright-verifier` | `GBHP3LVWYNHPDOV3ZBCJBGX6K5VSRCRWUKX6IOUL636P7AWJ77SP7TDT` | Signs verdict attestations |
| `pactwright-anchorer` | `GAKQTGNQFHVZDTPFW75J322QZYPSX7ULTFGEUFGDR5E4NFIHSSUVM4KZ` | `ReceiptAnchor` admin (S-04) |

Testnet is reset periodically; after a reset, re-fund the identities and re-run
`scripts/deploy.sh testnet all` (see [`docs/RUNBOOK-network-upgrade.md`](docs/RUNBOOK-network-upgrade.md)).

## Pubnet

None. Pubnet deployment follows the audit (ROADMAP P8-03); `scripts/deploy.sh pubnet` refuses
to run with placeholder inputs or without `PACTWRIGHT_CONFIRM_PUBNET=yes`.
