//! Mirror of CPython's `_Py_DebugOffsets` header.
//!
//! See [`Include/internal/pycore_debug_offsets.h`][header] in the CPython
//! source tree for the full layout. We mirror only the stable header here —
//! the cookie, version, and free-threaded flag — and validate that we're
//! looking at a CPython 3.14+ interpreter before consumers use any of the
//! field offsets contained in the rest of the struct.
//!
//! [header]: https://github.com/python/cpython/blob/main/Include/internal/pycore_debug_offsets.h

use crate::handle::ProcessHandle;
use crate::{AttachError, PyVersion};

/// CPython writes this fixed string at the start of `_Py_DebugOffsets` so
/// that out-of-process tools can confirm they've located the symbol
/// correctly.
const DEBUG_COOKIE: &[u8; 8] = b"xdebugpy";

/// First CPython release that ships the PEP 768-compatible safe attach
/// surface.
const MIN_VERSION: PyVersion = PyVersion {
    major: 3,
    minor: 14,
    patch: 0,
};

/// Maximum CPython release we know how to talk to. Bumped explicitly when
/// new releases are validated against this crate.
const MAX_VERSION: PyVersion = PyVersion {
    major: 3,
    minor: 15,
    patch: u8::MAX,
};

/// Stable header of `_Py_DebugOffsets`.
///
/// The bytes following this header are version-dependent sub-structs
/// (`runtime_state`, `interpreter_state`, `thread_state`, `frame`, `code`,
/// ...). Consumers must read those via the offsets the header advertises,
/// not via fixed Rust offsets — those layouts evolve between minor
/// releases.
#[derive(Debug, Clone, Copy)]
#[expect(
    dead_code,
    reason = "fields read once frame walking is fully implemented"
)]
pub(crate) struct PyDebugOffsets {
    cookie: [u8; 8],
    version_packed: u64,
    free_threaded: u64,
    /// Absolute address in the target's address space where the header was
    /// read from. Used as the base for the version-specific sub-struct
    /// reads that follow.
    pub(crate) header_addr: u64,
}

// SAFETY: plain old data; layout matches the CPython on-disk header.
#[expect(
    unsafe_code,
    reason = "PyDebugOffsetsRaw is repr(C) POD, safe for the Pod contract"
)]
unsafe impl crate::handle::Pod for PyDebugOffsetsRaw {}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PyDebugOffsetsRaw {
    cookie: [u8; 8],
    version: u64,
    free_threaded: u64,
}

impl PyDebugOffsets {
    pub(crate) fn read(handle: &ProcessHandle, addr: u64) -> Result<Self, AttachError> {
        let raw: PyDebugOffsetsRaw = handle.read(addr)?;
        Ok(Self {
            cookie: raw.cookie,
            version_packed: raw.version,
            free_threaded: raw.free_threaded,
            header_addr: addr,
        })
    }

    /// Parse the packed version field.
    ///
    /// CPython encodes its version using the [`PY_VERSION_HEX`][hex] format:
    /// `(major << 24) | (minor << 16) | (patch << 8) | (release << 4) | serial`.
    ///
    /// [hex]: https://docs.python.org/3/c-api/apiabiversion.html
    pub(crate) fn version(&self) -> PyVersion {
        let v = self.version_packed;
        PyVersion {
            major: ((v >> 24) & 0xff) as u8,
            minor: ((v >> 16) & 0xff) as u8,
            patch: ((v >> 8) & 0xff) as u8,
        }
    }

    #[expect(dead_code, reason = "consumed once frame walking is fully implemented")]
    pub(crate) fn is_free_threaded(&self) -> bool {
        self.free_threaded != 0
    }

    pub(crate) fn validate(&self) -> Result<(), AttachError> {
        if &self.cookie != DEBUG_COOKIE {
            return Err(AttachError::DebugOffsetsNotFound { pid: 0 });
        }
        let version = self.version();
        if version < MIN_VERSION || version > MAX_VERSION {
            return Err(AttachError::UnsupportedVersion { found: version });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(cookie: [u8; 8], version_packed: u64) -> PyDebugOffsets {
        PyDebugOffsets {
            cookie,
            version_packed,
            free_threaded: 0,
            header_addr: 0,
        }
    }

    #[test]
    fn parses_version() {
        // Python 3.14.0
        let h = header(*DEBUG_COOKIE, 0x030E_0000);
        assert_eq!(
            h.version(),
            PyVersion {
                major: 3,
                minor: 14,
                patch: 0
            }
        );
        // Python 3.15.2
        let h = header(*DEBUG_COOKIE, 0x030F_0200);
        assert_eq!(
            h.version(),
            PyVersion {
                major: 3,
                minor: 15,
                patch: 2
            }
        );
    }

    #[test]
    fn validates_cookie() {
        let h = header(*b"notpycky", 0x030E_0000);
        assert!(matches!(
            h.validate(),
            Err(AttachError::DebugOffsetsNotFound { .. })
        ));
    }

    #[test]
    fn rejects_old_versions() {
        let h = header(*DEBUG_COOKIE, 0x030D_0000); // 3.13
        assert!(matches!(
            h.validate(),
            Err(AttachError::UnsupportedVersion { .. })
        ));
    }

    #[test]
    fn accepts_supported_versions() {
        for hex in [0x030E_0000_u64, 0x030E_0500, 0x030F_0000] {
            let h = header(*DEBUG_COOKIE, hex);
            assert!(h.validate().is_ok(), "should accept hex={hex:#x}");
        }
    }
}
