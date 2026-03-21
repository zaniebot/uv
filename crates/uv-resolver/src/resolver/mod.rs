//! Given a set of requirements, find a set of compatible packages.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::{Display, Formatter, Write};
use std::ops::Bound;
use std::sync::Arc;
use std::time::Instant;
use std::{iter, slice, thread};

use dashmap::DashMap;
use either::Either;
use futures::{FutureExt, StreamExt};
use itertools::Itertools;
use pubgrub::{Id, IncompId, Incompatibility, Kind, Range, Ranges, State};
use rustc_hash::{FxHashMap, FxHashSet};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{Level, debug, info, instrument, trace, warn};

use uv_configuration::{Constraints, Excludes, Overrides};
use uv_distribution::{ArchiveMetadata, DistributionDatabase};
use uv_distribution_types::{
    BuiltDist, CompatibleDist, DerivationChain, Dist, DistErrorKind, Identifier, IncompatibleDist,
    IncompatibleSource, IncompatibleWheel, IndexCapabilities, IndexLocations, IndexMetadata,
    IndexUrl, InstalledDist, Name, PythonRequirementKind, RemoteSource, Requirement, ResolvedDist,
    ResolvedDistRef, SourceDist, VersionOrUrlRef, implied_markers,
};
use uv_git::GitResolver;
use uv_normalize::{ExtraName, GroupName, PackageName};
use uv_pep440::{MIN_VERSION, Version, VersionSpecifiers, release_specifiers_to_ranges};
use uv_pep508::{
    MarkerEnvironment, MarkerExpression, MarkerOperator, MarkerTree, MarkerValueString,
};
use uv_platform_tags::{IncompatibleTag, Tags};
use uv_pypi_types::{ConflictItem, ConflictItemRef, ConflictKindRef, Conflicts, VerbatimParsedUrl};
use uv_torch::TorchStrategy;
use uv_types::{BuildContext, HashStrategy, InstalledPackagesProvider};
use uv_warnings::warn_user_once;

use crate::candidate_selector::{Candidate, CandidateDist, CandidateSelector};
use crate::dependency_provider::UvDependencyProvider;
use crate::error::{NoSolutionError, ResolveError};
use crate::fork_indexes::ForkIndexes;
use crate::fork_strategy::ForkStrategy;
use crate::fork_urls::ForkUrls;
use crate::manifest::Manifest;
use crate::pins::FilePins;
use crate::preferences::{PreferenceSource, Preferences};
use crate::pubgrub::{
    DependencySource, PubGrubDependency, PubGrubPackage, PubGrubPackageInner, PubGrubPriorities,
    PubGrubPython,
};
use crate::python_requirement::PythonRequirement;
use crate::resolution::ResolverOutput;
use crate::resolution_mode::ResolutionStrategy;
pub(crate) use crate::resolver::availability::{
    ResolverVersion, UnavailableErrorChain, UnavailablePackage, UnavailableReason,
    UnavailableVersion,
};
use crate::resolver::batch_prefetch::BatchPrefetcher;
pub use crate::resolver::derivation::DerivationChainBuilder;
pub use crate::resolver::environment::ResolverEnvironment;
use crate::resolver::environment::{
    ForkingPossibility, fork_version_by_marker, fork_version_by_python_requirement,
};
pub(crate) use crate::resolver::fork_map::{ForkMap, ForkSet};
pub use crate::resolver::index::InMemoryIndex;
use crate::resolver::indexes::Indexes;
pub use crate::resolver::provider::{
    DefaultResolverProvider, MetadataResponse, PackageVersionsResult, ResolverProvider,
    VersionsResponse, WheelMetadataResult,
};
pub use crate::resolver::reporter::{BuildId, Reporter};
use crate::resolver::system::SystemDependency;
pub(crate) use crate::resolver::urls::Urls;
use crate::universal_marker::{ConflictMarker, UniversalMarker};
use crate::yanks::AllowedYanks;
use crate::{
    DependencyMode, ExcludeNewer, Exclusions, FlatIndex, Options, ResolutionMode, VersionMap,
    marker,
};
pub(crate) use provider::MetadataUnavailable;

