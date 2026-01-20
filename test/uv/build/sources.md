# Package Building - Source Discovery

Tests for source discovery and build-system detection.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Tool uv sources

<!-- Derived from [`build::build_tool_uv_sources`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1280-L1372) -->

`tool.uv.sources` can reference local packages used during the build.

```toml
# file: backend/pyproject.toml
[project]
name = "backend"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions>=3.10"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: backend/src/backend/__init__.py
def hello() -> str:
    return "Hello, world!"
```

```text
# file: backend/README.md
```

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>1"]

[build-system]
requires = ["hatchling", "backend==0.1.0"]
build-backend = "hatchling.build"

[tool.uv.sources]
backend = { path = "../backend" }
```

```python
# file: project/setup.py
from setuptools import setup

from backend import hello

hello()

setup()
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

Build succeeds with local backend dependency:

```console
$ cd project && uv build
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built dist/project-0.1.0.tar.gz
Successfully built dist/project-0.1.0-py3-none-any.whl
```

Artifacts are created:

```console
$ test -f project/dist/project-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

```console
$ test -f project/dist/project-0.1.0-py3-none-any.whl && echo "wheel exists"
success: true
exit_code: 0
----- stdout -----
wheel exists

----- stderr -----
```

## Non-package

<!-- Derived from [`build::build_non_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1429-L1525) -->

Building a package without a `[build-system]` table shows a helpful error.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = ["packages/*"]
```

```python
# file: project/src/__init__.py
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
```

```python
# file: project/packages/member/src/__init__.py
```

```text
# file: project/packages/member/README
```

Building a specific package without build-system fails:

````console
$ cd project && uv build --package member
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package `member` is missing a `build-system`. For example, to build with `setuptools`, add the following to `packages/member/pyproject.toml`:
```toml
[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"
````

````

No artifacts are created:

```console
$ test -f project/dist/member-0.1.0.tar.gz && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
````

Building all packages also fails:

````console
$ cd project && uv build --all --no-build-logs
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Workspace does not contain any buildable packages. For example, to build `member` with `setuptools`, add a `build-system` to `packages/member/pyproject.toml`:
```toml
[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"
````

````

## Not a project

<!-- Derived from [`build::build_pyproject_toml_not_a_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1989-L2013) -->

Building when pyproject.toml lacks build information shows a warning but succeeds.

```toml
# file: pyproject.toml
# Some other content we don't know about
[tool.black]
line-length = 88
````

Build without build-system or project table:

```console
$ uv build --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
warning: `[TEMP_DIR]/` does not appear to be a Python project, as the `pyproject.toml` does not include a `[build-system]` table, and neither `setup.py` nor `setup.cfg` are present in the directory
Building wheel from source distribution...
Successfully built dist/cache-0.0.0.tar.gz
Successfully built dist/UNKNOWN-0.0.0-py3-none-any.whl
```
