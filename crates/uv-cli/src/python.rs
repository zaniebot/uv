use super::*;

#[derive(Args)]
pub struct PythonNamespace {
    #[command(subcommand)]
    pub command: PythonCommand,
}

#[derive(Subcommand)]
pub enum PythonCommand {
    /// List the available Python installations.
    ///
    /// By default, installed Python versions and the downloads for latest available patch version
    /// of each supported Python major version are shown.
    ///
    /// Use `--managed-python` to view only managed Python versions.
    ///
    /// Use `--no-managed-python` to omit managed Python versions.
    ///
    /// Use `--all-versions` to view all available patch versions.
    ///
    /// Use `--only-installed` to omit available downloads.
    #[command(alias = "ls")]
    List(PythonListArgs),

    /// Download and install Python versions.
    ///
    /// Supports CPython and PyPy. CPython distributions are downloaded from the Astral
    /// `python-build-standalone` project. PyPy distributions are downloaded from `python.org`. The
    /// available Python versions are bundled with each uv release. To install new Python versions,
    /// you may need upgrade uv.
    ///
    /// Python versions are installed into the uv Python directory, which can be retrieved with `uv
    /// python dir`.
    ///
    /// By default, Python executables are added to a directory on the path with a minor version
    /// suffix, e.g., `python3.13`. To install `python3` and `python`, use the `--default` flag. Use
    /// `uv python dir --bin` to see the target directory.
    ///
    /// Multiple Python versions may be requested.
    ///
    /// See `uv help python` to view supported request formats.
    Install(PythonInstallArgs),

    /// Upgrade installed Python versions.
    ///
    /// Upgrades versions to the latest supported patch release. Requires the `python-upgrade`
    /// preview feature.
    ///
    /// A target Python minor version to upgrade may be provided, e.g., `3.13`. Multiple versions
    /// may be provided to perform more than one upgrade.
    ///
    /// If no target version is provided, then uv will upgrade all managed CPython versions.
    ///
    /// During an upgrade, uv will not uninstall outdated patch versions.
    ///
    /// When an upgrade is performed, virtual environments created by uv will automatically
    /// use the new version. However, if the virtual environment was created before the
    /// upgrade functionality was added, it will continue to use the old Python version; to enable
    /// upgrades, the environment must be recreated.
    ///
    /// Upgrades are not yet supported for alternative implementations, like PyPy.
    Upgrade(PythonUpgradeArgs),

    /// Search for a Python installation.
    ///
    /// Displays the path to the Python executable.
    ///
    /// See `uv help python` to view supported request formats and details on discovery behavior.
    Find(PythonFindArgs),

    /// Pin to a specific Python version.
    ///
    /// Writes the pinned Python version to a `.python-version` file, which is used by other uv
    /// commands to determine the required Python version.
    ///
    /// If no version is provided, uv will look for an existing `.python-version` file and display
    /// the currently pinned version. If no `.python-version` file is found, uv will exit with an
    /// error.
    ///
    /// See `uv help python` to view supported request formats.
    Pin(PythonPinArgs),

    /// Show the uv Python installation directory.
    ///
    /// By default, Python installations are stored in the uv data directory at
    /// `$XDG_DATA_HOME/uv/python` or `$HOME/.local/share/uv/python` on Unix and
    /// `%APPDATA%\uv\data\python` on Windows.
    ///
    /// The Python installation directory may be overridden with `$UV_PYTHON_INSTALL_DIR`.
    ///
    /// To view the directory where uv installs Python executables instead, use the `--bin` flag.
    /// The Python executable directory may be overridden with `$UV_PYTHON_BIN_DIR`. Note that
    /// Python executables are only installed when preview mode is enabled.
    Dir(PythonDirArgs),

    /// Uninstall Python versions.
    Uninstall(PythonUninstallArgs),

