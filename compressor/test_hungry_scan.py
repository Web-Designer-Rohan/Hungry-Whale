"""Smoke tests for hungry_scan (run: python3 test_hungry_scan.py)."""

import hashlib
import json
import os
import shutil
import sys
import tempfile

sys.path.insert(0, os.path.dirname(__file__))
from hungry_scan import IGNORE_DIRS, scan

def make_fake_project(root):
    os.makedirs(os.path.join(root, "src", "node_modules", "left-pad"))
    os.makedirs(os.path.join(root, ".venv", "bin"))
    os.makedirs(os.path.join(root, "target", "debug"))
    files = {
        "src/main.py": b"print('hi')\n",
        "src/node_modules/left-pad/index.js": b"module.exports = 1\n",
        ".venv/bin/python": b"fake\n",
        "target/debug/app": b"ELF...",
        "assets/logo.png": b"\x89PNG\r\n\x1a\n",
    }
    for rel, data in files.items():
        p = os.path.join(root, rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        with open(p, "wb") as f:
            f.write(data)
    return files

def main():
    tmp = tempfile.mkdtemp(prefix="hungry_test_")
    try:
        files = make_fake_project(tmp)
        m = scan(tmp)
        paths = {f["path"] for f in m["files"]}

        # 1. Ignored dirs are never descended into
        assert "src/node_modules/left-pad/index.js" not in paths, "cache file scanned!"
        assert ".venv/bin/python" not in paths, ".venv scanned!"
        assert "target/debug/app" not in paths, "target/ scanned!"

        # 2. Sources and binaries are kept, with correct hashes
        assert "src/main.py" in paths and "assets/logo.png" in paths
        by_path = {f["path"]: f for f in m["files"]}
        want = hashlib.sha256(files["src/main.py"]).hexdigest()
        assert by_path["src/main.py"]["sha256"] == want, "sha256 mismatch"
        assert by_path["src/main.py"]["category"] == "source"
        assert by_path["assets/logo.png"]["category"] == "binary"

        # 3. Deterministic: same folder -> same manifest
        assert m == scan(tmp), "scan is not deterministic"

        # 4. Manifest is JSON-serializable (schema sanity)
        json.dumps(m)

        # 5. Every ignore name is classified
        assert all(v in {"cache", "vcs"} for v in IGNORE_DIRS.values())

        print("all 5 checks passed")
    finally:
        shutil.rmtree(tmp)

if __name__ == "__main__":
    main()
