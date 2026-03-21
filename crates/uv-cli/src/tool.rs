use super::*;

#[derive(Args)]
pub struct ToolNamespace {
    #[command(subcommand)]
    pub command: ToolCommand,
}

#[derive(Subcommand)]
pub enum ToolCommand {
    /// Run a command provided by a Python package.
    ///
    /// By default, the package to install is assumed to match the command name.
    ///
    /// The name of the command can include an exact version in the format `<package>@<version>`,
    /// e.g., `uv tool run ruff@0.3.0`. If more complex version specification is desired or if the
    /// command is provided by a different package, use `--from`.
    ///
    /// `uvx` can be used to invoke Python, e.g., with `uvx python` or `uvx python@<version>`. A
    /// Python interpreter will be started in an isolated virtual environment.
    ///
    /// If the tool was previously installed, i.e., via `uv tool install`, the installed version
    /// will be used unless a version is requested or the `--isolated` flag is used.
    ///
    /// `uvx` is provided as a convenient alias for `uv tool run`, their behavior is identical.
    ///
    /// If no command is provided, the installed tools are displayed.
    ///
    /// Packages are installed into an ephemeral virtual environment in the uv cache directory.
    #[command(
        after_help = "Use `uvx` as a shortcut for `uv tool run`.\n\n\
        Use `uv help tool run` for more details.",
        after_long_help = ""
    )]
    Run(ToolRunArgs),
    /// Hidden alias for `uv tool run` for the `uvx` command
    #[command(
        hide = true,
        override_usage = "uvx [OPTIONS] [COMMAND]",
        about = "Run a command provided by a Python package.",
        after_help = "Use `uv help tool run` for more details.",
        after_long_help = "",
        display_name = "uvx",
        long_version = crate::version::uv_self_version()
    )]
    Uvx(UvxArgs),
    /// Install commands provided by a Python package.
    ///
    /// Packages are installed into an isolated virtual environment in the uv tools directory. The
    /// executables are linked the tool executable directory, which is determined according to the
    /// XDG standard and can be retrieved with `uv tool dir --bin`.
    ///
    /// If the tool was previously installed, the existing tool will generally be replaced.
    Install(ToolInstallArgs),
    /// Upgrade installed tools.
    ///
    /// If a tool was installed with version constraints, they will be respected on upgrade — to
    /// upgrade a tool beyond the originally provided constraints, use `uv tool install` again.
    ///
    /// If a tool was installed with specific settings, they will be respected on upgraded. For
    /// example, if `--prereleases allow` was provided during installation, it will continue to be
    /// respected in upgrades.
    #[command(alias = "update")]
    Upgrade(ToolUpgradeArgs),
    /// List installed tools.
    #[command(alias = "ls")]
    List(ToolListArgs),
    /// Uninstall a tool.
    Uninstall(ToolUninstallArgs),
    /// Ensure that the tool executable directory is on the `PATH`.
    ///
    /// If the tool executable directory is not present on the `PATH`, uv will attempt to add it to
    /// the relevant shell configuration files.
    ///
    /// If the shell configuration files already include a blurb to add the executable directory to
    /// the path, but the directory is not present on the `PATH`, uv will exit with an error.
    ///
    /// The tool executable directory is determined according to the XDG standard and can be
    /// retrieved with `uv tool dir --bin`.
    #[command(alias = "ensurepath")]
    UpdateShell,
    /// Show the path to the uv tools directory.
    ///
    /// The tools directory is used to store environments and metadata for installed tools.
    ///
    /// By default, tools are stored in the uv data directory at `$XDG_DATA_HOME/uv/tools` or
    /// `$HOME/.local/share/uv/tools` on Unix and `%APPDATA%\uv\data\tools` on Windows.
    ///
    /// The tool installation directory may be overridden with `$UV_TOOL_DIR`.
    ///
    /// To instead view the directory uv installs executables into, use the `--bin` flag.
    Dir(ToolDirArgs),
}

#[derive(Args)]
pub struct ToolRunArgs {
    /// The command to run.
    ///
    /// WARNING: The documentation for [`Self::command`] is not included in help output
    #[command(subcommand)]
    pub command: Option<ExternalCommand>,

