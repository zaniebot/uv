use std::borrow::Cow;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};
use itertools::Itertools;
use tracing::{debug, trace, warn};

use uv_cache::Cache;
use uv_client::BaseClientBuilder;
use uv_configuration::{
    Concurrency, DependencyGroups, DryRun, EditableMode, EnvFile, ExtrasSpecification,
};
use uv_fs::{PythonExt, Simplified, create_symlink};
use uv_installer::SitePackages;
use uv_normalize::{DefaultExtras, PackageName};
use uv_preview::Preview;
use uv_python::{
    Interpreter, PyVenvConfiguration, PythonDownloads, PythonEnvironment, PythonPreference,
    PythonRequest,
};
use uv_requirements::{RequirementsSource, RequirementsSpecification};
use uv_resolver::Preference;
use uv_settings::PythonInstallMirrors;
use uv_shell::Shell;
use uv_static::EnvVars;
use uv_warnings::warn_user;
use uv_workspace::{DiscoveryOptions, VirtualProject, WorkspaceCache};

use crate::commands::pip::loggers::{
    DefaultInstallLogger, DefaultResolveLogger, SummaryInstallLogger, SummaryResolveLogger,
};
use crate::commands::pip::operations::Modifications;
use crate::commands::project::environment::{CachedEnvironment, EphemeralEnvironment};
use crate::commands::project::install_target::InstallTarget;
use crate::commands::project::lock::LockMode;
use crate::commands::project::run::{CopyEntrypointError, can_skip_ephemeral, copy_entrypoint};
use crate::commands::project::{
    EnvironmentSpecification, PreferenceLocation, ProjectEnvironment, ProjectError, UniversalState,
    default_dependency_groups,
};
use crate::commands::{ExitStatus, diagnostics, project};
use crate::printer::Printer;
use crate::settings::{FrozenSource, LockCheck, ResolverInstallerSettings};

