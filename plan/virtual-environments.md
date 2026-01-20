# Virtual Environments

Creating and managing virtual environments.

## Test File Organization

- `commands/uv-venv.md` - Main venv command tests
- `python-version-files.md` - `.python-version` and `.python-versions` file tests
- `settings/environment-variables.md` - Environment variable parsing tests
- `projects/custom-environment-path.md` - `UV_PROJECT_ENVIRONMENT` tests

## Basic Creation

- [x] commands/uv-venv.md#creating-a-virtual-environment (from venv.rs::create_venv)
- [x] commands/uv-venv.md#creating-with-python-313 (from venv.rs::create_venv_313)
- [x] commands/uv-venv.md#creating-with-specific-python-patch-version (from
      venv.rs::create_venv_python_patch) - Feature-gated with `required-features = "python-patch"`
- [x] projects/custom-environment-path.md (from venv.rs::create_venv_project_environment) - 4 tests
- [x] commands/uv-venv.md#projects-without-a-project-section (from venv.rs::virtual_empty)
- [x] commands/uv-venv.md#virtual-projects-with-dependency-groups (from
      venv.rs::virtual_dependency_group)
- [x] commands/uv-venv.md#default-location (from venv.rs::create_venv_defaults_to_cwd)

## Working Directory

- [x] commands/uv-venv.md#creating-a-virtual-environment-in-the-current-directory-unix (from
      venv.rs::create_venv_current_working_directory)
- [x] commands/uv-venv.md#creating-a-virtual-environment-in-the-current-directory-windows (from
      venv.rs::create_venv_current_working_directory) - Platform-gated

## Python Version Discovery

- [x] python-version-files.md#reading-python-version-file (from
      venv.rs::create_venv_reads_request_from_python_version_file)
- [x] python-version-files.md#reading-python-versions-file (from
      venv.rs::create_venv_reads_request_from_python_versions_file)
- [x] commands/uv-venv.md#respecting-pyproject-requires-python (from
      venv.rs::create_venv_respects_pyproject_requires_python)
- [x] commands/uv-venv.md#respecting-group-requires-python (from
      venv.rs::create_venv_respects_group_requires_python) - Simplified version
- [x] commands/uv-venv.md#missing-pyproject-toml-metadata-is-ignored (from
      venv.rs::create_venv_ignores_missing_pyproject_metadata)
- [x] commands/uv-venv.md#invalid-pyproject-toml-produces-a-warning (from
      venv.rs::create_venv_warns_user_on_requires_python_discovery_error)
- [x] python-version-files.md#explicit-python-overrides-python-version-file (from
      venv.rs::create_venv_explicit_request_takes_priority_over_python_version_file)
- [x] commands/uv-venv.md#virtual-env-is-ignored (from
      venv.rs::create_venv_ignores_virtual_env_variable)

## Python Version Selection

- [x] commands/uv-venv.md#unknown-python-minor-version (from
      venv.rs::create_venv_unknown_python_minor)
- [x] commands/uv-venv.md#unknown-python-patch-version (from
      venv.rs::create_venv_unknown_python_patch)
- [x] commands/uv-venv.md#python-preference-managed-vs-system (from venv.rs::venv_python_preference)

## Seeding

- [x] commands/uv-venv.md#seed-packages (from venv.rs::seed)
- [x] commands/uv-venv.md#seed-packages-with-older-python-version (from
      venv.rs::seed_older_python_version)

## Existing Directory Handling

- [x] commands/uv-venv.md#file-already-exists-at-target-path (from venv.rs::file_exists)
- [x] commands/uv-venv.md#empty-directory-exists (from venv.rs::empty_dir_exists)
- [x] commands/uv-venv.md#non-empty-directory-exists (from venv.rs::non_empty_dir_exists)
- [x] commands/uv-venv.md#using-allow-existing (from venv.rs::non_empty_dir_exists_allow_existing)
- [x] commands/uv-venv.md#running-allow-existing-after-initial-creation (from
      venv.rs::create_venv_then_allow_existing)
- [x] commands/uv-venv.md#using-no-clear-with-existing-directory (from
      venv.rs::no_clear_with_existing_directory)
- [x] commands/uv-venv.md#using-no-clear-with-non-existent-directory (from
      venv.rs::no_clear_with_non_existent_directory)
- [x] commands/uv-venv.md#using-no-clear-overrides-clear (from venv.rs::no_clear_overrides_clear)
- [x] commands/uv-venv.md#using-no-clear-conflicts-with-allow-existing (from
      venv.rs::no_clear_conflicts_with_allow_existing)

## Symlink Handling

- [x] commands/uv-venv.md#symlink-preservation-with-clear (from
      venv.rs::create_venv_symlink_clear_preservation)
- [x] commands/uv-venv.md#symlink-preservation-on-recreation (from
      venv.rs::create_venv_symlink_recreate_preservation)
- [x] commands/uv-venv.md#nested-symlink-preservation (from
      venv.rs::create_venv_nested_symlink_preservation)

## Configuration (pyvenv.cfg)

- [x] commands/uv-venv.md#verifying-pyvenv-cfg-contents (from venv.rs::verify_pyvenv_cfg)
- [x] commands/uv-venv.md#relocatable-virtual-environment (from
      venv.rs::verify_pyvenv_cfg_relocatable)
- [x] commands/uv-venv.md#nested-virtual-environment-uses-same-home (from
      venv.rs::verify_nested_pyvenv_cfg)

## Environment Variables

- [x] settings/environment-variables.md#invalid-uv-http-timeout (from
      venv.rs::create_venv_with_invalid_http_timeout)
- [x] settings/environment-variables.md#invalid-uv-concurrent-installs (from
      venv.rs::create_venv_with_invalid_concurrent_installs)

## Platform-Specific

- [x] commands/uv-venv.md#windows-shims (from venv.rs::windows_shims)
- [x] commands/uv-venv.md#path-with-trailing-space-error (from
      venv.rs::path_with_trailing_space_gives_proper_error)
- [x] commands/uv-venv.md#shell-activation-with-apostrophe-in-path (from
      venv.rs::create_venv_apostrophe)

## Summary

**Migrated:** 42 tests (100% complete)

All virtual environment tests have been successfully migrated to mdtest format, including:

- Platform-specific tests (Windows shims, Linux shell activation)
- Python version-specific seed packages
- All core venv functionality