    /// Use the given package to provide the command.
    ///
    /// By default, the package name is assumed to match the command name.
    #[arg(long, value_hint = ValueHint::Other)]
    pub from: Option<String>,

    /// Run with the given packages installed.
    #[arg(short = 'w', long, value_hint = ValueHint::Other)]
    pub with: Vec<comma::CommaSeparatedRequirements>,

    /// Run with the given packages installed in editable mode
    ///
    /// When used in a project, these dependencies will be layered on top of the uv tool's
    /// environment in a separate, ephemeral environment. These dependencies are allowed to conflict
    /// with those specified.
    #[arg(long, value_hint = ValueHint::DirPath)]
    pub with_editable: Vec<comma::CommaSeparatedRequirements>,

    /// Run with the packages listed in the given files.
    ///
    /// The following formats are supported: `requirements.txt`, `.py` files with inline metadata,
    /// and `pylock.toml`.
    #[arg(
        long,
        value_delimiter = ',',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub with_requirements: Vec<Maybe<PathBuf>>,

    /// Constrain versions using the given requirements files.
    ///
    /// Constraints files are `requirements.txt`-like files that only control the _version_ of a
    /// requirement that's installed. However, including a package in a constraints file will _not_
    /// trigger the installation of that package.
    ///
    /// This is equivalent to pip's `--constraint` option.
    #[arg(
        long,
        short,
        alias = "constraint",
        env = EnvVars::UV_CONSTRAINT,
        value_delimiter = ' ',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub constraints: Vec<Maybe<PathBuf>>,

    /// Constrain build dependencies using the given requirements files when building source
    /// distributions.
    ///
    /// Constraints files are `requirements.txt`-like files that only control the _version_ of a
    /// requirement that's installed. However, including a package in a constraints file will _not_
    /// trigger the installation of that package.
    #[arg(
        long,
        short,
        alias = "build-constraint",
        env = EnvVars::UV_BUILD_CONSTRAINT,
        value_delimiter = ' ',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub build_constraints: Vec<Maybe<PathBuf>>,

    /// Override versions using the given requirements files.
    ///
    /// Overrides files are `requirements.txt`-like files that force a specific version of a
    /// requirement to be installed, regardless of the requirements declared by any constituent
    /// package, and regardless of whether this would be considered an invalid resolution.
    ///
    /// While constraints are _additive_, in that they're combined with the requirements of the
    /// constituent packages, overrides are _absolute_, in that they completely replace the
    /// requirements of the constituent packages.
    #[arg(
        long,
        alias = "override",
        env = EnvVars::UV_OVERRIDE,
        value_delimiter = ' ',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub overrides: Vec<Maybe<PathBuf>>,

    /// Run the tool in an isolated virtual environment, ignoring any already-installed tools [env:
    /// UV_ISOLATED=]
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub isolated: bool,

    /// Load environment variables from a `.env` file.
    ///
    /// Can be provided multiple times, with subsequent files overriding values defined in previous
    /// files.
    #[arg(long, value_delimiter = ' ', env = EnvVars::UV_ENV_FILE, value_hint = ValueHint::FilePath)]
    pub env_file: Vec<PathBuf>,

    /// Avoid reading environment variables from a `.env` file [env: UV_NO_ENV_FILE=]
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub no_env_file: bool,

    #[command(flatten)]
    pub installer: ResolverInstallerArgs,

    #[command(flatten)]
    pub build: BuildOptionsArgs,

    #[command(flatten)]
    pub refresh: RefreshArgs,

    /// Whether to use Git LFS when adding a dependency from Git.
    #[arg(long)]
    pub lfs: bool,

