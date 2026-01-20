# Dependency Management - Groups

Tests for dependency groups (dev, optional, custom) with `uv add` and `uv remove`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Add remove dev

<!-- Derived from [`edit::add_remove_dev`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L1307-L1512) -->

Dev dependencies can be added and removed with the `--dev` flag.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --dev anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The dependency is added to the dev group:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
dev = [
    "anyio==3.7.0",
]
```

Removing without `--dev` fails with a helpful hint:

```console
$ uv remove anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
hint: `anyio` is in the `dev` group (try: `uv remove anyio --group dev`)
error: The dependency `anyio` could not be found in `project.dependencies`
```

Removing with `--dev` succeeds:

```console
$ uv remove --dev anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Uninstalled 3 packages in [TIME]
 - anyio==3.7.0
 - idna==3.6
 - sniffio==1.3.1
```

## Add remove optional

<!-- Derived from [`edit::add_remove_optional`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L1514-L1716) -->

Optional dependencies can be added and removed with the `--optional` flag.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --optional=io anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The dependency is added to optional dependencies:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
io = [
    "anyio==3.7.0",
]
```

Removing without `--optional` fails:

```console
$ uv remove anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
hint: `anyio` is in the `io` extra (try: `uv remove anyio --optional io`)
error: The dependency `anyio` could not be found in `project.dependencies`
```

Removing with `--optional` succeeds:

```console
$ uv remove --optional=io anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Uninstalled 3 packages in [TIME]
 - anyio==3.7.0
 - idna==3.6
 - sniffio==1.3.1
```

## Add remove inline optional

<!-- Derived from [`edit::add_remove_inline_optional`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L1718-L2047) -->

Inline optional dependencies can be added and removed.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
optional-dependencies = ["anyio==3.7.0"]
```

```console
$ uv add --optional=io anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The dependency is added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
optional-dependencies = ["anyio==3.7.0"]

[project.optional-dependencies]
io = [
    "anyio==3.7.0",
]
```

Removing the dependency:

```console
$ uv remove --optional=io anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Uninstalled 3 packages in [TIME]
 - anyio==3.7.0
 - idna==3.6
 - sniffio==1.3.1
```

## Update existing dev

<!-- Derived from [`edit::update_existing_dev`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2049-L2110) -->

Adding a dev dependency that exists in main dependencies moves it.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv add --dev anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Audited 3 packages in [TIME]
```

The dependency moves to dev:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
dev = [
    "anyio==3.7.0",
]
```

## Add existing dev

<!-- Derived from [`edit::add_existing_dev`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2112-L2167) -->

Adding a dev dependency that's already in dev is a no-op.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
dev = ["anyio==3.7.0"]
```

```console
$ uv add --dev anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Audited 3 packages in [TIME]
```

The dependency remains unchanged:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
dev = [
    "anyio==3.7.0",
]
```

## Update existing dev group

<!-- Derived from [`edit::update_existing_dev_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2169-L2223) -->

Adding a dependency to a custom group moves it from main dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv add --group foo anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Audited 3 packages in [TIME]
```

The dependency moves to the custom group:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = [
    "anyio==3.7.0",
]
```

## Add existing dev group

