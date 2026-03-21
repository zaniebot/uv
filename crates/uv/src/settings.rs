use std::env::VarError;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use std::time::Duration;

use rustc_hash::FxHashSet;

use crate::commands::{PythonUpgrade, PythonUpgradeSource};
use uv_auth::Service;
use uv_cache::{CacheArgs, Refresh};
use uv_cli::comma::CommaSeparatedRequirements;
use uv_cli::{
    AddArgs, AuditArgs, AuthLoginArgs, AuthLogoutArgs, AuthTokenArgs, ColorChoice, ExternalCommand,
    GlobalArgs, InitArgs, ListFormat, LockArgs, Maybe, PipCheckArgs, PipCompileArgs, PipFreezeArgs,
    PipInstallArgs, PipListArgs, PipShowArgs, PipSyncArgs, PipTreeArgs, PipUninstallArgs,
    PythonFindArgs, PythonInstallArgs, PythonListArgs, PythonListFormat, PythonPinArgs,
    PythonUninstallArgs, PythonUpgradeArgs, RemoveArgs, RunArgs, SyncArgs, SyncFormat, ToolDirArgs,
    ToolInstallArgs, ToolListArgs, ToolRunArgs, ToolUninstallArgs, TreeArgs, VenvArgs, VersionArgs,
    VersionBumpSpec, VersionFormat,
};
use uv_cli::{
    AuthorFrom, BuildArgs, ExportArgs, FormatArgs, PublishArgs, PythonDirArgs,
    ResolverInstallerArgs, ToolUpgradeArgs,
    options::{
        Flag, FlagSource, check_conflicts, flag, resolve_flag, resolver_installer_options,
        resolver_options,
    },
};
use uv_client::Connectivity;
use uv_configuration::{
    BuildIsolation, BuildOptions, Concurrency, DependencyGroups, DryRun, EditableMode, EnvFile,
    ExportFormat, ExtrasSpecification, GitLfsSetting, HashCheckingMode, IndexStrategy,
    InstallOptions, KeyringProviderType, NoBinary, NoBuild, NoSources, PipCompileFormat,
    ProjectBuildBackend, ProxyUrl, Reinstall, RequiredVersion, TargetTriple, TrustedHost,
    TrustedPublishing, Upgrade, VersionControlSystem,
};
use uv_distribution_types::{
    ConfigSettings, DependencyMetadata, ExtraBuildVariables, Index, IndexLocations, IndexUrl,
    PackageConfigSettings, Requirement,
};
use uv_install_wheel::LinkMode;
use uv_normalize::{ExtraName, PackageName, PipGroupName};
use uv_pep508::{MarkerTree, RequirementOrigin};
use uv_preview::Preview;
use uv_pypi_types::SupportedEnvironments;
use uv_python::{Prefix, PythonDownloads, PythonPreference, PythonVersion, Target};
use uv_redacted::DisplaySafeUrl;
use uv_resolver::{
    AnnotationStyle, DependencyMode, ExcludeNewer, ExcludeNewerPackage, ForkStrategy,
    PrereleaseMode, ResolutionMode,
};
use uv_settings::{
    Combine, EnvironmentOptions, FilesystemOptions, Options, PipOptions, PublishOptions,
    PythonInstallMirrors, ResolverInstallerOptions, ResolverInstallerSchema, ResolverOptions,
};
use uv_static::EnvVars;
use uv_torch::TorchMode;
use uv_warnings::warn_user_once;
use uv_workspace::pyproject::{DependencyType, ExtraBuildDependencies};
use uv_workspace::pyproject_mut::AddBoundsKind;

use crate::commands::ToolRunCommand;
use crate::commands::{InitKind, InitProjectKind, pip::operations::Modifications};

mod commands;
mod shared;
pub(crate) use commands::*;

/// The default publish URL.
const PYPI_PUBLISH_URL: &str = "https://upload.pypi.org/legacy/";

/// The resolved global settings to use for any invocation of the CLI.
#[derive(Debug, Clone)]
pub(crate) struct GlobalSettings {
    pub(crate) required_version: Option<RequiredVersion>,
    pub(crate) quiet: u8,
    pub(crate) verbose: u8,
    pub(crate) color: ColorChoice,
    pub(crate) network_settings: NetworkSettings,
    pub(crate) concurrency: Concurrency,
    pub(crate) show_settings: bool,
    pub(crate) preview: Preview,
    pub(crate) python_preference: PythonPreference,
    pub(crate) python_downloads: PythonDownloads,
    pub(crate) no_progress: bool,
    pub(crate) installer_metadata: bool,
}

