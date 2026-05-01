//! Stub for platforms where remote memory access is not yet implemented.

use std::io;

use crate::AttachError;

pub(crate) struct Inner;

impl Inner {
    pub(crate) fn open(_pid: u32) -> Result<Self, AttachError> {
        Err(AttachError::Unsupported)
    }

    pub(crate) fn read_at(&self, _addr: u64, _buf: &mut [u8]) -> io::Result<()> {
        Err(io::Error::other(
            "remote memory I/O is not supported on this platform",
        ))
    }
}
