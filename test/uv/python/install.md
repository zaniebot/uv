# Python Management - Install

Tests for `uv python install` command.

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
required-features = ["python-managed"]
```

## Install

<!-- Derived from [`python_install::python_install`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L22-L145) -->

Basic Python installation and uninstallation.

Install the latest Python version:

```console
$ uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python3.14)
```

Already installed returns message:

```console
$ uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python is already installed. Use `uv python install <request>` to install another version.
```

Requesting already installed version:

```console
$ uv python install 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.14 is already installed
```

Reinstall with flag:

```console
$ uv python install 3.14 --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 ~ cpython-3.14.[X]-[PLATFORM] (python3.14)
```

Uninstallation requires argument:

```console
$ uv python uninstall
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the following required arguments were not provided:
  <TARGETS>...

Usage: uv python uninstall --install-dir <INSTALL_DIR> <TARGETS>...

For more information, try '--help'.
```

Uninstall a version:

```console
$ uv python uninstall 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.14
Uninstalled Python 3.14.[X] in [TIME]
 - cpython-3.14.[X]-[PLATFORM] (python3.14)
```

## Reinstall

<!-- Derived from [`python_install::python_reinstall`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L148-L201) -->

Reinstalling Python versions.

Install multiple versions:

```console
$ uv python install 3.12 3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.12.[X]-[PLATFORM] (python3.12)
 + cpython-3.13.[X]-[PLATFORM] (python3.13)
```

Reinstall a single version:

```console
$ uv python install 3.13 --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 ~ cpython-3.13.[X]-[PLATFORM] (python3.13)
```

Reinstall all installed versions:

```console
$ uv python install --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 ~ cpython-3.12.[X]-[PLATFORM] (python3.12)
 ~ cpython-3.13.[X]-[PLATFORM] (python3.13)
```

Reinstall version that isn't installed:

```console
$ uv python install 3.11 --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.11.[X] in [TIME]
 + cpython-3.11.[X]-[PLATFORM] (python3.11)
```

## Reinstall patch

<!-- Derived from [`python_install::python_reinstall_patch`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L204-L235) -->

Reinstalling specific patch versions.

Install specific patch versions:

```console
$ uv python install 3.12.6 3.12.7
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.12.6-[PLATFORM]
 + cpython-3.12.7-[PLATFORM] (python3.12)
```

Reinstall minor version installs latest patch:

```console
$ uv python install 3.12 --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.[X] in [TIME]
 + cpython-3.12.[X]-[PLATFORM] (python3.12)
```

## Automatic

<!-- Derived from [`python_install::python_install_automatic`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L238-L345) -->

Automatic Python installation during `uv run`.

With downloads disabled, automatic install fails:

```console
$ uv run --no-python-downloads python -c "import sys; print(sys.version_info[:2])"
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found in [PYTHON SOURCES]

hint: A managed Python download is available, but Python downloads are set to 'never'
```

Without restriction, fetches latest Python:

```console
$ uv run python -c "import sys; print(sys.version_info[:2])"
success: true
exit_code: 0
----- stdout -----
(3, 14)

----- stderr -----
```

Subsequently usable even with downloads disabled:

```console
$ uv run --no-python-downloads python -c "import sys; print(sys.version_info[:2])"
success: true
exit_code: 0
----- stdout -----
(3, 14)

----- stderr -----
```

Respects Python version request:

```console
$ uv run -p 3.12 python -c "import sys; print(sys.version_info[:2])"
success: true
exit_code: 0
----- stdout -----
(3, 12)

----- stderr -----
```

Invalid requests fail:

```console
$ uv run -p foobar python -c "import sys; print(sys.version_info[:2])"
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for executable name `foobar` in [PYTHON SOURCES]
```

## Regression cpython

<!-- Derived from [`python_install::regression_cpython`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L348-L380) -->

Regression test for CPython runtime issue.

Create test script:

```python
# file: mre.py
class Foo(str): ...

a = []
new_value = Foo("1")
a += new_value
```

Run with Python 3.12:

```console
$ uv run -p 3.12 mre.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Force

<!-- Derived from [`python_install::python_install_force`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L383-L444) -->

Force replacement of executables.

Install Python:

```console
$ uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python3.14)
```

Force replacement of existing executable:

```console
$ uv python install --force
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python3.14)
```

Create unmanaged executable:

```text
# file: bin/python3.14
# This file simulates an unmanaged Python executable
```

Install without force shows warning:

```console
$ UV_TOOL_BIN_DIR=bin uv python install 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Failed to install executable for cpython-3.14.[X]-[PLATFORM]
  Caused by: Executable already exists at `[BIN]/python3.14` but is not managed by uv; use `--force` to replace it
```

Force replaces unmanaged executable:

```console
$ UV_TOOL_BIN_DIR=bin uv python install --force 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python3.14)
```

## Minor

<!-- Derived from [`python_install::python_install_minor`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L447-L504) -->

Installing a minor version.

Install Python 3.11:

```console
$ uv python install 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.11.[X] in [TIME]
 + cpython-3.11.[X]-[PLATFORM] (python3.11)
```

Uninstall Python 3.11:

```console
$ uv python uninstall 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.11
Uninstalled Python 3.11.[X] in [TIME]
 - cpython-3.11.[X]-[PLATFORM] (python3.11)
```

## Multiple patch

<!-- Derived from [`python_install::python_install_multiple_patch`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L507-L584) -->

Installing multiple patch versions.

Install two patch versions:

```console
$ uv python install 3.12.8 3.12.6
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.12.6-[PLATFORM]
 + cpython-3.12.8-[PLATFORM] (python3.12)
```

Uninstall the newer patch:

```console
$ uv python uninstall 3.12.8
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.12.8
Uninstalled Python 3.12.8 in [TIME]
 - cpython-3.12.8-[PLATFORM] (python3.12)
```

