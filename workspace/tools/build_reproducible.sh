#!/usr/bin/env bash
# build_reproducible.sh — the Docker/WASM reproducible build.
#
# Builds the kernel crate for the wasm32 target and writes the artifact
# + its SHA-256 checksum into kernel/artifacts/.
#
# For a fully reproducible CI build, run inside the pinned builder image:
#   docker run --rm -v "$PWD":/src -w /src rust:1.81 bash tools/build_reproducible.sh
set -euo pipefail
cd "$(dirname "$0")/.."

TARGET="${TARGET:-wasm32-unknown-unknown}"
OUT="kernel/artifacts"
mkdir -p "$OUT"

echo "=== building the kernel crate for $TARGET ==="
cargo build -p zxv-core-kernel --release --target "$TARGET"

WASM="target/$TARGET/release/zxv_core_kernel.wasm"
if [ ! -f "$WASM" ]; then
  echo "ERROR: $WASM not found"; exit 1
fi

cp "$WASM" "$OUT/zxv_near_contract.wasm"
SHA=$(shasum -a 256 "$OUT/zxv_near_contract.wasm" | awk '{print $1}')
echo "$SHA  zxv_near_contract.wasm" > "$OUT/checksums.sha256"

echo "=== artifact written ==="
echo "  kernel/artifacts/zxv_near_contract.wasm"
echo "  SHA-256: $SHA"
echo ""
echo "NOTE: this is the reference build of the deployed logic. Compare"
echo "the on-chain code hash (retrievable via the NEAR RPC contract_code"
echo "query) to this checksum for full provenance."