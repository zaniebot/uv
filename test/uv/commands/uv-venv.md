# Virtual Environments

Tests for creating and managing virtual environments with `uv venv`.

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false

[tree]
exclude = ["cache"]
```

## Basic Creation

### Creating a virtual environment

<!-- Derived from [`venv::create_venv`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L14-L71) -->

Create a virtual environment at `.venv`.

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Creating again without `--clear` shows a warning about the existing venv.

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
warning: A virtual environment already exists at `.venv`. In the future, uv will require `--clear` to replace it
Activate with: source .venv/[BIN]/activate
```

Using `--clear` replaces the existing virtual environment without warning.

```console
$ uv venv .venv --clear --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

The virtual environment should have the standard structure:

```tree depth=2
.
└── .venv/
    ├── .gitignore
    ├── CACHEDIR.TAG
    ├── [BIN]/
    ├── [LIB]/
    └── pyvenv.cfg
```

### Default location

<!-- Derived from [`venv::create_venv_defaults_to_cwd`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L268-L286) -->

When no path is provided, the virtual environment is created at `.venv`.

```console
$ uv venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Projects without a `[project]` section

<!-- Derived from [`venv::virtual_empty`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L213-L237) -->

`uv venv` works with a pyproject.toml that has no `[project]` section.

```toml
# file: pyproject.toml

[tool.mycooltool]
wow = "someconfig"
```

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Virtual projects with dependency groups

<!-- Derived from [`venv::virtual_dependency_group`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L239-L266) -->

`uv venv` works with a virtual project (no `[project]` but has `[dependency-groups]`).

```toml
# file: pyproject.toml

[dependency-groups]
foo = ["sortedcontainers"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Creating with Python 3.13

<!-- Derived from [`venv::create_venv_313`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L74-L93) -->

`uv venv` works with Python 3.13.

```toml
# mdtest

[environment]
python-version = "3.13"
```

```console
$ uv venv .venv --python 3.13
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.13.[X] interpreter at: [PYTHON-3.13]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Creating with specific Python patch version

<!-- Derived from [`venv::create_venv_python_patch`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L970-L990) -->

`uv venv` works with a specific Python patch version when the `python-patch` feature is enabled.

```toml
# mdtest

[environment]
python-versions = "3.12.9"
required-features = "python-patch"
```

```console
$ uv venv .venv --python 3.12.9
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.9 interpreter at: [PYTHON-3.12.9]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

## Python Version Discovery

### VIRTUAL_ENV is ignored

<!-- Derived from [`venv::create_venv_ignores_virtual_env_variable`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L288-L305) -->

`VIRTUAL_ENV` pointing to a non-existent directory is ignored since venv creation requires a system
interpreter.

```toml
# mdtest

[environment]
env = { VIRTUAL_ENV = "does-not-exist" }
```

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Missing pyproject.toml metadata is ignored

<!-- Derived from [`venv::create_venv_ignores_missing_pyproject_metadata`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L748-L770) -->

A pyproject.toml without a `[project]` section is ignored for Python version discovery.

```toml
# file: pyproject.toml

[tool.no.project.here]
```

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Invalid `pyproject.toml` produces a warning

<!-- Derived from [`venv::create_venv_warns_user_on_requires_python_discovery_error`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L772-L808) -->

An invalid pyproject.toml produces a warning but venv creation still succeeds.

```text
# file: pyproject.toml

invalid toml
```

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 1, column 9
    |
  1 | invalid toml
    |         ^
  key with no value, expected `=`

warning: Failed to parse `pyproject.toml` during environment creation:
  TOML parse error at line 1, column 9
    |
  1 | invalid toml
    |         ^
  key with no value, expected `=`

Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Respecting `pyproject.toml` `requires-python`

<!-- Derived from [`venv::create_venv_respects_pyproject_requires_python`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L14-L196) -->

The `requires-python` setting in `pyproject.toml` is used to select an appropriate Python version.

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.9", "3.10", "3.12"]
```

Without a Python requirement, we use the first on the PATH (3.11):

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `requires-python = "<3.11"`, we select the first compatible version (3.9):

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = "<3.11"
dependencies = []
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `requires-python = "==3.11.*"`, we select the exact version (3.11):

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = "==3.11.*"
dependencies = []
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `requires-python = ">=3.11,<3.12"`, we select 3.11 (only version in range):

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.11,<3.12"
dependencies = []
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `requires-python = ">=3.11"`, we select the first compatible version (3.11):

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.11"
dependencies = []
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `requires-python = ">3.11"`, we select 3.11 (3.11.x satisfies >3.11.0):

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = ">3.11"
dependencies = []
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `requires-python = ">=3.12"`, we select 3.12:

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