    /// The Python interpreter to use to build the run environment.
    ///
    /// See `uv help python` for details on Python discovery and supported request formats.
    #[arg(
        long,
        short,
        env = EnvVars::UV_PYTHON,
        verbatim_doc_comment,
        help_heading = "Python options",
        value_parser = parse_maybe_string,
        value_hint = ValueHint::Other,
    )]
    pub python: Option<Maybe<String>>,

    /// Whether to show resolver and installer output from any environment modifications [env:
    /// UV_SHOW_RESOLUTION=]
    ///
    /// By default, environment modifications are omitted, but enabled under `--verbose`.
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new(), hide = true)]
    pub show_resolution: bool,

    /// The platform for which requirements should be installed.
    ///
    /// Represented as a "target triple", a string that describes the target platform in terms of
    /// its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or
    /// `aarch64-apple-darwin`.
    ///
    /// When targeting macOS (Darwin), the default minimum version is `13.0`. Use
    /// `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.
    ///
    /// When targeting iOS, the default minimum version is `13.0`. Use
    /// `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.
    ///
    /// When targeting Android, the default minimum Android API level is `24`. Use
    /// `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.
    ///
    /// WARNING: When specified, uv will select wheels that are compatible with the _target_
    /// platform; as a result, the installed distributions may not be compatible with the _current_
    /// platform. Conversely, any distributions that are built from source may be incompatible with
    /// the _target_ platform, as they will be built for the _current_ platform. The
    /// `--python-platform` option is intended for advanced use cases.
    #[arg(long)]
    pub python_platform: Option<TargetTriple>,

    /// The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`)
    ///
    /// When set, uv will ignore the configured index URLs for packages in the PyTorch ecosystem,
    /// and will instead use the defined backend.
    ///
    /// For example, when set to `cpu`, uv will use the CPU-only PyTorch index; when set to `cu126`,
    /// uv will use the PyTorch index for CUDA 12.6.
    ///
    /// The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently
    /// installed CUDA drivers.
    ///
    /// This option is in preview and may change in any future release.
    #[arg(long, value_enum, env = EnvVars::UV_TORCH_BACKEND)]
    pub torch_backend: Option<TorchMode>,

    #[arg(long, hide = true)]
    pub generate_shell_completion: Option<clap_complete_command::Shell>,
}

#[derive(Args)]
pub struct UvxArgs {
    #[command(flatten)]
    pub tool_run: ToolRunArgs,

    /// Display the uvx version.
    #[arg(short = 'V', long, action = clap::ArgAction::Version)]
    pub version: Option<bool>,
}

#[derive(Args)]
pub struct ToolInstallArgs {
    /// The package to install commands from.
    #[arg(value_hint = ValueHint::Other)]
    pub package: String,

    /// The package to install commands from.
    ///
    /// This option is provided for parity with `uv tool run`, but is redundant with `package`.
    #[arg(long, hide = true, value_hint = ValueHint::Other)]
    pub from: Option<String>,

    /// Include the following additional requirements.
    #[arg(short = 'w', long, value_hint = ValueHint::Other)]
    pub with: Vec<comma::CommaSeparatedRequirements>,

    /// Run with the packages listed in the given files.
    ///
    /// The following formats are supported: `requirements.txt`, `.py` files with inline metadata,
    /// and `pylock.toml`.
    #[arg(long, value_delimiter = ',', value_parser = parse_maybe_file_path, value_hint = ValueHint::FilePath)]
    pub with_requirements: Vec<Maybe<PathBuf>>,

    /// Install the target package in editable mode, such that changes in the package's source
    /// directory are reflected without reinstallation.
    #[arg(short, long)]
    pub editable: bool,

    /// Include the given packages in editable mode.
    #[arg(long, value_hint = ValueHint::DirPath)]
    pub with_editable: Vec<comma::CommaSeparatedRequirements>,

    /// Install executables from the following packages.
    #[arg(long, value_hint = ValueHint::Other)]
    pub with_executables_from: Vec<comma::CommaSeparatedRequirements>,

    /// Constrain versions using the given requirements files.
    ///
    /// Constraints files are `requirements.txt`-like files that only control the _version_ of a
    /// requirement that's installed. However, including a package in a constraints file will _not_
    /// trigger the installation of that package.
    ///
    /// This is equivalent to pip's `--constraint` option.
    #[arg(
        long,
        short,
        alias = "constraint",
        env = EnvVars::UV_CONSTRAINT,
        value_delimiter = ' ',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub constraints: Vec<Maybe<PathBuf>>,

