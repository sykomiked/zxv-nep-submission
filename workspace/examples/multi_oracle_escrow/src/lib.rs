//! # multi_oracle_escrow
//!
//! The reference `near-sdk` contract for the NEP: a multi-oracle
//! dispute escrow. **Path A: the inline library** — the resolution
//! logic is compiled directly into this contract's WASM (zero
//! cross-contract overhead, nanoseconds, a single comparison).
//!
//! The key demonstration: where a classical contract would call
//! `env::panic_str()` on an oracle mismatch, this contract **holds
//! the `BOTH` state**, preserves the receipt hash, and emits the
//! NEP-297 event — no revert, no gas burn, no lost evidence.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_panic_free::{conjoin_slice, is_settleable, NepPanicFreeResolution, BOTH, TRUE};
use near_panic_free_sdk::contention_event;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DisputeEscrow {
    pub pending_state: u8,
    pub contention_log: Vec<String>,
}

#[near_bindgen]
impl NepPanicFreeResolution for DisputeEscrow {
    fn nep_conjoin(&self, inputs: &[u8]) -> u8 {
        conjoin_slice(inputs)
    }

    fn nep_is_settleable(&self, state: u8) -> bool {
        is_settleable(state)
    }

    fn nep_invariant(&self) -> &'static str {
        "Only TRUE settles. TRUE+FALSE=BOTH: Contradictions are held, never coerced."
    }
}

#[near_bindgen]
impl DisputeEscrow {
    #[init]
    pub fn new() -> Self {
        Self {
            pending_state: 0,
            contention_log: Vec::new(),
        }
    }

    /// Submit oracle votes for the escrow condition.
    ///
    /// - All votes agree  -> `TRUE`  -> settlement may execute.
    /// - Votes conflict   -> `BOTH`  -> held: the receipt is kept, the
    ///   NEP-297 event is emitted, and NO revert happens.
    /// - Any vote is no   -> `FALSE` -> the escrow refuses.
    pub fn submit_oracle_assertion(&mut self, oracle_votes: Vec<u8>, receipt_hash: String) {
        let resolved = conjoin_slice(&oracle_votes);
        self.pending_state = resolved;

        if is_settleable(resolved) {
            // Unanimous settlement
            env::log_str("Settlement executed: State is TRUE");
        } else if resolved == BOTH {
            // Contradiction preserved without reverting
            self.contention_log.push(receipt_hash.clone());
            env::log_str(&contention_event(&receipt_hash));
        } else {
            // FALSE (or NEITHER): refused / waiting
            env::log_str(&format!("Escrow state held: {}", resolved));
        }
    }

    /// View the pending state (gas-free).
    pub fn pending(&self) -> u8 {
        self.pending_state
    }

    /// View the retained contention evidence (gas-free).
    pub fn contention(&self) -> Vec<String> {
        self.contention_log.clone()
    }

    /// View the invariant (gas-free).
    pub fn invariant(&self) -> &'static str {
        "Only TRUE settles. TRUE+FALSE=BOTH: Contradictions are held, never coerced."
    }

    // The escrow release is intentionally not shown: the point of the
    // example is the RESOLUTION LOGIC. A real escrow would gate
    // `ft_transfer` / `near_transfer` on `is_settleable(self.pending_state)`.
}