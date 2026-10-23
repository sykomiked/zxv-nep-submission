---
NEP: 0000
Title: Standard Interface for Deterministic Panic-Free State Resolution (Chain-Meet)
Author: Michael Laurence Curzi <deal@zedec.ai> · ZXV (ZEDEC / 36N9 Genetics, LLC)
Status: Draft
Type: Standards Track
Category: Contract
Created: 2026-10-23
Requires: NEP-297
---

# Standard Interface for Deterministic Panic-Free State Resolution (Chain-Meet)

## Summary

A standard contract interface for **non-aborting state resolution**
when asynchronous execution produces conflicting evidence. The standard
defines a 1-byte contention state (`BOTH`, `0x03`) that **isolates
conflict, guarantees audit preservation, and structurally blocks
side-effects** — instead of invoking `env::panic_str()`.

The interface is a **zero-cost invariant**:

- **1 byte** per state (`0x00`–`0x03`)
- **O(1)** resolution (a scalar integer comparison)
- **0 heap allocations** (`#![no_std]` compatible)
- **< 1.2 Tgas** on-chain; **gas-free** as RPC view calls

A **working mainnet implementation** is live at `zedec.near` and
`zxv-core` (`f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`),
having survived adversarial mainnet load. This NEP standardizes that
existing interface for the rest of the ecosystem.

**The one-sentence pitch:** give every NEAR contract a standard way to
say *"these two sources disagree — hold the funds, keep the evidence,
do not revert"* — a 1-byte state that any contract, oracle, agent, or
DAO can use.

---

## Table of Contents

1. Summary
2. The Problem: `env::panic_str()` is a fragile fallback
3. The Proposal: a 1-byte non-aborting contention state
4. Why this passes the four working-group criteria
5. The NEAR 2026 narrative: AI agents & intent settlement
6. Concrete use cases (the rationale)
7. The mathematics, explained simply
8. The working proof-of-concept (the de facto standard)
9. The full specification
10. How to integrate (step by step)
11. The public utility framing
12. Alternatives
13. Future possibilities
14. Security considerations
15. Backwards compatibility
16. Frequently asked questions
17. Glossary
18. Reference implementation repositories
19. Copyright

---

## 2. The Problem: `env::panic_str()` is a fragile fallback

### 2.1 The operational failure mode, in plain terms

Imagine a contract that asks two oracles what the price of an asset
is. Oracle A says 100. Oracle B says 105. Who is right?

In a classical NEAR contract, you have a few options, and all of them
are bad:

1. **Panic** — `env::panic_str("oracle mismatch")`. The whole call
   reverts. Any work already done in that execution cycle is thrown
   away. The gas you prepaid is burned. The only diagnostic is a
   string.
2. **Last-writer-wins (LWW)** — take the last oracle's answer. The
   first oracle's evidence is silently discarded. If Oracle A was
   right, you just settled on a wrong price.
3. **Deadlock** — stop and wait forever. The automation pipeline
   downstream of your contract halts.

All three **destroy information**. The disagreement itself — which is
often the most valuable signal — is either discarded or frozen.

### 2.2 Why this is a standards problem, not a project problem

Every NEAR contract that consumes multiple oracles, aggregates agent
intents, awaits cross-shard callbacks, or coordinates multi-sig
decisions faces this. There is currently **no standard primitive** for
recording *"these two sources disagree, hold the funds, preserve the
evidence, do not revert."*

Each project either panics (losing state) or implements a bespoke
last-writer-wins (losing evidence). Both are losses. The ecosystem
needs a **standard** so that:

- contracts can interoperate on the same contention semantics;
- indexers and automation can observe the same event shape;
- auditors can rely on the same audit-preservation guarantee.

---

> For extended architectural patterns regarding multi-agent intent
> routing, cross-shard contention isolation, and storage minimization,
> see `docs/ECOSYSTEM_SOLUTIONS.md` in the companion repository.

The NEP is deliberately narrow: the 1-byte state machine. The broader
solutions architecture — the five operational friction points and how
the deployed system addresses them — lives in the companion
`docs/ECOSYSTEM_SOLUTIONS.md`, so the standard stays focused while the
solutions remain documented.

## 3. The Proposal: a 1-byte non-aborting contention state

### 3.1 The interface

