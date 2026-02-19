//! Local mock index for packse scenario tests.
//!
//! This module provides a [`PackseServer`] that reads packse scenario TOML definitions
//! and serves a PEP 691 Simple API + wheel/sdist downloads via a local wiremock server.
//! Each test gets its own server instance, so package names need no prefix mangling.

mod scenario;
mod server;
mod wheel;

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub use server::PackseServer;

/// A shared [`PackseServer`] for the bird-themed general package index.
///
/// Lazily started on first access and kept alive for the entire test binary.
/// This avoids spending ~142ms per test on server startup.
static SHARED_BYPY: LazyLock<PackseServer> =
    LazyLock::new(|| PackseServer::for_packages("general.toml"));

/// Return the index URL of the shared bird-themed package server.
pub fn shared_bypy_index_url() -> String {
    SHARED_BYPY.index_url()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .expect("CARGO_MANIFEST_DIR should be nested under workspace root")
}

/// Base directory containing the vendored packse scenario TOML files.
fn scenarios_dir() -> PathBuf {
    workspace_root().join("test").join("scenarios")
}

/// Base directory containing general-purpose package index TOML files.
fn packages_dir() -> PathBuf {
    workspace_root().join("test").join("packages")
}

/// Base directory containing vendored build-dependency wheels (hatchling, etc.).
fn vendor_dir() -> PathBuf {
    workspace_root().join("test").join("vendor")
}
