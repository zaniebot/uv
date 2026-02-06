//! Data structures for the markdown test framework.

use serde::Deserialize;
use std::path::PathBuf;

/// A complete markdown test file containing multiple tests.
#[derive(Debug)]
pub struct MarkdownTestFile {
    /// The source file path (for error reporting).
    pub path: PathBuf,
    /// All tests extracted from the file.
    pub tests: Vec<MarkdownTest>,
}

/// A single test extracted from a markdown file.
///
/// Each test corresponds to a leaf section (a section that contains code blocks
/// rather than subsections).
#[derive(Debug)]
pub struct MarkdownTest {
    /// Human-readable name derived from the header hierarchy.
    /// Example: "Lock - Basic locking"
    pub name: String,
    /// Configuration for this test (inherited from parent sections).
    pub config: TestConfig,
    /// Ordered sequence of steps to execute (files, trees, commands, snapshots in document order).
    pub steps: Vec<TestStep>,
    /// Line number in the source file where this test starts (for error reporting).
    pub line_number: usize,
}

impl MarkdownTest {
    /// Check if this test should run on the current platform with given enabled features.
    ///
    /// Returns `true` if:
    /// - `target-os` matches the current system
    /// - `target-family` matches the current system
    /// - All `required-features` are present in `enabled_features`
    #[must_use]
    pub fn should_run(&self, enabled_features: &[String]) -> bool {
        self.config.should_run_on_current_platform()
            && self.config.has_required_features(enabled_features)
    }
}

/// A single step in test execution, preserving document order.
#[derive(Debug, Clone)]
pub enum TestStep {
    /// Create/write a file to the test directory.
    WriteFile(EmbeddedFile),
    /// Create a directory tree structure.
    CreateTree(TreeCreation),
    /// Copy files or directories from a source path.
    CopyFrom(CopyFrom),
    /// Execute a command and validate output.
    RunCommand(Command),
    /// Verify a file snapshot.
    CheckFileSnapshot(FileSnapshot),
    /// Check a content assertion.
    CheckContentAssertion(ContentAssertion),
    /// Verify a directory tree snapshot.
    CheckTreeSnapshot(TreeSnapshot),
}

/// An embedded file to be written to the test directory.
#[derive(Debug, Clone)]
pub struct EmbeddedFile {
    /// Relative path within the test directory.
    pub path: PathBuf,
    /// Content of the file.
    pub content: String,
    /// Line number in the markdown source where this file is defined.
    pub line_number: usize,
}

/// Copy files or directories from an external source path.
///
/// The source path can use variable substitution (e.g., `${WORKSPACE}/test/packages/foo`).
#[derive(Debug, Clone)]
pub struct CopyFrom {
    /// Source path (may contain variable references like `${WORKSPACE}`).
    pub source: String,
    /// Destination path relative to the test directory.
    pub dest: PathBuf,
    /// Line number in the markdown source where this copy is defined.
    pub line_number: usize,
}

/// A command to execute during the test.
#[derive(Debug, Clone)]
pub struct Command {
    /// The full command line (without the `$ ` prefix).
    pub command: String,
    /// Expected output in the `uv_snapshot` format:
    /// - `success: true/false`
    /// - `exit_code: N`
    /// - `----- stdout -----` section
    /// - `----- stderr -----` section
    pub expected_output: String,
    /// Working directory relative to the test directory.
    /// If `None`, commands run in the test directory root.
    pub working_dir: Option<PathBuf>,
    /// Line number in the markdown source where this command is defined.
    pub line_number: usize,
}

/// A file to snapshot after test execution.
#[derive(Debug, Clone)]
pub struct FileSnapshot {
    /// Relative path within the test directory.
    pub path: PathBuf,
    /// Expected content of the file (to be compared against actual content).
    pub expected_content: String,
    /// Line number in the markdown source where this snapshot is defined.
    pub line_number: usize,
}

/// A content assertion to check after test execution.
#[derive(Debug, Clone)]
pub struct ContentAssertion {
    /// Relative path within the test directory.
    pub path: PathBuf,
    /// The kind of assertion.
    pub kind: AssertKind,
    /// The expected content to check against.
    pub expected: String,
    /// Line number in the markdown source where this assertion is defined.
    pub line_number: usize,
}

