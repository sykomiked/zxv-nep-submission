# The ZXV Developer Kit (SDK + Tools)

**Everything a NEAR developer needs to use the paraconsistent
settlement layer — the SDK, the CLI, and the reference
implementations.**

This toolkit accompanies the NEP: *"Standard Interface for
Deterministic Panic-Free State Resolution (Chain-Meet)."*

---

## The toolkit

| Tool | Language | What it does | Verify |
|---|---|---|---|
| **`zxv-nep-gate` crate** | Rust | the reference implementation — `#![no_std]`, zero allocations | `cargo test --release` → 9/9 |
| **`zxv-nep-gate` SDK** | TypeScript / JavaScript | the off-chain + on-chain gate (gas-free view calls) | `node test.js` → 10/10 |
| **`zxv-gate` CLI** | Node.js | the command-line interface (conjoin, settle, verify) | `node zxv-gate.js verify` |
| **`nep_gate.rs`** | Rust | the minimal reference (in `reference-implementation/`) | `cargo test --release` → 8/8 |

---

## The SDK (TypeScript / JavaScript)

```js
const { LocalGate, ZxvGate } = require("zxv-nep-gate");

// off-chain (zero gas, zero deps)
const gate = new LocalGate();
gate.conjoin([1, 2]);        // 2 (FALSE)  — TRUE+FALSE=FALSE
gate.conjoin([3, 1]);        // 3 (BOTH)   — held, not reverted
gate.isSettleable(1);        // true       — only TRUE settles

// on-chain (gas-free view calls against the live deployment)
const zxv = new ZxvGate();
await zxv.isSettleableOnChain("BOTH");   // false
await zxv.conjoinOnChain([1, 2]);        // 2
await zxv.route();                       // the canonical topology
```

## The CLI

```bash
node zxv-gate.js conjoin 1 2     # 2 (FALSE)
node zxv-gate.js settle BOTH     # false
node zxv-gate.js settle TRUE     # true
node zxv-gate.js invariant       # the invariant string
node zxv-gate.js route           # the canonical router topology
node zxv-gate.js verify          # the full on-chain verification suite
```

## The Rust crate

```rust
use zxv_nep_gate::{conjoin, is_settleable, invariant};

assert!(!is_settleable(conjoin(&[1, 2]))); // TRUE+FALSE -> FALSE
assert!(is_settleable(conjoin(&[1, 1])));  // TRUE+TRUE  -> TRUE
assert_eq!(conjoin(&[3, 1]), 3);           // BOTH+TRUE  -> BOTH (held)
```

---

## The addresses

| Role | Address |
|---|---|
| **Canonical (routing)** | `zedec.near` |
| **The protocol (110 kits)** | `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf` |
| **The registry (136 capabilities)** | `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a` |
| **The gate (kernel invariant)** | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` |

---

*The system is open. Verify it yourself.*