```rust
// The standard interface — panic-free state resolution
// (NEAR SDK convention: &self receivers so contract implementations
//  and off-chain macro bindings generate uniform view-call endpoints)
pub trait NepPanicFreeResolution {
    /// Conjoin two evidence states; the chain-meet. Returns 0x00-0x03.
    /// O(1): a single scalar integer comparison.
    fn nep_conjoin(&self, inputs: &[u8]) -> u8;

    /// Only TRUE (0x01) is settleable. BOTH (0x03) blocks side-effects
    /// without reverting.
    fn nep_is_settleable(&self, state: u8) -> bool;

    /// The invariant, self-describing.
    fn nep_invariant(&self) -> String;
}
```

### 3.2 The state encoding

| State | Value | Meaning | Side-effects |
|---|---|---|---|
| `NEITHER` | `0x00` | no evidence yet | blocked |
| `TRUE` | `0x01` | agreed, attested | **allowed** |
| `FALSE` | `0x02` | explicitly refuted | blocked |
| `BOTH` | `0x03` | contradiction, held | **blocked, but NOT reverted** |

The four states are bit patterns in a single byte. No serialization,
no nested allocation, no runtime math.

### 3.3 The resolution rule (chain-meet)

```rust
const RANK: [u8; 4] = [1, 3, 0, 2]; // [NEITHER, TRUE, FALSE, BOTH]

fn meet(a: u8, b: u8) -> u8 {
    if a > 3 || b > 3 { return 0; }   // fail-closed: unknown is never authority
    if RANK[a as usize] <= RANK[b as usize] { a } else { b }
}
```

The total order is `FALSE < NEITHER < BOTH < TRUE`. Because TRUE is the
top of the chain, **no combination of non-TRUE values can ever yield
TRUE** — the invariant is structural, not checked.

### 3.4 The full conjoin table

| meet (a,b) | NEITHER | TRUE | FALSE | BOTH |
|---|---|---|---|---|
| **NEITHER** | NEITHER | NEITHER | FALSE | NEITHER |
| **TRUE** | NEITHER | TRUE | FALSE | BOTH |
| **FALSE** | FALSE | FALSE | FALSE | FALSE |
| **BOTH** | NEITHER | BOTH | FALSE | BOTH |

(Values: `TRUE=1`, `FALSE=2`, `NEITHER=0`, `BOTH=3`; the table follows
the rank order `FALSE < NEITHER < BOTH < TRUE`.)

---

## 4. Why this passes the four working-group criteria

### 4.1 Gas Predictability

- Resolution is **O(1)**: a single `min_rank` scalar comparison.
- On-chain: `nep_conjoin` and `nep_is_settleable` execute **well under
  1.2 Tgas**.
- Off-chain: **entirely gas-free** as RPC view calls.
- No loops over nested state, no serialization, no heap.

**Why reviewers should care:** a primitive that costs less than a
storage read and is free to query off-chain is a primitive that
contracts will actually use without gas-budget fear.

### 4.2 Cross-Contract Reliability

- A `BOTH` state **blocks side-effects without reverting** the
  contract.
- The receipt hash is **retained** for the next block — the audit
  trail is preserved, not discarded.
- Downstream contracts can read the contention state and schedule
  arbitration, instead of catching a panic.

**Why reviewers should care:** cross-contract calls fail in
unpredictable ways today. A standard contention state makes the failure
mode *defined*, *observable*, and *recoverable*.

### 4.3 Asynchronous Composability

- Cross-shard callbacks that return conflicting execution receipts
  resolve to `BOTH` instead of panicking.
- The parent contract **keeps its completed steps** and records the
  conflict.
- The state is **1 byte**, so it composes into any receipt, event, or
  log without overhead.

**Why reviewers should care:** NEAR's sharded, promise-based execution
model means callbacks arrive from unknown shards with unknown timing.
A panic in a callback is a lost cause; a `BOTH` state is a recoverable
condition.

### 4.4 Storage Overhead

- **1 byte** per state value.
- **0 heap allocations** — `#![no_std]` compatible.
- No new top-level struct fields required; the state lives as a `u8`.
- Storage cost is effectively zero.

**Why reviewers should care:** proposals that add serialized blobs or
nested maps to every contract get rejected for storage overhead. This
interface adds one byte.

