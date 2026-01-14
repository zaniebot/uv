//! Unix-specific functionality for uv.

mod resource_limits;

pub use resource_limits::adjust_open_file_limit;
