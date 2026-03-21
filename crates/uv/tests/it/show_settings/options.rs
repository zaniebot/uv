use assert_fs::prelude::*;
use uv_static::EnvVars;

use uv_test::uv_snapshot;

use super::add_shared_args;

/// Deserialize an insecure host.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn allow_insecure_host() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        allow-insecure-host = ["google.com", { host = "example.com" }]
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

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
            allow_insecure_host: [
                Host {
                    scheme: None,
                    host: "google.com",
                    port: None,
                },
                Host {
                    scheme: None,
                    host: "example.com",
                    port: None,
                },
            ],
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

    Ok(())
}

/// Prioritize indexes defined across multiple configuration sources.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn index_priority() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [[index]]
        url = "https://file.pypi.org/simple"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("requirements.in")
        .arg("--show-settings")
        .arg("--index-url")
        .arg("https://cli.pypi.org/simple"), @r#"
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
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "cli.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://cli.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Cli,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                    Index {
                        name: None,
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "file.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://file.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: false,
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

    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("requirements.in")
        .arg("--show-settings")
        .arg("--default-index")
        .arg("https://cli.pypi.org/simple"), @r#"
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
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "cli.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://cli.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Cli,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                    Index {
                        name: None,
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "file.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://file.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: false,
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

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        index-url = "https://file.pypi.org/simple"
    "#})?;

    // Prefer the `--default-index` from the CLI, and treat it as the default.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("requirements.in")
        .arg("--show-settings")
        .arg("--default-index")
        .arg("https://cli.pypi.org/simple"), @r#"
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
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "cli.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://cli.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Cli,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                    Index {
                        name: None,
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "file.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://file.pypi.org/simple",
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

    // Prefer the `--index` from the CLI, but treat the index from the file as the default.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("requirements.in")
        .arg("--show-settings")
        .arg("--index")
        .arg("https://cli.pypi.org/simple"), @r#"
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
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "cli.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://cli.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: false,
                        origin: Some(
                            Cli,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                    Index {
                        name: None,
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "file.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://file.pypi.org/simple",
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

    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [[index]]
        url = "https://file.pypi.org/simple"
        default = true
    "#})?;

    // Prefer the `--index-url` from the CLI, and treat it as the default.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("requirements.in")
        .arg("--show-settings")
        .arg("--index-url")
        .arg("https://cli.pypi.org/simple"), @r#"
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
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "cli.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://cli.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: true,
                        origin: Some(
                            Cli,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                    Index {
                        name: None,
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "file.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://file.pypi.org/simple",
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

    // Prefer the `--extra-index-url` from the CLI, but not as the default.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("requirements.in")
        .arg("--show-settings")
        .arg("--extra-index-url")
        .arg("https://cli.pypi.org/simple"), @r#"
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
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "cli.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://cli.pypi.org/simple",
                                ),
                            },
                        ),
                        explicit: false,
                        default: false,
                        origin: Some(
                            Cli,
                        ),
                        format: Simple,
                        publish_url: None,
                        authenticate: Auto,
                        ignore_error_codes: None,
                        cache_control: None,
                    },
                    Index {
                        name: None,
                        url: Url(
                            VerbatimUrl {
                                url: DisplaySafeUrl {
                                    scheme: "https",
                                    cannot_be_a_base: false,
                                    username: "",
                                    password: None,
                                    host: Some(
                                        Domain(
                                            "file.pypi.org",
                                        ),
                                    ),
                                    port: None,
                                    path: "/simple",
                                    query: None,
                                    fragment: None,
                                },
                                given: Some(
                                    "https://file.pypi.org/simple",
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

/// Verify hashes by default.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn verify_hashes() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    uv_snapshot!(context.filters(), add_shared_args(context.pip_install())
        .arg("-r")
        .arg("requirements.in")
        .arg("--show-settings"), @r#"
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
    PipInstallSettings {
        package: [],
        requirements: [
            "requirements.in",
        ],
        editables: [],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        dry_run: Disabled,
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        modifications: Sufficient,
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

    uv_snapshot!(context.filters(), add_shared_args(context.pip_install())
        .arg("-r")
        .arg("requirements.in")
        .arg("--no-verify-hashes")
        .arg("--show-settings"), @r#"
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
    PipInstallSettings {
        package: [],
        requirements: [
            "requirements.in",
        ],
        editables: [],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        dry_run: Disabled,
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        modifications: Sufficient,
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
            hash_checking: None,
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

    uv_snapshot!(context.filters(), add_shared_args(context.pip_install())
        .arg("-r")
        .arg("requirements.in")
        .arg("--require-hashes")
        .arg("--show-settings"), @r#"
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
    PipInstallSettings {
        package: [],
        requirements: [
            "requirements.in",
        ],
        editables: [],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        dry_run: Disabled,
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        modifications: Sufficient,
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
                Require,
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

    uv_snapshot!(context.filters(), add_shared_args(context.pip_install())
        .arg("-r")
        .arg("requirements.in")
        .arg("--no-require-hashes")
        .arg("--show-settings"), @r#"
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
    PipInstallSettings {
        package: [],
        requirements: [
            "requirements.in",
        ],
        editables: [],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        dry_run: Disabled,
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        modifications: Sufficient,
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
            hash_checking: None,
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

    uv_snapshot!(context.filters(), add_shared_args(context.pip_install())
        .arg("-r")
        .arg("requirements.in")
        .env(EnvVars::UV_NO_VERIFY_HASHES, "1")
        .arg("--show-settings"), @r#"
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
    PipInstallSettings {
        package: [],
        requirements: [
            "requirements.in",
        ],
        editables: [],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        dry_run: Disabled,
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        modifications: Sufficient,
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
            hash_checking: None,
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

    uv_snapshot!(context.filters(), add_shared_args(context.pip_install())
        .arg("-r")
        .arg("requirements.in")
        .arg("--verify-hashes")
        .arg("--no-require-hashes")
        .arg("--show-settings"), @r#"
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
    PipInstallSettings {
        package: [],
        requirements: [
            "requirements.in",
        ],
        editables: [],
        constraints: [],
        overrides: [],
        excludes: [],
        build_constraints: [],
        dry_run: Disabled,
        constraints_from_workspace: [],
        overrides_from_workspace: [],
        excludes_from_workspace: [],
        build_constraints_from_workspace: [],
        modifications: Sufficient,
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

    Ok(())
}

/// Test preview feature flagging.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn preview_features() {
    let context = uv_test::test_context!("3.12");

    let cmd = || {
        let mut cmd = context.version();
        cmd.arg("--show-settings");
        add_shared_args(cmd)
    };

    uv_snapshot!(context.filters(), cmd().arg("--preview"), @r#"
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
            flags: [
                PythonInstallDefault,
                PythonUpgrade,
                JsonOutput,
                Pylock,
                AddBounds,
                PackageConflicts,
                ExtraBuildDependencies,
                DetectModuleConflicts,
                Format,
                NativeAuth,
                S3Endpoint,
                CacheSize,
                InitProjectFlag,
                WorkspaceMetadata,
                WorkspaceDir,
                WorkspaceList,
                SbomExport,
                AuthHelper,
                DirectPublish,
                TargetWorkspaceDiscovery,
                MetadataJson,
                GcsEndpoint,
                AdjustUlimit,
                SpecialCondaEnvNames,
                RelocatableEnvsDefault,
                PublishRequireNormalized,
                Audit,
                ProjectDirectoryMustExist,
            ],
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
    VersionSettings {
        value: None,
        bump: [],
        short: false,
        output_format: Text,
        dry_run: false,
        lock_check: Disabled,
        frozen: None,
        active: None,
        no_sync: false,
        package: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverInstallerSettings {
            resolver: ResolverSettings {
                build_options: BuildOptions {
                    no_binary: None,
                    no_build: None,
                },
                config_setting: ConfigSettings(
                    {},
                ),
                config_settings_package: PackageConfigSettings(
                    {},
                ),
                dependency_metadata: DependencyMetadata(
                    {},
                ),
                exclude_newer: ExcludeNewer {
                    global: None,
                    package: ExcludeNewerPackage(
                        {},
                    ),
                },
                fork_strategy: RequiresPython,
                index_locations: IndexLocations {
                    indexes: [],
                    flat_index: [],
                    no_index: false,
                },
                index_strategy: FirstIndex,
                keyring_provider: Disabled,
                link_mode: Clone,
                build_isolation: Isolate,
                extra_build_dependencies: ExtraBuildDependencies(
                    {},
                ),
                extra_build_variables: ExtraBuildVariables(
                    {},
                ),
                prerelease: IfNecessaryOrExplicit,
                resolution: Highest,
                sources: None,
                torch_backend: None,
                upgrade: Upgrade {
                    strategy: None,
                    constraints: {},
                },
            },
            compile_bytecode: false,
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    uv_snapshot!(context.filters(), cmd().arg("--preview").arg("--no-preview"), @r#"
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
    VersionSettings {
        value: None,
        bump: [],
        short: false,
        output_format: Text,
        dry_run: false,
        lock_check: Disabled,
        frozen: None,
        active: None,
        no_sync: false,
        package: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverInstallerSettings {
            resolver: ResolverSettings {
                build_options: BuildOptions {
                    no_binary: None,
                    no_build: None,
                },
                config_setting: ConfigSettings(
                    {},
                ),
                config_settings_package: PackageConfigSettings(
                    {},
                ),
                dependency_metadata: DependencyMetadata(
                    {},
                ),
                exclude_newer: ExcludeNewer {
                    global: None,
                    package: ExcludeNewerPackage(
                        {},
                    ),
                },
                fork_strategy: RequiresPython,
                index_locations: IndexLocations {
                    indexes: [],
                    flat_index: [],
                    no_index: false,
                },
                index_strategy: FirstIndex,
                keyring_provider: Disabled,
                link_mode: Clone,
                build_isolation: Isolate,
                extra_build_dependencies: ExtraBuildDependencies(
                    {},
                ),
                extra_build_variables: ExtraBuildVariables(
                    {},
                ),
                prerelease: IfNecessaryOrExplicit,
                resolution: Highest,
                sources: None,
                torch_backend: None,
                upgrade: Upgrade {
                    strategy: None,
                    constraints: {},
                },
            },
            compile_bytecode: false,
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    uv_snapshot!(context.filters(), cmd().arg("--preview").arg("--preview-features").arg("python-install-default"), @r#"
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
            flags: [
                PythonInstallDefault,
                PythonUpgrade,
                JsonOutput,
                Pylock,
                AddBounds,
                PackageConflicts,
                ExtraBuildDependencies,
                DetectModuleConflicts,
                Format,
                NativeAuth,
                S3Endpoint,
                CacheSize,
                InitProjectFlag,
                WorkspaceMetadata,
                WorkspaceDir,
                WorkspaceList,
                SbomExport,
                AuthHelper,
                DirectPublish,
                TargetWorkspaceDiscovery,
                MetadataJson,
                GcsEndpoint,
                AdjustUlimit,
                SpecialCondaEnvNames,
                RelocatableEnvsDefault,
                PublishRequireNormalized,
                Audit,
                ProjectDirectoryMustExist,
            ],
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
    VersionSettings {
        value: None,
        bump: [],
        short: false,
        output_format: Text,
        dry_run: false,
        lock_check: Disabled,
        frozen: None,
        active: None,
        no_sync: false,
        package: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverInstallerSettings {
            resolver: ResolverSettings {
                build_options: BuildOptions {
                    no_binary: None,
                    no_build: None,
                },
                config_setting: ConfigSettings(
                    {},
                ),
                config_settings_package: PackageConfigSettings(
                    {},
                ),
                dependency_metadata: DependencyMetadata(
                    {},
                ),
                exclude_newer: ExcludeNewer {
                    global: None,
                    package: ExcludeNewerPackage(
                        {},
                    ),
                },
                fork_strategy: RequiresPython,
                index_locations: IndexLocations {
                    indexes: [],
                    flat_index: [],
                    no_index: false,
                },
                index_strategy: FirstIndex,
                keyring_provider: Disabled,
                link_mode: Clone,
                build_isolation: Isolate,
                extra_build_dependencies: ExtraBuildDependencies(
                    {},
                ),
                extra_build_variables: ExtraBuildVariables(
                    {},
                ),
                prerelease: IfNecessaryOrExplicit,
                resolution: Highest,
                sources: None,
                torch_backend: None,
                upgrade: Upgrade {
                    strategy: None,
                    constraints: {},
                },
            },
            compile_bytecode: false,
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    uv_snapshot!(context.filters(), cmd().arg("--preview-features").arg("python-install-default,python-upgrade"), @r#"
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
            flags: [
                PythonInstallDefault,
                PythonUpgrade,
            ],
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
    VersionSettings {
        value: None,
        bump: [],
        short: false,
        output_format: Text,
        dry_run: false,
        lock_check: Disabled,
        frozen: None,
        active: None,
        no_sync: false,
        package: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverInstallerSettings {
            resolver: ResolverSettings {
                build_options: BuildOptions {
                    no_binary: None,
                    no_build: None,
                },
                config_setting: ConfigSettings(
                    {},
                ),
                config_settings_package: PackageConfigSettings(
                    {},
                ),
                dependency_metadata: DependencyMetadata(
                    {},
                ),
                exclude_newer: ExcludeNewer {
                    global: None,
                    package: ExcludeNewerPackage(
                        {},
                    ),
                },
                fork_strategy: RequiresPython,
                index_locations: IndexLocations {
                    indexes: [],
                    flat_index: [],
                    no_index: false,
                },
                index_strategy: FirstIndex,
                keyring_provider: Disabled,
                link_mode: Clone,
                build_isolation: Isolate,
                extra_build_dependencies: ExtraBuildDependencies(
                    {},
                ),
                extra_build_variables: ExtraBuildVariables(
                    {},
                ),
                prerelease: IfNecessaryOrExplicit,
                resolution: Highest,
                sources: None,
                torch_backend: None,
                upgrade: Upgrade {
                    strategy: None,
                    constraints: {},
                },
            },
            compile_bytecode: false,
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    uv_snapshot!(context.filters(), cmd().arg("--preview-features").arg("python-install-default").arg("--preview-feature").arg("python-upgrade"), @r#"
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
            flags: [
                PythonInstallDefault,
                PythonUpgrade,
            ],
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
    VersionSettings {
        value: None,
        bump: [],
        short: false,
        output_format: Text,
        dry_run: false,
        lock_check: Disabled,
        frozen: None,
        active: None,
        no_sync: false,
        package: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverInstallerSettings {
            resolver: ResolverSettings {
                build_options: BuildOptions {
                    no_binary: None,
                    no_build: None,
                },
                config_setting: ConfigSettings(
                    {},
                ),
                config_settings_package: PackageConfigSettings(
                    {},
                ),
                dependency_metadata: DependencyMetadata(
                    {},
                ),
                exclude_newer: ExcludeNewer {
                    global: None,
                    package: ExcludeNewerPackage(
                        {},
                    ),
                },
                fork_strategy: RequiresPython,
                index_locations: IndexLocations {
                    indexes: [],
                    flat_index: [],
                    no_index: false,
                },
                index_strategy: FirstIndex,
                keyring_provider: Disabled,
                link_mode: Clone,
                build_isolation: Isolate,
                extra_build_dependencies: ExtraBuildDependencies(
                    {},
                ),
                extra_build_variables: ExtraBuildVariables(
                    {},
                ),
                prerelease: IfNecessaryOrExplicit,
                resolution: Highest,
                sources: None,
                torch_backend: None,
                upgrade: Upgrade {
                    strategy: None,
                    constraints: {},
                },
            },
            compile_bytecode: false,
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    uv_snapshot!(context.filters(), cmd()
        .arg("--preview-features").arg("python-install-default").arg("--preview-feature").arg("python-upgrade")
        .arg("--no-preview"), @r#"
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
    VersionSettings {
        value: None,
        bump: [],
        short: false,
        output_format: Text,
        dry_run: false,
        lock_check: Disabled,
        frozen: None,
        active: None,
        no_sync: false,
        package: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverInstallerSettings {
            resolver: ResolverSettings {
                build_options: BuildOptions {
                    no_binary: None,
                    no_build: None,
                },
                config_setting: ConfigSettings(
                    {},
                ),
                config_settings_package: PackageConfigSettings(
                    {},
                ),
                dependency_metadata: DependencyMetadata(
                    {},
                ),
                exclude_newer: ExcludeNewer {
                    global: None,
                    package: ExcludeNewerPackage(
                        {},
                    ),
                },
                fork_strategy: RequiresPython,
                index_locations: IndexLocations {
                    indexes: [],
                    flat_index: [],
                    no_index: false,
                },
                index_strategy: FirstIndex,
                keyring_provider: Disabled,
                link_mode: Clone,
                build_isolation: Isolate,
                extra_build_dependencies: ExtraBuildDependencies(
                    {},
                ),
                extra_build_variables: ExtraBuildVariables(
                    {},
                ),
                prerelease: IfNecessaryOrExplicit,
                resolution: Highest,
                sources: None,
                torch_backend: None,
                upgrade: Upgrade {
                    strategy: None,
                    constraints: {},
                },
            },
            compile_bytecode: false,
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );
}

/// Track the interactions between `upgrade` and `upgrade-package` across the `uv pip` CLI and a
/// configuration file.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn upgrade_pip_cli_config_interaction() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    // `--no-upgrade` overrides `--upgrade-package`.
    // TODO(charlie): This should mark `sniffio` for upgrade, but it doesn't.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--no-upgrade")
        .arg("--upgrade-package")
        .arg("sniffio")
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
                strategy: Packages(
                    {
                        PackageName(
                            "sniffio",
                        ),
                    },
                ),
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r"
        [pip]
        upgrade = false
    "})?;

    // Despite `upgrade = false` in the configuration file, we should mark `idna` for upgrade.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--upgrade-package")
        .arg("idna")
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

    // Write a `uv.toml` file to the directory.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r"
        [pip]
        upgrade = true
    "})?;

    // Despite `--upgrade-package idna` in the command line, we should upgrade all packages.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--upgrade-package")
        .arg("idna")
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
                strategy: All,
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Write a `uv.toml` file to the directory.
    config.write_str(indoc::indoc! {r#"
        [pip]
        upgrade-package = ["idna"]
    "#})?;

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should disable upgrades.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--no-upgrade")
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
                strategy: Packages(
                    {
                        PackageName(
                            "idna",
                        ),
                    },
                ),
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should enable all upgrades.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--upgrade")
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
                strategy: Packages(
                    {
                        PackageName(
                            "idna",
                        ),
                    },
                ),
                constraints: {},
            },
            reinstall: None,
        },
    }

    ----- stderr -----
    "#
    );

    // Mark both `sniffio` and `idna` for upgrade.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--upgrade-package")
        .arg("sniffio")
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
                strategy: Packages(
                    {
                        PackageName(
                            "sniffio",
                        ),
                        PackageName(
                            "idna",
                        ),
                    },
                ),
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

/// Track the interactions between `upgrade` and `upgrade-package` across the project CLI and a
/// configuration file.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn upgrade_project_cli_config_interaction() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]
    "#})?;

    // `--no-upgrade` overrides `--upgrade-package`.
    // TODO(charlie): This should mark `sniffio` for upgrade, but it doesn't.
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .arg("--no-upgrade")
        .arg("--upgrade-package")
        .arg("sniffio")
        .arg("--show-settings"), @r#"
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
    LockSettings {
        lock_check: Disabled,
        frozen: None,
        dry_run: Disabled,
        script: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverSettings {
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            dependency_metadata: DependencyMetadata(
                {},
            ),
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            fork_strategy: RequiresPython,
            index_locations: IndexLocations {
                indexes: [],
                flat_index: [],
                no_index: false,
            },
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            link_mode: Clone,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            prerelease: IfNecessaryOrExplicit,
            resolution: Highest,
            sources: None,
            torch_backend: None,
            upgrade: Upgrade {
                strategy: Packages(
                    {
                        PackageName(
                            "sniffio",
                        ),
                    },
                ),
                constraints: {},
            },
        },
    }

    ----- stderr -----
    "#
    );

    // Add `upgrade = false` to the configuration file.
    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]

        [tool.uv]
        upgrade = false
    "#})?;

    // Despite `upgrade = false` in the configuration file, we should mark `idna` for upgrade.
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .arg("--upgrade-package")
        .arg("idna")
        .arg("--show-settings"), @r#"
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
    LockSettings {
        lock_check: Disabled,
        frozen: None,
        dry_run: Disabled,
        script: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverSettings {
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            dependency_metadata: DependencyMetadata(
                {},
            ),
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            fork_strategy: RequiresPython,
            index_locations: IndexLocations {
                indexes: [],
                flat_index: [],
                no_index: false,
            },
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            link_mode: Clone,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            prerelease: IfNecessaryOrExplicit,
            resolution: Highest,
            sources: None,
            torch_backend: None,
            upgrade: Upgrade {
                strategy: None,
                constraints: {},
            },
        },
    }

    ----- stderr -----
    "#
    );

    // Add `upgrade = true` to the configuration file.
    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]

        [tool.uv]
        upgrade = true
    "#})?;

    // Despite `--upgrade-package idna` on the CLI, we should upgrade all packages.
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .arg("--upgrade-package")
        .arg("idna")
        .arg("--show-settings"), @r#"
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
    LockSettings {
        lock_check: Disabled,
        frozen: None,
        dry_run: Disabled,
        script: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverSettings {
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            dependency_metadata: DependencyMetadata(
                {},
            ),
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            fork_strategy: RequiresPython,
            index_locations: IndexLocations {
                indexes: [],
                flat_index: [],
                no_index: false,
            },
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            link_mode: Clone,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            prerelease: IfNecessaryOrExplicit,
            resolution: Highest,
            sources: None,
            torch_backend: None,
            upgrade: Upgrade {
                strategy: All,
                constraints: {},
            },
        },
    }

    ----- stderr -----
    "#
    );

    pyproject_toml.write_str(indoc::indoc! {r#"
        [project]
        name = "foo"
        version = "0.0.0"
        dependencies = ["anyio>3.0.0"]

        [tool.uv]
        upgrade-package = ["idna"]
    "#})?;

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should disable upgrades.
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .arg("--no-upgrade")
        .arg("--show-settings"), @r#"
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
    LockSettings {
        lock_check: Disabled,
        frozen: None,
        dry_run: Disabled,
        script: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverSettings {
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            dependency_metadata: DependencyMetadata(
                {},
            ),
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            fork_strategy: RequiresPython,
            index_locations: IndexLocations {
                indexes: [],
                flat_index: [],
                no_index: false,
            },
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            link_mode: Clone,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            prerelease: IfNecessaryOrExplicit,
            resolution: Highest,
            sources: None,
            torch_backend: None,
            upgrade: Upgrade {
                strategy: Packages(
                    {
                        PackageName(
                            "idna",
                        ),
                    },
                ),
                constraints: {},
            },
        },
    }

    ----- stderr -----
    "#
    );

    // Despite `upgrade-package = ["idna"]` in the configuration file, we should enable all upgrades.
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .arg("--upgrade")
        .arg("--show-settings"), @r#"
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
    LockSettings {
        lock_check: Disabled,
        frozen: None,
        dry_run: Disabled,
        script: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverSettings {
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            dependency_metadata: DependencyMetadata(
                {},
            ),
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            fork_strategy: RequiresPython,
            index_locations: IndexLocations {
                indexes: [],
                flat_index: [],
                no_index: false,
            },
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            link_mode: Clone,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            prerelease: IfNecessaryOrExplicit,
            resolution: Highest,
            sources: None,
            torch_backend: None,
            upgrade: Upgrade {
                strategy: Packages(
                    {
                        PackageName(
                            "idna",
                        ),
                    },
                ),
                constraints: {},
            },
        },
    }

    ----- stderr -----
    "#
    );

    // Mark both `sniffio` and `idna` for upgrade.
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .arg("--upgrade-package")
        .arg("sniffio")
        .arg("--show-settings"), @r#"
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
    LockSettings {
        lock_check: Disabled,
        frozen: None,
        dry_run: Disabled,
        script: None,
        python: None,
        install_mirrors: PythonInstallMirrors {
            python_install_mirror: None,
            pypy_install_mirror: None,
            python_downloads_json_url: None,
        },
        refresh: None(
            Timestamp(
                SystemTime {
                    tv_sec: [TIME],
                    tv_nsec: [TIME],
                },
            ),
        ),
        settings: ResolverSettings {
            build_options: BuildOptions {
                no_binary: None,
                no_build: None,
            },
            config_setting: ConfigSettings(
                {},
            ),
            config_settings_package: PackageConfigSettings(
                {},
            ),
            dependency_metadata: DependencyMetadata(
                {},
            ),
            exclude_newer: ExcludeNewer {
                global: None,
                package: ExcludeNewerPackage(
                    {},
                ),
            },
            fork_strategy: RequiresPython,
            index_locations: IndexLocations {
                indexes: [],
                flat_index: [],
                no_index: false,
            },
            index_strategy: FirstIndex,
            keyring_provider: Disabled,
            link_mode: Clone,
            build_isolation: Isolate,
            extra_build_dependencies: ExtraBuildDependencies(
                {},
            ),
            extra_build_variables: ExtraBuildVariables(
                {},
            ),
            prerelease: IfNecessaryOrExplicit,
            resolution: Highest,
            sources: None,
            torch_backend: None,
            upgrade: Upgrade {
                strategy: Packages(
                    {
                        PackageName(
                            "sniffio",
                        ),
                        PackageName(
                            "idna",
                        ),
                    },
                ),
                constraints: {},
            },
        },
    }

    ----- stderr -----
    "#
    );

    Ok(())
}

/// Test that setting `build-isolation = true` in pyproject.toml followed by
/// `--no-build-isolation-package numpy` on the CLI disables build isolation for `numpy`.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn build_isolation_override() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` file to disable build isolation.
    let uv_toml = context.temp_dir.child("uv.toml");
    uv_toml.write_str(indoc::indoc! {r"
        no-build-isolation = true
    "})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("numpy")?;

    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in")
        .arg("--no-build-isolation-package").arg("numpy"), @r#"
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
            build_isolation: Shared,
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
    "#);

    // Now enable build isolation for all packages except `numpy`.
    uv_toml.write_str(indoc::indoc! {r"
        no-build-isolation = false
    "})?;

    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in")
        .arg("--no-build-isolation-package").arg("numpy"), @r#"
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
            build_isolation: SharedPackage(
                [
                    PackageName(
                        "numpy",
                    ),
                ],
            ),
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
    "#);

    Ok(())
}
