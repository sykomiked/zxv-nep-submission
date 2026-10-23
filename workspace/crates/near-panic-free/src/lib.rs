#![no_std]

//! # near-panic-free
//!
//! The reference implementation of the NEAR NEP: **Standard Interface
//! for Deterministic Panic-Free State Resolution (Chain-Meet)**.
//!
//! A pure `#![no_std]`, **zero-dependency** core: the 1-byte
//! four-valued state machine, the chain-meet resolution, and the
//! structural invariant "only TRUE settles".
//!
//! | State    | Value | Meaning              | Side-effects        |
//! |----------|-------|----------------------|---------------------|
//! | NEITHER  | 0x00  | no evidence yet      | blocked             |
//! | TRUE     | 0x01  | agreed, attested     | **allowed**         |
//! | FALSE    | 0x02  | explicitly refuted   | blocked             |
//! | BOTH     | 0x03  | contradiction, held  | blocked, not reverted |
//!
//! The total order is `FALSE < NEITHER < BOTH < TRUE`. Because TRUE is
//! the top, **no combination of non-TRUE values can ever yield TRUE** —
//! the invariant is structural, not checked.
//!
//! Mirrors the live mainnet implementation at `zedec.near` /
//! `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

pub use crate::traits::NepPanicFreeResolution;

/// The four states of the standard (byte values).
pub const NEITHER: u8 = 0x00;
pub const TRUE: u8 = 0x01;
pub const FALSE: u8 = 0x02;
pub const BOTH: u8 = 0x03;

/// Total order: FALSE (0) < NEITHER (1) < BOTH (2) < TRUE (3).
///
/// Indexed by state value: `RANK[NEITHER]=1`, `RANK[TRUE]=3`,
/// `RANK[FALSE]=0`, `RANK[BOTH]=2`.
pub const RANK: [u8; 4] = [1, 3, 0, 2];

/// The chain-meet of two states, O(1).
///
/// Returns the state with the smaller rank. Out-of-range inputs are
/// fail-closed to NEITHER (an unknown phase is never authority).
#[inline(always)]
pub fn conjoin_pair(a: u8, b: u8) -> u8 {
    if a > 3 || b > 3 {
        return NEITHER; // Fail-closed: unmapped phases cannot authorize
    }
    if RANK[b as usize] < RANK[a as usize] {
        b
    } else {
        a
    }
}

/// The chain-meet over a slice of evidence states, O(n).
///
/// Seeds with the first input (NEITHER is not an identity for the
/// min-rank meet). Any out-of-range input fails closed to NEITHER.
#[inline(always)]
pub fn conjoin_slice(inputs: &[u8]) -> u8 {
    let mut it = inputs.iter();
    let mut state = match it.next() {
        Some(&v) if v <= 3 => v,
        _ => return NEITHER,
    };
    for &v in it {
        if v > 3 {
            return NEITHER;
        }
        if RANK[v as usize] < RANK[state as usize] {
            state = v;
        }
    }
    state
}

/// Only TRUE (0x01) is settleable.
#[inline(always)]
pub fn is_settleable(state: u8) -> bool {
    state == TRUE
}

/// The invariant, self-describing.
#[inline(always)]
pub fn invariant() -> &'static str {
    "Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic."
}

mod traits {
    /// The standard interface: deterministic panic-free state
    /// resolution. NEAR SDK convention: `&self` receivers so contract
    /// implementations and off-chain macro bindings generate uniform
    /// view-call endpoints.
    pub trait NepPanicFreeResolution {
        /// Conjoin evidence states; the chain-meet (O(1) per pair).
        /// Returns 0x00-0x03. Out-of-range inputs are fail-closed to
        /// NEITHER.
        fn nep_conjoin(&self, inputs: &[u8]) -> u8;

        /// Only TRUE (0x01) is settleable.
        fn nep_is_settleable(&self, state: u8) -> bool;

        /// The invariant, self-describing.
        fn nep_invariant(&self) -> &'static str;
    }
}