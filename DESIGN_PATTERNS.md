# Design Patterns in uv

A catalog of recurring Rust design patterns found across the 67 crates in the uv workspace. Each
pattern includes representative examples and the rationale behind its use.

---

## Newtype Wrappers

Single-field tuple structs that add type safety, validation, and domain semantics at zero runtime
cost.

- **`PackageName(SmallString)`** (`crates/uv-normalize/src/package_name.rs`) — normalized package
  names with `FromStr` validation; custom serde `Visitor` avoids allocating when the input is
  already a `&str`.
- **`ExtraName(SmallString)`**, **`GroupName(SmallString)`**, **`DistInfoName`**
  (`crates/uv-normalize/`) — same pattern for extras, dependency groups, and `.dist-info` directory
  names.
- **`PythonVersion(StringVersion)`** (`crates/uv-python/src/python_version.rs`) — rejects dev/local
  versions in `from_str`.
- **`Target(PathBuf)`** (`crates/uv-python/src/target.rs`) — wraps a `--target` directory with
  methods like `scheme()` and `site_packages()`.
- **`SmallString(arcstr::ArcStr)`** (`crates/uv-small-str/src/lib.rs`) — reference-counted,
  inline-optimized immutable string used as the backing store for most normalized identifiers.
- **`GitOid([u8; 40])`** (`crates/uv-git-types/src/oid.rs`) — fixed-size, stack-allocated SHA
  representation; validated in `FromStr`.
- **`ConfigSettings(BTreeMap<String, ConfigSettingValue>)`**, **`SourceAnnotations(BTreeMap<…>)`**
  (`crates/uv-distribution-types/`) — semantic map wrappers with `FromIterator` conversions.

---

## Builder Pattern

Fluent, chainable construction of complex structs. Methods return `Self` (or `&mut Self`) and a
terminal `build()` method produces the final value.

- **`OptionsBuilder`** (`crates/uv-resolver/src/options.rs`) — `#[must_use]` setters like
  `resolution_mode()`, `prerelease_mode()`, `dependency_mode()`; terminal `build()` returns
  `Options`.
- **`BaseClientBuilder<'a>`** (`crates/uv-client/src/base_client.rs`) — 20+ configuration fields
  with a lifetime parameter for borrowed marker data; `RegistryClientBuilder` wraps and delegates to
  it.
- **`Installer`** (`crates/uv-installer/src/installer.rs`) — `with_link_mode()`, `with_cache()`,
  `with_reporter()`; final `install()` method spawns work on a rayon thread pool and returns results
  via a oneshot channel.
- **`GitSource`** (`crates/uv-git/src/source.rs`) — `dangerous()` disables SSL,
  `with_reporter(Arc<dyn Reporter>)` adds progress callbacks.
- **`Cache`** (`crates/uv-cache/src/lib.rs`) — uses `with_refresh()`, `with_virtualenv()` to return
  modified copies via struct update syntax (`Self { field, ..self }`).

---

## Enum-Based Polymorphism

Multi-level enum hierarchies that replace runtime type checks with exhaustive `match` statements.

- **Distribution type hierarchy** (`crates/uv-distribution-types/src/lib.rs`):
  ```text
  Dist ─┬─ Built ─┬─ Registry
        │         ├─ DirectUrl
        │         └─ Path
        └─ Source ─┬─ Registry
                   ├─ DirectUrl
                   ├─ Git
                   ├─ Path
                   └─ Directory
  ```
  Each leaf variant wraps a distinct struct. Traits like `Name`, `DistributionMetadata`, and
  `Identifier` are implemented per-variant.
- **`Source`** (`crates/uv-resolver/src/lock/`) — `Registry`, `Git`, `Direct`, `Path`, `Directory`,
  `Editable`, `Virtual`; each carries source-specific data (URL, path, hash).
- **`IndexUrl`** (`crates/uv-distribution-types/src/index_url.rs`) — `Pypi`, `Url`, `Path`; smart
  `parse()` with configurable root directory.
- **`GitReference`** (`crates/uv-git-types/src/reference.rs`) — `Branch`, `Tag`, `BranchOrTag`,
  `NamedRef`, `DefaultBranch`; auto-detection in `from_rev()`.
