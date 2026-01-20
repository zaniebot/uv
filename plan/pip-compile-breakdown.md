# pip compile Test Breakdown

**Progress: 163 of 372 tests (43.8%) migrated | 209 tests remaining**

Breaking down pip_compile.rs (372 tests) into logical feature categories.

> **Note:** Many tests are already migrated! Check MIGRATION.md or run
> `grep "pip_compile.rs::" plan/MIGRATION.md | grep "\[x\]" | wc -l` to see current status.

## 📝 Basic Compilation (20 tests)

**File**: `test/uv/resolution/compile/basic.md`

Core compilation mechanics:

- compile_requirements_in
- compile_requirements_in_annotation_line
- compile_requirements_in_stdin
- missing_requirements_in
- missing_venv
- empty_output
- compile_pyproject_toml
- compile_pyproject_toml_dynamic_version
- compile_pyproject_toml_with_line_annotation
- compile_pyproject_toml_eager_validation
- compile_pyproject_toml_setuptools
- compile_setup_cfg
- compile_setup_py
- compile_pyproject_toml_invalid_name
- compile_pyproject_toml_poetry
- compile_pyproject_toml_poetry_empty_dependencies
- compile_pyproject_toml_poetry_invalid_dependencies
- compile_requirements_file_extra
- invalid_extra_name
- omit_non_matching_annotation

## ⭐ Extras (25 tests)

**File**: `test/uv/resolution/extras.md`

Extra dependency handling:

- compile_pyproject_toml_extra
- compile_pyproject_toml_extra_name_normalization
- compile_pyproject_toml_extra_missing
- compile_pyproject_toml_extras_missing
- compile_pyproject_toml_all_extras
- compile_pyproject_toml_all_extras_annotation_line
- compile_does_not_allow_both_extra_and_all_extras
- compile_none_extra
- compile_pyproject_toml_recursive_extra
- compile_pyproject_toml_recursive_extra_marker
- compile_pyproject_toml_mutually_recursive_extra
- compile_pyproject_toml_deeply_recursive_extra
- avoid_irrelevant_extras
- avoid_irrelevant_recursive_extras
- compile_lowest_extra_unpinned_warning
- compile_constraint_extra
- extra_disjoint_extras_from_same_registry_package
- extra_overlapping_extras_from_same_registry_package
- extra_extras_compatibility
- transitive_extra_markers
- transitive_extra_markers_patch
- transitive_extra_markers_enabled
- extra_missing_extra
- extra_nested_extra
- extra_missing_nested_extra

## 🔒 Constraints (25 tests)

**File**: `test/uv/resolution/constraints.md`

Constraint handling across commands:

- compile_constraints_txt
- compile_constraints_inline
- compile_constraints_markers
- compile_constraint_extra
- compile_constraints_omit_impossible_dependencies
- compile_constraints_compatible_url
- compile_constraints_compatible_url_version
- compile_constraints_compatible_version
- compile_constraints_incompatible_url
- compile_constraints_incompatible_version
- compatible_build_constraint
- compatible_build_constraint_in_pyproject_toml
- compatible_build_constraint_merged_with_pyproject_toml
- incompatible_build_constraint
- incompatible_build_constraint_from_stdin
- incompatible_build_constraint_in_pyproject_toml
- incompatible_build_constraint_merged_with_pyproject_toml
- constraint_to_direct_url_dependency
- constraint_url_to_direct_url_dependency
- constraint_compatible_version_disjoint_markers
- constraint_compatible_version_overlapping_markers
- constraint_compatible_version_single_marker
- constraint_version_incompatible_version
- constraint_version_to_registry_dependency
- constraint_wheel_to_registry_dependency

## 🐍 Python Version & Platform (35 tests)

**File**: `test/uv/resolution/python-platform.md`

Python version compatibility and platform markers:

- compile_python_312
- compile_python_312_annotation_line
- compile_python_312_no_deps
- compile_python_37
- compile_python_build_version_different_than_target
- compile_python_conflicts
- compile_fallback_interpreter
- compile_fallback_interpreter_broken_in_path
- compile_python_invalid_version
- compile_python_dev_version
- compile_numpy_py38
- compile_missing_python
- compile_missing_python_version
- compile_missing_python_version_default_fallback
- compile_missing_python_version_patch_fallback
- compile_preserve_requires_python_split
- platform_marker_name_normalization
- platform_marker_simplification_combined
- platform_marker_simplification_combined_disjoint
- platform_marker_simplification_combined_non_overlapping
- platform_marker_simplification_disjoint_python
- platform_marker_simplification_dos
- platform_marker_simplification_empty
- platform_marker_simplification_exclude_newer
- platform_marker_simplification_keep_extra
- platform_marker_simplification_os
- platform_marker_simplification_os_full_version
- platform_marker_simplification_python_version
- platform_marker_simplification_single_line
- platform_marker_simplification_triple_or
- sys_platform_windows
- switch_requires_python
- compile_universal_python_platform_macos
- compile_universal_python_platform_windows
- compile_universal_requires_python

## 🌳 Git Dependencies (20 tests)

**File**: `test/uv/resolution/git.md`

Git dependency resolution (reusable across commands):

- compile_git_https_dependency
- compile_git_branch_https_dependency
- compile_git_tag_https_dependency
- compile_git_date_tag_https_dependency
- compile_git_long_commit_https_dependency
- compile_git_short_commit_https_dependency
- compile_git_refs_https_dependency
- compile_git_subdirectory_dependency
- compile_git_subdirectory_static_metadata
- compile_git_concurrent_access
- compile_git_unnamed_concurrent_access
- compile_git_mismatched_name
- allowed_transitive_git_dependency
- disallowed_transitive_git_dependency
- git_dependency_direct_path
- git_dependency_git_shallow_since
- git_dependency_subdirectory_with_dynamic_version
- git_dependency_subdirectory_with_static_metadata
- git_dependency_transitive_static_metadata
- git_dependency_unnamed

## #️⃣ Hashes (15 tests)

**File**: `test/uv/resolution/hashes.md`

Hash handling (reusable across commands):

- generate_hashes_with_editable
- generate_hashes_with_url_source
- generate_hashes_editable_wheel
- generate_hashes_with_git_source
- generate_hashes_required_hashes
- hash_default
- hash_editable
- hash_git
- hash_missing
- hash_mismatch
- hash_optional
- hash_required
- hash_url
- hash_yanked
- emit_hashes_with_multiple_matching_urls

## 📇 Index Handling (20 tests)

**File**: `test/uv/resolution/indexes.md`

Index configuration (reusable across commands):

- compile_index_url_fallback
- compile_index_url_fallback_prefer_primary
- compile_index_url_first_match_all_versions
- compile_index_url_first_match_base
- compile_index_url_first_match_marker
- compile_index_url_unsafe_highest
- compile_index_url_unsafe_lowest
- default_index_url
- extra_index_url
- extra_index_url_explicit_priority
- extra_index_url_explicit_priority_invalid
- extra_index_url_implicit_priority
- extra_index_url_implicit_priority_invalid
- find_links_url
- index_strategy_unsafe_best_match_version
- index_strategy_unsafe_first_match_direct
- no_build_isolation_transitive
- no_index_html_link_rel_homepage
- no_index_html_link_rel_download
- require_exact_match_no_index

## 🎯 Overrides (18 tests)

**File**: `test/uv/resolution/overrides.md`

Override behavior:

- override_dependency_multiple_urls
- override_dependency_non_existent
- override_dependency_version_specifiers_dont_match
- override_does_not_override_non_dependencies
- override_invalid_extra
- override_non_package_name
- override_sub_dependencies_non_overlapping_markers
- override_sub_dependencies_overlapping_markers
- override_to_git_dependency
- override_transitive_markers
- override_url_source
- override_version_specifiers_match
- override_wheel_source
- allow_recursive_url_local_path_override
- allow_recursive_url_local_path_override_constraint
- allow_url_local_path_override
- allow_url_local_path_override_transitive
- local_path_override_multiple_sources

## 🔄 Resolution Strategy (30 tests)

**File**: `test/uv/resolution/strategy.md`

Resolution modes and strategies:

- resolution_highest
- resolution_lowest
- resolution_lowest_direct
- resolution_no_pins
- resolution_prefer_no_pins_upgrade
- resolution_prefer_pins_upgrade
- resolution_prefer_stable_prerelease
- resolution_prefer_stable_upgrade
- resolution_upgrade_package_version
- resolution_upgrade_package_version_forced
- resolution_prefer_prerelease_to_yanked
- upgrade_not_installed
- upgrade_nothing
- upgrade_package_already_latest
- upgrade_package_disallowed_by_direct_dependency_constraint
- upgrade_package_excluded
- upgrade_package_no_change
- upgrade_package_only
- upgrade_package_version
- upgrade_package_version_both_direct_and_transitive
- upgrade_package_version_dependency_non_contiguous
- upgrade_package_version_excluded
- upgrade_package_version_forced
- upgrade_package_version_incompatible_direct_dependency
- upgrade_package_version_incompatible_python_version
- upgrade_transitive_package_version
- upgrade_with_lower_python_bound
- fork_conflict_split_both_dependencies_unsatisfiable
- fork_resolution_strategy_prefer_no_split
- fork_resolution_strategy_prefer_split

## 🏷️ Markers (25 tests)

**File**: `test/uv/resolution/markers.md`

Marker evaluation and simplification (reusable across commands):

- marker_accidental_ignore
- marker_direct_disjoint
- marker_direct_overlap
- marker_normalize
- marker_normalize_extra
- marker_normalize_python_full_version
- marker_normalize_python_version
- marker_propagation_identical_nested_dependencies
- marker_propagation_nesting_disjoint
- marker_propagation_nesting_overlap
- marker_propagation_nesting_transitive
- marker_simplification
- marker_simplification_disjoint_requirements
- marker_track_multiple_forks
- marker_transitive_disjoint
- marker_transitive_overlap
- marker_transitive_via_extra
- platform_marker_name_normalization
- platform*marker_simplification*\* (multiple)
- transitive_extra_markers
- transitive_extra_markers_patch
- transitive_extra_markers_enabled

## 📦 Output Formats (15 tests)

**File**: `test/uv/resolution/output.md`

Output formatting specific to pip compile:

- compile_annotation_style_line
- compile_annotation_style_split
- compile_custom_compile_header
- compile_disable_annotate
- compile_empty_annotation_style
- compile_emit_index_annotation
- compile_emit_index_url_annotation
- compile_emit_marker_expression
- compile_header_comment
- compile_no_emit_package
- compile_output_file
- compile_output_file_hashes
- compile_stdin_to_output_file
- emit_build_options_registry
- emit_index_annotation_unnamed

## 🔀 Forking & Conflict Resolution (45 tests)

**File**: `test/uv/resolution/forking.md`

Universal dependency resolution forking:

- fork_allows_non_conflicting_non_local_urls_disjoint
- fork_allows_non_conflicting_non_local_urls_overlapping
- fork_allows_non_conflicting_non_local_urls_requires_python
- fork_basic
- fork_basic_requires_python
- fork_conflict_in_fork
- fork_conflict_split_both_dependencies_unsatisfiable
- fork_incomplete_markers
- fork_markers_limit_propagation
- fork_markers_partial_fork
- fork_multiple_identical_nested_dependencies
- fork_multiple_identical_nested_dependencies_extra
- fork_non_conflicting_register_url
- fork_non_forking_extra
- fork_non_local_fork_marker_accidental_transitive
- fork_non_local_fork_marker_limited_inherit
- fork_non_local_fork_marker_selection
- fork_overlapping_markers
- fork_overlapping_markers_requires_python
- fork_python_greater_than_star_or
- fork_python_patch_version
- fork_relative_python_markers
- fork_resolution_strategy_prefer_no_split
- fork_resolution_strategy_prefer_split
- fork_sorted_python_markers
- fork_split_deeper_independent_dependency
- fork_split_independent_dependency
- fork_split_marker_conflict
- fork_split_transitive_non_local_fork
- fork_triggered_via_marker_disjoint
- fork_triggered_via_marker_full_overlap
- fork_triggered_via_marker_partial_overlap
- fork_urls_disjoint_markers
- fork_urls_disjoint_python_requirement
- fork_urls_full_overlap
- fork_urls_overlapping_markers
- fork_urls_partial_overlap
- fork_urls_python_marker_matching
- fork_urls_python_version
- fork_urls_requires_python
- fork_version_to_url
- non_local_fork_marker_transitive
- requires_python_full_union_allows_non_fork
- requires_python_fork_allows_non_local
- requires_python_fork_disallows_non_local

