use anyhow::Result;
use assert_fs::prelude::*;

use uv_static::EnvVars;

use crate::common::{TestContext, uv_snapshot};

/// When the cache directory cannot be created (e.g., due to permissions), we should show a
/// chained error message that indicates we failed to initialize the cache.
#[test]
#[cfg(unix)]
fn cache_init_failure() -> Result<()> {
    use crate::common::ReadOnlyDirectoryGuard;

    // Skip if running as root, as root bypasses permission checks
    if nix::unistd::geteuid().is_root() {
        return Ok(());
    }

    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    // Create a read-only directory that will serve as the parent of the cache.
    // The guard sets it to read-only and restores original permissions on drop (including panic).
    let cache_parent = context.temp_dir.child("cache_parent");
    fs_err::create_dir(&cache_parent)?;
    let _guard = ReadOnlyDirectoryGuard::new(cache_parent.path())?;

    // Point the cache to a subdirectory within the read-only parent
    let cache_dir = cache_parent.child("cache");

    let mut filters = context.filters();
    filters.push((
        r"failed to create directory `.*`",
        "failed to create directory `[CACHE_DIR]`",
    ));

    // Running a command should fail with a chained error about cache initialization
    uv_snapshot!(&filters, context.sync().env(EnvVars::UV_CACHE_DIR, cache_dir.path()), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to initialize cache at `[CACHE_DIR]`
      Caused by: failed to create directory `[CACHE_DIR]`: Permission denied (os error 13)
    ");

    Ok(())
}
