use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;

use uv_cache::Cache;
use uv_static::EnvVars;

use uv_test::uv_snapshot;

#[tokio::test]
async fn clean_force() -> Result<()> {
    let context = uv_test::test_context!("3.12").with_filtered_counts();

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt.write_str("typing-extensions\niniconfig")?;

    // Install a requirement, to populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .assert()
        .success();

    // When unlocked, `--force` should still take a lock
    uv_snapshot!(context.filters(), context.clean().arg("--verbose").arg("--force"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG uv [VERSION] ([COMMIT] DATE)
    DEBUG Acquired exclusive lock for `[CACHE_DIR]/`
    Clearing cache at: [CACHE_DIR]/
    DEBUG Released lock at `[CACHE_DIR]/.lock`
    Removed [N] files ([SIZE])
    ");

    // Install a requirement, to re-populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .assert()
        .success();

    // When locked, `--force` should proceed without blocking
    let _cache = uv_cache::Cache::from_path(context.cache_dir.path())
        .with_exclusive_lock()
        .await;
    uv_snapshot!(context.filters(), context.clean().arg("--verbose").arg("--force"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG uv [VERSION] ([COMMIT] DATE)
    DEBUG Lock is busy for `[CACHE_DIR]/`
    DEBUG Cache is currently in use, proceeding due to `--force`
    Clearing cache at: [CACHE_DIR]/
    Removed [N] files ([SIZE])
    ");

    Ok(())
}

#[tokio::test]
async fn cache_timeout() {
    let context = uv_test::test_context!("3.12");

    // Simulate another uv process running and locking the cache, e.g., with a source build.
    let _cache = Cache::from_path(context.cache_dir.path())
        .with_exclusive_lock()
        .await;

    uv_snapshot!(context.filters(), context.clean().env(EnvVars::UV_LOCK_TIMEOUT, "1"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Cache is currently in-use, waiting for other uv processes to finish (use `--force` to override)
    error: Timeout ([TIME]) when waiting for lock on `[CACHE_DIR]/` at `[CACHE_DIR]/.lock`, is another uv process running? You can set `UV_LOCK_TIMEOUT` to increase the timeout.
    ");
}
