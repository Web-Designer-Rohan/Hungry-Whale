# Phase 2 — Step 1: Manifest & Purge Engine

## Goal

Scan the project folder, keep the real files, remove build caches, and write a
small JSON "shopping list" (`hungry.manifest`) that tells us how to rebuild
everything.

## What we must do

### T2.1 Scan and sort files [Must] — Needs: T1.4

Walk the folder and label every file: source, cache, or binary. First take the
SHA-256 fingerprint of everything (TRD §5), then sort.

Done when:
- [ ] The sorting rules are written down; no source file is ever called a cache
- [ ] We get a list of files to remove + a fingerprint snapshot
- [ ] It can scan 30 GB without running out of memory

### T2.2 Remove caches, keep rebuild info [Must] — Needs: T2.1

Delete caches (`node_modules`, `.venv`, `__pycache__`) and save what we need
to rebuild them: package files (`package.json`, `pyproject.toml`,
`Cargo.toml`), exact versions, dependency trees, and the database schema
(`schema.sql`).

Done when:
- [ ] Only caches are removed; sources stay safe (checked by fingerprints)
- [ ] Rebuild info has exact versions and the full dependency tree
- [ ] `schema.sql` is saved for database parts

### T2.3 Make hungry.manifest [Must] — Needs: T2.2

Write the small JSON file with the folder layout, versions, and file pointers.
This becomes Block 2 of the `.hng` file.

Done when:
- [ ] The JSON passes its schema check
- [ ] It has layout, versions, and file pointers
- [ ] It fits into Block 2 without losing anything

### T2.4 Dry-run mode (see before you delete) [Must] — Needs: T2.2

A preview mode: show exactly what will be deleted and how it will be rebuilt.
Nothing changes until you say "yes".

Done when:
- [ ] Preview mode changes nothing on disk
- [ ] Every planned delete is listed with its rebuild source
- [ ] If a fingerprint check fails, the tool stops safely

## Phase done when

- [ ] Test project cleans up well; sources stay byte-identical
- [ ] `hungry.manifest` is made and passes its schema
- [ ] Both preview and real delete work, and the safety stop works
