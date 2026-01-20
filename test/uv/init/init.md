# Project Initialization - Basic

Tests for basic `uv init` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Basic init

<!-- Derived from [`init::init`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L15-L65) -->

Running `uv init` creates a basic project structure.

```console working-dir="foo"
$ uv init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The generated pyproject.toml:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

The project can be locked:

```console working-dir="foo"
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 1 package in [TIME]
```

A .python-version file is created:

```text title="foo/.python-version" snapshot=true
3.12
```

## Bare init

<!-- Derived from [`init::init_bare`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L68-L112) -->

The `--bare` flag creates a minimal project without extra files.

```console working-dir="foo"
$ uv init --bare
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

Only pyproject.toml is created:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

## Dot args

<!-- Derived from [`init::init_dot_args`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L114-L155) -->

Using `.` as the project name initializes in the current directory.

```console
$ uv init .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `[TEMP_DIR]`
```

Project is created at the current directory:

```toml title="pyproject.toml" snapshot=true
[project]
name = "[TEMP_DIR_NAME]"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Normalized names

<!-- Derived from [`init::init_normalized_names`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L157-L220) -->

Project names are normalized according to PEP 503.

```console working-dir="foo-bar"
$ uv init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo-bar`
```

The normalized name is used in pyproject.toml:

```toml title="foo-bar/pyproject.toml" snapshot=true
[project]
name = "foo-bar"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Hidden directories

<!-- Derived from [`init::init_hidden`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L222-L269) -->

Hidden directories (starting with `.`) are handled correctly.

```console working-dir=".foo"
$ uv init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `.foo`
```

The project name includes the dot:

```toml title=".foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Non-ASCII directory names

<!-- Derived from [`init::init_non_ascii_directory`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L271-L318) -->

Non-ASCII characters in directory names are normalized.

```console working-dir="foo-β"
$ uv init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo-β`
```

The name is normalized:

```toml title="foo-β/pyproject.toml" snapshot=true
[project]
name = "foo-beta"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Init with cache

<!-- Derived from [`init::init_cache`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L320-L364) -->

Initializing a project works with custom cache directories.

```console working-dir="foo"
$ uv init --cache-dir .cache
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
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