    /// Ensure that the Python executable directory is on the `PATH`.
    ///
    /// If the Python executable directory is not present on the `PATH`, uv will attempt to add it to
    /// the relevant shell configuration files.
    ///
    /// If the shell configuration files already include a blurb to add the executable directory to
    /// the path, but the directory is not present on the `PATH`, uv will exit with an error.
    ///
    /// The Python executable directory is determined according to the XDG standard and can be
    /// retrieved with `uv python dir --bin`.
    #[command(alias = "ensurepath")]
    UpdateShell,
}

#[derive(Args)]
pub struct PythonListArgs {
    /// A Python request to filter by.
    ///
    /// See `uv help python` to view supported request formats.
    pub request: Option<String>,

    /// List all Python versions, including old patch versions.
    ///
    /// By default, only the latest patch version is shown for each minor version.
    #[arg(long)]
    pub all_versions: bool,

    /// List Python downloads for all platforms.
    ///
    /// By default, only downloads for the current platform are shown.
    #[arg(long)]
    pub all_platforms: bool,

    /// List Python downloads for all architectures.
    ///
    /// By default, only downloads for the current architecture are shown.
    #[arg(long, alias = "all_architectures")]
    pub all_arches: bool,

    /// Only show installed Python versions.
    ///
    /// By default, installed distributions and available downloads for the current platform are shown.
    #[arg(long, conflicts_with("only_downloads"))]
    pub only_installed: bool,

    /// Only show available Python downloads.
    ///
    /// By default, installed distributions and available downloads for the current platform are shown.
    #[arg(long, conflicts_with("only_installed"))]
    pub only_downloads: bool,

    /// Show the URLs of available Python downloads.
    ///
    /// By default, these display as `<download available>`.
    #[arg(long)]
    pub show_urls: bool,

    /// Select the output format.
    #[arg(long, value_enum, default_value_t = PythonListFormat::default())]
    pub output_format: PythonListFormat,

    /// URL pointing to JSON of custom Python installations.
    #[arg(long, value_hint = ValueHint::Other)]
    pub python_downloads_json_url: Option<String>,
}

#[derive(Args)]
pub struct PythonDirArgs {
    /// Show the directory into which `uv python` will install Python executables.
    ///
    /// Note that this directory is only used when installing Python with preview mode enabled.
    ///
    /// The Python executable directory is determined according to the XDG standard and is derived
    /// from the following environment variables, in order of preference:
    ///
    /// - `$UV_PYTHON_BIN_DIR`
    /// - `$XDG_BIN_HOME`
    /// - `$XDG_DATA_HOME/../bin`
    /// - `$HOME/.local/bin`
    #[arg(long, verbatim_doc_comment)]
    pub bin: bool,
}

#[derive(Args)]
pub struct PythonInstallCompileBytecodeArgs {
    /// Compile Python's standard library to bytecode after installation.
    ///
    /// By default, uv does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`);
    /// instead, compilation is performed lazily the first time a module is imported. For use-cases
    /// in which start time is important, such as CLI applications and Docker containers, this
    /// option can be enabled to trade longer installation times and some additional disk space for
    /// faster start times.
    ///
    /// When enabled, uv will process the Python version's `stdlib` directory. It will ignore any
    /// compilation errors.
    #[arg(
        long,
        alias = "compile",
        overrides_with("no_compile_bytecode"),
        env = EnvVars::UV_COMPILE_BYTECODE,
        value_parser = clap::builder::BoolishValueParser::new(),
    )]
    pub compile_bytecode: bool,

    #[arg(
        long,
        alias = "no-compile",
        overrides_with("compile_bytecode"),
        hide = true
    )]
    pub no_compile_bytecode: bool,
}

#[derive(Args)]
pub struct PythonInstallArgs {
    /// The directory to store the Python installation in.
    ///
    /// If provided, `UV_PYTHON_INSTALL_DIR` will need to be set for subsequent operations for uv to
    /// discover the Python installation.
    ///
    /// See `uv python dir` to view the current Python installation directory. Defaults to
    /// `~/.local/share/uv/python`.
    #[arg(long, short, env = EnvVars::UV_PYTHON_INSTALL_DIR, value_hint = ValueHint::DirPath)]
    pub install_dir: Option<PathBuf>,

