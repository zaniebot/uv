use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Context;
use tracing::debug;

use uv_cache::Cache;
use uv_client::RegistryClient;
use uv_configuration::{BuildOptions, Concurrency, DryRun, Reinstall};
use uv_dispatch::BuildDispatch;
use uv_distribution::DistributionDatabase;
use uv_distribution_types::{
    CachedDist, Dist, InstalledDist, InstalledVersion, LocalDist, VersionOrUrlRef,
};
use uv_distribution_types::{DistributionMetadata, InstalledMetadata, Name, Resolution};
use uv_fs::Simplified;
use uv_install_wheel::LinkMode;
use uv_installer::{InstallationStrategy, Plan, Planner, Preparer, SitePackages};
use uv_normalize::PackageName;
use uv_pep440::Version;
use uv_pep508::VerbatimUrl;
use uv_platform_tags::Tags;
use uv_preview::Preview;
use uv_python::PythonEnvironment;
use uv_types::{BuildContext, HashStrategy, InFlight};
use uv_warnings::warn_user;

use crate::bytecode::compile_bytecode;
use crate::loggers::InstallLogger;
use uv_cli_output::printer::Printer;
use uv_cli_output::reporters::{InstallReporter, PrepareReporter};

use crate::Error;
#[derive(Debug, Clone, Copy)]
pub enum Modifications {
    /// Use `pip install` semantics, whereby existing installations are left as-is, unless they are
    /// marked for re-installation or upgrade.
    ///
    /// Ensures that the resulting environment is sufficient to meet the requirements, but without
    /// any unnecessary changes.
    Sufficient,
    /// Use `pip sync` semantics, whereby any existing, extraneous installations are removed.
    ///
    /// Ensures that the resulting environment is an exact match for the requirements, but may
    /// result in more changes than necessary.
    Exact,
}

/// A distribution which was or would be modified
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[expect(clippy::large_enum_variant)]
pub enum ChangedDist {
    Local(LocalDist),
    Remote(Arc<Dist>),
}

impl Name for ChangedDist {
    fn name(&self) -> &PackageName {
        match self {
            Self::Local(dist) => dist.name(),
            Self::Remote(dist) => dist.name(),
        }
    }
}

/// The [`Version`] or [`VerbatimUrl`] for a changed dist.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ShortSpecifier<'a> {
    Version(&'a Version),
    Url(&'a VerbatimUrl),
}

impl std::fmt::Display for ShortSpecifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Version(version) => version.fmt(f),
            Self::Url(url) => write!(f, " @ {url}"),
        }
    }
}

/// The [`InstalledVersion`] or [`VerbatimUrl`] for a changed dist.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum LongSpecifier<'a> {
    InstalledVersion(InstalledVersion<'a>),
    Url(&'a VerbatimUrl),
}

impl std::fmt::Display for LongSpecifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InstalledVersion(version) => version.fmt(f),
            Self::Url(url) => write!(f, " @ {url}"),
        }
    }
}

impl ChangedDist {
    pub fn short_specifier(&self) -> ShortSpecifier<'_> {
        match self {
            Self::Local(dist) => ShortSpecifier::Version(dist.installed_version().version()),
            Self::Remote(dist) => match dist.version_or_url() {
                VersionOrUrlRef::Version(version) => ShortSpecifier::Version(version),
                VersionOrUrlRef::Url(url) => ShortSpecifier::Url(url),
            },
        }
    }

    pub fn long_specifier(&self) -> LongSpecifier<'_> {
        match self {
            Self::Local(dist) => LongSpecifier::InstalledVersion(dist.installed_version()),
            Self::Remote(dist) => match dist.version_or_url() {
                VersionOrUrlRef::Version(version) => {
                    LongSpecifier::InstalledVersion(InstalledVersion::Version(version))
                }
                VersionOrUrlRef::Url(url) => LongSpecifier::Url(url),
            },
        }
    }

    pub fn version(&self) -> Option<&Version> {
        match self {
            Self::Local(dist) => Some(dist.installed_version().version()),
            Self::Remote(dist) => dist.version(),
        }
    }
}