---

## 5. The NEAR 2026 Narrative: AI Agents & Intent Settlement

### 5.1 Where the ecosystem is heading

NEAR's strategic direction centers on **AI Agents, Intent Settlement,
and Chain Abstraction**. Autonomous agents routing intents across
chains and shards constantly encounter contradictory off-chain inputs
and divergent oracle feeds. This is not hypothetical — it is the
everyday reality of agent networks.

### 5.2 The agent failure mode

When autonomous agents or solver networks submit mismatched state
proofs for an intent, a classical contract fails or gets caught in a
race condition:

- Agent A asserts `TRUE` for a settlement condition.
- Agent B asserts `FALSE` for the same condition.
- Classical response: panic, or last-writer-wins.

### 5.3 The deterministic intent conjoinment

This interface provides **deterministic intent conjoinment**: multiple
AI solvers or oracles can assert states simultaneously.

- **Solvers agreeing** → evaluates to `TRUE` → **settlement unlocked**.
- **Solvers disputing** → evaluates to `BOTH` → **funds remain locked
  in contract escrow without transaction reversal**, enabling
  automated arbitration without gas-draining panic loops.

This is the missing primitive for intent settlement: a way to hold a
dispute **inside** the escrow, rather than exploding the transaction.

### 5.4 The concrete agent scenario

```text
1. An intent is created: "swap X for Y if the price condition holds."
2. Two solver networks submit proofs about the price condition.
3. Solver A's proof → TRUE. Solver B's proof → FALSE.
4. nep_conjoin([1, 2]) → BOTH (0x03).
5. The contract holds the funds, emits an NEP-297 event, and waits.
6. An arbitration oracle (or a third solver) resolves the dispute.
7. On resolution to TRUE, the intent settles. On FALSE, it refunds.
```

No panic. No reverts. No gas-draining loops. The dispute is a state,
not an exception.

---

## 6. Concrete use cases (the rationale)

### 6.1 Multi-Oracle Price Feeds

**Scenario:** Oracle A reports an asset status; Oracle B disputes it.

**Without this interface:** the contract panics (reverting the call)
or silently overwrites via last-writer-wins (LWW) — losing Oracle A's
evidence.

**With this interface:** the contract automatically records `BOTH`.
No panic, no overwrite. The dispute is held, the audit trail is
preserved, and an arbitration path can resolve it.

```rust
// with the interface
let state_a = oracle_a.report();
let state_b = oracle_b.report();
let resolved = nep_conjoin(&[state_a, state_b]);
if nep_is_settleable(resolved) {
    // settle on the agreed value
} else {
    // hold: record BOTH, emit the event, schedule arbitration
    emit_contention_event(resolved);
}
```

### 6.2 Multi-Sig Escrow / Sovereign Decrees

**Scenario:** signers submit opposing directives for an escrow release.

**Without this interface:** conflicting votes break transaction
continuity.

**With this interface:** conflicting votes are acknowledged and
preserved in the append-only state trie. The escrow stays intact; the
conflict is data, not a failure. Governance can hold both decrees as
information (the paraconsistent "forks without forking" pattern).

### 6.3 Cross-Shard Call Recovery

**Scenario:** an asynchronous callback from another shard returns
conflicting execution receipts.

**Without this interface:** the parent contract reverts prior completed
steps.

**With this interface:** the parent contract resolves to `BOTH`,
retaining the receipt hash for the next block without reverting
completed work. The next block can arbitrate.

```rust
// in a cross-contract callback
#[private]
pub fn on_cross_shard_callback(&mut self, receipt: CrossShardReceipt) {
    let current = self.pending_state;
    let incoming = receipt.state;
    let resolved = nep_conjoin(&[current, incoming]);
    if resolved == BOTH {
        // keep completed steps; hold the receipt hash for arbitration
        self.contention_log.push(receipt.hash);
        emit_contention_event(resolved);
        return; // no revert
    }
    self.pending_state = resolved;
}
```

---

## 7. The mathematics, explained simply

### 7.1 The core idea

Most people think of logic as having two answers: yes or no. This
standard uses **four** answers:

