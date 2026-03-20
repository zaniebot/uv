//! Mach-O executable detection for macOS.
//!
//! On macOS, code signature verification is cached per-inode (vnode). When using reflinks
//! (copy-on-write), each cloned file gets a new inode, requiring full signature re-validation
//! on first execution. This causes significant cold-start performance issues.
//!
//! The [`is_macos_executable`] predicate identifies files that benefit from inode preservation
//! (i.e., hardlinking instead of reflinking). It is intended for use with
//! [`LinkOptions::with_wants_preserved_inode_filter`](uv_fs::link::LinkOptions::with_wants_preserved_inode_filter).

use std::path::Path;

use goblin::mach::{fat, header};

/// Mach-O and universal-binary magic numbers, as big-endian byte arrays for direct comparison
/// against the first four bytes of a file.
const MACH_O_MAGICS: [[u8; 4]; 6] = [
    (header::MH_MAGIC).to_be_bytes(),
    (header::MH_CIGAM).to_be_bytes(),
    (header::MH_MAGIC_64).to_be_bytes(),
    (header::MH_CIGAM_64).to_be_bytes(),
    (fat::FAT_MAGIC).to_be_bytes(),
    (fat::FAT_CIGAM).to_be_bytes(),
];

/// Check if a file is a Mach-O executable by reading its magic bytes.
pub fn is_macos_executable(path: &Path) -> bool {
    use std::io::Read;

    let Ok(mut file) = std::fs::File::open(path) else {
        return false;
    };

    let mut buf = [0u8; 4];
    if file.read_exact(&mut buf).is_err() {
        return false;
    }

    MACH_O_MAGICS.contains(&buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.path().join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }

    #[test]
    fn test_by_magic_bytes() {
        let dir = TempDir::new().unwrap();

        // All six magic variants
        for (i, magic) in MACH_O_MAGICS.iter().enumerate() {
            let name = format!("bin_{i}");
            assert!(
                is_macos_executable(&create_file(&dir, &name, magic)),
                "MACH_O_MAGICS[{i}] should be detected"
            );
        }

        // Non-Mach-O
        assert!(!is_macos_executable(&create_file(
            &dir,
            "random",
            b"\x00\x01\x02\x03more"
        )));
        assert!(!is_macos_executable(&create_file(&dir, "empty", b"")));
        assert!(!is_macos_executable(&create_file(
            &dir,
            "short",
            b"\xCF\xFA"
        )));
    }

    #[test]
    fn test_extensionless_binary() {
        let dir = TempDir::new().unwrap();

        let mut content = header::MH_CIGAM_64.to_be_bytes().to_vec();
        content.extend_from_slice(b"rest of the binary");
        assert!(is_macos_executable(&create_file(&dir, "python3", &content)));

        assert!(!is_macos_executable(&create_file(
            &dir,
            "README",
            b"This is a readme"
        )));
    }

    #[test]
    fn test_nonexistent_file() {
        assert!(!is_macos_executable(&PathBuf::from(
            "/nonexistent/path/to/file"
        )));
    }
}