- **`CompatibleDist<'a>`** (`crates/uv-distribution-types/src/prioritized_distribution.rs`) —
  four-way enum for distribution compatibility states (`InstalledDist`, `SourceDist`,
  `CompatibleWheel`, `IncompatibleWheel`).

---

## Mode-to-Strategy Conversion

Public configuration enums that are transformed into richer internal strategy types, separating
user-facing API from resolution logic.

- **`ResolutionMode` -> `ResolutionStrategy`** (`crates/uv-resolver/src/resolution_mode.rs`) —
  `LowestDirect` variant gains a precomputed `ForkSet` of direct dependencies.
- **`PrereleaseMode` -> `PrereleaseStrategy`** (`crates/uv-resolver/src/prerelease.rs`) —
  three-level pipeline: `Mode -> Strategy -> AllowPrerelease` decision per package.
- **CLI `Commands` enum -> per-command `Settings::resolve()`** (`crates/uv/src/settings.rs`) — each
  CLI subcommand's args are resolved against filesystem and environment options before being
  dispatched.

---

## Layered Configuration

A three-tier settings system where each layer takes precedence over the next: CLI flags >
environment variables > filesystem config files > defaults.

- **`GlobalSettings::resolve(args, workspace, environment)`** (`crates/uv/src/settings.rs`) — every
  command has a parallel `*Settings` struct with a static `resolve()` method.
- **`FilesystemOptions`** (`crates/uv-settings/src/lib.rs`) — searches system (`/etc/xdg/…`), user
  (`~/.config/uv/`), and project (`uv.toml` / `pyproject.toml [tool.uv]`) directories; results
  merged via `Combine`.
- **`Combine` trait** (`crates/uv-settings/`) — `self.combine(other)` prefers `self`; generic impls
  for `Option<T>` (first wins), `Option<Vec<T>>` (extend), `Option<BTreeMap<K, Vec<T>>>` (merge by
  key).
- **`CombineOptions` derive macro** (`crates/uv-macros/src/lib.rs`) — auto-generates `Combine` for
  structs by combining each field.
- **`Flag` enum with `FlagSource`** (`crates/uv-cli/src/options.rs`) — tracks whether a boolean flag
  was set from CLI, env var, or config file.

---

## Wire Types for Serialization

Separate `*Wire` structs that match the on-disk format (flat, kebab-case TOML) and are converted to
richly-typed internal structs during deserialization.

- **`PackageWire` <-> `Package`**, **`DependencyWire` <-> `Dependency`**, **`SourceDistWire` <->
  `SourceDist`**, **`WheelWire` <-> `Wheel`**, **`SourceWire` <-> `Source`**
  (`crates/uv-resolver/src/lock/`) — each pair uses `TryFrom` (or a custom `unwire()` method) with
  validation.
- **`TomlCredentialWire` <-> `TomlCredential`** (`crates/uv-auth/src/store.rs`) — enforces
  scheme-specific rules during `TryFrom` (Basic requires username, Bearer requires token).
- **`LockWire` -> `Lock`** — top-level lockfile uses `#[serde(try_from = "LockWire")]` for invariant
  checking during deserialization.

---

## Trait-Based Abstraction

Traits that define shared behavior across multiple distribution, reporter, or provider types.

- **`Name`**, **`DistributionMetadata`**, **`InstalledMetadata`**, **`RemoteSource`**,
  **`Identifier`** (`crates/uv-distribution-types/src/traits.rs`) — compositional trait bounds
  (`DistributionMetadata: Name`) with default method implementations.
- **`ResolverProvider`** (`crates/uv-resolver/src/resolver/provider.rs`) — `get_package_versions()`,
  `get_or_build_wheel_metadata()`; `DefaultResolverProvider` wraps a real `DistributionDatabase`.
- **`Reporter` trait family** — `ResolverReporter`, `InstallReporter`, `PrepareReporter`,
  `CleanReporter`; all stored as `Option<Arc<dyn Reporter>>` for optional progress callbacks.
- **`Cacheable`** (`crates/uv-client/src/cached_client.rs`) — associated type `Target` with
  `from_aligned_bytes()`/`to_bytes()` methods; two impls cover serde-based and rkyv-based
  serialization.
