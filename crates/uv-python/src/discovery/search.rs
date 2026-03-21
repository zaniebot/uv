use super::*;

fn python_executables_from_virtual_environments<'a>(
    preview: Preview,
) -> impl Iterator<Item = Result<(PythonSource, PathBuf), Error>> + 'a {
    let from_active_environment = iter::once_with(|| {
        virtualenv_from_env()
            .into_iter()
            .map(virtualenv_python_executable)
            .map(|path| Ok((PythonSource::ActiveEnvironment, path)))
    })
    .flatten();

    // N.B. we prefer the conda environment over discovered virtual environments
    let from_conda_environment = iter::once_with(move || {
        conda_environment_from_env(CondaEnvironmentKind::Child, preview)
            .into_iter()
            .map(virtualenv_python_executable)
            .map(|path| Ok((PythonSource::CondaPrefix, path)))
    })
    .flatten();

    let from_discovered_environment = iter::once_with(|| {
        virtualenv_from_working_dir()
            .map(|path| {
                path.map(virtualenv_python_executable)
                    .map(|path| (PythonSource::DiscoveredEnvironment, path))
                    .into_iter()
            })
            .map_err(Error::from)
    })
    .flatten_ok();

    from_active_environment
        .chain(from_conda_environment)
        .chain(from_discovered_environment)
}

/// Lazily iterate over Python executables installed on the system.
///
/// The following sources are supported:
///
/// - Managed Python installations (e.g. `uv python install`)
/// - The search path (i.e. `PATH`)
/// - The registry (Windows only)
///
/// The ordering and presence of each source is determined by the [`PythonPreference`].
///
/// If a [`VersionRequest`] is provided, we will skip executables that we know do not satisfy the request
/// and (as discussed in [`python_executables_from_search_path`]) additional version-specific executables may
/// be included. However, the caller MUST query the returned executables to ensure they satisfy the request;
/// this function does not guarantee that the executables provide any particular version. See
/// [`find_python_installation`] instead.
///
/// This function does not guarantee that the executables are valid Python interpreters.
/// See [`python_interpreters_from_executables`].
fn python_executables_from_installed<'a>(
    version: &'a VersionRequest,
    implementation: Option<&'a ImplementationName>,
    platform: PlatformRequest,
    preference: PythonPreference,
) -> Box<dyn Iterator<Item = Result<(PythonSource, PathBuf), Error>> + 'a> {
    let from_managed_installations = iter::once_with(move || {
        ManagedPythonInstallations::from_settings(None)
            .map_err(Error::from)
            .and_then(|installed_installations| {
                debug!(
                    "Searching for managed installations at `{}`",
                    installed_installations.root().user_display()
                );
                let installations = installed_installations.find_matching_current_platform()?;

                let build_versions = python_build_versions_from_env()?;

                // Check that the Python version and platform satisfy the request to avoid
                // unnecessary interpreter queries later
                Ok(installations
                    .into_iter()
                    .filter(move |installation| {
                        if !version.matches_version(&installation.version()) {
                            debug!("Skipping managed installation `{installation}`: does not satisfy `{version}`");
                            return false;
                        }
                        if !platform.matches(installation.platform()) {
                            debug!("Skipping managed installation `{installation}`: does not satisfy requested platform `{platform}`");
                            return false;
                        }

                        if let Some(requested_build) = build_versions.get(&installation.implementation()) {
                            let Some(installation_build) = installation.build() else {
                                debug!(
                                    "Skipping managed installation `{installation}`: a build version was requested but is not recorded for this installation"
                                );
                                return false;
                            };
                            if installation_build != requested_build {
                                debug!(
                                    "Skipping managed installation `{installation}`: requested build version `{requested_build}` does not match installation build version `{installation_build}`"
                                );
                                return false;
                            }
                        }

                        true
                    })
                    .inspect(|installation| debug!("Found managed installation `{installation}`"))
                    .map(move |installation| {
                        // If it's not a patch version request, then attempt to read the stable
                        // minor version link.
                        let executable = version
                                .patch()
                                .is_none()
                                .then(|| {
                                    PythonMinorVersionLink::from_installation(
                                        &installation,
                                    )
                                    .filter(PythonMinorVersionLink::exists)
                                    .map(
                                        |minor_version_link| {
                                            minor_version_link.symlink_executable.clone()
                                        },
                                    )
                                })
                                .flatten()
                                .unwrap_or_else(|| installation.executable(false));
                        (PythonSource::Managed, executable)
                    })
                )
            })
    })
    .flatten_ok();

    let from_search_path = iter::once_with(move || {
        python_executables_from_search_path(version, implementation)
            .enumerate()
            .map(|(i, path)| {
                if i == 0 {
                    Ok((PythonSource::SearchPathFirst, path))
                } else {
                    Ok((PythonSource::SearchPath, path))
                }
            })
    })
    .flatten();

    let from_windows_registry = iter::once_with(move || {
        #[cfg(windows)]
        {
            // Skip interpreter probing if we already know the version doesn't match.
            let version_filter = move |entry: &WindowsPython| {
                if let Some(found) = &entry.version {
                    // Some distributions emit the patch version (example: `SysVersion: 3.9`)
                    if found.string.chars().filter(|c| *c == '.').count() == 1 {
                        version.matches_major_minor(found.major(), found.minor())
                    } else {
                        version.matches_version(found)
                    }
                } else {
                    true
                }
            };

            env::var_os(EnvVars::UV_TEST_PYTHON_PATH)
                .is_none()
                .then(|| {
                    registry_pythons()
                        .map(|entries| {
                            entries
                                .into_iter()
                                .filter(version_filter)
                                .map(|entry| (PythonSource::Registry, entry.path))
                                .chain(
                                    find_microsoft_store_pythons()
                                        .filter(version_filter)
                                        .map(|entry| (PythonSource::MicrosoftStore, entry.path)),
                                )
                        })
                        .map_err(Error::from)
                })
                .into_iter()
                .flatten_ok()
        }
        #[cfg(not(windows))]
        {
            Vec::new()
        }
    })
    .flatten();

    match preference {
        PythonPreference::OnlyManaged => {
            // TODO(zanieb): Ideally, we'd create "fake" managed installation directories for tests,
            // but for now... we'll just include the test interpreters which are always on the
            // search path.
            if std::env::var(uv_static::EnvVars::UV_INTERNAL__TEST_PYTHON_MANAGED).is_ok() {
                Box::new(from_managed_installations.chain(from_search_path))
            } else {
                Box::new(from_managed_installations)
            }
        }
        PythonPreference::Managed => Box::new(
            from_managed_installations
                .chain(from_search_path)
                .chain(from_windows_registry),
        ),
        PythonPreference::System => Box::new(
            from_search_path
                .chain(from_windows_registry)
                .chain(from_managed_installations),
        ),
        PythonPreference::OnlySystem => Box::new(from_search_path.chain(from_windows_registry)),
    }
}

