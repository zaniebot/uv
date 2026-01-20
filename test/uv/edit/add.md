# Dependency Management - Adding Dependencies

Tests for adding dependencies to a project using `uv add`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Registry

<!-- Derived from [`edit::add_registry`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L29-L139) -->

`uv add` adds a dependency from PyPI to the project.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The dependency is added to pyproject.toml:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio==3.7.0",
]
```

## Unnamed

<!-- Derived from [`edit::add_unnamed`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L1214-L1303) -->

`uv add` can add a Git dependency without an explicit name.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add git+https://github.com/astral-test/uv-public-pypackage --tag=0.0.1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

The dependency name is inferred and added with source configuration:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "uv-public-pypackage",
]

[tool.uv.sources]
uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1" }
```

The project can be synced:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 1 package in [TIME]
```

## Repeat

<!-- Derived from [`edit::add_repeat`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L5440-L5513) -->

Adding a dependency that already exists is a no-op.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

The dependency is added with a lower bound:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio>=4.3.0",
]
```

Adding the same dependency again is a no-op:

```console
$ uv add anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Audited 3 packages in [TIME]
```

The pyproject.toml is unchanged:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio>=4.3.0",
]
```

## Frozen

<!-- Derived from [`edit::add_frozen`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4286-L4332) -->

The `--frozen` flag skips locking and syncing.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --frozen anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
```

The dependency is added but no lock file or environment is created:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio==3.7.0",
]
```

No lock file exists:

```console
$ test -f uv.lock && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

## No sync

<!-- Derived from [`edit::add_no_sync`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4334-L4383) -->

The `--no-sync` flag creates a lock file but skips environment installation.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --no-sync anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 4 packages in [TIME]
```

The dependency is added and locked:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio==3.7.0",
]
```

Lock file exists:

```console
$ test -f uv.lock && echo "exists"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```

## Error

<!-- Derived from [`edit::add_error`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4453-L4490) -->

When adding a non-existent package, changes are not persisted.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add xyz
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because there are no versions of xyz and your project depends on xyz, we can conclude that your project's requirements are unsatisfiable.
  help: If you want to add the package regardless of the failed resolution, provide the `--frozen` flag to skip locking and syncing.
```

Using `--frozen` allows adding despite resolution failure:

```console
$ uv add --frozen xyz
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

No lock file is created:

```console
$ test -f uv.lock && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

## Environment YML error

<!-- Derived from [`edit::add_environment_yml_error`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4492-L4525) -->

Conda environment.yml files are not supported.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```yaml
# file: environment.yml
name: test-env
channels:
  - conda-forge
dependencies:
  - python>=3.12
```

```console
$ uv add -r environment.yml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Conda environment files (i.e., `environment.yml`) are not supported
```

## Ambiguous

<!-- Derived from [`edit::add_ambiguous`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L13883-L13925) -->

When a package appears multiple times, adding fails with an ambiguity error.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "0.1.0"
requires-python = ">=3.12.0"
dependencies = [
    "anyio>=4.0.0",
    "anyio>=4.1.0",
]
[dependency-groups]
bar = ["anyio>=4.1.0", "anyio>=4.2.0"]
```

```console
$ uv add anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Cannot perform ambiguous update; found multiple entries for `anyio`:
- `anyio>=4.0.0`
- `anyio>=4.1.0`
```

The error also applies to dependency groups:

```console
$ uv add --group bar anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Cannot perform ambiguous update; found multiple entries for `anyio`:
- `anyio>=4.1.0`
- `anyio>=4.2.0`
```

## Self

<!-- Derived from [`edit::add_self`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L11388-L11468) -->

Adding a dependency with the same name as the project is not allowed.

```toml
# file: pyproject.toml
[project]
name = "anyio"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add anyio==3.7.0
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Requirement name `anyio` matches project name `anyio`, but self-dependencies are not permitted without the `--dev` or `--optional` flags. If your project name (`anyio`) is shadowing that of a third-party dependency, consider renaming the project.
```

However, recursive extras are allowed:

```toml
# file: pyproject.toml
[project]
name = "anyio"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
types = ["typing-extensions>=4"]
```

```console
$ uv add --optional all "anyio[types]"
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + typing-extensions==4.10.0
```

The optional dependency is added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "anyio"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
all = [
    "anyio[types]",
]
types = [
    "typing-extensions>=4",
]
```

## Shadowed name

<!-- Derived from [`edit::add_shadowed_name`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L9605-L9652) -->

When a project name shadows a dependency name, resolution fails with a helpful error.

```toml
# file: pyproject.toml
[project]
name = "dagster"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add dagster-webserver==1.6.13
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because dagster-webserver==1.6.13 depends on your project and your project depends on dagster-webserver==1.6.13, we can conclude that your project's requirements are unsatisfiable.

      hint: The package `dagster-webserver` depends on the package `dagster` but the name is shadowed by your project. Consider changing the name of the project.
  help: If you want to add the package regardless of the failed resolution, provide the `--frozen` flag to skip locking and syncing.
