# Building Packages

Building sdists and wheels from projects (distinct from build backend internals).

## Basic Building

- [x] build/build.md#basic (from build.rs::build_basic)
- [x] build/build.md#sdist (from build.rs::build_sdist)
- [x] build/build.md#wheel (from build.rs::build_wheel)
- [x] build/build.md#sdist-wheel (from build.rs::build_sdist_wheel)
- [x] build/build.md#wheel-from-sdist (from build.rs::build_wheel_from_sdist)
- [x] build/build.md#fail (from build.rs::build_fail)
- [x] build/build.md#fast-path (from build.rs::build_fast_path)

## Workspaces

- [x] build/workspaces.md#workspace (from build.rs::build_workspace)
- [x] build/workspaces.md#all-with-failure (from build.rs::build_all_with_failure)
- [x] build/workspaces.md#virtual-root (from build.rs::build_workspace_virtual_root)
- [x] build/workspaces.md#trailing-slash (from build.rs::test_workspace_trailing_slash)

## Options

- [x] build/options.md#constraints (from build.rs::build_constraints)
- [x] build/options.md#sha (from build.rs::build_sha)
- [x] build/options.md#quiet (from build.rs::build_quiet)
- [x] build/options.md#no-build-logs (from build.rs::build_no_build_logs)
- [x] build/options.md#hide-output-env-var (from build.rs::build_hide_build_output_env_var)
- [x] build/options.md#hide-output-on-failure (from build.rs::build_hide_build_output_on_failure)
- [x] build/options.md#clear (from build.rs::build_clear)
- [x] build/options.md#no-gitignore (from build.rs::build_no_gitignore)

## Source Discovery

- [x] build/sources.md#non-package (from build.rs::build_non_package)
- [x] build/sources.md#not-a-project (from build.rs::build_pyproject_toml_not_a_project)
- [x] build/sources.md#tool-uv-sources (from build.rs::build_tool_uv_sources)

## File Listing

- [x] build/list-files.md#list-files (from build.rs::build_list_files)
- [x] build/list-files.md#list-files-errors (from build.rs::build_list_files_errors)

## Filesystem Handling

- [x] build/filesystem.md#symlink (from build.rs::build_with_symlink)
- [x] build/filesystem.md#hardlink (from build.rs::build_with_hardlink)
- [x] build/filesystem.md#git-boundary (from build.rs::build_git_boundary_in_dist_build)
- [x] build/filesystem.md#venv-in-sdist (from build.rs::venv_included_in_sdist)

## Build Backend Compatibility

- [x] build/compatibility.md#unconfigured-setuptools (from build.rs::build_unconfigured_setuptools)
- [x] build/compatibility.md#force-pep517 (from build.rs::force_pep517)

## Validation

- [x] build/validation.md#version-mismatch (from build.rs::build_version_mismatch)
- [x] build/validation.md#nonnormalized-name (from build.rs::build_with_nonnormalized_name)