/// Lazily iterate over all discoverable Python executables.
///
/// Note that Python executables may be excluded by the given [`EnvironmentPreference`],
/// [`PythonPreference`], and [`PlatformRequest`]. However, these filters are only applied for
/// performance. We cannot guarantee that the all requests or preferences are satisfied until we
/// query the interpreter.
///
/// See [`python_executables_from_installed`] and [`python_executables_from_virtual_environments`]
/// for more information on discovery.
fn python_executables<'a>(
    version: &'a VersionRequest,
    implementation: Option<&'a ImplementationName>,
    platform: PlatformRequest,
    environments: EnvironmentPreference,
    preference: PythonPreference,
    preview: Preview,
) -> Box<dyn Iterator<Item = Result<(PythonSource, PathBuf), Error>> + 'a> {
    // Always read from `UV_INTERNAL__PARENT_INTERPRETER` — it could be a system interpreter
    let from_parent_interpreter = iter::once_with(|| {
        env::var_os(EnvVars::UV_INTERNAL__PARENT_INTERPRETER)
            .into_iter()
            .map(|path| Ok((PythonSource::ParentInterpreter, PathBuf::from(path))))
    })
    .flatten();

    // Check if the base conda environment is active
    let from_base_conda_environment = iter::once_with(move || {
        conda_environment_from_env(CondaEnvironmentKind::Base, preview)
            .into_iter()
            .map(virtualenv_python_executable)
            .map(|path| Ok((PythonSource::BaseCondaPrefix, path)))
    })
    .flatten();

    let from_virtual_environments = python_executables_from_virtual_environments(preview);
    let from_installed =
        python_executables_from_installed(version, implementation, platform, preference);

    // Limit the search to the relevant environment preference; this avoids unnecessary work like
    // traversal of the file system. Subsequent filtering should be done by the caller with
    // `source_satisfies_environment_preference` and `interpreter_satisfies_environment_preference`.
    match environments {
        EnvironmentPreference::OnlyVirtual => {
            Box::new(from_parent_interpreter.chain(from_virtual_environments))
        }
        EnvironmentPreference::ExplicitSystem | EnvironmentPreference::Any => Box::new(
            from_parent_interpreter
                .chain(from_virtual_environments)
                .chain(from_base_conda_environment)
                .chain(from_installed),
        ),
        EnvironmentPreference::OnlySystem => Box::new(
            from_parent_interpreter
                .chain(from_base_conda_environment)
                .chain(from_installed),
        ),
    }
}

