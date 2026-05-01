//! Linux RSS reader via `/proc/<pid>/statm`.
//!
//! Field 1 of `statm` is "resident set size" in pages. The page size is
//! read once at startup via `sysconf(_SC_PAGESIZE)`.

use std::sync::OnceLock;

static PAGE_SIZE: OnceLock<u64> = OnceLock::new();

fn page_size() -> u64 {
    *PAGE_SIZE.get_or_init(|| {
        // SAFETY: `sysconf` is a thread-safe FFI call; an error returns -1
        // which we coerce to a sane fallback.
        #[expect(unsafe_code, reason = "sysconf has no safe binding in `libc`")]
        let raw = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        u64::try_from(raw).unwrap_or(4096)
    })
}

pub(crate) fn read_rss_bytes(pid: u32) -> Option<u64> {
    let path = format!("/proc/{pid}/statm");
    let contents = fs_err::read_to_string(&path).ok()?;
    let resident_pages: u64 = contents.split_whitespace().nth(1)?.parse().ok()?;
    Some(resident_pages.saturating_mul(page_size()))
}
