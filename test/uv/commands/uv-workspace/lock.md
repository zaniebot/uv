# Workspace Locking

Tests for locking workspace projects with `uv lock`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Lock idempotence tests

**Note:** The lock idempotence tests (`workspace_lock_idempotence_root_workspace` and
`workspace_lock_idempotence_virtual_workspace`) remain in the Rust test suite at
`crates/uv/tests/it/workspace.rs`. These tests use fixture data and helper functions that are better
suited to the Rust testing infrastructure.

## Lock with conflicting workspace members

<!-- Derived from [`lock::lock_conflicting_workspace_members`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L3094-L3275) -->

Workspace members can have conflicting dependencies when explicitly declared in `tool.uv.conflicts`.

```toml
# file: pyproject.toml

[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.3.0"]

[tool.uv.workspace]
members = ["subexample"]

[tool.uv]
conflicts = [
  [
    { package = "example" },
    { package = "subexample" },
  ],
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.setuptools.packages.find]
include = ["example"]
```

```toml
# file: subexample/pyproject.toml

[project]
name = "subexample"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking succeeds with the declared conflict:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Declaring conflicts for packages (`package = ...`) is experimental and may change without warning. Pass `--preview-features package-conflicts` to disable this warning.
Resolved 4 packages in [TIME]
```

