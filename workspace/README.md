# zxv-nep-paraconsistent — the workspace

**The self-contained developer bundle for the NEP: *Standard Interface
for Deterministic Panic-Free State Resolution (Chain-Meet).***

A workspace so reviewers can `cargo add` or import the trait
immediately — no implementation friction, no committee stall.

## The layout

```
zxv-nep-paraconsistent/
├── Cargo.toml                  ← workspace root
├── crates/
│   ├── near-panic-free/        ← the core #![no_std] logic & trait
│   │   ├── src/lib.rs          ← enums, the rank table, meet/join
│   │   ├── src/traits.rs       ← NepPanicFreeResolution
│   │   └── tests/invariants.rs ← 16/64 KAT + the 65,536 fuzz
│   └── near-panic-free-sdk/    ← NEP-297 event helpers (+ optional
│                                 near-sdk bindings)
├── kernel/
│   ├── src/lib.rs              ← the canonical reference (zxv-core)
│   └── artifacts/              ← the ACTUAL deployed wasm + SHA-256
├── examples/
│   ├── multi_oracle_escrow/    ← the reference near-sdk contract
│   ├── agent_intent_solver/    ← the NEAR-2026 intent dispute
│   ├── inline_resolver/        ← Path A: in-binary crate consumption
│   └── cross_contract_resolver/← Path B: routing to the public kernel
└── tools/
    ├── verify.sh               ← the single-shot verification
    ├── verify_mainnet_gate.sh  ← the live RPC assertion harness
    └── build_reproducible.sh   ← the reproducible wasm build
```

## The two integration paths

| Path | How developers use it | Primary benefit |
|---|---|---|
| **A: inline library** | `near-panic-free` compiled directly into the contract binary | zero cross-contract overhead — resolution executes in the same WASM frame in nanoseconds (a single comparison) |
| **B: public utility** | the contract queries the shared `zedec.near` / `zxv-core` kernel | zero contract-size increase — small contracts route contentious inputs to the shared dispute utility |

## Quick start

```bash
bash tools/verify.sh   # the whole bundle, one shot
```

```toml
[dependencies]
near-panic-free = { version = "0.1", default-features = false }
```

```rust
use near_panic_free::{conjoin_slice, is_settleable, invariant};

assert!(!is_settleable(conjoin_slice(&[1, 2]))); // TRUE+FALSE -> FALSE
assert!(is_settleable(conjoin_slice(&[1, 1])));  // TRUE+TRUE  -> TRUE
```

## The verification record

| Check | Result |
|---|---|
| core `#![no_std]` wasm32 check | pass |
| core KAT + fuzz (16/64 KAT, associativity, absorption, invariant, 65,536 fuzz) | 11/11 |
| sdk NEP-297 event schema | 2/2 |
| workspace resolution (7 members) | pass |
| live mainnet gate (BOTH→false, TRUE→true, route→ok) | pass |
| deployed artifacts + SHA-256 (fetched from mainnet) | pass |

## The addresses

- **Canonical (routing):** `zedec.near`
- **The protocol (110 kits):**
  `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`
- **The registry (136 capabilities):**
  `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a`
- **The gate (kernel invariant):**
  `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8`

---

*The system is open. Verify it yourself.*

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC ·
SPDX-License-Identifier: Apache-2.0*