/// Launch a shell in the project environment.
#[allow(clippy::fn_params_excessive_bools)]
pub(crate) async fn shell(
    project_dir: &Path,
    lock_check: LockCheck,
    frozen: Option<FrozenSource>,
    no_project: bool,
    no_sync: bool,
    show_resolution: bool,
    active: Option<bool>,
    isolated: bool,
    requirements: Vec<RequirementsSource>,
    all_packages: bool,
    package: Option<PackageName>,
    extras: ExtrasSpecification,
    groups: DependencyGroups,
    editable: Option<EditableMode>,
    modifications: Modifications,
    python: Option<String>,
    install_mirrors: PythonInstallMirrors,
    settings: ResolverInstallerSettings,
    client_builder: BaseClientBuilder<'_>,
    python_preference: PythonPreference,
    python_downloads: PythonDownloads,
    installer_metadata: bool,
    concurrency: Concurrency,
    cache: Cache,
    printer: Printer,
    env_file: EnvFile,
    preview: Preview,
) -> anyhow::Result<ExitStatus> {
    // Check for nested shell sessions.
    if std::env::var_os(EnvVars::UV_SHELL_ACTIVE).is_some() {
        warn_user!(
            "A `uv shell` session is already active; nested shells may cause unexpected behavior"
        );
    }

    // Read from the `.env` file, if necessary.
    for env_file_path in env_file.iter().rev().map(PathBuf::as_path) {
        match dotenvy::from_path(env_file_path) {
            Err(dotenvy::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
                bail!(
                    "No environment file found at: `{}`",
                    env_file_path.simplified_display()
                );
            }
            Err(dotenvy::Error::Io(err)) => {
                bail!(
                    "Failed to read environment file `{}`: {err}",
                    env_file_path.simplified_display()
                );
            }
            Err(dotenvy::Error::LineParse(content, position)) => {
                warn_user!(
                    "Failed to parse environment file `{}` at position {position}: {content}",
                    env_file_path.simplified_display(),
                );
            }
            Err(err) => {
                warn_user!(
                    "Failed to parse environment file `{}`: {err}",
                    env_file_path.simplified_display(),
                );
            }
            Ok(()) => {
                debug!(
                    "Loaded environment file at: `{}`",
                    env_file_path.simplified_display()
                );
            }
        }
    }

    // If `--no-project` is set, look for an active or local venv without project discovery.
    if no_project {
        debug!("Skipping project discovery due to `--no-project`");
        return shell_without_project(project_dir).await;
    }

    // Initialize shared state.
    let lock_state = UniversalState::default();
    let sync_state = lock_state.fork();
    let workspace_cache = WorkspaceCache::default();

    // Discover the project.
    let project =
        match VirtualProject::discover(project_dir, &DiscoveryOptions::default(), &workspace_cache)
            .await
        {
            Ok(project) => project,
            Err(err) => {
                return Err(err.into());
            }
        };

    if let Some(project_name) = project.project_name() {
        debug!(
            "Discovered project `{project_name}` at: {}",
            project.workspace().install_path().display()
        );
    } else {
        debug!(
            "Discovered virtual workspace at: {}",
            project.workspace().install_path().display()
        );
    }

    // Determine the groups and extras to include.
    let default_groups = default_dependency_groups(project.pyproject_toml())?;
    let default_extras = DefaultExtras::default();
    let groups = groups.with_defaults(default_groups);
    let extras = extras.with_defaults(default_extras);

    // Keep track of the temp dir for isolated mode so it lives long enough.
    let _temp_dir;

    // Resolve Python and create/find the environment.
    let venv = if isolated {
        debug!("Creating isolated virtual environment");

        // For isolated mode, create a temporary venv.
        _temp_dir = cache.venv_dir()?;
        let environment = ProjectEnvironment::get_or_init(
            project.workspace(),
            &groups,
            python.as_deref().map(PythonRequest::parse),
            &install_mirrors,
            &client_builder,
            python_preference,
            python_downloads,
            no_sync,
            false, // no_config
            active,
            &cache,
            DryRun::Disabled,
            printer,
            preview,
        )
        .await?
        .into_environment()?;

        // Create a fresh temporary venv with the same interpreter.
        uv_virtualenv::create_venv(
            _temp_dir.path(),
            environment.into_interpreter(),
            uv_virtualenv::Prompt::None,
            false,
            uv_virtualenv::OnExisting::Remove(uv_virtualenv::RemovalReason::TemporaryEnvironment),
            false,
            false,
            false,
            preview,
        )?
    } else {
        _temp_dir = cache.venv_dir()?; // unused but needs to exist for type consistency
        ProjectEnvironment::get_or_init(
            project.workspace(),
            &groups,
            python.as_deref().map(PythonRequest::parse),
            &install_mirrors,
            &client_builder,
            python_preference,
            python_downloads,
            no_sync,
            false, // no_config
            active,
            &cache,
            DryRun::Disabled,
            printer,
            preview,
        )
        .await?
        .into_environment()?
    };

    // Track the lock result for `--with` preferences.
    let mut base_lock = None;

    // Lock and sync, unless told not to.
    if no_sync {
        debug!("Skipping environment synchronization due to `--no-sync`");
    } else {
        let _lock = venv
            .lock()
            .await
            .inspect_err(|err| {
                warn!("Failed to acquire environment lock: {err}");
            })
            .ok();

        // Determine the lock mode.
        let mode = if let Some(frozen_source) = frozen {
            LockMode::Frozen(frozen_source.into())
        } else if let LockCheck::Enabled(lock_check) = lock_check {
            LockMode::Locked(venv.interpreter(), lock_check)
        } else if isolated {
            LockMode::DryRun(venv.interpreter())
        } else {
            LockMode::Write(venv.interpreter())
        };

        let result = match Box::pin(
            project::lock::LockOperation::new(
                mode,
                &settings.resolver,
                &client_builder,
                &lock_state,
                if show_resolution {
                    Box::new(DefaultResolveLogger)
                } else {
                    Box::new(SummaryResolveLogger)
                },
                concurrency,
                &cache,
                &workspace_cache,
                printer,
                preview,
            )
            .execute(project.workspace().into()),
        )
        .await
        {
            Ok(result) => result,
            Err(ProjectError::Operation(err)) => {
                return diagnostics::OperationDiagnostic::native_tls(
                    client_builder.is_native_tls(),
                )
                .report(err)
                .map_or(Ok(ExitStatus::Failure), |err| Err(err.into()));
            }
            Err(err) => return Err(err.into()),
        };

        // Identify the installation target.
        let target = match &project {
            VirtualProject::Project(project) => {
                if all_packages {
                    InstallTarget::Workspace {
                        workspace: project.workspace(),
                        lock: result.lock(),
                    }
                } else if let Some(package) = package.as_ref() {
                    InstallTarget::Project {
                        workspace: project.workspace(),
                        name: package,
                        lock: result.lock(),
                    }
                } else {
                    InstallTarget::Project {
                        workspace: project.workspace(),
                        name: project.project_name(),
                        lock: result.lock(),
                    }
                }
            }
            VirtualProject::NonProject(workspace) => {
                if all_packages {
                    InstallTarget::NonProjectWorkspace {
                        workspace,
                        lock: result.lock(),
                    }
                } else if let Some(package) = package.as_ref() {
                    InstallTarget::Project {
                        workspace,
                        name: package,
                        lock: result.lock(),
                    }
                } else {
                    InstallTarget::NonProjectWorkspace {
                        workspace,
                        lock: result.lock(),
                    }
                }
            }
        };

        let install_options = uv_configuration::InstallOptions::default();

        match project::sync::do_sync(
            target,
            &venv,
            &extras,
            &groups,
            editable,
            install_options,
            modifications,
            None, // python_platform
            (&settings).into(),
            &client_builder,
            &sync_state,
            if show_resolution {
                Box::new(DefaultInstallLogger)
            } else {
                Box::new(SummaryInstallLogger)
            },
            installer_metadata,
            concurrency,
            &cache,
            workspace_cache,
            DryRun::Disabled,
            printer,
            preview,
        )
        .await
        {
            Ok(_) => {}
            Err(ProjectError::Operation(err)) => {
                return diagnostics::OperationDiagnostic::native_tls(
                    client_builder.is_native_tls(),
                )
                .report(err)
                .map_or(Ok(ExitStatus::Failure), |err| Err(err.into()));
            }
            Err(err) => return Err(err.into()),
        }

        base_lock = Some((
            result.into_lock(),
            project.workspace().install_path().to_owned(),
        ));
    }

    // Handle `--with` requirements by creating an ephemeral environment layered on top.
    let base_interpreter = venv.interpreter().clone();
    let extra_path_dirs = if requirements.is_empty() {
        vec![]
    } else {
        resolve_with_requirements(
            &requirements,
            &base_interpreter,
            base_lock.as_ref(),
            show_resolution,
            &settings,
            &client_builder,
            &sync_state,
            installer_metadata,
            concurrency,
            &cache,
            printer,
            preview,
        )
        .await?
    };

    // Detect the user's shell.
    let shell = Shell::from_env()
        .context("Could not detect the current shell. Ensure a supported shell is in use.")?;

    debug!("Detected shell: {shell}");

    // Determine paths for activation.
    let venv_bin = venv.scripts();
    let sys_prefix = venv.root();

    // Determine the prompt name.
    let prompt_name = project
        .project_name()
        .map(ToString::to_string)
        .unwrap_or_else(|| {
            project
                .workspace()
                .install_path()
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| "venv".to_string())
        });

    // Spawn the subshell.
    #[cfg(unix)]
    {
        spawn_unix_shell(shell, venv_bin, sys_prefix, &prompt_name, &extra_path_dirs)
    }
    #[cfg(windows)]
    {
        spawn_windows_shell(shell, venv_bin, sys_prefix, &prompt_name, &extra_path_dirs).await
    }
}

