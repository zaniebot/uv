# Project Lifecycle

Standalone project initialization, adding/removing dependencies, version management.

## Init - Basic

- [x] init/init.md#basic (from init.rs::init)
- [x] init/init.md#bare (from init.rs::init_bare)
- [x] init/init.md#dot-args (from init.rs::init_dot_args)
- [x] init/init.md#normalized-names (from init.rs::init_normalized_names)
- [x] init/init.md#hidden (from init.rs::init_hidden)
- [x] init/init.md#non-ascii-directory (from init.rs::init_non_ascii_directory)
- [x] init/init.md#cache (from init.rs::init_cache)

## Init - Applications

- [x] init/applications.md#application (from init.rs::init_application)
- [x] init/applications.md#application-hello-exists (from init.rs::init_application_hello_exists)
- [x] init/applications.md#application-other-python-exists (from
      init.rs::init_application_other_python_exists)
- [x] init/applications.md#application-package (from init.rs::init_application_package)
- [x] init/applications.md#application-current-dir (from init.rs::init_application_current_dir)

## Init - Libraries

- [x] init/libraries.md#library (from init.rs::init_library)
- [x] init/libraries.md#library-no-package (from init.rs::init_library_no_package)
- [x] init/libraries.md#library-current-dir (from init.rs::init_library_current_dir)
- [x] init/libraries.md#bare-lib (from init.rs::init_bare_lib)
- [x] init/libraries.md#bare-package (from init.rs::init_bare_package)
- [x] init/libraries.md#bare-opt-in (from init.rs::init_bare_opt_in)
- [x] init/libraries.md#package-preview (from init.rs::init_package_preview)
- [x] init/libraries.md#py-typed-exists (from init.rs::init_py_typed_exists)

## Init - Scripts

- [x] init/scripts.md#script (from init.rs::init_script)
- [x] init/scripts.md#script-bare (from init.rs::init_script_bare)
- [x] init/scripts.md#script-python-version (from init.rs::init_script_python_version)
- [x] init/scripts.md#script-create-directory (from init.rs::init_script_create_directory)
- [x] init/scripts.md#script-file-conflicts (from init.rs::init_script_file_conflicts)
- [x] init/scripts.md#script-shebang (from init.rs::init_script_shebang)
- [x] init/scripts.md#script-picks-latest-stable-version (from
      init.rs::init_script_picks_latest_stable_version)

## Init - Build Backends

- [x] init/build-backends.md#application-package-flit (from init.rs::init_application_package_flit)
- [x] init/build-backends.md#library-flit (from init.rs::init_library_flit)
- [x] init/build-backends.md#library-poetry (from init.rs::init_library_poetry)
- [x] init/build-backends.md#app-build-backend-maturin (from
      init.rs::init_app_build_backend_maturin)
- [x] init/build-backends.md#app-build-backend-scikit (from init.rs::init_app_build_backend_scikit)
- [x] init/build-backends.md#lib-build-backend-maturin (from
      init.rs::init_lib_build_backend_maturin)
- [x] init/build-backends.md#lib-build-backend-scikit (from init.rs::init_lib_build_backend_scikit)
- [x] init/build-backends.md#application-package-hatchling (from
      init.rs::init_application_package_hatchling)
- [x] init/build-backends.md#backend-implies-package (from init.rs::init_backend_implies_package)

## Init - Git/VCS Integration

- [x] init/vcs.md#git (from init.rs::init_git)
- [x] init/vcs.md#vcs-none (from init.rs::init_vcs_none)
- [x] init/vcs.md#inside-git-repo (from init.rs::init_inside_git_repo)
- [x] init/vcs.md#git-not-installed (from init.rs::init_git_not_installed)
- [x] init/vcs.md#git-states (from init.rs::git_states)

## Init - Python Requirements

- [x] init/python.md#requires-python-version (from init.rs::init_requires_python_version)
- [x] init/python.md#requires-python-specifiers (from init.rs::init_requires_python_specifiers)
- [x] init/python.md#requires-python-version-file (from init.rs::init_requires_python_version_file)
- [x] init/python.md#python-variant (from init.rs::init_python_variant)

## Init - Options

- [x] init/options.md#no-readme (from init.rs::init_no_readme)
- [x] init/options.md#no-pin-python (from init.rs::init_no_pin_python)
- [x] init/options.md#with-author (from init.rs::init_with_author)
- [x] init/options.md#with-description (from init.rs::init_with_description)
- [x] init/options.md#without-description (from init.rs::init_without_description)
- [x] init/options.md#isolated (from init.rs::init_isolated)
- [x] init/options.md#unmanaged (from init.rs::init_unmanaged)

## Init - Existing Environments

- [x] init/existing.md#existing-environment (from init.rs::init_existing_environment)
- [x] init/existing.md#existing-environment-parent (from init.rs::init_existing_environment_parent)
- [x] init/existing.md#project-inside-project (from init.rs::init_project_inside_project)
- [x] init/existing.md#virtual-project (from init.rs::init_virtual_project)
- [x] init/existing.md#matches-members (from init.rs::init_matches_members)
- [x] init/existing.md#matches-exclude (from init.rs::init_matches_exclude)
- [x] init/existing.md#working-directory-change (from init.rs::init_working_directory_change)

## Init - Error Handling

