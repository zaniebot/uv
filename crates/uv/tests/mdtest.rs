//! Markdown-based integration tests for uv.
//!
//! This test runner uses the `uv-mdtest` crate to run tests defined in markdown files.
//! Tests are located in `test/uv/` at the workspace root.
//!
//! Each section in a markdown file becomes a separate test, allowing full parallelism
//! with nextest.

#![expect(clippy::print_stderr)]

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use fs_err as fs;
use libtest_mimic::{Arguments, Failed, Trial};
use regex::Regex;
use walkdir::WalkDir;

use serde::Deserialize;
use uv_mdtest::{MarkdownTestFile, SnapshotMode, SnapshotUpdater};
use uv_test::{READ_ONLY_GITHUB_TOKEN, TestContext, decode_token, get_bin};

/// Uv-specific test configuration, deserialized from the `#! mdtest` TOML blocks.
/// The framework merges the raw TOML; we deserialize the merged result here.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
struct UvTestConfig {
    environment: UvEnvironmentConfig,
    filters: UvFilterConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
struct UvEnvironmentConfig {
    #[serde(alias = "python-version")]
    python_versions: PythonVersions,
    managed_python_versions: PythonVersions,
    exclude_newer: Option<String>,
    http_timeout: Option<String>,
    concurrent_installs: Option<String>,
    env: HashMap<String, String>,
    env_remove: Vec<String>,
    create_venv: Option<bool>,
    required_features: Vec<String>,
    // These are handled by the framework but still present in the raw TOML,
    // so we need to accept them during deserialization.
    #[serde(default)]
    target_os: Option<toml::Value>,
    #[serde(default)]
    target_family: Option<toml::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
struct UvFilterConfig {
    counts: bool,
    exe_suffix: bool,
    python_names: bool,
    virtualenv_bin: bool,
    python_install_bin: bool,
    python_sources: bool,
    pyvenv_cfg: bool,
    link_mode_warning: bool,
    not_executable: bool,
    python_keys: bool,
    latest_python_versions: bool,
    compiled_file_count: bool,
    cyclonedx: bool,
    collapse_whitespace: bool,
    cache_size: bool,
    cache_entry: bool,
    missing_file_error: bool,
}

/// Python versions specification.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum PythonVersions {
    #[default]
    Default,
    None,
    Only(Vec<String>),
}

impl<'de> Deserialize<'de> for PythonVersions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = PythonVersions;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("string or array of strings")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(PythonVersions::Only(vec![v.to_string()]))
            }
            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut versions = Vec::new();
                while let Some(s) = seq.next_element::<String>()? {
                    versions.push(s);
                }
                if versions.is_empty() {
                    Ok(PythonVersions::None)
                } else {
                    Ok(PythonVersions::Only(versions))
                }
            }
        }
        deserializer.deserialize_any(V)
    }
}

/// Deserialize a test's raw TOML config into our uv-specific struct.
fn parse_uv_config(test: &uv_mdtest::MarkdownTest) -> UvTestConfig {
    match test.config.raw.clone().try_into() {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "Warning: failed to deserialize uv config for '{}': {e}",
                test.name
            );
            UvTestConfig::default()
        }
    }
}

