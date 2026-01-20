# Tools

Tool installation (uvx), running tools, upgrading, listing installed tools.

## Summary

| Category  | Migrated | Total   | Notes    |
| --------- | -------- | ------- | -------- |
| Dir       | 2        | 2       | Complete |
| Install   | 53       | 53      | Complete |
| List      | 12       | 12      | Complete |
| Run       | 51       | 51      | Complete |
| Uninstall | 5        | 5       | Complete |
| Upgrade   | 15       | 15      | Complete |
| **Total** | **138**  | **138** | 100%     |

### Blockers - Missing mdtest features needed

- ~~**File deletion**~~: RESOLVED - `$ rm` command now supported in mdtest
  - [x] `tool_uninstall_missing_receipt`, `tool_uninstall_all_missing_receipt` - migrated
  - [x] `tool_list_missing_receipt`, `tool_list_bad_environment` - migrated
  - `tool_upgrade_not_stop_if_upgrade_fails` - actually uses Write (not rm) for corruption

- ~~**Empty file creation (touch)**~~: RESOLVED - Use `tree create=true` with `UV_TOOL_BIN_DIR`
  - [x] `tool_install_force` - migrated using `tree create=true`

- ~~**Git operations**~~: RESOLVED - Tests that clone from Git repositories require `git` feature
  - Use `required-features = "git"` and run with `UV_MDTEST_FEATURES=git`
  - [x] `tool_run_git` - migrated
  - [x] `tool_install_git` - migrated
  - [x] `tool_run_git_lfs`, `tool_install_git_lfs` - migrated

- ~~**Multiple Python versions**~~: RESOLVED - Use `python-versions = ["3.12", "3.11"]`
  - [x] `tool_upgrade_python`, `tool_upgrade_python_with_all`,
        `test_tool_upgrade_additional_entrypoints`
  - [x] `tool_run_python`, `tool_run_python_at_version`, `tool_run_python_from`

- ~~**Editable installs**~~: RESOLVED - Use `${WORKSPACE}` to reference test fixtures
  - [x] `tool_install_editable`, `tool_run_with_editable`

## Dir

- [x] tools/dir.md#tool-directory-displaying-the-tool-directory (from tool_dir.rs::tool_dir)
- [x] tools/dir.md#tool-directory-displaying-the-bin-directory (from tool_dir.rs::tool_dir_bin)

## Install - Basic

- [x] tools/install.md#tool-install-installing-a-tool (from tool_install.rs::tool_install)
- [x] tools/install.md#tool-install-already-installed-tool (from
      tool_install.rs::tool_install_already_installed)
- [x] tools/install.md#installing-with-existing-executable-force-required (from
      tool_install.rs::tool_install_force)
- [x] tools/install.md#upgrading-a-tool (from tool_install.rs::tool_install_upgrade)
- [x] tools/install.md#tool-install-installing-with-resolution-settings (from
      tool_install.rs::tool_install_settings)

## Install - Version Specifiers

- [x] tools/install.md#tool-install-installing-with-version-specifier (from
      tool_install.rs::tool_install_version)
- [x] tools/install.md#tool-install-installing-with-version-syntax (from
      tool_install.rs::tool_install_at_version)
- [x] tools/install.md#tool-install-installing-with-latest-syntax (from
      tool_install.rs::tool_install_at_latest)
- [x] tools/install.md#upgrading-with-latest-syntax (from
      tool_install.rs::tool_install_at_latest_upgrade)

## Install - From Package

- [x] tools/install.md#tool-install-installing-with-from-errors (from
      tool_install.rs::tool_install_from) - error cases only
- [x] tools/install.md#installing-with-from-and-latest (from
      tool_install.rs::tool_install_from_at_latest)
- [x] tools/install.md#installing-with-from-and-version (from
      tool_install.rs::tool_install_from_at_version)
- [x] tools/install.md#installing-with-unnamed-url-using-from (from
      tool_install.rs::tool_install_unnamed_from)
- [x] tools/install.md#installing-with-unnamed-url-using-with (from
      tool_install.rs::tool_install_unnamed_with)