    /// Override versions using the given requirements files.
    ///
    /// Overrides files are `requirements.txt`-like files that force a specific version of a
    /// requirement to be installed, regardless of the requirements declared by any constituent
    /// package, and regardless of whether this would be considered an invalid resolution.
    ///
    /// While constraints are _additive_, in that they're combined with the requirements of the
    /// constituent packages, overrides are _absolute_, in that they completely replace the
    /// requirements of the constituent packages.
    #[arg(
        long,
        alias = "override",
        env = EnvVars::UV_OVERRIDE,
        value_delimiter = ' ',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub overrides: Vec<Maybe<PathBuf>>,

    /// Exclude packages from resolution using the given requirements files.
    ///
    /// Excludes files are `requirements.txt`-like files that specify packages to exclude
    /// from the resolution. When a package is excluded, it will be omitted from the
    /// dependency list entirely and its own dependencies will be ignored during the resolution
    /// phase. Excludes are unconditional in that requirement specifiers and markers are ignored;
    /// any package listed in the provided file will be omitted from all resolved environments.
    #[arg(
        long,
        alias = "exclude",
        env = EnvVars::UV_EXCLUDE,
        value_delimiter = ' ',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub excludes: Vec<Maybe<PathBuf>>,

    /// Constrain build dependencies using the given requirements files when building source
    /// distributions.
    ///
    /// Constraints files are `requirements.txt`-like files that only control the _version_ of a
    /// requirement that's installed. However, including a package in a constraints file will _not_
    /// trigger the installation of that package.
    #[arg(
        long,
        short,
        alias = "build-constraint",
        env = EnvVars::UV_BUILD_CONSTRAINT,
        value_delimiter = ' ',
        value_parser = parse_maybe_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub build_constraints: Vec<Maybe<PathBuf>>,

    #[command(flatten)]
    pub installer: ResolverInstallerArgs,

    #[command(flatten)]
    pub build: BuildOptionsArgs,

    #[command(flatten)]
    pub refresh: RefreshArgs,

    /// Force installation of the tool.
    ///
    /// Will recreate any existing environment for the tool and replace any existing entry points
    /// with the same name in the executable directory.
    #[arg(long)]
    pub force: bool,

    /// Whether to use Git LFS when adding a dependency from Git.
    #[arg(long)]
    pub lfs: bool,

    /// The Python interpreter to use to build the tool environment.
    ///
    /// See `uv help python` for details on Python discovery and supported request formats.
    #[arg(
        long,
        short,
        env = EnvVars::UV_PYTHON,
        verbatim_doc_comment,
        help_heading = "Python options",
        value_parser = parse_maybe_string,
        value_hint = ValueHint::Other,
    )]
    pub python: Option<Maybe<String>>,

    /// The platform for which requirements should be installed.
    ///
    /// Represented as a "target triple", a string that describes the target platform in terms of
    /// its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or
    /// `aarch64-apple-darwin`.
    ///
    /// When targeting macOS (Darwin), the default minimum version is `13.0`. Use
    /// `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.
    ///
    /// When targeting iOS, the default minimum version is `13.0`. Use
    /// `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.
    ///
    /// When targeting Android, the default minimum Android API level is `24`. Use
    /// `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.
    ///
    /// WARNING: When specified, uv will select wheels that are compatible with the _target_
    /// platform; as a result, the installed distributions may not be compatible with the _current_
    /// platform. Conversely, any distributions that are built from source may be incompatible with
    /// the _target_ platform, as they will be built for the _current_ platform. The
    /// `--python-platform` option is intended for advanced use cases.
    #[arg(long)]
    pub python_platform: Option<TargetTriple>,

    /// The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`)
    ///
    /// When set, uv will ignore the configured index URLs for packages in the PyTorch ecosystem,
    /// and will instead use the defined backend.
    ///
    /// For example, when set to `cpu`, uv will use the CPU-only PyTorch index; when set to `cu126`,
    /// uv will use the PyTorch index for CUDA 12.6.
    ///
    /// The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently
    /// installed CUDA drivers.
    ///
    /// This option is in preview and may change in any future release.
    #[arg(long, value_enum, env = EnvVars::UV_TORCH_BACKEND)]
    pub torch_backend: Option<TorchMode>,
}