/// Convert a test name to a URL-friendly slug (like markdown anchors).
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn main() {
    let args = Arguments::from_args();
    let snapshot_mode = SnapshotMode::from_env();

    // Create a shared snapshot updater for batch updates
    let updater = Arc::new(SnapshotUpdater::new());

    // Find all markdown test files
    // mdtest files live in the workspace root's test/uv/ directory
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/
        .and_then(Path::parent) // workspace root
        .expect("Failed to find workspace root");
    let test_dir = workspace_root.join("test/uv");
    let mut trials = Vec::new();

    for entry in WalkDir::new(&test_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path().to_path_buf();
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));

        let test_file = match MarkdownTestFile::parse(path.clone(), &source) {
            Ok(f) => f,
            Err(uv_mdtest::ParseError::NoTests) => continue,
            Err(e) => panic!("Failed to parse {}: {e}", path.display()),
        };

        // Get relative path for display
        let relative_path = path
            .strip_prefix(&test_dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        // Build list of enabled Cargo features (checked at compile time)
        // These mirror the Cargo features in Cargo.toml and gate which tests can run
        let mut enabled_features: Vec<String> = Vec::new();
        if cfg!(feature = "pypi") {
            enabled_features.push("pypi".to_string());
        }
        if cfg!(feature = "git") {
            enabled_features.push("git".to_string());
        }
        if cfg!(feature = "git-lfs") {
            enabled_features.push("git-lfs".to_string());
        }
        if cfg!(feature = "github") {
            enabled_features.push("github".to_string());
        }
        if cfg!(feature = "python") {
            enabled_features.push("python".to_string());
        }
        if cfg!(feature = "python-managed") {
            enabled_features.push("python-managed".to_string());
        }
        if cfg!(feature = "python-patch") {
            enabled_features.push("python-patch".to_string());
        }
        if cfg!(feature = "crates-io") {
            enabled_features.push("crates-io".to_string());
        }
        if cfg!(feature = "r2") {
            enabled_features.push("r2".to_string());
        }

        // Create one trial per test (section)
        for test in test_file.tests {
            let test_name = format!("{}#{}", relative_path, slugify(&test.name));
            let path = path.clone();
            let should_run = test.should_run(&enabled_features);
            let test = Arc::new(test);
            let updater = Arc::clone(&updater);
            let workspace_root = workspace_root.to_path_buf();

            let mut trial = Trial::test(test_name, move || {
                run_single_test(&path, &test, snapshot_mode, &updater, &workspace_root)
            });

            // Skip tests that don't match the current platform (target-os, target-family)
            if !should_run {
                trial = trial.with_ignored_flag(true);
            }

            trials.push(trial);
        }
    }

    let conclusion = libtest_mimic::run(&args, trials);

    // Commit all snapshot updates after tests complete
    if snapshot_mode == SnapshotMode::Update {
        // Try to unwrap the Arc - if there are still references, wait isn't possible
        // In practice, all test closures should be done at this point
        match Arc::try_unwrap(updater) {
            Ok(updater) => match updater.commit() {
                Ok(updated_files) => {
                    for file in updated_files {
                        eprintln!("Updated snapshots in: {}", file.display());
                    }
                }
                Err(e) => {
                    eprintln!("Failed to commit snapshot updates: {e}");
                }
            },
            Err(_) => {
                eprintln!("Warning: Could not commit snapshots - updater still has references");
            }
        }
    }

    conclusion.exit();
}

