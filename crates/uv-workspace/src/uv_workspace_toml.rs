//! Support for `uv-workspace.toml`, an alternative project configuration file where
//! `[tool.uv.*]` keys are promoted to top-level keys.
//!
//! When a `uv-workspace.toml` file exists alongside a `pyproject.toml`, uv will read
//! from the `uv-workspace.toml` and sync its contents to the `pyproject.toml`.

use std::path::Path;

use toml_edit::{DocumentMut, Item, Table};

/// The filename for the uv workspace configuration.
pub const UV_WORKSPACE_TOML: &str = "uv-workspace.toml";

/// The keys that live under `[tool.uv]` in `pyproject.toml` but are top-level
/// in `uv-workspace.toml`.
const TOOL_UV_KEYS: &[&str] = &[
    "sources",
    "index",
    "workspace",
    "managed",
    "package",
    "dev-dependencies",
    "default-groups",
    "override-dependencies",
    "constraint-dependencies",
    "environments",
    "conflicts",
    "required-version",
    "cache-keys",
    "build-backend",
    "pip",
    "exclude-newer",
    "index-strategy",
    "keyring-provider",
    "resolution",
    "prerelease",
    "fork-strategy",
    "config-settings",
    "no-build-isolation",
    "no-build-isolation-package",
    "no-build",
    "no-build-package",
    "no-binary",
    "no-binary-package",
    "compile-bytecode",
    "no-sources",
    "upgrade",
    "upgrade-package",
    "reinstall",
    "reinstall-package",
    "python-downloads",
    "python-preference",
    "python-install-mirror",
    "pypy-install-mirror",
    "link-mode",
    "git-lfs",
];

/// Sync the contents of a `uv-workspace.toml` into `pyproject.toml` format.
///
/// This takes the raw content of a `uv-workspace.toml`, remaps top-level keys
/// that belong under `[tool.uv]`, and returns the resulting `pyproject.toml` content.
pub fn sync_to_pyproject(uv_workspace_content: &str) -> Result<String, toml_edit::TomlError> {
    let uv_doc: DocumentMut = uv_workspace_content.parse()?;
    let mut pyproject_doc = DocumentMut::new();

    // Copy non-tool-uv keys directly (e.g., `[project]`, `[build-system]`, `[dependency-groups]`).
    // Move tool-uv keys into `[tool.uv]`.
    let mut tool_uv_table = Table::new();
    let mut has_tool_uv = false;

    for (key, item) in uv_doc.as_table().iter() {
        if is_tool_uv_key(key) {
            tool_uv_table.insert(key, item.clone());
            has_tool_uv = true;
        } else {
            pyproject_doc.insert(key, item.clone());
        }
    }

    if has_tool_uv {
        // Create `[tool]` table if needed.
        let tool = pyproject_doc
            .entry("tool")
            .or_insert(Item::Table(Table::new()));
        if let Some(tool_table) = tool.as_table_mut() {
            tool_table.insert("uv", Item::Table(tool_uv_table));
        }
    }

    Ok(pyproject_doc.to_string())
}

/// Returns `true` if the given key is a `[tool.uv]` key that should be at the
/// top level in `uv-workspace.toml`.
fn is_tool_uv_key(key: &str) -> bool {
    TOOL_UV_KEYS.contains(&key)
}

/// Check if a `uv-workspace.toml` file exists at the given project root.
pub fn has_uv_workspace_toml(root: &Path) -> bool {
    root.join(UV_WORKSPACE_TOML).is_file()
}

/// Read the `uv-workspace.toml` file at the given project root.
pub fn read_uv_workspace_toml(root: &Path) -> Result<String, std::io::Error> {
    fs_err::read_to_string(root.join(UV_WORKSPACE_TOML))
}

/// If a `uv-workspace.toml` exists at the given root, sync it to `pyproject.toml`
/// and return the synced pyproject.toml content. Otherwise return `None`.
///
/// This ensures the `pyproject.toml` is up-to-date before workspace discovery reads it.
pub fn sync_if_needed(root: &Path) -> Result<Option<String>, std::io::Error> {
    let uv_workspace_path = root.join(UV_WORKSPACE_TOML);
    if !uv_workspace_path.is_file() {
        return Ok(None);
    }

    let uv_workspace_content = fs_err::read_to_string(&uv_workspace_path)?;
    let pyproject_content =
        sync_to_pyproject(&uv_workspace_content).map_err(std::io::Error::other)?;

    // Write the synced pyproject.toml to disk so other tools can read it.
    fs_err::write(root.join("pyproject.toml"), &pyproject_content)?;

    Ok(Some(pyproject_content))
}

/// Write the updated content to both the `uv-workspace.toml` and the synced `pyproject.toml`.
///
/// Returns `true` if either file was modified.
pub fn write_uv_workspace_toml(
    root: &Path,
    uv_workspace_content: &str,
    original_uv_workspace_content: &str,
) -> Result<bool, std::io::Error> {
    if uv_workspace_content == original_uv_workspace_content {
        return Ok(false);
    }

    // Write the uv-workspace.toml.
    fs_err::write(root.join(UV_WORKSPACE_TOML), uv_workspace_content)?;

    // Sync to pyproject.toml.
    let pyproject_content =
        sync_to_pyproject(uv_workspace_content).map_err(|e| std::io::Error::other(e))?;
    fs_err::write(root.join("pyproject.toml"), pyproject_content)?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_to_pyproject_basic() {
        let uv_workspace = r#"
[project]
name = "my-project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio>=3.7.0"]

[sources]
anyio = { git = "https://github.com/agronholm/anyio", tag = "3.7.0" }
"#;

        let result = sync_to_pyproject(uv_workspace).unwrap();
        let doc: DocumentMut = result.parse().unwrap();

        // project should be at top level
        assert!(doc.get("project").is_some());

        // sources should be under tool.uv
        let tool_uv = doc
            .get("tool")
            .and_then(|t| t.as_table())
            .and_then(|t| t.get("uv"))
            .and_then(|t| t.as_table());
        assert!(tool_uv.is_some());
        assert!(tool_uv.unwrap().get("sources").is_some());

        // sources should NOT be at top level
        assert!(doc.get("sources").is_none());
    }

    #[test]
    fn test_sync_to_pyproject_no_uv_keys() {
        let uv_workspace = r#"
[project]
name = "my-project"
version = "0.1.0"
dependencies = []
"#;

        let result = sync_to_pyproject(uv_workspace).unwrap();
        let doc: DocumentMut = result.parse().unwrap();

        assert!(doc.get("project").is_some());
        assert!(doc.get("tool").is_none());
    }

    #[test]
    fn test_sync_to_pyproject_workspace_and_index() {
        let uv_workspace = r#"
[project]
name = "my-project"
version = "0.1.0"
dependencies = []

[workspace]
members = ["packages/*"]

[[index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cu121"
"#;

        let result = sync_to_pyproject(uv_workspace).unwrap();
        let doc: DocumentMut = result.parse().unwrap();

        let tool_uv = doc
            .get("tool")
            .and_then(|t| t.as_table())
            .and_then(|t| t.get("uv"))
            .and_then(|t| t.as_table())
            .unwrap();

        assert!(tool_uv.get("workspace").is_some());
        assert!(tool_uv.get("index").is_some());
    }
}
