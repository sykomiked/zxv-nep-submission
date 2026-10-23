//! # zxv-core-kernel
//!
//! The **canonical reference implementation** of the
//! `near-panic-free` standard — the deployed `zxv-core` kernel.
//!
//! ## The functional role (clearly delineated for the committee)
//!
//! This crate is the **living testbed** of the standard: the exact
//! logic deployed on NEAR mainnet at
//! `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`
//! (the gate at `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8`).
//!
//! **It is NOT a mandatory dependency for NEP compliance.** The NEP
//! specifies the *interface and the behavior*; any contract may
//! implement the trait inline (Path A) or route to this public kernel
//! (Path B). This crate exists so reviewers can:
//!
//! 1. build the exact deployed logic reproducibly,
//! 2. compare the artifact checksum to the on-chain code hash,
//! 3. query the live gate via RPC view calls.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_panic_free::{conjoin_slice, is_settleable, invariant, NepPanicFreeResolution};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ZxvCore {
    /// The settled/held state of the gate (0x00-0x03).
    pub gate_state: u8,
}

#[near_bindgen]
impl NepPanicFreeResolution for ZxvCore {
    /// The chain-meet: join evidence states (O(1) per pair).
    fn nep_conjoin(&self, inputs: &[u8]) -> u8 {
        conjoin_slice(inputs)
    }

    /// Only TRUE (0x01) is settleable.
    fn nep_is_settleable(&self, state: u8) -> bool {
        is_settleable(state)
    }

    /// The invariant, self-describing.
    fn nep_invariant(&self) -> &'static str {
        invariant()
    }
}

#[near_bindgen]
impl ZxvCore {
    #[init]
    pub fn new() -> Self {
        Self { gate_state: 0 }
    }

    /// The canonical interface names (matching the deployed contract):
    /// `conjoin` and `is_settleable` (the deployed method names).
    pub fn conjoin(&self, inputs: Vec<u8>) -> u8 {
        conjoin_slice(&inputs)
    }

    pub fn is_settleable(&self, state: String) -> bool {
        state == "TRUE"
    }

    /// The gate state (gas-free view).
    pub fn gate(&self) -> u8 {
        self.gate_state
    }
}