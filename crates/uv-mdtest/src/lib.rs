//! Markdown-based test framework.
//!
//! A generic framework for writing tests in markdown format. The framework
//! handles parsing, config merging, command execution, and output comparison.
//! Application-specific config (e.g., Python versions, filters) is opaque
//! to the framework — the harness deserializes `TestConfig::raw` as needed.
//!
//! Inspired by [ty's mdtest framework](https://github.com/astral-sh/ruff/tree/main/crates/ty_test).

pub mod parser;
pub mod runner;
pub mod snapshot;
pub mod types;

pub use parser::{ParseError, is_directive_line};
pub use runner::{Mismatch, MismatchKind, RunError, TestResult, run_rm_command};
pub use snapshot::{SnapshotMode, SnapshotUpdater};
pub use types::{
    AssertKind, ContentAssertion, CopyFrom, MarkdownTest, MarkdownTestFile, TestConfig, TreeConfig,
    TreeCreation, TreeEntry, TreeSnapshot,
};
