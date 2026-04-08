use std::process::Command;

use assert_fs::prelude::*;
use uv_static::EnvVars;

use uv_test::uv_snapshot;

/// Output from running a `--show-settings` command.
struct ShowSettings {
    stdout: String,
    stderr: String,
}

impl ShowSettings {
    /// Extract a field and its complete (possibly multi-line) value from the Debug
    /// pretty-print output.
    ///
    /// For a simple field like `resolution: Highest`, returns `"resolution: Highest"`.
    /// For a nested field like `upgrade: Upgrade { ... }`, returns the full block with
    /// relative indentation preserved.
    fn extract_field(&self, field_name: &str) -> String {
        let lines: Vec<&str> = self.stdout.lines().collect();
        let prefix = format!("{field_name}: ");
        for (i, line) in lines.iter().enumerate() {
            let stripped = line.trim_start();
            if !stripped.starts_with(&prefix) {
                continue;
            }
            let field_indent = line.len() - stripped.len();

            // Track bracket/paren/brace depth to find end of value.
            let mut depth: i32 = 0;
            let mut result_lines = vec![stripped.to_string()];
            for ch in stripped.chars() {
                match ch {
                    '(' | '{' | '[' => depth += 1,
                    ')' | '}' | ']' => depth -= 1,
                    _ => {}
                }
            }
            // Simple single-line value (no open brackets).
            if depth == 0 {
                let s = stripped.trim_end_matches(',');
                return s.to_string();
            }
            // Multi-line: keep collecting until brackets balance.
            for line in &lines[i + 1..] {
                let next_stripped = line.trim_start();
                if next_stripped.is_empty() {
                    continue;
                }
                let next_indent = line.len() - next_stripped.len();
                let rel_indent = next_indent.saturating_sub(field_indent);
                result_lines.push(format!(
                    "{:indent$}{next_stripped}",
                    "",
                    indent = rel_indent
                ));
                for ch in next_stripped.chars() {
                    match ch {
                        '(' | '{' | '[' => depth += 1,
                        ')' | '}' | ']' => depth -= 1,
                        _ => {}
                    }
                }
                if depth == 0 {
                    break;
                }
            }
            let result = result_lines.join("\n");
            return result.trim_end_matches(',').to_string();
        }
        panic!(
            "Field `{field_name}` not found in settings output.\n\nFull stdout:\n{}",
            self.stdout
        );
    }

    /// Assert that stderr contains the given string.
    #[track_caller]
    fn assert_stderr_contains(&self, expected: &str) {
        assert!(
            self.stderr.contains(expected),
            "Expected stderr to contain:\n{expected}\n\nFull stderr:\n{}",
            self.stderr,
        );
    }

    /// Assert that stderr is empty.
    #[track_caller]
    fn assert_stderr_empty(&self) {
        let trimmed = self.stderr.trim();
        assert!(
            trimmed.is_empty(),
            "Expected stderr to be empty, but got:\n{}",
            self.stderr,
        );
    }
}

/// Assert that a field extracted from settings output matches an inline snapshot.
///
/// Usage:
/// ```ignore
/// assert_field_snapshot!(settings, "resolution", @"resolution: Highest");
/// assert_field_snapshot!(settings, "upgrade", @r#"
///     upgrade: Upgrade {
///         strategy: Packages(
///             {
///                 PackageName(
///                     "sniffio",
///                 ),
///             },
///         ),
///         constraints: {},
///     }
/// "#);
/// ```
macro_rules! assert_field_snapshot {
    ($settings:expr, $field:expr, @$snapshot:literal) => {{
        let extracted = $settings.extract_field($field);
        ::insta::assert_snapshot!(extracted, @$snapshot);
    }};
}

