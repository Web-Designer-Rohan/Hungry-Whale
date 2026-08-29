# Phase 4 — Step 3: Binary-to-Math Vectorizer

## Goal

Turn media (`.mp4`, `.mp3`, `.png`) and AI weights (`.gguf`) into small math
seeds instead of raw bytes. This is the riskiest part — we test it **first**.

## What we must do

### T4.1 Small test: can math rebuild media? [Should] — Needs: T1.4

Try matrix factorization + Fourier math on PNG/MP3/MP4. Find out: can we
rebuild the file byte for byte from math seeds? Write a short report with
numbers, then say yes or no.

Done when:
- [ ] Report shows byte-exact vs. not-exact results per media type
- [ ] Clear yes/no answer with saved numbers
- [ ] Backup plan written (if no: ship the compressed raw file — still exact,
      just bigger)

### T4.2 AI weights → math seeds [Should] — Needs: T4.1 answer

Turn `.gguf` weights into matrix/math seeds. Goal: byte-exact.

Done when:
- [ ] Seeds rebuild the test weights byte for byte
- [ ] Seed size vs. raw size is measured and saved

### T4.3 Save Block 4 (seed stream) [Should] — Needs: T4.2, T1.2

Write Block 4 of the `.hng` file: the compressed math coefficients.

Done when:
- [ ] Block 4 follows the frozen format
- [ ] Coefficients compress well and unpack correctly
- [ ] The Rust side can read it (T5.1)

### T4.4 Check exactness and damage [Should] — Needs: T4.3

Round-trip media + weights. Check SHA-256. Also check: if a seed is damaged,
can we **detect** it during unpacking?

Done when:
- [ ] All test media/weights come back byte-exact (or the difference is written
      down)
- [ ] A damaged seed is caught at unpack time

## Phase done when

- [ ] Test result recorded with numbers
- [ ] Block 4 write/read round-trip works
- [ ] Exactness tests pass (or the gap is written down)
