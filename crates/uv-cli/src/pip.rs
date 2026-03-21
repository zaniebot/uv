use super::*;

#[derive(Args)]
pub struct PipNamespace {
    #[command(subcommand)]
    pub command: PipCommand,
}

#[derive(Subcommand)]
pub enum PipCommand {
    /// Compile a `requirements.in` file to a `requirements.txt` or `pylock.toml` file.
    #[command(
        after_help = "Use `uv help pip compile` for more details.",
        after_long_help = ""
    )]
    Compile(PipCompileArgs),
    /// Sync an environment with a `requirements.txt` or `pylock.toml` file.
    ///
    /// When syncing an environment, any packages not listed in the `requirements.txt` or
    /// `pylock.toml` file will be removed. To retain extraneous packages, use `uv pip install`
    /// instead.
    ///
    /// The input file is presumed to be the output of a `pip compile` or `uv export` operation,
    /// in which it will include all transitive dependencies. If transitive dependencies are not
    /// present in the file, they will not be installed. Use `--strict` to warn if any transitive
    /// dependencies are missing.
    #[command(
        after_help = "Use `uv help pip sync` for more details.",
        after_long_help = ""
    )]
    Sync(Box<PipSyncArgs>),
    /// Install packages into an environment.
    #[command(
        after_help = "Use `uv help pip install` for more details.",
        after_long_help = ""
    )]
    Install(PipInstallArgs),
    /// Uninstall packages from an environment.
    #[command(
        after_help = "Use `uv help pip uninstall` for more details.",
        after_long_help = ""
    )]
    Uninstall(PipUninstallArgs),
    /// List, in requirements format, packages installed in an environment.
    #[command(
        after_help = "Use `uv help pip freeze` for more details.",
        after_long_help = ""
    )]
    Freeze(PipFreezeArgs),
    /// List, in tabular format, packages installed in an environment.
    #[command(
        after_help = "Use `uv help pip list` for more details.",
        after_long_help = "",
        alias = "ls"
    )]
    List(PipListArgs),
    /// Show information about one or more installed packages.
    #[command(
        after_help = "Use `uv help pip show` for more details.",
        after_long_help = ""
    )]
    Show(PipShowArgs),
    /// Display the dependency tree for an environment.
    #[command(
        after_help = "Use `uv help pip tree` for more details.",
        after_long_help = ""
    )]
    Tree(PipTreeArgs),
    /// Verify installed packages have compatible dependencies.
    #[command(
        after_help = "Use `uv help pip check` for more details.",
        after_long_help = ""
    )]
    Check(PipCheckArgs),
    /// Display debug information (unsupported)
    #[command(hide = true)]
    Debug(PipDebugArgs),
}

#[derive(Args)]
#[command(group = clap::ArgGroup::new("sources").required(true).multiple(true))]
pub struct PipCompileArgs {
    /// Include the packages listed in the given files.
    ///
    /// The following formats are supported: `requirements.txt`, `.py` files with inline metadata,
    /// `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.
    ///
    /// If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, uv will extract the
    /// requirements for the relevant project.
    ///
    /// If `-` is provided, then requirements will be read from stdin.
    ///
    /// The order of the requirements files and the requirements in them is used to determine
    /// priority during resolution.
    #[arg(group = "sources", value_parser = parse_file_path, value_hint = ValueHint::FilePath)]
    pub src_file: Vec<PathBuf>,

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

    /// Include optional dependencies from the specified extra name; may be provided more than once.
    ///
    /// Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
    #[arg(long, value_delimiter = ',', conflicts_with = "all_extras", value_parser = extra_name_with_clap_error)]
    pub extra: Option<Vec<ExtraName>>,

    /// Include all optional dependencies.
    ///
    /// Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
    #[arg(long, conflicts_with = "extra")]
    pub all_extras: bool,

    #[arg(long, overrides_with("all_extras"), hide = true)]
    pub no_all_extras: bool,

    /// Install the specified dependency group from a `pyproject.toml`.
    ///
    /// If no path is provided, the `pyproject.toml` in the working directory is used.
    ///
    /// May be provided multiple times.
    #[arg(long, group = "sources")]
    pub group: Vec<PipGroupName>,

    #[command(flatten)]
    pub resolver: ResolverArgs,

    #[command(flatten)]
    pub refresh: RefreshArgs,

    /// Ignore package dependencies, instead only add those packages explicitly listed
    /// on the command line to the resulting requirements file.
    #[arg(long)]
    pub no_deps: bool,

    #[arg(long, overrides_with("no_deps"), hide = true)]
    pub deps: bool,

    /// Write the compiled requirements to the given `requirements.txt` or `pylock.toml` file.
    ///
    /// If the file already exists, the existing versions will be preferred when resolving
    /// dependencies, unless `--upgrade` is also specified.
    #[arg(long, short, value_hint = ValueHint::FilePath)]
    pub output_file: Option<PathBuf>,

    /// The format in which the resolution should be output.
    ///
    /// Supports both `requirements.txt` and `pylock.toml` (PEP 751) output formats.
    ///
    /// uv will infer the output format from the file extension of the output file, if
    /// provided. Otherwise, defaults to `requirements.txt`.
    #[arg(long, value_enum)]
    pub format: Option<PipCompileFormat>,

