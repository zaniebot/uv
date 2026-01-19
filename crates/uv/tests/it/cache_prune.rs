use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;

use uv_test::uv_snapshot;

#[tokio::test]
async fn prune_force() -> Result<()> {
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
    uv_snapshot!(context.filters(), context.prune().arg("--verbose").arg("--force"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG uv [VERSION] ([COMMIT] DATE)
    DEBUG Acquired exclusive lock for `[CACHE_DIR]/`
    Pruning cache at: [CACHE_DIR]/
    No unused entries found
    DEBUG Released lock at `[CACHE_DIR]/.lock`
    ");

    // Add a stale directory to the cache.
    let simple = context.cache_dir.child("simple-v4");
    simple.create_dir_all()?;

    // When locked, `--force` should proceed without blocking
    let _cache = uv_cache::Cache::from_path(context.cache_dir.path())
        .with_exclusive_lock()
        .await;
    uv_snapshot!(context.filters(), context.prune().arg("--verbose").arg("--force"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG uv [VERSION] ([COMMIT] DATE)
    DEBUG Lock is busy for `[CACHE_DIR]/`
    DEBUG Cache is currently in use, proceeding due to `--force`
    Pruning cache at: [CACHE_DIR]/
    DEBUG Removing dangling cache bucket: [CACHE_DIR]/simple-v4
    Removed 1 directory
    ");

    Ok(())
}
