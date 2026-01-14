use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use insta::assert_snapshot;

use uv_static::EnvVars;

use crate::common::{TestContext, uv_snapshot};

#[test]
fn sync_dev() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [tool.uv]
        dev-dependencies = ["anyio"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--only-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
    Resolved 5 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--no-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
    Resolved 5 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 3 packages in [TIME]
    Installed 1 package in [TIME]
     - anyio==4.3.0
     - idna==3.6
     - sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
    Resolved 5 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Using `--no-default-groups` should remove dev dependencies
    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
    Resolved 5 packages in [TIME]
    Uninstalled 3 packages in [TIME]
     - anyio==4.3.0
     - idna==3.6
     - sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn sync_dev_group() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [tool.uv]
        dev-dependencies = ["anyio"]

        [dependency-groups]
        dev = ["iniconfig"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
    Resolved 6 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    Ok(())
}

#[test]
fn sync_group() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [tool.uv]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + iniconfig==2.0.0
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 4 packages in [TIME]
    Uninstalled 4 packages in [TIME]
    Installed 4 packages in [TIME]
     - anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     - iniconfig==2.0.0
     + requests==2.31.0
     - sniffio==1.3.1
     - typing-extensions==4.10.0
     + urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo").arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + iniconfig==2.0.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Audited 9 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups").arg("--no-group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 4 packages in [TIME]
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - requests==2.31.0
     - urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups").arg("--no-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 4 packages in [TIME]
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     - iniconfig==2.0.0
     + requests==2.31.0
     + urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 7 packages in [TIME]
    Installed 1 package in [TIME]
     - anyio==4.3.0
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     + iniconfig==2.0.0
     - requests==2.31.0
     - sniffio==1.3.1
     - urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--dev").arg("--no-group").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--no-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 8 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + urllib3==2.2.1
    ");

    // Using `--no-default-groups` should exclude all groups
    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 8 packages in [TIME]
     - anyio==4.3.0
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     - iniconfig==2.0.0
     - requests==2.31.0
     - sniffio==1.3.1
     - urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 8 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + urllib3==2.2.1
    ");

    // Using `--no-default-groups` with `--group foo` and `--group bar` should include those groups,
    // excluding the remaining `dev` group.
    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups").arg("--group").arg("foo").arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    Ok(())
}

/// Sync with `--only-group`, where the group includes a workspace member.
#[test]
fn sync_group_member() -> Result<()> {
    let context = TestContext::new("3.12");

    // Create a workspace.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=2"]

        [dependency-groups]
        foo = ["child", "typing-extensions>=4"]

        [tool.uv.workspace]
        members = ["child"]

        [tool.uv.sources]
        child = { workspace = true }
        "#,
    )?;

    // Add a workspace member.
    context
        .temp_dir
        .child("child")
        .child("pyproject.toml")
        .write_str(
            r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
        )?;

    // Generate a lockfile.
    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + iniconfig==2.0.0
     + typing-extensions==4.10.0
    ");

    Ok(())
}