/// Resolve `--with` requirements into an ephemeral environment, returning extra PATH directories
/// to prepend.
async fn resolve_with_requirements(
    requirements: &[RequirementsSource],
    base_interpreter: &Interpreter,
    base_lock: Option<&(uv_resolver::Lock, PathBuf)>,
    show_resolution: bool,
    settings: &ResolverInstallerSettings,
    client_builder: &BaseClientBuilder<'_>,
    sync_state: &project::PlatformState,
    installer_metadata: bool,
    concurrency: Concurrency,
    cache: &Cache,
    printer: Printer,
    preview: Preview,
) -> anyhow::Result<Vec<PathBuf>> {
    // Read the requirements.
    let spec = RequirementsSpecification::from_simple_sources(requirements, client_builder).await?;

    // Check if the requirements are already satisfied in the base environment.
    let base_site_packages = SitePackages::from_interpreter(base_interpreter)?;
    if can_skip_ephemeral(&spec, base_interpreter, &base_site_packages, settings) {
        return Ok(vec![]);
    }

    debug!("Syncing `--with` requirements to cached environment");

    // Read the build constraints from the lock file.
    let build_constraints = base_lock
        .as_ref()
        .map(|(lock, path)| lock.build_constraints(path));

    // Read the preferences.
    let spec = EnvironmentSpecification::from(spec).with_preferences(
        if let Some((lock, install_path)) = base_lock.as_ref() {
            PreferenceLocation::Lock { lock, install_path }
        } else {
            PreferenceLocation::Entries(
                base_site_packages
                    .iter()
                    .filter_map(Preference::from_installed)
                    .collect::<Vec<_>>(),
            )
        },
    );

    let result = CachedEnvironment::from_spec(
        spec,
        build_constraints.unwrap_or_default(),
        base_interpreter,
        None, // python_platform
        settings,
        client_builder,
        sync_state,
        if show_resolution {
            Box::new(DefaultResolveLogger)
        } else {
            Box::new(SummaryResolveLogger)
        },
        if show_resolution {
            Box::new(DefaultInstallLogger)
        } else {
            Box::new(SummaryInstallLogger)
        },
        installer_metadata,
        concurrency,
        cache,
        printer,
        preview,
    )
    .await;

    let requirements_env = match result {
        Ok(resolution) => PythonEnvironment::from(resolution),
        Err(ProjectError::Operation(err)) => {
            return diagnostics::OperationDiagnostic::native_tls(client_builder.is_native_tls())
                .with_context("`--with`")
                .report(err)
                .map_or(Ok(vec![]), |err| Err(err.into()));
        }
        Err(err) => return Err(err.into()),
    };

    // Create an ephemeral environment layered on top.
    let ephemeral_dir = cache.venv_dir()?;
    debug!(
        "Creating ephemeral environment at: `{}`",
        ephemeral_dir.path().simplified_display()
    );

    let ephemeral_env: EphemeralEnvironment = uv_virtualenv::create_venv(
        ephemeral_dir.path(),
        base_interpreter.clone(),
        uv_virtualenv::Prompt::None,
        false,
        uv_virtualenv::OnExisting::Remove(uv_virtualenv::RemovalReason::TemporaryEnvironment),
        false,
        false,
        false,
        preview,
    )?
    .into();

    // Set up the overlay: add site directories from the requirements env and base env.
    let requirements_site_packages = requirements_env
        .site_packages()
        .next()
        .ok_or_else(|| anyhow!("Requirements environment has no site packages directory"))?;
    let mut base_site_packages_dirs = base_interpreter
        .runtime_site_packages()
        .iter()
        .map(|path| Cow::Borrowed(path.as_path()))
        .chain(base_interpreter.site_packages())
        .peekable();
    if base_site_packages_dirs.peek().is_none() {
        return Err(anyhow!("Base environment has no site packages directory"));
    }

    let overlay_content = format!(
        "import site; {}",
        std::iter::once(requirements_site_packages)
            .chain(base_site_packages_dirs)
            .dedup()
            .inspect(|path| debug!("Adding `{}` to site packages", path.display()))
            .map(|path| format!("site.addsitedir(\"{}\")", path.escape_for_python()))
            .collect::<Vec<_>>()
            .join("; ")
    );

    ephemeral_env.set_overlay(overlay_content)?;

    // Copy entrypoints from the requirements env and base env into the ephemeral env.
    for interpreter in [requirements_env.interpreter(), base_interpreter] {
        let scripts = match fs_err::read_dir(interpreter.scripts()) {
            Ok(scripts) => scripts,
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => return Err(err.into()),
        };
        for entry in scripts {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            match copy_entrypoint(
                &entry.path(),
                &ephemeral_env.scripts().join(entry.file_name()),
                interpreter.sys_executable(),
                ephemeral_env.sys_executable(),
            ) {
                Ok(()) => {}
                Err(CopyEntrypointError::Io(err))
                    if err.kind() == std::io::ErrorKind::AlreadyExists =>
                {
                    trace!(
                        "Skipping copy of entrypoint `{}`: already exists",
                        &entry.path().display()
                    );
                }
                Err(CopyEntrypointError::Io(err))
                    if err.kind() == std::io::ErrorKind::PermissionDenied =>
                {
                    trace!(
                        "Skipping copy of entrypoint `{}`: permission denied",
                        &entry.path().display()
                    );
                }
                Err(err) => return Err(err.into()),
            }
        }

        // Link data directories from the base environment to the ephemeral environment.
        for dir in &["etc/jupyter", "share/jupyter"] {
            let source = interpreter.sys_prefix().join(dir);
            if !matches!(source.try_exists(), Ok(true)) {
                continue;
            }
            if !source.is_dir() {
                continue;
            }
            let target = ephemeral_env.sys_prefix().join(dir);
            if let Some(parent) = target.parent() {
                fs_err::create_dir_all(parent)?;
            }
            match create_symlink(&source, &target) {
                Ok(()) => trace!(
                    "Created link for {} -> {}",
                    target.simplified_display(),
                    source.simplified_display()
                ),
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(err) => return Err(err.into()),
            }
        }
    }

    // Write the parent environment marker.
    ephemeral_env.set_parent_environment(base_interpreter.sys_prefix())?;

    // If `--system-site-packages` is enabled, propagate it.
    if base_interpreter.is_virtualenv()
        && PyVenvConfiguration::parse(base_interpreter.sys_prefix().join("pyvenv.cfg"))
            .is_ok_and(|cfg| cfg.include_system_site_packages())
    {
        ephemeral_env.set_system_site_packages()?;
    }

    // The ephemeral env scripts dir should come first in PATH, then the requirements env scripts.
    let ephemeral_env = PythonEnvironment::from(ephemeral_env);
    let mut extra_dirs = vec![ephemeral_env.scripts().to_path_buf()];
    extra_dirs.push(requirements_env.scripts().to_path_buf());

    // Keep the temp dir alive by leaking it (it will be cleaned up on process exit).
    std::mem::forget(ephemeral_dir);

    Ok(extra_dirs)
}

