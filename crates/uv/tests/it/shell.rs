use assert_fs::prelude::*;

use crate::common::{TestContext, uv_snapshot};

/// `uv shell` outside a project should fail with an error.
#[test]
fn shell_no_project() {
    let context = TestContext::new("3.12");

    // Remove the pyproject.toml that TestContext may have created.
    let _ = std::fs::remove_file(context.temp_dir.child("pyproject.toml").path());

    uv_snapshot!(context.filters(), context.shell(), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No `pyproject.toml` found in current directory or any parent directory
    ");
}

/// `uv shell` with `--frozen` outside a project should fail.
#[test]
fn shell_frozen_no_project() {
    let context = TestContext::new("3.12");

    let _ = std::fs::remove_file(context.temp_dir.child("pyproject.toml").path());

    uv_snapshot!(context.filters(), context.shell().arg("--frozen"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No `pyproject.toml` found in current directory or any parent directory
    ");
}

/// `uv shell --env-file` with a missing file should fail.
#[test]
fn shell_env_file_missing() {
    let context = TestContext::new("3.12");

    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(
            r#"
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
"#,
        )
        .unwrap();

    uv_snapshot!(context.filters(), context.shell().arg("--env-file").arg("missing.env"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No environment file found at: `missing.env`
    ");
}

/// `uv shell --with` outside a project should fail with a project error.
#[test]
fn shell_with_no_project() {
    let context = TestContext::new("3.12");

    let _ = std::fs::remove_file(context.temp_dir.child("pyproject.toml").path());

    uv_snapshot!(context.filters(), context.shell().arg("--with").arg("requests"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No `pyproject.toml` found in current directory or any parent directory
    ");
}

/// `uv shell --isolated` outside a project should fail with a project error.
#[test]
fn shell_isolated_no_project() {
    let context = TestContext::new("3.12");

    let _ = std::fs::remove_file(context.temp_dir.child("pyproject.toml").path());

    uv_snapshot!(context.filters(), context.shell().arg("--isolated"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    warning: The `--isolated` flag is deprecated and has no effect. Instead, use `--no-config` to prevent uv from discovering configuration files.
    error: No `pyproject.toml` found in current directory or any parent directory
    ");
}

/// `uv shell --show-resolution` is accepted.
#[test]
fn shell_show_resolution_flag_accepted() {
    let context = TestContext::new("3.12");

    let _ = std::fs::remove_file(context.temp_dir.child("pyproject.toml").path());

    uv_snapshot!(context.filters(), context.shell().arg("--show-resolution"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No `pyproject.toml` found in current directory or any parent directory
    ");
}
