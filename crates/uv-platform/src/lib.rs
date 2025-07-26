//! Platform detection for operating system, architecture, and libc.

pub use crate::arch::{Arch, ArchVariant};
pub use crate::libc::{Libc, LibcDetectionError, LibcVersion};
pub use crate::os::Os;
pub use crate::platform::Error;

mod arch;
mod cpuinfo;
mod libc;
mod os;
mod platform;
