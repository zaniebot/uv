//! Support for `uv-workspace.toml`, an alternative project configuration file where
//! `[tool.uv.*]` keys are promoted to top-level keys.
//!
//! When a `uv-workspace.toml` file exists alongside a `pyproject.toml`, uv will read
//! from the `uv-workspace.toml` and sync its contents to the `pyproject.toml`.
//!
//! ## Workspace dependency inheritance
//!
//! The workspace root `uv-workspace.toml` can define shared dependency constraints
//! under `[workspace.dependencies]`:
//!
//! ```toml
//! [workspace]
//! members = ["packages/*"]
//! dependencies = { requests = ">=2.28", flask = ">=2.0" }
//! ```
//!
//! Workspace members can then reference these with `{ workspace = "pkg" }` in their
//! dependency arrays:
//!
//! ```toml
//! [project]
//! dependencies = [
//!     { workspace = "requests" },
//! ]
//! ```
//!
//! When syncing to `pyproject.toml`, these are resolved to standard PEP 508 strings:
//!
//! ```toml
//! [project]
//! dependencies = [
//!     "requests>=2.28",
//! ]
//! ```

use std::collections::BTreeMap;
use std::path::Path;

use toml_edit::{Array, DocumentMut, Item, Table, Value};

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
///
/// If `workspace_deps` is provided, any `{ workspace = "pkg" }` entries in dependency
/// arrays are resolved to PEP 508 strings using the workspace-level constraints.
pub fn sync_to_pyproject(
    uv_workspace_content: &str,
    workspace_deps: Option<&BTreeMap<String, String>>,
) -> Result<String, toml_edit::TomlError> {
    let uv_doc: DocumentMut = uv_workspace_content.parse()?;
    let mut pyproject_doc = DocumentMut::new();

    // Copy non-tool-uv keys directly (e.g., `[project]`, `[build-system]`, `[dependency-groups]`).
    // Move tool-uv keys into `[tool.uv]`.
    let mut tool_uv_table = Table::new();
    let mut has_tool_uv = false;

    for (key, item) in uv_doc.as_table().iter() {
        if is_tool_uv_key(key) {
            // Strip `workspace.dependencies` before syncing — it's only used for
            // resolving `{ workspace = "pkg" }` references in members.
            if key == "workspace" {
                if let Some(table) = item.as_table() {
                    let mut filtered = table.clone();
                    filtered.remove("dependencies");
                    if !filtered.is_empty() {
                        tool_uv_table.insert(key, Item::Table(filtered));
                        has_tool_uv = true;
                    }
                } else {
                    tool_uv_table.insert(key, item.clone());
                    has_tool_uv = true;
                }
            } else {
                tool_uv_table.insert(key, item.clone());
                has_tool_uv = true;
            }
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

    // Resolve `{ workspace = "pkg" }` entries in dependency arrays.
    if let Some(ws_deps) = workspace_deps {
        resolve_workspace_dependencies(&mut pyproject_doc, ws_deps);
    }

    Ok(pyproject_doc.to_string())
}

/// Extract `[workspace.dependencies]` from a `uv-workspace.toml` content string.
///
/// Returns a map of package name to version specifier string.
pub fn extract_workspace_dependencies(
    uv_workspace_content: &str,
) -> Result<BTreeMap<String, String>, toml_edit::TomlError> {
    let doc: DocumentMut = uv_workspace_content.parse()?;
    let mut deps = BTreeMap::new();

    if let Some(workspace) = doc.get("workspace").and_then(Item::as_table) {
        if let Some(dep_item) = workspace.get("dependencies") {
            // Handle both `[workspace.dependencies]` (regular table) and
            // `dependencies = { ... }` (inline table).
            if let Some(table) = dep_item.as_table() {
                for (name, value) in table.iter() {
                    if let Some(version) = value.as_str() {
                        deps.insert(name.to_string(), version.to_string());
                    }
                }
            } else if let Some(inline) = dep_item.as_inline_table() {
                for (name, value) in inline.iter() {
                    if let Some(version) = value.as_str() {
                        deps.insert(name.to_string(), version.to_string());
                    }
                }
            }
        }
    }

    Ok(deps)
}

/// Resolve `{ workspace = "pkg" }` inline tables in dependency arrays to PEP 508 strings.
///
/// This processes `project.dependencies`, `project.optional-dependencies.*`, and
/// `dependency-groups.*` arrays.
fn resolve_workspace_dependencies(
    doc: &mut DocumentMut,
    workspace_deps: &BTreeMap<String, String>,
) {
    // Resolve in `project.dependencies`.
    if let Some(project) = doc.get_mut("project").and_then(|p| p.as_table_mut()) {
        if let Some(deps) = project
            .get_mut("dependencies")
            .and_then(|d| d.as_array_mut())
        {
            resolve_dep_array(deps, workspace_deps);
        }

        // Resolve in `project.optional-dependencies.*`.
        if let Some(opt_deps) = project
            .get_mut("optional-dependencies")
            .and_then(|d| d.as_table_mut())
        {
            for (_group, items) in opt_deps.iter_mut() {
                if let Some(arr) = items.as_array_mut() {
                    resolve_dep_array(arr, workspace_deps);
                }
            }
        }
    }

    // Resolve in `dependency-groups.*`.
    if let Some(groups) = doc
        .get_mut("dependency-groups")
        .and_then(|d| d.as_table_mut())
    {
        for (_group, items) in groups.iter_mut() {
            if let Some(arr) = items.as_array_mut() {
                resolve_dep_array(arr, workspace_deps);
            }
        }
    }
}

/// Resolve `{ workspace = "pkg" }` entries in a single TOML array to PEP 508 strings.
fn resolve_dep_array(array: &mut Array, workspace_deps: &BTreeMap<String, String>) {
    let mut i = 0;
    while i < array.len() {
        let should_replace = if let Some(inline) = array.get(i).and_then(Value::as_inline_table) {
            inline.get("workspace").and_then(Value::as_str).is_some()
        } else {
            false
        };

        if should_replace {
            let inline = array.get(i).unwrap().as_inline_table().unwrap();
            let pkg_name = inline.get("workspace").unwrap().as_str().unwrap();

            // Look up the version specifier from workspace dependencies.
            let version_spec = workspace_deps
                .get(pkg_name)
                .map(String::as_str)
                .unwrap_or("");

            let pep508 = if version_spec.is_empty() {
                pkg_name.to_string()
            } else {
                format!("{pkg_name}{version_spec}")
            };

            array.replace(i, Value::from(pep508));
        }
        i += 1;
    }
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
    sync_if_needed_with_deps(root, None)
}

/// Like [`sync_if_needed`], but resolves `{ workspace = "pkg" }` entries in dependency
/// arrays using the provided workspace-level dependency constraints.
pub fn sync_if_needed_with_deps(
    root: &Path,
    workspace_deps: Option<&BTreeMap<String, String>>,
) -> Result<Option<String>, std::io::Error> {
    let uv_workspace_path = root.join(UV_WORKSPACE_TOML);
    if !uv_workspace_path.is_file() {
        return Ok(None);
    }

    let uv_workspace_content = fs_err::read_to_string(&uv_workspace_path)?;
    let pyproject_content =
        sync_to_pyproject(&uv_workspace_content, workspace_deps).map_err(std::io::Error::other)?;

    // Write the synced pyproject.toml to disk so other tools can read it.
    fs_err::write(root.join("pyproject.toml"), &pyproject_content)?;

    Ok(Some(pyproject_content))
}

/// Read workspace-level dependencies from the workspace root's `uv-workspace.toml`.
///
/// Returns `None` if there is no `uv-workspace.toml` or no `[workspace.dependencies]`.
pub fn read_workspace_dependencies(
    workspace_root: &Path,
) -> Result<Option<BTreeMap<String, String>>, std::io::Error> {
    let uv_workspace_path = workspace_root.join(UV_WORKSPACE_TOML);
    if !uv_workspace_path.is_file() {
        return Ok(None);
    }

    let content = fs_err::read_to_string(&uv_workspace_path)?;
    let deps = extract_workspace_dependencies(&content).map_err(std::io::Error::other)?;

    if deps.is_empty() {
        Ok(None)
    } else {
        Ok(Some(deps))
    }
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
        sync_to_pyproject(uv_workspace_content, None).map_err(|e| std::io::Error::other(e))?;
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

        let result = sync_to_pyproject(uv_workspace, None).unwrap();
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

        let result = sync_to_pyproject(uv_workspace, None).unwrap();
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

        let result = sync_to_pyproject(uv_workspace, None).unwrap();
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

    #[test]
    fn test_extract_workspace_dependencies() {
        let content = r#"
[workspace]
members = ["packages/*"]
dependencies = { requests = ">=2.28", flask = ">=2.0,<3" }
"#;
        let deps = extract_workspace_dependencies(content).unwrap();
        assert_eq!(deps.get("requests").unwrap(), ">=2.28");
        assert_eq!(deps.get("flask").unwrap(), ">=2.0,<3");
    }

    #[test]
    fn test_sync_resolves_workspace_deps() {
        let member_content = r#"
[project]
name = "my-package"
version = "0.1.0"
dependencies = [
    "local>=1.0",
    { workspace = "requests" },
    { workspace = "flask" },
]
"#;
        let mut ws_deps = BTreeMap::new();
        ws_deps.insert("requests".to_string(), ">=2.28".to_string());
        ws_deps.insert("flask".to_string(), ">=2.0,<3".to_string());

        let result = sync_to_pyproject(member_content, Some(&ws_deps)).unwrap();
        assert!(result.contains("\"requests>=2.28\""));
        assert!(result.contains("\"flask>=2.0,<3\""));
        assert!(result.contains("\"local>=1.0\""));
    }

    #[test]
    fn test_sync_strips_workspace_dependencies_from_output() {
        let root_content = r#"
[workspace]
members = ["packages/*"]
dependencies = { requests = ">=2.28" }
"#;
        let result = sync_to_pyproject(root_content, None).unwrap();
        let doc: DocumentMut = result.parse().unwrap();

        // workspace.dependencies should not appear in synced pyproject.toml
        let tool_uv = doc
            .get("tool")
            .and_then(|t| t.as_table())
            .and_then(|t| t.get("uv"))
            .and_then(|t| t.as_table());

        // The workspace table should still exist (for members), but without dependencies
        if let Some(uv) = tool_uv {
            if let Some(ws) = uv.get("workspace").and_then(Item::as_table) {
                assert!(ws.get("dependencies").is_none());
            }
        }
    }
}
