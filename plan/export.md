# Export

Exporting dependencies to requirements.txt, PEP 751, CycloneDX, etc.

## requirements.txt

- [x] export/requirements-txt.md#basic-dependency-export (from
      export.rs::requirements_txt_dependency)
- [x] export/requirements-txt.md#no-header (from export.rs::requirements_txt_export_no_header)
- [x] export/requirements-txt.md#dependency-with-extra (from
      export.rs::requirements_txt_dependency_extra)
- [x] export/requirements-txt.md#project-extras (from export.rs::requirements_txt_project_extra)
- [x] export/requirements-txt.md#pruning-dependencies (from export.rs::requirements_txt_prune)
- [x] export/requirements-txt.md#dependency-marker (from
      export.rs::requirements_txt_dependency_marker)
- [x] export/requirements-txt.md#multiple-markers (from
      export.rs::requirements_txt_dependency_multiple_markers)
- [x] export/requirements-txt.md#conflicting-markers (from
      export.rs::requirements_txt_dependency_conflicting_markers)
- [x] export/requirements-txt.md#non-root-package-export (from export.rs::requirements_txt_non_root)
- [x] export/requirements-txt.md#all-packages (from export.rs::allrequirements*txt*)
- [x] export/requirements-txt.md#frozen (from export.rs::requirements_txt_frozen)
- [x] export/requirements-txt.md#create-missing-dir (from
      export.rs::requirements_txt_create_missing_dir)
- [x] export/requirements-txt.md#non-project (from export.rs::requirements_txt_non_project)
- [x] export/requirements-txt.md#virtual-empty-project (from export.rs::virtual_empty)
- [x] export/requirements-txt.md#virtual-dependency-groups (from
      export.rs::virtual_dependency_group)
- [x] export/requirements-txt.md#https-git-credentials (from
      export.rs::requirements_txt_https_git_credentials)
- [x] export/requirements-txt.md#ssh-git-username (from
      export.rs::requirements_txt_ssh_git_username)
- [x] export/requirements-txt.md#https-credentials (from
      export.rs::requirements_txt_https_credentials)
- [x] export/requirements-txt.md#non-project-marker (from
      export.rs::requirements_txt_non_project_marker)
- [x] export/requirements-txt.md#non-project-workspace (from
      export.rs::requirements_txt_non_project_workspace)
- [x] export/requirements-txt.md#non-project-fork (from
      export.rs::requirements_txt_non_project_fork)
- [x] export/requirements-txt.md#relative-path (from export.rs::requirements_txt_relative_path)
- [x] export/requirements-txt.md#dev (from export.rs::devrequirements*txt*)
- [x] export/requirements-txt.md#no-hashes (from export.rs::requirements_txt_no_hashes)
- [x] export/requirements-txt.md#output-to-file (from export.rs::requirements_txt_output_file)
- [x] export/requirements-txt.md#no-emit (from export.rs::requirements_txt_no_emit)
- [x] export/requirements-txt.md#only-emit (from export.rs::requirements_txt_only_emit)
- [x] export/requirements-txt.md#no-editable (from export.rs::requirements_txt_no_editable)
- [x] export/requirements-txt.md#export-group (from export.rs::requirements_txt_export_group)
- [x] export/requirements-txt.md#script (from export.rs::requirements_txt_script)
- [x] export/requirements-txt.md#conflicts (from export.rs::requirements_txt_conflicts)
- [x] export/requirements-txt.md#simple-conflict-markers (from
      export.rs::requirements_txt_simple_conflict_markers)
- [x] export/requirements-txt-torch.md#complex-conflict-markers (from
      export.rs::requirements_txt_complex_conflict_markers)
- [x] export/requirements-txt.md#cyclic-dependencies (from
      export.rs::requirements_txt_cyclic_dependencies)
- [x] export/requirements-txt.md#cyclic-dependencies-conflict (from
      export.rs::requirements_txt_cyclic_dependencies_conflict)

## PEP 751