mod availability;
mod batch_prefetch;
mod derivation;
mod environment;
mod fork_map;
mod index;
mod indexes;
mod provider;
mod reporter;
mod state;
mod system;
mod urls;

pub(crate) use state::{
    ForkState, Request, Resolution, ResolutionDependencyEdge, ResolutionPackage,
};

/// The number of conflicts a package may accumulate before we re-prioritize and backtrack.
const CONFLICT_THRESHOLD: usize = 5;

pub struct Resolver<Provider: ResolverProvider, InstalledPackages: InstalledPackagesProvider> {
    state: ResolverState<InstalledPackages>,
    provider: Provider,
}

/// State that is shared between the prefetcher and the PubGrub solver during
/// resolution, across all forks.
struct ResolverState<InstalledPackages: InstalledPackagesProvider> {
    project: Option<PackageName>,
    requirements: Vec<Requirement>,
    constraints: Constraints,
    overrides: Overrides,
    excludes: Excludes,
    preferences: Preferences,
    git: GitResolver,
    capabilities: IndexCapabilities,
    locations: IndexLocations,
    exclusions: Exclusions,
    urls: Urls,
    indexes: Indexes,
    dependency_mode: DependencyMode,
    hasher: HashStrategy,
    env: ResolverEnvironment,
    // The environment of the current Python interpreter.
    current_environment: MarkerEnvironment,
    tags: Option<Tags>,
    python_requirement: PythonRequirement,
    conflicts: Conflicts,
    workspace_members: BTreeSet<PackageName>,
    selector: CandidateSelector,
    index: InMemoryIndex,
    installed_packages: InstalledPackages,
    /// Incompatibilities for packages that are entirely unavailable.
    unavailable_packages: DashMap<PackageName, UnavailablePackage>,
    /// Incompatibilities for packages that are unavailable at specific versions.
    incomplete_packages: DashMap<PackageName, DashMap<Version, MetadataUnavailable>>,
    /// The options that were used to configure this resolver.
    options: Options,
    /// The reporter to use for this resolver.
    reporter: Option<Arc<dyn Reporter>>,
}

impl<'a, Context: BuildContext, InstalledPackages: InstalledPackagesProvider>
    Resolver<DefaultResolverProvider<'a, Context>, InstalledPackages>
{
    /// Initialize a new resolver using the default backend doing real requests.
    ///
    /// Reads the flat index entries.
    ///
    /// # Marker environment
    ///
    /// The marker environment is optional.
    ///
    /// When a marker environment is not provided, the resolver is said to be
    /// in "universal" mode. When in universal mode, the resolution produced
    /// may contain multiple versions of the same package. And thus, in order
    /// to use the resulting resolution, there must be a "universal"-aware
    /// reader of the resolution that knows to exclude distributions that can't
    /// be used in the current environment.
    ///
    /// When a marker environment is provided, the resolver is in
    /// "non-universal" mode, which corresponds to standard `pip` behavior that
    /// works only for a specific marker environment.
    pub fn new(
        manifest: Manifest,
        options: Options,
        python_requirement: &'a PythonRequirement,
        env: ResolverEnvironment,
        current_environment: &MarkerEnvironment,
        conflicts: Conflicts,
        tags: Option<&'a Tags>,
        flat_index: &'a FlatIndex,
        index: &'a InMemoryIndex,
        hasher: &'a HashStrategy,
        build_context: &'a Context,
        installed_packages: InstalledPackages,
        database: DistributionDatabase<'a, Context>,
    ) -> Result<Self, ResolveError> {
        let provider = DefaultResolverProvider::new(
            database,
            flat_index,
            tags,
            python_requirement.target(),
            AllowedYanks::from_manifest(&manifest, &env, options.dependency_mode),
            hasher,
            options.exclude_newer.clone(),
            build_context.build_options(),
            build_context.capabilities(),
        );

        Self::new_custom_io(
            manifest,
            options,
            hasher,
            env,
            current_environment,
            tags.cloned(),
            python_requirement,
            conflicts,
            index,
            build_context.git(),
            build_context.capabilities(),
            build_context.locations(),
            provider,
            installed_packages,
        )
    }
}

