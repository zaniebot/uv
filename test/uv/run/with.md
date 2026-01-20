# Run with Dependencies

Tests for `uv run --with` to add ephemeral dependencies.

## Basic --with usage

### Add an unsatisfied requirement

<!-- from run.rs::run_with -->

The `--with` flag installs additional dependencies in an ephemeral environment.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["sniffio==1.3.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python main.py
import sniffio

print(sniffio.__version__)
```

```console
$ uv run --with iniconfig main.py
success: true
exit_code: 0
----- stdout -----
1.3.0

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/)
 + sniffio==1.3.0
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Add a satisfied requirement

<!-- from run.rs::run_with -->

Adding a requirement that's already satisfied uses the base environment.

```console
$ uv run --with sniffio main.py
success: true
exit_code: 0
----- stdout -----
1.3.0

----- stderr -----
Resolved 2 packages in [TIME]
Audited 2 packages in [TIME]
```

### Add a different version

<!-- from run.rs::run_with -->

Specifying a different version installs that version.

```console
$ uv run --with "sniffio<1.3.0" main.py
success: true
exit_code: 0
----- stdout -----
1.2.0

----- stderr -----
Resolved 2 packages in [TIME]
Audited 2 packages in [TIME]
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + sniffio==1.2.0
```

### Add a dependency that has dependencies

<!-- from run.rs::run_with -->

Adding a package respects existing dependencies from the base environment.

```console
$ uv run --with anyio main.py
success: true
exit_code: 0
----- stdout -----
1.3.0

----- stderr -----
Resolved 2 packages in [TIME]
Audited 2 packages in [TIME]
Resolved 3 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.0
```

### Add with --no-sync

<!-- from run.rs::run_with -->

The `--with` flag works with `--no-sync` to skip syncing the base environment.

```console
$ uv run --with "anyio==4.2.0" --no-sync main.py
success: true
exit_code: 0
----- stdout -----
1.3.0

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.2.0
 + idna==3.6
 + sniffio==1.3.0
```

## Error handling

### Unresolvable dependency

<!-- from run.rs::run_with -->

Unresolvable dependencies show an appropriate error.

```console
$ uv run --with add main.py
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Audited 2 packages in [TIME]
  × No solution found when resolving `--with` dependencies:
  ╰─▶ Because there are no versions of add and you require add, we can conclude that your requirements are unsatisfiable.
```