/// Launch a shell without project discovery.
///
/// Looks for an active virtual environment (`VIRTUAL_ENV`) or a `.venv` directory in the
/// current directory. If found, activates it; otherwise, spawns a plain shell.
#[allow(clippy::unused_async)]
async fn shell_without_project(project_dir: &Path) -> anyhow::Result<ExitStatus> {
    let shell = Shell::from_env()
        .context("Could not detect the current shell. Ensure a supported shell is in use.")?;

    debug!("Detected shell: {shell}");

    // Check for an active virtual environment or a local `.venv`.
    let venv_root = std::env::var_os(EnvVars::VIRTUAL_ENV)
        .map(std::path::PathBuf::from)
        .or_else(|| {
            let candidate = project_dir.join(".venv");
            if candidate.is_dir() {
                Some(candidate)
            } else {
                None
            }
        });

    if let Some(venv_root) = venv_root {
        debug!("Activating virtual environment at: {}", venv_root.display());
        let venv_bin = if cfg!(windows) {
            venv_root.join("Scripts")
        } else {
            venv_root.join("bin")
        };
        let prompt_name = venv_root
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "venv".to_string());

        #[cfg(unix)]
        {
            spawn_unix_shell(shell, &venv_bin, &venv_root, &prompt_name, &[])
        }
        #[cfg(windows)]
        {
            spawn_windows_shell(shell, &venv_bin, &venv_root, &prompt_name, &[]).await
        }
    } else {
        debug!("No virtual environment found; spawning a plain shell");
        // Spawn a shell without activation.
        #[cfg(unix)]
        {
            spawn_unix_shell_plain(shell)
        }
        #[cfg(windows)]
        {
            spawn_windows_shell_plain(shell).await
        }
    }
}

