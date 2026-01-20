# Workspaces

Workspace discovery, members, virtual vs root workspaces, inheritance, initializing projects within
workspaces.

**Note:**

- Tests are located at `test/uv/commands/uv-workspace/*.md`
- For mdtest features (snapshots, filters, tree, etc.), see `test/uv/readme.md`
- File snapshot syntax: ` ```toml title="pyproject.toml" snapshot=true`
- For directory creation, use `tree create=true` blocks (not `mkdir` commands)
- Command argument parsing doesn't handle shell quoting - avoid arguments with spaces

## Blockers

### Git-based tests

Tests that clone Git repositories (e.g., `transitive_dep_in_git_workspace_*`) require network access
and may not be suitable for mdtest.

### uv_build version filter (TODO: add filter)

Some init tests use dynamic uv_build version filtering (e.g., `uv_build>=[CURRENT_VERSION]`). Add a
filter like `uv-build-version = true` to mdtest for this.

## Discovery (migrated to sync.md)

- [x] uv-workspace/sync.md#just-a-project-no-workspace (from
      workspace.rs::test_albatross_just_project)
- [x] uv-workspace/sync.md#root-workspace (from workspace.rs::test_albatross_root_workspace)
- [x] uv-workspace/sync.md#root-workspace-from-member (from
      workspace.rs::test_albatross_root_workspace_bird_feeder)
- [x] uv-workspace/sync.md#virtual-workspace (from workspace.rs::test_albatross_virtual_workspace)
- [x] uv-workspace/sync.md#project-in-excluded-directory (from
      workspace.rs::test_albatross_project_in_excluded)
- [x] uv-workspace/sync.md#example-directory-as-workspace (from
      workspace.rs::test_albatross_in_examples)

## Running Commands

- [x] uv-workspace/run.md#run-with-package-in-virtual-workspace (from
      workspace.rs::test_uv_run_with_package_virtual_workspace)
- [x] uv-workspace/run.md#run-from-virtual-workspace-root (from
      workspace.rs::test_uv_run_virtual_workspace_root)
- [x] uv-workspace/run.md#run-with-package-in-root-workspace (from
      workspace.rs::test_uv_run_with_package_root_workspace)
- [x] uv-workspace/run.md#run-with-isolated (from workspace.rs::test_uv_run_isolate)
- [x] uv-workspace/run.md#run-in-workspace-with-dependencies (from run.rs::run_in_workspace)
- [x] uv-workspace/run.md#run-with-target-workspace-discovery (from
      run.rs::run_target_workspace_discovery)

## Locking

- [ ] commands/uv-workspace/lock.md#lock-idempotence-in-root-workspace (from
      workspace.rs::workspace_lock_idempotence_root_workspace) - **Staying in Rust** (requires
      fixture data and helper functions)
- [ ] commands/uv-workspace/lock.md#lock-idempotence-in-virtual-workspace (from
      workspace.rs::workspace_lock_idempotence_virtual_workspace) - **Staying in Rust** (requires
      fixture data and helper functions)
- [x] commands/uv-workspace/lock.md#lock-with-conflicting-workspace-members (from
      lock.rs::lock_conflicting_workspace_members)
- [x] commands/uv-workspace/lock.md#lock-with-conflicting-dependencies-on-direct-dependency (from
      lock.rs::lock_conflicting_workspace_members_depends_direct)
- [x]
  commands/uv-workspace/lock.md#lock-with-conflicting-dependencies-on-direct-dependency-with-extra
  (from lock.rs::lock_conflicting_workspace_members_depends_direct_extra)
- [x] commands/uv-workspace/lock.md#lock-with-conflicting-dependencies-on-transitive-dependency
      (from lock.rs::lock_conflicting_workspace_members_depends_transitive)
- [x]
  commands/uv-workspace/lock.md#lock-with-conflicting-dependencies-on-transitive-dependency-with-extra
  (from lock.rs::lock_conflicting_workspace_members_depends_transitive_extra)
- [x] commands/uv-workspace/lock.md#lock-with-non-workspace-source (from
      lock.rs::lock_non_workspace_source)
- [x] commands/uv-workspace/lock.md#lock-with-no-workspace-source (from
      lock.rs::lock_no_workspace_source)
- [x] commands/uv-workspace/lock.md#lock-workspace-member-from-index (from
      lock.rs::lock_index_workspace_member)
- [x] commands/uv-workspace/lock.md#lock-with-dependency-groups-in-workspace (from
      lock.rs::lock_group_workspace)
- [x] commands/uv-workspace/lock.md#lock-workspace-member-with-dynamic-version (from
      lock.rs::lock_dynamic_version_workspace_member)
- [x] commands/uv-workspace/lock.md#lock-path-dependency-with-explicit-index-in-workspace (from
      lock.rs::lock_path_dependency_explicit_index_workspace_member)

## Members

- [x] commands/uv-workspace/members.md#empty-member-directory (from
      workspace.rs::workspace_empty_member)
- [x] commands/uv-workspace/members.md#hidden-directories-are-ignored (from
      workspace.rs::workspace_hidden_files)
- [x] commands/uv-workspace/members.md#hidden-member-with-valid-pyproject-toml (from
      workspace.rs::workspace_hidden_member)
- [x] commands/uv-workspace/members.md#non-included-project-is-independent (from
      workspace.rs::workspace_non_included_member)
- [x] commands/uv-workspace/members.md#member-with-leading-dot-slash (from
      workspace.rs::workspace_members_with_leading_dot_slash)
- [x] commands/uv-workspace/members.md#member-with-parent-directory-reference (from
      workspace.rs::workspace_members_with_parent_directory)
- [x] commands/uv-workspace/members.md#member-with-complex-relative-paths (from
      workspace.rs::workspace_members_with_complex_relative_paths)

## Inheritance

- [x] commands/uv-workspace/inheritance.md#inherit-sources (from
      workspace.rs::workspace_inherit_sources)
- [x] commands/uv-workspace/inheritance.md#path-hopping (from workspace.rs::test_path_hopping)
- [x] commands/uv-workspace/dependencies.md#cross-workspace-path-dependencies (from
      workspace.rs::workspace_to_workspace_paths_dependencies)

## Dependencies

- [x] commands/uv-workspace/dependencies.md#unsatisfiable-member-dependency (from
      workspace.rs::workspace_unsatisfiable_member_dependencies)
- [x] commands/uv-workspace/dependencies.md#conflicting-member-dependencies (from
      workspace.rs::workspace_unsatisfiable_member_dependencies_conflicting)
- [x] commands/uv-workspace/dependencies.md#three-way-conflicting-dependencies (from
      workspace.rs::workspace_unsatisfiable_member_dependencies_conflicting_threeway)
- [x] commands/uv-workspace/dependencies.md#conflicting-optional-dependency (from
      workspace.rs::workspace_unsatisfiable_member_dependencies_conflicting_extra)
- [ ] dependencies.md#unsatisfiable-dev (from
      workspace.rs::workspace_unsatisfiable_member_dependencies_conflicting_dev)
- [x] commands/uv-workspace/dependencies.md#member-name-shadows-external-dependency (from
      workspace.rs::workspace_member_name_shadows_dependencies)
- [ ] dependencies.md#transitive-git-no-root (from
      workspace.rs::transitive_dep_in_git_workspace_no_root)
- [ ] dependencies.md#transitive-git-with-root (from
      workspace.rs::transitive_dep_in_git_workspace_with_root)

## Syncing

- [x] commands/uv-workspace/sync.md#sync-workspace-members-with-transitive-dependencies (from
      sync.rs::sync_workspace_members_with_transitive_dependencies)
- [x] commands/uv-workspace/sync.md#sync-non-existent-extra-in-workspace-member (from
      sync.rs::sync_non_existent_extra_workspace_member)
- [x] commands/uv-workspace/sync.md#sync-non-existent-extra-in-virtual-workspace (from
      sync.rs::sync_non_existent_extra_non_project_workspace)
- [x] commands/uv-workspace/sync.md#sync-with---no-install-workspace (from
      sync.rs::no_install_workspace)
- [x] commands/uv-workspace/sync.md#sync-workspace-with-custom-environment-path (from
      sync.rs::sync_workspace_custom_environment_path)
- [x] commands/uv-workspace/sync.md#sync-workspace-with-build-system-requires (from
      sync.rs::build_system_requires_workspace)
- [x] commands/uv-workspace/sync.md#toggle-workspace-editable-mode (from
      sync.rs::toggle_workspace_editable)
- [x] commands/uv-workspace/sync.md#workspace-editable-conflict-resolution (from
      sync.rs::workspace_editable_conflict)

## Initialization

- [x] commands/uv-workspace/init.md#initialize-project-inside-project (from
      init.rs::init_project_inside_project)
- [x] commands/uv-workspace/init.md#initialize-in-workspace-with-explicit-members (from
      init.rs::init_explicit_workspace)
- [x] commands/uv-workspace/init.md#initialize-in-virtual-workspace (from
      init.rs::init_virtual_workspace)
- [x] commands/uv-workspace/init.md#initialize-nested-virtual-workspace (from
      init.rs::init_nested_virtual_workspace)
- [x] commands/uv-workspace/init.md#initialize-when-path-matches-members-glob (from
      init.rs::init_matches_members)
- [x] commands/uv-workspace/init.md#initialize-when-path-matches-exclude (from
      init.rs::init_matches_exclude)
- [x] commands/uv-workspace/init.md#initialize-with---no-workspace (from init.rs::init_no_workspace)
- [x] commands/uv-workspace/init.md#initialize-multiple-projects-in-workspace (from
      init.rs::init_workspace)
- [x] commands/uv-workspace/init.md#initialize-with-relative-path-argument (from
      init.rs::init_workspace_relative_sub_package)
- [x] commands/uv-workspace/init.md#initialize-from-outside-workspace-directory (from
      init.rs::init_workspace_outside)
- [x] commands/uv-workspace/init.md#initialize-with---no-workspace-produces-no-warning (from
      init.rs::init_no_workspace_warning)
- [x] commands/uv-workspace/init.md#initialize-member-inherits-workspace-requires-python (from
      init.rs::init_requires_python_workspace)

## Editing Dependencies

- [x] commands/uv-workspace/edit.md#add-remove-in-workspace (from edit.rs::add_remove_workspace)
- [x] commands/uv-workspace/edit.md#add-with-editable (from edit.rs::add_workspace_editable)
- [x] commands/uv-workspace/edit.md#add-workspace-path-dependency (from edit.rs::add_workspace_path)
- [x] commands/uv-workspace/edit.md#add-path-with-implicit-workspace-creation (from
      edit.rs::add_path_implicit_workspace)
- [x] commands/uv-workspace/edit.md#add-path-with---no-workspace-flag (from
      edit.rs::add_path_no_workspace)
- [x] commands/uv-workspace/edit.md#failed-add-reverts-workspace-changes-at-root (from
      edit.rs::fail_to_add_revert_workspace_root)
- [x] commands/uv-workspace/edit.md#failed-add-reverts-workspace-changes-at-member (from
      edit.rs::fail_to_add_revert_workspace_member)
- [x] commands/uv-workspace/edit.md#add-path-with-existing-workspace (from
      edit.rs::add_path_with_existing_workspace)
- [x] commands/uv-workspace/edit.md#add-path-with---workspace-flag (from
      edit.rs::add_path_with_workspace)
- [x] commands/uv-workspace/edit.md#add-path-within-workspace-defaults-to-workspace (from
      edit.rs::add_path_within_workspace_defaults_to_workspace)
- [x] commands/uv-workspace/edit.md#add-path-with-explicit---no-workspace (from
      edit.rs::add_path_with_no_workspace)
- [x] commands/uv-workspace/edit.md#add-path-outside-workspace-defaults-to-path (from
      edit.rs::add_path_outside_workspace_no_default)

## Commands

- [x] commands/uv-workspace/dir.md#simple-workspace (from workspace_dir.rs::workspace_dir_simple)
- [x] commands/uv-workspace/dir.md#specific-package (from
      workspace_dir.rs::workspace_dir_specific_package)
- [x] commands/uv-workspace/dir.md#from-member (from
      workspace_dir.rs::workspace_metadata_from_member)
- [x] commands/uv-workspace/dir.md#package-doesnt-exist (from
      workspace_dir.rs::workspace_dir_package_doesnt_exist)
- [x] commands/uv-workspace/dir.md#no-project (from workspace_dir.rs::workspace_metadata_no_project)
- [x] commands/uv-workspace/list.md#simple-workspace (from workspace_list.rs::workspace_list_simple)
- [x] commands/uv-workspace/list.md#root-workspace (from
      workspace_list.rs::workspace_list_root_workspace)
- [x] commands/uv-workspace/list.md#virtual-workspace (from
      workspace_list.rs::workspace_list_virtual_workspace)
- [x] commands/uv-workspace/list.md#list-from-member (from
      workspace_list.rs::workspace_list_from_member)
- [x] commands/uv-workspace/list.md#multiple-members (from
      workspace_list.rs::workspace_list_multiple_members)
- [x] commands/uv-workspace/list.md#single-project (from
      workspace_list.rs::workspace_list_single_project)
- [x] commands/uv-workspace/list.md#with-excluded-packages (from
      workspace_list.rs::workspace_list_with_excluded)
- [x] commands/uv-workspace/list.md#no-project (from workspace_list.rs::workspace_list_no_project)
- [x] commands/uv-workspace/metadata.md#simple-workspace (from
      workspace_metadata.rs::workspace_metadata_simple)
- [x] commands/uv-workspace/metadata.md#root-workspace (from
      workspace_metadata.rs::workspace_metadata_root_workspace)
- [x] commands/uv-workspace/metadata.md#virtual-workspace (from
      workspace_metadata.rs::workspace_metadata_virtual_workspace)
- [x] commands/uv-workspace/metadata.md#from-member (from
      workspace_metadata.rs::workspace_metadata_from_member)
- [x] commands/uv-workspace/metadata.md#multiple-members (from
      workspace_metadata.rs::workspace_metadata_multiple_members)
- [x] commands/uv-workspace/metadata.md#single-project (from
      workspace_metadata.rs::workspace_metadata_single_project)
- [x] commands/uv-workspace/metadata.md#with-excluded-packages (from
      workspace_metadata.rs::workspace_metadata_with_excluded)
- [x] commands/uv-workspace/metadata.md#no-project (from
      workspace_metadata.rs::workspace_metadata_no_project)
