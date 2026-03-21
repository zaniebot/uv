use super::*;

impl<'a, T: BuildContext> SourceDistributionBuilder<'a, T> {
    pub(super) async fn git(
        &self,
        source: &BuildableSource<'_>,
        resource: &GitSourceUrl<'_>,
        tags: &Tags,
        hashes: HashPolicy<'_>,
        client: &ManagedClient<'_>,
    ) -> Result<BuiltWheelMetadata, Error> {
        // Before running the build, check that the hashes match.
        if hashes.is_validate() {
            return Err(Error::HashesNotSupportedGit(source.to_string()));
        }

        // Fetch the Git repository.
        let fetch = self
            .build_context
            .git()
            .fetch(
                resource.git,
                client.unmanaged.disable_ssl(resource.git.repository()),
                client.unmanaged.connectivity() == Connectivity::Offline,
                self.build_context.cache().bucket(CacheBucket::Git),
                self.reporter
                    .clone()
                    .map(|reporter| reporter.into_git_reporter()),
            )
            .await?;

        // Validate that the subdirectory exists.
        if let Some(subdirectory) = resource.subdirectory {
            if !fetch.path().join(subdirectory).is_dir() {
                return Err(Error::MissingSubdirectory(
                    resource.url.to_url(),
                    subdirectory.to_path_buf(),
                ));
            }
        }

        // Validate that LFS artifacts were fully initialized
        if resource.git.lfs().enabled() && !fetch.lfs_ready() {
            if GIT_LFS.is_err() {
                return Err(Error::MissingGitLfsArtifacts(
                    resource.url.to_url(),
                    GitError::GitLfsNotFound,
                ));
            }
            return Err(Error::MissingGitLfsArtifacts(
                resource.url.to_url(),
                GitError::GitLfsNotConfigured,
            ));
        }

        let git_sha = fetch.git().precise().expect("Exact commit after checkout");
        let cache_shard = self.build_context.cache().shard(
            CacheBucket::SourceDistributions,
            WheelCache::Git(resource.url, git_sha.as_short_str()).root(),
        );
        let metadata_entry = cache_shard.entry(METADATA);

        // Acquire the advisory lock.
        let _lock = cache_shard.lock().await.map_err(Error::CacheLock)?;

        // We don't track any cache information for Git-based source distributions; they're assumed
        // to be immutable.
        let cache_info = CacheInfo::default();

        // We don't compute hashes for Git-based source distributions, since the Git commit SHA is
        // used as the identifier.
        let hashes = HashDigests::empty();

        // If there are build settings or extra build dependencies, we need to scope to a cache shard.
        let config_settings = self.config_settings_for(source.name());
        let extra_build_deps = self.extra_build_dependencies_for(source.name());
        let extra_build_variables = self.extra_build_variables_for(source.name());
        let build_info =
            BuildInfo::from_settings(&config_settings, extra_build_deps, extra_build_variables);
        let cache_shard = build_info
            .cache_shard()
            .map(|digest| cache_shard.shard(digest))
            .unwrap_or(cache_shard);

        // If the cache contains a compatible wheel, return it.
        if let Some(file) = BuiltWheelFile::find_in_cache(tags, &cache_shard)
            .ok()
            .flatten()
            .filter(|file| file.matches(source.name(), source.version()))
        {
            return Ok(BuiltWheelMetadata::from_file(
                file, hashes, cache_info, build_info,
            ));
        }

        let task = self
            .reporter
            .as_ref()
            .map(|reporter| reporter.on_build_start(source));

        let (disk_filename, filename, metadata) = self
            .build_distribution(
                source,
                fetch.path(),
                resource.subdirectory,
                &cache_shard,
                self.build_context.sources().clone(),
            )
            .await?;

        if let Some(task) = task {
            if let Some(reporter) = self.reporter.as_ref() {
                reporter.on_build_complete(source, task);
            }
        }

        // Store the metadata.
        write_atomic(metadata_entry.path(), rmp_serde::to_vec(&metadata)?)
            .await
            .map_err(Error::CacheWrite)?;

        Ok(BuiltWheelMetadata {
            path: cache_shard.join(&disk_filename).into_boxed_path(),
            target: cache_shard.join(filename.stem()).into_boxed_path(),
            filename,
            hashes,
            cache_info,
            build_info,
        })
    }

