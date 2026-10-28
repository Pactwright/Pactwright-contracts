//! Test suites and the shared fixture.
extern crate std;

mod budget;
mod deadlines;
mod fuzz;
mod integration;
mod invariants;
mod lifecycle;
mod vectors;

use ed25519_dalek::{Signer, SigningKey};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{StellarAssetClient, TokenClient},
    Address, BytesN, Env,
};

use crate::{
    hashing, MeterTerms, Metering, OpenParams, PactContract, PactContractClient, Silence, Verdict,
    VerdictAttestation,
};

pub const START_LEDGER: u32 = 1_000;
pub const CAP: i128 = 10_000_000; // 1 USDC in stroops
pub const FUNDS: i128 = 100 * CAP;
pub const MAX_PACT_LEDGERS: u32 = 100_000;
pub const VERDICT_LEDGERS: u32 = 120;
pub const SETTLE_LEDGERS: u32 = 720;

pub fn signing_key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

pub fn public_key(env: &Env, key: &SigningKey) -> BytesN<32> {
    BytesN::from_array(env, &key.verifying_key().to_bytes())
}

pub struct Fixture {
    pub env: Env,
    pub admin: Address,
    pub buyer: Address,
    pub seller: Address,
    pub token: TokenClient<'static>,
    pub pact: PactContractClient<'static>,
    pub verifier: SigningKey,
}

impl Fixture {
    pub fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().set_sequence_number(START_LEDGER);

        let admin = Address::generate(&env);
        let buyer = Address::generate(&env);
        let seller = Address::generate(&env);
        let issuer = Address::generate(&env);

        let sac = env.register_stellar_asset_contract_v2(issuer);
        let token_address = sac.address();
        StellarAssetClient::new(&env, &token_address).mint(&buyer, &FUNDS);

        let verifier = signing_key(7);
        let contract_id = env.register(
            PactContract,
            (
                admin.clone(),
                token_address.clone(),
                public_key(&env, &verifier),
                MAX_PACT_LEDGERS,
            ),
        );

        let token = TokenClient::new(&env, &token_address);
        let pact = PactContractClient::new(&env, &contract_id);
        Fixture {
            env,
            admin,
            buyer,
            seller,
            token,
            pact,
            verifier,
        }
    }

    pub fn now(&self) -> u32 {
        self.env.ledger().sequence()
    }

    pub fn advance_to(&self, ledger: u32) {
        self.env.ledger().set_sequence_number(ledger);
    }

    pub fn condition_hash(&self, seed: u8) -> BytesN<32> {
        BytesN::from_array(&self.env, &[seed; 32])
    }

    /// Default escrow pact params: verdict-gated, silence = Refund.
    pub fn params(&self, nonce: u64) -> OpenParams {
        OpenParams {
            buyer: self.buyer.clone(),
            seller: self.seller.clone(),
            cap: CAP,
            nonce,
            condition_hash: self.condition_hash(1),
            meter: Metering::None,
            silence: Silence::Refund,
            verdict_deadline: self.now() + VERDICT_LEDGERS,
            settle_deadline: self.now() + SETTLE_LEDGERS,
        }
    }

    pub fn metered_params(&self, nonce: u64) -> OpenParams {
        let mut params = self.params(nonce);
        params.meter = Metering::Units(MeterTerms {
            unit_price: 1_000,
            meter_key: public_key(&self.env, &signing_key(9)),
        });
        params
    }

    pub fn open(&self, nonce: u64) -> BytesN<32> {
        self.pact.open(&self.params(nonce))
    }

    pub fn attestation(
        &self,
        pact_id: &BytesN<32>,
        verdict: Verdict,
        amount: i128,
    ) -> VerdictAttestation {
        VerdictAttestation {
            pact_id: pact_id.clone(),
            condition_hash: self.condition_hash(1),
            envelope_hash: BytesN::from_array(&self.env, &[0xee; 32]),
            verdict,
            amount,
            issued_ledger: self.now(),
        }
    }

    pub fn sign_with(&self, key: &SigningKey, attestation: &VerdictAttestation) -> BytesN<64> {
        let digest = hashing::verdict_message(&self.env, attestation).to_array();
        BytesN::from_array(&self.env, &key.sign(&digest).to_bytes())
    }

    pub fn sign(&self, attestation: &VerdictAttestation) -> BytesN<64> {
        self.sign_with(&self.verifier, attestation)
    }

    /// Records a correctly signed verdict.
    pub fn verdict(&self, pact_id: &BytesN<32>, verdict: Verdict, amount: i128) {
        let attestation = self.attestation(pact_id, verdict, amount);
        let signature = self.sign(&attestation);
        self.pact.record_verdict(pact_id, &attestation, &signature);
    }

    /// The buyer's meter-key signature over `UsageVoucher { pact_id, units }`.
    pub fn sign_usage(&self, pact_id: &BytesN<32>, units: u32) -> BytesN<64> {
        let voucher = crate::UsageVoucher {
            pact_id: pact_id.clone(),
            units,
        };
        let digest = hashing::usage_message(&self.env, &voucher).to_array();
        BytesN::from_array(&self.env, &signing_key(9).sign(&digest).to_bytes())
    }

    pub fn balance(&self, who: &Address) -> i128 {
        self.token.balance(who)
    }

    pub fn escrow_balance(&self) -> i128 {
        self.token.balance(&self.pact.address)
    }
}
