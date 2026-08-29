# Phase 7 — Final Checks and Docs

## Goal

Prove the three big promises (size, exactness, offline) on the real 30 GB
project, and write the user guides.

## What we must do

### T7.1 The scoreboard [Must] — Needs: T6.1

An automatic test that measures:
- **size:** 30 GB → under 10 MB (target 3 MB)
- **exactness:** unpacked project = original, byte for byte (SHA-256)
- **offline:** zero internet calls while unpacking

Done when:
- [ ] Scoreboard runs after every release
- [ ] All three numbers pass (or blockers are filed with owners)

### T7.2 Guides and runbooks [Must] — Needs: T6.1

Write:
- the `.hng` format spec (TRD §3)
- a simple user guide for pack and unpack
- rollback steps and an offline install runbook

Done when:
- [ ] Docs are saved and reviewed
- [ ] Runbooks are tested once in a practice run

## Phase done when

- [ ] All three promises verified on the 30 GB project
- [ ] Docs and runbooks are done and tested