When an explicit `--python` is incompatible with `requires-python`, we warn but proceed:

```console
$ uv venv --clear --python 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
warning: The requested interpreter resolved to Python 3.11.[X], which is incompatible with the project's Python requirement: `>=3.12` (from `project.requires-python`)
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Respecting group `requires-python`

<!-- Derived from [`venv::create_venv_respects_group_requires_python`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L570-L807) -->

The `dev` group's `requires-python` setting is respected when creating a virtual environment.

```toml
# mdtest

[environment]
python-versions = ["3.9", "3.10", "3.11", "3.12"]
```

Without a Python requirement, we use the first on the PATH (3.9):

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `requires-python = ">=3.10"` on the default group, Python 3.10 is selected. Note that
non-default groups (like `other` with `>=3.12`) are NOT consulted:

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
dependencies = []

[dependency-groups]
dev = ["sortedcontainers"]
other = ["sniffio"]

[tool.uv.dependency-groups]
dev = {requires-python = ">=3.10"}
other = {requires-python = ">=3.12"}
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.10.[X] interpreter at: [PYTHON-3.10]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

When the top-level `requires-python` and default group `requires-python` both apply, their
intersection is used. Here the top-level (`>=3.11`) wins:

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.11"
dependencies = []

[dependency-groups]
dev = ["sortedcontainers"]
other = ["sniffio"]

[tool.uv.dependency-groups]
dev = {requires-python = ">=3.10"}
other = {requires-python = ">=3.12"}
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

When the group's constraint is stricter, the group wins. Here `>=3.11` from the group is stricter
than `>=3.10` from the top-level:

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.10"
dependencies = []

[dependency-groups]
dev = ["sortedcontainers"]
other = ["sniffio"]

[tool.uv.dependency-groups]
dev = {requires-python = ">=3.11"}
other = {requires-python = ">=3.12"}
```

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

We warn if we receive an incompatible version via explicit `--python`:

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
dependencies = []

[dependency-groups]
dev = ["sortedcontainers"]

[tool.uv.dependency-groups]
dev = {requires-python = ">=3.12"}
```

```console
$ uv venv --clear --python 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
warning: The requested interpreter resolved to Python 3.11.[X], which is incompatible with the project's Python requirement: `>=3.12` (from `tool.uv.dependency-groups.dev.requires-python`).
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

We error if there's no compatible version between top-level and group constraints. Note that
non-default groups are NOT consulted here:

```toml
# file: pyproject.toml

[project]
name = "foo"
version = "1.0.0"
requires-python = "<3.12"
dependencies = []

[dependency-groups]
dev = ["sortedcontainers"]
other = ["sniffio"]

[tool.uv.dependency-groups]
dev = {requires-python = ">=3.12"}
other = {requires-python = ">=3.11"}
```

```console
$ uv venv --clear --python 3.11
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Found conflicting Python requirements:
- foo: <3.12
- foo:dev: >=3.12
```

### Python preference (managed vs system)

<!-- Derived from [`venv::venv_python_preference`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L169-L232) -->

When managed Python interpreters are available, uv prefers them over system interpreters by default.

```toml
# mdtest

[environment]
python-versions = ["3.11"]
managed-python-versions = ["3.12"]
```

By default, uv uses the managed Python (3.12):

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

With `--no-managed-python`, uv uses the system Python (3.11):

```console
$ uv venv --no-managed-python
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
warning: A virtual environment already exists at `.venv`. In the future, uv will require `--clear` to replace it
Activate with: source .venv/[BIN]/activate
```

Running again with `--no-managed-python` continues to use system Python:

```console
$ uv venv --no-managed-python
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment at: .venv
warning: A virtual environment already exists at `.venv`. In the future, uv will require `--clear` to replace it
Activate with: source .venv/[BIN]/activate
```

Without the flag, uv goes back to using managed Python:

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X]
Creating virtual environment at: .venv
warning: A virtual environment already exists at `.venv`. In the future, uv will require `--clear` to replace it
Activate with: source .venv/[BIN]/activate
```

With `--managed-python`, uv explicitly uses managed Python:

```console
$ uv venv --managed-python
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X]
Creating virtual environment at: .venv
warning: A virtual environment already exists at `.venv`. In the future, uv will require `--clear` to replace it
Activate with: source .venv/[BIN]/activate
```

### Unknown Python minor version

<!-- Derived from [`venv::create_venv_unknown_python_minor`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L917-L941) -->

Requesting an unknown Python minor version produces an error.

```toml
# mdtest

