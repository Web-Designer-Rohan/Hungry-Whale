# Technical Requirements Document (TRD) — Hungry-Whale

## 1. How the system works

Hungry-Whale uses a 3-step pipeline:

```
[ Raw 30 GB Workspace ]
        │
        ▼
[ Step 1: Manifest & Purge Engine ]  → removes caches, writes rebuild instructions
        │
        ▼
[ Step 2: Cross-File Pattern Matrix ]  → finds repeated text, keeps one copy
        │
        ▼
[ Step 3: Binary-to-Math Vectorizer ]  → turns AI weights and media into math seeds
        │
        ▼
[ .hng Compressed Package (~3 MB) ]
```

## 2. Parts of the system

### 2.1 Step 1: Manifest & Purge Engine
- **Job:** scan the folder, keep real source files, remove rebuildable caches
  (`node_modules`, `.venv`, `__pycache__`).
- **Made with:** Python 3.12.
- **Output:** a small JSON file (`hungry.manifest`) with package versions,
  dependency trees, and database schema outlines (`schema.sql`).

### 2.2 Step 2: Cross-File Pattern Matrix
- **Job:** read all text files (HTML, CSS, TypeScript, Rust, SQL) as one big
  pool. Text that repeats in many files is stored once; the rest point to it.
- **Made with:** a custom sliding-window dictionary that mixes LZMA2 and
  Zstandard.
- **Output:** one master dictionary. Duplicate text points to a single shared
  entry.

### 2.3 Step 3: Binary-to-Math Vectorizer
- **Job:** handle media (`.mp4`, `.mp3`, `.png`) and AI files (`.gguf`) that do
  not compress well as bytes.
- **Made with:** matrix factorization and Fourier transforms — blocks become
  math wave equations.
- **Output:** small math seeds and coefficients, not raw bytes.

### 2.4 The Extraction & Rebuilder Engine
- **Job:** rebuild the 30 GB workspace on an offline machine.
- **Made with:** Rust — fast, safe, and cross-platform.
- **Flow:** read the `.hng` header → expand the dictionary → run CPU math to
  redraw media and AI → run install scripts using OS libraries already on the
  machine.
- **Runs on:** Windows and Linux (one codebase, two builds).

## 3. The .hng file format

One file, four blocks:

| Bytes | Block | What it holds |
|---|---|---|
| 0x00 – 0x3F | Magic Header & Checksum | 64-byte `HUNGRY_V1` signature + root SHA-256 hash |
| 0x40 – 0xFF | Manifest Ledger | compressed JSON: layout, versions, file pointers |
| 0x100 – end | Cross-File Master Dictionary | one copy of repeated text + folder overlaps |
| end | Algebraic Seed Stream | compressed math coefficients for AI and media |

## 4. Speed and memory limits

**Compression (your machine)**
- Up to 32 GB RAM.
- Uses all CPU cores.
- Can take hours — that is fine.

**Extraction (target machine)**
- Max 4 GB RAM.
- Should finish in 10–15 minutes.

## 5. Safety, errors, checks

- **Before anything:** the engine makes a full SHA-256 list of the source folder.
- **If something fails:** any bad coefficient or pointer stops the unpacking at
  once and rolls back. No half-built workspace.
- **Buffer safety:** fixed-size buffers reject bad or hacked headers. No memory
  overflow, no recursive attacks.