<!-- Derived from [`edit::add_existing_dev_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2225-L2282) -->

Adding a dependency that's already in a custom group is a no-op.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = ["anyio==3.7.0"]
```

```console
$ uv add --group foo anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Audited 3 packages in [TIME]
```

The dependency remains unchanged:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = [
    "anyio==3.7.0",
]
```

## Remove both dev

<!-- Derived from [`edit::remove_both_dev`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2284-L2338) -->

When a dependency exists in both main and dev, removal is ambiguous.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[dependency-groups]
dev = ["anyio==3.7.0"]
```

```console
$ uv remove anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `anyio` is present in multiple dependency sets; please specify the dependency set to remove it from with one of: `--group dev`, or `--dev` for short
```

## Remove both dev group

<!-- Derived from [`edit::remove_both_dev_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2376-L2433) -->

When a dependency exists in both main and a custom group, removal is ambiguous.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[dependency-groups]
foo = ["anyio==3.7.0"]
```

```console
$ uv remove anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `anyio` is present in multiple dependency sets; please specify the dependency set to remove it from with one of: `--group foo`
```

## Disallow group script add

<!-- Derived from [`edit::disallow_group_script_add`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L2340-L2374) -->

Adding dependencies to groups is not allowed in scripts.

```python
# file: main.py
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
```

```console
$ uv add --script main.py --group foo anyio
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Groups are not supported for Python scripts
```

## Add group

<!-- Derived from [`edit::add_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L6169-L6311) -->

Custom groups can be created with `--group`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --group foo anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

A custom group is created:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = [
    "anyio==3.7.0",
]
```

## Add group normalize

<!-- Derived from [`edit::add_group_normalize`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L6313-L6457) -->

Group names are normalized.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --group "Foo Bar" anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The group name is normalized:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo-bar = [
    "anyio==3.7.0",
]
```

## Add group before commented groups

<!-- Derived from [`edit::add_group_before_commented_groups`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L6459-L6526) -->

New groups are added before commented groups.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
# Commented group
# bar = []
```

```console
$ uv add --group foo anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The new group is added before comments:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = [
    "anyio==3.7.0",
]
# Commented group
# bar = []
```

## Add group between commented groups

<!-- Derived from [`edit::add_group_between_commented_groups`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L6528-L6596) -->

New groups are added between commented groups alphabetically.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
# Comment A
# aaa = []
# Comment C
# ccc = []
```

```console
$ uv add --group bbb anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The new group is inserted alphabetically:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
# Comment A
# aaa = []
bbb = [
    "anyio==3.7.0",
]
# Comment C
# ccc = []
```

## Add group to unsorted

<!-- Derived from [`edit::add_group_to_unsorted`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L6598-L6661) -->

Groups can be added to unsorted dependency groups.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
zzz = []
aaa = []
```

```console
$ uv add --group mmm anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The new group is added at the end:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
zzz = []
aaa = []
mmm = [
    "anyio==3.7.0",
]
```

## Remove group

<!-- Derived from [`edit::remove_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L6663-L7112) -->

Dependencies can be removed from custom groups.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = ["anyio==3.7.0"]
```

```console
$ uv remove --group foo anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Uninstalled 3 packages in [TIME]
 - anyio==3.7.0
 - idna==3.6
 - sniffio==1.3.1
```

The dependency is removed:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = []
```

## Add group comment

<!-- Derived from [`edit::add_group_comment`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L11152-L11257) -->

Comments in groups are preserved when adding dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
# This is a comment
foo = []
```

```console
$ uv add --group foo anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The comment is preserved:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
# This is a comment
foo = [
    "anyio==3.7.0",
]
```

## Add empty requirements group

<!-- Derived from [`edit::add_empty_requirements_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L5190-L5241) -->

Empty dependency groups can be created.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --group foo --requirements /dev/null
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

An empty group is created:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
foo = []
```

## Add empty requirements optional

<!-- Derived from [`edit::add_empty_requirements_optional`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L5243-L5294) -->

Empty optional dependencies can be created.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --optional foo --requirements /dev/null
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

An empty optional group is created:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
foo = []
```

## Add include default groups

<!-- Derived from [`edit::add_include_default_groups`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L8399-L8463) -->

The special `__default__` group includes default dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --group __default__ anyio==3.7.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The dependency is added to default dependencies:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio==3.7.0",
]
```

## Remove include default groups

<!-- Derived from [`edit::remove_include_default_groups`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L8465-L8529) -->

Dependencies can be removed from the `__default__` group.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv remove --group __default__ anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Uninstalled 3 packages in [TIME]
 - anyio==3.7.0
 - idna==3.6
 - sniffio==1.3.1
```

The dependency is removed:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

## Add optional normalize

<!-- Derived from [`edit::add_optional_normalize`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L13929-L14003) -->

Optional dependency group names are normalized.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
cloud_export_to_parquet = [
    "anyio==3.7.0",
]
```

```console
$ uv add --optional cloud_export_to_parquet iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + iniconfig==2.0.0
 + sniffio==1.3.1
```

The normalized name is used:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
cloud-export-to-parquet = [
    "anyio==3.7.0",
    "iniconfig>=2.0.0",
]
```

## Dependency groups

### Add to dependency group

<!-- from edit.rs::add_group -->

The `--group` flag adds dependencies to a dependency group.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add anyio==3.7.0 --group test
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

The dependency group is created:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
test = [
    "anyio==3.7.0",
]
```
