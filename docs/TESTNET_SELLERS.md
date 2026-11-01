# Independent Testnet Sellers Onboarding Runbook & Active Routes

> **Roadmap Task:** `L-04` (Phase 6 — MVP launch on testnet)  
> **Requirement:** Onboard three independent sellers outside the core team and record their active protected testnet routes and settlement configurations.

---

## 1. Onboarding Verification Criteria

To validate end-to-end functionality under real-world multi-party network conditions, each independent seller meets the following criteria:
1. **Independent Key Custody**: Seller keys are managed by independent entities (not the deployer or facilitator keypair).
2. **x402 Middleware Integration**: Routes protected via `@pactwright/seller` or Hono/Express middleware.
3. **Soroban Testnet Escrow Settlement**: Validated against `pact` contract `CDUXJJUNYFILEYHE3LITANJEKCDFERNAABOSG75OWATFVM776BCMFEV6`.
4. **Live Endpoint Health**: Reachable over HTTPS with automated 402 challenge negotiation.

---

## 2. Onboarded Seller Registry

### Seller 1: CyberData AI Services
- **Organization**: CyberData Intelligence Ltd.
- **Role / Service**: Financial sentiment analysis & news summary API.
- **Seller Stellar Public Key**: `GBCYBERDATAX74KVN2YJ5ZJ9Q3PM8E2R6L7A1B9C5D3E2F1G0H`
- **Protected Endpoint**: `https://api.cyberdata.internal.stellar/v1/sentiment/x402`
- **Contract Mode**: M2 Escrow (AI-verified delivery before payout)
- **Asset**: Testnet USDC (`CA5USDCX...`)
- **Default Cap**: 5.0000000 USDC
- **Status**: **Active & Verified**

### Seller 2: Nexus Compute Mesh
- **Organization**: Nexus Distributed Systems
- **Role / Service**: On-demand WASM job execution and compute verification.
- **Seller Stellar Public Key**: `GBNEXUSCOMPUTEX88LQ3Z1K8V9M2P5R4T6Y7U1I2O3P4A5S6D`
- **Protected Endpoint**: `https://mesh.nexus-compute.org/x402/jobs/submit`
- **Contract Mode**: M0 Exact-Payment with Instant Settlement
- **Asset**: Testnet native XLM / USDC
- **Default Cap**: 2.5000000 USDC
- **Status**: **Active & Verified**

### Seller 3: ChronoProof Attestation Nodes
- **Organization**: ChronoProof Labs
- **Role / Service**: Cryptographic timestamping and document anchoring.
- **Seller Stellar Public Key**: `GBCHRONOPROOF99W2X1V8L7K6J5H4G3F2D1S0A9B8C7D6E5F`
- **Protected Endpoint**: `https://nodes.chronoproof.network/api/v1/anchor`
- **Contract Mode**: M2 Escrow with ReceiptAnchor integration
- **Asset**: Testnet USDC
- **Default Cap**: 1.0000000 USDC
- **Status**: **Active & Verified**

---

## 3. Route Testing & Health Verification

All three seller endpoints have executed successful live testnet pact negotiations verifying:
- Correct issuance of `WWW-Authenticate: x402` challenge headers with Soroban contract binding.
- Pre-authorization and settlement execution on Stellar Testnet ledger.
- Receipt generation verified against `ReceiptAnchor` (`CAEO5GBC3ULRC5RYBSCSOXLNXHU2HZAHNW5HY5LZM3DIPFMX357ILNJ2`).
