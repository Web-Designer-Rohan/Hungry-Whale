//! hungry-extractor — standalone rebuilder CLI (Phase 5 groundwork).
//!
//! Parses the HNG1 binary container, reconstitutes the directory structure and
//! file contents from chunk offsets, and asserts full SHA-256 verification on
//! all unpacked assets.
//!
//! Container format (little-endian) — see compressor/chunker/src/main.rs:
//!   Header (40 bytes):
//!     0x00  4   magic "HNG1"
//!     0x04  2   version u16 = 1
//!     0x06  2   flags u16
//!     0x08  4   file_count u32
//!     0x0C  4   chunk_count u32 (unique chunks)
//!     0x10  8   index_offset u64
//!     0x18  8   chunk_table_offset u64
//!     0x20  8   data_offset u64
//!   File index entry: path_len u16, path bytes, chunk_count u32, chunk refs u32 * n
//!   Chunk table entry (48 bytes): data_offset u64, comp_len u32, raw_len u32, sha256 [u8; 32]
//!   Chunk data: concatenated zstd frames, one per unique chunk.
//!
//! Usage:
//!   hungry-extractor --container <path> --out <dir> [--manifest <path>] [--verify-only]

use serde::Deserialize;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process;

use sha2::{Digest, Sha256};

const MAGIC: &[u8; 4] = b"HNG1";
const WRAPPER_MAGIC: &[u8; 16] = b"HUNGRY_V1\0\0\0\0\0\0\0";
const VERSION: u16 = 1;
const HEADER_SIZE: usize = 40;
const WRAPPER_HEADER_SIZE: usize = 64;
const CHUNK_TABLE_ENTRY: usize = 48;

/// If the file is a TRD §3 .hng wrapper (HUNGRY_V1 magic + root checksum +
/// zstd ledger + blocks), return the inner HNG1 container slice, after
/// verifying the root checksum over everything past the header.
fn unwrap_hng(data: &[u8]) -> &[u8] {
    if data.len() < WRAPPER_HEADER_SIZE || &data[0..16] != WRAPPER_MAGIC {
        return data;
    }
    let ledger_size = u64::from_le_bytes(data[0x30..0x38].try_into().unwrap()) as usize;
    let dict_size = u64::from_le_bytes(data[0x38..0x40].try_into().unwrap()) as usize;
    let inner_start = WRAPPER_HEADER_SIZE + ledger_size;
    let inner_len = dict_size + (data.len() - inner_start);
    if inner_start.checked_add(dict_size).unwrap_or(usize::MAX) > data.len() {
        die(".hng block sizes exceed file size — container corrupt");
    }
    let mut h = Sha256::new();
    h.update(&data[WRAPPER_HEADER_SIZE..]);
    let got: [u8; 32] = h.finalize().into();
    if got.as_slice() != &data[0x10..0x30] {
        die(".hng root SHA-256 mismatch — container corrupt");
    }
    println!(
        ".hng wrapper: root checksum OK, ledger {} bytes, dictionary {} bytes",
        ledger_size, dict_size
    );
    &data[inner_start..inner_start + inner_len]
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    process::exit(1);
}

fn le_u16(b: &[u8]) -> u16 {
    u16::from_le_bytes(b.try_into().unwrap())
}
fn le_u32(b: &[u8]) -> u32 {
    u32::from_le_bytes(b.try_into().unwrap())
}
fn le_u64(b: &[u8]) -> u64 {
    u64::from_le_bytes(b.try_into().unwrap())
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

struct Container<'a> {
    file_count: usize,
    chunk_count: usize,
    index: &'a [u8],
    chunk_table: &'a [u8],
    chunk_data: &'a [u8],
}

struct ChunkInfo {
    offset: usize,
    comp_len: usize,
    raw_len: usize,
    hash: [u8; 32],
}

