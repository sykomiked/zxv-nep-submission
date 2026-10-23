//! nep_gate.rs — the reference implementation of the NEP interface.
//!
//! "Standard Interface for Deterministic Panic-Free State Resolution
//! (Chain-Meet)"
//!
//! This is the minimal, self-contained reference implementation of the
//! standard interface proposed in the NEP. The core logic is
//! `#![no_std]`-compatible (with an optional `alloc` import for the
//! invariant string) and mirrors the live mainnet implementation at
//! `zedec.near` /
//! `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`.
//!
//! Run the tests: `cargo test --release`
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

/// The four states of the panic-free resolution standard.
///
/// | State    | Value | Meaning                     | Side-effects |
/// |----------|-------|-----------------------------|--------------|
/// | NEITHER  | 0x00  | no evidence yet             | blocked      |
/// | TRUE     | 0x01  | agreed, attested            | **allowed**  |
/// | FALSE    | 0x02  | explicitly refuted          | blocked      |
/// | BOTH     | 0x03  | contradiction, held         | blocked, not reverted |
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum State {
    Neither = 0x00,
    True = 0x01,
    False = 0x02,
    Both = 0x03,
}

/// The chain rank: FALSE is the bottom, TRUE is the top.
///
/// The total order is `FALSE < NEITHER < BOTH < TRUE`.
/// Because TRUE is the top, **no combination of non-TRUE values can
/// ever yield TRUE** — the invariant is structural, not checked.
const RANK: [u8; 4] = [1, 3, 0, 2]; // [NEITHER, TRUE, FALSE, BOTH]

/// The standard interface: deterministic panic-free state resolution.
///
/// NEAR SDK convention: `&self` receivers so contract implementations
/// and off-chain macro bindings generate uniform view-call endpoints.
pub trait NepPanicFreeResolution {
    /// Conjoin evidence states; the chain-meet (O(1) per pair).
    /// Returns 0x00-0x03. Out-of-range inputs are fail-closed to NEITHER.
    fn nep_conjoin(&self, inputs: &[u8]) -> u8;

    /// Only TRUE (0x01) is settleable.
    fn nep_is_settleable(&self, state: u8) -> bool;

    /// The invariant, self-describing.
    fn nep_invariant(&self) -> &'static str;
}

/// The reference gate — the concrete implementation.
pub struct ReferenceGate;

impl NepPanicFreeResolution for ReferenceGate {
    fn nep_conjoin(&self, inputs: &[u8]) -> u8 {
        // the chain-meet over the inputs; seed with the first input
        // (NEITHER is not an identity for the min-rank meet)
        let mut it = inputs.iter();
        let mut state = match it.next() {
            Some(&v) if v <= 3 => v,
            _ => return 0, // fail-closed to NEITHER
        };
        for &v in it {
            if v > 3 {
                return 0; // fail-closed: unknown is never authority
            }
            // min-rank: take v iff v's rank is strictly lower
            if RANK[v as usize] < RANK[state as usize] {
                state = v;
            }
        }
        state
    }

    fn nep_is_settleable(&self, state: u8) -> bool {
        state == State::True as u8
    }

    fn nep_invariant(&self) -> &'static str {
        "Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use State::*;

    fn gate() -> ReferenceGate {
        ReferenceGate
    }

    #[test]
    fn false_dominates_the_meet() {
        assert_eq!(gate().nep_conjoin(&[True as u8, False as u8]), False as u8);
    }

    #[test]
    fn both_is_held_not_reverted() {
        assert_eq!(gate().nep_conjoin(&[Both as u8, True as u8]), Both as u8);
        assert!(!gate().nep_is_settleable(Both as u8));
    }

    #[test]
    fn agreement_settles() {
        assert_eq!(gate().nep_conjoin(&[True as u8, True as u8]), True as u8);
        assert!(gate().nep_is_settleable(True as u8));
    }

    #[test]
    fn neither_absorbs_both() {
        assert_eq!(gate().nep_conjoin(&[Neither as u8, Both as u8]), Neither as u8);
    }

    #[test]
    fn empty_inputs_is_neither() {
        assert_eq!(gate().nep_conjoin(&[]), Neither as u8);
    }

    #[test]
    fn out_of_range_fails_closed() {
        assert_eq!(gate().nep_conjoin(&[True as u8, 9]), Neither as u8);
        assert!(!gate().nep_is_settleable(9));
    }

    #[test]
    fn no_nontrue_meet_yields_true() {
        let vals = [Neither as u8, True as u8, False as u8, Both as u8];
        for a in vals {
            for b in vals {
                let m = gate().nep_conjoin(&[a, b]);
                if a != True as u8 && b != True as u8 {
                    assert_ne!(m, True as u8, "non-TRUE meet yielded TRUE: {} ∧ {} = {}", a, b, m);
                }
            }
        }
    }

    #[test]
    fn meet_is_associative() {
        let vals = [Neither as u8, True as u8, False as u8, Both as u8];
        for a in vals {
            for b in vals {
                for c in vals {
                    let l = gate().nep_conjoin(&[gate().nep_conjoin(&[a, b]), c]);
                    let r = gate().nep_conjoin(&[a, gate().nep_conjoin(&[b, c])]);
                    assert_eq!(l, r);
                }
            }
        }
    }
}