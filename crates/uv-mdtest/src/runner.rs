//! Test runner for markdown tests.
//!
//! This module provides the logic for executing markdown tests. The actual
//! integration with the test framework happens in the test entry point.

use std::collections::HashMap;
use std::io::Read;
#[cfg(unix)]
use std::os::unix::io::FromRawFd;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

use fs_err as fs;
use regex::Regex;
use thiserror::Error;

pub use crate::types::MarkdownTest;
use crate::types::{AssertKind, TestStep, TreeConfig, TreeCreation, TreeEntry};

/// Errors that can occur during test execution.
#[derive(Debug, Error)]
pub enum RunError {
    #[error("Failed to create directory {path}: {source}")]
    CreateDir {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to write file {path}: {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to read file {path}: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to execute command: {source}")]
    ExecuteCommand { source: std::io::Error },

    #[error("Command output mismatch at line {line}")]
    OutputMismatch { line: usize },

    #[error("File snapshot mismatch for {path} at line {line}")]
    SnapshotMismatch { path: PathBuf, line: usize },

    #[error("Failed to create symlink {path}: {source}")]
    CreateSymlink {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to copy from {src} to {dest}")]
    CopyFailed {
        src: PathBuf,
        dest: PathBuf,
        #[source]
        err: std::io::Error,
    },

    #[error("{0}")]
    Custom(String),
}

impl TreeCreation {
    /// Create the directory structure defined by this tree.
    ///
    /// Creates directories, empty files, and symlinks as specified in the entries.
    pub fn create(&self, base_dir: &Path) -> Result<(), RunError> {
        for entry in &self.entries {
            match entry {
                TreeEntry::Directory { path } => {
                    let full_path = base_dir.join(path);
                    fs::create_dir_all(&full_path).map_err(|e| RunError::CreateDir {
                        path: full_path,
                        source: e,
                    })?;
                }
                TreeEntry::File { path } => {
                    let full_path = base_dir.join(path);
                    // Ensure parent directory exists
                    if let Some(parent) = full_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| RunError::CreateDir {
                            path: parent.to_path_buf(),
                            source: e,
                        })?;
                    }
                    // Create empty file
                    fs::write(&full_path, "").map_err(|e| RunError::WriteFile {
                        path: full_path,
                        source: e,
                    })?;
                }
                TreeEntry::Symlink { path, target } => {
                    let full_path = base_dir.join(path);
                    // Ensure parent directory exists
                    if let Some(parent) = full_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| RunError::CreateDir {
                            path: parent.to_path_buf(),
                            source: e,
                        })?;
                    }
                    // Create symlink (platform-specific)
                    #[cfg(unix)]
                    {
                        fs_err::os::unix::fs::symlink(target, &full_path).map_err(|e| {
                            RunError::CreateSymlink {
                                path: full_path.clone(),
                                source: e,
                            }
                        })?;
                    }
                    #[cfg(windows)]
                    {
                        // On Windows, we need to determine if target is a directory or file
                        // For simplicity, try directory first, then file
                        let target_full = base_dir.join(target);
                        if target_full.is_dir() {
                            std::os::windows::fs::symlink_dir(target, &full_path).map_err(|e| {
                                RunError::CreateSymlink {
                                    path: full_path.clone(),
                                    source: e,
                                }
                            })?;
                        } else {
                            std::os::windows::fs::symlink_file(target, &full_path).map_err(
                                |e| RunError::CreateSymlink {
                                    path: full_path.clone(),
                                    source: e,
                                },
                            )?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

/// Result of running a single test.
#[derive(Debug)]
pub struct TestResult {
    /// Name of the test.
    pub name: String,
    /// Whether the test passed.
    pub passed: bool,
    /// Any mismatch details if the test failed.
    pub mismatch: Option<Mismatch>,
}

/// Details about a test mismatch.
#[derive(Debug)]
pub struct Mismatch {
    /// Kind of mismatch.
    pub kind: MismatchKind,
    /// Expected content.
    pub expected: String,
    /// Actual content.
    pub actual: String,
    /// Line number in the markdown source.
    pub line: usize,
}

/// Kind of mismatch.
#[derive(Debug)]
pub enum MismatchKind {
    /// Command output didn't match.
    CommandOutput { command: String },
    /// File snapshot didn't match.
    FileSnapshot { path: PathBuf },
    /// Content assertion failed.
    ContentAssertion { path: PathBuf, kind: AssertKind },
    /// Tree snapshot didn't match.
    TreeSnapshot,
}

impl MarkdownTest {
    /// Run this test using a custom command builder.
    ///
    /// This allows integration with external test frameworks like `TestContext`.
    pub fn run_with_command_builder<F>(
        &self,
        test_dir: &Path,
        filters: &[(Regex, String)],
        substitutions: &HashMap<String, String>,
        command_builder: F,
    ) -> Result<TestResult, RunError>
    where
        F: Fn(&str) -> Command,
    {
        let command_runner = |cmd_str: &str, working_dir: &Path| {
            run_command_with_builder(cmd_str, working_dir, substitutions, &command_builder)
        };
        let filter_applier = |output: String| apply_filters(filters, output);

        self.run_steps(test_dir, substitutions, command_runner, filter_applier)
    }

    /// Execute test steps in document order.
    ///
    /// This is the core step processor shared by both `run()` and `run_with_command_builder()`.
    fn run_steps<R, A>(
        &self,
        test_dir: &Path,
        substitutions: &HashMap<String, String>,
        run_cmd: R,
        apply_filter: A,
    ) -> Result<TestResult, RunError>
    where
        R: Fn(&str, &Path) -> Result<String, RunError>,
        A: Fn(String) -> String,
    {
        for step in &self.steps {
            match step {
                TestStep::WriteFile(file) => {
                    let file_path = test_dir.join(&file.path);
                    if let Some(parent) = file_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| RunError::CreateDir {
                            path: parent.to_path_buf(),
                            source: e,
                        })?;
                    }
                    fs::write(&file_path, &file.content).map_err(|e| RunError::WriteFile {
                        path: file_path,
                        source: e,
                    })?;
                }
                TestStep::CreateTree(tree) => {
                    tree.create(test_dir)?;
                }
                TestStep::CopyFrom(copy) => {
                    // Apply variable substitutions to the source path
                    let resolved_source = substitute_vars(&copy.source, substitutions);
                    let source_path = PathBuf::from(&resolved_source);
                    let dest_path = test_dir.join(&copy.dest);

                    // Ensure parent directory exists
                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| RunError::CreateDir {
                            path: parent.to_path_buf(),
                            source: e,
                        })?;
                    }

                    // Copy file or directory
                    copy_recursive(&source_path, &dest_path)?;
                }
                TestStep::RunCommand(cmd) => {
                    let working_dir = match &cmd.working_dir {
                        Some(rel_dir) => test_dir.join(rel_dir),
                        None => test_dir.to_path_buf(),
                    };
                    // Create the working directory if it doesn't exist
                    if !working_dir.exists() {
                        fs::create_dir_all(&working_dir).map_err(|e| RunError::CreateDir {
                            path: working_dir.clone(),
                            source: e,
                        })?;
                    }
                    let result = run_cmd(&cmd.command, &working_dir)?;
                    let filtered_output = apply_filter(result);

                    if filtered_output.trim() != cmd.expected_output.trim() {
                        return Ok(TestResult {
                            name: self.name.clone(),
                            passed: false,
                            mismatch: Some(Mismatch {
                                kind: MismatchKind::CommandOutput {
                                    command: cmd.command.clone(),
                                },
                                expected: cmd.expected_output.clone(),
                                actual: filtered_output,
                                line: cmd.line_number,
                            }),
                        });
                    }
                }
                TestStep::CheckFileSnapshot(snapshot) => {
                    let file_path = test_dir.join(&snapshot.path);
                    let actual_content =
                        fs::read_to_string(&file_path).map_err(|e| RunError::ReadFile {
                            path: file_path.clone(),
                            source: e,
                        })?;
                    let filtered_content = apply_filter(actual_content);

                    if filtered_content.trim() != snapshot.expected_content.trim() {
                        return Ok(TestResult {
                            name: self.name.clone(),
                            passed: false,
                            mismatch: Some(Mismatch {
                                kind: MismatchKind::FileSnapshot {
                                    path: snapshot.path.clone(),
                                },
                                expected: snapshot.expected_content.clone(),
                                actual: filtered_content,
                                line: snapshot.line_number,
                            }),
                        });
                    }
                }
                TestStep::CheckContentAssertion(assertion) => {
                    let file_path = test_dir.join(&assertion.path);
                    let actual_content =
                        fs::read_to_string(&file_path).map_err(|e| RunError::ReadFile {
                            path: file_path.clone(),
                            source: e,
                        })?;

                    let assertion_failed = match assertion.kind {
                        AssertKind::Contains => !actual_content.contains(&assertion.expected),
                    };

                    if assertion_failed {
                        return Ok(TestResult {
                            name: self.name.clone(),
                            passed: false,
                            mismatch: Some(Mismatch {
                                kind: MismatchKind::ContentAssertion {
                                    path: assertion.path.clone(),
                                    kind: assertion.kind,
                                },
                                expected: assertion.expected.clone(),
                                actual: actual_content,
                                line: assertion.line_number,
                            }),
                        });
                    }
                }
                TestStep::CheckTreeSnapshot(tree_snapshot) => {
                    let actual_tree =
                        generate_tree(test_dir, tree_snapshot.depth, &self.config.tree)?;

                    if actual_tree.trim() != tree_snapshot.expected_content.trim() {
                        return Ok(TestResult {
                            name: self.name.clone(),
                            passed: false,
                            mismatch: Some(Mismatch {
                                kind: MismatchKind::TreeSnapshot,
                                expected: tree_snapshot.expected_content.clone(),
                                actual: actual_tree,
                                line: tree_snapshot.line_number,
                            }),
                        });
                    }
                }
            }
        }

        Ok(TestResult {
            name: self.name.clone(),
            passed: true,
            mismatch: None,
        })
    }
}

/// Apply filters to output.
fn apply_filters(filters: &[(Regex, String)], mut output: String) -> String {
    for (regex, replacement) in filters {
        output = regex.replace_all(&output, replacement.as_str()).to_string();
    }
    output
}

/// Substitute variables in a string.
///
/// Replaces `${VAR_NAME}` patterns with values from the substitutions map.
/// If a variable is not found in the map, checks environment variables.
/// If neither found, leaves the variable unchanged.
fn substitute_vars(input: &str, vars: &HashMap<String, String>) -> String {
    let mut result = input.to_string();

    // Find all ${VAR_NAME} patterns
    let re = regex::Regex::new(r"\$\{([^}]+)\}").unwrap();
    let matches: Vec<_> = re.captures_iter(input).collect();

    for cap in matches {
        let full_match = &cap[0];
        let var_name = &cap[1];

        // Try explicit substitutions first, then fall back to environment
        if let Some(value) = vars.get(var_name) {
            result = result.replace(full_match, value);
        } else if let Ok(env_value) = std::env::var(var_name) {
            result = result.replace(full_match, &env_value);
        }
        // If neither found, leave the variable unchanged
    }

    result
}

/// Recursively copy a file or directory.
fn copy_recursive(source: &Path, dest: &Path) -> Result<(), RunError> {
    let metadata = fs::metadata(source).map_err(|e| RunError::CopyFailed {
        src: source.to_path_buf(),
        dest: dest.to_path_buf(),
        err: e,
    })?;

    if metadata.is_dir() {
        // Create the destination directory
        fs::create_dir_all(dest).map_err(|e| RunError::CreateDir {
            path: dest.to_path_buf(),
            source: e,
        })?;

        // Copy contents recursively
        for entry in fs::read_dir(source).map_err(|e| RunError::ReadFile {
            path: source.to_path_buf(),
            source: e,
        })? {
            let entry = entry.map_err(|e| RunError::ReadFile {
                path: source.to_path_buf(),
                source: e,
            })?;
            let entry_path = entry.path();
            let entry_name = entry.file_name();
            let dest_path = dest.join(entry_name);
            copy_recursive(&entry_path, &dest_path)?;
        }
    } else {
        // Copy single file
        fs::copy(source, dest).map_err(|e| RunError::CopyFailed {
            src: source.to_path_buf(),
            dest: dest.to_path_buf(),
            err: e,
        })?;
    }

    Ok(())
}

/// Cross-platform implementation of `rm` command.
///
/// Supports common flags:
/// - `-r`, `-R`: Recursive removal
/// - `-f`: Force (ignore nonexistent files)
/// - Combined: `-rf`, `-fr`
pub fn run_rm_command(args: &[String], working_dir: &Path) -> Result<String, RunError> {
    let mut recursive = false;
    let mut force = false;
    let mut paths = Vec::new();

    // Parse arguments
    for arg in args {
        if arg.starts_with('-') && !arg.starts_with("--") {
            // Parse flags
            for ch in arg.chars().skip(1) {
                match ch {
                    'r' | 'R' => recursive = true,
                    'f' => force = true,
                    _ => {
                        return Err(RunError::Custom(format!("rm: invalid option -- '{ch}'")));
                    }
                }
            }
        } else if arg == "--" {
            // All remaining args are paths, not handled yet
            break;
        } else {
            // This is a path
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        return Err(RunError::Custom("rm: missing operand".to_string()));
    }

    let mut stderr_output = String::new();
    let mut success = true;
    let mut exit_code = 0;

    // Process each path
    for path_str in paths {
        let path = working_dir.join(path_str);

        // Check if path exists
        let metadata = match fs::metadata(&path) {
            Ok(m) => m,
            Err(e) => {
                if !force {
                    stderr_output.push_str(&format!(
                        "rm: cannot remove '{}': {}\n",
                        path_str,
                        if e.kind() == std::io::ErrorKind::NotFound {
                            "No such file or directory"
                        } else {
                            "Permission denied"
                        }
                    ));
                    success = false;
                    exit_code = 1;
                }
                continue;
            }
        };

        // Remove the file or directory
        let result = if metadata.is_dir() {
            if recursive {
                fs::remove_dir_all(&path)
            } else {
                // Try to remove as empty directory
                fs::remove_dir(&path).map_err(|e| {
                    if e.kind() == std::io::ErrorKind::Other
                        || e.raw_os_error() == Some(39)
                        || e.raw_os_error() == Some(41)
                    {
                        std::io::Error::new(std::io::ErrorKind::Other, "Directory not empty")
                    } else {
                        e
                    }
                })
            }
        } else {
            fs::remove_file(&path)
        };

        if let Err(_e) = result {
            if !force {
                let error_msg = if metadata.is_dir() && !recursive {
                    "cannot remove: Is a directory"
                } else {
                    "Permission denied"
                };
                stderr_output.push_str(&format!(
                    "rm: cannot remove '{}': {}\n",
                    path_str, error_msg
                ));
                success = false;
                exit_code = 1;
            }
        }
    }

    // Format output like a real command
    let output = format!(
        "success: {}\nexit_code: {}\n----- stdout -----\n\n----- stderr -----\n{}",
        success,
        exit_code,
        stderr_output.trim_end()
    )
    .trim_end()
    .to_string();

    Ok(output)
}

/// Run a command using a command builder and return the formatted output.
fn run_command_with_builder<F>(
    command_line: &str,
    working_dir: &Path,
    substitutions: &HashMap<String, String>,
    command_builder: &F,
) -> Result<String, RunError>
where
    F: Fn(&str) -> Command,
{
    // Apply variable substitutions (e.g., ${GITHUB_TOKEN} -> actual token)
    let command_line = substitute_vars(command_line, substitutions);

    let mut cmd = command_builder(&command_line);
    cmd.current_dir(working_dir);

    let output = run_combined(cmd)?;
    Ok(format_output(&output))
}

/// Generate a directory tree representation.
///
/// Produces output similar to the `tree` command.
fn generate_tree(
    dir: &Path,
    max_depth: Option<usize>,
    tree_config: &TreeConfig,
) -> Result<String, RunError> {
    let mut output = String::new();
    output.push('.');
    output.push('\n');

    // Check if the root directory itself is a venv
    let in_venv = is_venv_directory(dir);
    generate_tree_recursive(dir, &mut output, "", max_depth, 0, tree_config, in_venv)?;

    // Remove trailing newline for consistency with expected content
    if output.ends_with('\n') {
        output.pop();
    }

    Ok(output)
}

/// Check if a name matches any of the exclude patterns.
fn should_exclude(name: &str, exclude_patterns: &[String]) -> bool {
    for pattern in exclude_patterns {
        // Simple glob matching: support * as wildcard
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                let (prefix, suffix) = (parts[0], parts[1]);
                if name.starts_with(prefix) && name.ends_with(suffix) {
                    return true;
                }
            }
        } else if name == pattern {
            return true;
        }
    }
    false
}

/// Check if a directory appears to be a virtual environment.
fn is_venv_directory(dir: &Path) -> bool {
    dir.join("pyvenv.cfg").exists()
}

/// Apply default tree filters to normalize a name for cross-platform compatibility.
///
/// If `in_venv` is true, applies venv-specific normalizations like `bin`/`Scripts` -> `[BIN]`
/// and `lib`/`Lib` -> `[LIB]`.
fn apply_tree_default_filters(name: &str, in_venv: bool) -> String {
    // Normalize virtualenv bin directory (only when inside a venv)
    if in_venv && (name == "Scripts" || name == "bin") {
        return "[BIN]".to_string();
    }

    // Normalize virtualenv lib directory (only when inside a venv)
    // Windows uses `Lib`, Unix uses `lib`
    if in_venv && (name == "Lib" || name == "lib") {
        return "[LIB]".to_string();
    }

    name.to_string()
}

/// Recursively generate tree output.
fn generate_tree_recursive(
    dir: &Path,
    output: &mut String,
    prefix: &str,
    max_depth: Option<usize>,
    current_depth: usize,
    tree_config: &TreeConfig,
    in_venv: bool,
) -> Result<(), RunError> {
    // Check depth limit
    if let Some(max) = max_depth {
        if current_depth >= max {
            return Ok(());
        }
    }

    // Read and sort directory entries
    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(|e| RunError::ReadFile {
            path: dir.to_path_buf(),
            source: e,
        })?
        .filter_map(std::result::Result::ok)
        .collect();

    // Filter out excluded entries
    entries.retain(|e| {
        let name = e.file_name();
        let name_str = name.to_string_lossy();
        !should_exclude(&name_str, &tree_config.exclude)
    });

    entries.sort_by_key(|e| {
        let name = e.file_name();
        let name_str = name.to_string_lossy().to_string();
        if tree_config.default_filters {
            apply_tree_default_filters(&name_str, in_venv)
        } else {
            name_str
        }
    });

    // Filter out lib64 symlinks in venvs (Linux-specific artifact that just points to lib)
    if in_venv && tree_config.default_filters {
        entries.retain(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            if name_str == "lib64" {
                // Check if it's a symlink
                if let Ok(metadata) = fs::symlink_metadata(e.path()) {
                    return !metadata.is_symlink();
                }
            }
            true
        });
    }