/// Generate a shell-specific activation script.
///
/// `extra_path_dirs` are prepended to PATH before the venv bin directory, e.g. for ephemeral
/// `--with` environments.
fn generate_activation_script(
    shell: Shell,
    sys_prefix: &Path,
    venv_bin: &Path,
    prompt_name: &str,
    extra_path_dirs: &[PathBuf],
) -> String {
    let sys_prefix_str = sys_prefix.display();
    let venv_bin_str = venv_bin.display();
    let marker = "UV_SHELL_ACTIVATION_DONE";

    // Build the PATH value: extra dirs first, then venv bin.
    let path_prefix = extra_path_dirs
        .iter()
        .map(|p| p.display().to_string())
        .chain(std::iter::once(venv_bin_str.to_string()))
        .collect::<Vec<_>>();

    match shell {
        Shell::Bash | Shell::Zsh | Shell::Ksh => {
            let path_val = path_prefix.join(":");
            format!(
                r#"export VIRTUAL_ENV="{sys_prefix_str}"
export PATH="{path_val}:$PATH"
unset PYTHONHOME
export UV_SHELL_ACTIVE=1
PS1="({prompt_name}) ${{PS1-}}"
echo {marker}
"#
            )
        }
        Shell::Fish => {
            let set_path = path_prefix
                .iter()
                .map(|p| format!("\"{p}\""))
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                r#"set -gx VIRTUAL_ENV "{sys_prefix_str}"
set -gx PATH {set_path} $PATH
set -e PYTHONHOME
set -gx UV_SHELL_ACTIVE 1
functions -q fish_prompt; and functions -c fish_prompt _uv_old_fish_prompt
function fish_prompt; set_color green; echo -n "({prompt_name}) "; set_color normal; _uv_old_fish_prompt; end
echo {marker}
"#
            )
        }
        Shell::Csh => {
            let path_val = path_prefix.join(":");
            format!(
                r#"setenv VIRTUAL_ENV "{sys_prefix_str}"
setenv PATH "{path_val}:$PATH"
unsetenv PYTHONHOME
setenv UV_SHELL_ACTIVE 1
set prompt="({prompt_name}) $prompt"
echo {marker}
"#
            )
        }
        Shell::Nushell => {
            let items = path_prefix
                .iter()
                .map(|p| format!("\"{p}\""))
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                r#"$env.VIRTUAL_ENV = "{sys_prefix_str}"
$env.PATH = [{items} ...$env.PATH]
hide-env -i PYTHONHOME
$env.UV_SHELL_ACTIVE = "1"
echo {marker}
"#
            )
        }
        Shell::Powershell => {
            let path_val = path_prefix.join(";");
            format!(
                r#"$env:VIRTUAL_ENV = "{sys_prefix_str}"
$env:PATH = "{path_val};" + $env:PATH
Remove-Item Env:PYTHONHOME -ErrorAction SilentlyContinue
$env:UV_SHELL_ACTIVE = "1"
function prompt {{ "({prompt_name}) " + (Get-Location) + "> " }}
Write-Host "{marker}"
"#
            )
        }
        Shell::Cmd => {
            let path_val = path_prefix.join(";");
            format!(
                "set VIRTUAL_ENV={sys_prefix_str}\n\
                 set PATH={path_val};%PATH%\n\
                 set PYTHONHOME=\n\
                 set UV_SHELL_ACTIVE=1\n\
                 prompt ({prompt_name}) $P$G\n\
                 echo {marker}\n"
            )
        }
    }
}

