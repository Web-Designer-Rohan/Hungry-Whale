## Technical Requirements Document (TRD): Project Hungry## 1. System Architecture Overview
Project Hungry uses a 3-Tier Asymmetric Pipeline Architecture. It separates data processing into ingestion, semantic reduction, and deterministic reconstruction.

[ Raw 30 GB Workspace ] 
         │
         ▼
[ Tier 1: Manifest & Purge Engine ] ──► Strips transient caches / generates rebuild specs
         │
         ▼
[ Tier 2: Cross-File Pattern Matrix ] ──► Cross-references syntax and builds single-instance dictionaries
         │
         ▼
[ Tier 3: Binary-to-Math Vectorizer ] ──► Transforms high-entropy AI weights & media into algebraic seeds
         │
         ▼
[ .hng Compressed Master Package (~3 MB) ]

------------------------------
## 2. Component Specifications## 2.1 Tier 1: The Manifest & Purge Engine

* Responsibility: Scans the target directory, isolates root source files, and obliterates rebuildable transient caches (node_modules, .venv, __pycache__).
* Implementation: Python 3.12 core orchestration script.
* Output: Generates a lightweight JSON configuration manifest (hungry.manifest) containing package versions, dependency trees, and database schema outlines (schema.sql).

## 2.2 Tier 2: The Cross-File Pattern Matrix

* Responsibility: Eliminates file-boundary isolation. It indexes all plain text and structured source files (HTML, CSS, TypeScript, Rust, SQL dumps) into a global memory space.
* Implementation: Custom sliding-window dictionary builder using a modified LZMA2/Zstandard hybrid approach.
* Output: A unified master token dictionary where duplicate text segments across different folders point to a single physical memory address.

## 2.3 Tier 3: The Binary-to-Math Vectorizer

* Responsibility: Handles uncompressible media (.mp4, .mp3, .png) and AI model files (.gguf).
* Implementation: Uses localized matrix factorization and Fourier transform algorithms to approximate binary blocks as mathematical wave equations.
* Output: Compact coordinate seeds and algebraic coefficients rather than raw static pixels or weight arrays.

## 2.4 The Extraction and Rebuilder Engine

* Responsibility: Reconstructs the 30 GB environment on an air-gapped target machine without network access.
* Implementation: Native executable binary written in Rust for lightning-fast memory allocation and file-system writing.
* Execution Flow: Parses the .hng header, expands the unified dictionary, runs local CPU wave calculations to render media/AI seeds, and executes local manifest installation scripts using pre-cached OS libraries.

------------------------------
## 3. Data Schema: The .hng File Format Specification
The proprietary archive format (.hng) is structured into four distinct sequential byte blocks:

| Byte Offset | Block Name | Description |
|---|---|---|
| 0x00 - 0x3F | Magic Header & Checksum | 64-byte signature HUNGRY_V1 followed by root SHA-256 validation hash. |
| 0x40 - 0xFF | Manifest Ledger | Compressed JSON block defining environmental layout, dependency versions, and file pointers. |
| 0x100 - Variable | Cross-File Master Dictionary | Single-instance shared text patterns and overlapping directory structures. |
| Variable - End | Algebraic Seed Stream | Compressed mathematical coefficients and coordinate functions for AI/Media synthesis. |

------------------------------
## 4. Performance, Memory, and CPU Constraints

* Compression Phase (Host Machine):
* Max memory allocation: Up to 32 GB RAM allowed during global pattern matrix calculation.
   * CPU utilization: Multi-threaded (100% capacity across all available CPU cores).
   * Time limit: Non-real-time; compression can run for several hours to achieve maximum entropy reduction.
* Decompression Phase (Target Machine):
* Max memory allocation: Capped at 4 GB RAM to ensure compatibility with low-spec offline machines.
   * Extraction time: Target completion within 10 to 15 minutes of intensive local CPU calculation.

------------------------------
## 5. Security, Error Handling, & Integrity Verification

* Pre-Processing Validation: The engine computes a recursive SHA-256 manifest of the 30 GB source directory before executing any purge or transform operations.
* Corrupted Packet Defense: If any mathematical coefficient or dictionary pointer fails local validation during unpacking, the extraction routine halts instantly and rolls back changes to prevent a partial or corrupted workspace state.
* Buffer Overflow Prevention: Fixed-size memory buffers are enforced in the Rust extraction runtime to reject malformed or maliciously inflated header overrides.
