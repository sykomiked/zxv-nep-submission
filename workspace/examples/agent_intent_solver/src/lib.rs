//! # agent_intent_solver
//!
//! The NEAR-2026 example: an autonomous **intent solver** that uses
//! `near-panic-free` to hold a dispute **inside the escrow** instead
//! of exploding the transaction.
//!
//! Scenario: an intent "swap X for Y if the price condition holds" is
//! routed to two solver networks. Solver A's proof asserts TRUE;
//! solver B's proof asserts FALSE. A classical contract panics or
//! race-conditions. This solver **conjoins the proofs to BOTH** — the
//! funds stay locked, the receipt hashes are retained, and an
//! arbitration oracle can resolve later. No gas-draining panic loops.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_panic_free::{conjoin_slice, is_settleable, BOTH, FALSE, NEITHER, TRUE};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct IntentSolver {
    /// The intent id (the escrow key).
    pub intent_id: String,
    /// The resolution state of the price condition.
    pub condition_state: u8,
    /// The retained dispute evidence (solver proof hashes).
    pub dispute_log: Vec<String>,
    /// Whether settlement has been unlocked.
    pub settled: bool,
}

#[near_bindgen]
impl IntentSolver {
    #[init]
    pub fn new(intent_id: String) -> Self {
        Self {
            intent_id,
            condition_state: NEITHER,
            dispute_log: Vec::new(),
            settled: false,
        }
    }

    /// A solver network submits its proof about the price condition.
    ///
    /// Deterministic intent conjoinment:
    /// - all solvers agree  -> TRUE  -> settlement unlocked.
    /// - solvers dispute    -> BOTH  -> funds stay locked; the dispute
    ///   is held, not reverted; arbitration can resolve later.
    pub fn submit_solver_proof(&mut self, proof_hash: String, asserts: u8) {
        // assert a valid state
        let asserted = if asserts > 3 { NEITHER } else { asserts };
        let resolved = conjoin_slice(&[self.condition_state, asserted]);
        self.condition_state = resolved;

        if is_settleable(resolved) {
            self.settled = true;
            env::log_str("INTENT SETTLED: condition is TRUE");
        } else if resolved == BOTH {
            self.dispute_log.push(proof_hash);
            env::log_str(&format!(
                "EVENT_JSON:{{\"standard\":\"nep0000\",\"version\":\"1.0.0\",\"event\":\"contention_held\",\"data\":[{{\"state\":3,\"state_name\":\"BOTH\",\"receipt_hash\":\"{}\"}}]}}",
                proof_hash
            ));
        } else if resolved == FALSE {
            env::log_str("INTENT REFUSED: condition is FALSE");
        }
        // NEITHER: still waiting for evidence
    }

    /// Arbitration resolves the held dispute.
    pub fn arbitrate(&mut self, verdict: u8) {
        let v = if verdict > 3 { NEITHER } else { verdict };
        // an arbitrator's verdict joins the held state
        let resolved = conjoin_slice(&[self.condition_state, v]);
        self.condition_state = resolved;
        if is_settleable(resolved) {
            self.settled = true;
            env::log_str("ARBITRATION: settled TRUE");
        }
    }

    pub fn condition(&self) -> u8 {
        self.condition_state
    }

    pub fn is_settled(&self) -> bool {
        self.settled
    }

    pub fn disputes(&self) -> Vec<String> {
        self.dispute_log.clone()
    }
}