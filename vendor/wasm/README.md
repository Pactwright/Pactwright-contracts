# Vendored Accensa WASMs (ADR-008)

Pinned, hash-verified builds of Accensa contracts that Pactwright calls or tests against.
`scripts/check_vendor_hashes.sh` (CI: `vendor-hashes.yml`) fails if any byte changes or an
unlisted `.wasm` appears.

| File | Source | Version | sha256 |
|---|---|---|---|
| `receipt_anchor.wasm` | accensa/accensa-contracts `contracts/receipt-anchor` | v0.2.0 (`50ced70`) | `d7ec929b…61f31e` |
| `refund_vault.wasm` | accensa/accensa-contracts `contracts/refund-vault` | v0.2.0 (`50ced70`) | `2be8571d…cbc374e` |

Full hashes: `SHA256SUMS`.

## Reproduce

```bash
git clone --depth 1 --branch v0.2.0 https://github.com/accensa/accensa-contracts.git
cd accensa-contracts
stellar contract build --locked          # stellar-cli 28.1.0, rustc 1.98.1
shasum -a 256 target/wasm32v1-none/release/{receipt_anchor,refund_vault}.wasm
```

## Interface notes (v0.2.0)

v0.2.0 is the latest tagged release. Its interface differs from Accensa's unreleased `main`:

- `ReceiptAnchor`: `initialize(merchant)`, `anchor_batch(root, count, period_start, period_end) -> batch_id`,
  `verify_receipt(batch_id, leaf, proof) -> bool`, max 1000 leaves per batch. **No shards.**
- `RefundVault`: `initialize(merchant, token, refund_window_ledgers)`, `deposit(from, amount)`,
  `refund(payment_ref, recipient, amount, paid_at_ledger)`. Refunds require the merchant's auth
  (no refund-only role) and reject a second refund for the same `payment_ref`.

Upgrading to a later Accensa release is a separate change: rebuild, update `SHA256SUMS`, this
table, and any caller that depends on the interface.
