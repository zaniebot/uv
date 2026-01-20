# Sync Extras

Tests for extras handling during `uv sync`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Non-existent extras

### Sync with non-existent extra

<!-- from sync.rs::sync_non_existent_extra -->

Requesting a non-existent extra should fail.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[project.optional-dependencies]
types = ["sniffio>1"]
async = ["anyio>3"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --extra baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
error: Extra `baz` is not defined in the project's `optional-dependencies` table
```

### Exclude non-existent extra with all-extras

<!-- from sync.rs::sync_non_existent_extra -->

Excluding a non-existing extra when requesting all extras should fail.

```console
$ uv sync --all-extras --no-extra baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
error: Extra `baz` is not defined in the project's `optional-dependencies` table
```

## Non-existent extra without optional-dependencies

### Sync extra without optional-dependencies table

<!-- from sync.rs::sync_non_existent_extra_no_optional_dependencies -->

Requesting a non-existent extra when no optional-dependencies are defined should fail.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --extra baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
error: Extra `baz` is not defined in the project's `optional-dependencies` table
```

### All-extras with no-extra and no optional-dependencies

<!-- from sync.rs::sync_non_existent_extra_no_optional_dependencies -->

```console
$ uv sync --all-extras --no-extra baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
error: Extra `baz` is not defined in the project's `optional-dependencies` table
```
