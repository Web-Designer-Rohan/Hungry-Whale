# Hungry-Whale — Task Map

This is the map. The detailed tasks live in the phase folders below, each with
its own "done" checks.

## The 7 phases

| Phase | Folder | What it does | Tasks |
|---|---|---|---|
| 1 | [tasks/phase-01-foundations/](tasks/phase-01-foundations/README.md) | Setup: tools, file format, checks, test data | T1.1–T1.4 |
| 2 | [tasks/phase-02-manifest-purge-engine/](tasks/phase-02-manifest-purge-engine/README.md) | Step 1: scan, clean caches, make manifest | T2.1–T2.4 |
| 3 | [tasks/phase-03-pattern-matrix/](tasks/phase-03-pattern-matrix/README.md) | Step 2: find repeated text, build dictionary | T3.1–T3.4 |
| 4 | [tasks/phase-04-binary-to-math/](tasks/phase-04-binary-to-math/README.md) | Step 3: media and AI → math seeds | T4.1–T4.4 |
| 5 | [tasks/phase-05-extraction-rebuilder/](tasks/phase-05-extraction-rebuilder/README.md) | The unpacking tool (Rust, Windows + Linux) | T5.1–T5.6 |
| 6 | [tasks/phase-06-integration-hardening/](tasks/phase-06-integration-hardening/README.md) | Put it together, CLI, safety, speed | T6.1–T6.4 |
| 7 | [tasks/phase-07-acceptance/](tasks/phase-07-acceptance/README.md) | Final checks: size, exactness, offline | T7.1–T7.2 |

## Order of work

- Main road: Phase 1 → 2 → 3 → 5 → 6 → 7.
- Phase 4 runs side by side. Its first task (T4.1) is a small test: can math
  seeds really rebuild media and AI files byte for byte? If yes, we can reach
  3 MB. If no, we ship compressed raw files — still exact, just bigger.
- Hard limits (TRD §4): compression ≤32 GB RAM; extraction ≤4 GB RAM in
  10–15 min.
- Platforms: Windows and Linux. No Mac for now — the code stays portable, so
  adding Mac later is easy.
- We assume the target machine has basic OS libraries. We use them, we do not
  carry them.
