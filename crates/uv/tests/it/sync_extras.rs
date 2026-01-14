use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use indoc::indoc;
use insta::assert_snapshot;

use crate::common::{TestContext, uv_snapshot};

#[test]
fn sync_non_existent_extra() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        [project.optional-dependencies]
        types = ["sniffio>1"]
        async = ["anyio>3"]
        "#,
    )?;

    context.lock().assert().success();

    // Requesting a non-existent extra should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("baz"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    error: Extra `baz` is not defined in the project's `optional-dependencies` table
    ");

    // Excluding a non-existing extra when requesting all extras should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--all-extras").arg("--no-extra").arg("baz"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    error: Extra `baz` is not defined in the project's `optional-dependencies` table
    ");

    Ok(())
}

#[test]
fn sync_non_existent_extra_no_optional_dependencies() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        "#,
    )?;

    context.lock().assert().success();

    // Requesting a non-existent extra should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("baz"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    error: Extra `baz` is not defined in the project's `optional-dependencies` table
    ");

    // Excluding a non-existing extra when requesting all extras should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--all-extras").arg("--no-extra").arg("baz"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    error: Extra `baz` is not defined in the project's `optional-dependencies` table
    ");

    Ok(())
}

/// Ensures that we do not perform validation of extras against a lock file that was generated on a
/// version of uv that predates when `provides-extras` feature was added.
#[test]
fn sync_ignore_extras_check_when_no_provides_extras() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        [project.optional-dependencies]
        types = ["sniffio>1"]
        "#,
    )?;

    // Write a lockfile that does not have `provides-extra`, simulating a version that predates when
    // the feature was added.
    context.temp_dir.child("uv.lock").write_str(indoc! {r#"
        version = 1
        requires-python = ">=3.12"

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }

        [package.optional-dependencies]
        types = [
            { name = "sniffio" },
        ]

        [package.metadata]
        requires-dist = [{ name = "sniffio", marker = "extra == 'types'", specifier = ">1" }]

        [[package]]
        name = "sniffio"
        version = "1.3.1"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/a2/87/a6771e1546d97e7e041b6ae58d80074f81b7d5121207425c964ddf5cfdbd/sniffio-1.3.1.tar.gz", hash = "sha256:f4324edc670a0f49750a81b895f35c3adb843cca46f0530f79fc1babb23789dc", size = 20372 }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/e9/44/75a9c9421471a6c4805dbf2356f7c181a29c1879239abab1ea2cc8f38b40/sniffio-1.3.1-py3-none-any.whl", hash = "sha256:2f6da418d1f1e0fddd844478f41680e794e6051915791a034ff65e5f100525a2", size = 10235 },
        ]
    "#})?;

    // Requesting a non-existent extra should not fail, as no validation should be performed.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--extra").arg("baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Audited in [TIME]
    ");

    Ok(())
}

