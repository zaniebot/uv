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

## Sync workspace members with transitive dependencies

<!-- Derived from [`sync::sync_workspace_members_with_transitive_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L5095-L5187) -->

Syncing a virtual workspace installs all members and their transitive dependencies.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = [
    "packages/*",
]
```

Create three packages with transitive dependencies pkg-c -> pkg-b -> pkg-a:

```toml
# file: packages/pkg-a/pyproject.toml

[project]
name = "pkg-a"
version = "0.0.1"
requires-python = ">=3.12"
dependencies = ["anyio"]
```

```toml
# file: packages/pkg-b/pyproject.toml

[project]
name = "pkg-b"
version = "0.0.1"
requires-python = ">=3.12"
dependencies = ["pkg-a"]

[tool.uv.sources]
pkg-a = { workspace = true }
```

```toml
# file: packages/pkg-c/pyproject.toml

[project]
name = "pkg-c"
version = "0.0.1"
requires-python = ">=3.12"
dependencies = ["pkg-b"]

[tool.uv.sources]
pkg-b = { workspace = true }
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + pkg-a==0.0.1 (from file://[TEMP_DIR]/packages/pkg-a)
 + pkg-b==0.0.1 (from file://[TEMP_DIR]/packages/pkg-b)
 + sniffio==1.3.1
```

## Sync non-existent extra in workspace member

<!-- Derived from [`sync::sync_non_existent_extra_workspace_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L5190-L5259) -->

Requesting an extra that only exists in a workspace member fails unless syncing that specific
member.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[project.optional-dependencies]
types = ["sniffio>1"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"

[project.optional-dependencies]
async = ["anyio>3"]
```

Lock the workspace:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Requesting an extra that only exists in the child fails:

```console
$ uv sync --extra async
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
error: Extra `async` is not defined in the project's `optional-dependencies` table
```

Unless syncing the child specifically:

```console
$ uv sync --package child --extra async
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

## Sync non-existent extra in virtual workspace

<!-- Derived from [`sync::sync_non_existent_extra_non_project_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L5262-L5341) -->

In a virtual workspace, syncing an extra that exists in any member succeeds.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ["child", "other"]
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"

[project.optional-dependencies]
async = ["anyio>3"]
```

```toml
# file: other/pyproject.toml

[project]
name = "other"
version = "0.1.0"
requires-python = ">=3.12"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Requesting an extra that exists in child succeeds when syncing all members:

```console
$ uv sync --extra async
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

Syncing from the child also succeeds:

```console
$ uv sync --package child --extra async
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Audited 3 packages in [TIME]
```

Syncing from an unrelated member fails:

```console
$ uv sync --package other --extra async
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
error: Extra `async` is not defined in the project's `optional-dependencies` table
```

## Sync with --no-install-workspace

<!-- Derived from [`sync::no_install_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L5440-L5528) -->

The `--no-install-workspace` flag installs dependencies but not the workspace packages themselves.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0", "child"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=1"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: child/src/child/__init__.py
pass
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
```

With `--no-install-workspace`, only dependencies are installed:

```console
$ uv sync --no-install-workspace
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + iniconfig==2.0.0
 + sniffio==1.3.1
```

## Sync workspace with custom environment path

<!-- Derived from [`sync::sync_workspace_custom_environment_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L7051-L7141) -->

Workspace members share the virtual environment at the workspace root, which can be customized.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

Initialize a child member:

```console
$ uv init child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `child` as member of workspace `[TEMP_DIR]/`
Initialized project `child` at `[TEMP_DIR]/child`
```

Syncing creates `.venv` in the workspace root:

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

Syncing from the child uses the workspace root's `.venv`:

```console
$ uv sync
working-dir: child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Uninstalled 1 package in [TIME]
 - iniconfig==2.0.0
```

Custom environment path can be specified:

```console
$ uv sync
env: { UV_PROJECT_ENVIRONMENT = "foo" }
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: foo
Resolved 3 packages in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

## Sync workspace with build-system requires

<!-- Derived from [`sync::build_system_requires_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L8415-L8494) -->

Build system dependencies can reference workspace members.

```toml
# file: backend/pyproject.toml

[project]
name = "backend"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions>=3.10"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: backend/src/backend/__init__.py
def hello() -> str:
    return "Hello, world!"
```

```toml
# file: project/pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=1"]

[build-system]
requires = ["setuptools>=42", "backend==0.1.0"]
build-backend = "setuptools.build_meta"

[tool.uv.workspace]
members = ["../backend"]

[tool.uv.sources]
backend = { workspace = true }
```

```python
# file: project/setup.py
from setuptools import setup

from backend import hello

hello()

setup()
```

```console
$ uv sync
working-dir: project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 4 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + iniconfig==2.0.0
 + project==0.1.0 (from file://[TEMP_DIR]/project)
```

## Toggle workspace editable mode

<!-- Derived from [`sync::toggle_workspace_editable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L14774-L14989) -->

Workspace packages are editable by default but can be toggled to non-editable.

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=1"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child/src/child/__init__.py
pass
```

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

By default, workspace members are editable:

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + child==0.1.0 (from file://[TEMP_DIR]/child)
 + iniconfig==2.0.0
```

Toggle to non-editable:

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true, editable = false }
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 ~ child==0.1.0 (from file://[TEMP_DIR]/child)
```

## Workspace editable conflict resolution

<!-- Derived from [`sync::workspace_editable_conflict`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/sync.rs#L14992-L15111) -->

When multiple members reference the same workspace package with different editable settings, the
explicit `editable = true` takes precedence.

```toml
# file: child1/pyproject.toml

[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=1"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child1/src/child1/__init__.py
pass
```

```toml
# file: child2/pyproject.toml

[project]
name = "child2"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child1"]

[tool.uv.sources]
child1 = { workspace = true }

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child2/src/child2/__init__.py
pass
```

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child1"]

[tool.uv.workspace]
members = ["child1", "child2"]

[tool.uv.sources]
child1 = { workspace = true, editable = true }
```

When one member declares `editable = true` and another omits it, editable wins:

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + child1==0.1.0 (from file://[TEMP_DIR]/child1)
 + iniconfig==2.0.0
```