- **`Simplified`** (`crates/uv-fs/src/path.rs`) — multiple display modes for paths
  (`user_display()`, `portable_display()`).
- **`Visit`** (`crates/uv-options-metadata/src/lib.rs`) — visitor for configuration metadata with
  `record_field()` and `record_set()`.

---

## Smart Pointer & Interior Mutability Patterns

Strategic use of `Arc`, `Cow`, `OnceLock`, `LazyLock`, `DashMap`, `RwLock`, and `Mutex` to balance
sharing, laziness, and thread safety.

- **`Arc<dyn T>`** — reporters (`Arc<dyn Reporter>`), middleware (`Arc<dyn Middleware>`), error
  chains (`Arc<dyn std::error::Error + Send + Sync>`).
- **`Cow<'_, str>` / `Cow<'_, Path>`** — avoids allocation in the common case;
  `PackageName::as_dist_info_name()` returns `Cow::Borrowed` when no dash replacement is needed.
- **`OnceLock`** — per-instance lazy fields; `Interpreter.tags`, and `InstalledDist.metadata_cache`
  / `tags_cache`.
- **`LazyLock`** — static singletons; `PYPI_URL`, `DEFAULT_INDEX`, `MIN_VERSION`, `CWD`,
  `LOCK_TIMEOUT`, regex patterns in `uv-extract`.
- **`DashMap`** — concurrent lock-free maps in the resolver: `unavailable_packages`,
  `incomplete_packages`; in git: `GitResolver(Arc<DashMap<…>>)`.
- **`RwLock<FxHashMap<…>>`** — credentials cache with realm, URL-trie, and fetch-based lookup
  layers.
- **`Arc<Mutex<Option<NamedTempFile>>>`** — retry loop for atomic file persistence on Windows
  (antivirus interference).

---

## Shared State with `fork()`

Cloneable state bundles that use `Arc` internally so copies share expensive resources while getting
fresh per-operation state.

- **`SharedState`** (`crates/uv-dispatch/src/lib.rs`) — holds `GitResolver`, `IndexCapabilities`,
  `InMemoryIndex`, `InFlight`, `BuildArena`; `fork()` creates a child with a fresh index and
  in-flight set but shares git and capabilities.

---

## Concurrency Patterns

- **Rayon thread pool** — `Installer` and `uv-extract` parallel ZIP extraction via `rayon::spawn()`
  with `LazyLock`-initialized pools.
- **Oneshot channels** — resolver spawns PubGrub solver on a dedicated thread, sends results back
  via `tokio::sync::oneshot`.
- **Async streams** — `Preparer::prepare_stream()` returns `impl Stream<Item = Result<CachedDist>>`
  for parallel downloads.
- **Tokio blocking spawn** — file locking operations (`LockedFile`) dispatched to tokio's blocking
  thread pool.

---

## Memoization & Caching

- **`CachedClient`** (`crates/uv-client/src/cached_client.rs`) — generic
  `get_cacheable<Payload, Callback>()` method with `CacheControl` policy enum (`None`,
  `MustRevalidate`, `AllowStale`, `Override`).
- **Multi-level credential caching** (`crates/uv-auth/src/cache.rs`) — realm cache (RFC 7235),
  URL-trie cache, fetch-result cache; each with different lookup strategies.
- **`VersionMap`** (`crates/uv-resolver/src/version_map.rs`) — `Lazy` vs `Eager` inner state;
  distributions computed on first access.
- **`PrioritizedDist`** (`crates/uv-distribution-types/src/prioritized_distribution.rs`) —
  `best_wheel_index` caches the index of the best-matching wheel.
- **Cache bucket versioning** (`crates/uv-cache/src/lib.rs`) — enum variants like `Wheels` map to
  `"wheels-v6"`, `SourceDistributions` to `"sdists-v9"`, enabling schema migration by bumping the
  suffix.

---

## Data-Driven Size Optimization

- **`Version`** (`crates/uv-pep440/`) — `VersionInner::Small` fits ~90% of PyPI versions in a
  compact struct; only uncommon formats allocate an `Arc<VersionFull>`. Based on empirical analysis
  of real-world data.
- **`Tags`** (`crates/uv-platform-tags/`) — nested
  `Arc<FxHashMap<Language, FxHashMap<Abi, FxHashMap<Platform, Priority>>>>` for O(1) tag lookups;
  cached `best` field for the common single-lookup case.
