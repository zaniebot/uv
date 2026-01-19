# Workspace Sync

Tests for syncing workspace projects with `uv sync`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Just a project (no workspace)

<!-- Derived from [`workspace::test_albatross_just_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

A simple project with no workspace members syncs normally.

```toml
# file: pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=2,<3"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: src/albatross/__init__.py

def fly():
    pass
```

```python
# file: check_installed_albatross.py

from albatross import fly

fly()
print("Success")
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + albatross==0.1.0 (from file://[TEMP_DIR]/)
 + iniconfig==2.0.0
```

Syncing again is a no-op:

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Audited 2 packages in [TIME]
```

## Root workspace

<!-- Derived from [`workspace::test_albatross_root_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

A root workspace has a package at the workspace root plus additional members.

```toml
# file: pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["bird-feeder", "iniconfig>=2,<3"]

[tool.uv.sources]
bird-feeder = { workspace = true }

[tool.uv.workspace]
members = ["packages/*"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: src/albatross/__init__.py

import iniconfig
from bird_feeder import use


def fly():
    pass


if __name__ == "__main__":
    print("Caw")
    use()
```

```python
# file: check_installed_albatross.py

from albatross import fly

fly()
print("Success")
```

```toml
# file: packages/bird-feeder/pyproject.toml

[project]
name = "bird-feeder"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["iniconfig>=2,<3", "seeds"]

[tool.uv.sources]
seeds = { workspace = true }

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/bird-feeder/src/bird_feeder/__init__.py

import iniconfig


def use():
    print("squirrel")
```

```python
# file: packages/bird-feeder/check_installed_bird_feeder.py

from bird_feeder import use

try:
    from albatross import fly

    raise RuntimeError("albatross installed")
except ModuleNotFoundError:
    pass

print("Success")
```

```toml
# file: packages/seeds/pyproject.toml

[project]
name = "seeds"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["idna==3.6"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/seeds/src/seeds/__init__.py

import idna


def seeds():
    print("sunflower")
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + albatross==0.1.0 (from file://[TEMP_DIR]/)
 + bird-feeder==1.0.0 (from file://[TEMP_DIR]/packages/bird-feeder)
 + idna==3.6
 + iniconfig==2.0.0
 + seeds==1.0.0 (from file://[TEMP_DIR]/packages/seeds)
```

Syncing again is a no-op:

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Audited 5 packages in [TIME]
```

## Root workspace from member

<!-- Derived from [`workspace::test_albatross_root_workspace_bird_feeder`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Syncing from a member directory installs that member's dependencies.

```toml
# file: pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["bird-feeder", "iniconfig>=2,<3"]

[tool.uv.sources]
bird-feeder = { workspace = true }

[tool.uv.workspace]
members = ["packages/*"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: src/albatross/__init__.py

import iniconfig
from bird_feeder import use


def fly():
    pass
```

```toml
# file: packages/bird-feeder/pyproject.toml

[project]
name = "bird-feeder"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["iniconfig>=2,<3", "seeds"]

[tool.uv.sources]
seeds = { workspace = true }

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/bird-feeder/src/bird_feeder/__init__.py

import iniconfig


def use():
    print("squirrel")
```

```python
# file: packages/bird-feeder/check_installed_bird_feeder.py

from bird_feeder import use

try:
    from albatross import fly

    raise RuntimeError("albatross installed")
except ModuleNotFoundError:
    pass

print("Success")
```

```toml
# file: packages/seeds/pyproject.toml

[project]
name = "seeds"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["idna==3.6"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/seeds/src/seeds/__init__.py

import idna


def seeds():
    print("sunflower")
```

Running from the member directory creates the venv at the workspace root:

```console working-dir="packages/bird-feeder"
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + bird-feeder==1.0.0 (from file://[TEMP_DIR]/packages/bird-feeder)
 + idna==3.6
 + iniconfig==2.0.0
 + seeds==1.0.0 (from file://[TEMP_DIR]/packages/seeds)
```

Syncing again is a no-op:

```console working-dir="packages/bird-feeder"
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Audited 4 packages in [TIME]
```

## Virtual workspace

<!-- Derived from [`workspace::test_albatross_virtual_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

A virtual workspace has no package at the root, only members.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["packages/*"]
```

```toml
# file: packages/albatross/pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["bird-feeder", "iniconfig>=2,<3"]

[tool.uv.sources]
bird-feeder = { workspace = true }

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/albatross/src/albatross/__init__.py

import iniconfig
from bird_feeder import use


def fly():
    pass
```

```python
# file: packages/albatross/check_installed_albatross.py

from albatross import fly

fly()
print("Success")
```

```toml
# file: packages/bird-feeder/pyproject.toml

[project]
name = "bird-feeder"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["anyio>=4.3.0,<5", "seeds"]

[tool.uv.sources]
seeds = { workspace = true }

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/bird-feeder/src/bird_feeder/__init__.py

import anyio


def use():
    print("squirrel")
```

```python
# file: packages/bird-feeder/check_installed_bird_feeder.py

from bird_feeder import use

try:
    from albatross import fly

    raise RuntimeError("albatross installed")
except ModuleNotFoundError:
    pass

print("Success")
```

```toml
# file: packages/seeds/pyproject.toml

[project]
name = "seeds"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["idna==3.6"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/seeds/src/seeds/__init__.py

import idna


def seeds():
    print("sunflower")
```

Syncing from a member directory:

```console working-dir="packages/bird-feeder"
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + anyio==4.3.0
 + bird-feeder==1.0.0 (from file://[TEMP_DIR]/packages/bird-feeder)
 + idna==3.6
 + seeds==1.0.0 (from file://[TEMP_DIR]/packages/seeds)
 + sniffio==1.3.1
```

Syncing again is a no-op:

```console working-dir="packages/bird-feeder"
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Audited 5 packages in [TIME]
```

## Project in excluded directory

<!-- Derived from [`workspace::test_albatross_project_in_excluded`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Projects in excluded directories are treated as standalone projects.

```toml
# file: pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=2,<3"]

[tool.uv.workspace]
members = ["packages/*"]
exclude = ["excluded/*"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: src/albatross/__init__.py

import iniconfig
from bird_feeder import use

print("Caw")
use()
```

```toml
# file: excluded/bird-feeder/pyproject.toml

[project]
name = "bird-feeder"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=2,<3"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: excluded/bird-feeder/src/bird_feeder/__init__.py

import iniconfig


def use():
    print("squirrel")
```

```python
# file: excluded/bird-feeder/check_installed_bird_feeder.py

from bird_feeder import use

try:
    from albatross import fly

    raise RuntimeError("albatross installed")
except ModuleNotFoundError:
    pass

print("Success")
```

```toml
# file: packages/seeds/pyproject.toml

[project]
name = "seeds"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["idna==3.6"]

[tool.uv]
managed = false

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: packages/seeds/src/seeds/__init__.py

import idna


def seeds():
    print("sunflower")
```

Syncing the main workspace:

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + albatross==0.1.0 (from file://[TEMP_DIR]/)
 + iniconfig==2.0.0
```

The excluded project syncs independently with its own venv:

```console working-dir="excluded/bird-feeder"
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: `VIRTUAL_ENV=[VENV]/` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 2 packages in [TIME]
 + bird-feeder==1.0.0 (from file://[TEMP_DIR]/excluded/bird-feeder)
 + iniconfig==2.0.0
```

An unmanaged member fails to sync:

```console working-dir="packages/seeds"
$ uv sync
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The project is marked as unmanaged: `[TEMP_DIR]/packages/seeds`
```

## Example directory as workspace

<!-- Derived from [`workspace::test_albatross_in_examples`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

A project that has example projects in a subdirectory.

```toml
# file: pyproject.toml

[project]
name = "albatross"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=2,<3"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: src/albatross/__init__.py

import iniconfig


def fly():
    pass
```

```python
# file: check_installed_albatross.py

from albatross import fly

try:
    from bird_feeder import use

    raise RuntimeError("bird-feeder installed")
except ModuleNotFoundError:
    pass

fly()
print("Success")
```

```toml
# file: examples/bird-feeder/pyproject.toml

[project]
name = "bird-feeder"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=2,<3"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: examples/bird-feeder/src/bird_feeder/__init__.py

import iniconfig


def use():
    print("squirrel")
```

```python
# file: examples/bird-feeder/check_installed_bird_feeder.py

from bird_feeder import use

try:
    from albatross import fly

    raise RuntimeError("albatross installed")
except ModuleNotFoundError:
    pass

print("Success")
```

Syncing the main project:

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + albatross==0.1.0 (from file://[TEMP_DIR]/)
 + iniconfig==2.0.0
```

The example project syncs independently:

```console working-dir="examples/bird-feeder"
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: `VIRTUAL_ENV=[VENV]/` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 2 packages in [TIME]
 + bird-feeder==1.0.0 (from file://[TEMP_DIR]/examples/bird-feeder)
 + iniconfig==2.0.0
```
