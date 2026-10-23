# The Submission Index

**What is attached to this NEP, versus what lives in the repository.**

The NEP platform accepts **Markdown attachments only**. Everything else
(the Rust crate, the kernel source, the wasm artifacts, the SDK, the
examples) is in the dedicated submission repository — reference it by
link; do not try to attach it.

---

## Part A — Attached to this NEP (Markdown, 6 files)

| File | What it is |
|---|---|
| `01-NEP-Document.md` | the NEP proposal — submit as `neps/nep-0000-panic-free-resolution.md` |
| `02-Verification-Guide.md` | reproduce every claim (on-chain curl + local tests) |
| `03-Ecosystem-Solutions.md` | the constructive architecture map (the 5 NEAR friction points) |
| `04-Submission-Index.md` | this file — the map of what is attached vs. what is in the repo |
| `06-Address-Ledger.md` | every contract address + role |
| `07-Tutorial.md` | how to use the whole system |

## Part B — In the repository (link only, not attached)

**https://github.com/sykomiked/zxv-nep-submission**

| Repo path | What it is |
|---|---|
| `workspace/crates/near-panic-free/` | the core `#![no_std]` Rust crate (`cargo add near-panic-free`) |
| `workspace/crates/near-panic-free-sdk/` | the NEP-297 event helpers |
| `workspace/examples/` | the 4 example contracts |
| `workspace/kernel/` | the canonical reference + the actual deployed wasm artifacts + SHA-256 |
| `workspace/tools/` | `verify.sh`, `verify_mainnet_gate.sh`, `build_reproducible.sh` |
| `reference-implementation/nep_gate.rs` | the minimal Rust reference (the file that cannot be attached here) |
| `deploy-repo/` | the complete deployable kernel closure + the build script + `DEPLOYMENT.md` |
| `contracts/` | the deployed contract sources |
| `sdk/` | the SDK + CLI |

## The verification path

```bash
# the whole bundle, one command (from the repo)
cd zxv-nep-submission/workspace && bash tools/verify.sh

# the live mainnet gate (gas-free)
bash tools/verify_mainnet_gate.sh
```

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