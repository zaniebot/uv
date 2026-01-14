use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use indoc::indoc;
use insta::assert_snapshot;

use uv_static::EnvVars;

use crate::common::{TestContext, download_to_disk, uv_snapshot};

#[test]
fn sync() -> Result<()> {
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

    // Running `uv sync` should generate a lockfile.
    uv_snapshot!(context.filters(), context.sync(), @r"
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

    Ok(())
}

#[test]
fn locked() -> Result<()> {
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

    // Running with `--locked` should error, if no lockfile is present.
    uv_snapshot!(context.filters(), context.sync().arg("--locked"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Unable to find lockfile at `uv.lock`, but `--locked` was provided. To create a lockfile, run `uv lock` or `uv sync` without the flag.
    "###);

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

    // Running with `--locked` should error.
    uv_snapshot!(context.filters(), context.sync().arg("--locked"), @r###"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
    "###);

    let updated = context.read("uv.lock");

    // And the lockfile should be unchanged.
    assert_eq!(existing, updated);

    Ok(())
}

#[test]
fn frozen() -> Result<()> {
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

    // Running with `--frozen` should error, if no lockfile is present.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Unable to find lockfile at `uv.lock`, but `--frozen` was provided. To create a lockfile, run `uv lock` or `uv sync` without the flag.
    "###);

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

    // Running with `--frozen` should install the stale lockfile.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn empty() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r"
        [tool.uv.workspace]
        members = []
        ",
    )?;

    // Running `uv sync` should generate an empty lockfile.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
    Resolved in [TIME]
    Audited in [TIME]
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    // Running `uv sync` again should succeed.
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

/// Sync an individual package within a workspace.
#[test]
fn package() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "root"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child", "anyio>3"]

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
        dependencies = ["iniconfig>=1"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    let src = child.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    uv_snapshot!(context.filters(), context.sync().arg("--package").arg("child"), @r"
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

    Ok(())
}

/// Sync multiple packages within a workspace.
#[test]
fn multiple_packages() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "root"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["foo", "bar", "baz"]

        [tool.uv.sources]
        foo = { workspace = true }
        bar = { workspace = true }
        baz = { workspace = true }

        [tool.uv.workspace]
        members = ["packages/*"]
        "#,
    )?;

    context
        .temp_dir
        .child("packages")
        .child("foo")
        .child("pyproject.toml")
        .write_str(
            r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio"]
        "#,
        )?;

    context
        .temp_dir
        .child("packages")
        .child("bar")
        .child("pyproject.toml")
        .write_str(
            r#"
        [project]
        name = "bar"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]
        "#,
        )?;

    context
        .temp_dir
        .child("packages")
        .child("baz")
        .child("pyproject.toml")
        .write_str(
            r#"
        [project]
        name = "baz"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
        )?;

    // Sync `foo` and `bar`.
    uv_snapshot!(context.filters(), context.sync()
        .arg("--package").arg("foo")
        .arg("--package").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 9 packages in [TIME]
    Prepared 6 packages in [TIME]
    Installed 6 packages in [TIME]
     + anyio==4.3.0
     + bar==0.1.0 (from file://[TEMP_DIR]/packages/bar)
     + foo==0.1.0 (from file://[TEMP_DIR]/packages/foo)
     + idna==3.6
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // Sync `foo`, `bar`, and `baz`.
    uv_snapshot!(context.filters(), context.sync()
        .arg("--package").arg("foo")
        .arg("--package").arg("bar")
        .arg("--package").arg("baz"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 9 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + baz==0.1.0 (from file://[TEMP_DIR]/packages/baz)
     + iniconfig==2.0.0
    ");

    Ok(())
}

/// Test json output
#[test]
fn sync_json() -> Result<()> {
    let context = TestContext::new("3.12")
        .with_filtered_python_names()
        .with_filtered_virtualenv_bin();

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

    uv_snapshot!(context.filters(), context.sync()
        .arg("--output-format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "project",
      "project": {
        "path": "[TEMP_DIR]/",
        "workspace": {
          "path": "[TEMP_DIR]/"
        }
      },
      "sync": {
        "environment": {
          "path": "[VENV]/",
          "python": {
            "path": "[VENV]/[BIN]/[PYTHON]",
            "version": "3.12.[X]",
            "implementation": "cpython"
          }
        },
        "action": "check",
        "changes": [
          {
            "name": "iniconfig",
            "version": "2.0.0",
            "action": "installed"
          }
        ]
      },
      "lock": {
        "path": "[TEMP_DIR]/uv.lock",
        "action": "create"
      },
      "dry_run": false
    }

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    "#);

    assert!(context.temp_dir.child("uv.lock").exists());

    uv_snapshot!(context.filters(), context.sync()
        .arg("--frozen")
        .arg("--output-format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "project",
      "project": {
        "path": "[TEMP_DIR]/",
        "workspace": {
          "path": "[TEMP_DIR]/"
        }
      },
      "sync": {
        "environment": {
          "path": "[VENV]/",
          "python": {
            "path": "[VENV]/[BIN]/[PYTHON]",
            "version": "3.12.[X]",
            "implementation": "cpython"
          }
        },
        "action": "check",
        "changes": []
      },
      "lock": {
        "path": "[TEMP_DIR]/uv.lock",
        "action": "use"
      },
      "dry_run": false
    }

    ----- stderr -----
    Audited 1 package in [TIME]
    "#);

    uv_snapshot!(context.filters(), context.sync()
        .arg("--locked")
        .arg("--output-format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "project",
      "project": {
        "path": "[TEMP_DIR]/",
        "workspace": {
          "path": "[TEMP_DIR]/"
        }
      },
      "sync": {
        "environment": {
          "path": "[VENV]/",
          "python": {
            "path": "[VENV]/[BIN]/[PYTHON]",
            "version": "3.12.[X]",
            "implementation": "cpython"
          }
        },
        "action": "check",
        "changes": []
      },
      "lock": {
        "path": "[TEMP_DIR]/uv.lock",
        "action": "check"
      },
      "dry_run": false
    }

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 1 package in [TIME]
    "#);

    // Invalidate the lockfile by changing the requirements.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig<2"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync()
        .arg("--locked")
        .arg("--output-format").arg("json"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
    ");

    // Test that JSON output is shown even with --quiet flag
    uv_snapshot!(context.filters(), context.sync()
        .arg("--quiet")
        .arg("--frozen")
        .arg("--output-format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "project",
      "project": {
        "path": "[TEMP_DIR]/",
        "workspace": {
          "path": "[TEMP_DIR]/"
        }
      },
      "sync": {
        "environment": {
          "path": "[VENV]/",
          "python": {
            "path": "[VENV]/[BIN]/[PYTHON]",
            "version": "3.12.[X]",
            "implementation": "cpython"
          }
        },
        "action": "check",
        "changes": []
      },
      "lock": {
        "path": "[TEMP_DIR]/uv.lock",
        "action": "use"
      },
      "dry_run": false
    }

    ----- stderr -----
    "#);

    Ok(())
}

/// Test --dry json output
#[test]
fn sync_dry_json() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.12"])
        .with_filtered_python_names()
        .with_filtered_virtualenv_bin();

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

    // Running `uv sync` should report intent to create the environment and lockfile
    uv_snapshot!(context.filters(), context.sync()
        .arg("--output-format").arg("json")
        .arg("--dry-run"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "target": "project",
      "project": {
        "path": "[TEMP_DIR]/",
        "workspace": {
          "path": "[TEMP_DIR]/"
        }
      },
      "sync": {
        "environment": {
          "path": "[VENV]/",
          "python": {
            "path": "[VENV]/[BIN]/[PYTHON]",
            "version": "3.12.[X]",
            "implementation": "cpython"
          }
        },
        "action": "create",
        "changes": [
          {
            "name": "iniconfig",
            "version": "2.0.0",
            "action": "installed"
          }
        ]
      },
      "lock": {
        "path": "[TEMP_DIR]/uv.lock",
        "action": "create"
      },
      "dry_run": true
    }

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Resolved 2 packages in [TIME]
    Would download 1 package
    Would install 1 package
     + iniconfig==2.0.0
    "#);

    Ok(())
}

/// Ensure that we use the maximum Python version when a workspace contains mixed requirements.
#[test]
fn mixed_requires_python() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.9", "3.12"]);

    // Create a workspace root with a minimum Python requirement of Python 3.12.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "albatross"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["bird-feeder", "anyio>3"]

        [tool.uv.sources]
        bird-feeder = { workspace = true }

        [tool.uv.workspace]
        members = ["packages/*"]
        "#,
    )?;

    let src = context.temp_dir.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    // Create a child with a minimum Python requirement of Python 3.9.
    let child = context.temp_dir.child("packages").child("bird-feeder");
    child.create_dir_all()?;

    let src = context.temp_dir.child("src").child("bird_feeder");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    let pyproject_toml = child.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "bird-feeder"
        version = "0.1.0"
        requires-python = ">=3.9"

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    // Running `uv sync` should succeed, locking for Python 3.12.
    uv_snapshot!(context.filters(), context.sync().arg("-p").arg("3.12"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 5 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + bird-feeder==0.1.0 (from file://[TEMP_DIR]/packages/bird-feeder)
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Running `uv sync` again should fail.
    uv_snapshot!(context.filters(), context.sync().arg("-p").arg("3.9"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
    error: The requested interpreter resolved to Python 3.9.[X], which is incompatible with the project's Python requirement: `>=3.12` (from workspace member `albatross`'s `project.requires-python`).
    ");

    Ok(())
}

/// Ensure that group requires-python solves an actual problem
#[test]
#[cfg(not(windows))]
#[cfg(feature = "python-eol")]
fn group_requires_python_useful_defaults() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.8", "3.9"]);

    // Require 3.8 for our project, but have a dev-dependency on a version of sphinx that needs 3.9
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "pharaohs-tomp"
        version = "0.1.0"
        requires-python = ">=3.8"
        dependencies = ["anyio"]

        [dependency-groups]
        dev = ["sphinx>=7.2.6"]
        "#,
    )?;

    let src = context.temp_dir.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    // Running `uv sync --no-dev` should ideally succeed, locking for Python 3.8.
    // ...but once we pick the 3.8 interpreter the lock freaks out because it sees
    // that the dependency-group containing sphinx will never successfully install,
    // even though it's not enabled!
    uv_snapshot!(context.filters(), context.sync()
        .arg("--no-dev"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.8.[X] interpreter at: [PYTHON-3.8]
    Creating virtual environment at: .venv
      × No solution found when resolving dependencies for split (markers: python_full_version == '3.8.*'):
      ╰─▶ Because the requested Python version (>=3.8) does not satisfy Python>=3.9 and sphinx==7.2.6 depends on Python>=3.9, we can conclude that sphinx==7.2.6 cannot be used.
          And because only sphinx<=7.2.6 is available, we can conclude that sphinx>=7.2.6 cannot be used.
          And because pharaohs-tomp:dev depends on sphinx>=7.2.6 and your project requires pharaohs-tomp:dev, we can conclude that your project's requirements are unsatisfiable.

          hint: The `requires-python` value (>=3.8) includes Python versions that are not supported by your dependencies (e.g., sphinx==7.2.6 only supports >=3.9). Consider using a more restrictive `requires-python` value (like >=3.9).
    ");

    // Running `uv sync` should always fail, as now sphinx is involved
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving dependencies for split (markers: python_full_version == '3.8.*'):
      ╰─▶ Because the requested Python version (>=3.8) does not satisfy Python>=3.9 and sphinx==7.2.6 depends on Python>=3.9, we can conclude that sphinx==7.2.6 cannot be used.
          And because only sphinx<=7.2.6 is available, we can conclude that sphinx>=7.2.6 cannot be used.
          And because pharaohs-tomp:dev depends on sphinx>=7.2.6 and your project requires pharaohs-tomp:dev, we can conclude that your project's requirements are unsatisfiable.

          hint: The `requires-python` value (>=3.8) includes Python versions that are not supported by your dependencies (e.g., sphinx==7.2.6 only supports >=3.9). Consider using a more restrictive `requires-python` value (like >=3.9).
    ");

    // Adding group requires python should fix it
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "pharaohs-tomp"
        version = "0.1.0"
        requires-python = ">=3.8"
        dependencies = ["anyio"]

        [dependency-groups]
        dev = ["sphinx>=7.2.6"]

        [tool.uv.dependency-groups]
        dev = {requires-python = ">=3.9"}
        "#,
    )?;

    // Running `uv sync --no-dev` should succeed, still using the Python 3.8.
    uv_snapshot!(context.filters(), context.sync()
        .arg("--no-dev"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 29 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + anyio==4.3.0
     + exceptiongroup==1.2.0
     + idna==3.6
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // Running `uv sync` should succeed, bumping to Python 3.9 as sphinx is now involved.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 29 packages in [TIME]
    Prepared 22 packages in [TIME]
    Installed 27 packages in [TIME]
     + alabaster==0.7.16
     + anyio==4.3.0
     + babel==2.14.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + docutils==0.20.1
     + exceptiongroup==1.2.0
     + idna==3.6
     + imagesize==1.4.1
     + importlib-metadata==7.1.0
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + packaging==24.0
     + pygments==2.17.2
     + requests==2.31.0
     + sniffio==1.3.1
     + snowballstemmer==2.2.0
     + sphinx==7.2.6
     + sphinxcontrib-applehelp==1.0.8
     + sphinxcontrib-devhelp==1.0.6
     + sphinxcontrib-htmlhelp==2.0.5
     + sphinxcontrib-jsmath==1.0.1
     + sphinxcontrib-qthelp==1.0.7
     + sphinxcontrib-serializinghtml==1.1.10
     + typing-extensions==4.10.0
     + urllib3==2.2.1
     + zipp==3.18.1
    ");

    Ok(())
}

/// Ensure that group requires-python solves an actual problem
#[test]
#[cfg(not(windows))]
#[cfg(feature = "python-eol")]
fn group_requires_python_useful_non_defaults() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.8", "3.9"]);

    // Require 3.8 for our project, but have a dev-dependency on a version of sphinx that needs 3.9
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "pharaohs-tomp"
        version = "0.1.0"
        requires-python = ">=3.8"
        dependencies = ["anyio"]

        [dependency-groups]
        mygroup = ["sphinx>=7.2.6"]
        "#,
    )?;

    let src = context.temp_dir.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    // Running `uv sync` should ideally succeed, locking for Python 3.8.
    // ...but once we pick the 3.8 interpreter the lock freaks out because it sees
    // that the dependency-group containing sphinx will never successfully install,
    // even though it's not enabled, or even a default!
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.8.[X] interpreter at: [PYTHON-3.8]
    Creating virtual environment at: .venv
      × No solution found when resolving dependencies for split (markers: python_full_version == '3.8.*'):
      ╰─▶ Because the requested Python version (>=3.8) does not satisfy Python>=3.9 and sphinx==7.2.6 depends on Python>=3.9, we can conclude that sphinx==7.2.6 cannot be used.
          And because only sphinx<=7.2.6 is available, we can conclude that sphinx>=7.2.6 cannot be used.
          And because pharaohs-tomp:mygroup depends on sphinx>=7.2.6 and your project requires pharaohs-tomp:mygroup, we can conclude that your project's requirements are unsatisfiable.

          hint: The `requires-python` value (>=3.8) includes Python versions that are not supported by your dependencies (e.g., sphinx==7.2.6 only supports >=3.9). Consider using a more restrictive `requires-python` value (like >=3.9).
    ");

    // Running `uv sync --group mygroup` should definitely fail, as now sphinx is involved
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("mygroup"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving dependencies for split (markers: python_full_version == '3.8.*'):
      ╰─▶ Because the requested Python version (>=3.8) does not satisfy Python>=3.9 and sphinx==7.2.6 depends on Python>=3.9, we can conclude that sphinx==7.2.6 cannot be used.
          And because only sphinx<=7.2.6 is available, we can conclude that sphinx>=7.2.6 cannot be used.
          And because pharaohs-tomp:mygroup depends on sphinx>=7.2.6 and your project requires pharaohs-tomp:mygroup, we can conclude that your project's requirements are unsatisfiable.

          hint: The `requires-python` value (>=3.8) includes Python versions that are not supported by your dependencies (e.g., sphinx==7.2.6 only supports >=3.9). Consider using a more restrictive `requires-python` value (like >=3.9).
    ");

    // Adding group requires python should fix it
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "pharaohs-tomp"
        version = "0.1.0"
        requires-python = ">=3.8"
        dependencies = ["anyio"]

        [dependency-groups]
        mygroup = ["sphinx>=7.2.6"]

        [tool.uv.dependency-groups]
        mygroup = {requires-python = ">=3.9"}
        "#,
    )?;

    // Running `uv sync` should succeed, locking for the previous picked Python 3.8.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 29 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + anyio==4.3.0
     + exceptiongroup==1.2.0
     + idna==3.6
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // Running `uv sync --group mygroup` should pass, bumping the interpreter to 3.9,
    // as the group requires-python saves us
    uv_snapshot!(context.filters(), context.sync()
        .arg("--group").arg("mygroup"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 29 packages in [TIME]
    Prepared 22 packages in [TIME]
    Installed 27 packages in [TIME]
     + alabaster==0.7.16
     + anyio==4.3.0
     + babel==2.14.0
     + certifi==2024.2.2
     + charset-normalizer==3.3.2
     + docutils==0.20.1
     + exceptiongroup==1.2.0
     + idna==3.6
     + imagesize==1.4.1
     + importlib-metadata==7.1.0
     + jinja2==3.1.3
     + markupsafe==2.1.5
     + packaging==24.0
     + pygments==2.17.2
     + requests==2.31.0
     + sniffio==1.3.1
     + snowballstemmer==2.2.0
     + sphinx==7.2.6
     + sphinxcontrib-applehelp==1.0.8
     + sphinxcontrib-devhelp==1.0.6
     + sphinxcontrib-htmlhelp==2.0.5
     + sphinxcontrib-jsmath==1.0.1
     + sphinxcontrib-qthelp==1.0.7
     + sphinxcontrib-serializinghtml==1.1.10
     + typing-extensions==4.10.0
     + urllib3==2.2.1
     + zipp==3.18.1
    ");

    Ok(())
}

#[test]
fn check() -> Result<()> {
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

    // Running `uv sync --check` should fail.
    uv_snapshot!(context.filters(), context.sync().arg("--check"), @r###"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Would use project environment at: .venv
    Resolved 2 packages in [TIME]
    Would create lockfile at: uv.lock
    Would download 1 package
    Would install 1 package
     + iniconfig==2.0.0
    The environment is outdated; run `uv sync` to update the environment
    "###);

    // Sync the environment.
    uv_snapshot!(context.filters(), context.sync(), @r"
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

    // Running `uv sync --check` should pass now that the environment is up to date.
    uv_snapshot!(context.filters(), context.sync().arg("--check"), @r"
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

/// Avoid using incompatible versions for build dependencies that are also part of the resolved
/// environment. This is a very subtle issue, but: when locking, we don't enforce platform
/// compatibility. So, if we reuse the resolver state to install, and the install itself has to
/// perform a resolution (e.g., for the build dependencies of a source distribution), that
/// resolution may choose incompatible versions.
///
/// The key property here is that there's a shared package between the build dependencies and the
/// project dependencies.
#[test]
fn sync_reset_state() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["pydantic-core"]

        [build-system]
        requires = ["setuptools", "pydantic-core"]
        build-backend = "setuptools.build_meta:__legacy__"
        "#,
    )?;

    let setup_py = context.temp_dir.child("setup.py");
    setup_py.write_str(indoc::indoc! { r#"
        from setuptools import setup
        import pydantic_core

        setup(
            name="project",
            version="0.1.0",
            packages=["project"],
            install_requires=["pydantic-core"],
        )
    "# })?;

    let src = context.temp_dir.child("project");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    // Running `uv sync` should succeed.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + project==0.1.0 (from file://[TEMP_DIR]/)
     + pydantic-core==2.17.0
     + typing-extensions==4.10.0
    ");

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

/// Test that relative wheel paths are correctly preserved.
#[test]
fn sync_relative_wheel() -> Result<()> {
    let context = TestContext::new("3.12");

    let requirements = r#"[project]
    name = "relative_wheel"
    version = "0.1.0"
    requires-python = ">=3.12"
    dependencies = ["ok"]

    [tool.uv.sources]
    ok = { path = "wheels/ok-1.0.0-py3-none-any.whl" }

    [build-system]
    requires = ["hatchling"]
    build-backend = "hatchling.build"
    "#;

    context
        .temp_dir
        .child("src/relative_wheel/__init__.py")
        .touch()?;

    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(requirements)?;

    context.temp_dir.child("wheels").create_dir_all()?;
    fs_err::copy(
        "../../test/links/ok-1.0.0-py3-none-any.whl",
        context.temp_dir.join("wheels/ok-1.0.0-py3-none-any.whl"),
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 2 packages in [TIME]
     + ok==1.0.0 (from file://[TEMP_DIR]/wheels/ok-1.0.0-py3-none-any.whl)
     + relative-wheel==0.1.0 (from file://[TEMP_DIR]/)
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
            name = "ok"
            version = "1.0.0"
            source = { path = "wheels/ok-1.0.0-py3-none-any.whl" }
            wheels = [
                { filename = "ok-1.0.0-py3-none-any.whl", hash = "sha256:79f0b33e6ce1e09eaa1784c8eee275dfe84d215d9c65c652f07c18e85fdaac5f" },
            ]

            [[package]]
            name = "relative-wheel"
            version = "0.1.0"
            source = { editable = "." }
            dependencies = [
                { name = "ok" },
            ]

            [package.metadata]
            requires-dist = [{ name = "ok", path = "wheels/ok-1.0.0-py3-none-any.whl" }]
            "#
            );
        }
    );

    // Check that we can re-read the lockfile.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Audited 2 packages in [TIME]
    ");

    Ok(())
}

/// Syncing against an unstable environment should fail (but locking should succeed).
#[test]
fn sync_environment() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.10"
        dependencies = ["iniconfig"]

        [tool.uv]
        environments = ["python_version < '3.11'"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    error: The current Python platform is not compatible with the lockfile's supported environments: `python_full_version < '3.11'`
    "###);

    assert!(context.temp_dir.child("uv.lock").exists());

    Ok(())
}

#[test]
fn sync_workspace_members_with_transitive_dependencies() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [tool.uv.workspace]
        members = [
            "packages/*",
        ]
        "#,
    )?;

    let packages = context.temp_dir.child("packages");
    packages.create_dir_all()?;

    // Create three workspace members with transitive dependency from
    // pkg-c -> pkg-b -> pkg-a
    let pkg_a = packages.child("pkg-a");
    pkg_a.create_dir_all()?;
    let pkg_a_pyproject_toml = pkg_a.child("pyproject.toml");
    pkg_a_pyproject_toml.write_str(
        r#"
        [project]
        name = "pkg-a"
        version = "0.0.1"
        requires-python = ">=3.12"
        dependencies = ["anyio"]
        "#,
    )?;

    let pkg_b = packages.child("pkg-b");
    pkg_b.create_dir_all()?;
    let pkg_b_pyproject_toml = pkg_b.child("pyproject.toml");
    pkg_b_pyproject_toml.write_str(
        r#"
        [project]
        name = "pkg-b"
        version = "0.0.1"
        requires-python = ">=3.12"
        dependencies = ["pkg-a"]

        [tool.uv.sources]
        pkg-a = { workspace = true }
        "#,
    )?;

    let pkg_c = packages.child("pkg-c");
    pkg_c.create_dir_all()?;
    let pkg_c_pyproject_toml = pkg_c.child("pyproject.toml");
    pkg_c_pyproject_toml.write_str(
        r#"
        [project]
        name = "pkg-c"
        version = "0.0.1"
        requires-python = ">=3.12"
        dependencies = ["pkg-b"]

        [tool.uv.sources]
        pkg-b = { workspace = true }
        "#,
    )?;

    // Syncing should build the two transitive dependencies pkg-a and pkg-b,
    // but not pkg-c, which is not a dependency.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 5 packages in [TIME]
    Installed 5 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + pkg-a==0.0.1 (from file://[TEMP_DIR]/packages/pkg-a)
     + pkg-b==0.0.1 (from file://[TEMP_DIR]/packages/pkg-b)
     + sniffio==1.3.1
    ");

    // The lockfile should be valid.
    uv_snapshot!(context.filters(), context.lock().arg("--check"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    ");

    Ok(())
}

/// Regression test for <https://github.com/astral-sh/uv/issues/6316>.
///
/// Previously, we would read metadata statically from pyproject.toml and write that to `uv.lock`. In
/// this sync pass, we had also built the project with setuptools, which sorts specifiers by python
/// string sort through packaging. On the second run, we read the cache that now has the setuptools
/// sorting, changing the lockfile.
#[test]
fn read_metadata_statically_over_the_cache() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        # Python string sorting is the other way round.
        dependencies = ["anyio>=4,<5"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    context.sync().assert().success();
    let lock1 = context.read("uv.lock");
    // Assert we're reading static metadata.
    assert!(lock1.contains(">=4,<5"));
    assert!(!lock1.contains("<5,>=4"));
    context.sync().assert().success();
    let lock2 = context.read("uv.lock");
    // Assert stability.
    assert_eq!(lock1, lock2);

    Ok(())
}

/// Convert from a virtual project to a package.
#[test]
fn convert_to_package() -> Result<()> {
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

    // Running `uv sync` should not install the project itself.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
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

    // Add the build system.
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
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
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

    Ok(())
}

#[test]
fn sync_update_project() -> Result<()> {
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

    // Bump the project version.
    pyproject_toml.write_str(
        r#"
        [project]
        name = "my-project"
        version = "0.2.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + my-project==0.2.0 (from file://[TEMP_DIR]/)
    ");

    Ok(())
}

/// Avoid validating workspace members when `--no-sources` is provided. Rather than reporting that
/// `./anyio` is missing, install `anyio` from the registry.
#[test]
fn sync_no_sources_missing_member() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "root"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio"]

        [tool.uv.sources]
        anyio = { workspace = true }

        [tool.uv.workspace]
        members = ["anyio"]
        "#,
    )?;

    let src = context.temp_dir.child("src").child("albatross");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    uv_snapshot!(context.filters(), context.sync().arg("--no-sources"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn sync_python_version() -> Result<()> {
    let context: TestContext = TestContext::new_with_versions(&["3.10", "3.11", "3.12"]);

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.11"
        dependencies = ["anyio==3.7.0"]
    "#})?;

    // We should respect the project's required version, not the first on the path
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    Creating virtual environment at: .venv
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Unless explicitly requested...
    uv_snapshot!(context.filters(), context.sync().arg("--python").arg("3.10"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.10.[X] interpreter at: [PYTHON-3.10]
    error: The requested interpreter resolved to Python 3.10.[X], which is incompatible with the project's Python requirement: `>=3.11` (from `project.requires-python`)
    ");

    // But a pin should take precedence
    uv_snapshot!(context.filters(), context.python_pin().arg("3.12"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Pinned `.python-version` to `3.12`

    ----- stderr -----
    "###);

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 4 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Create a pin that's incompatible with the project
    uv_snapshot!(context.filters(), context.python_pin().arg("3.10").arg("--no-workspace"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Updated `.python-version` from `3.12` -> `3.10`

    ----- stderr -----
    "###);

    // We should warn on subsequent uses, but respect the pinned version?
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.10.[X] interpreter at: [PYTHON-3.10]
    error: The Python request from `.python-version` resolved to Python 3.10.[X], which is incompatible with the project's Python requirement: `>=3.11` (from `project.requires-python`)
    Use `uv python pin` to update the `.python-version` file to a compatible version
    ");

    // Unless the pin file is outside the project, in which case we should just ignore it entirely
    let child_dir = context.temp_dir.child("child");
    child_dir.create_dir_all().unwrap();

    let pyproject_toml = child_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc::indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.11"
        dependencies = ["anyio==3.7.0"]
    "#})
        .unwrap();

    uv_snapshot!(context.filters(), context.sync().current_dir(&child_dir), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    Creating virtual environment at: .venv
    Resolved 4 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn sync_explicit() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "root"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "idna>2",
        ]

        [[tool.uv.index]]
        name = "test"
        url = "https://test.pypi.org/simple"
        explicit = true

        [tool.uv.sources]
        idna = { index = "test" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + idna==2.7
    ");

    // Clear the environment.
    fs_err::remove_dir_all(&context.venv)?;

    // The package should be drawn from the cache.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + idna==2.7
    ");

    Ok(())
}

/// Sync all members in a workspace.
#[test]
fn sync_all() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio>3", "child"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"

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

    // Sync all workspace members.
    uv_snapshot!(context.filters(), context.sync().arg("--all-packages"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    Prepared 6 packages in [TIME]
    Installed 6 packages in [TIME]
     + anyio==4.3.0
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + idna==3.6
     + iniconfig==2.0.0
     + project==0.1.0 (from file://[TEMP_DIR]/)
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn sync_multiple_sources_index_disjoint_extras() -> Result<()> {
    let context = TestContext::new("3.12").with_exclude_newer("2025-01-30T00:00Z");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [project.optional-dependencies]
        cu118 = ["jinja2==3.1.2"]
        cu124 = ["jinja2==3.1.3"]

        [tool.uv]
        constraint-dependencies = ["markupsafe<3"]
        conflicts = [
            [
                { extra = "cu118" },
                { extra = "cu124" },
            ],
        ]

        [tool.uv.sources]
        jinja2 = [
            { index = "torch-cu118", extra = "cu118" },
            { index = "torch-cu124", extra = "cu124" },
        ]

        [[tool.uv.index]]
        name = "torch-cu118"
        url = "https://astral-sh.github.io/pytorch-mirror/whl/cu118"
        explicit = true

        [[tool.uv.index]]
        name = "torch-cu124"
        url = "https://astral-sh.github.io/pytorch-mirror/whl/cu124"
        explicit = true
        "#,
    )?;

    // Generate a lockfile.
    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync().arg("--extra").arg("cu124"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + jinja2==3.1.3
     + markupsafe==2.1.5
    ");

    Ok(())
}

#[test]
fn sync_derivation_chain() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["wsgiref"]

        [[tool.uv.dependency-metadata]]
        name = "wsgiref"
        version = "0.1.2"
        requires-dist = []
        "#,
    )?;

    let filters = context
        .filters()
        .into_iter()
        .chain([(r"/.*/src", "/[TMP]/src")])
        .collect::<Vec<_>>();

    uv_snapshot!(filters, context.sync(), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to build `wsgiref==0.1.2`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `setuptools.build_meta:__legacy__.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 14, in <module>
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 325, in get_requires_for_build_wheel
              return self._get_build_requires(config_settings, requirements=['wheel'])
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 295, in _get_build_requires
              self.run_setup()
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 487, in run_setup
              super().run_setup(setup_script=setup_script)
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 311, in run_setup
              exec(code, locals())
            File "<string>", line 5, in <module>
            File "[CACHE_DIR]/[TMP]/src/ez_setup/__init__.py", line 170
              print "Setuptools version",version,"or greater has been installed."
              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          SyntaxError: Missing parentheses in call to 'print'. Did you mean print(...)?

          hint: This usually indicates a problem with the package or the build environment.
      help: `wsgiref` (v0.1.2) was included because `project` (v0.1.0) depends on `wsgiref`
    "#);

    Ok(())
}

#[test]
fn sync_derivation_chain_extra() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []
        optional-dependencies = { wsgi = ["wsgiref"] }

        [[tool.uv.dependency-metadata]]
        name = "wsgiref"
        version = "0.1.2"
        requires-dist = []
        "#,
    )?;

    let filters = context
        .filters()
        .into_iter()
        .chain([(r"/.*/src", "/[TMP]/src")])
        .collect::<Vec<_>>();

    uv_snapshot!(filters, context.sync().arg("--extra").arg("wsgi"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to build `wsgiref==0.1.2`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `setuptools.build_meta:__legacy__.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 14, in <module>
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 325, in get_requires_for_build_wheel
              return self._get_build_requires(config_settings, requirements=['wheel'])
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 295, in _get_build_requires
              self.run_setup()
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 487, in run_setup
              super().run_setup(setup_script=setup_script)
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 311, in run_setup
              exec(code, locals())
            File "<string>", line 5, in <module>
            File "[CACHE_DIR]/[TMP]/src/ez_setup/__init__.py", line 170
              print "Setuptools version",version,"or greater has been installed."
              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          SyntaxError: Missing parentheses in call to 'print'. Did you mean print(...)?

          hint: This usually indicates a problem with the package or the build environment.
      help: `wsgiref` (v0.1.2) was included because `project[wsgi]` (v0.1.0) depends on `wsgiref`
    "#);

    Ok(())
}

#[test]
fn sync_derivation_chain_group() -> Result<()> {
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
        wsgi = ["wsgiref"]

        [[tool.uv.dependency-metadata]]
        name = "wsgiref"
        version = "0.1.2"
        requires-dist = []
        "#,
    )?;

    let filters = context
        .filters()
        .into_iter()
        .chain([(r"/.*/src", "/[TMP]/src")])
        .collect::<Vec<_>>();

    uv_snapshot!(filters, context.sync().arg("--group").arg("wsgi"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to build `wsgiref==0.1.2`
      ├─▶ The build backend returned an error
      ╰─▶ Call to `setuptools.build_meta:__legacy__.build_wheel` failed (exit status: 1)

          [stderr]
          Traceback (most recent call last):
            File "<string>", line 14, in <module>
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 325, in get_requires_for_build_wheel
              return self._get_build_requires(config_settings, requirements=['wheel'])
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 295, in _get_build_requires
              self.run_setup()
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 487, in run_setup
              super().run_setup(setup_script=setup_script)
            File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 311, in run_setup
              exec(code, locals())
            File "<string>", line 5, in <module>
            File "[CACHE_DIR]/[TMP]/src/ez_setup/__init__.py", line 170
              print "Setuptools version",version,"or greater has been installed."
              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          SyntaxError: Missing parentheses in call to 'print'. Did you mean print(...)?

          hint: This usually indicates a problem with the package or the build environment.
      help: `wsgiref` (v0.1.2) was included because `project:wsgi` (v0.1.0) depends on `wsgiref`
    "#);

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/9743>
#[test]
#[cfg(all(feature = "slow-tests", feature = "git"))]
fn sync_stale_egg_info() -> Result<()> {
    let context = TestContext::new("3.13");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = [
            "member @ git+https://github.com/astral-sh/uv-stale-egg-info-test.git#subdirectory=member",
            "root @ git+https://github.com/astral-sh/uv-stale-egg-info-test.git",
        ]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "###);

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
            requires-python = ">=3.13"

            [options]
            exclude-newer = "2024-03-25T00:00:00Z"

            [[package]]
            name = "foo"
            version = "0.1.0"
            source = { virtual = "." }
            dependencies = [
                { name = "member" },
                { name = "root" },
            ]

            [package.metadata]
            requires-dist = [
                { name = "member", git = "https://github.com/astral-sh/uv-stale-egg-info-test.git?subdirectory=member" },
                { name = "root", git = "https://github.com/astral-sh/uv-stale-egg-info-test.git" },
            ]

            [[package]]
            name = "member"
            version = "0.1.dev5+gfea1041"
            source = { git = "https://github.com/astral-sh/uv-stale-egg-info-test.git?subdirectory=member#fea10416b9c479ac88fb217e14e40249b63bfbee" }
            dependencies = [
                { name = "setuptools" },
            ]

            [[package]]
            name = "root"
            version = "0.1.dev5+gfea1041"
            source = { git = "https://github.com/astral-sh/uv-stale-egg-info-test.git#fea10416b9c479ac88fb217e14e40249b63bfbee" }
            dependencies = [
                { name = "member" },
            ]

            [[package]]
            name = "setuptools"
            version = "69.2.0"
            source = { registry = "https://pypi.org/simple" }
            sdist = { url = "https://files.pythonhosted.org/packages/4d/5b/dc575711b6b8f2f866131a40d053e30e962e633b332acf7cd2c24843d83d/setuptools-69.2.0.tar.gz", hash = "sha256:0ff4183f8f42cd8fa3acea16c45205521a4ef28f73c6391d8a25e92893134f2e", size = 2222950, upload-time = "2024-03-13T11:20:59.219Z" }
            wheels = [
                { url = "https://files.pythonhosted.org/packages/92/e1/1c8bb3420105e70bdf357d57dd5567202b4ef8d27f810e98bb962d950834/setuptools-69.2.0-py3-none-any.whl", hash = "sha256:c21c49fb1042386df081cb5d86759792ab89efca84cf114889191cd09aacc80c", size = 821485, upload-time = "2024-03-13T11:20:54.103Z" },
            ]
            "#
            );
        }
    );

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + member==0.1.dev5+gfea1041 (from git+https://github.com/astral-sh/uv-stale-egg-info-test.git@fea10416b9c479ac88fb217e14e40249b63bfbee#subdirectory=member)
     + root==0.1.dev5+gfea1041 (from git+https://github.com/astral-sh/uv-stale-egg-info-test.git@fea10416b9c479ac88fb217e14e40249b63bfbee)
     + setuptools==69.2.0
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/8887>
#[test]
#[cfg(feature = "git")]
fn sync_git_repeated_member_static_metadata() -> Result<()> {
    let context = TestContext::new("3.13");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["uv-git-workspace-in-root", "workspace-member-in-subdir"]

        [tool.uv.sources]
        uv-git-workspace-in-root = { git = "https://github.com/astral-sh/workspace-in-root-test.git" }
        workspace-member-in-subdir = { git = "https://github.com/astral-sh/workspace-in-root-test.git", subdirectory = "workspace-member-in-subdir" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "###);

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
            requires-python = ">=3.13"

            [options]
            exclude-newer = "2024-03-25T00:00:00Z"

            [[package]]
            name = "foo"
            version = "0.1.0"
            source = { virtual = "." }
            dependencies = [
                { name = "uv-git-workspace-in-root" },
                { name = "workspace-member-in-subdir" },
            ]

            [package.metadata]
            requires-dist = [
                { name = "uv-git-workspace-in-root", git = "https://github.com/astral-sh/workspace-in-root-test.git" },
                { name = "workspace-member-in-subdir", git = "https://github.com/astral-sh/workspace-in-root-test.git?subdirectory=workspace-member-in-subdir" },
            ]

            [[package]]
            name = "uv-git-workspace-in-root"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/workspace-in-root-test.git#d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68" }

            [[package]]
            name = "workspace-member-in-subdir"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/workspace-in-root-test.git?subdirectory=workspace-member-in-subdir#d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68" }
            dependencies = [
                { name = "uv-git-workspace-in-root" },
            ]
            "#
            );
        }
    );

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + uv-git-workspace-in-root==0.1.0 (from git+https://github.com/astral-sh/workspace-in-root-test.git@d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68)
     + workspace-member-in-subdir==0.1.0 (from git+https://github.com/astral-sh/workspace-in-root-test.git@d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68#subdirectory=workspace-member-in-subdir)
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/8887>
#[test]
#[cfg(feature = "git")]
fn sync_git_repeated_member_dynamic_metadata() -> Result<()> {
    let context = TestContext::new("3.13");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["package", "dependency"]

        [tool.uv.sources]
        package = { git = "https://git@github.com/astral-sh/uv-dynamic-metadata-test.git" }
        dependency = { git = "https://git@github.com/astral-sh/uv-dynamic-metadata-test.git", subdirectory = "dependency" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "###);

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
            requires-python = ">=3.13"

            [options]
            exclude-newer = "2024-03-25T00:00:00Z"

            [[package]]
            name = "dependency"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/uv-dynamic-metadata-test.git?subdirectory=dependency#6c5aa0a65db737c9e7e2e60dc865bd8087012e64" }
            dependencies = [
                { name = "iniconfig" },
            ]

            [[package]]
            name = "foo"
            version = "0.1.0"
            source = { virtual = "." }
            dependencies = [
                { name = "dependency" },
                { name = "package" },
            ]

            [package.metadata]
            requires-dist = [
                { name = "dependency", git = "https://github.com/astral-sh/uv-dynamic-metadata-test.git?subdirectory=dependency" },
                { name = "package", git = "https://github.com/astral-sh/uv-dynamic-metadata-test.git" },
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
            name = "package"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/uv-dynamic-metadata-test.git#6c5aa0a65db737c9e7e2e60dc865bd8087012e64" }
            dependencies = [
                { name = "dependency" },
                { name = "typing-extensions" },
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
        }
    );

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + dependency==0.1.0 (from git+https://github.com/astral-sh/uv-dynamic-metadata-test.git@6c5aa0a65db737c9e7e2e60dc865bd8087012e64#subdirectory=dependency)
     + iniconfig==2.0.0
     + package==0.1.0 (from git+https://github.com/astral-sh/uv-dynamic-metadata-test.git@6c5aa0a65db737c9e7e2e60dc865bd8087012e64)
     + typing-extensions==4.10.0
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/8887>
#[test]
#[cfg(feature = "git")]
fn sync_git_repeated_member_backwards_path() -> Result<()> {
    let context = TestContext::new("3.13");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["package", "dependency"]

        [tool.uv.sources]
        package = { git = "https://github.com/astral-sh/uv-backwards-path-test", subdirectory = "root" }
        dependency = { git = "https://github.com/astral-sh/uv-backwards-path-test", subdirectory = "dependency" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "###);

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
            requires-python = ">=3.13"

            [options]
            exclude-newer = "2024-03-25T00:00:00Z"

            [[package]]
            name = "dependency"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/uv-backwards-path-test?subdirectory=dependency#4bcc7fcd2e548c2ab7ba6b97b1c4e3ababccc7a9" }

            [[package]]
            name = "foo"
            version = "0.1.0"
            source = { virtual = "." }
            dependencies = [
                { name = "dependency" },
                { name = "package" },
            ]

            [package.metadata]
            requires-dist = [
                { name = "dependency", git = "https://github.com/astral-sh/uv-backwards-path-test?subdirectory=dependency" },
                { name = "package", git = "https://github.com/astral-sh/uv-backwards-path-test?subdirectory=root" },
            ]

            [[package]]
            name = "package"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/uv-backwards-path-test?subdirectory=root#4bcc7fcd2e548c2ab7ba6b97b1c4e3ababccc7a9" }
            dependencies = [
                { name = "dependency" },
            ]
            "#
            );
        }
    );

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + dependency==0.1.0 (from git+https://github.com/astral-sh/uv-backwards-path-test@4bcc7fcd2e548c2ab7ba6b97b1c4e3ababccc7a9#subdirectory=dependency)
     + package==0.1.0 (from git+https://github.com/astral-sh/uv-backwards-path-test@4bcc7fcd2e548c2ab7ba6b97b1c4e3ababccc7a9#subdirectory=root)
    ");

    Ok(())
}

/// The project itself is marked as an editable dependency, but under the wrong name. The project
/// is a package.
#[test]
fn mismatched_name_self_editable() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["foo"]

        [tool.uv.sources]
        foo = { path = ".", editable = true }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to build `foo @ file://[TEMP_DIR]/`
      ╰─▶ Package metadata name `project` does not match given name `foo`
      help: `foo` was included because `project` (v0.1.0) depends on `foo`
    ");

    Ok(())
}

/// A wheel is available in the cache, but was requested under the wrong name.
#[test]
fn mismatched_name_cached_wheel() -> Result<()> {
    let context = TestContext::new("3.12");

    // Cache the `iniconfig` wheel.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig @ https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0 (from https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz)
    ");

    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["foo @ https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz"]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × Failed to download and build `foo @ https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz`
      ╰─▶ Package metadata name `iniconfig` does not match given name `foo`
    ");

    Ok(())
}

/// Sync a Git repository that depends on a package within the same repository via a `path` source.
///
/// See: <https://github.com/astral-sh/uv/issues/9516>
#[test]
#[cfg(feature = "git")]
fn sync_git_path_dependency() -> Result<()> {
    let context = TestContext::new("3.13");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["package2"]

        [tool.uv.sources]
        package2 = { git = "https://git@github.com/astral-sh/uv-path-dependency-test.git", subdirectory = "package2" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "###);

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
            requires-python = ">=3.13"

            [options]
            exclude-newer = "2024-03-25T00:00:00Z"

            [[package]]
            name = "foo"
            version = "0.1.0"
            source = { virtual = "." }
            dependencies = [
                { name = "package2" },
            ]

            [package.metadata]
            requires-dist = [{ name = "package2", git = "https://github.com/astral-sh/uv-path-dependency-test.git?subdirectory=package2" }]

            [[package]]
            name = "package1"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/uv-path-dependency-test.git?subdirectory=package1#28781b32cf1f260cdb2c8040628079eb265202bd" }

            [[package]]
            name = "package2"
            version = "0.1.0"
            source = { git = "https://github.com/astral-sh/uv-path-dependency-test.git?subdirectory=package2#28781b32cf1f260cdb2c8040628079eb265202bd" }
            dependencies = [
                { name = "package1" },
            ]
            "#
            );
        }
    );

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + package1==0.1.0 (from git+https://github.com/astral-sh/uv-path-dependency-test.git@28781b32cf1f260cdb2c8040628079eb265202bd#subdirectory=package1)
     + package2==0.1.0 (from git+https://github.com/astral-sh/uv-path-dependency-test.git@28781b32cf1f260cdb2c8040628079eb265202bd#subdirectory=package2)
    ");

    Ok(())
}

#[test]
fn url_hash_mismatch() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [tool.uv.sources]
        iniconfig = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz" }
        "#,
    )?;

    // Write a lockfile with an invalid hash.
    context.temp_dir.child("uv.lock").write_str(indoc! {r#"
        version = 1
        requires-python = ">=3.12"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz" }
        sdist = { hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b4" }

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "iniconfig" },
        ]

        [package.metadata]
        requires-dist = [{ name = "iniconfig", url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz" }]
    "#})?;

    // Running `uv sync` should fail.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to download and build `iniconfig @ https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz`
      ╰─▶ Hash mismatch for `iniconfig @ https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz`

          Expected:
            sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b4

          Computed:
            sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3
      help: `iniconfig` was included because `project` (v0.1.0) depends on `iniconfig`
    ");

    Ok(())
}

#[test]
fn path_hash_mismatch() -> Result<()> {
    let context = TestContext::new("3.12");

    // Download the source.
    let archive = context.temp_dir.child("iniconfig-2.0.0.tar.gz");
    download_to_disk(
        "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz",
        &archive,
    );

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [tool.uv.sources]
        iniconfig = { path = "iniconfig-2.0.0.tar.gz" }
        "#,
    )?;

    // Write a lockfile with an invalid hash.
    context.temp_dir.child("uv.lock").write_str(indoc! {r#"
        version = 1
        requires-python = ">=3.12"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { path = "iniconfig-2.0.0.tar.gz" }
        sdist = { hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b4" }

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "iniconfig" },
        ]

        [package.metadata]
        requires-dist = [{ name = "iniconfig", path = "iniconfig-2.0.0.tar.gz" }]
    "#})?;

    // Running `uv sync` should fail.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
      × Failed to build `iniconfig @ file://[TEMP_DIR]/iniconfig-2.0.0.tar.gz`
      ╰─▶ Hash mismatch for `iniconfig @ file://[TEMP_DIR]/iniconfig-2.0.0.tar.gz`

          Expected:
            sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b4

          Computed:
            sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3
      help: `iniconfig` was included because `project` (v0.1.0) depends on `iniconfig`
    ");

    Ok(())
}

#[test]
fn find_links_relative_in_config_works_from_subdir() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "subdir_test"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["ok==1.0.0"]

        [tool.uv]
        find-links = ["packages/"]
    "#})?;

    // Create packages/ subdirectory and copy our "offline" tqdm wheel there
    let packages = context.temp_dir.child("packages");
    packages.create_dir_all()?;

    let wheel_src = context
        .workspace_root
        .join("test/links/ok-1.0.0-py3-none-any.whl");
    let wheel_dst = packages.child("ok-1.0.0-py3-none-any.whl");
    fs_err::copy(&wheel_src, &wheel_dst)?;

    // Create a separate subdir, which will become our working directory
    let subdir = context.temp_dir.child("subdir");
    subdir.create_dir_all()?;

    // Run `uv sync --offline` from subdir. We expect it to find the local wheel in ../packages/.
    uv_snapshot!(context.filters(), context.sync().current_dir(&subdir).arg("--offline"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + ok==1.0.0
    ");

    Ok(())
}

#[test]
fn unsupported_git_scheme() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.12"]);

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["foo"]

        [tool.uv.sources]
        # `c:/...` looks like an absolute path, but this field requires a URL such as `file:///...`.
        foo = { git = "c:/home/ferris/projects/foo", rev = "7701ffcbae245819b828dc5f885a5201158897ef" }
        "#},
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
      × Failed to build `foo @ file://[TEMP_DIR]/`
      ├─▶ Failed to parse entry: `foo`
      ╰─▶ Unsupported Git URL scheme `c:` in `c:/home/ferris/projects/foo` (expected one of `https:`, `ssh:`, or `file:`)
    ");
    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11703>
#[test]
fn prune_cache_url_subdirectory() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "root",
        ]

        [tool.uv.sources]
        root = { url = "https://github.com/user-attachments/files/18216295/subdirectory-test.tar.gz", subdirectory = "packages/root" }
    "#})?;

    // Lock the project.
    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "###);

    // Prune the cache.
    context.prune().arg("--ci").assert().success();

    // Install the project.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + root==0.0.1 (from https://github.com/user-attachments/files/18216295/subdirectory-test.tar.gz#subdirectory=packages/root)
     + sniffio==1.3.1
    ");

    Ok(())
}

/// Test that incoherence in the versions in a package entry of the lockfile versions is caught.
///
/// See <https://github.com/astral-sh/uv/issues/12164>
#[test]
fn locked_version_coherence() -> Result<()> {
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

    uv_snapshot!(context.filters(), context.lock(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
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
        "#);
    });

    // Write an inconsistent iniconfig entry
    context
        .temp_dir
        .child("uv.lock")
        .write_str(&lock.replace(r#"version = "2.0.0""#, r#"version = "1.0.0""#))?;

    // An inconsistent lockfile should fail with `--locked`
    uv_snapshot!(context.filters(), context.sync().arg("--locked"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse `uv.lock`
      Caused by: The entry for package `iniconfig` (1.0.0) has wheel `iniconfig-2.0.0-py3-none-any.whl` with inconsistent version (2.0.0), which indicates a malformed wheel. If this is intentional, set `UV_SKIP_WHEEL_FILENAME_CHECK=1`.
    ");

    // Without `--locked`, we could fail or recreate the lockfile, currently, we fail.
    uv_snapshot!(context.filters(), context.lock(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse `uv.lock`
      Caused by: The entry for package `iniconfig` (1.0.0) has wheel `iniconfig-2.0.0-py3-none-any.whl` with inconsistent version (2.0.0), which indicates a malformed wheel. If this is intentional, set `UV_SKIP_WHEEL_FILENAME_CHECK=1`.
    ");

    Ok(())
}

/// Ensure that existing `uv.lock` files can use `upload_time` or `upload-time` interchangeably.
#[test]
fn sync_upload_time() -> Result<()> {
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

    uv_snapshot!(context.filters(), context.lock(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "###);

    let uv_lock = context.temp_dir.child("uv.lock");
    uv_lock.write_str(r#"
        version = 1
        revision = 2
        requires-python = ">=3.12"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "anyio"
        version = "3.7.0"
        source = { registry = "https://pypi.org/simple" }
        dependencies = [
            { name = "idna" },
            { name = "sniffio" },
        ]
        sdist = { url = "https://files.pythonhosted.org/packages/c6/b3/fefbf7e78ab3b805dec67d698dc18dd505af7a18a8dd08868c9b4fa736b5/anyio-3.7.0.tar.gz", hash = "sha256:275d9973793619a5374e1c89a4f4ad3f4b0a5510a2b5b939444bee8f4c4d37ce", size = 142737, upload_time = "2023-05-27T11:12:46.688Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/68/fe/7ce1926952c8a403b35029e194555558514b365ad77d75125f521a2bec62/anyio-3.7.0-py3-none-any.whl", hash = "sha256:eddca883c4175f14df8aedce21054bfca3adb70ffe76a9f607aef9d7fa2ea7f0", size = 80873, upload_time = "2023-05-27T11:12:44.474Z" },
        ]

        [[package]]
        name = "idna"
        version = "3.6"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/bf/3f/ea4b9117521a1e9c50344b909be7886dd00a519552724809bb1f486986c2/idna-3.6.tar.gz", hash = "sha256:9ecdbbd083b06798ae1e86adcbfe8ab1479cf864e4ee30fe4e46a003d12491ca", size = 175426, upload_time = "2023-11-25T15:40:54.902Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl", hash = "sha256:c05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f", size = 61567, upload_time = "2023-11-25T15:40:52.604Z" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "anyio" },
        ]

        [package.metadata]
        requires-dist = [{ name = "anyio", specifier = "==3.7.0" }]

        [[package]]
        name = "sniffio"
        version = "1.3.1"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/a2/87/a6771e1546d97e7e041b6ae58d80074f81b7d5121207425c964ddf5cfdbd/sniffio-1.3.1.tar.gz", hash = "sha256:f4324edc670a0f49750a81b895f35c3adb843cca46f0530f79fc1babb23789dc", size = 20372, upload_time = "2024-02-25T23:20:04.057Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/e9/44/75a9c9421471a6c4805dbf2356f7c181a29c1879239abab1ea2cc8f38b40/sniffio-1.3.1-py3-none-any.whl", hash = "sha256:2f6da418d1f1e0fddd844478f41680e794e6051915791a034ff65e5f100525a2", size = 10235, upload_time = "2024-02-25T23:20:01.196Z" },
        ]
    "#)?;

    // Install from the lockfile.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==3.7.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Re-install from the lockfile.
    uv_snapshot!(context.filters(), context.sync().arg("--frozen"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Audited 3 packages in [TIME]
    ");

    Ok(())
}

/// Ensure that workspace members that are also development dependencies are not duplicated with
/// `--all-packages`.
///
/// See: <https://github.com/astral-sh/uv/issues/13673#issuecomment-2912196406>
#[test]
fn repeated_dev_member_all_packages() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "first"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [dependency-groups]
        dev = ["second"]

        [tool.uv.sources]
        second = { workspace = true }

        [tool.uv.workspace]
        members = ["second"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;

    let src = context.temp_dir.child("src").child("first");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    let child = context.temp_dir.child("second");
    fs_err::create_dir_all(&child)?;

    let pyproject_toml = child.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "second"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;

    let src = child.child("src").child("second");
    src.create_dir_all()?;

    let init = src.child("__init__.py");
    init.touch()?;

    uv_snapshot!(context.filters(), context.sync().arg("--all-packages"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + first==0.1.0 (from file://[TEMP_DIR]/)
     + iniconfig==2.0.0
     + second==0.1.0 (from file://[TEMP_DIR]/second)
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--all-packages"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Audited 3 packages in [TIME]
    ");

    Ok(())
}

/// Test that hash checking doesn't fail with dependency metadata.
#[test]
fn direct_url_dependency_metadata() -> Result<()> {
    let context = TestContext::new("3.12");
    context.temp_dir.child("pyproject.toml").write_str(r#"
        [project]
        name = "debug"
        version = "0.1.0"
        requires-python = ">=3.9"
        dependencies = [
            "tqdm",
        ]

        [tool.uv]
        dependency-metadata = [
          { name = "tqdm", version = "4.67.1", requires-dist = [] },
        ]

        [tool.uv.sources]
        tqdm = { url = "https://files.pythonhosted.org/packages/d0/30/dc54f88dd4a2b5dc8a0279bdd7270e735851848b762aeb1c1184ed1f6b14/tqdm-4.67.1-py3-none-any.whl" }
        "#
    )?;

    uv_snapshot!(context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + tqdm==4.67.1 (from https://files.pythonhosted.org/packages/d0/30/dc54f88dd4a2b5dc8a0279bdd7270e735851848b762aeb1c1184ed1f6b14/tqdm-4.67.1-py3-none-any.whl)
    ");

    Ok(())
}

#[test]
fn sync_url_with_query_parameters() -> Result<()> {
    let context = TestContext::new("3.13").with_exclude_newer("2025-03-24T19:00:00Z");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(r#"
        [project]
        name = "example"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["source-distribution @ https://files.pythonhosted.org/packages/1f/e5/5b016c945d745f8b108e759d428341488a6aee8f51f07c6c4e33498bb91f/source_distribution-0.0.3.tar.gz?foo=bar"]
        "#
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + source-distribution==0.0.3 (from https://files.pythonhosted.org/packages/1f/e5/5b016c945d745f8b108e759d428341488a6aee8f51f07c6c4e33498bb91f/source_distribution-0.0.3.tar.gz?foo=bar)
    ");

    Ok(())
}

/// Test uv sync with --exclude-newer-package
#[test]
fn sync_exclude_newer_package() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "tqdm",
    "requests",
]
"#,
    )?;

    // First sync with only the global exclude-newer to show the baseline
    uv_snapshot!(context.filters(), context
        .sync()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--exclude-newer")
        .arg("2022-04-04T12:00:00Z"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + certifi==2021.10.8
     + charset-normalizer==2.0.12
     + idna==3.3
     + requests==2.27.1
     + tqdm==4.64.0
     + urllib3==1.26.9
    "
    );

    // Now sync with --exclude-newer-package to allow tqdm to use a newer version
    uv_snapshot!(context.filters(), context
        .sync()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--exclude-newer")
        .arg("2022-04-04T12:00:00Z")
        .arg("--exclude-newer-package")
        .arg("tqdm=2022-09-04T00:00:00Z"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Ignoring existing lockfile due to addition of exclude newer `2022-09-04T00:00:00Z` for package `tqdm`
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Uninstalled [N] packages in [TIME]
    Installed [N] packages in [TIME]
     - tqdm==4.64.0
     + tqdm==4.64.1
    "
    );

    Ok(())
}

/// Test exclude-newer-package in pyproject.toml configuration
#[test]
fn sync_exclude_newer_package_config() -> Result<()> {
    let context = TestContext::new("3.12").with_filtered_counts();

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "tqdm",
    "requests",
]

[tool.uv]
exclude-newer = "2022-04-04T12:00:00Z"
"#,
    )?;

    // First sync with only the global exclude-newer from the config
    uv_snapshot!(context.filters(), context
        .sync()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + certifi==2021.10.8
     + charset-normalizer==2.0.12
     + idna==3.3
     + requests==2.27.1
     + tqdm==4.64.0
     + urllib3==1.26.9
    "
    );

    // Now add the package-specific exclude-newer to the config
    pyproject_toml.write_str(
        r#"
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "tqdm",
    "requests",
]

[tool.uv]
exclude-newer = "2022-04-04T12:00:00Z"
exclude-newer-package = { tqdm = "2022-09-04T00:00:00Z" }
"#,
    )?;

    // Sync again with the package-specific override
    uv_snapshot!(context.filters(), context
        .sync()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Ignoring existing lockfile due to addition of exclude newer `2022-09-04T00:00:00Z` for package `tqdm`
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Uninstalled [N] packages in [TIME]
    Installed [N] packages in [TIME]
     - tqdm==4.64.0
     + tqdm==4.64.1
    "
    );

    Ok(())
}

#[test]
#[cfg(unix)]
fn read_only() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

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

    uv_snapshot!(context.filters(), context.sync(), @r"
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

    // Remove the flock.
    fs_err::remove_file(context.venv.child(".lock"))?;

    // Make the virtual environment read and execute (but not write).
    fs_err::set_permissions(&context.venv, std::fs::Permissions::from_mode(0o555))?;

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

#[test]
fn sync_python_platform() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["black"]
        "#,
    )?;

    // Lock the project
    context.lock().assert().success();

    // Sync with a specific platform should filter packages
    uv_snapshot!(context.filters(), context.sync().arg("--python-platform").arg("linux"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    Prepared 6 packages in [TIME]
    Installed 6 packages in [TIME]
     + black==24.3.0
     + click==8.1.7
     + mypy-extensions==1.0.0
     + packaging==24.0
     + pathspec==0.12.1
     + platformdirs==4.2.0
    ");

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11648>
#[test]
#[cfg(not(windows))]
fn conflicting_editable() -> Result<()> {
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
            "child",
        ]
        bar = [
            "child",
        ]
        [tool.uv]
        conflicts = [
          [
            { group = "foo" },
            { group = "bar" },
          ],
        ]
        [tool.uv.sources]
        child = [
            { path = "./child", editable = true, group = "foo" },
            { path = "./child", editable = false, group = "bar" },
        ]
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
        dependencies = []
        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
        )?;
    context
        .temp_dir
        .child("child")
        .child("src")
        .child("child")
        .child("__init__.py")
        .touch()?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Audited in [TIME]
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
        conflicts = [[
            { package = "project", group = "bar" },
            { package = "project", group = "foo" },
        ]]

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "child"
        version = "0.1.0"
        source = { directory = "child" }

        [[package]]
        name = "child"
        version = "0.1.0"
        source = { editable = "child" }

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }

        [package.dev-dependencies]
        bar = [
            { name = "child", version = "0.1.0", source = { directory = "child" } },
        ]
        foo = [
            { name = "child", version = "0.1.0", source = { editable = "child" } },
        ]

        [package.metadata]

        [package.metadata.requires-dev]
        bar = [{ name = "child", directory = "child" }]
        foo = [{ name = "child", editable = "child" }]
        "#
        );
    });

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    uv_snapshot!(context.filters(), context.pip_list().arg("--format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    [{"name":"child","version":"0.1.0","editable_project_location":"[TEMP_DIR]/child"}]

    ----- stderr -----
    "#);

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    uv_snapshot!(context.filters(), context.pip_list().arg("--format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    [{"name":"child","version":"0.1.0"}]

    ----- stderr -----
    "#);

    Ok(())
}

/// See: <https://github.com/astral-sh/uv/issues/11648>
#[test]
#[cfg(not(windows))]
fn undeclared_editable() -> Result<()> {
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
            "child",
        ]
        bar = [
            "child",
        ]
        [tool.uv]
        conflicts = [
          [
            { group = "foo" },
            { group = "bar" },
          ],
        ]
        [tool.uv.sources]
        child = [
            { path = "./child", editable = true, group = "foo" },
            { path = "./child", group = "bar" },
        ]
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
        dependencies = []
        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
        )?;
    context
        .temp_dir
        .child("child")
        .child("src")
        .child("child")
        .child("__init__.py")
        .touch()?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Audited in [TIME]
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
        conflicts = [[
            { package = "project", group = "bar" },
            { package = "project", group = "foo" },
        ]]

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "child"
        version = "0.1.0"
        source = { directory = "child" }

        [[package]]
        name = "child"
        version = "0.1.0"
        source = { editable = "child" }

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }

        [package.dev-dependencies]
        bar = [
            { name = "child", version = "0.1.0", source = { directory = "child" } },
        ]
        foo = [
            { name = "child", version = "0.1.0", source = { editable = "child" } },
        ]

        [package.metadata]

        [package.metadata.requires-dev]
        bar = [{ name = "child", directory = "child" }]
        foo = [{ name = "child", editable = "child" }]
        "#
        );
    });

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("foo"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    uv_snapshot!(context.filters(), context.pip_list().arg("--format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    [{"name":"child","version":"0.1.0","editable_project_location":"[TEMP_DIR]/child"}]

    ----- stderr -----
    "#);

    uv_snapshot!(context.filters(), context.sync().arg("--group").arg("bar"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    uv_snapshot!(context.filters(), context.pip_list().arg("--format").arg("json"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    [{"name":"child","version":"0.1.0"}]

    ----- stderr -----
    "#);

    Ok(())
}

#[test]
fn sync_python_preference() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.12", "3.11"]);

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

    // Run an initial sync, with 3.12 as an "unmanaged" interpreter
    context.sync().assert().success();

    // Mark 3.12 as a managed interpreter for the rest of the tests
    let context = context.with_versions_as_managed(&["3.12"]);
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    // We should invalidate the environment and switch to 3.11
    uv_snapshot!(context.filters(), context.sync().arg("--no-managed-python"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    // We will use the environment if it exists
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    // Unless the user requests a Python preference that is incompatible
    uv_snapshot!(context.filters(), context.sync().arg("--managed-python"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    // If a interpreter cannot be found, we'll fail
    uv_snapshot!(context.filters(), context.sync().arg("--managed-python").arg("-p").arg("3.11"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No interpreter found for Python 3.11 in managed installations

    hint: A managed Python download is available for Python 3.11, but Python downloads are set to 'never'
    ");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.11"
        dependencies = []

        [tool.uv]
        python-preference = "only-system"
        "#,
    )?;

    // We'll respect a `python-preference` in the `pyproject.toml` file
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    // But it can be overridden via the CLI
    uv_snapshot!(context.filters(), context.sync().arg("--managed-python"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    // `uv run` will invalidate the environment too
    uv_snapshot!(context.filters(), context.run().arg("python").arg("--version"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Python 3.11.[X]

    ----- stderr -----
    Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
    Removed virtual environment at: .venv
    Creating virtual environment at: .venv
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    Ok(())
}

#[test]
fn sync_config_settings_package() -> Result<()> {
    let context = TestContext::new("3.12").with_exclude_newer("2025-07-25T00:00:00Z");

    // Create a child project that uses `setuptools`.
    let dependency = context.temp_dir.child("dependency");
    dependency.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "dependency"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []
        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;
    dependency
        .child("dependency")
        .child("__init__.py")
        .touch()?;

    // Install the `dependency` without `editable_mode=compat`.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["dependency"]

        [tool.uv.sources]
        dependency = { path = "dependency", editable = true }
        "#,
    )?;

    // Lock the project
    context.lock().assert().success();

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + dependency==0.1.0 (from file://[TEMP_DIR]/dependency)
    ");

    // When installed without `editable_mode=compat`, the `finder.py` file should be present.
    let finder = context
        .site_packages()
        .join("__editable___dependency_0_1_0_finder.py");
    assert!(finder.exists());

    // Remove the virtual environment.
    fs_err::remove_dir_all(&context.venv)?;

    // Install the `dependency` with `editable_mode=compat` scoped to the package.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["dependency"]

        [tool.uv.sources]
        dependency = { path = "dependency", editable = true }

        [tool.uv.config-settings-package]
        dependency = { editable_mode = "compat" }
        "#,
    )?;

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
     + dependency==0.1.0 (from file://[TEMP_DIR]/dependency)
    ");

    // When installed with `editable_mode=compat`, the `finder.py` file should _not_ be present.
    let finder = context
        .site_packages()
        .join("__editable___dependency_0_1_0_finder.py");
    assert!(!finder.exists());

    // Remove the virtual environment.
    fs_err::remove_dir_all(&context.venv)?;

    // Install the `dependency` with `editable_mode=compat` scoped to another package.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["dependency"]

        [tool.uv.sources]
        dependency = { path = "dependency", editable = true }

        [tool.uv.config-settings-package]
        setuptools = { editable_mode = "compat" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
    Creating virtual environment at: .venv
    Resolved 2 packages in [TIME]
    Installed 1 package in [TIME]
     + dependency==0.1.0 (from file://[TEMP_DIR]/dependency)
    ");

    // When installed without `editable_mode=compat`, the `finder.py` file should be present.
    let finder = context
        .site_packages()
        .join("__editable___dependency_0_1_0_finder.py");
    assert!(finder.exists());

    Ok(())
}

#[test]
fn reject_unmatched_runtime() -> Result<()> {
    let context = TestContext::new("3.12").with_exclude_newer("2025-01-01T00:00Z");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
         [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["source-distribution", "iniconfig"]

        [tool.uv.extra-build-dependencies]
        source-distribution = [{ requirement = "iniconfig", match-runtime = true }]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.lock(), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
      × Failed to download and build `source-distribution==0.0.3`
      ╰─▶ Extra build requirement `iniconfig` was declared with `match-runtime = true`, but `source-distribution` does not declare static metadata, making runtime-matching impossible
      help: `source-distribution` (v0.0.3) was included because `foo` (v0.1.0) depends on `source-distribution`
    ");

    Ok(())
}

/// Test Git LFS configuration.
#[test]
#[cfg(feature = "git-lfs")]
fn sync_git_lfs() -> Result<()> {
    let context = TestContext::new("3.13").with_git_lfs_config();
    let pyproject_toml = context.temp_dir.child("pyproject.toml");

    // Set `lfs = true` in the source
    pyproject_toml.write_str(
        r#"
        [project]
        name = "test-project"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["test-lfs-repo"]

        [tool.uv.sources]
        test-lfs-repo = { git = "https://github.com/astral-sh/test-lfs-repo.git", rev = "657500f0703dc173ac5d68dfa1d7e8c985c84424", lfs = true }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().env_remove(EnvVars::UV_GIT_LFS), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
    ");

    // Verify that we can import the module and access LFS content
    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    "#);

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => context.filters(),
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.13"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "test-lfs-repo"
        version = "0.1.0"
        source = { git = "https://github.com/astral-sh/test-lfs-repo.git?lfs=true&rev=657500f0703dc173ac5d68dfa1d7e8c985c84424#657500f0703dc173ac5d68dfa1d7e8c985c84424" }

        [[package]]
        name = "test-project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "test-lfs-repo" },
        ]

        [package.metadata]
        requires-dist = [{ name = "test-lfs-repo", git = "https://github.com/astral-sh/test-lfs-repo.git?lfs=true&rev=657500f0703dc173ac5d68dfa1d7e8c985c84424" }]
        "#
        );
    });

    // `UV_GIT_LFS=false` should not override `lfs = true`
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_GIT_LFS, "false").arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
    ");

    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    "#);

    // Set `lfs = false` in the source
    pyproject_toml.write_str(
        r#"
        [project]
        name = "test-project"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["test-lfs-repo"]

        [tool.uv.sources]
        test-lfs-repo = { git = "https://github.com/astral-sh/test-lfs-repo.git", rev = "657500f0703dc173ac5d68dfa1d7e8c985c84424", lfs = false }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().env_remove(EnvVars::UV_GIT_LFS).arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
     + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424)
    ");

    // Verify that LFS content is missing (import should fail)
    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Traceback (most recent call last):
      File "<string>", line 1, in <module>
        import test_lfs_repo.lfs_module
      File "[SITE_PACKAGES]/test_lfs_repo/lfs_module.py", line 1
        version https://git-lfs.github.com/spec/v1
                ^^^^^
    SyntaxError: invalid syntax
    "#);

    // `UV_GIT_lfs=true` should not override `lfs = false`
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_GIT_LFS, "true").arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424)
    ");

    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Traceback (most recent call last):
      File "<string>", line 1, in <module>
        import test_lfs_repo.lfs_module
      File "[SITE_PACKAGES]/test_lfs_repo/lfs_module.py", line 1
        version https://git-lfs.github.com/spec/v1
                ^^^^^
    SyntaxError: invalid syntax
    "#);

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => context.filters(),
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.13"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "test-lfs-repo"
        version = "0.1.0"
        source = { git = "https://github.com/astral-sh/test-lfs-repo.git?rev=657500f0703dc173ac5d68dfa1d7e8c985c84424#657500f0703dc173ac5d68dfa1d7e8c985c84424" }

        [[package]]
        name = "test-project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "test-lfs-repo" },
        ]

        [package.metadata]
        requires-dist = [{ name = "test-lfs-repo", git = "https://github.com/astral-sh/test-lfs-repo.git?rev=657500f0703dc173ac5d68dfa1d7e8c985c84424" }]
        "#
        );
    });

    // `UV_GIT_LFS = true` should work without explicit lfs flag
    pyproject_toml.write_str(
        r#"
        [project]
        name = "test-project"
        version = "0.1.0"
        requires-python = ">=3.13"
        dependencies = ["test-lfs-repo"]

        [tool.uv.sources]
        test-lfs-repo = { git = "https://github.com/astral-sh/test-lfs-repo.git", rev = "657500f0703dc173ac5d68dfa1d7e8c985c84424" }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_GIT_LFS, "true").arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424)
     + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
    ");

    // Verify that we can import the module when UV_GIT_LFS is set
    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module; print('LFS module imported via env var')"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    LFS module imported via env var

    ----- stderr -----
    "#);

    // Cache should be primed with non-LFS sources
    uv_snapshot!(context.filters(), context.sync().env_remove(EnvVars::UV_GIT_LFS).arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
     + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424)
    ");

    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Traceback (most recent call last):
      File "<string>", line 1, in <module>
        import test_lfs_repo.lfs_module
      File "[SITE_PACKAGES]/test_lfs_repo/lfs_module.py", line 1
        version https://git-lfs.github.com/spec/v1
                ^^^^^
    SyntaxError: invalid syntax
    "#);

    // Cache should be primed with LFS sources
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_GIT_LFS, "true").arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424)
     + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
    ");

    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module; print('LFS module imported via env var')"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    LFS module imported via env var

    ----- stderr -----
    "#);

    // Cache should hit non-LFS sources
    uv_snapshot!(context.filters(), context.sync().env_remove(EnvVars::UV_GIT_LFS).arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
     + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424)
    ");

    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Traceback (most recent call last):
      File "<string>", line 1, in <module>
        import test_lfs_repo.lfs_module
      File "[SITE_PACKAGES]/test_lfs_repo/lfs_module.py", line 1
        version https://git-lfs.github.com/spec/v1
                ^^^^^
    SyntaxError: invalid syntax
    "#);

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => context.filters(),
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.13"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "test-lfs-repo"
        version = "0.1.0"
        source = { git = "https://github.com/astral-sh/test-lfs-repo.git?rev=657500f0703dc173ac5d68dfa1d7e8c985c84424#657500f0703dc173ac5d68dfa1d7e8c985c84424" }

        [[package]]
        name = "test-project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "test-lfs-repo" },
        ]

        [package.metadata]
        requires-dist = [{ name = "test-lfs-repo", git = "https://github.com/astral-sh/test-lfs-repo.git?rev=657500f0703dc173ac5d68dfa1d7e8c985c84424" }]
        "#
        );
    });

    // Cache should hit LFS sources
    uv_snapshot!(context.filters(), context.sync().env(EnvVars::UV_GIT_LFS, "true").arg("--reinstall"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424)
     + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo.git@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
    ");

    uv_snapshot!(context.filters(), context.python_command()
        .arg("-c")
        .arg("import test_lfs_repo.lfs_module; print('LFS module imported via env var')"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    LFS module imported via env var

    ----- stderr -----
    "#);

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => context.filters(),
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.13"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [[package]]
        name = "test-lfs-repo"
        version = "0.1.0"
        source = { git = "https://github.com/astral-sh/test-lfs-repo.git?lfs=true&rev=657500f0703dc173ac5d68dfa1d7e8c985c84424#657500f0703dc173ac5d68dfa1d7e8c985c84424" }

        [[package]]
        name = "test-project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "test-lfs-repo" },
        ]

        [package.metadata]
        requires-dist = [{ name = "test-lfs-repo", git = "https://github.com/astral-sh/test-lfs-repo.git?lfs=true&rev=657500f0703dc173ac5d68dfa1d7e8c985c84424" }]
        "#
        );
    });

    Ok(())
}

#[test]
fn match_runtime_optional() -> Result<()> {
    let context = TestContext::new("3.12").with_exclude_newer("2025-01-01T00:00Z");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [project.optional-dependencies]
        bar = ["iniconfig", "typing-extensions"]

        [tool.uv.sources]
        typing-extensions = { url = "https://files.pythonhosted.org/packages/72/94/1a15dd82efb362ac84269196e94cf00f187f7ed21c242792a923cdb1c61f/typing_extensions-4.15.0.tar.gz" }

        [tool.uv.extra-build-dependencies]
        typing-extensions = [{ requirement = "iniconfig", match-runtime = true }]
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: The `extra-build-dependencies` option is experimental and may change without warning. Pass `--preview-features extra-build-dependencies` to disable this warning.
    Resolved 3 packages in [TIME]
    Audited in [TIME]
    ");

    Ok(())
}

#[test]
#[cfg(not(windows))]
fn toggle_workspace_editable() -> Result<()> {
    let context = TestContext::new("3.12");

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

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [tool.uv.workspace]
        members = ["child"]

        [tool.uv.sources]
        child = { workspace = true }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/child)
     + iniconfig==2.0.0
    ");

    let lock = context.read("uv.lock");

    // The child should be editable by default.
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
            "project",
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
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "child" },
        ]

        [package.metadata]
        requires-dist = [{ name = "child", editable = "child" }]
        "#
        );
    });

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [tool.uv.workspace]
        members = ["child"]

        [tool.uv.sources]
        child = { workspace = true, editable = false }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    let lock = context.read("uv.lock");

    // Setting `--no-editable` should make the child non-editable.
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
            "project",
        ]

        [[package]]
        name = "child"
        version = "0.1.0"
        source = { directory = "child" }
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
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "child" },
        ]

        [package.metadata]
        requires-dist = [{ name = "child", directory = "child" }]
        "#
        );
    });

    // Verify that `_child.pth` does not exist in the site-packages directory.
    assert!(!context.site_packages().join("_child.pth").exists());

    // But `--editable` on the command line should override the lockfile.
    uv_snapshot!(context.filters(), context.sync().arg("--editable"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child==0.1.0 (from file://[TEMP_DIR]/child)
    ");

    // Verify that `_child.pth` exists in the site-packages directory.
    assert!(context.site_packages().join("_child.pth").exists());

    Ok(())
}

#[test]
#[cfg(not(windows))]
fn workspace_editable_conflict() -> Result<()> {
    let context = TestContext::new("3.12");

    let child1 = context.temp_dir.child("child1");
    let pyproject_toml = child1.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "child1"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>=1"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;
    child1
        .child("src")
        .child("child1")
        .child("__init__.py")
        .touch()?;

    let child2 = context.temp_dir.child("child2");
    let pyproject_toml = child2.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "child2"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child1"]

        [tool.uv.sources]
        child1 = { workspace = true }

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;
    child2
        .child("src")
        .child("child2")
        .child("__init__.py")
        .touch()?;

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child1"]

        [tool.uv.workspace]
        members = ["child1", "child2"]

        [tool.uv.sources]
        child1 = { workspace = true, editable = true }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 2 packages in [TIME]
     + child1==0.1.0 (from file://[TEMP_DIR]/child1)
     + iniconfig==2.0.0
    ");

    let lock = context.read("uv.lock");

    // If one member declares `editable = true`, and the other omits `editable`, use editable.
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
            "child1",
            "child2",
            "project",
        ]

        [[package]]
        name = "child1"
        version = "0.1.0"
        source = { editable = "child1" }
        dependencies = [
            { name = "iniconfig" },
        ]

        [package.metadata]
        requires-dist = [{ name = "iniconfig", specifier = ">=1" }]

        [[package]]
        name = "child2"
        version = "0.1.0"
        source = { editable = "child2" }
        dependencies = [
            { name = "child1" },
        ]

        [package.metadata]
        requires-dist = [{ name = "child1", editable = "child1" }]

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
            { name = "child1" },
        ]

        [package.metadata]
        requires-dist = [{ name = "child1", editable = "child1" }]
        "#
        );
    });

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child1"]

        [tool.uv.workspace]
        members = ["child1", "child2"]

        [tool.uv.sources]
        child1 = { workspace = true, editable = false }
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     ~ child1==0.1.0 (from file://[TEMP_DIR]/child1)
    ");

    let lock = context.read("uv.lock");

    // If one member declares `editable = false`, and the other omits `editable`, use non-editable.
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
            "child1",
            "child2",
            "project",
        ]

        [[package]]
        name = "child1"
        version = "0.1.0"
        source = { directory = "child1" }
        dependencies = [
            { name = "iniconfig" },
        ]

        [package.metadata]
        requires-dist = [{ name = "iniconfig", specifier = ">=1" }]

        [[package]]
        name = "child2"
        version = "0.1.0"
        source = { editable = "child2" }
        dependencies = [
            { name = "child1" },
        ]

        [package.metadata]
        requires-dist = [{ name = "child1", directory = "child1" }]

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
            { name = "child1" },
        ]

        [package.metadata]
        requires-dist = [{ name = "child1", directory = "child1" }]
        "#
        );
    });

    let child2 = context.temp_dir.child("child2");
    let pyproject_toml = child2.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "child2"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child1"]

        [tool.uv.sources]
        child1 = { workspace = true, editable = true }

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;

    // If the `editable` declarations are conflicting, raise an error.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Workspace member `child1` was requested as both `editable = true` and `editable = false`
    ");

    Ok(())
}

/// `uv sync --no-sources` should consistently switch from editable to package installation.
///
/// See: <https://github.com/astral-sh/uv/issues/15190>
#[test]
fn sync_no_sources_editable_to_package_switch() -> Result<()> {
    let context = TestContext::new("3.12");

    // Create a local package that will be used as editable dependency.
    let local_dep = context.temp_dir.child("local_dep");
    local_dep.create_dir_all()?;

    let local_dep_pyproject = local_dep.child("pyproject.toml");
    local_dep_pyproject.write_str(
        r#"
        [project]
        name = "anyio"
        version = "4.3.0"
        description = "Local test package mimicking anyio"
        requires-python = ">=3.8"

        [build-system]
        requires = ["setuptools>=61", "wheel"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    // Create main project with editable source for the local dependency.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "test_no_sources"
        version = "0.0.1"
        requires-python = ">=3.12"
        dependencies = ["anyio"]

        [tool.uv.sources]
        anyio = { path = "./local_dep", editable = true }

        [build-system]
        requires = ["setuptools>=67"]
        build-backend = "setuptools.build_meta"

        [tool.setuptools.packages.find]
        exclude = ["local_dep*"]
        "#,
    )?;

    // Step 1: `uv sync --no-sources` should install `anyio` from PyPI.
    uv_snapshot!(context.filters(), context.sync().arg("--no-sources"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Prepared 4 packages in [TIME]
    Installed 4 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
     + test-no-sources==0.0.1 (from file://[TEMP_DIR]/)
    ");

    // Step 2: `uv sync` should switch to an editable installation.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 3 packages in [TIME]
    Installed 1 package in [TIME]
     - anyio==4.3.0
     + anyio==4.3.0 (from file://[TEMP_DIR]/local_dep)
     - idna==3.6
     - sniffio==1.3.1
    ");

    // Step 3: `uv sync --no-sources` again should switch back to PyPI package.
    uv_snapshot!(context.filters(), context.sync().arg("--no-sources"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 3 packages in [TIME]
     - anyio==4.3.0 (from file://[TEMP_DIR]/local_dep)
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    Ok(())
}

#[test]
fn sync_fails_ambiguous_url() -> Result<()> {
    let context = TestContext::new("3.12");
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [[tool.uv.index]]
        name = "bug"
        url = "https://user/name:password@domain/a/b/c"
        default = true
        "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r#"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    warning: Failed to parse `pyproject.toml` during settings discovery:
      TOML parse error at line 10, column 15
         |
      10 |         url = "https://user/name:password@domain/a/b/c"
         |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      ambiguous user/pass authority in URL (not percent-encoded?): https:***@domain/a/b/c

    error: Failed to parse: `pyproject.toml`
      Caused by: TOML parse error at line 10, column 15
       |
    10 |         url = "https://user/name:password@domain/a/b/c"
       |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ambiguous user/pass authority in URL (not percent-encoded?): https:***@domain/a/b/c
    "#);

    Ok(())
}

/// Test that when a local directory dependency's version changes, the planner reinstalls it
/// even if the source directory content (cache info) hasn't changed.
///
/// Regression test for: <https://github.com/astral-sh/uv/issues/17370>
#[test]
fn sync_reinstalls_on_version_change() -> Result<()> {
    let context = TestContext::new("3.12");

    // Create a workspace with a local directory dependency.
    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["child"]

        [tool.uv.sources]
        child = { path = "packages/child" }
        "#,
    )?;

    // Create the child package with version 0.1.0.
    let child = context.temp_dir.child("packages/child");
    child.create_dir_all()?;
    child.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "child"
        version = "0.1.0"
        requires-python = ">=3.12"

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

    // Lock and sync (installs child v0.1.0).
    uv_snapshot!(context.filters(), context.lock(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    ");

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + child==0.1.0 (from file://[TEMP_DIR]/packages/child)
    ");

    // Now bump the child's version to 0.1.1.
    child.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "child"
        version = "0.1.1"
        requires-python = ">=3.12"

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
        "#,
    )?;

    // Lock again; lockfile should show v0.1.1.
    uv_snapshot!(context.filters(), context.lock(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Updated child v0.1.0 -> v0.1.1
    ");

    // Sync should reinstall child with the new version. Before the fix for #17370,
    // this would incorrectly say "Audited 2 packages" and not reinstall the child package.
    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    Prepared 1 package in [TIME]
    Uninstalled 1 package in [TIME]
    Installed 1 package in [TIME]
     - child==0.1.0 (from file://[TEMP_DIR]/packages/child)
     + child==0.1.1 (from file://[TEMP_DIR]/packages/child)
    ");

    Ok(())
}