/// Lazily iterate over Python executables in the `PATH`.
///
/// The [`VersionRequest`] and [`ImplementationName`] are used to determine the possible
/// Python interpreter names, e.g. if looking for Python 3.9 we will look for `python3.9`
/// or if looking for `PyPy` we will look for `pypy` in addition to the default names.
///
/// Executables are returned in the search path order, then by specificity of the name, e.g.
/// `python3.9` is preferred over `python3` and `pypy3.9` is preferred over `python3.9`.
///
/// If a `version` is not provided, we will only look for default executable names e.g.
/// `python3` and `python` — `python3.9` and similar will not be included.
fn python_executables_from_search_path<'a>(
    version: &'a VersionRequest,
    implementation: Option<&'a ImplementationName>,
) -> impl Iterator<Item = PathBuf> + 'a {
    // `UV_TEST_PYTHON_PATH` can be used to override `PATH` to limit Python executable availability in the test suite
    let search_path = env::var_os(EnvVars::UV_TEST_PYTHON_PATH)
        .unwrap_or(env::var_os(EnvVars::PATH).unwrap_or_default());

    let possible_names: Vec<_> = version
        .executable_names(implementation)
        .into_iter()
        .map(|name| name.to_string())
        .collect();

    trace!(
        "Searching PATH for executables: {}",
        possible_names.join(", ")
    );

    // Split and iterate over the paths instead of using `which_all` so we can
    // check multiple names per directory while respecting the search path order and python names
    // precedence.
    let search_dirs: Vec<_> = env::split_paths(&search_path).collect();
    let mut seen_dirs = FxHashSet::with_capacity_and_hasher(search_dirs.len(), FxBuildHasher);
    search_dirs
        .into_iter()
        .filter(|dir| dir.is_dir())
        .flat_map(move |dir| {
            // Clone the directory for second closure
            let dir_clone = dir.clone();
            trace!(
                "Checking `PATH` directory for interpreters: {}",
                dir.display()
            );
            same_file::Handle::from_path(&dir)
                // Skip directories we've already seen, to avoid inspecting interpreters multiple
                // times when directories are repeated or symlinked in the `PATH`
                .map(|handle| seen_dirs.insert(handle))
                .inspect(|fresh_dir| {
                    if !fresh_dir {
                        trace!("Skipping already seen directory: {}", dir.display());
                    }
                })
                // If we cannot determine if the directory is unique, we'll assume it is
                .unwrap_or(true)
                .then(|| {
                    possible_names
                        .clone()
                        .into_iter()
                        .flat_map(move |name| {
                            // Since we're just working with a single directory at a time, we collect to simplify ownership
                            which::which_in_global(&*name, Some(&dir))
                                .into_iter()
                                .flatten()
                                // We have to collect since `which` requires that the regex outlives its
                                // parameters, and the dir is local while we return the iterator.
                                .collect::<Vec<_>>()
                        })
                        .chain(find_all_minor(implementation, version, &dir_clone))
                        .filter(|path| !is_windows_store_shim(path))
                        .inspect(|path| {
                            trace!("Found possible Python executable: {}", path.display());
                        })
                        .chain(
                            // TODO(zanieb): Consider moving `python.bat` into `possible_names` to avoid a chain
                            cfg!(windows)
                                .then(move || {
                                    which::which_in_global("python.bat", Some(&dir_clone))
                                        .into_iter()
                                        .flatten()
                                        .collect::<Vec<_>>()
                                })
                                .into_iter()
                                .flatten(),
                        )
                })
                .into_iter()
                .flatten()
        })
}

/// Find all acceptable `python3.x` minor versions.
///
/// For example, let's say `python` and `python3` are Python 3.10. When a user requests `>= 3.11`,
/// we still need to find a `python3.12` in PATH.
fn find_all_minor(
    implementation: Option<&ImplementationName>,
    version_request: &VersionRequest,
    dir: &Path,
) -> impl Iterator<Item = PathBuf> + use<> {
    match version_request {
        &VersionRequest::Any
        | VersionRequest::Default
        | VersionRequest::Major(_, _)
        | VersionRequest::Range(_, _) => {
            let regex = if let Some(implementation) = implementation {
                Regex::new(&format!(
                    r"^({}|python3)\.(?<minor>\d\d?)t?{}$",
                    regex::escape(&implementation.to_string()),
                    regex::escape(EXE_SUFFIX)
                ))
                .unwrap()
            } else {
                Regex::new(&format!(
                    r"^python3\.(?<minor>\d\d?)t?{}$",
                    regex::escape(EXE_SUFFIX)
                ))
                .unwrap()
            };
            let all_minors = fs_err::read_dir(dir)
                .into_iter()
                .flatten()
                .flatten()
                .map(|entry| entry.path())
                .filter(move |path| {
                    let Some(filename) = path.file_name() else {
                        return false;
                    };
                    let Some(filename) = filename.to_str() else {
                        return false;
                    };
                    let Some(captures) = regex.captures(filename) else {
                        return false;
                    };

                    // Filter out interpreter we already know have a too low minor version.
                    let minor = captures["minor"].parse().ok();
                    if let Some(minor) = minor {
                        // Optimization: Skip generally unsupported Python versions without querying.
                        if minor < 6 {
                            return false;
                        }
                        // Optimization 2: Skip excluded Python (minor) versions without querying.
                        if !version_request.matches_major_minor(3, minor) {
                            return false;
                        }
                    }
                    true
                })
                .filter(|path| is_executable(path))
                .collect::<Vec<_>>();
            Either::Left(all_minors.into_iter())
        }
        VersionRequest::MajorMinor(_, _, _)
        | VersionRequest::MajorMinorPatch(_, _, _, _)
        | VersionRequest::MajorMinorPrerelease(_, _, _, _) => Either::Right(iter::empty()),
    }
}

/// Lazily iterate over all discoverable Python interpreters.
///
/// Note interpreters may be excluded by the given [`EnvironmentPreference`], [`PythonPreference`],
/// [`VersionRequest`], or [`PlatformRequest`].
///
/// The [`PlatformRequest`] is currently only applied to managed Python installations before querying
/// the interpreter. The caller is responsible for ensuring it is applied otherwise.
///
/// See [`python_executables`] for more information on discovery.
fn python_installations<'a>(
    version: &'a VersionRequest,
    implementation: Option<&'a ImplementationName>,
    platform: PlatformRequest,
    environments: EnvironmentPreference,
    preference: PythonPreference,
    cache: &'a Cache,
    preview: Preview,
) -> impl Iterator<Item = Result<PythonInstallation, Error>> + 'a {
    let installations = python_installations_from_executables(
        // Perform filtering on the discovered executables based on their source. This avoids
        // unnecessary interpreter queries, which are generally expensive. We'll filter again
        // with `interpreter_satisfies_environment_preference` after querying.
        python_executables(
            version,
            implementation,
            platform,
            environments,
            preference,
            preview,
        )
        .filter_ok(move |(source, path)| {
            source_satisfies_environment_preference(*source, path, environments)
        }),
        cache,
    )
    .filter_ok(move |installation| {
        interpreter_satisfies_environment_preference(
            installation.source,
            &installation.interpreter,
            environments,
        )
    })
    .filter_ok(move |installation| {
        let request = version.clone().into_request_for_source(installation.source);
        if request.matches_interpreter(&installation.interpreter) {
            true
        } else {
            debug!(
                "Skipping interpreter at `{}` from {}: does not satisfy request `{request}`",
                installation.interpreter.sys_executable().user_display(),
                installation.source,
            );
            false
        }
    })
    .filter_ok(move |installation| preference.allows_installation(installation));

    if std::env::var(uv_static::EnvVars::UV_INTERNAL__TEST_PYTHON_MANAGED).is_ok() {
        Either::Left(installations.map_ok(|mut installation| {
            // In test mode, change the source to `Managed` if a version was marked as such via
            // `TestContext::with_versions_as_managed`.
            if installation.interpreter.is_managed() {
                installation.source = PythonSource::Managed;
            }
            installation
        }))
    } else {
        Either::Right(installations)
    }
}

