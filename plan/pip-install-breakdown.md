# pip install Test Breakdown

**Progress: 37 of 269 tests (13.8%) migrated | 232 tests remaining**

Breaking down pip_install.rs (269 tests) into logical feature categories.

> **Note:** Some tests are already migrated! Check MIGRATION.md or run
> `grep "pip_install.rs::" plan/MIGRATION.md | grep "\[x\]" | wc -l` to see current status.

## 📦 Core Installation (25 tests)

**File**: `test/uv/pip/install/basic.md`

Basic package installation mechanics that don't fit other categories:

- install_package
- install_requirements_txt
- install_from_stdin
- install_from_dev_stdin
- install_pyproject_toml_poetry
- install_package_basic_auth_from_url (basic case)
- empty_requirements_txt
- install_utf16be_requirements
- install_utf16le_requirements
- install_site_packages_mtime_updated
- install_pinned_polars_invalid_metadata
- install_sdist_resolution_lowest
- missing_requirements_txt
- missing_pyproject_toml
- missing_pip
- no_solution
- missing_top_level
- no_extension
- invalid_extension
- launcher
- launcher_with_symlink
- strip_shebang_arguments
- reserved_script_name
- sklearn
- unmanaged

## 🔐 Authentication (13 tests)

**File**: `test/uv/pip/install/auth.md`

All authentication mechanisms:

- install_package_basic_auth_from_keyring
- install_package_basic_auth_from_keyring_wrong_password
- install_package_basic_auth_from_keyring_wrong_username
- install_package_basic_auth_from_netrc
- install_package_basic_auth_from_netrc_default
- install_package_basic_auth_from_netrc_index_in_requirements
- install_git_private_https_interactive
- install_git_private_https_multiple_pat
- install_git_private_https_pat
- install_git_private_https_pat_and_username
- install_git_private_https_pat_at_ref
- install_git_private_https_pat_mixed_with_public
- install_git_private_https_pat_not_authorized

## 🌳 Git Dependencies (15 tests)

**File**: `test/uv/pip/install/git.md`

Git-based package installation:

- install_git_public_https
- install_implicit_git_public_https
- update_ref_git_public_https
- install_git_public_https_exact_commit
- install_git_public_https_missing_branch_or_tag
- install_git_public_https_missing_commit
- install_git_source_respects_offline_mode
- install_github_artifact_private_https_multiple_pat
- install_github_artifact_private_https_pat_mixed_with_public
- missing_git_prefix
- missing_subdirectory_git
- unsupported_git_scheme
- invalidate_path_on_commit
- direct_url_json_git_default
- direct_url_json_git_tag

## ✏️ Editable Installs (28 tests)

**File**: `test/uv/pip/install/editable.md`

All editable installation scenarios:

- install_editable
- install_editable_and_registry
- install_editable_no_binary
- install_editable_compatible_constraint
- install_editable_incompatible_constraint_version
- install_editable_incompatible_constraint_url
- install_editable_pep_508_requirements_txt
- install_editable_pep_508_cli
- install_editable_bare_cli
- install_editable_bare_requirements_txt
- invalid_editable_no_url
- invalid_editable_unnamed_https_url
- invalid_editable_named_https_url
- editable_dynamic
- editable_url_with_marker
- no_deps_editable
- only_binary_editable
- only_binary_editable_setup_py
- only_binary_dependent_editables
- prefer_editable
- pip_install_no_sources_editable_to_registry_switch
- requires_python_editable
- verify_hashes_editable
- static_metadata_already_installed
- already_installed_dependent_editable
- invalidate_editable_on_change
- stale_egg_info
- require_hashes_editable

## ⭐ Extras & Dependency Groups (25 tests)

**File**: `test/uv/pip/install/extras.md`

Extras, dependency groups, and PEP 751:

- install_extras
- reinstall_extras
- install_constraints_extra
- recursive_extra_transitive_url
- compile_pyproject_toml_extra (if moved from compile)
- dependency_group
- directory_and_group
- invalid_group
- many_pyproject_group
- other_sources_group
- project_and_group
- recursive_dependency_group
- suspicious_group
- virtual_dependency_group
- pep_751_dependency
- pep_751_groups
- pep_751_hash_mismatch
- pep_751_install_directory
- pep_751_install_git
- pep_751_install_path_sdist
- pep_751_install_path_wheel
- pep_751_install_registry_sdist
- pep_751_install_registry_wheel
- pep_751_install_url_sdist
- pep_751_install_url_wheel
- pep_751_mix
- pep_751_multiple_sources
- pep_751_requires_python

## 🔒 Constraints (20 tests)

