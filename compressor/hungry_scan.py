"""hungry_scan.py — Phase 1/2 groundwork (T1.3 + T2.1, T2.3).

Recursively scans a target directory, computes a SHA-256 checksum for every
file (streamed, low memory), classifies ignore targets (node_modules, .venv,
target, __pycache__, .git), and writes a JSON manifest.

Usage:
    python hungry_scan.py <target_dir> [--out manifest.json]
"""

import argparse
import hashlib
import json
import os
import sys

# T2.1/T2.2 ignore targets: caches and VCS dirs are labelled, never hashed as sources.
IGNORE_DIRS = {
    "node_modules": "cache",
    ".venv": "cache",
    "venv": "cache",
    "__pycache__": "cache",
    "target": "cache",
    ".git": "vcs",
}

CHUNK_SIZE = 1 << 20  # 1 MiB streamed reads keep memory low on 30 GB trees


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(CHUNK_SIZE), b""):
            h.update(chunk)
    return h.hexdigest()


def classify(rel_path):
    """Return (category, ignore_reason). Sources are never caches."""
    parts = rel_path.replace("\\", "/").split("/")
    for p in parts[:-1]:
        if p in IGNORE_DIRS:
            return "ignored", IGNORE_DIRS[p]
    name = parts[-1]
    if name == "hungry_scan.py":
        return "source", None
    ext = os.path.splitext(name)[1].lower()
    if ext in {".py", ".rs", ".ts", ".js", ".json", ".md", ".toml", ".sql", ".html", ".css"}:
        return "source", None
    if ext in {".png", ".jpg", ".mp3", ".mp4", ".gguf", ".bin"}:
        return "binary", None
    return "other", None


def scan(root):
    root = os.path.abspath(root)
    entries = []
    for dirpath, dirnames, filenames in os.walk(root):
        # Prune ignored dirs in-place so os.walk never descends into them.
        dirnames[:] = sorted(d for d in dirnames if d not in IGNORE_DIRS)
        for fname in sorted(filenames):
            full = os.path.join(dirpath, fname)
            rel = os.path.relpath(full, root).replace("\\", "/")
            category, _ = classify(rel)
            entries.append({
                "path": rel,
                "sha256": sha256_file(full),
                "size": os.path.getsize(full),
                "category": category,
            })
    return {
        "format": "hungry-manifest-v1",
        "root": root,
        "algorithm": "sha256",
        "file_count": len(entries),
        "files": entries,
    }


def main():
    ap = argparse.ArgumentParser(description="Scan a folder into a hungry.manifest")
    ap.add_argument("target")
    ap.add_argument("--out", default="hungry.manifest")
    args = ap.parse_args()
    if not os.path.isdir(args.target):
        print(f"error: not a directory: {args.target}", file=sys.stderr)
        return 1
    manifest = scan(args.target)
    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)
    print(f"{manifest['file_count']} files -> {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
