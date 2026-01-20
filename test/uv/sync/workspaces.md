# Sync Workspaces

Tests for `uv sync` with workspace packages.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Package selection

### Sync single workspace package

<!-- from sync.rs::package -->

The `--package` flag syncs only the specified package.

```toml
# file: pyproject.toml
[project]
name = "root"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child", "anyio>3"]

[tool.uv.sources]
child = { workspace = true }

[tool.uv.workspace]
members = ["child"]
```

```python src/albatross/__init__.py

```

```toml child/pyproject.toml
[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=1"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python child/src/albatross/__init__.py

```

```console
$ uv sync --package child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + child==0.1.0 (from file://[TEMP_DIR]/child)
 + iniconfig==2.0.0
```

### Sync multiple workspace packages

<!-- from sync.rs::multiple_packages -->

Multiple `--package` flags sync multiple packages.

```toml
# file: pyproject.toml
[project]
name = "root"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["foo", "bar", "baz"]

[tool.uv.sources]
foo = { workspace = true }
bar = { workspace = true }
baz = { workspace = true }

[tool.uv.workspace]
members = ["packages/*"]
```

```toml packages/foo/pyproject.toml
[project]
name = "foo"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio"]
```

```toml packages/bar/pyproject.toml
[project]
name = "bar"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]
```

```toml packages/baz/pyproject.toml
[project]
name = "baz"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

Sync only foo and bar.

```console
$ uv sync --package foo --package bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 9 packages in [TIME]
Prepared 6 packages in [TIME]
Installed 6 packages in [TIME]
 + anyio==4.3.0
 + bar==0.1.0 (from file://[TEMP_DIR]/packages/bar)
 + foo==0.1.0 (from file://[TEMP_DIR]/packages/foo)
 + idna==3.6
 + sniffio==1.3.1
 + typing-extensions==4.10.0
```

Add baz to the sync.

```console
$ uv sync --package foo --package bar --package baz
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 9 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + baz==0.1.0 (from file://[TEMP_DIR]/packages/baz)
 + iniconfig==2.0.0
```

## Explicit index

### Sync with explicit index

<!-- from sync.rs::sync_explicit -->

Using an explicit index with sources configuration.

```toml
# file: pyproject.toml
[project]
name = "root"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["idna>2"]

[[tool.uv.index]]
name = "test"
url = "https://test.pypi.org/simple"
explicit = true

[tool.uv.sources]
idna = { index = "test" }
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
 + idna==2.7
```