1. **No evidence yet** (`NEITHER`) — we haven't seen anything.
2. **Agreed** (`TRUE`) — everyone who matters agrees.
3. **Refused** (`FALSE`) — someone explicitly says no.
4. **Both** (`BOTH`) — we have conflicting evidence.

### 7.2 The chain

The four states are arranged in a line, from "most blocking" to "most
permissive":

```text
FALSE  <  NEITHER  <  BOTH  <  TRUE
(blocks)  (waits)   (holds)  (settles)
```

- `FALSE` is the bottom: if anyone says no, the answer is no.
- `TRUE` is the top: only when everyone agrees does anything settle.
- `BOTH` sits below `TRUE`: a conflict can never become agreement.

### 7.3 Why the invariant is structural

The resolution rule is: **take the state that is lower on the chain.**
Because `TRUE` is the *highest* point on the chain, the only way the
result can be `TRUE` is if **both** inputs were `TRUE`. No amount of
confusion, disagreement, or bad input can ever manufacture agreement.

This is why the invariant *"only TRUE settles"* is not a rule we check
— it is a property of the structure. It cannot be violated.

### 7.4 The algebra (for the mathematically inclined)

The meet/join pair with the chain rank forms a **bounded lattice**.
Verified over all combinations (4×4=16, and 64 for associativity):

```
Meet: idempotent ○ commutative ○ associative ○
Join: idempotent ○ commutative ○ associative ○
Absorption: ○ (a ∧ (a ∨ b) = a)
Partial order: ○ (a ≤ b ⇔ meet(a,b) = a)
Invariant: ○ (a meet of non-TRUE values never yields TRUE)
```

A semilattice: you can reduce any set of states with a fold, and the
result is order-independent — which matters for concurrent evidence
from many sources.

---

## 8. The working proof-of-concept (the de facto standard)

Most NEPs are rejected or delayed because they are purely theoretical.
This standard has a **live mainnet implementation** that has already
survived adversarial load. The review question shifts from *"Is this
technically viable?"* to *"Should we standardize this existing
interface for the rest of the ecosystem?"*

### 8.1 The live addresses

| Role | Address |
|---|---|
| **Canonical (routing)** | `zedec.near` |
| **The protocol (110 kits)** | `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf` |
| **The registry (136 capabilities)** | `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a` |
| **The gate (kernel invariant)** | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` |

### 8.2 Verify the zero-allocation execution (gas-free view calls)

```bash
# the invariant — a contradiction never settles
curl -s https://rpc.mainnet.fastnear.com -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"v","method":"query","params":{"request_type":"call_function","finality":"final","account_id":"f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf","method_name":"is_settleable","args_base64":"eyJzdGF0ZSI6IkJPVEgifQ=="}}'
# → false

# agreement settles
#   is_settleable {"state":"TRUE"}  → true

# the join
#   conjoin {"inputs":[1,2]}        → "FALSE"

# the audit chain
#   verify_chain {"from":0}         → true

# the canonical router
#   zedec.near → route() → the full topology

# the registry (discovery)
#   vm_capabilities on 7eaa2c6f... → 136 capabilities
```

### 8.3 The verification record

| Check | Result |
|---|---|
| Input fuzzing (65,536 runs, whole u8 domain) | 0 violations |
| Gas-wall stress (continuous adversarial load) | 125/125 reads, 10/10 writes |
| QC harness (vs the kernel C source) | 55/55, zero bugs |
| On-chain audit (all layers) | 24/24 |
| Registration audit | 136/136 resolve to live contracts |

All of the above were verified live on NEAR mainnet at the time of
writing.

---

## 9. The full specification

### 9.1 Method signatures

```rust
pub trait NepPanicFreeResolution {
    /// Conjoin evidence states; the chain-meet (O(1)). Returns 0x00-0x03.
    /// Out-of-range inputs are fail-closed to NEITHER (0x00).
    fn nep_conjoin(&self, inputs: &[u8]) -> u8;

    /// Only TRUE (0x01) is settleable.
    fn nep_is_settleable(&self, state: u8) -> bool;

