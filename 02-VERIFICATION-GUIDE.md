# NEP Verification Guide

**Reproducible verification for "Standard Interface for Deterministic
Panic-Free State Resolution (Chain-Meet)".**

Every claim in the NEP can be verified two ways: **on-chain** (the
live mainnet implementation) and **locally** (the reference
implementation in `reference-implementation/nep_gate.rs`).

---

## 1. On-chain verification (gas-free, any client)

### The invariant

```bash
# a contradiction never settles
curl -s https://rpc.mainnet.fastnear.com -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":"v","method":"query","params":{"request_type":"call_function","finality":"final","account_id":"f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf","method_name":"is_settleable","args_base64":"eyJzdGF0ZSI6IkJPVEgifQ=="}}'
# → false

# agreement settles
#   is_settleable {"state":"TRUE"}  → true
```

### The join

```bash
# conjoin {"inputs":[1,2]} → "FALSE"   (TRUE+FALSE=FALSE in the join)
```

### The audit chain

```bash
# verify_chain {"from":0} → true
```

### The canonical router

```bash
# zedec.near → route() → the full topology
```

### The discovery

```bash
# vm_capabilities on 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a
# → 136 capabilities
```

---

## 2. Local verification (the reference implementation)

```bash
cd reference-implementation
cargo test --release
```

Expected: **8/8 tests pass**:

| Test | Verifies |
|---|---|
| `false_dominates_the_meet` | TRUE+FALSE → FALSE |
| `both_is_held_not_reverted` | BOTH+TRUE → BOTH; not settleable |
| `agreement_settles` | TRUE+TRUE → TRUE; settleable |
| `neither_absorbs_both` | NEITHER+BOTH → NEITHER |
| `empty_inputs_is_neither` | empty → NEITHER |
| `out_of_range_fails_closed` | unknown phases never authority |
| `no_nontrue_meet_yields_true` | the structural invariant (16 combos) |
| `meet_is_associative` | the semilattice property (64 triples) |

---

## 3. The C demonstration (compile in seconds)

```c
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
typedef enum { NEITHER = 0x00, TRUE = 0x01, FALSE = 0x02, BOTH = 0x03 } state_t;
static const uint8_t rank[4] = { 1, 3, 0, 2 };
static state_t conjoin(state_t a, state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] <= rank[b] ? a : b;
}
static bool is_settleable(state_t s) { return s == TRUE; }
int main(void) {
    printf("BOTH settleable? %s\n", is_settleable(BOTH) ? "YES" : "NO (held, not reverted)");
    printf("TRUE settleable? %s\n", is_settleable(conjoin(TRUE, TRUE)) ? "YES" : "NO");
    return 0;
}
```

```bash
gcc -O3 -Wall -Wextra -pedantic -o gate gate.c && ./gate
```

Output:
```
BOTH settleable? NO (held, not reverted)
TRUE settleable? YES
```

---

## 4. The fuzzing record (from the live system)

| Run | Inputs | Violations |
|---|---|---|
| whole-u8 domain fuzz | 65,536 | 0 |
| gas-wall stress (reads) | 125 | 0 |
| gas-wall stress (writes) | 10 | 0 |

---

*Every claim verified live on NEAR mainnet at the time of writing.
The system is open — verify it yourself.*