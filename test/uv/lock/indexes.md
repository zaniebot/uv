# Lock Indexes

Tests for index configuration and behavior.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Explicit indexes

### Explicit index with source reference

<!-- from lock.rs::lock_explicit_index -->

An explicit index can be referenced via `tool.uv.sources` while other packages use PyPI.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0", "iniconfig==2.0.0"]

[tool.uv.sources]
iniconfig = { index = "test" }

[[tool.uv.index]]
name = "test"
url = "https://test.pypi.org/simple"
explicit = true
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

### Explicit default index

<!-- from lock.rs::lock_explicit_default_index -->

An index can be both explicit (only used when referenced) and default.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[tool.uv.sources]
iniconfig = { index = "test" }

[[tool.uv.index]]
name = "test"
url = "https://test.pypi.org/simple"
explicit = true
default = true
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Named indexes

### Named index configuration

<!-- from lock.rs::lock_named_index -->

A named index can be configured and used for dependency resolution.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[[tool.uv.index]]
name = "test"
url = "https://test.pypi.org/simple"

[tool.uv.sources]
iniconfig = { index = "test" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

### Duplicate named index

<!-- from lock.rs::lock_repeat_named_index -->

Defining the same index name twice produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[[tool.uv.index]]
name = "pytorch"
url = "https://astral-sh.github.io/pytorch-mirror/whl/cu121"

[[tool.uv.index]]
name = "pytorch"
url = "https://example.com"
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 7, column 1
  |
7 | [[tool.uv.index]]
  | ^^^^^^^^^^^^^^^^^
duplicate index name `pytorch`
```