#[derive(Args)]
pub struct ToolListArgs {
    /// Whether to display the path to each tool environment and installed executable.
    #[arg(long)]
    pub show_paths: bool,

    /// Whether to display the version specifier(s) used to install each tool.
    #[arg(long)]
    pub show_version_specifiers: bool,

    /// Whether to display the additional requirements installed with each tool.
    #[arg(long)]
    pub show_with: bool,

    /// Whether to display the extra requirements installed with each tool.
    #[arg(long)]
    pub show_extras: bool,

    /// Whether to display the Python version associated with each tool.
    #[arg(long)]
    pub show_python: bool,

    /// List outdated tools.
    ///
    /// The latest version of each tool will be shown alongside the installed version. Up-to-date
    /// tools will be omitted from the output.
    #[arg(long, overrides_with("no_outdated"))]
    pub outdated: bool,

    #[arg(long, overrides_with("outdated"), hide = true)]
    pub no_outdated: bool,

    // Hide unused global Python options.
    #[arg(long, hide = true)]
    pub python_preference: Option<PythonPreference>,

    #[arg(long, hide = true)]
    pub no_python_downloads: bool,
}

#[derive(Args)]
pub struct ToolDirArgs {
    /// Show the directory into which `uv tool` will install executables.
    ///
    /// By default, `uv tool dir` shows the directory into which the tool Python environments
    /// themselves are installed, rather than the directory containing the linked executables.
    ///
    /// The tool executable directory is determined according to the XDG standard and is derived
    /// from the following environment variables, in order of preference:
    ///
    /// - `$UV_TOOL_BIN_DIR`
    /// - `$XDG_BIN_HOME`
    /// - `$XDG_DATA_HOME/../bin`
    /// - `$HOME/.local/bin`
    #[arg(long, verbatim_doc_comment)]
    pub bin: bool,
}

#[derive(Args)]
pub struct ToolUninstallArgs {
    /// The name of the tool to uninstall.
    #[arg(required = true, value_hint = ValueHint::Other)]
    pub name: Vec<PackageName>,

    /// Uninstall all tools.
    #[arg(long, conflicts_with("name"))]
    pub all: bool,
}

#[derive(Args)]
pub struct ToolUpgradeArgs {
    /// The name of the tool to upgrade, along with an optional version specifier.
    #[arg(required = true, value_hint = ValueHint::Other)]
    pub name: Vec<String>,

    /// Upgrade all tools.
    #[arg(long, conflicts_with("name"))]
    pub all: bool,

