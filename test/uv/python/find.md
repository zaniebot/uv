# Python Management - Find

Tests for `uv python find` command.

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
```

## Basic

<!-- Derived from [`python_find::python_find`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L12-L151) -->

The `uv python find` command locates Python interpreters.

No interpreters available fails:

```console
$ UV_TEST_PYTHON_PATH= uv python find
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found in [PYTHON SOURCES]
```

Find first interpreter on path:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

Request Python 3.12:

```console
$ uv python find 3.12
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Request Python 3.12 with version specifier:

```console
$ uv python find ==3.12.*
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Request Python 3.11:

```console
$ uv python find 3.11
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

Request CPython (any version):

```console
$ uv python find cpython
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

Request CPython 3.12:

```console
$ uv python find cpython@3.12
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Request CPython 3.12 via partial key syntax:

```console
$ uv python find cpython-3.12
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Request Python 3.12 with placeholders:

```console
$ uv python find any-3.12-any
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Request PyPy (not available):

```console
$ uv python find pypy
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for PyPy in [PYTHON SOURCES]
```

## Find pin

<!-- Derived from [`python_find::python_find_pin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L154-L228) -->

Pinned versions take precedence over the first interpreter on the path.

Pin to a version:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12`

----- stderr -----
```

Find respects the pinned version:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Unless explicitly requested:

```console
$ uv python find 3.11
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

Or --no-config is used:

```console
$ uv python find --no-config
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

Create a child directory:

```console
$ mkdir child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Pinned versions are found in parent directories:

```console
$ cd child && uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Pin in child directory:

```console
$ cd child && uv python pin 3.11
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.11`

----- stderr -----
```

Child pin takes precedence:

```console
$ cd child && uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

## Find pin arbitrary name

<!-- Derived from [`python_find::python_find_pin_arbitrary_name`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L231-L304) -->

Arbitrary names in version files are not supported.

Try to pin to arbitrary name:

```console
$ uv python pin foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Requests for arbitrary names (e.g., `foo`) are not supported in version files
```

Create arbitrary name pin bypassing uv:

```text
# file: .python-version
foo
```

Arbitrary names are ignored with warning:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
warning: Ignoring unsupported Python request `foo` in version file: [TEMP_DIR]/.python-version
```

Pin is updatable:

```console
$ uv python pin 3.11
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.11`

----- stderr -----
warning: Ignoring unsupported Python request `foo` in version file: [TEMP_DIR]/.python-version
```

No warnings afterwards:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `3.11` -> `3.12`

----- stderr -----
```

Create subdirectory with arbitrary pin:

```text
# file: foo/.python-version
foo
```

Arbitrary name ignored in subdirectory:

```console
$ cd foo && uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
warning: Ignoring unsupported Python request `foo` in version file: [TEMP_DIR]/foo/.python-version
```

## Find project

<!-- Derived from [`python_find::python_find_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L307-L415) -->

```toml
# mdtest

[environment]
python-versions = ["3.10", "3.11", "3.12"]
```

Projects with `requires-python` constraints affect Python discovery.

Create a project:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = ["anyio==3.7.0"]
```

Find respects project's required version:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

Unless explicitly requested:

```console
$ uv python find 3.10
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.10]

----- stderr -----
warning: The requested interpreter resolved to Python 3.10.[X], which is incompatible with the project's Python requirement: `>=3.11` (from `project.requires-python`)
```

Or --no-project is used:

```console
$ uv python find --no-project
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.10]

----- stderr -----
```

Pin takes precedence over project:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12`

----- stderr -----
```

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Create incompatible pin:

```console
$ uv python pin 3.10 --no-workspace
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `3.12` -> `3.10`

----- stderr -----
```

Warning for incompatible pinned version:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.10]

----- stderr -----
warning: The Python request from `.python-version` resolved to Python 3.10.[X], which is incompatible with the project's Python requirement: `>=3.11` (from `project.requires-python`)
Use `uv python pin` to update the `.python-version` file to a compatible version
```

Create child project:

```toml
# file: child/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = ["anyio==3.7.0"]
```

Pin outside project is ignored:

```console
$ cd child && uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

## Virtual empty

<!-- Derived from [`python_find::virtual_empty`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L418-L500) -->

Projects without `[project]` table work normally.

```toml
# file: pyproject.toml
[tool.mycooltool]
wow = "someconfig"
```

Find works normally:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.10]

----- stderr -----
```

With --no-project:

```console
$ uv python find --no-project
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.10]

----- stderr -----
```

Specific version request:

```console
$ uv python find 3.11
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

Create pin:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12`

----- stderr -----
```

Pin is respected:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.12]

----- stderr -----
```

Specific version overrides pin:

```console
$ uv python find 3.11
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

## Virtual dependency group

<!-- Derived from [`python_find::virtual_dependency_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L503-L589) -->

Virtual projects with dependency groups work normally.

```toml
# file: pyproject.toml
[dependency-groups]
foo = ["sortedcontainers"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

Find works:

```console
$ uv python find
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.10]

----- stderr -----
```

With --no-project:

```console
$ uv python find --no-project
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.10]

----- stderr -----
```

Specific version:

```console
$ uv python find 3.11
success: true
exit_code: 0
----- stdout -----
[PYTHON-3.11]

----- stderr -----
```

## Unsupported version

<!-- Derived from [`python_find::python_find_unsupported_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L778-L850) -->

```toml
# mdtest

[environment]
target-family = "unix"
python-versions = ["3.12"]
```

Requesting unsupported Python versions fails with a clear error.

```console
$ uv python find 3.6
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Invalid version request: Python <3.7 is not supported but 3.6 was requested.
```

## Show version

<!-- Derived from [`python_find::python_find_show_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L1165-L1208) -->

The `--show-version` flag displays only the Python version, not the full path.

No interpreters found:

```console
$ UV_TEST_PYTHON_PATH= uv python find --show-version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found in [PYTHON SOURCES]
```

Show first version found:

```console
$ uv python find --show-version
success: true
exit_code: 0
----- stdout -----
3.11.[X]

----- stderr -----
```

Show specific version:

```console
$ uv python find --show-version 3.12
success: true
exit_code: 0
----- stdout -----
3.12.[X]

----- stderr -----
```

Show another version:

```console
$ uv python find --show-version 3.11
success: true
exit_code: 0
----- stdout -----
3.11.[X]

----- stderr -----
```

## Path

<!-- Derived from [`python_find::python_find_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_find.rs#L1211-L1249) -->

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
```

Finding Python by path validates the provided location.

Create test directories:

```console
$ mkdir foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

```console
$ touch bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

No interpreter in directory:

```console
$ uv python find foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found in directory `foo`
```

Non-executable file fails:

```console
$ uv python find bar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to inspect Python interpreter from provided path at `bar`
  Caused by: Failed to query Python interpreter at `[TEMP_DIR]/bar`
  Caused by: [PERMISSION DENIED]
```

Non-existent path fails:

```console
$ uv python find foobar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found at path `foobar`
```