#[test]
fn sync_non_existent_extra_workspace_member() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [project.optional-dependencies]
        types = ["sniffio>1"]

        [tool.uv.workspace]
        members = ["child"]

        [tool.uv.sources]
        child = { workspace = true }
        "#,
    )?;

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

        [project.optional-dependencies]
        async = ["anyio>3"]
        "#,
        )?;

    context.lock().assert().success();

    // Requesting an extra that only exists in the child should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("async"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    error: Extra `async` is not defined in the project's `optional-dependencies` table
    ");

    // Unless we sync from the child directory.
    uv_snapshot!(context.filters(), context.sync().arg("--package").arg("child").arg("--extra").arg("async"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn sync_non_existent_extra_non_project_workspace() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [tool.uv.workspace]
        members = ["child", "other"]
        "#,
    )?;

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

        [project.optional-dependencies]
        async = ["anyio>3"]
        "#,
        )?;

    context
        .temp_dir
        .child("other")
        .child("pyproject.toml")
        .write_str(
            r#"
        [project]
        name = "other"
        version = "0.1.0"
        requires-python = ">=3.12"
        "#,
        )?;

    context.lock().assert().success();

    // Requesting an extra that only exists in the child should succeed, since we sync all members
    // by default.
    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("async"), @r"
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

    // Syncing from the child should also succeed.
    uv_snapshot!(context.filters(), context.sync().arg("--package").arg("child").arg("--extra").arg("async"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Audited 3 packages in [TIME]
    ");

    // Syncing from an unrelated child should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--package").arg("other").arg("--extra").arg("async"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    error: Extra `async` is not defined in the project's `optional-dependencies` table
    ");

    Ok(())
}

#[test]
fn sync_dynamic_extra() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        dynamic = ["optional-dependencies"]

        [tool.setuptools.dynamic.optional-dependencies]
        dev = { file = "requirements-dev.txt" }

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    context
        .temp_dir
        .child("requirements-dev.txt")
        .write_str("typing-extensions")?;

    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + iniconfig==2.0.0
     + project==0.1.0 (from file://[TEMP_DIR]/)
     + typing-extensions==4.10.0
    ");

    let lock = context.read("uv.lock");

    insta::with_settings!(
        {
            filters => context.filters(),
        },
        {
            assert_snapshot!(
                lock, @r#"
            version = 1
            revision = 3
            requires-python = ">=3.12"

            [options]
            exclude-newer = "2024-03-25T00:00:00Z"

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
            dev = [
                { name = "typing-extensions" },
            ]

            [package.metadata]
            requires-dist = [
                { name = "iniconfig" },
                { name = "typing-extensions", marker = "extra == 'dev'" },
            ]
            provides-extras = ["dev"]

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
        }
    );

    // Check that we can re-read the lockfile.
    uv_snapshot!(context.filters(), context.sync().arg("--locked"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - typing-extensions==4.10.0
    ");

    Ok(())
}

/// Sync all members in a workspace with extras attached.
#[test]
fn sync_all_extras() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [project.optional-dependencies]
        types = ["sniffio>1"]
        async = ["anyio>3"]

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

        [project.optional-dependencies]
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

    // Sync an extra that exists in both the parent and child.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--extra").arg("types"), @r"
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

    // Sync an extra that only exists in the child.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--extra").arg("testing"), @r"
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

    // Sync all extras.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--all-extras"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // Sync all extras excluding an extra that exists in both the parent and child.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--all-extras").arg("--no-extra").arg("types"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - typing-extensions==4.10.0
    ");

    // Sync an extra that doesn't exist.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--extra").arg("foo"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    error: Extra `foo` is not defined in any project's `optional-dependencies` table
    ");

    // Sync all extras excluding an extra that doesn't exist.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--all-extras").arg("--no-extra").arg("foo"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    error: Extra `foo` is not defined in any project's `optional-dependencies` table
    ");

    Ok(())
}

/// Sync all members in a workspace with dynamic extras.
#[test]
fn sync_all_extras_dynamic() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [project.optional-dependencies]
        types = ["sniffio>1"]
        async = ["anyio>3"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

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
        dynamic = ["optional-dependencies"]

        [tool.setuptools.dynamic.optional-dependencies]
        dev = { file = "requirements-dev.txt" }

        [tool.uv]
        cache-keys = ["pyproject.toml"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;
    child
        .child("src")
        .child("child")
        .child("__init__.py")
        .touch()?;

    child
        .child("requirements-dev.txt")
        .write_str("typing-extensions==4.10.0")?;

    // Generate a lockfile.
    context.lock().assert().success();

    // Sync an extra that exists in the parent.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--extra").arg("types"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + project==0.1.0 (from file://[TEMP_DIR]/)
     + sniffio==1.3.1
    ");

    // Sync a dynamic extra that exists in the child.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--extra").arg("dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // Sync a dynamic extra that doesn't exist in the child.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--extra").arg("foo"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    error: Extra `foo` is not defined in any project's `optional-dependencies` table
    ");

    Ok(())
}