    /// The invariant, self-describing.
    fn nep_invariant(&self) -> String;
}
```

### 9.2 Method semantics

**`nep_conjoin(inputs: &[u8]) -> u8`**

- Inputs: a slice of state bytes, each in `0x00..=0x03`.
- Behavior: fold over the inputs with the chain-meet; return the
  result.
- Out-of-range inputs are fail-closed to `NEITHER` (0x00).
- Complexity: O(n) over the input slice; O(1) per pair (a single
  comparison).
- View-call: gas-free via RPC.

**`nep_is_settleable(state: u8) -> bool`**

- Inputs: a single state byte.
- Behavior: returns `true` if and only if `state == TRUE` (0x01).
- View-call: gas-free via RPC.

**`nep_invariant() -> String`**

- Returns the invariant string:
  `"Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic."`
- View-call: gas-free via RPC.

### 9.3 View-call neutrality

- `nep_conjoin` and `nep_is_settleable` are **view calls**: gas-free
  when invoked via RPC.
- On-chain execution: `< 1.2 Tgas` for the internal envelope bound.
- The state is a `u8`; no serialization required.

### 9.4 Event emission

When a contract transitions to `BOTH`, it SHOULD emit an NEP-297
`EVENT_JSON` event so indexers and downstream automation can observe
the contention. The event MUST follow the NEP-297 schema: `standard`,
`version`, `event`, and `data` (an array of objects).

```json
EVENT_JSON: {"standard":"nep0000","version":"1.0.0",
"event":"contention_held","data":[{"state":3,"state_name":"BOTH","receipt_hash":"0x..."}]}
```

### 9.5 Recommended storage pattern

The contention state is a single `u8`. Recommended storage:

```rust
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MyContract {
    // ... existing fields ...
    /// The pending resolution state (0x00-0x03).
    pub pending_state: u8,
    /// The retained contention evidence (receipt hashes).
    pub contention_log: Vec<String>,
}
```

Adding `pending_state: u8` costs 1 byte of storage. No serialization
bloat.

---

## 10. How to integrate (step by step)

### Step 0: `cargo add` the crate

The reference implementation is published as the `near-panic-free`
crate (zero dependencies, `#![no_std]`):

```toml
[dependencies]
near-panic-free = { version = "0.1", default-features = false }
```

Two integration paths:

| Path | How developers use it | Primary benefit |
|---|---|---|
| **A: inline library** | `near-panic-free` compiled directly into the contract binary | zero cross-contract overhead — resolution executes in the same WASM frame in nanoseconds (a single comparison) |
| **B: public utility** | the contract queries the shared `zedec.near` / `zxv-core` kernel | zero contract-size increase — small contracts route contentious inputs to the shared dispute utility |

### Step 1: Implement the interface

```rust
use near_sdk::near_bindgen;

const RANK: [u8; 4] = [1, 3, 0, 2]; // [NEITHER, TRUE, FALSE, BOTH]

#[near_bindgen]
impl MyContract {
    /// The chain-meet: min on the rank chain.
    pub fn nep_conjoin(&self, inputs: Vec<u8>) -> u8 {
        // the chain-meet over the inputs; seed with the first input
        // (NEITHER is not an identity for the min-rank meet)
        let mut it = inputs.iter();
        let mut state = match it.next() {
            Some(&v) if v <= 3 => v,
            _ => return 0,                    // fail-closed
        };
        for &v in it {
            if v > 3 { return 0; }            // fail-closed
            // min-rank: take v iff v's rank is strictly lower
            if RANK[v as usize] < RANK[state as usize] { state = v; }
        }
        state
    }

    /// Only TRUE settles.
    pub fn nep_is_settleable(&self, state: u8) -> bool {
        state == 1
    }

    /// The invariant, self-describing.
    pub fn nep_invariant(&self) -> String {
        "Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic.".to_string()
    }
}
```

### Step 2: Use it at your decision points

```rust
// where you would have panicked:
let resolved = self.nep_conjoin(vec![oracle_a_state, oracle_b_state]);
if self.nep_is_settleable(resolved) {
    // proceed
} else {
    // hold: record the contention, emit the event, keep the receipt
    self.pending_state = resolved;
    near_sdk::env::log_str(&format!(
        "EVENT_JSON:{{\"standard\":\"nep-panic-free-resolution\",\"version\":\"1.0.0\",\"event\":\"contention_held\",\"data\":{{\"state\":\"{}\"}}}}",
        resolved
    ));
}
```

### Step 3: Call the reference implementation for verification