- [x] export/pep-751.md#dependency (from export.rs::pep_751_dependency)
- [x] export/pep-751.md#no-header (from export.rs::pep_751_export_no_header)
- [x] export/pep-751.md#no-editable (from export.rs::pep_751_export_no_editable)
- [x] export/pep-751.md#dependency-extra (from export.rs::pep_751_dependency_extra)
- [x] export/pep-751.md#project-extra (from export.rs::pep_751_project_extra)
- [x] export/pep-751.md#git-dependency (from export.rs::pep_751_git_dependency)
- [x] export/pep-751.md#wheel-url (from export.rs::pep_751_wheel_url)
- [x] export/pep-751.md#sdist-url (from export.rs::pep_751_sdist_url)
- [x] export/pep-751.md#sdist-url-subdirectory (from export.rs::pep_751_sdist_url_subdirectory)
- [x] export/pep-751.md#infer-output-format (from export.rs::pep_751_infer_output_format)
- [x] export/pep-751.md#filename (from export.rs::pep_751_filename)
- [x] export/pep-751.md#https-git-credentials (from export.rs::pep_751_https_git_credentials)
- [x] export/pep-751.md#https-credentials (from export.rs::pep_751_https_credentials)

## CycloneDX

- [x] export/cyclonedx.md#basic-export (from export.rs::cyclonedx_export_basic)
- [x] export/cyclonedx.md#direct-url (from export.rs::cyclonedx_export_direct_url)
- [x] export/cyclonedx.md#git-dependency (from export.rs::cyclonedx_export_git_dependency)
- [x] export/cyclonedx.md#no-dependencies (from export.rs::cyclonedx_export_no_dependencies)
- [x] export/cyclonedx.md#mixed-source-types (from export.rs::cyclonedx_export_mixed_source_types)
- [x] export/cyclonedx.md#project-extra (from export.rs::cyclonedx_export_project_extra)
- [x] export/cyclonedx.md#project-extra-optional-flag (from
      export.rs::cyclonedx_export_project_extra_with_optional_flag)
- [x] export/cyclonedx.md#workspace-member (from export.rs::cyclonedx_export_with_workspace_member)
- [x] export/cyclonedx.md#workspace-non-root (from export.rs::cyclonedx_export_workspace_non_root)
- [x] export/cyclonedx.md#workspace-extras (from export.rs::cyclonedx_export_workspace_with_extras)
- [x] export/cyclonedx.md#workspace-frozen (from export.rs::cyclonedx_export_workspace_frozen)
- [x] export/cyclonedx.md#workspace-all-packages (from
      export.rs::cyclonedx_export_workspace_all_packages)
- [x] export/cyclonedx.md#all-packages-non-workspace-root (from
      export.rs::cyclonedx_export_all_packages_non_workspace_root_dependency)
- [x] export/cyclonedx.md#workspace-mixed-dependencies (from
      export.rs::cyclonedx_export_workspace_mixed_dependencies)
- [x] export/cyclonedx.md#dependency-marker (from export.rs::cyclonedx_export_dependency_marker)
- [x] export/cyclonedx.md#multiple-dependency-markers (from
      export.rs::cyclonedx_export_multiple_dependency_markers)
- [x] export/cyclonedx.md#dependency-extra (from export.rs::cyclonedx_export_dependency_extra)
- [x] export/cyclonedx.md#prune (from export.rs::cyclonedx_export_prune)
- [x] export/cyclonedx.md#group (from export.rs::cyclonedx_export_group)
- [x] export/cyclonedx.md#non-project (from export.rs::cyclonedx_export_non_project)
- [x] export/cyclonedx.md#no-emit (from export.rs::cyclonedx_export_no_emit)
- [x] export/cyclonedx.md#relative-path (from export.rs::cyclonedx_export_relative_path)
- [x] export/cyclonedx.md#cyclic-dependencies (from export.rs::cyclonedx_export_cyclic_dependencies)
- [x] export/cyclonedx.md#dev-dependencies (from export.rs::cyclonedx_export_dev_dependencies)
- [x] export/cyclonedx.md#conflicting-workspace-members (from
      export.rs::cyclonedx_export_all_packages_conflicting_workspace_members)
- [x] export/cyclonedx.md#alternative-registry (from
      export.rs::cyclonedx_export_alternative_registry)

## Common Options

- [x] export/options.md#no-editable-env-var (from export.rs::no_editable_env_var)
- [x] export/options.md#group-extra-conflict (from export.rs::export_only_group_and_extra_conflict)
- [x] export/options.md#lock-workspace-mismatch (from
      export.rs::export_lock_workspace_mismatch_with_frozen)
- [x] export/options.md#multiple-packages (from export.rs::multiple_packages)
