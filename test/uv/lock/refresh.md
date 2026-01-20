# Lock Refresh

Tests for `uv lock --refresh` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Basic refresh

### Refresh all packages

<!-- from lock.rs::lock_refresh -->

The `--refresh` flag re-fetches all package metadata.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Refresh all packages.

```console
$ uv lock --refresh
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

### Refresh specific package

<!-- from lock.rs::lock_refresh -->

The `--refresh-package` flag re-fetches metadata for a specific package.

```console
$ uv lock --refresh-package anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```