[environment]
env-remove = ["UV_TEST_PYTHON_PATH"]

[filters]
python-sources = true
```

```console
$ uv venv .venv --python 3.100
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.100 in [PYTHON SOURCES]
```

### Unknown Python patch version

<!-- Derived from [`venv::create_venv_unknown_python_patch`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L943-L968) -->

Requesting an unknown Python patch version produces an error.

```toml
# mdtest

[environment]
env-remove = ["UV_TEST_PYTHON_PATH"]

[filters]
python-sources = true
```

```console
$ uv venv .venv --python 3.12.100
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.12.[X] in [PYTHON SOURCES]
```

## Existing Directory Handling

### File already exists at target path

<!-- Derived from [`venv::file_exists`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L992-L1016) -->

Creating a venv where a file already exists fails.

```text
# file: .venv

not a directory
```

```console
$ uv venv .venv --python 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
error: Failed to create virtual environment
  Caused by: File exists at `.venv`
```

### Empty directory exists

<!-- Derived from [`venv::empty_dir_exists`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1018-L1042) -->

Creating a venv where an empty directory already exists succeeds.

```tree create=true
.
└── .venv/
```

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Non-empty directory exists

<!-- Derived from [`venv::non_empty_dir_exists`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1044-L1087) -->

Creating a venv in a non-empty directory fails without `--clear`.

```text
# file: .venv/file

some content
```

```console
$ uv venv .venv --python 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
error: Failed to create virtual environment
  Caused by: A directory already exists at: .venv

hint: Use the `--clear` flag or set `UV_VENV_CLEAR=1` to replace the existing directory
```

With `--clear`, the existing directory is replaced.

```console
$ uv venv .venv --clear --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Using --allow-existing

<!-- Derived from [`venv::non_empty_dir_exists_allow_existing`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1089-L1151) -->

Using `--allow-existing` allows creating a venv in a non-empty directory.

```text
# file: .venv/file

some content
```

```console
$ uv venv .venv --allow-existing --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Running again also succeeds, overwriting existing symlinks.

```console
$ uv venv .venv --allow-existing --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Running --allow-existing after initial creation

<!-- Derived from [`venv::create_venv_then_allow_existing`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1153-L1184) -->

Running `uv venv` followed by `uv venv --allow-existing` works.

```console
$ uv venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

```console
$ uv venv --allow-existing
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Using --no-clear with existing directory

<!-- Derived from [`venv::no_clear_with_existing_directory`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1772-L1811) -->

`--no-clear` fails if a virtual environment already exists.

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

```console
$ uv venv .venv --no-clear --python 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
error: Failed to create virtual environment
  Caused by: A virtual environment already exists at: .venv

hint: Use the `--clear` flag or set `UV_VENV_CLEAR=1` to replace the existing virtual environment
```

### Using --no-clear with non-existent directory

<!-- Derived from [`venv::no_clear_with_non_existent_directory`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1813-L1835) -->

`--no-clear` succeeds when the directory doesn't exist.

```console
$ uv venv .venv --no-clear --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Using --no-clear overrides --clear

<!-- Derived from [`venv::no_clear_overrides_clear`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1837-L1865) -->

`--no-clear` overrides `--clear` when both are provided.

```text
# file: .venv/file

some content
```

```console
$ uv venv .venv --clear --no-clear --python 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
error: Failed to create virtual environment
  Caused by: A directory already exists at: .venv

hint: Use the `--clear` flag or set `UV_VENV_CLEAR=1` to replace the existing directory
```

### Using --no-clear conflicts with --allow-existing

<!-- Derived from [`venv::no_clear_conflicts_with_allow_existing`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1867-L1890) -->

`--no-clear` and `--allow-existing` are mutually exclusive.

```console
$ uv venv .venv --no-clear --allow-existing --python 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--no-clear' cannot be used with '--allow-existing'

Usage: uv venv --python <PYTHON> --exclude-newer <EXCLUDE_NEWER> <PATH>

