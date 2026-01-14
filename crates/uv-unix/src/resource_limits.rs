//! Helper for adjusting Unix resource limits.
//!
//! Linux has a historically low default limit of 1024 open file descriptors per process.
//! On modern multi-core machines, this can cause "too many open files" errors, particularly
//! when running operations like `uv pip install --compile` that spawn many subprocesses.
//!
//! This module attempts to raise the soft limit to the hard limit at startup to avoid these
//! errors without requiring users to manually configure their shell's `ulimit` settings.
//!
//! See: <https://github.com/astral-sh/uv/issues/16999>

use nix::sys::resource::{Resource, getrlimit, setrlimit};
use tracing::debug;

/// Attempt to raise the open file descriptor limit to the maximum allowed.
///
/// This function tries to set the soft limit to the hard limit. If the operation fails,
/// it silently ignores the error since the default limits may still be sufficient for
/// the current workload.
///
/// Returns the new soft limit on success, or the current soft limit if adjustment failed.
pub fn adjust_open_file_limit() -> u64 {
    let (soft, hard) = match getrlimit(Resource::RLIMIT_NOFILE) {
        Ok(limits) => limits,
        Err(err) => {
            debug!("Failed to get open file limit: {err}");
            return 0;
        }
    };

    debug!("Current open file limits: soft={soft}, hard={hard}");

    if soft >= hard {
        return soft;
    }

    // Try to raise the soft limit to the hard limit
    match setrlimit(Resource::RLIMIT_NOFILE, hard, hard) {
        Ok(()) => {
            debug!("Raised open file limit from {soft} to {hard}");
            hard
        }
        Err(err) => {
            debug!("Failed to raise open file limit from {soft} to {hard}: {err}");
            soft
        }
    }
}