    let len = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == len - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let entry_path = entry.path();

        // Use symlink_metadata to detect symlinks without following them
        let metadata = fs::symlink_metadata(&entry_path).map_err(|e| RunError::ReadFile {
            path: entry_path.clone(),
            source: e,
        })?;

        // Apply default filters to the display name if enabled
        let base_name = if tree_config.default_filters {
            apply_tree_default_filters(&name_str, in_venv)
        } else {
            name_str.to_string()
        };

        // Build display name with type indicators
        let display_name = if metadata.is_symlink() {
            // For symlinks, show "name -> target"
            let target = fs::read_link(&entry_path)
                .map(|t| t.to_string_lossy().to_string())
                .unwrap_or_else(|_| "?".to_string());
            format!("{base_name} -> {target}")
        } else if metadata.is_dir() {
            // For directories, add trailing /
            format!("{base_name}/")
        } else {
            // Regular file, no suffix
            base_name
        };

        output.push_str(prefix);
        output.push_str(connector);
        output.push_str(&display_name);
        output.push('\n');

        // Recurse into directories (but not symlinks, even if they point to directories)
        if metadata.is_dir() && !metadata.is_symlink() {
            let new_prefix = if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            // Check if this directory is a venv (for filtering children)
            let child_in_venv = in_venv || is_venv_directory(&entry_path);
            generate_tree_recursive(
                &entry_path,
                output,
                &new_prefix,
                max_depth,
                current_depth + 1,
                tree_config,
                child_in_venv,
            )?;
        }
    }

    Ok(())
}