## Preview

<!-- Derived from [`python_install::python_install_preview`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L587-L681) -->

Installing with --preview creates additional symlinks.

Install with preview:

```console
$ uv python install --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python, python3, python3.14)
```

Already installed:

```console
$ uv python install --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python is already installed. Use `uv python install <request>` to install another version.
```

Reinstall:

```console
$ uv python install --preview --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 ~ cpython-3.14.[X]-[PLATFORM] (python, python3, python3.14)
```

Force replacement:

```console
$ uv python install --preview --force
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python, python3, python3.14)
```

Create unmanaged executable for testing:

```text
# file: bin/python3.14
```

Install with unmanaged executable shows warning:

```console
$ UV_TOOL_BIN_DIR=bin uv python install --preview 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Failed to install executable for cpython-3.14.[X]-[PLATFORM]
  Caused by: Executable already exists at `[BIN]/python3.14` but is not managed by uv; use `--force` to replace it
```

With --bin flag, error instead of warning:

```console
$ UV_TOOL_BIN_DIR=bin uv python install --preview --bin 3.14
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: Failed to install executable for cpython-3.14.[X]-[PLATFORM]
  Caused by: Executable already exists at `[BIN]/python3.14` but is not managed by uv; use `--force` to replace it
```

With UV_PYTHON_INSTALL_BIN environment variable:

```console
$ UV_TOOL_BIN_DIR=bin UV_PYTHON_INSTALL_BIN=1 uv python install --preview 3.14
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: Failed to install executable for cpython-3.14.[X]-[PLATFORM]
  Caused by: Executable already exists at `[BIN]/python3.14` but is not managed by uv; use `--force` to replace it
```

With --no-bin, installation is silent:

```console
$ UV_TOOL_BIN_DIR=bin uv python install --preview --no-bin 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.14 is already installed
```

With UV_PYTHON_INSTALL_BIN=0:

```console
$ UV_TOOL_BIN_DIR=bin UV_PYTHON_INSTALL_BIN=0 uv python install --preview 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.14 is already installed
```

Force replaces unmanaged file:

```console
$ UV_TOOL_BIN_DIR=bin uv python install --preview --force 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python3.14)
```

Uninstall Python 3.14:

```console
$ uv python uninstall 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.14
Uninstalled Python 3.14.[X] in [TIME]
 - cpython-3.14.[X]-[PLATFORM] (python, python3, python3.14)
```

Install Python 3.11 with preview:

```console
$ uv python install 3.11 --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.11.[X] in [TIME]
 + cpython-3.11.[X]-[PLATFORM] (python3.11)
```

Uninstall Python 3.11:

```console
$ uv python uninstall 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.11
Uninstalled Python 3.11.[X] in [TIME]
 - cpython-3.11.[X]-[PLATFORM] (python3.11)
```

Install multiple patch versions with preview:

```console
$ uv python install --preview 3.12.8 3.12.6
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.12.6-[PLATFORM]
 + cpython-3.12.8-[PLATFORM] (python3.12)
```

## Preview no bin

<!-- Derived from [`python_install::python_install_preview_no_bin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L864-L909) -->

Installing with --no-bin skips executable installation.

Install without bin executables:

```console
$ uv python install --preview --no-bin
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM]
```

--no-bin conflicts with --default:

```console
$ uv python install --preview --no-bin --default
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--no-bin' cannot be used with '--default'

Usage: uv python install --no-bin --install-dir <INSTALL_DIR> [TARGETS]...

For more information, try '--help'.
```

## Preview upgrade

<!-- Derived from [`python_install::python_install_preview_upgrade`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L912-L1046) -->

Installing with preview mode and patch version upgrades.

Install Python 3.12.5 with preview:

```console
$ uv python install --preview 3.12.5
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.5 in [TIME]
 + cpython-3.12.5-[PLATFORM] (python3.12)
```

Installing older patch doesn't replace executable:

```console
$ uv python install --preview 3.12.4
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.4 in [TIME]
 + cpython-3.12.4-[PLATFORM]
```

Installing newer patch upgrades automatically:

```console
$ uv python install --preview 3.12.6
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.6 in [TIME]
 + cpython-3.12.6-[PLATFORM] (python3.12)
```

## Freethreaded

<!-- Derived from [`python_install::python_install_freethreaded`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L1071-L1225) -->

Installing freethreaded Python builds.

Install Python 3.13 freethreaded:

```console
$ uv python install --preview 3.13t
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]+freethreaded-[PLATFORM] (python3.13t)
```

Find freethreaded Python with 't' suffix:

```console
$ uv python find 3.13t
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+freethreaded-[PLATFORM]/[INSTALL-BIN]/[PYTHON]

----- stderr -----
```

Find with explicit +freethreaded:

```console
$ uv python find 3.13+freethreaded
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+freethreaded-[PLATFORM]/[INSTALL-BIN]/[PYTHON]

----- stderr -----
```

Create virtual environment with freethreaded Python:

```console
$ uv venv --python 3.13t
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.13.[X]+freethreaded
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Freethreaded is distinct from standard Python 3.13:

```console
$ uv python install 3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]-[PLATFORM] (python3.13)
```

Freethreaded not available for Python 3.12:

```console
$ uv python install 3.12t
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No download found for request: cpython-3.12+freethreaded-[PLATFORM]
```

Uninstall all Python 3.13 versions:

```console
$ uv python uninstall --all
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python installations
Uninstalled 2 versions in [TIME]
 - cpython-3.13.[X]+freethreaded-[PLATFORM] (python3.13t)
 - cpython-3.13.[X]-[PLATFORM] (python3.13)
```

## Debug freethreaded

<!-- Derived from [`python_install::python_install_debug_freethreaded`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L1387-L1534) -->

Installing debug freethreaded Python (combines both flags).

Install Python 3.13 debug+freethreaded:

```console
$ uv python install --preview 3.13td
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]+freethreaded+debug-[PLATFORM] (python3.13td)
```

Find with 'td' suffix:

```console
$ uv python find 3.13td
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+freethreaded+debug-[PLATFORM]/bin/python3.13td

----- stderr -----
```

Standard 3.13 not found when only debug+freethreaded installed:

```console
$ uv python find 3.13
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.13 in virtual environments, managed installations, or search path
```

Find with explicit flags:

```console
$ uv python find 3.13+freethreaded+debug
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+freethreaded+debug-[PLATFORM]/bin/python3.13td

----- stderr -----
```

Each variant is distinct - install standard 3.13:

```console
$ uv python install 3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]-[PLATFORM] (python3.13)
```

Install freethreaded variant:

```console
$ uv python install 3.13t
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]+freethreaded-[PLATFORM] (python3.13t)
```

Install debug variant:

```console
$ uv python install 3.13d
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]+debug-[PLATFORM] (python3.13d)
```

Standard version preferred without opt-in:

```console
$ uv python find 3.13
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.[X]-[PLATFORM]/bin/python3.13

----- stderr -----
```

Each variant findable with suffix:

```console
$ uv python find 3.13t
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.[X]+freethreaded-[PLATFORM]/bin/python3.13t

----- stderr -----
```

```console
$ uv python find 3.13td
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+freethreaded+debug-[PLATFORM]/bin/python3.13td

----- stderr -----
```

Uninstall all 3.13 variants:

```console
$ uv python uninstall --all
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python installations
Uninstalled 4 versions in [TIME]
 - cpython-3.13.[X]+freethreaded+debug-[PLATFORM] (python3.13td)
 - cpython-3.13.[X]+freethreaded-[PLATFORM] (python3.13t)
 - cpython-3.13.[X]+debug-[PLATFORM] (python3.13d)
 - cpython-3.13.[X]-[PLATFORM] (python3.13)
```

## Invalid request

<!-- Derived from [`python_install::python_install_invalid_request`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L1537-L1573) -->

Error handling for invalid installation requests.

Invalid Python request:

```console
$ uv python install foobar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `foobar` is not a valid Python download request; see `uv help python` for supported formats and `uv python list --only-downloads` for available versions
```

Version without download:

```console
$ uv python install 3.8.0
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No download found for request: cpython-3.8.0-[PLATFORM]
```

Mixed valid and invalid requests:

```console
$ uv python install 3.8.0 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No download found for request: cpython-3.8.0-[PLATFORM]
```

## Default

<!-- Derived from [`python_install::python_install_default`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L1576-L1737) -->

Installing Python as the default (python, python3 executables).

Install Python 3.14:

```console
$ uv python install 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python3.14)
```

Install again with --default flag:

```console
$ uv python install --default 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `--default` option is experimental and may change without warning. Pass `--preview-features python-install-default` to disable this warning
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python, python3)
```

## Default preview

<!-- Derived from [`python_install::python_install_default_preview`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L1816-L2201) -->

Installing Python as default with preview mode.

Install with preview:

```console
$ uv python install --preview 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python3.14)
```

Install with --default in preview mode:

```console
$ uv python install --preview --default 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python, python3)
```

## Upgrade not allowed

<!-- Derived from [`python_install::python_upgrade_not_allowed`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L1228-L1253) -->

The `uv python upgrade` command only accepts minor versions.

Request a patch upgrade:

```console
$ uv python upgrade --preview 3.13.0
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: `uv python upgrade` only accepts minor versions, got: 3.13.0
```

Request a pre-release upgrade:

```console
$ uv python upgrade --preview 3.14rc3
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: `uv python upgrade` only accepts minor versions, got: 3.14rc3
```

## Debug

<!-- Derived from [`python_install::python_install_debug`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L1258-L1382) -->

```toml
# mdtest

[environment]
target-family = "unix"
```

Install Python 3.13 with debug symbols:

```console
$ uv python install --preview 3.13+debug
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]+debug-[PLATFORM] (python3.13d)
```

The executable should work:

```console
$ python3.13d -c "import subprocess; print('hello world')"
success: true
exit_code: 0
----- stdout -----
hello world

----- stderr -----
```

Find debug version with opt-in:

```console
$ uv python find 3.13d
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+debug-[PLATFORM]/bin/python3.13d

----- stderr -----
```

Debug version is found even without opt-in when it's the only 3.13:

```console
$ uv python find 3.13
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+debug-[PLATFORM]/bin/python3.13d

----- stderr -----
```

Install standard 3.13:

```console
$ uv python install 3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]-[PLATFORM] (python3.13)
```

Now we prefer the non-debug version without opt-in:

```console
$ uv python find 3.13
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.[X]-[PLATFORM]/bin/python3.13

----- stderr -----
```

But still select debug with opt-in:

```console
$ uv python find 3.13d
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+debug-[PLATFORM]/bin/python3.13d

----- stderr -----
```

Allow selection with +debug syntax:

```console
$ uv python find 3.13+debug
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13+debug-[PLATFORM]/bin/python3.13d

----- stderr -----
```

Works with older Python versions:

```console
$ uv python install 3.12d
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.[X] in [TIME]
 + cpython-3.12.[X]+debug-[PLATFORM] (python3.12d)
```

Uninstall all:

```console
$ uv python uninstall --all
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python installations
Uninstalled 3 versions in [TIME]
 - cpython-3.12.[X]+debug-[PLATFORM] (python3.12d)
 - cpython-3.13.[X]+debug-[PLATFORM] (python3.13d)
 - cpython-3.13.[X]-[PLATFORM] (python3.13)
```

## Unknown

<!-- Derived from [`python_install::python_install_unknown`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2204-L2230) -->

Invalid Python download requests are rejected.

Unknown request:

