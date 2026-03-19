use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main, measurement::WallTime};
use uv_cache::Cache;
use uv_client::{BaseClientBuilder, RegistryClientBuilder};
use uv_distribution_types::Requirement;
use uv_python::PythonEnvironment;
use uv_resolver::Manifest;

fn resolve_warm_jupyter(c: &mut Criterion<WallTime>) {
    let run = setup(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("jupyter==1.0.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_jupyter", |b| b.iter(|| run(false)));
}

fn resolve_warm_jupyter_universal(c: &mut Criterion<WallTime>) {
    let run = setup(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("jupyter==1.0.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_jupyter_universal", |b| b.iter(|| run(true)));
}

fn resolve_warm_airflow(c: &mut Criterion<WallTime>) {
    let run = setup(Manifest::simple(vec![
        Requirement::from(uv_pep508::Requirement::from_str("apache-airflow[all]==2.9.3").unwrap()),
        Requirement::from(
            uv_pep508::Requirement::from_str("apache-airflow-providers-apache-beam>3.0.0").unwrap(),
        ),
    ]));
    c.bench_function("resolve_warm_airflow", |b| b.iter(|| run(false)));
}

// This takes >5m to run in CodSpeed.
// fn resolve_warm_airflow_universal(c: &mut Criterion<WallTime>) {
//     let run = setup(Manifest::simple(vec![
//         Requirement::from(uv_pep508::Requirement::from_str("apache-airflow[all]").unwrap()),
//         Requirement::from(
//             uv_pep508::Requirement::from_str("apache-airflow-providers-apache-beam>3.0.0").unwrap(),
//         ),
//     ]));
//     c.bench_function("resolve_warm_airflow_universal", |b| b.iter(|| run(true)));
// }

/// Benchmark `lock --check` with a valid lockfile for `jupyter==1.0.0`.
///
/// This runs the actual `uv lock --check` command against a project with an
/// already-valid lockfile, exercising the real code path end-to-end: workspace
/// discovery, lockfile parsing, `Lock::satisfies()` validation, and early return.
fn lock_check_jupyter(c: &mut Criterion<WallTime>) {
    let uv = uv_bin();

    // Create a temporary project with a `pyproject.toml`.
    let temp_dir = tempfile::tempdir().unwrap();
    let cache_dir = temp_dir.path().join("cache");
    let project_dir = temp_dir.path().join("project");
    fs_err::create_dir_all(&project_dir).unwrap();

    fs_err::write(
        project_dir.join("pyproject.toml"),
        indoc::indoc! {r#"
            [project]
            name = "bench"
            version = "0.1.0"
            requires-python = ">=3.12"
            dependencies = ["jupyter==1.0.0"]
        "#},
    )
    .unwrap();

    // Run `uv lock` once to produce a valid lockfile.
    let status = Command::new(&uv)
        .args(["lock", "--cache-dir"])
        .arg(&cache_dir)
        .arg("--directory")
        .arg(&project_dir)
        .status()
        .unwrap();
    assert!(status.success(), "initial `uv lock` failed");

    c.bench_function("lock_check_jupyter", |b| {
        b.iter(|| {
            let output = Command::new(black_box(&uv))
                .args(["lock", "--check", "--cache-dir"])
                .arg(&cache_dir)
                .arg("--directory")
                .arg(&project_dir)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "`uv lock --check` failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        });
    });
}

/// Benchmark `lock --check` with a valid lockfile for `apache-airflow[all]==2.9.3`.
///
/// This exercises a much larger lockfile than jupyter, providing a benchmark for
/// `lock --check` performance with complex dependency trees.
fn lock_check_airflow(c: &mut Criterion<WallTime>) {
    let uv = uv_bin();

    // Create a temporary project with a `pyproject.toml`.
    let temp_dir = tempfile::tempdir().unwrap();
    let cache_dir = temp_dir.path().join("cache");
    let project_dir = temp_dir.path().join("project");
    fs_err::create_dir_all(&project_dir).unwrap();

    fs_err::write(
        project_dir.join("pyproject.toml"),
        indoc::indoc! {r#"
            [project]
            name = "bench"
            version = "0.1.0"
            requires-python = ">=3.12"
            dependencies = [
                "apache-airflow[all]==2.9.3",
                "apache-airflow-providers-apache-beam>3.0.0",
            ]
        "#},
    )
    .unwrap();

    // Run `uv lock` once to produce a valid lockfile.
    let status = Command::new(&uv)
        .args(["lock", "--cache-dir"])
        .arg(&cache_dir)
        .arg("--directory")
        .arg(&project_dir)
        .status()
        .unwrap();
    assert!(status.success(), "initial `uv lock` failed");

    c.bench_function("lock_check_airflow", |b| {
        b.iter(|| {
            let output = Command::new(black_box(&uv))
                .args(["lock", "--check", "--cache-dir"])
                .arg(&cache_dir)
                .arg("--directory")
                .arg(&project_dir)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "`uv lock --check` failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        });
    });
}

/// Locate the `uv` binary to use for benchmarks.
///
/// Checks the `UV_BIN` environment variable first, then falls back to the release
/// binary in the target directory.
fn uv_bin() -> PathBuf {
    if let Ok(bin) = std::env::var("UV_BIN") {
        let path = PathBuf::from(bin);
        assert!(path.exists(), "UV_BIN does not exist: {}", path.display());
        return path;
    }

    let release = PathBuf::from("../../target/release/uv");
    assert!(
        release.exists(),
        "uv binary not found at {}; build with `cargo build --release` or set UV_BIN",
        release.display()
    );
    release
}

criterion_group!(
    uv,
    resolve_warm_jupyter,
    resolve_warm_jupyter_universal,
    resolve_warm_airflow,
    lock_check_jupyter,
    lock_check_airflow
);
criterion_main!(uv);

fn setup(manifest: Manifest) -> impl Fn(bool) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        // CodSpeed limits the total number of threads to 500
        .max_blocking_threads(256)
        .enable_all()
        .build()
        .unwrap();

    let cache = Cache::from_path("../../.cache")
        .init_no_wait()
        .expect("No cache contention when running benchmarks")
        .unwrap();
    let interpreter = PythonEnvironment::from_root("../../.venv", &cache)
        .unwrap()
        .into_interpreter();
    let client = RegistryClientBuilder::new(BaseClientBuilder::default(), cache.clone()).build();

    move |universal| {
        runtime
            .block_on(resolver::resolve(
                black_box(manifest.clone()),
                black_box(cache.clone()),
                black_box(&client),
                &interpreter,
                universal,
            ))
            .unwrap();
    }
}

mod resolver {
    use std::sync::LazyLock;

    use anyhow::Result;

    use uv_cache::Cache;
    use uv_client::RegistryClient;
    use uv_configuration::{BuildOptions, Concurrency, Constraints, IndexStrategy, NoSources};
    use uv_dispatch::{BuildDispatch, SharedState};
    use uv_distribution::DistributionDatabase;
    use uv_distribution_types::{
        ConfigSettings, DependencyMetadata, ExtraBuildRequires, ExtraBuildVariables,
        IndexLocations, PackageConfigSettings, RequiresPython,
    };
    use uv_install_wheel::LinkMode;
    use uv_pep440::Version;
    use uv_pep508::{MarkerEnvironment, MarkerEnvironmentBuilder};
    use uv_platform_tags::{Arch, Os, Platform, Tags};
    use uv_preview::Preview;
    use uv_pypi_types::{Conflicts, ResolverMarkerEnvironment};
    use uv_python::Interpreter;
    use uv_resolver::{
        ExcludeNewer, FlatIndex, InMemoryIndex, Manifest, OptionsBuilder, PythonRequirement,
        Resolver, ResolverEnvironment, ResolverOutput,
    };
    use uv_types::{BuildIsolation, EmptyInstalledPackages, HashStrategy};
    use uv_workspace::WorkspaceCache;

    static MARKERS: LazyLock<MarkerEnvironment> = LazyLock::new(|| {
        MarkerEnvironment::try_from(MarkerEnvironmentBuilder {
            implementation_name: "cpython",
            implementation_version: "3.11.5",
            os_name: "posix",
            platform_machine: "arm64",
            platform_python_implementation: "CPython",
            platform_release: "21.6.0",
            platform_system: "Darwin",
            platform_version: "Darwin Kernel Version 21.6.0: Mon Aug 22 20:19:52 PDT 2022; root:xnu-8020.140.49~2/RELEASE_ARM64_T6000",
            python_full_version: "3.11.5",
            python_version: "3.11",
            sys_platform: "darwin",
        }).unwrap()
    });

    static PLATFORM: Platform = Platform::new(
        Os::Macos {
            major: 21,
            minor: 6,
        },
        Arch::Aarch64,
    );

    static TAGS: LazyLock<Tags> = LazyLock::new(|| {
        Tags::from_env(&PLATFORM, (3, 11), "cpython", (3, 11), false, false, false).unwrap()
    });

    pub(crate) async fn resolve(
        manifest: Manifest,
        cache: Cache,
        client: &RegistryClient,
        interpreter: &Interpreter,
        universal: bool,
    ) -> Result<ResolverOutput> {
        let build_isolation = BuildIsolation::default();
        let extra_build_requires = ExtraBuildRequires::default();
        let extra_build_variables = ExtraBuildVariables::default();
        let build_options = BuildOptions::default();
        let concurrency = Concurrency::default();
        let config_settings = ConfigSettings::default();
        let config_settings_package = PackageConfigSettings::default();
        let exclude_newer = ExcludeNewer::global(
            jiff::civil::date(2024, 9, 1)
                .to_zoned(jiff::tz::TimeZone::UTC)
                .unwrap()
                .timestamp()
                .into(),
        );
        let build_constraints = Constraints::default();
        let flat_index = FlatIndex::default();
        let hashes = HashStrategy::default();
        let state = SharedState::default();
        let index = InMemoryIndex::default();
        let index_locations = IndexLocations::default();
        let installed_packages = EmptyInstalledPackages;
        let options = OptionsBuilder::new()
            .exclude_newer(exclude_newer.clone())
            .build();
        let sources = NoSources::default();
        let dependency_metadata = DependencyMetadata::default();
        let conflicts = Conflicts::empty();
        let workspace_cache = WorkspaceCache::default();

        let python_requirement = if universal {
            PythonRequirement::from_requires_python(
                interpreter,
                RequiresPython::greater_than_equal_version(&Version::new([3, 11])),
            )
        } else {
            PythonRequirement::from_interpreter(interpreter)
        };

        let build_context = BuildDispatch::new(
            client,
            &cache,
            &build_constraints,
            interpreter,
            &index_locations,
            &flat_index,
            &dependency_metadata,
            state,
            IndexStrategy::default(),
            &config_settings,
            &config_settings_package,
            build_isolation,
            &extra_build_requires,
            &extra_build_variables,
            LinkMode::default(),
            &build_options,
            &hashes,
            exclude_newer,
            sources,
            workspace_cache,
            concurrency.clone(),
            Preview::default(),
        );

        let markers = if universal {
            ResolverEnvironment::universal(vec![])
        } else {
            ResolverEnvironment::specific(ResolverMarkerEnvironment::from(MARKERS.clone()))
        };

        let resolver = Resolver::new(
            manifest,
            options,
            &python_requirement,
            markers,
            interpreter.markers(),
            conflicts,
            Some(&TAGS),
            &flat_index,
            &index,
            &hashes,
            &build_context,
            installed_packages,
            DistributionDatabase::new(
                client,
                &build_context,
                concurrency.downloads_semaphore.clone(),
            ),
        )?;

        Ok(resolver.resolve().await?)
    }
}
