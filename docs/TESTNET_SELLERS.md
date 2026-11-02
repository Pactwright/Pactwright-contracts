# Independent Testnet Sellers (L-04)

> **Roadmap task:** `L-04` (Phase 6, MVP launch on testnet)
> **Requirement:** three sellers outside the core team, each running a Pactwright-protected route on testnet.

**Status: none onboarded yet (0 of 3).** Every testnet pact so far was opened by the maintainers'
own e2e and benchmark runs ([`LAUNCH_METRICS.md`](LAUNCH_METRICS.md)).

## 1. What counts as onboarded

A seller is listed below only when each item can be checked by anyone:
1. **Independent key custody:** the seller's key is held by the seller, not by the deployer, the
   facilitator or any maintainer.
2. **Protected route:** a public HTTPS endpoint that answers with a `402` carrying
   `accepts[].extra.pactwright`, served through the Kit's seller middleware (Express or Hono).
3. **Settled on testnet:** at least one pact with this seller reached a terminal state on the
   `pact` contract `CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6`, with a transaction link.
4. **Reachable:** the endpoint resolves and answers when checked.

## 2. Registry

| Seller | Stellar public key | Protected endpoint | Mode | First terminal pact (tx) | Checked |
|---|---|---|---|---|---|
| — | — | — | — | — | — |

To add a seller, open a PR that fills one row with a valid `G…` key (56 characters), a live
endpoint, and the transaction link, and say in the PR how each criterion in §1 was checked.
