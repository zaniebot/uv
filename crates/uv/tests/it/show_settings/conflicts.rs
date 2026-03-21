use assert_fs::prelude::*;
use uv_static::EnvVars;

use uv_test::uv_snapshot;

use super::add_shared_args;

/// Tests that errors when parsing `conflicts` are reported.
#[test]
fn invalid_conflicts() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");
    let pyproject = context.temp_dir.child("pyproject.toml");

    // Write in `pyproject.toml` schema and test the singleton case.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
        requires-python = ">=3.12"

        [tool.uv]
        conflicts = [
            [{extra = "dev"}],
        ]
    "#})?;

    // The file should be rejected for violating the schema.
    uv_snapshot!(context.filters(), add_shared_args(context.lock()), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `pyproject.toml`
      Caused by: TOML parse error at line 7, column 13
      |
    7 | conflicts = [
      |             ^
    Each set of conflicts must have at least two entries, but found only one
    "
    );

    // Now test the empty case.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
        requires-python = ">=3.12"

        [tool.uv]
        conflicts = [[]]
    "#})?;

    // The file should be rejected for violating the schema.
    uv_snapshot!(context.filters(), add_shared_args(context.lock()), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `pyproject.toml`
      Caused by: TOML parse error at line 7, column 13
      |
    7 | conflicts = [[]]
      |             ^^^^
    Each set of conflicts must have at least two entries, but found none
    "
    );

    Ok(())
}

/// Tests that valid `conflicts` are parsed okay.
#[test]
fn valid_conflicts() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");
    let xdg = assert_fs::TempDir::new().expect("Failed to create temp dir");
    let pyproject = context.temp_dir.child("pyproject.toml");

    // Write in `pyproject.toml` schema.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"
        requires-python = ">=3.12"

        [tool.uv]
        conflicts = [
            [{extra = "x1"}, {extra = "x2"}],
        ]
    "#})?;
    uv_snapshot!(context.filters(), add_shared_args(context.lock())
        .env(EnvVars::XDG_CONFIG_HOME, xdg.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    "
    );

    Ok(())
}

/// Read from a `--config-file` command line argument.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_config_file() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Write a `uv.toml` to a temporary location. (Use the cache directory for convenience, since
    // it's already obfuscated in the fixtures.)
    let config_dir = &context.cache_dir;
    let config = config_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str("anyio>3.0.0")?;

    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("--config-file")
        .arg(config.path())
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

    // Write in `pyproject.toml` schema.
    config.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
    "#})?;

    // The file should be rejected for violating the schema.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("--config-file")
        .arg(config.path())
        .arg("requirements.in"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse: `[CACHE_DIR]/uv.toml`
      Caused by: TOML parse error at line 1, column 2
      |
    1 | [project]
      |  ^^^^^^^
    unknown field `project`, expected one of `required-version`, `native-tls`, `offline`, `no-cache`, `cache-dir`, `preview`, `python-preference`, `python-downloads`, `concurrent-downloads`, `concurrent-builds`, `concurrent-installs`, `index`, `index-url`, `extra-index-url`, `no-index`, `find-links`, `index-strategy`, `keyring-provider`, `http-proxy`, `https-proxy`, `no-proxy`, `allow-insecure-host`, `resolution`, `prerelease`, `fork-strategy`, `dependency-metadata`, `config-settings`, `config-settings-package`, `no-build-isolation`, `no-build-isolation-package`, `extra-build-dependencies`, `extra-build-variables`, `exclude-newer`, `exclude-newer-package`, `link-mode`, `compile-bytecode`, `no-sources`, `no-sources-package`, `upgrade`, `upgrade-package`, `reinstall`, `reinstall-package`, `no-build`, `no-build-package`, `no-binary`, `no-binary-package`, `torch-backend`, `python-install-mirror`, `pypy-install-mirror`, `python-downloads-json-url`, `publish-url`, `trusted-publishing`, `check-url`, `add-bounds`, `pip`, `cache-keys`, `override-dependencies`, `exclude-dependencies`, `constraint-dependencies`, `build-constraint-dependencies`, `environments`, `required-environments`, `conflicts`, `workspace`, `sources`, `managed`, `package`, `default-groups`, `dependency-groups`, `dev-dependencies`, `build-backend`
    "
    );

    // Write an _actual_ `pyproject.toml`.
    let config = config_dir.child("pyproject.toml");
    config.write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.0.0"

        [tool.uv.pip]
        resolution = "lowest-direct"
        generate-hashes = true
        index-url = "https://pypi.org/simple"
        """#
    })?;

    // The file should be rejected for violating the schema, with a custom warning.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("--config-file")
        .arg(config.path())
        .arg("requirements.in"), @r#"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    warning: The `--config-file` argument expects to receive a `uv.toml` file, not a `pyproject.toml`. If you're trying to run a command from another project, use the `--project` argument instead.
    error: Failed to parse: `[CACHE_DIR]/pyproject.toml`
      Caused by: TOML parse error at line 9, column 3
      |
    9 | ""
      |   ^
    key with no value, expected `=`
    "#
    );

    Ok(())
}

/// Ignore empty `pyproject.toml` files when discovering configuration.
#[test]
#[cfg_attr(
    windows,
    ignore = "Configuration tests are not yet supported on Windows"
)]
fn resolve_skip_empty() -> anyhow::Result<()> {
    let context = uv_test::test_context!("3.12");

    // Set `lowest-direct` in a `uv.toml`.
    let config = context.temp_dir.child("uv.toml");
    config.write_str(indoc::indoc! {r#"
        [pip]
        resolution = "lowest-direct"
    "#})?;

    let child = context.temp_dir.child("child");
    fs_err::create_dir(&child)?;

    // Create an empty in a `pyproject.toml`.
    let pyproject = child.child("pyproject.toml");
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "child"
        dependencies = [
          "httpx",
        ]
    "#})?;

    // Resolution in `child` should use lowest-direct, skipping the `pyproject.toml`, which lacks a
    // `tool.uv`.
    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in")
        .current_dir(&child), @r#"
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

    // Adding a `tool.uv` section should cause us to ignore the `uv.toml`.
    pyproject.write_str(indoc::indoc! {r#"
        [project]
        name = "child"
        dependencies = [
          "httpx",
        ]

        [tool.uv]
    "#})?;

    uv_snapshot!(context.filters(), add_shared_args(context.pip_compile())
        .arg("--show-settings")
        .arg("requirements.in")
        .current_dir(&child), @r#"
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

    Ok(())
}
