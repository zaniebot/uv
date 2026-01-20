# Project Initialization - Error Handling

Tests for error handling during `uv init`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Failure

<!-- Derived from [`init::init_failure`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2726-L2770) -->

When an invalid pyproject.toml exists in a parent directory, `uv init` fails.

```toml
# file: pyproject.toml
```

```console
$ uv init foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to discover parent workspace; use `uv init --no-workspace` to ignore
  Caused by: No `project` table found in: `[TEMP_DIR]/pyproject.toml`
```

Using `--no-workspace` ignores the invalid parent:

```console
$ uv init --no-workspace foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The project is created successfully:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Failure with invalid option named backend

<!-- Derived from [`init::init_failure_with_invalid_option_named_backend`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2772-L2804) -->

Using `--backend` instead of `--build-backend` shows a helpful error.

```console
$ uv init --backend foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: unexpected argument '--backend' found

  tip: a similar argument exists: '--build-backend'

Usage: uv init [OPTIONS] [PATH]

For more information, try '--help'.
```

The error applies even with a value:

```console
$ uv init --backend maturin foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: unexpected argument '--backend' found

  tip: a similar argument exists: '--build-backend'

Usage: uv init [OPTIONS] [PATH]

For more information, try '--help'.
```

## Project flag not allowed under preview

<!-- Derived from [`init::init_project_flag_is_not_allowed_under_preview`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L4075-L4104) -->

The `--project` flag is not allowed when the preview feature is enabled.

With a positional path:

```console
$ uv init --preview-features init-project-flag --project foo bar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The `--project` option cannot be used in `uv init`. Use `--directory` instead.
```

Without a positional path:

```console
$ uv init --preview-features init-project-flag --project foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: The `--project` option cannot be used in `uv init`. Use `--directory` or a positional path instead.
```

## Project flag ignored with explicit path

<!-- Derived from [`init::init_project_flag_is_ignored_with_explicit_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L4106-L4137) -->

The `--project` flag is deprecated and ignored when a positional path is provided.

```console
$ uv init --project bar foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Use of the `--project` option in `uv init` is deprecated and will be removed in a future release. Since a positional path was provided, the `--project` option has no effect. Consider using `--directory` instead.
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The project is created at the positional path:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Project flag warned without path

<!-- Derived from [`init::init_project_flag_is_warned_without_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L4139-L4158) -->

The `--project` flag is deprecated and shows a warning when used without a path.

```console
$ uv init --project bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Use of the `--project` option in `uv init` is deprecated and will be removed in a future release. Consider using `uv init <PATH>` instead.
Initialized project `bar`
```

The project is created:

```console
$ test -f bar/pyproject.toml && echo "exists"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```
