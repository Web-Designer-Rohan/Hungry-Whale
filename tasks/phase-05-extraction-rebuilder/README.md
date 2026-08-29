# Phase 5 — The Unpacking Tool (Rust)

## Goal

Build the Rust tool that rebuilds the 30 GB project on an offline machine.
Works on **Windows and Linux**. Max 4 GB RAM, done in 10–15 minutes.

## What we must do

### T5.1 Read the .hng file safely [Must] — Needs: T1.2

Read all four blocks. Check the `HUNGRY_V1` header, the root SHA-256, and every
block checksum **before** unpacking anything. Fixed-size buffers reject bad or
hacked headers (TRD §5).

Done when:
- [ ] All four blocks read per the format
- [ ] Bad or oversized headers are refused — no crashes
- [ ] Root hash checked before any work starts

### T5.2 Rebuild text from the dictionary [Must] — Needs: T5.1, T3.3

Expand the dictionary and rebuild the text files from their pointers.

Done when:
- [ ] All test text files come back byte-identical
- [ ] Folder structure and overlaps are restored

### T5.3 Redraw media/AI from seeds [Should] — Needs: T5.1, T4.3

Run the CPU math to rebuild media and AI files from seeds. Memory stays under
4 GB (TRD §4).

Done when:
- [ ] Rebuilt files are byte-exact (wherever Step 3 is lossless)
- [ ] Memory never goes over 4 GB
- [ ] Time per file is measured against the 10–15 min budget

### T5.4 Rebuild the environment offline [Must] — Needs: T5.1, T2.3

Run the rebuild instructions using OS libraries already on the machine.
Rebuild `node_modules` / `.venv` with **zero** internet calls.

Done when:
- [ ] Environment rebuilds with 0 network calls (checked with a network guard)
- [ ] Installed versions match `hungry.manifest` exactly

### T5.5 Stop and rollback on damage [Must] — Needs: T5.2, T5.3, T5.4

Check every pointer and coefficient as we use it. If anything fails, stop at
once and roll back. Never leave a half-built project.

Done when:
- [ ] Damage tests: a broken block → clean stop + full rollback, no half state
- [ ] Rollback proven on the test project

### T5.6 One file for Windows and Linux [Must] — Needs: T5.5

Ship a single native program for Windows and one for Linux. No extra installs.

Done when:
- [ ] CI builds both
- [ ] Both run and unpack a test `.hng` on clean machines

## Phase done when

- [ ] Test `.hng` unpacks byte-exact, offline, within limits
- [ ] Damage + rollback tests pass
- [ ] Windows and Linux builds work
