# Python Upgrade

Tests for upgrading Python installations to the latest patch version.

## Upgrade

<!-- from python_upgrade.rs::python_upgrade -->

Install an earlier patch version:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Upgrade to latest patch version:

```console
$ uv python upgrade --preview 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

Check the upgraded version:

```console
$ uv python list
success: true
exit_code: 0
----- stdout -----
cpython-3.10.19-[PLATFORM]       python3.10  [MANAGED_DIR]/cpython-3.10.19-[PLATFORM]/[BIN]/python3.10

----- stderr -----
```

## Without version

<!-- from python_upgrade.rs::python_upgrade_without_version -->

Install multiple earlier patch versions:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.10.17 3.11.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
 + cpython-3.11.10-[PLATFORM] (python3.11)
```

Upgrade all installed versions without specifying versions:

```console
$ uv python upgrade --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed 2 versions in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
 + cpython-3.11.11-[PLATFORM] (python3.11)
```

## Transparent from venv

<!-- from python_upgrade.rs::python_upgrade_transparent_from_venv -->

Virtual environments should transparently track the latest installed patch version.

Install an earlier patch version:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create first virtual environment:

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

Check version in first venv:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.17

----- stderr -----
```

Create second virtual environment:

```console
$ uv venv -p 3.10 .venv2
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.10.17
Creating virtual environment at: .venv2
Activate with: source .venv2/[BIN]/activate
```

Check version in second venv:

```console
$ uv run python --version
success: true
exit_code: 0
env:
  VIRTUAL_ENV: .venv2
----- stdout -----
Python 3.10.17

----- stderr -----
```

Upgrade patch version:

```console
$ uv python upgrade --preview 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

First virtual environment should reflect upgraded patch:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.19

----- stderr -----
```

Second virtual environment should also reflect upgraded patch:

```console
$ uv run python --version
success: true
exit_code: 0
env:
  VIRTUAL_ENV: .venv2
----- stdout -----
Python 3.10.19

----- stderr -----
```

## Transparent from venv preview

<!-- from python_upgrade.rs::python_upgrade_transparent_from_venv_preview -->

Installing Python in preview mode should not prevent virtual environments from transparently
upgrading.

Install an earlier patch version using --preview:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install 3.10.17 --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create a virtual environment:

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

Check version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.17

----- stderr -----
```

Upgrade patch version:

```console
$ uv python upgrade --preview 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

Virtual environment should reflect upgraded patch:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.19

----- stderr -----
```

## Ignored with python pin

<!-- from python_upgrade.rs::python_upgrade_ignored_with_python_pin -->

Pinned Python versions prevent transparent upgrades.

Install an earlier patch version:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create a virtual environment:

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.10.17
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Pin to the older patch version:

```console
$ uv python pin 3.10.17
success: true
exit_code: 0
----- stdout -----
Pinned `.python-version` to `3.10.17`

----- stderr -----
```

Upgrade patch version:

```console
$ uv python upgrade --preview 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

Virtual environment should continue to respect pinned patch version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.17

----- stderr -----
```

## No transparent upgrade with venv patch

<!-- from python_upgrade.rs::python_no_transparent_upgrade_with_venv_patch_specification -->

Virtual environments record patch versions. `uv venv -p 3.x.y` will prevent transparent upgrades.

Install an earlier patch version:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create a virtual environment with a patch version:

```console
$ uv venv -p 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.10.17
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Check version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.17

----- stderr -----
```

Upgrade patch version:

```console
$ uv python upgrade --preview 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

The virtual environment Python version remains the same:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.10.17

----- stderr -----
```

## Transparent upgrade venv venv

<!-- from python_upgrade.rs::python_transparent_upgrade_venv_venv -->

Transparent upgrades should work for virtual environments created within virtual environments.

Install an earlier patch version:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create an initial virtual environment:

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

Create a new virtual environment from within a virtual environment:

```console
$ uv venv .venv2 -p .venv/[BIN]/python
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.10.17 interpreter at: .venv/[BIN]/python
Creating virtual environment at: .venv2
Activate with: source .venv2/[BIN]/activate
```

