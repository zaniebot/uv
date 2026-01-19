# Git Error Handling

Tests for Git-related error handling.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = ["git"]
```

## Add error

<!-- Derived from [`edit::add_git_error`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Providing Git-specific options for non-Git sources is an error.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
```

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited in [TIME]
```

Providing a tag without a Git source:

```console
$ uv add flask --tag 0.0.1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a Git repository, but a Git reference (`--tag 0.0.1`) was provided.
```

Providing a branch with a non-Git source:

```console
$ uv add flask@https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl --branch 0.0.1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a Git repository, but a Git reference (`--branch 0.0.1`) was provided.
```

## Unsupported scheme

<!-- Derived from [`edit::add_unsupported_git_scheme`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Using an unsupported Git URL scheme is an error.

```console
$ uv init .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `temp` at `[TEMP_DIR]/`
```

```console
$ uv add git+fantasy://ferris/dreams/of/urls@7701ffcbae245819b828dc5f885a5201158897ef
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `git+fantasy://ferris/dreams/of/urls@7701ffcbae245819b828dc5f885a5201158897ef`
  Caused by: Unsupported Git URL scheme `fantasy:` in `fantasy://ferris/dreams/of/urls` (expected one of `https:`, `ssh:`, or `file:`)
git+fantasy://ferris/dreams/of/urls@7701ffcbae245819b828dc5f885a5201158897ef
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