/// Run a `--show-settings` command and return the parsed output.
///
/// Asserts that the command exits successfully.
fn run_show_settings(command: &mut Command) -> ShowSettings {
    let output = command.output().expect("Failed to execute command");
    assert!(
        output.status.success(),
        "Command failed with exit code: {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr),
    );
    ShowSettings {
        stdout: String::from_utf8(output.stdout).expect("stdout is not valid UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("stderr is not valid UTF-8"),
    }
}

/// Add shared arguments to a command.
///
/// In particular, remove any user-defined environment variables and set any machine-specific
/// environment variables to static values.
fn add_shared_args(mut command: Command) -> Command {
    command
        .env(EnvVars::UV_LINK_MODE, "clone")
        .env(EnvVars::UV_CONCURRENT_DOWNLOADS, "50")
        .env(EnvVars::UV_CONCURRENT_BUILDS, "16")
        .env(EnvVars::UV_CONCURRENT_INSTALLS, "8")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env_remove(EnvVars::UV_PYTHON_DOWNLOADS);

    if cfg!(unix) {
        // Avoid locale issues in tests
        command.env(EnvVars::LC_ALL, "C");
    }
    command
}

/// Read from a `uv.toml` file in the current directory.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_uv_toml() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // Resolution should use the lowest direct version, and generate hashes.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Pypi(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_empty();

    // CLI `--resolution=highest` should override the config's `lowest-direct`,
    // but `generate-hashes` from the config should still be honored.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .arg("--resolution=highest"),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: Highest");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");

    // CLI `--no-generate-hashes` should override the config's `generate-hashes = true`.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .arg("--resolution=highest")
            .arg("--no-generate-hashes"),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: Highest");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: false");

    Ok(())
}

/// Read from a `pyproject.toml` file in the current directory.
///
/// We prefer `uv.toml` when both are present, but respect `pyproject.toml` otherwise.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_pyproject_toml() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // Write a `pyproject.toml` file to the directory.
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // With both uv.toml and pyproject.toml, uv.toml settings should take effect.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Pypi(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);

    // Remove the `uv.toml` file.
    fs_err::remove_file(config.path())?;

    // Without uv.toml, defaults should be used (pyproject.toml has no `[tool.uv.pip]` yet).
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: Highest");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: false");

    // Add configuration to the `pyproject.toml` file.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        python-platform = "x86_64-unknown-linux-gnu"
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // pyproject.toml settings should now take effect.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Pypi(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    assert_field_snapshot!(settings, "python_platform", @"
    python_platform: Some(
        X8664UnknownLinuxGnu,
    )
    ");

    Ok(())
}

/// Merge index URLs across configuration.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_index_url() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `pyproject.toml` file to the directory.
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        index-url = "https://test.pypi.org/simple"
        extra-index-url = ["https://pypi.org/simple"]
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Pypi(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "test.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://test.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_empty();

    // Providing an additional index URL on the command-line should be merged with the
    // configuration file.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple"),
    );
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "test.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://test.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: Some(
                Cli,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Pypi(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "test.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://test.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_empty();

    Ok(())
}

/// Allow `--find-links` in configuration files.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_find_links() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `pyproject.toml` file to the directory.
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        no-index = true
        find-links = ["https://download.pytorch.org/whl/torch_stable.html"]
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("tqdm")?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "no_index", @"no_index: true");
    assert_field_snapshot!(settings, "indexes", @"indexes: []");
    assert_field_snapshot!(settings, "flat_index", @r#"
    flat_index: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "download.pytorch.org",
                            ),
                        ),
                        port: None,
                        path: "/whl/torch_stable.html",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://download.pytorch.org/whl/torch_stable.html",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: None,
            format: Flat,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_empty();

    Ok(())
}

/// Merge configuration between the top-level `tool.uv` and the more specific `tool.uv.pip`.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_top_level() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write out to the top-level (`tool.uv`, rather than `tool.uv.pip`).
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv]
        resolution = "lowest-direct"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    settings.assert_stderr_empty();

    // Write out to both the top-level (`tool.uv`) and the pip section (`tool.uv.pip`). The
    // `tool.uv.pip` section should take precedence when combining.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv]
        resolution = "lowest-direct"
        extra-index-url = ["https://test.pypi.org/simple"]

        [tool.uv.pip]
        resolution = "highest"
        extra-index-url = ["https://download.pytorch.org/whl"]
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: Highest");
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "download.pytorch.org",
                            ),
                        ),
                        port: None,
                        path: "/whl",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://download.pytorch.org/whl",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "test.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://test.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_empty();

    // But the command-line should take precedence over both.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .arg("--resolution=lowest-direct"),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "download.pytorch.org",
                            ),
                        ),
                        port: None,
                        path: "/whl",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://download.pytorch.org/whl",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "test.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://test.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_empty();

    Ok(())
}