/// A summary of the changes made to the environment during an installation.
#[derive(Debug, Clone, Default)]
pub struct Changelog {
    /// The distributions that were installed.
    pub installed: HashSet<ChangedDist>,
    /// The distributions that were uninstalled.
    pub uninstalled: HashSet<ChangedDist>,
    /// The distributions that were reinstalled.
    pub reinstalled: HashSet<ChangedDist>,
}

impl Changelog {
    /// Create a [`Changelog`] from two iterators of [`ChangedDist`]s.
    pub fn new<I, U>(installed: I, uninstalled: U) -> Self
    where
        I: IntoIterator<Item = ChangedDist>,
        U: IntoIterator<Item = ChangedDist>,
    {
        // SAFETY: This is allowed because `LocalDist` implements `Hash` and `Eq` based solely on
        // the inner `kind`, and omits the types that rely on internal mutability.
        #[expect(clippy::mutable_key_type)]
        let mut uninstalled: HashSet<_> = uninstalled.into_iter().collect();
        let (reinstalled, installed): (HashSet<_>, HashSet<_>) = installed
            .into_iter()
            .partition(|dist| uninstalled.contains(dist));
        uninstalled.retain(|dist| !reinstalled.contains(dist));

        Self {
            installed,
            uninstalled,
            reinstalled,
        }
    }

    /// Create a [`Changelog`] from a list of local distributions.
    pub fn from_local(installed: Vec<CachedDist>, uninstalled: Vec<InstalledDist>) -> Self {
        Self::new(
            installed
                .into_iter()
                .map(|dist| ChangedDist::Local(dist.into())),
            uninstalled
                .into_iter()
                .map(|dist| ChangedDist::Local(dist.into())),
        )
    }

    /// Create a [`Changelog`] from a list of installed distributions.
    pub fn from_installed(installed: Vec<CachedDist>) -> Self {
        Self::from_local(installed, Vec::new())
    }

    /// Returns `true` if the changelog includes a distribution with the given name, either via
    /// an installation or uninstallation.
    pub fn includes(&self, name: &PackageName) -> bool {
        self.installed.iter().any(|dist| dist.name() == name)
            || self.uninstalled.iter().any(|dist| dist.name() == name)
    }

    /// Returns `true` if the changelog is empty.
    pub fn is_empty(&self) -> bool {
        self.installed.is_empty() && self.uninstalled.is_empty()
    }
}