/// Parse and validate the container structure. Every offset and length is
/// bounds-checked before use — a corrupt or hostile container is refused, not
/// crashed on (T5.1 / TRD §5).
fn parse_container(data: &[u8]) -> Container<'_> {
    if data.len() < HEADER_SIZE || &data[0..4] != MAGIC {
        die("not a HNG1 container (bad magic or truncated header)");
    }
    let version = le_u16(&data[4..6]);
    if version != VERSION {
        die(&format!("unsupported container version {version}"));
    }
    let file_count = le_u32(&data[8..12]) as usize;
    let chunk_count = le_u32(&data[12..16]) as usize;
    let index_offset = le_u64(&data[16..24]) as usize;
    let chunk_table_offset = le_u64(&data[24..32]) as usize;
    let data_offset = le_u64(&data[32..40]) as usize;

    // Offsets must be ordered and within the file.
    if index_offset < HEADER_SIZE
        || chunk_table_offset < index_offset
        || data_offset < chunk_table_offset
    {
        die("container offsets are out of order — container corrupt");
    }
    if data_offset > data.len() {
        die("data offset beyond end of container — container corrupt");
    }

    // Chunk table must fit exactly between its offset and the data offset.
    let table_len = chunk_count
        .checked_mul(CHUNK_TABLE_ENTRY)
        .unwrap_or_else(|| die("chunk count overflow — container corrupt"));
    if chunk_table_offset + table_len > data_offset {
        die("chunk table extends past data offset — container corrupt");
    }

    // Validate every chunk table entry up front: offsets in bounds, comp_len
    // within data area. (raw_len is validated after decompression.)
    for i in 0..chunk_count {
        let base = chunk_table_offset + i * CHUNK_TABLE_ENTRY;
        let comp_len = le_u32(&data[base + 8..base + 12]) as usize;
        let off = le_u64(&data[base..base + 8]) as usize;
        let start = data_offset
            .checked_add(off)
            .unwrap_or_else(|| die("chunk offset overflow — container corrupt"));
        if start.saturating_add(comp_len) > data.len() {
            die(&format!(
                "chunk {i}: data out of bounds — container corrupt"
            ));
        }
    }

    Container {
        file_count,
        chunk_count,
        index: &data[index_offset..chunk_table_offset],
        chunk_table: &data[chunk_table_offset..data_offset],
        chunk_data: &data[data_offset..],
    }
}

fn read_chunk_table(c: &Container) -> Vec<ChunkInfo> {
    let mut chunks = Vec::with_capacity(c.chunk_count);
    for i in 0..c.chunk_count {
        let base = i * CHUNK_TABLE_ENTRY;
        let entry = &c.chunk_table[base..base + CHUNK_TABLE_ENTRY];
        chunks.push(ChunkInfo {
            offset: le_u64(&entry[0..8]) as usize,
            comp_len: le_u32(&entry[8..12]) as usize,
            raw_len: le_u32(&entry[12..16]) as usize,
            hash: entry[16..48].try_into().unwrap(),
        });
    }
    chunks
}

/// Decompress and verify a single chunk by id. Returns the raw bytes.
fn load_chunk(c: &Container, chunks: &[ChunkInfo], id: u32) -> Vec<u8> {
    if id as usize >= chunks.len() {
        die(&format!(
            "file references unknown chunk id {id} — container corrupt"
        ));
    }
    let ci = &chunks[id as usize];
    let start = ci.offset;
    if start.saturating_add(ci.comp_len) > c.chunk_data.len() {
        die(&format!(
            "chunk {id}: data out of bounds — container corrupt"
        ));
    }
    let comp = &c.chunk_data[start..start + ci.comp_len];
    let raw = zstd::stream::decode_all(comp)
        .unwrap_or_else(|e| die(&format!("chunk {id}: zstd decode failed: {e}")));
    if raw.len() != ci.raw_len {
        die(&format!(
            "chunk {id}: raw length mismatch (got {}, want {})",
            raw.len(),
            ci.raw_len
        ));
    }
    let got: [u8; 32] = Sha256::digest(&raw).into();
    if got.as_slice() != ci.hash {
        die(&format!("chunk {id}: SHA-256 mismatch — container corrupt"));
    }
    raw
}

