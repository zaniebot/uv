use std::path::Path;

use anyhow::Result;
use rustc_hash::FxHashMap;

use uv_configuration::Upgrade;
use uv_distribution_types::Requirement;
use uv_fs::CWD;
use uv_git::ResolvedRepositoryReference;
use uv_normalize::PackageName;
use uv_requirements_txt::RequirementsTxt;
use uv_resolver::{Lock, LockError, Preference, PreferenceError, PylockToml, PylockTomlErrorKind};

/// The "lowered" upgrade specification after resolving groups to packages.
///
/// This is the result of "lowering" an [`Upgrade`] once we have access to the lock file.
/// Groups are resolved to their constituent packages, making it simpler for consumers
/// to determine which packages should be upgraded.
#[derive(Debug, Default, Clone)]
pub enum LoweredUpgrade {
    /// Prefer pinned versions from the existing lockfile, if possible.
    #[default]
    None,

    /// Allow package upgrades for all packages, ignoring the existing lockfile.
    All,

    /// Allow package upgrades, but only for the specified packages.
    /// The map contains optional version constraints for each package.
    Packages(FxHashMap<PackageName, Vec<Requirement>>),
}

impl LoweredUpgrade {
    /// Lower an [`Upgrade`] to a [`LoweredUpgrade`] by resolving groups to packages.
    ///
    /// This requires access to the lock file to determine which packages belong to which groups.
    pub fn from_upgrade(upgrade: &Upgrade, lock: &Lock) -> Self {
        match upgrade {
            Upgrade::None => Self::None,
            Upgrade::All => Self::All,
            Upgrade::Packages(packages) => Self::Packages(packages.clone()),
            Upgrade::Groups(groups) => {
                let packages = resolve_groups_to_packages(lock, groups);
                Self::Packages(packages.into_iter().map(|p| (p, vec![])).collect())
            }
            Upgrade::PackagesAndGroups { packages, groups } => {
                let group_packages = resolve_groups_to_packages(lock, groups);
                let mut all_packages = packages.clone();
                for pkg in group_packages {
                    all_packages.entry(pkg).or_default();
                }
                Self::Packages(all_packages)
            }
        }
    }

    /// Returns `true` if no packages should be upgraded.
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if all packages should be upgraded.
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    /// Returns `true` if the specified package should be upgraded.
    pub fn contains(&self, package_name: &PackageName) -> bool {
        match self {
            Self::None => false,
            Self::All => true,
            Self::Packages(packages) => packages.contains_key(package_name),
        }
    }
}

/// Resolve group names to package names using the lock file.
///
/// Looks at both manifest-level dependency groups (for projects without [project] table)
/// and package-level dependency groups (the standard case).
fn resolve_groups_to_packages(
    lock: &Lock,
    groups: &rustc_hash::FxHashSet<uv_normalize::GroupName>,
) -> rustc_hash::FxHashSet<PackageName> {
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
    let lowered = LoweredUpgrade::from_upgrade(upgrade, lock);

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