impl<Provider: ResolverProvider, InstalledPackages: InstalledPackagesProvider>
    Resolver<Provider, InstalledPackages>
{
    /// Initialize a new resolver using a user provided backend.
    pub fn new_custom_io(
        manifest: Manifest,
        options: Options,
        hasher: &HashStrategy,
        env: ResolverEnvironment,
        current_environment: &MarkerEnvironment,
        tags: Option<Tags>,
        python_requirement: &PythonRequirement,
        conflicts: Conflicts,
        index: &InMemoryIndex,
        git: &GitResolver,
        capabilities: &IndexCapabilities,
        locations: &IndexLocations,
        provider: Provider,
        installed_packages: InstalledPackages,
    ) -> Result<Self, ResolveError> {
        let state = ResolverState {
            index: index.clone(),
            git: git.clone(),
            capabilities: capabilities.clone(),
            selector: CandidateSelector::for_resolution(&options, &manifest, &env),
            dependency_mode: options.dependency_mode,
            urls: Urls::from_manifest(&manifest, &env, git, options.dependency_mode),
            indexes: Indexes::from_manifest(&manifest, &env, options.dependency_mode),
            project: manifest.project,
            workspace_members: manifest.workspace_members,
            requirements: manifest.requirements,
            constraints: manifest.constraints,
            overrides: manifest.overrides,
            excludes: manifest.excludes,
            preferences: manifest.preferences,
            exclusions: manifest.exclusions,
            hasher: hasher.clone(),
            locations: locations.clone(),
            env,
            current_environment: current_environment.clone(),
            tags,
            python_requirement: python_requirement.clone(),
            conflicts,
            installed_packages,
            unavailable_packages: DashMap::default(),
            incomplete_packages: DashMap::default(),
            options,
            reporter: None,
        };
        Ok(Self { state, provider })
    }

    /// Set the [`Reporter`] to use for this installer.
    #[must_use]
    pub fn with_reporter(self, reporter: Arc<dyn Reporter>) -> Self {
        Self {
            state: ResolverState {
                reporter: Some(reporter.clone()),
                ..self.state
            },
            provider: self
                .provider
                .with_reporter(reporter.into_distribution_reporter()),
        }
    }

    /// Resolve a set of requirements into a set of pinned versions.
    pub async fn resolve(self) -> Result<ResolverOutput, ResolveError> {
        let state = Arc::new(self.state);
        let provider = Arc::new(self.provider);

        // A channel to fetch package metadata (e.g., given `flask`, fetch all versions) and version
        // metadata (e.g., given `flask==1.0.0`, fetch the metadata for that version).
        // Channel size is set large to accommodate batch prefetching.
        let (request_sink, request_stream) = mpsc::channel(300);

        // Run the fetcher.
        let requests_fut = state.clone().fetch(provider.clone(), request_stream).fuse();

        // Spawn the PubGrub solver on a dedicated thread.
        let solver = state.clone();
        let (tx, rx) = oneshot::channel();
        thread::Builder::new()
            .name("uv-resolver".into())
            .spawn(move || {
                let result = solver.solve(&request_sink);

                // This may fail if the main thread returned early due to an error.
                let _ = tx.send(result);
            })
            .unwrap();

        let resolve_fut = async move { rx.await.map_err(|_| ResolveError::ChannelClosed) };

        // Wait for both to complete.
        let ((), resolution) = tokio::try_join!(requests_fut, resolve_fut)?;

        state.on_complete();
        resolution
    }
}

