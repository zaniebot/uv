use std::hint::black_box;
use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main, measurement::WallTime};
use uv_cache::Cache;
use uv_client::{BaseClientBuilder, RegistryClientBuilder};
use uv_distribution_types::Requirement;
use uv_python::PythonEnvironment;
use uv_resolver::Manifest;

// =============================================================================
// Existing resolution benchmarks
// =============================================================================

fn resolve_warm_jupyter(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("jupyter==1.0.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_jupyter", |b| b.iter(|| run(false)));
}

fn resolve_warm_jupyter_universal(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("jupyter==1.0.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_jupyter_universal", |b| b.iter(|| run(true)));
}

fn resolve_warm_airflow(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![
        Requirement::from(uv_pep508::Requirement::from_str("apache-airflow[all]==2.9.3").unwrap()),
        Requirement::from(
            uv_pep508::Requirement::from_str("apache-airflow-providers-apache-beam>3.0.0").unwrap(),
        ),
    ]));
    c.bench_function("resolve_warm_airflow", |b| b.iter(|| run(false)));
}

// This takes >5m to run in CodSpeed.
// fn resolve_warm_airflow_universal(c: &mut Criterion<WallTime>) {
//     let run = setup_resolver(Manifest::simple(vec![
//         Requirement::from(uv_pep508::Requirement::from_str("apache-airflow[all]").unwrap()),
//         Requirement::from(
//             uv_pep508::Requirement::from_str("apache-airflow-providers-apache-beam>3.0.0").unwrap(),
//         ),
//     ]));
//     c.bench_function("resolve_warm_airflow_universal", |b| b.iter(|| run(true)));
// }

// =============================================================================
// Lightweight package resolution benchmarks (fast, for quick regression detection)
// =============================================================================

fn resolve_warm_ruff(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("ruff>=0.4.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_ruff", |b| b.iter(|| run(false)));
}

fn resolve_warm_black(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("black>=24.0.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_black", |b| b.iter(|| run(false)));
}

fn resolve_warm_pytest(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("pytest>=8.0.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_pytest", |b| b.iter(|| run(false)));
}

// Trio is a medium-complexity package used in the manual benchmarks (BENCHMARKS.md)
fn resolve_warm_trio(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("trio>=0.25.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_trio", |b| b.iter(|| run(false)));
}

// =============================================================================
// Web framework resolution benchmarks (medium complexity)
// =============================================================================

fn resolve_warm_django(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("django>=5.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_django", |b| b.iter(|| run(false)));
}

fn resolve_warm_django_universal(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("django>=5.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_django_universal", |b| b.iter(|| run(true)));
}

fn resolve_warm_fastapi(c: &mut Criterion<WallTime>) {
    // FastAPI with common extras for a realistic web API project
    let run = setup_resolver(Manifest::simple(vec![
        Requirement::from(uv_pep508::Requirement::from_str("fastapi>=0.111.0").unwrap()),
        Requirement::from(uv_pep508::Requirement::from_str("uvicorn[standard]>=0.29.0").unwrap()),
    ]));
    c.bench_function("resolve_warm_fastapi", |b| b.iter(|| run(false)));
}

// =============================================================================
// Data science / ML package resolution benchmarks
// =============================================================================

fn resolve_warm_numpy(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("numpy>=1.26.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_numpy", |b| b.iter(|| run(false)));
}

fn resolve_warm_pandas(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("pandas>=2.2.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_pandas", |b| b.iter(|| run(false)));
}

fn resolve_warm_scipy(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![Requirement::from(
        uv_pep508::Requirement::from_str("scipy>=1.13.0").unwrap(),
    )]));
    c.bench_function("resolve_warm_scipy", |b| b.iter(|| run(false)));
}

// A realistic data science stack
fn resolve_warm_datascience_stack(c: &mut Criterion<WallTime>) {
    let run = setup_resolver(Manifest::simple(vec![
        Requirement::from(uv_pep508::Requirement::from_str("numpy>=1.26.0").unwrap()),
        Requirement::from(uv_pep508::Requirement::from_str("pandas>=2.2.0").unwrap()),
        Requirement::from(uv_pep508::Requirement::from_str("scikit-learn>=1.4.0").unwrap()),
        Requirement::from(uv_pep508::Requirement::from_str("matplotlib>=3.8.0").unwrap()),
    ]));
    c.bench_function("resolve_warm_datascience_stack", |b| b.iter(|| run(false)));
}