/// Format command output in the `uv_snapshot` format.
/// Result of running a command with combined stdout+stderr.
struct CommandOutput {
    /// Interleaved stdout and stderr (via shared pipe).
    combined: String,
    /// Process exit status.
    status: ExitStatus,
}

/// Run a `Command` with stdout and stderr merged into a single pipe.
///
/// Uses `pipe()` + `dup()` to point both stdout and stderr at the same OS pipe,
/// giving true interleaved output in write order (deterministic).
///
/// # Safety
///
/// Uses `pipe()`, `dup()`, `close()`, and `from_raw_fd()` — all standard POSIX
/// operations on file descriptors created within this function. Each fd is created
/// once and ownership is transferred exactly once via `File::from_raw_fd`.
#[expect(unsafe_code)]
fn run_combined(mut cmd: Command) -> Result<CommandOutput, RunError> {
    // SAFETY: pipe() creates a unidirectional pipe, returning two file descriptors.
    // dup() duplicates a file descriptor. Both are standard POSIX and cannot cause UB
    // when given valid fd values (which pipe() guarantees).
    let (read_fd, write_fd) = {
        let mut fds = [0i32; 2];
        let ret = unsafe { libc::pipe(fds.as_mut_ptr()) };
        if ret != 0 {
            return Err(RunError::ExecuteCommand {
                source: std::io::Error::last_os_error(),
            });
        }
        (fds[0], fds[1])
    };

    let write_fd2 = unsafe { libc::dup(write_fd) };
    if write_fd2 < 0 {
        unsafe { libc::close(read_fd) };
        unsafe { libc::close(write_fd) };
        return Err(RunError::ExecuteCommand {
            source: std::io::Error::last_os_error(),
        });
    }

    // SAFETY: from_raw_fd takes ownership of the fd. We created write_fd/write_fd2
    // above via pipe()/dup() and transfer ownership exactly once each.
    // Stdio::from(File) consumes the File; spawn() will dup2 the fd for the child
    // and then the File (inside Stdio, inside Command) closes the parent's copy on drop.
    cmd.stdout(unsafe { Stdio::from(std::fs::File::from_raw_fd(write_fd)) });
    cmd.stderr(unsafe { Stdio::from(std::fs::File::from_raw_fd(write_fd2)) });

    let mut child = cmd
        .spawn()
        .map_err(|e| RunError::ExecuteCommand { source: e })?;

    // IMPORTANT: drop cmd now. spawn() takes &mut self, so cmd is still alive.
    // The Command struct holds the Stdio objects which own the parent's write fds.
    // Until cmd is dropped, the write fds stay open in the parent, and the reader
    // will never see EOF. Dropping cmd closes the parent's write fds.
    drop(cmd);

    // SAFETY: from_raw_fd takes ownership of read_fd, created by pipe() above.
    let read_file = unsafe { std::fs::File::from_raw_fd(read_fd) };

    // Read in a thread to avoid deadlock (pipe buffer could fill before child exits).
    let reader = std::thread::spawn(move || {
        let mut buf = String::new();
        let mut f = std::io::BufReader::new(read_file);
        f.read_to_string(&mut buf).map(|_| buf)
    });

    let status = child
        .wait()
        .map_err(|e| RunError::ExecuteCommand { source: e })?;

    let combined = reader
        .join()
        .map_err(|_| RunError::ExecuteCommand {
            source: std::io::Error::new(std::io::ErrorKind::Other, "reader thread panicked"),
        })?
        .map_err(|e| RunError::ExecuteCommand { source: e })?;

    Ok(CommandOutput { combined, status })
}

