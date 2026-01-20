# Lock Overrides

Tests for locking with override-dependencies, constraints, and excludes.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Override dependencies

### Lock with override-dependencies

<!-- from lock.rs::lock_project_with_overrides -->

The `override-dependencies` setting forces a specific version of a transitive dependency.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["flask==3.0.0"]

[tool.uv]
override-dependencies = ["werkzeug==2.3.8"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 9 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 9 packages in [TIME]
```

Install and verify the override is applied.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.0
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==2.3.8
```

## Constraints

### Lock with constraint-dependencies

<!-- from lock.rs::lock_project_with_constraints -->

The `constraint-dependencies` setting limits versions but doesn't add packages.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["flask==3.0.0"]

[tool.uv]
constraint-dependencies = ["werkzeug<3"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
```

Install and verify the constraint is applied.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.0
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==2.3.8
```

### Lock with constraint from idna<3.4

<!-- from lock.rs::lock_project_with_constraints -->

The constraint limits the version of idna.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv]
constraint-dependencies = ["idna<3.4"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Install and verify the constraint is applied (idna==3.3 instead of 3.6).

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.3
 + sniffio==1.3.1
```

### Override with URL source

<!-- from lock.rs::lock_project_with_override_sources -->

Override dependencies can use `tool.uv.sources` for custom locations.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv]
override-dependencies = ["idna==3.2"]

[tool.uv.sources]
idna = { url = "https://files.pythonhosted.org/packages/d7/77/ff688d1504cdc4db2a938e2b7b9adee5dd52e34efbd2431051efc9984de9/idna-3.2-py3-none-any.whl" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 2 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.2 (from https://files.pythonhosted.org/packages/d7/77/ff688d1504cdc4db2a938e2b7b9adee5dd52e34efbd2431051efc9984de9/idna-3.2-py3-none-any.whl)
 + sniffio==1.3.1
```

## Exclude dependencies

### Lock with exclude-dependencies

<!-- from lock.rs::lock_project_with_excludes -->

The `exclude-dependencies` setting removes a dependency from the lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["flask==3.0.0"]

[tool.uv]
exclude-dependencies = ["werkzeug"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
```

Install without werkzeug.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 6 packages in [TIME]
Installed 6 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.0
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
```
