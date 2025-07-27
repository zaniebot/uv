//! Tool-specific CLI functionality for uv.

pub mod common;
pub mod dir;
pub mod install;
pub mod list;
pub mod run;
pub mod uninstall;
pub mod update_shell;
pub mod upgrade;

mod target;

pub use target::{Target, ToolRequest};