    /// Include extras in the output file.
    ///
    /// By default, uv strips extras, as any packages pulled in by the extras are already included
    /// as dependencies in the output file directly. Further, output files generated with
    /// `--no-strip-extras` cannot be used as constraints files in `install` and `sync` invocations.
    #[arg(long, overrides_with("strip_extras"))]
    pub no_strip_extras: bool,

    #[arg(long, overrides_with("no_strip_extras"), hide = true)]
    pub strip_extras: bool,

    /// Include environment markers in the output file.
    ///
    /// By default, uv strips environment markers, as the resolution generated by `compile` is
    /// only guaranteed to be correct for the target environment.
    #[arg(long, overrides_with("strip_markers"))]
    pub no_strip_markers: bool,

    #[arg(long, overrides_with("no_strip_markers"), hide = true)]
    pub strip_markers: bool,

    /// Exclude comment annotations indicating the source of each package.
    #[arg(long, overrides_with("annotate"))]
    pub no_annotate: bool,

    #[arg(long, overrides_with("no_annotate"), hide = true)]
    pub annotate: bool,

    /// Exclude the comment header at the top of the generated output file.
    #[arg(long, overrides_with("header"))]
    pub no_header: bool,

    #[arg(long, overrides_with("no_header"), hide = true)]
    pub header: bool,

    /// The style of the annotation comments included in the output file, used to indicate the
    /// source of each package.
    ///
    /// Defaults to `split`.
    #[arg(long, value_enum)]
    pub annotation_style: Option<AnnotationStyle>,

    /// The header comment to include at the top of the output file generated by `uv pip compile`.
    ///
    /// Used to reflect custom build scripts and commands that wrap `uv pip compile`.
    #[arg(long, env = EnvVars::UV_CUSTOM_COMPILE_COMMAND, value_hint = ValueHint::Other)]
    pub custom_compile_command: Option<String>,

    /// The Python interpreter to use during resolution.
    ///
    /// A Python interpreter is required for building source distributions to determine package
    /// metadata when there are not wheels.
    ///
    /// The interpreter is also used to determine the default minimum Python version, unless
    /// `--python-version` is provided.
    ///
    /// This option respects `UV_PYTHON`, but when set via environment variable, it is overridden
    /// by `--python-version`.
    ///
    /// See `uv help python` for details on Python discovery and supported request formats.
    #[arg(
        long,
        short,
        verbatim_doc_comment,
        help_heading = "Python options",
        value_parser = parse_maybe_string,
        value_hint = ValueHint::Other,
    )]
    pub python: Option<Maybe<String>>,

    /// Install packages into the system Python environment.
    ///
    /// By default, uv uses the virtual environment in the current working directory or any parent
    /// directory, falling back to searching for a Python executable in `PATH`. The `--system`
    /// option instructs uv to avoid using a virtual environment Python and restrict its search to
    /// the system path.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// Include distribution hashes in the output file.
    #[arg(long, overrides_with("no_generate_hashes"))]
    pub generate_hashes: bool,

    #[arg(long, overrides_with("generate_hashes"), hide = true)]
    pub no_generate_hashes: bool,

    /// Don't build source distributions.
    ///
    /// When enabled, resolving will not run arbitrary Python code. The cached wheels of
    /// already-built source distributions will be reused, but operations that require building
    /// distributions will exit with an error.
    ///
    /// Alias for `--only-binary :all:`.
    #[arg(
        long,
        conflicts_with = "no_binary",
        conflicts_with = "only_binary",
        overrides_with("build")
    )]
    pub no_build: bool,

    #[arg(
        long,
        conflicts_with = "no_binary",
        conflicts_with = "only_binary",
        overrides_with("no_build"),
        hide = true
    )]
    pub build: bool,

    /// Don't install pre-built wheels.
    ///
    /// The given packages will be built and installed from source. The resolver will still use
    /// pre-built wheels to extract package metadata, if available.
    ///
    /// Multiple packages may be provided. Disable binaries for all packages with `:all:`.
    /// Clear previously specified packages with `:none:`.
    #[arg(long, value_delimiter = ',', conflicts_with = "no_build")]
    pub no_binary: Option<Vec<PackageNameSpecifier>>,

    /// Only use pre-built wheels; don't build source distributions.
    ///
    /// When enabled, resolving will not run code from the given packages. The cached wheels of already-built
    /// source distributions will be reused, but operations that require building distributions will
    /// exit with an error.
    ///
    /// Multiple packages may be provided. Disable binaries for all packages with `:all:`.
    /// Clear previously specified packages with `:none:`.
    #[arg(long, value_delimiter = ',', conflicts_with = "no_build")]
    pub only_binary: Option<Vec<PackageNameSpecifier>>,

    /// The Python version to use for resolution.
    ///
    /// For example, `3.8` or `3.8.17`.
    ///
    /// Defaults to the version of the Python interpreter used for resolution.
    ///
    /// Defines the minimum Python version that must be supported by the
    /// resolved requirements.
    ///
    /// If a patch version is omitted, the minimum patch version is assumed. For
    /// example, `3.8` is mapped to `3.8.0`.
    #[arg(long, help_heading = "Python options")]
    pub python_version: Option<PythonVersion>,

    /// The platform for which requirements should be resolved.
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
    #[arg(long)]
    pub python_platform: Option<TargetTriple>,

    /// Perform a universal resolution, attempting to generate a single `requirements.txt` output
    /// file that is compatible with all operating systems, architectures, and Python
    /// implementations.
    ///
    /// In universal mode, the current Python version (or user-provided `--python-version`) will be
    /// treated as a lower bound. For example, `--universal --python-version 3.7` would produce a
    /// universal resolution for Python 3.7 and later.
    ///
    /// Implies `--no-strip-markers`.
    #[arg(
        long,
        overrides_with("no_universal"),
        conflicts_with("python_platform"),
        conflicts_with("strip_markers")
    )]
    pub universal: bool,

    #[arg(long, overrides_with("universal"), hide = true)]
    pub no_universal: bool,

    /// Specify a package to omit from the output resolution. Its dependencies will still be
    /// included in the resolution. Equivalent to pip-compile's `--unsafe-package` option.
    #[arg(long, alias = "unsafe-package", value_delimiter = ',', value_hint = ValueHint::Other)]
    pub no_emit_package: Option<Vec<PackageName>>,

    /// Include `--index-url` and `--extra-index-url` entries in the generated output file.
    #[arg(long, overrides_with("no_emit_index_url"))]
    pub emit_index_url: bool,

    #[arg(long, overrides_with("emit_index_url"), hide = true)]
    pub no_emit_index_url: bool,

    /// Include `--find-links` entries in the generated output file.
    #[arg(long, overrides_with("no_emit_find_links"))]
    pub emit_find_links: bool,

    #[arg(long, overrides_with("emit_find_links"), hide = true)]
    pub no_emit_find_links: bool,

    /// Include `--no-binary` and `--only-binary` entries in the generated output file.
    #[arg(long, overrides_with("no_emit_build_options"))]
    pub emit_build_options: bool,

    #[arg(long, overrides_with("emit_build_options"), hide = true)]
    pub no_emit_build_options: bool,

    /// Whether to emit a marker string indicating when it is known that the
    /// resulting set of pinned dependencies is valid.
    ///
    /// The pinned dependencies may be valid even when the marker expression is
    /// false, but when the expression is true, the requirements are known to
    /// be correct.
    #[arg(long, overrides_with("no_emit_marker_expression"), hide = true)]
    pub emit_marker_expression: bool,

    #[arg(long, overrides_with("emit_marker_expression"), hide = true)]
    pub no_emit_marker_expression: bool,

    /// Include comment annotations indicating the index used to resolve each package (e.g.,
    /// `# from https://pypi.org/simple`).
    #[arg(long, overrides_with("no_emit_index_annotation"))]
    pub emit_index_annotation: bool,

    #[arg(long, overrides_with("emit_index_annotation"), hide = true)]
    pub no_emit_index_annotation: bool,

    /// The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`).
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

    #[command(flatten)]
    pub compat_args: compat::PipCompileCompatArgs,
}

