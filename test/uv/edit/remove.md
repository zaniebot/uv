# Dependency Management - Removing Dependencies

Tests for removing dependencies from a project using `uv remove`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Remove registry

<!-- Derived from [`edit::remove_registry`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4082-L4178) -->

`uv remove` removes a dependency from the project and updates the lock file.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

Lock the project:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Sync the project:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

Remove the dependency:

```console
$ uv remove anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Uninstalled 3 packages in [TIME]
 - anyio==3.7.0
 - idna==3.6
 - sniffio==1.3.1
```

The dependency is removed from pyproject.toml:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

The lock file only contains the project itself:

```toml title="uv.lock" snapshot=true
version = 1
revision = 3
requires-python = ">=3.12"

[options]
exclude-newer = "2024-03-25T00:00:00Z"

[[package]]
name = "project"
version = "0.1.0"
source = { virtual = "." }
```

Syncing from the lockfile succeeds:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited in [TIME]
```

## Remove repeated

<!-- Derived from [`edit::remove_repeated`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L7514-L7647) -->

When a dependency appears in multiple locations, `uv remove` can remove it from specific groups.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio"]

[project.optional-dependencies]
foo = ["anyio"]

[tool.uv]
dev-dependencies = ["anyio"]

[tool.uv.sources]
anyio = { path = "test/packages/anyio_local" }
```

Removing without flags removes only from production dependencies:

```console
$ uv remove anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==4.3.0+foo (from file://[WORKSPACE]/test/packages/anyio_local)
```

The dependency is removed from production but remains in optional and dev:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
foo = ["anyio"]

[tool.uv]
dev-dependencies = ["anyio"]

[tool.uv.sources]
anyio = { path = "[WORKSPACE]/test/packages/anyio_local" }
```

Removing from the optional group:

```console
$ uv remove anyio --optional foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 2 packages in [TIME]
Audited 1 package in [TIME]
```

The dependency is removed from the optional group:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
foo = []

[tool.uv]
dev-dependencies = ["anyio"]

[tool.uv.sources]
anyio = { path = "[WORKSPACE]/test/packages/anyio_local" }
```

Removing from dev dependencies:

```console
$ uv remove anyio --dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 1 package in [TIME]
Uninstalled 1 package in [TIME]
 - anyio==4.3.0+foo (from file://[WORKSPACE]/test/packages/anyio_local)
```

The dependency is removed from dev dependencies:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
foo = []

[tool.uv]
dev-dependencies = []
```

## Remove requirement

<!-- Derived from [`edit::remove_requirement`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L12293-L12332) -->

`uv remove` can remove dependencies even when specified with extras.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["flask"]
```

Removing with extras specification:

```console
$ uv remove flask[dotenv]
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

The dependency is removed:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

## Remove all with comments

<!-- Derived from [`edit::remove_all_with_comments`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L12336-L12383) -->

When removing all dependencies, comments in the dependencies array are preserved.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "duct",
    "minilog",
    # foo
    # bar
]
```

Removing all dependencies:

```console
$ uv remove duct minilog
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

The dependencies are removed but comments remain:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    # foo
    # bar
]
```
