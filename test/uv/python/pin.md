# Python Management - Pin

Tests for `uv python pin` command.

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
```

## Pin

<!-- Derived from [`python_pin::python_pin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L13-L183) -->

The `uv python pin` command manages Python version pinning via `.python-version` files.

Without arguments, reading non-existent pin fails:

```console
$ uv python pin
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No Python version file found; specify a version to create one
```

Pin to a version:

```console
$ uv python pin any
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `any`

----- stderr -----
```

Verify file contents:

```text
# file: .python-version
# snapshot: true
any
```

Read current pin:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
any

----- stderr -----
```

Update to Python 3.12:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `any` -> `3.12`

----- stderr -----
```

Verify update:

```text
# file: .python-version
# snapshot: true
3.12
```

Update to Python 3.11:

```console
$ uv python pin 3.11
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `3.12` -> `3.11`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
3.11
```

Pin to CPython:

```console
$ uv python pin cpython
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `3.11` -> `cpython`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
cpython
```

Pin to CPython 3.12:

```console
$ uv python pin cpython@3.12
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `cpython` -> `cpython@3.12`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
cpython@3.12
```

Pin via non-canonical syntax normalizes:

```console
$ uv python pin cp3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `cpython@3.12`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
cpython@3.12
```

Pin via partial key syntax:

```console
$ uv python pin cpython-3.12
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `cpython@3.12` -> `cpython-3.12-any-any-any`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
cpython-3.12-any-any-any
```

Pin to specific path:

```console
$ uv python pin [PYTHON-3.11]
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `cpython-3.12-any-any-any` -> `[PYTHON-3.11]`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.11]
```

Pin to implementation not installed shows warning:

```console
$ uv python pin pypy
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `[PYTHON-3.11]` -> `pypy`

----- stderr -----
warning: No interpreter found for PyPy in managed installations or search path
```

Verify:

```text
# file: .python-version
# snapshot: true
pypy
```

Pin to version not installed shows warning:

```console
$ uv python pin 3.7
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `pypy` -> `3.7`

----- stderr -----
warning: No interpreter found for Python 3.7 in managed installations or search path
```

Verify:

```text
# file: .python-version
# snapshot: true
3.7
```

## Global if no local

<!-- Derived from [`python_pin::python_pin_global_if_no_local`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L186-L223) -->

If no project-level pin exists, respect the global pin.

No pin exists yet:

```console
$ uv python pin
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No Python version file found; specify a version to create one
```

Create global pin:

```console
$ uv python pin 3.11 --global
success: true
exit_code: 0
----- stdout -----
Pinned `[UV_USER_CONFIG_DIR]/.python-version` to `3.11`

----- stderr -----
```

Global pin is used when no local pin:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
3.11

----- stderr -----
```

## Global use local if available

<!-- Derived from [`python_pin::python_pin_global_use_local_if_available`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L227-L305) -->

Project-level pins take precedence over global pins.

Create global pin:

```console
$ uv python pin 3.12 --global
success: true
exit_code: 0
----- stdout -----
Pinned `[UV_USER_CONFIG_DIR]/.python-version` to `3.12`

----- stderr -----
```

Global pin is used without local:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
3.12

----- stderr -----
```

Create local pin:

```console
$ uv python pin 3.11
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.11`

----- stderr -----
```

Local overrides global:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
3.11

----- stderr -----
```

Check global pin explicitly:

```console
$ uv python pin --global
success: true
exit_code: 0
----- stdout -----
3.12

----- stderr -----
```

Verify local pin:

```text
# file: .python-version
# snapshot: true
3.11
```

## Global creates parent dirs

<!-- Derived from [`python_pin::python_pin_global_creates_parent_dirs`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L307-L330) -->

```toml
# mdtest

[environment]
python-versions = ["3.12"]
```

Global pin creates parent directories automatically.

```console
$ uv python pin 3.12 --global
success: true
exit_code: 0
----- stdout -----
Pinned `[UV_USER_CONFIG_DIR]/.python-version` to `3.12`

----- stderr -----
```

## No python

