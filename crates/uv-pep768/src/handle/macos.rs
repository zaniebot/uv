//! macOS remote memory I/O via Mach `mach_vm_read_overwrite`.
#![expect(unsafe_code, reason = "Mach memory access requires FFI")]
//!
//! `task_for_pid` requires the `com.apple.security.cs.debugger` entitlement
//! on a signed binary, or running as root. When the call returns
//! `KERN_FAILURE`, we surface [`AttachError::PermissionDenied`] so callers
//! can fall back to RSS-only sampling.

use std::io;

use mach2::kern_return::KERN_SUCCESS;
use mach2::mach_types::task_t;
use mach2::traps::{mach_task_self, task_for_pid};
use mach2::vm::mach_vm_read_overwrite;
use mach2::vm_types::{mach_vm_address_t, mach_vm_size_t};

use crate::AttachError;

pub(crate) struct Inner {
    task: task_t,
}

impl Inner {
    pub(crate) fn open(pid: u32) -> Result<Self, AttachError> {
        let pid = libc::pid_t::try_from(pid).map_err(|_| AttachError::PidNotFound(pid))?;
        let mut task: task_t = 0;
        // SAFETY: `task_for_pid` is FFI; the receiving target pointer is
        // valid for writes and the input pid/self are values.
        let kr = unsafe { task_for_pid(mach_task_self(), pid, &mut task) };
        if kr != KERN_SUCCESS {
            // KERN_FAILURE / KERN_INVALID_ARGUMENT both mean we can't
            // attach; the most actionable interpretation is permissions.
            return Err(AttachError::PermissionDenied(pid as u32));
        }
        Ok(Self { task })
    }

    pub(crate) fn read_at(&self, addr: u64, buf: &mut [u8]) -> io::Result<()> {
        if buf.is_empty() {
            return Ok(());
        }
        let mut out_size: mach_vm_size_t = 0;
        // SAFETY: `buf` points to a writable region of `buf.len()` bytes.
        // `mach_vm_read_overwrite` writes up to `buf.len()` bytes from the
        // target task's address space into `buf` and reports the actual
        // count via `out_size`.
        let kr = unsafe {
            mach_vm_read_overwrite(
                self.task,
                addr as mach_vm_address_t,
                buf.len() as mach_vm_size_t,
                buf.as_mut_ptr() as mach_vm_address_t,
                &mut out_size,
            )
        };
        if kr != KERN_SUCCESS {
            return Err(io::Error::other(format!(
                "mach_vm_read_overwrite failed: kr={kr}"
            )));
        }
        if (out_size as usize) != buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!(
                    "mach_vm_read_overwrite: short read ({out_size} of {} bytes)",
                    buf.len()
                ),
            ));
        }
        Ok(())
    }
}
