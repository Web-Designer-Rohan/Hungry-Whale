use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use fastcdc::FastCDC;
use serde::Deserialize;
use sha2::{Digest, Sha256};

/// Container: header | file index | chunk table | chunk data (all little-endian)
///
/// Header (40 bytes):
///   0x00  4   magic "HNG1"
///   0x04  2   version u16 = 1
///   0x06  2   flags u16 = 0
///   0x08  4   file_count u32
///   0x0C  4   chunk_count u32 (unique chunks)
///   0x10  8   index_offset u64
///   0x18  8   chunk_table_offset u64
///   0x20  8   data_offset u64
///
/// File index entry: path_len u16, path bytes, chunk_count u32, chunk refs u32 * n
/// Chunk table entry (48 bytes): data_offset u64, comp_len u32, raw_len u32, sha256 [u8; 32]
/// Chunk data: concatenated zstd frames, one per unique chunk.
//
const MAGIC: &[u8; 4] = b"HNG1";
const VERSION: u16 = 1;
const HEADER_SIZE: usize = 40;
const CHUNK_TABLE_ENTRY: usize = 48;

#[derive(Deserialize)]
struct Manifest {
    root: String,
    #[serde(default)]
    files: Vec<ManifestFile>,
}

#[derive(Deserialize)]
struct ManifestFile {
    path: String,
    #[serde(default)]
    category: String,
}

struct ChunkEntry {
    offset: u64,
    comp_len: u32,
    raw_len: u32,
    hash: [u8; 32],
}

fn put_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}
fn put_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}
fn put_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    process::exit(1);
}

fn get_opt(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_num<T: std::str::FromStr>(args: &[String], key: &str, default: T) -> T {
    get_opt(args, key)
        .map(|v| {
            v.parse()
                .unwrap_or_else(|_| die(&format!("bad value for {key}")))
        })
        .unwrap_or(default)
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  \
         chunker pack --manifest <path> [--out <container>] [--min N] [--avg N] [--max N] [--level N]\n  \
         chunker info --container <path>\n  \
         chunker verify --container <path>\n\n\
         defaults: --min 8192 --avg 16384 --max 32768 --level 3 (zstd)"
    );
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("pack") => cmd_pack(&args[2..]),
        Some("info") => cmd_info(&args[2..]),
        Some("verify") => cmd_verify(&args[2..]),
        _ => usage(),
    }
}

fn cmd_pack(args: &[String]) {
    let manifest_path =
        get_opt(args, "--manifest").unwrap_or_else(|| die("pack: --manifest <path> is required"));
    let out_path = get_opt(args, "--out").unwrap_or_else(|| "container.hng".to_string());
    let min: usize = parse_num(args, "--min", 8192);
    let avg: usize = parse_num(args, "--avg", 16384);
    let max: usize = parse_num(args, "--max", 32768);
    let level: i32 = parse_num(args, "--level", 3);
    if !(min < avg && avg < max) {
        die("chunk sizes must satisfy min < avg < max");
    }

    let raw = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| die(&format!("cannot read manifest {manifest_path}: {e}")));
    let manifest: Manifest =
        serde_json::from_str(&raw).unwrap_or_else(|e| die(&format!("bad manifest: {e}")));
    let root = PathBuf::from(&manifest.root);

    let mut file_index: Vec<u8> = Vec::new();
    let mut chunks: Vec<ChunkEntry> = Vec::new();
    let mut chunk_data: Vec<u8> = Vec::new();
    let mut dedupe: HashMap<[u8; 32], u32> = HashMap::new();
    let mut file_count: u32 = 0;

    let mut total_raw: u64 = 0;
    let mut total_comp: u64 = 0;
    let mut dup_chunks: u64 = 0;
    let mut dup_bytes: u64 = 0;

    for mf in &manifest.files {
        if mf.category == "ignored" {
            continue;
        }
        let path = root.join(&mf.path);
        let data = fs::read(&path)
            .unwrap_or_else(|e| die(&format!("cannot read {}: {e}", path.display())));

        let mut refs: Vec<u32> = Vec::with_capacity(data.len() / avg + 1);
        for chunk in FastCDC::new(&data, min, avg, max) {
            let raw = &data[chunk.offset..chunk.offset + chunk.length];
            let hash: [u8; 32] = Sha256::digest(raw).into();
            total_raw += chunk.length as u64;

            let id = match dedupe.get(&hash) {
                Some(&id) => {
                    dup_chunks += 1;
                    dup_bytes += chunk.length as u64;
                    id
                }
                None => {
                    let comp = zstd::stream::encode_all(raw, level)
                        .unwrap_or_else(|e| die(&format!("zstd encode failed: {e}")));
                    chunk_data.extend_from_slice(&comp);
                    total_comp += comp.len() as u64;
                    chunks.push(ChunkEntry {
                        offset: chunk_data.len() as u64 - comp.len() as u64,
                        comp_len: comp.len() as u32,
                        raw_len: chunk.length as u32,
                        hash,
                    });
                    let id = (chunks.len() - 1) as u32;
                    dedupe.insert(hash, id);
                    id
                }
            };
            refs.push(id);
        }

        let path_bytes = mf.path.as_bytes();
        if path_bytes.len() > u16::MAX as usize {
            die(&format!("path too long: {}", mf.path));
        }
        put_u16(&mut file_index, path_bytes.len() as u16);
        file_index.extend_from_slice(path_bytes);
        put_u32(&mut file_index, refs.len() as u32);
        for r in &refs {
            put_u32(&mut file_index, *r);
        }
        file_count += 1;
    }

    // Assemble container: header | file index | chunk table | chunk data
    let mut out = Vec::with_capacity(
        HEADER_SIZE + file_index.len() + chunks.len() * CHUNK_TABLE_ENTRY + chunk_data.len(),
    );
    out.extend_from_slice(MAGIC);
    put_u16(&mut out, VERSION);
    put_u16(&mut out, 0); // flags
    put_u32(&mut out, file_count);
    put_u32(&mut out, chunks.len() as u32);
    out.extend_from_slice(&[0u8; 24]); // 3 x u64 offsets, patched below

    let index_offset = out.len() as u64;
    out.extend_from_slice(&file_index);

    let chunk_table_offset = out.len() as u64;
    for c in &chunks {
        put_u64(&mut out, c.offset);
        put_u32(&mut out, c.comp_len);
        put_u32(&mut out, c.raw_len);
        out.extend_from_slice(&c.hash);
    }

    let data_offset = out.len() as u64;
    out.extend_from_slice(&chunk_data);

    out[16..24].copy_from_slice(&index_offset.to_le_bytes());
    out[24..32].copy_from_slice(&chunk_table_offset.to_le_bytes());
    out[32..40].copy_from_slice(&data_offset.to_le_bytes());

    fs::write(&out_path, &out).unwrap_or_else(|e| die(&format!("cannot write {out_path}: {e}")));

    println!("files packed:        {file_count}");
    println!("unique chunks:       {}", chunks.len());
    println!("duplicate chunks:    {dup_chunks} ({dup_bytes} bytes deduped)");
    println!("raw chunk bytes:     {total_raw}");
    println!("compressed bytes:    {total_comp}");
    println!("container size:      {} bytes", out.len());
    println!(
        "saved by dedupe:     {} bytes",
        total_raw.saturating_sub(dup_bytes)
    );
}