/// Lazily convert Python executables into installations.
fn python_installations_from_executables<'a>(
    executables: impl Iterator<Item = Result<(PythonSource, PathBuf), Error>> + 'a,
    cache: &'a Cache,
) -> impl Iterator<Item = Result<PythonInstallation, Error>> + 'a {
    executables.map(|result| match result {
        Ok((source, path)) => Interpreter::query(&path, cache)
            .map(|interpreter| PythonInstallation {
                source,
                interpreter,
            })
            .inspect(|installation| {
                debug!(
                    "Found `{}` at `{}` ({source})",
                    installation.key(),
                    path.display()
                );
            })
            .map_err(|err| Error::Query(Box::new(err), path, source))
            .inspect_err(|err| debug!("{err}")),
        Err(err) => Err(err),
    })
}

/// Whether a [`Interpreter`] matches the [`EnvironmentPreference`].
///
/// This is the correct way to determine if an interpreter matches the preference. In contrast,
/// [`source_satisfies_environment_preference`] only checks if a [`PythonSource`] **could** satisfy
/// preference as a pre-filtering step. We cannot definitively know if a Python interpreter is in
/// a virtual environment until we query it.
fn interpreter_satisfies_environment_preference(
    source: PythonSource,
    interpreter: &Interpreter,
    preference: EnvironmentPreference,
) -> bool {
    match (
        preference,
        // Conda environments are not conformant virtual environments but we treat them as such.
        interpreter.is_virtualenv() || (matches!(source, PythonSource::CondaPrefix)),
    ) {
        (EnvironmentPreference::Any, _) => true,
        (EnvironmentPreference::OnlyVirtual, true) => true,
        (EnvironmentPreference::OnlyVirtual, false) => {
            debug!(
                "Ignoring Python interpreter at `{}`: only virtual environments allowed",
                interpreter.sys_executable().display()
            );
            false
        }
        (EnvironmentPreference::ExplicitSystem, true) => true,
        (EnvironmentPreference::ExplicitSystem, false) => {
            if matches!(
                source,
                PythonSource::ProvidedPath | PythonSource::ParentInterpreter
            ) {
                debug!(
                    "Allowing explicitly requested system Python interpreter at `{}`",
                    interpreter.sys_executable().display()
                );
                true
            } else {
                debug!(
                    "Ignoring Python interpreter at `{}`: system interpreter not explicitly requested",
                    interpreter.sys_executable().display()
                );
                false
            }
        }
        (EnvironmentPreference::OnlySystem, true) => {
            debug!(
                "Ignoring Python interpreter at `{}`: system interpreter required",
                interpreter.sys_executable().display()
            );
            false
        }
        (EnvironmentPreference::OnlySystem, false) => true,
    }
}

/// Returns true if a [`PythonSource`] could satisfy the [`EnvironmentPreference`].
///
/// This is useful as a pre-filtering step. Use of [`interpreter_satisfies_environment_preference`]
/// is required to determine if an [`Interpreter`] satisfies the preference.
///
/// The interpreter path is only used for debug messages.
fn source_satisfies_environment_preference(
    source: PythonSource,
    interpreter_path: &Path,
    preference: EnvironmentPreference,
) -> bool {
    match preference {
        EnvironmentPreference::Any => true,
        EnvironmentPreference::OnlyVirtual => {
            if source.is_maybe_virtualenv() {
                true
            } else {
                debug!(
                    "Ignoring Python interpreter at `{}`: only virtual environments allowed",
                    interpreter_path.display()
                );
                false
            }
        }
        EnvironmentPreference::ExplicitSystem => {
            if source.is_maybe_virtualenv() {
                true
            } else {
                debug!(
                    "Ignoring Python interpreter at `{}`: system interpreter not explicitly requested",
                    interpreter_path.display()
                );
                false
            }
        }
        EnvironmentPreference::OnlySystem => {
            if source.is_maybe_system() {
                true
            } else {
                debug!(
                    "Ignoring Python interpreter at `{}`: system interpreter required",
                    interpreter_path.display()
                );
                false
            }
        }
    }
}

