# Python Management

Installing, finding, pinning, and listing Python versions.

## Directory

- [x] python/dir.md#python-dir (from python_dir.rs::python_dir)

## Find

- [x] python/find.md#basic (from python_find.rs::python_find)
- [x] python/find.md#find-pin (from python_find.rs::python_find_pin)
- [x] python/find.md#find-pin-arbitrary-name (from python_find.rs::python_find_pin_arbitrary_name)
- [x] python/find.md#find-project (from python_find.rs::python_find_project)
- [x] python/find.md#virtual-empty (from python_find.rs::virtual_empty)
- [x] python/find.md#virtual-dependency-group (from python_find.rs::virtual_dependency_group)
- [ ] BLOCKED: find-venv (from python_find.rs::python_find_venv) - complex venv test
- [x] python/find.md#unsupported-version (from python_find.rs::python_find_unsupported_version)
- [ ] BLOCKED: venv-invalid (from python_find.rs::python_find_venv_invalid) - requires venv
- [ ] BLOCKED: managed (from python_find.rs::python_find_managed) - requires managed python
- [ ] BLOCKED: required-python-major-minor (from
      python_find.rs::python_required_python_major_minor) - requires python-managed feature
- [ ] BLOCKED: script (from python_find.rs::python_find_script) - script support needed
- [ ] BLOCKED: script-no-environment (from python_find.rs::python_find_script_no_environment) -
      script support needed
- [ ] BLOCKED: script-python-not-found (from python_find.rs::python_find_script_python_not_found) -
      script support needed
- [ ] BLOCKED: script-no-such-version (from python_find.rs::python_find_script_no_such_version) -
      script support needed
- [x] python/find.md#show-version (from python_find.rs::python_find_show_version)
- [x] python/find.md#path (from python_find.rs::python_find_path)
- [ ] BLOCKED: freethreaded-313 (from python_find.rs::python_find_freethreaded_313) - requires
      python-managed feature
- [ ] BLOCKED: freethreaded-314 (from python_find.rs::python_find_freethreaded_314) - requires
      python-managed feature
- [ ] BLOCKED: prerelease-version-specifiers (from
      python_find.rs::python_find_prerelease_version_specifiers) - requires python-managed feature
- [ ] BLOCKED: prerelease-with-patch-request (from
      python_find.rs::python_find_prerelease_with_patch_request) - requires python-managed feature

## Install - Basic

- [x] python/install.md#install (from python_install.rs::python_install)
- [x] python/install.md#reinstall (from python_install.rs::python_reinstall)
- [x] python/install.md#reinstall-patch (from python_install.rs::python_reinstall_patch)
- [x] python/install.md#automatic (from python_install.rs::python_install_automatic)
- [x] python/install.md#regression-cpython (from python_install.rs::regression_cpython)
- [x] python/install.md#force (from python_install.rs::python_install_force)
- [x] python/install.md#minor (from python_install.rs::python_install_minor)
- [x] python/install.md#default (from python_install.rs::python_install_default)
- [x] python/install.md#default-from-env (from python_install.rs::python_install_default_from_env)
- [x] python/install.md#unknown (from python_install.rs::python_install_unknown)
- [x] python/install.md#broken-link (from python_install.rs::python_install_broken_link)
- [x] python/install.md#invalid-request (from python_install.rs::python_install_invalid_request)
- [x] python/install.md#cached (from python_install.rs::python_install_cached)
- [x] python/install.md#no-cache (from python_install.rs::python_install_no_cache)

## Install - Patch Management

- [x] python/install.md#multiple-patch (from python_install.rs::python_install_multiple_patch)
- [x] python/install.md#patch-dylib (from python_install.rs::python_install_patch_dylib)
- [x] python/install.md#transparent-patch-upgrade-uv-venv (from
      python_install.rs::install_transparent_patch_upgrade_uv_venv)
- [x] python/install.md#install-multiple-patches (from python_install.rs::install_multiple_patches)
- [x] python/install.md#uninstall-highest-patch (from python_install.rs::uninstall_highest_patch)
- [x] python/install.md#no-transparent-upgrade-with-venv-patch (from
      python_install.rs::install_no_transparent_upgrade_with_venv_patch_specification)
