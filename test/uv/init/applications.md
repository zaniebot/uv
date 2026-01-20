# Project Initialization - Applications

Tests for initializing application projects with `uv init --app`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Application

<!-- Derived from [`init::init_application`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L116-L182) -->

The `--app` flag creates an application project with a main.py file.

```console working-dir="foo"
$ uv init --app
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The pyproject.toml is created:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

A main.py file is created:

```python title="foo/main.py" snapshot=true
def main():
    print("Hello from foo!")


if __name__ == "__main__":
    main()
```

The application can be run:

```console working-dir="foo"
$ uv run main.py
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
warning: `VIRTUAL_ENV=[VENV]/` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Audited in [TIME]
```

## Application when main.py exists

<!-- Derived from [`init::init_application_hello_exists`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L186-L232) -->

When main.py already exists, it is not overwritten.

```python
# file: foo/main.py
```

```console working-dir="foo"
$ uv init --app
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The existing main.py remains empty:

```python title="foo/main.py" snapshot=true

```

## Application when other Python files exist

<!-- Derived from [`init::init_application_other_python_exists`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L236-L290) -->

When other Python files exist, main.py is still created.

```python
# file: foo/foo.py
```

```console working-dir="foo"
$ uv init --app
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

main.py is created with default content:

```python title="foo/main.py" snapshot=true
def main():
    print("Hello from foo!")


if __name__ == "__main__":
    main()
```

## Application package

<!-- Derived from [`init::init_application_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L294-L389) -->

The `--app --package` flags create a packaged application with src layout.

```console working-dir="foo"
$ uv init --app --package
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

A src-layout package structure is created:

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
requires = ["hatchling"]
build-backend = "hatchling.build"
```

The **init**.py contains the main function:

```python title="foo/src/foo/__init__.py" snapshot=true
def main() -> None:
    print("Hello from foo!")
```

## Application in current directory

<!-- Derived from [`init::init_application_current_dir`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L391-L456) -->

Using `--app` in the current directory creates an application.

```console working-dir="foo"
$ uv init --app .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

Application files are created in the current directory:

```python title="foo/main.py" snapshot=true
def main():
    print("Hello from foo!")


if __name__ == "__main__":
    main()
```

The application can be run:

```console working-dir="foo"
$ uv run main.py
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
warning: `VIRTUAL_ENV=[VENV]/` does not match the project environment path `.venv` and will be ignored; use `--active` to target the active environment instead
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Audited in [TIME]
```
