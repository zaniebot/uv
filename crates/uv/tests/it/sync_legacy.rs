use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use insta::assert_snapshot;

use uv_static::EnvVars;

use crate::common::{TestContext, uv_snapshot};

/// Sync development dependencies in a (legacy) non-project workspace root.
#[test]
fn sync_legacy_non_project_dev_dependencies() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [tool.uv]
        dev-dependencies = ["anyio>3", "requests[socks]", "typing-extensions ; sys_platform == ''"]

        [tool.uv.workspace]
        members = ["child"]
        "#,
    )?;
    context
        .temp_dir
        .child("src")
        .child("albatross")
        .child("__init__.py")
        .touch()?;

    let child = context.temp_dir.child("child");
    let pyproject_toml = child.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]

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

    // Syncing with `--no-dev` should omit all dependencies except `iniconfig`.
    uv_snapshot!(context.filters(), context.sync().arg("--no-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 11 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + iniconfig==2.0.0
    ");

    // Syncing without `--no-dev` should include `anyio`, `requests`, `pysocks`, and their
    // dependencies, but not `typing-extensions`.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 11 packages in [TIME]
    Prepared 8 packages in [TIME]
    Installed 8 packages in [TIME]
     + anyio==4.3.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + idna==3.6
     + pysocks==1.7.1
     + requests==2.31.0
     + sniffio==1.3.1
     + urllib3==2.2.1
    ");

    Ok(())
}

/// Sync development dependencies in a (legacy) non-project workspace root with `--frozen`.
#[test]
fn sync_legacy_non_project_frozen() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [tool.uv.workspace]
        members = ["foo", "bar"]
        "#,
    )?;

    context
        .temp_dir
        .child("foo")
        .child("pyproject.toml")
        .write_str(
            r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]
        "#,
        )?;

    context
        .temp_dir
        .child("bar")
        .child("pyproject.toml")
        .write_str(
            r#"
        [project]
        name = "bar"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions>=4"]
        "#,
        )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--package").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + typing-extensions==4.10.0
    ");

    Ok(())
}

/// Sync development dependencies in a (legacy) non-project workspace root.
#[test]
fn sync_legacy_non_project_group() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [dependency-groups]
        foo = ["anyio"]
        bar = ["typing-extensions"]

        [tool.uv.workspace]
        members = ["child"]
        "#,
    )?;

    context
        .temp_dir
        .child("src")
        .child("albatross")
        .child("__init__.py")
        .touch()?;

    let child = context.temp_dir.child("child");
    let pyproject_toml = child.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]

        [dependency-groups]
        baz = ["typing-extensions"]

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

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
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
    Resolved 6 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 5 packages in [TIME]
    Installed 1 package in [TIME]
     - anyio==4.3.0
     - child==0.1.0 (from file://[TEMP_DIR]/child)
     - idna==3.6
     - iniconfig==2.0.0
     - sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Installed 2 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("bop"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    error: Group `bop` is not defined in any project's `dependency-groups` table
    ");

    Ok(())
}

/// Sync development dependencies in a (legacy) non-project workspace root with `--frozen`.
///
/// Modify the `pyproject.toml` after locking.
#[test]
fn sync_legacy_non_project_frozen_modification() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [tool.uv.workspace]
        members = []

        [dependency-groups]
        async = ["anyio"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--group").arg("async"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Modify the "live" dependency groups.
    pyproject_toml.write_str(
        r#"
        [tool.uv.workspace]
        members = []

        [dependency-groups]
        async = ["iniconfig"]
        "#,
    )?;

    // This should succeed.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--group").arg("async"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Audited 3 packages in [TIME]
    ");

    Ok(())
}

/// Sync with `--only-group`, where the group includes a legacy non-`[project]` workspace member.
#[test]
fn sync_group_legacy_non_project_member() -> Result<()> {
    let context = TestContext::new("3.12");

    // Create a workspace.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
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
    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
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

        [manifest]
        members = [
            "child",
        ]

        [manifest.dependency-groups]
        foo = [
            { name = "child", editable = "child" },
            { name = "typing-extensions", specifier = ">=4" },
        ]

        [[package]]
        name = "child"
        version = "0.1.0"
        source = { editable = "child" }
        dependencies = [
            { name = "iniconfig" },
        ]

        [package.metadata]
        requires-dist = [{ name = "iniconfig", specifier = ">=1" }]

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3", size = 4646, upload-time = "2023-01-07T11:08:11.254Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl", hash = "sha256:b6a85871a79d2e3b22d2d1b94ac2824226a63c6b741c88f7ae975f18b6778374", size = 5892, upload-time = "2023-01-07T11:08:09.864Z" },
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
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + iniconfig==2.0.0
     + typing-extensions==4.10.0
    ");

    Ok(())
}

/// Test for warnings when `VIRTUAL_ENV` is set but will not be respected.
#[test]
fn sync_legacy_non_project_warning() -> Result<()> {
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

    // We should not warn if it matches the project environment
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, context.temp_dir.join(".venv")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // Including if it's a relative path that matches
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, ".venv"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // Or, if it's a link that resolves to the same path
    #[cfg(unix)]
    {
        use fs_err::os::unix::fs::symlink;

        let link = context.temp_dir.join("link");
        symlink(context.temp_dir.join(".venv"), &link)?;

        uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, link), @r"
        success: true
        exit_code: 0
        ----- stdout -----

        ----- stderr -----
        Resolved 2 packages in [TIME]
        Audited 1 package in [TIME]
        ");
    }

    // But we should warn if it's a different path
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, "foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: `VIRTUAL_ENV=foo` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // Including absolute paths
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, context.temp_dir.join("foo")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: `VIRTUAL_ENV=foo` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // We should not warn if the project environment has been customized and matches
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, "foo").env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: foo
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // But we should warn if they don't match still
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, "foo").env(EnvVars::UV_PROJECT_ENVIRONMENT, "bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: `VIRTUAL_ENV=foo` does not match the project environment path `bar` and will be ignored; use `--active` to target the active environment instead
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: bar
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    let child = context.temp_dir.child("child");
    child.create_dir_all()?;

    // And `VIRTUAL_ENV` is resolved relative to the project root so with relative paths we should
    // warn from a child too
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, "foo").env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo").current_dir(&child), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: `VIRTUAL_ENV=foo` does not match the project environment path `[TEMP_DIR]/foo` and will be ignored; use `--active` to target the active environment instead
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // But, a matching absolute path shouldn't warn
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, context.temp_dir.join("foo")).env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo").current_dir(&child), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    Ok(())
}
