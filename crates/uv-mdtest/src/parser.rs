//! Markdown test file parser.
//!
//! Parses markdown files into test definitions using pulldown-cmark.

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::path::PathBuf;
use thiserror::Error;

use crate::types::{
    AssertKind, CodeBlockAttributes, Command, ContentAssertion, CopyFrom, EmbeddedFile,
    FileSnapshot, MarkdownTest, MarkdownTestFile, TestConfig, TestStep, TreeCreation, TreeEntry,
    TreeSnapshot,
};

/// Errors that can occur during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid TOML configuration at line {line}: {message}")]
    InvalidConfig { line: usize, message: String },

    #[error("Command block missing expected output at line {line}")]
    MissingExpectedOutput { line: usize },

    #[error("File contains no tests")]
    NoTests,

    #[error(
        "Multiple title headers found at line {line}. Only one level-1 header (`#`) is allowed per file."
    )]
    MultipleTitles { line: usize },
}

impl MarkdownTestFile {
    /// Parse a markdown file into a test file structure.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path (for error reporting)
    /// * `source` - The markdown source content
    ///
    /// # Returns
    ///
    /// A `MarkdownTestFile` containing all tests extracted from the file.
    pub fn parse(path: PathBuf, source: &str) -> Result<Self, ParseError> {
        let mut parser_state = ParserState::new(path.clone(), source);
        parser_state.parse()?;

        let tests = parser_state.finalize()?;

        Ok(Self { path, tests })
    }
}

/// Internal parser state.
struct ParserState<'a> {
    /// Source file path (reserved for future error reporting).
    #[expect(dead_code)]
    path: PathBuf,
    /// Source content.
    source: &'a str,
    /// Current line number (1-indexed).
    current_line: usize,
    /// Stack of section headers for building test names.
    header_stack: Vec<(usize, String)>, // (level, title)
    /// File-level config (applies to all tests).
    file_config: TestConfig,
    /// Current section's config override.
    section_config: Option<TestConfig>,
    /// Current section's steps (files, trees, commands, snapshots in document order).
    current_steps: Vec<TestStep>,
    /// Line number where current section starts.
    section_start_line: usize,
    /// Whether we've seen a section header (level 2 or deeper).
    /// Level 1 headers are treated as document titles, not test sections.
    seen_section_header: bool,
    /// Whether we've seen the document title (level 1 header).
    seen_title: bool,
    /// All completed tests.
    tests: Vec<MarkdownTest>,
}

impl<'a> ParserState<'a> {
    fn new(path: PathBuf, source: &'a str) -> Self {
        Self {
            path,
            source,
            current_line: 1,
            header_stack: Vec::new(),
            file_config: TestConfig::default(),
            section_config: None,
            current_steps: Vec::new(),
            section_start_line: 1,
            seen_section_header: false,
            seen_title: false,
            tests: Vec::new(),
        }
    }

    fn parse(&mut self) -> Result<(), ParseError> {
        let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
        let parser = Parser::new_ext(self.source, options);

        let mut in_heading = false;
        let mut current_heading_level = 0usize;
        let mut current_heading_text = String::new();
        let mut in_code_block = false;
        let mut code_block_info = String::new();
        let mut code_block_content = String::new();
        let mut code_block_start_line = 0;

        // Track line numbers by counting newlines in the source
        let mut byte_offset = 0usize;

        for (event, range) in parser.into_offset_iter() {
            // Update line number based on byte offset
            while byte_offset < range.start {
                if self.source.as_bytes().get(byte_offset) == Some(&b'\n') {
                    self.current_line += 1;
                }
                byte_offset += 1;
            }

            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    current_heading_level = heading_level_to_usize(level);
                    current_heading_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    self.handle_heading(current_heading_level, &current_heading_text)?;
                }
                Event::Text(text) if in_heading => {
                    current_heading_text.push_str(&text);
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    code_block_content.clear();
                    code_block_start_line = self.current_line;
                    code_block_info = match kind {
                        CodeBlockKind::Fenced(info) => info.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    self.handle_code_block(
                        &code_block_info,
                        &code_block_content,
                        code_block_start_line,
                    )?;
                }
                Event::Text(text) if in_code_block => {
                    code_block_content.push_str(&text);
                }
                _ => {}
            }
        }