/// Spawn a subshell on Unix using a PTY.
#[cfg(unix)]
fn spawn_unix_shell(
    shell: Shell,
    venv_bin: &Path,
    sys_prefix: &Path,
    prompt_name: &str,
    extra_path_dirs: &[PathBuf],
) -> anyhow::Result<ExitStatus> {
    use uv_pty::unix::PtySession;

    // Generate the activation script.
    let activation_script =
        generate_activation_script(shell, sys_prefix, venv_bin, prompt_name, extra_path_dirs);

    // Write the activation script to a temporary file.
    let temp_dir = tempfile::tempdir()?;
    let script_path = temp_dir.path().join("activate.sh");
    fs_err::write(&script_path, &activation_script)?;

    // Build the shell command.
    let mut command = std::process::Command::new(shell.executable());
    for arg in shell.interactive_args() {
        command.arg(arg);
    }

    // Spawn the shell in a PTY.
    let mut session =
        PtySession::new(command).with_context(|| format!("Failed to spawn {shell} shell"))?;

    // Send the activation command (space prefix to skip shell history).
    let source_cmd = shell.source_command(&script_path);
    session.send_line(&source_cmd)?;

    // Interact with the shell, waiting for the activation marker before showing output.
    let exit_code = session.interact(Some("UV_SHELL_ACTIVATION_DONE"))?;

    match exit_code {
        Some(0) | None => Ok(ExitStatus::Success),
        Some(code) => Ok(ExitStatus::External(u8::try_from(code).unwrap_or(1))),
    }
}

