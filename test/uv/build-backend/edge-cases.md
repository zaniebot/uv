# Build Backend - Edge Cases

Tests for edge cases and special scenarios in the uv build backend.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Long path

<!-- Derived from [`build_backend::build_sdist_with_long_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L537-L577) -->

Building an sdist with very long file paths succeeds.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

```python
# file: src/foo/__init__.py
print("Hi from foo")
```

Create a file with a very long path (over 100 characters in the directory name):

```python
# file: src/foo/looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong/__init__.py
print("Hi from foo")
```

Build the sdist:

```console
$ uv build --sdist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Successfully built dist/foo-1.0.0.tar.gz
```

## Symlinked file

<!-- Derived from [`build_backend::symlinked_file`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L854-L937) -->

```toml
# mdtest
[environment]
target-family = "unix"
```

Symlinked files (like licenses) are properly included in builds.

Initialize a project:

```console
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project`
```

Configure to use uv build backend with license file:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "1.0.0"
license-files = ["LICENSE"]

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

Create a license file outside the project:

```text
# file: ../LICENSE
Project license
```

Create a symlink to the license:

```console
$ ln -s ../LICENSE LICENSE
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Build sdist:

```console
$ uv build --sdist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Successfully built dist/project-1.0.0.tar.gz
```

Build wheel:

```console
$ uv build --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
Successfully built dist/project-1.0.0-py3-none-any.whl
```

Install the wheel:

```console
$ uv pip install dist/project-1.0.0-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + project==1.0.0 (from file://[TEMP_DIR]/dist/project-1.0.0-py3-none-any.whl)
```

Verify the license content was included (not a broken symlink):

```console
$ cat .venv/lib/python*/site-packages/project-1.0.0.dist-info/licenses/LICENSE
success: true
exit_code: 0
----- stdout -----
Project license

----- stderr -----
```

## Invalid settings

<!-- Derived from [`build_backend::invalid_build_backend_settings_are_ignored`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L943-L974) -->

Invalid build backend settings are ignored when not building, allowing forward/backward
compatibility.

```toml
# file: pyproject.toml
[project]
name = "built-by-uv"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.build-backend]
# Error: `source-include` must be a list
source-include = "data/build-script.py"

[build-system]
requires = ["uv_build>=10000,<10001"]
build-backend = "uv_build"
```

Locking (which doesn't build) succeeds despite invalid settings:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
```

## Venv in source

<!-- Derived from [`build_backend::venv_in_source_tree`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L1081-L1119) -->

Virtual environments in the source tree are detected and rejected with a helpful error.

Initialize a project:

```console
$ uv init --lib --name foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

Create a venv inside the source tree:

```console
$ uv venv src/foo/.venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: src/foo/.venv
Activate with: source src/foo/.venv/bin/activate
```

Building sdist fails with helpful error:

```console
$ uv build
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Virtual environments must not be added to source distributions or wheels, remove the directory or exclude it from the build: src/foo/.venv
```

Building wheel also fails:

```console
$ uv build --wheel
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Virtual environments must not be added to source distributions or wheels, remove the directory or exclude it from the build: src/foo/.venv
```

## Redundant module names

<!-- Derived from [`build_backend::warn_on_redundant_module_names`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L1123-L1187) -->

The build backend warns about redundant module names in configuration.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"

[tool.uv.build-backend]
module-name = ["foo", "foo.bar", "foo", "foo.bar.baz", "foobar", "bar", "foobar.baz", "baz.bar"]
```

Create the required modules:

```python
# file: src/foo/__init__.py
```

```python
# file: src/foobar/__init__.py
```

```python
# file: src/baz/bar/__init__.py
```

```python
# file: src/bar/__init__.py
```

Building shows warnings about redundant names:

```console
$ uv build
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
warning: Ignoring redundant module names in `tool.uv.build-backend.module-name`: `foo.bar`, `foo`, `foo.bar.baz`, `foobar.baz`
Building wheel from source distribution (uv build backend)...
Successfully built dist/project-0.1.0.tar.gz
Successfully built dist/project-0.1.0-py3-none-any.whl
```

With --no-sources, warnings are suppressed (user doesn't control the build):

```console
$ uv build --no-sources
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built dist/project-0.1.0.tar.gz
Successfully built dist/project-0.1.0-py3-none-any.whl
```
