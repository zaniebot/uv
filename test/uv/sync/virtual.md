# Sync Virtual Workspaces

Tests for `uv sync` with virtual workspaces (no `[project]` section).

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Empty virtual workspace

### Sync with minimal pyproject.toml

<!-- from sync.rs::virtual_empty -->

A pyproject.toml with no `[project]` section and nothing useful for uv syncs without error.

```toml
# file: pyproject.toml
[tool.mycooltool]
wow = "someconfig"
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved in [TIME]
Audited in [TIME]
```

## Virtual with dependency groups

### Sync default groups in virtual workspace

<!-- from sync.rs::virtual_dependency_group -->

A virtual workspace with dependency groups syncs the dev group by default.

```toml
# file: pyproject.toml
[dependency-groups]
foo = ["sortedcontainers"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + sniffio==1.3.1
```

### Sync specific group in virtual workspace

<!-- from sync.rs::virtual_dependency_group -->

Using `--group` syncs a specific group.

```console
$ uv sync --group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + sortedcontainers==2.4.0
```

### Sync all groups in virtual workspace

<!-- from sync.rs::virtual_dependency_group -->

Using `--all-groups` syncs all dependency groups.

```console
$ uv sync --all-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```