/// Verify that user configuration is respected.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_user_configuration() -> anyhow::Result<()> {
    let xdg = assert_fs::TempDir::new().expect("Failed to create temp dir");
    let uv = xdg.child("uv");
    let config = uv.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
    "#})?;

    let context = uv_test::test_context!("3.12");

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // Resolution should use the lowest direct version.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .env(EnvVars::XDG_CONFIG_HOME, xdg.path()),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    settings.assert_stderr_empty();

    // Add a local configuration to generate hashes.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r"
        [pip]
        generate-hashes = true
    "})?;

    // Resolution should use the lowest direct version and generate hashes.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .env(EnvVars::XDG_CONFIG_HOME, xdg.path()),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");
    settings.assert_stderr_empty();

    // Add a local configuration to override the user configuration.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "highest"
    "#})?;

    // Resolution should use the highest version.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .env(EnvVars::XDG_CONFIG_HOME, xdg.path()),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: Highest");
    settings.assert_stderr_empty();

    // However, the user-level `tool.uv.pip` settings override the project-level `tool.uv` settings.
    // This is awkward, but we merge the user configuration into the workspace configuration, so
    // the resulting configuration has both `tool.uv.pip.resolution` (from the user configuration)
    // and `tool.uv.resolution` (from the workspace settings), so we choose the former.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        resolution = "highest"
    "#})?;

    // Resolution should use the highest version.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .env(EnvVars::XDG_CONFIG_HOME, xdg.path()),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    settings.assert_stderr_empty();

    Ok(())
}

/// When running a user-level command (like `uv tool install`), we should read user configuration,
/// but ignore project-local configuration.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_tool() -> anyhow::Result<()> {
    // Create a temporary directory to store the user configuration.
    let xdg = assert_fs::TempDir::new().expect("Failed to create temp dir");
    let uv = xdg.child("uv");
    let config = uv.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        resolution = "lowest-direct"
    "#})?;

    let context = uv_test::test_context!("3.12");

    // Add a local configuration to disable build isolation.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r"
        no-build-isolation = true
    "})?;

    // If we're running a user-level command, like `uv tool install`, we should use lowest direct,
    // but retain build isolation (since we ignore the local configuration).
    let mut cmd = add_shared_args(context.tool_install());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .env(EnvVars::XDG_CONFIG_HOME, xdg.path()),
    );
    assert!(settings.stdout.contains("ToolInstallSettings"));
    assert_field_snapshot!(settings, "resolution", @"
    resolution: Some(
        LowestDirect,
    )
    ");
    assert_field_snapshot!(settings, "build_isolation", @"build_isolation: None");
    settings.assert_stderr_empty();

    Ok(())
}

/// Read from a `pyproject.toml` file in the current directory. In this case, the `pyproject.toml`
/// file uses the Poetry schema.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_poetry_toml() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("pyproject.toml");
    config.write_str(indoc::indoc! {r#"
        [tool.poetry]
        name = "project"
        version = "0.1.0"

        [tool.poetry.dependencies]
        python = "^3.10"
        rich = "^13.7.1"

        [build-system]
        requires = ["poetry-core"]
        build-backend = "poetry.core.masonry.api"

        [tool.uv.pip]
        resolution = "lowest-direct"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // Resolution should use the lowest direct version, and generate hashes.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    settings.assert_stderr_empty();

    Ok(())
}

/// Read from both a `uv.toml` and `pyproject.toml` file in the current directory.
///
/// Some fields in `[tool.uv]` are masked by `uv.toml` being defined, and should be warned about.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_both() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // Write a `pyproject.toml` file to the directory
    let config = context.temp_dir.child("pyproject.toml");
    config.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv]
        offline = true
        dev-dependencies = ["pytest"]

        [tool.uv.pip]
        resolution = "highest"
        extra-index-url = ["https://test.pypi.org/simple"]
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // Resolution should succeed, but warn that the `pip` section in `pyproject.toml` is ignored.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Pypi(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_contains("Found both a `uv.toml` file and a `[tool.uv]` section");
    settings.assert_stderr_contains("dev-dependencies");

    Ok(())
}