#[derive(Args)]
pub struct PipSyncArgs {
    /// Include the packages listed in the given files.
    ///
    /// The following formats are supported: `requirements.txt`, `.py` files with inline metadata,
    /// `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.
    ///
    /// If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, uv will
    /// extract the requirements for the relevant project.
    ///
    /// If `-` is provided, then requirements will be read from stdin.
    #[arg(required(true), value_parser = parse_file_path, value_hint = ValueHint::FilePath)]
    pub src_file: Vec<PathBuf>,

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

    /// Include optional dependencies from the specified extra name; may be provided more than once.
    ///
    /// Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
    #[arg(long, value_delimiter = ',', conflicts_with = "all_extras", value_parser = extra_name_with_clap_error)]
    pub extra: Option<Vec<ExtraName>>,

    /// Include all optional dependencies.
    ///
    /// Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
    #[arg(long, conflicts_with = "extra", overrides_with = "no_all_extras")]
    pub all_extras: bool,

    #[arg(long, overrides_with("all_extras"), hide = true)]
    pub no_all_extras: bool,

    /// Install the specified dependency group from a `pylock.toml` or `pyproject.toml`.
    ///
    /// If no path is provided, the `pylock.toml` or `pyproject.toml` in the working directory is
    /// used.
    ///
    /// May be provided multiple times.
    #[arg(long, group = "sources")]
    pub group: Vec<PipGroupName>,

    #[command(flatten)]
    pub installer: InstallerArgs,

    #[command(flatten)]
    pub refresh: RefreshArgs,

