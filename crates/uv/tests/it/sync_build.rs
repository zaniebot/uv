use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use indoc::{formatdoc, indoc};
use insta::assert_snapshot;

use uv_fs::Simplified;
use uv_static::EnvVars;

use crate::common::{TestContext, uv_snapshot};

/// Use a `pip install` step to pre-install build dependencies for `--no-build-isolation`.
#[test]
fn sync_build_isolation() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz"]
        "#,
    )?;

    // Running `uv sync` should fail (but it could fail when building the root project, or when
    // building `source-distribution`).
    context
        .sync()
        .arg("--no-build-isolation")
        .assert()
        .failure();

    // Install `setuptools` (for the root project) plus `hatchling` (for `source-distribution`).
    uv_snapshot!(context.filters(), context.pip_install().arg("wheel").arg("setuptools").arg("hatchling"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 7 packages in [TIME]
    Installed 7 packages in [TIME]
     + hatchling==1.22.4
     + packaging==24.0
     + pathspec==0.12.1
     + pluggy==1.4.0
     + setuptools==69.2.0
     + trove-classifiers==2024.3.3
     + wheel==0.43.0
    "###);

    // Running `uv sync` should succeed.
    uv_snapshot!(context.filters(), context.sync().arg("--no-build-isolation"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 7 packages in [TIME]
    Installed 1 package in [TIME]
     - hatchling==1.22.4
     - packaging==24.0
     - pathspec==0.12.1
     - pluggy==1.4.0
     - setuptools==69.2.0
     + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
     - trove-classifiers==2024.3.3
     - wheel==0.43.0
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

/// Use a `pip install` step to pre-install build dependencies for `--no-build-isolation-package`.
#[test]
fn sync_build_isolation_package() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz",
        ]

        [tool.uv]
        no-build-isolation-package = ["source-distribution"]
        "#,
    )?;

    // Running `uv sync` should fail.
    uv_snapshot!(context.filters(), context.sync(), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to build `source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `hatchling.build.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 8, in <module>
          ModuleNotFoundError: No module named 'hatchling'

          hint: This error likely indicates that `source-distribution` depends on `hatchling`, but doesn't declare it as a build dependency. If `source-distribution` is a first-party package, consider adding `hatchling` to its `build-system.requires`. Otherwise, either add it to your `pyproject.toml` under:

          [tool.uv.extra-build-dependencies]
          source-distribution = ["hatchling"]

          or `uv pip install hatchling` into the environment and re-run with `--no-build-isolation`.
      help: `source-distribution` was included because `project` (v0.1.0) depends on `source-distribution`
    "#);

    // Install `hatchling` for `source-distribution`.
    uv_snapshot!(context.filters(), context.pip_install().arg("hatchling"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + hatchling==1.22.4
     + packaging==24.0
     + pathspec==0.12.1
     + pluggy==1.4.0
     + trove-classifiers==2024.3.3
    "###);

    // Running `uv sync` should succeed.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 5 packages in [TIME]
    Installed 1 package in [TIME]
     - hatchling==1.22.4
     - packaging==24.0
     - pathspec==0.12.1
     - pluggy==1.4.0
     + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
     - trove-classifiers==2024.3.3
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

/// By default, isolated dependencies should be installed before non-isolated dependencies.
#[test]
fn sync_build_isolation_package_order() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz",
        ]

        [tool.uv]
        no-build-isolation-package = ["source-distribution"]
        "#,
    )?;

    // Running `uv sync` should fail.
    uv_snapshot!(context.filters(), context.sync(), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to build `source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `hatchling.build.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 8, in <module>
          ModuleNotFoundError: No module named 'hatchling'

          hint: This error likely indicates that `source-distribution` depends on `hatchling`, but doesn't declare it as a build dependency. If `source-distribution` is a first-party package, consider adding `hatchling` to its `build-system.requires`. Otherwise, either add it to your `pyproject.toml` under:

          [tool.uv.extra-build-dependencies]
          source-distribution = ["hatchling"]

          or `uv pip install hatchling` into the environment and re-run with `--no-build-isolation`.
      help: `source-distribution` was included because `project` (v0.1.0) depends on `source-distribution`
    "#);

    // Add `hatchling`.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "hatchling",
            "source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz",
        ]

        [tool.uv]
        no-build-isolation-package = ["source-distribution"]
        "#,
    )?;

    // Running `uv sync` should succeed; `hatchling` should be installed first.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
    Prepared 1 package without build isolation in [TIME]
    Installed 1 package in [TIME]
     + hatchling==1.22.4
     + packaging==24.0
     + pathspec==0.12.1
     + pluggy==1.4.0
     + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
     + trove-classifiers==2024.3.3
    ");

    // Modify the version.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "source-distribution @ https://files.pythonhosted.org/packages/1f/e5/5b016c945d745f8b108e759d428341488a6aee8f51f07c6c4e33498bb91f/source_distribution-0.0.3.tar.gz",
        ]

        [tool.uv]
        no-build-isolation-package = ["source-distribution"]
        "#,
    )?;

    // Running `uv sync` should uninstall `hatchling`, then build `source-distribution`, then uninstall
    // the existing `source-distribution`, and finally install the new one.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 6 packages in [TIME]
    Installed 1 package in [TIME]
     - hatchling==1.22.4
     - packaging==24.0
     - pathspec==0.12.1
     - pluggy==1.4.0
     - source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
     + source-distribution==0.0.3 (from https://files.pythonhosted.org/packages/1f/e5/5b016c945d745f8b108e759d428341488a6aee8f51f07c6c4e33498bb91f/source_distribution-0.0.3.tar.gz)
     - trove-classifiers==2024.3.3
    ");

    // Revert back.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "hatchling",
            "source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz",
        ]

        [tool.uv]
        no-build-isolation-package = ["source-distribution"]
        "#,
    )?;

    // Running `uv sync` should install everything in a single phase, since the build is cached.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 6 packages in [TIME]
     + hatchling==1.22.4
     + packaging==24.0
     + pathspec==0.12.1
     + pluggy==1.4.0
     - source-distribution==0.0.3 (from https://files.pythonhosted.org/packages/1f/e5/5b016c945d745f8b108e759d428341488a6aee8f51f07c6c4e33498bb91f/source_distribution-0.0.3.tar.gz)
     + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
     + trove-classifiers==2024.3.3
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

/// Use dedicated extra groups to install dependencies for `--no-build-isolation-package`.
#[test]
fn sync_build_isolation_extra() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [project.optional-dependencies]
        build = ["hatchling"]
        compile = ["source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz"]

        [build-system]
        requires = ["setuptools >= 40.9.0"]
        build-backend = "setuptools.build_meta"

        [tool.uv]
        no-build-isolation-package = ["source-distribution"]
        "#,
    )?;

    // Running `uv sync` should fail for the `compile` extra.
    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("compile"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
      × Failed to build `source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `hatchling.build.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 8, in <module>
          ModuleNotFoundError: No module named 'hatchling'

          hint: This error likely indicates that `source-distribution` depends on `hatchling`, but doesn't declare it as a build dependency. If `source-distribution` is a first-party package, consider adding `hatchling` to its `build-system.requires`. Otherwise, either add it to your `pyproject.toml` under:

          [tool.uv.extra-build-dependencies]
          source-distribution = ["hatchling"]

          or `uv pip install hatchling` into the environment and re-run with `--no-build-isolation`.
      help: `source-distribution` was included because `project[compile]` (v0.1.0) depends on `source-distribution`
    "#);

    // Running `uv sync` with `--all-extras` should succeed, because we install the build dependencies
    // first.
    uv_snapshot!(context.filters(), context.sync().arg("--all-extras"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
    Prepared [N] packages without build isolation in [TIME]
    Installed [N] packages in [TIME]
     + hatchling==1.22.4
     + packaging==24.0
     + pathspec==0.12.1
     + pluggy==1.4.0
     + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
     + trove-classifiers==2024.3.3
    ");

    // Clear the virtual environment.
    context.venv().arg("--clear").assert().success();

    // Clear the cache.
    fs_err::remove_dir_all(&context.cache_dir)?;

    // Install the build dependencies.
    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("build"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + hatchling==1.22.4
     + packaging==24.0
     + pathspec==0.12.1
     + pluggy==1.4.0
     + project==0.1.0 (from file://[TEMP_DIR]/)
     + trove-classifiers==2024.3.3
    ");

    // Running `uv sync` for the `compile` extra should succeed, and remove the build dependencies.
    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("compile"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Uninstalled [N] packages in [TIME]
    Installed [N] packages in [TIME]
     - hatchling==1.22.4
     - packaging==24.0
     - pathspec==0.12.1
     - pluggy==1.4.0
     + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
     - trove-classifiers==2024.3.3
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

#[test]
fn sync_extra_build_dependencies() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Write a test package that arbitrarily requires `anyio` at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(indoc! {r#"
        import sys

        from hatchling.build import *

        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)
    "#})?;
    child.child("src/child/__init__.py").touch()?;

    let parent = &context.temp_dir;
    let pyproject_toml = parent.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    context.venv().arg("--clear").assert().success();
    // Running `uv sync` should fail due to missing build-dependencies
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Missing `anyio` module

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    // Adding `extra-build-dependencies` should solve the issue
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio"]
    "#})?;

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Adding `extra-build-dependencies` with the wrong name should fail the build
    // (the cache is invalidated when extra build dependencies change)
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        wrong_name = ["anyio"]
    "#})?;

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Missing `anyio` module

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    // Write a test package that arbitrarily bans `anyio` at build time
    let bad_child = context.temp_dir.child("bad_child");
    bad_child.create_dir_all()?;
    let bad_child_pyproject_toml = bad_child.child("pyproject.toml");
    bad_child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "bad_child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;
    let build_backend = bad_child.child("build_backend.py");
    build_backend.write_str(indoc! {r#"
        import sys

        from hatchling.build import *

        try:
            import anyio
        except ModuleNotFoundError:
            pass
        else:
            print("Found `anyio` module", file=sys.stderr)
            sys.exit(1)
    "#})?;
    bad_child.child("src/bad_child/__init__.py").touch()?;

    // Depend on `bad_child` too
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child", "bad_child"]

        [tool.uv.sources]
        child = { path = "child" }
        bad_child = { path = "bad_child" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio"]
        bad_child = ["anyio"]
    "#})?;

    // Confirm that `bad_child` fails if anyio is provided
    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
      × Failed to build `bad-child @ file://[TEMP_DIR]/bad_child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Found `anyio` module

          hint: This usually indicates a problem with the package or the build environment.
      help: `bad-child` was included because `parent` (v0.1.0) depends on `bad-child`
    ");

    // But `anyio` is not provided to `bad_child` if scoped to `child`
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child", "bad_child"]

        [tool.uv.sources]
        child = { path = "child" }
        bad_child = { path = "bad_child" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio"]
    "#})?;

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + bad-child==0.1.0 (from file://[TEMP_DIR]/bad_child)
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_setuptools_legacy() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Write a test package that uses legacy setuptools (no pyproject.toml) and requires `anyio` at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;

    // Create a setup.py that checks for anyio during build
    let setup_py = child.child("setup.py");
    setup_py.write_str(indoc! {r#"
        import sys
        from setuptools import setup, find_packages

        try:
            import anyio
            print("anyio is available!", file=sys.stderr)
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        setup(
            name="child",
            version="0.1.0",
            packages=find_packages(),
        )
    "#})?;
    child.child("child").create_dir_all()?;
    child.child("child/__init__.py").touch()?;

    let parent = &context.temp_dir;
    let pyproject_toml = parent.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    // Running `uv sync` should fail due to missing build-dependencies
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `setuptools.build_meta:__legacy__.build_wheel` failed (exit status: 1)

          [stderr]
          Missing `anyio` module

          hint: This usually indicates a problem with the package or the build environment.
    ");

    // Adding `extra-build-dependencies` should solve the issue
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio"]
    "#})?;

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_setuptools() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Write a test package that uses setuptools with pyproject.toml and requires `anyio` at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;

    // Create a pyproject.toml that uses setuptools
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["setuptools"]
        build-backend = "setuptools.build_meta"
    "#})?;

    // Create a setup.py that checks for anyio during build
    let setup_py = child.child("setup.py");
    setup_py.write_str(indoc! {r#"
        import sys
        from setuptools import setup, find_packages

        try:
            import anyio
            print("anyio is available!", file=sys.stderr)
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        setup(
            name="child",
            version="0.1.0",
            packages=find_packages(),
        )
    "#})?;
    child.child("child").create_dir_all()?;
    child.child("child/__init__.py").touch()?;

    let parent = &context.temp_dir;
    let pyproject_toml = parent.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    // Running `uv sync` should fail due to missing build-dependencies
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `setuptools.build_meta.build_wheel` failed (exit status: 1)

          [stderr]
          Missing `anyio` module

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    // Adding `extra-build-dependencies` should solve the issue
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio"]
    "#})?;

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_sources() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    let anyio_local = context.workspace_root.join("test/packages/anyio_local");

    // Write a test package that arbitrarily requires `anyio` at a specific _path_ at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(&formatdoc! {r#"
        import sys

        from hatchling.build import *

        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        # Check that we got the local version of anyio by checking for the marker
        if not hasattr(anyio, 'LOCAL_ANYIO_MARKER'):
            print("Found system anyio instead of local anyio", file=sys.stderr)
            sys.exit(1)
    "#})?;
    child.child("src/child/__init__.py").touch()?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(&formatdoc! {r#"
            [project]
            name = "project"
            version = "0.1.0"
            requires-python = ">=3.12"
            dependencies = ["child"]

            [tool.uv.sources]
            anyio = {{ path = "{anyio_local}" }}
            child = {{ path = "child" }}

            [tool.uv.extra-build-dependencies]
            child = ["anyio"]
        "#,
        anyio_local = anyio_local.portable_display(),
    })?;

    // Running `uv sync` should succeed, as `anyio` is provided as a source
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // TODO(zanieb): We want to test with `--no-sources` too but unfortunately that's not easy
    // because it'll disable the `child` path source too!

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_index() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Write a test package that arbitrarily requires `anyio` at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling", "anyio"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;

    // Create a build backend that checks for a specific version of anyio
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(indoc! {r#"
        import os
        import sys
        from hatchling.build import *

        expected_version = os.environ.get("EXPECTED_ANYIO_VERSION", "")
        if not expected_version:
            print("`EXPECTED_ANYIO_VERSION` not set", file=sys.stderr)
            sys.exit(1)

        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        from importlib.metadata import version
        anyio_version = version("anyio")

        if not anyio_version.startswith(expected_version):
            print(f"Expected `anyio` version {expected_version} but got {anyio_version}", file=sys.stderr)
            sys.exit(1)

        print(f"Found expected `anyio` version {anyio_version}", file=sys.stderr)
    "#})?;
    child.child("src/child/__init__.py").touch()?;

    let parent = &context.temp_dir;
    let pyproject_toml = parent.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    // Ensure our build backend is checking the version correctly
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::EXPECTED_ANYIO_VERSION, "3.0"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Expected `anyio` version 3.0 but got 4.3.0

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    // Ensure that we're resolving to `4.3.0`, the "latest" on PyPI.
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::EXPECTED_ANYIO_VERSION, "4.3"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Pin `anyio` to the Test PyPI.
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
        anyio = { index = "test" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio"]

        [[tool.uv.index]]
        url = "https://test.pypi.org/simple"
        name = "test"
        explicit = true
    "#})?;

    // The child should be rebuilt with `3.5` on reinstall, the "latest" on Test PyPI.
    uv_snapshot!(context.filters(), context.sync()
        .arg("--reinstall-package").arg("child").env(EnvVars::EXPECTED_ANYIO_VERSION, "4.3"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Expected `anyio` version 4.3 but got 3.5.0

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    uv_snapshot!(context.filters(), context.sync()
        .arg("--reinstall-package").arg("child").env(EnvVars::EXPECTED_ANYIO_VERSION, "3.5"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Uninstalled [N] packages in [TIME]
    Installed [N] packages in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_sources_from_child() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    let anyio_local = context.workspace_root.join("test/packages/anyio_local");

    // Write a test package that arbitrarily requires `anyio` at a specific _path_ at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(&formatdoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling"]
        backend-path = ["."]
        build-backend = "build_backend"

        [tool.uv.sources]
        anyio = {{ path = "{}" }}
    "#, anyio_local.portable_display()
    })?;
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(&formatdoc! {r#"
        import sys

        from hatchling.build import *

        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        # Check that we got the local version of anyio by checking for the marker
        if not hasattr(anyio, 'LOCAL_ANYIO_MARKER'):
            print("Found system anyio instead of local anyio", file=sys.stderr)
            sys.exit(1)
    "#})?;
    child.child("src/child/__init__.py").touch()?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
            [project]
            name = "project"
            version = "0.1.0"
            requires-python = ">=3.12"
            dependencies = ["child"]

            [tool.uv.sources]
            child = { path = "child" }

            [tool.uv.extra-build-dependencies]
            child = ["anyio"]
        "#,
    })?;

    // Running `uv sync` should fail due to the unapplied source
    uv_snapshot!(context.filters(), context.sync().arg("--reinstall").arg("--refresh"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Found system anyio instead of local anyio

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `project` (v0.1.0) depends on `child`
    ");

    Ok(())
}

#[test]
fn sync_build_dependencies_module_error_hints() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Write a test package that arbitrarily requires `anyio` at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(indoc! {r"
        import sys

        from hatchling.build import *
        import anyio
    "})?;
    child.child("src/child/__init__.py").touch()?;

    let parent = &context.temp_dir;
    let pyproject_toml = parent.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    context.venv().arg("--clear").assert().success();
    // Running `uv sync` should fail due to missing build-dependencies
    uv_snapshot!(context.filters(), context.sync(), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 8, in <module>
            File "[TEMP_DIR]/child/build_backend.py", line 4, in <module>
              import anyio
          ModuleNotFoundError: No module named 'anyio'

          hint: This error likely indicates that `child@0.1.0` depends on `anyio`, but doesn't declare it as a build dependency. If `child` is a first-party package, consider adding `anyio` to its `build-system.requires`. Otherwise, either add it to your `pyproject.toml` under:

          [tool.uv.extra-build-dependencies]
          child = ["anyio"]

          or `uv pip install anyio` into the environment and re-run with `--no-build-isolation`.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    "#);

    // Adding `extra-build-dependencies` should solve the issue
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio"]
    "#})?;

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Assert pipreqs module name to package name lookup works.
    build_backend.write_str(indoc! {r"
        import sys

        from hatchling.build import *
        import anyio
        import sklearn
    "})?;

    context.venv().arg("--clear").assert().success();
    // Running `uv sync` should fail due to missing build-dependencies
    uv_snapshot!(context.filters(), context.sync().arg("--reinstall"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 8, in <module>
            File "[TEMP_DIR]/child/build_backend.py", line 5, in <module>
              import sklearn
          ModuleNotFoundError: No module named 'sklearn'

          hint: This error likely indicates that `child@0.1.0` depends on `scikit-learn`, but doesn't declare it as a build dependency. If `child` is a first-party package, consider adding `scikit-learn` to its `build-system.requires`. Otherwise, either add it to your `pyproject.toml` under:

          [tool.uv.extra-build-dependencies]
          child = ["scikit-learn"]

          or `uv pip install scikit-learn` into the environment and re-run with `--no-build-isolation`.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    "#);

    // Adding `extra-build-dependencies` should solve the issue
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = ["anyio", "scikit-learn"]
    "#})?;

    context.venv().arg("--clear").assert().success();
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_script() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Write a test package that arbitrarily requires `anyio` at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"
        [build-system]
        requires = ["hatchling"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(indoc! {r#"
        import sys
        from hatchling.build import *
        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)
    "#})?;
    child.child("src/child/__init__.py").touch()?;

    // Create a script that depends on the child package
    let script = context.temp_dir.child("script.py");
    script.write_str(indoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = ["child"]
        #
        # [tool.uv.sources]
        # child = { path = "child" }
        # ///
    "#})?;

    let filters = context
        .filters()
        .into_iter()
        .chain(vec![(
            r"environments-v2/script-[a-z0-9]+",
            "environments-v2/script-[HASH]",
        )])
        .collect::<Vec<_>>();

    // Running `uv sync` should fail due to missing build-dependencies
    uv_snapshot!(filters, context.sync().arg("--script").arg("script.py"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Creating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Missing `anyio` module

          hint: This usually indicates a problem with the package or the build environment.
    ");

    // Add extra build dependencies to the script
    script.write_str(indoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = ["child"]
        #
        # [tool.uv.sources]
        # child = { path = "child" }
        #
        # [tool.uv.extra-build-dependencies]
        # child = ["anyio"]
        # ///
    "#})?;

    // Running `uv sync` should now succeed due to extra build-dependencies
    context.venv().arg("--clear").assert().success();
    uv_snapshot!(filters, context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_script_sources() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();
    let anyio_local = context.workspace_root.join("test/packages/anyio_local");

    // Write a test package that arbitrarily requires `anyio` at a specific _path_ at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"
        [build-system]
        requires = ["hatchling"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(&formatdoc! {r#"
        import sys
        from hatchling.build import *
        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        # Check that we got the local version of anyio by checking for the marker
        if not hasattr(anyio, 'LOCAL_ANYIO_MARKER'):
            print("Found system anyio instead of local anyio", file=sys.stderr)
            sys.exit(1)
    "#})?;
    child.child("src/child/__init__.py").touch()?;

    // Create a script that depends on the child package
    let script = context.temp_dir.child("script.py");
    script.write_str(&formatdoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = ["child"]
        #
        # [tool.uv.sources]
        # anyio = {{ path = "{}" }}
        # child = {{ path = "child" }}
        #
        # [tool.uv.extra-build-dependencies]
        # child = ["anyio"]
        # ///
    "#, anyio_local.portable_display()
    })?;

    let filters = context
        .filters()
        .into_iter()
        .chain(vec![(
            r"environments-v2/script-[a-z0-9]+",
            "environments-v2/script-[HASH]",
        )])
        .collect::<Vec<_>>();

    // Running `uv sync` should succeed with the sources applied
    uv_snapshot!(filters, context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Creating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    Ok(())
}

#[test]
fn build_system_requires_workspace() -> Result<()> {
    let context = TestContext::new("3.12");

    let build = context.temp_dir.child("backend");
    build.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "backend"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions>=3.10"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    build
        .child("src")
        .child("backend")
        .child("__init__.py")
        .write_str(indoc! { r#"
            def hello() -> str:
                return "Hello, world!"
        "#})?;
    build.child("README.md").touch()?;

    let pyproject_toml = context.temp_dir.child("project").child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]

        [build-system]
        requires = ["setuptools>=42", "backend==0.1.0"]
        build-backend = "setuptools.build_meta"

        [tool.uv.workspace]
        members = ["../backend"]

        [tool.uv.sources]
        backend = { workspace = true }
        "#,
    )?;

    context
        .temp_dir
        .child("project")
        .child("setup.py")
        .write_str(indoc! {r"
        from setuptools import setup

        from backend import hello

        hello()

        setup()
        ",
        })?;

    uv_snapshot!(context.filters(), context.sync().current_dir(context.temp_dir.child("project")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 4 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + iniconfig==2.0.0
     + project==0.1.0 (from file://[TEMP_DIR]/project)
    ");

    Ok(())
}

#[test]
fn build_system_requires_path() -> Result<()> {
    let context = TestContext::new("3.12");

    let build = context.temp_dir.child("backend");
    build.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "backend"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions>=3.10"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    build
        .child("src")
        .child("backend")
        .child("__init__.py")
        .write_str(indoc! { r#"
            def hello() -> str:
                return "Hello, world!"
        "#})?;
    build.child("README.md").touch()?;

    let pyproject_toml = context.temp_dir.child("project").child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]

        [build-system]
        requires = ["setuptools>=42", "backend==0.1.0"]
        build-backend = "setuptools.build_meta"

        [tool.uv.sources]
        backend = { path = "../backend" }
        "#,
    )?;

    context
        .temp_dir
        .child("project")
        .child("setup.py")
        .write_str(indoc! {r"
        from setuptools import setup

        from backend import hello

        hello()

        setup()
        ",
        })?;

    uv_snapshot!(context.filters(), context.sync().current_dir(context.temp_dir.child("project")), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + iniconfig==2.0.0
     + project==0.1.0 (from file://[TEMP_DIR]/project)
    ");

    Ok(())
}

/// Sync a package with multiple wheels at the same version, differing only in the build tag. We
/// should choose the wheel with the highest build tag.
#[test]
fn sync_build_tag() -> Result<()> {
    let context = TestContext::new("3.12");

    // Populate the `--find-links` entries.
    fs_err::create_dir_all(context.temp_dir.join("links"))?;

    for entry in fs_err::read_dir(context.workspace_root.join("test/links"))? {
        let entry = entry?;
        let path = entry.path();
        if path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .is_some_and(|file_name| file_name.starts_with("build_tag-"))
        {
            let dest = context
                .temp_dir
                .join("links")
                .join(path.file_name().unwrap());
            fs_err::copy(&path, &dest)?;
        }
    }

    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(&formatdoc! { r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["build-tag"]

        [tool.uv]
        find-links = ["{}"]
        "#,
            context.temp_dir.join("links/").portable_display(),
        })?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "###);

    let lock = fs_err::read_to_string(context.temp_dir.child("uv.lock")).unwrap();

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
        name = "build-tag"
        version = "1.0.0"
        source = { registry = "links" }
        wheels = [
            { path = "build_tag-1.0.0-1-py2.py3-none-any.whl" },
            { path = "build_tag-1.0.0-3-py2.py3-none-any.whl" },
            { path = "build_tag-1.0.0-5-py2.py3-none-any.whl" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "build-tag" },
        ]

        [package.metadata]
        requires-dist = [{ name = "build-tag" }]
        "#
        );
    });

    // Re-run with `--locked`.
    uv_snapshot!(context.filters(), context.lock().arg("--locked"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "###);

    // Install from the lockfile.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + build-tag==1.0.0
    ");

    // Ensure that we choose the highest build tag (5).
    uv_snapshot!(context.filters(), context.run().arg("--no-sync").arg("python").arg("-c").arg("import build_tag; build_tag.main()"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    5

    ----- stderr -----
    "###);

    Ok(())
}

/// `uv sync` should respect build constraints. In this case, `json-merge-patch` should _not_ fail
/// to build, despite the fact that `setuptools==78.0.1` is the most recent version and _does_ fail
/// to build that package.
///
/// See: <https://github.com/astral-sh/uv/issues/12434>
#[test]
fn sync_build_constraints() -> Result<()> {
    let context = TestContext::new("3.12").with_exclude_newer("2025-03-24T19:00:00Z");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["json-merge-patch"]

        [tool.uv]
        build-constraint-dependencies = ["setuptools<78"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().arg("--no-binary-package").arg("json-merge-patch"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + json-merge-patch==0.2
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
            exclude-newer = "2025-03-24T19:00:00Z"

            [manifest]
            build-constraints = [{ name = "setuptools", specifier = "<78" }]

            [[package]]
            name = "json-merge-patch"
            version = "0.2"
            source = { registry = "https://pypi.org/simple" }
            sdist = { url = "https://files.pythonhosted.org/packages/39/62/3b783faabac9a099877397d8f7a7cc862a03fbf9fb1b90d414ea7c6bb096/json-merge-patch-0.2.tar.gz", hash = "sha256:09898b6d427c08754e2a97c709cf2dfd7e28bd10c5683a538914975eab778d39", size = 3081, upload-time = "2017-11-09T11:38:15.773Z" }

            [[package]]
            name = "project"
            version = "0.1.0"
            source = { virtual = "." }
            dependencies = [
                { name = "json-merge-patch" },
            ]

            [package.metadata]
            requires-dist = [{ name = "json-merge-patch" }]
            "#
            );
        }
    );

    fs_err::remove_dir_all(&context.cache_dir)?;
    fs_err::remove_dir_all(&context.venv)?;

    // We should also be able to read from the lockfile.
    uv_snapshot!(context.filters(), context.sync().arg("--locked"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + json-merge-patch==0.2
    ");

    // Modify the build constraints.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["json-merge-patch"]

        [tool.uv]
        build-constraint-dependencies = ["setuptools<77"]
        "#,
    )?;

    // This should fail, given that the build constraints have changed.
    uv_snapshot!(context.filters(), context.sync().arg("--locked"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
    ");

    // Changing the build constraints should lead to a re-resolve.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    Ok(())
}

/// Test that build dependencies respect locked versions from the lockfile.
#[test]
fn sync_build_dependencies_respect_locked_versions() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Write a test package that arbitrarily requires `anyio` at build time
    let child = context.temp_dir.child("child");
    child.create_dir_all()?;
    let child_pyproject_toml = child.child("pyproject.toml");
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling", "anyio"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;

    // Create a build backend that checks for a specific version of anyio
    let build_backend = child.child("build_backend.py");
    build_backend.write_str(indoc! {r#"
        import os
        import sys
        from hatchling.build import *

        expected_version = os.environ.get("EXPECTED_ANYIO_VERSION", "")
        if not expected_version:
            print("`EXPECTED_ANYIO_VERSION` not set", file=sys.stderr)
            sys.exit(1)

        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        from importlib.metadata import version
        anyio_version = version("anyio")

        if not anyio_version.startswith(expected_version):
            print(f"Expected `anyio` version {expected_version} but got {anyio_version}", file=sys.stderr)
            sys.exit(1)

        print(f"Found expected `anyio` version {anyio_version}", file=sys.stderr)
    "#})?;
    child.child("src/child/__init__.py").touch()?;

    // Create a project that will resolve to a non-latest version of `anyio`
    let parent = &context.temp_dir;
    let pyproject_toml = parent.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["anyio<4.1"]
    "#})?;

    uv_snapshot!(context.filters(), context.lock(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    ");

    // Now add the child dependency.
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["anyio<4.1", "child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    // Ensure our build backend is checking the version correctly
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::EXPECTED_ANYIO_VERSION, "3.0"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Expected `anyio` version 3.0 but got 4.3.0

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    // Now constrain the `anyio` build dependency to match the runtime
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["anyio<4.1", "child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = [{ requirement = "anyio", match-runtime = true }]
    "#})?;

    // The child should be built with anyio 4.0
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::EXPECTED_ANYIO_VERSION, "4.0"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + anyio==4.0.0
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Change the constraints on anyio
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["anyio<3.8", "child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = [{ requirement = "anyio", match-runtime = true }]
    "#})?;

    // The child should be rebuilt with anyio 3.7, without `--reinstall`
    uv_snapshot!(context.filters(), context.sync()
        .arg("--reinstall-package").arg("child").env(EnvVars::EXPECTED_ANYIO_VERSION, "4.0"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_wheel` failed (exit status: 1)

          [stderr]
          Expected `anyio` version 4.0 but got 3.7.1

          hint: This usually indicates a problem with the package or the build environment.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    uv_snapshot!(context.filters(), context.sync()
        .arg("--reinstall-package").arg("child").env(EnvVars::EXPECTED_ANYIO_VERSION, "3.7"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Uninstalled [N] packages in [TIME]
    Installed [N] packages in [TIME]
     - anyio==4.0.0
     + anyio==3.7.1
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // With preview enabled, there's no warning
    uv_snapshot!(context.filters(), context.sync()
        .arg("--preview-features").arg("extra-build-dependencies")
        .arg("--reinstall-package").arg("child")
        .env(EnvVars::EXPECTED_ANYIO_VERSION, "3.7"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Uninstalled [N] packages in [TIME]
    Installed [N] packages in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Now, we'll set a constraint in the parent project
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["anyio<3.8", "child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = [{ requirement = "anyio", match-runtime = true }]
    "#})?;

    // And an incompatible constraint in the child project
    child_pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling", "anyio>3.8,<4.2"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;

    // This should fail
    uv_snapshot!(context.filters(), context.sync()
        .arg("--reinstall-package").arg("child").env(EnvVars::EXPECTED_ANYIO_VERSION, "4.1"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
      × Failed to build `child @ file://[TEMP_DIR]/child`
      ├─▶ Failed to resolve requirements from `build-system.requires` and `extra-build-dependencies`
      ├─▶ No solution found when resolving: `hatchling`, `anyio>3.8, <4.2`, `anyio==3.7.1 (index: https://pypi.org/simple)`
      ╰─▶ Because you require anyio>3.8,<4.2 and anyio==3.7.1, we can conclude that your requirements are unsatisfiable.
      help: `child` was included because `parent` (v0.1.0) depends on `child`
    ");

    // Adding a version specifier should also fail
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["anyio<4.1", "child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = [{ requirement = "anyio>4", match-runtime = true }]
    "#})?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved [N] packages in [TIME]
    error: Dependencies marked with `match-runtime = true` cannot include version specifiers, but found: `anyio>4`
    ");

    Ok(())
}

#[test]
fn sync_extra_build_variables() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    // Create a build backend that asserts that `EXPECTED_ANYIO_VERSION` matches the installed version of `anyio`.
    let build_backend = context.temp_dir.child("build_backend.py");
    build_backend.write_str(indoc! {r#"
        import os
        import sys
        from hatchling.build import *

        expected_version = os.environ.get("EXPECTED_ANYIO_VERSION", "")
        if not expected_version:
            print("`EXPECTED_ANYIO_VERSION` not set", file=sys.stderr)
            sys.exit(1)

        try:
            import anyio
        except ModuleNotFoundError:
            print("Missing `anyio` module", file=sys.stderr)
            sys.exit(1)

        from importlib.metadata import version
        anyio_version = version("anyio")

        if not anyio_version.startswith(expected_version):
            print(f"Expected `anyio` version {expected_version} but got {anyio_version}", file=sys.stderr)
            sys.exit(1)

        print(f"Found expected `anyio` version {anyio_version}", file=sys.stderr)
    "#})?;

    // Create a project that will resolve to a non-latest version of `anyio`
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling", "anyio"]
        backend-path = ["."]
        build-backend = "build_backend"
    "#})?;
    context.temp_dir.child("src/parent/__init__.py").touch()?;

    uv_snapshot!(context.filters(), context.lock(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    ");

    // Ensure our build backend is checking the version correctly.
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::EXPECTED_ANYIO_VERSION, "3.0"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
      × Failed to build `parent @ file://[TEMP_DIR]/`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_editable` failed (exit status: 1)

          [stderr]
          Expected `anyio` version 3.0 but got 4.3.0

          hint: This usually indicates a problem with the package or the build environment.
    ");

    // Set the variable in TOML (to an incorrect value).
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling", "anyio"]
        backend-path = ["."]
        build-backend = "build_backend"

        [tool.uv.extra-build-variables]
        parent = { EXPECTED_ANYIO_VERSION = "3.0" }
    "#})?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
      × Failed to build `parent @ file://[TEMP_DIR]/`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `build_backend.build_editable` failed (exit status: 1)

          [stderr]
          Expected `anyio` version 3.0 but got 4.3.0

          hint: This usually indicates a problem with the package or the build environment.
    ");

    // Set the variable in TOML (to a correct value).
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling", "anyio"]
        backend-path = ["."]
        build-backend = "build_backend"

        [tool.uv.extra-build-variables]
        parent = { EXPECTED_ANYIO_VERSION = "4.3.0" }
    "#})?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + parent==0.1.0 (from file://[TEMP_DIR]/)
    ");

    Ok(())
}

#[test]
fn sync_extra_build_dependencies_cache() -> Result<()> {
    let context = TestContext::new("3.12");

    // Write a test package.
    context
        .temp_dir
        .child("child")
        .child("pyproject.toml")
        .write_str(indoc! {r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
    "#})?;
    context
        .temp_dir
        .child("child")
        .child("src/child/__init__.py")
        .touch()?;

    // Create a project that depends on the test package.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    // Running `uv sync` should build the child package.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Running `uv sync` again should be a no-op.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // Add a build dependency.
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = ["iniconfig"]
    "#})?;

    // Running `uv sync` should rebuild the child package with the new build dependency.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Running `uv sync` again should be a no-op.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    // Adding a version specifier is fine if match-runtime is false
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-dependencies]
        child = [{ requirement = "iniconfig>0", match-runtime = false }]
    "#})?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Remove the build dependency.
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }
    "#})?;

    // Running `uv sync` should reinstall the child package, but not rebuild it (since it's already
    // cached).
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Add a build variable.
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "parent"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "child" }

        [tool.uv.extra-build-variables]
        child = { INI_CONFIG_VERSION = "1.0.0" }
    "#})?;

    // Running `uv sync` should rebuild the child package with the new build dependency.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Running `uv sync` again should be a no-op.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    ");

    Ok(())
}
