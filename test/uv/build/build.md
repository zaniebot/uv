# Package Building - Basic Building

Tests for building source distributions and wheels from projects using `uv build`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Basic

<!-- Derived from [`build::build_basic`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L14-L129) -->

`uv build` builds both a source distribution and wheel by default.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

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

Build from a specified path:

```console
$ uv build project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

Both artifacts are created:

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

Build from the current working directory:

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

Error when there's no project to build:

```console
$ uv build
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ [TEMP_DIR]/ does not appear to be a Python project, as neither `pyproject.toml` nor `setup.py` are present in the directory
```

Build to a custom output directory:

```console
$ cd project && uv build --out-dir out
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built out/project-0.1.0.tar.gz
Successfully built out/project-0.1.0-py3-none-any.whl
```

Artifacts are created in the custom directory:

```console
$ test -f project/out/project-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

## Sdist

<!-- Derived from [`build::build_sdist`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L132-L185) -->

The `--sdist` flag builds only the source distribution.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

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

Build only the sdist:

```console
$ cd project && uv build --sdist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Successfully built dist/project-0.1.0.tar.gz
```

Only the sdist is created:

```console
$ test -f project/dist/project-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

```console
$ test -f project/dist/project-0.1.0-py3-none-any.whl && echo "wheel exists" || echo "wheel missing"
success: true
exit_code: 0
----- stdout -----
wheel missing

----- stderr -----
```

## Wheel

<!-- Derived from [`build::build_wheel`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L188-L241) -->

The `--wheel` flag builds only the wheel.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

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

Build only the wheel:

```console
$ cd project && uv build --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building wheel...
Successfully built dist/project-0.1.0-py3-none-any.whl
```

Only the wheel is created:

```console
$ test -f project/dist/project-0.1.0.tar.gz && echo "sdist exists" || echo "sdist missing"
success: true
exit_code: 0
----- stdout -----
sdist missing

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

## Sdist wheel

<!-- Derived from [`build::build_sdist_wheel`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L244-L299) -->

Both `--sdist` and `--wheel` can be specified together.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

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

Build both sdist and wheel:

```console
$ cd project && uv build --sdist --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel...
Successfully built dist/project-0.1.0.tar.gz
Successfully built dist/project-0.1.0-py3-none-any.whl
```

Both artifacts are created:

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

## Wheel from sdist

<!-- Derived from [`build::build_wheel_from_sdist`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L302-L408) -->

A wheel can be built from an existing source distribution.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

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

First build the sdist:

```console
$ cd project && uv build --sdist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Successfully built dist/project-0.1.0.tar.gz
```

Building from sdist without `--wheel` fails:

```console
$ cd project && uv build ./dist/project-0.1.0.tar.gz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
  × Failed to build `[TEMP_DIR]/project/dist/project-0.1.0.tar.gz`
  ╰─▶ Pass `--wheel` explicitly to build a wheel from a source distribution
```

Building sdist from sdist is not supported:

```console
$ cd project && uv build ./dist/project-0.1.0.tar.gz --sdist
success: false
exit_code: 2
----- stdout -----

----- stderr -----
  × Failed to build `[TEMP_DIR]/project/dist/project-0.1.0.tar.gz`
  ╰─▶ Building an `--sdist` from a source distribution is not supported
```

Build wheel from the sdist:

```console
$ cd project && uv build ./dist/project-0.1.0.tar.gz --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building wheel from source distribution...
Successfully built dist/project-0.1.0-py3-none-any.whl
```

Both artifacts now exist:

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

Passing a wheel as input is an error:

```console
$ cd project && uv build ./dist/project-0.1.0-py3-none-any.whl --wheel
success: false
exit_code: 2
----- stdout -----

----- stderr -----
  × Failed to build `[TEMP_DIR]/project/dist/project-0.1.0-py3-none-any.whl`
  ╰─▶ `dist/project-0.1.0-py3-none-any.whl` is not a valid build source. Expected to receive a source directory, or a source distribution ending in one of: `.tar.gz`, `.zip`, `.tar.bz2`, `.tar.lz`, `.tar.lzma`, `.tar.xz`, `.tar.zst`, `.tar`, `.tbz`, `.tgz`, `.tlz`, or `.txz`.
```

## Fail

<!-- Derived from [`build::build_fail`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L411-L483) -->

Build failures are reported with clear error messages.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

```python
# file: project/setup.py

from setuptools import setup

setup(
    name="project",
    version="0.1.0",
    packages=["project"],
    install_requires=["foo==3.7.0"],
)
```

The build fails due to indentation error in setup.py:

```console
$ uv build project
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
Traceback (most recent call last):
  File "<string>", line 14, in <module>
  File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 328, in get_requires_for_build_sdist
    return self._get_build_requires(config_settings, requirements=[])
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 295, in _get_build_requires
    self.run_setup()
  File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 311, in run_setup
    exec(code, locals())
  File "<string>", line 2
    from setuptools import setup
IndentationError: unexpected indent
  × Failed to build `[TEMP_DIR]/project`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `setuptools.build_meta.build_sdist` failed (exit status: 1)
      hint: This usually indicates a problem with the package or the build environment.
```

## Fast path

<!-- Derived from [`build::build_fast_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1533-L1629) -->

The uv build backend provides a fast path for building packages.

Build both sdist and wheel (default):

```console
$ uv build test/packages/built-by-uv --out-dir output1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built output1/built_by_uv-0.1.0.tar.gz
Successfully built output1/built_by_uv-0.1.0-py3-none-any.whl
```

Both artifacts are created:

```console
$ test -f output1/built_by_uv-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

```console
$ test -f output1/built_by_uv-0.1.0-py3-none-any.whl && echo "wheel exists"
success: true
exit_code: 0
----- stdout -----
wheel exists

----- stderr -----
```

Build only sdist:

```console
$ uv build test/packages/built-by-uv --out-dir output2 --sdist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Successfully built output2/built_by_uv-0.1.0.tar.gz
```

Only the sdist is created:

```console
$ test -f output2/built_by_uv-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

Build only wheel:

```console
$ uv build test/packages/built-by-uv --out-dir output3 --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
Successfully built output3/built_by_uv-0.1.0-py3-none-any.whl
```

Only the wheel is created:

```console
$ test -f output3/built_by_uv-0.1.0-py3-none-any.whl && echo "wheel exists"
success: true
exit_code: 0
----- stdout -----
wheel exists

----- stderr -----
```

Build both with explicit flags:

```console
$ uv build test/packages/built-by-uv --out-dir output4 --sdist --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel (uv build backend)...
Successfully built output4/built_by_uv-0.1.0.tar.gz
Successfully built output4/built_by_uv-0.1.0-py3-none-any.whl
```

Both artifacts are created:

```console
$ test -f output4/built_by_uv-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

```console
$ test -f output4/built_by_uv-0.1.0-py3-none-any.whl && echo "wheel exists"
success: true
exit_code: 0
----- stdout -----
wheel exists

----- stderr -----
```