/// Install a set of requirements into the current environment.
///
/// Returns a [`Changelog`] summarizing the changes made to the environment.
pub async fn install(
    resolution: &Resolution,
    site_packages: SitePackages,
    installation: InstallationStrategy,
    modifications: Modifications,
    reinstall: &Reinstall,
    build_options: &BuildOptions,
    link_mode: LinkMode,
    compile: bool,
    hasher: &HashStrategy,
    tags: &Tags,
    client: &RegistryClient,
    in_flight: &InFlight,
    concurrency: &Concurrency,
    build_dispatch: &BuildDispatch<'_>,
    cache: &Cache,
    venv: &PythonEnvironment,
    logger: Box<dyn InstallLogger>,
    installer_metadata: bool,
    dry_run: DryRun,
    printer: Printer,
    preview: Preview,
) -> Result<Changelog, Error> {
    let start = std::time::Instant::now();

    // Partition into those that should be linked from the cache (`local`), those that need to be
    // downloaded (`remote`), and those that should be removed (`extraneous`).
    let plan = Planner::new(resolution)
        .build(
            site_packages,
            installation,
            reinstall,
            build_options,
            hasher,
            build_dispatch.locations(),
            build_dispatch.config_settings(),
            build_dispatch.config_settings_package(),
            build_dispatch.extra_build_requires(),
            build_dispatch.extra_build_variables(),
            cache,
            venv,
            tags,
        )
        .context("Failed to determine installation plan")?;

    if dry_run.enabled() {
        return report_dry_run(
            dry_run,
            resolution,
            plan,
            modifications,
            start,
            logger.as_ref(),
            printer,
        );
    }

    let Plan {
        cached,
        remote,
        reinstalls,
        extraneous,
    } = plan;

    // If we're in `install` mode, ignore any extraneous distributions.
    let extraneous = match modifications {
        Modifications::Sufficient => vec![],
        Modifications::Exact => extraneous,
    };

    // Nothing to do.
    if remote.is_empty()
        && cached.is_empty()
        && reinstalls.is_empty()
        && extraneous.is_empty()
        && !compile
    {
        logger.on_check(resolution.len(), start, printer, dry_run)?;
        return Ok(Changelog::default());
    }

    // Partition into two sets: those that require build isolation, and those that disable it. This
    // is effectively a heuristic to make `--no-build-isolation` work "more often" by way of giving
    // `--no-build-isolation` packages "access" to the rest of the environment.
    let (isolated_phase, shared_phase) = Plan {
        cached,
        remote,
        reinstalls,
        extraneous,
    }
    .partition(|name| build_dispatch.build_isolation().is_isolated(Some(name)));

    let has_isolated_phase = !isolated_phase.is_empty();
    let has_shared_phase = !shared_phase.is_empty();

    let mut installs = vec![];
    let mut uninstalls = vec![];

    // Execute the isolated-build phase.
    if has_isolated_phase {
        let (isolated_installs, isolated_uninstalls) = execute_plan(
            isolated_phase,
            None,
            resolution,
            build_options,
            link_mode,
            hasher,
            tags,
            client,
            in_flight,
            concurrency,
            build_dispatch,
            cache,
            venv,
            logger.as_ref(),
            installer_metadata,
            printer,
            preview,
        )
        .await?;
        installs.extend(isolated_installs);
        uninstalls.extend(isolated_uninstalls);
    }

    if has_shared_phase {
        let (shared_installs, shared_uninstalls) = execute_plan(
            shared_phase,
            if has_isolated_phase {
                Some(InstallPhase::Shared)
            } else {
                None
            },
            resolution,
            build_options,
            link_mode,
            hasher,
            tags,
            client,
            in_flight,
            concurrency,
            build_dispatch,
            cache,
            venv,
            logger.as_ref(),
            installer_metadata,
            printer,
            preview,
        )
        .await?;
        installs.extend(shared_installs);
        uninstalls.extend(shared_uninstalls);
    }

    if compile {
        compile_bytecode(venv, concurrency, cache, printer).await?;
    }

    // Construct a summary of the changes made to the environment.
    let changelog = Changelog::from_local(installs, uninstalls);

    // Notify the user of any environment modifications.
    logger.on_complete(&changelog, printer, dry_run)?;

    Ok(changelog)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstallPhase {
    /// A dedicated phase for building and installing packages with build-isolation disabled.
    Shared,
}

impl InstallPhase {
    fn label(self) -> &'static str {
        match self {
            Self::Shared => "without build isolation",
        }
    }
}

