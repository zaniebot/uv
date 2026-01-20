# Build Backend

uv's native build backend - building sdists and wheels, module discovery, metadata handling.

## Basic Building

- [x] build-backend/build.md#direct-wheel (from build_backend.rs::built_by_uv_direct_wheel)
- [x] build-backend/build.md#direct (from build_backend.rs::built_by_uv_direct)
- [x] build-backend/build.md#editable (from build_backend.rs::built_by_uv_editable)

## Module Discovery

- [x] build-backend/module.md#rename (from build_backend.rs::rename_module)
- [x] build-backend/module.md#rename-editable (from build_backend.rs::rename_module_editable_build)
- [x] build-backend/module.md#normalization (from build_backend.rs::build_module_name_normalization)
- [x] build-backend/module.md#complex-namespace (from build_backend.rs::complex_namespace_packages)
- [x] build-backend/errors.md#missing-module (from build_backend.rs::sdist_error_without_module)
- [x] build-backend/edge-cases.md#redundant-module-names (from
      build_backend.rs::warn_on_redundant_module_names)

## Metadata and Licensing

- [x] build-backend/metadata.md#all-metadata (from build_backend.rs::build_with_all_metadata)
- [x] build-backend/errors.md#license-glob (from
      build_backend.rs::license_glob_without_matches_errors)
- [x] build-backend/errors.md#license-utf8 (from build_backend.rs::license_file_must_be_utf8)

## Edge Cases

- [ ] BLOCKED: preserve-executable-bit (from build_backend.rs::preserve_executable_bit) - requires
      git feature
- [x] build-backend/edge-cases.md#long-path (from build_backend.rs::build_sdist_with_long_path)
- [x] build-backend/edge-cases.md#symlinked-file (from build_backend.rs::symlinked_file)
- [x] build-backend/edge-cases.md#venv-in-source (from build_backend.rs::venv_in_source_tree)

## Error Handling

- [x] build-backend/edge-cases.md#invalid-settings (from
      build_backend.rs::invalid_build_backend_settings_are_ignored)
- [x] build-backend/errors.md#module-root-outside (from
      build_backend.rs::error_on_relative_module_root_outside_project_root)
- [x] build-backend/errors.md#data-dir-outside (from
      build_backend.rs::error_on_relative_data_dir_outside_project_root)
- [x] build-backend/errors.md#invalid-pyproject (from build_backend.rs::invalid_pyproject_toml)
