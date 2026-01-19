# Workspace Metadata

Tests for the `uv workspace metadata` command.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Simple workspace

<!-- Derived from [`workspace_metadata::workspace_metadata_simple`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L8-L38) -->

Initialize a workspace with one member and get its metadata.

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

```console working-dir="foo"
$ uv workspace metadata
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "workspace_root": "[TEMP_DIR]/foo",
  "members": [
    {
      "name": "foo",
      "path": "[TEMP_DIR]/foo"
    }
  ]
}

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
```

## Root workspace

<!-- Derived from [`workspace_metadata::workspace_metadata_root_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L40-L84) -->

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
$ uv workspace metadata
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "workspace_root": "[TEMP_DIR]/",
  "members": [
    {
      "name": "albatross",
      "path": "[TEMP_DIR]/"
    },
    {
      "name": "bird-feeder",
      "path": "[TEMP_DIR]/packages/bird-feeder"
    },
    {
      "name": "seeds",
      "path": "[TEMP_DIR]/packages/seeds"
    }
  ]
}

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
```

## Virtual workspace

<!-- Derived from [`workspace_metadata::workspace_metadata_virtual_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L86-L130) -->

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
$ uv workspace metadata
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "workspace_root": "[TEMP_DIR]/",
  "members": [
    {
      "name": "albatross",
      "path": "[TEMP_DIR]/packages/albatross"
    },
    {
      "name": "bird-feeder",
      "path": "[TEMP_DIR]/packages/bird-feeder"
    },
    {
      "name": "seeds",
      "path": "[TEMP_DIR]/packages/seeds"
    }
  ]
}

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
```

## Metadata from member

<!-- Derived from [`workspace_metadata::workspace_metadata_from_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L132-L178) -->

Running from a workspace member directory returns the full workspace metadata.

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
$ uv workspace metadata
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "workspace_root": "[TEMP_DIR]/",
  "members": [
    {
      "name": "albatross",
      "path": "[TEMP_DIR]/"
    },
    {
      "name": "bird-feeder",
      "path": "[TEMP_DIR]/packages/bird-feeder"
    },
    {
      "name": "seeds",
      "path": "[TEMP_DIR]/packages/seeds"
    }
  ]
}

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
```

## Multiple members

<!-- Derived from [`workspace_metadata::workspace_metadata_multiple_members`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L180-L234) -->

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
$ uv workspace metadata
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "workspace_root": "[TEMP_DIR]/pkg-a",
  "members": [
    {
      "name": "pkg-a",
      "path": "[TEMP_DIR]/pkg-a"
    },
    {
      "name": "pkg-b",
      "path": "[TEMP_DIR]/pkg-a/pkg-b"
    },
    {
      "name": "pkg-c",
      "path": "[TEMP_DIR]/pkg-a/pkg-c"
    }
  ]
}

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
```

## Single project

<!-- Derived from [`workspace_metadata::workspace_metadata_single_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L236-L266) -->

A single project (not a workspace) returns metadata with itself as the only member.

```console
$ uv init my-project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `my-project` at `[TEMP_DIR]/my-project`
```

```console working-dir="my-project"
$ uv workspace metadata
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "workspace_root": "[TEMP_DIR]/my-project",
  "members": [
    {
      "name": "my-project",
      "path": "[TEMP_DIR]/my-project"
    }
  ]
}

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
```

## With excluded packages

<!-- Derived from [`workspace_metadata::workspace_metadata_with_excluded`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L268-L304) -->

Excluded packages are not included in the metadata.

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

The `bird-feeder` package in `excluded/` is not included:

```console
$ uv workspace metadata
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "workspace_root": "[TEMP_DIR]/",
  "members": [
    {
      "name": "albatross",
      "path": "[TEMP_DIR]/"
    },
    {
      "name": "seeds",
      "path": "[TEMP_DIR]/packages/seeds"
    }
  ]
}

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
```

## No project

<!-- Derived from [`workspace_metadata::workspace_metadata_no_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace_metadata.rs#L306-L321) -->

Running outside a project produces an error.

```console
$ uv workspace metadata
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: The `uv workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
error: No `pyproject.toml` found in current directory or any parent directory
```
