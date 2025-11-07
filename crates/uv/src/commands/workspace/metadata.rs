use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Serialize;

use uv_fs::PortablePathBuf;
use uv_normalize::{ExtraName, GroupName, PackageName};
use uv_pep440::Version;
use uv_preview::{Preview, PreviewFeatures};
use uv_resolver::Lock;
use uv_warnings::warn_user;
use uv_workspace::{DiscoveryOptions, Workspace, WorkspaceCache};

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// The schema version for the metadata report.
#[derive(Serialize, Debug)]
#[serde(untagged)]
enum SchemaVersion {
    /// Schema version number
    Version(u32),
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self::Version(1)
    }
}

/// Source type for a package
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum PackageSource {
    Registry {
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    },
    Git {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        subdirectory: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rev: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        branch: Option<String>,
    },
    Direct {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        subdirectory: Option<String>,
    },
    Path {
        path: String,
    },
    Directory {
        path: String,
    },
    Editable {
        path: String,
    },
    Virtual {
        path: String,
    },
}

/// Package identifier
#[derive(Serialize, Debug, Clone)]
struct PackageId {
    name: PackageName,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<Version>,
    source: PackageSource,
}

/// A dependency in the resolved graph
#[derive(Serialize, Debug, Clone)]
struct ResolvedDependency {
    /// Package ID of the dependency
    #[serde(flatten)]
    id: PackageId,
    /// Requested extras
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    extras: BTreeSet<ExtraName>,
    /// Environment marker (if conditional)
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<String>,
}

/// Metadata for a single package (from manifest)
#[derive(Serialize, Debug, Clone)]
struct PackageManifestMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    requires_python: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    authors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    keywords: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    classifiers: Vec<String>,
}

/// A package in the workspace
#[derive(Serialize, Debug, Clone)]
struct Package {
    /// Package identifier
    #[serde(flatten)]
    id: PackageId,
    /// Path to the package's pyproject.toml (for workspace members)
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest_path: Option<PathBuf>,
    /// Dependencies (as declared in the manifest)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    dependencies: Vec<String>,
    /// Optional dependencies (extras)
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    optional_dependencies: BTreeMap<ExtraName, Vec<String>>,
    /// Dependency groups (PEP 735)
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    dependency_groups: BTreeMap<GroupName, Vec<String>>,
    /// Package metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<PackageManifestMetadata>,
}

/// A node in the resolved dependency graph
#[derive(Serialize, Debug, Clone)]
struct ResolveNode {
    /// Package ID
    #[serde(flatten)]
    id: PackageId,
    /// Resolved dependencies
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    dependencies: Vec<ResolvedDependency>,
}

/// The resolved dependency graph
#[derive(Serialize, Debug)]
struct Resolve {
    /// Nodes in the dependency graph
    packages: Vec<ResolveNode>,
}

/// The report for a metadata operation.
#[derive(Serialize, Debug)]
struct MetadataReport {
    /// The schema version
    version: SchemaVersion,
    /// The Python version requirement
    #[serde(skip_serializing_if = "Option::is_none")]
    requires_python: Option<String>,
    /// The workspace root directory
    workspace_root: PortablePathBuf,
    /// Workspace member names
    workspace_members: Vec<PackageName>,
    /// All packages (workspace members + dependencies)
    packages: Vec<Package>,
    /// The resolved dependency graph (if lock file exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve: Option<Resolve>,
}

/// Build a Package from a lock file package
fn package_from_lock(lock_pkg: &uv_resolver::Package, root: &Path) -> Result<Package> {
    // Use available public APIs
    let name = lock_pkg.name().clone();
    let version = lock_pkg.version().cloned();

    // Determine source type using available public methods
    let source = if let Ok(Some(index)) = lock_pkg.index(root) {
        PackageSource::Registry {
            url: Some(index.to_string()),
        }
    } else if let Ok(Some(git_ref)) = lock_pkg.as_git_ref() {
        PackageSource::Git {
            url: git_ref.reference.url.to_string(),
            subdirectory: None,
            rev: Some(git_ref.sha.to_string()),
            tag: None,
            branch: None,
        }
    } else {
        // Default to registry if we can't determine
        PackageSource::Registry { url: None }
    };

    // Extract dependency groups as strings
    let dependency_groups = lock_pkg
        .dependency_groups()
        .iter()
        .map(|(name, reqs)| {
            let deps: Vec<String> = reqs.iter().map(|r| r.to_string()).collect();
            (name.clone(), deps)
        })
        .collect();

    Ok(Package {
        id: PackageId {
            name,
            version,
            source,
        },
        manifest_path: None,
        dependencies: vec![],
        optional_dependencies: BTreeMap::new(),
        dependency_groups,
        metadata: None,
    })
}

