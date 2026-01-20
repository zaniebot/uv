# Lock Cycles

Tests for locking packages with cyclic dependencies.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Dependency cycles

### Lock packages with cycles

<!-- from lock.rs::lock_cycles -->

Lock packages that have cyclic dependencies (testtools and fixtures depend on each other).

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["testtools==2.3.0", "fixtures==3.0.0"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 11 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 11 packages in [TIME]
```
