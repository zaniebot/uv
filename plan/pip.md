# pip Commands

Package management commands compatible with pip.

## pip check

- [x] pip/check.md#compatible-packages (from pip_check.rs::check_compatible_packages)
- [x] pip/check.md#incompatible-packages (from pip_check.rs::check_incompatible_packages)
- [x] pip/check.md#multiple-incompatible-packages (from
      pip_check.rs::check_multiple_incompatible_packages)
- [x] pip/check.md#python-version (from pip_check.rs::check_python_version)

**Status**: 4/4 tests (100% complete)

## pip freeze

- [x] pip/freeze.md#many (from pip_freeze.rs::freeze_many)
- [x] pip/freeze.md#duplicate (from pip_freeze.rs::freeze_duplicate)
- [x] pip/freeze.md#url (from pip_freeze.rs::freeze_url)
- [x] pip/freeze.md#with-editable (from pip_freeze.rs::freeze_with_editable)
- [x] pip/freeze.md#with-egg-info (from pip_freeze.rs::freeze_with_egg_info)
- [x] pip/freeze.md#with-egg-info-no-py (from pip_freeze.rs::freeze_with_egg_info_no_py)
- [x] pip/freeze.md#with-egg-info-file (from pip_freeze.rs::freeze_with_egg_info_file)
- [x] pip/freeze.md#with-legacy-editable (from pip_freeze.rs::freeze_with_legacy_editable)
- [x] pip/freeze.md#path (from pip_freeze.rs::freeze_path)
- [x] pip/freeze.md#multiple-paths (from pip_freeze.rs::freeze_multiple_paths)
- [x] pip/freeze.md#nonexistent-path (from pip_freeze.rs::freeze_nonexistent_path)
- [x] pip/freeze.md#with-quiet-flag (from pip_freeze.rs::freeze_with_quiet_flag)
- [x] pip/freeze.md#target (from pip_freeze.rs::freeze_target)
- [x] pip/freeze.md#prefix (from pip_freeze.rs::freeze_prefix)
- [x] pip/freeze.md#exclude (from pip_freeze.rs::freeze_exclude)

**Status**: 15/15 tests (100% complete)

## pip list

- [x] pip/list.md#empty-columns (from pip_list.rs::list_empty_columns)
- [x] pip/list.md#empty-freeze (from pip_list.rs::list_empty_freeze)
- [x] pip/list.md#empty-json (from pip_list.rs::list_empty_json)
- [x] pip/list.md#single-no-editable (from pip_list.rs::list_single_no_editable)
- [x] pip/list.md#outdated-columns (from pip_list.rs::list_outdated_columns)
- [x] pip/list.md#outdated-json (from pip_list.rs::list_outdated_json)
- [x] pip/list.md#outdated-freeze (from pip_list.rs::list_outdated_freeze)
- [x] pip/list.md#outdated-git (from pip_list.rs::list_outdated_git)
- [x] pip/list.md#outdated-index (from pip_list.rs::list_outdated_index)
- [x] pip/list.md#editable (from pip_list.rs::list_editable)
- [x] pip/list.md#editable-only (from pip_list.rs::list_editable_only)
- [x] pip/list.md#exclude (from pip_list.rs::list_exclude)
- [x] pip/list.md#format-json (from pip_list.rs::list_format_json)
- [x] pip/list.md#format-freeze (from pip_list.rs::list_format_freeze)
- [x] pip/list.md#legacy-editable (from pip_list.rs::list_legacy_editable)
- [x] pip/list.md#legacy-editable-invalid-version (from
      pip_list.rs::list_legacy_editable_invalid_version)
- [x] pip/list.md#ignores-quiet-flag-format-freeze (from
      pip_list.rs::list_ignores_quiet_flag_format_freeze)
- [x] pip/list.md#target (from pip_list.rs::list_target)
- [x] pip/list.md#prefix (from pip_list.rs::list_prefix)

**Status**: 19/19 tests (100% complete)

## pip show

- [x] pip/show.md#empty (from pip_show.rs::show_empty)
- [x] pip/show.md#requires-multiple (from pip_show.rs::show_requires_multiple)
- [x] pip/show.md#python-version-marker (from pip_show.rs::show_python_version_marker)
- [x] pip/show.md#found-single-package (from pip_show.rs::show_found_single_package)
- [x] pip/show.md#found-multiple-packages (from pip_show.rs::show_found_multiple_packages)
- [x] pip/show.md#found-one-out-of-three (from pip_show.rs::show_found_one_out_of_three)
- [x] pip/show.md#found-one-out-of-two-quiet (from pip_show.rs::show_found_one_out_of_two_quiet)
- [x] pip/show.md#empty-quiet (from pip_show.rs::show_empty_quiet)
- [x] pip/show.md#editable (from pip_show.rs::show_editable)
- [x] pip/show.md#required-by-multiple (from pip_show.rs::show_required_by_multiple)
- [x] pip/show.md#files (from pip_show.rs::show_files)
- [x] pip/show.md#target (from pip_show.rs::show_target)
- [x] pip/show.md#prefix (from pip_show.rs::show_prefix)

**Status**: 13/13 tests (100% complete)

## pip uninstall

- [x] pip/uninstall.md#no-arguments (from pip_uninstall.rs::no_arguments)
- [x] pip/uninstall.md#invalid-requirement (from pip_uninstall.rs::invalid_requirement)
- [x] pip/uninstall.md#missing-requirements-txt (from pip_uninstall.rs::missing_requirements_txt)
- [x] pip/uninstall.md#invalid-requirements-txt-requirement (from
      pip_uninstall.rs::invalid_requirements_txt_requirement)
- [x] pip/uninstall.md#uninstall (from pip_uninstall.rs::uninstall)
- [x] pip/uninstall.md#missing-record (from pip_uninstall.rs::missing_record)
- [x] pip/uninstall.md#uninstall-editable-by-name (from
      pip_uninstall.rs::uninstall_editable_by_name)
- [x] pip/uninstall.md#uninstall-by-path (from pip_uninstall.rs::uninstall_by_path)
- [x] pip/uninstall.md#uninstall-duplicate-by-path (from
      pip_uninstall.rs::uninstall_duplicate_by_path)
- [x] pip/uninstall.md#uninstall-duplicate (from pip_uninstall.rs::uninstall_duplicate)
- [x] pip/uninstall.md#uninstall-egg-info (from pip_uninstall.rs::uninstall_egg_info)
- [x] pip/uninstall.md#uninstall-legacy-editable (from pip_uninstall.rs::uninstall_legacy_editable)
- [x] pip/uninstall.md#dry-run-uninstall-egg-info (from
      pip_uninstall.rs::dry_run_uninstall_egg_info)

**Status**: 13/13 tests (100% complete)

## pip compile

- [ ] pip_compile.rs - 372 tests (NOT STARTED)

## pip install

- [ ] pip_install.rs - 271 tests (NOT STARTED)

## pip sync

- [ ] pip_sync.rs - ~160 tests (NOT STARTED)

## pip tree

- [ ] pip_tree.rs - 22 tests (NOT STARTED)

## pip debug

- [ ] pip_debug.rs - 1 test (NOT STARTED)

## Summary

**Completed**: 64/64 pip commands migrated (check, freeze, list, show, uninstall) **Remaining**:
Large categories (compile, install, sync) + tree + debug

The completed pip commands cover all the simpler, self-contained functionality. The remaining
categories are the large, complex ones that interact heavily with dependency resolution and
installation.
