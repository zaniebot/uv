//! Locate the loaded CPython binary in the target process and resolve the
//! addresses of `_Py_DebugOffsets` and `_PyRuntime`.

use std::path::{Path, PathBuf};

use crate::AttachError;
use crate::handle::ProcessHandle;

/// Symbol exported by CPython that contains the runtime field offsets used
/// for safe out-of-process introspection. Stable since CPython 3.12.
const DEBUG_OFFSETS_SYMBOL: &str = "_Py_DebugOffsets";

/// Symbol that points at the global [`_PyRuntimeState`] used by all
/// embedded interpreters. Stable since CPython 3.12.
const RUNTIME_SYMBOL: &str = "_PyRuntime";

/// Resolved location of the loaded Python image in the target.
pub(crate) struct PythonImage {
    /// Absolute address of `_Py_DebugOffsets` in the target's address space.
    pub(crate) debug_offsets_addr: u64,
    /// Absolute address of `_PyRuntime` in the target's address space.
    pub(crate) runtime_addr: u64,
}

impl PythonImage {
    pub(crate) fn locate(pid: u32, _handle: &ProcessHandle) -> Result<Self, AttachError> {
        let mappings = list_mappings(pid)?;
        let candidate =
            pick_python_mapping(&mappings).ok_or(AttachError::DebugOffsetsNotFound { pid })?;

        let bytes = fs_err::read(&candidate.file).map_err(AttachError::Io)?;

        let symbols = resolve_symbols(&bytes, &[DEBUG_OFFSETS_SYMBOL, RUNTIME_SYMBOL])
            .map_err(|_| AttachError::DebugOffsetsNotFound { pid })?;

        let debug_offsets_offset = symbols
            .get(DEBUG_OFFSETS_SYMBOL)
            .copied()
            .ok_or(AttachError::DebugOffsetsNotFound { pid })?;

        let runtime_offset = symbols
            .get(RUNTIME_SYMBOL)
            .copied()
            .ok_or(AttachError::DebugOffsetsNotFound { pid })?;

        Ok(Self {
            debug_offsets_addr: candidate.base.wrapping_add(debug_offsets_offset),
            runtime_addr: candidate.base.wrapping_add(runtime_offset),
        })
    }
}

#[derive(Debug)]
struct Mapping {
    /// The on-disk path of the mapped file.
    file: PathBuf,
    /// The load address of the mapping in the target's address space.
    base: u64,
    /// The path of the mapped file as a string, for filtering.
    file_string: String,
}

#[cfg(target_os = "linux")]
fn list_mappings(pid: u32) -> Result<Vec<Mapping>, AttachError> {
    use std::collections::HashMap;
    use std::io::BufRead;

    let path = format!("/proc/{pid}/maps");
    let file = fs_err::File::open(&path).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => AttachError::PidNotFound(pid),
        std::io::ErrorKind::PermissionDenied => AttachError::PermissionDenied(pid),
        _ => AttachError::Io(err),
    })?;
    let reader = std::io::BufReader::new(file);

    // First mapping wins per file: the lowest mapped address of each file
    // is the load base.
    let mut bases: HashMap<PathBuf, u64> = HashMap::new();
    for line in reader.lines() {
        let line = line.map_err(AttachError::Io)?;
        if let Some(parsed) = parse_maps_line(&line) {
            bases.entry(parsed.file).or_insert(parsed.base);
        }
    }

    Ok(bases
        .into_iter()
        .map(|(file, base)| Mapping {
            file_string: file.display().to_string(),
            file,
            base,
        })
        .collect())
}

#[cfg(target_os = "linux")]
fn parse_maps_line(line: &str) -> Option<ParsedMap> {
    let mut parts = line.split_whitespace();
    let range = parts.next()?;
    let _perms = parts.next()?;
    let _offset = parts.next()?;
    let _dev = parts.next()?;
    let _inode = parts.next()?;
    let path = parts.collect::<Vec<_>>().join(" ");
    if path.is_empty() {
        return None;
    }
    if path.starts_with('[') {
        // Anonymous mappings ([heap], [stack], [vdso], ...) - skip.
        return None;
    }
    let (start_str, _end_str) = range.split_once('-')?;
    let base = u64::from_str_radix(start_str, 16).ok()?;
    Some(ParsedMap {
        file: PathBuf::from(path),
        base,
    })
}

#[cfg(target_os = "linux")]
struct ParsedMap {
    file: PathBuf,
    base: u64,
}