/// Read from both a `uv.toml` and `pyproject.toml` file in the current directory.
///
/// But the fields `[tool.uv]` defines aren't allowed in `uv.toml` so there's no warning.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_both_special_fields() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // Write a `pyproject.toml` file to the directory
    let config = context.temp_dir.child("pyproject.toml");
    config.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [dependency-groups]
        mygroup = ["iniconfig"]

        [tool.uv]
        dev-dependencies = ["pytest"]

        [tool.uv.dependency-groups]
        mygroup = {requires-python = ">=3.12"}
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // Resolution should succeed, but warn that the `pip` section in `pyproject.toml` is ignored.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");
    settings.assert_stderr_contains("dev-dependencies");

    Ok(())
}

/// Tests that errors when parsing `conflicts` are reported.
#[test]
fn invalid_conflicts() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");
    let pyproject = context.temp_dir.child("pyproject.toml");

    // Write in `pyproject.toml` schema and test the singleton case.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
        requires-python = ">=3.12"

        [tool.uv]
        conflicts = [
            [{extra = "dev"}],
        ]
    "#})?;

    // The file should be rejected for violating the schema.
    uv_snapshot!(context.filters(), add_shared_args(context.lock()), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `pyproject.toml`
      Caused by: TOML parse error at line 7, column 13
      |
    7 | conflicts = [
      |             ^
    Each set of conflicts must have at least two entries, but found only one
    "
    );

    // Now test the empty case.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
        requires-python = ">=3.12"

        [tool.uv]
        conflicts = [[]]
    "#})?;

    // The file should be rejected for violating the schema.
    uv_snapshot!(context.filters(), add_shared_args(context.lock()), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `pyproject.toml`
      Caused by: TOML parse error at line 7, column 13
      |
    7 | conflicts = [[]]
      |             ^^^^
    Each set of conflicts must have at least two entries, but found none
    "
    );

    Ok(())
}

/// Tests that valid `conflicts` are parsed okay.
#[test]
fn valid_conflicts() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");
    let xdg = assert_fs::TempDir::new().expect("Failed to create temp dir");
    let pyproject = context.temp_dir.child("pyproject.toml");

    // Write in `pyproject.toml` schema.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
        requires-python = ">=3.12"

        [tool.uv]
        conflicts = [
            [{extra = "x1"}, {extra = "x2"}],
        ]
    "#})?;
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .env(EnvVars::XDG_CONFIG_HOME, xdg.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    "
    );

    Ok(())
}

