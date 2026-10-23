# kernel — the canonical reference implementation (the living testbed)

The deployed `zxv-core` kernel, as a standalone wasm build, with the
**actual mainnet artifacts** fetched from the NEAR RPC.

## The functional role (clearly delineated)

The NEP specifies the **interface and the behavior**
(`NepPanicFreeResolution`, the 1-byte state machine, the chain-meet
ranking). This crate is the **canonical reference implementation and
living testbed** — the exact logic deployed on NEAR mainnet.

It is **NOT a mandatory dependency for NEP compliance.** The standard
is neutral: any contract may implement the interface inline (Path A)
or route to this public kernel (Path B). This crate exists so
reviewers can build the deployed logic reproducibly and compare the
artifact checksum to the on-chain code.

## The artifacts (fetched from mainnet, verified)

| Artifact | Bytes | SHA-256 | Deployed at |
|---|---|---|---|
| `zxv_near_contract.wasm` | 453,307 | `667d44d1d8035ab8309912c815e1e11b9c8464b8ff9fa25af576030e1bbbc119` | `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf` (the 110-kit protocol) |
| `zxv_gate_contract.wasm` | 46,981 | `9051f96a3a3119de80c4eaca1ffc07fddecb5197edcf03f717f7454e6039cc3a` | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` (the gate / kernel invariant) |

Both were retrieved directly from the NEAR RPC (`request_type:
view_code`) at the time of writing — these are the **actual deployed
binaries**, not reconstructions. The `checksums.sha256` file records
their digests.

## The canonical router

`zedec.near` — a live contract routing to the whole utility
(`route()`, `protocol_meta()`, `is_settleable()`).

## Reproducible build

```bash
bash tools/build_reproducible.sh
```

Builds the kernel crate for `wasm32-unknown-unknown` and writes the
artifact + checksum. Compare against the on-chain code hash for full
provenance.

## The live verification

```bash
bash tools/verify_mainnet_gate.sh
```

Proves the invariant on the deployed gate, gas-free:
`is_settleable(BOTH) -> false`, `is_settleable(TRUE) -> true`,
`zedec.near route() -> ok`.