<!-- Derived from [`python_pin::python_pin_no_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L336-L348) -->

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
target-family = "unix"
```

Pin without Python interpreter available.

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12`

----- stderr -----
warning: No interpreter found for Python 3.12 in managed installations or search path
```

## Compatible with requires-python

<!-- Derived from [`python_pin::python_pin_compatible_with_requires_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L350-L517) -->

```toml
# mdtest

[environment]
python-versions = ["3.10", "3.11"]
```

Python pin validates compatibility with project requires-python.

Create project:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = ["iniconfig"]
```

Incompatible version rejected:

```console
$ uv python pin 3.10
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The requested Python version `3.10` is incompatible with the project `requires-python` value of `>=3.11`.
```

Incompatible implementation version rejected:

```console
$ uv python pin cpython@3.10
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The requested Python version `cpython@3.10` is incompatible with the project `requires-python` value of `>=3.11`.
```

Bypass check with --no-project:

```console
$ uv python pin cpython@3.10 --no-project
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `cpython@3.10`

----- stderr -----
```

Bypass with --no-workspace (alias):

```console
$ uv python pin cpython@3.10 --no-workspace
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `cpython@3.10`

----- stderr -----
```

Complex version range with warning:

```console
$ uv python pin >3.8,<3.11
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `cpython@3.10` -> `>3.8, <3.11`

----- stderr -----
warning: The requested Python version `>3.8, <3.11` resolves to `3.10.[X]` which  is incompatible with the project `requires-python` value of `>=3.11`.
```

Compatible version succeeds:

```console
$ uv python pin 3.11
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `>3.8, <3.11` -> `3.11`

----- stderr -----
```

Compatible freethreaded variant:

```console
$ uv python pin 3.13t
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `3.11` -> `3.13+freethreaded`

----- stderr -----
warning: No interpreter found for Python 3.13+freethreaded in [PYTHON SOURCES]
```

Compatible implementation version:

```console
$ uv python pin cpython@3.11
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `3.13+freethreaded` -> `cpython@3.11`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
cpython@3.11
```

Update requires-python:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

Reading incompatible pin shows warning:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
cpython@3.11

----- stderr -----
warning: The pinned Python version `cpython@3.11` is incompatible with the project `requires-python` value of `>=3.12`.
```

Implementation resolving to incompatible version:

```console
$ uv python pin cpython
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `cpython@3.11` -> `cpython`

----- stderr -----
warning: The requested Python version `cpython` resolves to `3.10.[X]` which  is incompatible with the project `requires-python` value of `>=3.12`.
```

Reading incompatible resolved pin:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
cpython

----- stderr -----
warning: The pinned Python version `cpython` resolves to `3.10.[X]` which  is incompatible with the project `requires-python` value of `>=3.12`.
```

Complex version range resolving to incompatible:

```console
$ uv python pin >3.8,<3.12
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `cpython` -> `>3.8, <3.12`

----- stderr -----
warning: The requested Python version `>3.8, <3.12` resolves to `3.10.[X]` which  is incompatible with the project `requires-python` value of `>=3.12`.
```

Reading shows warning:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
>3.8, <3.12

----- stderr -----
warning: The pinned Python version `>3.8, <3.12` resolves to `3.10.[X]` which  is incompatible with the project `requires-python` value of `>=3.12`.
```

## Warning not installed

<!-- Derived from [`python_pin::warning_pinned_python_version_not_installed`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L519-L558) -->

```toml
# mdtest

[environment]
python-versions = ["3.10", "3.11"]
```

Warning when pinned version cannot be resolved.

Create project:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = ["iniconfig"]
```

Create pin for unavailable version:

```text
# file: .python-version
3.12
```

Reading unresolved pin shows warning:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
3.12

----- stderr -----
warning: Failed to resolve pinned Python version `3.12`: No interpreter found for Python 3.12 in [PYTHON SOURCES]
```

## Resolve no python

<!-- Derived from [`python_pin::python_pin_resolve_no_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L561-L575) -->

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
```

Resolved pins require a Python interpreter.

```console
$ uv python pin --resolved 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.12 in [PYTHON SOURCES]

hint: A managed Python download is available for Python 3.12, but Python downloads are set to 'never'
```

## Resolve

<!-- Derived from [`python_pin::python_pin_resolve`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L577-L744) -->

```toml
# mdtest

[environment]
python-versions = ["3.12", "3.13"]
```

Resolved pins use full interpreter paths.

Pin first interpreter:

```console
$ uv python pin --resolved any
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `[PYTHON-3.12]`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.12]
```

Pin to Python 3.13:

```console
$ uv python pin --resolved 3.13
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `[PYTHON-3.12]` -> `[PYTHON-3.13]`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.13]
```

Pin same version again:

```console
$ uv python pin --resolved 3.13
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `[PYTHON-3.13]`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.13]
```

Pin to CPython:

```console
$ uv python pin --resolved cpython
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `[PYTHON-3.13]` -> `[PYTHON-3.12]`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.12]
```

Pin to CPython 3.13:

```console
$ uv python pin --resolved cpython@3.13
success: true
exit_code: 0
----- stdout -----
Updated `.python-version` from `[PYTHON-3.12]` -> `[PYTHON-3.13]`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.13]
```

Pin via partial key syntax:

```console
$ uv python pin --resolved cpython-3.13
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `[PYTHON-3.13]`