    /// Build the source distribution's metadata from a Git repository.
    ///
    /// If the build backend supports `prepare_metadata_for_build_wheel`, this method will avoid
    /// building the wheel.
    pub(super) async fn git_metadata(
        &self,
        source: &BuildableSource<'_>,
        resource: &GitSourceUrl<'_>,
        hashes: HashPolicy<'_>,
        client: &ManagedClient<'_>,
        credentials_cache: &CredentialsCache,
    ) -> Result<ArchiveMetadata, Error> {
        // Before running the build, check that the hashes match.
        if hashes.is_validate() {
            return Err(Error::HashesNotSupportedGit(source.to_string()));
        }

        // If the reference appears to be a commit, and we've already checked it out, avoid taking
        // the GitHub fast path.
        let cache_shard = resource
            .git
            .reference()
            .as_str()
            .and_then(|reference| GitOid::from_str(reference).ok())
            .map(|oid| {
                self.build_context.cache().shard(
                    CacheBucket::SourceDistributions,
                    WheelCache::Git(resource.url, oid.as_short_str()).root(),
                )
            });
        if cache_shard
            .as_ref()
            .is_some_and(|cache_shard| cache_shard.is_dir())
        {
            debug!("Skipping GitHub fast path for: {source} (shard exists)");
        } else {
            debug!("Attempting GitHub fast path for: {source}");

            // If this is GitHub URL, attempt to resolve to a precise commit using the GitHub API.
            match self
                .build_context
                .git()
                .github_fast_path(
                    resource.git,
                    client
                        .unmanaged
                        .uncached_client(resource.git.repository())
                        .raw_client(),
                )
                .await
            {
                Ok(Some(precise)) => {
                    // There's no need to check the cache, since we can't use cached metadata if there are
                    // sources, and we can't know if there are sources without fetching the
                    // `pyproject.toml`.
                    //
                    // For the same reason, there's no need to write to the cache, since we won't be able to
                    // use it on subsequent runs.
                    match self
                        .github_metadata(precise, source, resource, client)
                        .await
                    {
                        Ok(Some(metadata)) => {
                            // Validate the metadata, but ignore it if the metadata doesn't match.
                            match validate_metadata(source, &metadata) {
                                Ok(()) => {
                                    debug!(
                                        "Found static metadata via GitHub fast path for: {source}"
                                    );
                                    return Ok(ArchiveMetadata {
                                        metadata: Metadata::from_metadata23(metadata),
                                        hashes: HashDigests::empty(),
                                    });
                                }
                                Err(err) => {
                                    debug!(
                                        "Ignoring `pyproject.toml` from GitHub for {source}: {err}"
                                    );
                                }
                            }
                        }
                        Ok(None) => {
                            // Nothing to do.
                        }
                        Err(err) => {
                            debug!(
                                "Failed to fetch `pyproject.toml` via GitHub fast path for: {source} ({err})"
                            );
                        }
                    }
                }
                Ok(None) => {
                    // Nothing to do.
                }
                Err(err) => {
                    debug!("Failed to resolve commit via GitHub fast path for: {source} ({err})");
                }
            }
        }

        // Fetch the Git repository.
        let fetch = self
            .build_context
            .git()
            .fetch(
                resource.git,
                client.unmanaged.disable_ssl(resource.git.repository()),
                client.unmanaged.connectivity() == Connectivity::Offline,
                self.build_context.cache().bucket(CacheBucket::Git),
                self.reporter
                    .clone()
                    .map(|reporter| reporter.into_git_reporter()),
            )
            .await?;

        // Validate that the subdirectory exists.
        if let Some(subdirectory) = resource.subdirectory {
            if !fetch.path().join(subdirectory).is_dir() {
                return Err(Error::MissingSubdirectory(
                    resource.url.to_url(),
                    subdirectory.to_path_buf(),
                ));
            }
        }

        // Validate that LFS artifacts were fully initialized
        if resource.git.lfs().enabled() && !fetch.lfs_ready() {
            if GIT_LFS.is_err() {
                return Err(Error::MissingGitLfsArtifacts(
                    resource.url.to_url(),
                    GitError::GitLfsNotFound,
                ));
            }
            return Err(Error::MissingGitLfsArtifacts(
                resource.url.to_url(),
                GitError::GitLfsNotConfigured,
            ));
        }

        let git_sha = fetch.git().precise().expect("Exact commit after checkout");
        let cache_shard = self.build_context.cache().shard(
            CacheBucket::SourceDistributions,
            WheelCache::Git(resource.url, git_sha.as_short_str()).root(),
        );
        let metadata_entry = cache_shard.entry(METADATA);

        // Acquire the advisory lock.
        let _lock = cache_shard.lock().await.map_err(Error::CacheLock)?;

        let path = if let Some(subdirectory) = resource.subdirectory {
            Cow::Owned(fetch.path().join(subdirectory))
        } else {
            Cow::Borrowed(fetch.path())
        };

        let git_member = GitWorkspaceMember {
            fetch_root: fetch.path(),
            git_source: resource,
        };

        // If the metadata is static, return it.
        let dynamic =
            match StaticMetadata::read(source, fetch.path(), resource.subdirectory).await? {
                StaticMetadata::Some(metadata) => {
                    return Ok(ArchiveMetadata::from(
                        Metadata::from_workspace(
                            metadata,
                            &path,
                            Some(&git_member),
                            self.build_context.locations(),
                            self.build_context.sources().clone(),
                            self.build_context.workspace_cache(),
                            credentials_cache,
                        )
                        .await?,
                    ));
                }
                StaticMetadata::Dynamic => true,
                StaticMetadata::None => false,
            };

        // If the cache contains compatible metadata, return it.
        if self
            .build_context
            .cache()
            .freshness(&metadata_entry, source.name(), source.source_tree())
            .map_err(Error::CacheRead)?
            .is_fresh()
        {
            match CachedMetadata::read(&metadata_entry).await {
                Ok(Some(metadata)) => {
                    if metadata.matches(source.name(), source.version()) {
                        debug!("Using cached metadata for: {source}");

                        let git_member = GitWorkspaceMember {
                            fetch_root: fetch.path(),
                            git_source: resource,
                        };
                        return Ok(ArchiveMetadata::from(
                            Metadata::from_workspace(
                                metadata.into(),
                                &path,
                                Some(&git_member),
                                self.build_context.locations(),
                                self.build_context.sources().clone(),
                                self.build_context.workspace_cache(),
                                credentials_cache,
                            )
                            .await?,
                        ));
                    }
                    debug!(
                        "Cached metadata does not match expected name and version for: {source}"
                    );
                }
                Ok(None) => {}
                Err(err) => {
                    debug!("Failed to deserialize cached metadata for: {source} ({err})");
                }
            }
        }

        // If the backend supports `prepare_metadata_for_build_wheel`, use it.
        if let Some(metadata) = self
            .build_metadata(
                source,
                fetch.path(),
                resource.subdirectory,
                self.build_context.sources().clone(),
            )
            .boxed_local()
            .await?
        {
            // If necessary, mark the metadata as dynamic.
            let metadata = if dynamic {
                ResolutionMetadata {
                    dynamic: true,
                    ..metadata
                }
            } else {
                metadata
            };

            // Store the metadata.
            fs::create_dir_all(metadata_entry.dir())
                .await
                .map_err(Error::CacheWrite)?;
            write_atomic(metadata_entry.path(), rmp_serde::to_vec(&metadata)?)
                .await
                .map_err(Error::CacheWrite)?;

            return Ok(ArchiveMetadata::from(
                Metadata::from_workspace(
                    metadata,
                    &path,
                    Some(&git_member),
                    self.build_context.locations(),
                    self.build_context.sources().clone(),
                    self.build_context.workspace_cache(),
                    credentials_cache,
                )
                .await?,
            ));
        }

        // If there are build settings or extra build dependencies, we need to scope to a cache shard.
        let config_settings = self.config_settings_for(source.name());
        let extra_build_deps = self.extra_build_dependencies_for(source.name());
        let extra_build_variables = self.extra_build_variables_for(source.name());
        let build_info =
            BuildInfo::from_settings(&config_settings, extra_build_deps, extra_build_variables);
        let cache_shard = build_info
            .cache_shard()
            .map(|digest| cache_shard.shard(digest))
            .unwrap_or(cache_shard);

        // Otherwise, we need to build a wheel.
        let task = self
            .reporter
            .as_ref()
            .map(|reporter| reporter.on_build_start(source));

        let (_disk_filename, _filename, metadata) = self
            .build_distribution(
                source,
                fetch.path(),
                resource.subdirectory,
                &cache_shard,
                self.build_context.sources().clone(),
            )
            .await?;

        if let Some(task) = task {
            if let Some(reporter) = self.reporter.as_ref() {
                reporter.on_build_complete(source, task);
            }
        }

        // If necessary, mark the metadata as dynamic.
        let metadata = if dynamic {
            ResolutionMetadata {
                dynamic: true,
                ..metadata
            }
        } else {
            metadata
        };

        // Store the metadata.
        write_atomic(metadata_entry.path(), rmp_serde::to_vec(&metadata)?)
            .await
            .map_err(Error::CacheWrite)?;

        Ok(ArchiveMetadata::from(
            Metadata::from_workspace(
                metadata,
                fetch.path(),
                Some(&git_member),
                self.build_context.locations(),
                self.build_context.sources().clone(),
                self.build_context.workspace_cache(),
                credentials_cache,
            )
            .await?,
        ))
    }

