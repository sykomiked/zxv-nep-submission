//! # inline_resolver — Path A
//!
//! **Integration Path A: the inline library.**
//!
//! The contract imports `near-panic-free` directly into its binary.
//! Resolution executes in the exact same WASM execution frame, in
//! nanoseconds, via a single comparison — zero cross-contract
//! overhead, zero contract-size increase beyond the ~1KB logic core.
//!
//! Use this when your contract wants the logic embedded and
//! self-contained (no external dependency at runtime).
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_panic_free::{conjoin_slice, is_settleable, BOTH};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct InlineResolver {
    pub state: u8,
}

#[near_bindgen]
impl InlineResolver {
    #[init]
    pub fn new() -> Self {
        Self { state: 0 }
    }

    /// Resolve inline (same-frame, O(n), zero cross-contract calls).
    pub fn resolve(&mut self, evidence: Vec<u8>) -> u8 {
        let r = conjoin_slice(&evidence);
        self.state = r;
        if is_settleable(r) {
            env::log_str("resolved TRUE — side effects authorized");
        } else if r == BOTH {
            env::log_str("held BOTH — side effects blocked, no revert");
        }
        r
    }

    pub fn state(&self) -> u8 {
        self.state
    }
}