Check version from within second virtual environment:

```console
$ uv run python --version
success: true
exit_code: 0
env:
  VIRTUAL_ENV: .venv2
----- stdout -----
Python 3.10.17

----- stderr -----
```

Upgrade patch version:

```console
$ uv python upgrade --preview 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

Should have transparently upgraded in second virtual environment:

```console
$ uv run python --version
success: true
exit_code: 0
env:
  VIRTUAL_ENV: .venv2
----- stdout -----
Python 3.10.19

----- stderr -----
```

## Transparent from venv module

<!-- from python_upgrade.rs::python_upgrade_transparent_from_venv_module -->

Transparent upgrades should work for virtual environments created using the venv module.

Install earlier patch version:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.12.9
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.9 in [TIME]
 + cpython-3.12.9-[PLATFORM] (python3.12)
```

Create a virtual environment using venv module:

```console
$ uv run python -m venv .venv --without-pip
success: true
exit_code: 0
env:
  PATH: [TEMP_DIR]/bin
----- stdout -----

----- stderr -----
```

Check version:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.9

----- stderr -----
```

Upgrade patch version:

```console
$ uv python upgrade --preview 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.12 in [TIME]
 + cpython-3.12.12-[PLATFORM] (python3.12)
```

Virtual environment should reflect upgraded patch:

```console
$ uv run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.12

----- stderr -----
```

## Transparent from venv module in venv

<!-- from python_upgrade.rs::python_upgrade_transparent_from_venv_module_in_venv -->

Transparent Python upgrades should work in environments created using the venv module within an
existing virtual environment.

Install earlier patch version:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install --preview 3.10.17
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.17 in [TIME]
 + cpython-3.10.17-[PLATFORM] (python3.10)
```

Create first virtual environment:

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

Create a virtual environment using venv module from within the first virtual environment:

```console
$ uv run python -m venv .venv2 --without-pip
success: true
exit_code: 0
env:
  PATH: [TEMP_DIR]/bin
----- stdout -----

----- stderr -----
```

Check version within second virtual environment:

```console
$ uv run python --version
success: true
exit_code: 0
env:
  VIRTUAL_ENV: .venv2
----- stdout -----
Python 3.10.17

----- stderr -----
```

Upgrade patch version:

```console
$ uv python upgrade --preview 3.10
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.10.19 in [TIME]
 + cpython-3.10.19-[PLATFORM] (python3.10)
```

Second virtual environment should reflect upgraded patch:

```console
$ uv run python --version
success: true
exit_code: 0
env:
  VIRTUAL_ENV: .venv2
----- stdout -----
Python 3.10.19

----- stderr -----
```

## Force install

<!-- from python_upgrade.rs::python_upgrade_force_install -->

`uv python upgrade` will warn if trying to install over a non-managed interpreter.

Create a non-managed python3.12 executable:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ touch [BIN]/python3.12
success: true
exit_code: 0
```

Try to upgrade with a non-managed interpreter installed in bin:

```console
$ uv python upgrade --preview 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Executable already exists at `[BIN]/python3.12` but is not managed by uv; use `uv python install 3.12 --force` to replace it
Installed Python 3.12.12 in [TIME]
 + cpython-3.12.12-[PLATFORM]
```

Force the bin install:

```console
$ uv python install 3.12 --force --preview 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.12.12 in [TIME]
 + cpython-3.12.12-[PLATFORM] (python3.12)
```

## Implementation

<!-- from python_upgrade.rs::python_upgrade_implementation -->

Don't install CPython when upgrading if only alternative implementations are installed.

Install pypy:

```toml
[test]
required-features = ["python-managed"]
```

```console
$ uv python install pypy@3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Installed Python 3.11.10 in [TIME]
 + pypy-3.11.10-[PLATFORM] (python3.11, pypy3.11)
```

Run the upgrade, we should not install cpython:

```console
$ uv python upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: `uv python upgrade` is experimental and may change without warning. Pass `--preview-features python-upgrade` to disable this warning
All versions already on latest supported patch release
```