/// Execute a [`Plan`] to install distributions into a Python environment.
async fn execute_plan(
    plan: Plan,
    phase: Option<InstallPhase>,
    resolution: &Resolution,
    build_options: &BuildOptions,
    link_mode: LinkMode,
    hasher: &HashStrategy,
    tags: &Tags,
    client: &RegistryClient,
    in_flight: &InFlight,
    concurrency: &Concurrency,
    build_dispatch: &BuildDispatch<'_>,
    cache: &Cache,
    venv: &PythonEnvironment,
    logger: &dyn InstallLogger,
    installer_metadata: bool,
    printer: Printer,
    preview: Preview,
) -> Result<(Vec<CachedDist>, Vec<InstalledDist>), Error> {
    let Plan {
        cached,
        remote,
        reinstalls,
        extraneous,
    } = plan;

    // Download, build, and unzip any missing distributions.
    let wheels = if remote.is_empty() {
        vec![]
    } else {
        let start = std::time::Instant::now();

        let preparer = Preparer::new(
            cache,
            tags,
            hasher,
            build_options,
            DistributionDatabase::new(
                client,
                build_dispatch,
                concurrency.downloads_semaphore.clone(),
            ),
        )
        .with_reporter(Arc::new(
            PrepareReporter::from(printer).with_length(remote.len() as u64),
        ));

        let wheels = preparer
            .prepare(remote.clone(), in_flight, resolution)
            .await?;

        logger.on_prepare(
            wheels.len(),
            phase.map(InstallPhase::label),
            start,
            printer,
            DryRun::Disabled,
        )?;

        wheels
    };

    // Remove any upgraded or extraneous installations.
    let uninstalls = extraneous.into_iter().chain(reinstalls).collect::<Vec<_>>();
    if !uninstalls.is_empty() {
        let start = std::time::Instant::now();

        let layout = venv.interpreter().layout();
        for dist_info in &uninstalls {
            match uv_installer::uninstall(dist_info, &layout).await {
                Ok(summary) => {
                    debug!(
                        "Uninstalled {} ({} file{}, {} director{})",
                        dist_info.name(),
                        summary.file_count,
                        if summary.file_count == 1 { "" } else { "s" },
                        summary.dir_count,
                        if summary.dir_count == 1 { "y" } else { "ies" },
                    );
                }
                Err(uv_installer::UninstallError::Uninstall(
                    uv_install_wheel::Error::MissingRecord(_),
                )) => {
                    warn_user!(
                        "Failed to uninstall package at {} due to missing `RECORD` file. Installation may result in an incomplete environment.",
                        dist_info.install_path().user_display().cyan(),
                    );
                }
                Err(uv_installer::UninstallError::Uninstall(
                    uv_install_wheel::Error::MissingTopLevel(_),
                )) => {
                    warn_user!(
                        "Failed to uninstall package at {} due to missing `top_level.txt` file. Installation may result in an incomplete environment.",
                        dist_info.install_path().user_display().cyan(),
                    );
                }
                Err(err) => return Err(err.into()),
            }
        }

        logger.on_uninstall(uninstalls.len(), start, printer, DryRun::Disabled)?;
    }

    // Install the resolved distributions.
    let mut installs = wheels.into_iter().chain(cached).collect::<Vec<_>>();
    if !installs.is_empty() {
        let start = std::time::Instant::now();
        installs = uv_installer::Installer::new(venv, preview)
            .with_link_mode(link_mode)
            .with_cache(cache)
            .with_installer_metadata(installer_metadata)
            .with_reporter(Arc::new(
                InstallReporter::from(printer).with_length(installs.len() as u64),
            ))
            // This technically can block the runtime, but we are on the main thread and
            // have no other running tasks at this point, so this lets us avoid spawning a blocking
            // task.
            .install_blocking(installs)?;

        logger.on_install(installs.len(), start, printer, DryRun::Disabled)?;
    }

    Ok((installs, uninstalls))
}

/// Report on the results of a dry-run installation.
fn report_dry_run(
    dry_run: DryRun,
    resolution: &Resolution,
    plan: Plan,
    modifications: Modifications,
    start: std::time::Instant,
    logger: &dyn InstallLogger,
    printer: Printer,
) -> Result<Changelog, Error> {
    let Plan {
        cached,
        remote,
        reinstalls,
        extraneous,
    } = plan;

    // If we're in `install` mode, ignore any extraneous distributions.
    let extraneous = match modifications {
        Modifications::Sufficient => vec![],
        Modifications::Exact => extraneous,
    };

    // Nothing to do.
    if remote.is_empty() && cached.is_empty() && reinstalls.is_empty() && extraneous.is_empty() {
        logger.on_check(resolution.len(), start, printer, dry_run)?;
        return Ok(Changelog::default());
    }

    // Download, build, and unzip any missing distributions.
    let wheels = if remote.is_empty() {
        vec![]
    } else {
        logger.on_prepare(remote.len(), None, start, printer, dry_run)?;
        remote.clone()
    };

    // Remove any upgraded or extraneous installations.
    let uninstalls = extraneous.len() + reinstalls.len();

    if uninstalls > 0 {
        logger.on_uninstall(uninstalls, start, printer, dry_run)?;
    }

    // Install the resolved distributions.
    let installs = wheels.len() + cached.len();

    if installs > 0 {
        logger.on_install(installs, start, printer, dry_run)?;
    }

    let uninstalled = reinstalls
        .into_iter()
        .chain(extraneous)
        .map(|dist| ChangedDist::Local(dist.into()));
    let installed = wheels.into_iter().map(ChangedDist::Remote).chain(
        cached
            .into_iter()
            .map(|dist| ChangedDist::Local(dist.into())),
    );

    let changelog = Changelog::new(installed, uninstalled);

    logger.on_complete(&changelog, printer, dry_run)?;

    if matches!(dry_run, DryRun::Check) {
        return Err(Error::OutdatedEnvironment(Box::new(changelog)));
    }

    Ok(changelog)
}
