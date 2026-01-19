# Cache

Tests for cache initialization behavior.

```toml
# mdtest

[environment]
python-version = "3.12"
target-family = "unix"
```

## Cache initialization failure

<!-- Derived from [`cache::cache_init_failure`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache.rs#L11-L70) -->

When the cache directory cannot be created (e.g., due to permissions), uv should show a chained
error message that indicates it failed to initialize the cache.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

Create a read-only directory that will serve as the parent of the cache:

```console
$ mkdir cache_parent
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

```console
$ chmod 000 cache_parent
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Running a command with the cache pointing to a subdirectory within the read-only parent should fail:

```console
$ uv sync --cache-dir cache_parent/cache
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to initialize cache at `cache_parent/cache`
  Caused by: failed to create directory `cache_parent/cache`: Permission denied (os error 13)
```

Restore permissions for cleanup:

```console
$ chmod 755 cache_parent
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```
