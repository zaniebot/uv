use assert_fs::prelude::*;
use uv_static::EnvVars;

use uv_test::uv_snapshot;

use super::add_shared_args;

/// Read from a `uv.toml` file in the current directory.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_uv_toml() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // Resolution should use the lowest direct version, and generate hashes.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    GlobalSettings {
        required_version: None,
        quiet: 0,
        verbose: 0,
        color: Auto,
        network_settings: NetworkSettings {
            connectivity: Online,
            offline: Disabled,
            native_tls: false,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            allow_insecure_host: [],
            read_timeout: [TIME],
            connect_timeout: [TIME],
            retries: 3,
        },
        concurrency: Concurrency {
            downloads: 50,
            builds: 16,
            installs: 8,
        },
        show_settings: true,
        preview: Preview {
            flags: [],
        },
        python_preference: Managed,
        python_downloads: Automatic,
        no_progress: false,
        installer_metadata: true,
    }
    CacheSettings {
        no_cache: false,
        cache_dir: Some(
            "[CACHE_DIR]/",
        ),
    }
    PipCompileSettings {
        format: None,
        src_file: [
            "requirements.in",
        ],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        environments: SupportedEnvironments(
            [],
        ),
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: PipSettings {
            index_locations: IndexLocations {
                indexes: [
                    Index {
                        name: None,
                        url: Pypi(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Project,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                ],
                flat_index: [],
                no_index: false,
            },
            python: None,
            install_mirrors: PythonInstallMirrors {
                python_install_mirror: None,
                pypy_install_mirror: None,
                python_downloads_json_url: None,
            },
            system: false,
            extras: ExtrasSpecification(
                ExtrasSpecificationInner {
                    include: Some(
                        [],
                    ),
                    exclude: [],
                    only_extras: false,
                    history: ExtrasSpecificationHistory {
                        extra: [],
                        only_extra: [],
                        no_extra: [],
                        all_extras: false,
                        no_default_extras: false,
                        defaults: List(
                            [],
                        ),
                    },
                },
            ),
            groups: [],
            break_system_packages: false,
            target: None,
            prefix: None,
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            torch_backend: None,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            allow_empty_requirements: false,
            strict: false,
            dependency_mode: Transitive,
            resolution: LowestDirect,
            prerelease: IfNecessaryOrExplicit,
            fork_strategy: RequiresPython,
            dependency_metadata: DependencyMetadata(
                {},
            ),
            output_file: None,
            no_strip_extras: false,
            no_strip_markers: false,
            no_annotate: false,
            no_header: false,
            custom_compile_command: None,
            generate_hashes: true,
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            python_version: None,
            python_platform: None,
            universal: false,
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            no_emit_package: [],
            emit_index_url: false,
            emit_find_links: false,
            emit_build_options: false,
            emit_marker_expression: false,
            emit_index_annotation: false,
            annotation_style: Split,
            link_mode: Clone,
            compile_bytecode: false,
            sources: None,
            hash_checking: Some(
                Verify,
            ),
            upgrade: Upgrade {
                strategy: None,
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Resolution should use the highest version, and generate hashes.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in")
        .arg("--resolution=highest"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    GlobalSettings {
        required_version: None,
        quiet: 0,
        verbose: 0,
        color: Auto,
        network_settings: NetworkSettings {
            connectivity: Online,
            offline: Disabled,
            native_tls: false,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            allow_insecure_host: [],
            read_timeout: [TIME],
            connect_timeout: [TIME],
            retries: 3,
        },
        concurrency: Concurrency {
            downloads: 50,
            builds: 16,
            installs: 8,
        },
        show_settings: true,
        preview: Preview {
            flags: [],
        },
        python_preference: Managed,
        python_downloads: Automatic,
        no_progress: false,
        installer_metadata: true,
    }
    CacheSettings {
        no_cache: false,
        cache_dir: Some(
            "[CACHE_DIR]/",
        ),
    }
    PipCompileSettings {
        format: None,
        src_file: [
            "requirements.in",
        ],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        environments: SupportedEnvironments(
            [],
        ),
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: PipSettings {
            index_locations: IndexLocations {
                indexes: [
                    Index {
                        name: None,
                        url: Pypi(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Project,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                ],
                flat_index: [],
                no_index: false,
            },
            python: None,
            install_mirrors: PythonInstallMirrors {
                python_install_mirror: None,
                pypy_install_mirror: None,
                python_downloads_json_url: None,
            },
            system: false,
            extras: ExtrasSpecification(
                ExtrasSpecificationInner {
                    include: Some(
                        [],
                    ),
                    exclude: [],
                    only_extras: false,
                    history: ExtrasSpecificationHistory {
                        extra: [],
                        only_extra: [],
                        no_extra: [],
                        all_extras: false,
                        no_default_extras: false,
                        defaults: List(
                            [],
                        ),
                    },
                },
            ),
            groups: [],
            break_system_packages: false,
            target: None,
            prefix: None,
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            torch_backend: None,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            allow_empty_requirements: false,
            strict: false,
            dependency_mode: Transitive,
            resolution: Highest,
            prerelease: IfNecessaryOrExplicit,
            fork_strategy: RequiresPython,
            dependency_metadata: DependencyMetadata(
                {},
            ),
            output_file: None,
            no_strip_extras: false,
            no_strip_markers: false,
            no_annotate: false,
            no_header: false,
            custom_compile_command: None,
            generate_hashes: true,
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            python_version: None,
            python_platform: None,
            universal: false,
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            no_emit_package: [],
            emit_index_url: false,
            emit_find_links: false,
            emit_build_options: false,
            emit_marker_expression: false,
            emit_index_annotation: false,
            annotation_style: Split,
            link_mode: Clone,
            compile_bytecode: false,
            sources: None,
            hash_checking: Some(
                Verify,
            ),
            upgrade: Upgrade {
                strategy: None,
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Resolution should use the highest version, and omit hashes.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in")
        .arg("--resolution=highest")
        .arg("--no-generate-hashes"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    GlobalSettings {
        required_version: None,
        quiet: 0,
        verbose: 0,
        color: Auto,
        network_settings: NetworkSettings {
            connectivity: Online,
            offline: Disabled,
            native_tls: false,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            allow_insecure_host: [],
            read_timeout: [TIME],
            connect_timeout: [TIME],
            retries: 3,
        },
        concurrency: Concurrency {
            downloads: 50,
            builds: 16,
            installs: 8,
        },
        show_settings: true,
        preview: Preview {
            flags: [],
        },
        python_preference: Managed,
        python_downloads: Automatic,
        no_progress: false,
        installer_metadata: true,
    }
    CacheSettings {
        no_cache: false,
        cache_dir: Some(
            "[CACHE_DIR]/",
        ),
    }
    PipCompileSettings {
        format: None,
        src_file: [
            "requirements.in",
        ],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        environments: SupportedEnvironments(
            [],
        ),
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: PipSettings {
            index_locations: IndexLocations {
                indexes: [
                    Index {
                        name: None,
                        url: Pypi(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Project,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                ],
                flat_index: [],
                no_index: false,
            },
            python: None,
            install_mirrors: PythonInstallMirrors {
                python_install_mirror: None,
                pypy_install_mirror: None,
                python_downloads_json_url: None,
            },
            system: false,
            extras: ExtrasSpecification(
                ExtrasSpecificationInner {
                    include: Some(
                        [],
                    ),
                    exclude: [],
                    only_extras: false,
                    history: ExtrasSpecificationHistory {
                        extra: [],
                        only_extra: [],
                        no_extra: [],
                        all_extras: false,
                        no_default_extras: false,
                        defaults: List(
                            [],
                        ),
                    },
                },
            ),
            groups: [],
            break_system_packages: false,
            target: None,
            prefix: None,
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            torch_backend: None,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            allow_empty_requirements: false,
            strict: false,
            dependency_mode: Transitive,
            resolution: Highest,
            prerelease: IfNecessaryOrExplicit,
            fork_strategy: RequiresPython,
            dependency_metadata: DependencyMetadata(
                {},
            ),
            output_file: None,
            no_strip_extras: false,
            no_strip_markers: false,
            no_annotate: false,
            no_header: false,
            custom_compile_command: None,
            generate_hashes: false,
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            python_version: None,
            python_platform: None,
            universal: false,
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            no_emit_package: [],
            emit_index_url: false,
            emit_find_links: false,
            emit_build_options: false,
            emit_marker_expression: false,
            emit_index_annotation: false,
            annotation_style: Split,
            link_mode: Clone,
            compile_bytecode: false,
            sources: None,
            hash_checking: Some(
                Verify,
            ),
            upgrade: Upgrade {
                strategy: None,
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    Ok(())
}

/// Read from a `pyproject.toml` file in the current directory.
///
/// We prefer `uv.toml` when both are present, but respect `pyproject.toml` otherwise.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_pyproject_toml() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // Write a `pyproject.toml` file to the directory.
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // Resolution should use the lowest direct version, and generate hashes.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    GlobalSettings {
        required_version: None,
        quiet: 0,
        verbose: 0,
        color: Auto,
        network_settings: NetworkSettings {
            connectivity: Online,
            offline: Disabled,
            native_tls: false,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            allow_insecure_host: [],
            read_timeout: [TIME],
            connect_timeout: [TIME],
            retries: 3,
        },
        concurrency: Concurrency {
            downloads: 50,
            builds: 16,
            installs: 8,
        },
        show_settings: true,
        preview: Preview {
            flags: [],
        },
        python_preference: Managed,
        python_downloads: Automatic,
        no_progress: false,
        installer_metadata: true,
    }
    CacheSettings {
        no_cache: false,
        cache_dir: Some(
            "[CACHE_DIR]/",
        ),
    }
    PipCompileSettings {
        format: None,
        src_file: [
            "requirements.in",
        ],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        environments: SupportedEnvironments(
            [],
        ),
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: PipSettings {
            index_locations: IndexLocations {
                indexes: [
                    Index {
                        name: None,
                        url: Pypi(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Project,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                ],
                flat_index: [],
                no_index: false,
            },
            python: None,
            install_mirrors: PythonInstallMirrors {
                python_install_mirror: None,
                pypy_install_mirror: None,
                python_downloads_json_url: None,
            },
            system: false,
            extras: ExtrasSpecification(
                ExtrasSpecificationInner {
                    include: Some(
                        [],
                    ),
                    exclude: [],
                    only_extras: false,
                    history: ExtrasSpecificationHistory {
                        extra: [],
                        only_extra: [],
                        no_extra: [],
                        all_extras: false,
                        no_default_extras: false,
                        defaults: List(
                            [],
                        ),
                    },
                },
            ),
            groups: [],
            break_system_packages: false,
            target: None,
            prefix: None,
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            torch_backend: None,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            allow_empty_requirements: false,
            strict: false,
            dependency_mode: Transitive,
            resolution: LowestDirect,
            prerelease: IfNecessaryOrExplicit,
            fork_strategy: RequiresPython,
            dependency_metadata: DependencyMetadata(
                {},
            ),
            output_file: None,
            no_strip_extras: false,
            no_strip_markers: false,
            no_annotate: false,
            no_header: false,
            custom_compile_command: None,
            generate_hashes: true,
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            python_version: None,
            python_platform: None,
            universal: false,
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            no_emit_package: [],
            emit_index_url: false,
            emit_find_links: false,
            emit_build_options: false,
            emit_marker_expression: false,
            emit_index_annotation: false,
            annotation_style: Split,
            link_mode: Clone,
            compile_bytecode: false,
            sources: None,
            hash_checking: Some(
                Verify,
            ),
            upgrade: Upgrade {
                strategy: None,
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Remove the `uv.toml` file.
    fs_err::remove_file(config.path())?;

    // Resolution should use the highest version, and omit hashes.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    GlobalSettings {
        required_version: None,
        quiet: 0,
        verbose: 0,
        color: Auto,
        network_settings: NetworkSettings {
            connectivity: Online,
            offline: Disabled,
            native_tls: false,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            allow_insecure_host: [],
            read_timeout: [TIME],
            connect_timeout: [TIME],
            retries: 3,
        },
        concurrency: Concurrency {
            downloads: 50,
            builds: 16,
            installs: 8,
        },
        show_settings: true,
        preview: Preview {
            flags: [],
        },
        python_preference: Managed,
        python_downloads: Automatic,
        no_progress: false,
        installer_metadata: true,
    }
    CacheSettings {
        no_cache: false,
        cache_dir: Some(
            "[CACHE_DIR]/",
        ),
    }
    PipCompileSettings {
        format: None,
        src_file: [
            "requirements.in",
        ],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        environments: SupportedEnvironments(
            [],
        ),
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: PipSettings {
            index_locations: IndexLocations {
                indexes: [],
                flat_index: [],
                no_index: false,
            },
            python: None,
            install_mirrors: PythonInstallMirrors {
                python_install_mirror: None,
                pypy_install_mirror: None,
                python_downloads_json_url: None,
            },
            system: false,
            extras: ExtrasSpecification(
                ExtrasSpecificationInner {
                    include: Some(
                        [],
                    ),
                    exclude: [],
                    only_extras: false,
                    history: ExtrasSpecificationHistory {
                        extra: [],
                        only_extra: [],
                        no_extra: [],
                        all_extras: false,
                        no_default_extras: false,
                        defaults: List(
                            [],
                        ),
                    },
                },
            ),
            groups: [],
            break_system_packages: false,
            target: None,
            prefix: None,
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            torch_backend: None,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            allow_empty_requirements: false,
            strict: false,
            dependency_mode: Transitive,
            resolution: Highest,
            prerelease: IfNecessaryOrExplicit,
            fork_strategy: RequiresPython,
            dependency_metadata: DependencyMetadata(
                {},
            ),
            output_file: None,
            no_strip_extras: false,
            no_strip_markers: false,
            no_annotate: false,
            no_header: false,
            custom_compile_command: None,
            generate_hashes: false,
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            python_version: None,
            python_platform: None,
            universal: false,
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            no_emit_package: [],
            emit_index_url: false,
            emit_find_links: false,
            emit_build_options: false,
            emit_marker_expression: false,
            emit_index_annotation: false,
            annotation_style: Split,
            link_mode: Clone,
            compile_bytecode: false,
            sources: None,
            hash_checking: Some(
                Verify,
            ),
            upgrade: Upgrade {
                strategy: None,
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Add configuration to the `pyproject.toml` file.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        python-platform = "x86_64-unknown-linux-gnu"
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // Resolution should use the lowest direct version, and generate hashes.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    GlobalSettings {
        required_version: None,
        quiet: 0,
        verbose: 0,
        color: Auto,
        network_settings: NetworkSettings {
            connectivity: Online,
            offline: Disabled,
            native_tls: false,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            allow_insecure_host: [],
            read_timeout: [TIME],
            connect_timeout: [TIME],
            retries: 3,
        },
        concurrency: Concurrency {
            downloads: 50,
            builds: 16,
            installs: 8,
        },
        show_settings: true,
        preview: Preview {
            flags: [],
        },
        python_preference: Managed,
        python_downloads: Automatic,
        no_progress: false,
        installer_metadata: true,
    }
    CacheSettings {
        no_cache: false,
        cache_dir: Some(
            "[CACHE_DIR]/",
        ),
    }
    PipCompileSettings {
        format: None,
        src_file: [
            "requirements.in",
        ],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        environments: SupportedEnvironments(
            [],
        ),
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: PipSettings {
            index_locations: IndexLocations {
                indexes: [
                    Index {
                        name: None,
                        url: Pypi(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: None,
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                ],
                flat_index: [],
                no_index: false,
            },
            python: None,
            install_mirrors: PythonInstallMirrors {
                python_install_mirror: None,
                pypy_install_mirror: None,
                python_downloads_json_url: None,
            },
            system: false,
            extras: ExtrasSpecification(
                ExtrasSpecificationInner {
                    include: Some(
                        [],
                    ),
                    exclude: [],
                    only_extras: false,
                    history: ExtrasSpecificationHistory {
                        extra: [],
                        only_extra: [],
                        no_extra: [],
                        all_extras: false,
                        no_default_extras: false,
                        defaults: List(
                            [],
                        ),
                    },
                },
            ),
            groups: [],
            break_system_packages: false,
            target: None,
            prefix: None,
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            torch_backend: None,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            allow_empty_requirements: false,
            strict: false,
            dependency_mode: Transitive,
            resolution: LowestDirect,
            prerelease: IfNecessaryOrExplicit,
            fork_strategy: RequiresPython,
            dependency_metadata: DependencyMetadata(
                {},
            ),
            output_file: None,
            no_strip_extras: false,
            no_strip_markers: false,
            no_annotate: false,
            no_header: false,
            custom_compile_command: None,
            generate_hashes: true,
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            python_version: None,
            python_platform: Some(
                X8664UnknownLinuxGnu,
            ),
            universal: false,
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            no_emit_package: [],
            emit_index_url: false,
            emit_find_links: false,
            emit_build_options: false,
            emit_marker_expression: false,
            emit_index_annotation: false,
            annotation_style: Split,
            link_mode: Clone,
            compile_bytecode: false,
            sources: None,
            hash_checking: Some(
                Verify,
            ),
            upgrade: Upgrade {
                strategy: None,
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    Ok(())
}
