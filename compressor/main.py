#!/usr/bin/env python3
"""compressor/main.py — Hungry-Whale compressor pipeline orchestrator.

Pipeline (TRD §1):
  1. Purge engine  (hungry_scan): scan the workspace, label cache targets
     (node_modules, .venv, target, __pycache__, .git), SHA-256 every kept
     file, and write hungry.manifest. (Cache deletion itself stays behind the
     T2.4 dry-run safety gate and is not done here.)
  2. Rust chunker  (compressor/chunker): FastCDC + zstd + cross-file chunk
     dedupe, invoked via subprocess; emits an HNG1 container file.
  3. Assemble the final .hng per TRD §3, strictly in block order:
       0x00-0x3F   64-byte header: "HUNGRY_V1" magic + root SHA-256 + sizes
       0x40-...    Block 2: zstd-compressed manifest ledger (starts right
                   after the header; the TRD's nominal 0xFF boundary only
                   holds if the compressed manifest fits in 192 bytes — the
                   header records the real size so readers can seek)
       ...         Block 3: Cross-File Master Dictionary (file index + chunk
                   table = the pointer/dedupe layer of the HNG1 container)
       end         Block 4: chunk stream (unique chunk payloads)

Header layout (64 bytes, little-endian):
  0x00  16  magic  b"HUNGRY_V1" + 7 zero bytes
  0x10  32  root SHA-256 over every byte after the header
  0x30   8  u64 ledger_size (compressed manifest bytes)
  0x38   8  u64 dict_size   (master dictionary bytes)

Usage:
  python3 main.py <workspace_root> [--out workspace.hng] [--manifest path]
                  [--min N] [--avg N] [--max N] [--level N] [--zstd-ledger N]
"""

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

try:  # preferred: python-zstandard
    import zstandard as _zstd

    def zstd_compress(data: bytes, level: int) -> bytes:
        return _zstd.ZstdCompressor(level=level).compress(data)

except ImportError:  # fallback: legacy python-zstd bindings
    try:
        import zstd as _zstd  # type: ignore

        def zstd_compress(data: bytes, level: int) -> bytes:
            return _zstd.compress(data, level)

    except ImportError:
        print("error: no zstd bindings found — pip install zstandard", file=sys.stderr)
        sys.exit(1)

sys.path.insert(0, str(Path(__file__).resolve().parent))
from hungry_scan import scan  # the purge engine (T2.1): fingerprint + label

MAGIC = b"HUNGRY_V1".ljust(16, b"\x00")
HEADER_SIZE = 64
COPY_CHUNK = 1 << 20  # 1 MiB streaming copies

HERE = Path(__file__).resolve().parent


def find_chunker() -> Path:
    for profile in ("release", "debug"):
        cand = HERE / "chunker" / "target" / profile / "hungry-chunker"
        if cand.is_file():
            return cand
    print(
        "error: chunker binary not found — build it with:\n"
        "  cd compressor/chunker && cargo build --release",
        file=sys.stderr,
    )
    sys.exit(1)


def run_purge_engine(root: Path, manifest_path: Path, out_path: Path) -> dict:
    """Step 1: scan the workspace and write hungry.manifest.

    The manifest and the .hng output are metadata, not source files — they are
    excluded so the manifest never contains a stale hash of itself.
    """
    manifest = scan(str(root))

    def excluded(p: Path) -> bool:
        try:
            rel = p.resolve().relative_to(root).as_posix()
        except ValueError:
            return False  # not inside the workspace
        return any(f["path"] == rel for f in manifest["files"])

    skip = {"path": set()}
    for p in (manifest_path, out_path):
        if excluded(p):
            try:
                skip["path"].add(p.resolve().relative_to(root).as_posix())
            except ValueError:
                pass
    if skip["path"]:
        manifest["files"] = [f for f in manifest["files"] if f["path"] not in skip["path"]]
        manifest["file_count"] = len(manifest["files"])
        print(f"      excluded from manifest: {sorted(skip['path'])}")

    manifest_path.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
    print(f"[1/3] purge engine: {manifest['file_count']} files -> {manifest_path}")
    return manifest


def run_chunker(
    manifest_path: Path, block_path: Path, min_s: int, avg_s: int, max_s: int, level: int
) -> None:
    """Step 2: invoke the Rust chunker on the manifest's valid files."""
    chunker = find_chunker()
    cmd = [
        str(chunker),
        "pack",
        "--manifest",
        str(manifest_path),
        "--out",
        str(block_path),
        "--min",
        str(min_s),
        "--avg",
        str(avg_s),
        "--max",
        str(max_s),
        "--level",
        str(level),
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True)
    if proc.returncode != 0:
        sys.stderr.write(proc.stdout + proc.stderr)
        print("error: chunker failed", file=sys.stderr)
        sys.exit(1)
    sys.stdout.write(proc.stdout.rstrip("\n") + "\n")
    print(f"[2/3] chunker: HNG1 container -> {block_path}")


