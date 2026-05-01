//! macOS RSS reader via `proc_pid_rusage`.
//!
//! `proc_pid_rusage(pid, RUSAGE_INFO_V2, ...)` returns
//! `rusage_info_v2.ri_resident_size` in bytes — exactly what we need with
//! no scaling. Failure is coerced to `None` so callers can fall back.

#![expect(unsafe_code, reason = "proc_pid_rusage has no safe binding")]

use libc::{c_int, c_void};

const RUSAGE_INFO_V2: c_int = 2;

#[repr(C)]
#[derive(Default)]
struct RusageInfoV2 {
    ri_uuid: [u8; 16],
    ri_user_time: u64,
    ri_system_time: u64,
    ri_pkg_idle_wkups: u64,
    ri_interrupt_wkups: u64,
    ri_pageins: u64,
    ri_wired_size: u64,
    ri_resident_size: u64,
    ri_phys_footprint: u64,
    ri_proc_start_abstime: u64,
    ri_proc_exit_abstime: u64,
    ri_child_user_time: u64,
    ri_child_system_time: u64,
    ri_child_pkg_idle_wkups: u64,
    ri_child_interrupt_wkups: u64,
    ri_child_pageins: u64,
    ri_child_elapsed_abstime: u64,
    ri_diskio_bytesread: u64,
    ri_diskio_byteswritten: u64,
}

unsafe extern "C" {
    fn proc_pid_rusage(pid: c_int, flavor: c_int, buffer: *mut c_void) -> c_int;
}

pub(crate) fn read_rss_bytes(pid: u32) -> Option<u64> {
    let pid = c_int::try_from(pid).ok()?;
    let mut info = RusageInfoV2::default();
    // SAFETY: `info` is a valid writable struct of the size proc_pid_rusage
    // expects for `RUSAGE_INFO_V2`.
    let rc = unsafe {
        proc_pid_rusage(
            pid,
            RUSAGE_INFO_V2,
            std::ptr::from_mut(&mut info).cast::<c_void>(),
        )
    };
    if rc != 0 {
        return None;
    }
    Some(info.ri_resident_size)
}
