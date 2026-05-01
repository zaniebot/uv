//! Cross-platform remote-process memory access.
#![expect(
    unsafe_code,
    reason = "remote-process memory access requires FFI and bit-cast reads"
)]

use std::io;

use crate::AttachError;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as imp;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as imp;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
mod unsupported;
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
use unsupported as imp;

/// A handle that allows reading from (and on supported platforms, writing
/// to) the address space of another process.
pub(crate) struct ProcessHandle {
    inner: imp::Inner,
}

impl ProcessHandle {
    pub(crate) fn open(pid: u32) -> Result<Self, AttachError> {
        Ok(Self {
            inner: imp::Inner::open(pid)?,
        })
    }

    /// Read exactly `buf.len()` bytes from `addr` in the target's address
    /// space into `buf`. Short reads are surfaced as errors.
    pub(crate) fn read_at(&self, addr: u64, buf: &mut [u8]) -> io::Result<()> {
        self.inner.read_at(addr, buf)
    }

    /// Read a fixed-size value from `addr`.
    pub(crate) fn read<T: Pod>(&self, addr: u64) -> io::Result<T> {
        let mut value = T::zeroed();
        // SAFETY: `T: Pod` is plain old data; any byte pattern is a valid
        // representation, and we initialise the entire value via `read_at`
        // before observing it.
        let buf = unsafe {
            std::slice::from_raw_parts_mut(
                std::ptr::from_mut(&mut value).cast::<u8>(),
                std::mem::size_of::<T>(),
            )
        };
        self.read_at(addr, buf)?;
        Ok(value)
    }
}

/// Marker for "plain old data" types that can be safely zero-initialised
/// and bit-cast from arbitrary byte patterns. Not a public trait.
///
/// # Safety
///
/// Implementors must be `#[repr(C)]` (or transparent over a primitive),
/// must contain no padding observed via uninit reads, and must not have
/// any invalid bit patterns.
pub(crate) unsafe trait Pod: Sized + Copy {
    fn zeroed() -> Self {
        // SAFETY: implementor's safety contract requires that any byte
        // pattern, including all-zeros, is a valid value of `Self`.
        unsafe { std::mem::zeroed() }
    }
}

// SAFETY: integer primitives accept any bit pattern.
unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}

// SAFETY: arrays of `Pod` are `Pod`.
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}
