# Workspace Editing

Tests for editing dependencies in workspaces using `uv add` and `uv remove`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Add and remove workspace dependency

<!-- Derived from [`edit::add_remove_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a workspace package as a dependency automatically sets up the workspace source.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["child1", "child2"]
```

```toml
# file: child1/pyproject.toml

[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child1/src/child1/__init__.py
pass
```

```toml
# file: child2/pyproject.toml

[project]
name = "child2"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child2/src/child2/__init__.py
pass
```

Workspace packages should be detected automatically:

```console
$ uv add child2 --package child1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + child1==0.1.0 (from file://[TEMP_DIR]/child1)
 + child2==0.1.0 (from file://[TEMP_DIR]/child2)
```

The pyproject.toml should have the workspace source:

```toml title="child1/pyproject.toml" snapshot=true
[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "child2",
]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.uv.sources]
child2 = { workspace = true }
```

Remove the dependency:

```console working-dir="child1"
$ uv remove child2
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Uninstalled 2 packages in [TIME]
Installed 1 package in [TIME]
 ~ child1==0.1.0 (from file://[TEMP_DIR]/child1)
 - child2==0.1.0 (from file://[TEMP_DIR]/child2)
```

The dependency and source should be removed:

```toml title="child1/pyproject.toml" snapshot=true
[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

## Add workspace editable

<!-- Derived from [`edit::add_workspace_editable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding `--editable` to a workspace dependency works since workspace packages are always editable.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["child1", "child2"]
```

```toml
# file: child1/pyproject.toml

[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child1/src/child1/__init__.py
pass
```

```toml
# file: child2/pyproject.toml

[project]
name = "child2"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child2/src/child2/__init__.py
pass
```

```console
$ uv add child2 --editable --package child1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + child1==0.1.0 (from file://[TEMP_DIR]/child1)
 + child2==0.1.0 (from file://[TEMP_DIR]/child2)
```

The source should have workspace and editable set:

```toml title="child1/pyproject.toml" snapshot=true
[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "child2",
]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.uv.sources]
child2 = { workspace = true, editable = true }
```

## Add workspace path dependency

<!-- Derived from [`edit::add_workspace_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2638-L2757) -->

Adding a path dependency within a workspace automatically adds it as a workspace member.

```toml
# file: pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.uv.workspace]
members = ["child"]
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child/src/child/__init__.py
pass
```

```console
$ uv add ./child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + child==0.1.0 (from file://[TEMP_DIR]/child)
```

The dependency is added with workspace source:

```toml title="pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "child",
]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

## Add path with implicit workspace creation

<!-- Derived from [`edit::add_path_implicit_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2764-L2890) -->

Adding a path dependency implicitly creates a workspace if one doesn't exist.

Create a project without a workspace:

```toml
# file: workspace/pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

Create a child package:

```toml
# file: workspace/packages/child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: workspace/packages/child/src/child/__init__.py
pass
```

```console
$ uv add packages/child
working-dir: workspace
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Added `packages/child` to workspace members
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + child==0.1.0 (from file://[TEMP_DIR]/workspace/packages/child)
```

A workspace is automatically created:

```toml title="workspace/pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "child",
]

[tool.uv.workspace]
members = [
    "packages/child",
]

[tool.uv.sources]
child = { workspace = true }
```

## Add path with --no-workspace flag

<!-- Derived from [`edit::add_path_no_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2892-L3004) -->

Using `--no-workspace` adds a path as a direct path dependency instead of workspace member.

```toml
# file: workspace/pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```toml
# file: workspace/packages/child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: workspace/packages/child/src/child/__init__.py
pass
```

```console
$ uv add packages/child --no-workspace
working-dir: workspace
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + child==0.1.0 (from file://[TEMP_DIR]/workspace/packages/child)
```

The dependency is added as a direct path, not a workspace member:

```toml title="workspace/pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "child",
]

[tool.uv.sources]
child = { path = "packages/child" }
```

## Failed add reverts workspace changes at root

<!-- Derived from [`edit::fail_to_add_revert_workspace_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L8716-L8824) -->

When `uv add` fails during installation, workspace membership changes are reverted.

```toml
# file: pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

Create a valid child package:

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"
```

Create a broken package that will fail to build:

```toml
# file: broken/pyproject.toml

[project]
name = "broken"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"
```

```python
# file: broken/setup.py
1/0
```

```console
$ uv add ./broken
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Added `broken` to workspace members
Resolved 3 packages in [TIME]
  × Failed to build `broken @ file://[TEMP_DIR]/broken`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `setuptools.build_meta.build_editable` failed (exit status: 1)

      [stderr]
      Traceback (most recent call last):
        ...
      ZeroDivisionError: division by zero

      hint: This usually indicates a problem with the package or the build environment.
  help: If you want to add the package regardless of the failed resolution, provide the `--frozen` flag to skip locking and syncing.