```console
$ uv python install foobar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `foobar` is not a valid Python download request; see `uv help python` for supported formats and `uv python list --only-downloads` for available versions
```

Directory path:

```console
$ mkdir foo
```

```console
$ uv python install ./foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `./foo` is not a valid Python download request; see `uv help python` for supported formats and `uv python list --only-downloads` for available versions
```

## Broken link

<!-- Derived from [`python_install::python_install_broken_link`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2234-L2269) -->

```toml
# mdtest

[environment]
target-family = "unix"
```

Broken symlinks are replaced during installation.

Create broken symlink:

```console
$ mkdir -p [BIN_DIR]
$ ln -s [TEMP_DIR]/does-not-exist [BIN_DIR]/python3.13
```

Install Python replaces the broken symlink:

```console
$ uv python install 3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.11 in [TIME]
 + cpython-3.13.11-[PLATFORM] (python3.13)
```

## Default prerelease

<!-- Derived from [`python_install::python_install_default_prerelease`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2275-L2308) -->

The --default flag works with pre-release Python versions.

This verifies the fix for issue #16696 where --default didn't create python.exe and python3.exe
links for pre-release versions.

Install Python 3.15 (currently pre-release) as default:

```console
$ uv python install --default --preview-features python-install-default 3.15
success: true
exit_code: 0
```

All three executables should exist:

```console
$ ls [BIN_DIR]/python3.15[EXE] [BIN_DIR]/python3[EXE] [BIN_DIR]/python[EXE]
success: true
exit_code: 0
```

## Default from env

<!-- Derived from [`python_install::python_install_default_from_env`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2311-L2397) -->

The UV_PYTHON environment variable specifies default install target.

Install version from UV_PYTHON:

```console
$ UV_PYTHON=3.12 uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.[X] in [TIME]
 + cpython-3.12.[X]-[PLATFORM] (python3.12)
```

Explicit requests override UV_PYTHON:

```console
$ UV_PYTHON=3.12 uv python install 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.11.[X] in [TIME]
 + cpython-3.11.[X]-[PLATFORM] (python3.11)
```

Uninstall ignores UV_PYTHON and requires targets:

```console
$ UV_PYTHON=3.12 uv python uninstall
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the following required arguments were not provided:
  <TARGETS>...

Usage: uv python uninstall --install-dir <INSTALL_DIR> <TARGETS>...

For more information, try '--help'.
```

Uninstall --all ignores UV_PYTHON:

```console
$ UV_PYTHON=3.11 uv python uninstall --all
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python installations
Uninstalled 2 versions in [TIME]
 - cpython-3.11.[X]-[PLATFORM] (python3.11)
 - cpython-3.12.[X]-[PLATFORM] (python3.12)
```

Uninstall with conflicting options errors:

```console
$ uv python uninstall --all 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--all' cannot be used with '<TARGETS>...'

Usage: uv python uninstall --all --install-dir <INSTALL_DIR> <TARGETS>...

For more information, try '--help'.
```

## Patch dylib

<!-- Derived from [`python_install::python_install_patch_dylib`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2401-L2444) -->

```toml
# mdtest

[environment]
target-os = "macos"
```

On macOS, the Python dylib is patched to have the correct install path.

Install Python 3.13.1:

```console
$ uv python install --preview 3.13.1
success: true
exit_code: 0
```

Check dylib install name:

```console
$ otool -D [TEMP_DIR]/managed/cpython-3.13.1-[PLATFORM]/lib/libpython3.13.dylib
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.1-[PLATFORM]/lib/libpython3.13.dylib:
[TEMP_DIR]/managed/cpython-3.13.1-[PLATFORM]/lib/libpython3.13.dylib

----- stderr -----
```

## Prerelease

<!-- Derived from [`python_install::python_install_prerelease`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2447-L2478) -->

Install pre-release Python versions.

Install 3.15 (currently pre-release):

```console
$ uv python install 3.15
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.15.[X] in [TIME]
 + cpython-3.15.[X]-[PLATFORM] (python3.15)
```

Install specific pre-release:

```console
$ uv python install 3.15.0a2
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.15.0a2 in [TIME]
 + cpython-3.15.0a2-[PLATFORM]
```

## Find prerelease

<!-- Derived from [`python_install::python_find_prerelease`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2481-L2544) -->

Finding pre-release Python versions.

Install pre-releases:

```console
$ uv python install 3.15
success: true
exit_code: 0
```

```console
$ uv python install 3.15.0a2
success: true
exit_code: 0
```

Find without opt-in when no stable release is installed:

```console
$ uv python find 3.15
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.15.[X]-[PLATFORM]/[INSTALL-BIN]/[PYTHON]

----- stderr -----
```

Pre-releases match >= requests:

```console
$ uv python find ">=3.15"
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.15.[X]-[PLATFORM]/[INSTALL-BIN]/[PYTHON]

----- stderr -----
```

Pre-releases match major version:

```console
$ uv python find 3
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.15.[X]-[PLATFORM]/[INSTALL-BIN]/[PYTHON]

----- stderr -----
```

Stable versions are preferred once installed:

```console
$ uv python install 3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.[X] in [TIME]
 + cpython-3.13.[X]-[PLATFORM] (python3.13)
```

Now major version finds stable:

```console
$ uv python find 3
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.[X]-[PLATFORM]/[INSTALL-BIN]/[PYTHON]

----- stderr -----
```

## Cached

<!-- Derived from [`python_install::python_install_cached`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2550-L2640) -->

Python downloads are cached in UV_PYTHON_CACHE_DIR.

Install with cache:

```console
$ UV_PYTHON_CACHE_DIR=[TEMP_DIR]/python-cache uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
```

No-op when already installed:

```console
$ UV_PYTHON_CACHE_DIR=[TEMP_DIR]/python-cache uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python is already installed. Use `uv python install <request>` to install another version.
```

