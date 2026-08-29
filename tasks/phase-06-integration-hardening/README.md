# Phase 6 — Put It All Together

## Goal

Join the parts into one working pipeline, add a friendly command-line screen,
make it safe against attacks, and tune it to the speed limits (TRD §4).

## What we must do

### T6.1 The full journey [Must] — Needs: Phases 2, 3, 5

One command: project → clean → dictionary → seeds → `.hng` → unpack → rebuild →
check. SHA-256 fingerprints are taken before and after.

Done when:
- [ ] Test project round-trips byte-exact
- [ ] One command runs the whole journey
- [ ] Fingerprints before and after match

### T6.2 Friendly command-line screen [Could] — Needs: T6.1

A simple screen that detects the machine (OS, runtimes) and guides the user.

Done when:
- [ ] Works on Windows and Linux
- [ ] Guides pack and unpack with progress shown

### T6.3 Safety against attacks [Must] — Needs: T5.6

Fuzz the `.hng` reader. Try bad headers, huge sizes, deep recursion, wrong
checksums. No memory overflow. No recursive attacks.

Done when:
- [ ] Fuzzing finds zero crashes
- [ ] All bad test files are refused safely

### T6.4 Speed tuning [Must] — Needs: T6.1

Check the limits from TRD §4: packing ≤32 GB RAM; unpacking ≤4 GB RAM in
10–15 min.

Done when:
- [ ] Packing memory measured ≤32 GB on the 30 GB project
- [ ] Unpacking measured ≤4 GB and within 10–15 min (or gap written down + plan)

## Phase done when

- [ ] One-command journey works with checks
- [ ] Fuzzing and attack tests pass
- [ ] Speed numbers saved against TRD §4
