use super::*;

impl<'a, T: BuildContext> SourceDistributionBuilder<'a, T> {
    pub(super) fn config_settings_for(
        &self,
        name: Option<&PackageName>,
    ) -> Cow<'_, ConfigSettings> {
        if let Some(name) = name {
            if let Some(package_settings) = self.build_context.config_settings_package().get(name) {
                Cow::Owned(
                    package_settings
                        .clone()
                        .merge(self.build_context.config_settings().clone()),
                )
            } else {
                Cow::Borrowed(self.build_context.config_settings())
            }
        } else {
            Cow::Borrowed(self.build_context.config_settings())
        }
    }

    /// Determine the extra build dependencies for the given package name.
    pub(super) fn extra_build_dependencies_for(
        &self,
        name: Option<&PackageName>,
    ) -> &[ExtraBuildRequirement] {
        name.and_then(|name| {
            self.build_context
                .extra_build_requires()
                .get(name)
                .map(Vec::as_slice)
        })
        .unwrap_or(&[])
    }

    /// Determine the extra build variables for the given package name.
    pub(super) fn extra_build_variables_for(
        &self,
        name: Option<&PackageName>,
    ) -> Option<&BuildVariables> {
        name.and_then(|name| self.build_context.extra_build_variables().get(name))
    }

    /// Build a source distribution from a remote URL.
    pub(super) async fn build_distribution(
        &self,
        source: &BuildableSource<'_>,
        source_root: &Path,
        subdirectory: Option<&Path>,
        cache_shard: &CacheShard,
        no_sources: NoSources,
    ) -> Result<(String, WheelFilename, ResolutionMetadata), Error> {
        debug!("Building: {source}");

        // Guard against build of source distributions when disabled.
        if self
            .build_context
            .build_options()
            .no_build_requirement(source.name())
        {
            if source.is_editable() {
                debug!("Allowing build for editable source distribution: {source}");
            } else {
                return Err(Error::NoBuild);
            }
        }

        // Build into a temporary directory, to prevent partial builds.
        let temp_dir = self
            .build_context
            .cache()
            .build_dir()
            .map_err(Error::CacheWrite)?;

        // Build the wheel.
        fs::create_dir_all(&cache_shard)
            .await
            .map_err(Error::CacheWrite)?;

        // Try a direct build if that isn't disabled and the uv build backend is used.
        let disk_filename = if let Some(name) = self
            .build_context
            .direct_build(
                source_root,
                subdirectory,
                temp_dir.path(),
                no_sources.clone(),
                if source.is_editable() {
                    BuildKind::Editable
                } else {
                    BuildKind::Wheel
                },
                Some(&source.to_string()),
            )
            .await
            .map_err(|err| Error::Build(err.into()))?
        {
            // In the uv build backend, the normalized filename and the disk filename are the same.
            name.to_string()
        } else {
            // Identify the base Python interpreter to use in the cache key.
            let base_python = if cfg!(unix) {
                self.build_context
                    .interpreter()
                    .await
                    .find_base_python()
                    .map_err(Error::BaseInterpreter)?
            } else {
                self.build_context
                    .interpreter()
                    .await
                    .to_base_python()
                    .map_err(Error::BaseInterpreter)?
            };

            let build_kind = if source.is_editable() {
                BuildKind::Editable
            } else {
                BuildKind::Wheel
            };

            let build_key = BuildKey {
                base_python: base_python.into_boxed_path(),
                source_root: source_root.to_path_buf().into_boxed_path(),
                subdirectory: subdirectory
                    .map(|subdirectory| subdirectory.to_path_buf().into_boxed_path()),
                no_sources: no_sources.clone(),
                build_kind,
            };

            if let Some(builder) = self.build_context.build_arena().remove(&build_key) {
                debug!("Creating build environment for: {source}");
                let wheel = builder.wheel(temp_dir.path()).await.map_err(Error::Build)?;

                // Store the build context.
                self.build_context.build_arena().insert(build_key, builder);

                wheel
            } else {
                debug!("Reusing existing build environment for: {source}");

                let builder = self
                    .build_context
                    .setup_build(
                        source_root,
                        subdirectory,
                        source_root,
                        Some(&source.to_string()),
                        source.as_dist(),
                        &no_sources,
                        if source.is_editable() {
                            BuildKind::Editable
                        } else {
                            BuildKind::Wheel
                        },
                        if uv_flags::contains(uv_flags::EnvironmentFlags::HIDE_BUILD_OUTPUT) {
                            BuildOutput::Quiet
                        } else {
                            BuildOutput::Debug
                        },
                        self.build_stack.cloned().unwrap_or_default(),
                    )
                    .await
                    .map_err(|err| Error::Build(err.into()))?;

                // Build the wheel.
                let wheel = builder.wheel(temp_dir.path()).await.map_err(Error::Build)?;

                // Store the build context.
                self.build_context.build_arena().insert(build_key, builder);

                wheel
            }
        };

        // Read the metadata from the wheel.
        let filename = WheelFilename::from_str(&disk_filename)?;
        let metadata = read_wheel_metadata(&filename, &temp_dir.path().join(&disk_filename))?;

        // Validate the metadata.
        validate_metadata(source, &metadata)?;
        validate_filename(&filename, &metadata)?;

        // Move the wheel to the cache.
        rename_with_retry(
            temp_dir.path().join(&disk_filename),
            cache_shard.join(&disk_filename),
        )
        .await
        .map_err(Error::CacheWrite)?;

        debug!("Built `{source}` into `{disk_filename}`");
        Ok((disk_filename, filename, metadata))
    }

    /// Build the metadata for a source distribution.
    #[instrument(skip_all, fields(dist = %source))]
    pub(super) async fn build_metadata(
        &self,
        source: &BuildableSource<'_>,
        source_root: &Path,
        subdirectory: Option<&Path>,
        no_sources: NoSources,
    ) -> Result<Option<ResolutionMetadata>, Error> {
        debug!("Preparing metadata for: {source}");

        // Ensure that the _installed_ Python version is compatible with the `requires-python`
        // specifier.
        if let Some(requires_python) = source.requires_python() {
            let installed = self.build_context.interpreter().await.python_version();
            let target = release_specifiers_to_ranges(requires_python.clone())
                .bounding_range()
                .map(|bounding_range| bounding_range.0.cloned())
                .unwrap_or(Bound::Unbounded);
            let is_compatible = match target {
                Bound::Included(target) => *installed >= target,
                Bound::Excluded(target) => *installed > target,
                Bound::Unbounded => true,
            };
            if !is_compatible {
                return Err(Error::RequiresPython(
                    requires_python.clone(),
                    installed.clone(),
                ));
            }
        }

        // Identify the base Python interpreter to use in the cache key.
        let base_python = if cfg!(unix) {
            self.build_context
                .interpreter()
                .await
                .find_base_python()
                .map_err(Error::BaseInterpreter)?
        } else {
            self.build_context
                .interpreter()
                .await
                .to_base_python()
                .map_err(Error::BaseInterpreter)?
        };

        // Determine whether this is an editable or non-editable build.
        let build_kind = if source.is_editable() {
            BuildKind::Editable
        } else {
            BuildKind::Wheel
        };

        // Set up the builder.
        let mut builder = self
            .build_context
            .setup_build(
                source_root,
                subdirectory,
                source_root,
                Some(&source.to_string()),
                source.as_dist(),
                &no_sources,
                build_kind,
                if uv_flags::contains(uv_flags::EnvironmentFlags::HIDE_BUILD_OUTPUT) {
                    BuildOutput::Quiet
                } else {
                    BuildOutput::Debug
                },
                self.build_stack.cloned().unwrap_or_default(),
            )
            .await
            .map_err(|err| Error::Build(err.into()))?;

        // Build the metadata.
        let dist_info = builder.metadata().await.map_err(Error::Build)?;

        // Store the build context.
        self.build_context.build_arena().insert(
            BuildKey {
                base_python: base_python.into_boxed_path(),
                source_root: source_root.to_path_buf().into_boxed_path(),
                subdirectory: subdirectory
                    .map(|subdirectory| subdirectory.to_path_buf().into_boxed_path()),
                no_sources,
                build_kind,
            },
            builder,
        );

        // Return the `.dist-info` directory, if it exists.
        let Some(dist_info) = dist_info else {
            return Ok(None);
        };

        // Read the metadata from disk.
        debug!("Prepared metadata for: {source}");
        let content = fs::read(dist_info.join("METADATA"))
            .await
            .map_err(Error::CacheRead)?;
        let metadata = ResolutionMetadata::parse_metadata(&content)?;

        // Validate the metadata.
        validate_metadata(source, &metadata)?;

        Ok(Some(metadata))
    }
}