/// A directory tree snapshot to check after test execution.
#[derive(Debug, Clone)]
pub struct TreeSnapshot {
    /// Expected tree content (in `tree` command format).
    pub expected_content: String,
    /// Optional depth limit for tree generation.
    pub depth: Option<usize>,
    /// Line number in the markdown source where this snapshot is defined.
    pub line_number: usize,
}

/// An entry in a tree creation block.
#[derive(Debug, Clone)]
pub enum TreeEntry {
    /// A directory to create.
    Directory { path: PathBuf },
    /// A file to create (empty).
    File { path: PathBuf },
    /// A symbolic link to create.
    Symlink { path: PathBuf, target: PathBuf },
}

/// A directory tree structure to create before running commands.
#[derive(Debug, Clone)]
pub struct TreeCreation {
    /// Entries to create (directories, files, symlinks).
    pub entries: Vec<TreeEntry>,
    /// Line number in the markdown source where this tree is defined.
    pub line_number: usize,
}

/// Test configuration parsed from `#! mdtest` TOML blocks.
///
/// The framework extracts only what it needs (skip logic + tree config)
/// and passes the full raw TOML to the harness for interpretation.
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// The raw merged TOML value. The harness deserializes this into its
    /// own application-specific config struct (e.g., python versions, filters).
    pub raw: toml::Value,
    /// Target OS strings from `[environment].target-os`.
    /// Empty means "run on all OSes".
    pub target_os: Vec<String>,
    /// Target OS family strings from `[environment].target-family`.
    /// Empty means "run on all families".
    pub target_family: Vec<String>,
    /// Required features from `[environment].required-features`.
    /// Empty means "no features required".
    pub required_features: Vec<String>,
    /// Tree configuration from `[tree]`.
    pub tree: TreeConfig,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            raw: toml::Value::Table(toml::map::Map::new()),
            target_os: Vec::new(),
            target_family: Vec::new(),
            required_features: Vec::new(),
            tree: TreeConfig::default(),
        }
    }
}

impl TestConfig {
    /// Parse a `TestConfig` from a TOML string.
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        let value: toml::Value = toml::from_str(toml_str)?;
        Ok(Self::from_value(value))
    }

    /// Create a `TestConfig` from a parsed TOML value.
    pub fn from_value(value: toml::Value) -> Self {
        let target_os = extract_string_or_array(&value, &["environment", "target-os"]);
        let target_family = extract_string_or_array(&value, &["environment", "target-family"]);
        let required_features =
            extract_string_or_array(&value, &["environment", "required-features"]);
        let tree: TreeConfig = value
            .get("tree")
            .cloned()
            .and_then(|v| v.try_into().ok())
            .unwrap_or_default();

        Self {
            raw: value,
            target_os,
            target_family,
            required_features,
            tree,
        }
    }

    /// Merge two configs, with `other` (child) taking precedence.
    ///
    /// Performs a shallow table merge: child keys override parent keys.
    /// For sub-tables, the merge is recursive.
    #[must_use]
    pub fn merge(&self, other: &Self) -> Self {
        let merged = merge_toml_values(&self.raw, &other.raw);
        Self::from_value(merged)
    }

    /// Check if this test should run on the current platform.
    #[must_use]
    pub fn should_run_on_current_platform(&self) -> bool {
        self.matches_target_os() && self.matches_target_family()
    }

    /// Check if all required features are enabled.
    #[must_use]
    pub fn has_required_features(&self, enabled_features: &[String]) -> bool {
        self.required_features
            .iter()
            .all(|f| enabled_features.contains(f))
    }

    fn matches_target_os(&self) -> bool {
        if self.target_os.is_empty() {
            return true;
        }
        let current = current_os();
        self.target_os.iter().any(|os| os == &current)
    }

    fn matches_target_family(&self) -> bool {
        if self.target_family.is_empty() {
            return true;
        }
        self.target_family.iter().any(|f| is_current_family(f))
    }
}

/// Get the current OS as a string (matching common test config values).
fn current_os() -> String {
    // Map std::env::consts::OS to the names used in test configs
    match std::env::consts::OS {
        "macos" => "macos".to_string(),
        other => other.to_string(),
    }
}

/// Check if the given family string matches the current OS family.
fn is_current_family(family: &str) -> bool {
    match family {
        "unix" => cfg!(target_family = "unix"),
        "windows" => cfg!(target_family = "windows"),
        "wasm" => cfg!(target_family = "wasm"),
        _ => false,
    }
}