Uninstall:

```console
$ uv python uninstall 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.14
Uninstalled Python 3.14.2 in [TIME]
 - cpython-3.14.2-[PLATFORM] (python3.14)
```

Cached archive can be installed offline:

```console
$ UV_PYTHON_CACHE_DIR=[TEMP_DIR]/python-cache uv python install --offline
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
```

Uncached version cannot be installed offline:

```console
$ UV_PYTHON_CACHE_DIR=[TEMP_DIR]/python-cache uv python install 3.12 --offline
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: Failed to install cpython-3.12.12-[PLATFORM]
  Caused by: An offline Python installation was requested, but cpython-3.12.[X]-[WILDCARD]-[PLATFORM].tar.gz) is missing in python-cache
```

## No cache

<!-- Derived from [`python_install::python_install_no_cache`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2644-L2751) -->

Python installation without cache directory.

Install latest version:

```console
$ uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
```

No-op when already installed:

```console
$ uv python install
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python is already installed. Use `uv python install <request>` to install another version.
```

No-op when requested version is already installed:

```console
$ uv python install 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.14 is already installed
```

Reinstall with --reinstall:

```console
$ uv python install 3.14 --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 ~ cpython-3.14.2-[PLATFORM] (python3.14)
```

Uninstall requires targets:

```console
$ uv python uninstall
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the following required arguments were not provided:
  <TARGETS>...

Usage: uv python uninstall --install-dir <INSTALL_DIR> <TARGETS>...

For more information, try '--help'.
```

Uninstall specific version:

```console
$ uv python uninstall 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.14
Uninstalled Python 3.14.2 in [TIME]
 - cpython-3.14.2-[PLATFORM] (python3.14)
```

Uncached version cannot be installed offline:

```console
$ uv python install 3.12 --offline
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: Failed to install cpython-3.12.12-[PLATFORM]
  Caused by: Failed to download https://github.com/astral-sh/python-build-standalone/releases/download/[WILDCARD]/cpython-3.12.[X]-[WILDCARD]-[PLATFORM].tar.gz
  Caused by: Network connectivity is disabled, but the requested data wasn't found in the cache for: `https://github.com/astral-sh/python-build-standalone/releases/download/[WILDCARD]/cpython-3.12.[X]-[WILDCARD]-[PLATFORM].tar.gz`
```

Uninstall removes all executables:

```console
$ uv python uninstall --all
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python installations
Uninstalled Python 3.14.[X] in [TIME]
 - cpython-3.14.[X]-[PLATFORM] (python, python3, python3.14)
```

Default install without version installs latest:

```console
$ uv python install --default --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.[X] in [TIME]
 + cpython-3.14.[X]-[PLATFORM] (python, python3, python3.14)
```

Uninstall:

```console
$ uv python uninstall 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.14
Uninstalled Python 3.14.[X] in [TIME]
 - cpython-3.14.[X]-[PLATFORM] (python, python3, python3.14)
```

Cannot use --default with multiple targets:

```console
$ uv python install --preview 3.12 3.14 --default
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The `--default` flag cannot be used with multiple targets
```

Install 3.12 as default:

```console
$ uv python install --preview 3.12 --default
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.[X] in [TIME]
 + cpython-3.12.[X]-[PLATFORM] (python, python3, python3.12)
```

## Emulated macOS

<!-- Derived from [`python_install::python_install_emulated_macos`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2755-L2837) -->

```toml
# mdtest

[environment]
target-os = "macos"
# Note: This test is for aarch64 macOS with Rosetta, but mdtest doesn't support architecture filtering
```

Install and use x86_64 Python on aarch64 macOS via Rosetta emulation.

Before installation, list shows only native download:

```console
$ uv python list 3.13
success: true
exit_code: 0
----- stdout -----
cpython-3.13.11-macos-aarch64-none    <download available>

----- stderr -----
```

Install x86_64 version:

```console
$ uv python install 3.13-x86_64
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.11 in [TIME]
 + cpython-3.13.11-macos-x86_64-none (python3.13)
```

Discoverable with find:

```console
$ uv python find 3.13
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.11-macos-x86_64-none/bin/python3.13

----- stderr -----
```

Included in list:

```console
$ uv python list 3.13
success: true
exit_code: 0
----- stdout -----
cpython-3.13.11-macos-aarch64-none    <download available>
cpython-3.13.11-macos-x86_64-none     managed/cpython-3.13.11-macos-x86_64-none/bin/python3.13

----- stderr -----
```

Install aarch64 version:

```console
$ uv python install 3.13-aarch64
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.11 in [TIME]
 + cpython-3.13.11-macos-aarch64-none
```

Native version is preferred:

```console
$ uv python find 3.13
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.11-macos-aarch64-none/bin/python3.13

----- stderr -----
```

## Emulated Windows x86 on x64

<!-- Derived from [`python_install::python_install_emulated_windows_x86_on_x64`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2841-L2908) -->

```toml
# mdtest

[environment]
target-os = "windows"
# Note: This test is for x86_64 Windows, but mdtest doesn't support architecture filtering
```

Install and use x86 Python on x64 Windows.

Before installation, list shows only native download:

```console
$ uv python list 3.13
success: true
exit_code: 0
----- stdout -----
cpython-3.13.11-windows-x86_64-none    <download available>

----- stderr -----
```

Install x86 version:

```console
$ uv python install 3.13-x86
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.11 in [TIME]
 + cpython-3.13.11-windows-x86-none (python3.13)
```

Discoverable with find:

```console
$ uv python find 3.13
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.11-windows-x86-none/python

----- stderr -----
```

Included in list:

```console
$ uv python list 3.13
success: true
exit_code: 0
----- stdout -----
cpython-3.13.11-windows-x86_64-none    <download available>
cpython-3.13.11-windows-x86-none       managed/cpython-3.13.11-windows-x86-none/python

