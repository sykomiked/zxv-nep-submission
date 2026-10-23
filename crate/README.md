# zxv-nep-gate

**The reference implementation of the NEAR NEP: Standard Interface for
Deterministic Panic-Free State Resolution (Chain-Meet).**

A `#![no_std]`, zero-allocation, four-valued logic gate. Only `TRUE`
settles; contradictions (`BOTH`) are held, never reverted.

## The invariant

> **Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded,
> never coerced, never a panic.**

The total order is `FALSE < NEITHER < BOTH < TRUE`. Because TRUE is the
top of the chain, **no combination of non-TRUE values can ever yield
TRUE** — the invariant is structural, not checked.

## Usage

```rust
use zxv_nep_gate::{conjoin, is_settleable, invariant};

assert!(!is_settleable(conjoin(&[1, 2]))); // TRUE+FALSE -> FALSE
assert!(is_settleable(conjoin(&[1, 1])));  // TRUE+TRUE  -> TRUE
assert_eq!(conjoin(&[3, 1]), 3);           // BOTH+TRUE  -> BOTH (held)
```

## The live mainnet reference

The same logic is deployed on NEAR mainnet:

- **Canonical (routing):** `zedec.near`
- **The protocol (110 kits):**
  `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`

Verify gas-free:

```bash
curl -s https://rpc.mainnet.fastnear.com -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"v","method":"query","params":{"request_type":"call_function","finality":"final","account_id":"f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf","method_name":"is_settleable","args_base64":"eyJzdGF0ZSI6IkJPVEgifQ=="}}'
# → false
```

## Tests

```bash
cargo test --release   # 9/9 tests
```

## License

Apache-2.0 · Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9
Genetics, LLC