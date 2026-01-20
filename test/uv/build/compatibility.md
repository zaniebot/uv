# Package Building - Build Backend Compatibility

Tests for build backend compatibility and PEP 517 handling.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Unconfigured setuptools

<!-- Derived from [`build::build_unconfigured_setuptools`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1918-L1956) -->

Projects with `[project]` but no `[build-system]` fall back to setuptools.

```toml
# file: pyproject.toml
[project]
name = "greet"
version = "0.1.0"
```

```python
# file: src/greet/__init__.py
print('Greetings!')
```

Installation works with implicit setuptools backend:

```console
$ uv pip install .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + greet==0.1.0 (from file://[TEMP_DIR]/)
```

The package can be imported:

```console
$ python -c "import greet"
success: true
exit_code: 0
----- stdout -----
Greetings!

----- stderr -----
```

## Force PEP517

<!-- Derived from [`build::force_pep517`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L2077-L2123) -->

The `--force-pep517` flag changes error messages from direct build to PEP 517 build.

Initialize a project:

```console
$ uv init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project`
```

Configure with invalid uv build backend:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "1.0.0"

[tool.uv.build-backend]
module-name = "does_not_exist"

[build-system]
requires = ["uv_build>=0.5.15,<10000"]
build-backend = "uv_build"
```

Direct build shows uv-specific error:

```console
$ RUST_BACKTRACE=0 uv build
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Expected a Python module at: src/does_not_exist/__init__.py
```

With --force-pep517, shows PEP 517 error:

```console
$ RUST_BACKTRACE=0 uv build --force-pep517
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
Error: Missing source directory at: `src`
  × Failed to build `[TEMP_DIR]/`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `uv_build.build_sdist` failed (exit status: 1)
      hint: This usually indicates a problem with the package or the build environment.
```