```

The same error appears with a version range:

```console
$ uv add "dagster-webserver>=1.6.11,<1.7.0"
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because only the following versions of dagster-webserver are available:
          dagster-webserver<=1.6.11
          dagster-webserver==1.6.12
          dagster-webserver==1.6.13
      and dagster-webserver==1.6.11 depends on your project, we can conclude that dagster-webserver>=1.6.11,<1.6.12 depends on your project.
      And because dagster-webserver==1.6.12 depends on your project, we can conclude that dagster-webserver>=1.6.11,<1.6.13 depends on your project.
      And because dagster-webserver==1.6.13 depends on your project and your project depends on dagster-webserver>=1.6.11, we can conclude that your project's requirements are unsatisfiable.

      hint: The package `dagster-webserver` depends on the package `dagster` but the name is shadowed by your project. Consider changing the name of the project.
  help: If you want to add the package regardless of the failed resolution, provide the `--frozen` flag to skip locking and syncing.
```

## Update

<!-- from edit.rs::update -->

`uv add` can update an existing dependency to add extras or markers.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["requests==2.31.0"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
```

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

Enable an extra (note the version specifier is preserved):

```console
$ uv add "requests[security]"
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Audited 5 packages in [TIME]
```

The dependency is updated with the extra:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "requests[security]==2.31.0",
]
```

## Git error

```toml
# mdtest

[environment]
required-features = ["pypi", "git"]
```

<!-- from edit.rs::add_git_error -->

Git reference options require a Git URL.

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

Providing a tag without a Git source fails:

```console
$ uv add flask --tag 0.0.1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a Git repository, but a Git reference (`--tag 0.0.1`) was provided.
```

Providing a branch with a non-Git URL fails:

```console
$ uv add "flask @ https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl" --branch 0.0.1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a Git repository, but a Git reference (`--branch 0.0.1`) was provided.
```

## Git branch

<!-- from edit.rs::add_git_branch -->

Add a Git dependency with a branch reference.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add "uv-public-pypackage @ git+https://github.com/astral-test/uv-public-pypackage" --branch test-branch
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

## Raw sources errors

### Tag with raw sources

<!-- from edit.rs::add_raw_error -->

Using `--tag` with `--raw-sources` should fail.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add "uv-public-pypackage @ git+https://github.com/astral-test/uv-public-pypackage" --tag 0.0.1 --raw-sources
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--tag <TAG>' cannot be used with '--raw'

Usage: uv add --cache-dir [CACHE_DIR] --tag <TAG> --exclude-newer <EXCLUDE_NEWER> <PACKAGES|--requirements <REQUIREMENTS>>

For more information, try '--help'.
```

## Editable errors

### Editable with non-local source

<!-- from edit.rs::add_editable_error -->

Using `--editable` with a non-local source should fail.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add "flask @ https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl" --editable
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a local directory, but the `--editable` flag was provided. Editable installs are only supported for local directories.
```

## Resolution errors

### Add non-existent package

<!-- from edit.rs::add_error -->

Adding a package that doesn't exist shows an error with a helpful hint.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add xyz
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because there are no versions of xyz and your project depends on xyz, we can conclude that your project's requirements are unsatisfiable.
  help: If you want to add the package regardless of the failed resolution, provide the `--frozen` flag to skip locking and syncing.
```

### Add non-existent package with frozen

<!-- from edit.rs::add_error -->

Using `--frozen` allows adding a package even when resolution fails.

```console
$ uv add xyz --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Lower bounds

### Add with lower bound

<!-- from edit.rs::add_lower_bound -->

Adding an unconstrained dependency sets a lower bound automatically.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

## Unsupported files

### Add from Conda environment.yml

<!-- from edit.rs::add_environment_yml_error -->

Adding dependencies from a Conda environment.yml file shows an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```yaml environment.yml
name: test-env
channels:
  - conda-forge
dependencies:
  - python>=3.12
```

```console
$ uv add -r environment.yml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Conda environment files (i.e., `environment.yml`) are not supported
```

## Inexact resolution

### Add removes stale packages

<!-- from edit.rs::add_inexact -->

Adding a new dependency properly removes stale packages from a modified lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio == 3.7.0"]
```

Lock the project:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Manually remove the dependency:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

Add a new dependency:

```console
$ uv add iniconfig==2.0.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

## Validation errors

### Invalid ignore-error-codes

<!-- from edit.rs::add_invalid_ignore_error_code -->

Using an invalid HTTP status code in ignore-error-codes produces an error.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.11, <4"
dependencies = []
[[tool.uv.index]]
name = "my-index"
url = "https://pypi-proxy.fly.dev/basic-auth/simple"
ignore-error-codes = [401, 403, 1234]
```

```console
$ uv add anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 9, column 22
    |
  9 | ignore-error-codes = [401, 403, 1234]
    |                      ^^^^^^^^^^^^^^^^
  1234 is not a valid HTTP status code

error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 9, column 22
  |
9 | ignore-error-codes = [401, 403, 1234]
  |                      ^^^^^^^^^^^^^^^^
1234 is not a valid HTTP status code
```

### Invalid requires-python specifier

<!-- from edit.rs::add_invalid_requires_python -->

Using an invalid requires-python specifier produces a helpful error.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = "3.12"
dependencies = []
```

```console
$ uv add anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 4, column 19
  |
4 | requires-python = "3.12"
  |                   ^^^^^^
Failed to parse version: Unexpected end of version specifier, expected operator. Did you mean `==3.12`?:
3.12
^^^^
```