enum Dependencies {
    /// Package dependencies are not available.
    Unavailable(UnavailableVersion),
    /// Container for all available package versions.
    ///
    /// Note that in universal mode, it is possible and allowed for multiple
    /// `PubGrubPackage` values in this list to have the same package name.
    /// These conflicts are resolved via `Dependencies::fork`.
    Available(Vec<PubGrubDependency>),
    /// Dependencies that should never result in a fork.
    ///
    /// For example, the dependencies of a `Marker` package will have the
    /// same name and version, but differ according to marker expressions.
    /// But we never want this to result in a fork.
    Unforkable(Vec<PubGrubDependency>),
}

impl Dependencies {
    /// Turn this flat list of dependencies into a potential set of forked
    /// groups of dependencies.
    ///
    /// A fork *only* occurs when there are multiple dependencies with the same
    /// name *and* those dependency specifications have corresponding marker
    /// expressions that are completely disjoint with one another.
    fn fork(
        self,
        env: &ResolverEnvironment,
        python_requirement: &PythonRequirement,
        conflicts: &Conflicts,
    ) -> ForkedDependencies {
        let deps = match self {
            Self::Available(deps) => deps,
            Self::Unforkable(deps) => return ForkedDependencies::Unforked(deps),
            Self::Unavailable(err) => return ForkedDependencies::Unavailable(err),
        };
        let mut name_to_deps: BTreeMap<PackageName, Vec<PubGrubDependency>> = BTreeMap::new();
        for dep in deps {
            let name = dep
                .package
                .name()
                .expect("dependency always has a name")
                .clone();
            name_to_deps.entry(name).or_default().push(dep);
        }
        let Forks {
            mut forks,
            diverging_packages,
        } = Forks::new(name_to_deps, env, python_requirement, conflicts);
        if forks.is_empty() {
            ForkedDependencies::Unforked(vec![])
        } else if forks.len() == 1 {
            ForkedDependencies::Unforked(forks.pop().unwrap().dependencies)
        } else {
            ForkedDependencies::Forked {
                forks,
                diverging_packages: diverging_packages.into_iter().collect(),
            }
        }
    }
}

/// Information about the (possibly forked) dependencies for a particular
/// package.
///
/// This is like `Dependencies` but with an extra variant that only occurs when
/// a `Dependencies` list has multiple dependency specifications with the same
/// name and non-overlapping marker expressions (i.e., a fork occurs).
#[derive(Debug)]
enum ForkedDependencies {
    /// Package dependencies are not available.
    Unavailable(UnavailableVersion),
    /// No forking occurred.
    ///
    /// This is the same as `Dependencies::Available`.
    Unforked(Vec<PubGrubDependency>),
    /// Forked containers for all available package versions.
    ///
    /// Note that there is always at least two forks. If there would
    /// be fewer than 2 forks, then there is no fork at all and the
    /// `Unforked` variant is used instead.
    Forked {
        forks: Vec<Fork>,
        /// The package(s) with different requirements for disjoint markers.
        diverging_packages: Vec<PackageName>,
    },
}

/// A list of forks determined from the dependencies of a single package.
///
/// Any time a marker expression is seen that is not true for all possible
/// marker environments, it is possible for it to introduce a new fork.
#[derive(Debug, Default)]
struct Forks {
    /// The forks discovered among the dependencies.
    forks: Vec<Fork>,
    /// The package(s) that provoked at least one additional fork.
    diverging_packages: BTreeSet<PackageName>,
}