/// Check if an encountered error is critical and should stop discovery.
///
/// Returns false when an error could be due to a faulty Python installation and we should continue searching for a working one.
impl Error {
    pub fn is_critical(&self) -> bool {
        match self {
            // When querying the Python interpreter fails, we will only raise errors that demonstrate that something is broken
            // If the Python interpreter returned a bad response, we'll continue searching for one that works
            Self::Query(err, _, source) => match &**err {
                InterpreterError::Encode(_)
                | InterpreterError::Io(_)
                | InterpreterError::SpawnFailed { .. } => true,
                InterpreterError::UnexpectedResponse(UnexpectedResponseError { path, .. })
                | InterpreterError::StatusCode(StatusCodeError { path, .. }) => {
                    debug!(
                        "Skipping bad interpreter at {} from {source}: {err}",
                        path.display()
                    );
                    false
                }
                InterpreterError::QueryScript { path, err } => {
                    debug!(
                        "Skipping bad interpreter at {} from {source}: {err}",
                        path.display()
                    );
                    false
                }
                #[cfg(windows)]
                InterpreterError::CorruptWindowsPackage { path, err } => {
                    debug!(
                        "Skipping bad interpreter at {} from {source}: {err}",
                        path.display()
                    );
                    false
                }
                InterpreterError::PermissionDenied { path, err } => {
                    debug!(
                        "Skipping unexecutable interpreter at {} from {source}: {err}",
                        path.display()
                    );
                    false
                }
                InterpreterError::NotFound(path)
                | InterpreterError::BrokenLink(BrokenLink { path, .. }) => {
                    // If the interpreter is from an active, valid virtual environment, we should
                    // fail because it's broken
                    if matches!(source, PythonSource::ActiveEnvironment)
                        && uv_fs::is_virtualenv_executable(path)
                    {
                        true
                    } else {
                        trace!("Skipping missing interpreter at {}", path.display());
                        false
                    }
                }
            },
            Self::VirtualEnv(VirtualEnvError::MissingPyVenvCfg(path)) => {
                trace!("Skipping broken virtualenv at {}", path.display());
                false
            }
            _ => true,
        }
    }
}

/// Create a [`PythonInstallation`] from a Python interpreter path.
fn python_installation_from_executable(
    path: &PathBuf,
    cache: &Cache,
) -> Result<PythonInstallation, crate::interpreter::Error> {
    Ok(PythonInstallation {
        source: PythonSource::ProvidedPath,
        interpreter: Interpreter::query(path, cache)?,
    })
}

/// Create a [`PythonInstallation`] from a Python installation root directory.
fn python_installation_from_directory(
    path: &PathBuf,
    cache: &Cache,
) -> Result<PythonInstallation, crate::interpreter::Error> {
    let executable = virtualenv_python_executable(path);
    python_installation_from_executable(&executable, cache)
}

/// Lazily iterate over all Python installations on the path with the given executable name.
fn python_installations_with_executable_name<'a>(
    name: &'a str,
    cache: &'a Cache,
) -> impl Iterator<Item = Result<PythonInstallation, Error>> + 'a {
    python_installations_from_executables(
        which_all(name)
            .into_iter()
            .flat_map(|inner| inner.map(|path| Ok((PythonSource::SearchPath, path)))),
        cache,
    )
}

