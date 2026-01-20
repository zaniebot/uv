# Tree Basics

Tests for basic `uv tree` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Basic tree display

### Nested dependencies

<!-- from tree.rs::nested_dependencies -->

Display the dependency tree with nested dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["scikit-learn==1.4.1.post1"]
```

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── scikit-learn v1.4.1.post1
    ├── joblib v1.3.2
    ├── numpy v1.26.4
    ├── scipy v1.12.0
    │   └── numpy v1.26.4
    └── threadpoolctl v3.4.0

----- stderr -----
Resolved 6 packages in [TIME]
```

## Inverted tree

### Basic invert

<!-- from tree.rs::invert -->

The `--invert` flag shows which packages depend on each package.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["scikit-learn==1.4.1.post1"]
```

```console
$ uv tree --invert
success: true
exit_code: 0
----- stdout -----
joblib v1.3.2
└── scikit-learn v1.4.1.post1
    └── project v0.1.0
numpy v1.26.4
├── scikit-learn v1.4.1.post1 (*)
└── scipy v1.12.0
    └── scikit-learn v1.4.1.post1 (*)
threadpoolctl v3.4.0
└── scikit-learn v1.4.1.post1 (*)
(*) Package tree already displayed

----- stderr -----
Resolved 6 packages in [TIME]
```

### Invert with no-dedupe

<!-- from tree.rs::invert -->

The `--no-dedupe` flag shows the full tree without deduplication.

```console
$ uv tree --invert --no-dedupe
success: true
exit_code: 0
----- stdout -----
joblib v1.3.2
└── scikit-learn v1.4.1.post1
    └── project v0.1.0
numpy v1.26.4
├── scikit-learn v1.4.1.post1
│   └── project v0.1.0
└── scipy v1.12.0
    └── scikit-learn v1.4.1.post1
        └── project v0.1.0
threadpoolctl v3.4.0
└── scikit-learn v1.4.1.post1
    └── project v0.1.0

----- stderr -----
Resolved 6 packages in [TIME]
```

## Frozen mode

### Tree with frozen shows stale data

<!-- from tree.rs::frozen -->

Running `uv tree --frozen` shows the tree from the existing lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio"]
```

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── anyio v4.3.0
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
Resolved 4 packages in [TIME]
```

Update the dependencies and verify `--frozen` shows the old tree.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv tree --frozen
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── anyio v4.3.0
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
```

## Outdated packages

### Show outdated versions

<!-- from tree.rs::outdated -->

The `--outdated` flag shows the latest version available.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.0.0"]
```

```console
$ uv tree --outdated --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── anyio v3.0.0 (latest: v4.3.0)
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
Resolved 4 packages in [TIME]
```

## Package filtering

### Filter by package

<!-- from tree.rs::package -->

The `--package` flag shows the tree for a specific package.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["scikit-learn==1.4.1.post1", "pandas"]
```

```console
$ uv tree
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── pandas v2.2.1
│   ├── numpy v1.26.4
│   ├── python-dateutil v2.9.0.post0
│   │   └── six v1.16.0
│   ├── pytz v2024.1
│   └── tzdata v2024.1
└── scikit-learn v1.4.1.post1
    ├── joblib v1.3.2
    ├── numpy v1.26.4
    ├── scipy v1.12.0
    │   └── numpy v1.26.4
    └── threadpoolctl v3.4.0

----- stderr -----
Resolved 11 packages in [TIME]
```

```console
$ uv tree --package scipy
success: true
exit_code: 0
----- stdout -----
scipy v1.12.0
└── numpy v1.26.4

----- stderr -----
Resolved 11 packages in [TIME]
```

### Filter by package inverted

<!-- from tree.rs::package -->

The `--package` flag combined with `--invert` shows who depends on a package.

```console
$ uv tree --package numpy --invert
success: true
exit_code: 0
----- stdout -----
numpy v1.26.4
├── pandas v2.2.1
│   └── project v0.1.0
├── scikit-learn v1.4.1.post1
│   └── project v0.1.0
└── scipy v1.12.0
    └── scikit-learn v1.4.1.post1 (*)
(*) Package tree already displayed

----- stderr -----
Resolved 11 packages in [TIME]
```

## Dependency groups

### Tree with dependency groups

<!-- from tree.rs::group -->