impl GlobalSettings {
    /// Resolve the [`GlobalSettings`] from the CLI and filesystem configuration.
    pub(crate) fn resolve(
        args: &GlobalArgs,
        workspace: Option<&FilesystemOptions>,
        environment: &EnvironmentOptions,
    ) -> Self {
        let network_settings = NetworkSettings::resolve(args, workspace, environment);
        let python_preference = resolve_python_preference(args, workspace, environment);
        Self {
            required_version: workspace
                .and_then(|workspace| workspace.globals.required_version.clone()),
            quiet: args.quiet,
            verbose: args.verbose,
            color: if let Some(color_choice) = args.color {
                // If `--color` is passed explicitly, use its value.
                color_choice
            } else if args.no_color {
                // If `--no-color` is passed explicitly, disable color output.
                ColorChoice::Never
            } else if std::env::var_os(EnvVars::NO_COLOR)
                .filter(|v| !v.is_empty())
                .is_some()
            {
                // If the `NO_COLOR` is set, disable color output.
                ColorChoice::Never
            } else if std::env::var_os(EnvVars::FORCE_COLOR)
                .filter(|v| !v.is_empty())
                .is_some()
                || std::env::var_os(EnvVars::CLICOLOR_FORCE)
                    .filter(|v| !v.is_empty())
                    .is_some()
            {
                // If `FORCE_COLOR` or `CLICOLOR_FORCE` is set, always enable color output.
                ColorChoice::Always
            } else {
                ColorChoice::Auto
            },
            network_settings,
            concurrency: Concurrency::new(
                environment
                    .concurrency
                    .downloads
                    .combine(workspace.and_then(|workspace| workspace.globals.concurrent_downloads))
                    .map(NonZeroUsize::get)
                    .unwrap_or(Concurrency::DEFAULT_DOWNLOADS),
                environment
                    .concurrency
                    .builds
                    .combine(workspace.and_then(|workspace| workspace.globals.concurrent_builds))
                    .map(NonZeroUsize::get)
                    .unwrap_or_else(Concurrency::threads),
                environment
                    .concurrency
                    .installs
                    .combine(workspace.and_then(|workspace| workspace.globals.concurrent_installs))
                    .map(NonZeroUsize::get)
                    .unwrap_or_else(Concurrency::threads),
            ),
            show_settings: args.show_settings,
            preview: Preview::from_args(
                resolve_preview(args, workspace, environment),
                args.no_preview,
                &args.preview_features,
            ),
            python_preference,
            python_downloads: flag(
                args.allow_python_downloads,
                args.no_python_downloads,
                "python-downloads",
            )
            .map(PythonDownloads::from)
            .combine(env(env::UV_PYTHON_DOWNLOADS))
            .combine(workspace.and_then(|workspace| workspace.globals.python_downloads))
            .unwrap_or_default(),
            // Disable the progress bar with `RUST_LOG` to avoid progress fragments interleaving
            // with log messages.
            no_progress: resolve_flag(args.no_progress, "no-progress", environment.no_progress)
                .is_enabled()
                || std::env::var_os(EnvVars::RUST_LOG).is_some(),
            installer_metadata: !resolve_flag(
                args.no_installer_metadata,
                "no-installer-metadata",
                environment.no_installer_metadata,
            )
            .is_enabled(),
        }
    }
}

fn resolve_python_preference(
    args: &GlobalArgs,
    workspace: Option<&FilesystemOptions>,
    environment: &EnvironmentOptions,
) -> PythonPreference {
    // Resolve flags from CLI and environment variables.
    let managed_python = resolve_flag(
        args.managed_python,
        "managed-python",
        environment.managed_python,
    );
    let no_managed_python = resolve_flag(
        args.no_managed_python,
        "no-managed-python",
        environment.no_managed_python,
    );

    // Check for conflicts between managed_python and python_preference.
    if managed_python.is_enabled() && args.python_preference.is_some() {
        check_conflicts(managed_python, Flag::from_cli("python-preference"));
    }

    // Check for conflicts between no_managed_python and python_preference.
    if no_managed_python.is_enabled() && args.python_preference.is_some() {
        check_conflicts(no_managed_python, Flag::from_cli("python-preference"));
    }

    if managed_python.is_enabled() {
        PythonPreference::OnlyManaged
    } else if no_managed_python.is_enabled() {
        PythonPreference::OnlySystem
    } else {
        args.python_preference
            .combine(workspace.and_then(|workspace| workspace.globals.python_preference))
            .unwrap_or_default()
    }
}

/// Resolve the preview setting from CLI, environment, and workspace config.
pub(crate) fn resolve_preview(
    args: &GlobalArgs,
    workspace: Option<&FilesystemOptions>,
    environment: &EnvironmentOptions,
) -> bool {
    // CLI takes precedence
    match flag(args.preview, args.no_preview, "preview") {
        Some(value) => value,
        None => {
            // Check environment variable
            if environment.preview.value == Some(true) {
                true
            } else {
                // Fall back to workspace config
                workspace
                    .and_then(|workspace| workspace.globals.preview)
                    .unwrap_or(false)
            }
        }
    }
}

/// The resolved network settings to use for any invocation of the CLI.
#[derive(Debug, Clone)]
pub(crate) struct NetworkSettings {
    pub(crate) connectivity: Connectivity,
    pub(crate) offline: Flag,
    pub(crate) native_tls: bool,
    pub(crate) http_proxy: Option<ProxyUrl>,
    pub(crate) https_proxy: Option<ProxyUrl>,
    pub(crate) no_proxy: Option<Vec<String>>,
    pub(crate) allow_insecure_host: Vec<TrustedHost>,
    pub(crate) read_timeout: Duration,
    pub(crate) connect_timeout: Duration,
    pub(crate) retries: u32,
}

