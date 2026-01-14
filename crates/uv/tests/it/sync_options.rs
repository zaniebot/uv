use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;

use uv_static::EnvVars;

use crate::common::{TestContext, download_to_disk, uv_snapshot};

/// Avoid syncing the project package when `--no-install-project` is provided.
#[test]
fn no_install_project() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    // Generate a lockfile.
    context.lock().assert().success();

    // Running with `--no-install-project` should install `anyio`, but not `project`.
    uv_snapshot!(context.filters(), context.sync().arg("--no-install-project"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // However, we do require the `pyproject.toml`.
    fs_err::remove_file(pyproject_toml)?;

    uv_snapshot!(context.filters(), context.sync().arg("--no-install-project"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No `pyproject.toml` found in current directory or any parent directory
    "###);

    Ok(())
}

/// Avoid syncing workspace members and the project when `--no-install-workspace` is provided, but
/// include all dependencies.
#[test]
fn no_install_workspace() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0", "child"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"

        [tool.uv.workspace]
        members = ["child"]

        [tool.uv.sources]
        child = { workspace = true }
        "#,
    )?;

    // Add a workspace member.
    let child = context.temp_dir.child("child");
    child.child("pyproject.toml").write_str(
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
    child
        .child("src")
        .child("child")
        .child("__init__.py")
        .touch()?;

    // Generate a lockfile.
    context.lock().assert().success();

    // Running with `--no-install-workspace` should install `anyio` and `iniconfig`, but not
    // `project` or `child`.
    uv_snapshot!(context.filters(), context.sync().arg("--no-install-workspace"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
    ");

    // Remove the virtual environment.
    fs_err::remove_dir_all(&context.venv)?;

    // We don't require the `pyproject.toml` for non-root members, if `--frozen` is provided.
    fs_err::remove_file(child.join("pyproject.toml"))?;

    uv_snapshot!(context.filters(), context.sync().arg("--no-install-workspace").arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Installed 4 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
    ");

    // Even if `--package` is used.
    uv_snapshot!(context.filters(), context.sync().arg("--package").arg("child").arg("--no-install-workspace").arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Uninstalled 3 packages in [TIME]
     - anyio==3.7.0
     - idna==3.6
     - sniffio==1.3.1
    ");

    // Unless the package doesn't exist.
    uv_snapshot!(context.filters(), context.sync().arg("--package").arg("fake").arg("--no-install-workspace").arg("--frozen"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Could not find root package `fake`
    ");

    // Even if `--all-packages` is used.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages").arg("--no-install-workspace").arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // But we do require the root `pyproject.toml`.
    fs_err::remove_file(context.temp_dir.join("pyproject.toml"))?;

    uv_snapshot!(context.filters(), context.sync().arg("--no-install-workspace").arg("--frozen"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No `pyproject.toml` found in current directory or any parent directory
    "###);

    Ok(())
}

/// Avoid syncing local packages when `--no-install-local` is provided.
#[test]
fn no_install_local() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0", "local", "local-editable", "workspace-member"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"

        [tool.uv.sources]
        local = { path = "./local" }
        local-editable = { path = "./local-editable", editable = true }
        workspace-member = { workspace = true }

        [tool.uv.workspace]
        members = ["workspace-member"]
        "#,
    )?;

    // Add a local package, local editable package, and then a workspace member
    // as a dependency.
    let local = context.temp_dir.child("local");
    local.create_dir_all()?;
    let local_pyproject = local.child("pyproject.toml");
    local_pyproject.write_str(
        r#"
        [project]
        name = "local"
        version = "0.1.0"
        requires-python = ">=3.12"
        "#,
    )?;

    let local_editable = context.temp_dir.child("local-editable");
    local_editable.create_dir_all()?;
    let local_editable_pyproject = local_editable.child("pyproject.toml");
    local_editable_pyproject.write_str(
        r#"
        [project]
        name = "local-editable"
        version = "0.1.0"
        requires-python = ">=3.12"
        "#,
    )?;

    let workspace_member = context.temp_dir.child("workspace-member");
    workspace_member.create_dir_all()?;
    let member_pyproject = workspace_member.child("pyproject.toml");
    member_pyproject.write_str(
        r#"
        [project]
        name = "workspace-member"
        version = "0.1.0"
        requires-python = ">=3.12"
        "#,
    )?;

    context.lock().assert().success();
    uv_snapshot!(context.filters(), context.sync().arg("--no-install-local"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

/// Avoid syncing the target package when `--no-install-package` is provided.
#[test]
fn no_install_package() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    // Generate a lockfile.
    context.lock().assert().success();

    // Running with `--no-install-package anyio` should skip anyio but include everything else
    uv_snapshot!(context.filters(), context.sync().arg("--no-install-package").arg("anyio"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + idna==3.6
     + project==0.1.0 (from file://[TEMP_DIR]/)
     + sniffio==1.3.1
    ");

    // Running with `--no-install-package project` should skip the project itself (not as a special
    // case, that's just the name of the project)
    uv_snapshot!(context.filters(), context.sync().arg("--no-install-package").arg("project"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     + anyio==3.7.0
     - project==0.1.0 (from file://[TEMP_DIR]/)
    ");

    Ok(())
}

/// Ensure that `--no-build` isn't enforced for projects that aren't installed in the first place.
#[test]
fn no_install_project_no_build() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    // Generate a lockfile.
    context.lock().assert().success();

    // `--no-build` should raise an error, since we try to install the project.
    uv_snapshot!(context.filters(), context.sync().arg("--no-build"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    error: Distribution `project==0.1.0 @ editable+.` can't be installed because it is marked as `--no-build` but has no binary distribution
    ");

    // But it's fine to combine `--no-install-project` with `--no-build`. We shouldn't error, since
    // we aren't building the project.
    uv_snapshot!(context.filters(), context.sync().arg("--no-install-project").arg("--no-build").arg("--locked"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn no_binary() -> Result<()> {
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

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--no-binary-package").arg("iniconfig"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").arg("--no-binary"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").env(EnvVars::UV_NO_BINARY_PACKAGE, "iniconfig"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").env(EnvVars::UV_NO_BINARY, "1"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ iniconfig==2.0.0
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").env(EnvVars::UV_NO_BINARY, "iniconfig"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'iniconfig' for '--no-binary': value was not a boolean

    For more information, try '--help'.
    "###);

    Ok(())
}

#[test]
fn no_binary_error() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["odrive"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--no-binary-package").arg("odrive"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 31 packages in [TIME]
    error: Distribution `odrive==0.6.8 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-binary` but has no source distribution
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

#[test]
fn no_build() -> Result<()> {
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

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--no-build-package").arg("iniconfig"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").env(EnvVars::UV_NO_BUILD_PACKAGE, "iniconfig"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ iniconfig==2.0.0
    ");

    Ok(())
}

#[test]
fn no_build_error() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["django_allauth==0.51.0"]
        "#,
    )?;

    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--no-build-package").arg("django-allauth"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 19 packages in [TIME]
    error: Distribution `django-allauth==0.51.0 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-build` but has no binary distribution
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--no-build"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 19 packages in [TIME]
    error: Distribution `django-allauth==0.51.0 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-build` but has no binary distribution
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").env(EnvVars::UV_NO_BUILD, "1"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 19 packages in [TIME]
    error: Distribution `django-allauth==0.51.0 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-build` but has no binary distribution
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").env(EnvVars::UV_NO_BUILD_PACKAGE, "django-allauth"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 19 packages in [TIME]
    error: Distribution `django-allauth==0.51.0 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-build` but has no binary distribution
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").env(EnvVars::UV_NO_BUILD, "django-allauth"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'django-allauth' for '--no-build': value was not a boolean

    For more information, try '--help'.
    "###);

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

#[test]
fn sync_wheel_url_source_error() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "uv-test"
        version = "0.0.0"
        requires-python = ">=3.10"
        dependencies = [
            "cffi @ https://files.pythonhosted.org/packages/08/fd/cc2fedbd887223f9f5d170c96e57cbf655df9831a6546c1727ae13fa977a/cffi-1.17.1-cp310-cp310-macosx_11_0_arm64.whl",
        ]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    error: Distribution `cffi==1.17.1 @ direct+https://files.pythonhosted.org/packages/08/fd/cc2fedbd887223f9f5d170c96e57cbf655df9831a6546c1727ae13fa977a/cffi-1.17.1-cp310-cp310-macosx_11_0_arm64.whl` can't be installed because the binary distribution is incompatible with the current platform

    hint: You're using CPython 3.12 (`cp312`), but `cffi` (v1.17.1) only has wheels with the following Python ABI tag: `cp310`
    ");

    Ok(())
}

#[test]
fn sync_wheel_path_source_error() -> Result<()> {
    let context = TestContext::new("3.12");

    // Download a wheel.
    let archive = context
        .temp_dir
        .child("cffi-1.17.1-cp310-cp310-macosx_11_0_arm64.whl");
    download_to_disk(
        "https://files.pythonhosted.org/packages/08/fd/cc2fedbd887223f9f5d170c96e57cbf655df9831a6546c1727ae13fa977a/cffi-1.17.1-cp310-cp310-macosx_11_0_arm64.whl",
        &archive,
    );

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "uv-test"
        version = "0.0.0"
        requires-python = ">=3.10"
        dependencies = ["cffi"]

        [tool.uv.sources]
        cffi = { path = "cffi-1.17.1-cp310-cp310-macosx_11_0_arm64.whl" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "###);

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    error: Distribution `cffi==1.17.1 @ path+cffi-1.17.1-cp310-cp310-macosx_11_0_arm64.whl` can't be installed because the binary distribution is incompatible with the current platform

    hint: You're using CPython 3.12 (`cp312`), but `cffi` (v1.17.1) only has wheels with the following Python ABI tag: `cp310`
    ");

    Ok(())
}

#[test]
fn sync_override_package() -> Result<()> {
    let context = TestContext::new("3.12");

    // Create a dependency.
    let pyproject_toml = context.temp_dir.child("core").child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "core"
        version = "0.1.0"
        requires-python = ">=3.12"

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv]
        package = false
        "#,
    )?;

    context
        .temp_dir
        .child("core")
        .child("src")
        .child("core")
        .child("__init__.py")
        .touch()?;

    // Create a package that depends on it.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.0.0"
        requires-python = ">=3.12"
        dependencies = ["core"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv.sources]
        core = { path = "./core" }
        "#,
    )?;

    context
        .temp_dir
        .child("src")
        .child("project")
        .child("__init__.py")
        .touch()?;

    // Syncing the project should _not_ install `core`.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + project==0.0.0 (from file://[TEMP_DIR]/)
    ");

    // Mark the source as `package = true`.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.0.0"
        requires-python = ">=3.12"
        dependencies = ["core"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv.sources]
        core = { path = "./core", package = true }
        "#,
    )?;

    // Syncing the project _should_ install `core`.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 2 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 2 packages in [TIME]
     + core==0.1.0 (from file://[TEMP_DIR]/core)
     ~ project==0.0.0 (from file://[TEMP_DIR]/)
    ");

    // Remove `package = false`.
    let pyproject_toml = context.temp_dir.child("core").child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "core"
        version = "0.1.0"
        requires-python = ">=3.12"

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;

    // Syncing the project _should_ install `core`.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ core==0.1.0 (from file://[TEMP_DIR]/core)
    ");

    // Mark the source as `package = false`.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.0.0"
        requires-python = ">=3.12"
        dependencies = ["core"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv.sources]
        core = { path = "./core", package = false }
        "#,
    )?;

    // Syncing the project should _not_ install `core`.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 2 packages in [TIME]
    Installed 1 package in [TIME]
     - core==0.1.0 (from file://[TEMP_DIR]/core)
     ~ project==0.0.0 (from file://[TEMP_DIR]/)
    ");

    // Update the source `tool.uv` to `package = true`
    let pyproject_toml = context.temp_dir.child("core").child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "core"
        version = "0.1.0"
        requires-python = ">=3.12"

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv]
        package = true
        "#,
    )?;

    // Mark the source as `package = false`.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.0.0"
        requires-python = ">=3.12"
        dependencies = ["core"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv.sources]
        core = { path = "./core", package = false }
        "#,
    )?;

    // Syncing the project should _not_ install `core`.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ project==0.0.0 (from file://[TEMP_DIR]/)
    ");

    // Remove the `package = false` mark.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.0.0"
        requires-python = ">=3.12"
        dependencies = ["core"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv.sources]
        core = { path = "./core" }
        "#,
    )?;

    // Syncing the project _should_ install `core`.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 2 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 2 packages in [TIME]
     + core==0.1.0 (from file://[TEMP_DIR]/core)
     ~ project==0.0.0 (from file://[TEMP_DIR]/)
    ");

    Ok(())
}

/// Avoid installing dev dependencies of transitive dependencies.
#[test]
fn transitive_dev() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "root"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [tool.uv]
        dev-dependencies = ["anyio>3"]

        [tool.uv.sources]
        child = { workspace = true }

        [tool.uv.workspace]
        members = ["child"]
        "#,
    )?;

    let src = context.temp_dir.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    let child = context.temp_dir.child("child");
    fs_err::create_dir_all(&child)?;

    let pyproject_toml = child.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.12"

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"

        [tool.uv]
        dev-dependencies = ["iniconfig>=1"]
        "#,
    )?;

    let src = child.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    uv_snapshot!(context.filters(), context.sync().arg("--dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `tool.uv.dev-dependencies` field (used in `child/pyproject.toml`, `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
    Resolved 6 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

/// Avoid installing dev dependencies of transitive dependencies.
#[test]
fn sync_no_editable() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "root"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"

        [tool.uv.sources]
        child = { workspace = true }

        [tool.uv.workspace]
        members = ["child"]
        "#,
    )?;

    let src = context.temp_dir.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    let child = context.temp_dir.child("child");
    fs_err::create_dir_all(&child)?;

    let pyproject_toml = child.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.12"

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    let src = child.child("src").child("child");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    uv_snapshot!(context.filters(), context.sync().arg("--no-editable"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + root==0.1.0 (from file://[TEMP_DIR]/)
    ");

    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_NO_EDITABLE, "1"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 2 packages in [TIME]
    ");

    // Remove the project.
    fs_err::remove_dir_all(&child)?;

    // Ensure that we can still import it.
    uv_snapshot!(context.filters(), context.run().arg("--no-sync").arg("python").arg("-c").arg("import child"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    "###);

    Ok(())
}

#[test]
fn sync_dry_run() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.9", "3.12"]);

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

    // Perform a `--dry-run`.
    uv_snapshot!(context.filters(), context.sync().arg("--dry-run"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Would create project environment at: .venv
    Resolved 2 packages in [TIME]
    Would create lockfile at: uv.lock
    Would download 1 package
    Would install 1 package
     + iniconfig==2.0.0
    ");

    // Perform a full sync.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // Update the requirements.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().arg("--dry-run"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Would use project environment at: .venv
    Resolved 2 packages in [TIME]
    Would update lockfile at: uv.lock
    Would download 1 package
    Would uninstall 1 package
    Would install 1 package
     - iniconfig==2.0.0
     + typing-extensions==4.10.0
    ");

    // Update the `requires-python`.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = "==3.9.*"
        dependencies = ["iniconfig"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().arg("--dry-run"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
    Would replace project environment at: .venv
    warning: Resolving despite existing lockfile due to fork markers being disjoint with `requires-python`: `python_full_version >= '3.12'` vs `python_full_version == '3.9.*'`
    Resolved 2 packages in [TIME]
    Would update lockfile at: uv.lock
    Would install 1 package
     + iniconfig==2.0.0
    ");

    // Perform a full sync.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    warning: Resolving despite existing lockfile due to fork markers being disjoint with `requires-python`: `python_full_version >= '3.12'` vs `python_full_version == '3.9.*'`
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // TMP: Attempt to catch this flake with verbose output
    // See https://github.com/astral-sh/uv/issues/13744
    let output = context.sync().arg("--dry-run").arg("-vv").output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Would replace existing virtual environment"),
        "{}",
        stderr
    );

    uv_snapshot!(context.filters(), context.sync().arg("--dry-run"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Would use project environment at: .venv
    Resolved 2 packages in [TIME]
    Found up-to-date lockfile at: uv.lock
    Audited 1 package in [TIME]
    Would make no changes
    ");

    Ok(())
}

#[test]
fn sync_dry_run_and_locked() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]
        "#,
    )?;

    // Lock the initial requirements.
    context.lock().assert().success();

    let existing = context.read("uv.lock");

    // Update the requirements.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    // Running with `--locked` and `--dry-run` should error.
    uv_snapshot!(context.filters(), context.sync().arg("--locked").arg("--dry-run"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Would use project environment at: .venv
    Resolved 2 packages in [TIME]
    Would download 1 package
    Would install 1 package
     + iniconfig==2.0.0
    The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
    ");

    let updated = context.read("uv.lock");

    // And the lockfile should be unchanged.
    assert_eq!(existing, updated);

    Ok(())
}

#[test]
fn sync_dry_run_and_frozen() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]
        "#,
    )?;

    // Lock the initial requirements.
    context.lock().assert().success();

    // Update the requirements.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    // Running with `--frozen` with `--dry-run` should preview dependencies to be installed.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen").arg("--dry-run"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Would use project environment at: .venv
    Would download 3 packages
    Would install 3 packages
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}