/// Format command output: combined text, with exit code appended for failures.
///
/// For exit code 0: just the output text.
/// For non-zero: output text + blank line + `[command failed with exit code N]`.
fn format_output(output: &CommandOutput) -> String {
    let mut result = output.combined.trim_end().to_string();
    let exit_code = output.status.code().unwrap_or(-1);
    if exit_code != 0 {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&format!("\n[command failed with exit code {exit_code}]"));
    }
    result
}

/// Compare two strings and generate a diff if they differ.
pub fn diff_strings(expected: &str, actual: &str) -> Option<String> {
    use std::fmt::Write;

    if expected.trim() == actual.trim() {
        return None;
    }

    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();

    let mut diff = String::new();
    diff.push_str("--- expected\n");
    diff.push_str("+++ actual\n");

    let max_len = expected_lines.len().max(actual_lines.len());
    for i in 0..max_len {
        let exp_line = expected_lines.get(i).copied().unwrap_or("");
        let act_line = actual_lines.get(i).copied().unwrap_or("");

        if exp_line != act_line {
            let _ = writeln!(diff, "-{exp_line}");
            let _ = writeln!(diff, "+{act_line}");
        } else {
            let _ = writeln!(diff, " {exp_line}");
        }
    }

    Some(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_output_success() {
        let output = CommandOutput {
            combined: "hello\nworld\n".to_string(),
            status: std::process::ExitStatus::default(),
        };

        let formatted = format_output(&output);
        assert_eq!(formatted, "hello\nworld");
        // No exit code line for success
        assert!(!formatted.contains("[command failed"));
    }

    #[test]
    fn test_format_output_empty_success() {
        let output = CommandOutput {
            combined: String::new(),
            status: std::process::ExitStatus::default(),
        };

        let formatted = format_output(&output);
        assert_eq!(formatted, "");
    }

    #[test]
    fn test_diff_strings_same() {
        assert!(diff_strings("hello\nworld", "hello\nworld").is_none());
    }

    #[test]
    fn test_diff_strings_different() {
        let diff = diff_strings("hello\nworld", "hello\nplanet").unwrap();
        assert!(diff.contains("-world"));
        assert!(diff.contains("+planet"));
    }

    #[test]
    fn test_apply_filters() {
        let filters = vec![
            (Regex::new(r"\d+ms").unwrap(), "[TIME]".to_string()),
            (Regex::new(r"\d+\.\d+s").unwrap(), "[TIME]".to_string()),
        ];

        let output = "Resolved in 123ms";
        assert_eq!(
            apply_filters(&filters, output.to_string()),
            "Resolved in [TIME]"
        );

        let output = "Resolved in 1.5s";
        assert_eq!(
            apply_filters(&filters, output.to_string()),
            "Resolved in [TIME]"
        );
    }

    #[test]
    fn test_substitute_vars() {
        let mut vars = HashMap::new();
        vars.insert("TOKEN".to_string(), "secret123".to_string());
        vars.insert("USER".to_string(), "alice".to_string());

        // Single substitution
        assert_eq!(
            substitute_vars("git+https://${TOKEN}@github.com/repo", &vars),
            "git+https://secret123@github.com/repo"
        );

        // Multiple substitutions
        assert_eq!(
            substitute_vars("${USER}:${TOKEN}", &vars),
            "alice:secret123"
        );

        // No substitution needed
        assert_eq!(substitute_vars("plain text", &vars), "plain text");

        // Empty vars
        let empty_vars = HashMap::new();
        assert_eq!(substitute_vars("${TOKEN}", &empty_vars), "${TOKEN}");
    }

    #[test]
    fn test_shlex_parsing_simple() {
        // Simple command
        let parts = shlex::split("uv pip install requests").unwrap();
        assert_eq!(parts, vec!["uv", "pip", "install", "requests"]);
    }

    #[test]
    fn test_shlex_parsing_double_quotes() {
        // Double-quoted argument with spaces
        let parts = shlex::split(r#"uv pip install "package with spaces""#).unwrap();
        assert_eq!(parts, vec!["uv", "pip", "install", "package with spaces"]);
    }

    #[test]
    fn test_shlex_parsing_single_quotes() {
        // Single-quoted argument
        let parts = shlex::split("uv pip install 'package with spaces'").unwrap();
        assert_eq!(parts, vec!["uv", "pip", "install", "package with spaces"]);
    }

    #[test]
    fn test_shlex_parsing_mixed_quotes() {
        // Mixed quoting
        let parts = shlex::split(r#"uv pip install "one thing" 'another thing'"#).unwrap();
        assert_eq!(
            parts,
            vec!["uv", "pip", "install", "one thing", "another thing"]
        );
    }

    #[test]
    fn test_shlex_parsing_git_url_with_token() {
        // Git URL with token (after substitution)
        let parts =
            shlex::split("uv pip install 'pkg @ git+https://token@github.com/org/repo'").unwrap();
        assert_eq!(
            parts,
            vec![
                "uv",
                "pip",
                "install",
                "pkg @ git+https://token@github.com/org/repo"
            ]
        );
    }

    #[test]
    fn test_substitute_and_shlex_combined() {
        // Test the full workflow: substitution followed by shlex parsing
        let mut vars = HashMap::new();
        vars.insert("GITHUB_TOKEN".to_string(), "ghp_secret123".to_string());

        let command = "uv pip install 'pkg @ git+https://${GITHUB_TOKEN}@github.com/org/repo'";
        let substituted = substitute_vars(command, &vars);
        let parts = shlex::split(&substituted).unwrap();

        assert_eq!(
            parts,
            vec![
                "uv",
                "pip",
                "install",
                "pkg @ git+https://ghp_secret123@github.com/org/repo"
            ]
        );
    }

    #[test]
    fn test_shlex_parsing_escaped_quotes() {
        // Escaped quotes within double quotes
        let parts = shlex::split(r#"echo "he said \"hello\"""#).unwrap();
        assert_eq!(parts, vec!["echo", r#"he said "hello""#]);
    }

    #[test]
    fn test_rm_single_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "content").unwrap();
        assert!(test_file.exists());

        let args = vec!["test.txt".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        assert!(result.contains("success: true"));
        assert!(result.contains("exit_code: 0"));
        assert!(!test_file.exists());
    }

    #[test]
    fn test_rm_multiple_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "content1").unwrap();
        fs::write(&file2, "content2").unwrap();
        assert!(file1.exists());
        assert!(file2.exists());

        let args = vec!["file1.txt".to_string(), "file2.txt".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        assert!(result.contains("success: true"));
        assert!(result.contains("exit_code: 0"));
        assert!(!file1.exists());
        assert!(!file2.exists());
    }

    #[test]
    fn test_rm_directory_without_recursive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_dir = temp_dir.path().join("testdir");
        fs::create_dir(&test_dir).unwrap();
        fs::write(test_dir.join("file.txt"), "content").unwrap();
        assert!(test_dir.exists());

        let args = vec!["testdir".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        // Should fail without -r flag
        assert!(result.contains("success: false"));
        assert!(result.contains("exit_code: 1"));
        assert!(result.contains("Is a directory"));
        assert!(test_dir.exists());
    }

    #[test]
    fn test_rm_directory_with_recursive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_dir = temp_dir.path().join("testdir");
        fs::create_dir(&test_dir).unwrap();
        fs::write(test_dir.join("file.txt"), "content").unwrap();
        assert!(test_dir.exists());

        let args = vec!["-r".to_string(), "testdir".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        assert!(result.contains("success: true"));
        assert!(result.contains("exit_code: 0"));
        assert!(!test_dir.exists());
    }

    #[test]
    fn test_rm_nonexistent_file_without_force() {
        let temp_dir = tempfile::tempdir().unwrap();

        let args = vec!["nonexistent.txt".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        assert!(result.contains("success: false"));
        assert!(result.contains("exit_code: 1"));
        assert!(result.contains("No such file or directory"));
    }

    #[test]
    fn test_rm_nonexistent_file_with_force() {
        let temp_dir = tempfile::tempdir().unwrap();

        let args = vec!["-f".to_string(), "nonexistent.txt".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        // With -f, should succeed even if file doesn't exist
        assert!(result.contains("success: true"));
        assert!(result.contains("exit_code: 0"));
    }

    #[test]
    fn test_rm_combined_flags() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_dir = temp_dir.path().join("testdir");
        fs::create_dir(&test_dir).unwrap();
        fs::write(test_dir.join("file.txt"), "content").unwrap();
        assert!(test_dir.exists());

        // Test -rf (combined)
        let args = vec!["-rf".to_string(), "testdir".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        assert!(result.contains("success: true"));
        assert!(result.contains("exit_code: 0"));
        assert!(!test_dir.exists());
    }

    #[test]
    fn test_rm_empty_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_dir = temp_dir.path().join("emptydir");
        fs::create_dir(&test_dir).unwrap();
        assert!(test_dir.exists());

        // Should be able to remove empty directory without -r
        let args = vec!["emptydir".to_string()];
        let result = run_rm_command(&args, temp_dir.path()).unwrap();

        assert!(result.contains("success: true"));
        assert!(result.contains("exit_code: 0"));
        assert!(!test_dir.exists());
    }

    #[test]
    fn test_rm_no_arguments() {
        let temp_dir = tempfile::tempdir().unwrap();

        let args = vec![];
        let result = run_rm_command(&args, temp_dir.path());

        assert!(result.is_err());
        if let Err(RunError::Custom(msg)) = result {
            assert!(msg.contains("missing operand"));
        } else {
            panic!("Expected Custom error");
        }
    }
}
