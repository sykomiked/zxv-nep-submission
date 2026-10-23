# The ZXV Address Ledger

**Every contract address in the ecosystem, with its role and
verification. All addresses verified live on NEAR mainnet.**

---

## The core addresses

| Role | Address | What it does |
|---|---|---|
| **Canonical (routing)** | `zedec.near` | the community front door — routes to the whole utility (`route()`, `protocol_meta()`, `is_settleable()`) |
| **The protocol (110 kits)** | `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf` | the micro-contract hosting every kit (gate, treasury, rails, receipts, gas station, port, decrees, ...) |
| **The registry (136 capabilities)** | `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a` | discovery — `vm_capabilities()` lists all 136 |
| **The gate (kernel invariant)** | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` | the kernel invariant — only TRUE settles |

## The deployed kernel (launched on NEAR)

The kernel that launched on NEAR is deployed at the gate address
`6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8`
(the C kernel's NEAR architecture, `kernel/arch/near/near_contract.c`).
Its code hash is verifiable on-chain.

## The original subsystems (the 13)

| Capability | Address |
|---|---|
| gate, ledger (the kernel) | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` |
| registry (the boot sector) | `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a` |
| oracle | `6a57f581ea7c7c986769af6c874b16e8187410b62fe7190cdfb06001fb8998aa` |
| bus | `3c70e9c629068ac1fad6bd61ea26d4b4486c7e2a033c87d3635051bc2db0276e` |
| shield | `789457e61798d8deb3e45d60e84dec1ba0ca0fa8e7181eaa1646485f0ea89846` |
| micro_infer | `86f80143df70df0b02aa7625eddba7d4900bd543de24fa193b7fd4e0330a4dbe` |
| model_registry | `77361b15a4a8f7e43fdbf7b276138eeaf9d8fb5f6c2e73ec25c8c8b76a719c65` |
| consumer | `8947dcb7d35102dca4dc6cb852038b17efa1ff1a421209e5d0e7122425b0c654` |
| paraconsistent_oracle | `ea85f5381ceff17553fa3093a2d67a0e6cc4853abd7c19487ed7a33391acc2f7` |
| settlement_router | `8bf177019402d3d843b361a04d3ed7e21de637bf5d252956a15208f7950effae` |
| yhv_wrapper | `1a051ac2465c9a04e532fe420e2efbd8cc12f9a1d9f2a350470782fd3255ecf1` |
| ai_verdict | `781fc03d4dfb9b29d88df561157b11db6dc1e02368eae6deef7998d188ff966e` |

## The zedec.near address book (the 15 routing names)

| Name | Resolves to |
|---|---|
| `zedec.near` | the canonical (the live router) |
| `zedec.near/core` | `f18889ea…` |
| `zedec.near/registry` | `7eaa2c6f…` |
| `zedec.near/gate` | `6ca890f7…` |
| `zedec.near/kernel` | `6ca890f7…` |
| `zedec.near/oracle` | `6a57f581…` |
| `zedec.near/bus` | `3c70e9c6…` |
| `zedec.near/shield` | `789457e6…` |
| `zedec.near/micro_infer` | `86f80143…` |
| `zedec.near/model_registry` | `77361b15…` |
| `zedec.near/consumer` | `8947dcb7…` |
| `zedec.near/oracle_pc` | `ea85f538…` |
| `zedec.near/router` | `8bf17701…` |
| `zedec.near/wrapper` | `1a051ac2…` |
| `zedec.near/ai` | `781fc03d…` |

## Verification

```bash
# the invariant (gas-free)
#   is_settleable {"state":"BOTH"}  → false
#   is_settleable {"state":"TRUE"}  → true
# the discovery
#   vm_capabilities on 7eaa2c6f... → 136
# the audit chain
#   verify_chain {"from":0} → true
```

---

*All addresses verified live on NEAR mainnet. The system is open.*