# Project Initialization - Existing Environments

Tests for initializing projects in existing environments and workspaces.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Existing environment

<!-- Derived from [`init::init_existing_environment`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2554-L2601) -->

`uv init` infers the Python version from an existing .venv.

```toml
# mdtest

[environment]
python-versions = ["3.9", "3.12"]
```

Create a virtual environment:

```console working-dir="foo"
$ uv venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Initialize the project:

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The requires-python is inferred from the virtual environment:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Existing environment parent

<!-- Derived from [`init::init_existing_environment_parent`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2603-L2649) -->

`uv init` ignores Python version from a parent .venv.

```toml
# mdtest

[environment]
python-versions = ["3.9", "3.12"]
```

Create a virtual environment in the parent directory:

```console
$ uv venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Initialize a project in a subdirectory:

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The requires-python uses the default version (not from parent .venv):

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.9"
dependencies = []
```

## Project inside project

<!-- Derived from [`init::init_project_inside_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1956-L2030) -->

Nested projects are added to the root workspace.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

Create a child project:

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

Create a grandchild project from within the child:

```console working-dir="foo"
$ uv init bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `bar` as member of workspace `[TEMP_DIR]/`
Initialized project `bar` at `[TEMP_DIR]/foo/bar`
```

Both are added to the root workspace:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = [
    "foo",
    "foo/bar",
]
```

The child project is created:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Virtual project

<!-- Derived from [`init::init_virtual_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2083-L2152) -->

The `--virtual` flag creates a virtual project that can become a workspace.

```console working-dir="foo"
$ uv init --virtual
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

A regular project is created:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

When a child project is added, the virtual project becomes a workspace:

```console working-dir="foo"
$ uv init bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `bar` as member of workspace `[TEMP_DIR]/foo`
Initialized project `bar` at `[TEMP_DIR]/foo/bar`
```

The workspace configuration is added:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[tool.uv.workspace]
members = [
    "bar",
]
```

## Matches members

<!-- Derived from [`init::init_matches_members`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2257-L2297) -->

When a project matches a workspace members pattern, it's not added again.

```toml
# file: pyproject.toml
[tool.uv.workspace]
members = ['packages/*']
```

```console working-dir="packages/foo"
$ uv init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Project `foo` is already a member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/packages/foo`
```

The workspace configuration is unchanged:

```toml title="pyproject.toml" snapshot=true
[tool.uv.workspace]
members = ['packages/*']
```

## Matches exclude

<!-- Derived from [`init::init_matches_exclude`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2299-L2340) -->

When a project matches a workspace exclude pattern, it's not added.

```toml
# file: pyproject.toml
[tool.uv.workspace]
exclude = ['packages/foo']
members = ['packages/*']
```

```console working-dir="packages/foo"
$ uv init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Project `foo` is excluded by workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/packages/foo`
```

The workspace configuration is unchanged:

```toml title="pyproject.toml" snapshot=true
[tool.uv.workspace]
exclude = ['packages/foo']
members = ['packages/*']
```

## Working directory change

<!-- Derived from [`init::init_working_directory_change`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L4160-L4183) -->

The `--directory` flag changes the base directory for project initialization.

```console
$ uv init --directory bar foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/bar/foo`
```

The project is created in the specified directory:

```console
$ test -f bar/foo/pyproject.toml && echo "exists"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```