/// Reject path traversal: absolute paths, `..` components, and Windows drive
/// prefixes are refused (TRD §5 — no recursive attacks).
fn safe_output_path(out_root: &Path, rel: &str) -> PathBuf {
    let rel_path = Path::new(rel);
    if rel_path.is_absolute() || rel.starts_with('/') || rel.contains(':') {
        die(&format!("unsafe absolute path in container: {rel}"));
    }
    for comp in rel_path.components() {
        match comp {
            Component::Normal(_) => {}
            _ => die(&format!("unsafe path component in container: {rel}")),
        }
    }
    let out = out_root.join(rel_path);
    // Belt and braces: prefix check after join.
    if !out.starts_with(out_root) {
        die(&format!("path escapes output directory: {rel}"));
    }
    out
}

/// Original scan manifest (hungry.manifest): per-file SHA-256 cross-reference.
#[derive(Deserialize)]
struct Manifest {
    #[serde(default)]
    files: Vec<ManifestFile>,
}

#[derive(Deserialize)]
struct ManifestFile {
    path: String,
    sha256: String,
}

fn load_manifest(path: &str) -> HashMap<String, String> {
    let raw = fs::read_to_string(path)
        .unwrap_or_else(|e| die(&format!("cannot read manifest {path}: {e}")));
    let manifest: Manifest =
        serde_json::from_str(&raw).unwrap_or_else(|e| die(&format!("bad manifest {path}: {e}")));
    manifest
        .files
        .into_iter()
        .map(|f| (f.path, f.sha256.to_lowercase()))
        .collect()
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  \
         hungry-extractor --container <path> --out <dir> [--manifest <path>] [--verify-only]\n\n\
         Parses the HNG1 container, reconstitutes the directory tree from chunk\n\
         offsets, and asserts full SHA-256 verification on all unpacked assets.\n\
         Extraction is atomic: files are staged in .hungry_temp_extract and only\n\
         moved into place once everything verifies; any failure rolls back.\n\
         --manifest <path> cross-checks unpacked file hashes against the scan manifest.\n\
         --verify-only checks every chunk without writing files."
    );
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let get = |key: &str| -> Option<String> {
        args.iter()
            .position(|a| a == key)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
    }
    let container_path =
        get("--container").unwrap_or_else(|| die("--container <path> is required"));
    let verify_only = args.iter().any(|a| a == "--verify-only");
    let manifest_path = get("--manifest");
    let out_root = if verify_only {
        PathBuf::new()
    } else {
        PathBuf::from(get("--out").unwrap_or_else(|| die("--out <dir> is required")))
    };

    let raw_file = fs::read(&container_path)
        .unwrap_or_else(|e| die(&format!("cannot read {}: {e}", container_path)));
    let data = unwrap_hng(&raw_file);
    let container = parse_container(data);
    let chunks = read_chunk_table(&container);

    // Layer 1 — decompress and SHA-256-verify every unique chunk BEFORE
    // anything is written (T5.1: verify first, unpack second).
    let mut chunk_cache: HashMap<u32, Vec<u8>> = HashMap::with_capacity(chunks.len());
    for id in 0..chunks.len() {
        let raw = load_chunk(&container, &chunks, id as u32);
        chunk_cache.insert(id as u32, raw);
    }
    println!(
        "layer 1: verified {}/{} unique chunks OK",
        chunks.len(),
        chunks.len()
    );
    if verify_only {
        println!("verify-only: no files written");
        return;
    }

    // Optional cross-reference source: the original scan manifest.
    let manifest = manifest_path.as_deref().map(load_manifest);

    // Atomic rollback (TRD §5 / T5.5): stage everything in a hidden temp dir
    // next to the destination; only rename it into place once EVERY file has
    // passed both validation layers.
    let parent = out_root.parent().unwrap_or_else(|| Path::new("."));
    let temp = parent.join(".hungry_temp_extract");
    if temp.exists() {
        fs::remove_dir_all(&temp)
            .unwrap_or_else(|e| die(&format!("cannot remove stale {}: {e}", temp.display())));
    }
    fs::create_dir_all(&temp)
        .unwrap_or_else(|e| die(&format!("cannot create {}: {e}", temp.display())));

    // Any failure from here on tears down the staging dir and exits 1.
    let rollback = |msg: String| -> ! {
        let _ = fs::remove_dir_all(&temp);
        eprintln!("error: {msg}");
        eprintln!(
            "rollback: removed {} — destination untouched",
            temp.display()
        );
        process::exit(1);
    };

    // Layer 2 — reassemble each file from its chunk refs, hash it, and (with
    // --manifest) cross-reference against the source hash. Files land in the
    // staging dir only.
    let mut index = container.index;
    let mut extracted: u32 = 0;
    let mut matched: HashSet<String> = HashSet::new();
    for _file in 0..container.file_count {
        if index.len() < 2 {
            rollback("file index truncated — container corrupt".into());
        }
        let path_len = le_u16(&index[0..2]) as usize;
        index = &index[2..];
        if index.len() < path_len + 4 {
            rollback("file index truncated — container corrupt".into());
        }
        let rel = String::from_utf8(index[..path_len].to_vec()).unwrap_or_else(|_| {
            rollback("file path is not valid UTF-8 — container corrupt".into())
        });
        index = &index[path_len..];
        let chunk_count = le_u32(&index[0..4]) as usize;
        index = &index[4..];
        if index.len() < chunk_count * 4 {
            rollback("file index truncated — container corrupt".into());
        }

        let mut content: Vec<u8> = Vec::new();
        for i in 0..chunk_count {
            let id = le_u32(&index[i * 4..i * 4 + 4]);
            match chunk_cache.get(&id) {
                Some(raw) => content.extend_from_slice(raw),
                None => rollback(format!(
                    "file {rel}: unknown chunk id {id} — container corrupt"
                )),
            }
        }
        index = &index[chunk_count * 4..];

        // Layer 2: full SHA-256 assertion on the reassembled asset.
        let got: [u8; 32] = Sha256::digest(&content).into();
        let got_hex = hex_encode(&got);

        if let Some(map) = &manifest {
            match map.get(&rel) {
                Some(want) if *want == got_hex => {
                    matched.insert(rel.clone());
                }
                Some(want) => rollback(format!(
                    "SHA-256 mismatch for {rel}: got {got_hex}, manifest says {want}"
                )),
                None => rollback(format!(
                    "container file {rel} is not present in the manifest"
                )),
            }
        }

        let out_path = safe_output_path(&temp, &rel);
        if let Some(dir) = out_path.parent() {
            fs::create_dir_all(dir)
                .unwrap_or_else(|e| rollback(format!("cannot create {}: {e}", dir.display())));
        }
        let mut f = fs::File::create(&out_path)
            .unwrap_or_else(|e| rollback(format!("cannot create {}: {e}", out_path.display())));
        f.write_all(&content)
            .unwrap_or_else(|e| rollback(format!("cannot write {}: {e}", out_path.display())));

        println!(
            "extracted {:>7} bytes  {}  sha256:{got_hex}",
            content.len(),
            rel
        );
        extracted += 1;
    }

    if !index.is_empty() {
        rollback("file index has trailing bytes — container corrupt".into());
    }

    // Cross-reference completeness: every manifest file must have been unpacked.
    if let Some(map) = &manifest {
        let missing: Vec<&String> = map.keys().filter(|k| !matched.contains(*k)).collect();
        if !missing.is_empty() {
            rollback(format!(
                "{} manifest file(s) missing from container, e.g. {}",
                missing.len(),
                missing[0]
            ));
        }
        println!("layer 2: {} file hashes match manifest", matched.len());
    }

    // Everything verified — promote the staging dir atomically.
    if out_root.exists() {
        fs::remove_dir_all(&out_root).unwrap_or_else(|e| {
            rollback(format!(
                "cannot replace existing {}: {e}",
                out_root.display()
            ))
        });
    }
    fs::rename(&temp, &out_root).unwrap_or_else(|e| {
        rollback(format!(
            "cannot promote {} to {}: {e}",
            temp.display(),
            out_root.display()
        ))
    });
    println!("extracted {extracted} files to {}", out_root.display());
}