/// Read from a `--config-file` command line argument.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_config_file() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` to a temporary location. (Use the cache directory for convenience, since
    // it's already obfuscated in the fixtures.)
    let config_dir = &context.cache_dir;
    let config = config_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    let settings = run_show_settings(
        add_shared_args(context.pip_compile())
            .arg("--show-settings")
            .arg("--config-file")
            .arg(config.path())
            .arg("requirements.in"),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: true");
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Pypi(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: None,
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);
    settings.assert_stderr_empty();

    // Write in `pyproject.toml` schema.
    config.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // The file should be rejected for violating the schema.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("--config-file")
        .arg(config.path())
        .arg("requirements.in"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `[CACHE_DIR]/uv.toml`
      Caused by: TOML parse error at line 1, column 2
      |
    1 | [project]
      |  ^^^^^^^
    unknown field `project`, expected one of `required-version`, `system-certs`, `native-tls`, `offline`, `no-cache`, `cache-dir`, `preview`, `python-preference`, `python-downloads`, `concurrent-downloads`, `concurrent-builds`, `concurrent-installs`, `index`, `index-url`, `extra-index-url`, `no-index`, `find-links`, `index-strategy`, `keyring-provider`, `http-proxy`, `https-proxy`, `no-proxy`, `allow-insecure-host`, `resolution`, `prerelease`, `fork-strategy`, `dependency-metadata`, `config-settings`, `config-settings-package`, `no-build-isolation`, `no-build-isolation-package`, `extra-build-dependencies`, `extra-build-variables`, `exclude-newer`, `exclude-newer-package`, `link-mode`, `compile-bytecode`, `no-sources`, `no-sources-package`, `upgrade`, `upgrade-package`, `reinstall`, `reinstall-package`, `no-build`, `no-build-package`, `no-binary`, `no-binary-package`, `torch-backend`, `python-install-mirror`, `pypy-install-mirror`, `python-downloads-json-url`, `publish-url`, `trusted-publishing`, `check-url`, `add-bounds`, `audit`, `pip`, `cache-keys`, `override-dependencies`, `exclude-dependencies`, `constraint-dependencies`, `build-constraint-dependencies`, `environments`, `required-environments`, `conflicts`, `workspace`, `sources`, `managed`, `package`, `default-groups`, `dependency-groups`, `dev-dependencies`, `build-backend`
    "
    );

    // Write an _actual_ `pyproject.toml`.
    let config = config_dir.child("pyproject.toml");
    config.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
        """#
    })?;

    // The file should be rejected for violating the schema, with a custom warning.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("--config-file")
        .arg(config.path())
        .arg("requirements.in"), @r#"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    warning: The `--config-file` argument expects to receive a `uv.toml` file, not a `pyproject.toml`. If you're trying to run a command from another project, use the `--project` argument instead.
    error: Failed to parse: `[CACHE_DIR]/pyproject.toml`
      Caused by: TOML parse error at line 9, column 3
      |
    9 | ""
      |   ^
    key with no value, expected `=`
    "#
    );

    Ok(())
}

/// Ignore empty `pyproject.toml` files when discovering configuration.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_skip_empty() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Set `lowest-direct` in a `uv.toml`.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
    "#})?;

    let child = context.temp_dir.child("child");
    fs_err::create_dir(&child)?;

    // Create an empty in a `pyproject.toml`.
    let pyproject = child.child("pyproject.toml");
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "child"
        dependencies = [
          "httpx",
        ]
    "#})?;

    // Resolution in `child` should use lowest-direct, skipping the `pyproject.toml`, which lacks a
    // `tool.uv`.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .current_dir(&child),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: LowestDirect");

    // Adding a `tool.uv` section should cause us to ignore the `uv.toml`.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "child"
        dependencies = [
          "httpx",
        ]

        [tool.uv]
    "#})?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .current_dir(&child),
    );
    assert_field_snapshot!(settings, "resolution", @"resolution: Highest");

    Ok(())
}

/// Deserialize an insecure host.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn allow_insecure_host() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        allow-insecure-host = ["google.com", { host = "example.com" }]
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("requirements.in"));
    assert_field_snapshot!(settings, "allow_insecure_host", @r#"
    allow_insecure_host: [
        Host {
            scheme: None,
            host: "google.com",
            port: None,
        },
        Host {
            scheme: None,
            host: "example.com",
            port: None,
        },
    ]
    "#);

    Ok(())
}

/// Prioritize indexes defined across multiple configuration sources.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn index_priority() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [[index]]
        url = "https://file.pypi.org/simple"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("requirements.in")
            .arg("--show-settings")
            .arg("--index-url")
            .arg("https://cli.pypi.org/simple"),
    );
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "cli.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://cli.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Cli,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "file.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://file.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("requirements.in")
            .arg("--show-settings")
            .arg("--default-index")
            .arg("https://cli.pypi.org/simple"),
    );
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "cli.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://cli.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Cli,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "file.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://file.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        index-url = "https://file.pypi.org/simple"
    "#})?;

    // Prefer the `--default-index` from the CLI, and treat it as the default.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("requirements.in")
            .arg("--show-settings")
            .arg("--default-index")
            .arg("https://cli.pypi.org/simple"),
    );
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "cli.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://cli.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Cli,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "file.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://file.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);

    // Prefer the `--index` from the CLI, but treat the index from the file as the default.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("requirements.in")
            .arg("--show-settings")
            .arg("--index")
            .arg("https://cli.pypi.org/simple"),
    );
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "cli.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://cli.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: Some(
                Cli,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "file.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://file.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [[index]]
        url = "https://file.pypi.org/simple"
        default = true
    "#})?;

    // Prefer the `--index-url` from the CLI, and treat it as the default.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("requirements.in")
            .arg("--show-settings")
            .arg("--index-url")
            .arg("https://cli.pypi.org/simple"),
    );
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "cli.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://cli.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Cli,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "file.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://file.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);

    // Prefer the `--extra-index-url` from the CLI, but not as the default.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("requirements.in")
            .arg("--show-settings")
            .arg("--extra-index-url")
            .arg("https://cli.pypi.org/simple"),
    );
    assert_field_snapshot!(settings, "indexes", @r#"
    indexes: [
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "cli.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://cli.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: false,
            origin: Some(
                Cli,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
        Index {
            name: None,
            url: Url(
                VerbatimUrl {
                    url: DisplaySafeUrl {
                        scheme: "https",
                        cannot_be_a_base: false,
                        username: "",
                        password: None,
                        host: Some(
                            Domain(
                                "file.pypi.org",
                            ),
                        ),
                        port: None,
                        path: "/simple",
                        query: None,
                        fragment: None,
                    },
                    given: Some(
                        "https://file.pypi.org/simple",
                    ),
                    expanded: false,
                },
            ),
            explicit: false,
            default: true,
            origin: Some(
                Project,
            ),
            format: Simple,
            publish_url: None,
            authenticate: Auto,
            ignore_error_codes: None,
            cache_control: None,
        },
    ]
    "#);

    Ok(())
}

/// Verify hashes by default.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn verify_hashes() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    let mut cmd = add_shared_args(context.pip_install());
    let settings = run_show_settings(cmd.arg("-r").arg("requirements.in").arg("--show-settings"));
    assert_field_snapshot!(settings, "hash_checking", @"
    hash_checking: Some(
        Verify,
    )
    ");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: false");

    let mut cmd = add_shared_args(context.pip_install());
    let settings = run_show_settings(
        cmd.arg("-r")
            .arg("requirements.in")
            .arg("--require-hashes")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "hash_checking", @"
    hash_checking: Some(
        Require,
    )
    ");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: false");

    let mut cmd = add_shared_args(context.pip_install());
    let settings = run_show_settings(
        cmd.arg("-r")
            .arg("requirements.in")
            .arg("--verify-hashes")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "hash_checking", @"
    hash_checking: Some(
        Verify,
    )
    ");
    assert_field_snapshot!(settings, "generate_hashes", @"generate_hashes: false");

    let mut cmd = add_shared_args(context.pip_install());
    let settings = run_show_settings(
        cmd.arg("-r")
            .arg("requirements.in")
            .arg("--no-verify-hashes")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "hash_checking", @"hash_checking: None");

    let mut cmd = add_shared_args(context.pip_install());
    let settings = run_show_settings(
        cmd.arg("-r")
            .arg("requirements.in")
            .arg("--require-hashes")
            .arg("--no-verify-hashes")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "hash_checking", @"
    hash_checking: Some(
        Require,
    )
    ");

    let mut cmd = add_shared_args(context.pip_install());
    let settings = run_show_settings(
        cmd.arg("-r")
            .arg("requirements.in")
            .arg("--no-verify-hashes")
            .arg("--require-hashes")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "hash_checking", @"
    hash_checking: Some(
        Require,
    )
    ");

    Ok(())
}

/// Test preview feature flagging.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn preview_features() {
    let context = uv_test::test_context!("3.12");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(cmd.arg("--show-settings").arg("--preview"));
    assert_field_snapshot!(settings, "preview", @"
    preview: Preview {
        flags: [
            PythonInstallDefault,
            PythonUpgrade,
            JsonOutput,
            Pylock,
            AddBounds,
            PackageConflicts,
            ExtraBuildDependencies,
            DetectModuleConflicts,
            Format,
            NativeAuth,
            S3Endpoint,
            CacheSize,
            InitProjectFlag,
            WorkspaceMetadata,
            WorkspaceDir,
            WorkspaceList,
            SbomExport,
            AuthHelper,
            DirectPublish,
            TargetWorkspaceDiscovery,
            MetadataJson,
            GcsEndpoint,
            AdjustUlimit,
            SpecialCondaEnvNames,
            RelocatableEnvsDefault,
            PublishRequireNormalized,
            Audit,
            ProjectDirectoryMustExist,
        ],
    }
    ");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("--preview")
            .arg("--no-preview"),
    );
    assert_field_snapshot!(settings, "preview", @"
    preview: Preview {
        flags: [],
    }
    ");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("--preview")
            .arg("--preview-features")
            .arg("python-install-default"),
    );
    assert_field_snapshot!(settings, "preview", @"
    preview: Preview {
        flags: [
            PythonInstallDefault,
            PythonUpgrade,
            JsonOutput,
            Pylock,
            AddBounds,
            PackageConflicts,
            ExtraBuildDependencies,
            DetectModuleConflicts,
            Format,
            NativeAuth,
            S3Endpoint,
            CacheSize,
            InitProjectFlag,
            WorkspaceMetadata,
            WorkspaceDir,
            WorkspaceList,
            SbomExport,
            AuthHelper,
            DirectPublish,
            TargetWorkspaceDiscovery,
            MetadataJson,
            GcsEndpoint,
            AdjustUlimit,
            SpecialCondaEnvNames,
            RelocatableEnvsDefault,
            PublishRequireNormalized,
            Audit,
            ProjectDirectoryMustExist,
        ],
    }
    ");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("--preview-features")
            .arg("python-install-default,python-upgrade"),
    );
    assert_field_snapshot!(settings, "preview", @"
    preview: Preview {
        flags: [
            PythonInstallDefault,
            PythonUpgrade,
        ],
    }
    ");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("--preview-features")
            .arg("python-install-default")
            .arg("--preview-feature")
            .arg("python-upgrade"),
    );
    assert_field_snapshot!(settings, "preview", @"
    preview: Preview {
        flags: [
            PythonInstallDefault,
            PythonUpgrade,
        ],
    }
    ");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("--preview-features")
            .arg("python-install-default")
            .arg("--no-preview"),
    );
    assert_field_snapshot!(settings, "preview", @"
    preview: Preview {
        flags: [],
    }
    ");
}

#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn system_certs_cli_aliases_override_env() {
    let context = uv_test::test_context!("3.12");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("--no-native-tls")
            .env(EnvVars::UV_SYSTEM_CERTS, "1"),
    );
    assert_field_snapshot!(settings, "system_certs", @"system_certs: false");

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("--native-tls")
            .env(EnvVars::UV_SYSTEM_CERTS, "0"),
    );
    assert_field_snapshot!(settings, "system_certs", @"system_certs: true");
}

#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn system_certs_config_aliases() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let config = context.temp_dir.child("uv.toml");
    config.write_str("system-certs = true\n")?;

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(cmd.arg("--show-settings"));
    assert_field_snapshot!(settings, "system_certs", @"system_certs: true");

    config.write_str(indoc::indoc! {r"
        system-certs = false
        native-tls = true
    "})?;

    let mut cmd = add_shared_args(context.version());
    let settings = run_show_settings(cmd.arg("--show-settings"));
    assert_field_snapshot!(settings, "system_certs", @"system_certs: false");

    Ok(())
}

/// Track the interactions between `upgrade` and `upgrade-package` across the `uv pip` CLI and a
/// configuration file.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn upgrade_pip_cli_config_interaction() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // `--no-upgrade` overrides `--upgrade-package`.
    // TODO(charlie): This should mark `sniffio` for upgrade, but it doesn't.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--no-upgrade")
            .arg("--upgrade-package")
            .arg("sniffio")
            .arg("--show-settings")
            .arg("requirements.in"),
    );
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "sniffio",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r"
        [pip]
        upgrade = false
    "})?;

    // Despite `upgrade = false` in the configuration file, we should mark `idna` for upgrade.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--upgrade-package")
            .arg("idna")
            .arg("--show-settings")
            .arg("requirements.in"),
    );
    assert_field_snapshot!(settings, "upgrade", @"
    upgrade: Upgrade {
        strategy: None,
        constraints: {},
    }
    ");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r"
        [pip]
        upgrade = true
    "})?;

    // Despite `--upgrade-package idna` in the command line, we should upgrade all packages.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--upgrade-package")
            .arg("idna")
            .arg("--show-settings")
            .arg("requirements.in"),
    );
    assert_field_snapshot!(settings, "upgrade", @"
    upgrade: Upgrade {
        strategy: All,
        constraints: {},
    }
    ");

    // Write a `uv.toml` file to the directory.
    config.write_str(indoc::indoc! {r#"
        [pip]
        upgrade-package = ["idna"]
    "#})?;

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should disable upgrades.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--upgrade-package")
            .arg("idna")
            .arg("--show-settings")
            .arg("requirements.in"),
    );
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "idna",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should enable all upgrades.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--no-upgrade")
            .arg("--show-settings")
            .arg("requirements.in"),
    );
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "idna",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    // Mark both `sniffio` and `idna` for upgrade.
    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--upgrade")
            .arg("--show-settings")
            .arg("requirements.in"),
    );
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "idna",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    Ok(())
}

/// Track the interactions between `upgrade` and `upgrade-package` across the project CLI and a
/// configuration file.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn upgrade_project_cli_config_interaction() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]
    "#})?;

    // `--no-upgrade` overrides `--upgrade-package`.
    // TODO(charlie): This should mark `sniffio` for upgrade, but it doesn't.
    let mut cmd = add_shared_args(context.lock());
    let settings = run_show_settings(
        cmd.arg("--no-upgrade")
            .arg("--upgrade-package")
            .arg("sniffio")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "sniffio",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    // Add `upgrade = false` to the configuration file.
    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]

        [tool.uv]
        upgrade = false
    "#})?;

    // Despite `upgrade = false` in the configuration file, we should mark `idna` for upgrade.
    let mut cmd = add_shared_args(context.lock());
    let settings = run_show_settings(
        cmd.arg("--upgrade-package")
            .arg("idna")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "upgrade", @"
    upgrade: Upgrade {
        strategy: None,
        constraints: {},
    }
    ");

    // Add `upgrade = true` to the configuration file.
    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]

        [tool.uv]
        upgrade = true
    "#})?;

    // Despite `--upgrade-package idna` on the CLI, we should upgrade all packages.
    let mut cmd = add_shared_args(context.lock());
    let settings = run_show_settings(
        cmd.arg("--upgrade-package")
            .arg("idna")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "upgrade", @"
    upgrade: Upgrade {
        strategy: All,
        constraints: {},
    }
    ");

    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]

        [tool.uv]
        upgrade-package = ["idna"]
    "#})?;

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should disable upgrades.
    let mut cmd = add_shared_args(context.lock());
    let settings = run_show_settings(cmd.arg("--no-upgrade").arg("--show-settings"));
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "idna",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should enable all upgrades.
    let mut cmd = add_shared_args(context.lock());
    let settings = run_show_settings(cmd.arg("--upgrade").arg("--show-settings"));
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "idna",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    // Mark both `sniffio` and `idna` for upgrade.
    let mut cmd = add_shared_args(context.lock());
    let settings = run_show_settings(
        cmd.arg("--upgrade-package")
            .arg("idna")
            .arg("--show-settings"),
    );
    assert_field_snapshot!(settings, "upgrade", @r#"
    upgrade: Upgrade {
        strategy: Packages(
            {
                PackageName(
                    "idna",
                ),
            },
        ),
        constraints: {},
    }
    "#);

    Ok(())
}

/// Test that setting `build-isolation = true` in pyproject.toml followed by
/// `--no-build-isolation-package numpy` on the CLI disables build isolation for `numpy`.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn build_isolation_override() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to disable build isolation.
    let uv_toml = context.temp_dir.child("uv.toml");
    uv_toml.write_str(indoc::indoc! {r"
        no-build-isolation = true
    "})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("numpy")?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .arg("--no-build-isolation-package")
            .arg("numpy"),
    );
    assert_field_snapshot!(settings, "build_isolation", @"build_isolation: Shared");

    // Now enable build isolation for all packages except `numpy`.
    uv_toml.write_str(indoc::indoc! {r"
        no-build-isolation = false
    "})?;

    let mut cmd = add_shared_args(context.pip_compile());
    let settings = run_show_settings(
        cmd.arg("--show-settings")
            .arg("requirements.in")
            .arg("--no-build-isolation-package")
            .arg("numpy"),
    );
    assert_field_snapshot!(settings, "build_isolation", @r#"
    build_isolation: SharedPackage(
        [
            PackageName(
                "numpy",
            ),
        ],
    )
    "#);

    Ok(())
}