- **`PrioritizedDist`** (`crates/uv-distribution-types/`) — boxed inner struct
  (`Box<PrioritizedDistInner>`) to control enum variant size.

---

## Bitflags

- **`CPythonAbiVariants`** (`crates/uv-platform-tags/`) — `#[repr(transparent)]` `u8` bitfield via
  `bitflags::bitflags!`; flags: `Freethreading`, `WideUnicode`, `Debug`, `Pymalloc`. Custom
  `Display` renders as suffix chars (`t`, `u`, `d`, `m`).

---

## Custom Derive Macros

- **`OptionsMetadata`** (`crates/uv-macros/src/lib.rs`) — generates `Visit` trait impls from
  `#[option(default, value_type, example)]` attributes and doc comments; powers schema generation
  and documentation.
- **`CombineOptions`** — generates `Combine` for structs by combining each field.
- **`attribute_env_vars_metadata`** — procedural attribute macro that validates all constants have
  `#[attr_added_in(...)]` annotations.

---

## Error Handling

- **`thiserror` derive** — all major crates use `#[derive(thiserror::Error)]` with `#[error(…)]`
  format strings and `#[source]` / `#[from]` annotations.
- **Boxed error kinds** — `uv-client` uses `Error(Box<ErrorKind>)` to keep error enum sizes small;
  `Display` includes retry count and duration.
- **Polymorphic callback errors** — `CachedClientError<CallbackError>` is generic over the
  callback's error type, separating HTTP errors from application errors.
- **Secret-safe `Debug`** — `Password` and `Token` (`crates/uv-auth/src/credentials.rs`) print
  `****` to prevent credential leaks in logs.

---

## Clap CLI Architecture

- **Namespace structs** — commands grouped via `#[derive(Args)]` wrapper structs containing a
  `#[command(subcommand)]` enum (`PipNamespace`, `AuthNamespace`, `ProjectCommand`, etc.).
- **Flattened global args** — `TopLevelArgs` flattens `CacheArgs` and `GlobalArgs`; individual flags
  bind to environment variables via `#[arg(env = EnvVars::UV_*)]`.
- **Boolean flag pairs** — `--locked` / `--no-locked` resolved via `flag(yes, no)` helper that
  errors on conflict.
- **`Maybe<T>` enum** (`crates/uv-cli/src/lib.rs`) — distinguishes "not provided" from "explicitly
  set to None" for nullable CLI arguments.

---

## Atomic & Platform-Specific Filesystem Operations

- **Atomic writes** (`crates/uv-fs/src/lib.rs`) — `write_atomic()` writes to a temp file then
  renames; `persist_with_retry()` handles Windows antivirus locks with exponential backoff.
- **Platform symlinks** — Unix uses `symlink`, Windows uses junctions; `replace_symlink()` provides
  atomic replacement.
- **File locking** — `LockedFile` wraps a file with `Shared`/`Exclusive` mode; timeout-based
  acquisition with configurable retries; auto-release on drop.

---

## Multi-Level Indexing

- **`SitePackages`** (`crates/uv-installer/src/site_packages.rs`) — double-indexed by name
  (`FxHashMap<PackageName, Vec<usize>>`) and URL (`FxHashMap<DisplaySafeUrl, Vec<usize>>`); indices
  point into a flat `Vec<Option<InstalledDist>>` for efficient mutation.
- **`FlatIndex`** (`crates/uv-resolver/src/flat_index.rs`) — two-level
  `FxHashMap<PackageName, BTreeMap<Version, PrioritizedDist>>`.
- **`Tags`** — three-level nested `FxHashMap` (language -> ABI -> platform -> priority).

---

## Marker Algebra

- **`UniversalMarker`** (`crates/uv-resolver/src/universal_marker.rs`) — encodes conflict markers as
  synthetic PEP 508 extras (`extra == 'extra-3-foo-x1'`), keeping everything within the PEP 508
  grammar while extending expressiveness for multi-platform resolution.
- **Dual marker representation** in lockfile dependencies — `simplified_marker` (assumes
  `requires-python` satisfied) and `complexified_marker` (standalone); enables both efficient
  deduplication and correct standalone evaluation.