For more information, try '--help'.
```

## Configuration (pyvenv.cfg)

### Verifying pyvenv.cfg contents

<!-- Derived from [`venv::verify_pyvenv_cfg`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1234-L1250) -->

The `pyvenv.cfg` file should contain the uv version.

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

```text title=".venv/pyvenv.cfg" assert=contains
uv =
```

### Relocatable virtual environment

<!-- Derived from [`venv::verify_pyvenv_cfg_relocatable`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1253-L1305) -->

The `--relocatable` flag creates a virtual environment that can be moved to a different location.

```console
$ uv venv .venv --relocatable --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

The `pyvenv.cfg` should contain `relocatable = true`:

```text title=".venv/pyvenv.cfg" assert=contains
relocatable = true
```

### Nested virtual environment uses same home

<!-- Derived from [`venv::verify_nested_pyvenv_cfg`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L272-L321) -->

When creating a virtual environment while `VIRTUAL_ENV` points to an existing venv, the new venv's
`home` should point to the same Python installation (not the parent venv).

```toml
# mdtest

[environment]
env = { VIRTUAL_ENV = ".venv" }

[filters]
pyvenv-cfg = true
```

First, create a parent virtual environment. `VIRTUAL_ENV=.venv` is set but `.venv` doesn't exist
yet, so it's ignored:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Now create a nested virtual environment. Since `VIRTUAL_ENV=.venv` and `.venv` exists, uv detects
we're "inside" a venv:

```console
$ uv venv .subvenv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .subvenv
Activate with: source .subvenv/[BIN]/activate
```

Both `pyvenv.cfg` files should have the same `home` directory (pointing to the real Python, not the
parent venv). The `pyvenv-cfg` filter normalizes the paths:

```text
# file: .venv/pyvenv.cfg

home = [PYTHON-DIR]
implementation = CPython
uv = [UV-VERSION]
version_info = 3.12.[X]
include-system-site-packages = false
```

```text
# file: .subvenv/pyvenv.cfg

home = [PYTHON-DIR]
implementation = CPython
uv = [UV-VERSION]
version_info = 3.12.[X]
include-system-site-packages = false
```

## Seeding

### Seed packages

<!-- Derived from [`venv::seed`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L837-L857) -->

The `--seed` flag installs seed packages (pip) into the virtual environment.

```toml
# mdtest

[filters]
counts = true
```

```console
$ uv venv .venv --seed --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment with seed packages at: .venv
 + pip==24.0
Activate with: source .venv/[BIN]/activate
```

### Seed packages with older Python version

