# Security Policy

Pactwright handles escrow and financial settlement for AI-agent transactions. We treat security as our highest priority. Please report any potential vulnerabilities privately.

## Reporting a Vulnerability

- **Do not** open a public issue or discuss vulnerabilities in public forums.
- Report vulnerabilities privately through GitHub's Security Advisories:
  <https://github.com/Pactwright/Pactwright-contracts/security/advisories/new>
- If you cannot use GitHub Security Advisories, contact the core team directly at `security@pactwright.org`.
- Include in your report:
  - Affected smart contracts, functions, or scripts.
  - Step-by-step reproduction instructions or a test case.
  - Potential impact assessment.
  - Recommended fix or mitigation, if available.

We acknowledge reports within 72 hours and provide status updates as triage proceeds.

## Audit Status Disclaimer

> **Pre-release / Testnet Only**: The smart contracts in this repository are currently deployed on the Stellar testnet for evaluation and integration testing. They have **not yet undergone an independent formal security audit**. Do not deploy or use these contracts on Stellar mainnet (pubnet) with real funds until formal audits (ROADMAP tasks P8-01 and P8-02) have been completed and verified.

## Core Security Invariants

In accordance with `ARCHITECTURE_ESSENTIALS.md`:
1. **No admin backdoors**: There is no administrative mechanism or backdoor capable of draining pact funds or overriding verifier decisions.
2. **Fail-closed settlement**: Funds are only released to a seller upon a valid, verified cryptographic attestation. If deadlines lapse without verification, exact refunds execute automatically.
3. **PR Security Guard**: All pull requests are scanned against Trojan Source (CVE-2021-42576), invisible Unicode overrides, shell injections, and untrusted binaries via `.github/workflows/pr-malware-guard.yml`.
