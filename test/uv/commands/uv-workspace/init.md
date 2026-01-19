# Workspace Initialization

Tests for initializing projects within workspaces using `uv init`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Initialize project inside project

<!-- Derived from [`init::init_project_inside_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs) -->

Creating a project inside another project automatically creates a workspace.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

Create a child project from the workspace root:

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

Create a grandchild project from the child directory:

```console working-dir="foo"
$ uv init bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `bar` as member of workspace `[TEMP_DIR]/`
Initialized project `bar` at `[TEMP_DIR]/foo/bar`
```

The workspace root pyproject.toml should have both members listed:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = [
    "foo",
    "foo/bar",
]
```

The child pyproject.toml should be a simple project:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Initialize in workspace with explicit members

<!-- Derived from [`init::init_explicit_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs) -->

Run `uv init` from within a workspace with an explicit members array.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = []
```

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The workspace members should be updated:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[tool.uv.workspace]
members = [
    "foo",
]
```

## Initialize in virtual workspace

<!-- Derived from [`init::init_virtual_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs) -->

Run `uv init` from within a virtual workspace (no root package).

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = []
```

```console
$ uv init bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `bar` as member of workspace `[TEMP_DIR]/`
Initialized project `bar` at `[TEMP_DIR]/bar`
```

The workspace should have the new member:

```toml title="pyproject.toml" snapshot=true
[tool.uv.workspace]
members = [
    "bar",
]
```

## Initialize nested virtual workspace

<!-- Derived from [`init::init_nested_virtual_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs) -->

Run `uv init --virtual` from within a workspace to create a virtual project.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = []
```

```console
$ uv init --virtual foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The new project should be a virtual project (no build-system):

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

The workspace root should have the member:

```toml title="pyproject.toml" snapshot=true
[tool.uv.workspace]
members = [
    "foo",
]
```

## Initialize when path matches members glob

<!-- Derived from [`init::init_matches_members`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs) -->

Run `uv init` when the path is already included via the `members` glob.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = ['packages/*']
```

Create the packages directory structure:

```tree create=true
packages/
└── foo/
```

Initialize the project - it should recognize it's already a member:

```console working-dir="packages"
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Project `foo` is already a member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/packages/foo`
```

The workspace should remain unchanged:

```toml title="pyproject.toml" snapshot=true
[tool.uv.workspace]
members = ['packages/*']
```

## Initialize when path matches exclude

<!-- Derived from [`init::init_matches_exclude`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs) -->

Run `uv init` when the path matches the `exclude` glob.

```toml
# file: pyproject.toml

[tool.uv.workspace]
exclude = ['packages/foo']
members = ['packages/*']
```

Create the packages directory:

```tree create=true
packages/
```

Initialize the project - it should recognize it's excluded:

```console working-dir="packages"
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Project `foo` is excluded by workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/packages/foo`
```

The workspace should remain unchanged:

```toml title="pyproject.toml" snapshot=true
[tool.uv.workspace]
exclude = ['packages/foo']
members = ['packages/*']
```

## Initialize with --no-workspace

<!-- Derived from [`init::init_no_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs) -->

Using `--no-workspace` prevents adding the project to the workspace.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

Create a directory and initialize with --no-workspace:

```tree create=true
foo/
```

```console working-dir="foo"
$ uv init --no-workspace
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The workspace root should remain unchanged:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
```

## Initialize multiple projects in workspace

<!-- Derived from [`init::init_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1320-L1503) -->

When initializing projects in a workspace, they are automatically added as workspace members.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

Create the first member:

```tree create=true
foo/
```

```console working-dir="foo"
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo`
```

The workspace should have the first member:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = [
    "foo",
]
```

Locking should resolve both workspace members and dependencies:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Add another member:

```tree create=true
bar/
```

```console working-dir="bar"
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `bar` as member of workspace `[TEMP_DIR]/`
Initialized project `bar`
```

The workspace should have both members:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = [
    "foo",
    "bar",
]
```

Add a third member:

```tree create=true
baz/
```

```console working-dir="baz"
$ uv init --lib
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `baz` as member of workspace `[TEMP_DIR]/`
Initialized project `baz`
```

The workspace should have all three members:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = [
    "foo",
    "bar",
    "baz",
]
```

## Initialize with relative path argument

<!-- Derived from [`init::init_workspace_relative_sub_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1506-L1595) -->

Running `uv init <path>` from the workspace root with a relative path.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv init --lib foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The workspace should have the member added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = [
    "foo",
]
```

Locking should work correctly:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

## Initialize from outside workspace directory

<!-- Derived from [`init::init_workspace_outside`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1602-L1696) -->

Running `uv init <path>` from outside the workspace (e.g., home directory) correctly adds the
member.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv init --lib foo
working-dir: [HOME]
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The workspace should have the member added:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[tool.uv.workspace]
members = [
    "foo",
]
```

Locking should work:

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

## Initialize with --no-workspace produces no warning

<!-- Derived from [`init::init_no_workspace_warning`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L1924-L1953) -->

Using `--no-workspace` flag produces no warning and creates a standalone project.

```console
$ uv init --no-workspace --name project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project`
```

The project should be a standalone project without workspace configuration:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []
```

## Initialize member inherits workspace requires-python

<!-- Derived from [`init::init_requires_python_workspace`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2344-L2398) -->

When initializing a member in a workspace, it inherits the workspace's `requires-python` instead of
using the default.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.10"

[tool.uv.workspace]
members = []
```

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Adding `foo` as member of workspace `[TEMP_DIR]/`
Initialized project `foo` at `[TEMP_DIR]/foo`
```

The member should inherit the workspace's `requires-python`:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.10"
dependencies = []
```

The member should have a `.python-version` file matching the test environment:

```console
$ cat foo/.python-version
success: true
exit_code: 0
----- stdout -----
3.12

----- stderr -----
```