struct Header {
    file_count: u32,
    chunk_count: u32,
    index_offset: usize,
    chunk_table_offset: usize,
    data_offset: usize,
    size: usize,
}

fn read_header(path: &str) -> Header {
    let data = fs::read(path).unwrap_or_else(|e| die(&format!("cannot read {path}: {e}")));
    if data.len() < HEADER_SIZE || &data[0..4] != MAGIC {
        die(&format!("{path} is not a HNG1 container"));
    }
    let version = u16::from_le_bytes(data[4..6].try_into().unwrap());
    if version != VERSION {
        die(&format!("unsupported container version {version}"));
    }
    Header {
        file_count: u32::from_le_bytes(data[8..12].try_into().unwrap()),
        chunk_count: u32::from_le_bytes(data[12..16].try_into().unwrap()),
        index_offset: u64::from_le_bytes(data[16..24].try_into().unwrap()) as usize,
        chunk_table_offset: u64::from_le_bytes(data[24..32].try_into().unwrap()) as usize,
        data_offset: u64::from_le_bytes(data[32..40].try_into().unwrap()) as usize,
        size: data.len(),
    }
}

fn cmd_info(args: &[String]) {
    let path =
        get_opt(args, "--container").unwrap_or_else(|| die("info: --container <path> required"));
    let h = read_header(&path);
    println!("container:    {path}");
    println!("size:         {} bytes", h.size);
    println!("files:        {}", h.file_count);
    println!("unique chunks: {}", h.chunk_count);
    println!(
        "index @ {}, chunk table @ {}, data @ {}",
        h.index_offset, h.chunk_table_offset, h.data_offset
    );
}

fn cmd_verify(args: &[String]) {
    let path =
        get_opt(args, "--container").unwrap_or_else(|| die("verify: --container <path> required"));
    let data = fs::read(&path).unwrap_or_else(|e| die(&format!("cannot read {path}: {e}")));
    if data.len() < HEADER_SIZE || &data[0..4] != MAGIC {
        die(&format!("{path} is not a HNG1 container"));
    }
    let chunk_count = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
    let chunk_table_offset = u64::from_le_bytes(data[24..32].try_into().unwrap()) as usize;
    let data_offset = u64::from_le_bytes(data[32..40].try_into().unwrap()) as usize;

    for i in 0..chunk_count {
        let base = chunk_table_offset + i * CHUNK_TABLE_ENTRY;
        if base + CHUNK_TABLE_ENTRY > data_offset {
            die("chunk table extends past data offset — container corrupt");
        }
        let off = u64::from_le_bytes(data[base..base + 8].try_into().unwrap()) as usize;
        let comp_len = u32::from_le_bytes(data[base + 8..base + 12].try_into().unwrap()) as usize;
        let raw_len = u32::from_le_bytes(data[base + 12..base + 16].try_into().unwrap()) as usize;
        let hash = &data[base + 16..base + 48];

        let start = data_offset + off;
        if start + comp_len > data.len() {
            die("chunk data out of bounds — container corrupt");
        }
        let raw = zstd::stream::decode_all(&data[start..start + comp_len])
            .unwrap_or_else(|e| die(&format!("chunk {i}: zstd decode failed: {e}")));
        if raw.len() != raw_len {
            die(&format!("chunk {i}: raw length mismatch"));
        }
        let got: [u8; 32] = Sha256::digest(&raw).into();
        if got.as_slice() != hash {
            die(&format!("chunk {i}: hash mismatch — container corrupt"));
        }
    }
    println!("verified {chunk_count}/{chunk_count} chunks OK");
}
