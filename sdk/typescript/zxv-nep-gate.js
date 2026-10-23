/**
 * zxv-nep-gate.js — the JavaScript SDK for the NEAR NEP:
 * "Standard Interface for Deterministic Panic-Free State Resolution
 * (Chain-Meet)".
 *
 * Zero-dependency. Works in Node.js and the browser. All reads are
 * gas-free RPC view calls.
 *
 * Usage:
 *   const { ZxvGate, LocalGate } = require("zxv-nep-gate");
 *   new LocalGate().conjoin([1, 2]);       // 2 (FALSE)
 *   await new ZxvGate().isSettleableOnChain("BOTH"); // false
 *
 * Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
 * SPDX-License-Identifier: Apache-2.0
 */

"use strict";

/** The four states of the standard. */
const State = Object.freeze({
  NEITHER: 0x00,
  TRUE: 0x01,
  FALSE: 0x02,
  BOTH: 0x03,
});

/** The chain rank: FALSE is the bottom, TRUE is the top. */
const RANK = [1, 3, 0, 2]; // [NEITHER, TRUE, FALSE, BOTH]

const DEFAULT_RPC = "https://rpc.mainnet.fastnear.com";
const DEFAULT_CONTRACT =
  "f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf";

function toBase64(s) {
  if (typeof Buffer !== "undefined") return Buffer.from(s, "utf8").toString("base64");
  return btoa(unescape(encodeURIComponent(s)));
}

async function viewCall(rpc, contract, method, args) {
  const res = await fetch(rpc, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: "zxv-nep-gate",
      method: "query",
      params: {
        request_type: "call_function",
        finality: "final",
        account_id: contract,
        method_name: method,
        args_base64: toBase64(JSON.stringify(args)),
      },
    }),
  });
  const json = await res.json();
  if (json.error) throw new Error(JSON.stringify(json.error));
  const bytes = json.result?.result ?? [];
  if (!bytes.length) throw new Error("empty view result");
  const text = typeof Buffer !== "undefined"
    ? Buffer.from(bytes).toString("utf8")
    : new TextDecoder().decode(new Uint8Array(bytes));
  return JSON.parse(text);
}

/** The local, off-chain gate (the chain-meet, O(1) per pair). */
class LocalGate {
  conjoin(inputs) {
    if (!inputs || inputs.length === 0) return State.NEITHER;
    let state = inputs[0];
    if (state > 3) return State.NEITHER; // fail-closed
    for (let i = 1; i < inputs.length; i++) {
      const v = inputs[i];
      if (v > 3) return State.NEITHER; // fail-closed
      if (RANK[v] < RANK[state]) state = v;
    }
    return state;
  }

  isSettleable(state) {
    return state === State.TRUE;
  }

  invariant() {
    return "Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic.";
  }
}

/** The on-chain gate — gas-free view calls against the live deployment. */
class ZxvGate extends LocalGate {
  constructor(opts) {
    super();
    this.rpc = (opts && opts.rpc) || DEFAULT_RPC;
    this.contract = (opts && opts.contract) || DEFAULT_CONTRACT;
  }

  async isSettleableOnChain(state) {
    const s = typeof state === "number" ? ["NEITHER", "TRUE", "FALSE", "BOTH"][state] : state;
    return (await viewCall(this.rpc, this.contract, "is_settleable", { state: s })) === true;
  }

  async conjoinOnChain(inputs) {
    return await viewCall(this.rpc, this.contract, "conjoin", { inputs });
  }

  async route() {
    return await viewCall(this.rpc, "zedec.near", "route", {});
  }
}

module.exports = { State, LocalGate, ZxvGate };