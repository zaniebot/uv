# Lock Sources

Tests for `tool.uv.sources` validation and behavior.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Multiple sources with markers

### Multiple sources with disjoint markers

<!-- from lock.rs::lock_multiple_sources -->

Using multiple sources with disjoint markers resolves correctly.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[tool.uv.sources]
iniconfig = [
    { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl", marker = "sys_platform != 'win32'" },
    { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", marker = "sys_platform == 'win32'" },
]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Multiple sources validation

### Overlapping source markers

<!-- from lock.rs::lock_multiple_sources_conflict -->

When multiple sources are provided, their markers must be disjoint.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[tool.uv.sources]
iniconfig = [
    { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl", marker = "sys_platform == 'win32' and python_version == '3.12'" },
    { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", marker = "sys_platform == 'win32'" },
]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 8, column 13
  |
8 | iniconfig = [
  |             ^
Source markers must be disjoint, but the following markers overlap: `python_full_version == '3.12.*' and sys_platform == 'win32'` and `sys_platform == 'win32'`.

hint: replace `sys_platform == 'win32'` with `python_full_version != '3.12.*' and sys_platform == 'win32'`.
```

### Missing markers for multiple sources

<!-- from lock.rs::lock_multiple_sources_no_marker -->

When multiple sources are provided, each must include a platform marker.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[tool.uv.sources]
iniconfig = [
    { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl" },
    { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz" },
]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 8, column 13
  |
8 | iniconfig = [
  |             ^
When multiple sources are provided, each source must include a platform marker (e.g., `marker = "sys_platform == 'linux'"`)
```

## Source extras and groups

### Source with missing extra

<!-- from lock.rs::lock_multiple_index_with_missing_extra -->

A source with an extra that doesn't exist causes an error.

```toml
# mdtest

[environment]
exclude-newer = "2025-01-30T00:00:00Z"
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["jinja2"]

[tool.uv.sources]
jinja2 = [
    { index = "torch-cu118", extra = "cu118" },
]

[[tool.uv.index]]
name = "torch-cu118"
url = "https://astral-sh.github.io/pytorch-mirror/whl/cu118"
explicit = true
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `project @ file://[TEMP_DIR]/`
  ╰─▶ Source entry for `jinja2` only applies to extra `cu118`, but the `cu118` extra does not exist. When an extra is present on a source (e.g., `extra = "cu118"`), the relevant package must be included in the `project.optional-dependencies` section for that extra (e.g., `project.optional-dependencies = { "cu118" = ["jinja2"] }`).
```

### Source extra without package in optional-dependencies

<!-- from lock.rs::lock_multiple_index_with_absent_extra -->

A source with an extra requires the package in that extra's optional-dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["jinja2>=3"]

[project.optional-dependencies]
cu118 = []

[tool.uv.sources]
jinja2 = [
    { index = "torch-cu118", extra = "cu118" },
]

[[tool.uv.index]]
name = "torch-cu118"
url = "https://astral-sh.github.io/pytorch-mirror/whl/cu118"
explicit = true
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `project @ file://[TEMP_DIR]/`
  ╰─▶ Source entry for `jinja2` only applies to extra `cu118`, but `jinja2` was not found under the `project.optional-dependencies` section for that extra. When an extra is present on a source (e.g., `extra = "cu118"`), the relevant package must be included in the `project.optional-dependencies` section for that extra (e.g., `project.optional-dependencies = { "cu118" = ["jinja2"] }`).
```

### Source with missing group

<!-- from lock.rs::lock_multiple_index_with_missing_group -->

A source with a group that doesn't exist causes an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["jinja2"]

[tool.uv.sources]
jinja2 = [
    { index = "torch-cu118", group = "cu118" },
]

[[tool.uv.index]]
name = "torch-cu118"
url = "https://astral-sh.github.io/pytorch-mirror/whl/cu118"
explicit = true
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `project @ file://[TEMP_DIR]/`
  ╰─▶ Source entry for `jinja2` only applies to dependency group `cu118`, but the `cu118` group does not exist. When a group is present on a source (e.g., `group = "cu118"`), the relevant package must be included in the `dependency-groups` section for that extra (e.g., `dependency-groups = { "cu118" = ["jinja2"] }`).
```

### Source group without package in dependency-groups

<!-- from lock.rs::lock_multiple_index_with_absent_group -->

A source with a group requires the package in that group's dependency-groups.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["jinja2>=3"]

[dependency-groups]
cu118 = []

[tool.uv.sources]
jinja2 = [
    { index = "torch-cu118", group = "cu118" },
]

[[tool.uv.index]]
name = "torch-cu118"
url = "https://astral-sh.github.io/pytorch-mirror/whl/cu118"
explicit = true
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `project @ file://[TEMP_DIR]/`
  ╰─▶ Source entry for `jinja2` only applies to dependency group `cu118`, but `jinja2` was not found under the `dependency-groups` section for that group. When a group is present on a source (e.g., `group = "cu118"`), the relevant package must be included in the `dependency-groups` section for that extra (e.g., `dependency-groups = { "cu118" = ["jinja2"] }`).
```