/// Iterate over all Python installations that satisfy the given request.
pub fn find_python_installations<'a>(
    request: &'a PythonRequest,
    environments: EnvironmentPreference,
    preference: PythonPreference,
    cache: &'a Cache,
    preview: Preview,
) -> Box<dyn Iterator<Item = Result<FindPythonResult, Error>> + 'a> {
    let sources = DiscoveryPreferences {
        python_preference: preference,
        environment_preference: environments,
    }
    .sources(request);

    match request {
        PythonRequest::File(path) => Box::new(iter::once({
            if preference.allows_source(PythonSource::ProvidedPath) {
                debug!("Checking for Python interpreter at {request}");
                match python_installation_from_executable(path, cache) {
                    Ok(installation) => Ok(Ok(installation)),
                    Err(InterpreterError::NotFound(_) | InterpreterError::BrokenLink(_)) => {
                        Ok(Err(PythonNotFound {
                            request: request.clone(),
                            python_preference: preference,
                            environment_preference: environments,
                        }))
                    }
                    Err(err) => Err(Error::Query(
                        Box::new(err),
                        path.clone(),
                        PythonSource::ProvidedPath,
                    )),
                }
            } else {
                Err(Error::SourceNotAllowed(
                    request.clone(),
                    PythonSource::ProvidedPath,
                    preference,
                ))
            }
        })),
        PythonRequest::Directory(path) => Box::new(iter::once({
            if preference.allows_source(PythonSource::ProvidedPath) {
                debug!("Checking for Python interpreter in {request}");
                match python_installation_from_directory(path, cache) {
                    Ok(installation) => Ok(Ok(installation)),
                    Err(InterpreterError::NotFound(_) | InterpreterError::BrokenLink(_)) => {
                        Ok(Err(PythonNotFound {
                            request: request.clone(),
                            python_preference: preference,
                            environment_preference: environments,
                        }))
                    }
                    Err(err) => Err(Error::Query(
                        Box::new(err),
                        path.clone(),
                        PythonSource::ProvidedPath,
                    )),
                }
            } else {
                Err(Error::SourceNotAllowed(
                    request.clone(),
                    PythonSource::ProvidedPath,
                    preference,
                ))
            }
        })),
        PythonRequest::ExecutableName(name) => {
            if preference.allows_source(PythonSource::SearchPath) {
                debug!("Searching for Python interpreter with {request}");
                Box::new(
                    python_installations_with_executable_name(name, cache)
                        .filter_ok(move |installation| {
                            interpreter_satisfies_environment_preference(
                                installation.source,
                                &installation.interpreter,
                                environments,
                            )
                        })
                        .map_ok(Ok),
                )
            } else {
                Box::new(iter::once(Err(Error::SourceNotAllowed(
                    request.clone(),
                    PythonSource::SearchPath,
                    preference,
                ))))
            }
        }
        PythonRequest::Any => Box::new({
            debug!("Searching for any Python interpreter in {sources}");
            python_installations(
                &VersionRequest::Any,
                None,
                PlatformRequest::default(),
                environments,
                preference,
                cache,
                preview,
            )
            .map_ok(Ok)
        }),
        PythonRequest::Default => Box::new({
            debug!("Searching for default Python interpreter in {sources}");
            python_installations(
                &VersionRequest::Default,
                None,
                PlatformRequest::default(),
                environments,
                preference,
                cache,
                preview,
            )
            .map_ok(Ok)
        }),
        PythonRequest::Version(version) => {
            if let Err(err) = version.check_supported() {
                return Box::new(iter::once(Err(Error::InvalidVersionRequest(err))));
            }
            Box::new({
                debug!("Searching for {request} in {sources}");
                python_installations(
                    version,
                    None,
                    PlatformRequest::default(),
                    environments,
                    preference,
                    cache,
                    preview,
                )
                .map_ok(Ok)
            })
        }
        PythonRequest::Implementation(implementation) => Box::new({
            debug!("Searching for a {request} interpreter in {sources}");
            python_installations(
                &VersionRequest::Default,
                Some(implementation),
                PlatformRequest::default(),
                environments,
                preference,
                cache,
                preview,
            )
            .filter_ok(|installation| implementation.matches_interpreter(&installation.interpreter))
            .map_ok(Ok)
        }),
        PythonRequest::ImplementationVersion(implementation, version) => {
            if let Err(err) = version.check_supported() {
                return Box::new(iter::once(Err(Error::InvalidVersionRequest(err))));
            }
            Box::new({
                debug!("Searching for {request} in {sources}");
                python_installations(
                    version,
                    Some(implementation),
                    PlatformRequest::default(),
                    environments,
                    preference,
                    cache,
                    preview,
                )
                .filter_ok(|installation| {
                    implementation.matches_interpreter(&installation.interpreter)
                })
                .map_ok(Ok)
            })
        }
        PythonRequest::Key(request) => {
            if let Some(version) = request.version() {
                if let Err(err) = version.check_supported() {
                    return Box::new(iter::once(Err(Error::InvalidVersionRequest(err))));
                }
            }

            Box::new({
                debug!("Searching for {request} in {sources}");
                python_installations(
                    request.version().unwrap_or(&VersionRequest::Default),
                    request.implementation(),
                    request.platform(),
                    environments,
                    preference,
                    cache,
                    preview,
                )
                .filter_ok(move |installation| {
                    request.satisfied_by_interpreter(&installation.interpreter)
                })
                .map_ok(Ok)
            })
        }
    }
}

/// Find a Python installation that satisfies the given request.
///
/// If an error is encountered while locating or inspecting a candidate installation,
/// the error will raised instead of attempting further candidates.
pub(crate) fn find_python_installation(
    request: &PythonRequest,
    environments: EnvironmentPreference,
    preference: PythonPreference,
    cache: &Cache,
    preview: Preview,
) -> Result<FindPythonResult, Error> {
    let installations =
        find_python_installations(request, environments, preference, cache, preview);
    let mut first_prerelease = None;
    let mut first_debug = None;
    let mut first_managed = None;
    let mut first_error = None;
    for result in installations {
        // Iterate until the first critical error or happy result
        if !result.as_ref().err().is_none_or(Error::is_critical) {
            // Track the first non-critical error
            if first_error.is_none() {
                if let Err(err) = result {
                    first_error = Some(err);
                }
            }
            continue;
        }

        // If it's an error, we're done.
        let Ok(Ok(ref installation)) = result else {
            return result;
        };

        // Check if we need to skip the interpreter because it is "not allowed", e.g., if it is a
        // pre-release version or an alternative implementation, using it requires opt-in.

        // If the interpreter has a default executable name, e.g. `python`, and was found on the
        // search path, we consider this opt-in to use it.
        let has_default_executable_name = installation.interpreter.has_default_executable_name()
            && matches!(
                installation.source,
                PythonSource::SearchPath | PythonSource::SearchPathFirst
            );

        // If it's a pre-release and pre-releases aren't allowed, skip it — but store it for later
        // since we'll use a pre-release if no other versions are available.
        if installation.python_version().pre().is_some()
            && !request.allows_prereleases()
            && !installation.source.allows_prereleases()
            && !has_default_executable_name
        {
            debug!("Skipping pre-release installation {}", installation.key());
            if first_prerelease.is_none() {
                first_prerelease = Some(installation.clone());
            }
            continue;
        }

        // If it's a debug build and debug builds aren't allowed, skip it — but store it for later
        // since we'll use a debug build if no other versions are available.
        if installation.key().variant().is_debug()
            && !request.allows_debug()
            && !installation.source.allows_debug()
            && !has_default_executable_name
        {
            debug!("Skipping debug installation {}", installation.key());
            if first_debug.is_none() {
                first_debug = Some(installation.clone());
            }
            continue;
        }

        // If it's an alternative implementation and alternative implementations aren't allowed,
        // skip it. Note we avoid querying these interpreters at all if they're on the search path
        // and are not requested, but other sources such as the managed installations can include
        // them.
        if installation.is_alternative_implementation()
            && !request.allows_alternative_implementations()
            && !installation.source.allows_alternative_implementations()
            && !has_default_executable_name
        {
            debug!("Skipping alternative implementation {}", installation.key());
            continue;
        }

        // If it's a managed Python installation, and system interpreters are preferred, skip it
        // for now.
        if matches!(preference, PythonPreference::System) && installation.is_managed() {
            debug!(
                "Skipping managed installation {}: system installation preferred",
                installation.key()
            );
            if first_managed.is_none() {
                first_managed = Some(installation.clone());
            }
            continue;
        }

        // If we didn't skip it, this is the installation to use
        return result;
    }

    // If we only found managed installations, and the preference allows them, we should return
    // the first one.
    if let Some(installation) = first_managed {
        debug!(
            "Allowing managed installation {}: no system installations",
            installation.key()
        );
        return Ok(Ok(installation));
    }

    // If we only found debug installations, they're implicitly allowed and we should return the
    // first one.
    if let Some(installation) = first_debug {
        debug!(
            "Allowing debug installation {}: no non-debug installations",
            installation.key()
        );
        return Ok(Ok(installation));
    }

    // If we only found pre-releases, they're implicitly allowed and we should return the first one.
    if let Some(installation) = first_prerelease {
        debug!(
            "Allowing pre-release installation {}: no stable installations",
            installation.key()
        );
        return Ok(Ok(installation));
    }

    // If we found a Python, but it was unusable for some reason, report that instead of saying we
    // couldn't find any Python interpreters.
    if let Some(err) = first_error {
        return Err(err);
    }

    Ok(Err(PythonNotFound {
        request: request.clone(),
        environment_preference: environments,
        python_preference: preference,
    }))
}

