# Phase 1 — Setup and Groundwork

## Goal

Get everything ready before we write the real code: the folders, the file
format, the checks, and the test data.

## What we must do

### T1.1 Make the project skeleton [Must] — Needs: nothing

Make two folders:
- `compressor/` — Python 3.12 (this packs the project)
- `extractor/` — Rust (this unpacks it)

Set up tests, linting, and CI. CI must build and test on **both Windows and
Linux**.

Done when:
- [ ] `python -m pytest` and `cargo test` pass in CI
- [ ] Both Windows and Linux builds work
- [ ] Both sides share one file of constants (magic bytes, offsets, checksum type)

### T1.2 Freeze the .hng file format [Must] — Needs: nothing

Write down the exact file format (see TRD §3) so both sides build against the
same rules.

| Bytes | Block | What it holds |
|---|---|---|
| 0x00 – 0x3F | Magic Header & Checksum | `HUNGRY_V1` + root SHA-256 hash |
| 0x40 – 0xFF | Manifest Ledger | compressed JSON: layout, versions, file pointers |
| 0x100 – end | Cross-File Master Dictionary | one copy of repeated text + folder overlaps |
| end | Algebraic Seed Stream | math coefficients for AI and media |

Done when:
- [ ] The spec is written and saved (version 1)
- [ ] Tests check the magic bytes and block offsets
- [ ] Every block has a checksum plan (needed for the stop-and-rollback rule)

### T1.3 SHA-256 file fingerprint walker [Must] — Needs: T1.1

A tool that walks the whole source folder and gives every file a SHA-256
fingerprint. Run it **before** any cleanup (TRD §5). It must handle 30 GB
without using too much memory.

Done when:
- [ ] Same folder always gives the same list
- [ ] Big files are read in small pieces (memory stays low)
- [ ] Python and Rust can both use it

### T1.4 Sample test project [Must] — Needs: T1.1

Build a small fake project for testing that has a bit of everything:
TypeScript, Rust, HTML, CSS, SQL, a fake `node_modules` and `.venv`, some
PNG/MP3/MP4, and a small `.gguf` AI file.

Done when:
- [ ] The sample can be recreated any time
- [ ] It has every kind of file the real tool must handle
- [ ] Its SHA-256 fingerprint list is saved

## Phase done when

- [ ] File format is fixed; both sides use the same constants
- [ ] Both toolchains build on Windows and Linux
- [ ] Test project and its fingerprints are ready
