#!/usr/bin/env bash
# verify_mainnet_gate.sh — the live RPC assertion harness.
#
# Proves the invariant on the deployed kernel, gas-free:
#   is_settleable(BOTH) -> false
#   is_settleable(TRUE) -> true
#   zedec.near route()  -> ok
#
# If any assertion fails, exit non-zero.
set -euo pipefail

RPC="${RPC:-https://rpc.mainnet.fastnear.com}"
CORE="f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf"
ROUTER="zedec.near"

# view account method args_b64 -> the DECODED string of the view result
view() {
  curl -s "$RPC" -H 'Content-Type: application/json' -d "{\"jsonrpc\":\"2.0\",\"id\":\"v\",\"method\":\"query\",\"params\":{\"request_type\":\"call_function\",\"finality\":\"final\",\"account_id\":\"$1\",\"method_name\":\"$2\",\"args_base64\":\"$3\"}}" \
    | python3 -c "
import sys, json
r = json.load(sys.stdin).get('result', {})
if 'result' in r:
    print(bytes(r['result']).decode('utf-8'))
else:
    print('ERR:' + str(r.get('error', r))[:80])
"
}

echo "=== live mainnet gate assertions ==="

# is_settleable(BOTH) must be false
BOTH=$(view "$CORE" is_settleable "eyJzdGF0ZSI6IkJPVEgifQ==")
[ "$BOTH" = "false" ] || { echo "FAIL: is_settleable(BOTH)=$BOTH (expected false)"; exit 1; }
echo "PASS: is_settleable(BOTH) -> false"

# is_settleable(TRUE) must be true
TRUE=$(view "$CORE" is_settleable "eyJzdGF0ZSI6IlRSVUUifQ==")
[ "$TRUE" = "true" ] || { echo "FAIL: is_settleable(TRUE)=$TRUE (expected true)"; exit 1; }
echo "PASS: is_settleable(TRUE) -> true"

# the router must answer (the result is a JSON string; strip the quotes)
ROUTE=$(view "$ROUTER" route "e30=" | tr -d '"')
case "$ROUTE" in
  canonical=*) echo "PASS: zedec.near route() -> ok" ;;
  *) echo "FAIL: zedec.near route()=$ROUTE"; exit 1 ;;
esac

echo "=== all live gate assertions passed (gas-free) ==="