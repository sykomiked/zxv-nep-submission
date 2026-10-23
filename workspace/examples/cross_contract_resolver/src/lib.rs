//! # cross_contract_resolver — Path B
//!
//! **Integration Path B: the canonical public utility.**
//!
//! The contract queries the shared `zedec.near` / `zxv-core` contract
//! via a cross-contract promise or a view call. The contract keeps its
//! size minimal (it does not embed the logic gate); the public kernel
//! is the shared dispute utility.
//!
//! The deployed kernel is an **open, public utility and verification
//! anchor** — not a mandatory dependency for NEP compliance. Any
//! contract may implement the interface inline (Path A) instead. This
//! keeps the standard neutral; the kernel is the canonical benchmark.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

/// The canonical public kernel (the deployed zxv-core).
pub const KERNEL_CONTRACT: &str =
    "f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf";

/// The canonical router.
pub const CANONICAL_ROUTER: &str = "zedec.near";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CrossContractResolver {
    pub state: u8,
}

#[near_bindgen]
impl CrossContractResolver {
    #[init]
    pub fn new() -> Self {
        Self { state: 0 }
    }

    /// Route the resolution to the shared public kernel.
    ///
    /// Note: in production, use a cross-contract promise and store the
    /// callback. The `env::promise_then` pattern is:
    ///
    /// ```ignore
    /// let args = near_sdk::serde_json::json!({ "inputs": evidence }).to_string().into_bytes();
    /// let p = env::promise_create(KERNEL_CONTRACT.into(), "conjoin", args, 0, GAS);
    /// env::promise_then(p, env::current_account_id(), "on_conjoin", b"{}", 0, GAS);
    /// ```
    ///
    /// This example shows the *view-call* form (off-chain / indexer),
    /// which is gas-free.
    pub fn resolve_via_kernel(&self, evidence: Vec<u8>) -> u8 {
        // Off-chain form: a gas-free view call against the public
        // kernel. (The on-chain form uses the promise pattern above.)
        // Placeholder for the RPC helper; the important point is the
        // routing: the public kernel is the shared dispute utility.
        let r = near_panic_free::conjoin_slice(&evidence);
        self.state = r;
        r
    }

    pub fn kernel_address(&self) -> String {
        KERNEL_CONTRACT.to_string()
    }

    pub fn canonical_router(&self) -> String {
        CANONICAL_ROUTER.to_string()
    }

    pub fn state(&self) -> u8 {
        self.state
    }
}