Dependency groups are shown with the group annotation.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio"]
bar = ["iniconfig"]
dev = ["sniffio"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv tree
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── typing-extensions v4.10.0
└── sniffio v1.3.1 (group: dev)

----- stderr -----
Resolved 6 packages in [TIME]
```

### Show only specific group

<!-- from tree.rs::group -->

The `--only-group` flag shows only a specific dependency group.

```console
$ uv tree --only-group bar
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── iniconfig v2.0.0 (group: bar)

----- stderr -----
Resolved 6 packages in [TIME]
```

### Show additional group

<!-- from tree.rs::group -->

The `--group` flag adds a group to the tree output.

```console
$ uv tree --group foo
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── typing-extensions v4.10.0
├── sniffio v1.3.1 (group: dev)
└── anyio v4.3.0 (group: foo)
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
Resolved 6 packages in [TIME]
```

### Show multiple groups

<!-- from tree.rs::group -->

Multiple `--group` flags can be used together.

```console
$ uv tree --group foo --group bar
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── typing-extensions v4.10.0
├── iniconfig v2.0.0 (group: bar)
├── sniffio v1.3.1 (group: dev)
└── anyio v4.3.0 (group: foo)
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
Resolved 6 packages in [TIME]
```

## Optional dependencies

### Tree with optional dependencies

<!-- from tree.rs::optional_dependencies -->

Optional dependencies are shown with the extra annotation.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig", "flask[dotenv]"]

[project.optional-dependencies]
async = ["anyio"]
```

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── flask[dotenv] v3.0.2
│   ├── blinker v1.7.0
│   ├── click v8.1.7
│   │   └── colorama v0.4.6
│   ├── itsdangerous v2.1.2
│   ├── jinja2 v3.1.3
│   │   └── markupsafe v2.1.5
│   ├── werkzeug v3.0.1
│   │   └── markupsafe v2.1.5
│   └── python-dotenv v1.0.1 (extra: dotenv)
├── iniconfig v2.0.0
└── anyio v4.3.0 (extra: async)
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
Resolved 14 packages in [TIME]
```

## Cyclic dependencies

### Tree with cycles

<!-- from tree.rs::cycle -->

The tree shows cyclic dependencies with deduplication markers.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["testtools==2.3.0", "fixtures==3.0.0"]
```

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── fixtures v3.0.0
│   ├── pbr v6.0.0
│   ├── six v1.16.0
│   └── testtools v2.3.0
│       ├── extras v1.0.0
│       ├── fixtures v3.0.0 (*)
│       ├── pbr v6.0.0
│       ├── python-mimeparse v1.6.0
│       ├── six v1.16.0
│       ├── traceback2 v1.4.0
│       │   └── linecache2 v1.0.0
│       └── unittest2 v1.1.0
│           ├── argparse v1.4.0
│           ├── six v1.16.0
│           └── traceback2 v1.4.0 (*)
└── testtools v2.3.0 (*)
(*) Package tree already displayed

----- stderr -----
Resolved 11 packages in [TIME]
```

## Dev dependencies

### Tree with dev dependencies

<!-- from tree.rs::dev_dependencies -->

The tree includes dev dependencies with group annotation.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[tool.uv]
dev-dependencies = ["anyio"]
```

```console
$ uv tree
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── iniconfig v2.0.0
└── anyio v4.3.0 (group: dev)
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 5 packages in [TIME]
```

### Tree excluding dev dependencies

<!-- from tree.rs::dev_dependencies -->

The `--no-dev` flag excludes dev dependencies from the tree.

```console
$ uv tree --no-dev
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── iniconfig v2.0.0

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 5 packages in [TIME]
```

## Repeated dependencies

### Tree with repeated dependencies

<!-- from tree.rs::repeated_dependencies -->

In universal mode, the tree shows multiple versions of a package with different dependencies.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "anyio < 2 ; sys_platform == 'win32'",
    "anyio > 2 ; sys_platform == 'linux'",
]
```

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── anyio v1.4.0
│   ├── async-generator v1.10
│   ├── idna v3.6
│   └── sniffio v1.3.1
└── anyio v4.3.0
    ├── idna v3.6
    └── sniffio v1.3.1

----- stderr -----
Resolved 6 packages in [TIME]
```

## Show sizes

### Tree with package sizes

<!-- from tree.rs::show_sizes -->

The `--show-sizes` flag displays the installed size of each package.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]
```

```console
$ uv tree --show-sizes --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── iniconfig v2.0.0 ([SIZE])

----- stderr -----
Resolved 2 packages in [TIME]
```

## Virtual workspace

### Tree for non-project workspace

<!-- from tree.rs::non_project -->

A virtual workspace (no [project]) with dependency groups shows the tree correctly.

```toml
# file: pyproject.toml
[tool.uv.workspace]
members = []

[dependency-groups]
async = ["anyio"]
```

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
anyio v4.3.0 (group: async)
├── idna v3.6
└── sniffio v1.3.1

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 3 packages in [TIME]
```

## Only dependency group

### Tree with only one group

<!-- from tree.rs::only_group -->

The `--only-group` flag shows only the specified dependency group.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "iniconfig",
    "pip",
]

[dependency-groups]
dev = [
    "plotly",
    "pip",
]
test = [
    "pytest",
]
```

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── iniconfig v2.0.0
├── pip v24.0
├── pip v24.0 (group: dev)
└── plotly v5.20.0 (group: dev)
    ├── packaging v24.0
    └── tenacity v8.2.3

----- stderr -----
Resolved 9 packages in [TIME]
```

```console
$ uv tree --universal --only-group dev
success: true
exit_code: 0
----- stdout -----
project v0.1.0
├── pip v24.0 (group: dev)
└── plotly v5.20.0 (group: dev)
    ├── packaging v24.0
    └── tenacity v8.2.3

----- stderr -----
Resolved 9 packages in [TIME]
```