    /// Require a matching hash for each requirement.
    ///
    /// By default, uv will verify any available hashes in the requirements file, but will not
    /// require that all requirements have an associated hash.
    ///
    /// When `--require-hashes` is enabled, _all_ requirements must include a hash or set of hashes,
    /// and _all_ requirements must either be pinned to exact versions (e.g., `==1.0.0`), or be
    /// specified via direct URL.
    ///
    /// Hash-checking mode introduces a number of additional constraints:
    ///
    /// - Git dependencies are not supported.
    /// - Editable installations are not supported.
    /// - Local dependencies are not supported, unless they point to a specific wheel (`.whl`) or
    ///   source archive (`.zip`, `.tar.gz`), as opposed to a directory.
    #[arg(
        long,
        env = EnvVars::UV_REQUIRE_HASHES,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_require_hashes"),
    )]
    pub require_hashes: bool,

    #[arg(long, overrides_with("require_hashes"), hide = true)]
    pub no_require_hashes: bool,

    #[arg(long, overrides_with("no_verify_hashes"), hide = true)]
    pub verify_hashes: bool,

    /// Disable validation of hashes in the requirements file.
    ///
    /// By default, uv will verify any available hashes in the requirements file, but will not
    /// require that all requirements have an associated hash. To enforce hash validation, use
    /// `--require-hashes`.
    #[arg(
        long,
        env = EnvVars::UV_NO_VERIFY_HASHES,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("verify_hashes"),
    )]
    pub no_verify_hashes: bool,

    /// The Python interpreter into which packages should be installed.
    ///
    /// By default, syncing requires a virtual environment. A path to an alternative Python can be
    /// provided, but it is only recommended in continuous integration (CI) environments and should
    /// be used with caution, as it can modify the system Python installation.
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

    /// Install packages into the system Python environment.
    ///
    /// By default, uv installs into the virtual environment in the current working directory or any
    /// parent directory. The `--system` option instructs uv to instead use the first Python found
    /// in the system `PATH`.
    ///
    /// WARNING: `--system` is intended for use in continuous integration (CI) environments and
    /// should be used with caution, as it can modify the system Python installation.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// Allow uv to modify an `EXTERNALLY-MANAGED` Python installation.
    ///
    /// WARNING: `--break-system-packages` is intended for use in continuous integration (CI)
    /// environments, when installing into Python installations that are managed by an external
    /// package manager, like `apt`. It should be used with caution, as such Python installations
    /// explicitly recommend against modifications by other package managers (like uv or `pip`).
    #[arg(
        long,
        env = EnvVars::UV_BREAK_SYSTEM_PACKAGES,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_break_system_packages")
    )]
    pub break_system_packages: bool,

    #[arg(long, overrides_with("break_system_packages"))]
    pub no_break_system_packages: bool,

    /// Install packages into the specified directory, rather than into the virtual or system Python
    /// environment. The packages will be installed at the top-level of the directory.
    ///
    /// Unlike other install operations, this command does not require discovery of an existing Python
    /// environment and only searches for a Python interpreter to use for package resolution.
    /// If a suitable Python interpreter cannot be found, uv will install one.
    /// To disable this, add `--no-python-downloads`.
    #[arg(short = 't', long, conflicts_with = "prefix", value_hint = ValueHint::DirPath)]
    pub target: Option<PathBuf>,

    /// Install packages into `lib`, `bin`, and other top-level folders under the specified
    /// directory, as if a virtual environment were present at that location.
    ///
    /// In general, prefer the use of `--python` to install into an alternate environment, as
    /// scripts and other artifacts installed via `--prefix` will reference the installing
    /// interpreter, rather than any interpreter added to the `--prefix` directory, rendering them
    /// non-portable.
    ///
    /// Unlike other install operations, this command does not require discovery of an existing Python
    /// environment and only searches for a Python interpreter to use for package resolution.
    /// If a suitable Python interpreter cannot be found, uv will install one.
    /// To disable this, add `--no-python-downloads`.
    #[arg(long, conflicts_with = "target", value_hint = ValueHint::DirPath)]
    pub prefix: Option<PathBuf>,

    /// Don't build source distributions.
    ///
    /// When enabled, resolving will not run arbitrary Python code. The cached wheels of
    /// already-built source distributions will be reused, but operations that require building
    /// distributions will exit with an error.
    ///
    /// Alias for `--only-binary :all:`.
    #[arg(
        long,
        conflicts_with = "no_binary",
        conflicts_with = "only_binary",
        overrides_with("build")
    )]
    pub no_build: bool,

    #[arg(
        long,
        conflicts_with = "no_binary",
        conflicts_with = "only_binary",
        overrides_with("no_build"),
        hide = true
    )]
    pub build: bool,

    /// Don't install pre-built wheels.
    ///
    /// The given packages will be built and installed from source. The resolver will still use
    /// pre-built wheels to extract package metadata, if available.
    ///
    /// Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear
    /// previously specified packages with `:none:`.
    #[arg(long, value_delimiter = ',', conflicts_with = "no_build")]
    pub no_binary: Option<Vec<PackageNameSpecifier>>,

    /// Only use pre-built wheels; don't build source distributions.
    ///
    /// When enabled, resolving will not run code from the given packages. The cached wheels of
    /// already-built source distributions will be reused, but operations that require building
    /// distributions will exit with an error.
    ///
    /// Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear
    /// previously specified packages with `:none:`.
    #[arg(long, value_delimiter = ',', conflicts_with = "no_build")]
    pub only_binary: Option<Vec<PackageNameSpecifier>>,

    /// Allow sync of empty requirements, which will clear the environment of all packages.
    #[arg(long, overrides_with("no_allow_empty_requirements"))]
    pub allow_empty_requirements: bool,

    #[arg(long, overrides_with("allow_empty_requirements"))]
    pub no_allow_empty_requirements: bool,

    /// The minimum Python version that should be supported by the requirements (e.g., `3.7` or
    /// `3.7.9`).
    ///
    /// If a patch version is omitted, the minimum patch version is assumed. For example, `3.7` is
    /// mapped to `3.7.0`.
    #[arg(long)]
    pub python_version: Option<PythonVersion>,

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

    /// Validate the Python environment after completing the installation, to detect packages with
    /// missing dependencies or other issues.
    #[arg(long, overrides_with("no_strict"))]
    pub strict: bool,

    #[arg(long, overrides_with("strict"), hide = true)]
    pub no_strict: bool,

    /// Perform a dry run, i.e., don't actually install anything but resolve the dependencies and
    /// print the resulting plan.
    #[arg(long)]
    pub dry_run: bool,

    /// The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`).
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

    #[command(flatten)]
    pub compat_args: compat::PipSyncCompatArgs,
}