    /// Upgrade a tool, and specify it to use the given Python interpreter to build its environment.
    /// Use with `--all` to apply to all tools.
    ///
    /// See `uv help python` for details on Python discovery and supported request formats.
    #[arg(
        long,
        short,
        env = EnvVars::UV_PYTHON,
        verbatim_doc_comment,
        help_heading = "Python options",
        value_parser = parse_maybe_string,
        value_hint = ValueHint::Other,
    )]
    pub python: Option<Maybe<String>>,

    /// The platform for which requirements should be installed.
    ///
    /// Represented as a "target triple", a string that describes the target platform in terms of
    /// its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or
    /// `aarch64-apple-darwin`.
    ///
    /// When targeting macOS (Darwin), the default minimum version is `13.0`. Use
    /// `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.
    ///
    /// When targeting iOS, the default minimum version is `13.0`. Use
    /// `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.
    ///
    /// When targeting Android, the default minimum Android API level is `24`. Use
    /// `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.
    ///
    /// WARNING: When specified, uv will select wheels that are compatible with the _target_
    /// platform; as a result, the installed distributions may not be compatible with the _current_
    /// platform. Conversely, any distributions that are built from source may be incompatible with
    /// the _target_ platform, as they will be built for the _current_ platform. The
    /// `--python-platform` option is intended for advanced use cases.
    #[arg(long)]
    pub python_platform: Option<TargetTriple>,

    // The following is equivalent to flattening `ResolverInstallerArgs`, with the `--upgrade`, and
    // `--upgrade-package` options hidden, and the `--no-upgrade` option removed.
    /// Allow package upgrades, ignoring pinned versions in any existing output file. Implies
    /// `--refresh`.
    #[arg(hide = true, long, short = 'U', help_heading = "Resolver options")]
    pub upgrade: bool,

    /// Allow upgrades for a specific package, ignoring pinned versions in any existing output
    /// file. Implies `--refresh-package`.
    #[arg(hide = true, long, short = 'P', help_heading = "Resolver options")]
    pub upgrade_package: Vec<Requirement<VerbatimParsedUrl>>,

    #[command(flatten)]
    pub index_args: IndexArgs,

    /// Reinstall all packages, regardless of whether they're already installed. Implies
    /// `--refresh`.
    #[arg(
        long,
        alias = "force-reinstall",
        overrides_with("no_reinstall"),
        help_heading = "Installer options"
    )]
    pub reinstall: bool,

    #[arg(
        long,
        overrides_with("reinstall"),
        hide = true,
        help_heading = "Installer options"
    )]
    pub no_reinstall: bool,

    /// Reinstall a specific package, regardless of whether it's already installed. Implies
    /// `--refresh-package`.
    #[arg(long, help_heading = "Installer options", value_hint = ValueHint::Other)]
    pub reinstall_package: Vec<PackageName>,

    /// The strategy to use when resolving against multiple index URLs.
    ///
    /// By default, uv will stop at the first index on which a given package is available, and limit
    /// resolutions to those present on that first index (`first-index`). This prevents "dependency
    /// confusion" attacks, whereby an attacker can upload a malicious package under the same name
    /// to an alternate index.
    #[arg(
        long,
        value_enum,
        env = EnvVars::UV_INDEX_STRATEGY,
        help_heading = "Index options"
    )]
    pub index_strategy: Option<IndexStrategy>,

    /// Attempt to use `keyring` for authentication for index URLs.
    ///
    /// At present, only `--keyring-provider subprocess` is supported, which configures uv to use
    /// the `keyring` CLI to handle authentication.
    ///
    /// Defaults to `disabled`.
    #[arg(
        long,
        value_enum,
        env = EnvVars::UV_KEYRING_PROVIDER,
        help_heading = "Index options"
    )]
    pub keyring_provider: Option<KeyringProviderType>,

    /// The strategy to use when selecting between the different compatible versions for a given
    /// package requirement.
    ///
    /// By default, uv will use the latest compatible version of each package (`highest`).
    #[arg(
        long,
        value_enum,
        env = EnvVars::UV_RESOLUTION,
        help_heading = "Resolver options"
    )]
    pub resolution: Option<ResolutionMode>,

    /// The strategy to use when considering pre-release versions.
    ///
    /// By default, uv will accept pre-releases for packages that _only_ publish pre-releases, along
    /// with first-party requirements that contain an explicit pre-release marker in the declared
    /// specifiers (`if-necessary-or-explicit`).
    #[arg(
        long,
        value_enum,
        env = EnvVars::UV_PRERELEASE,
        help_heading = "Resolver options"
    )]
    pub prerelease: Option<PrereleaseMode>,

    #[arg(long, hide = true)]
    pub pre: bool,

    /// The strategy to use when selecting multiple versions of a given package across Python
    /// versions and platforms.
    ///
    /// By default, uv will optimize for selecting the latest version of each package for each
    /// supported Python version (`requires-python`), while minimizing the number of selected
    /// versions across platforms.
    ///
    /// Under `fewest`, uv will minimize the number of selected versions for each package,
    /// preferring older versions that are compatible with a wider range of supported Python
    /// versions or platforms.
    #[arg(
        long,
        value_enum,
        env = EnvVars::UV_FORK_STRATEGY,
        help_heading = "Resolver options"
    )]
    pub fork_strategy: Option<ForkStrategy>,

    /// Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs.
    #[arg(
        long,
        short = 'C',
        alias = "config-settings",
        help_heading = "Build options"
    )]
    pub config_setting: Option<Vec<ConfigSettingEntry>>,

    /// Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs.
    #[arg(
        long,
        alias = "config-settings-package",
        help_heading = "Build options"
    )]
    pub config_setting_package: Option<Vec<ConfigSettingPackageEntry>>,

    /// Disable isolation when building source distributions.
    ///
    /// Assumes that build dependencies specified by PEP 518 are already installed.
    #[arg(
        long,
        overrides_with("build_isolation"),
        help_heading = "Build options",
        env = EnvVars::UV_NO_BUILD_ISOLATION,
        value_parser = clap::builder::BoolishValueParser::new(),
    )]
    pub no_build_isolation: bool,

    /// Disable isolation when building source distributions for a specific package.
    ///
    /// Assumes that the packages' build dependencies specified by PEP 518 are already installed.
    #[arg(long, help_heading = "Build options", value_hint = ValueHint::Other)]
    pub no_build_isolation_package: Vec<PackageName>,

    #[arg(
        long,
        overrides_with("no_build_isolation"),
        hide = true,
        help_heading = "Build options"
    )]
    pub build_isolation: bool,

    /// Limit candidate packages to those that were uploaded prior to the given date.
    ///
    /// Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format
    /// (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly"
    /// duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`,
    /// `P7D`, `P30D`).
    ///
    /// Durations do not respect semantics of the local time zone and are always resolved to a fixed
    /// number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored).
    /// Calendar units such as months and years are not allowed.
    #[arg(long, env = EnvVars::UV_EXCLUDE_NEWER, help_heading = "Resolver options")]
    pub exclude_newer: Option<ExcludeNewerValue>,

    /// Limit candidate packages for specific packages to those that were uploaded prior to the
    /// given date.
    ///
    /// Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339
    /// timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g.,
    /// `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration
    /// (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`,
    /// `P30D`).
    ///
    /// Durations do not respect semantics of the local time zone and are always resolved to a fixed
    /// number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored).
    /// Calendar units such as months and years are not allowed.
    ///
    /// Can be provided multiple times for different packages.
    #[arg(long, help_heading = "Resolver options")]
    pub exclude_newer_package: Option<Vec<ExcludeNewerPackageEntry>>,

    /// The method to use when installing packages from the global cache.
    ///
    /// Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on
    /// Windows.
    ///
    /// WARNING: The use of symlink link mode is discouraged, as they create tight coupling between
    /// the cache and the target environment. For example, clearing the cache (`uv cache clean`)
    /// will break all installed packages by way of removing the underlying source files. Use
    /// symlinks with caution.
    #[arg(
        long,
        value_enum,
        env = EnvVars::UV_LINK_MODE,
        help_heading = "Installer options"
    )]
    pub link_mode: Option<uv_install_wheel::LinkMode>,

    /// Compile Python files to bytecode after installation.
    ///
    /// By default, uv does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`);
    /// instead, compilation is performed lazily the first time a module is imported. For use-cases
    /// in which start time is critical, such as CLI applications and Docker containers, this option
    /// can be enabled to trade longer installation times for faster start times.
    ///
    /// When enabled, uv will process the entire site-packages directory (including packages that
    /// are not being modified by the current operation) for consistency. Like pip, it will also
    /// ignore errors.
    #[arg(
        long,
        alias = "compile",
        overrides_with("no_compile_bytecode"),
        help_heading = "Installer options",
        env = EnvVars::UV_COMPILE_BYTECODE,
        value_parser = clap::builder::BoolishValueParser::new(),
    )]
    pub compile_bytecode: bool,

    #[arg(
        long,
        alias = "no-compile",
        overrides_with("compile_bytecode"),
        hide = true,
        help_heading = "Installer options"
    )]
    pub no_compile_bytecode: bool,

    /// Ignore the `tool.uv.sources` table when resolving dependencies. Used to lock against the
    /// standards-compliant, publishable package metadata, as opposed to using any workspace, Git,
    /// URL, or local path sources.
    #[arg(
        long,
        env = EnvVars::UV_NO_SOURCES,
        value_parser = clap::builder::BoolishValueParser::new(),
        help_heading = "Resolver options",
    )]
    pub no_sources: bool,

    /// Don't use sources from the `tool.uv.sources` table for the specified packages.
    #[arg(long, help_heading = "Resolver options", env = EnvVars::UV_NO_SOURCES_PACKAGE, value_delimiter = ' ')]
    pub no_sources_package: Vec<PackageName>,

    #[command(flatten)]
    pub build: BuildOptionsArgs,
}