/// Build a Package from a workspace member
fn package_from_member(member: &uv_workspace::WorkspaceMember, _root: &Path) -> Package {
    let project = member.project();

    // Extract dependencies as strings
    let dependencies = project
        .dependencies
        .as_ref()
        .map(|deps| deps.clone())
        .unwrap_or_default();

    // Extract optional dependencies
    let optional_dependencies = project
        .optional_dependencies
        .as_ref()
        .map(|opt_deps| opt_deps.clone())
        .unwrap_or_default();

    // Extract dependency groups
    let dependency_groups = member
        .pyproject_toml()
        .dependency_groups
        .as_ref()
        .map(|groups| {
            groups
                .iter()
                .map(|(name, specs)| {
                    let reqs: Vec<String> = specs
                        .iter()
                        .filter_map(|spec| {
                            if let uv_pypi_types::DependencyGroupSpecifier::Requirement(req) = spec {
                                Some(req.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    (name.clone(), reqs)
                })
                .collect()
        })
        .unwrap_or_default();

    // Build metadata
    let metadata = Some(PackageManifestMetadata {
        requires_python: project.requires_python.as_ref().map(ToString::to_string),
        description: None,  // TODO: Access from PyProjectToml
        authors: vec![],    // TODO: Access from PyProjectToml
        license: None,      // TODO: Access from PyProjectToml
        keywords: vec![],   // TODO: Access from PyProjectToml
        classifiers: vec![], // TODO: Access from PyProjectToml
    });

    Package {
        id: PackageId {
            name: project.name.clone(),
            version: project.version.clone(),
            source: PackageSource::Directory {
                path: member.root().display().to_string(),
            },
        },
        manifest_path: Some(member.root().join("pyproject.toml")),
        dependencies,
        optional_dependencies,
        dependency_groups,
        metadata,
    }
}

/// Display package metadata.
pub(crate) async fn metadata(
    project_dir: &Path,
    preview: Preview,
    printer: Printer,
) -> Result<ExitStatus> {
    if preview.is_enabled(PreviewFeatures::WORKSPACE_METADATA) {
        warn_user!(
            "The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features {}` to disable this warning.",
            PreviewFeatures::WORKSPACE_METADATA
        );
    }

    let workspace_cache = WorkspaceCache::default();
    let workspace =
        Workspace::discover(project_dir, &DiscoveryOptions::default(), &workspace_cache).await?;

    let root = workspace.install_path();

    // Collect workspace member names
    let workspace_members: Vec<PackageName> = workspace
        .packages()
        .values()
        .map(|package| package.project().name.clone())
        .collect();

    // Build packages from workspace members
    let mut packages: Vec<Package> = workspace
        .packages()
        .values()
        .map(|member| package_from_member(member, root))
        .collect();

    // Try to load the lock file
    let lock_path = root.join("uv.lock");
    let (resolve, lock_packages, requires_python) = if lock_path.exists() {
        let lock_content = fs_err::read_to_string(&lock_path)
            .context("Failed to read uv.lock")?;

        match toml::from_str::<Lock>(&lock_content) {
            Ok(lock) => {
                // Get requires_python from lock file
                let requires_python = Some(lock.requires_python().to_string());

                // Add packages from lock file that aren't workspace members
                let workspace_names: BTreeSet<_> = workspace_members.iter().collect();

                let mut lock_packages = vec![];
                let mut resolve_nodes = vec![];

                for lock_pkg in lock.packages() {
                    // Convert lock package to our Package type
                    if let Ok(pkg) = package_from_lock(lock_pkg, root) {
                        // Build resolve node
                        let resolve_node = ResolveNode {
                            id: pkg.id.clone(),
                            dependencies: vec![], // TODO: Add dependencies when we have public access
                        };
                        resolve_nodes.push(resolve_node);

                        // Only add to packages list if it's not a workspace member
                        if !workspace_names.contains(&pkg.id.name) {
                            lock_packages.push(pkg);
                        }
                    }
                }

                let resolve = if !resolve_nodes.is_empty() {
                    Some(Resolve {
                        packages: resolve_nodes,
                    })
                } else {
                    None
                };

                (resolve, lock_packages, requires_python)
            }
            Err(e) => {
                // If lock file exists but can't be read, warn but continue
                warn_user!("Failed to read uv.lock: {e}");
                let requires_python = workspace
                    .packages()
                    .values()
                    .next()
                    .and_then(|member| {
                        member.project().requires_python.as_ref().map(ToString::to_string)
                    });
                (None, vec![], requires_python)
            }
        }
    } else {
        // No lock file, get requires_python from workspace
        let requires_python = workspace
            .packages()
            .values()
            .next()
            .and_then(|member| {
                member.project().requires_python.as_ref().map(ToString::to_string)
            });
        (None, vec![], requires_python)
    };

    // Add lock file packages to the packages list
    packages.extend(lock_packages);

    let report = MetadataReport {
        version: SchemaVersion::default(),
        requires_python,
        workspace_root: PortablePathBuf::from(root as &Path),
        workspace_members,
        packages,
        resolve,
    };

    writeln!(
        printer.stdout(),
        "{}",
        serde_json::to_string_pretty(&report)?
    )?;

    Ok(ExitStatus::Success)
}
