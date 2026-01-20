# Lock Source Distributions

Tests for locking packages from source distributions.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Registry source distribution

### Lock sdist from registry

<!-- from lock.rs::lock_sdist_registry -->

Lock a dependency that only has source distributions on PyPI.

```toml
# mdtest

[environment]
exclude-newer = "2025-01-29T00:00:00Z"
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["source-distribution==0.0.1"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + source-distribution==0.0.1
```

Re-install to verify audit.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 1 package in [TIME]
```