----- stderr -----
```

Install x86_64 version:

```console
$ uv python install 3.13-x86_64
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.11 in [TIME]
 + cpython-3.13.11-windows-x86_64-none
```

Native version is preferred:

```console
$ uv python find 3.13
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.13.11-windows-x86_64-none/python

----- stderr -----
```

## Transparent patch upgrade uv venv

<!-- Derived from [`python_install::install_transparent_patch_upgrade_uv_venv`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L2912-L3001) -->

```toml
# mdtest

[environment]
python-versions = ["3.13"]
```

Virtual environments track the latest installed patch version.

Install lower patch version:

```console
$ uv python install --preview 3.12.9
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.9 in [TIME]
 + cpython-3.12.9-[PLATFORM] (python3.12)
```

Create virtual environment:

```console
$ uv venv -p 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.9
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Verify version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.9

----- stderr -----
```

Install higher patch version:

```console
$ uv python install --preview 3.12.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.11 in [TIME]
 + cpython-3.12.11-[PLATFORM] (python3.12)
```

Virtual environment reflects higher version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.11

----- stderr -----
```

Install lower patch version:

```console
$ uv python install --preview 3.12.8
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.8 in [TIME]
 + cpython-3.12.8-[PLATFORM]
```

Virtual environment reflects highest version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.11

----- stderr -----
```

## Install multiple patches

<!-- Derived from [`python_install::install_multiple_patches`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3006-L3092) -->

When installing multiple patches simultaneously, virtual environments use the highest.

Install patches in ascending order:

```console
$ uv python install --preview 3.12.9 3.12.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.12.9-[PLATFORM]
 + cpython-3.12.11-[PLATFORM] (python3.12)
```

Create virtual environment:

```console
$ uv venv -p 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.11
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Virtual environment on highest patch:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.11

----- stderr -----
```

Remove virtual environment:

```console
$ rm -rf .venv
```

Install patches in descending order:

```console
$ uv python install --preview 3.10.17 3.10.16
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.10.16-[PLATFORM]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create virtual environment on 3.10:

```console
$ uv venv -p 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.10.17
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Virtual environment on highest patch:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.17

----- stderr -----
```

## Uninstall highest patch

<!-- Derived from [`python_install::uninstall_highest_patch`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3097-L3165) -->

After uninstalling the highest patch, virtual environments point to the next highest.

Install patches:

```console
$ uv python install --preview 3.12.11 3.12.9 3.12.8
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 3 versions in [TIME]
 + cpython-3.12.8-[PLATFORM]
 + cpython-3.12.9-[PLATFORM]
 + cpython-3.12.11-[PLATFORM] (python3.12)
```

Create virtual environment:

```console
$ uv venv -p 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.11
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Verify version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.11

----- stderr -----
```

Uninstall highest patch:

```console
$ uv python uninstall --preview 3.12.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.12.11
Uninstalled Python 3.12.11 in [TIME]
 - cpython-3.12.11-[PLATFORM] (python3.12)
```

Virtual environment on next highest:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.9

----- stderr -----
```

## No transparent upgrade with venv patch

<!-- Derived from [`python_install::install_no_transparent_upgrade_with_venv_patch_specification`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3171-L3236) -->

```toml
# mdtest

[environment]
python-versions = ["3.13"]
```

Virtual environments record minor versions only; patch specifications don't prevent transparent
upgrades.

Install patch version:

```console
$ uv python install --preview 3.12.9
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.9 in [TIME]
 + cpython-3.12.9-[PLATFORM] (python3.12)
```

Create virtual environment with patch version:

```console
$ uv venv -p 3.12.9
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.9
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Verify version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.9

----- stderr -----
```

Install higher patch:

```console
$ uv python install --preview 3.12.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.11 in [TIME]
 + cpython-3.12.11-[PLATFORM] (python3.12)
```

Virtual environment does NOT transparently upgrade (still on 3.12.9):

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.9

----- stderr -----
```

## Transparent patch upgrade venv module

<!-- Derived from [`python_install::install_transparent_patch_upgrade_venv_module`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3241-L3314) -->

Virtual environments created with the venv module track the latest patch version.

Install patch version:

```console
$ uv python install --preview 3.12.9
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.9 in [TIME]
 + cpython-3.12.9-[PLATFORM] (python3.12)
```

Verify version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.9

----- stderr -----
```

Create virtual environment using venv module:

```console
$ uv run python -m venv .venv --without-pip
success: true
exit_code: 0
```

Verify version in venv:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.9

----- stderr -----
```

Install higher patch:

```console
$ uv python install --preview 3.12.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.11 in [TIME]
 + cpython-3.12.11-[PLATFORM] (python3.12)
```

Virtual environment reflects highest patch:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.11

----- stderr -----
```

## Install lower patch automatically

<!-- Derived from [`python_install::install_lower_patch_automatically`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3319-L3386) -->

Automatically installing a lower patch version should not downgrade virtual environments.

Install higher patch:

```console
$ uv python install --preview 3.12.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.11 in [TIME]
 + cpython-3.12.11-[PLATFORM] (python3.12)
```

Create virtual environment:

```console
$ uv venv -p 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.11
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Initialize project with lower patch:

```console
$ uv init -p 3.12.9 proj
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `proj` at `[TEMP_DIR]/proj`
```

Create venv to trigger automatic installation of lower patch:

```console
$ uv venv --directory proj -p 3.12.9
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.9
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Original venv still on higher patch:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.11

----- stderr -----
```

## Uninstall last patch

<!-- Derived from [`python_install::uninstall_last_patch`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3389-L3469) -->

```toml
# mdtest

[environment]
target-family = "unix"
```

Uninstalling the last patch version breaks the virtual environment symlink.

Install patch:

```console
$ uv python install --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create venv:

```console
$ uv venv -p 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.10.17
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Verify version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.17