    /// Install a Python executable into the `bin` directory.
    ///
    /// This is the default behavior. If this flag is provided explicitly, uv will error if the
    /// executable cannot be installed.
    ///
    /// This can also be set with `UV_PYTHON_INSTALL_BIN=1`.
    ///
    /// See `UV_PYTHON_BIN_DIR` to customize the target directory.
    #[arg(long, overrides_with("no_bin"), hide = true)]
    pub bin: bool,

    /// Do not install a Python executable into the `bin` directory.
    ///
    /// This can also be set with `UV_PYTHON_INSTALL_BIN=0`.
    #[arg(long, overrides_with("bin"), conflicts_with("default"))]
    pub no_bin: bool,

    /// Register the Python installation in the Windows registry.
    ///
    /// This is the default behavior on Windows. If this flag is provided explicitly, uv will error if the
    /// registry entry cannot be created.
    ///
    /// This can also be set with `UV_PYTHON_INSTALL_REGISTRY=1`.
    #[arg(long, overrides_with("no_registry"), hide = true)]
    pub registry: bool,

    /// Do not register the Python installation in the Windows registry.
    ///
    /// This can also be set with `UV_PYTHON_INSTALL_REGISTRY=0`.
    #[arg(long, overrides_with("registry"))]
    pub no_registry: bool,

    /// The Python version(s) to install.
    ///
    /// If not provided, the requested Python version(s) will be read from the `UV_PYTHON`
    /// environment variable then `.python-versions` or `.python-version` files. If none of the
    /// above are present, uv will check if it has installed any Python versions. If not, it will
    /// install the latest stable version of Python.
    ///
    /// See `uv help python` to view supported request formats.
    #[arg(env = EnvVars::UV_PYTHON)]
    pub targets: Vec<String>,

    /// Set the URL to use as the source for downloading Python installations.
    ///
    /// The provided URL will replace
    /// `https://github.com/astral-sh/python-build-standalone/releases/download` in, e.g.,
    /// `https://github.com/astral-sh/python-build-standalone/releases/download/20240713/cpython-3.12.4%2B20240713-aarch64-apple-darwin-install_only.tar.gz`.
    ///
    /// Distributions can be read from a local directory by using the `file://` URL scheme.
    #[arg(long, value_hint = ValueHint::Url)]
    pub mirror: Option<String>,

    /// Set the URL to use as the source for downloading PyPy installations.
    ///
    /// The provided URL will replace `https://downloads.python.org/pypy` in, e.g.,
    /// `https://downloads.python.org/pypy/pypy3.8-v7.3.7-osx64.tar.bz2`.
    ///
    /// Distributions can be read from a local directory by using the `file://` URL scheme.
    #[arg(long, value_hint = ValueHint::Url)]
    pub pypy_mirror: Option<String>,

    /// URL pointing to JSON of custom Python installations.
    #[arg(long, value_hint = ValueHint::Other)]
    pub python_downloads_json_url: Option<String>,

    /// Reinstall the requested Python version, if it's already installed.
    ///
    /// By default, uv will exit successfully if the version is already
    /// installed.
    #[arg(long, short)]
    pub reinstall: bool,

    /// Replace existing Python executables during installation.
    ///
    /// By default, uv will refuse to replace executables that it does not manage.
    ///
    /// Implies `--reinstall`.
    #[arg(long, short)]
    pub force: bool,

    /// Upgrade existing Python installations to the latest patch version.
    ///
    /// By default, uv will not upgrade already-installed Python versions to newer patch releases.
    /// With `--upgrade`, uv will upgrade to the latest available patch version for the specified
    /// minor version(s).
    ///
    /// If the requested versions are not yet installed, uv will install them.
    ///
    /// This option is only supported for minor version requests, e.g., `3.12`; uv will exit with an
    /// error if a patch version, e.g., `3.12.2`, is requested.
    #[arg(long, short = 'U')]
    pub upgrade: bool,