**File**: `test/uv/pip/install/constraints.md`

Constraint handling:

- install_constraints_txt
- install_constraints_from_pyproject
- install_constraints_inline
- install_constraints_inline_remote
- install_constraints_remote
- install_constraints_txt_from_stdin
- install_constraints_with_markers
- install_constraints_respects_offline_mode
- install_requirements_txt_conflicting_pins
- compatible_build_constraint
- compatible_build_constraint_in_pyproject_toml
- compatible_build_constraint_merged_with_pyproject_toml
- incompatible_build_constraint
- incompatible_build_constraint_from_stdin
- incompatible_build_constraint_in_pyproject_toml
- incompatible_build_constraint_merged_with_pyproject_toml
- require_hashes_constraint
- install_with_overrides_from_stdin
- install_with_excludes_from_stdin
- offline_refresh_conflict

## #️⃣ Hashes & Verification (18 tests)

**File**: `test/uv/pip/install/hashes.md`

Hash verification:

- require_hashes
- require_hashes_build_dependencies
- require_hashes_constraint
- require_hashes_editable
- require_hashes_marker
- require_hashes_mismatch
- require_hashes_missing_dependency
- require_hashes_no_deps
- require_hashes_override
- require_hashes_unnamed
- require_hashes_unnamed_repeated
- verify_hashes
- verify_hashes_editable
- verify_hashes_match
- verify_hashes_mismatch
- verify_hashes_missing_version
- verify_hashes_omit_dependency
- pep_751_hash_mismatch

## 🔧 Binary/Source Control (15 tests)

**File**: `test/uv/pip/install/binary-source.md`

Binary and source distribution preferences:

- install_no_binary_cache
- install_no_binary_comma_separated
- install_no_binary_env
- install_no_binary_overrides_only_binary_all
- install_only_binary_all_and_no_binary_all
- install_only_binary_comma_separated
- install_only_binary_overrides_no_binary_all
- only_binary_editable
- only_binary_editable_setup_py
- only_binary_dependent_editables
- only_binary_requirements_txt
- reinstall_no_binary
- find_links
- find_links_no_binary
- no_prerelease_hint_source_builds

## 🐍 Python Version & Platform (22 tests)

**File**: `test/uv/pip/install/platform.md`

Python version compatibility and platform switching:

- install_incompatible_python_version
- install_incompatible_python_version_interpreter_broken_in_path
- install_missing_python_no_target
- install_missing_python_version_with_target
- install_missing_python_with_target
- install_python_preference
- invalid_python_version
- switch_python_version
- switch_platform
- requires_python_direct_url
- requires_python_editable
- pep_751_requires_python
- install_with_system_interpreter
- abi3_wheel_on_freethreaded_python
- build_backend_wrong_wheel_platform
- build_tag
- accept_existing_prerelease
- build_prerelease_hint

## 📇 Indexes (13 tests)

**File**: `test/uv/pip/install/indexes.md`

Index configuration and handling:

- install_no_index
- install_no_index_version
- install_extra_index_url_has_priority
- local_index_absolute
- local_index_fallback
- local_index_relative
- local_index_requirements_txt_absolute
- local_index_requirements_txt_relative
- install_index_with_relative_links
- install_index_with_relative_links_authenticated
- reinstall_no_index
- missing_find_links
- cache_uv_toml_credentials

## 🔄 Reinstall & Upgrade (12 tests)

**File**: `test/uv/pip/install/reinstall.md`

Reinstallation and upgrade behavior:

- respect_installed_and_reinstall
- reinstall_extras
- reinstall_incomplete
- reinstall_build_system
- reinstall_duplicate
- reinstall_no_binary
- reinstall_no_index
- install_upgrade
- install_no_downgrade
- install_relocatable
- exact_install_removes_extraneous_packages
- allow_incompatibilities

## 🏗️ Build System (18 tests)

**File**: `test/uv/pip/install/build.md`

Build system configuration and isolation:

- install_build_isolation_package
- no_build_isolation
- respect_no_build_isolation_env_var
- config_settings_package
- config_settings_path
- config_settings_registry
- transitive_dependency_config_settings_invalidation
- pip_install_build_dependencies_respect_locked_versions
- reinstall_build_system
- cyclic_build_dependency
- build_backend_wrong_wheel_platform
- build_prerelease_hint
- build_tag
- respect_no_installer_metadata_env_var
- static_metadata_pyproject_toml
- static_metadata_source_tree
- test_dynamic_version_sdist_wrong_version
- resolve_derivation_chain

## 🗂️ Path & Local Dependencies (15 tests)

**File**: `test/uv/pip/install/path.md`

Local path-based installations:

- already_installed_local_path_dependent
- invalidate_path_on_cache_key
- invalidate_path_on_change
- invalidate_path_on_commit
- invalidate_path_on_env_var
- path_changes_with_same_name
- path_name_version_change
- already_installed_url_dependency_no_sources
- missing_subdirectory_url
- direct_url_json_direct_url
- direct_url_zip_file_bunk_permissions
- tool_uv_sources
- tool_uv_sources_is_in_preview
- pip_install_no_sources_package
- no_sources_workspace_discovery

## 🧪 Dry Run & Testing (7 tests)

**File**: `test/uv/pip/install/dry-run.md`

Dry run mode:

- dry_run_install
- dry_run_install_already_installed
- dry_run_install_then_upgrade
- dry_run_install_transitive_dependency_already_installed
- dry_run_install_url_dependency
- dry_run_uninstall_url_dependency
- no_deps

## 💾 Caching (7 tests)

**File**: `test/uv/pip/install/caching.md`

Cache behavior:

- cache_priority
- cache_uv_toml_credentials
- avoid_cached_wheel
- install_no_binary_cache
- already_installed_multiple_versions
- already_installed_remote_dependencies
- already_installed_remote_url

## 📝 File Formats & Encoding (15 tests)

**File**: `test/uv/pip/install/formats.md`

Input file formats and encoding:

- install_utf16be_requirements
- install_utf16le_requirements
- utf8_to_utf16_with_bom_be
- utf8_to_utf16_with_bom_le
- concatenated_quoted_arguments
- double_quoted_arguments
- single_quoted_arguments
- unquoted_arguments
- install_unsupported_environment_yml
- install_unsupported_flag
- invalid_pyproject_toml_syntax
- invalid_pyproject_toml_project_schema
- invalid_pyproject_toml_option_schema
- invalid_pyproject_toml_option_unknown_field
- invalid_toml_filename

## ⚙️ Configuration Files (12 tests)

**File**: `test/uv/pip/install/config.md`

pyproject.toml and uv.toml configuration:

- invalid_uv_toml_option_disallowed_automatic_discovery
- invalid_uv_toml_option_disallowed_command_line
- cache_uv_toml_credentials
- invalid_pyproject_toml_requirement_indirect
- install_with_dependencies_from_script
- tool_uv_sources
- tool_uv_sources_is_in_preview
- install_pyproject_toml_poetry
- static_metadata_pyproject_toml
- compatible_build_constraint_in_pyproject_toml
- incompatible_build_constraint_in_pyproject_toml
- install_constraints_from_pyproject

## 🎯 Executables & Symlinks (10 tests)

**File**: `test/uv/pip/install/executables.md`

Executable installation and linking:

- install_executable
- install_executable_copy
- install_executable_hardlink
- install_symlink
- launcher
- launcher_with_symlink
- change_layout_custom_directory
- change_layout_src
- record_uses_forward_slashes
- install_relocatable

## 🚨 Archive Validation (11 tests)

**File**: `test/uv/pip/install/archive-validation.md`

Archive security and validation:

- reject_invalid_archive_member_names
- reject_invalid_central_directory_offset
- reject_invalid_chained_extra_field
- reject_invalid_crc32_mismatch
- reject_invalid_crc32_non_data_descriptor
- reject_invalid_double_zip
- reject_invalid_duplicate_extra_field
- reject_invalid_short_usize
- reject_invalid_short_usize_zip64
- reject_invalid_streaming_zip
- bad_crc32

## 🔀 Miscellaneous (15 tests)

**File**: `test/uv/pip/install/misc.md`

Tests that don't fit other categories:

- overlapping_packages_warning
- already_installed_local_version_of_remote_package
- deptry_gitignore
- conflicting_flags_clap_bug
- install_with_dependencies_from_script

---

## Summary

**Original**: 1 file with 271 tests **Proposed**: 19 files averaging ~14 tests each

### Benefits

1. **Logical Organization**: Tests grouped by feature, not by command
2. **Easier Navigation**: Find authentication tests in auth.md, not buried in install
3. **Reusability**: Features like "git dependencies" apply to multiple commands
4. **Clearer Coverage**: See what features are well-tested vs. under-tested
5. **Better Maintainability**: Changes to one feature don't affect unrelated tests

### Cross-Command Features

Many of these categories apply to multiple pip commands:

- **Git** → Used in: install, compile, sync, freeze, list
- **Authentication** → Used in: install, compile, sync
- **Constraints** → Used in: install, compile, sync
- **Hashes** → Used in: install, compile
- **Extras** → Used in: install, compile, sync

This organization makes it easier to ensure consistent behavior across commands.
