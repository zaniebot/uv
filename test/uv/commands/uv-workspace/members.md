# Workspace Members

Tests for workspace member discovery and handling.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Empty member directory

<!-- Derived from [`workspace::workspace_empty_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

A workspace member directory without a `pyproject.toml` produces an error.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: packages/a/pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["b"]

[tool.uv.sources]
b = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/a/src/a/__init__.py
pass
```

```toml
# file: packages/b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/b/src/b/__init__.py
pass
```

Create an empty directory for member `c`:

```tree create=true
packages/c/
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Workspace member `[TEMP_DIR]/packages/c` is missing a `pyproject.toml` (matches: `packages/*`)
```

## Hidden directories are ignored

<!-- Derived from [`workspace::workspace_hidden_files`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Workspace discovery ignores hidden directories (starting with `.`).

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: packages/a/pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["b"]

[tool.uv.sources]
b = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/a/src/a/__init__.py
pass
```

```toml
# file: packages/b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/b/src/b/__init__.py
pass
```

Create a hidden directory `.c`:

```tree create=true
packages/
└── .c/
```

The hidden directory is ignored:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Hidden member with valid pyproject.toml

<!-- Derived from [`workspace::workspace_hidden_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

A hidden directory with a valid `pyproject.toml` that is a dependency is accepted.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: packages/a/pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["b"]

[tool.uv.sources]
b = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/a/src/a/__init__.py
pass
```

```toml
# file: packages/b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["c"]

[tool.uv.sources]
c = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/b/src/b/__init__.py
pass
```

```toml
# file: packages/.c/pyproject.toml

[project]
name = "c"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/.c/src/c/__init__.py
pass
```

The hidden directory `.c` is discovered because it's a dependency:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Non-included project is independent

<!-- Derived from [`workspace::workspace_non_included_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

A project that isn't matched by the workspace `members` glob is treated as independent.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: packages/a/pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["b"]

[tool.uv.sources]
b = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/a/src/a/__init__.py
pass
```

```toml
# file: packages/b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/b/src/b/__init__.py
pass
```

Create project `c` outside the `packages/` directory:

```toml
# file: c/pyproject.toml

[project]
name = "c"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: c/src/c/__init__.py
pass
```

Locking from `c` should not include any workspace members:

```console working-dir="c"
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 1 package in [TIME]
```

## Member with leading dot-slash

<!-- Derived from [`workspace::workspace_members_with_leading_dot_slash`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Workspace members can use `./` prefix.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["./packages/*"]
```

```toml
# file: packages/a/pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/a/src/a/__init__.py
pass
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
```

## Member with parent directory reference

<!-- Derived from [`workspace::workspace_members_with_parent_directory`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs#L2006-L2048) -->

Workspaces can reference members outside their directory using `../` paths.

```toml
# file: workspace/pyproject.toml

[tool.uv.workspace]
members = ["../external-package"]
```

```toml
# file: external-package/pyproject.toml

[project]
name = "external-package"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ cd workspace

$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 1 package in [TIME]
```

The lock file correctly references the external member:

```console
$ cat uv.lock
success: true
exit_code: 0
----- stdout -----
version = 1
requires-python = ">=3.12"

[[package]]
name = "external-package"
version = "0.1.0"
source = { editable = "../external-package" }

[package.metadata]
requires-dist = []

----- stderr -----
```

## Member with complex relative paths

<!-- Derived from [`workspace::workspace_members_with_complex_relative_paths`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs#L2051-L2093) -->

Workspaces normalize complex relative paths like `./subdir/../../sibling-package`.

```toml
# file: workspace/pyproject.toml

[tool.uv.workspace]
members = ["./subdir/../../sibling-package"]
```

```toml
# file: sibling-package/pyproject.toml

[project]
name = "sibling-package"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ cd workspace

$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 1 package in [TIME]
```

The complex path is normalized to `../sibling-package`:

```console
$ cat uv.lock
success: true
exit_code: 0
----- stdout -----
version = 1
requires-python = ">=3.12"

[[package]]
name = "sibling-package"
version = "0.1.0"
source = { editable = "../sibling-package" }

[package.metadata]
requires-dist = []

----- stderr -----
```