    /// Resolve a source to a specific revision.
    pub(crate) async fn resolve_revision(
        &self,
        source: &BuildableSource<'_>,
        client: &ManagedClient<'_>,
    ) -> Result<Option<GitOid>, Error> {
        let git = match source {
            BuildableSource::Dist(SourceDist::Git(source)) => &*source.git,
            BuildableSource::Url(SourceUrl::Git(source)) => source.git,
            _ => {
                return Ok(None);
            }
        };

        // If the URL is already precise, return it.
        if let Some(precise) = self.build_context.git().get_precise(git) {
            debug!("Precise commit already known: {source}");
            return Ok(Some(precise));
        }

        // If this is GitHub URL, attempt to resolve to a precise commit using the GitHub API.
        if let Some(precise) = self
            .build_context
            .git()
            .github_fast_path(
                git,
                client
                    .unmanaged
                    .uncached_client(git.repository())
                    .raw_client(),
            )
            .await?
        {
            debug!("Resolved to precise commit via GitHub fast path: {source}");
            return Ok(Some(precise));
        }

        // Otherwise, fetch the Git repository.
        let fetch = self
            .build_context
            .git()
            .fetch(
                git,
                client.unmanaged.disable_ssl(git.repository()),
                client.unmanaged.connectivity() == Connectivity::Offline,
                self.build_context.cache().bucket(CacheBucket::Git),
                self.reporter
                    .clone()
                    .map(|reporter| reporter.into_git_reporter()),
            )
            .await?;

        Ok(fetch.git().precise())
    }

