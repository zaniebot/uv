# Lock Python Versions

Tests for locking with various Python version requirements.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Upper bounds

### Lock with Python upper bound

<!-- from lock.rs::lock_requires_python_upper -->

Test locking with a Python version upper bound using star syntax.

```toml
# mdtest

[environment]
python-version = "3.11"
exclude-newer = "2024-08-29T00:00:00Z"
```

```toml
# file: pyproject.toml
[project]
name = "warehouse"
version = "1.0.0"
requires-python = "==3.11.*"
dependencies = ["pydantic"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

## Minimum versions

### Lock with Python minimum version

Lock a project with a Python minimum version requirement.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = ["iniconfig"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Range versions

### Lock with Python version range

Lock a project with a Python version range.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.10, <4"
dependencies = ["typing-extensions"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```