impl NetworkSettings {
    pub(crate) fn resolve(
        args: &GlobalArgs,
        workspace: Option<&FilesystemOptions>,
        environment: &EnvironmentOptions,
    ) -> Self {
        // Resolve offline flag from CLI, environment variable, and workspace config.
        // Precedence: CLI > Env var > Workspace config > default (false).
        let offline = match flag(args.offline, args.no_offline, "offline") {
            Some(true) => Flag::from_cli("offline"),
            Some(false) => Flag::disabled(),
            None => {
                // CLI didn't provide a value, check environment variable.
                let env_flag = resolve_flag(false, "offline", environment.offline);
                if env_flag.is_enabled() {
                    env_flag
                } else if workspace
                    .and_then(|workspace| workspace.globals.offline)
                    .unwrap_or(false)
                {
                    // Workspace config enabled offline mode.
                    Flag::from_config("offline")
                } else {
                    Flag::disabled()
                }
            }
        };

        let connectivity = if offline.is_enabled() {
            Connectivity::Offline
        } else {
            Connectivity::Online
        };
        let native_tls = match flag(args.native_tls, args.no_native_tls, "native-tls") {
            Some(value) => value,
            None => {
                if environment.native_tls.value == Some(true) {
                    true
                } else {
                    workspace
                        .and_then(|workspace| workspace.globals.native_tls)
                        .unwrap_or(false)
                }
            }
        };
        let allow_insecure_host = args
            .allow_insecure_host
            .as_ref()
            .map(|allow_insecure_host| {
                allow_insecure_host
                    .iter()
                    .filter_map(|value| value.clone().into_option())
            })
            .into_iter()
            .flatten()
            .chain(
                workspace
                    .and_then(|workspace| workspace.globals.allow_insecure_host.clone())
                    .into_iter()
                    .flatten(),
            )
            .collect();
        let http_proxy = workspace.and_then(|workspace| workspace.globals.http_proxy.clone());
        let https_proxy = workspace.and_then(|workspace| workspace.globals.https_proxy.clone());
        let no_proxy = workspace.and_then(|workspace| workspace.globals.no_proxy.clone());

        Self {
            connectivity,
            offline,
            native_tls,
            http_proxy,
            https_proxy,
            no_proxy,
            allow_insecure_host,
            read_timeout: environment.http_read_timeout,
            connect_timeout: environment.http_connect_timeout,
            retries: environment.http_retries,
        }
    }

    /// Check if offline mode conflicts with a refresh request.
    ///
    /// This should be called when a command uses refresh functionality to ensure
    /// offline mode and refresh are not both enabled.
    pub(crate) fn check_refresh_conflict(&self, refresh: &Refresh) {
        if !matches!(refresh, Refresh::None(_)) {
            // TODO(charlie): `Refresh` isn't a `Flag`, so we create a synthetic one here
            // (which matches Clap's representation). Consider a dedicated helper for
            // conflicts with CLI-only arguments.
            check_conflicts(self.offline, Flag::from_cli("refresh"));
        }
    }
}

/// The resolved cache settings to use for any invocation of the CLI.
#[derive(Debug, Clone)]
pub(crate) struct CacheSettings {
    pub(crate) no_cache: bool,
    pub(crate) cache_dir: Option<PathBuf>,
}

impl CacheSettings {
    /// Resolve the [`CacheSettings`] from the CLI and filesystem configuration.
    pub(crate) fn resolve(args: CacheArgs, workspace: Option<&FilesystemOptions>) -> Self {
        Self {
            no_cache: args.no_cache
                || workspace
                    .and_then(|workspace| workspace.globals.no_cache)
                    .unwrap_or(false),
            cache_dir: args
                .cache_dir
                .or_else(|| workspace.and_then(|workspace| workspace.globals.cache_dir.clone())),
        }
    }
}

mod env {
    use uv_static::EnvVars;
    pub(super) const UV_PYTHON_DOWNLOADS: (&str, &str) = (
        EnvVars::UV_PYTHON_DOWNLOADS,
        "one of 'auto', 'true', 'manual', 'never', or 'false'",
    );
}

/// Attempt to load and parse an environment variable with the given name.
///
/// Exits the program and prints an error message containing the expected type if
/// parsing values.
fn env<T>((name, expected): (&str, &str)) -> Option<T>
where
    T: FromStr,
{
    let val = match std::env::var(name) {
        Ok(val) => val,
        Err(VarError::NotPresent) => return None,
        Err(VarError::NotUnicode(_)) => parse_failure(name, expected),
    };
    Some(
        val.parse()
            .unwrap_or_else(|_| parse_failure(name, expected)),
    )
}

/// Prints a parse error and exits the process.
#[expect(clippy::exit, clippy::print_stderr)]
fn parse_failure(name: &str, expected: &str) -> ! {
    eprintln!("error: invalid value for {name}, expected {expected}");
    process::exit(1)
}