- [x] tools/install.md#installing-with-from-name-conflict (from
      tool_install.rs::tool_install_unnamed_conflict)
- [x] tools/install.md#installing-bare-url-package (from
      tool_install.rs::tool_install_unnamed_package)
- [x] tools/install.md#tool-install-installing-with-mismatched-package-name-from-url (from
      tool_install.rs::tool_install_mismatched_name)

## Install - Editable

- [x] tools/install.md#tool-install-installing-an-editable-package (from
      tool_install.rs::tool_install_editable)
- [x] tools/install.md#installing-with-with-editable (from
      tool_install.rs::tool_install_with_editable)
- [x] tools/install.md#installing-editable-with-from (from
      tool_install.rs::tool_install_editable_from)

## Install - Git Dependencies

- [x] tools/install.md#tool-install-installing-from-git-repository (from
      tool_install.rs::tool_install_git)
- [x] tools/install.md#installing-from-git-repository-with-lfs (from
      tool_install.rs::tool_install_git_lfs)

## Install - Requirements Files

- [x] tools/install.md#installing-with-dependencies-from-script (from
      tool_install.rs::tool_install_with_dependencies_from_script)
- [x] tools/install.md#installing-with-requirements-file (from
      tool_install.rs::tool_install_requirements_txt)
- [x] tools/install.md#installing-with-requirements-file-arguments (from
      tool_install.rs::tool_install_requirements_txt_arguments)

## Install - Constraints and Overrides

- [x] tools/install.md#tool-install-installing-with-constraints-file (from
      tool_install.rs::tool_install_constraints)
- [x] tools/install.md#installing-with-overrides (from tool_install.rs::tool_install_overrides)
- [x] tools/install.md#installing-with-compatible-build-constraints (from
      tool_install.rs::tool_install_with_compatible_build_constraints)
- [x] tools/install.md#installing-with-incompatible-build-constraints (from
      tool_install.rs::tool_install_with_incompatible_build_constraints)

## Install - Python Selection

- [x] tools/install.md#installing-with-global-python-version (from
      tool_install.rs::tool_install_with_global_python)
- [x] tools/install.md#installing-python-itself-is-not-allowed (from
      tool_install.rs::tool_install_python)
- [x] tools/install.md#installing-with-python-version-flag (from
      tool_install.rs::tool_install_python_requests)
- [x] tools/install.md#installing-with-python-preference (from
      tool_install.rs::tool_install_python_preference)
- [x] tools/install.md#installing-with-python-platform (from
      tool_install.rs::tool_install_python_platform)

## Install - Environment and Paths

- [x] tools/install.md#installing-with-home-directory (from tool_install.rs::tool_install_home)
- [x] tools/install.md#installing-with-xdg-data-home (from
      tool_install.rs::tool_install_xdg_data_home)
- [x] tools/install.md#installing-with-xdg-bin-home (from
      tool_install.rs::tool_install_xdg_bin_home)
- [x] tools/install.md#installing-with-uv-tool-bin-dir (from
      tool_install.rs::tool_install_tool_bin_dir)
- [x] tools/install.md#installing-with-preserved-environment (from
      tool_install.rs::tool_install_preserve_environment)
- [x] tools/install.md#installing-with-path-warning (from tool_install.rs::tool_install_warn_path)

## Install - Entrypoints and Executables

- [x] tools/install.md#tool-install-installing-a-package-without-executables (from
      tool_install.rs::tool_install_no_entrypoints)
- [x] tools/install.md#installing-with-executables-from-multiple-packages (from
      tool_install.rs::tool_install_with_executables_from)
- [x] tools/install.md#installing-with-executables-from-package-without-entrypoints (from
      tool_install.rs::tool_install_with_executables_from_no_entrypoints)
- [x] tools/install.md#suggesting-packages-with-desired-executable (from
      tool_install.rs::tool_install_suggest_other_packages_with_executable)

## Install - Credentials

- [x] tools/install.md#installing-from-authenticated-index (from
      tool_install.rs::tool_install_credentials)