#[derive(Args)]
#[command(group = clap::ArgGroup::new("sources").required(true).multiple(true))]
pub struct PipInstallArgs {
    /// Install all listed packages.
    ///
    /// The order of the packages is used to determine priority during resolution.
    #[arg(group = "sources", value_hint = ValueHint::Other)]
    pub package: Vec<String>,

    /// Install the packages listed in the given files.
    ///
    /// The following formats are supported: `requirements.txt`, `.py` files with inline metadata,
    /// `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.
    ///
    /// If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, uv will extract the
    /// requirements for the relevant project.
    ///
    /// If `-` is provided, then requirements will be read from stdin.
    #[arg(
        long,
        short,
        alias = "requirement",
        group = "sources",
        value_parser = parse_file_path,
        value_hint = ValueHint::FilePath,
    )]
    pub requirements: Vec<PathBuf>,

    /// Install the editable package based on the provided local file path.
    #[arg(long, short, group = "sources")]
    pub editable: Vec<String>,

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

    /// Include optional dependencies from the specified extra name; may be provided more than once.
    ///
    /// Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
    #[arg(long, value_delimiter = ',', conflicts_with = "all_extras", value_parser = extra_name_with_clap_error)]
    pub extra: Option<Vec<ExtraName>>,

    /// Include all optional dependencies.
    ///
    /// Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
    #[arg(long, conflicts_with = "extra", overrides_with = "no_all_extras")]
    pub all_extras: bool,

    #[arg(long, overrides_with("all_extras"), hide = true)]
    pub no_all_extras: bool,

    /// Install the specified dependency group from a `pylock.toml` or `pyproject.toml`.
    ///
    /// If no path is provided, the `pylock.toml` or `pyproject.toml` in the working directory is
    /// used.
    ///
    /// May be provided multiple times.
    #[arg(long, group = "sources")]
    pub group: Vec<PipGroupName>,

    #[command(flatten)]
    pub installer: ResolverInstallerArgs,

    #[command(flatten)]
    pub refresh: RefreshArgs,

    /// Ignore package dependencies, instead only installing those packages explicitly listed
    /// on the command line or in the requirements files.
    #[arg(long, overrides_with("deps"))]
    pub no_deps: bool,

    #[arg(long, overrides_with("no_deps"), hide = true)]
    pub deps: bool,

    /// Require a matching hash for each requirement.
    ///
    /// By default, uv will verify any available hashes in the requirements file, but will not
    /// require that all requirements have an associated hash.
    ///
    /// When `--require-hashes` is enabled, _all_ requirements must include a hash or set of hashes,
    /// and _all_ requirements must either be pinned to exact versions (e.g., `==1.0.0`), or be
    /// specified via direct URL.
    ///
    /// Hash-checking mode introduces a number of additional constraints:
    ///
    /// - Git dependencies are not supported.
    /// - Editable installations are not supported.
    /// - Local dependencies are not supported, unless they point to a specific wheel (`.whl`) or
    ///   source archive (`.zip`, `.tar.gz`), as opposed to a directory.
    #[arg(
        long,
        env = EnvVars::UV_REQUIRE_HASHES,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_require_hashes"),
    )]
    pub require_hashes: bool,

    #[arg(long, overrides_with("require_hashes"), hide = true)]
    pub no_require_hashes: bool,

    #[arg(long, overrides_with("no_verify_hashes"), hide = true)]
    pub verify_hashes: bool,

    /// Disable validation of hashes in the requirements file.
    ///
    /// By default, uv will verify any available hashes in the requirements file, but will not
    /// require that all requirements have an associated hash. To enforce hash validation, use
    /// `--require-hashes`.
    #[arg(
        long,
        env = EnvVars::UV_NO_VERIFY_HASHES,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("verify_hashes"),
    )]
    pub no_verify_hashes: bool,

    /// The Python interpreter into which packages should be installed.
    ///
    /// By default, installation requires a virtual environment. A path to an alternative Python can
    /// be provided, but it is only recommended in continuous integration (CI) environments and
    /// should be used with caution, as it can modify the system Python installation.
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

    /// Install packages into the system Python environment.
    ///
    /// By default, uv installs into the virtual environment in the current working directory or any
    /// parent directory. The `--system` option instructs uv to instead use the first Python found
    /// in the system `PATH`.
    ///
    /// WARNING: `--system` is intended for use in continuous integration (CI) environments and
    /// should be used with caution, as it can modify the system Python installation.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// Allow uv to modify an `EXTERNALLY-MANAGED` Python installation.
    ///
    /// WARNING: `--break-system-packages` is intended for use in continuous integration (CI)
    /// environments, when installing into Python installations that are managed by an external
    /// package manager, like `apt`. It should be used with caution, as such Python installations
    /// explicitly recommend against modifications by other package managers (like uv or `pip`).
    #[arg(
        long,
        env = EnvVars::UV_BREAK_SYSTEM_PACKAGES,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_break_system_packages")
    )]
    pub break_system_packages: bool,

    #[arg(long, overrides_with("break_system_packages"))]
    pub no_break_system_packages: bool,

    /// Install packages into the specified directory, rather than into the virtual or system Python
    /// environment. The packages will be installed at the top-level of the directory.
    ///
    /// Unlike other install operations, this command does not require discovery of an existing Python
    /// environment and only searches for a Python interpreter to use for package resolution.
    /// If a suitable Python interpreter cannot be found, uv will install one.
    /// To disable this, add `--no-python-downloads`.
    #[arg(short = 't', long, conflicts_with = "prefix", value_hint = ValueHint::DirPath)]
    pub target: Option<PathBuf>,

    /// Install packages into `lib`, `bin`, and other top-level folders under the specified
    /// directory, as if a virtual environment were present at that location.
    ///
    /// In general, prefer the use of `--python` to install into an alternate environment, as
    /// scripts and other artifacts installed via `--prefix` will reference the installing
    /// interpreter, rather than any interpreter added to the `--prefix` directory, rendering them
    /// non-portable.
    ///
    /// Unlike other install operations, this command does not require discovery of an existing Python
    /// environment and only searches for a Python interpreter to use for package resolution.
    /// If a suitable Python interpreter cannot be found, uv will install one.
    /// To disable this, add `--no-python-downloads`.
    #[arg(long, conflicts_with = "target", value_hint = ValueHint::DirPath)]
    pub prefix: Option<PathBuf>,

    /// Don't build source distributions.
    ///
    /// When enabled, resolving will not run arbitrary Python code. The cached wheels of
    /// already-built source distributions will be reused, but operations that require building
    /// distributions will exit with an error.
    ///
    /// Alias for `--only-binary :all:`.
    #[arg(
        long,
        conflicts_with = "no_binary",
        conflicts_with = "only_binary",
        overrides_with("build")
    )]
    pub no_build: bool,

    #[arg(
        long,
        conflicts_with = "no_binary",
        conflicts_with = "only_binary",
        overrides_with("no_build"),
        hide = true
    )]
    pub build: bool,

    /// Don't install pre-built wheels.
    ///
    /// The given packages will be built and installed from source. The resolver will still use
    /// pre-built wheels to extract package metadata, if available.
    ///
    /// Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear
    /// previously specified packages with `:none:`.
    #[arg(long, value_delimiter = ',', conflicts_with = "no_build")]
    pub no_binary: Option<Vec<PackageNameSpecifier>>,

    /// Only use pre-built wheels; don't build source distributions.
    ///
    /// When enabled, resolving will not run code from the given packages. The cached wheels of
    /// already-built source distributions will be reused, but operations that require building
    /// distributions will exit with an error.
    ///
    /// Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear
    /// previously specified packages with `:none:`.
    #[arg(long, value_delimiter = ',', conflicts_with = "no_build")]
    pub only_binary: Option<Vec<PackageNameSpecifier>>,

    /// The minimum Python version that should be supported by the requirements (e.g., `3.7` or
    /// `3.7.9`).
    ///
    /// If a patch version is omitted, the minimum patch version is assumed. For example, `3.7` is
    /// mapped to `3.7.0`.
    #[arg(long)]
    pub python_version: Option<PythonVersion>,

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

    /// Do not remove extraneous packages present in the environment.
    #[arg(long, overrides_with("exact"), alias = "no-exact", hide = true)]
    pub inexact: bool,

    /// Perform an exact sync, removing extraneous packages.
    ///
    /// By default, installing will make the minimum necessary changes to satisfy the requirements.
    /// When enabled, uv will update the environment to exactly match the requirements, removing
    /// packages that are not included in the requirements.
    #[arg(long, overrides_with("inexact"))]
    pub exact: bool,

    /// Validate the Python environment after completing the installation, to detect packages with
    /// missing dependencies or other issues.
    #[arg(long, overrides_with("no_strict"))]
    pub strict: bool,

    #[arg(long, overrides_with("strict"), hide = true)]
    pub no_strict: bool,

    /// Perform a dry run, i.e., don't actually install anything but resolve the dependencies and
    /// print the resulting plan.
    #[arg(long)]
    pub dry_run: bool,

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

    #[command(flatten)]
    pub compat_args: compat::PipInstallCompatArgs,
}

