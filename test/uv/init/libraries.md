# Project Initialization - Libraries

Tests for initializing library projects with `uv init --lib`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Library

<!-- Derived from [`init::init_library`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L369-L447) -->

The `--lib` flag creates a library project with src layout.

```console working-dir="foo"
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

A src-layout library structure is created:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

The **init**.py contains a sample function:

```python title="foo/src/foo/__init__.py" snapshot=true
def hello() -> str:
    return "Hello from foo!"
```

An empty py.typed file is created:

```text title="foo/src/foo/py.typed" snapshot=true

```

The library can be imported:

```console working-dir="foo"
$ uv run python -c "import foo; print(foo.hello())"
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
warning: `VIRTUAL_ENV=[VENV]/` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==0.1.0 (from file://[TEMP_DIR]/foo)
```

## Library no package

<!-- Derived from [`init::init_library_no_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1019-L1124) -->

Using `--lib --no-package` is not allowed.

```console working-dir="foo"
$ uv init --lib --no-package
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--lib' cannot be used with '--no-package'

Usage: uv init [OPTIONS] [PATH]

For more information, try '--help'.
```

## Library current directory

<!-- Derived from [`init::init_library_current_dir`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1126-L1225) -->

Using `--lib` in the current directory creates a library.

```console working-dir="foo"
$ uv init --lib .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

Library files are created in the current directory:

```python title="foo/src/foo/__init__.py" snapshot=true
def hello() -> str:
    return "Hello from foo!"
```

The library can be imported:

```console working-dir="foo"
$ uv run python -c "import foo; print(foo.hello())"
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
warning: `VIRTUAL_ENV=[VENV]/` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==0.1.0 (from file://[TEMP_DIR]/foo)
```

## Bare lib

<!-- Derived from [`init::init_bare_lib`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L495-L544) -->

The `--bare --lib` flags create a minimal library project.

```console working-dir="foo"
$ uv init --bare --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

Only pyproject.toml is created:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

## Bare package

<!-- Derived from [`init::init_bare_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L547-L596) -->

The `--bare --package` flags create a minimal package project.

```console working-dir="foo"
$ uv init --bare --package
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

Only pyproject.toml is created:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

## Bare opt-in

<!-- Derived from [`init::init_bare_opt_in`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L599-L652) -->

With `--bare`, you can still opt-in to extras.

```console working-dir="foo"
$ uv init --bare --description foo --pin-python --vcs git
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The project includes the requested extras:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "foo"
requires-python = ">=3.12"
dependencies = []
```

Python version is pinned:

```text title="foo/.python-version" snapshot=true
3.12
```

## Package preview

<!-- Derived from [`init::init_package_preview`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L452-L492) -->

The `--package --preview` flags use the uv build backend.

```console working-dir="foo"
$ uv init --package --preview
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The uv build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.scripts]
foo = "foo:main"

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

## py.typed exists

<!-- Derived from [`init::init_py_typed_exists`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L985-L1015) -->

When py.typed already exists, it is not overwritten.

```text
# file: foo/src/foo/py.typed
partial
```

```console working-dir="foo"
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The existing py.typed content is preserved:

```text title="foo/src/foo/py.typed" snapshot=true
partial
```