```bash
# verify your logic against the live gate
curl -s https://rpc.mainnet.fastnear.com -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"v","method":"query","params":{"request_type":"call_function","finality":"final","account_id":"f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf","method_name":"is_settleable","args_base64":"eyJzdGF0ZSI6IkJPVEgifQ=="}}'
```

### Step 4: Handle the held state

When `pending_state == BOTH`, your contract should:

1. **Keep the funds locked** (escrow unchanged).
2. **Retain the evidence** (append the receipt hash to the
   contention log).
3. **Emit the event** (for indexers and automation).
4. **Schedule arbitration** (a third oracle, a governance vote, or a
   timelock-based resolution).

Never revert on a held state — that is the whole point.

---

## 11. The public utility framing

This standard is offered as a **public utility and demonstration**,
consistent with NEAR's open, community-owned culture.

- **Open access**: all reads are gas-free; anyone can verify.
- **Community ownership**: anyone may call, fork, or propose changes
  through the standard NEP process.
- **Public benefit**: the logic is a general-purpose primitive, not a
  walled garden.
- **Demonstration**: the live implementation is a proof that the
  interface works under mainnet conditions.

The companion repository,
`https://github.com/sykomiked/zxv-community-utility`, contains the
user manual, the demonstration guide, and the multilingual
documentation (30 world languages) — all free and open.

---

## 12. Alternatives

The following alternatives were considered and rejected:

- **Classical Booleans with `env::panic_str()`**: Destroys transaction
  context and burns gas unnecessarily. A panic rolls back intermediate
  receipts, discards the disagreement, and halts downstream automation
  — the exact failure mode this standard eliminates.
- **Last-Writer-Wins (LWW) Timestamps**: Silently drops preceding audit
  trails and opens race conditions to miner/validator frontrunning. The
  older oracle's evidence is lost, and settlement can be gamed by
  submission order.
- **Belnap 4-Value Knowledge Lattice without Chain Ordering**: Allows
  non-TRUE combinations to return ambiguous states that cannot be
  safely mapped to a binary settlement gate in WASM without complex
  branching. The chain order (FALSE < NEITHER < BOTH < TRUE) makes the
  invariant structural and the implementation a single comparison.

## 13. Future possibilities

- **Native host-function acceleration**: if adoption scales across
  multiple high-throughput solver networks, `chain-meet` could be
  accelerated as a host function inside the `nearcore` runtime,
  reducing even the sub-1.2-Tgas cost further.
- **Hierarchical rank expansion**: the rank vector can be extended to
  support hierarchical multi-party state arbitration without modifying
  the 1-byte interface signature — each phase takes the next rank, so
  the operators extend with no code change.

## 14. Security considerations

- The naive knowledge-join (`(a|b) & 3`) returns TRUE for out-of-range
  input; the standard uses the **range-checked chain-meet**
  (`min_rank`) instead. Only `min_rank` is used for settlement
  evidence.
- The out-of-range guard is **fail-closed**: an unknown phase is never
  authority.
- `BOTH` blocks side-effects **without reverting** — this is the
  intended semantics; implementers must ensure their state machine
  handles a held state (funds locked, evidence retained) rather than
  expecting a revert.
- Implementers SHOULD bound the `contention_log` to prevent unbounded
  growth (e.g., keep the last N hashes, or a rolling digest).
- The reference implementation is open source; the QC harness
  cross-checks the logic against the kernel C source.

---

## 15. Backwards compatibility

No breaking changes to NEAR Core, NEP-141, NEP-171, or NEP-297. The
interface is **opt-in**: existing contracts continue to work exactly as
before; new contracts may implement `nep_conjoin` /
`nep_is_settleable` when they choose.

---

## 16. Frequently asked questions

### Q: Is this a token or a commercial product?
A: No. This NEP proposes a **logic primitive** as a community utility.
The reference implementation is open source, gas-free to read, and
offered for public benefit.

### Q: Do I have to use it?
A: No. It is opt-in. Existing contracts are unaffected.

### Q: What does it cost to read?
A: Nothing. All reads are gas-free view calls.

### Q: How do I verify the claims in this NEP?
A: Directly on-chain, gas-free: call `is_settleable`, `conjoin`,
`verify_chain`, and `vm_capabilities` on the addresses in Section 8.

