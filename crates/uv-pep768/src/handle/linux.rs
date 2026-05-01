//! Linux remote memory I/O via `process_vm_readv(2)`.
#![expect(unsafe_code, reason = "process_vm_readv requires raw FFI")]

use std::io;

use crate::AttachError;

pub(crate) struct Inner {
    pid: libc::pid_t,
}

impl Inner {
    pub(crate) fn open(pid: u32) -> Result<Self, AttachError> {
        let proc_path = format!("/proc/{pid}/status");
        match fs_err::metadata(&proc_path) {
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Err(AttachError::PidNotFound(pid));
            }
            Err(err) => return Err(AttachError::Io(err)),
        }

        let pid = libc::pid_t::try_from(pid).map_err(|_| AttachError::PidNotFound(pid))?;
        Ok(Self { pid })
    }

    pub(crate) fn read_at(&self, addr: u64, buf: &mut [u8]) -> io::Result<()> {
        if buf.is_empty() {
            return Ok(());
        }

        let local = libc::iovec {
            iov_base: buf.as_mut_ptr().cast::<libc::c_void>(),
            iov_len: buf.len(),
        };
        let remote = libc::iovec {
            iov_base: addr as *mut libc::c_void,
            iov_len: buf.len(),
        };

        // SAFETY: `local` and `remote` describe valid `iovec`s with matching
        // total lengths. The kernel reads `buf.len()` bytes from `addr` in
        // the target into `buf` in our address space; we treat short reads
        // as errors below.
        let read = unsafe {
            libc::process_vm_readv(
                self.pid,
                std::ptr::from_ref(&local),
                1,
                std::ptr::from_ref(&remote),
                1,
                0,
            )
        };
        if read < 0 {
            return Err(io::Error::last_os_error());
        }
        let read_unsigned = usize::try_from(read).map_err(|_| {
            io::Error::other(format!("process_vm_readv: negative byte count ({read})"))
        })?;
        if read_unsigned != buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!(
                    "process_vm_readv: short read ({read_unsigned} of {} bytes)",
                    buf.len()
                ),
            ));
        }
        Ok(())
    }
}
