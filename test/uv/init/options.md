# Project Initialization - Options

Tests for various options during `uv init`.

```toml
#! mdtest

[environment]
python-version = "3.12"
```

## No readme

<!-- Derived from [`init::init_no_readme`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1061-L1090) -->

The `--no-readme` flag skips creating a README file.

```console
$ uv init --no-readme foo
Initialized project `foo` at `[TEMP_DIR]/foo`
```

No readme field in pyproject.toml:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
requires-python = ">=3.12"
dependencies = []
```

No README.md file is created:

```console
$ test -f foo/README.md && echo "exists" || echo "missing"
missing
```

## No pin Python

<!-- Derived from [`init::init_no_pin_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1092-L1123) -->

The `--no-pin-python` flag skips creating a .python-version file.

```console
$ uv init --no-pin-python foo
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The pyproject.toml is created normally:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

No .python-version file is created:

```console
$ test -f foo/.python-version && echo "exists" || echo "missing"
missing
```

## With author

<!-- Derived from [`init::init_with_author`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2932-L3061) -->

The `--author-from` flag controls whether to include author information from Git config.

Initialize a Git repository with author configuration:

```console
$ git init
```

```console
$ git config --local user.name Alice
```

```console
$ git config --local user.email alice@example.com
```

By default, authors is not filled for non-package applications:

```console
$ uv init foo
Initialized project `foo` at `[TEMP_DIR]/foo`
```

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

Use `--author-from auto` to explicitly fill it:

```console
$ uv init --author-from auto bar
Initialized project `bar` at `[TEMP_DIR]/bar`
```

```toml title="bar/pyproject.toml" snapshot=true
[project]
name = "bar"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
authors = [
    { name = "Alice", email = "alice@example.com" }
]
requires-python = ">=3.12"
dependencies = []
```

Authors are filled for libraries by default:

```console
$ uv init --lib baz
Initialized project `baz` at `[TEMP_DIR]/baz`
```

```toml title="baz/pyproject.toml" snapshot=true
[project]
name = "baz"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
authors = [
    { name = "Alice", email = "alice@example.com" }
]
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

Use `--author-from none` to prevent it:

```console
$ uv init --lib --author-from none qux
Initialized project `qux` at `[TEMP_DIR]/qux`
```

```toml title="qux/pyproject.toml" snapshot=true
[project]
name = "qux"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

## With description

<!-- Derived from [`init::init_with_description`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3912-L3953) -->

The `--description` flag sets a custom description.

```console working-dir="foo"
$ uv init --description "A sample project description" --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The custom description is in pyproject.toml:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "A sample project description"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

## Without description

<!-- Derived from [`init::init_without_description`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3955-L3995) -->

Without `--description`, a default description is used.

```console working-dir="bar"
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `bar`
```

The default description is in pyproject.toml:

```toml title="bar/pyproject.toml" snapshot=true
[project]
name = "bar"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["uv_build>=[CURRENT_VERSION],<[NEXT_BREAKING]"]
build-backend = "uv_build"
```

## Isolated

<!-- Derived from [`init::init_isolated`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1798-L1846) -->

The `--isolated` flag is deprecated but still adds to workspace.

```toml
#! file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

```console working-dir="foo"
$ uv init --isolated
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `--isolated` flag is deprecated and has no effect. Instead, use `--no-config` to prevent uv from discovering configuration files or `--no-workspace` to prevent uv from adding the initialized project to the containing workspace.
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo`
```

The workspace is updated:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = [
    "foo",
]
```

## Unmanaged

<!-- Derived from [`init::init_unmanaged`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2653-L2686) -->

When the parent workspace has `managed = false`, projects are not added to the workspace.

```toml
#! file: pyproject.toml
[tool.uv]
managed = false
```

```console
$ uv init foo
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The workspace configuration remains unchanged:

```toml title="pyproject.toml" snapshot=true
[tool.uv]
managed = false
```