#[derive(Args)]
#[command(group = clap::ArgGroup::new("sources").required(true).multiple(true))]
pub struct PipUninstallArgs {
    /// Uninstall all listed packages.
    #[arg(group = "sources", value_hint = ValueHint::Other)]
    pub package: Vec<String>,

    /// Uninstall the packages listed in the given files.
    ///
    /// The following formats are supported: `requirements.txt`, `.py` files with inline metadata,
    /// `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.
    #[arg(long, short, alias = "requirement", group = "sources", value_parser = parse_file_path, value_hint = ValueHint::FilePath)]
    pub requirements: Vec<PathBuf>,

    /// The Python interpreter from which packages should be uninstalled.
    ///
    /// By default, uninstallation requires a virtual environment. A path to an alternative Python
    /// can be provided, but it is only recommended in continuous integration (CI) environments and
    /// should be used with caution, as it can modify the system Python installation.
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

    /// Attempt to use `keyring` for authentication for remote requirements files.
    ///
    /// At present, only `--keyring-provider subprocess` is supported, which configures uv to use
    /// the `keyring` CLI to handle authentication.
    ///
    /// Defaults to `disabled`.
    #[arg(long, value_enum, env = EnvVars::UV_KEYRING_PROVIDER)]
    pub keyring_provider: Option<KeyringProviderType>,

