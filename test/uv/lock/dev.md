# Lock Dev Dependencies

Tests for locking projects with dev dependencies.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Legacy dev dependencies

### Lock with tool.uv.dev-dependencies

<!-- from lock.rs::lock_dev -->

Lock a project with legacy `tool.uv.dev-dependencies`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[tool.uv]
dev-dependencies = ["typing-extensions @ https://files.pythonhosted.org/packages/26/9f/ad63fc0248c5379346306f8668cda6e2e2e9c95e01216d2b8ffd9ff037d0/typing_extensions-4.12.2-py3-none-any.whl"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 3 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 3 packages in [TIME]
```

### Sync without dev dependencies

<!-- from lock.rs::lock_dev -->

Install from the lockfile excluding dev dependencies.

```console
$ uv sync --frozen --no-dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Sync with dev dependencies

<!-- from lock.rs::lock_dev -->

Install from the lockfile including dev dependencies (the default).

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Installed 1 package in [TIME]
 + typing-extensions==4.12.2 (from https://files.pythonhosted.org/packages/26/9f/ad63fc0248c5379346306f8668cda6e2e2e9c95e01216d2b8ffd9ff037d0/typing_extensions-4.12.2-py3-none-any.whl)
```
