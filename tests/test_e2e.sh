#!/usr/bin/env bash
# tests/test_e2e.sh — full end-to-end test (Phase 7 groundwork).
#
# 1. Generates a dummy ~1GB project: redundant text files (shared boilerplate
#    so the master dictionary + chunk dedupe have something to bite on) and
#    mock binaries (incompressible random data).
# 2. Runs the Python compressor (main.py) to produce a .hng file.
# 3. Runs the Rust extractor to unpack it into a fresh directory.
# 4. Asserts the extracted tree matches the source byte-for-byte (diff -r).
#
# Usage: bash tests/test_e2e.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="$(mktemp -d /tmp/hungry_e2e.XXXXXX)"
SRC="$WORK/src_project"
OUT="$WORK/out_project"
SIZE_MB="${E2E_SIZE_MB:-1000}"   # total text+binary payload, ~1GB default
BIN_MB="${E2E_BIN_MB:-200}"      # portion that is mock binary data

cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

echo "==> [1/5] Generating dummy project in $SRC (text ${SIZE_MB} MB total incl. ${BIN_MB} MB binaries)"
mkdir -p "$SRC/src" "$SRC/assets" "$SRC/node_modules/fake-pkg" "$SRC/.venv/bin"

# Shared boilerplate: every text file repeats it, so the cross-file
# dictionary and FastCDC dedupe have real redundancy to exploit.
BOILER="$WORK/boiler.txt"
{
  for i in $(seq 1 400); do
    echo "import { Component, OnInit } from '@angular/core';"
    echo "import { Observable, of } from 'rxjs';"
    echo "// Hungry-Whale boilerplate line $i — repeated across all text files"
  done
} > "$BOILER"

# Redundant text files: unique tail + shared boilerplate repeated to bulk up.
TEXT_MB=$((SIZE_MB - BIN_MB))
N_TEXT=8
UNIQ_MB=$((TEXT_MB / N_TEXT))
for i in $(seq 1 $N_TEXT); do
  {
    echo "// unique file $i"
    for r in $(seq 1 $((UNIQ_MB * 8))); do   # ~128KB per repeat-block unit
      cat "$BOILER"
      echo "export const uniqueToken_$i = $i;"
    done
  } > "$SRC/src/module_$i.ts"
done

# Mock binaries: incompressible random data.
for i in 1 2 3 4; do
  dd if=/dev/urandom of="$SRC/assets/blob_$i.bin" bs=1M count=$((BIN_MB / 4)) status=none
done

# Cache dirs that must be excluded from packing entirely.
echo "junk" > "$SRC/node_modules/fake-pkg/index.js"
echo "junk" > "$SRC/.venv/bin/python"

SRC_BYTES=$(du -sb "$SRC" | cut -f1)
echo "    source tree: $SRC_BYTES bytes"

echo "==> [2/5] Compressing to .hng"
python3 "$ROOT/compressor/main.py" "$SRC" --out "$WORK/test.hng"

HNG_BYTES=$(stat -c%s "$WORK/test.hng")
echo "    .hng size: $HNG_BYTES bytes ($((SRC_BYTES / HNG_BYTES))x smaller)"

echo "==> [3/5] Extracting with Rust rebuilder"
"$ROOT/extractor/target/release/hungry-extractor" \
  --container "$WORK/test.hng" --out "$OUT" \
  --manifest "$SRC/hungry.manifest" > "$WORK/extract.log"
tail -1 "$WORK/extract.log"

echo "==> [4/5] Verifying no staging dir remains"
if [ -e "$WORK/.hungry_temp_extract" ]; then
  echo "FAIL: .hungry_temp_extract left behind"; exit 1
fi

echo "==> [5/5] Byte-for-byte recursive diff (extracted vs source)"
diff -r "$SRC" "$OUT" --exclude=hungry.manifest
echo "PASS: extracted tree is byte-identical to source"
