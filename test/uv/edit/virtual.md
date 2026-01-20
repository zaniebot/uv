# Dependency Management - Virtual Projects

Tests for adding and removing dependencies in virtual projects (projects without a `[project]`
table).

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Add non-project

<!-- Derived from [`edit::add_non_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4933-L5028) -->

Virtual workspace roots can only have dev dependencies, not production or optional dependencies.

```toml
# file: pyproject.toml
[tool.uv.workspace]
members = []
```

Adding a production dependency fails:

```console
$ uv add iniconfig
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Project is missing a `[project]` table; add a `[project]` table to use production dependencies, or run `uv add --dev` instead
```

Adding an optional dependency also fails:

```console
$ uv add iniconfig --optional async
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Project is missing a `[project]` table; add a `[project]` table to use optional dependencies, or run `uv add --dev` instead
```

Adding a dev dependency succeeds:

```console
$ uv add iniconfig --dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

The dev dependency is added to `[dependency-groups]`:

```toml title="pyproject.toml" snapshot=true
[tool.uv.workspace]
members = []

[dependency-groups]
dev = [
    "iniconfig>=2.0.0",
]
```

The lock file contains the dev dependency:

```toml title="uv.lock" snapshot=true
version = 1
revision = 3
requires-python = ">=3.12"

[options]
exclude-newer = "2024-03-25T00:00:00Z"

[manifest]

[manifest.dependency-groups]
dev = [{ name = "iniconfig", specifier = ">=2.0.0" }]

[[package]]
name = "iniconfig"
version = "2.0.0"
source = { registry = "https://pypi.org/simple" }
sdist = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3", size = 4646, upload-time = "2023-01-07T11:08:11.254Z" }
wheels = [
    { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl", hash = "sha256:b6a85871a79d2e3b22d2d1b94ac2824226a63c6b741c88f7ae975f18b6778374", size = 5892, upload-time = "2023-01-07T11:08:09.864Z" },
]
```

## Add virtual empty

<!-- Derived from [`edit::add_virtual_empty`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L5031-L5100) -->

When a pyproject.toml has no `[project]` table, production dependencies cannot be added, but
dependency groups can.

```toml
# file: pyproject.toml
[tool.mycooltool]
wow = "someconfig"
```

Adding a production dependency fails:

```console
$ uv add sortedcontainers
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Project is missing a `[project]` table; add a `[project]` table to use production dependencies, or run `uv add --dev` instead
```

The file is unchanged:

```toml title="pyproject.toml" snapshot=true
[tool.mycooltool]
wow = "someconfig"
```

Adding to a dependency group succeeds:

```console
$ uv add sortedcontainers --group dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + sortedcontainers==2.4.0
```

The dependency is added to the group:

```toml title="pyproject.toml" snapshot=true
[tool.mycooltool]
wow = "someconfig"

[dependency-groups]
dev = [
    "sortedcontainers>=2.4.0",
]
```

## Add virtual dependency group

<!-- Derived from [`edit::add_virtual_dependency_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L5103-L5186) -->

Dependency groups work in virtual projects without a `[project]` table.

```toml
# file: pyproject.toml
[dependency-groups]
foo = ["sortedcontainers"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

Adding to an existing group:

```console
$ uv add sortedcontainers --group dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 3 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + sniffio==1.3.1
 + sortedcontainers==2.4.0
```

The dependency is added to the dev group:

```toml title="pyproject.toml" snapshot=true
[dependency-groups]
foo = ["sortedcontainers"]
bar = ["iniconfig"]
dev = [
    "sniffio",
    "sortedcontainers>=2.4.0",
]
```

Adding to a new group creates the group:

```console
$ uv add sortedcontainers --group baz
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 3 packages in [TIME]
Audited 2 packages in [TIME]
```

The new group is created:

```toml title="pyproject.toml" snapshot=true
[dependency-groups]
foo = ["sortedcontainers"]
bar = ["iniconfig"]
dev = [
    "sniffio",
    "sortedcontainers>=2.4.0",
]
baz = [
    "sortedcontainers>=2.4.0",
]
```

## Remove virtual empty

<!-- Derived from [`edit::remove_virtual_empty`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L5296-L5361) -->

Removing dependencies from a virtual project without any dependencies fails.

```toml
# file: pyproject.toml

[tool.mycooltool]
wow = "someconfig"
```

Removing a production dependency fails:

```console
$ uv remove sortedcontainers
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The dependency `sortedcontainers` could not be found in `project.dependencies`
```

The file is unchanged:

```toml title="pyproject.toml" snapshot=true

[tool.mycooltool]
wow = "someconfig"
```

Removing from a dependency group also fails:

```console
$ uv remove sortedcontainers --group dev
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The dependency `sortedcontainers` could not be found in `tool.uv.dev-dependencies` or `tool.uv.dependency-groups.dev`
```

The file remains unchanged:

```toml title="pyproject.toml" snapshot=true

[tool.mycooltool]
wow = "someconfig"
```

## Remove virtual dependency group

<!-- Derived from [`edit::remove_virtual_dependency_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L5364-L5436) -->

Removing dependencies from dependency groups works in virtual projects.

```toml
# file: pyproject.toml
[dependency-groups]
foo = ["sortedcontainers"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

Removing from a group:

```console
$ uv remove sortedcontainers --group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + sniffio==1.3.1
```

The dependency is removed, leaving an empty group:

```toml title="pyproject.toml" snapshot=true
[dependency-groups]
foo = []
bar = ["iniconfig"]
dev = ["sniffio"]
```

Removing from a non-existent group fails:

```console
$ uv remove sortedcontainers --group baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The dependency `sortedcontainers` could not be found in `dependency-groups.baz`
```

The file is unchanged:

```toml title="pyproject.toml" snapshot=true
[dependency-groups]
foo = []
bar = ["iniconfig"]
dev = ["sniffio"]
```
