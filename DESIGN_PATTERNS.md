# Design Principles in uv

Ten recurring design principles observed across the uv codebase.

1. **Make invalid states unrepresentable.** Newtype wrappers like `PackageName`, `ExtraName`, and
   `GitOid` validate on construction so downstream code never handles raw, unchecked values. Wire
   types (`PackageWire` → `Package`) enforce invariants during deserialization rather than after.
   Enum hierarchies (`Dist` → `BuiltDist`/`SourceDist` → leaf variants) force exhaustive handling of
   every case at compile time.

2. **Separate configuration from execution.** Public "mode" enums (`ResolutionMode`,
   `PrereleaseMode`) are transformed into richer internal "strategy" types that carry precomputed
   data. CLI arguments, environment variables, and filesystem config are each parsed independently,
   then merged through a `Combine` trait with clear precedence rules (CLI > env > file > default).

3. **Pay for what you use.** `Version` stores ~90% of real-world PyPI versions in a compact inline
   `VersionSmall`; only uncommon formats allocate an `Arc<VersionFull>`. `Cow<'_, str>` avoids
   cloning when no transformation is needed. `OnceLock` and `LazyLock` defer expensive work (tag
   computation, regex compilation, static singletons) until first access.

4. **Share expensive resources, isolate cheap ones.** `SharedState::fork()` clones a state bundle
   where `Arc`-wrapped resources (git resolver, index capabilities) are shared across forks while
   per-operation state (in-flight set, index) is created fresh. `DashMap` provides lock-free
   concurrent access to shared resolver state without coarse locking.

5. **Build complex objects incrementally.** Fluent builders (`BaseClientBuilder`, `OptionsBuilder`,
   `Installer`) accumulate configuration through `#[must_use]` chainable setters and produce the
   final value via a terminal `build()` or `install()` method. Optional concerns like reporters are
   attached via `with_reporter()` without polluting constructors.

6. **Decouple serialization format from domain model.** Dedicated `*Wire` structs match on-disk
   layout (flat TOML, kebab-case keys) and convert to richly-typed internal structs via `TryFrom`.
   The `Cacheable` trait abstracts over serde and rkyv serialization so the caching layer is
   payload-agnostic. `#[serde(try_from = "LockWire")]` runs validation as part of deserialization.

7. **Use traits for extension points, enums for closed sets.** Traits like `ResolverProvider` and
   `Reporter` enable swapping implementations (real vs. test, with vs. without progress reporting)
   via `Arc<dyn T>`. Closed taxonomies (distribution sources, git reference kinds, cache buckets)
   use enums so the compiler catches missing cases.

8. **Make concurrency explicit at boundaries.** Rayon thread pools handle CPU-bound work (wheel
   installation, ZIP extraction). Async streams drive parallel downloads. Oneshot channels bridge
   between blocking solver threads and the async runtime. File locks use `LockedFile` with
   drop-based release and configurable timeouts rather than implicit locking.

9. **Optimize indexing for access patterns.** `SitePackages` double-indexes by name and URL into a
   flat `Vec`. Platform `Tags` use a three-level nested `FxHashMap` (language → ABI → platform) for
   O(1) compatibility lookups. `PrioritizedDist` caches the best wheel index to avoid repeated
   scans. Multi-level credential caches (realm, URL-trie, fetch) each serve different lookup
   patterns.

10. **Derive, don't hand-write.** Custom proc macros (`OptionsMetadata`, `CombineOptions`) generate
    trait implementations from struct definitions and doc-comment attributes, keeping configuration
    schema, documentation, and merge logic in sync. Standard derives (`thiserror`, `clap`, `serde`,
    `rkyv`) eliminate boilerplate for error types, CLI parsing, and serialization.
