# Security model: `pact` contract

Expands ARCHITECTURE.md §9 for the on-chain part. Report vulnerabilities as described in
SECURITY.md.

## Assets and trust

| Party | Trusted for | Not trusted for |
|---|---|---|
| Buyer | Signing `open` (escrow) and, for metered pacts, usage vouchers | Anything after funding; cannot take back an escrow outside the release rules |
| Seller | Calling `settle` / `close_meter` for itself | Choosing an amount above the attested or metered amount |
| Verifier (one key per pact, snapshotted at `open`) | The ALLOW/DENY verdict and owed amount | Moving funds; it only signs attestations |
| Admin | Rotating the verifier key for **new** pacts | Moving funds or changing any open pact (invariant 4) |
| Anyone | Calling `record_verdict` with a valid attestation, `release` when allowed, `extend_pact_ttl` | Directing funds anywhere but the buyer or seller |

## Authorization by entrypoint

| Entrypoint | Auth | Funds can move to |
|---|---|---|
| `open` | buyer | escrow (contract) |
| `record_verdict` | none: ed25519 signature by the pact's verifier key | nowhere |
| `settle` | seller | seller `actual`, buyer the rest |
| `close_meter` | seller + buyer's meter-key signature over the voucher | seller `units × unit_price`, buyer the rest |
| `release` | none | buyer only |
| `set_verifier` | admin | nowhere |
| `extend_pact_ttl`, reads | none | nowhere |

## Threats and controls

| Threat | Control |
|---|---|
| Forged or replayed verdict | ed25519 over `sha256("pactwright-verdict-v1:" ‖ xdr(attestation))`; attestation bound to `pact_id`, `condition_hash`, and an `issued_ledger` inside the pact's life. Distinct domain prefixes (verdict, usage, delivery, receipt) prevent cross-use. |
| Two conflicting verdicts | `record_verdict` requires `Funded`, so only the first valid one lands (invariant 5); the verifier also signs at most one per pact. |
| Verifier key compromise | Exposure limited to open pacts until their `verdict_deadline`; `set_verifier` protects new pacts. Short verdict windows are the mitigation. |
| Seller overcharging | `settle` caps `actual` at the attested amount; `close_meter` caps at `units × unit_price ≤ cap`. |
| Seller or verifier disappears | `release` after the silence or settle deadline returns the escrow to the buyer; no funds can be stranded (ADR-011). |
| Archived storage with funds inside | `open` extends the pact entry's TTL past `settle_deadline + GRACE_LEDGERS` and rejects deadlines beyond `max_ttl` (ADR-009). |
| Arithmetic overflow | `units × unit_price` uses `checked_mul`; amounts are `i128`; release-profile `overflow-checks = true`. |
| Malicious token | The token is fixed in `Config` at construction (USDC SAC); pacts cannot choose another token. |
| Front-running `open` with someone else's `pact_id` | `pact_id` hashes the buyer's address, and `open` requires the buyer's auth. |
| Self-dealing | `open` rejects `buyer == seller`. |

## Known residual risks

- `release` cannot pay a buyer whose USDC trustline is frozen by the issuer; the funds stay in
  escrow until the freeze is lifted.
- The admin cannot be changed in the current contract; losing the admin key only blocks
  verifier rotation, never funds.
- M0 (exact-refund) protection depends on the seller's `RefundVault` and is outside this contract.
- The contract is **unaudited** (ROADMAP P8-01); it is deployed on testnet only.