    /// Use the system Python to uninstall packages.
    ///
    /// By default, uv uninstalls from the virtual environment in the current working directory or
    /// any parent directory. The `--system` option instructs uv to instead use the first Python
    /// found in the system `PATH`.
    ///
    /// WARNING: `--system` is intended for use in continuous integration (CI) environments and
    /// should be used with caution, as it can modify the system Python installation.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// Allow uv to modify an `EXTERNALLY-MANAGED` Python installation.
    ///
    /// WARNING: `--break-system-packages` is intended for use in continuous integration (CI)
    /// environments, when installing into Python installations that are managed by an external
    /// package manager, like `apt`. It should be used with caution, as such Python installations
    /// explicitly recommend against modifications by other package managers (like uv or `pip`).
    #[arg(
        long,
        env = EnvVars::UV_BREAK_SYSTEM_PACKAGES,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_break_system_packages")
    )]
    pub break_system_packages: bool,

    #[arg(long, overrides_with("break_system_packages"))]
    pub no_break_system_packages: bool,

    /// Uninstall packages from the specified `--target` directory.
    #[arg(short = 't', long, conflicts_with = "prefix", value_hint = ValueHint::DirPath)]
    pub target: Option<PathBuf>,

    /// Uninstall packages from the specified `--prefix` directory.
    #[arg(long, conflicts_with = "target", value_hint = ValueHint::DirPath)]
    pub prefix: Option<PathBuf>,

    /// Perform a dry run, i.e., don't actually uninstall anything but print the resulting plan.
    #[arg(long)]
    pub dry_run: bool,

    #[command(flatten)]
    pub compat_args: compat::PipGlobalCompatArgs,
}

#[derive(Args)]
pub struct PipFreezeArgs {
    /// Exclude any editable packages from output.
    #[arg(long)]
    pub exclude_editable: bool,

    /// Exclude the specified package(s) from the output.
    #[arg(long)]
    pub r#exclude: Vec<PackageName>,

    /// Validate the Python environment, to detect packages with missing dependencies and other
    /// issues.
    #[arg(long, overrides_with("no_strict"))]
    pub strict: bool,

    #[arg(long, overrides_with("strict"), hide = true)]
    pub no_strict: bool,

    /// The Python interpreter for which packages should be listed.
    ///
    /// By default, uv lists packages in a virtual environment but will show packages in a system
    /// Python environment if no virtual environment is found.
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

    /// Restrict to the specified installation path for listing packages (can be used multiple times).
    #[arg(long("path"), value_parser = parse_file_path, value_hint = ValueHint::DirPath)]
    pub paths: Option<Vec<PathBuf>>,

    /// List packages in the system Python environment.
    ///
    /// Disables discovery of virtual environments.
    ///
    /// See `uv help python` for details on Python discovery.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// List packages from the specified `--target` directory.
    #[arg(short = 't', long, conflicts_with_all = ["prefix", "paths"], value_hint = ValueHint::DirPath)]
    pub target: Option<PathBuf>,

    /// List packages from the specified `--prefix` directory.
    #[arg(long, conflicts_with_all = ["target", "paths"], value_hint = ValueHint::DirPath)]
    pub prefix: Option<PathBuf>,

    #[command(flatten)]
    pub compat_args: compat::PipGlobalCompatArgs,
}

#[derive(Args)]
pub struct PipListArgs {
    /// Only include editable projects.
    #[arg(short, long)]
    pub editable: bool,

    /// Exclude any editable packages from output.
    #[arg(long, conflicts_with = "editable")]
    pub exclude_editable: bool,

    /// Exclude the specified package(s) from the output.
    #[arg(long, value_hint = ValueHint::Other)]
    pub r#exclude: Vec<PackageName>,

    /// Select the output format.
    #[arg(long, value_enum, default_value_t = ListFormat::default())]
    pub format: ListFormat,