    /// Use as the default Python version.
    ///
    /// By default, only a `python{major}.{minor}` executable is installed, e.g., `python3.10`. When
    /// the `--default` flag is used, `python{major}`, e.g., `python3`, and `python` executables are
    /// also installed.
    ///
    /// Alternative Python variants will still include their tag. For example, installing
    /// 3.13+freethreaded with `--default` will include `python3t` and `pythont` instead of
    /// `python3` and `python`.
    ///
    /// If multiple Python versions are requested, uv will exit with an error.
    #[arg(long, conflicts_with("no_bin"))]
    pub default: bool,

    #[command(flatten)]
    pub compile_bytecode: PythonInstallCompileBytecodeArgs,
}

impl PythonInstallArgs {
    #[must_use]
    pub fn install_mirrors(&self) -> PythonInstallMirrors {
        PythonInstallMirrors {
            python_install_mirror: self.mirror.clone(),
            pypy_install_mirror: self.pypy_mirror.clone(),
            python_downloads_json_url: self.python_downloads_json_url.clone(),
        }
    }
}

#[derive(Args)]
pub struct PythonUpgradeArgs {
    /// The directory Python installations are stored in.
    ///
    /// If provided, `UV_PYTHON_INSTALL_DIR` will need to be set for subsequent operations for uv to
    /// discover the Python installation.
    ///
    /// See `uv python dir` to view the current Python installation directory. Defaults to
    /// `~/.local/share/uv/python`.
    #[arg(long, short, env = EnvVars::UV_PYTHON_INSTALL_DIR, value_hint = ValueHint::DirPath)]
    pub install_dir: Option<PathBuf>,

    /// The Python minor version(s) to upgrade.
    ///
    /// If no target version is provided, then uv will upgrade all managed CPython versions.
    #[arg(env = EnvVars::UV_PYTHON)]
    pub targets: Vec<String>,

    /// Set the URL to use as the source for downloading Python installations.
    ///
    /// The provided URL will replace
    /// `https://github.com/astral-sh/python-build-standalone/releases/download` in, e.g.,
    /// `https://github.com/astral-sh/python-build-standalone/releases/download/20240713/cpython-3.12.4%2B20240713-aarch64-apple-darwin-install_only.tar.gz`.
    ///
    /// Distributions can be read from a local directory by using the `file://` URL scheme.
    #[arg(long, value_hint = ValueHint::Url)]
    pub mirror: Option<String>,

    /// Set the URL to use as the source for downloading PyPy installations.
    ///
    /// The provided URL will replace `https://downloads.python.org/pypy` in, e.g.,
    /// `https://downloads.python.org/pypy/pypy3.8-v7.3.7-osx64.tar.bz2`.
    ///
    /// Distributions can be read from a local directory by using the `file://` URL scheme.
    #[arg(long, value_hint = ValueHint::Url)]
    pub pypy_mirror: Option<String>,

    /// Reinstall the latest Python patch, if it's already installed.
    ///
    /// By default, uv will exit successfully if the latest patch is already
    /// installed.
    #[arg(long, short)]
    pub reinstall: bool,

    /// URL pointing to JSON of custom Python installations.
    #[arg(long, value_hint = ValueHint::Other)]
    pub python_downloads_json_url: Option<String>,

    #[command(flatten)]
    pub compile_bytecode: PythonInstallCompileBytecodeArgs,
}

impl PythonUpgradeArgs {
    #[must_use]
    pub fn install_mirrors(&self) -> PythonInstallMirrors {
        PythonInstallMirrors {
            python_install_mirror: self.mirror.clone(),
            pypy_install_mirror: self.pypy_mirror.clone(),
            python_downloads_json_url: self.python_downloads_json_url.clone(),
        }
    }
}

