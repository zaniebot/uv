# Package Building - Workspaces

Tests for building packages in workspace configurations.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Workspace

<!-- Derived from [`build::build_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L486-L688) -->

`uv build` can build specific packages within a workspace using `--package` or all packages using
`--all`.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = ["packages/*"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

```toml
# file: project/packages/member/pyproject.toml
[project]
name = "member"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/packages/member/src/member/__init__.py
```

```text
# file: project/packages/member/README
```

```toml
# file: project/packages/virtual/pyproject.toml
[project]
name = "virtual"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```python
# file: project/packages/virtual/src/virtual/__init__.py
```

```text
# file: project/packages/virtual/README
```

Build a specific member:

```console
$ cd project && uv build --package member
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built dist/member-0.1.0.tar.gz
Successfully built dist/member-0.1.0-py3-none-any.whl
```

Member artifacts are created:

```console
$ test -f project/dist/member-0.1.0.tar.gz && echo "member sdist exists"
success: true
exit_code: 0
----- stdout -----
member sdist exists

----- stderr -----
```

```console
$ test -f project/dist/member-0.1.0-py3-none-any.whl && echo "member wheel exists"
success: true
exit_code: 0
----- stdout -----
member wheel exists

----- stderr -----
```

Build all packages:

```console
$ cd project && uv build --all --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
[PKG] Building source distribution...
[PKG] Building source distribution...
[PKG] Building wheel from source distribution...
[PKG] Building wheel from source distribution...
Successfully built dist/member-0.1.0.tar.gz
Successfully built dist/member-0.1.0-py3-none-any.whl
Successfully built dist/project-0.1.0.tar.gz
Successfully built dist/project-0.1.0-py3-none-any.whl
```

All artifacts are created:

```console
$ test -f project/dist/project-0.1.0.tar.gz && test -f project/dist/project-0.1.0-py3-none-any.whl && test -f project/dist/member-0.1.0.tar.gz && test -f project/dist/member-0.1.0-py3-none-any.whl && echo "all artifacts exist"
success: true
exit_code: 0
----- stdout -----
all artifacts exist

----- stderr -----
```

Workspace discovery from source path:

```console
$ uv build ./project --package member
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/member-0.1.0.tar.gz
Successfully built project/dist/member-0.1.0-py3-none-any.whl
```

Build all from source path:

```console
$ uv build ./project --all --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
[PKG] Building source distribution...
[PKG] Building source distribution...
[PKG] Building wheel from source distribution...
[PKG] Building wheel from source distribution...
Successfully built project/dist/member-0.1.0.tar.gz
Successfully built project/dist/member-0.1.0-py3-none-any.whl
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

Error when --package is used outside a workspace:

```console
$ uv build --package member
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--package` was provided, but no workspace was found
  Caused by: No `pyproject.toml` found in current directory or any parent directory
```

Error when --all is used outside a workspace:

```console
$ uv build --all
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--all-packages` was provided, but no workspace was found
  Caused by: No `pyproject.toml` found in current directory or any parent directory
```

Error when --package specifies non-existent member:

```console
$ cd project && uv build --package fail
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package `fail` not found in workspace
```

## All with failure

<!-- Derived from [`build::build_all_with_failure`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L691-L834) -->

When building all packages, successful packages are built even if one package fails.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = ["packages/*"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

```toml
# file: project/packages/member_a/pyproject.toml
[project]
name = "member_a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/packages/member_a/src/member_a/__init__.py
```

```text
# file: project/packages/member_a/README
```

```toml
# file: project/packages/member_b/pyproject.toml
[project]
name = "member_b"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: project/packages/member_b/src/member_b/__init__.py
```

```text
# file: project/packages/member_b/README
```

```python
# file: project/packages/member_b/setup.py

from setuptools import setup

setup(
    name="project",
    version="0.1.0",
    packages=["project"],
    install_requires=["foo==3.7.0"],
)
```

Build all packages (one fails):

```console
$ cd project && uv build --all --no-build-logs
success: false
exit_code: 2
----- stdout -----

----- stderr -----
[PKG] Building source distribution...
[PKG] Building source distribution...
[PKG] Building source distribution...
[PKG] Building wheel from source distribution...
[PKG] Building wheel from source distribution...
Successfully built dist/member_a-0.1.0.tar.gz
Successfully built dist/member_a-0.1.0-py3-none-any.whl
  × Failed to build `member-b @ [TEMP_DIR]/project/packages/member_b`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `setuptools.build_meta.build_sdist` failed (exit status: 1)
      hint: This usually indicates a problem with the package or the build environment.
Successfully built dist/project-0.1.0.tar.gz
Successfully built dist/project-0.1.0-py3-none-any.whl
```

Successful packages are still built:

```console
$ test -f project/dist/project-0.1.0.tar.gz && test -f project/dist/project-0.1.0-py3-none-any.whl && echo "project artifacts exist"
success: true
exit_code: 0
----- stdout -----
project artifacts exist

----- stderr -----
```

```console
$ test -f project/dist/member_a-0.1.0.tar.gz && test -f project/dist/member_a-0.1.0-py3-none-any.whl && echo "member_a artifacts exist"
success: true
exit_code: 0
----- stdout -----
member_a artifacts exist

----- stderr -----
```

## Virtual root

<!-- Derived from [`build::build_workspace_virtual_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1961-L1984) -->

Building a virtual workspace root (without a `[project]` table) shows a warning but succeeds.

```toml
# file: pyproject.toml
[tool.uv.workspace]
members = ["packages/*"]
```

Build without a build system:

```console
$ uv build --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
warning: `[TEMP_DIR]/` appears to be a workspace root without a Python project; consider using `uv sync` to install the workspace, or add a `[build-system]` table to `pyproject.toml`
Building wheel from source distribution...
Successfully built dist/cache-0.0.0.tar.gz
Successfully built dist/UNKNOWN-0.0.0-py3-none-any.whl
```

## Trailing slash

<!-- Derived from [`build::test_workspace_trailing_slash`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L2188-L2244) -->

Workspace discovery works correctly with trailing slashes and path normalization.

Create a workspace with a root and a member:

```console
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project`
```

```console
$ uv init --lib child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `child` at `[TEMP_DIR]/child`
```

Build without trailing slash:

```console
$ uv build child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built dist/child-0.1.0.tar.gz
Successfully built dist/child-0.1.0-py3-none-any.whl
```

Build with trailing slash:

```console
$ uv build child/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built dist/child-0.1.0.tar.gz
Successfully built dist/child-0.1.0-py3-none-any.whl
```

Build with ./ prefix and trailing slash:

```console
$ uv build ./child/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built dist/child-0.1.0.tar.gz
Successfully built dist/child-0.1.0-py3-none-any.whl
```

Build with complex path normalization:

```console
$ uv build ./child/../child/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built dist/child-0.1.0.tar.gz
Successfully built dist/child-0.1.0-py3-none-any.whl
```