        // Finalize the last section
        self.flush_section();

        Ok(())
    }

    fn handle_heading(&mut self, level: usize, title: &str) -> Result<(), ParseError> {
        // First, flush any pending content from the previous section
        self.flush_section();

        // Check for multiple title headers
        if level == 1 {
            if self.seen_title {
                return Err(ParseError::MultipleTitles {
                    line: self.current_line,
                });
            }
            self.seen_title = true;
        }

        // Pop headers that are at the same or deeper level
        while let Some((existing_level, _)) = self.header_stack.last() {
            if *existing_level >= level {
                self.header_stack.pop();
            } else {
                break;
            }
        }

        // Push the new header
        self.header_stack.push((level, title.to_string()));

        // Mark that we've seen a section header (level 2 or deeper).
        // Level 1 headers are document titles, not test sections.
        if level >= 2 {
            self.seen_section_header = true;
        }

        // Reset section state
        self.section_start_line = self.current_line;
        self.section_config = None;

        Ok(())
    }

    fn handle_code_block(
        &mut self,
        info_string: &str,
        content: &str,
        line_number: usize,
    ) -> Result<(), ParseError> {
        let attrs = CodeBlockAttributes::parse(info_string);
        if !attrs.extra.is_empty() {
            // Skip code blocks with fence attributes — they need to be migrated
            // to content directives. Log for visibility but don't fail the parse.
            eprintln!(
                "Warning: code fence attributes at line {line_number} are not supported: {:?}. \
                 Use content directives instead (e.g., `#! file: path`, `#! snapshot`).",
                attrs.extra
            );
            return Ok(());
        }
        let content = content.trim_end_matches('\n').to_string();

        // Extract directives from content (# file:, # mdtest, # snapshot, etc.)
        let (directives, content) = extract_directives(&content);

        // Check if this is a config block
        if directives.is_mdtest {
            let config =
                TestConfig::from_toml(&content).map_err(|e| ParseError::InvalidConfig {
                    line: line_number,
                    message: e.to_string(),
                })?;

            if self.seen_section_header {
                self.section_config = Some(self.file_config.merge(&config));
            } else {
                self.file_config = self.file_config.merge(&config);
            }
            return Ok(());
        }

        // Check if this is a command block (starts with `$ `)
        if content.starts_with("$ ") {
            let working_dir = directives.working_dir.map(PathBuf::from);
            let command = parse_command_block(&content, line_number, working_dir)?;
            self.current_steps.push(TestStep::RunCommand(command));
            return Ok(());
        }

        // Check if this is a tree block (# tree [snapshot=true] [depth=N])
        if let Some(tree) = directives.tree {
            if tree.snapshot {
                self.current_steps
                    .push(TestStep::CheckTreeSnapshot(TreeSnapshot {
                        expected_content: content,
                        depth: tree.depth,
                        line_number,
                    }));
            } else {
                let entries = parse_tree_content(&content);
                self.current_steps.push(TestStep::CreateTree(TreeCreation {
                    entries,
                    line_number,
                }));
            }
            return Ok(());
        }

        // Check if this is a copy block (# copy, content: source -> dest)
        if directives.copy {
            let (source, dest) = parse_copy_content(&content, line_number)?;
            self.current_steps.push(TestStep::CopyFrom(CopyFrom {
                source,
                dest: PathBuf::from(dest),
                line_number,
            }));
            return Ok(());
        }

        // Check if this is a content assertion
        if let Some(kind) = directives.assert_kind {
            if let Some(ref path) = directives.file {
                self.current_steps
                    .push(TestStep::CheckContentAssertion(ContentAssertion {
                        path: PathBuf::from(path),
                        kind,
                        expected: content,
                        line_number,
                    }));
            }
            return Ok(());
        }

        // Check if this is a file snapshot
        if directives.snapshot {
            if let Some(ref path) = directives.file {
                self.current_steps
                    .push(TestStep::CheckFileSnapshot(FileSnapshot {
                        path: PathBuf::from(path),
                        expected_content: content,
                        line_number,
                    }));
            }
            return Ok(());
        }

        // Check if this is an embedded file
        if let Some(path) = directives.file {
            self.current_steps.push(TestStep::WriteFile(EmbeddedFile {
                path: PathBuf::from(path),
                content,
                line_number,
            }));
        }

        Ok(())
    }

    fn flush_section(&mut self) {
        // Only create a test if we have steps that require execution or verification
        let has_executable_content = self.current_steps.iter().any(|step| {
            matches!(
                step,
                TestStep::RunCommand(_)
                    | TestStep::CheckFileSnapshot(_)
                    | TestStep::CheckContentAssertion(_)
                    | TestStep::CheckTreeSnapshot(_)
            )
        });

        if has_executable_content {
            // Build the test name from the header hierarchy
            let name = self
                .header_stack
                .iter()
                .map(|(_, title)| title.as_str())
                .collect::<Vec<_>>()
                .join(" - ");

            // Use section config if set, otherwise use file config
            let config = self
                .section_config
                .clone()
                .unwrap_or_else(|| self.file_config.clone());

            self.tests.push(MarkdownTest {
                name,
                config,
                steps: std::mem::take(&mut self.current_steps),
                line_number: self.section_start_line,
            });
        }

        // Clear current section state
        self.current_steps.clear();
        self.section_config = None;
    }

    fn finalize(self) -> Result<Vec<MarkdownTest>, ParseError> {
        if self.tests.is_empty() {
            return Err(ParseError::NoTests);
        }

        Ok(self.tests)
    }
}

