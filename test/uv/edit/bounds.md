# Dependency Management - Version Bounds

Tests for version bound behavior when adding dependencies with `uv add`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Lower bound

<!-- Derived from [`edit::add_lower_bound`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4527-L4575) -->

When adding unconstrained dependencies, a lower bound is automatically set.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

A lower bound is added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio>=4.3.0",
]
```

## Lower bound existing

<!-- Derived from [`edit::add_lower_bound_existing`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4577-L4626) -->

When adding a dependency that already exists, no lower bound is set.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio"]
```

```console
$ uv add anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

The existing constraint is preserved:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio",
]
```

## Lower bound raw

<!-- Derived from [`edit::add_lower_bound_raw`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4628-L4676) -->

The `--raw` flag skips setting a lower bound.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio"]
```

```console
$ uv add --raw anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

No lower bound is added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio",
]
```

## Lower bound dev

<!-- Derived from [`edit::add_lower_bound_dev`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4678-L4729) -->

Lower bounds are added for dev dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --dev anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

A lower bound is added in the dev group:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
dev = [
    "anyio>=4.3.0",
]
```

## Lower bound optional

<!-- Derived from [`edit::add_lower_bound_optional`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4731-L4844) -->

Lower bounds are added for optional dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --optional=io anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

A lower bound is added in the optional dependencies:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
io = [
    "anyio>=4.3.0",
]
```

## Lower bound local

<!-- Derived from [`edit::add_lower_bound_local`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L4846-L4929) -->

Local version segments are omitted from lower bounds (since `>=1.2.3+local` is invalid).

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add --index https://astral-sh.github.io/packse/PACKSE_VERSION/simple-html/ local-simple-a
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + local-simple-a==1.2.3+foo
```

The lower bound omits the local segment:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "local-simple-a>=1.2.3",
]

[[tool.uv.index]]
url = "https://astral-sh.github.io/packse/PACKSE_VERSION/simple-html/"
```

## Bounds

<!-- Derived from [`edit::add_bounds`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L14075-L14250) -->

The `add-bounds` option controls version bound behavior.

```toml
# file: uv.toml
add-bounds = "exact"
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

With `add-bounds = "exact"`, exact versions are used:

```console
$ uv add idna
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `bounds` option is in preview and may change in any future release. Pass `--preview-features add-bounds` to disable this warning.
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + idna==3.6
```

An exact version is added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "idna==3.6",
]
```

With `add-bounds = "major"` in pyproject.toml:

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv]
add-bounds = "major"
```

```console
$ uv add anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `bounds` option is in preview and may change in any future release. Pass `--preview-features add-bounds` to disable this warning.
Resolved 4 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + anyio==4.3.0
 + sniffio==1.3.1
```

A major version bound is added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio>=4.3.0,<5.0.0",
]

[tool.uv]
add-bounds = "major"
```

## Bounds requirement over bounds kind

<!-- Derived from [`edit::add_bounds_requirement_over_bounds_kind`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs#L14252-L14300) -->

Explicit version requirements take precedence over bounds preferences.

```toml
# file: uv.toml
add-bounds = "exact"
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

```console
$ uv add --preview --bounds minor anyio==4.2 idna
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
note: Using explicit requirement `anyio==4.2` over bounds preference `minor`
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.2.0
 + idna==3.6
 + sniffio==1.3.1
```

The explicit requirement is used as-is:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio==4.2",
    "idna>=3.6,<3.7",
]
```