- [x] python/install.md#transparent-patch-upgrade-venv-module (from
      python_install.rs::install_transparent_patch_upgrade_venv_module)
- [x] python/install.md#install-lower-patch-automatically (from
      python_install.rs::install_lower_patch_automatically)
- [x] python/install.md#uninstall-last-patch (from python_install.rs::uninstall_last_patch)

## Install - Preview and Prerelease

- [x] python/install.md#preview (from python_install.rs::python_install_preview)
- [x] python/install.md#preview-no-bin (from python_install.rs::python_install_preview_no_bin)
- [x] python/install.md#preview-upgrade (from python_install.rs::python_install_preview_upgrade)
- [x] python/install.md#default-preview (from python_install.rs::python_install_default_preview)
- [x] python/install.md#default-prerelease (from
      python_install.rs::python_install_default_prerelease)
- [x] python/install.md#prerelease (from python_install.rs::python_install_prerelease)
- [x] python/install.md#find-prerelease (from python_install.rs::python_find_prerelease)

## Install - Freethreaded and Debug

- [x] python/install.md#freethreaded (from python_install.rs::python_install_freethreaded)
- [x] python/install.md#debug (from python_install.rs::python_install_debug)
- [x] python/install.md#debug-freethreaded (from
      python_install.rs::python_install_debug_freethreaded)

## Install - Platform-Specific

- [x] python/install.md#emulated-macos (from python_install.rs::python_install_emulated_macos)
- [x] python/install.md#emulated-windows-x86-on-x64 (from
      python_install.rs::python_install_emulated_windows_x86_on_x64)
- [x] python/install.md#armv7 (from python_install.rs::python_install_armv7)

## Install - Alternative Implementations

- [x] python/install.md#pyodide (from python_install.rs::python_install_pyodide)
- [x] python/install.md#build-version (from python_install.rs::python_install_build_version)
- [x] python/install.md#build-version-pypy (from
      python_install.rs::python_install_build_version_pypy)

## Install - Bytecode Compilation

- [x] python/install.md#compile-bytecode (from python_install.rs::python_install_compile_bytecode)
- [x] python/install.md#compile-bytecode-existing (from
      python_install.rs::python_install_compile_bytecode_existing)
- [x] python/install.md#compile-bytecode-upgrade (from
      python_install.rs::python_install_compile_bytecode_upgrade)
- [x] python/install.md#compile-bytecode-multiple (from
      python_install.rs::python_install_compile_bytecode_multiple)
- [x] python/install.md#compile-bytecode-pyodide (from
      python_install.rs::python_install_compile_bytecode_pyodide)
- [x] python/install.md#compile-bytecode-graalpy (from
      python_install.rs::python_install_compile_bytecode_graalpy)
- [x] python/install.md#compile-bytecode-pypy (from
      python_install.rs::python_install_compile_bytecode_pypy)

## Install - Upgrade

- [x] python/install.md#upgrade-not-allowed (from python_install.rs::python_upgrade_not_allowed)
- [x] python/install.md#upgrade (from python_install.rs::python_install_upgrade)
- [x] python/install.md#upgrade-version-file (from
      python_install.rs::python_install_upgrade_version_file)

## List

- [x] python/list.md#list (from python_list.rs::python_list)
- [x] python/list.md#list-pin (from python_list.rs::python_list_pin)
- [ ] BLOCKED: list-venv (from python_list.rs::python_list_venv) - requires venv creation
- [x] python/list.md#unsupported-version (from python_list.rs::python_list_unsupported_version)
- [x] python/list.md#duplicate-path-entries (from
      python_list.rs::python_list_duplicate_path_entries)
- [x] python/list.md#downloads (from python_list.rs::python_list_downloads)
- [ ] BLOCKED: downloads-installed (from python_list.rs::python_list_downloads_installed) - requires
      python-managed feature
- [x] python/list.md#with-mirrors (from python_list.rs::python_list_with_mirrors)

## Module (python -m uv)

