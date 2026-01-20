# Python Management - Directory

Tests for `uv python dir` command.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Python dir

<!-- Derived from [`python_dir::python_dir`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_dir.rs#L8-L21) -->

The `uv python dir` command displays the Python installation directory.

Create a custom Python directory:

```console
$ mkdir python
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Display the directory:

```console
$ UV_PYTHON_INSTALL_DIR=python uv python dir
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/python

----- stderr -----
```