def parse_container_data_offset(container_path: Path) -> int:
    """Split point inside the HNG1 container: dictionary (index+table) | chunk data."""
    header = container_path.read_bytes()[:40]
    if len(header) < 40 or header[:4] != b"HNG1":
        print("error: chunker output is not an HNG1 container", file=sys.stderr)
        sys.exit(1)
    return int.from_bytes(header[32:40], "little")


def copy_hashed(src, out, nbytes: int, h) -> None:
    """Stream exactly nbytes from src to out, updating the running hash."""
    left = nbytes
    while left > 0:
        chunk = src.read(min(COPY_CHUNK, left))
        if not chunk:
            raise IOError("source ended before expected block size")
        h.update(chunk)
        out.write(chunk)
        left -= len(chunk)


def assemble(out_path: Path, manifest_path: Path, container_path: Path, zstd_level: int) -> None:
    """Step 3: write the .hng file per TRD §3 block order."""
    manifest_bytes = manifest_path.read_bytes()
    ledger = zstd_compress(manifest_bytes, zstd_level)
    data_offset = parse_container_data_offset(container_path)
    container_size = container_path.stat().st_size
    dict_size = data_offset
    stream_size = container_size - data_offset

    root_hash = hashlib.sha256()
    with open(out_path, "wb") as out:
        out.write(b"\x00" * HEADER_SIZE)  # header placeholder, root hash patched below
        out.write(ledger)
        root_hash.update(ledger)
        with open(container_path, "rb") as c:
            copy_hashed(c, out, dict_size, root_hash)  # Block 3: master dictionary
            copy_hashed(c, out, stream_size, root_hash)  # Block 4: chunk stream

        header = bytearray(HEADER_SIZE)
        header[0:16] = MAGIC
        header[0x10:0x30] = root_hash.digest()
        header[0x30:0x38] = len(ledger).to_bytes(8, "little")
        header[0x38:0x40] = dict_size.to_bytes(8, "little")
        out.seek(0)
        out.write(header)

    total = HEADER_SIZE + len(ledger) + dict_size + stream_size
    print(
        f"[3/3] assembled {out_path.name}: header 64 + ledger {len(ledger)} "
        f"+ dictionary {dict_size} + chunk stream {stream_size} = {total} bytes"
    )


def verify_hng(out_path: Path) -> None:
    """Read back the .hng and assert magic, block sizes, and the root checksum."""
    size = out_path.stat().st_size
    with open(out_path, "rb") as f:
        header = f.read(HEADER_SIZE)
        if len(header) < HEADER_SIZE or header[0:16] != MAGIC:
            sys.exit("error: verify failed — bad HUNGRY_V1 magic")
        stored_root = header[0x10:0x30]
        ledger_size = int.from_bytes(header[0x30:0x38], "little")
        dict_size = int.from_bytes(header[0x38:0x40], "little")
        stream_size = size - HEADER_SIZE - ledger_size - dict_size
        if stream_size < 0:
            sys.exit("error: verify failed — block sizes exceed file size")

        h = hashlib.sha256()
        left = size - HEADER_SIZE
        while left > 0:
            chunk = f.read(min(COPY_CHUNK, left))
            h.update(chunk)
            left -= len(chunk)

    if h.digest() != stored_root:
        sys.exit("error: verify failed — root SHA-256 mismatch")
    print(
        f"verify OK: magic HUNGRY_V1, blocks ledger={ledger_size} "
        f"dict={dict_size} stream={stream_size}, root checksum matches"
    )


def main() -> None:
    ap = argparse.ArgumentParser(description="Hungry-Whale compressor orchestrator")
    ap.add_argument("root", help="workspace directory to compress")
    ap.add_argument("--out", default="hungry.hng", help="output .hng path")
    ap.add_argument("--manifest", default=None, help="manifest path (default: <root>/hungry.manifest)")
    ap.add_argument("--min", type=int, default=8192, help="FastCDC min chunk size")
    ap.add_argument("--avg", type=int, default=16384, help="FastCDC avg chunk size")
    ap.add_argument("--max", type=int, default=32768, help="FastCDC max chunk size")
    ap.add_argument("--level", type=int, default=3, help="zstd level for chunks")
    ap.add_argument("--zstd-ledger", type=int, default=19, help="zstd level for the manifest ledger")
    args = ap.parse_args()

    root = Path(args.root).resolve()
    if not root.is_dir():
        sys.exit(f"error: not a directory: {root}")
    out_path = Path(args.out)
    manifest_path = Path(args.manifest) if args.manifest else root / "hungry.manifest"
    block_path = out_path.with_suffix(out_path.suffix + ".container.tmp")

    try:
        run_purge_engine(root, manifest_path, out_path)
        run_chunker(manifest_path, block_path, args.min, args.avg, args.max, args.level)
        assemble(out_path, manifest_path, block_path, args.zstd_ledger)
        verify_hng(out_path)
    finally:
        block_path.unlink(missing_ok=True)

    raw = sum(f["size"] for f in json.loads(manifest_path.read_text())["files"])
    packed = out_path.stat().st_size
    ratio = raw / packed if packed else 0.0
    print(f"done: {raw} raw bytes -> {packed} byte .hng ({ratio:.1f}x smaller)")


if __name__ == "__main__":
    main()