----- stderr -----
```

Verify:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.13]
```

Unavailable implementation fails:

```console
$ uv python pin --resolved pypy
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for PyPy in managed installations or search path

hint: A managed Python download is available for PyPy, but Python downloads are set to 'never'
```

Verify unchanged:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.13]
```

Unavailable version fails:

```console
$ uv python pin --resolved 3.7
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.7 in managed installations or search path
```

Verify unchanged:

```text
# file: .python-version
# snapshot: true
[PYTHON-3.13]
```

## With comments

<!-- Derived from [`python_pin::python_pin_with_comments`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L746-L783) -->

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
```

Comments and multiple versions in version files.

Create version file with comments:

```text
# file: .python-version
3.12

# 3.11
3.10
```

Reading shows uncommented versions:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
3.12
3.10

----- stderr -----
```

Create versions file with comments:

```text
# file: .python-versions
3.12

# 3.11
3.10
```

Reading shows uncommented versions:

```console
$ uv python pin
success: true
exit_code: 0
----- stdout -----
3.12
3.10

----- stderr -----
```

## Rm

<!-- Derived from [`python_pin::python_pin_rm`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_pin.rs#L813-L899) -->

```toml
# mdtest

[environment]
python-versions = ["3.12"]
```

Removing pins with --rm flag.

No pin to remove:

```console
$ uv python pin --rm
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No Python version file found
```

Create and remove local pin:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12`

----- stderr -----
```

```console
$ uv python pin --rm
success: true
exit_code: 0
----- stdout -----
Removed Python version file at `.python-version`

----- stderr -----
```

No global pin to remove:

```console
$ uv python pin --rm --global
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No global Python pin found
```

Local pin doesn't count as global:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12`

----- stderr -----
```

```console
$ uv python pin --rm --global
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No global Python pin found
```

Create and remove global pin:

```console
$ uv python pin 3.12 --global
success: true
exit_code: 0
----- stdout -----
Pinned `[UV_USER_CONFIG_DIR]/.python-version` to `3.12`

----- stderr -----
```

```console
$ uv python pin --rm --global
success: true
exit_code: 0
----- stdout -----
Removed global Python pin at `[UV_USER_CONFIG_DIR]/.python-version`

----- stderr -----
```

Create global pin again:

```console
$ uv python pin 3.12 --global
success: true
exit_code: 0
----- stdout -----
Pinned `[UV_USER_CONFIG_DIR]/.python-version` to `3.12`

----- stderr -----
```

Remove local pin:

```console
$ uv python pin --rm
success: true
exit_code: 0
----- stdout -----
Removed Python version file at `.python-version`

----- stderr -----
```

Global pin not removed without --global:

```console
$ uv python pin --rm
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No Python version file found; use `--rm --global` to remove the global pin
```