#[test]
fn sync_corner_groups() -> Result<()> {
    // Testing a bunch of random corner cases of flags so their behaviour is tracked.
    // It's fine if we decide we want to support these later!
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["sniffio"]
        bar = ["requests"]
        "#,
    )?;

    context.lock().assert().success();

    // --no-dev and --only-dev should error
    // (This one could be made to work with overloading)
    uv_snapshot!(context.filters(), context.sync()
        .arg("--no-dev")
        .arg("--only-dev"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--no-dev' cannot be used with '--only-dev'

    Usage: uv sync --cache-dir [CACHE_DIR] --no-dev --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --dev and --only-group should error if they don't match
    uv_snapshot!(context.filters(), context.sync()
        .arg("--dev")
        .arg("--only-group").arg("bar"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--dev' cannot be used with '--only-group <ONLY_GROUP>'

    Usage: uv sync --cache-dir [CACHE_DIR] --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --dev and --only-group should error even if it's dev still
    // (This one could be made to work the same as --dev --only-dev)
    uv_snapshot!(context.filters(), context.sync()
        .arg("--dev")
        .arg("--only-group").arg("dev"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--dev' cannot be used with '--only-group <ONLY_GROUP>'

    Usage: uv sync --cache-dir [CACHE_DIR] --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --group and --only-dev should error if they don't match
    // (This one could be made to work the same as --dev --only-dev)
    uv_snapshot!(context.filters(), context.sync()
        .arg("--only-dev")
        .arg("--group").arg("bar"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--only-dev' cannot be used with '--group <GROUP>'

    Usage: uv sync --cache-dir [CACHE_DIR] --only-dev --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --group and --only-dev should error even if it's dev still
    // (This one could be made to work the same as --dev --only-dev)
    uv_snapshot!(context.filters(), context.sync()
        .arg("--only-dev")
        .arg("--group").arg("dev"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--only-dev' cannot be used with '--group <GROUP>'

    Usage: uv sync --cache-dir [CACHE_DIR] --only-dev --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --all-groups and --only-dev should error
    uv_snapshot!(context.filters(), context.sync()
        .arg("--all-groups")
        .arg("--only-dev"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--all-groups' cannot be used with '--only-dev'

    Usage: uv sync --cache-dir [CACHE_DIR] --all-groups --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --all-groups and --only-group should error
    uv_snapshot!(context.filters(), context.sync()
        .arg("--all-groups")
        .arg("--only-group").arg("bar"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--all-groups' cannot be used with '--only-group <ONLY_GROUP>'

    Usage: uv sync --cache-dir [CACHE_DIR] --all-groups --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --group and --only-group should error if they name disjoint things
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("foo")
        .arg("--only-group").arg("bar"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--group <GROUP>' cannot be used with '--only-group <ONLY_GROUP>'

    Usage: uv sync --cache-dir [CACHE_DIR] --group <GROUP> --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --group and --only-group should error if they name same things
    // (This one would be fair to allow, but... is it worth it?)
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("foo")
        .arg("--only-group").arg("foo"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--group <GROUP>' cannot be used with '--only-group <ONLY_GROUP>'

    Usage: uv sync --cache-dir [CACHE_DIR] --group <GROUP> --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    ");

    // --all-groups and --no-default-groups is redundant but should be --all-groups
    uv_snapshot!(context.filters(), context.sync()
        .arg("--all-groups")
        .arg("--no-default-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 9 packages in [TIME]
    Prepared 8 packages in [TIME]
    Installed 8 packages in [TIME]
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
     + urllib3==2.2.1
    ");

    // --dev --only-dev should saturate as --only-dev
    uv_snapshot!(context.filters(), context.sync()
        .arg("--dev")
        .arg("--only-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 9 packages in [TIME]
    Uninstalled 7 packages in [TIME]
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     - requests==2.31.0
     - sniffio==1.3.1
     - typing-extensions==4.10.0
     - urllib3==2.2.1
    ");
    Ok(())
}

#[test]
fn sync_default_groups() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]
        "#,
    )?;

    context.lock().assert().success();

    // The `dev` group should be synced by default.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + iniconfig==2.0.0
     + typing-extensions==4.10.0
    ");

    // If we remove it from the `default-groups` list, it should be removed.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]

        [tool.uv]
        default-groups = []
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    // If we set a different default group, it should be synced instead.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]

        [tool.uv]
        default-groups = ["foo"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // `--no-group` should remove from the defaults.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]

        [tool.uv]
        default-groups = ["foo"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().arg("--no-group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 3 packages in [TIME]
     - anyio==4.3.0
     - idna==3.6
     - sniffio==1.3.1
    ");

    // Using `--group` should include the defaults
    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
    ");

    // Using `--all-groups` should include the defaults
    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + requests==2.31.0
     + urllib3==2.2.1
    ");

    // Using `--only-group` should exclude the defaults
    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 8 packages in [TIME]
     - anyio==4.3.0
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     - requests==2.31.0
     - sniffio==1.3.1
     - typing-extensions==4.10.0
     - urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 8 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + requests==2.31.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
     + urllib3==2.2.1
    ");

    // Using `--no-default-groups` should exclude all groups
    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 8 packages in [TIME]
     - anyio==4.3.0
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     - iniconfig==2.0.0
     - requests==2.31.0
     - sniffio==1.3.1
     - urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 8 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + urllib3==2.2.1
    ");

    // Using `--no-default-groups` with `--group foo` and `--group bar` should include those groups,
    // excluding the remaining `dev` group.
    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups").arg("--group").arg("foo").arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    Ok(())
}

/// default-groups = "all" sugar works
#[test]
fn sync_default_groups_all() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "myproject"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]

        [tool.uv]
        default-groups = "all"
        "#,
    )?;

    context.lock().assert().success();

    // groups = "all" should behave like --all-groups in contexts where defaults exist
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 9 packages in [TIME]
    Installed 9 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
     + urllib3==2.2.1
    ");

    // Using `--no-default-groups` should still work
    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 8 packages in [TIME]
     - anyio==4.3.0
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     - iniconfig==2.0.0
     - requests==2.31.0
     - sniffio==1.3.1
     - urllib3==2.2.1
    ");

    // Using `--all-groups` should be redundant and work fine
    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 8 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + urllib3==2.2.1
    ");

    // Using `--no-dev` should exclude just the dev group
    uv_snapshot!(context.filters(), context.sync().arg("--no-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    // Using `--group` should be redundant and still work fine
    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // Using `--only-group` should still disable defaults
    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 6 packages in [TIME]
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - iniconfig==2.0.0
     - requests==2.31.0
     - typing-extensions==4.10.0
     - urllib3==2.2.1
    ");

    Ok(())
}

/// default-groups = "gibberish" error
#[test]
fn sync_default_groups_gibberish() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]

        [tool.uv]
        default-groups = "gibberish"
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r#"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `pyproject.toml`
      Caused by: TOML parse error at line 14, column 26
       |
    14 |         default-groups = "gibberish"
       |                          ^^^^^^^^^^^
    default-groups must be "all" or a ["list", "of", "groups"]
    "#);

    Ok(())
}

#[test]
fn sync_non_existent_default_group() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        foo = []

        [tool.uv]
        default-groups = ["bar"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync(), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Default group `bar` (from `tool.uv.default-groups`) is not defined in the project's `dependency-groups` table
    "###);

    Ok(())
}

#[test]
fn sync_non_existent_group() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        foo = []
        bar = ["requests"]
        "#,
    )?;

    context.lock().assert().success();

    // Requesting a non-existent group should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("baz"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    error: Group `baz` is not defined in the project's `dependency-groups` table
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--no-group").arg("baz"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    error: Group `baz` is not defined in the project's `dependency-groups` table
    ");

    // Requesting an empty group should succeed.
    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + typing-extensions==4.10.0
    ");

    // Requesting with `--frozen` should respect the groups in the lockfile, rather than the
    // `pyproject.toml`.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + requests==2.31.0
     + urllib3==2.2.1
    ");

    // Replace `bar` with `baz`.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        baz = ["iniconfig"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Audited 6 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--group").arg("baz"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Group `baz` is not defined in the project's `dependency-groups` table
    ");

    Ok(())
}

#[test]
fn sync_exclude_group() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        foo = ["anyio", {include-group = "bar"}]
        bar = ["iniconfig"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo").arg("--no-group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Uninstalled 4 packages in [TIME]
     - anyio==4.3.0
     - idna==3.6
     - iniconfig==2.0.0
     - sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
     - typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("bar").arg("--no-group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    Ok(())
}

#[test]
fn sync_exclude_group_with_environment_variable() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        foo = ["anyio"]
        bar = ["iniconfig"]
        baz = ["certifi"]
        "#,
    )?;

    context.lock().assert().success();

    // Test single group exclusion via environment variable
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("foo")
        .arg("--group").arg("bar")
        .env(EnvVars::UV_NO_GROUP, "bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // Test multiple group exclusion via environment variable (space-separated)
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("foo")
        .arg("--group").arg("bar")
        .arg("--group").arg("baz")
        .env(EnvVars::UV_NO_GROUP, "bar baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Audited 4 packages in [TIME]
    ");

    // Test that CLI flag takes precedence over environment variable
    // When --no-group is used on CLI, it overrides UV_NO_GROUP env var
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("foo")
        .arg("--group").arg("bar")
        .arg("--group").arg("baz")
        .arg("--no-group").arg("bar")
        .env(EnvVars::UV_NO_GROUP, "baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + certifi==2024.2.2
    ");

    Ok(())
}

#[test]
fn sync_include_group() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        foo = ["anyio", {include-group = "bar"}]
        bar = ["iniconfig"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Uninstalled 4 packages in [TIME]
     - anyio==4.3.0
     - idna==3.6
     - sniffio==1.3.1
     - typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo").arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Installed 1 package in [TIME]
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Uninstalled 4 packages in [TIME]
     - anyio==4.3.0
     - idna==3.6
     - iniconfig==2.0.0
     - sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--no-default-groups").arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Audited 5 packages in [TIME]
    ");

    Ok(())
}

#[test]
fn sync_disable_default_groups_with_environment_variable() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]

        [tool.uv]
        default-groups = ["foo"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 9 packages in [TIME]
    Installed 9 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
     + urllib3==2.2.1
    ");

    // Using `UV_NO_DEFAULT_GROUPS` should exclude all groups.
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_NO_DEFAULT_GROUPS, "true"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 8 packages in [TIME]
     - anyio==4.3.0
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     - iniconfig==2.0.0
     - requests==2.31.0
     - sniffio==1.3.1
     - urllib3==2.2.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-groups"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Installed 8 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + urllib3==2.2.1
    ");

    // Using `UV_NO_DEFAULT_GROUPS` with `--group foo` and `--group bar` should include those groups,
    // excluding the remaining `dev` group.
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("foo")
        .arg("--group").arg("bar")
        .env(EnvVars::UV_NO_DEFAULT_GROUPS, "true"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    Ok(())
}

#[test]
fn sync_disable_default_groups_all_with_environment_variable() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "myproject"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        dev = ["iniconfig"]
        foo = ["anyio"]
        bar = ["requests"]

        [tool.uv]
        default-groups = "all"
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Prepared 9 packages in [TIME]
    Installed 9 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + iniconfig==2.0.0
     + requests==2.31.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
     + urllib3==2.2.1
    ");

    // Using `UV_NO_DEFAULT_GROUPS` should exclude all groups.
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_NO_DEFAULT_GROUPS, "true"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 10 packages in [TIME]
    Uninstalled 8 packages in [TIME]
     - anyio==4.3.0
     - certifi==2024.2.2
     - charset-normalizer==3.3.2
     - idna==3.6
     - iniconfig==2.0.0
     - requests==2.31.0
     - sniffio==1.3.1
     - urllib3==2.2.1
    ");

    Ok(())
}

/// Sync with `--only-group`, where the group includes the project itself.
#[test]
fn sync_group_self() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=2"]

        [project.optional-dependencies]
        test = ["idna>=3"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"

        [dependency-groups]
        foo = ["project", "typing-extensions>=4"]
        bar = ["project[test]"]
        "#,
    )?;

    // Generate a lockfile.
    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "###);

    let lock = context.read("uv.lock");

    insta::with_settings!({
        filters => context.filters(),
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "idna"
        version = "3.6"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/bf/3f/ea4b9117521a1e9c50344b909be7886dd00a519552724809bb1f486986c2/idna-3.6.tar.gz", hash = "sha256:9ecdbbd083b06798ae1e86adcbfe8ab1479cf864e4ee30fe4e46a003d12491ca", size = 175426, upload-time = "2023-11-25T15:40:54.902Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl", hash = "sha256:c05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f", size = 61567, upload-time = "2023-11-25T15:40:52.604Z" },
        ]

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3", size = 4646, upload-time = "2023-01-07T11:08:11.254Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl", hash = "sha256:b6a85871a79d2e3b22d2d1b94ac2824226a63c6b741c88f7ae975f18b6778374", size = 5892, upload-time = "2023-01-07T11:08:09.864Z" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { editable = "." }
        dependencies = [
            { name = "iniconfig" },
        ]

        [package.optional-dependencies]
        test = [
            { name = "idna" },
        ]

        [package.dev-dependencies]
        bar = [
            { name = "project", extra = ["test"] },
        ]
        foo = [
            { name = "project" },
            { name = "typing-extensions" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "idna", marker = "extra == 'test'", specifier = ">=3" },
            { name = "iniconfig", specifier = ">=2" },
        ]
        provides-extras = ["test"]

        [package.metadata.requires-dev]
        bar = [{ name = "project", extras = ["test"] }]
        foo = [
            { name = "project" },
            { name = "typing-extensions", specifier = ">=4" },
        ]

        [[package]]
        name = "typing-extensions"
        version = "4.10.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/16/3a/0d26ce356c7465a19c9ea8814b960f8a36c3b0d07c323176620b7b483e44/typing_extensions-4.10.0.tar.gz", hash = "sha256:b0abd7c89e8fb96f98db18d86106ff1d90ab692004eb746cf6eda2682f91b3cb", size = 77558, upload-time = "2024-02-25T22:12:49.693Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/f9/de/dc04a3ea60b22624b51c703a84bbe0184abcd1d0b9bc8074b5d6b7ab90bb/typing_extensions-4.10.0-py3-none-any.whl", hash = "sha256:69b1a937c3a517342112fb4c6df7e72fc39a38e7891a5730ed4985b5214b5475", size = 33926, upload-time = "2024-02-25T22:12:47.72Z" },
        ]
        "#
        );
    });

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + iniconfig==2.0.0
     + project==0.1.0 (from file://[TEMP_DIR]/)
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     + idna==3.6
     - typing-extensions==4.10.0
    ");

    Ok(())
}

/// Sync all members in a workspace with dependency groups attached.
#[test]
fn sync_all_groups() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [dependency-groups]
        types = ["sniffio>=1"]
        async = ["anyio>=3"]
        empty = []

        [tool.uv.workspace]
        members = ["child"]

        [tool.uv.sources]
        child = { workspace = true }
        "#,
    )?;
    context
        .temp_dir
        .child("src")
        .child("project")
        .child("__init__.py")
        .touch()?;

    // Add a workspace member.
    let child = context.temp_dir.child("child");
    child.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]

        [dependency-groups]
        types = ["typing-extensions>=4"]
        testing = ["packaging>=24"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;
    child
        .child("src")
        .child("child")
        .child("__init__.py")
        .touch()?;

    // Generate a lockfile.
    context.lock().assert().success();

    // Sync a group that exists in both the parent and child.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--group").arg("types"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + iniconfig==2.0.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // Sync a group that only exists in the child.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--group").arg("testing"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 2 packages in [TIME]
    Installed 1 package in [TIME]
     + packaging==24.0
     - sniffio==1.3.1
     - typing-extensions==4.10.0
    ");

    // Sync a group that doesn't exist.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--group").arg("foo"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    error: Group `foo` is not defined in any project's `dependency-groups` table
    ");

    // Sync an empty group.
    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("empty"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - packaging==24.0
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11648>
#[test]
fn multiple_group_conflicts() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [dependency-groups]
        foo = [
            "iniconfig>=2",
        ]
        bar = [
            "iniconfig<2",
        ]
        baz = [
            "iniconfig",
        ]

        [tool.uv]
        conflicts = [
          [
            { group = "foo" },
            { group = "bar" },
          ],
        ]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Audited in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo").arg("--group").arg("baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("bar").arg("--group").arg("baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - iniconfig==2.0.0
     + iniconfig==1.1.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo").arg("--group").arg("bar"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    error: Groups `bar` and `foo` are incompatible with the conflicts: {`project:bar`, `project:foo`}
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11232>
#[test]
fn transitive_group_conflicts_shallow() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "example"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [dependency-groups]
        dev = [
            { include-group = "test" },
        ]
        test = ["anyio>4"]
        magic = ["anyio<4"]

        [tool.uv]
        conflicts = [
            [
                { group = "test" },
                { group = "magic" },
            ],
        ]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.lock().arg("--check"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Audited 3 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--group").arg("test"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Audited 3 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("test").arg("--group").arg("magic"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    error: Groups `magic` and `test` are incompatible with the conflicts: {`example:magic`, `example:test`}
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--group").arg("magic"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    error: Groups `dev` and `magic` are incompatible with the conflicts: {`example:dev`, `example:magic`}
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11232>
#[test]
fn transitive_group_conflicts_deep() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "example"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [dependency-groups]
        dev = [
            { include-group = "intermediate" },
        ]
        intermediate = [
            { include-group = "test" },
            { include-group = "other" },
        ]
        test = ["iniconfig>=2"]
        magic = ["iniconfig<2", "anyio<4"]
        other = ["anyio>4"]

        [tool.uv]
        conflicts = [
            [
                { group = "test" },
                { group = "magic" },
            ],
            [
                { group = "other" },
                { group = "magic" },
            ],
        ]"#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Audited 4 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--group").arg("test"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Audited 4 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--group").arg("magic"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    error: Groups `dev` and `magic` are incompatible with the conflicts: {`example:dev`, `example:magic`}
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--no-dev").arg("--group").arg("intermediate").arg("--group").arg("magic"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    error: Groups `intermediate` and `magic` are incompatible with the conflicts: {`example:intermediate`, `example:magic`}
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11232>
#[test]
fn transitive_group_conflicts_siblings() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "example"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [dependency-groups]
        dev = [
            { include-group = "test" },
        ]
        dev2 = [
            { include-group = "magic" },
        ]
        test = ["anyio>4"]
        magic = ["anyio<4"]

        [tool.uv]
        conflicts = [
            [
                { group = "test" },
                { group = "magic" },
            ],
        ]"#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Audited 3 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--no-dev").arg("--group").arg("dev2"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - anyio==4.3.0
     + anyio==3.7.1
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev2"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    error: Groups `dev` (enabled by default) and `dev2` are incompatible with the conflicts: {`example:dev`, `example:dev2`}
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--group").arg("dev2"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    error: Groups `dev` and `dev2` are incompatible with the conflicts: {`example:dev`, `example:dev2`}
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11232>
#[test]
fn transitive_group_conflicts_cycle() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "example"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [dependency-groups]
        dev = [
            { include-group = "test" },
        ]
        test = [
            "anyio>4",
            { include-group = "dev" },
        ]
        magic = ["anyio<4"]

        [tool.uv]
        conflicts = [
            [
                { group = "test" },
                { group = "magic" },
            ],
        ]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project `example` has malformed dependency groups
      Caused by: Detected a cycle in `dependency-groups`: `dev` -> `test` -> `dev`
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project `example` has malformed dependency groups
      Caused by: Detected a cycle in `dependency-groups`: `dev` -> `test` -> `dev`
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--group").arg("test"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project `example` has malformed dependency groups
      Caused by: Detected a cycle in `dependency-groups`: `dev` -> `test` -> `dev`
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("test").arg("--group").arg("magic"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project `example` has malformed dependency groups
      Caused by: Detected a cycle in `dependency-groups`: `dev` -> `test` -> `dev`
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("dev").arg("--group").arg("magic"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project `example` has malformed dependency groups
      Caused by: Detected a cycle in `dependency-groups`: `dev` -> `test` -> `dev`
    ");

    Ok(())
}

#[test]
fn only_group_and_extra_conflict() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [project.optional-dependencies]
        test = ["pytest"]

        [dependency-groups]
        dev = ["ruff"]
        "#,
    )?;

    // Using --only-group and --extra together should error.
    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("dev").arg("--extra").arg("test"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--only-group <ONLY_GROUP>' cannot be used with '--extra <EXTRA>'

    Usage: uv sync --cache-dir [CACHE_DIR] --only-group <ONLY_GROUP> --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    "###);

    // Using --only-group and --all-extras together should also error.
    uv_snapshot!(context.filters(), context.sync().arg("--only-group").arg("dev").arg("--all-extras"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--only-group <ONLY_GROUP>' cannot be used with '--all-extras'

    Usage: uv sync --cache-dir [CACHE_DIR] --only-group <ONLY_GROUP> --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    "###);

    Ok(())
}