## 🎭 Yanked Packages (20 tests)

**File**: `test/uv/resolution/yanked.md`

Yanked package handling:

- transitive_yanked_and_unyanked_dependency
- yanked_preference
- yanked_version_build_tag
- yanked_version_has_yank
- yanked_version_local_version_identifier
- yanked_version_out_of_range
- yanked_version_prefer_no_sdist
- yanked_version_prefer_sdist
- yanked_version_prefer_wheel
- yanked_version_python_version
- yanked_version_tag_excluded
- yanked_version_tag_included
- yanked_version_tag_not_included_by_default
- yanked_prefer_unyanked_in_range_lower_bound
- yanked_prefer_unyanked_in_range_upper_bound
- yanked_prefers_non_yanked_with_same_timestamp
- prefer_non_yanked_among_compatible_versions
- prefer_non_yanked_higher_version
- prefer_unyank_over_exclude_newer
- prefer_exclude_newer_over_yanked

## 📋 Annotations & Headers (15 tests)

**File**: `test/uv/resolution/annotations.md`

Output annotation styles:

- compile_annotation_style_line
- compile_annotation_style_split
- compile_custom_compile_header
- compile_disable_annotate
- compile_empty_annotation_style
- compile_emit_index_annotation
- compile_emit_index_url_annotation
- compile_emit_marker_expression
- compile_header_comment
- compile_no_emit_package
- emit_build_options_registry
- emit_index_annotation_unnamed
- emit_index_url
- no_emit_index_url
- no_strip_extras

## 🔄 Upgrade & Update (35 tests)

**File**: `test/uv/resolution/upgrade.md`

Upgrade strategies and behavior:

- resolution_prefer_no_pins_upgrade
- resolution_prefer_pins_upgrade
- resolution_prefer_stable_upgrade
- resolution_upgrade_package_version
- resolution_upgrade_package_version_forced
- upgrade_not_installed
- upgrade_nothing
- upgrade_package_already_latest
- upgrade_package_disallowed_by_direct_dependency_constraint
- upgrade_package_excluded
- upgrade_package_no_change
- upgrade_package_only
- upgrade_package_version
- upgrade_package_version_both_direct_and_transitive
- upgrade_package_version_dependency_non_contiguous
- upgrade_package_version_excluded
- upgrade_package_version_forced
- upgrade_package_version_incompatible_direct_dependency
- upgrade_package_version_incompatible_python_version
- upgrade_transitive_package_version
- upgrade_with_lower_python_bound
- update_file_exists
- update_marker_environment
- update_marker_extra
- update_marker_os_name
- update_marker_platform_machine
- update_marker_python_version
- update_marker_python_version_patch
- update_marker_sys_platform
- update_package_not_installed
- update_no_change
- update_default
- update_unrelated_package_changed
- update_requires_python
- update_with_local_version_identifier

## 🌐 URLs & Direct Dependencies (18 tests)

**File**: `test/uv/resolution/urls.md`

URL-based dependencies:

- compile_wheel_url_dependency
- compile_sdist_url_dependency
- direct_url_from_url
- direct_url_source_urls_and_wheels
- direct_url_with_marker
- direct_url_with_multiple_markers
- direct_url_with_mutually_exclusive_markers
- direct_url_zip
- transitive_urls
- url_to_path_dependency
- url_to_registry_dependency
- url_to_url_dependency
- constraint_to_direct_url_dependency
- constraint_url_to_direct_url_dependency
- optional_dependency_url_source_extra_not_requested
- optional_dependency_url_source_extra_requested
- requires_url_editable_to_git_transitively
- source_tree_url

## 📦 Wheel Selection (25 tests)

**File**: `test/uv/resolution/wheels.md`

Wheel compatibility and selection:

- no_binary_disables_wheel_cache
- no_binary_does_not_override_build_wheels
- no_binary_wheel_to_sdist
- no_build_isolation_transitive
- only_binary_build_isolation_package
- prefer_binary
- prefer_binary_prefer_oldest
- prefer_binary_using_find_links
- prefer_binary_with_markers
- prefer_newest_transitive
- prefer_source_dist
- prefer_source_dist_with_hash
- prerelease_in_find_links
- prerelease_version_matching
- prerelease_via_explicit
- release_only_binary
- release_only_dev
- source_distribution_cached_builds
- source_distribution_conditional_extra
- source_distribution_extra
- source_distribution_markers_disjoint
- source_distribution_markers_overlap
- source_distribution_no_pep517
- wheel_metadata_1_2
- wheel_only

## 🔐 Overrides (continues from above - 30+ tests)

**File**: `test/uv/resolution/overrides.md`

More override scenarios beyond basic ones listed above.

## 📊 Dependency Tracking (20 tests)

**File**: `test/uv/resolution/tracking.md`

Dependency graph and tracking:

- transitive_build_dependency
- transitive_dependencies_metadata
- transitive_package_extra
- transitive_url_extra
- transitive_dependency_circular_build_dependency
- transitive_dependency_extra_with_marker
- transitive_dev_dependency
- recursive_editable_conditional_url
- recursive_editable_conditional_registry
- recursive_package_extra
- recursive_url_editable
- recursive_url_registry
- only_explicit_names

## 🚫 Exclusions & Filtering (12 tests)

**File**: `test/uv/resolution/exclusions.md`

Package exclusion and filtering:

- no_deps
- no_annotate_markers
- no_strip_extras
- omit_transitive_extra
- no_emit_index_url
- compile_no_emit_package
- exclude_newer_cache
- exclude_newer_does_not_apply_to_installed_packages
- exclude_newer_does_not_apply_to_local_builds
- exclude_newer_git_uses_last_modified
- exclude_newer_higher_version_available
- exclude_newer_no_patches_available

## 🎪 Edge Cases & Special Scenarios (25+ tests)

**File**: `test/uv/resolution/edge-cases.md`

Uncommon scenarios and edge cases:

- compile_universal_python_platform_macos
- compile_universal_python_platform_windows
- compile_universal_requires_python
- deterministic_url
- empty_requirements
- local_wheel_to_remote_wheel
- name_normalization_dependency
- name_normalization_extra
- name_normalization_installed
- path_dependency_name_match
- propagate_marker_to_transitive_dependency_missing_extra
- relative_find_links
- relative_requirements_txt
- sdist_add_and_remove
- transitive_build_dependency_exclude_newer
- transitive_extra_missing_extra
- virtual_workspace_transitive_path_dependency_self
- wheel_no_requires_dist
- wildcard_version_includes_postrelease
- wildcard_version_includes_prerelease

---

## Summary

**Original**: 1 file with 372 tests **Proposed**: ~15-20 files averaging 15-25 tests each

### Key Insight

Many of these "pip compile" tests are actually testing **universal dependency resolution features**:

- Extras
- Constraints
- Markers
- Python version compatibility
- Git dependencies
- Hashes
- Indexes
- Overrides

These features should be organized by FEATURE rather than by COMMAND, since they apply across:

- `pip compile`
- `pip install`
- `pip sync`
- `lock`
- `sync`

### Proposed Directory Structure

```
test/uv/
├── resolution/          # Universal dependency resolution features
│   ├── extras.md
│   ├── constraints.md
│   ├── markers.md
│   ├── python-platform.md
│   ├── git.md
│   ├── hashes.md
│   ├── indexes.md
│   ├── overrides.md
│   ├── strategy.md
│   ├── yanked.md
│   └── wheels.md
├── pip/
│   ├── compile/
│   │   ├── basic.md       # pip compile specific
│   │   ├── output.md      # annotations, headers
│   │   └── update.md      # -U flag behavior
│   ├── install/
│   │   ├── basic.md       # pip install specific
│   │   ├── editable.md
│   │   ├── auth.md
│   │   └── ...
│   └── sync/
│       └── basic.md       # pip sync specific
```

This organization makes it clear which tests are command-specific vs. which test universal resolver
behavior.
