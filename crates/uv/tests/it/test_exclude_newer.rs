use anyhow::Result;
use assert_fs::prelude::*;

use crate::common::{TestContext, uv_snapshot};

/// Test that exclude-newer in pyproject.toml doesn't update existing lockfile
#[test]
fn lock_exclude_newer_pyproject_no_update() -> Result<()> {
    let context = TestContext::new("3.12");
    let pyproject_toml = context.temp_dir.child("pyproject.toml");

    // 1. Create initial lockfile with exclude-newer via CLI
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock().arg("--exclude-newer").arg("2024-03-25T00:00:00Z"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "###);

    // Verify the lockfile contains exclude-newer = "2024-03-25T00:00:00Z"
    let lock = context.read("uv.lock");
    assert!(lock.contains("exclude-newer = \"2024-03-25T00:00:00Z\""));

    // 2. Add different exclude-newer to pyproject.toml
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        
        [tool.uv]
        exclude-newer = "2024-01-01T00:00:00Z"
        "#,
    )?;

    // 3. Verify lockfile DOES change with uv lock (respecting pyproject.toml)
    // Run uv lock - verify lockfile now has 2024-01-01T00:00:00Z from pyproject.toml
    
    // IMPORTANT: Remove the UV_EXCLUDE_NEWER env var that the test context sets by default
    // so that pyproject.toml value can be respected
    uv_snapshot!(context.filters(), context.lock().env_remove("UV_EXCLUDE_NEWER"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Ignoring existing lockfile due to change in timestamp cutoff: `global: 2024-03-25T00:00:00Z` vs. `global: 2024-01-01T00:00:00Z`
    Resolved 2 packages in [TIME]
    "###);

    let lock = context.read("uv.lock");
    assert!(lock.contains("exclude-newer = \"2024-01-01T00:00:00Z\""));
    assert!(!lock.contains("exclude-newer = \"2024-03-25T00:00:00Z\""));

    // Run uv lock --upgrade - verify lockfile still has 2024-01-01T00:00:00Z from pyproject.toml
    uv_snapshot!(context.filters(), context.lock().arg("--upgrade").env_remove("UV_EXCLUDE_NEWER"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "###);

    let lock = context.read("uv.lock");
    assert!(lock.contains("exclude-newer = \"2024-01-01T00:00:00Z\""));
    assert!(!lock.contains("exclude-newer = \"2024-03-25T00:00:00Z\""));

    // Run uv lock --refresh - verify lockfile still has 2024-01-01T00:00:00Z from pyproject.toml
    uv_snapshot!(context.filters(), context.lock().arg("--refresh").env_remove("UV_EXCLUDE_NEWER"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "###);

    let lock = context.read("uv.lock");
    assert!(lock.contains("exclude-newer = \"2024-01-01T00:00:00Z\""));
    assert!(!lock.contains("exclude-newer = \"2024-03-25T00:00:00Z\""));

    // 4. Verify explicit CLI argument can override pyproject.toml
    uv_snapshot!(context.filters(), context.lock().arg("--exclude-newer").arg("2024-06-01T00:00:00Z"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Ignoring existing lockfile due to change in timestamp cutoff: `global: 2024-01-01T00:00:00Z` vs. `global: 2024-06-01T00:00:00Z`
    Resolved 2 packages in [TIME]
    "###);

    // Now verify lockfile updated to 2024-06-01T00:00:00Z (CLI overrides pyproject.toml)
    let lock = context.read("uv.lock");
    assert!(lock.contains("exclude-newer = \"2024-06-01T00:00:00Z\""));
    assert!(!lock.contains("exclude-newer = \"2024-01-01T00:00:00Z\""));

    Ok(())
}