Installing from the lockfile installs the root package:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + example==0.1.0 (from file://[TEMP_DIR]/)
 + sortedcontainers==2.3.0
```

Installing only subexample uses its dependency version:

```console
$ uv sync --frozen --package subexample
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 2 packages in [TIME]
Uninstalled 2 packages in [TIME]
Installed 2 packages in [TIME]
 - example==0.1.0 (from file://[TEMP_DIR]/)
 - sortedcontainers==2.3.0
 + sortedcontainers==2.4.0
 + subexample==0.1.0 (from file://[TEMP_DIR]/subexample)
```

Attempting to install both packages together fails:

```console
$ uv sync --frozen --all-packages
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package `example` and package `subexample` are incompatible with the declared conflicts: {example, subexample}
```

Explicit package selection also fails:

```console
$ uv sync --frozen --package example --package subexample
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package `example` and package `subexample` are incompatible with the declared conflicts: {example, subexample}
```

## Lock with conflicting dependencies on direct dependency

<!-- Derived from [`lock::lock_conflicting_workspace_members_depends_direct`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L3278-L3348) -->

When a root project directly depends on a conflicting workspace member, locking fails.

```toml
# file: pyproject.toml

[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.3.0", "subexample"]

[tool.uv.workspace]
members = ["subexample"]

[tool.uv]
conflicts = [
  [
    { package = "example" },
    { package = "subexample" },
  ],
]

[tool.uv.sources]
subexample = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.setuptools.packages.find]
include = ["example"]
```

```toml
# file: subexample/pyproject.toml

[project]
name = "subexample"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking fails because the root depends on both conflicting packages:

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
warning: Declaring conflicts for packages (`package = ...`) is experimental and may change without warning. Pass `--preview-features package-conflicts` to disable this warning.
  × No solution found when resolving dependencies for split (included: example; excluded: subexample):
  ╰─▶ Because subexample depends on sortedcontainers==2.4.0 and example depends on sortedcontainers==2.3.0, we can conclude that example and subexample are incompatible.
      And because example depends on subexample and your workspace requires example, we can conclude that your workspace's requirements are unsatisfiable.
```

## Lock with conflicting dependencies on direct dependency with extra

<!-- Derived from [`lock::lock_conflicting_workspace_members_depends_direct_extra`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L3351-L3549) -->

Conflicts can be declared for dependencies that are only present through extras.

```toml
# file: pyproject.toml

[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.3.0"]

[project.optional-dependencies]
foo = ["subexample"]

[tool.uv.workspace]
members = ["subexample"]

[tool.uv]
conflicts = [
    [
        { package = "example" },
        # TODO(zanieb): Technically, we shouldn't need to include the extra in the list of
        # conflicts however, the resolver forking algorithm is not currently sophisticated
        # enough to pick this up by itself
        { package = "example", extra = "foo"},
        { package = "subexample" },
    ],
]

[tool.uv.sources]
subexample = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.setuptools.packages.find]
include = ["example"]
```

```toml
# file: subexample/pyproject.toml

[project]
name = "subexample"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking succeeds because the conflict is optional:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Declaring conflicts for packages (`package = ...`) is experimental and may change without warning. Pass `--preview-features package-conflicts` to disable this warning.
Resolved 4 packages in [TIME]
```

Installing from the lockfile installs the root without the extra:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + example==0.1.0 (from file://[TEMP_DIR]/)
 + sortedcontainers==2.3.0
```

Attempting to install with the extra selected fails:

```console
$ uv sync --frozen --extra foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Extra `foo` and package `example` are incompatible with the declared conflicts: {`example[foo]`, example, subexample}
```

Installing just the child package works:

```console
$ uv sync --frozen --package subexample
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 2 packages in [TIME]
Uninstalled 2 packages in [TIME]
Installed 2 packages in [TIME]
 - example==0.1.0 (from file://[TEMP_DIR]/)
 - sortedcontainers==2.3.0
 + sortedcontainers==2.4.0
 + subexample==0.1.0 (from file://[TEMP_DIR]/subexample)
```

Installing with only development dependencies works:

```console
$ uv sync --frozen --only-dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 2 packages in [TIME]
 - sortedcontainers==2.4.0
 - subexample==0.1.0 (from file://[TEMP_DIR]/subexample)
```

## Lock with conflicting dependencies on transitive dependency

<!-- Derived from [`lock::lock_conflicting_workspace_members_depends_transitive`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L3552-L3646) -->

Members can have conflicts on transitive dependencies through an intermediate package.

```toml
# file: pyproject.toml

[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.3.0", "indirection"]

[tool.uv.workspace]
members = ["subexample", "indirection"]

[tool.uv]
conflicts = [
  [
    { package = "example" },
    { package = "subexample" },
  ],
]

[tool.uv.sources]
subexample = { workspace = true }
indirection = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.setuptools.packages.find]
include = ["example"]
```

```toml
# file: indirection/pyproject.toml

[project]
name = "indirection"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["subexample"]

[tool.uv.sources]
subexample = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: subexample/pyproject.toml

[project]
name = "subexample"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking fails due to the transitive conflict:

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
warning: Declaring conflicts for packages (`package = ...`) is experimental and may change without warning. Pass `--preview-features package-conflicts` to disable this warning.
  × No solution found when resolving dependencies for split (included: example; excluded: subexample):
  ╰─▶ Because subexample depends on sortedcontainers==2.4.0 and indirection depends on subexample, we can conclude that indirection depends on sortedcontainers==2.4.0.
      And because example depends on sortedcontainers==2.3.0, we can conclude that example and indirection are incompatible.
      And because your workspace requires example and indirection, we can conclude that your workspace's requirements are unsatisfiable.
```

## Lock with conflicting dependencies on transitive dependency with extra

<!-- Derived from [`lock::lock_conflicting_workspace_members_depends_transitive_extra`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L3649-L3849) -->

Transitive dependency conflicts through extras are supported.

```toml
# file: pyproject.toml

[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.3.0", "indirection[foo]"]

[tool.uv.workspace]
members = ["subexample", "indirection"]

[tool.uv]
conflicts = [
  [
    { package = "example" },
    { package = "subexample" },
  ],
]

[tool.uv.sources]
subexample = { workspace = true }
indirection = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[tool.setuptools.packages.find]
include = ["example"]
```

```toml
# file: indirection/pyproject.toml

[project]
name = "indirection"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
foo = ["subexample"]

[tool.uv.sources]
subexample = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: subexample/pyproject.toml

[project]
name = "subexample"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking succeeds, but the example package is unusable due to the unconditional conflict:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Declaring conflicts for packages (`package = ...`) is experimental and may change without warning. Pass `--preview-features package-conflicts` to disable this warning.
Resolved 5 packages in [TIME]
```

Installing from the lockfile fails due to the conflict:

```console
$ uv sync --frozen
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package `example` and package `subexample` are incompatible with the declared conflicts: {example, subexample}
```

Installing with `--only-dev` succeeds as it skips the conflicting packages:

```console
$ uv sync --frozen --only-dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited in [TIME]
```

Installing just the child package works:

```console
$ uv sync --frozen --package subexample
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + sortedcontainers==2.4.0
 + subexample==0.1.0 (from file://[TEMP_DIR]/subexample)
```

## Lock with non-workspace source

<!-- Derived from [`lock::lock_non_workspace_source`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L9229-L9280) -->

Workspace members must use `workspace = true` sources, not path sources.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { path = "child" }
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking fails because workspace members cannot use path sources:

```console working-dir="child"
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `project @ file://[TEMP_DIR]/`
  ├─▶ Failed to parse entry: `child`
  ╰─▶ `child` is included as a workspace member, but references a path in `tool.uv.sources`. Workspace members must be declared as workspace sources (e.g., `child = { workspace = true }`).
```

## Lock with no workspace source

<!-- Derived from [`lock::lock_no_workspace_source`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L9283-L9436) -->

Workspace members must have an entry in `tool.uv.sources`.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[tool.uv.workspace]
members = ["child"]
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking fails because the workspace member is missing a source entry:

```console working-dir="child"
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `project @ file://[TEMP_DIR]/`
  ├─▶ Failed to parse entry: `child`
  ╰─▶ `child` is included as a workspace member, but is missing an entry in `tool.uv.sources` (e.g., `child = { workspace = true }`)
```

## Lock workspace member from index

<!-- Derived from [`lock::lock_index_workspace_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L9439-L9534) -->

Workspace members can define explicit indexes that require authentication.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>=2"]

[[tool.uv.index]]
name = "my-index"
url = "https://pypi-proxy.fly.dev/basic-auth/simple"
explicit = true

[tool.uv.sources]
iniconfig = { index = "my-index" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking without credentials fails:

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because iniconfig was not found in the package registry and child depends on iniconfig>=2, we can conclude that child's requirements are unsatisfiable.
      And because your workspace requires child, we can conclude that your workspace's requirements are unsatisfiable.
```

Locking with credentials succeeds:

```toml
# mdtest

[environment.env]
UV_INDEX_MY_INDEX_USERNAME = "public"
UV_INDEX_MY_INDEX_PASSWORD = "heron"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

Re-running with `--locked` succeeds:

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Lock with dependency groups in workspace

<!-- Derived from [`lock::lock_group_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L24525-L24775) -->

Dependency groups work correctly in workspace contexts.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[dependency-groups]
types = ["sniffio>1"]
async = ["anyio>3"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig>1"]

[dependency-groups]
types = ["typing-extensions>4"]
testing = ["pytest>8"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Locking resolves all dependency groups from both workspace members:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 11 packages in [TIME]
```

## Lock workspace member with dynamic version

<!-- Derived from [`lock::lock_dynamic_version_workspace_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L25174-L25305) -->

Workspace members with dynamic versions (from build backends) are supported.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["dynamic", "iniconfig>=2"]

[tool.uv.workspace]
members = ["dynamic"]

[tool.uv.sources]
dynamic = { workspace = true }
```

```toml
# file: dynamic/pyproject.toml

[project]
name = "dynamic"
requires-python = ">=3.12"
dynamic = ["version"]

[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"

[tool.uv]
cache-keys = [{ file = "pyproject.toml" }, { file = "src/dynamic/__init__.py" }]

[tool.setuptools.dynamic]
version = { attr = "dynamic.__version__" }

[tool.setuptools]
package-dir = { "" = "src" }

[tool.setuptools.packages.find]
where = ["src"]
```

```python
# file: dynamic/src/dynamic/__init__.py
__version__ = '0.1.0'
```

Locking succeeds with dynamic version:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

After bumping the version, `--locked` still succeeds (dynamic versions are omitted from lockfile):

```python
# file: dynamic/src/dynamic/__init__.py
__version__ = '0.1.1'
```

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Lock path dependency with explicit index in workspace

<!-- Derived from [`lock::lock_path_dependency_explicit_index_workspace_member`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/lock.rs#L32274-L32388) -->

Path dependencies in workspaces can explicitly specify an index.

```toml
# file: pkg_a/pyproject.toml

[project]
name = "pkg-a"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[tool.uv.sources]
iniconfig = { index = "inner-index" }

[[tool.uv.index]]
name = "inner-index"
url = "https://pypi-proxy.fly.dev/simple"
explicit = true
```

```toml
# file: member/pyproject.toml

[project]
name = "member"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["pkg-a"]

[tool.uv.sources]
pkg-a = { path = "../pkg_a/", editable = true }
black = { index = "middle-index" }

[[tool.uv.index]]
name = "middle-index"
url = "https://middle-index.com/simple"
explicit = true
```

```toml
# file: pyproject.toml

[project]
name = "root-project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["member"]

[tool.uv.workspace]
members = ["member"]

[tool.uv.sources]
member = { workspace = true }
anyio = { index = "outer-index" }

[[tool.uv.index]]
name = "outer-index"
url = "https://outer-index.com/simple"
explicit = true
```

Locking succeeds with nested explicit indexes:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Re-running with `--check` succeeds:

```console
$ uv lock --check
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

---

**Migration Note**: The locking tests are particularly complex as they involve detailed lockfile
validation and often use fixture data from the test suite. These 12 tests represent the remaining
workspace tests to be fully migrated to mdtest format. The tests cover:

- Lock idempotence across workspace directories
- Conflict resolution between workspace members
- Various source types (workspace, path, index)
- Dynamic versions
- Dependency groups

To complete the migration, each test needs:

1. Setup of the workspace structure
2. The locking command
3. Validation of the resulting `uv.lock` file contents
4. Often includes snapshots of complex lockfile structures
