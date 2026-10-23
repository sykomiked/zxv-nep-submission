// test.js — verify the zxv-nep-gate SDK (zero-dependency, plain Node).
const { LocalGate, State } = require("./zxv-nep-gate.js");

const gate = new LocalGate();
let pass = 0, fail = 0;
function check(name, got, want) {
  const ok = got === want;
  ok ? pass++ : fail++;
  console.log(`  [${ok ? "PASS" : "FAIL"}] ${name}: ${got} ${ok ? "" : `(expected ${want})`}`);
}

console.log("=== the SDK local-gate tests ===");
check("conjoin([1,2]) = FALSE(2)", gate.conjoin([1, 2]), State.FALSE);
check("conjoin([3,1]) = BOTH(3)", gate.conjoin([3, 1]), State.BOTH);
check("conjoin([1,1]) = TRUE(1)", gate.conjoin([1, 1]), State.TRUE);
check("conjoin([]) = NEITHER(0)", gate.conjoin([]), State.NEITHER);
check("conjoin([1,9]) = NEITHER(0) fail-closed", gate.conjoin([1, 9]), State.NEITHER);
check("conjoin([0,3]) = NEITHER(0) absorbs BOTH", gate.conjoin([0, 3]), State.NEITHER);
check("isSettleable(BOTH) = false", gate.isSettleable(State.BOTH), false);
check("isSettleable(TRUE) = true", gate.isSettleable(State.TRUE), true);
check("isSettleable(FALSE) = false", gate.isSettleable(State.FALSE), false);
check("invariant is non-empty", gate.invariant().length > 20, true);
console.log(`\n${pass} passed, ${fail} failed`);
process.exit(fail ? 1 : 0);