<!-- Derived from [`venv::seed_older_python_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/venv.rs#L11-L35) -->

When seeding with Python 3.11, older versions of pip, setuptools, and wheel are installed.

```toml
# mdtest

[environment]
python-versions = ["3.11"]

[filters]
counts = true
```

```console
$ uv venv .venv --seed --python 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.11.[X] interpreter at: [PYTHON-3.11]
Creating virtual environment with seed packages at: .venv
 + pip==24.0
 + setuptools==69.2.0
 + wheel==0.43.0
Activate with: source .venv/[BIN]/activate
```

## Working Directory

### Creating a virtual environment in the current directory (Unix)

<!-- Derived from [`venv::create_venv_current_working_directory`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1690-L1729) -->

On Unix, creating a virtual environment in the current working directory using `.` as the path
should work.

```toml
# mdtest

[environment]
target-family = "unix"
```

First, create a virtual environment at `.venv`:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Then, from within that directory, create a venv at `.` (the current directory):

```console working-dir=".venv"
$ uv venv . --clear --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .
Activate with: source [BIN]/activate
```

### Creating a virtual environment in the current directory (Windows)

<!-- Derived from [`venv::create_venv_current_working_directory`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L1731-L1770) -->

On Windows, creating a virtual environment in the current working directory fails because you cannot
delete the current working directory.

```toml
# mdtest

[environment]
target-family = "windows"
```

First, create a virtual environment at `.venv`:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Then, from within that directory, attempting to create a venv at `.` fails:

```console working-dir=".venv"
$ uv venv . --clear --python 3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .
error: Failed to create virtual environment
  Caused by: failed to remove directory `[VENV]/`: The process cannot access the file because it is being used by another process. (os error 32)
```

## Symlink Handling

### Symlink preservation with --clear

<!-- Derived from [`venv::create_venv_symlink_clear_preservation`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L236-L290) -->

When creating a virtual environment at a symlink path, the symlink should be preserved (not replaced
with a real directory), even when using `--clear`.

```toml
# mdtest

[environment]
target-family = "unix"
```

Create a target directory and a symlink pointing to it:

```tree create=true
.
├── target/
└── .venv -> target
```

Create a virtual environment at the symlink location:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

The symlink should still be preserved:

```tree depth=1
.
├── .venv -> target
└── target/
```

Run `uv venv` with `--clear` to test symlink preservation during clear:

```console
$ uv venv .venv --clear --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

The symlink should still be preserved after `--clear`:

```tree depth=1
.
├── .venv -> target
└── target/
```

### Symlink preservation on recreation

<!-- Derived from [`venv::create_venv_symlink_recreate_preservation`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L292-L348) -->

When recreating a virtual environment at a symlink path without `--clear`, the symlink should be
preserved.

```toml
# mdtest

[environment]
target-family = "unix"
```

Create a target directory and a symlink pointing to it:

```tree create=true
.
├── target/
└── .venv -> target
```

Create a virtual environment at the symlink location:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

The symlink should be preserved after first creation:

```tree depth=1
.
├── .venv -> target
└── target/
```

Run `uv venv` again WITHOUT `--clear` to test recreation behavior:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
warning: A virtual environment already exists at `.venv`. In the future, uv will require `--clear` to replace it
Activate with: source .venv/[BIN]/activate
```

The symlink should still be preserved after recreation:

```tree depth=1
.
├── .venv -> target
└── target/
```

### Nested symlink preservation

<!-- Derived from [`venv::create_venv_nested_symlink_preservation`](https://github.com/astral-sh/uv/blob/c83066b8ee71432543ec3ff183bec4681beca2e7/crates/uv/tests/it/venv.rs#L350-L413) -->

When creating a virtual environment at a nested symlink path (symlink pointing to another symlink),
both symlinks should be preserved.

```toml
# mdtest

[environment]
target-family = "unix"
```

Create a target directory and nested symlinks:

```tree create=true
.
├── target/
├── intermediate -> target
└── .venv -> intermediate
```

Create a virtual environment at the nested symlink location:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Both symlinks should be preserved:

```tree depth=1
.
├── .venv -> intermediate
├── intermediate -> target
└── target/
```

Run `uv venv` again to test nested symlink preservation during recreation:

```console
$ uv venv .venv --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
warning: A virtual environment already exists at `.venv`. In the future, uv will require `--clear` to replace it
Activate with: source .venv/[BIN]/activate
```

Both nested symlinks should still be preserved:

```tree depth=1
.
├── .venv -> intermediate
├── intermediate -> target
└── target/
```

## Platform-Specific

### Windows shims

<!-- Derived from [`venv::windows_shims`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/venv.rs#L37-L82) -->

On Windows, uv should correctly follow `.bat` shim scripts to find the Python installation.

```toml
# mdtest

[environment]
target-family = "windows"
python-versions = ["3.10", "3.9"]
```

Create a shim directory with a `python.bat` script that forwards to Python 3.9:

```bat
# file: shim/python.bat

@echo off
python3.9 %*
```

Run `uv venv` with the shim directory in `UV_TEST_PYTHON_PATH`. The shim should be followed and
Python 3.9 should be used:

```console
$ UV_TEST_PYTHON_PATH="shim;${UV_TEST_PYTHON_PATH}" uv venv .venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.9.[X] interpreter at: [PYTHON-3.9]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

### Path with trailing space error

<!-- Derived from [`venv::path_with_trailing_space_gives_proper_error`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/venv.rs#L84-L112) -->

On Windows, setting `UV_CACHE_DIR` to a path with a trailing space should produce a clear error
message.

```toml
# mdtest

[environment]
target-family = "windows"
python-versions = ["3.12"]
```

```console
$ UV_CACHE_DIR="${UV_CACHE_DIR} " uv venv
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to initialize cache at `[CACHE_DIR]/ `
  Caused by: failed to open file `[CACHE_DIR]/ /CACHEDIR.TAG`: The system cannot find the path specified. (os error 3)
```

### Shell activation with apostrophe in path

<!-- Derived from [`venv::create_venv_apostrophe`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/venv.rs#L114-L166) -->

On Linux, creating a virtual environment in a directory with an apostrophe in its name should work,
and the activation script should function correctly.

```toml
# mdtest

[environment]
target-os = "linux"
python-versions = ["3.12"]
```

```console
$ uv venv "Testing's" --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: Testing's
Activate with: source Testing's/[BIN]/activate
```

The activation script should work correctly with the apostrophe in the path:

```console working-dir="Testing's"
$ bash -c ". bin/activate && python -c 'import sys; print(sys.prefix)'"
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/Testing's

----- stderr -----
```
