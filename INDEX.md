# The Submission Index

**The complete attachment manifest for the NEP: *Standard Interface
for Deterministic Panic-Free State Resolution (Chain-Meet)*.**

Every file in this package, what it is, and exactly how to use it.
Each filename is unique. The package is self-contained: verify
everything, build everything, deploy everything.

---

## The package at a glance

```
submission-package/
├── 01-NEP.md                          ← the NEP (submit as neps/nep-0000-panic-free-resolution.md)
├── 02-VERIFICATION-GUIDE.md           ← reproduce every claim (on-chain + local)
├── 03-ECOSYSTEM_SOLUTIONS.md          ← the constructive architecture map (5 NEAR friction points)
├── INDEX.md                           ← this file
├── reference-implementation/
│   └── nep_gate.rs                    ← the minimal #![no_std] reference (8/8 tests)
├── workspace/                         ← the zxv-nep-paraconsistent cargo workspace
├── deploy-repo/                       ← the complete, deployable kernel closure
├── contracts/
│   ├── zxv-core-contract.rs           ← the deployed 110-kit contract source
│   └── zedec-near-canonical.rs        ← the canonical router (zedec.near) source
├── ledger/
│   ├── ADDRESS_LEDGER.md              ← every contract address + role
│   └── TUTORIAL.md                    ← how to use the whole system
└── sdk/                               ← the SDK + CLI (also inside workspace/)
```

---

## The file manifest (serialized)

| # | File | What it is | How to use it |
|---|---|---|---|
| 1 | `01-NEP.md` | The NEP proposal | Submit as `neps/nep-0000-panic-free-resolution.md` in the PR |
| 2 | `02-VERIFICATION-GUIDE.md` | The reproducible verification | Run the curl commands; run the tests — every claim is checkable |
| 3 | `03-ECOSYSTEM_SOLUTIONS.md` | The architecture map | Read for the 5 friction points + the case studies (referenced from the NEP's Rationale) |
| 4 | `INDEX.md` | This manifest | The map to everything |
| 5 | `reference-implementation/nep_gate.rs` | The minimal Rust reference | `cargo test` it, or read it — the whole standard in one file |
| 6 | `workspace/` | The cargo workspace (7 members) | `bash tools/verify.sh` — one command verifies the whole bundle |
| 7 | `workspace/crates/near-panic-free/` | The core `#![no_std]` crate | `cargo add near-panic-free` — the importable standard |
| 8 | `workspace/crates/near-panic-free-sdk/` | The NEP-297 event helpers | Import for the event schema + optional near-sdk bindings |
| 9 | `workspace/examples/multi_oracle_escrow/` | The reference near-sdk contract | Read it — the few lines that replace `env::panic_str()` |
| 10 | `workspace/examples/agent_intent_solver/` | The NEAR-2026 intent solver | Read it — disputes held, not reverted |
| 11 | `workspace/examples/inline_resolver/` | Path A integration | The inline-library consumption pattern |
| 12 | `workspace/examples/cross_contract_resolver/` | Path B integration | The public-utility routing pattern |
| 13 | `workspace/kernel/` | The canonical reference (Rust) | The deployed logic as a crate |
| 14 | `workspace/kernel/artifacts/` | The ACTUAL deployed wasm + SHA-256 | Compare to the on-chain code hash |
| 15 | `workspace/tools/verify.sh` | The single-shot verification | `bash tools/verify.sh` |
| 16 | `workspace/tools/verify_mainnet_gate.sh` | The live RPC assertions | `bash tools/verify_mainnet_gate.sh` |
| 17 | `workspace/tools/build_reproducible.sh` | The reproducible wasm build | `bash tools/build_reproducible.sh` |
| 18 | `deploy-repo/DEPLOYMENT.md` | The deployment guide | Build + deploy exactly as deployed (pinned clang, exact flags) |
| 19 | `deploy-repo/build_system/build_near_contract.sh` | THE build script | The exact recipe that produced the deployed wasm |
| 20 | `deploy-repo/kernel/` | The complete deployable kernel closure | Build it, compare the SHA-256 to the artifacts |
| 21 | `contracts/zxv-core-contract.rs` | The deployed 110-kit source | The provenance of the deployed protocol |
| 22 | `contracts/zedec-near-canonical.rs` | The canonical router source | The provenance of `zedec.near` |
| 23 | `ledger/ADDRESS_LEDGER.md` | Every address + role | The map of the deployed system |
| 24 | `ledger/TUTORIAL.md` | How to use the whole system | The user manual |
| 25 | `sdk/` | The SDK + CLI | The developer toolkit (also in the workspace) |

---

## The three ways to verify (pick any)

```bash
# 1. the whole workspace, one command
cd workspace && bash tools/verify.sh

# 2. the live mainnet gate (gas-free)
cd workspace && bash tools/verify_mainnet_gate.sh

# 3. the core crate's tests (KAT + fuzz)
cd workspace && cargo test -p near-panic-free --release
```

## The deployment path (deploy exactly as deployed)

```bash
cd deploy-repo
CLANG=/path/to/clang-23.1.0 sh build_system/build_near_contract.sh
# compare dist/near/zxv_near_contract.wasm's SHA-256 to:
#   667d44d1d8035ab8309912c815e1e11b9c8464b8ff9fa25af576030e1bbbc119
```

## The addresses (the whole system)

- **Canonical (routing):** `zedec.near`
- **The protocol (110 kits):**
  `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`
- **The registry (136 capabilities):**
  `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a`
- **The gate (kernel invariant):**
  `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8`

---

*The system is open. Verify it yourself.*