----- stderr -----
```

Uninstall last patch:

```console
$ uv python uninstall --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Searching for Python versions matching: Python 3.10.17
Uninstalled Python 3.10.17 in [TIME]
 - cpython-3.10.17-[PLATFORM] (python3.10)
```

Broken symlink error:

```console
$ uv run python --version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to inspect Python interpreter from active virtual environment at `.venv/[BIN]/python`
  Caused by: Broken symlink at `.venv/[BIN]/python`, was the underlying Python interpreter removed?

hint: Consider recreating the environment (e.g., with `uv venv`)
```

## Pyodide

<!-- Derived from [`python_install::python_install_pyodide`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3473-L3640) -->

```toml
# mdtest

[environment]
target-family = "unix"
```

Install and use Pyodide Python for WebAssembly.

Install via full key:

```console
$ uv python install cpython-3.13.2-emscripten-wasm32-musl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.2 in [TIME]
 + pyodide-3.13.2-emscripten-wasm32-musl (python3.13)
```

The executable should work:

```console
$ python3.13 -c "import subprocess; print('hello world')"
success: true
exit_code: 0
----- stdout -----
hello world

----- stderr -----
```

Find Pyodide interpreter:

```console
$ uv python find cpython-3.13.2-emscripten-wasm32-musl
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/pyodide-3.13.2-emscripten-wasm32-musl/python

----- stderr -----
```

Create venv with Pyodide:

```console
$ uv venv --python cpython-3.13.2-emscripten-wasm32-musl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.13.2
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Run Python in venv:

```console
$ uv run python -c "import subprocess; print('hello world')"
success: true
exit_code: 0
----- stdout -----
hello world

----- stderr -----
```

Clean up:

```console
$ uv python uninstall --all
success: true
exit_code: 0
```

```console
$ rm -rf .venv
```

Install via pyodide shorthand:

```console
$ uv python install pyodide
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.2 in [TIME]
 + pyodide-3.13.2-emscripten-wasm32-musl (python3.13)
```

Clean up:

```console
$ uv python uninstall --all
success: true
exit_code: 0
```

Install via pyodide@version:

```console
$ uv python install pyodide@3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.2 in [TIME]
 + pyodide-3.13.2-emscripten-wasm32-musl (python3.13)
```

Find via pyodide:

```console
$ uv python find pyodide
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/pyodide-3.13.2-emscripten-wasm32-musl/python

----- stderr -----
```

Find without request fails:

```console
$ uv python find
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found in virtual environments, managed installations, or search path
```

Find with cpython fails:

```console
$ uv python find cpython
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for CPython in virtual environments, managed installations, or search path
```

Install CPython:

```console
$ uv python install cpython
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
```

CPython is preferred by default:

```console
$ uv python find any
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.14.2-[PLATFORM]/bin/python3.14

----- stderr -----
```

Unless we request pyodide:

```console
$ uv python find pyodide
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/pyodide-3.13.2-emscripten-wasm32-musl/python

----- stderr -----
```

## Build version