- [x] python/module.md#find-uv-bin-venv (from python_module.rs::find_uv_bin_venv)
- [x] python/module.md#find-uv-bin-target (from python_module.rs::find_uv_bin_target)
- [x] python/module.md#find-uv-bin-prefix (from python_module.rs::find_uv_bin_prefix)
- [x] python/module.md#find-uv-bin-base-prefix (from python_module.rs::find_uv_bin_base_prefix)
- [x] python/module.md#find-uv-bin-ephemeral (from
      python_module.rs::find_uv_bin_in_ephemeral_environment)
- [x] python/module.md#find-uv-bin-parent-ephemeral (from
      python_module.rs::find_uv_bin_in_parent_of_ephemeral_environment)
- [x] python/module.md#find-uv-bin-user-bin (from python_module.rs::find_uv_bin_user_bin)
- [x] python/module.md#find-uv-bin-error-message (from python_module.rs::find_uv_bin_error_message)
- [x] python/module.md#find-uv-bin-py38 (from python_module.rs::find_uv_bin_py38)
- [x] python/module.md#find-uv-bin-py39 (from python_module.rs::find_uv_bin_py39)
- [x] python/module.md#find-uv-bin-py310 (from python_module.rs::find_uv_bin_py310)
- [x] python/module.md#find-uv-bin-py311 (from python_module.rs::find_uv_bin_py311)
- [x] python/module.md#find-uv-bin-py312 (from python_module.rs::find_uv_bin_py312)
- [x] python/module.md#find-uv-bin-py313 (from python_module.rs::find_uv_bin_py313)
- [x] python/module.md#find-uv-bin-py314 (from python_module.rs::find_uv_bin_py314)

## Pin

- [x] python/pin.md#pin (from python_pin.rs::python_pin)
- [x] python/pin.md#global-if-no-local (from python_pin.rs::python_pin_global_if_no_local)
- [x] python/pin.md#global-use-local-if-available (from
      python_pin.rs::python_pin_global_use_local_if_available)
- [x] python/pin.md#global-creates-parent-dirs (from
      python_pin.rs::python_pin_global_creates_parent_dirs)
- [x] python/pin.md#no-python (from python_pin.rs::python_pin_no_python)
- [x] python/pin.md#compatible-with-requires-python (from
      python_pin.rs::python_pin_compatible_with_requires_python)
- [x] python/pin.md#warning-not-installed (from
      python_pin.rs::warning_pinned_python_version_not_installed)
- [x] python/pin.md#resolve-no-python (from python_pin.rs::python_pin_resolve_no_python)
- [x] python/pin.md#resolve (from python_pin.rs::python_pin_resolve)
- [x] python/pin.md#with-comments (from python_pin.rs::python_pin_with_comments)
- [ ] BLOCKED: install (from python_pin.rs::python_pin_install) - requires python-managed feature
- [x] python/pin.md#rm (from python_pin.rs::python_pin_rm)

## Upgrade

- [x] python/upgrade.md#upgrade (from python_upgrade.rs::python_upgrade)
- [x] python/upgrade.md#without-version (from python_upgrade.rs::python_upgrade_without_version)
- [x] python/upgrade.md#transparent-from-venv (from
      python_upgrade.rs::python_upgrade_transparent_from_venv)
- [x] python/upgrade.md#transparent-from-venv-preview (from
      python_upgrade.rs::python_upgrade_transparent_from_venv_preview)
- [x] python/upgrade.md#ignored-with-python-pin (from
      python_upgrade.rs::python_upgrade_ignored_with_python_pin)
- [x] python/upgrade.md#no-transparent-upgrade-with-venv-patch (from
      python_upgrade.rs::python_no_transparent_upgrade_with_venv_patch_specification)
- [x] python/upgrade.md#transparent-upgrade-venv-venv (from
      python_upgrade.rs::python_transparent_upgrade_venv_venv)
- [x] python/upgrade.md#transparent-from-venv-module (from
      python_upgrade.rs::python_upgrade_transparent_from_venv_module)
- [x] python/upgrade.md#transparent-from-venv-module-in-venv (from
      python_upgrade.rs::python_upgrade_transparent_from_venv_module_in_venv)
- [x] python/upgrade.md#force-install (from python_upgrade.rs::python_upgrade_force_install)
- [x] python/upgrade.md#implementation (from python_upgrade.rs::python_upgrade_implementation)