#[cfg(target_os = "macos")]
fn list_mappings(_pid: u32) -> Result<Vec<Mapping>, AttachError> {
    // macOS dyld image discovery is implemented via `task_info` with
    // `TASK_DYLD_INFO`. Wiring that up requires more Mach FFI than
    // [`mach2`] currently exposes by default; track this as a follow-up
    // and surface a clear error so callers can degrade.
    Err(AttachError::DebugOffsetsNotFound { pid: 0 })
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn list_mappings(_pid: u32) -> Result<Vec<Mapping>, AttachError> {
    Err(AttachError::Unsupported)
}

fn pick_python_mapping(mappings: &[Mapping]) -> Option<&Mapping> {
    // Prefer a libpython shared library; fall back to a `python` executable
    // for static-linked builds.
    if let Some(m) = mappings
        .iter()
        .find(|m| is_libpython(Path::new(&m.file_string)))
    {
        return Some(m);
    }
    mappings
        .iter()
        .find(|m| is_python_executable(Path::new(&m.file_string)))
}

fn is_libpython(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    // libpython3.14.so, libpython3.14.so.1.0, libpython3.14.dylib, etc.
    if !name.starts_with("libpython") {
        return false;
    }
    let lower = name.to_ascii_lowercase();
    lower.contains(".so") || lower.contains(".dylib")
}

fn is_python_executable(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    // python, python3, python3.14, python3.14t (free-threaded), etc.
    name == "python"
        || name == "python3"
        || (name.starts_with("python3.")
            && name[8..]
                .chars()
                .all(|c| c.is_ascii_digit() || c == 't' || c == 'd'))
}

fn resolve_symbols(
    bytes: &[u8],
    wanted: &[&str],
) -> Result<std::collections::HashMap<String, u64>, ResolveError> {
    let mut out = std::collections::HashMap::new();
    match detect_format(bytes) {
        BinaryFormat::Elf => {
            let elf = goblin::elf::Elf::parse(bytes).map_err(|_| ResolveError::Parse)?;
            for sym in &elf.dynsyms {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    if wanted.contains(&name) {
                        out.insert(name.to_owned(), sym.st_value);
                    }
                }
            }
            for sym in &elf.syms {
                if let Some(name) = elf.strtab.get_at(sym.st_name) {
                    if wanted.contains(&name) && !out.contains_key(name) {
                        out.insert(name.to_owned(), sym.st_value);
                    }
                }
            }
        }
        BinaryFormat::MachO => {
            let mach = goblin::mach::Mach::parse(bytes).map_err(|_| ResolveError::Parse)?;
            let symbols = match mach {
                goblin::mach::Mach::Binary(macho) => macho.symbols,
                goblin::mach::Mach::Fat(_) => return Err(ResolveError::FatBinary),
            };
            if let Some(sym_iter) = symbols {
                for entry in &sym_iter {
                    let (name, nlist) = entry.map_err(|_| ResolveError::Parse)?;
                    // Mach-O symbols are prefixed with an underscore.
                    let stripped = name.strip_prefix('_').unwrap_or(name);
                    if wanted.contains(&stripped) {
                        out.insert(stripped.to_owned(), nlist.n_value);
                    }
                }
            }
        }
        BinaryFormat::Unknown => return Err(ResolveError::Format),
    }
    Ok(out)
}

#[derive(Debug)]
enum ResolveError {
    Parse,
    Format,
    FatBinary,
}

enum BinaryFormat {
    Elf,
    MachO,
    Unknown,
}

fn detect_format(bytes: &[u8]) -> BinaryFormat {
    if bytes.len() < 4 {
        return BinaryFormat::Unknown;
    }
    let magic = &bytes[..4];
    if magic == b"\x7fELF" {
        return BinaryFormat::Elf;
    }
    // 32-bit, 64-bit, and reverse-byte-order Mach-O magics, plus FAT magics.
    if matches!(
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        0xfeed_face | 0xfeed_facf | 0xcefa_edfe | 0xcffa_edfe | 0xcafe_babe | 0xbeba_feca
    ) {
        return BinaryFormat::MachO;
    }
    BinaryFormat::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_libpython_names() {
        assert!(is_libpython(Path::new("/usr/lib/libpython3.14.so")));
        assert!(is_libpython(Path::new("/usr/lib/libpython3.14.so.1.0")));
        assert!(is_libpython(Path::new("/usr/lib/libpython3.14.dylib")));
        assert!(!is_libpython(Path::new("/usr/lib/libc.so.6")));
        assert!(!is_libpython(Path::new("/bin/python3.14")));
    }

    #[test]
    fn detects_python_executables() {
        assert!(is_python_executable(Path::new("/usr/bin/python3.14")));
        assert!(is_python_executable(Path::new("/usr/bin/python3.14t")));
        assert!(is_python_executable(Path::new("/usr/bin/python3")));
        assert!(is_python_executable(Path::new("/usr/bin/python")));
        assert!(!is_python_executable(Path::new("/usr/bin/pip")));
        assert!(!is_python_executable(Path::new("/usr/bin/python-config")));
    }
}