    /// Fetch static [`ResolutionMetadata`] from a GitHub repository, if possible.
    ///
    /// Attempts to fetch the `pyproject.toml` from the resolved commit using the GitHub API.
    pub(super) async fn github_metadata(
        &self,
        commit: GitOid,
        source: &BuildableSource<'_>,
        resource: &GitSourceUrl<'_>,
        client: &ManagedClient<'_>,
    ) -> Result<Option<ResolutionMetadata>, Error> {
        let GitSourceUrl {
            git, subdirectory, ..
        } = resource;

        // The fast path isn't available for subdirectories. If a `pyproject.toml` is in a
        // subdirectory, it could be part of a workspace; and if it's part of a workspace, it could
        // have `tool.uv.sources` entries that it inherits from the workspace root.
        if subdirectory.is_some() {
            return Ok(None);
        }

        let Some(GitHubRepository { owner, repo }) = GitHubRepository::parse(git.repository())
        else {
            return Ok(None);
        };

        // Fetch the `pyproject.toml` from the resolved commit.
        let url =
            format!("https://raw.githubusercontent.com/{owner}/{repo}/{commit}/pyproject.toml");

        debug!("Attempting to fetch `pyproject.toml` from: {url}");

        let content = client
            .managed(async |client| {
                let response = client
                    .uncached_client(git.repository())
                    .get(&url)
                    .send()
                    .await?;

                // If the `pyproject.toml` does not exist, the GitHub API will return a 404.
                if response.status() == StatusCode::NOT_FOUND {
                    return Ok::<Option<String>, Error>(None);
                }
                response.error_for_status_ref()?;

                let content = response.text().await?;
                Ok::<Option<String>, Error>(Some(content))
            })
            .await?;

        let Some(content) = content else {
            debug!("GitHub API returned a 404 for: {url}");
            return Ok(None);
        };

        // Parse the `pyproject.toml`.
        let pyproject_toml = match PyProjectToml::from_toml(&content, source) {
            Ok(metadata) => metadata,
            Err(
                uv_pypi_types::MetadataError::InvalidPyprojectTomlSyntax(..)
                | uv_pypi_types::MetadataError::InvalidPyprojectTomlSchema(..),
            ) => {
                debug!("Failed to read `pyproject.toml` from GitHub API for: {url}");
                return Ok(None);
            }
            Err(err) => return Err(err.into()),
        };

        // Parse the metadata.
        let metadata =
            match ResolutionMetadata::parse_pyproject_toml(pyproject_toml, source.version()) {
                Ok(metadata) => metadata,
                Err(
                    uv_pypi_types::MetadataError::Pep508Error(..)
                    | uv_pypi_types::MetadataError::DynamicField(..)
                    | uv_pypi_types::MetadataError::FieldNotFound(..)
                    | uv_pypi_types::MetadataError::PoetrySyntax,
                ) => {
                    debug!("Failed to extract static metadata from GitHub API for: {url}");
                    return Ok(None);
                }
                Err(err) => return Err(err.into()),
            };

        // Determine whether the project has `tool.uv.sources`. If the project has sources, it must
        // be lowered, which requires access to the workspace. For example, it could have workspace
        // members that need to be translated to concrete paths on disk.
        //
        // TODO(charlie): We could still use the `pyproject.toml` if the sources are all `git` or
        // `url` sources; this is only applicable to `workspace` and `path` sources. It's awkward,
        // though, because we'd need to pass a path into the lowering routine, and that path would
        // be incorrect (we'd just be relying on it not being used).
        match has_sources(&content) {
            Ok(false) => {}
            Ok(true) => {
                debug!("Skipping GitHub fast path; `pyproject.toml` has sources: {url}");
                return Ok(None);
            }
            Err(err) => {
                debug!("Failed to parse `tool.uv.sources` from GitHub API for: {url} ({err})");
                return Ok(None);
            }
        }

        Ok(Some(metadata))
    }
}
