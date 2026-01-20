# Project Initialization - Python Requirements

Tests for Python version requirements during `uv init`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Requires-python version

<!-- Derived from [`init::init_requires_python_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2402-L2456) -->

The `--python` flag sets the `requires-python` field in pyproject.toml.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = []
```

```console
$ uv init --python 3.9 foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The requires-python uses the specified version:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.9"
dependencies = []
```

The .python-version file is created:

```text title="foo/.python-version" snapshot=true
3.9
```

## Requires-python specifiers

<!-- Derived from [`init::init_requires_python_specifiers`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2458-L2515) -->

The `--python` flag preserves exact specifiers verbatim.

```toml
# mdtest

[environment]
python-versions = ["3.9", "3.12"]
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = []
```

```console
$ uv init --python "==3.9.*" foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The exact specifier is preserved:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = "==3.9.*"
dependencies = []
```

The .python-version file uses the resolved version:

```text title="foo/.python-version" snapshot=true
3.9
```

## Requires-python version file

<!-- Derived from [`init::init_requires_python_version_file`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2517-L2552) -->

`uv init` infers `requires-python` from an existing .python-version file.

```toml
# mdtest

[environment]
python-versions = ["3.9", "3.12"]
```

```text
# file: .python-version
3.9
```

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The requires-python is inferred from the .python-version file:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.9"
dependencies = []
```

## Python variant

<!-- Derived from [`init::init_python_variant`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3999-L4012) -->

The `--python` flag supports Python variants like freethreaded.

```toml
# mdtest

[environment]
python-version = "3.13"
```

```console
$ uv init --python 3.13t foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The .python-version file includes the variant:

```text title="foo/.python-version" snapshot=true
3.13+freethreaded
```
