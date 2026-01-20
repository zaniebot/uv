# Python Management - List

Tests for `uv python list` command.

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
```

## List

<!-- Derived from [`python_list::python_list`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_list.rs#L12-L132) -->

The `uv python list` command lists available Python interpreters.

No interpreters found:

```console
$ UV_TEST_PYTHON_PATH= uv python list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

List all interpreters:

```console
$ uv python list
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]
cpython-3.11.[X]-[PLATFORM] [PYTHON-3.11]

----- stderr -----
```

Filter by Python 3.12:

```console
$ uv python list 3.12
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]

----- stderr -----
```

Filter by Python 3.11:

```console
$ uv python list 3.11
success: true
exit_code: 0
----- stdout -----
cpython-3.11.[X]-[PLATFORM] [PYTHON-3.11]

----- stderr -----
```

Filter by CPython:

```console
$ uv python list cpython
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]
cpython-3.11.[X]-[PLATFORM] [PYTHON-3.11]

----- stderr -----
```

Filter by CPython 3.12:

```console
$ uv python list cpython@3.12
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]

----- stderr -----
```

Filter by partial key syntax:

```console
$ uv python list cpython-3.12
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]

----- stderr -----
```

Filter by unavailable implementation:

```console
$ uv python list pypy
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## List pin

<!-- Derived from [`python_list::python_list_pin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_list.rs#L135-L172) -->

Pinned versions don't affect listing.

Create pin:

```console
$ uv python pin 3.12
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12`

----- stderr -----
```

Pin doesn't affect listing:

```console
$ uv python list
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]
cpython-3.11.[X]-[PLATFORM] [PYTHON-3.11]

----- stderr -----
```

--no-config has no effect on listing:

```console
$ uv python list --no-config
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]
cpython-3.11.[X]-[PLATFORM] [PYTHON-3.11]

----- stderr -----
```

## Unsupported version

<!-- Derived from [`python_list::python_list_unsupported_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_list.rs#L217-L290) -->

```toml
# mdtest

[environment]
python-versions = ["3.12"]
target-family = "unix"
```

Requesting unsupported Python versions fails.

Request Python 3.6:

```console
$ uv python list 3.6
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Invalid version request: Python <3.7 is not supported but 3.6 was requested.
```

Request Python 3.6.9:

```console
$ uv python list 3.6.9
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Invalid version request: Python <3.7 is not supported but 3.6.9 was requested.
```

Request Python 2.6:

```console
$ uv python list 2.6
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Invalid version request: Python <3.7 is not supported but 2.6 was requested.
```

Request Python 2.6.8:

```console
$ uv python list 2.6.8
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Invalid version request: Python <3.7 is not supported but 2.6.8 was requested.
```

Future version returns empty:

```console
$ uv python list 4.2
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Low version range returns empty:

```console
$ uv python list <3.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Freethreaded on unsupported version:

```console
$ uv python list 3.12t
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Invalid version request: Python <3.13 does not support free-threading but 3.12+freethreaded was requested.
```

## Duplicate path entries

<!-- Derived from [`python_list::python_list_duplicate_path_entries`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_list.rs#L293-L359) -->

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
```

Duplicate entries in PATH are deduplicated.

List with duplicated path shows each interpreter once:

```console
$ UV_TEST_PYTHON_PATH=[PYTHON-3.12]:[PYTHON-3.11]:[PYTHON-3.12]:[PYTHON-3.11] uv python list
success: true
exit_code: 0
----- stdout -----
cpython-3.12.[X]-[PLATFORM] [PYTHON-3.12]
cpython-3.11.[X]-[PLATFORM] [PYTHON-3.11]

----- stderr -----
```

## Downloads

<!-- Derived from [`python_list::python_list_downloads`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_list.rs#L362-L412) -->

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
```

Available Python downloads are shown.

List available downloads for Python 3.10:

```console
$ uv python list 3.10
success: true
exit_code: 0
----- stdout -----
cpython-3.10.19-[PLATFORM]    <download available>
pypy-3.10.16-[PLATFORM]       <download available>
graalpy-3.10.0-[PLATFORM]     <download available>

----- stderr -----
```

Show all patch versions:

```console
$ uv python list 3.10 --all-versions
success: true
exit_code: 0
----- stdout -----
cpython-3.10.19-[PLATFORM]    <download available>
cpython-3.10.18-[PLATFORM]    <download available>
cpython-3.10.17-[PLATFORM]    <download available>
cpython-3.10.16-[PLATFORM]    <download available>
cpython-3.10.15-[PLATFORM]    <download available>
cpython-3.10.14-[PLATFORM]    <download available>
cpython-3.10.13-[PLATFORM]    <download available>
cpython-3.10.12-[PLATFORM]    <download available>
cpython-3.10.11-[PLATFORM]    <download available>
cpython-3.10.9-[PLATFORM]     <download available>
cpython-3.10.8-[PLATFORM]     <download available>
cpython-3.10.7-[PLATFORM]     <download available>
cpython-3.10.6-[PLATFORM]     <download available>
cpython-3.10.5-[PLATFORM]     <download available>
cpython-3.10.4-[PLATFORM]     <download available>
cpython-3.10.3-[PLATFORM]     <download available>
cpython-3.10.2-[PLATFORM]     <download available>
cpython-3.10.0-[PLATFORM]     <download available>
pypy-3.10.16-[PLATFORM]       <download available>
pypy-3.10.14-[PLATFORM]       <download available>
pypy-3.10.13-[PLATFORM]       <download available>
pypy-3.10.12-[PLATFORM]       <download available>
graalpy-3.10.0-[PLATFORM]     <download available>

----- stderr -----
```

## With mirrors

<!-- Derived from [`python_list::python_list_with_mirrors`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_list.rs#L595-L685) -->

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
```

Mirror URLs are used for Python downloads.

List with Python mirror:

```console
$ UV_PYTHON_INSTALL_MIRROR=https://mirror.example.com uv python list cpython@3.10.19 --show-urls
success: true
exit_code: 0
----- stdout -----
cpython-3.10.19-[PLATFORM] https://mirror.example.com/[WILDCARD]

----- stderr -----
```

List with PyPy mirror:

```console
$ UV_PYPY_INSTALL_MIRROR=https://pypy-mirror.example.com uv python list pypy@3.10 --show-urls
success: true
exit_code: 0
----- stdout -----
pypy-3.10.16-[PLATFORM] https://pypy-mirror.example.com/[WILDCARD]

----- stderr -----
```

List with both mirrors:

```console
$ UV_PYTHON_INSTALL_MIRROR=https://python-mirror.example.com UV_PYPY_INSTALL_MIRROR=https://pypy-mirror.example.com uv python list 3.10 --show-urls
success: true
exit_code: 0
----- stdout -----
cpython-3.10.19-[PLATFORM] https://python-mirror.example.com/[WILDCARD]
pypy-3.10.16-[PLATFORM] https://pypy-mirror.example.com/[WILDCARD]
graalpy-3.10.0-[PLATFORM] https://github.com/oracle/graalpython/releases/download/[WILDCARD]

----- stderr -----
```

List without mirrors shows default URLs:

```console
$ uv python list 3.10 --show-urls
success: true
exit_code: 0
----- stdout -----
cpython-3.10.19-[PLATFORM] https://github.com/astral-sh/python-build-standalone/releases/download/[WILDCARD]
pypy-3.10.16-[PLATFORM] https://downloads.python.org/pypy/[WILDCARD]
graalpy-3.10.0-[PLATFORM] https://github.com/oracle/graalpython/releases/download/[WILDCARD]

----- stderr -----
```
