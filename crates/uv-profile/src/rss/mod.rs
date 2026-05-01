//! Cross-platform resident-set-size reader for a remote process.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as imp;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as imp;

/// Read the current resident-set size of `pid` in bytes. Returns `None`
/// if the process has exited or the platform is unsupported.
pub(crate) fn read(pid: u32) -> Option<u64> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        imp::read_rss_bytes(pid)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = pid;
        None
    }
}