/// Extract a string or array of strings from a nested TOML path.
fn extract_string_or_array(value: &toml::Value, path: &[&str]) -> Vec<String> {
    let mut current = value;
    for key in path {
        match current.get(key) {
            Some(v) => current = v,
            None => return Vec::new(),
        }
    }
    match current {
        toml::Value::String(s) => vec![s.clone()],
        toml::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

/// Recursively merge two TOML values, with `child` taking precedence.
///
/// - Tables: merge keys recursively (child keys override parent)
/// - Arrays: child replaces parent (no concatenation)
/// - Scalars: child replaces parent
fn merge_toml_values(parent: &toml::Value, child: &toml::Value) -> toml::Value {
    match (parent, child) {
        (toml::Value::Table(parent_table), toml::Value::Table(child_table)) => {
            let mut merged = parent_table.clone();
            for (key, child_val) in child_table {
                if let Some(parent_val) = merged.get(key) {
                    merged.insert(key.clone(), merge_toml_values(parent_val, child_val));
                } else {
                    merged.insert(key.clone(), child_val.clone());
                }
            }
            toml::Value::Table(merged)
        }
        // For non-tables, child wins
        (_, child) => child.clone(),
    }
}

/// Tree configuration for directory tree snapshots.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TreeConfig {
    /// Patterns to exclude from tree output.
    /// Supports glob patterns like `*.pyc`, `__pycache__`, `cache`.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Whether to apply default filters to tree output (e.g., normalizing
    /// `Scripts` to `bin` on Windows). Defaults to true.
    #[serde(default = "default_true")]
    pub default_filters: bool,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            exclude: Vec::new(),
            default_filters: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Code block info string, parsed into language and any extra attributes.
///
/// Only the language identifier is expected in the fence (e.g., ` ```toml `).
/// All other metadata must use content directives (`#! file: path`, `#! snapshot`, etc.).
/// Any `key=value` pairs in the fence are collected as errors.
#[derive(Debug, Clone, Default)]
pub struct CodeBlockAttributes {
    /// The language of the code block (e.g., "toml", "python", "console", "tree", "copy").
    pub language: Option<String>,
    /// Any `key=value` pairs found in the fence (these are errors).
    pub extra: Vec<String>,
}

/// Kind of assertion for file content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssertKind {
    /// Assert that the file contains the specified content.
    Contains,
}

impl CodeBlockAttributes {
    /// Parse a code block info string.
    ///
    /// Only the language identifier is recognized (e.g., `toml`, `console`).
    /// Any `key=value` pairs are collected in `extra` and treated as errors
    /// by the parser.
    pub fn parse(info_string: &str) -> Self {
        let mut attrs = Self::default();

        for (i, part) in info_string.split_whitespace().enumerate() {
            if i == 0 && !part.contains('=') {
                attrs.language = Some(part.to_string());
            } else {
                attrs.extra.push(part.to_string());
            }
        }

        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_block_attributes_empty() {
        let attrs = CodeBlockAttributes::parse("");
        assert!(attrs.language.is_none());
        assert!(attrs.extra.is_empty());
    }

    #[test]
    fn test_code_block_attributes_language_only() {
        let attrs = CodeBlockAttributes::parse("toml");
        assert_eq!(attrs.language.as_deref(), Some("toml"));
        assert!(attrs.extra.is_empty());
    }

    #[test]
    fn test_code_block_attributes_extra_rejected() {
        let attrs = CodeBlockAttributes::parse("toml title=\"pyproject.toml\"");
        assert_eq!(attrs.language.as_deref(), Some("toml"));
        assert_eq!(attrs.extra, vec!["title=\"pyproject.toml\""]);
    }

    #[test]
    fn test_config_from_toml() {
        let config = TestConfig::from_toml(
            r#"
[environment]
target-os = "linux"
target-family = "unix"
required-features = ["pypi", "git"]

[tree]
exclude = ["*.pyc"]
"#,
        )
        .unwrap();

        assert_eq!(config.target_os, vec!["linux"]);
        assert_eq!(config.target_family, vec!["unix"]);
        assert_eq!(config.required_features, vec!["pypi", "git"]);
        assert_eq!(config.tree.exclude, vec!["*.pyc"]);
    }

    #[test]
    fn test_config_from_toml_string_values() {
        // Single strings (not arrays) should also work
        let config = TestConfig::from_toml(
            r#"
[environment]
target-os = "macos"
required-features = "pypi"
"#,
        )
        .unwrap();

        assert_eq!(config.target_os, vec!["macos"]);
        assert_eq!(config.required_features, vec!["pypi"]);
    }

    #[test]
    fn test_config_merge_child_overrides() {
        let parent = TestConfig::from_toml(
            r#"
[environment]
python-versions = "3.11"
exclude-newer = "2024-01-01"
env = { FOO = "bar" }
"#,
        )
        .unwrap();

        let child = TestConfig::from_toml(
            r#"
[environment]
python-versions = "3.12"
env = { BAZ = "qux" }
"#,
        )
        .unwrap();

        let merged = parent.merge(&child);
        // Child overrides python-versions
        assert_eq!(
            merged.raw["environment"]["python-versions"].as_str(),
            Some("3.12")
        );
        // Parent's exclude-newer is preserved
        assert_eq!(
            merged.raw["environment"]["exclude-newer"].as_str(),
            Some("2024-01-01")
        );
        // Env tables are merged (child keys override, parent keys preserved)
        let env = merged.raw["environment"]["env"].as_table().unwrap();
        assert_eq!(env.get("BAZ").and_then(|v| v.as_str()), Some("qux"));
        assert_eq!(env.get("FOO").and_then(|v| v.as_str()), Some("bar"));
    }

    #[test]
    fn test_config_merge_preserves_parent_defaults() {
        let parent = TestConfig::from_toml(
            r#"
[environment]
target-os = "linux"
"#,
        )
        .unwrap();

        let child = TestConfig::from_toml(
            r#"
[environment]
python-versions = "3.12"
"#,
        )
        .unwrap();

        let merged = parent.merge(&child);
        // Parent's target-os is preserved
        assert_eq!(merged.target_os, vec!["linux"]);
        // Child's python-versions is present
        assert_eq!(
            merged.raw["environment"]["python-versions"].as_str(),
            Some("3.12")
        );
    }

    #[test]
    fn test_config_should_run_matches_current() {
        // Empty target_os/target_family = run everywhere
        let config = TestConfig::default();
        assert!(config.should_run_on_current_platform());
    }

    #[test]
    fn test_config_should_run_wrong_os() {
        let config = TestConfig::from_toml(
            r#"
[environment]
target-os = "freebsd"
"#,
        )
        .unwrap();
        // Unless we're on FreeBSD, this should not run
        if std::env::consts::OS != "freebsd" {
            assert!(!config.should_run_on_current_platform());
        }
    }

    #[test]
    fn test_config_has_required_features() {
        let config = TestConfig::from_toml(
            r#"
[environment]
required-features = ["pypi", "git"]
"#,
        )
        .unwrap();

        let enabled = vec!["pypi".to_string(), "git".to_string(), "python".to_string()];
        assert!(config.has_required_features(&enabled));

        let partial = vec!["pypi".to_string()];
        assert!(!config.has_required_features(&partial));
    }

    #[test]
    fn test_config_raw_passthrough() {
        // Harness-specific fields should be available in raw
        let config = TestConfig::from_toml(
            r#"
[environment]
python-versions = "3.12"
create-venv = false

[filters]
counts = true
python-names = true
"#,
        )
        .unwrap();

        assert_eq!(
            config.raw["environment"]["python-versions"].as_str(),
            Some("3.12")
        );
        assert_eq!(
            config.raw["environment"]["create-venv"].as_bool(),
            Some(false)
        );
        assert_eq!(config.raw["filters"]["counts"].as_bool(), Some(true));
        assert_eq!(config.raw["filters"]["python-names"].as_bool(), Some(true));
    }

    #[test]
    fn test_merge_toml_recursive() {
        let parent = toml::from_str::<toml::Value>(
            r#"
[a]
x = 1
y = 2
[b]
z = 3
"#,
        )
        .unwrap();

        let child = toml::from_str::<toml::Value>(
            r#"
[a]
y = 99
[c]
w = 4
"#,
        )
        .unwrap();

        let merged = merge_toml_values(&parent, &child);
        assert_eq!(merged["a"]["x"].as_integer(), Some(1)); // parent preserved
        assert_eq!(merged["a"]["y"].as_integer(), Some(99)); // child overrides
        assert_eq!(merged["b"]["z"].as_integer(), Some(3)); // parent preserved
        assert_eq!(merged["c"]["w"].as_integer(), Some(4)); // child added
    }
}
