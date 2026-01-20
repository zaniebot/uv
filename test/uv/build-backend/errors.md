# Build Backend - Error Handling

Tests for error detection and messages in the uv build backend.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Missing module

<!-- Derived from [`build_backend::sdist_error_without_module`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L580-L624) -->

Building fails with a helpful error when the expected module is missing.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

Building sdist without module fails:

```console
$ uv build --sdist
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Expected a Python module at: src/foo/__init__.py
```

Create src directory:

```console
$ mkdir src
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Still fails with src directory but no module:

```console
$ uv build --sdist
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Expected a Python module at: src/foo/__init__.py
```

## License glob

<!-- Derived from [`build_backend::license_glob_without_matches_errors`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L764-L806) -->

License file globs that don't match any files produce an error.

Initialize a project:

```console
$ uv init --lib --name missing-license
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `missing-license`
```

Create one license file:

```text
# file: LICENSE.txt
permissive license
```

Configure with a glob that includes non-existent file:

```toml
# file: pyproject.toml
[project]
name = "missing-license"
version = "1.0.0"
license-files = ["abc", "LICENSE.txt"]

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

Building fails due to non-matching glob:

```console
$ uv build --wheel
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ├─▶ Invalid project metadata
  ╰─▶ `project.license-files` glob `abc` did not match any files
```

## License UTF8

<!-- Derived from [`build_backend::license_file_must_be_utf8`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L809-L849) -->

License files must be UTF-8 encoded.

Initialize a project:

```console
$ uv init --lib --name license-utf8
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `license-utf8`
```

Configure with license file:

```toml
# file: pyproject.toml
[project]
name = "license-utf8"
version = "1.0.0"
license-files = ["LICENSE.bin"]

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

Create non-UTF8 license file:

```console
$ printf '\xff' > LICENSE.bin
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Building fails with encoding error:

```console
$ uv build --wheel
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ├─▶ Invalid project metadata
  ╰─▶ License file `LICENSE.bin` must be UTF-8 encoded
```

## Module root outside

<!-- Derived from [`build_backend::error_on_relative_module_root_outside_project_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L979-L1022) -->

Module root must be inside the project directory.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.build-backend]
module-root = ".."

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

```python
# file: __init__.py
```

Building sdist fails:

```console
$ uv build
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Module root must be inside the project: ..
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
  ╰─▶ Module root must be inside the project: ..
```

## Data dir outside

<!-- Derived from [`build_backend::error_on_relative_data_dir_outside_project_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L1027-L1077) -->

Data directories must be inside the project directory.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.build-backend.data]
headers = "../header"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

```python
# file: project/src/project/__init__.py
```

Create the headers directory outside the project:

```console
$ mkdir headers
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Building sdist fails:

```console
$ uv build project
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
  × Failed to build `[TEMP_DIR]/project`
  ╰─▶ The path for the data directory headers must be inside the project: ../header
```

Building wheel also fails:

```console
$ uv build project --wheel
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
  × Failed to build `[TEMP_DIR]/project`
  ╰─▶ The path for the data directory headers must be inside the project: ../header
```

## Invalid pyproject

<!-- Derived from [`build_backend::invalid_pyproject_toml`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L1190-L1224) -->

Invalid pyproject.toml files produce clear error messages.

```toml
# file: child/pyproject.toml
[project]
name = 1
version = "1.0.0"

[build-system]
requires = ["uv_build>=0.9,<10000"]
build-backend = "uv_build"
```

Building fails with parse error:

```console
$ uv build child
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
  × Failed to build `[TEMP_DIR]/child`
  ├─▶ Invalid metadata format in: child/pyproject.toml
  ╰─▶ TOML parse error at line 2, column 8
        |
      2 | name = 1
        |        ^
      invalid type: integer `1`, expected a string
```
