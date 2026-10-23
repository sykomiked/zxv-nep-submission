//! # near-panic-free-sdk
//!
//! The `near-sdk` macro bindings and NEP-297 event helpers for the
//! `near-panic-free` standard.
//!
//! With the `near-sdk` feature (default), this crate provides a
//! ready-made `#[near_bindgen]` implementation of the
//! `NepPanicFreeResolution` interface, so a contract can be compliant
//! in a few lines.
//!
//! It also provides the NEP-297 `contention_held` event emitter —
//! the exact schema the standard requires.
//!
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

pub use near_panic_free::{conjoin_slice, is_settleable, NepPanicFreeResolution, BOTH, FALSE, NEITHER, TRUE};

/// The NEP-297 event standard name.
pub const EVENT_STANDARD: &str = "nep0000";
/// The event version.
pub const EVENT_VERSION: &str = "1.0.0";
/// The event name for a held contention.
pub const EVENT_CONTENTION_HELD: &str = "contention_held";

/// The state names (for events and logs).
pub const STATE_NAMES: [&str; 4] = ["NEITHER", "TRUE", "FALSE", "BOTH"];

/// Build the NEP-297 `EVENT_JSON` line for a held contention.
///
/// The schema follows the NEP-297 parser exactly: `standard`,
/// `version`, `event`, and `data` as an **array** of objects.
pub fn contention_event(receipt_hash: &str) -> String {
    format!(
        "EVENT_JSON:{{\"standard\":\"{}\",\"version\":\"{}\",\"event\":\"{}\",\"data\":[{{\"state\":{},\"state_name\":\"{}\",\"receipt_hash\":\"{}\"}}]}}",
        EVENT_STANDARD,
        EVENT_VERSION,
        EVENT_CONTENTION_HELD,
        BOTH,
        STATE_NAMES[BOTH as usize],
        receipt_hash
    )
}

/// Build the NEP-297 `EVENT_JSON` line for any resolution state.
pub fn resolution_event(state: u8, receipt_hash: &str) -> String {
    format!(
        "EVENT_JSON:{{\"standard\":\"{}\",\"version\":\"{}\",\"event\":\"{}\",\"data\":[{{\"state\":{},\"state_name\":\"{}\",\"receipt_hash\":\"{}\"}}]}}",
        EVENT_STANDARD,
        EVENT_VERSION,
        "resolution",
        state,
        STATE_NAMES[state as usize],
        receipt_hash
    )
}

#[cfg(feature = "near-sdk")]
pub mod bindings {
    //! The ready-made `#[near_bindgen]` implementation of the
    //! `NepPanicFreeResolution` interface.
    use near_panic_free::{conjoin_slice, is_settleable, NepPanicFreeResolution};
    use near_sdk::near_bindgen;

    /// A minimal binding that exposes the standard's three methods as
    /// view calls on any contract that embeds it.
    #[near_bindgen]
    #[derive(Default)]
    pub struct PanicFreeBindings;

    #[near_bindgen]
    impl NepPanicFreeResolution for PanicFreeBindings {
        fn nep_conjoin(&self, inputs: Vec<u8>) -> u8 {
            conjoin_slice(&inputs)
        }

        fn nep_is_settleable(&self, state: u8) -> bool {
            is_settleable(state)
        }

        fn nep_invariant(&self) -> &'static str {
            "Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contention_event_matches_nep297_schema() {
        let e = contention_event("0xabc");
        assert!(e.starts_with("EVENT_JSON:"));
        assert!(e.contains("\"standard\":\"nep0000\""));
        assert!(e.contains("\"version\":\"1.0.0\""));
        assert!(e.contains("\"event\":\"contention_held\""));
        // data MUST be an array
        assert!(e.contains("\"data\":["));
        assert!(e.contains("\"state\":3"));
        assert!(e.contains("\"state_name\":\"BOTH\""));
        assert!(e.contains("\"receipt_hash\":\"0xabc\""));
    }

    #[test]
    fn resolution_event_reports_state() {
        let e = resolution_event(TRUE, "0xdef");
        assert!(e.contains("\"event\":\"resolution\""));
        assert!(e.contains("\"state\":1"));
        assert!(e.contains("\"state_name\":\"TRUE\""));
    }
}