impl Forks {
    fn new(
        name_to_deps: BTreeMap<PackageName, Vec<PubGrubDependency>>,
        env: &ResolverEnvironment,
        python_requirement: &PythonRequirement,
        conflicts: &Conflicts,
    ) -> Self {
        let python_marker = python_requirement.to_marker_tree();

        let mut forks = vec![Fork::new(env.clone())];
        let mut diverging_packages = BTreeSet::new();
        for (name, mut deps) in name_to_deps {
            assert!(!deps.is_empty(), "every name has at least one dependency");
            // We never fork if there's only one dependency
            // specification for a given package name. This particular
            // strategy results in a "conservative" approach to forking
            // that gives up correctness in some cases in exchange for
            // more limited forking. More limited forking results in
            // simpler-and-easier-to-understand lock files and faster
            // resolving. The correctness we give up manifests when
            // two transitive non-sibling dependencies conflict. In
            // that case, we don't detect the fork ahead of time (at
            // present).
            if let [dep] = deps.as_slice() {
                // There's one exception: if the requirement increases the minimum-supported Python
                // version, we also fork in order to respect that minimum in the subsequent
                // resolution.
                //
                // For example, given `requires-python = ">=3.7"` and `uv ; python_version >= "3.8"`,
                // where uv itself only supports Python 3.8 and later, we need to fork to ensure
                // that the resolution can find a solution.
                if marker::requires_python(dep.package.marker())
                    .is_none_or(|bound| !python_requirement.raises(&bound))
                {
                    let dep = deps.pop().unwrap();
                    let marker = dep.package.marker();
                    for fork in &mut forks {
                        if fork.env.included_by_marker(marker) {
                            fork.add_dependency(dep.clone());
                        }
                    }
                    continue;
                }
            } else {
                // If all dependencies have the same markers, we should also avoid forking.
                if let Some(dep) = deps.first() {
                    let marker = dep.package.marker();
                    if deps.iter().all(|dep| marker == dep.package.marker()) {
                        // Unless that "same marker" is a Python requirement that is stricter than
                        // the current Python requirement. In that case, we need to fork to respect
                        // the stricter requirement.
                        if marker::requires_python(marker)
                            .is_none_or(|bound| !python_requirement.raises(&bound))
                        {
                            for dep in deps {
                                for fork in &mut forks {
                                    if fork.env.included_by_marker(marker) {
                                        fork.add_dependency(dep.clone());
                                    }
                                }
                            }
                            continue;
                        }
                    }
                }
            }
            for dep in deps {
                let mut forker = match ForkingPossibility::new(env, &dep) {
                    ForkingPossibility::Possible(forker) => forker,
                    ForkingPossibility::DependencyAlwaysExcluded => {
                        // If the markers can never be satisfied by the parent
                        // fork, then we can drop this dependency unceremoniously.
                        continue;
                    }
                    ForkingPossibility::NoForkingPossible => {
                        // Or, if the markers are always true, then we just
                        // add the dependency to every fork unconditionally.
                        for fork in &mut forks {
                            fork.add_dependency(dep.clone());
                        }
                        continue;
                    }
                };
                // Otherwise, we *should* need to add a new fork...
                diverging_packages.insert(name.clone());

                let mut new = vec![];
                for fork in std::mem::take(&mut forks) {
                    let Some((remaining_forker, envs)) = forker.fork(&fork.env) else {
                        new.push(fork);
                        continue;
                    };
                    forker = remaining_forker;

                    for fork_env in envs {
                        let mut new_fork = fork.clone();
                        new_fork.set_env(fork_env);
                        // We only add the dependency to this fork if it
                        // satisfies the fork's markers. Some forks are
                        // specifically created to exclude this dependency,
                        // so this isn't always true!
                        if forker.included(&new_fork.env) {
                            new_fork.add_dependency(dep.clone());
                        }
                        // Filter out any forks we created that are disjoint with our
                        // Python requirement.
                        if new_fork.env.included_by_marker(python_marker) {
                            new.push(new_fork);
                        }
                    }
                }
                forks = new;
            }
        }
        // When there is a conflicting group configuration, we need
        // to potentially add more forks. Each fork added contains an
        // exclusion list of conflicting groups where dependencies with
        // the corresponding package and extra name are forcefully
        // excluded from that group.
        //
        // We specifically iterate on conflicting groups and
        // potentially re-generate all forks for each one. We do it
        // this way in case there are multiple sets of conflicting
        // groups that impact the forks here.
        //
        // For example, if we have conflicting groups {x1, x2} and {x3,
        // x4}, we need to make sure the forks generated from one set
        // also account for the other set.
        for set in conflicts.iter() {
            let mut new = vec![];
            for fork in std::mem::take(&mut forks) {
                // Check if this conflict set is relevant to this fork. We need two conditions:
                //
                // 1. At least one item has dependencies in this fork (otherwise there's nothing to
                //    fork on).
                // 2. At least two items are not already excluded in this fork's environment
                //    (otherwise the conflict constraint is already satisfied and no fork is
                //    needed).
                let mut has_conflicting_dependency = false;
                for item in set.iter() {
                    if fork.contains_conflicting_item(item.as_ref()) {
                        has_conflicting_dependency = true;
                        diverging_packages.insert(item.package().clone());
                        break;
                    }
                }
                if !has_conflicting_dependency {
                    new.push(fork);
                    continue;
                }

                // If fewer than two items in this conflict set are still possible (not already
                // excluded) in this fork, the conflict constraint is already satisfied by prior
                // forking. We can skip the full N+1 fork split if the single remaining non-excluded
                // item doesn't appear in any other conflict set (since it would never need its own
                // "excluded" variant).
                let non_excluded: Vec<_> = set
                    .iter()
                    .filter(|item| fork.env.included_by_group(item.as_ref()))
                    .collect();
                if non_excluded.len() < 2 {
                    // Check if any non-excluded item still has a live conflict in another set —
                    // i.e., another set where this item AND at least one other non-excluded item
                    // both appear. If so, we still need to fork to create the "excluded" variant
                    // for that item.
                    let dominated = non_excluded.iter().all(|item| {
                        !conflicts.iter().any(|other_set| {
                            !std::ptr::eq(set, other_set)
                                && other_set.contains(item.package(), item.kind().as_ref())
                                && other_set
                                    .iter()
                                    .filter(|other_item| {
                                        other_item.package() != item.package()
                                            || other_item.kind() != item.kind()
                                    })
                                    .any(|other_item| {
                                        fork.env.included_by_group(other_item.as_ref())
                                    })
                        })
                    });
                    if dominated {
                        // When dependencies are added to forks, we check `included_by_marker` but
                        // not on whether the dependency's conflict item is included by the fork's
                        // environment so there may be extraneous dependencies and we need to filter
                        // the fork to clean up dependencies gated on already-excluded extras.
                        let rules: Vec<_> = set
                            .iter()
                            .filter(|item| !fork.env.included_by_group(item.as_ref()))
                            .cloned()
                            .map(Err)
                            .collect();
                        if let Some(filtered) = fork.filter(rules) {
                            new.push(filtered);
                        }
                        continue;
                    }
                }

                // Create a fork that excludes ALL conflicts.
                if let Some(fork_none) = fork.clone().filter(set.iter().cloned().map(Err)) {
                    new.push(fork_none);
                }

                // Now create a fork for each conflicting group, where
                // that fork excludes every *other* conflicting group.
                //
                // So if we have conflicting extras foo, bar and baz,
                // then this creates three forks: one that excludes
                // {foo, bar}, one that excludes {foo, baz} and one
                // that excludes {bar, baz}.
                for (i, _) in set.iter().enumerate() {
                    let fork_allows_group = fork.clone().filter(
                        set.iter()
                            .cloned()
                            .enumerate()
                            .map(|(j, group)| if i == j { Ok(group) } else { Err(group) }),
                    );
                    if let Some(fork_allows_group) = fork_allows_group {
                        new.push(fork_allows_group);
                    }
                }
            }
            forks = new;
        }
        Self {
            forks,
            diverging_packages,
        }
    }
}

