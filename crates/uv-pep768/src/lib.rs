//! PEP 768 safe external debugger client for CPython 3.14+.
//!
//! Attaches to a running CPython process and reads the current Python call
//! stack of each thread. No ptrace, no shelling out to py-spy.
//!
//! On 3.14+ this uses the cooperative attach surface introduced by PEP 768.
//! On platforms where the cooperative trigger is unavailable, this falls back
//! to a passive frame-walk via remote memory reads using the public
//! [`_Py_DebugOffsets`] symbol that CPython has exported since 3.12.
//!
//! [`_Py_DebugOffsets`]: https://github.com/python/cpython/blob/main/Include/internal/pycore_debug_offsets.h

use std::io;

mod frames;
mod handle;
mod image;
mod layout;
mod string_cache;

pub use frames::{Frame, MAX_STACK_DEPTH};

use crate::handle::ProcessHandle;
use crate::image::PythonImage;
use crate::layout::PyDebugOffsets;

/// A handle to a running CPython process from which Python frames can be
/// sampled.
pub struct Target {
    pid: u32,
    handle: ProcessHandle,
    offsets: PyDebugOffsets,
    runtime_addr: u64,
    strings: string_cache::StringCache,
}

impl Target {
    /// Attach to the CPython process with the given PID.
    pub fn attach(pid: u32) -> Result<Self, AttachError> {
        let handle = ProcessHandle::open(pid)?;
        let image = PythonImage::locate(pid, &handle)?;
        let offsets = PyDebugOffsets::read(&handle, image.debug_offsets_addr)?;
        offsets.validate()?;
        Ok(Self {
            pid,
            handle,
            offsets,
            runtime_addr: image.runtime_addr,
            strings: string_cache::StringCache::new(),
        })
    }

    /// Return the PID this target is attached to.
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// The CPython interpreter version the target reports.
    pub fn version(&self) -> PyVersion {
        self.offsets.version()
    }

    /// Sample the current Python call stack for each active thread.
    ///
    /// Returns one [`Vec<Frame>`] per thread, ordered top-of-stack first
    /// (innermost frame at index 0). The order of threads is interpreter
    /// order (typically the main thread first).
    pub fn sample_stacks(&mut self) -> Result<Vec<Vec<Frame>>, SampleError> {
        frames::sample(
            &self.handle,
            &self.offsets,
            self.runtime_addr,
            &mut self.strings,
        )
    }
}

/// CPython interpreter version reported by the target's `_Py_DebugOffsets`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PyVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl std::fmt::Display for PyVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Errors that can occur while attaching to a target process.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum AttachError {
    #[error("process {0} not found")]
    PidNotFound(u32),

    #[error("process {0} does not appear to be a CPython interpreter")]
    NotPython(u32),

    #[error(
        "CPython {found} is not supported by uv-pep768; \
         the safe attach interface requires CPython >= 3.14"
    )]
    UnsupportedVersion { found: PyVersion },

    #[error(
        "permission denied attaching to process {0}; \
         on macOS this requires the com.apple.security.cs.debugger entitlement \
         or running as root, on Linux it requires the same UID and a \
         permissive kernel.yama.ptrace_scope"
    )]
    PermissionDenied(u32),

    #[error(
        "could not locate the _Py_DebugOffsets symbol in process {pid}; \
         the libpython binary may be stripped or use an unsupported layout"
    )]
    DebugOffsetsNotFound { pid: u32 },

    #[error("memory access to process failed: {0}")]
    Io(#[from] io::Error),

    #[error("attaching is not supported on this platform")]
    Unsupported,
}

/// Errors that can occur while sampling stacks from an attached target.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum SampleError {
    #[error("memory access to target failed: {0}")]
    Io(#[from] io::Error),

    #[error("encountered an inconsistent CPython runtime layout while walking frames")]
    InconsistentLayout,

    #[error("target process exited during sampling")]
    TargetGone,
}
