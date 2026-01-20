# Format Basics

Tests for basic `uv format` functionality.

Note: `uv format` is experimental and shows a warning by default.

## Basic formatting

### Format a project

<!-- from format.rs::format_project -->

Format Python files in a project.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```python main.py
x    = 1
```

```console
$ uv format
success: true
exit_code: 0
----- stdout -----
1 file reformatted

----- stderr -----
warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
```

### Format without pyproject.toml

<!-- from format.rs::format_missing_pyproject_toml -->

Format works without a pyproject.toml.

```python main.py
x    = 1
```

```console
$ uv format
success: true
exit_code: 0
----- stdout -----
1 file reformatted

----- stderr -----
warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
```

### Format unmanaged project

<!-- from format.rs::format_unmanaged_project -->

Format works with unmanaged projects (managed = false).

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.uv]
managed = false
```

```python main.py
x    = 1
```

```console
$ uv format
success: true
exit_code: 0
----- stdout -----
1 file reformatted

----- stderr -----
warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
```

### Format with --no-project

<!-- from format.rs::format_no_project -->

The `--no-project` flag formats without project context.

```python main.py
x    = 1
```

```console
$ uv format --no-project
success: true
exit_code: 0
----- stdout -----
1 file reformatted

----- stderr -----
warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
```

## Check mode

### Format check

<!-- from format.rs::format_check -->

The `--check` flag checks formatting without modifying files.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```python main.py
x    = 1
```

```console
$ uv format --check
success: false
exit_code: 1
----- stdout -----
Would reformat: main.py
1 file would be reformatted

----- stderr -----
warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
```

## Diff mode

### Format diff

<!-- from format.rs::format_diff -->

The `--diff` flag shows what would change without modifying files.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```python main.py
x    = 1
```

```console
$ uv format --diff
success: false
exit_code: 1
----- stdout -----
--- main.py
+++ main.py
@@ -1 +1 @@
-x    = 1
+x = 1

1 file would be reformatted

----- stderr -----
warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
```

## Ruff version

### Format with specific Ruff version

<!-- from format.rs::format_version_option -->

The `--version` flag specifies a specific Ruff version to use.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = []
```

```python main.py
x    = 1
```

```console
$ uv format --version 0.8.2
success: true
exit_code: 0
----- stdout -----
1 file reformatted

----- stderr -----
warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
```

## Error handling

### Format with malformed pyproject.toml

<!-- from format.rs::format_fails_malformed_pyproject -->

Running `uv format` with a malformed pyproject.toml fails.

```toml
# file: pyproject.toml
malformed pyproject.toml
```

```python main.py
x    = 1
```

```console
$ uv format
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 1, column 11
    |
  1 | malformed pyproject.toml
    |           ^
  key with no value, expected `=`

warning: `uv format` is experimental and may change without warning. Pass `--preview-features format` to disable this warning.
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 11
  |
1 | malformed pyproject.toml
  |           ^
key with no value, expected `=`
```