/// A single fork in a list of dependencies.
///
/// A fork corresponds to the full list of dependencies for a package,
/// but with any conflicting dependency specifications omitted. For
/// example, if we have `a<2 ; sys_platform == 'foo'` and `a>=2 ;
/// sys_platform == 'bar'`, then because the dependency specifications
/// have the same name and because the marker expressions are disjoint,
/// a fork occurs. One fork will contain `a<2` but not `a>=2`, while
/// the other fork will contain `a>=2` but not `a<2`.
#[derive(Clone, Debug)]
struct Fork {
    /// The list of dependencies for this fork, guaranteed to be conflict
    /// free. (i.e., There are no two packages with the same name with
    /// non-overlapping marker expressions.)
    ///
    /// Note that callers shouldn't mutate this sequence directly. Instead,
    /// they should use `add_forked_package` or `add_nonfork_package`. Namely,
    /// it should be impossible for a package with a marker expression that is
    /// disjoint from the marker expression on this fork to be added.
    dependencies: Vec<PubGrubDependency>,
    /// The conflicting groups in this fork.
    ///
    /// This exists to make some access patterns more efficient. Namely,
    /// it makes it easy to check whether there's a dependency with a
    /// particular conflicting group in this fork.
    conflicts: crate::FxHashbrownSet<ConflictItem>,
    /// The resolver environment for this fork.
    ///
    /// Principally, this corresponds to the markers in this for. So in the
    /// example above, the `a<2` fork would have `sys_platform == 'foo'`, while
    /// the `a>=2` fork would have `sys_platform == 'bar'`.
    ///
    /// If this fork was generated from another fork, then this *includes*
    /// the criteria from its parent. i.e., Its marker expression represents
    /// the intersection of the marker expression from its parent and any
    /// additional marker expression generated by addition forking based on
    /// conflicting dependency specifications.
    env: ResolverEnvironment,
}