<!-- Derived from [`python_install::python_install_build_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3643-L3723) -->

Install Python with a specific build version.

Install with build version:

```console
$ UV_PYTHON_CPYTHON_BUILD=20240814 uv python install 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.5 in [TIME]
 + cpython-3.12.5-[PLATFORM] (python3.12)
```

Find with matching build:

```console
$ UV_PYTHON_CPYTHON_BUILD=20240814 uv python find 3.12
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/cpython-3.12.5-[PLATFORM]/[INSTALL-BIN]/[PYTHON]

----- stderr -----
```

Find with mismatched build fails:

```console
$ UV_PYTHON_CPYTHON_BUILD=99999999 uv python find 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.12 in [PYTHON SOURCES]
```

Install with invalid build fails:

```console
$ UV_PYTHON_CPYTHON_BUILD=99999999 uv python install 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No download found for request: cpython-3.12-[PLATFORM]
```

Install patch with wrong build fails:

```console
$ UV_PYTHON_CPYTHON_BUILD=20250814 uv python install 3.12.10
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No download found for request: cpython-3.12.10-[PLATFORM]
```

## Build version PyPy

<!-- Derived from [`python_install::python_install_build_version_pypy`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3726-L3794) -->

Install PyPy with a specific build version.

Install with build version:

```console
$ UV_PYTHON_PYPY_BUILD=7.3.19 uv python install pypy3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.16 in [TIME]
 + pypy-3.10.16-[PLATFORM] (python3.10)
```

Find with matching build:

```console
$ UV_PYTHON_PYPY_BUILD=7.3.19 uv python find pypy3.10
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/managed/pypy-3.10.16-[PLATFORM]/[INSTALL-BIN]/[PYPY]

----- stderr -----
```

Find with mismatched build fails:

```console
$ UV_PYTHON_PYPY_BUILD=99.99.99 uv python find pypy3.10
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for PyPy 3.10 in [PYTHON SOURCES]
```

Install with invalid build fails:

```console
$ UV_PYTHON_PYPY_BUILD=99.99.99 uv python install pypy3.10
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No download found for request: pypy-3.10-[PLATFORM]
```

## Upgrade

<!-- Derived from [`python_install::python_install_upgrade`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3797-L3920) -->

The --upgrade flag upgrades to the latest patch version.

Upgrade without version installs latest:

```console
$ uv python install --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
```

Upgrade again is no-op:

```console
$ uv python install --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
The default Python installation is already on the latest supported patch release. Use `uv python install <request>` to install another version.
```

Install earlier patch:

```console
$ uv python install 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Upgrade to latest patch:

```console
$ uv python install --upgrade 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

Patch version with upgrade fails:

```console
$ uv python install --upgrade 3.11.4
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: `uv python install --upgrade` only accepts minor versions, got: 3.11.4
```

Upgrade uninstalled version installs it:

```console
$ uv python install --upgrade 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.11.14 in [TIME]
 + cpython-3.11.14-[PLATFORM] (python3.11)
```

Upgrade again is no-op:

```console
$ uv python install --upgrade 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.11 is already on the latest supported patch release
```

Install outdated version:

```console
$ uv python install 3.9.5
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.9.5 in [TIME]
 + cpython-3.9.5-[PLATFORM] (python3.9)
```

Upgrade different version doesn't affect it:

```console
$ uv python install --upgrade 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.11 is already on the latest supported patch release
```

Upgrade multiple already satisfied:

```console
$ uv python install --upgrade 3.10 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
All requested versions already on latest supported patch release
```

Mix satisfied, unsatisfied, and missing:

```console
$ uv python install --upgrade 3.9 3.10 3.11 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.9.25-[PLATFORM] (python3.9)
 + cpython-3.12.12-[PLATFORM] (python3.12)
```

## Upgrade version file

<!-- Derived from [`python_install::python_install_upgrade_version_file`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3923-L3968) -->

Upgrade respects .python-version file.

Pin to minor version:

```console
$ uv python pin 3.13
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.13`

----- stderr -----
```

Upgrade without version uses pin:

```console
$ uv python install --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.11 in [TIME]
 + cpython-3.13.11-[PLATFORM] (python3.13)
```

Upgrade again is no-op:

```console
$ uv python install --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.13 is already on the latest supported patch release
```

Pin to patch version:

```console
$ uv python pin 3.12.4
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.12.4`

----- stderr -----
```

Upgrade with patch pin fails:

```console
$ uv python install --upgrade
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: `uv python install --upgrade` only accepts minor versions, got: 3.12.4

hint: The version request came from a `.python-version` file; change the patch version in the file to upgrade instead
```

## ARMv7

<!-- Derived from [`python_install::python_install_armv7`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L3971-L4001) -->

ARMv7 platform support.

Musl on ARMv7 is not supported:

```console
$ uv python install cpython-3.12.12-linux-armv7-musl
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: uv does not yet provide musl Python distributions on armv7.
```

GNUeabi on ARMv7 works:

```console
$ uv python install cpython-3.12.12-linux-armv7-gnueabi
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.12 in [TIME]
 + cpython-3.12.12-[PLATFORM] (python3.12)
```

## Compile bytecode

<!-- Derived from [`python_install::python_install_compile_bytecode`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L4004-L4087) -->

Compile Python bytecode (.pyc files) during installation.

Install with bytecode compilation:

```console
$ uv python install --compile-bytecode 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
Bytecode compiled [COUNT] files in [TIME]
```

Rerun compilation on already installed:

```console
$ uv python install --compile-bytecode 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.14 is already installed
Bytecode compiled [COUNT] files in [TIME]
```

Reinstall with bytecode compilation:

```console
$ uv python install --reinstall --compile-bytecode 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 ~ cpython-3.14.2-[PLATFORM] (python3.14)
Bytecode compiled [COUNT] files in [TIME]
```

## Compile bytecode existing

<!-- Derived from [`python_install::python_install_compile_bytecode_existing`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L4090-L4119) -->

Compile bytecode for existing installation.

Fresh install without compilation:

```console
$ uv python install 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
```

Compile later:

```console
$ uv python install --compile-bytecode 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Python 3.14 is already installed
Bytecode compiled [COUNT] files in [TIME]
```

## Compile bytecode upgrade

<!-- Derived from [`python_install::python_install_compile_bytecode_upgrade`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L4122-L4152) -->

Upgrade compiles bytecode.

Install old version:

```console
$ uv python install 3.14.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.0 in [TIME]
 + cpython-3.14.0-[PLATFORM] (python3.14)
```

Upgrade with bytecode compilation:

```console
$ uv python install --upgrade --compile-bytecode 3.14
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.14.2 in [TIME]
 + cpython-3.14.2-[PLATFORM] (python3.14)
Bytecode compiled [COUNT] files in [TIME]
```

## Compile bytecode multiple

<!-- Derived from [`python_install::python_install_compile_bytecode_multiple`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L4155-L4176) -->

Install and compile multiple versions.

Install multiple versions with compilation:

```console
$ uv python install --compile-bytecode 3.14 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.12.12-[PLATFORM] (python3.12)
 + cpython-3.14.2-[PLATFORM] (python3.14)
Bytecode compiled [COUNT] files in [TIME]
```

## Compile bytecode Pyodide

<!-- Derived from [`python_install::python_install_compile_bytecode_pyodide`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L4180-L4205) -->

```toml
# mdtest

[environment]
target-family = "unix"
```

Pyodide skips bytecode compilation.

Install Pyodide with compilation request:

```console
$ uv python install --compile-bytecode cpython-3.13.2-emscripten-wasm32-musl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.13.2 in [TIME]
 + pyodide-3.13.2-emscripten-wasm32-musl (python3.13)
No compatible versions to bytecode compile (skipped 1)
```

## Compile bytecode GraalPy

<!-- Derived from [`python_install::python_install_compile_bytecode_graalpy`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L4208-L4228) -->

GraalPy bytecode compilation works.

Install GraalPy with compilation:

```console
$ uv python install --compile-bytecode graalpy-3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.0 in [TIME]
 + graalpy-3.12.0-[PLATFORM] (python3.12)
Bytecode compiled [COUNT] files in [TIME]
```

## Compile bytecode PyPy

<!-- Derived from [`python_install::python_install_compile_bytecode_pypy`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_install.rs#L4231-L4251) -->

PyPy bytecode compilation works.

Install PyPy with compilation:

```console
$ uv python install --compile-bytecode pypy-3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.11.13 in [TIME]
 + pypy-3.11.13-[PLATFORM] (python3.11)
Bytecode compiled [COUNT] files in [TIME]
```
