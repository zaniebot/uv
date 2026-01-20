# Lock Basics

Tests for basic `uv lock` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Basic lock

### Lock a package from registry

<!-- from lock.rs::lock_wheel_registry -->

Lock a package and its dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

### Re-lock with --locked

<!-- from lock.rs::lock_wheel_registry -->

Re-running with `--locked` validates the lockfile.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

### Lock with --offline

<!-- from lock.rs::lock_wheel_registry -->

Re-running with `--locked --offline` validates without network.

```console
$ uv lock --locked --offline --no-cache
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

## Upgrade

### Upgrade a single package

<!-- from lock.rs::lock_upgrade_package -->

The `--upgrade-package` flag upgrades only the specified package.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio<=2", "idna<=3"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Remove the constraints and upgrade only `anyio`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio", "idna"]
```

```console
$ uv lock --upgrade-package anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Updated anyio v2.0.0 -> v4.3.0
```

Verify the lockfile is valid.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

## Dry run

### Dry run shows changes

<!-- from lock.rs::lock_dry_run -->

The `--dry-run` flag shows what would change without modifying the lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio<3"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv lock --dry-run
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Remove anyio v2.2.0
Add iniconfig v2.0.0
Remove idna v3.6
Remove sniffio v1.3.1
```

### Dry run on new project

<!-- from lock.rs::lock_dry_run_noop -->

The `--dry-run` flag shows what packages would be added.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv lock --dry-run
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Add iniconfig v2.0.0
Add project v0.1.0
```

## Source distributions

### Lock source distribution

<!-- from lock.rs::lock_sdist_registry -->

Lock a package that only provides source distributions.

```toml
# mdtest

[environment]
exclude-newer = "2025-01-29"
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["source-distribution==0.0.1"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Re-run with `--locked` to validate.

```console
$ uv lock --locked
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
 + source-distribution==0.0.1
```

## Git dependencies

```toml
# mdtest

[environment]
required-features = ["pypi", "git"]
```

### Lock Git source with tag

<!-- from lock.rs::lock_sdist_git -->

Lock a package from a Git repository using a tag.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["uv-public-pypackage"]

[tool.uv.sources]
uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Re-run with `--locked` to validate.

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
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

## Check mode

### Check without lockfile

<!-- from lock.rs::check_no_lock -->

Running `uv lock --check` without a lockfile fails.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = ["sortedcollections"]
```

```console
$ uv lock --check
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unable to find lockfile at `uv.lock`, but `--check` was provided. To create a lockfile, run `uv lock` or `uv sync` without the flag.
```

### Check with outdated lockfile

<!-- from lock.rs::check_outdated_lock -->

Running `uv lock --check` with an outdated lockfile fails with exit code 1.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = ["sortedcollections"]
```

First generate the lockfile:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

The check passes:

```console
$ uv lock --check
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Index validation

### Invalid index name

<!-- from lock.rs::lock_invalid_index -->

Index names with spaces or invalid characters cause an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0", "iniconfig==2.0.0"]

[tool.uv.sources]
iniconfig = { index = "internal proxy" }

[[tool.uv.index]]
name = "internal proxy"
url = "https://test.pypi.org/simple"
explicit = true
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 12, column 8
     |
  12 | name = "internal proxy"
     |        ^^^^^^^^^^^^^^^^
  Index names may only contain letters, digits, hyphens, underscores, and periods, but found unsupported character (` `) in: `internal proxy`

error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 9, column 23
  |
9 | iniconfig = { index = "internal proxy" }
  |                       ^^^^^^^^^^^^^^^^
Index names may only contain letters, digits, hyphens, underscores, and periods, but found unsupported character (` `) in: `internal proxy`
```

## Dependency group errors

### Missing include-group target

<!-- from lock.rs::lock_group_include_missing -->

Including a non-existent group causes an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio", {include-group = "bar"}]
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

### Invalid dependency group entry

<!-- from lock.rs::lock_group_invalid_entry_package -->

An invalid package specifier in a dependency group causes an error.

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

Using an invalid group name in include-group causes an error.

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
  Caused by: TOML parse error at line 9, column 8
  |
9 | foo = [{include-group = "invalid!"}]
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Not a valid package or extra name: "invalid!". Names must start and end with a letter or digit and may only contain -, _, ., and alphanumeric characters.
```

### Duplicate dependency group name

<!-- from lock.rs::lock_group_invalid_duplicate_group_name -->

Duplicate group names (after normalization) cause an error.

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
  Caused by: TOML parse error at line 8, column 1
  |
