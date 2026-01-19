# Adding Git Dependencies

Tests for adding Git dependencies with `uv add`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = ["git", "pypi"]
```

## Basic

<!-- Derived from [`edit::add_git`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git dependency with a tag reference.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

Adding with an ambiguous Git reference treats it as a revision:

```console
$ uv add uv-public-pypackage@git+https://github.com/astral-test/uv-public-pypackage@0.0.1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

Adding with an explicit `--tag` flag:

```console
$ uv add uv-public-pypackage@git+https://github.com/astral-test/uv-public-pypackage --tag=0.0.1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Audited 4 packages in [TIME]
```

The `pyproject.toml` should have the source configured:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio==3.7.0",
    "uv-public-pypackage",
]

[tool.uv.sources]
uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1" }
```

Install from the lockfile:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 4 packages in [TIME]
```

## Branch

<!-- Derived from [`edit::add_git_branch`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git dependency with a branch reference.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add uv-public-pypackage@git+https://github.com/astral-test/uv-public-pypackage --branch test-branch
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

## Raw

<!-- Derived from [`edit::add_git_raw`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git dependency with `--raw-sources` keeps the URL in the dependency list.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

Use an ambiguous tag reference with `--raw-sources`:

```console
$ uv add uv-public-pypackage@git+https://github.com/astral-test/uv-public-pypackage@0.0.1 --raw-sources
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

The `pyproject.toml` should have the raw URL in dependencies:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio==3.7.0",
    "uv-public-pypackage @ git+https://github.com/astral-test/uv-public-pypackage@0.0.1",
]
```

Install from the lockfile:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 4 packages in [TIME]
```

## Implicit

<!-- Derived from [`edit::add_git_implicit`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git dependency without the `git+` prefix.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

Omit the `git+` prefix:

```console
$ uv add uv-public-pypackage@https://github.com/astral-test/uv-public-pypackage.git
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@b270df1a2fb5d012294e9aaf05e7e0bab1e6a389)
```

## Update reference project

<!-- Derived from [`edit::add_update_git_reference_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Updating an existing Git reference with branch/tag/rev options without re-specifying the URL.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

First, add the package:

```console
$ uv add https://github.com/astral-test/uv-public-pypackage.git
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@b270df1a2fb5d012294e9aaf05e7e0bab1e6a389)
```

Update to a specific tag:

```console
$ uv add uv-public-pypackage --tag=0.0.1
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 - uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@b270df1a2fb5d012294e9aaf05e7e0bab1e6a389)
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

Update to a branch:

```console
$ uv add uv-public-pypackage --branch=main
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 - uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@b270df1a2fb5d012294e9aaf05e7e0bab1e6a389)
```

Update to a specific revision:

```console
$ uv add uv-public-pypackage --rev=2005223fcad0e2c06daf2e14b93b790604868e1e
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 - uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@b270df1a2fb5d012294e9aaf05e7e0bab1e6a389)
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage.git@2005223fcad0e2c06daf2e14b93b790604868e1e)
```

## Update reference script

<!-- Derived from [`edit::add_update_git_reference_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Updating an existing Git reference in a script.

```python
# file: script.py

# /// script
# requires-python = ">=3.11"
# dependencies = [ ]
# ///

import time
time.sleep(5)
```

Add a Git dependency:

```console
$ uv add https://github.com/astral-test/uv-public-pypackage.git --script=script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: `VIRTUAL_ENV=.venv` does not match the script environment path `[CACHE_DIR]/environments-v2/script-[HASH]` and will be ignored; use `--active` to target the active environment instead
Resolved 1 package in [TIME]
```

The script should have the source configured:

```python title="script.py" snapshot=true
# /// script
# requires-python = ">=3.11"
# dependencies = [
#  "uv-public-pypackage",
# ]
#
# [tool.uv.sources]
# uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage.git" }
# ///

import time
time.sleep(5)
```

Update to a specific branch:

```console
$ uv add uv-public-pypackage --branch=test-branch --script=script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: `VIRTUAL_ENV=.venv` does not match the script environment path `[CACHE_DIR]/environments-v2/script-[HASH]` and will be ignored; use `--active` to target the active environment instead
Resolved 1 package in [TIME]
```

The script should have the branch reference:

```python title="script.py" snapshot=true
# /// script
# requires-python = ">=3.11"
# dependencies = [
#  "uv-public-pypackage",
# ]
#
# [tool.uv.sources]
# uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage.git", branch = "test-branch" }
# ///

import time
time.sleep(5)
```

## To script

<!-- Derived from [`edit::add_git_to_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git dependency to a script.

```python
# file: script.py

# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "anyio",
# ]
# ///

import anyio
import uv_public_pypackage
```

```console
$ uv add uv-public-pypackage@git+https://github.com/astral-test/uv-public-pypackage --tag=0.0.1 --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: `VIRTUAL_ENV=.venv` does not match the script environment path `[CACHE_DIR]/environments-v2/script-[HASH]` and will be ignored; use `--active` to target the active environment instead
Resolved 4 packages in [TIME]
```

The script should have the source configured:

```python title="script.py" snapshot=true
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "anyio",
#   "uv-public-pypackage",
# ]
#
# [tool.uv.sources]
# uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1" }
# ///

import anyio
import uv_public_pypackage
```

## Reject multiple ref flags

<!-- Derived from [`edit::add_reject_multiple_git_ref_flags`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Using multiple Git reference flags is an error.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

Using `--tag` and `--branch` together:

```console
$ uv add foo --tag 0.0.1 --branch test
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--tag <TAG>' cannot be used with '--branch <BRANCH>'

Usage: uv add --tag <TAG> --exclude-newer <EXCLUDE_NEWER> <PACKAGES|--requirements <REQUIREMENTS>>

For more information, try '--help'.
```

Using `--tag` and `--rev` together:

```console
$ uv add foo --tag 0.0.1 --rev 326b943
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--tag <TAG>' cannot be used with '--rev <REV>'

Usage: uv add --tag <TAG> --exclude-newer <EXCLUDE_NEWER> <PACKAGES|--requirements <REQUIREMENTS>>

For more information, try '--help'.
```