/// Spawn a subshell on Windows using a script-based approach.
#[cfg(windows)]
async fn spawn_windows_shell(
    shell: Shell,
    venv_bin: &Path,
    sys_prefix: &Path,
    prompt_name: &str,
    extra_path_dirs: &[PathBuf],
) -> anyhow::Result<ExitStatus> {
    use tokio::process::Command;

    // Generate the activation script.
    let activation_script =
        generate_activation_script(shell, sys_prefix, venv_bin, prompt_name, extra_path_dirs);

    // Write the activation script to a temporary file.
    let temp_dir = tempfile::tempdir()?;
    let (script_path, mut command) = match shell {
        Shell::Powershell => {
            let path = temp_dir.path().join("activate.ps1");
            fs_err::write(&path, &activation_script)?;
            let mut cmd = Command::new("pwsh");
            cmd.args(["-NoLogo", "-NoExit", "-File"]);
            cmd.arg(&path);
            (path, cmd)
        }
        Shell::Cmd => {
            let path = temp_dir.path().join("activate.bat");
            fs_err::write(&path, &activation_script)?;
            let mut cmd = Command::new("cmd");
            cmd.arg("/K");
            cmd.arg(&path);
            (path, cmd)
        }
        _ => {
            // For bash-like shells on Windows (Git Bash), use --init-file.
            let path = temp_dir.path().join("activate.sh");
            fs_err::write(&path, &activation_script)?;
            let mut cmd = Command::new(shell.executable());
            cmd.arg("--init-file");
            cmd.arg(&path);
            (path, cmd)
        }
    };

    // Ignore Ctrl-C in the parent; the child handles it.
    let _ = ctrlc::set_handler(|| {});

    let status = command.status().await?;

    match status.code() {
        Some(0) | None => Ok(ExitStatus::Success),
        Some(code) => Ok(ExitStatus::External(u8::try_from(code).unwrap_or(1))),
    }
}

/// Spawn a plain shell (no virtual environment activation) on Unix.
#[cfg(unix)]
fn spawn_unix_shell_plain(shell: Shell) -> anyhow::Result<ExitStatus> {
    use uv_pty::unix::PtySession;

    let mut command = std::process::Command::new(shell.executable());
    for arg in shell.interactive_args() {
        command.arg(arg);
    }

    let mut session =
        PtySession::new(command).with_context(|| format!("Failed to spawn {shell} shell"))?;

    let exit_code = session.interact(None)?;

    match exit_code {
        Some(0) | None => Ok(ExitStatus::Success),
        Some(code) => Ok(ExitStatus::External(u8::try_from(code).unwrap_or(1))),
    }
}

/// Spawn a plain shell (no virtual environment activation) on Windows.
#[cfg(windows)]
async fn spawn_windows_shell_plain(shell: Shell) -> anyhow::Result<ExitStatus> {
    use tokio::process::Command;

    let mut command = Command::new(shell.executable());
    for arg in shell.interactive_args() {
        command.arg(arg);
    }

    let _ = ctrlc::set_handler(|| {});
    let status = command.status().await?;

    match status.code() {
        Some(0) | None => Ok(ExitStatus::Success),
        Some(code) => Ok(ExitStatus::External(u8::try_from(code).unwrap_or(1))),
    }
}
