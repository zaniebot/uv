# Run with Dependency Groups

Tests for `uv run` with dependency groups.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Basic group usage

### Run with default dev group

<!-- from run.rs::run_group -->

By default, the `dev` group is included.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

```python main.py
try:
    import anyio
    print("imported `anyio`")
except ImportError:
    print("failed to import `anyio`")

try:
    import iniconfig
    print("imported `iniconfig`")
except ImportError:
    print("failed to import `iniconfig`")

try:
    import typing_extensions
    print("imported `typing_extensions`")
except ImportError:
    print("failed to import `typing_extensions`")
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv run main.py
success: true
exit_code: 0
----- stdout -----
failed to import `anyio`
failed to import `iniconfig`
imported `typing_extensions`

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + sniffio==1.3.1
 + typing-extensions==4.10.0
```

### Run with --only-group

<!-- from run.rs::run_group -->

The `--only-group` flag includes only the specified group.

```console
$ uv run --only-group bar main.py
success: true
exit_code: 0
----- stdout -----
failed to import `anyio`
imported `iniconfig`
imported `typing_extensions`

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Run with --group

<!-- from run.rs::run_group -->

The `--group` flag adds a group to the default.

```console
$ uv run --group foo main.py
success: true
exit_code: 0
----- stdout -----
imported `anyio`
imported `iniconfig`
imported `typing_extensions`

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
```

### Run with multiple --group flags

<!-- from run.rs::run_group -->

Multiple `--group` flags can be used.

```console
$ uv run --group foo --group bar main.py
success: true
exit_code: 0
----- stdout -----
imported `anyio`
imported `iniconfig`
imported `typing_extensions`

----- stderr -----
Resolved 6 packages in [TIME]
Audited 5 packages in [TIME]
```

### Run with --all-groups

<!-- from run.rs::run_group -->

The `--all-groups` flag includes all dependency groups.

```console
$ uv run --all-groups main.py
success: true
exit_code: 0
----- stdout -----
imported `anyio`
imported `iniconfig`
imported `typing_extensions`

----- stderr -----
Resolved 6 packages in [TIME]
Audited 5 packages in [TIME]
```

## Default groups

### Run with default dev group

<!-- from run.rs::run_default_groups -->

By default, only main dependencies and the `dev` group are installed.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv run python -c "import typing_extensions"
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + sniffio==1.3.1
 + typing-extensions==4.10.0
```

### Run with custom default groups

<!-- from run.rs::run_default_groups -->

Setting `tool.uv.default-groups` changes which groups are synced by default.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio"]
bar = ["iniconfig"]
dev = ["sniffio"]

[tool.uv]
default-groups = ["foo"]
```

```console
$ uv run --exact python -c "import typing_extensions"
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
```
