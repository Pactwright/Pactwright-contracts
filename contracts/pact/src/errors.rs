//! Contract errors. ARCHITECTURE.md §4.7. Panics only via `panic_with_error!`.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyExists = 2,
    PactNotFound = 3,
    InvalidState = 4,
    InvalidAmount = 5,
    SelfDealing = 6,
    BadDeadlines = 7,
    VerdictWindowClosed = 8,
    SettleWindowClosed = 9,
    NotReleasable = 10,
    AttestationMismatch = 11,
    /// Reserved: `ed25519_verify` traps on a bad signature rather than returning.
    BadSignature = 12,
    NotMetered = 13,
    IsMetered = 14,
    Unauthorized = 15,
}