/// Find the best-matching Python installation.
///
/// If no Python version is provided, we will use the first available installation.
///
/// If a Python version is provided, we will first try to find an exact match. If
/// that cannot be found and a patch version was requested, we will look for a match
/// without comparing the patch version number. If that cannot be found, we fall back to
/// the first available version.
///
/// At all points, if the specified version cannot be found, we will attempt to
/// download it if downloads are enabled.
///
/// See [`find_python_installation`] for more details on installation discovery.
#[instrument(skip_all, fields(request))]
pub(crate) async fn find_best_python_installation(
    request: &PythonRequest,
    environments: EnvironmentPreference,
    preference: PythonPreference,
    downloads_enabled: bool,
    download_list: &ManagedPythonDownloadList,
    client: &BaseClient,
    retry_policy: &ExponentialBackoff,
    cache: &Cache,
    reporter: Option<&dyn crate::downloads::Reporter>,
    python_install_mirror: Option<&str>,
    pypy_install_mirror: Option<&str>,
    preview: Preview,
) -> Result<PythonInstallation, crate::Error> {
    debug!("Starting Python discovery for {request}");
    let original_request = request;

    let mut previous_fetch_failed = false;

    let request_without_patch = match request {
        PythonRequest::Version(version) => {
            if version.has_patch() {
                Some(PythonRequest::Version(version.clone().without_patch()))
            } else {
                None
            }
        }
        PythonRequest::ImplementationVersion(implementation, version) => Some(
            PythonRequest::ImplementationVersion(*implementation, version.clone().without_patch()),
        ),
        _ => None,
    };

    for (attempt, request) in iter::once(original_request)
        .chain(request_without_patch.iter())
        .chain(iter::once(&PythonRequest::Default))
        .enumerate()
    {
        debug!(
            "Looking for {request}{}",
            if request != original_request {
                format!(" attempt {attempt} (fallback after failing to find: {original_request})")
            } else {
                String::new()
            }
        );
        let result = find_python_installation(request, environments, preference, cache, preview);
        let error = match result {
            Ok(Ok(installation)) => {
                warn_on_unsupported_python(installation.interpreter());
                return Ok(installation);
            }
            // Continue if we can't find a matching Python and ignore non-critical discovery errors
            Ok(Err(error)) => error.into(),
            Err(error) if !error.is_critical() => error.into(),
            Err(error) => return Err(error.into()),
        };

        // Attempt to download the version if downloads are enabled
        if downloads_enabled
            && !previous_fetch_failed
            && let Some(download_request) = PythonDownloadRequest::from_request(request)
        {
            let download = download_request
                .clone()
                .fill()
                .map(|request| download_list.find(&request));

            let result = match download {
                Ok(Ok(download)) => PythonInstallation::fetch(
                    download,
                    client,
                    retry_policy,
                    cache,
                    reporter,
                    python_install_mirror,
                    pypy_install_mirror,
                )
                .await
                .map(Some),
                Ok(Err(crate::downloads::Error::NoDownloadFound(_))) => Ok(None),
                Ok(Err(error)) => Err(error.into()),
                Err(error) => Err(error.into()),
            };
            if let Ok(Some(installation)) = result {
                return Ok(installation);
            }
            // Emit a warning instead of failing since we may find a suitable
            // interpreter on the system after relaxing the request further.
            // Additionally, uv did not previously attempt downloads in this
            // code path and we want to minimize the fatal cases for
            // backwards compatibility.
            // Errors encountered here are either network errors or quirky
            // configuration problems.
            if let Err(error) = result {
                // If the request was for the default or any version, propagate
                // the error as nothing else we are about to do will help the
                // situation.
                if matches!(request, PythonRequest::Default | PythonRequest::Any) {
                    return Err(error);
                }

                let mut error_chain = String::new();
                // Writing to a string can't fail with errors (panics on allocation failure)
                let error = anyhow::Error::from(error).context(format!(
                    "A managed Python download is available for {request}, but an error occurred when attempting to download it."
                ));
                uv_warnings::write_error_chain(
                    error.as_ref(),
                    &mut error_chain,
                    "warning",
                    AnsiColors::Yellow,
                )
                .unwrap();
                anstream::eprint!("{}", error_chain);
                previous_fetch_failed = true;
            }
        }

        // If this was a request for the Default or Any version, this means that
        // either that's what we were called with, or we're on the last
        // iteration.
        //
        // The most recent find error therefore becomes a fatal one.
        if matches!(request, PythonRequest::Default | PythonRequest::Any) {
            return Err(match error {
                crate::Error::MissingPython(err, _) => PythonNotFound {
                    // Use a more general error in this case since we looked for multiple versions
                    request: original_request.clone(),
                    python_preference: err.python_preference,
                    environment_preference: err.environment_preference,
                }
                .into(),
                other => other,
            });
        }
    }

    unreachable!("The loop should have terminated when it reached PythonRequest::Default");
}