- [x] tools/install.md#installing-from-authenticated-default-index (from
      tool_install.rs::tool_install_default_credentials)

## Install - Package Sources

- [x] tools/install.md#installing-with-find-links (from tool_install.rs::tool_install_find_links)

## Install - Error Handling

- [x] tools/install.md#installing-uninstallable-package (from
      tool_install.rs::tool_install_uninstallable)
- [x] tools/install.md#removing-tool-on-installation-failure (from
      tool_install.rs::tool_install_remove_on_empty)
- [x] tools/install.md#reinstalling-with-invalid-receipt (from
      tool_install.rs::tool_install_bad_receipt)
- [x] tools/install.md#installing-package-with-malformed-dist-info (from
      tool_install.rs::tool_install_malformed_dist_info)

## List - Basic

- [x] tools/list.md#tool-list-listing-installed-tools (from tool_list.rs::tool_list)
- [x] tools/list.md#tool-list-empty-tool-list (from tool_list.rs::tool_list_empty)

## List - Paths

- [x] tools/list.md#tool-list-listing-with-paths (from tool_list.rs::tool_list_paths)
- [x] tools/list.md#listing-with-paths-on-windows (from tool_list.rs::tool_list_paths_windows)

## List - Display Options

- [x] tools/list.md#tool-list-listing-with-version-specifiers (from
      tool_list.rs::tool_list_show_version_specifiers)
- [x] tools/list.md#tool-list-listing-with-additional-dependencies (from
      tool_list.rs::tool_list_show_with)
- [x] tools/list.md#tool-list-listing-with-extras (from tool_list.rs::tool_list_show_extras)
- [x] tools/list.md#tool-list-listing-with-python-version (from tool_list.rs::tool_list_show_python)
- [x] tools/list.md#tool-list-listing-with-all-flags (from tool_list.rs::tool_list_show_all)
- [x] tools/list.md#listing-with-deprecated-receipt-format (from tool_list.rs::tool_list_deprecated)

## List - Error Handling

- [x] tools/list.md#tool-list-listing-with-missing-receipt (from
      tool_list.rs::tool_list_missing_receipt)
- [x] tools/list.md#tool-list-listing-with-bad-environment (from
      tool_list.rs::tool_list_bad_environment) - Unix-only currently

## Run - Basic

- [x] tools/run.md#tool-run-running-a-tool (from tool_run.rs::tool_run_args)
- [x] tools/run.md#running-without-output (from tool_run.rs::tool_run_without_output)
- [x] tools/run.md#running-with-verbose-hint (from tool_run.rs::tool_run_verbose_hint)
- [x] tools/run.md#tool-run-running-without-command-shows-installed-tools (from
      tool_run.rs::tool_run_list_installed)

## Run - Version Specifiers

- [x] tools/run.md#tool-run-running-with-invalid-version-syntax (from
      tool_run.rs::tool_run_at_version) - error case only
- [x] tools/run.md#tool-run-running-with-version-syntax (from tool_run.rs::tool_run_at_version)
- [x] tools/run.md#tool-run-running-with-from-and-version (from tool_run.rs::tool_run_from_version)
- [x] tools/run.md#tool-run-running-with-from-and-version-syntax (from
      tool_run.rs::tool_run_from_at)
- [x] tools/run.md#tool-run-running-with-latest-syntax (from tool_run.rs::tool_run_latest)
- [x] tools/run.md#tool-run-running-with-version-specifier (from tool_run.rs::tool_run_specifier)

## Run - Extras and Dependencies

- [x] tools/run.md#tool-run-running-with-extras (from tool_run.rs::tool_run_latest_extra)
- [x] tools/run.md#tool-run-running-with-extras (from tool_run.rs::tool_run_extra)
- [x] tools/run.md#running-with-comma-separated-dependencies-shorthand (from
      tool_run.rs::tool_run_csv_with_shorthand)
