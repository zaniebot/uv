use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use indoc::{formatdoc, indoc};
use insta::assert_snapshot;
use tempfile::tempdir_in;

use predicates::prelude::predicate;
use uv_static::EnvVars;

use crate::common::{TestContext, packse_index_url, uv_snapshot, venv_bin_path};

#[test]
fn virtual_no_build() -> Result<()> {
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

    // Generate a lockfile.
    context.lock().assert().success();

    // Clear the cache.
    fs_err::remove_dir_all(&context.cache_dir)?;

    // `--no-build` should not raise an error, since we don't install virtual projects.
    uv_snapshot!(context.filters(), context.sync().arg("--no-build"), @r"
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
fn virtual_empty() -> Result<()> {
    // testing how `uv sync` reacts to a pyproject with no `[project]` and nothing useful to it
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [tool.mycooltool]
        wow = "someconfig"
    "#})?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
    Resolved in [TIME]
    Audited in [TIME]
    ");

    Ok(())
}

#[test]
fn virtual_dependency_group() -> Result<()> {
    // testing basic `uv sync --group` functionality
    // when the pyproject.toml is fully virtual (no `[project]`)
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [dependency-groups]
        foo = ["sortedcontainers"]
        bar = ["iniconfig"]
        dev = ["sniffio"]
    "#})?;

    // default groups
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + sniffio==1.3.1
    ");

    // explicit --group
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // explicit --only-group
    uv_snapshot!(context.filters(), context.sync()
        .arg("--only-group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 2 packages in [TIME]
    Installed 1 package in [TIME]
     - iniconfig==2.0.0
     - sniffio==1.3.1
     + sortedcontainers==2.4.0
    ");

    Ok(())
}

#[test]
fn virtual_no_build_dynamic_cached() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dynamic = ["dependencies"]

        [tool.setuptools.dynamic]
        dependencies = {file = ["requirements.txt"]}
        "#,
    )?;

    context
        .temp_dir
        .child("requirements.txt")
        .write_str("anyio==3.7.0")?;

    // Generate a lockfile.
    context.lock().assert().success();

    // `--no-build` should not raise an error, since we don't build or install the project (given
    // that it's virtual and the metadata is cached).
    uv_snapshot!(context.filters(), context.sync().arg("--no-build"), @r"
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
fn virtual_no_build_dynamic_no_cache() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dynamic = ["dependencies"]

        [tool.setuptools.dynamic]
        dependencies = {file = ["requirements.txt"]}
        "#,
    )?;

    context
        .temp_dir
        .child("requirements.txt")
        .write_str("anyio==3.7.0")?;

    // Generate a lockfile.
    context.lock().assert().success();

    // Clear the cache.
    fs_err::remove_dir_all(&context.cache_dir)?;

    // `--no-build` should raise an error, since we need to build the project.
    uv_snapshot!(context.filters(), context.sync().arg("--no-build"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to generate package metadata for `project==0.1.0 @ virtual+.`
      Caused by: Building source distributions for `project` is disabled
    ");

    Ok(())
}

/// Convert from a package to a virtual project.
#[test]
fn convert_to_virtual() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    // Running `uv sync` should install the project itself.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + iniconfig==2.0.0
     + project==0.1.0 (from file://[TEMP_DIR]/)
    ");

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

        [package.metadata]
        requires-dist = [{ name = "iniconfig" }]
        "#
        );
    });

    // Remove the build system.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    // Running `uv sync` should remove the project itself.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - project==0.1.0 (from file://[TEMP_DIR]/)
    ");

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
        source = { virtual = "." }
        dependencies = [
            { name = "iniconfig" },
        ]

        [package.metadata]
        requires-dist = [{ name = "iniconfig" }]
        "#
        );
    });

    Ok(())
}

