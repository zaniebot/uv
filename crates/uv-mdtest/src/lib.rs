//! Markdown-based test framework for uv.
//!
//! This crate provides a way to write tests for uv commands in markdown format,
//! inspired by [ty's mdtest framework](https://github.com/astral-sh/ruff/tree/main/crates/ty_test).

pub mod parser;
pub mod runner;
pub mod snapshot;
pub mod types;

pub use parser::ParseError;
pub use runner::{
    Mismatch, MismatchKind, RunConfig, RunError, TestResult, get_platform_variables, run_rm_command,
};
pub use snapshot::{SnapshotMode, SnapshotUpdater};
pub use types::{
    AssertKind, ContentAssertion, CopyFrom, EnvironmentConfig, FilterConfig, MarkdownTest,
    MarkdownTestFile, PythonVersions, RequiredFeatures, TargetFamily, TargetOs, TestConfig,
    TreeConfig, TreeCreation, TreeEntry, TreeSnapshot,
};
