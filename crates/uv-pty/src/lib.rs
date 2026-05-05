// PTY file handles come from raw fd operations, not filesystem opens.
#![allow(clippy::disallowed_types)]
// PTY operations require extensive unsafe usage for fork/ioctl/dup2.
#![allow(unsafe_code)]
// Adapted from pixi_pty; some signatures pass by value intentionally.
#![allow(clippy::needless_pass_by_value)]
// eprintln is used in Drop impl where we can't propagate errors.
#![allow(clippy::print_stderr)]

#[cfg(unix)]
pub mod unix;
