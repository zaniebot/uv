# Workspace Directory

Tests for the `uv workspace dir` command.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Simple workspace

<!-- Derived from [`workspace_dir::workspace_dir_simple`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_dir.rs#L8-L27) -->

Initialize a workspace with one member and check the directory output.

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

```console working-dir="foo"
$ uv workspace dir
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/foo

----- stderr -----
warning: The `uv workspace dir` command is experimental and may change without warning. Pass `--preview-features workspace-dir` to disable this warning.
```

## Specific package with --package

<!-- Derived from [`workspace_dir::workspace_dir_specific_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_dir.rs#L29-L60) -->

Initialize a workspace with nested members and use `--package` to get specific member directories.

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

```console working-dir="foo"
$ uv init bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `bar` as member of workspace `[TEMP_DIR]/foo`
Initialized project `bar` at `[TEMP_DIR]/foo/bar`
```

The root workspace directory:

```console working-dir="foo"
$ uv workspace dir
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/foo

----- stderr -----
warning: The `uv workspace dir` command is experimental and may change without warning. Pass `--preview-features workspace-dir` to disable this warning.
```

With `--package bar`:

```console working-dir="foo"
$ uv workspace dir --package bar
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/foo/bar

----- stderr -----
warning: The `uv workspace dir` command is experimental and may change without warning. Pass `--preview-features workspace-dir` to disable this warning.
```

## Directory from member

<!-- Derived from [`workspace_dir::workspace_metadata_from_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_dir.rs#L62-L88) -->

When run from a workspace member directory, returns the workspace root.

```toml
# file: pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["bird-feeder"]

[tool.uv.sources]
bird-feeder = { workspace = true }

[tool.uv.workspace]
members = ["packages/*"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```toml
# file: packages/bird-feeder/pyproject.toml

[project]
name = "bird-feeder"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["seeds"]

[tool.uv.sources]
seeds = { workspace = true }

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```toml
# file: packages/seeds/pyproject.toml

[project]
name = "seeds"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

Running from the `bird-feeder` member returns the workspace root:

```console working-dir="packages/bird-feeder"
$ uv workspace dir
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/

----- stderr -----
warning: The `uv workspace dir` command is experimental and may change without warning. Pass `--preview-features workspace-dir` to disable this warning.
```

## Non-existent package

<!-- Derived from [`workspace_dir::workspace_dir_package_doesnt_exist`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_dir.rs#L90-L110) -->

Requesting a non-existent package produces an error.

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

```console working-dir="foo"
$ uv workspace dir --package bar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: The `uv workspace dir` command is experimental and may change without warning. Pass `--preview-features workspace-dir` to disable this warning.
error: Package `bar` not found in workspace.
```

## No project

<!-- Derived from [`workspace_dir::workspace_metadata_no_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_dir.rs#L112-L127) -->

Running outside a project produces an error.

```console
$ uv workspace dir
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: The `uv workspace dir` command is experimental and may change without warning. Pass `--preview-features workspace-dir` to disable this warning.
error: No `pyproject.toml` found in current directory or any parent directory
```