impl Fork {
    /// Create a new fork with no dependencies with the given resolver
    /// environment.
    fn new(env: ResolverEnvironment) -> Self {
        Self {
            dependencies: vec![],
            conflicts: crate::FxHashbrownSet::default(),
            env,
        }
    }

    /// Add a dependency to this fork.
    fn add_dependency(&mut self, dep: PubGrubDependency) {
        if let Some(conflicting_item) = dep.conflicting_item() {
            self.conflicts.insert(conflicting_item.to_owned());
        }
        self.dependencies.push(dep);
    }

    /// Sets the resolver environment to the one given.
    ///
    /// Any dependency in this fork that does not satisfy the given environment
    /// is removed.
    fn set_env(&mut self, env: ResolverEnvironment) {
        self.env = env;
        self.dependencies.retain(|dep| {
            let marker = dep.package.marker();
            if self.env.included_by_marker(marker) {
                return true;
            }
            if let Some(conflicting_item) = dep.conflicting_item() {
                self.conflicts.remove(&conflicting_item);
            }
            false
        });
    }

    /// Returns true if any of the dependencies in this fork contain a
    /// dependency with the given package and extra values.
    fn contains_conflicting_item(&self, item: ConflictItemRef<'_>) -> bool {
        self.conflicts.contains(&item)
    }

    /// Include or Exclude the given groups from this fork.
    ///
    /// This removes all dependencies matching the given conflicting groups.
    ///
    /// If the exclusion rules would result in a fork with an unsatisfiable
    /// resolver environment, then this returns `None`.
    fn filter(
        mut self,
        rules: impl IntoIterator<Item = Result<ConflictItem, ConflictItem>>,
    ) -> Option<Self> {
        self.env = self.env.filter_by_group(rules)?;
        self.dependencies.retain(|dep| {
            let Some(conflicting_item) = dep.conflicting_item() else {
                return true;
            };
            if self.env.included_by_group(conflicting_item) {
                return true;
            }
            match conflicting_item.kind() {
                // We should not filter entire projects unless they're a top-level dependency
                // Otherwise, we'll fail to solve for children of the project, like extras
                ConflictKindRef::Project => {
                    if dep.parent.is_some() {
                        return true;
                    }
                }
                ConflictKindRef::Group(_) => {}
                ConflictKindRef::Extra(_) => {}
            }
            self.conflicts.remove(&conflicting_item);
            false
        });
        Some(self)
    }

    /// Compare forks, preferring forks with g `requires-python` requirements.
    fn cmp_requires_python(&self, other: &Self) -> Ordering {
        // A higher `requires-python` requirement indicates a _higher-priority_ fork.
        //
        // This ordering ensures that we prefer choosing the highest version for each fork based on
        // its `requires-python` requirement.
        //
        // The reverse would prefer choosing fewer versions, at the cost of using older package
        // versions on newer Python versions. For example, if reversed, we'd prefer to solve `<3.7
        // before solving `>=3.7`, since the resolution produced by the former might work for the
        // latter, but the inverse is unlikely to be true.
        let self_bound = self.env.requires_python().unwrap_or_default();
        let other_bound = other.env.requires_python().unwrap_or_default();
        self_bound.lower().cmp(other_bound.lower())
    }