/// Parse a command block into a Command structure.
fn parse_command_block(
    content: &str,
    line_number: usize,
    working_dir: Option<PathBuf>,
) -> Result<Command, ParseError> {
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        return Err(ParseError::MissingExpectedOutput { line: line_number });
    }

    // First line is the command (starting with `$ `)
    let command_line = lines[0];
    let command = command_line
        .strip_prefix("$ ")
        .unwrap_or(command_line)
        .to_string();

    // Rest is the expected output
    let expected_output = if lines.len() > 1 {
        lines[1..].join("\n")
    } else {
        String::new()
    };

    Ok(Command {
        command,
        expected_output,
        working_dir,
        line_number,
    })
}

/// Parse copy block content in `source -> dest` format.
fn parse_copy_content(content: &str, line_number: usize) -> Result<(String, String), ParseError> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((source, dest)) = trimmed.split_once(" -> ") {
            return Ok((source.trim().to_string(), dest.trim().to_string()));
        }
    }
    Err(ParseError::InvalidConfig {
        line: line_number,
        message: "Copy block must contain `source -> dest`".to_string(),
    })
}

fn heading_level_to_usize(level: HeadingLevel) -> usize {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Known directives.
///
/// Simple directives: `# keyword` or `# keyword: value`
/// Directives with args: `# keyword arg1 arg2 key=value`
const KNOWN_DIRECTIVES: &[&str] = &[
    "file",
    "mdtest",
    "snapshot",
    "assert",
    "working-dir",
    "tree",
    "copy",
];

/// Check if a line is a content directive (`#! keyword` or `#! keyword: value`).
///
/// This is used by both the parser (to extract directives) and the snapshot
/// updater (to preserve directives when updating content).
pub fn is_directive_line(line: &str) -> bool {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("#! ") {
        // Get the first word (the directive name)
        let keyword = rest.split_whitespace().next().unwrap_or(rest);
        // Also handle `#! key: value` form
        let keyword = keyword.strip_suffix(':').unwrap_or(keyword);
        KNOWN_DIRECTIVES.contains(&keyword)
    } else {
        false
    }
}

/// Directives extracted from `# key` and `# key: value` comment lines
/// at the top of a code block's content.
///
/// All directives are stripped from the content before further processing.
#[derive(Debug, Default)]
struct ContentDirectives {
    /// File path from `# file: path`.
    file: Option<String>,
    /// Whether this is an mdtest config block (`# mdtest`).
    is_mdtest: bool,
    /// Whether this is a snapshot block (`# snapshot`).
    snapshot: bool,
    /// Assertion kind from `# assert: contains`.
    assert_kind: Option<AssertKind>,
    /// Working directory from `# working-dir: path`.
    working_dir: Option<String>,
    /// Tree block settings from `# tree [create] [depth=N]`.
    tree: Option<TreeDirective>,
    /// Whether this is a copy block (`# copy`). Content is `source -> dest`.
    copy: bool,
}

/// Arguments parsed from `# tree [snapshot=true] [depth=N]`.
///
/// Tree blocks default to creation mode. Use `snapshot=true` to verify
/// directory structure instead (consistent with `# file:` + `# snapshot`).
#[derive(Debug, Default)]
struct TreeDirective {
    snapshot: bool,
    depth: Option<usize>,
}

/// Extract directives from comment lines at the top of code block content.
///
/// Directives are comment lines at the top of a code block that control its behavior.
/// Parsing stops at the first non-directive, non-blank line.
/// A single blank line after the directive block is also consumed.
///
/// Simple directives:
/// - `#! file: <path>` — names the file (for creation or verification)
/// - `#! mdtest` — marks block as test configuration
/// - `#! snapshot` — marks block as file snapshot verification
/// - `#! assert: contains` — marks block as content assertion
/// - `#! working-dir: <path>` — sets working directory for commands
/// - `#! copy` — marks block as a copy operation (content: `source -> dest`)
///
/// Directives with arguments:
/// - `#! tree [snapshot=true] [depth=N]` — marks block as a tree snapshot or creation
fn extract_directives(content: &str) -> (ContentDirectives, String) {
    let mut directives = ContentDirectives::default();
    let lines: Vec<&str> = content.lines().collect();
    let mut consumed = 0;

    for line in &lines {
        let trimmed = line.trim();

        // Only consume lines that are known directives
        if !is_directive_line(trimmed) {
            break;
        }

        let rest = trimmed.strip_prefix("#! ").unwrap();

        // Parse directives with arguments (#! tree snapshot=true depth=2)
        let mut words = rest.split_whitespace();
        let keyword = words.next().unwrap_or(rest);
        // Strip trailing `:` for `#! key: value` form
        let keyword = keyword.strip_suffix(':').unwrap_or(keyword);

        match keyword {
            "file" => {
                // #! file: <path> — value is everything after "file: "
                if let Some(path) = rest.strip_prefix("file: ") {
                    directives.file = Some(path.to_string());
                }
            }
            "mdtest" => directives.is_mdtest = true,
            "snapshot" => directives.snapshot = true,
            "copy" => directives.copy = true,
            "assert" => {
                if let Some(value) = rest.strip_prefix("assert: ") {
                    directives.assert_kind = match value {
                        "contains" => Some(AssertKind::Contains),
                        _ => None,
                    };
                }
            }
            "working-dir" => {
                if let Some(value) = rest.strip_prefix("working-dir: ") {
                    directives.working_dir = Some(value.to_string());
                }
            }
            "tree" => {
                let mut tree = TreeDirective::default();
                for arg in words {
                    if let Some(value) = arg.strip_prefix("snapshot=") {
                        tree.snapshot = value == "true";
                    } else if let Some(value) = arg.strip_prefix("depth=") {
                        tree.depth = value.parse().ok();
                    }
                }
                // depth implies snapshot
                if tree.depth.is_some() {
                    tree.snapshot = true;
                }
                directives.tree = Some(tree);
            }
            _ => break,
        }
        consumed += 1;
    }

    // If we consumed any directives, also skip an optional blank line after them
    if consumed > 0 && lines.get(consumed).is_some_and(|l| l.is_empty()) {
        consumed += 1;
    }

    let remaining = lines[consumed..].join("\n");
    (directives, remaining)
}

/// Parse tree content into a list of tree entries.
///
/// Parses content like:
/// ```text
/// .
/// ├── packages/
/// │   ├── alpha/
/// │   └── beta/
/// └── shared -> packages/alpha
/// ```
///
/// Into a list of `TreeEntry` items that can be created.
fn parse_tree_content(content: &str) -> Vec<TreeEntry> {
    let mut entries = Vec::new();
    let mut path_stack: Vec<String> = Vec::new();

    for line in content.lines() {
        // Skip empty lines and the root "." line
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "." {
            continue;
        }

        // Calculate depth by looking for tree drawing characters
        // Each level of indentation is 4 characters: "│   " or "    "
        // The actual entry starts after "├── " or "└── "
        let (depth, name_part) = parse_tree_line(line);

        if name_part.is_empty() {
            continue;
        }

        // Trim the path stack to the current depth
        path_stack.truncate(depth);

        // Check if this is a symlink (contains " -> ")
        if let Some((name, target)) = name_part.split_once(" -> ") {
            let name = name.trim_end_matches('/');
            let full_path: PathBuf = path_stack.iter().collect::<PathBuf>().join(name);
            entries.push(TreeEntry::Symlink {
                path: full_path,
                target: PathBuf::from(target),
            });
            // Symlinks don't get added to path stack (we don't recurse into them)
        } else if name_part.ends_with('/') {
            // This is a directory
            let name = name_part.trim_end_matches('/');
            path_stack.push(name.to_string());
            let full_path: PathBuf = path_stack.iter().collect();
            entries.push(TreeEntry::Directory { path: full_path });
        } else {
            // This is a file
            let full_path: PathBuf = path_stack.iter().collect::<PathBuf>().join(name_part);
            entries.push(TreeEntry::File { path: full_path });
        }
    }

    entries
}

/// Parse a single tree line and return (depth, name).
///
/// Examples:
/// - "├── packages/" -> (0, "packages/")
/// - "│   ├── alpha/" -> (1, "alpha/")
/// - "    └── beta/" -> (1, "beta/")
fn parse_tree_line(line: &str) -> (usize, &str) {
    let mut depth = 0;
    let mut chars = line.chars().peekable();
    let mut byte_pos = 0;

    loop {
        // Look for the start of a connector ("├──" or "└──")
        match chars.peek() {
            Some('├' | '└') => {
                // Found a connector, skip "├── " or "└── " (4 chars, varying bytes)
                // Skip the connector character
                let c = chars.next().unwrap();
                byte_pos += c.len_utf8();

                // Skip "── " (3 chars = 6 bytes for em-dashes + 1 for space = 7, but could be regular dashes)
                // Actually the dashes are "──" which could be regular ASCII or unicode
                // Let's just skip until we hit a non-dash, non-space character that's the name
                while let Some(&c) = chars.peek() {
                    if c == '─' || c == '-' || c == ' ' {
                        chars.next();
                        byte_pos += c.len_utf8();
                    } else {
                        break;
                    }
                }

                // Return the name part
                return (depth, &line[byte_pos..]);
            }
            Some('│') => {
                // Part of indentation, skip "│   " (4 positions)
                chars.next();
                byte_pos += '│'.len_utf8();
                // Skip following spaces (should be 3)
                for _ in 0..3 {
                    if let Some(&c) = chars.peek() {
                        if c == ' ' {
                            chars.next();
                            byte_pos += 1;
                        }
                    }
                }
                depth += 1;
            }
            Some(' ') => {
                // Part of indentation after a "└──" in parent, skip "    " (4 spaces)
                let mut spaces = 0;
                while let Some(&c) = chars.peek() {
                    if c == ' ' && spaces < 4 {
                        chars.next();
                        byte_pos += 1;
                        spaces += 1;
                    } else {
                        break;
                    }
                }
                if spaces == 4 {
                    depth += 1;
                }
            }
            _ => {
                // No more prefix, this is the name (or empty)
                return (depth, &line[byte_pos..]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_test() {
        let source = r#"
# Lock

Tests for lock command.

## Basic locking

```toml
#! file: pyproject.toml

[project]
name = "test"
version = "0.1.0"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        let test = &result.tests[0];
        assert_eq!(test.name, "Lock - Basic locking");

        // Extract files and commands from steps
        let files: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::WriteFile(f) => Some(f),
                _ => None,
            })
            .collect();
        let commands: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::RunCommand(c) => Some(c),
                _ => None,
            })
            .collect();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, PathBuf::from("pyproject.toml"));
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "uv lock");
    }

    #[test]
    fn test_parse_with_file_level_config() {
        let source = r#"
```toml
#! mdtest

[environment]
python-version = "3.12"
exclude-newer = "2024-03-25T00:00:00Z"
```

# Tests

## Test one

```toml
#! file: pyproject.toml

[project]
name = "test"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```

## Test two

```toml
#! file: pyproject.toml

[project]
name = "test2"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 2);

        // Both tests should have the file-level config (raw TOML key is "python-version")
        assert_eq!(
            result.tests[0].config.raw["environment"]["python-version"]
                .as_str()
                .unwrap(),
            "3.12"
        );
        assert_eq!(
            result.tests[1].config.raw["environment"]["python-version"]
                .as_str()
                .unwrap(),
            "3.12"
        );
    }

    #[test]
    fn test_parse_with_file_level_create_venv() {
        let source = r#"
```toml
#! mdtest

[environment]
python-version = "3.12"
create-venv = false
```

# Tests

## Test one

```
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        // Test should have the file-level config including create_venv
        assert_eq!(
            result.tests[0].config.raw["environment"]["python-version"]
                .as_str()
                .unwrap(),
            "3.12"
        );
        assert_eq!(
            result.tests[0].config.raw["environment"]["create-venv"].as_bool(),
            Some(false)
        );
    }

    #[test]
    fn test_parse_sections_are_independent() {
        let source = r#"
# Tests

## Test A

```toml
#! file: a.toml

content = "a"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```

## Test B

```toml
#! file: b.toml

content = "b"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 2);

        // Extract files from steps
        let files_a: Vec<_> = result.tests[0]
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::WriteFile(f) => Some(f),
                _ => None,
            })
            .collect();
        let files_b: Vec<_> = result.tests[1]
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::WriteFile(f) => Some(f),
                _ => None,
            })
            .collect();

        // Test A should only have a.toml
        assert_eq!(files_a.len(), 1);
        assert_eq!(files_a[0].path, PathBuf::from("a.toml"));

        // Test B should only have b.toml (no inheritance from A)
        assert_eq!(files_b.len(), 1);
        assert_eq!(files_b[0].path, PathBuf::from("b.toml"));
    }

    #[test]
    fn test_parse_with_file_snapshot() {
        let source = r#"
# Lock

## With snapshot

```toml
#! file: pyproject.toml

[project]
name = "test"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```

```toml
#! file: uv.lock
#! snapshot

version = 1
requires-python = ">=3.12"
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        let test = &result.tests[0];
        let file_snapshots: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::CheckFileSnapshot(f) => Some(f),
                _ => None,
            })
            .collect();
        assert_eq!(file_snapshots.len(), 1);
        assert_eq!(file_snapshots[0].path, PathBuf::from("uv.lock"));
    }

    #[test]
    fn test_parse_code_block_attributes() {
        let attrs = CodeBlockAttributes::parse("toml");
        assert_eq!(attrs.language.as_deref(), Some("toml"));
        assert!(attrs.extra.is_empty());

        // Fence attributes are now rejected — collected as extra
        let attrs = CodeBlockAttributes::parse("toml title=\"uv.lock\" snapshot=true");
        assert_eq!(attrs.language.as_deref(), Some("toml"));
        assert_eq!(attrs.extra.len(), 2);
    }

    #[test]
    fn test_extract_directives_none() {
        let (dirs, content) = extract_directives("[project]\nname = \"test\"");
        assert!(dirs.file.is_none());
        assert!(!dirs.is_mdtest);
        assert!(!dirs.snapshot);
        assert_eq!(content, "[project]\nname = \"test\"");
    }

    #[test]
    fn test_extract_directives_file() {
        // With blank line after directive
        let (dirs, content) =
            extract_directives("#! file: pyproject.toml\n\n[project]\nname = \"test\"");
        assert_eq!(dirs.file.as_deref(), Some("pyproject.toml"));
        assert_eq!(content, "[project]\nname = \"test\"");

        // Without blank line
        let (dirs, content) =
            extract_directives("#! file: pyproject.toml\n[project]\nname = \"test\"");
        assert_eq!(dirs.file.as_deref(), Some("pyproject.toml"));
        assert_eq!(content, "[project]\nname = \"test\"");

        // Just the directive, no content
        let (dirs, content) = extract_directives("#! file: foo.txt");
        assert_eq!(dirs.file.as_deref(), Some("foo.txt"));
        assert_eq!(content, "");
    }

    #[test]
    fn test_extract_directives_mdtest() {
        // With blank line
        let (dirs, content) =
            extract_directives("#! mdtest\n\n[environment]\npython-version = \"3.12\"");
        assert!(dirs.is_mdtest);
        assert_eq!(content, "[environment]\npython-version = \"3.12\"");

        // Without blank line
        let (dirs, content) =
            extract_directives("#! mdtest\n[environment]\npython-version = \"3.12\"");
        assert!(dirs.is_mdtest);
        assert_eq!(content, "[environment]\npython-version = \"3.12\"");
    }

    #[test]
    fn test_extract_directives_snapshot() {
        let (dirs, content) = extract_directives(
            "#! file: pyproject.toml\n#! snapshot\n\n[project]\nname = \"test\"",
        );
        assert_eq!(dirs.file.as_deref(), Some("pyproject.toml"));
        assert!(dirs.snapshot);
        assert_eq!(content, "[project]\nname = \"test\"");
    }

    #[test]
    fn test_extract_directives_assert() {
        let (dirs, content) =
            extract_directives("#! file: .venv/pyvenv.cfg\n#! assert: contains\nuv =");
        assert_eq!(dirs.file.as_deref(), Some(".venv/pyvenv.cfg"));
        assert_eq!(dirs.assert_kind, Some(AssertKind::Contains));
        assert_eq!(content, "uv =");
    }

    #[test]
    fn test_extract_directives_working_dir() {
        let (dirs, content) =
            extract_directives("#! working-dir: packages/foo\n$ uv lock\nsuccess: true");
        assert_eq!(dirs.working_dir.as_deref(), Some("packages/foo"));
        assert_eq!(content, "$ uv lock\nsuccess: true");
    }

    #[test]
    fn test_extract_directives_tree_create() {
        let (dirs, content) = extract_directives("#! tree\n\n.\n├── foo/");
        assert!(dirs.tree.is_some());
        let tree = dirs.tree.unwrap();
        assert!(!tree.snapshot);
        assert_eq!(content, ".\n├── foo/");
    }

    #[test]
    fn test_extract_directives_tree_snapshot() {
        let (dirs, content) = extract_directives("#! tree snapshot=true depth=2\n\n.\n├── foo/");
        let tree = dirs.tree.unwrap();
        assert!(tree.snapshot);
        assert_eq!(tree.depth, Some(2));
        assert_eq!(content, ".\n├── foo/");
    }

    #[test]
    fn test_extract_directives_tree_depth_implies_snapshot() {
        let (dirs, _) = extract_directives("#! tree depth=1\n.\n├── foo/");
        let tree = dirs.tree.unwrap();
        assert!(tree.snapshot);
        assert_eq!(tree.depth, Some(1));
    }

    #[test]
    fn test_extract_directives_stops_at_unknown() {
        // Regular comment that looks like a directive but isn't known
        let (dirs, content) = extract_directives("# This is a comment\nreal content");
        assert!(dirs.file.is_none());
        assert!(!dirs.is_mdtest);
        assert_eq!(content, "# This is a comment\nreal content");
    }

    #[test]
    fn test_extract_directives_multiple() {
        let (dirs, content) = extract_directives("#! file: uv.lock\n#! snapshot\n\nversion = 1");
        assert_eq!(dirs.file.as_deref(), Some("uv.lock"));
        assert!(dirs.snapshot);
        assert_eq!(content, "version = 1");
    }

    #[test]
    fn test_parse_with_file_title_in_content() {
        let source = r#"
# Lock

## Basic locking

```toml
#! file: pyproject.toml

[project]
name = "test"
version = "0.1.0"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        let test = &result.tests[0];
        let files: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::WriteFile(f) => Some(f),
                _ => None,
            })
            .collect();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, PathBuf::from("pyproject.toml"));
        // Content should not include the title line
        assert!(!files[0].content.contains("#! file:"));
        assert!(files[0].content.contains("[project]"));
    }

    #[test]
    fn test_parse_with_mdtest_marker() {
        let source = r#"
```toml
#! mdtest

[environment]
python-version = "3.12"
```

# Tests

## Test one

```toml
#! file: pyproject.toml

[project]
name = "test"
```

```
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        // Test should have the config from #! mdtest block
        assert_eq!(
            result.tests[0].config.raw["environment"]["python-version"]
                .as_str()
                .unwrap(),
            "3.12"
        );
    }

    #[test]
    fn test_parse_snapshot_with_directives() {
        let source = r#"
# Lock

## Snapshot test

```toml
#! file: pyproject.toml

[project]
name = "test"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```

```toml
#! file: uv.lock
#! snapshot

version = 1
requires-python = ">=3.12"
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        let test = &result.tests[0];
        let snapshots: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::CheckFileSnapshot(f) => Some(f),
                _ => None,
            })
            .collect();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].path, PathBuf::from("uv.lock"));
        assert!(snapshots[0].expected_content.contains("version = 1"));
        // Directives should be stripped from content
        assert!(!snapshots[0].expected_content.contains("#! file:"));
        assert!(!snapshots[0].expected_content.contains("#! snapshot"));
    }

    #[test]
    fn test_parse_assert_with_directives() {
        let source = r#"
# Tests

## Assert test

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```

```text
#! file: .venv/pyvenv.cfg
#! assert: contains

uv =
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        let test = &result.tests[0];
        let assertions: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::CheckContentAssertion(a) => Some(a),
                _ => None,
            })
            .collect();
        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].path, PathBuf::from(".venv/pyvenv.cfg"));
        assert_eq!(assertions[0].kind, AssertKind::Contains);
        assert_eq!(assertions[0].expected, "uv =");
    }

    #[test]
    fn test_parse_working_dir_with_directives() {
        let source = r#"
# Tests

## Working dir test

```toml
#! file: packages/foo/pyproject.toml

[project]
name = "foo"
```

```console
#! working-dir: packages/foo
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        let test = &result.tests[0];
        let commands: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::RunCommand(c) => Some(c),
                _ => None,
            })
            .collect();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "uv lock");
        assert_eq!(commands[0].working_dir, Some(PathBuf::from("packages/foo")));
    }

    #[test]
    fn test_parse_tree_with_directives() {
        let source = r#"
# Tests

## Tree test

```text
#! tree

packages/
└── foo/
```

```console
$ uv version
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```

```text
#! tree snapshot=true depth=1

.
├── packages/
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source).unwrap();
        assert_eq!(result.tests.len(), 1);

        let test = &result.tests[0];

        // First step should be a tree creation (default for # tree)
        let creates: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::CreateTree(t) => Some(t),
                _ => None,
            })
            .collect();
        assert_eq!(creates.len(), 1);

        // Last step should be a tree snapshot with depth
        let tree_snapshots: Vec<_> = test
            .steps
            .iter()
            .filter_map(|s| match s {
                TestStep::CheckTreeSnapshot(t) => Some(t),
                _ => None,
            })
            .collect();
        assert_eq!(tree_snapshots.len(), 1);
        assert_eq!(tree_snapshots[0].depth, Some(1));
        // Directives should be stripped from content
        assert!(!tree_snapshots[0].expected_content.contains("#! tree"));
        assert!(tree_snapshots[0].expected_content.contains("packages/"));
    }

    #[test]
    fn test_parse_skips_blocks_with_fence_attributes() {
        // Blocks with fence attributes (like file=) are skipped with a warning.
        // If ALL blocks are skipped, the file has no tests and returns NoTests error.
        let source = r#"
# Tests

## Bad test

```toml file="pyproject.toml"
[project]
name = "test"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Done
```
"#;

        let result = MarkdownTestFile::parse(PathBuf::from("test.md"), source);
        // The file= block is skipped, but the console block still produces a test
        // (since $ uv lock is a valid command block)
        assert!(result.is_ok());
        let test_file = result.unwrap();
        assert_eq!(test_file.tests.len(), 1);
        // The file creation was skipped, only the command remains
        let steps = &test_file.tests[0].steps;
        let file_count = steps
            .iter()
            .filter(|s| matches!(s, TestStep::WriteFile(_)))
            .count();
        assert_eq!(file_count, 0, "file= block should be skipped");
    }
}