// =============================================================================
// Incremental resolution benchmark (adding one dependency to existing resolution)
// =============================================================================

fn resolve_warm_trio_incremental(c: &mut Criterion<WallTime>) {
    // Simulate adding a new dependency to an existing project
    // Start with trio, add httpx (a common addition)
    let run = setup_resolver(Manifest::simple(vec![
        Requirement::from(uv_pep508::Requirement::from_str("trio>=0.25.0").unwrap()),
        Requirement::from(uv_pep508::Requirement::from_str("httpx>=0.27.0").unwrap()),
    ]));
    c.bench_function("resolve_warm_trio_incremental", |b| b.iter(|| run(false)));
}

// =============================================================================
// Venv creation benchmark
// =============================================================================

fn venv_create(c: &mut Criterion<WallTime>) {
    let run = setup_venv_create();
    c.bench_function("venv_create", |b| b.iter(|| run()));
}

// =============================================================================
// Criterion groups
// =============================================================================

criterion_group!(
    name = existing_benchmarks;
    config = Criterion::default();
    targets =
        resolve_warm_jupyter,
        resolve_warm_jupyter_universal,
        resolve_warm_airflow
);

criterion_group!(
    name = lightweight_benchmarks;
    config = Criterion::default();
    targets =
        resolve_warm_ruff,
        resolve_warm_black,
        resolve_warm_pytest,
        resolve_warm_trio
);

criterion_group!(
    name = webframework_benchmarks;
    config = Criterion::default();
    targets =
        resolve_warm_django,
        resolve_warm_django_universal,
        resolve_warm_fastapi
);

criterion_group!(
    name = datascience_benchmarks;
    config = Criterion::default();
    targets =
        resolve_warm_numpy,
        resolve_warm_pandas,
        resolve_warm_scipy,
        resolve_warm_datascience_stack
);

criterion_group!(
    name = incremental_benchmarks;
    config = Criterion::default();
    targets =
        resolve_warm_trio_incremental
);

criterion_group!(
    name = venv_benchmarks;
    config = Criterion::default();
    targets =
        venv_create
);

criterion_main!(
    existing_benchmarks,
    lightweight_benchmarks,
    webframework_benchmarks,
    datascience_benchmarks,
    incremental_benchmarks,
    venv_benchmarks
);

fn setup_resolver(manifest: Manifest) -> impl Fn(bool) {
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

fn setup_venv_create() -> impl Fn() {
    use std::sync::atomic::{AtomicU64, Ordering};
    use uv_virtualenv::{Prompt, create_venv};

    let counter = AtomicU64::new(0);

    let cache = Cache::from_path("../../.cache")
        .init_no_wait()
        .expect("No cache contention when running benchmarks")
        .unwrap();
    let interpreter = PythonEnvironment::from_root("../../.venv", &cache)
        .unwrap()
        .into_interpreter();

    move || {
        let n = counter.fetch_add(1, Ordering::SeqCst);
        let venv_dir = std::env::temp_dir().join(format!("uv-bench-venv-{}", n));

        let _ = black_box(create_venv(
            &venv_dir,
            interpreter.clone(),
            Prompt::None,
            false, // system_site_packages
            uv_virtualenv::OnExisting::Allow,
            false, // relocatable
            false, // seed
            false, // upgradeable
            uv_preview::Preview::default(),
        ));

        // Clean up
        let _ = std::fs::remove_dir_all(&venv_dir);
    }
}

mod resolver {
    use std::sync::LazyLock;

    use anyhow::Result;

    use uv_cache::Cache;
    use uv_client::RegistryClient;
    use uv_configuration::{BuildOptions, Concurrency, Constraints, IndexStrategy, SourceStrategy};
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
        let sources = SourceStrategy::default();
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
            concurrency,
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
            DistributionDatabase::new(client, &build_context, concurrency.downloads),
        )?;

        Ok(resolver.resolve().await?)
    }
}
