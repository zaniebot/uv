# Lock Extras

Tests for locking projects with extras and optional dependencies.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Project extras

### Lock project with extras

<!-- from lock.rs::lock_project_extra -->

When resolving, all extras should be included in the lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[project.optional-dependencies]
test = ["iniconfig"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Install base dependencies.

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

Install the test extra.

```console
$ uv sync --frozen --extra test
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

## Dependency extras

### Lock dependency with extra

<!-- from lock.rs::lock_dependency_extra -->

Lock a dependency that requests an extra from another package.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["flask[dotenv]"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
```

Install from the lockfile. The dotenv extra brings in python-dotenv.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 8 packages in [TIME]
Installed 8 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + python-dotenv==1.0.1
 + werkzeug==3.0.1
```

### Lock dependency with non-existent extra

<!-- from lock.rs::lock_dependency_non_existent_extra -->

Requesting a non-existent extra from a dependency should warn.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["flask[nonexistent]"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
warning: The package `flask==3.0.2` does not have an extra named `nonexistent`
```

## Empty extras

### Lock with empty extra

<!-- from lock.rs::lock_empty_extra -->

Adding an empty extra triggers lockfile update.

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
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Add a non-empty extra.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[project.optional-dependencies]
foo = ["typing-extensions"]
```

Verify `--locked` fails.

```console
$ uv lock --locked
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
```

Regenerate the lockfile.

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Added typing-extensions v4.10.0
```

Add an empty extra.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[project.optional-dependencies]
foo = ["typing-extensions"]
bar = []
```

Empty extras also trigger lockfile update.

```console
$ uv lock --locked
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
```