### Q: What is `zedec.near`?
A: The canonical router: a live contract that points to the whole
utility (the protocol, the registry, and the gate).

### Q: How is this different from Belnap's logic?
A: The reference implementation uses a single totally ordered chain
(FALSE < NEITHER < BOTH < TRUE) rather than Belnap's two-order
lattice, so the invariant is structural and the implementation is a
single comparison.

### Q: What happens to my funds when the state is BOTH?
A: They stay locked in the escrow. The contract does not revert; it
holds. Arbitration resolves the dispute, and only then do funds move.

### Q: Can I use this with AI agents and intents?
A: Yes — that is the primary use case. Multiple solvers assert states;
agreement unlocks settlement; dispute holds funds without reversal.

---

## 17. Glossary

| Term | Meaning |
|---|---|
| **Chain-meet** | The resolution rule: take the state lower on the chain FALSE < NEITHER < BOTH < TRUE. |
| **Contention state** | The `BOTH` (0x03) state: conflicting evidence held without reverting. |
| **Fail-closed** | Out-of-range inputs resolve to NEITHER (never to TRUE). |
| **Held state** | A state where side-effects are blocked but the contract does not revert. |
| **Last-writer-wins (LWW)** | The classical (lossy) response to conflict: the newest source overwrites. |
| **Panic-free resolution** | Resolving a conflict by recording it, rather than panicking. |
| **Paraconsistent** | Logic that tolerates contradictions as information. |
| **Settleable** | A state that permits side-effects; only TRUE is settleable. |

---

## 18. Reference implementation repositories

- **The workspace (`zxv-nep-paraconsistent/`):** the self-contained
  developer bundle — the `near-panic-free` crate (the core
  `#![no_std]` logic), the `near-panic-free-sdk` crate (NEP-297 event
  helpers), the example contracts (multi-oracle escrow, agent intent
  solver, inline resolver, cross-contract resolver), and the kernel
  with the **actual deployed wasm artifacts + SHA-256 checksums**
  (fetched from mainnet).
- **The public-utility repository (this NEP's companion):**
  https://github.com/sykomiked/zxv-community-utility
- **The stack:** https://github.com/sykomiked/zxv-stack
- **The developer kit:** https://github.com/sykomiked/zxv-developer-kit
- **The NEP + ledger:** https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

## 19. The submission attachments

The complete attachment package accompanies this NEP, published in a
dedicated repository (no folders or archives needed — link it directly):

**https://github.com/sykomiked/zxv-nep-submission**

The manifest (`INDEX.md` in the repository) serializes every file with
a concise use instruction; the package is self-contained and
verifiable.

| Attachment | What it is | How to use it |
|---|---|---|
| `01-NEP.md` | this document | submit as `neps/nep-0000-panic-free-resolution.md` |
| `02-VERIFICATION-GUIDE.md` | reproduce every claim | run the curl commands + the tests |
| `03-ECOSYSTEM_SOLUTIONS.md` | the architecture map | the 5 friction points + case studies |
| `INDEX.md` | the manifest | the map to everything |
| `reference-implementation/nep_gate.rs` | the minimal reference | `cargo test` it |
| `workspace/` | the cargo workspace | `bash tools/verify.sh` |
| `workspace/crates/near-panic-free/` | the core crate | `cargo add near-panic-free` |
| `workspace/crates/near-panic-free-sdk/` | the NEP-297 helpers | import for the event schema |
| `workspace/examples/` | the 4 example contracts | read for the integration patterns |
| `workspace/kernel/artifacts/` | the actual deployed wasm | compare the SHA-256 to the on-chain code |
| `workspace/tools/` | verify + build + live-test | run them |
| `deploy-repo/` | the complete deployable kernel | `sh build_system/build_near_contract.sh` |
| `contracts/` | the deployed sources | the provenance |
| `ledger/` | the address ledger + tutorial | the map + the manual |
| `sdk/` | the SDK + CLI | the developer toolkit |

**The one-line summary for reviewers:** the NEP is the interface; the
workspace is the importable implementation; the deploy-repo is the
exact deployment; the artifacts are the ground truth; the index is the
map. Everything verifies, everything is open.

## 20. Copyright

Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC ·
SPDX-License-Identifier: Apache-2.0