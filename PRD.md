# Product Requirements Document (PRD) — Hungry-Whale

## 1. What is this?

Hungry-Whale is a tool that squeezes a huge 30 GB project into one small
3 MB file. Later, you can unpack that file and get your whole project back —
byte for byte. No internet needed. Nothing is lost.

## 2. Why do we need it?

Projects today are heavy. They carry big caches, AI model files, and large
media. Hungry-Whale does not store raw bytes. It stores smart instructions:
what to keep, what to rebuild, and how to rebuild it.

## 3. Who is it for?

- **The Air-Gapped Engineer** — works in secure offline networks and must carry
  heavy setups on small drives.
- **The Edge Deployment Specialist** — moves AI setups between low-bandwidth or
  offline machines.

## 4. How do we know it works?

- **Size:** 30 GB project → less than 10 MB file (target: 3 MB).
- **Exactness:** unpacked project is 100% the same as the source (SHA-256 check).
- **Offline:** zero internet calls while unpacking and booting.

## 5. What we will build

**Must Have**
- **Environment Blueprinting** — remove build caches (`node_modules`,
  `__pycache__`) and store simple rebuild instructions instead.
- **Cross-File Deduplication** — find repeated text across files, keep one copy.
- **Air-Gapped Extraction Engine** — a standalone tool that rebuilds everything
  offline.
- **Integrity Verification** — automatic SHA-256 checks before and after
  compress-extract.

**Should Have**
- **Mathematical Weight Seeding** — store AI model weights as small math seeds.
- **Media Wave Approximation** — store media as math equations, redraw later.

**Could Have**
- **Interactive CLI** — a friendly command screen that detects the target machine.

**Won't Have (v1)**
- No lossy changes to source code or working binaries.

## 6. Other promises

- **Reliability:** zero tolerance for corruption while mapping pointers.
- **Portability:** runs on Windows and Linux. No extra software needed.
- **Security:** safe file headers — no memory overflow, no recursive attacks
  while unpacking.

## 7. Assumptions

- The target machine has basic OS libraries and common runtimes. We use what is
  there; we do not carry it with us.
- We may use more CPU and memory while compressing, as long as the file stays
  tiny.