## Inverted dev dependencies

### Dev dependencies inverted

<!-- from tree.rs::dev_dependencies_inverted -->

The inverted tree shows dev dependencies with group annotation.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[tool.uv]
dev-dependencies = ["anyio"]
```

```console
$ uv tree --universal --invert
success: true
exit_code: 0
----- stdout -----
idna v3.6
└── anyio v4.3.0
    └── project v0.1.0 (group: dev)
iniconfig v2.0.0
└── project v0.1.0
sniffio v1.3.1
└── anyio v4.3.0 (*)
(*) Package tree already displayed

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 5 packages in [TIME]
```

### Dev dependencies inverted without dev

<!-- from tree.rs::dev_dependencies_inverted -->

The `--no-dev` flag excludes dev dependencies from the inverted tree.

```console
$ uv tree --universal --invert --no-dev
success: true
exit_code: 0
----- stdout -----
iniconfig v2.0.0
└── project v0.1.0

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 5 packages in [TIME]
```

## Inverted optional dependencies

### Optional dependencies inverted

<!-- from tree.rs::optional_dependencies_inverted -->

The inverted tree shows optional dependencies with extra annotation.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig", "flask[dotenv]"]

[project.optional-dependencies]
async = ["anyio"]
```

```console
$ uv tree --universal --invert
success: true
exit_code: 0
----- stdout -----
blinker v1.7.0
└── flask v3.0.2
    └── project[dotenv] v0.1.0
colorama v0.4.6
└── click v8.1.7
    └── flask v3.0.2 (*)
idna v3.6
└── anyio v4.3.0
    └── project v0.1.0 (extra: async)
iniconfig v2.0.0
└── project v0.1.0
itsdangerous v2.1.2
└── flask v3.0.2 (*)
markupsafe v2.1.5
├── jinja2 v3.1.3
│   └── flask v3.0.2 (*)
└── werkzeug v3.0.1
    └── flask v3.0.2 (*)
python-dotenv v1.0.1
└── flask v3.0.2 (extra: dotenv) (*)
sniffio v1.3.1
└── anyio v4.3.0 (*)
(*) Package tree already displayed

----- stderr -----
Resolved 14 packages in [TIME]
```

## Platform dependencies

### Nested platform dependencies

<!-- from tree.rs::nested_platform_dependencies -->

Show dependencies that are platform-specific with `--python-platform`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["jupyter-client"]
```

For Linux, some platform-specific dependencies are excluded:

```console
$ uv tree --python-platform linux
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── jupyter-client v8.6.1
    ├── jupyter-core v5.7.2
    │   ├── platformdirs v4.2.0
    │   └── traitlets v5.14.2
    ├── python-dateutil v2.9.0.post0
    │   └── six v1.16.0
    ├── pyzmq v25.1.2
    ├── tornado v6.4
    └── traitlets v5.14.2

----- stderr -----
Resolved 12 packages in [TIME]
```

### Universal platform dependencies

<!-- from tree.rs::nested_platform_dependencies -->

With `--universal`, platform-specific dependencies are included:

```console
$ uv tree --universal
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── jupyter-client v8.6.1
    ├── jupyter-core v5.7.2
    │   ├── platformdirs v4.2.0
    │   ├── pywin32 v306
    │   └── traitlets v5.14.2
    ├── python-dateutil v2.9.0.post0
    │   └── six v1.16.0
    ├── pyzmq v25.1.2
    │   └── cffi v1.16.0
    │       └── pycparser v2.21
    ├── tornado v6.4
    └── traitlets v5.14.2

----- stderr -----
Resolved 12 packages in [TIME]
```

### Platform-specific colorama dependency

<!-- from tree.rs::platform_dependencies -->

Colorama is only included on Windows. Use `--python-platform` to simulate a different platform.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["black"]
```

On Windows, colorama is included:

```console
$ uv tree --python-platform windows
success: true
exit_code: 0
----- stdout -----
project v0.1.0
└── black v24.3.0
    ├── click v8.1.7
    │   └── colorama v0.4.6
    ├── mypy-extensions v1.0.0
    ├── packaging v24.0
    ├── pathspec v0.12.1
    └── platformdirs v4.2.0

----- stderr -----
Resolved 8 packages in [TIME]
```

### Platform-specific inverted tree

<!-- from tree.rs::platform_dependencies_inverted -->

The inverted tree also respects `--python-platform`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["click"]
```

On Linux, colorama is not shown:

```console
$ uv tree --invert --python-platform linux
success: true
exit_code: 0
----- stdout -----
click v8.1.7
└── project v0.1.0

----- stderr -----
Resolved 3 packages in [TIME]
```

On Windows, colorama is shown:

```console
$ uv tree --invert --python-platform windows
success: true
exit_code: 0
----- stdout -----
colorama v0.4.6
└── click v8.1.7
    └── project v0.1.0

----- stderr -----
Resolved 3 packages in [TIME]
```
