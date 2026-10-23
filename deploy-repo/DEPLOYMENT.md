# Deploying the ZXV Kernel — exactly as it was deployed

**The complete, self-contained deployment closure.** This directory
contains everything needed to build and deploy the paraconsistent
kernel smart contract exactly as it was deployed on NEAR mainnet.

## The layout

```
deploy-repo/
├── build_system/
│   └── build_near_contract.sh   ← THE build script (the exact recipe)
└── kernel/
    ├── arch/near/               ← the NEAR host ABI files
    │   ├── near_contract.c      ← the contract entry (the on-chain methods)
    │   ├── near_rt.c            ← the runtime helpers
    │   └── near_env.h           ← the NEAR "env" host bindings
    ├── include/                 ← the kernel's include tree (31 headers,
    │                               incl. freestanding.h + zxv_fixed.h)
    └── src/                     ← the exact compiled source list:
        ├── finance/triple_ledger.c/.h   ← the economic ledger
        ├── vino/vino.c/.h              ← the rails (capital types)
        ├── surplus/surplus.c/.h        ← the Q32.32 fixed-point math
        ├── rmag/rmag_core.c            ← the vault
        ├── lpres/lpres.c/.h            ← THE paraconsistent logic
        ├── lpres/lpres_core.c/.h       ← the core state machine
        ├── lpres/lpres_phases.h        ← the rank table (145 states)
        ├── tribute/tribute.c/.h        ← the tribute
        ├── scp144/scp144.c/.h          ← the SCP-144 lattice
        ├── pqsec/mldsa/fips202.c       ← the SHA-3 (Keccak) primitive
        ├── zcapital/zcap_forms.h       ← the capital forms
        ├── edp_risk/edp_risk.c/.h      ← the EDP risk
        ├── event_space/event_space.h   ← the event space
        ├── robin_debanks/sha256.c/.h   ← SHA-256
        └── modbind/modbind.h           ← the module bindings
```

## Build it (the exact recipe)

The build script is the one used for the deployment. It requires the
**pinned clang** (23.1.0) with the wasm32 target:

```bash
cd deploy-repo
CLANG=/path/to/clang-23.1.0 sh build_system/build_near_contract.sh
# → writes dist/near/zxv_near_contract.wasm
```

The exact flags (from the script): `--target=wasm32 -std=c11 -O2
-ffreestanding -nostdlib`, the freestanding libc remaps
(`-Dmemset=fs_memset` …), the table bounds (`-DZXV_TL_MAX_ACCOUNTS=64`
…), and the linker flags (`--no-entry --strip-all --export-memory`,
`--initial-memory=8388608`).

## The deployed artifact (the ground truth)

The **actual deployed binaries** were fetched from the NEAR RPC
(`request_type: view_code`) — these are what's on mainnet right now:

| Artifact | Deployed at | SHA-256 |
|---|---|---|
| `zxv_near_contract.wasm` (453,307 bytes) | `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf` | `667d44d1d8035ab8309912c815e1e11b9c8464b8ff9fa25af576030e1bbbc119` |
| `zxv_gate_contract.wasm` (46,981 bytes) | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` | `9051f96a3a3119de80c4eaca1ffc07fddecb5197edcf03f717f7454e6039cc3a` |

Build from source, then compare your artifact's SHA-256 against these
to confirm byte-for-byte reproducibility.

## Deploy it (the standard NEAR flow)

```bash
# 1. build (above)
# 2. deploy to your own account (or a fresh subaccount)
near deploy <your-account.near> dist/near/zxv_near_contract.wasm

# 3. verify the invariant, gas-free
near view <your-account.near> is_settleable '{"state":"BOTH"}'   # false
near view <your-account.near> is_settleable '{"state":"TRUE"}'   # true
near view <your-account.near> conjoin '{"inputs":[1,2]}'         # "FALSE"
```

## The canonical topology

- **Router:** `zedec.near` (routes to the whole utility)
- **Protocol (110 kits):** `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`
- **Registry (136 capabilities):** `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a`
- **Gate (the kernel invariant):** `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8`

## The honest note

This is the **same C the fleet kernels link** — triple_ledger, vino,
surplus (Q32.32), scp144, sha256 — plus `arch/near/near_contract.c`,
which exports the on-chain methods and round-trips the ledger state
through `env.storage_*`. The table bounds are shrunk for the on-chain
build (the fleet defaults serialize to ~48 MB); the code and
arithmetic are unchanged.

*The system is open. Deploy it yourself.*