/// Run a single markdown test section.
fn run_single_test(
    path: &Path,
    test: &uv_mdtest::MarkdownTest,
    snapshot_mode: SnapshotMode,
    updater: &SnapshotUpdater,
    workspace_root: &Path,
) -> Result<(), Failed> {
    let uv_config = parse_uv_config(test);

    // Get the Python versions from test config, combining managed and regular versions
    let managed_versions: Vec<String> = match &uv_config.environment.managed_python_versions {
        PythonVersions::Default | PythonVersions::None => vec![],
        PythonVersions::Only(versions) => versions.clone(),
    };
    let base_versions: Vec<String> = match &uv_config.environment.python_versions {
        PythonVersions::Default => vec!["3.12".to_string()],
        PythonVersions::None => vec![],
        PythonVersions::Only(versions) => versions.clone(),
    };
    // Combine managed + base versions, with managed first (for preference order)
    // Deduplicate in case a version appears in both lists
    let mut python_versions = managed_versions.clone();
    for v in base_versions {
        if !python_versions.contains(&v) {
            python_versions.push(v);
        }
    }
    let version_strs: Vec<&str> = python_versions
        .iter()
        .map(std::string::String::as_str)
        .collect();

    // Create a TestContext for this test - this handles all the proper setup
    // Use new_with_versions_and_bin to avoid automatic venv creation, then conditionally create it
    let mut context = TestContext::new_with_versions_and_bin(&version_strs, get_bin!());
    if uv_config.environment.create_venv.unwrap_or(true) {
        context.reset_venv();
    }

    // Apply environment options from test config
    if let Some(exclude_newer) = &uv_config.environment.exclude_newer {
        context = context.with_exclude_newer(exclude_newer);
    }
    if let Some(http_timeout) = &uv_config.environment.http_timeout {
        context = context.with_http_timeout(http_timeout);
    }
    if let Some(concurrent_installs) = &uv_config.environment.concurrent_installs {
        context = context.with_concurrent_installs(concurrent_installs);
    }

    // Set up git-lfs config if the test requires it
    if uv_config
        .environment
        .required_features
        .contains(&"git-lfs".to_string())
    {
        context = context.with_git_lfs_config();
    }

    // Apply custom environment variables from test config
    for (key, value) in &uv_config.environment.env {
        context = context.with_env(key, value);
    }

    // Mark certain Python versions as managed for tests that need to distinguish
    // between managed and system Python installations
    if !managed_versions.is_empty() {
        context = context.with_versions_as_managed(
            &managed_versions
                .iter()
                .map(std::string::String::as_str)
                .collect::<Vec<_>>(),
        );
    }

    // Remove environment variables from test config
    for key in &uv_config.environment.env_remove {
        context = context.with_env_remove(key);
    }

    // Apply filter options from test config
    let filters_config = &uv_config.filters;
    if filters_config.counts {
        context = context.with_filtered_counts();
    }
    if filters_config.exe_suffix {
        context = context.with_filtered_exe_suffix();
    }
    if filters_config.python_names {
        context = context.with_filtered_python_names();
    }
    if filters_config.virtualenv_bin {
        context = context.with_filtered_virtualenv_bin();
    }
    if filters_config.python_install_bin {
        context = context.with_filtered_python_install_bin();
    }
    if filters_config.python_sources {
        context = context.with_filtered_python_sources();
    }
    if filters_config.pyvenv_cfg {
        context = context.with_pyvenv_cfg_filters();
    }
    if filters_config.link_mode_warning {
        context = context.with_filtered_link_mode_warning();
    }
    if filters_config.not_executable {
        context = context.with_filtered_not_executable();
    }
    if filters_config.python_keys {
        context = context.with_filtered_python_keys();
    }
    if filters_config.latest_python_versions {
        context = context.with_filtered_latest_python_versions();
    }
    if filters_config.compiled_file_count {
        context = context.with_filtered_compiled_file_count();
    }
    if filters_config.cyclonedx {
        context = context.with_cyclonedx_filters();
    }
    if filters_config.collapse_whitespace {
        context = context.with_collapsed_whitespace();
    }
    if filters_config.cache_size {
        context = context.with_filtered_cache_size();
    }
    if filters_config.cache_entry {
        context = context.with_filtered_cache_entry();
    }
    if filters_config.missing_file_error {
        context = context.with_filtered_missing_file_error();
    }

    // Build filters from TestContext
    let context_filters = context.filters();
    let mut filters: Vec<(Regex, String)> = Vec::new();
    for (pattern, replacement) in context_filters {
        if let Ok(regex) = Regex::new(pattern) {
            filters.push((regex, replacement.to_string()));
        }
    }

    // Build a command builder that uses TestContext
    // Note: File creation is now handled by the runner in document order
    let temp_dir_path = context.temp_dir.path().to_path_buf();
    let command_builder = |cmd_str: &str| -> Command {
        // Check if this command needs to be run through a shell
        // Shell operators: &&, ||, ;, |, >, <, etc.
        // Environment variable assignments at start: VAR=value command
        let needs_shell = cmd_str.contains("&&")
            || cmd_str.contains("||")
            || cmd_str.contains(';')
            || cmd_str.contains('|')
            || cmd_str.contains('>')
            || cmd_str.contains('<')
            || cmd_str
                .split_whitespace()
                .next()
                .is_some_and(|first| first.contains('='));

        if needs_shell {
            let mut command = Command::new("sh");
            command.arg("-c").arg(cmd_str);
            context.add_shared_env(&mut command, true);
            return command;
        }

        // Parse the command using shlex to properly handle quoted strings
        let Some(parts) = shlex::split(cmd_str) else {
            return Command::new("false");
        };
        if parts.is_empty() {
            return Command::new("false");
        }

        let cmd_name = &parts[0];
        let args = &parts[1..];

        // Use TestContext's command method for uv commands
        if cmd_name == "uv" {
            let mut command = context.command();
            command.args(args);
            command
        } else if cmd_name == "rm" {
            // Handle rm command using Rust implementation for cross-platform support
            // Create a fake command that will execute via our rm implementation
            // We'll use a custom script approach to integrate with the test runner
            let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            match uv_mdtest::run_rm_command(&args_vec, &temp_dir_path) {
                Ok(output) => {
                    // Parse the output to determine success/failure
                    let success = output.contains("success: true");
                    let exit_code_line = output.lines().find(|l| l.starts_with("exit_code:"));
                    let exit_code: i32 = exit_code_line
                        .and_then(|l| l.split(':').nth(1))
                        .and_then(|s| s.trim().parse().ok())
                        .unwrap_or(if success { 0 } else { 1 });

                    // Extract stdout and stderr from output
                    let parts: Vec<&str> = output.split("----- stdout -----").collect();
                    let stdout_and_stderr = parts.get(1).unwrap_or(&"");
                    let stderr_parts: Vec<&str> =
                        stdout_and_stderr.split("----- stderr -----").collect();
                    let stdout = stderr_parts.first().unwrap_or(&"").trim();
                    let stderr = stderr_parts.get(1).unwrap_or(&"").trim();

                    // Create a synthetic command that outputs the result
                    // This is a workaround to integrate rm output with the test framework
                    #[cfg(unix)]
                    let mut command = Command::new("sh");
                    #[cfg(windows)]
                    let mut command = Command::new("cmd");

                    #[cfg(unix)]
                    {
                        let script = format!(
                            "printf '%s' '{}'; printf '%s' '{}' >&2; exit {}",
                            stdout.replace('\'', "'\\''"),
                            stderr.replace('\'', "'\\''"),
                            exit_code
                        );
                        command.args(["-c", &script]);
                    }
                    #[cfg(windows)]
                    {
                        let script = format!(
                            "@echo off & echo {} & echo {} 1>&2 & exit {}",
                            stdout, stderr, exit_code
                        );
                        command.args(["/c", &script]);
                    }

                    command
                }
                Err(_e) => {
                    // If rm fails, return a command that will fail
                    Command::new("false")
                }
            }
        } else {
            // For non-uv commands, only set up the environment (not uv-specific args)
            let mut command = Command::new(cmd_name);
            command.args(args);
            context.add_shared_env(&mut command, true);
            command
        }
    };

    // Build substitutions map for variable substitution in commands
    let mut substitutions = HashMap::new();

    // Add built-in platform-specific variables
    if cfg!(unix) {
        substitutions.insert("VENV_BIN".to_string(), "bin".to_string());
        substitutions.insert("PATH_SEP".to_string(), ":".to_string());
        substitutions.insert("EXE_SUFFIX".to_string(), String::new());
    } else if cfg!(windows) {
        substitutions.insert("VENV_BIN".to_string(), "Scripts".to_string());
        substitutions.insert("PATH_SEP".to_string(), ";".to_string());
        substitutions.insert("EXE_SUFFIX".to_string(), ".exe".to_string());
    }

    // Always provide workspace root for referencing test fixtures
    substitutions.insert(
        "WORKSPACE".to_string(),
        workspace_root.to_string_lossy().to_string(),
    );

    // Provide GitHub token if the test requires github access
    if uv_config
        .environment
        .required_features
        .contains(&"github".to_string())
    {
        let token = decode_token(READ_ONLY_GITHUB_TOKEN);
        substitutions.insert("GITHUB_TOKEN".to_string(), token);
    }

    // Run the test using the command builder
    let result = test
        .run_with_command_builder(
            context.temp_dir.path(),
            &filters,
            &substitutions,
            command_builder,
        )
        .map_err(|e| {
            Failed::from(format!(
                "Test execution failed at {}:{}: {}",
                path.display(),
                test.line_number,
                e
            ))
        })?;

    if !result.passed {
        if let Some(mismatch) = &result.mismatch {
            if snapshot_mode == SnapshotMode::Update {
                // Queue the snapshot update (will be committed after all tests)
                updater.add(path, mismatch);
            } else {
                // Return the mismatch as a failure
                return Err(Failed::from(format!(
                    "Test failed at {}:{}\n\n{}",
                    path.display(),
                    test.line_number,
                    mismatch.format()
                )));
            }
        }
    }

    Ok(())
}
