use std::path::Path;

use anyhow::Result;
use rustc_hash::FxHashSet;
use thiserror::Error;

use uv_configuration::{LoweredUpgrade, Upgrade};
use uv_fs::CWD;
use uv_git::ResolvedRepositoryReference;
use uv_normalize::{GroupName, PackageName};
use uv_requirements_txt::RequirementsTxt;
use uv_resolver::{Lock, LockError, Preference, PreferenceError, PylockToml, PylockTomlErrorKind};
use uv_workspace::dependency_groups::FlatDependencyGroups;

/// Error returned when upgrade groups are specified but cannot be resolved.
#[derive(Debug, Error)]
#[error("`--upgrade-group` requires a project with dependency groups defined in `pyproject.toml`")]
pub struct UpgradeGroupError {
    /// The groups that were specified.
    pub groups: FxHashSet<GroupName>,
}

/// Lower an [`Upgrade`] to a [`LoweredUpgrade`] by resolving groups to packages
/// using the dependency groups from pyproject.toml.
///
/// This is the preferred method for lowering upgrades, as it uses the source of
/// truth for dependency groups.
pub fn lower_upgrade(upgrade: &Upgrade, dependency_groups: &FlatDependencyGroups) -> LoweredUpgrade {
    match upgrade {
        Upgrade::None => LoweredUpgrade::None,
        Upgrade::All => LoweredUpgrade::All,
        Upgrade::Packages(packages) => LoweredUpgrade::Packages(packages.clone()),
        Upgrade::Groups(groups) => {
            let packages = resolve_groups_from_pyproject(dependency_groups, groups);
            LoweredUpgrade::Packages(packages.into_iter().map(|p| (p, vec![])).collect())
        }
        Upgrade::PackagesAndGroups { packages, groups } => {
            let group_packages = resolve_groups_from_pyproject(dependency_groups, groups);
            let mut all_packages = packages.clone();
            for pkg in group_packages {
                all_packages.entry(pkg).or_default();
            }
            LoweredUpgrade::Packages(all_packages)
        }
    }
}

/// Lower an [`Upgrade`] to a [`LoweredUpgrade`] by resolving groups to packages
/// using the lock file's dependency group information.
///
/// This is useful when pyproject.toml is not available, but a lock file is.
pub fn lower_upgrade_with_lock(upgrade: &Upgrade, lock: &Lock) -> LoweredUpgrade {
    match upgrade {
        Upgrade::None => LoweredUpgrade::None,
        Upgrade::All => LoweredUpgrade::All,
        Upgrade::Packages(packages) => LoweredUpgrade::Packages(packages.clone()),
        Upgrade::Groups(groups) => {
            let packages = resolve_groups_from_lock(lock, groups);
            LoweredUpgrade::Packages(packages.into_iter().map(|p| (p, vec![])).collect())
        }
        Upgrade::PackagesAndGroups { packages, groups } => {
            let group_packages = resolve_groups_from_lock(lock, groups);
            let mut all_packages = packages.clone();
            for pkg in group_packages {
                all_packages.entry(pkg).or_default();
            }
            LoweredUpgrade::Packages(all_packages)
        }
    }
}

/// Lower an [`Upgrade`] to a [`LoweredUpgrade`] without group resolution.
///
/// This is used for contexts where dependency groups are not available (e.g., pip
/// commands without a pyproject.toml). Returns an error if the upgrade specification
/// contains groups.
pub fn lower_upgrade_without_groups(
    upgrade: &Upgrade,
) -> std::result::Result<LoweredUpgrade, UpgradeGroupError> {
    match upgrade {
        Upgrade::None => Ok(LoweredUpgrade::None),
        Upgrade::All => Ok(LoweredUpgrade::All),
        Upgrade::Packages(packages) => Ok(LoweredUpgrade::Packages(packages.clone())),
        Upgrade::Groups(groups) => Err(UpgradeGroupError {
            groups: groups.clone(),
        }),
        Upgrade::PackagesAndGroups { groups, .. } => Err(UpgradeGroupError {
            groups: groups.clone(),
        }),
    }
}

/// Resolve group names to package names using pyproject.toml's dependency groups.
fn resolve_groups_from_pyproject(
    dependency_groups: &FlatDependencyGroups,
    groups: &FxHashSet<GroupName>,
) -> FxHashSet<PackageName> {
    groups
        .iter()
        .filter_map(|group| dependency_groups.get(group))
        .flat_map(|group| group.requirements.iter().map(|req| req.name.clone()))
        .collect()
}

/// Resolve group names to package names using the lock file.
///
/// Looks at both manifest-level dependency groups (for projects without [project] table)
/// and package-level dependency groups (the standard case).
fn resolve_groups_from_lock(lock: &Lock, groups: &FxHashSet<GroupName>) -> FxHashSet<PackageName> {
    // First, check manifest-level dependency groups (for projects without [project] table).
    let manifest_packages = lock
        .dependency_groups()
        .iter()
        .filter(|(group, _)| groups.contains(group))
        .flat_map(|(_, requirements)| requirements.iter().map(|req| req.name.clone()));

    // Then, check package-level dependency groups (the standard case).
    let package_packages = lock.packages().iter().flat_map(|package| {
        package
            .dependency_groups()
            .iter()
            .filter(|(group, _)| groups.contains(group))
            .flat_map(|(_, requirements)| requirements.iter().map(|req| req.name.clone()))
    });

    manifest_packages.chain(package_packages).collect()
}