    /// List outdated packages.
    ///
    /// The latest version of each package will be shown alongside the installed version. Up-to-date
    /// packages will be omitted from the output.
    #[arg(long, overrides_with("no_outdated"))]
    pub outdated: bool,

    #[arg(long, overrides_with("outdated"), hide = true)]
    pub no_outdated: bool,

    /// Validate the Python environment, to detect packages with missing dependencies and other
    /// issues.
    #[arg(long, overrides_with("no_strict"))]
    pub strict: bool,

    #[arg(long, overrides_with("strict"), hide = true)]
    pub no_strict: bool,

    #[command(flatten)]
    pub fetch: FetchArgs,

    /// The Python interpreter for which packages should be listed.
    ///
    /// By default, uv lists packages in a virtual environment but will show packages in a system
    /// Python environment if no virtual environment is found.
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

    /// List packages in the system Python environment.
    ///
    /// Disables discovery of virtual environments.
    ///
    /// See `uv help python` for details on Python discovery.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// List packages from the specified `--target` directory.
    #[arg(short = 't', long, conflicts_with = "prefix", value_hint = ValueHint::DirPath)]
    pub target: Option<PathBuf>,

    /// List packages from the specified `--prefix` directory.
    #[arg(long, conflicts_with = "target", value_hint = ValueHint::DirPath)]
    pub prefix: Option<PathBuf>,

    #[command(flatten)]
    pub compat_args: compat::PipListCompatArgs,
}

#[derive(Args)]
pub struct PipCheckArgs {
    /// The Python interpreter for which packages should be checked.
    ///
    /// By default, uv checks packages in a virtual environment but will check packages in a system
    /// Python environment if no virtual environment is found.
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

    /// Check packages in the system Python environment.
    ///
    /// Disables discovery of virtual environments.
    ///
    /// See `uv help python` for details on Python discovery.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// The Python version against which packages should be checked.
    ///
    /// By default, the installed packages are checked against the version of the current
    /// interpreter.
    #[arg(long)]
    pub python_version: Option<PythonVersion>,

    /// The platform for which packages should be checked.
    ///
    /// By default, the installed packages are checked against the platform of the current
    /// interpreter.
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
    #[arg(long)]
    pub python_platform: Option<TargetTriple>,
}

#[derive(Args)]
pub struct PipShowArgs {
    /// The package(s) to display.
    #[arg(value_hint = ValueHint::Other)]
    pub package: Vec<PackageName>,

    /// Validate the Python environment, to detect packages with missing dependencies and other
    /// issues.
    #[arg(long, overrides_with("no_strict"))]
    pub strict: bool,

    #[arg(long, overrides_with("strict"), hide = true)]
    pub no_strict: bool,

    /// Show the full list of installed files for each package.
    #[arg(short, long)]
    pub files: bool,

    /// The Python interpreter to find the package in.
    ///
    /// By default, uv looks for packages in a virtual environment but will look for packages in a
    /// system Python environment if no virtual environment is found.
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

    /// Show a package in the system Python environment.
    ///
    /// Disables discovery of virtual environments.
    ///
    /// See `uv help python` for details on Python discovery.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    /// Show a package from the specified `--target` directory.
    #[arg(short = 't', long, conflicts_with = "prefix", value_hint = ValueHint::DirPath)]
    pub target: Option<PathBuf>,

    /// Show a package from the specified `--prefix` directory.
    #[arg(long, conflicts_with = "target", value_hint = ValueHint::DirPath)]
    pub prefix: Option<PathBuf>,

    #[command(flatten)]
    pub compat_args: compat::PipGlobalCompatArgs,
}

#[derive(Args)]
pub struct PipTreeArgs {
    /// Show the version constraint(s) imposed on each package.
    #[arg(long)]
    pub show_version_specifiers: bool,

    #[command(flatten)]
    pub tree: DisplayTreeArgs,

    /// Validate the Python environment, to detect packages with missing dependencies and other
    /// issues.
    #[arg(long, overrides_with("no_strict"))]
    pub strict: bool,

    #[arg(long, overrides_with("strict"), hide = true)]
    pub no_strict: bool,

    #[command(flatten)]
    pub fetch: FetchArgs,

    /// The Python interpreter for which packages should be listed.
    ///
    /// By default, uv lists packages in a virtual environment but will show packages in a system
    /// Python environment if no virtual environment is found.
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

    /// List packages in the system Python environment.
    ///
    /// Disables discovery of virtual environments.
    ///
    /// See `uv help python` for details on Python discovery.
    #[arg(
        long,
        env = EnvVars::UV_SYSTEM_PYTHON,
        value_parser = clap::builder::BoolishValueParser::new(),
        overrides_with("no_system")
    )]
    pub system: bool,

    #[arg(long, overrides_with("system"), hide = true)]
    pub no_system: bool,

    #[command(flatten)]
    pub compat_args: compat::PipGlobalCompatArgs,
}

#[derive(Args)]
pub struct PipDebugArgs {
    #[arg(long, hide = true)]
    pub platform: Option<String>,

    #[arg(long, hide = true)]
    pub python_version: Option<String>,

    #[arg(long, hide = true)]
    pub implementation: Option<String>,

    #[arg(long, hide = true)]
    pub abi: Option<String>,
}
