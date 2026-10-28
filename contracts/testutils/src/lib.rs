//! Shared fixtures: token setup, ledger helpers, keys (ROADMAP C-09).

use soroban_sdk::Env;

/// A fresh test environment with all auths mocked. Extend in C-09.
pub fn env() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}
