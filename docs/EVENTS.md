# Events

Defined in `contracts/pact/src/events.rs` with `#[contractevent]`. Every event has the topics
`("pact", <name>)`; fields are the event's data. Indexers (Pactwright-studio S-02) decode these.

| Event (`<name>`) | Emitted by | Fields |
|---|---|---|
| `opened` | `open` | `pact_id: BytesN<32>`, `buyer: Address`, `seller: Address`, `cap: i128`, `condition_hash: BytesN<32>`, `metered: bool`, `silence: Silence`, `verdict_deadline: u32`, `settle_deadline: u32` |
| `verdict` | `record_verdict` | `pact_id`, `verdict: Verdict` (Allow = 1, Deny = 2), `amount: i128`, `envelope_hash: BytesN<32>` |
| `settled` | `settle` | `pact_id`, `to_seller: i128`, `to_buyer: i128` |
| `meter_closed` | `close_meter` | `pact_id`, `units: u32`, `to_seller: i128`, `to_buyer: i128` |
| `released` | `release` | `pact_id`, `reason: ReleaseReason` (Denied = 1, Silence = 2, Deadline = 3), `to_buyer: i128` |
| `verifier_set` | `set_verifier` | `key: BytesN<32>` |

Amounts are stroops of the configured token (USDC SAC, 7 decimals). Exactly one of `settled`,
`meter_closed` or `released` is emitted per pact (invariant 1), and its amounts sum to `cap`
(invariant 2). Topics are a public interface: changing one is a breaking change.
