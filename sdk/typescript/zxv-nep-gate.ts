/**
 * zxv-nep-gate — the TypeScript SDK for the NEAR NEP:
 * "Standard Interface for Deterministic Panic-Free State Resolution
 * (Chain-Meet)".
 *
 * Zero-dependency. Works in Node.js and the browser. All reads are
 * gas-free RPC view calls.
 *
 * Usage:
 *   import { ZxvGate } from "zxv-nep-gate";
 *   const gate = new ZxvGate();
 *   await gate.isSettleable("BOTH"); // false
 *   await gate.conjoin([1, 2]);      // 2 (FALSE)
 *
 * Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
 * SPDX-License-Identifier: Apache-2.0
 */

/** The four states of the standard. */
export const State = {
  NEITHER: 0x00,
  TRUE: 0x01,
  FALSE: 0x02,
  BOTH: 0x03,
} as const;

/** The chain rank: FALSE is the bottom, TRUE is the top. */
const RANK: number[] = [1, 3, 0, 2]; // [NEITHER, TRUE, FALSE, BOTH]

/** The NEAR RPC endpoint (override for testnet). */
const DEFAULT_RPC = "https://rpc.mainnet.fastnear.com";

/** The default contract (the live zxv-core on mainnet). */
const DEFAULT_CONTRACT =
  "f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf";

/** A minimal base64 encoder (no external deps). */
function toBase64(bytes: Uint8Array): string {
  let bin = "";
  for (const b of bytes) bin += String.fromCharCode(b);
  return btoa ? btoa(bin) : Buffer.from(bin, "binary").toString("base64");
}

/** A minimal JSON-RPC view call. */
async function viewCall(
  rpc: string,
  contract: string,
  method: string,
  args: unknown
): Promise<unknown> {
  const body = {
    jsonrpc: "2.0",
    id: "zxv-nep-gate",
    method: "query",
    params: {
      request_type: "call_function",
      finality: "final",
      account_id: contract,
      method_name: method,
      args_base64: toBase64(new TextEncoder().encode(JSON.stringify(args))),
    },
  };
  const res = await fetch(rpc, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  const json = await res.json();
  if (json.error) throw new Error(JSON.stringify(json.error));
  const bytes = json.result?.result as number[];
  if (!bytes) throw new Error("empty view result");
  return JSON.parse(new TextDecoder().decode(new Uint8Array(bytes)));
}

/**
 * The local, off-chain gate (the chain-meet, O(1) per pair).
 */
export class LocalGate {
  /** Conjoin evidence states; the chain-meet. */
  conjoin(inputs: number[]): number {
    if (inputs.length === 0) return State.NEITHER;
    let state = inputs[0];
    if (state > 3) return State.NEITHER; // fail-closed
    for (let i = 1; i < inputs.length; i++) {
      const v = inputs[i];
      if (v > 3) return State.NEITHER; // fail-closed
      if (RANK[v] < RANK[state]) state = v;
    }
    return state;
  }

  /** Only TRUE settles. */
  isSettleable(state: number): boolean {
    return state === State.TRUE;
  }

  /** The invariant. */
  invariant(): string {
    return "Only TRUE settles. TRUE+FALSE=BOTH — contradictions are recorded, never coerced, never a panic.";
  }
}

/**
 * The on-chain gate — gas-free view calls against the live deployment.
 */
export class ZxvGate extends LocalGate {
  private rpc: string;
  private contract: string;

  constructor(opts?: { rpc?: string; contract?: string }) {
    super();
    this.rpc = opts?.rpc ?? DEFAULT_RPC;
    this.contract = opts?.contract ?? DEFAULT_CONTRACT;
  }

  /** Verify on-chain: is this state settleable? (gas-free) */
  async isSettleableOnChain(state: string | number): Promise<boolean> {
    const s = typeof state === "number" ? ["NEITHER", "TRUE", "FALSE", "BOTH"][state] : state;
    const r = await viewCall(this.rpc, this.contract, "is_settleable", { state: s });
    return r === true;
  }

  /** Verify on-chain: the chain-meet of inputs. (gas-free) */
  async conjoinOnChain(inputs: number[]): Promise<number> {
    return (await viewCall(this.rpc, this.contract, "conjoin", { inputs })) as number;
  }

  /** The canonical router (zedec.near). */
  async route(): Promise<string> {
    return (await viewCall(this.rpc, "zedec.near", "route", {})) as string;
  }
}

export default ZxvGate;