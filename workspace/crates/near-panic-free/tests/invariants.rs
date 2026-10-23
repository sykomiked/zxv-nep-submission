//! The invariant test harness for `near-panic-free`.
//!
//! 1. **KAT** — the full 4×4 known-answer table (16 cases).
//! 2. **Semilattice** — idempotence, commutativity, associativity
//!    (64 triples), absorption.
//! 3. **The structural invariant** — no non-TRUE combination yields
//!    TRUE (16 cases).
//! 4. **Fuzz** — the whole u8 domain (65,536 inputs), fail-closed.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_panic_free::{conjoin_pair, conjoin_slice, is_settleable, BOTH, FALSE, NEITHER, TRUE};

const STATES: [u8; 4] = [NEITHER, TRUE, FALSE, BOTH];

/// The KAT table, derived from the rank order
/// FALSE(0) < NEITHER(1) < BOTH(2) < TRUE(3).
fn kat(a: u8, b: u8) -> u8 {
    use near_panic_free::RANK;
    if RANK[b as usize] < RANK[a as usize] {
        b
    } else {
        a
    }
}

#[test]
fn kat_16_cases() {
    // the full 4x4 table
    for a in STATES {
        for b in STATES {
            assert_eq!(
                conjoin_pair(a, b),
                kat(a, b),
                "KAT mismatch: conjoin_pair({}, {})",
                a,
                b
            );
        }
    }
}

#[test]
fn kat_signature_cases() {
    // the signature rows a reviewer will check first
    assert_eq!(conjoin_pair(TRUE, FALSE), FALSE); // TRUE+FALSE=FALSE
    assert_eq!(conjoin_pair(BOTH, TRUE), BOTH); // BOTH held, never coerced
    assert_eq!(conjoin_pair(TRUE, TRUE), TRUE); // agreement settles
    assert_eq!(conjoin_pair(NEITHER, BOTH), NEITHER); // NEITHER absorbs BOTH
    assert_eq!(conjoin_pair(FALSE, TRUE), FALSE); // FALSE dominates
    assert_eq!(conjoin_pair(BOTH, NEITHER), NEITHER);
}

#[test]
fn idempotence() {
    for a in STATES {
        assert_eq!(conjoin_pair(a, a), a, "idempotence failed for {}", a);
    }
}

#[test]
fn commutativity() {
    for a in STATES {
        for b in STATES {
            assert_eq!(
                conjoin_pair(a, b),
                conjoin_pair(b, a),
                "commutativity failed for ({}, {})",
                a,
                b
            );
        }
    }
}

#[test]
fn associativity_64_triples() {
    for a in STATES {
        for b in STATES {
            for c in STATES {
                let l = conjoin_pair(conjoin_pair(a, b), c);
                let r = conjoin_pair(a, conjoin_pair(b, c));
                assert_eq!(l, r, "associativity failed for ({}, {}, {})", a, b, c);
            }
        }
    }
}

#[test]
fn absorption() {
    // meet(a, join(a, b)) = a; on the chain, join is max-rank:
    fn join_pair(a: u8, b: u8) -> u8 {
        use near_panic_free::RANK;
        if RANK[b as usize] > RANK[a as usize] {
            b
        } else {
            a
        }
    }
    for a in STATES {
        for b in STATES {
            assert_eq!(
                conjoin_pair(a, join_pair(a, b)),
                a,
                "absorption failed for ({}, {})",
                a,
                b
            );
        }
    }
}

#[test]
fn structural_invariant_no_nontrue_yields_true() {
    // THE invariant: only TRUE settles. No combination of non-TRUE
    // values can ever yield TRUE.
    for a in STATES {
        for b in STATES {
            let m = conjoin_pair(a, b);
            if a != TRUE && b != TRUE {
                assert_ne!(
                    m, TRUE,
                    "INVARIANT VIOLATION: meet({}, {}) = TRUE",
                    a, b
                );
            }
        }
    }
}

#[test]
fn settleable_only_true() {
    assert!(is_settleable(TRUE));
    assert!(!is_settleable(NEITHER));
    assert!(!is_settleable(FALSE));
    assert!(!is_settleable(BOTH));
}

#[test]
fn conjoin_slice_matches_pair_fold() {
    for a in STATES {
        for b in STATES {
            for c in STATES {
                let slice = conjoin_slice(&[a, b, c]);
                let fold = conjoin_pair(conjoin_pair(a, b), c);
                assert_eq!(slice, fold, "slice/fold mismatch for ({}, {}, {})", a, b, c);
            }
        }
    }
}

#[test]
fn conjoin_slice_empty_is_neither() {
    assert_eq!(conjoin_slice(&[]), NEITHER);
}

#[test]
fn fuzz_whole_u8_domain_65536() {
    // the full u8 domain: 256 x 256 = 65,536 input pairs.
    // The invariant must hold for every pair, and out-of-range inputs
    // must fail closed (never TRUE, never authority).
    for a in 0u16..=255 {
        for b in 0u16..=255 {
            let m = conjoin_pair(a as u8, b as u8);
            // fail-closed: an unknown phase is never authority
            if a > 3 || b > 3 {
                assert_ne!(m, TRUE, "out-of-range input produced TRUE: ({}, {})", a, b);
            } else {
                // the invariant on the valid domain
                if a as u8 != TRUE && b as u8 != TRUE {
                    assert_ne!(m, TRUE, "INVARIANT VIOLATION: ({}, {})", a, b);
                }
            }
        }
    }
}