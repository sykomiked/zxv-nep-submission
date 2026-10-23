# ECOSYSTEM_SOLUTIONS.md

**The constructive architectural map — how the paraconsistent
settlement layer addresses five operational friction points in the
NEAR ecosystem.**

> *A companion to the NEP: "Standard Interface for Deterministic
> Panic-Free State Resolution (Chain-Meet)." The NEP itself is
> deliberately narrow — the 1-byte interface. This document maps the
> broader solutions architecture, with `zedec.near` and the deployed
> contracts as live working examples.*

Every section follows the same constructive frame:

1. the operational context,
2. the practical optimization,
3. a case-study callout (a standard contract's failure mode, side by
   side with the resolution pattern).

---

## 1. Cross-Shard Promise Reversals & Cascading Failures

### The operational context

NEAR's sharded, asynchronous architecture means cross-contract calls
execute across multiple blocks. If step 3 of a 4-step promise chain
fails, an unhandled panic leaves dangling callback state or forces
complex manual rollback logic.

### The practical optimization

**Non-aborting state isolation** (`zxv-core`): rather than viewing
asynchronous callback disputes as contract errors requiring execution
halts, the engine models contention as an operational state (`BOTH`).
Preceding completed shard receipts remain committed, the contentious
receipt is isolated in an append-only log, and execution pipelines
continue without gas-draining panic loops.

### The case study

```text
classical:                       with the standard:
promise step 3 fails ──► panic   promise step 3 disputes ──► BOTH held
├─ step 1-2 receipts roll back   ├─ step 1-2 receipts stay committed
├─ prepaid gas burned            ├─ receipt hash kept in the audit log
└─ pipeline halts                └─ pipeline continues (arbitrable later)
```

---

## 2. Solver Collusion and Divergent AI Intent Routing

### The operational context

With NEAR's focus on AI agents and intents, solvers compete to settle
user intents. When two solvers submit conflicting execution paths or
divergent price oracles for a single intent, classical contracts either
pick one arbitrarily (last-write-wins) or crash.

### The practical optimization

**Deterministic intent conjoinment** via the chain-meet: when
decentralized solver networks disagree on intent preconditions, the
state machine applies an asymmetric chain-meet
(`FALSE < NEITHER < BOTH < TRUE`). Disputed settlement releases are
structurally blocked (`is_settleable == false`), escrowed capital
remains preserved, and automated arbitration can occur without rolling
back the transaction envelope.

### The case study

```text
classical:                        with the standard:
solver A says TRUE                solver A says TRUE
solver B says FALSE               solver B says FALSE
├─ LWW picks one ──► wrong order  ├─ chain-meet ──► BOTH
└─ or panic ──► gas burned        ├─ escrow preserved, no reversal
                                  └─ arbitration resolves later
```

---

## 3. State Trie Bloat & Storage Staking Costs

### The operational context

Storing complex dispute-resolution metadata, multi-sig arguments, and
oracle histories rapidly inflates contract storage, requiring
developers to lock large amounts of NEAR for storage staking.

### The practical optimization

**1-byte state encodings & off-chain CIDs**: complex epistemic
disputes are compressed into a single `u8` primitive. Rich operational
metadata, receipt histories, and cryptographic proofs are anchored via
content identifiers (CIDs) rather than raw string bloat in the
on-chain trie, reducing contract storage overhead to near zero.

### The case study

```text
classical:                          with the standard:
struct Dispute {                    pending_state: u8   // 1 byte
  metadata: String   // ~2 KB        contention_log: Vec<CID>  // hashes
  history: Vec<String> // unbounded  ├─ storage cost ≈ 1 byte + CID refs
  proofs: Vec<Vec<u8>>               └─ rich data lives off-chain, anchored
}
```

---

## 4. Oracle Latency & Contradictory Market Feeds

### The operational context

Traditional DeFi protocols on NEAR stall during volatile market events
when multiple decentralized oracle networks (e.g. Pyth, Band,
Chainlink) report out-of-sync or conflicting values.

### The practical optimization

**Paraconsistent multi-source settlement**: instead of relying on
crude timestamp overrides (which are susceptible to validator
front-running) or throwing reverts during high volatility, the contract
holds disputed valuations in an isolated contention state, protecting
liquidation pipelines from bad debt without pausing global protocol
execution.

### The case study

```text
classical:                        with the standard:
Oracle A: 100                     Oracle A: 100
Oracle B: 105 (volatile spike)    Oracle B: 105
├─ LWW: settle on 105 ──► bad debt ├─ chain-meet ──► BOTH (held)
└─ or revert ──► protocol pauses   ├─ no liquidation, no bad debt
                                   └─ arbitrate when feeds converge
```

---

## 5. Cross-Chain Intent Settlement Friction (Chain Abstraction)

### The operational context

In cross-chain abstraction, waiting for finality on foreign chains
(Bitcoin, Ethereum, Solana) creates long windows of uncertainty where
states are neither confirmed nor rejected.

### The practical optimization

**Tri- and four-valued epistemic staging** (`NEITHER` / `BOTH`):
classical binary booleans cannot represent cross-chain pending states
without inventing custom enums for every bridge. By establishing
`NEITHER` (pending external observation) and `BOTH` (disputed
cross-chain finality) as global standard byte primitives, cross-chain
intent bridges share a uniform interface for transaction lifecycle
states.

### The case study

```text
classical:                        with the standard:
foreign tx pending ──► ???        foreign tx pending ──► NEITHER (0x00)
├─ custom enum per bridge         foreign tx disputed ──► BOTH (0x03)
└─ no uniform lifecycle           ├─ one byte, every bridge
                                  └─ uniform intent lifecycle states
```

---

## The framing, stated plainly

This is not a replacement of NEAR's runtime. It is a set of
**higher-order primitives that build on existing NEAR standards**
(NEP-141, NEP-297) to make applications more resilient. The deployed
contracts are additive upgrades — a shared, gas-free, self-describing
utility — not a platform takeover.

## The live references

| Role | Address | What to verify |
|---|---|---|
| Canonical (routing) | `zedec.near` | `route()` → the topology |
| Protocol (110 kits) | `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf` | `is_settleable(BOTH)` → false |
| Registry (136 caps) | `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a` | `vm_capabilities()` → 136 |
| Gate (the invariant) | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` | `conjoin([1,2])` → FALSE |

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC ·
SPDX-License-Identifier: Apache-2.0*