#[test]
fn sync_custom_environment_path() -> Result<()> {
    let mut context = TestContext::new_with_versions(&["3.11", "3.12"])
        .with_filtered_virtualenv_bin()
        .with_filtered_python_names();

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

    // Running `uv sync` should create `.venv` by default
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

    context
        .temp_dir
        .child(".venv")
        .assert(predicate::path::is_dir());

    // Running `uv sync` should create `foo` in the project directory when customized
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo"), @r"
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

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::is_dir());

    // We don't delete `.venv`, though we arguably could
    context
        .temp_dir
        .child(".venv")
        .assert(predicate::path::is_dir());

    // An absolute path can be provided
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, "foobar/.venv"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: foobar/.venv
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    context
        .temp_dir
        .child("foobar")
        .assert(predicate::path::is_dir());

    context
        .temp_dir
        .child("foobar")
        .child(".venv")
        .assert(predicate::path::is_dir());

    // An absolute path can be provided
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, context.temp_dir.join("bar")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: bar
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    context
        .temp_dir
        .child("bar")
        .assert(predicate::path::is_dir());

    // And, it can be outside the project
    let tempdir = tempdir_in(TestContext::test_bucket_dir())?;
    context = context.with_filtered_path(tempdir.path(), "OTHER_TEMPDIR");
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, tempdir.path().join(".venv")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: [OTHER_TEMPDIR]/.venv
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    ChildPath::new(tempdir.path())
        .child(".venv")
        .assert(predicate::path::is_dir());

    // If the directory already exists and is not a virtual environment we should fail with an error
    fs_err::remove_dir_all(context.temp_dir.join("foo"))?;
    fs_err::create_dir(context.temp_dir.join("foo"))?;
    fs_err::write(context.temp_dir.join("foo").join("file"), b"")?;
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project virtual environment directory `[TEMP_DIR]/foo` cannot be used because it is not a valid Python environment (no Python executable was found)
    "###);

    // But if it's just an incompatible virtual environment...
    fs_err::remove_dir_all(context.temp_dir.join("foo"))?;
    uv_snapshot!(context.filters(), context.venv().arg("foo").arg("--python").arg("3.11"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    warning: The requested interpreter resolved to Python 3.11.[X], which is incompatible with the project's Python requirement: `>=3.12` (from `project.requires-python`)
    Creating virtual environment at: foo
    Activate with: source foo/[BIN]/activate
    ");

    // Even with some extraneous content...
    fs_err::write(context.temp_dir.join("foo").join("file"), b"")?;

    // We can delete and use it
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: foo
    Creating virtual environment at: foo
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    Ok(())
}

#[test]
fn sync_active_project_environment() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.11", "3.12"])
        .with_filtered_virtualenv_bin()
        .with_filtered_python_names();

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.11"
        dependencies = ["iniconfig"]
        "#,
    )?;

    // Running `uv sync` with `VIRTUAL_ENV` should warn
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, "foo"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: `VIRTUAL_ENV=foo` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    "###);

    context
        .temp_dir
        .child(".venv")
        .assert(predicate::path::is_dir());

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::missing());

    // Using `--active` should create the environment
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, "foo").arg("--active"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    Creating virtual environment at: foo
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::is_dir());

    // A subsequent sync will re-use the environment
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::VIRTUAL_ENV, "foo").arg("--active"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // Setting both the `VIRTUAL_ENV` and `UV_PROJECT_ENVIRONMENT` is fine if they agree
    uv_snapshot!(context.filters(), context.sync()
        .arg("--active")
        .env(EnvVars::VIRTUAL_ENV, "foo")
        .env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // If they disagree, we use `VIRTUAL_ENV` because of `--active`
    uv_snapshot!(context.filters(), context.sync()
        .arg("--active")
        .env(EnvVars::VIRTUAL_ENV, "foo")
        .env(EnvVars::UV_PROJECT_ENVIRONMENT, "bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    context
        .temp_dir
        .child("bar")
        .assert(predicate::path::missing());

    // Requesting another Python version will invalidate the environment
    uv_snapshot!(context.filters(), context.sync()
        .env(EnvVars::VIRTUAL_ENV, "foo").arg("--active").arg("-p").arg("3.12"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: foo
    Creating virtual environment at: foo
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    Ok(())
}

#[test]
fn sync_active_script_environment() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.11", "3.12"])
        .with_filtered_virtualenv_bin()
        .with_filtered_python_names();

    let script = context.temp_dir.child("script.py");
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.11"
        # dependencies = [
        #   "anyio",
        # ]
        # ///

        import anyio
       "#
    })?;

    // Running `uv sync --script` with `VIRTUAL_ENV` should warn
    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").env(EnvVars::VIRTUAL_ENV, "foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: `VIRTUAL_ENV=foo` does not match the script environment path `[CACHE_DIR]/environments-v2/script-[HASH]` and will be ignored; use `--active` to target the active environment instead
    Creating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::missing());

    // Using `--active` should create the environment
    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").env(EnvVars::VIRTUAL_ENV, "foo").arg("--active"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Creating script environment at: foo
    Resolved 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::is_dir());

    // A subsequent sync will re-use the environment
    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").env(EnvVars::VIRTUAL_ENV, "foo").arg("--active"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using script environment at: foo
    Resolved 3 packages in [TIME]
    Audited 3 packages in [TIME]
    ");

    // Requesting another Python version will invalidate the environment
    uv_snapshot!(context.filters(), context.sync()
        .arg("--script")
        .arg("script.py")
        .env(EnvVars::VIRTUAL_ENV, "foo")
        .arg("--active")
        .arg("-p")
        .arg("3.12"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Updating script environment at: foo
    Resolved 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn sync_active_script_environment_json() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.11", "3.12"])
        .with_filtered_virtualenv_bin()
        .with_filtered_exe_suffix();

    let script = context.temp_dir.child("script.py");
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.11"
        # dependencies = [
        #   "anyio",
        # ]
        # ///

        import anyio
       "#
    })?;

    // Running `uv sync --script` with `VIRTUAL_ENV` should warn
    uv_snapshot!(context.filters(), context.sync()
        .arg("--script").arg("script.py")
        .arg("--output-format").arg("json")
        .env(EnvVars::VIRTUAL_ENV, "foo"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "script",
      "script": {
        "path": "[TEMP_DIR]/script.py"
      },
      "sync": {
        "environment": {
          "path": "[CACHE_DIR]/environments-v2/script-[HASH]",
          "python": {
            "path": "[CACHE_DIR]/environments-v2/script-[HASH]/[BIN]/python",
            "version": "3.11.[X]",
            "implementation": "cpython"
          }
        },
        "action": "create",
        "changes": [
          {
            "name": "anyio",
            "version": "4.3.0",
            "action": "installed"
          },
          {
            "name": "idna",
            "version": "3.6",
            "action": "installed"
          },
          {
            "name": "sniffio",
            "version": "1.3.1",
            "action": "installed"
          }
        ]
      },
      "lock": null,
      "dry_run": false
    }

    ----- stderr -----
    warning: `VIRTUAL_ENV=foo` does not match the script environment path `[CACHE_DIR]/environments-v2/script-[HASH]` and will be ignored; use `--active` to target the active environment instead
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    "#);

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::missing());

    // Using `--active` should create the environment
    uv_snapshot!(context.filters(), context.sync()
        .arg("--script").arg("script.py")
        .arg("--output-format").arg("json")
        .env(EnvVars::VIRTUAL_ENV, "foo").arg("--active"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "script",
      "script": {
        "path": "[TEMP_DIR]/script.py"
      },
      "sync": {
        "environment": {
          "path": "[TEMP_DIR]/foo",
          "python": {
            "path": "[TEMP_DIR]/foo/[BIN]/python",
            "version": "3.11.[X]",
            "implementation": "cpython"
          }
        },
        "action": "create",
        "changes": [
          {
            "name": "anyio",
            "version": "4.3.0",
            "action": "installed"
          },
          {
            "name": "idna",
            "version": "3.6",
            "action": "installed"
          },
          {
            "name": "sniffio",
            "version": "1.3.1",
            "action": "installed"
          }
        ]
      },
      "lock": null,
      "dry_run": false
    }

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    "#);

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::is_dir());

    // A subsequent sync will re-use the environment
    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").env(EnvVars::VIRTUAL_ENV, "foo").arg("--active"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using script environment at: foo
    Resolved 3 packages in [TIME]
    Audited 3 packages in [TIME]
    ");

    // Requesting another Python version will invalidate the environment
    uv_snapshot!(context.filters(), context.sync()
        .arg("--script").arg("script.py")
        .arg("--output-format").arg("json")
        .env(EnvVars::VIRTUAL_ENV, "foo")
        .arg("--active")
        .arg("-p")
        .arg("3.12"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "script",
      "script": {
        "path": "[TEMP_DIR]/script.py"
      },
      "sync": {
        "environment": {
          "path": "[TEMP_DIR]/foo",
          "python": {
            "path": "[TEMP_DIR]/foo/[BIN]/python",
            "version": "3.12.[X]",
            "implementation": "cpython"
          }
        },
        "action": "update",
        "changes": [
          {
            "name": "anyio",
            "version": "4.3.0",
            "action": "installed"
          },
          {
            "name": "idna",
            "version": "3.6",
            "action": "installed"
          },
          {
            "name": "sniffio",
            "version": "1.3.1",
            "action": "installed"
          }
        ]
      },
      "lock": null,
      "dry_run": false
    }

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    "#);

    Ok(())
}

#[test]
#[cfg(feature = "git")]
fn sync_workspace_custom_environment_path() -> Result<()> {
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

    // Create a workspace member
    context.init().arg("child").assert().success();

    // Running `uv sync` should create `.venv` in the workspace root
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    context
        .temp_dir
        .child(".venv")
        .assert(predicate::path::is_dir());

    // Similarly, `uv sync` from the child project uses `.venv` in the workspace root
    uv_snapshot!(context.filters(), context.sync().current_dir(context.temp_dir.join("child")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    context
        .temp_dir
        .child(".venv")
        .assert(predicate::path::is_dir());

    context
        .temp_dir
        .child("child")
        .child(".venv")
        .assert(predicate::path::missing());

    // Running `uv sync` should create `foo` in the workspace root when customized
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: foo
    Resolved 3 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::is_dir());

    // We don't delete `.venv`, though we arguably could
    context
        .temp_dir
        .child(".venv")
        .assert(predicate::path::is_dir());

    // Similarly, `uv sync` from the child project uses `foo` relative to  the workspace root
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo").current_dir(context.temp_dir.join("child")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::is_dir());

    context
        .temp_dir
        .child("child")
        .child("foo")
        .assert(predicate::path::missing());

    // And, `uv sync --package child` uses `foo` relative to  the workspace root
    uv_snapshot!(context.filters(), context.sync().arg("--package").arg("child").env(EnvVars::UV_PROJECT_ENVIRONMENT, "foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Audited in [TIME]
    ");

    context
        .temp_dir
        .child("foo")
        .assert(predicate::path::is_dir());

    context
        .temp_dir
        .child("child")
        .child("foo")
        .assert(predicate::path::missing());

    Ok(())
}

#[test]
fn sync_empty_virtual_environment() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.12"]);

    // Create an empty directory
    context.temp_dir.child(".venv").create_dir_all()?;

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

    // Running `uv sync` should work
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

    Ok(())
}

#[test]
fn sync_environment_prompt() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.12"]);

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "my-project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    // Running `uv sync` should create `.venv`
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

    // The `pyvenv.cfg` should contain the prompt matching the project name
    let pyvenv_cfg = context.read(".venv/pyvenv.cfg");

    assert!(pyvenv_cfg.contains("prompt = my-project"));

    Ok(())
}

#[test]
fn sync_invalid_environment() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.11", "3.12"])
        .with_filtered_virtualenv_bin()
        .with_filtered_python_names();

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

    // If the directory already exists and is not a virtual environment we should fail with an error
    fs_err::create_dir(context.temp_dir.join(".venv"))?;
    fs_err::write(context.temp_dir.join(".venv").join("file"), b"")?;
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project virtual environment directory `[VENV]/` cannot be used because it is not a valid Python environment (no Python executable was found)
    ");

    // But if it's just an incompatible virtual environment...
    fs_err::remove_dir_all(context.temp_dir.join(".venv"))?;
    uv_snapshot!(context.filters(), context.venv().arg("--python").arg("3.11"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    warning: The requested interpreter resolved to Python 3.11.[X], which is incompatible with the project's Python requirement: `>=3.12` (from `project.requires-python`)
    Creating virtual environment at: .venv
    Activate with: source .venv/[BIN]/activate
    ");

    // Even with some extraneous content...
    fs_err::write(context.temp_dir.join(".venv").join("file"), b"")?;

    // We can delete and use it
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    let bin = venv_bin_path(context.temp_dir.join(".venv"));

    // If there's just a broken symlink, we should warn
    #[cfg(unix)]
    {
        fs_err::remove_file(bin.join("python"))?;
        fs_err::os::unix::fs::symlink(context.temp_dir.join("does-not-exist"), bin.join("python"))?;
        uv_snapshot!(context.filters(), context.sync(), @r"
        success: true
        exit_code: 0
        ----- stdout -----

        ----- stderr -----
        warning: Ignoring existing virtual environment linked to non-existent Python interpreter: .venv/[BIN]/[PYTHON] -> python
        Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
        Removed virtual environment at: .venv
        Creating virtual environment at: .venv
        Resolved 2 packages in [TIME]
        Installed 1 package in [TIME]
         + iniconfig==2.0.0
        ");
    }

    // If the Python executable is missing entirely, we'll delete and use it
    fs_err::remove_dir_all(&bin)?;
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // But if it's not a virtual environment...
    fs_err::remove_dir_all(context.temp_dir.join(".venv"))?;
    uv_snapshot!(context.filters(), context.venv().arg("--python").arg("3.11"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    warning: The requested interpreter resolved to Python 3.11.[X], which is incompatible with the project's Python requirement: `>=3.12` (from `project.requires-python`)
    Creating virtual environment at: .venv
    Activate with: source .venv/[BIN]/activate
    ");

    // Which we detect by the presence of a `pyvenv.cfg` file
    fs_err::remove_file(context.temp_dir.join(".venv").join("pyvenv.cfg"))?;

    // Let's make sure some extraneous content isn't removed
    fs_err::write(context.temp_dir.join(".venv").join("file"), b"")?;

    // We should never delete it
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    error: Project virtual environment directory `[VENV]/` cannot be used because it is not a compatible environment but cannot be recreated because it is not a virtual environment
    ");

    // Even if there's no Python executable
    fs_err::remove_dir_all(&bin)?;
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Project virtual environment directory `[VENV]/` cannot be used because it is not a valid Python environment (no Python executable was found)
    ");

    context
        .temp_dir
        .child(".venv")
        .assert(predicate::path::is_dir());

    context
        .temp_dir
        .child(".venv")
        .child("file")
        .assert(predicate::path::is_file());

    Ok(())
}

#[test]
fn sync_partial_environment_delete() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let context = TestContext::new_with_versions(&["3.13", "3.12"]);

    context.init().arg("-p").arg("3.12").assert().success();
    uv_snapshot!(context.filters(), context.sync().arg("-p").arg("3.13"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.13.[X] interpreter at: [PYTHON-3.13]
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    // Create a directory that's unreadable, erroring on trying to delete its children.
    // This relies on our implementation listing directory entries before deleting them — which is a
    // bit of a hack but accomplishes the goal here.
    let unreadable2 = context.temp_dir.child(".venv/z2.txt");
    fs_err::create_dir(&unreadable2)?;
    let perms = std::fs::Permissions::from_mode(0o000);
    fs_err::set_permissions(&unreadable2, perms)?;

    uv_snapshot!(context.filters(), context.sync().arg("-p").arg("3.12"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    error: failed to remove directory `[VENV]/z2.txt`: Permission denied (os error 13)
    ");

    uv_snapshot!(context.filters(), context.sync().arg("-p").arg("3.12"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    error: failed to remove directory `[VENV]/z2.txt`: Permission denied (os error 13)
    ");

    // Remove the unreadable directory
    fs_err::remove_dir(unreadable2)?;

    // We should be able to remove the venv now
    uv_snapshot!(context.filters(), context.sync().arg("-p").arg("3.12"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    Ok(())
}

#[test]
fn sync_when_virtual_environment_incompatible_with_interpreter() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.11"
        dependencies = []
        "#,
    )?;

    // Create a virtual environment at `.venv`.
    context
        .venv()
        .arg(context.venv.as_os_str())
        .arg("--clear")
        .arg("--python")
        .arg("3.12")
        .assert()
        .success();

    // Simulate an incompatible `pyvenv.cfg:version` value created
    // by the venv module.
    let pyvenv_cfg = context.venv.child("pyvenv.cfg");
    let contents = fs_err::read_to_string(&pyvenv_cfg)
        .unwrap()
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("version") {
                "version = 3.11.0".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs_err::write(&pyvenv_cfg, contents)?;

    // We should also be able to read from the lockfile.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    insta::with_settings!({
        filters => context.filters(),
    }, {
        let contents = fs_err::read_to_string(&pyvenv_cfg).unwrap();
        let lines: Vec<&str> = contents.split('\n').collect();
        assert_snapshot!(lines[3], @"version_info = 3.12.[X]");
    });

    // Simulate an incompatible `pyvenv.cfg:version_info` value created
    // by uv or virtualenv.
    let pyvenv_cfg = context.venv.child("pyvenv.cfg");
    let contents = fs_err::read_to_string(&pyvenv_cfg)
        .unwrap()
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("version") {
                "version_info = 3.11.0".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs_err::write(&pyvenv_cfg, contents)?;

    // We should also be able to read from the lockfile.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    insta::with_settings!({
        filters => context.filters(),
    }, {
        let contents = fs_err::read_to_string(&pyvenv_cfg).unwrap();
        let lines: Vec<&str> = contents.split('\n').collect();
        assert_snapshot!(lines[3], @"version_info = 3.12.[X]");
    });

    Ok(())
}

#[test]
fn sync_required_environment_hint() -> Result<()> {
    let context = TestContext::new("3.13");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(&formatdoc! {r#"
        [project]
        name = "example"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["no-sdist-no-wheels-with-matching-platform-a"]

        [[tool.uv.index]]
        name = "packse"
        url = "{}"
        default = true
        "#,
        packse_index_url()
    })?;

    uv_snapshot!(context.filters(), context.lock().env_remove(EnvVars::UV_EXCLUDE_NEWER), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    ");

    let mut filters = context.filters();
    filters.push((
        r"You're on [^ ]+ \(`.*`\)",
        "You're on [PLATFORM] (`[TAG]`)",
    ));
    filters.push((
        r"sys_platform == '[^']+' and platform_machine == '[^']+'",
        "sys_platform == '[PLATFORM]' and platform_machine == '[MACHINE]'",
    ));

    uv_snapshot!(filters, context.sync().env_remove(EnvVars::UV_EXCLUDE_NEWER), @r#"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    error: Distribution `no-sdist-no-wheels-with-matching-platform-a==1.0.0 @ registry+https://astral-sh.github.io/packse/PACKSE_VERSION/simple-html/` can't be installed because it doesn't have a source distribution or wheel for the current platform

    hint: You're on [PLATFORM] (`[TAG]`), but `no-sdist-no-wheels-with-matching-platform-a` (v1.0.0) only has wheels for the following platform: `macosx_10_0_ppc64`; consider adding "sys_platform == '[PLATFORM]' and platform_machine == '[MACHINE]'" to `tool.uv.required-environments` to ensure uv resolves to a version with compatible wheels
    "#);

    Ok(())
}

/// Ensure that when we sync to an empty virtual environment directory, we don't attempt to remove
/// it, which breaks Docker volume mounts.
#[test]
#[cfg(unix)]
fn sync_does_not_remove_empty_virtual_environment_directory() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let context = TestContext::new_with_versions(&["3.12"]);

    let project_dir = context.temp_dir.child("project");
    fs_err::create_dir(&project_dir)?;

    let pyproject_toml = project_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    let venv_dir = project_dir.child(".venv");
    fs_err::create_dir(&venv_dir)?;

    // Ensure the parent is read-only, to prevent deletion of the virtual environment
    fs_err::set_permissions(&project_dir, std::fs::Permissions::from_mode(0o555))?;

    // Note we do _not_ fail to create the virtual environment — we fail later when writing to the
    // project directory
    uv_snapshot!(context.filters(), context.sync().current_dir(&project_dir), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    error: failed to write to file `[TEMP_DIR]/project/uv.lock`: Permission denied (os error 13)
    ");

    Ok(())
}