8 | [dependency-groups]
  | ^^^^^^^^^^^^^^^^^^^
duplicate dependency group: `foo-bar`
```

### Unknown dependency group object

<!-- from lock.rs::lock_group_invalid_entry_table -->

An unknown object key in a dependency group causes an error.

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

Using a non-string value for include-group causes an error.

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
  Caused by: TOML parse error at line 9, column 25
  |
9 | foo = [{include-group = true}]
  |                         ^^^^
invalid type: boolean `true`, expected a string
```

### Empty dependency group object

<!-- from lock.rs::lock_group_empty_entry_table -->

An empty object in a dependency group causes an error.

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
  Caused by: TOML parse error at line 9, column 8
  |
9 | foo = [{}]
  |        ^^
missing field `include-group`
```

## Missing required fields

### Missing project name

<!-- from lock.rs::lock_missing_name -->

A pyproject.toml with [project] but no name produces an error.

```toml
# file: pyproject.toml
[project]
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 1
  |
1 | [project]
  | ^^^^^^^^^
`pyproject.toml` is using the `[project]` table, but the required `project.name` field is not set
```

### Missing project version

<!-- from lock.rs::lock_missing_version -->

A pyproject.toml with [project] but no version produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 1
  |
1 | [project]
  | ^^^^^^^^^
`pyproject.toml` is using the `[project]` table, but the required `project.version` field is neither set nor present in the `project.dynamic` list
```

## Workspace validation

### Conflicting Python requirements in workspace

<!-- from lock.rs::lock_requires_python_disjoint -->

Workspace members with disjoint Python requirements cause an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.uv.workspace]
members = ["child"]
```

```toml child/pyproject.toml
[project]
name = "child"
version = "0.1.0"
requires-python = "==3.10"
dependencies = []
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Found conflicting Python requirements:
- child: ==3.10
- project: >=3.12
```

### Conflicting dependency group version

<!-- from lock.rs::lock_conflicting_project_basic1 -->

A dependency group that conflicts with project dependencies causes an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.3.0"]

[dependency-groups]
foo = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because your project depends on sortedcontainers==2.3.0 and project:foo depends on sortedcontainers==2.4.0, we can conclude that your project and project:foo are incompatible.
      And because your project requires your project and project:foo, we can conclude that your project's requirements are unsatisfiable.
```

### Conflicting extra and dependency group

<!-- from lock.rs::lock_conflicting_mixed -->

An extra that conflicts with a dependency group causes an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[dependency-groups]
project1 = ["sortedcontainers==2.3.0"]

[project.optional-dependencies]
project2 = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because project:project1 depends on sortedcontainers==2.3.0 and project[project2] depends on sortedcontainers==2.4.0, we can conclude that project:project1 and project[project2] are incompatible.
      And because your project requires project[project2] and project:project1, we can conclude that your project's requirements are unsatisfiable.
```

### Incompatible Python version

<!-- from lock.rs::lock_requires_python -->

A dependency that's incompatible with `requires-python` produces an error with hints.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.7"
dependencies = ["pygls>=1.1.0"]
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies for split (markers: python_full_version >= '3.7' and python_full_version < '3.7.9'):
  ╰─▶ Because the requested Python version (>=3.7) does not satisfy Python>=3.7.9 and pygls>=1.1.0,<=1.2.1 depends on Python>=3.7.9,<4, we can conclude that pygls>=1.1.0,<=1.2.1 cannot be used.
      And because only the following versions of pygls are available:
          pygls<=1.1.0
          pygls==1.1.1
          pygls==1.1.2
          pygls==1.2.0
          pygls==1.2.1
          pygls==1.3.0
      we can conclude that pygls>=1.1.0,<1.3.0 cannot be used. (1)

      Because the requested Python version (>=3.7) does not satisfy Python>=3.8 and pygls==1.3.0 depends on Python>=3.8, we can conclude that pygls==1.3.0 cannot be used.
      And because we know from (1) that pygls>=1.1.0,<1.3.0 cannot be used, we can conclude that pygls>=1.1.0 cannot be used.
      And because your project depends on pygls>=1.1.0, we can conclude that your project's requirements are unsatisfiable.

      hint: The `requires-python` value (>=3.7) includes Python versions that are not supported by your dependencies (e.g., pygls>=1.1.0,<=1.2.1 only supports >=3.7.9, <4). Consider using a more restrictive `requires-python` value (like >=3.7.9, <4).

      hint: While the active Python version is 3.12, the resolution failed for other Python versions supported by your project. Consider limiting your project's supported Python versions using `requires-python`.
```