- [x] tools/run.md#running-with-comma-separated-dependencies (from tool_run.rs::tool_run_csv_with)
- [x] tools/run.md#running-with-repeated-with-flags (from tool_run.rs::tool_run_repeated_with)
- [x] tools/run.md#tool-run-running-with-editable-dependency (from
      tool_run.rs::tool_run_with_editable)

## Run - From Package

- [x] tools/run.md#using-installed-tool-version (from tool_run.rs::tool_run_from_install)
- [x] tools/run.md#using-installed-tool-with-constraints (from
      tool_run.rs::tool_run_from_install_constraints)
- [x] tools/run.md#tool-run-verbatim-package-name-handling (from
      tool_run.rs::tool_run_verbatim_name)
- [x] tools/run.md#warning-about-executable-not-in-from-package (from
      tool_run.rs::tool_run_warn_executable_not_in_from)
- [x] tools/run.md#tool-run-suggesting-valid-commands (from
      tool_run.rs::tool_run_suggest_valid_commands)

## Run - Remote Sources

- [x] tools/run.md#running-from-url (from tool_run.rs::tool_run_url)
- [x] tools/run.md#tool-run-running-from-git-repository (from tool_run.rs::tool_run_git)
- [x] tools/run.md#running-from-git-repository-with-lfs (from tool_run.rs::tool_run_git_lfs)

## Run - Requirements Files

- [x] tools/run.md#tool-run-running-with-requirements-file (from
      tool_run.rs::tool_run_requirements_txt)
- [x] tools/run.md#running-with-requirements-file-arguments (from
      tool_run.rs::tool_run_requirements_txt_arguments)

## Run - Constraints and Overrides

- [x] tools/run.md#tool-run-running-with-constraints-file (from tool_run.rs::tool_run_constraints)
- [x] tools/run.md#tool-run-running-with-overrides-file (from tool_run.rs::tool_run_overrides)
- [x] tools/run.md#running-with-compatible-build-constraints (from
      tool_run.rs::tool_run_with_compatible_build_constraints)
- [x] tools/run.md#running-with-incompatible-build-constraints (from
      tool_run.rs::tool_run_with_incompatible_build_constraints)

## Run - Python Selection

- [x] tools/run.md#tool-run-running-python-directly (from tool_run.rs::tool_run_python)
- [x] tools/run.md#tool-run-running-python-with-version-specifier (from
      tool_run.rs::tool_run_python_at_version)
- [x] tools/run.md#tool-run-running-python-with-from (from tool_run.rs::tool_run_python_from)
- [x] tools/run.md#hinting-when-python-version-not-available (from
      tool_run.rs::tool_run_hint_version_not_available)
- [x] tools/run.md#re-resolving-for-compatible-python-version (from
      tool_run.rs::tool_run_reresolve_python)

## Run - Scripts

- [x] tools/run.md#error-on-script-in-from-argument (from tool_run.rs::tool_run_with_from_script)
- [x] tools/run.md#error-on-script-and-from-script (from
      tool_run.rs::tool_run_with_script_and_from_script)
- [x] tools/run.md#running-with-dependencies-from-script (from
      tool_run.rs::tool_run_with_dependencies_from_script)
- [x] tools/run.md#error-running-existing-py-script (from
      tool_run.rs::tool_run_with_existing_py_script)
- [x] tools/run.md#error-running-existing-pyw-script (from
      tool_run.rs::tool_run_with_existing_pyw_script)
- [x] tools/run.md#error-with-nonexistent-py-script (from
      tool_run.rs::tool_run_with_nonexistent_py_script)
- [x] tools/run.md#error-with-nonexistent-pyw-script (from
      tool_run.rs::tool_run_with_nonexistent_pyw_script)

## Run - Environment

- [x] tools/run.md#running-with-environment-file (from tool_run.rs::run_with_env_file)
- [x] tools/run.md#caching-tool-environments (from tool_run.rs::tool_run_cache)

## Run - Windows

- [x] tools/run.md#windows-runnable-types (from tool_run.rs::tool_run_windows_runnable_types)
- [x] tools/run.md#windows-dotted-package-name (from
      tool_run.rs::tool_run_windows_dotted_package_name)

## Run - Authentication

