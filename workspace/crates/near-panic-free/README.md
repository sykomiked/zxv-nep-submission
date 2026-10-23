# near-panic-free

**Deterministic panic-free state resolution via chain-meet logic for
NEAR.** The reference implementation of the NEP: *Standard Interface
for Deterministic Panic-Free State Resolution (Chain-Meet)*.

A pure `#![no_std]`, **zero-dependency** crate: the 1-byte four-valued
state machine, the chain-meet resolution, and the structural invariant
"only TRUE settles."

## The invariant

> **Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded,
> never coerced, never a panic.**

The total order is `FALSE < NEITHER < BOTH < TRUE`. Because TRUE is the
top of the chain, **no combination of non-TRUE values can ever yield
TRUE** — the invariant is structural, not checked.

## Usage

```toml
[dependencies]
near-panic-free = { version = "0.1", default-features = false }
```

```rust
use near_panic_free::{conjoin_slice, is_settleable, invariant, BOTH, TRUE};

assert!(!is_settleable(conjoin_slice(&[1, 2]))); // TRUE+FALSE -> FALSE
assert!(is_settleable(conjoin_slice(&[1, 1])));  // TRUE+TRUE  -> TRUE
assert_eq!(conjoin_slice(&[3, 1]), 3);           // BOTH+TRUE  -> BOTH (held)
```

## Verification

```bash
# the KAT + fuzz harness (16/64 KAT, associativity, absorption,
# the structural invariant, and the whole-u8-domain fuzz)
cargo test --release

# the no_std proof (builds for the wasm32 target)
cargo check --no-default-features --target wasm32-unknown-unknown
```

## The live mainnet reference

The same logic is deployed at `zedec.near` and
`f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`.
Verify gas-free:

```bash
curl -s https://rpc.mainnet.fastnear.com -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"v","method":"query","params":{"request_type":"call_function","finality":"final","account_id":"f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf","method_name":"is_settleable","args_base64":"eyJzdGF0ZSI6IkJPVEgifQ=="}}'
# → false
```

## License

Apache-2.0 · Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9
Genetics, LLC