    /// Compare forks, preferring forks with upper bounds.
    fn cmp_upper_bounds(&self, other: &Self) -> Ordering {
        // We'd prefer to solve `numpy <= 2` before solving `numpy >= 1`, since the resolution
        // produced by the former might work for the latter, but the inverse is unlikely to be true
        // due to maximum version selection. (Selecting `numpy==2.0.0` would satisfy both forks, but
        // selecting the latest `numpy` would not.)
        let self_upper_bounds = self
            .dependencies
            .iter()
            .filter(|dep| {
                dep.version
                    .bounding_range()
                    .is_some_and(|(_, upper)| !matches!(upper, Bound::Unbounded))
            })
            .count();
        let other_upper_bounds = other
            .dependencies
            .iter()
            .filter(|dep| {
                dep.version
                    .bounding_range()
                    .is_some_and(|(_, upper)| !matches!(upper, Bound::Unbounded))
            })
            .count();

        self_upper_bounds.cmp(&other_upper_bounds)
    }
}

impl Eq for Fork {}

impl PartialEq for Fork {
    fn eq(&self, other: &Self) -> bool {
        self.dependencies == other.dependencies && self.env == other.env
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VersionFork {
    /// The environment to use in the fork.
    env: ResolverEnvironment,
    /// The initial package to select in the fork.
    id: Id<PubGrubPackage>,
    /// The initial version to set for the selected package in the fork.
    version: Option<Version>,
}

/// Enrich a [`ResolveError`] with additional information about why a given package was included.
fn enrich_dependency_error(
    error: ResolveError,
    id: Id<PubGrubPackage>,
    version: &Version,
    pubgrub: &State<UvDependencyProvider>,
) -> ResolveError {
    let Some(name) = pubgrub.package_store[id].name_no_root() else {
        return error;
    };
    let chain = DerivationChainBuilder::from_state(id, version, pubgrub).unwrap_or_default();
    ResolveError::Dependencies(Box::new(error), name.clone(), version.clone(), chain)
}

/// Compute the set of markers for which a package is known to be relevant.
fn find_environments(id: Id<PubGrubPackage>, state: &State<UvDependencyProvider>) -> MarkerTree {
    let package = &state.package_store[id];
    if package.is_root() {
        return MarkerTree::TRUE;
    }

    // Retrieve the incompatibilities for the current package.
    let Some(incompatibilities) = state.incompatibilities.get(&id) else {
        return MarkerTree::FALSE;
    };

    // Find all dependencies on the current package.
    let mut marker = MarkerTree::FALSE;
    for index in incompatibilities {
        let incompat = &state.incompatibility_store[*index];
        if let Kind::FromDependencyOf(id1, _, id2, _) = &incompat.kind {
            if id == *id2 {
                marker.or({
                    let mut marker = package.marker();
                    marker.and(find_environments(*id1, state));
                    marker
                });
            }
        }
    }
    marker
}

#[derive(Debug, Default, Clone)]
struct ConflictTracker {
    /// How often a decision on the package was discarded due to another package decided earlier.
    affected: FxHashMap<Id<PubGrubPackage>, usize>,
    /// Package(s) to be prioritized after the next unit propagation
    ///
    /// Distilled from `affected` for fast checking in the hot loop.
    prioritize: Vec<Id<PubGrubPackage>>,
    /// How often a package was decided earlier and caused another package to be discarded.
    culprit: FxHashMap<Id<PubGrubPackage>, usize>,
    /// Package(s) to be de-prioritized after the next unit propagation
    ///
    /// Distilled from `culprit` for fast checking in the hot loop.
    deprioritize: Vec<Id<PubGrubPackage>>,
}