- [x] tools/run.md#keyring-authentication-with-@latest (from
      tool_run.rs::tool_run_latest_keyring_auth)

## Run - Error Handling

- [x] tools/run.md#error-when-package-has-no-executables (from
      tool_run.rs::warn_no_executables_found)
- [x] tools/run.md#tool-run-upgrade-warning (from tool_run.rs::tool_run_upgrade_warn)
- [x] tools/run.md#tool-run-resolution-error (from tool_run.rs::tool_run_resolution_error)

## Uninstall

- [x] tools/uninstall.md#tool-uninstall-uninstalling-a-tool (from tool_uninstall.rs::tool_uninstall)
- [x] tools/uninstall.md#tool-uninstall-uninstalling-multiple-tools (from
      tool_uninstall.rs::tool_uninstall_multiple_names)
- [x] tools/uninstall.md#tool-uninstall-uninstalling-a-tool-that-is-not-installed (from
      tool_uninstall.rs::tool_uninstall_not_installed)
- [x] tools/uninstall.md#tool-uninstall-uninstalling-tool-with-missing-receipt (from
      tool_uninstall.rs::tool_uninstall_missing_receipt)
- [x] tools/uninstall.md#tool-uninstall-uninstalling-all-tools-with-missing-receipt (from
      tool_uninstall.rs::tool_uninstall_all_missing_receipt)

## Upgrade - Basic

- [x] tools/upgrade.md#tool-upgrade-upgrading-when-nothing-is-installed (from
      tool_upgrade.rs::tool_upgrade_empty)
- [x] tools/upgrade.md#tool-upgrade-upgrading-a-tool-by-name (from
      tool_upgrade.rs::tool_upgrade_name)
- [x] tools/upgrade.md#tool-upgrade-upgrading-multiple-tools-by-name (from
      tool_upgrade.rs::tool_upgrade_multiple_names)
- [x] tools/upgrade.md#tool-upgrade-upgrading-all-tools (from tool_upgrade.rs::tool_upgrade_all)
- [x] tools/upgrade.md#tool-upgrade-upgrading-preserves-resolution-settings (from
      tool_upgrade.rs::tool_upgrade_settings)

## Upgrade - Constraints

- [x] tools/upgrade.md#tool-upgrade-upgrading-with-pinned-version-shows-hint (from
      tool_upgrade.rs::tool_upgrade_pinned_hint)
- [x] tools/upgrade.md#tool-upgrade-upgrading-with-mixed-constraint-shows-pinned-hint (from
      tool_upgrade.rs::tool_upgrade_pinned_hint_with_mixed_constraint)
- [x] tools/upgrade.md#tool-upgrade-upgrading-respects-constraints (from
      tool_upgrade.rs::tool_upgrade_respect_constraints)
- [x] tools/upgrade.md#tool-upgrade-upgrading-with-inline-constraint (from
      tool_upgrade.rs::tool_upgrade_constraint)
- [x] tools/upgrade.md#tool-upgrade-upgrading-pinned-tool-updates-dependencies (from
      tool_upgrade.rs::tool_upgrade_with)

## Upgrade - Python Selection

- [x] tools/upgrade.md#tool-upgrade-upgrading-tool-to-different-python-version (from
      tool_upgrade.rs::tool_upgrade_python)
- [x] tools/upgrade.md#tool-upgrade-upgrading-all-tools-to-different-python-version (from
      tool_upgrade.rs::tool_upgrade_python_with_all)

## Upgrade - Error Handling

- [x] tools/upgrade.md#tool-upgrade-upgrading-a-non-existing-package (from
      tool_upgrade.rs::tool_upgrade_non_existing_package)
- [x] tools/upgrade.md#continuing-upgrade-when-one-tool-fails (from
      tool_upgrade.rs::tool_upgrade_not_stop_if_upgrade_fails)

## Upgrade - Entrypoints

- [x] tools/upgrade.md#tool-upgrade-upgrading-tool-with-additional-entrypoints-to-different-python
      (from tool_upgrade.rs::test_tool_upgrade_additional_entrypoints)
