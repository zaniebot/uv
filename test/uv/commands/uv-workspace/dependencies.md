# Workspace Dependencies

Tests for workspace dependency resolution and conflict handling.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Unsatisfiable member dependency

<!-- Derived from [`workspace::workspace_unsatisfiable_member_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Error message when a workspace member has an unsatisfiable dependency.

```toml
# file: pyproject.toml

[project]
name = "workspace"
version = "0.1.0"
dependencies = []
requires-python = ">=3.12"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["packages/*"]
```

```python
# file: src/__init__.py
pass
```

```toml
# file: packages/leaf/pyproject.toml

[project]
name = "leaf"
version = "0.1.0"
dependencies = ["httpx>9999"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/leaf/src/__init__.py
pass
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because only httpx<=0.27.0 is available and leaf depends on httpx>9999, we can conclude that leaf's requirements are unsatisfiable.
      And because your workspace requires leaf, we can conclude that your workspace's requirements are unsatisfiable.
```

## Conflicting member dependencies

<!-- Derived from [`workspace::workspace_unsatisfiable_member_dependencies_conflicting`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Two workspace members with conflicting dependency pins.

```toml
# file: pyproject.toml

[project]
name = "workspace"
version = "0.1.0"
dependencies = []
requires-python = ">=3.12"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["packages/*"]
```

```python
# file: src/__init__.py
pass
```

```toml
# file: packages/foo/pyproject.toml

[project]
name = "foo"
version = "0.1.0"
dependencies = ["anyio==4.1.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/foo/src/__init__.py
pass
```

```toml
# file: packages/bar/pyproject.toml

[project]
name = "bar"
version = "0.1.0"
dependencies = ["anyio==4.2.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/bar/src/__init__.py
pass
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because bar depends on anyio==4.2.0 and foo depends on anyio==4.1.0, we can conclude that bar and foo are incompatible.
      And because your workspace requires bar and foo, we can conclude that your workspace's requirements are unsatisfiable.
```

## Three-way conflicting dependencies

<!-- Derived from [`workspace::workspace_unsatisfiable_member_dependencies_conflicting_threeway`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Three workspace members with conflicting dependency pins.

```toml
# file: pyproject.toml

[project]
name = "workspace"
version = "0.1.0"
dependencies = []
requires-python = ">=3.12"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["packages/*"]
```

```python
# file: src/__init__.py
pass
```

```toml
# file: packages/red/pyproject.toml

[project]
name = "red"
version = "0.1.0"
dependencies = ["anyio==4.1.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/red/src/__init__.py
pass
```

```toml
# file: packages/knot/pyproject.toml

[project]
name = "knot"
version = "0.1.0"
dependencies = ["anyio==4.2.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/knot/src/__init__.py
pass
```

```toml
# file: packages/bird/pyproject.toml

[project]
name = "bird"
version = "0.1.0"
dependencies = ["anyio==4.3.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/bird/src/__init__.py
pass
```

The first conflict encountered is reported:

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because bird depends on anyio==4.3.0 and knot depends on anyio==4.2.0, we can conclude that bird and knot are incompatible.
      And because your workspace requires bird and knot, we can conclude that your workspace's requirements are unsatisfiable.
```

## Conflicting optional dependency

<!-- Derived from [`workspace::workspace_unsatisfiable_member_dependencies_conflicting_extra`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Conflict between a member's dependency and another member's optional dependency.

```toml
# file: pyproject.toml

[project]
name = "workspace"
version = "0.1.0"
dependencies = []
requires-python = ">=3.12"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["packages/*"]
```

```python
# file: src/__init__.py
pass
```

```toml
# file: packages/foo/pyproject.toml

[project]
name = "foo"
version = "0.1.0"
dependencies = ["anyio==4.1.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/foo/src/__init__.py
pass
```

```toml
# file: packages/bar/pyproject.toml

[project]
name = "bar"
version = "0.1.0"

[project.optional-dependencies]
some_extra = ["anyio==4.2.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/bar/src/__init__.py
pass
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because bar[some-extra] depends on anyio==4.2.0 and foo depends on anyio==4.1.0, we can conclude that foo and bar[some-extra] are incompatible.
      And because your workspace requires bar[some-extra] and foo, we can conclude that your workspace's requirements are unsatisfiable.
```

## Member name shadows external dependency

<!-- Derived from [`workspace::workspace_member_name_shadows_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Error when a workspace member name shadows an external dependency.

```toml
# file: pyproject.toml

[project]
name = "workspace"
version = "0.1.0"
dependencies = []
requires-python = ">=3.12"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["packages/*"]
```

```python
# file: src/__init__.py
pass
```

```toml
# file: packages/foo/pyproject.toml

[project]
name = "foo"
version = "0.1.0"
dependencies = ["anyio==4.1.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/foo/src/__init__.py
pass
```

Create a workspace member named `anyio` that shadows the external package:

```toml
# file: packages/anyio/pyproject.toml

[project]
name = "anyio"
version = "0.1.0"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/anyio/src/__init__.py
pass
```

The member shadows the external dependency but lacks a workspace source:

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `foo @ file://[TEMP_DIR]/packages/foo`
  ├─▶ Failed to parse entry: `anyio`
  ╰─▶ `anyio` is included as a workspace member, but is missing an entry in `tool.uv.sources` (e.g., `anyio = { workspace = true }`)
```

## Cross-workspace path dependencies

<!-- Derived from [`workspace::workspace_to_workspace_paths_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Path dependencies from one workspace into another are resolved correctly.

Main workspace with `a` and `b`:

```toml
# file: main-workspace/pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: main-workspace/packages/a/pyproject.toml

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
# file: main-workspace/packages/a/src/a/__init__.py
pass
```

```toml
# file: main-workspace/packages/b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["c"]

[tool.uv.sources]
c = { path = "../../../other-workspace/packages/c", editable = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: main-workspace/packages/b/src/b/__init__.py
pass
```

Second workspace with `c`, `d`, and `e`:

```toml
# file: other-workspace/pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: other-workspace/packages/c/pyproject.toml

[project]
name = "c"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["d"]

[tool.uv.sources]
d = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: other-workspace/packages/c/src/c/__init__.py
pass
```

```toml
# file: other-workspace/packages/d/pyproject.toml

[project]
name = "d"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: other-workspace/packages/d/src/d/__init__.py
pass
```

```toml
# file: other-workspace/packages/e/pyproject.toml

[project]
name = "e"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["numpy>=2.0.0,<3"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: other-workspace/packages/e/src/e/__init__.py
pass
```

Lock the main workspace - only `a`, `b`, `c`, `d` are included, not `e`:

```console working-dir="main-workspace"
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 4 packages in [TIME]
```

## Conflicting dev dependencies

<!-- Derived from [`workspace::workspace_unsatisfiable_member_dependencies_conflicting_dev`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs#L1595-L1660) -->

When workspace members have conflicting dev dependencies, the error message identifies the conflict.

```toml
# file: workspace/pyproject.toml

[project]
name = "workspace"
version = "0.1.0"
dependencies = []
requires-python = ">=3.12"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["packages/*"]
```

```python
# file: workspace/src/__init__.py
pass
```

Create foo with anyio==4.1.0:

```toml
# file: workspace/packages/foo/pyproject.toml

[project]
name = "foo"
version = "0.1.0"
dependencies = ["anyio==4.1.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: workspace/packages/foo/src/__init__.py
pass
```

Create bar with conflicting dev dependency anyio==4.2.0:

```toml
# file: workspace/packages/bar/pyproject.toml

[project]
name = "bar"
version = "0.1.0"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv]
dev-dependencies = ["anyio==4.2.0"]
```

```python
# file: workspace/packages/bar/src/__init__.py
pass
```

```console
$ cd workspace

$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `packages/bar/pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
  × No solution found when resolving dependencies:
  ╰─▶ Because bar:dev depends on anyio==4.2.0 and foo depends on anyio==4.1.0, we can conclude that foo and bar:dev are incompatible.
      And because your workspace requires bar:dev and foo, we can conclude that your workspace's requirements are unsatisfiable.
```

## Transitive dependencies in git workspace without root

<!-- Derived from [`workspace::transitive_dep_in_git_workspace_no_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs#L1796-L1864) -->

When a package depends on a member of a git workspace (virtual workspace), transitive workspace
dependencies are correctly resolved as git dependencies with subdirectories.

```toml
# mdtest
[environment]
required-features = "git"
```

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["c"]

[tool.uv.sources]
c = { git = "https://github.com/astral-sh/workspace-virtual-root-test", subdirectory = "packages/c", rev = "fac39c8d4c5d0ef32744e2bb309bbe34a759fd46" }
```

Locking resolves both c and its workspace dependency d:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
```

Both c and d are git dependencies with subdirectories:

```console
$ grep -A 2 "name = \"[cd]\"" uv.lock | head -6
success: true
exit_code: 0
----- stdout -----
name = "c"
version = "0.1.0"
source = { git = "https://github.com/astral-sh/workspace-virtual-root-test?subdirectory=packages%2Fc&rev=fac39c8d4c5d0ef32744e2bb309bbe34a759fd46#fac39c8d4c5d0ef32744e2bb309bbe34a759fd46" }
--
name = "d"
version = "0.1.0"
source = { git = "https://github.com/astral-sh/workspace-virtual-root-test?subdirectory=packages%2Fd&rev=fac39c8d4c5d0ef32744e2bb309bbe34a759fd46#fac39c8d4c5d0ef32744e2bb309bbe34a759fd46" }

----- stderr -----
```

## Transitive dependencies in git workspace with root

<!-- Derived from [`workspace::transitive_dep_in_git_workspace_with_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs#L1871-L1933) -->

When a package depends on a subdirectory member of a git workspace (root workspace), transitive
workspace dependencies at the root are resolved correctly without subdirectories.

```toml
# mdtest
[environment]
required-features = "git"
```

```toml
# file: pyproject.toml

[project]
name = "git-with-root"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "workspace-member-in-subdir",
]

[tool.uv.sources]
workspace-member-in-subdir = { git = "https://github.com/astral-sh/workspace-in-root-test", subdirectory = "workspace-member-in-subdir", rev = "d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

The member has a subdirectory, but the root workspace dependency does not:

```console
$ grep -A 2 "name = \".*workspace" uv.lock
success: true
exit_code: 0
----- stdout -----
name = "uv-git-workspace-in-root"
version = "0.1.0"
source = { git = "https://github.com/astral-sh/workspace-in-root-test?rev=d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68#d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68" }
--
name = "workspace-member-in-subdir"
version = "0.1.0"
source = { git = "https://github.com/astral-sh/workspace-in-root-test?subdirectory=workspace-member-in-subdir&rev=d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68#d3ab48d2338296d47e28dbb2fb327c5e2ac4ac68" }

----- stderr -----
```
