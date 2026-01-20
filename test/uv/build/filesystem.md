# Package Building - Filesystem Handling

Tests for handling special filesystem features during builds.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Symlink

<!-- Derived from [`build::build_with_symlink`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1838-L1874) -->

Building succeeds when pyproject.toml is a symlink.

```toml
# file: pyproject.toml.real
[project]
name = "softlinked"
version = "0.1.0"
requires-python = ">=3.12"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: src/softlinked/__init__.py
```

Create symlink and build:

```console
$ ln -s pyproject.toml.real pyproject.toml && uv build
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built dist/softlinked-0.1.0.tar.gz
Successfully built dist/softlinked-0.1.0-py3-none-any.whl
```

## Hardlink

<!-- Derived from [`build::build_with_hardlink`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1877-L1913) -->

Building succeeds when pyproject.toml is a hardlink.

```toml
# file: pyproject.toml.real
[project]
name = "hardlinked"
version = "0.1.0"
requires-python = ">=3.12"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: src/hardlinked/__init__.py
```

Create hardlink and build:

```console
$ ln pyproject.toml.real pyproject.toml && uv build
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built dist/hardlinked-0.1.0.tar.gz
Successfully built dist/hardlinked-0.1.0-py3-none-any.whl
```

## Git boundary

<!-- Derived from [`build::build_git_boundary_in_dist_build`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1376-L1426) -->

Git boundary detection works correctly when building in the dist/ directory.

```toml
# file: demo/pyproject.toml
[project]
name = "demo"
version = "0.1.0"
requires-python = ">=3.11"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: demo/src/demo/__init__.py

def run():
    print("Running like the wind!")
```

Build the project:

```console
$ cd demo && uv build
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built dist/demo-0.1.0.tar.gz
Successfully built dist/demo-0.1.0-py3-none-any.whl
```

Artifacts are created:

```console
$ test -f demo/dist/demo-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

```console
$ test -f demo/dist/demo-0.1.0-py3-none-any.whl && echo "wheel exists"
success: true
exit_code: 0
----- stdout -----
wheel exists

----- stderr -----
```

## Venv in sdist

<!-- Derived from [`build::venv_included_in_sdist`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L2133-L2182) -->

Virtual environments must be excluded from source distributions.

Initialize a project:

```console
$ uv init --name project --build-backend hatchling
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project`
```

Update pyproject.toml to force-include .venv:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12.0"

[tool.hatch.build.targets.sdist.force-include]
".venv" = ".venv"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

Create a venv:

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/bin/activate
```

Build fails with helpful error:

```console
$ uv build
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
  × Failed to build `[TEMP_DIR]/`
  ├─▶ Invalid tar file
  ├─▶ failed to unpack `[CACHE_DIR]/sdists-v9/[TMP]/python`
  ╰─▶ symlink path `[PYTHON-3.12]` is absolute, but external symlinks are not allowed
  help: This file seems to be part of a virtual environment. Virtual environments must be excluded from source distributions.
```