#[derive(Args)]
pub struct PythonUninstallArgs {
    /// The directory where the Python was installed.
    #[arg(long, short, env = EnvVars::UV_PYTHON_INSTALL_DIR, value_hint = ValueHint::DirPath)]
    pub install_dir: Option<PathBuf>,

    /// The Python version(s) to uninstall.
    ///
    /// See `uv help python` to view supported request formats.
    #[arg(required = true)]
    pub targets: Vec<String>,

    /// Uninstall all managed Python versions.
    #[arg(long, conflicts_with("targets"))]
    pub all: bool,
}

#[derive(Args)]
pub struct PythonFindArgs {
    /// The Python request.
    ///
    /// See `uv help python` to view supported request formats.
    pub request: Option<String>,

    /// Avoid discovering a project or workspace.
    ///
    /// Otherwise, when no request is provided, the Python requirement of a project in the current
    /// directory or parent directories will be used.
    #[arg(long, alias = "no_workspace")]
    pub no_project: bool,

    /// Only find system Python interpreters.
    ///
    /// By default, uv will report the first Python interpreter it would use, including those in an
    /// active virtual environment or a virtual environment in the current working directory or any
    /// parent directory.
    ///
    /// The `--system` option instructs uv to skip virtual environment Python interpreters and
    /// restrict its search to the system path.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// Find the environment for a Python script, rather than the current project.
    #[arg(
        long,
        conflicts_with = "request",
        conflicts_with = "no_project",
        conflicts_with = "system",
        conflicts_with = "no_system",
        value_hint = ValueHint::FilePath,
    )]
    pub script: Option<PathBuf>,

    /// Show the Python version that would be used instead of the path to the interpreter.
    #[arg(long)]
    pub show_version: bool,

    /// Resolve symlinks in the output path.
    ///
    /// When enabled, the output path will be canonicalized, resolving any symlinks.
    #[arg(long)]
    pub resolve_links: bool,

    /// URL pointing to JSON of custom Python installations.
    #[arg(long, value_hint = ValueHint::Other)]
    pub python_downloads_json_url: Option<String>,
}

#[derive(Args)]
pub struct PythonPinArgs {
    /// The Python version request.
    ///
    /// uv supports more formats than other tools that read `.python-version` files, i.e., `pyenv`.
    /// If compatibility with those tools is needed, only use version numbers instead of complex
    /// requests such as `cpython@3.10`.
    ///
    /// If no request is provided, the currently pinned version will be shown.
    ///
    /// See `uv help python` to view supported request formats.
    pub request: Option<String>,

    /// Write the resolved Python interpreter path instead of the request.
    ///
    /// Ensures that the exact same interpreter is used.
    ///
    /// This option is usually not safe to use when committing the `.python-version` file to version
    /// control.
    #[arg(long, overrides_with("resolved"))]
    pub resolved: bool,

    #[arg(long, overrides_with("no_resolved"), hide = true)]
    pub no_resolved: bool,

    /// Avoid validating the Python pin is compatible with the project or workspace.
    ///
    /// By default, a project or workspace is discovered in the current directory or any parent
    /// directory. If a workspace is found, the Python pin is validated against the workspace's
    /// `requires-python` constraint.
    #[arg(long, alias = "no-workspace")]
    pub no_project: bool,

    /// Update the global Python version pin.
    ///
    /// Writes the pinned Python version to a `.python-version` file in the uv user configuration
    /// directory: `XDG_CONFIG_HOME/uv` on Linux/macOS and `%APPDATA%/uv` on Windows.
    ///
    /// When a local Python version pin is not found in the working directory or an ancestor
    /// directory, this version will be used instead.
    #[arg(long)]
    pub global: bool,

    /// Remove the Python version pin.
    #[arg(long, conflicts_with = "request", conflicts_with = "resolved")]
    pub rm: bool,
}
