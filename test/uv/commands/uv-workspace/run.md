# Workspace Run

Tests for running commands in workspaces with `uv run`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Run with --package in virtual workspace

<!-- Derived from [`workspace::test_uv_run_with_package_virtual_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Run commands targeting specific packages in a virtual workspace.

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

Run from the bird-feeder member:

```console
$ uv run --package bird-feeder packages/bird-feeder/check_installed_bird_feeder.py
success: true
exit_code: 0
----- stdout -----
Success

----- stderr -----
Installed 5 packages in [TIME]
```

Run from the albatross member:

```console
$ uv run --package albatross packages/albatross/check_installed_albatross.py
success: true
exit_code: 0
----- stdout -----
Success

----- stderr -----
Installed 2 packages in [TIME]
```

## Run from virtual workspace root

<!-- Derived from [`workspace::test_uv_run_virtual_workspace_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Running from a virtual workspace root syncs all packages.

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
$ uv run packages/albatross/check_installed_albatross.py
success: true
exit_code: 0
----- stdout -----
Success

----- stderr -----
Installed 7 packages in [TIME]
```

## Run with --package in root workspace

<!-- Derived from [`workspace::test_uv_run_with_package_root_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Run commands targeting specific packages in a root workspace.

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

Run from the bird-feeder member:

```console
$ uv run --package bird-feeder packages/bird-feeder/check_installed_bird_feeder.py
success: true
exit_code: 0
----- stdout -----
Success

----- stderr -----
Installed 4 packages in [TIME]
```

Run from the albatross root package:

```console
$ uv run --package albatross check_installed_albatross.py
success: true
exit_code: 0
----- stdout -----
Success

----- stderr -----
Installed 1 package in [TIME]
```

## Run with --isolated

<!-- Derived from [`workspace::test_uv_run_isolate`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/workspace.rs) -->

Running with `--isolated` creates isolated virtual environments.

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

First, install the root package:

```console
$ uv run --package albatross check_installed_albatross.py
success: true
exit_code: 0
----- stdout -----
Success

----- stderr -----
Installed 5 packages in [TIME]
```

Running in bird-feeder without `--isolated` can still import albatross (due to shared venv):

```console
$ uv run --package bird-feeder check_installed_albatross.py
success: true
exit_code: 0
----- stdout -----
Success

----- stderr -----
```

With `--isolated`, albatross is not available:

```console
$ uv run --isolated --package bird-feeder check_installed_albatross.py
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Installed 4 packages in [TIME]
Traceback (most recent call last):
  File "[TEMP_DIR]/check_installed_albatross.py", line 1, in <module>
    from albatross import fly
ModuleNotFoundError: No module named 'albatross'
```

## Run in workspace with dependencies

<!-- Derived from [`run::run_in_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/run.rs#L1649-L1807) -->

Running scripts in a workspace only installs dependencies based on the selected package.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio>3"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.uv.workspace]
members = ["child1", "child2"]

[tool.uv.sources]
child1 = { workspace = true }
child2 = { workspace = true }
```

```python
# file: src/project/__init__.py
pass
```

```toml
# file: child1/pyproject.toml

[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>1"]

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
dependencies = ["typing-extensions>4"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: child2/src/child2/__init__.py
pass
```

Running a script that imports from the root project works:

```python
# file: main.py
import anyio
```

```console
$ uv run main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + project==0.1.0 (from file://[TEMP_DIR]/)
 + sniffio==1.3.1
```

Running a script that imports from child1 fails without --package:

```python
# file: main.py
import iniconfig
```

```console
$ uv run main.py
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
Audited 4 packages in [TIME]
Traceback (most recent call last):
  File "[TEMP_DIR]/main.py", line 1, in <module>
    import iniconfig
ModuleNotFoundError: No module named 'iniconfig'
```

With --package child1, iniconfig is available:

```console
$ uv run --package child1 main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + child1==0.1.0 (from file://[TEMP_DIR]/child1)
 + iniconfig==2.0.0
```

Running a script that imports from child2 fails without --package:

```python
# file: main.py
import typing_extensions
```

```console
$ uv run main.py
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
Audited 4 packages in [TIME]
Traceback (most recent call last):
  File "[TEMP_DIR]/main.py", line 1, in <module>
    import typing_extensions
ModuleNotFoundError: No module named 'typing_extensions'
```

With --all-packages, all child dependencies are available:

```console
$ uv run --all-packages main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + child2==0.1.0 (from file://[TEMP_DIR]/child2)
 + typing-extensions==4.10.0
```

## Run with target workspace discovery

<!-- Derived from [`run::run_target_workspace_discovery`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/run.rs#L6158-L6217) -->

The preview feature `target-workspace-discovery` discovers workspaces from the target script's
directory.

Create a workspace in a subdirectory:

```toml
# file: project/pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: project/script.py
import iniconfig
print('success')
```

Without the preview feature, running from the parent directory fails to find the workspace:

```console
$ uv run project/script.py
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Traceback (most recent call last):
  File "[TEMP_DIR]/project/script.py", line 1, in <module>
    import iniconfig
ModuleNotFoundError: No module named 'iniconfig'
```

With the preview feature, the workspace is discovered from the target's directory:

```console
$ uv run --preview-features target-workspace-discovery project/script.py
success: true
exit_code: 0
----- stdout -----
success

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: project/.venv
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/project)
 + iniconfig==2.0.0
```
