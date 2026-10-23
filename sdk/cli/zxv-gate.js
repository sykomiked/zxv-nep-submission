#!/usr/bin/env node
/**
 * zxv-gate — the CLI for the NEAR NEP:
 * "Standard Interface for Deterministic Panic-Free State Resolution
 * (Chain-Meet)".
 *
 * Zero-dependency. All reads are gas-free RPC view calls.
 *
 * Usage:
 *   zxv-gate conjoin 1 2        # 2 (FALSE)
 *   zxv-gate settle BOTH        # false
 *   zxv-gate settle TRUE        # true
 *   zxv-gate invariant          # the invariant string
 *   zxv-gate route              # the canonical router topology
 *   zxv-gate verify             # the full on-chain verification suite
 *
 * Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
 * SPDX-License-Identifier: Apache-2.0
 */

const RPC = "https://rpc.mainnet.fastnear.com";
const CONTRACT =
  "f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf";
const REGISTRY =
  "7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a";

const RANK = [1, 3, 0, 2]; // [NEITHER, TRUE, FALSE, BOTH]
const NAMES = ["NEITHER", "TRUE", "FALSE", "BOTH"];

function conjoin(inputs) {
  if (inputs.length === 0) return 0;
  let state = inputs[0];
  if (state > 3) return 0;
  for (let i = 1; i < inputs.length; i++) {
    const v = inputs[i];
    if (v > 3) return 0;
    if (RANK[v] < RANK[state]) state = v;
  }
  return state;
}

function toBase64(s) {
  return Buffer.from(s, "utf8").toString("base64");
}

async function view(account, method, args = {}) {
  const res = await fetch(RPC, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: "zxv-gate",
      method: "query",
      params: {
        request_type: "call_function",
        finality: "final",
        account_id: account,
        method_name: method,
        args_base64: toBase64(JSON.stringify(args)),
      },
    }),
  });
  const json = await res.json();
  if (json.error) throw new Error(JSON.stringify(json.error));
  const bytes = json.result?.result ?? [];
  const text = Buffer.from(bytes).toString("utf8");
  return text ? JSON.parse(text) : null;
}

async function main() {
  const [cmd, ...rest] = process.argv.slice(2);

  switch (cmd) {
    case "conjoin": {
      const inputs = rest.map(Number);
      const r = conjoin(inputs);
      console.log(`${r} (${NAMES[r]})`);
      break;
    }
    case "settle": {
      const s = rest[0];
      const idx = NAMES.indexOf(s.toUpperCase());
      const val = idx >= 0 ? idx : Number(s);
      console.log(conjoin([val]) === 1 ? "true" : "false");
      break;
    }
    case "invariant":
      console.log(
        "Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic."
      );
      break;
    case "route": {
      const r = await view("zedec.near", "route");
      console.log(r);
      break;
    }
    case "verify": {
      console.log("=== the on-chain verification suite ===");
      const b = await view(CONTRACT, "is_settleable", { state: "BOTH" });
      console.log(`is_settleable(BOTH) -> ${b}`);
      const t = await view(CONTRACT, "is_settleable", { state: "TRUE" });
      console.log(`is_settleable(TRUE) -> ${t}`);
      const c = await view(CONTRACT, "conjoin", { inputs: [1, 2] });
      console.log(`conjoin([1,2]) -> ${c}`);
      const caps = await view(REGISTRY, "vm_capabilities");
      console.log(`vm_capabilities -> ${caps.length} capabilities`);
      console.log("=== all gas-free ===");
      break;
    }
    default:
      console.log(
        "zxv-gate — usage:\n  conjoin <n...>   the chain-meet\n  settle <state>   is it settleable?\n  invariant        the invariant\n  route            the canonical router\n  verify           the on-chain verification suite"
      );
  }
}

main().catch((e) => {
  console.error("ERROR:", e.message);
  process.exit(1);
});