# Sync Build Options

Tests for `--no-binary` and `--no-build` options in `uv sync`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## No binary

### Sync with no-binary-package

<!-- from sync.rs::no_binary -->

The `--no-binary-package` flag forces building from source for a specific package.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --no-binary-package iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Sync with no-binary (all)

<!-- from sync.rs::no_binary -->

The `--no-binary` flag forces building from source for all packages.

```console
$ uv sync --reinstall --no-binary
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 ~ iniconfig==2.0.0
```

### No binary error when no source distribution

<!-- from sync.rs::no_binary_error -->

Using `--no-binary-package` fails when the package has no source distribution.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["odrive"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --no-binary-package odrive
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 31 packages in [TIME]
error: Distribution `odrive==0.6.8 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-binary` but has no source distribution
```

## No build

### Sync with no-build-package

<!-- from sync.rs::no_build -->

The `--no-build-package` flag prevents building a specific package from source.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --no-build-package iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### No build error when no binary distribution

<!-- from sync.rs::no_build_error -->

Using `--no-build-package` fails when the package has no binary distribution.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["django_allauth==0.51.0"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --no-build-package django-allauth
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 19 packages in [TIME]
error: Distribution `django-allauth==0.51.0 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-build` but has no binary distribution
```

### No build (all) error

<!-- from sync.rs::no_build_error -->

Using `--no-build` fails when any package has no binary distribution.

```console
$ uv sync --no-build
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 19 packages in [TIME]
error: Distribution `django-allauth==0.51.0 @ registry+https://pypi.org/simple` can't be installed because it is marked as `--no-build` but has no binary distribution
```
