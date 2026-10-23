# ZXV — The NEP Submission Package

**The complete attachment set for the NEAR NEP: *Standard Interface
for Deterministic Panic-Free State Resolution (Chain-Meet).***

This repository IS the submission package. No folders, no zips — every
file is here, indexed, verifiable, and referenced from the NEP.

---

## The NEP (the document to submit)

| File | What |
|---|---|
| **[`01-NEP.md`](01-NEP.md)** | the NEP proposal — submit as `neps/nep-0000-panic-free-resolution.md` in the `near/NEPs` PR |
| **[`02-VERIFICATION-GUIDE.md`](02-VERIFICATION-GUIDE.md)** | reproduce every claim (on-chain curl + local tests) |
| **[`03-ECOSYSTEM_SOLUTIONS.md`](03-ECOSYSTEM_SOLUTIONS.md)** | the constructive architecture map (the 5 NEAR friction points) |
| **[`INDEX.md`](INDEX.md)** | the serialized manifest — every file, what it is, how to use it |

## The workspace (the importable implementation)

[`workspace/`](workspace/) — the `zxv-nep-paraconsistent` cargo
workspace (7 members):

- [`crates/near-panic-free/`](workspace/crates/near-panic-free/) — the core `#![no_std]` crate (`cargo add near-panic-free`)
- [`crates/near-panic-free-sdk/`](workspace/crates/near-panic-free-sdk/) — the NEP-297 event helpers
- [`examples/`](workspace/examples/) — the 4 example contracts (multi-oracle escrow, agent intent solver, inline resolver, cross-contract resolver)
- [`kernel/`](workspace/kernel/) — the canonical reference + the **actual deployed wasm artifacts** with SHA-256 checksums
- [`tools/`](workspace/tools/) — `verify.sh` (one-shot), `verify_mainnet_gate.sh` (live RPC), `build_reproducible.sh`

```bash
cd workspace && bash tools/verify.sh
```

## The deployable kernel (deploy exactly as deployed)

[`deploy-repo/`](deploy-repo/) — the complete closure:

- [`build_system/build_near_contract.sh`](deploy-repo/build_system/build_near_contract.sh) — the exact build recipe
- [`kernel/`](deploy-repo/kernel/) — every source, the full include tree
- [`DEPLOYMENT.md`](deploy-repo/DEPLOYMENT.md) — build + deploy + compare the SHA-256

## The supporting material

| Path | What |
|---|---|
| [`contracts/`](contracts/) | the deployed contract sources (the provenance) |
| [`ledger/ADDRESS_LEDGER.md`](ledger/ADDRESS_LEDGER.md) | every contract address + role |
| [`ledger/TUTORIAL.md`](ledger/TUTORIAL.md) | how to use the whole system |
| [`sdk/`](sdk/) | the SDK + CLI |

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