#[derive(Debug, Default)]
pub struct LockedRequirements {
    /// The pinned versions from the lockfile.
    pub preferences: Vec<Preference>,
    /// The pinned Git SHAs from the lockfile.
    pub git: Vec<ResolvedRepositoryReference>,
}

impl LockedRequirements {
    /// Create a [`LockedRequirements`] from a list of preferences.
    pub fn from_preferences(preferences: Vec<Preference>) -> Self {
        Self {
            preferences,
            ..Self::default()
        }
    }
}

/// Load the preferred requirements from an existing `requirements.txt`, applying the upgrade strategy.
pub async fn read_requirements_txt(
    output_file: &Path,
    upgrade: &Upgrade,
) -> Result<Vec<Preference>> {
    // As an optimization, skip reading the lockfile is we're upgrading all packages anyway.
    if upgrade.is_all() {
        return Ok(Vec::new());
    }

    // Parse the requirements from the lockfile.
    let requirements_txt = RequirementsTxt::parse(output_file, &*CWD).await?;

    // Map each entry in the lockfile to a preference.
    let preferences = requirements_txt
        .requirements
        .into_iter()
        .map(Preference::from_entry)
        .filter_map(Result::transpose)
        .collect::<Result<Vec<_>, PreferenceError>>()?;

    // Apply the upgrade strategy to the requirements.
    Ok(match upgrade {
        // Respect all pinned versions from the existing lockfile.
        Upgrade::None => preferences,
        // Ignore all pinned versions from the existing lockfile.
        Upgrade::All => vec![],
        // Ignore pinned versions for the specified packages.
        Upgrade::Packages(packages) => preferences
            .into_iter()
            .filter(|preference| !packages.contains_key(preference.name()))
            .collect(),
        // For groups, we can't determine group membership in requirements.txt context,
        // so we respect all pinned versions (groups are only meaningful for uv.lock).
        Upgrade::Groups(_) => preferences,
        // For packages and groups, filter the explicitly specified packages.
        // Groups can't be resolved in requirements.txt context.
        Upgrade::PackagesAndGroups { packages, .. } => preferences
            .into_iter()
            .filter(|preference| !packages.contains_key(preference.name()))
            .collect(),
    })
}

/// Load the preferred requirements from an existing lockfile, applying the upgrade strategy.
pub fn read_lock_requirements(
    lock: &Lock,
    install_path: &Path,
    upgrade: &Upgrade,
) -> Result<LockedRequirements, LockError> {
    // Lower the upgrade specification by resolving groups to packages.
    let lowered = lower_upgrade_with_lock(upgrade, lock);

    // As an optimization, skip iterating over the lockfile if we're upgrading all packages anyway.
    if lowered.is_all() {
        return Ok(LockedRequirements::default());
    }

    let mut preferences = Vec::new();
    let mut git = Vec::new();

    for package in lock.packages() {
        // Skip the distribution if it should be upgraded.
        if lowered.contains(package.name()) {
            continue;
        }

        // Map each entry in the lockfile to a preference.
        if let Some(preference) = Preference::from_lock(package, install_path)? {
            preferences.push(preference);
        }

        // Map each entry in the lockfile to a Git SHA.
        if let Some(git_ref) = package.as_git_ref()? {
            git.push(git_ref);
        }
    }

    Ok(LockedRequirements { preferences, git })
}

/// Load the preferred requirements from an existing `pylock.toml` file, applying the upgrade strategy.
pub async fn read_pylock_toml_requirements(
    output_file: &Path,
    upgrade: &Upgrade,
) -> Result<LockedRequirements, PylockTomlErrorKind> {
    // Lower the upgrade specification. Groups are not supported in pylock.toml context.
    let lowered = lower_upgrade_without_groups(upgrade)
        .expect("--upgrade-group is not supported with pylock.toml");

    // As an optimization, skip iterating over the lockfile if we're upgrading all packages anyway.
    if lowered.is_all() {
        return Ok(LockedRequirements::default());
    }

    // Read the `pylock.toml` from disk, and deserialize it from TOML.
    let content = fs_err::tokio::read_to_string(&output_file).await?;
    let lock = toml::from_str::<PylockToml>(&content)?;

    let mut preferences = Vec::new();
    let mut git = Vec::new();

    for package in &lock.packages {
        // Skip the distribution if it should be upgraded.
        if lowered.contains(&package.name) {
            continue;
        }

        // Map each entry in the lockfile to a preference.
        if let Some(preference) = Preference::from_pylock_toml(package)? {
            preferences.push(preference);
        }

        // Map each entry in the lockfile to a Git SHA.
        if let Some(git_ref) = package.as_git_ref() {
            git.push(git_ref);
        }
    }

    Ok(LockedRequirements { preferences, git })
}
