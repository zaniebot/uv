# Sync Basics

Tests for basic `uv sync` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Basic sync

### Sync generates lockfile

<!-- from sync.rs::sync -->

Running `uv sync` should generate a lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

## Locked mode

### Locked without lockfile

<!-- from sync.rs::locked -->

Running with `--locked` should error if no lockfile is present.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv sync --locked
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unable to find lockfile at `uv.lock`, but `--locked` was provided. To create a lockfile, run `uv lock` or `uv sync` without the flag.
```

### Locked with outdated lockfile

<!-- from sync.rs::locked -->

Running with `--locked` should error if the lockfile is outdated.

```console
$ uv lock
success: true
exit_code: 0
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv sync --locked
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
```

## Frozen mode

### Frozen without lockfile

<!-- from sync.rs::frozen -->

Running with `--frozen` should error if no lockfile is present.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv sync --frozen
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unable to find lockfile at `uv.lock`, but `--frozen` was provided. To create a lockfile, run `uv lock` or `uv sync` without the flag.
```

### Frozen installs stale lockfile

<!-- from sync.rs::frozen -->

Running with `--frozen` should install from the existing lockfile.

```console
$ uv lock
success: true
exit_code: 0
```

Update the requirements and verify `--frozen` uses the old lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
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

## Empty workspace

### Sync empty workspace

<!-- from sync.rs::empty -->

Syncing an empty workspace generates an empty lockfile.

```toml
# file: pyproject.toml
[tool.uv.workspace]
members = []
```

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved in [TIME]
Audited in [TIME]
```

## No-install flags

### No-install-project

<!-- from sync.rs::no_install_project -->

The `--no-install-project` flag installs dependencies but not the project.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --no-install-project
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

## Check mode

### Sync check

<!-- from sync.rs::check -->

The `--check` flag exits with an error if the environment is not up-to-date.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv sync --check
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Environment at `.venv` does not match lockfile. Run `uv sync` to sync the environment.
```

After running `uv sync`, the check passes.

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

```console
$ uv sync --check
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 1 package in [TIME]
```

## JSON output

### Sync with JSON output

<!-- from sync.rs::sync_json -->

The `--output-format json` flag outputs sync information in JSON format.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv sync --output-format json
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "target": "project",
  "project": {
    "path": "[TEMP_DIR]/",
    "workspace": {
      "path": "[TEMP_DIR]/"
    }
  },
  "sync": {
    "environment": {
      "path": "[VENV]/",
      "python": {
        "path": "[VENV]/[BIN]/[PYTHON]",
        "version": "3.12.[X]",
        "implementation": "cpython"
      }
    },
    "action": "check",
    "changes": [
      {
        "name": "iniconfig",
        "version": "2.0.0",
        "action": "installed"
      }
    ]
  },
  "lock": {
    "path": "[TEMP_DIR]/uv.lock",
    "action": "create"
  },
  "dry_run": false
}

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Sync frozen with JSON output

<!-- from sync.rs::sync_json -->

With `--frozen`, the lock action is "use".

```console
$ uv sync --frozen --output-format json
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "target": "project",
  "project": {
    "path": "[TEMP_DIR]/",
    "workspace": {
      "path": "[TEMP_DIR]/"
    }
  },
  "sync": {
    "environment": {
      "path": "[VENV]/",
      "python": {
        "path": "[VENV]/[BIN]/[PYTHON]",
        "version": "3.12.[X]",
        "implementation": "cpython"
      }
    },
    "action": "check",
    "changes": []
  },
  "lock": {
    "path": "[TEMP_DIR]/uv.lock",
    "action": "use"
  },
  "dry_run": false
}

----- stderr -----
Audited 1 package in [TIME]
```

### Sync locked with JSON output

<!-- from sync.rs::sync_json -->

With `--locked`, the lock action is "check".

```console
$ uv sync --locked --output-format json
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "target": "project",
  "project": {
    "path": "[TEMP_DIR]/",
    "workspace": {
      "path": "[TEMP_DIR]/"
    }
  },
  "sync": {
    "environment": {
      "path": "[VENV]/",
      "python": {
        "path": "[VENV]/[BIN]/[PYTHON]",
        "version": "3.12.[X]",
        "implementation": "cpython"
      }
    },
    "action": "check",
    "changes": []
  },
  "lock": {
    "path": "[TEMP_DIR]/uv.lock",
    "action": "check"
  },
  "dry_run": false
}

----- stderr -----
Resolved 2 packages in [TIME]
Audited 1 package in [TIME]
```

## Dry run

### Sync dry run with JSON output

<!-- from sync.rs::sync_dry_json -->

Running `uv sync --dry-run` with JSON output reports intent without making changes.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv sync --output-format json --dry-run
success: true
exit_code: 0
----- stdout -----
{
  "schema": {
    "version": "preview"
  },
  "target": "project",
  "project": {
    "path": "[TEMP_DIR]/",
    "workspace": {
      "path": "[TEMP_DIR]/"
    }
  },
  "sync": {
    "environment": {
      "path": "[VENV]/",
      "python": {
        "path": "[VENV]/[BIN]/[PYTHON]",
        "version": "3.12.[X]",
        "implementation": "cpython"
      }
    },
    "action": "create",
    "changes": [
      {
        "name": "iniconfig",
        "version": "2.0.0",
        "action": "installed"
      }
    ]
  },
  "lock": {
    "path": "[TEMP_DIR]/uv.lock",
    "action": "create"
  },
  "dry_run": true
}

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 2 packages in [TIME]
Would download 1 package
Would install 1 package
 + iniconfig==2.0.0
```

## Environment restrictions

### Sync with incompatible environment

<!-- from sync.rs::sync_environment -->

Syncing fails when the current platform is incompatible with the configured environments.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = ["iniconfig"]

[tool.uv]
environments = ["python_version < '3.11'"]
```

```console
$ uv sync
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
error: The current Python platform is not compatible with the lockfile's supported environments: `python_full_version < '3.11'`
```

## Default groups errors

### Invalid default-groups format

<!-- from sync.rs::sync_default_groups_gibberish -->

Using an invalid `default-groups` value produces a parse error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
dev = ["iniconfig"]
foo = ["anyio"]
bar = ["requests"]

[tool.uv]
default-groups = "gibberish"
```

```console
$ uv sync
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 14, column 18
   |
14 | default-groups = "gibberish"
   |                  ^^^^^^^^^^^
default-groups must be "all" or a ["list", "of", "groups"]
```

### Non-existent default group

<!-- from sync.rs::sync_non_existent_default_group -->

Using a non-existent group in `default-groups` produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = []

[tool.uv]
default-groups = ["bar"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

```console
$ uv sync
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Default group `bar` (from `tool.uv.default-groups`) is not defined in the project's `dependency-groups` table
```
