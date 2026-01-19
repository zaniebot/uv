# Workspace List

Tests for the `uv workspace list` command.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Simple workspace

<!-- Derived from [`workspace_list::workspace_list_simple`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L8-L38) -->

Initialize a workspace with one member and list it.

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

```console working-dir="foo"
$ uv workspace list
success: true
exit_code: 0
----- stdout -----
foo

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

With `--paths`:

```console working-dir="foo"
$ uv workspace list --paths
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/foo

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

## Root workspace

<!-- Derived from [`workspace_list::workspace_list_root_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L40-L67) -->

A root workspace has a package at the workspace root plus additional members.

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

```console
$ uv workspace list
success: true
exit_code: 0
----- stdout -----
albatross
bird-feeder
seeds

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

## Virtual workspace

<!-- Derived from [`workspace_list::workspace_list_virtual_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L69-L96) -->

A virtual workspace has no package at the root, only members.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: packages/albatross/pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["bird-feeder"]

[tool.uv.sources]
bird-feeder = { workspace = true }

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

```console
$ uv workspace list
success: true
exit_code: 0
----- stdout -----
albatross
bird-feeder
seeds

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

## List from member

<!-- Derived from [`workspace_list::workspace_list_from_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L98-L127) -->

Running from a workspace member directory still lists all members.

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

```console working-dir="packages/bird-feeder"
$ uv workspace list
success: true
exit_code: 0
----- stdout -----
albatross
bird-feeder
seeds

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

## Multiple members

<!-- Derived from [`workspace_list::workspace_list_multiple_members`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L129-L179) -->

Initialize a workspace with multiple members using `uv init`.

```console
$ uv init pkg-a
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `pkg-a` at `[TEMP_DIR]/pkg-a`
```

```console working-dir="pkg-a"
$ uv init pkg-b
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `pkg-b` as member of workspace `[TEMP_DIR]/pkg-a`
Initialized project `pkg-b` at `[TEMP_DIR]/pkg-a/pkg-b`
```

```console working-dir="pkg-a"
$ uv init pkg-c
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `pkg-c` as member of workspace `[TEMP_DIR]/pkg-a`
Initialized project `pkg-c` at `[TEMP_DIR]/pkg-a/pkg-c`
```

```console working-dir="pkg-a"
$ uv workspace list
success: true
exit_code: 0
----- stdout -----
pkg-a
pkg-b
pkg-c

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

With `--paths`:

```console working-dir="pkg-a"
$ uv workspace list --paths
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/pkg-a
[TEMP_DIR]/pkg-a/pkg-b
[TEMP_DIR]/pkg-a/pkg-c

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

## Single project

<!-- Derived from [`workspace_list::workspace_list_single_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L181-L200) -->

A single project (not a workspace) still lists itself.

```console
$ uv init my-project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `my-project` at `[TEMP_DIR]/my-project`
```

```console working-dir="my-project"
$ uv workspace list
success: true
exit_code: 0
----- stdout -----
my-project

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

## With excluded packages

<!-- Derived from [`workspace_list::workspace_list_with_excluded`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L202-L227) -->

Excluded packages are not listed.

```toml
# file: pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.uv.workspace]
members = ["packages/*"]
exclude = ["excluded/*"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```toml
# file: excluded/bird-feeder/pyproject.toml

[project]
name = "bird-feeder"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = []

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

The `bird-feeder` package in `excluded/` is not listed:

```console
$ uv workspace list
success: true
exit_code: 0
----- stdout -----
albatross
seeds

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
```

## No project

<!-- Derived from [`workspace_list::workspace_list_no_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_list.rs#L229-L244) -->

Running outside a project produces an error.

```console
$ uv workspace list
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: The `uv workspace list` command is experimental and may change without warning. Pass `--preview-features workspace-list` to disable this warning.
error: No `pyproject.toml` found in current directory or any parent directory
```
