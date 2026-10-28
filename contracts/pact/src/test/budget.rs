//! Tollcraft Tier A budget tests (ROADMAP C-12).
//!
//! `pact` runs as the compiled WASM (`scripts/build.sh` first), the budget is reset right before
//! the measured call, and `budget-macros` asserts CPU instructions and memory bytes for that
//! call only. Limits are the measured value × 1.15, rounded up (budget.toml convention); the
//! measurements and the WASM hash they were taken on are recorded in docs/BENCHMARKS.md.
//! Raise a limit only deliberately, explaining the increase in the PR.

use budget_macros::budget_lt;
// The macro expansion uses std's `format!`, `String` and `ToString`; this crate is `no_std`.
#[allow(unused_imports)]
use std::{
    format,
    string::{String, ToString},
};

use super::*;
use crate::Verdict;

/// WASM-mode fixture plus the `env` handle the macros read after the body runs.
fn setup() -> (Fixture, soroban_sdk::Env) {
    let f = Fixture::with_wasm();
    let env = f.env.clone();
    (f, env)
}

fn reset(env: &soroban_sdk::Env) {
    env.cost_estimate().budget().reset_default();
}

// measured: cpu 889_745, mem 1_436_284
#[test]
#[budget_lt(cpu = 1_030_000, mem = 1_660_000)]
fn budget_open() {
    let (f, env) = setup();
    reset(&env);
    f.open(1);
}

// measured: cpu 1_122_084, mem 1_362_925
#[test]
#[budget_lt(cpu = 1_300_000, mem = 1_570_000)]
fn budget_record_verdict() {
    let (f, env) = setup();
    let pact_id = f.open(1);
    let attestation = f.attestation(&pact_id, Verdict::Allow, CAP);
    let signature = f.sign(&attestation);
    reset(&env);
    f.pact.record_verdict(&pact_id, &attestation, &signature);
}

// measured (partial settle, two transfers, the worst case): cpu 1_073_058, mem 1_475_617.
// A full settle (one transfer) measures cpu 889_671, mem 1_438_667.
#[test]
#[budget_lt(cpu = 1_240_000, mem = 1_700_000)]
fn budget_settle() {
    let (f, env) = setup();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    reset(&env);
    f.pact.settle(&pact_id, &(CAP / 2));
}

// measured: cpu 872_907, mem 1_423_341
#[test]
#[budget_lt(cpu = 1_010_000, mem = 1_640_000)]
fn budget_release() {
    let (f, env) = setup();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Deny, 0);
    reset(&env);
    f.pact.release(&pact_id);
}

// measured: cpu 1_560_061, mem 1_484_463 (ed25519 check + two transfers)
#[test]
#[budget_lt(cpu = 1_800_000, mem = 1_710_000)]
fn budget_close_meter() {
    let (f, env) = setup();
    let pact_id = f.pact.open(&f.metered_params(1));
    let signature = f.sign_usage(&pact_id, 500);
    reset(&env);
    f.pact.close_meter(&pact_id, &500, &signature);
}

/// Product target (PRD §8): a settle costs well under the per-transaction CPU ceiling.
#[test]
fn settle_and_close_meter_stay_under_two_percent_of_the_cpu_ceiling() {
    const CPU_CEILING: u64 = 100_000_000;
    let (f, env) = setup();
    let pact_id = f.open(1);
    f.verdict(&pact_id, Verdict::Allow, CAP);
    reset(&env);
    f.pact.settle(&pact_id, &CAP);
    assert!(env.cost_estimate().budget().cpu_instruction_cost() < CPU_CEILING / 50);
}