```

The workspace should be reverted to its original state:

```toml title="pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

## Failed add reverts workspace changes at member

<!-- Derived from [`edit::fail_to_add_revert_workspace_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L8826-L8938) -->

When `uv add` fails from a workspace member, workspace membership changes are reverted.

```toml
# file: pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child/src/child/__init__.py
pass
```

Create a broken package:

```toml
# file: broken/pyproject.toml

[project]
name = "broken"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"
```

```python
# file: broken/setup.py
1/0
```

```console
$ uv add ../broken
working-dir: child
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Added `broken` to workspace members
Resolved 4 packages in [TIME]
  × Failed to build `broken @ file://[TEMP_DIR]/broken`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `setuptools.build_meta.build_editable` failed (exit status: 1)

      [stderr]
      Traceback (most recent call last):
        ...
      ZeroDivisionError: division by zero

      hint: This usually indicates a problem with the package or the build environment.
  help: If you want to add the package regardless of the failed resolution, provide the `--frozen` flag to skip locking and syncing.
```

The workspace remains unchanged:

```toml title="pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

## Add path with existing workspace

<!-- Derived from [`edit::add_path_with_existing_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L14305-L14399) -->

Adding a path dependency from within a workspace member automatically adds it to the workspace.

```toml
# file: pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = ["project"]
```

```toml
# file: project/pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```toml
# file: dep/pyproject.toml

[project]
name = "dep"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add ../dep
working-dir: project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Added `dep` to workspace members
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + dep==0.1.0 (from file://[TEMP_DIR]/dep)
```

The workspace is updated to include the new member:

```toml title="pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = [
    "project",
    "dep",
]
```

The project has the workspace dependency:

```toml title="project/pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "dep",
]

[tool.uv.sources]
dep = { workspace = true }
```

## Add path with --workspace flag

<!-- Derived from [`edit::add_path_with_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L14401-L14467) -->

Using `--workspace` explicitly adds a path as a workspace member.

```toml
# file: pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
```

```toml
# file: dep/pyproject.toml

[project]
name = "dep"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add ./dep --workspace
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Added `dep` to workspace members
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + dep==0.1.0 (from file://[TEMP_DIR]/dep)
```

The workspace is created with the member:

```toml title="pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "dep",
]

[tool.uv.workspace]
members = [
    "dep",
]

[tool.uv.sources]
dep = { workspace = true }
```

## Add path within workspace defaults to workspace

<!-- Derived from [`edit::add_path_within_workspace_defaults_to_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L14469-L14535) -->

Paths within an existing workspace directory are automatically added as workspace members.

```toml
# file: pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.uv.workspace]
members = []
```

```toml
# file: dep/pyproject.toml

[project]
name = "dep"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add ./dep
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Added `dep` to workspace members
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + dep==0.1.0 (from file://[TEMP_DIR]/dep)
```

The path is added as a workspace member:

```toml title="pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "dep",
]

[tool.uv.workspace]
members = [
    "dep",
]

[tool.uv.sources]
dep = { workspace = true }
```

## Add path with explicit --no-workspace

<!-- Derived from [`edit::add_path_with_no_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L14537-L14600) -->

Using `--no-workspace` explicitly prevents adding path as workspace member.

```toml
# file: pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.uv.workspace]
members = []
```

```toml
# file: dep/pyproject.toml

[project]
name = "dep"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add ./dep --no-workspace
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + dep==0.1.0 (from file://[TEMP_DIR]/dep)
```

The path is added as a direct dependency:

```toml title="pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "dep",
]

[tool.uv.workspace]
members = []

[tool.uv.sources]
dep = { path = "dep" }
```

## Add path outside workspace defaults to path

<!-- Derived from [`edit::add_path_outside_workspace_no_default`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L14602-L14673) -->

Paths outside the workspace directory are added as direct path dependencies by default.

```toml
# file: workspace/pyproject.toml

[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.uv.workspace]
members = []
```

Create a dependency outside the workspace:

```toml
# file: external_dep/pyproject.toml

[project]
name = "dep"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add ../external_dep
working-dir: workspace
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + dep==0.1.0 (from file://[TEMP_DIR]/external_dep)
```

The path is added as a direct dependency, not a workspace member:

```toml title="workspace/pyproject.toml" snapshot=true
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "dep",
]

[tool.uv.workspace]
members = []

[tool.uv.sources]
dep = { path = "../external_dep" }
```
