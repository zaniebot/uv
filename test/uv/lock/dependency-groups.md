# Lock Dependency Groups

Tests for dependency group validation and behavior.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Invalid entries

### Invalid package name in group

<!-- from lock.rs::lock_group_invalid_entry_package -->

An invalid package name in a dependency group produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["invalid!"]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Project `project` has malformed dependency groups
  Caused by: Failed to parse entry in group `foo`: `invalid!`
  Caused by: no such comparison operator "!", must be one of ~= == != <= >= < > ===
invalid!
       ^
```

### Invalid include-group name

<!-- from lock.rs::lock_group_invalid_entry_group_name -->

An invalid group name in an include-group produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = [{include-group = "invalid!"}]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 8, column 8
  |
8 | foo = [{include-group = "invalid!"}]
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Not a valid package or extra name: "invalid!". Names must start and end with a letter or digit and may only contain -, _, ., and alphanumeric characters.
```

### Duplicate normalized group name

<!-- from lock.rs::lock_group_invalid_duplicate_group_name -->

Groups with names that normalize to the same value are detected as duplicates.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo-bar = []
foo_bar = []
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
7 | [dependency-groups]
  | ^^^^^^^^^^^^^^^^^^^
duplicate dependency group: `foo-bar`
```

### Unknown dependency object specifier

<!-- from lock.rs::lock_group_invalid_entry_table -->

An unknown key in a dependency object produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = [{bar = "unknown"}]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Project `project` has malformed dependency groups
  Caused by: Group `foo` contains an unknown dependency object specifier: {"bar": "unknown"}
```

### Invalid include-group type

<!-- from lock.rs::lock_group_invalid_entry_type -->

An include-group value must be a string.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = [{include-group = true}]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 8, column 25
  |
8 | foo = [{include-group = true}]
  |                         ^^^^
invalid type: boolean `true`, expected a string
```

### Empty entry table

<!-- from lock.rs::lock_group_empty_entry_table -->

An empty table in a dependency group produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = [{}]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 8, column 8
  |
8 | foo = [{}]
  |        ^^
missing field `include-group`
```

## Group include cycles

### Include cycle

<!-- from lock.rs::lock_group_include_cycle -->

A dependency group cannot include itself via a cycle.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = [{include-group = "bar"}]
bar = [{include-group = "foo"}]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Project `project` has malformed dependency groups
  Caused by: Detected a cycle in `dependency-groups`: `bar` -> `foo` -> `bar`
```

### Include missing group

<!-- from lock.rs::lock_group_include_missing -->

Including a non-existent group produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = [{include-group = "bar"}]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Project `project` has malformed dependency groups
  Caused by: Failed to find group `bar` included by `foo`
```

## Group includes

### Include another group

<!-- from lock.rs::lock_group_include -->

A dependency group can include another group using `include-group`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio", {include-group = "bar"}]
bar = ["trio"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 11 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 11 packages in [TIME]
```

### Adding empty group updates lockfile

<!-- from lock.rs::lock_add_empty_dependency_group -->

Adding an empty dependency group triggers a lockfile update.

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

Add an empty group.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[dependency-groups]
empty = []
```

`--locked` fails because the lockfile needs updating.

```console
$ uv lock --locked
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
```

Re-lock to update the lockfile.

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Now `--locked` succeeds.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Group requires-python

### Group with requires-python constraint

<!-- from lock.rs::lock_group_requires_python -->

Dependency groups can have Python version constraints via `[tool.uv.dependency-groups]`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["idna"]
bar = ["sortedcontainers", "sniffio"]

[tool.uv.dependency-groups]
bar = { requires-python = ">=3.13" }
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

## Empty groups

### Lock with empty dependency group

<!-- from lock.rs::lock_empty_dependency_group -->

A project with an empty dependency group locks correctly.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[dependency-groups]
empty = []
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Validate with `--locked --offline` (no network needed for immutable metadata).

```console
$ uv lock --locked --offline --no-cache
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```