/// Display a warning if the Python version of the [`Interpreter`] is unsupported by uv.
fn warn_on_unsupported_python(interpreter: &Interpreter) {
    // Warn on usage with an unsupported Python version
    if interpreter.python_tuple() < (3, 8) {
        warn_user_once!(
            "uv is only compatible with Python >=3.8, found Python {}",
            interpreter.python_version()
        );
    }
}

/// On Windows we might encounter the Windows Store proxy shim (enabled in:
/// Settings/Apps/Advanced app settings/App execution aliases). When Python is _not_ installed
/// via the Windows Store, but the proxy shim is enabled, then executing `python.exe` or
/// `python3.exe` will redirect to the Windows Store installer.
///
/// We need to detect that these `python.exe` and `python3.exe` files are _not_ Python
/// executables.
///
/// This method is taken from Rye:
///
/// > This is a pretty dumb way.  We know how to parse this reparse point, but Microsoft
/// > does not want us to do this as the format is unstable.  So this is a best effort way.
/// > we just hope that the reparse point has the python redirector in it, when it's not
/// > pointing to a valid Python.
///
/// See: <https://github.com/astral-sh/rye/blob/b0e9eccf05fe4ff0ae7b0250a248c54f2d780b4d/rye/src/cli/shim.rs#L108>
#[cfg(windows)]
pub(crate) fn is_windows_store_shim(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;
    use std::os::windows::prelude::OsStrExt;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_BACKUP_SEMANTICS,
        FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_MODE, MAXIMUM_REPARSE_DATA_BUFFER_SIZE,
        OPEN_EXISTING,
    };
    use windows::Win32::System::IO::DeviceIoControl;
    use windows::Win32::System::Ioctl::FSCTL_GET_REPARSE_POINT;
    use windows::core::PCWSTR;

    // The path must be absolute.
    if !path.is_absolute() {
        return false;
    }

    // The path must point to something like:
    //   `C:\Users\crmar\AppData\Local\Microsoft\WindowsApps\python3.exe`
    let mut components = path.components().rev();

    // Ex) `python.exe`, `python3.exe`, `python3.12.exe`, etc.
    if !components
        .next()
        .and_then(|component| component.as_os_str().to_str())
        .is_some_and(|component| {
            component.starts_with("python")
                && std::path::Path::new(component)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
        })
    {
        return false;
    }

    // Ex) `WindowsApps`
    if components
        .next()
        .is_none_or(|component| component.as_os_str() != "WindowsApps")
    {
        return false;
    }

    // Ex) `Microsoft`
    if components
        .next()
        .is_none_or(|component| component.as_os_str() != "Microsoft")
    {
        return false;
    }

    // The file is only relevant if it's a reparse point.
    let Ok(md) = fs_err::symlink_metadata(path) else {
        return false;
    };
    if md.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT.0 == 0 {
        return false;
    }

    let mut path_encoded = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    // SAFETY: The path is null-terminated.
    #[allow(unsafe_code)]
    let reparse_handle = unsafe {
        CreateFileW(
            PCWSTR(path_encoded.as_mut_ptr()),
            0,
            FILE_SHARE_MODE(0),
            None,
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
            None,
        )
    };

    let Ok(reparse_handle) = reparse_handle else {
        return false;
    };

    let mut buf = [0u16; MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize];
    let mut bytes_returned = 0;

    // SAFETY: The buffer is large enough to hold the reparse point.
    #[allow(unsafe_code, clippy::cast_possible_truncation)]
    let success = unsafe {
        DeviceIoControl(
            reparse_handle,
            FSCTL_GET_REPARSE_POINT,
            None,
            0,
            Some(buf.as_mut_ptr().cast()),
            buf.len() as u32 * 2,
            Some(&raw mut bytes_returned),
            None,
        )
        .is_ok()
    };

    // SAFETY: The handle is valid.
    #[allow(unsafe_code)]
    unsafe {
        let _ = CloseHandle(reparse_handle);
    }

    // If the operation failed, assume it's not a reparse point.
    if !success {
        return false;
    }

    let reparse_point = String::from_utf16_lossy(&buf[..bytes_returned as usize]);
    reparse_point.contains("\\AppInstallerPythonRedirector.exe")
}

/// On Unix, we do not need to deal with Windows store shims.
///
/// See the Windows implementation for details.
#[cfg(not(windows))]
fn is_windows_store_shim(_path: &Path) -> bool {
    false
}
