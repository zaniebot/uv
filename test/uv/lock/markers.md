# Lock Markers

Tests for locking with markers and conditional dependencies.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Conditional dependencies

### Conditional and unconditional dependency

<!-- from lock.rs::lock_conditional_unconditional -->

Lock a package that's included both conditionally and unconditionally.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig", "iniconfig ; python_version < '3.12'"]
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

### Multiple markers for same dependency

<!-- from lock.rs::lock_multiple_markers -->

Lock a package that's included twice with different markers.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "iniconfig ; sys_platform == 'darwin'",
    "iniconfig ; sys_platform == 'linux'"
]
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
