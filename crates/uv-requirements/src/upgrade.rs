use std::path::Path;

use anyhow::Result;

use uv_configuration::Upgrade;
use uv_fs::CWD;
use uv_git::ResolvedRepositoryReference;
use uv_requirements_txt::RequirementsTxt;
use uv_resolver::{Lock, LockError, Preference, PreferenceError, PylockToml, PylockTomlErrorKind};

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
    // As an optimization, skip iterating over the lockfile is we're upgrading all packages anyway.
    if upgrade.is_all() {
        return Ok(LockedRequirements::default());
    }

    // If upgrading by group, collect the package names from the specified groups.
    // We look at both the manifest-level dependency groups and the package-level
    // dependency groups (from all packages in the lock, typically the root package).
    let group_packages: Option<rustc_hash::FxHashSet<_>> = upgrade.groups().map(|groups| {
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
    });

    let mut preferences = Vec::new();
    let mut git = Vec::new();

    for package in lock.packages() {
        // Skip the distribution if it's not included in the upgrade strategy.
        if upgrade.contains(package.name()) {
            continue;
        }

        // If upgrading by group, skip packages that are direct dependencies of the specified groups.
        if let Some(ref group_pkgs) = group_packages {
            if group_pkgs.contains(package.name()) {
                continue;
            }
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
    // As an optimization, skip iterating over the lockfile is we're upgrading all packages anyway.
    if upgrade.is_all() {
        return Ok(LockedRequirements::default());
    }

    // Read the `pylock.toml` from disk, and deserialize it from TOML.
    let content = fs_err::tokio::read_to_string(&output_file).await?;
    let lock = toml::from_str::<PylockToml>(&content)?;

    let mut preferences = Vec::new();
    let mut git = Vec::new();

    for package in &lock.packages {
        // Skip the distribution if it's not included in the upgrade strategy.
        if upgrade.contains(&package.name) {
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
