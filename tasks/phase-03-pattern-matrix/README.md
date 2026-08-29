# Phase 3 — Step 2: Cross-File Pattern Matrix

## Goal

Read all text files (HTML, CSS, TypeScript, Rust, SQL) as one big pool. If the
same text appears in many files, keep it **once** and let the others point to
it. This becomes Block 3 of the `.hng` file.

## What we must do

### T3.1 Turn files into tokens [Must] — Needs: T1.4

Split every text file into small pieces (tokens) so we can compare files across
languages and folders.

Done when:
- [ ] All five formats (HTML, CSS, TS, Rust, SQL) tokenize correctly
- [ ] Same input always gives the same tokens
- [ ] Big files do not blow up memory

### T3.2 Build the shared dictionary [Must] — Needs: T3.1

Use a sliding window to find repeated text. Store one copy. Point everything
else to it. Uses all CPU cores; memory stays under 32 GB (TRD §4).

Done when:
- [ ] We can measure how much we saved; every repeat points to one copy
- [ ] It uses all cores and stays under 32 GB on the 30 GB project
- [ ] Same input always gives the same dictionary

### T3.3 Save Block 3 (dictionary) [Must] — Needs: T3.2, T1.2

Pack the dictionary + folder structure into Block 3 using LZMA2 + Zstandard.

Done when:
- [ ] Block 3 follows the frozen format
- [ ] Unpacking restores the dictionary and folder overlaps
- [ ] LZMA2/Zstd mix is tuned and tested

### T3.4 Text comes back exactly [Must] — Needs: T3.3

Pack all text files into Block 3 and unpack them. Every byte must come back the
same.

Done when:
- [ ] Every test text file is byte-identical after round-trip
- [ ] This check runs in CI

## Phase done when

- [ ] Dictionary built from the test project, savings measured
- [ ] Block 3 round-trip is byte-exact for all text
- [ ] Memory and CPU limits hold
