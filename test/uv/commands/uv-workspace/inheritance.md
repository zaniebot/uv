# Workspace Inheritance

Tests for configuration inheritance from workspace root to members.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Members inherit sources from root

<!-- Derived from [`workspace::workspace_inherit_sources`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Workspace members inherit sources from the root when not specified in the member.

Create the workspace root:

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

Create a leaf package that depends on `library`:

```toml
# file: packages/leaf/pyproject.toml

[project]
name = "leaf"
version = "0.1.0"
dependencies = ["library"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: packages/leaf/src/__init__.py
pass
```

Create a peripheral library (outside the workspace):

```toml
# file: ../library/pyproject.toml

[project]
name = "library"
version = "0.1.0"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: ../library/src/__init__.py
pass
```

Without a source, resolving fails:

```console
$ uv lock --offline
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because library was not found in the cache and leaf depends on library, we can conclude that leaf's requirements are unsatisfiable.
      And because your workspace requires leaf, we can conclude that your workspace's requirements are unsatisfiable.

      hint: Packages were unavailable because the network was disabled. When the network is disabled, registry packages may only be read from the cache.
```

Add the source to the leaf member:

```toml
# file: packages/leaf/pyproject.toml

[project]
name = "leaf"
version = "0.1.0"
dependencies = ["library"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.sources]
library = { path = "../../../library", editable = true }
```

Now resolving succeeds:

```console
$ uv lock --offline
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

Alternatively, the source can be defined at the workspace root and inherited by members:

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

[tool.uv.sources]
library = { path = "../library", editable = true }

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: packages/leaf/pyproject.toml

[project]
name = "leaf"
version = "0.1.0"
dependencies = ["library"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Resolving succeeds with the inherited source:

```console
$ uv lock --offline
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Path hopping

<!-- Derived from [`workspace::test_path_hopping`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs#L1738-L1789) -->

Path dependencies are correctly resolved even when they reference paths relative to intermediate
projects (transitive path dependencies).

Create a project that depends on foo:

```toml
# file: project/pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["foo"]

[tool.uv.sources]
foo = { path = "../libs/foo", editable = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

foo depends on bar with a relative path from its own location:

```toml
# file: libs/foo/pyproject.toml

[project]
name = "foo"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["bar"]

[tool.uv.sources]
bar = { path = "../../libs/bar", editable = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

bar is a simple package:

```toml
# file: libs/bar/pyproject.toml

[project]
name = "bar"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ cd project

$ uv lock --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 3 packages in [TIME]
```

All paths in the lockfile are correctly resolved relative to the project root:

```console
$ cat uv.lock
success: true
exit_code: 0
----- stdout -----
version = 1
requires-python = ">=3.12"

[[package]]
name = "bar"
version = "0.1.0"
source = { editable = "../libs/bar" }

[package.metadata]
requires-dist = []

[[package]]
name = "foo"
version = "0.1.0"
source = { editable = "../libs/foo" }
dependencies = [
    { name = "bar" },
]

[package.metadata]
requires-dist = [{ name = "bar", editable = "../../libs/bar" }]

[[package]]
name = "project"
version = "0.1.0"
source = { editable = "." }
dependencies = [
    { name = "foo" },
]

[package.metadata]
requires-dist = [{ name = "foo", editable = "../libs/foo" }]

----- stderr -----
```