- [x] init/errors.md#failure (from init.rs::init_failure)
- [x] init/errors.md#failure-with-invalid-option-named-backend (from
      init.rs::init_failure_with_invalid_option_named_backend)
- [x] init/errors.md#project-flag-not-allowed-under-preview (from
      init.rs::init_project_flag_is_not_allowed_under_preview)
- [x] init/errors.md#project-flag-ignored-with-explicit-path (from
      init.rs::init_project_flag_is_ignored_with_explicit_path)
- [x] init/errors.md#project-flag-warned-without-path (from
      init.rs::init_project_flag_is_warned_without_path)

## Edit - Adding Dependencies

- [x] edit/add.md#registry (from edit.rs::add_registry)
- [x] edit/add.md#unnamed (from edit.rs::add_unnamed)
- [x] edit/add.md#repeat (from edit.rs::add_repeat)
- [x] edit/add.md#frozen (from edit.rs::add_frozen)
- [x] edit/add.md#no-sync (from edit.rs::add_no_sync)
- [x] edit/add.md#error (from edit.rs::add_error)
- [x] edit/add.md#environment-yml-error (from edit.rs::add_environment_yml_error)
- [x] edit/add.md#ambiguous (from edit.rs::add_ambiguous)
- [x] edit/add.md#self (from edit.rs::add_self)
- [x] edit/add.md#shadowed-name (from edit.rs::add_shadowed_name)

## Edit - Version Bounds

- [x] edit/bounds.md#lower-bound (from edit.rs::add_lower_bound)
- [x] edit/bounds.md#lower-bound-existing (from edit.rs::add_lower_bound_existing)
- [x] edit/bounds.md#lower-bound-raw (from edit.rs::add_lower_bound_raw)
- [x] edit/bounds.md#lower-bound-dev (from edit.rs::add_lower_bound_dev)
- [x] edit/bounds.md#lower-bound-optional (from edit.rs::add_lower_bound_optional)
- [x] edit/bounds.md#lower-bound-local (from edit.rs::add_lower_bound_local)
- [x] edit/bounds.md#bounds (from edit.rs::add_bounds)
- [x] edit/bounds.md#bounds-requirement-over-bounds-kind (from
      edit.rs::add_bounds_requirement_over_bounds_kind)

## Edit - Groups

- [x] edit/groups.md#add-remove-dev (from edit.rs::add_remove_dev)
- [x] edit/groups.md#add-remove-optional (from edit.rs::add_remove_optional)
- [x] edit/groups.md#add-remove-inline-optional (from edit.rs::add_remove_inline_optional)
- [x] edit/groups.md#update-existing-dev (from edit.rs::update_existing_dev)
- [x] edit/groups.md#add-existing-dev (from edit.rs::add_existing_dev)
- [x] edit/groups.md#update-existing-dev-group (from edit.rs::update_existing_dev_group)
- [x] edit/groups.md#add-existing-dev-group (from edit.rs::add_existing_dev_group)
- [x] edit/groups.md#remove-both-dev (from edit.rs::remove_both_dev)
- [x] edit/groups.md#remove-both-dev-group (from edit.rs::remove_both_dev_group)
- [x] edit/groups.md#disallow-group-script-add (from edit.rs::disallow_group_script_add)
- [x] edit/groups.md#add-group (from edit.rs::add_group)
- [x] edit/groups.md#add-group-normalize (from edit.rs::add_group_normalize)
- [x] edit/groups.md#add-group-before-commented-groups (from
      edit.rs::add_group_before_commented_groups)
- [x] edit/groups.md#add-group-between-commented-groups (from
      edit.rs::add_group_between_commented_groups)
- [x] edit/groups.md#add-group-to-unsorted (from edit.rs::add_group_to_unsorted)
- [x] edit/groups.md#remove-group (from edit.rs::remove_group)
- [x] edit/groups.md#add-group-comment (from edit.rs::add_group_comment)
- [x] edit/groups.md#add-empty-requirements-group (from edit.rs::add_empty_requirements_group)
- [x] edit/groups.md#add-empty-requirements-optional (from edit.rs::add_empty_requirements_optional)
- [x] edit/groups.md#add-include-default-groups (from edit.rs::add_include_default_groups)
- [x] edit/groups.md#remove-include-default-groups (from edit.rs::remove_include_default_groups)
- [x] edit/groups.md#add-optional-normalize (from edit.rs::add_optional_normalize)

## Edit - Virtual Projects

- [x] edit/virtual.md#add-non-project (from edit.rs::add_non_project)
- [x] edit/virtual.md#add-virtual-empty (from edit.rs::add_virtual_empty)
- [x] edit/virtual.md#add-virtual-dependency-group (from edit.rs::add_virtual_dependency_group)
- [x] edit/virtual.md#remove-virtual-empty (from edit.rs::remove_virtual_empty)
- [x] edit/virtual.md#remove-virtual-dependency-group (from
      edit.rs::remove_virtual_dependency_group)

## Edit - Removing Dependencies

- [x] edit/remove.md#remove-registry (from edit.rs::remove_registry)
- [x] edit/remove.md#remove-repeated (from edit.rs::remove_repeated)
- [x] edit/remove.md#remove-requirement (from edit.rs::remove_requirement)
- [x] edit/remove.md#remove-all-with-comments (from edit.rs::remove_all_with_comments)
