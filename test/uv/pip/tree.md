# pip tree

Visualize installed package dependencies as a tree.

```toml
[environment]
target-family = "unix"
```

## No packages

<!-- from pip_tree.rs::no_package -->

Empty environment shows no output.

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Single package

<!-- from pip_tree.rs::single_package -->

Show dependency tree for a single package.

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
requests v2.31.0
├── certifi v2024.2.2
├── charset-normalizer v3.3.2
├── idna v3.6
└── urllib3 v2.2.1

----- stderr -----
```

## Nested dependencies

<!-- from pip_tree.rs::nested_dependencies -->

Show nested dependency tree with transitive dependencies.

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
flask v3.0.2
├── blinker v1.7.0
├── click v8.1.7
├── itsdangerous v2.1.2
├── jinja2 v3.1.3
│   └── markupsafe v2.1.5
└── werkzeug v3.0.1
    └── markupsafe v2.1.5

----- stderr -----
```

## Reverse/invert tree

<!-- from pip_tree.rs::reverse -->

Show which packages depend on each package (--reverse is alias for --invert).

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

```console
$ uv pip tree --reverse
success: true
exit_code: 0
----- stdout -----
blinker v1.7.0
└── flask v3.0.2
click v8.1.7
└── flask v3.0.2
itsdangerous v2.1.2
└── flask v3.0.2
markupsafe v2.1.5
├── jinja2 v3.1.3
│   └── flask v3.0.2
└── werkzeug v3.0.1
    └── flask v3.0.2

----- stderr -----
```

## Invert flag

<!-- from pip_tree.rs::invert -->

Use --invert to show reverse dependencies.

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

```console
$ uv pip tree --invert
success: true
exit_code: 0
----- stdout -----
blinker v1.7.0
└── flask v3.0.2
click v8.1.7
└── flask v3.0.2
itsdangerous v2.1.2
└── flask v3.0.2
markupsafe v2.1.5
├── jinja2 v3.1.3
│   └── flask v3.0.2
└── werkzeug v3.0.1
    └── flask v3.0.2

----- stderr -----
```

## Depth limiting

<!-- from pip_tree.rs::depth -->

Limit tree depth with --depth flag.

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

Show only top-level packages (depth 0):

```console
$ uv pip tree --depth 0
success: true
exit_code: 0
----- stdout -----
flask v3.0.2

----- stderr -----
```

Show one level deep (depth 1):

```console
$ uv pip tree --depth 1
success: true
exit_code: 0
----- stdout -----
flask v3.0.2
├── blinker v1.7.0
├── click v8.1.7
├── itsdangerous v2.1.2
├── jinja2 v3.1.3
└── werkzeug v3.0.1

----- stderr -----
```

Show two levels deep (depth 2):

```console
$ uv pip tree --depth 2
success: true
exit_code: 0
----- stdout -----
flask v3.0.2
├── blinker v1.7.0
├── click v8.1.7
├── itsdangerous v2.1.2
├── jinja2 v3.1.3
│   └── markupsafe v2.1.5
└── werkzeug v3.0.1
    └── markupsafe v2.1.5

----- stderr -----
```

## Pruning packages

<!-- from pip_tree.rs::prune -->

Exclude specific packages from the tree with --prune.

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

Prune werkzeug from the tree:

```console
$ uv pip tree --prune werkzeug
success: true
exit_code: 0
----- stdout -----
flask v3.0.2
├── blinker v1.7.0
├── click v8.1.7
├── itsdangerous v2.1.2
└── jinja2 v3.1.3
    └── markupsafe v2.1.5

----- stderr -----
```

Prune multiple packages:

```console
$ uv pip tree --prune werkzeug --prune jinja2
success: true
exit_code: 0
----- stdout -----
flask v3.0.2
├── blinker v1.7.0
├── click v8.1.7
└── itsdangerous v2.1.2
markupsafe v2.1.5

----- stderr -----
```

## Prune last in subgroup

<!-- from pip_tree.rs::prune_last_in_the_subgroup -->

Prune works when removing the last item in a subgroup.

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

```console
$ uv pip tree --prune certifi
success: true
exit_code: 0
----- stdout -----
requests v2.31.0
├── charset-normalizer v3.3.2
├── idna v3.6
└── urllib3 v2.2.1

----- stderr -----
```

## After removing dependency

<!-- from pip_tree.rs::removed_dependency -->

Tree updates correctly after uninstalling a package.

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

```console
$ uv pip uninstall requests
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 1 package in [TIME]
 - requests==2.31.0
```

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
certifi v2024.2.2
charset-normalizer v3.3.2
idna v3.6
urllib3 v2.2.1

----- stderr -----
```

## Multiple root packages

<!-- from pip_tree.rs::multiple_packages -->

Display tree with multiple root packages.

```toml title="requirements.txt" snapshot=true
requests==2.31.0
click==8.1.7
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 6 packages in [TIME]
Installed 6 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + click==8.1.7
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
click v8.1.7
requests v2.31.0
├── certifi v2024.2.2
├── charset-normalizer v3.3.2
├── idna v3.6
└── urllib3 v2.2.1

----- stderr -----
```

## Dependency cycles

<!-- from pip_tree.rs::cycle -->

Handle circular dependencies correctly.

```toml title="requirements.txt" snapshot=true
testtools==2.3.0
fixtures==3.0.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Prepared 10 packages in [TIME]
Installed 10 packages in [TIME]
 + argparse==1.4.0
 + extras==1.0.0
 + fixtures==3.0.0
 + linecache2==1.0.0
 + pbr==6.0.0
 + python-mimeparse==1.6.0
 + six==1.16.0
 + testtools==2.3.0
 + traceback2==1.4.0
 + unittest2==1.1.0
```

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
testtools v2.3.0
├── extras v1.0.0
├── fixtures v3.0.0
│   ├── pbr v6.0.0
│   ├── six v1.16.0
│   └── testtools v2.3.0 (*)
├── pbr v6.0.0
├── python-mimeparse v1.6.0
├── six v1.16.0
├── traceback2 v1.4.0
│   └── linecache2 v1.0.0
└── unittest2 v1.1.0
    ├── argparse v1.4.0
    ├── six v1.16.0
    └── traceback2 v1.4.0 (*)
(*) Package tree already displayed

----- stderr -----
```

## Shared descendant

<!-- from pip_tree.rs::multiple_packages_shared_descendant -->

Multiple packages sharing a common dependency.

```toml title="requirements.txt" snapshot=true
pendulum
time-machine
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + pendulum==3.0.0
 + python-dateutil==2.9.0.post0
 + six==1.16.0
 + time-machine==2.14.1
 + tzdata==2024.1
```

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
pendulum v3.0.0
├── python-dateutil v2.9.0.post0
│   └── six v1.16.0
├── time-machine v2.14.1
│   └── python-dateutil v2.9.0.post0 (*)
└── tzdata v2024.1
(*) Package tree already displayed

----- stderr -----
```

## No dedupe with invert

<!-- from pip_tree.rs::no_dedupe_and_invert -->

Disable deduplication with --no-dedupe and --invert.

```toml title="requirements.txt" snapshot=true
pendulum
time-machine
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + pendulum==3.0.0
 + python-dateutil==2.9.0.post0
 + six==1.16.0
 + time-machine==2.14.1
 + tzdata==2024.1
```

```console
$ uv pip tree --no-dedupe --invert
success: true
exit_code: 0
----- stdout -----
six v1.16.0
└── python-dateutil v2.9.0.post0
    ├── pendulum v3.0.0
    └── time-machine v2.14.1
        └── pendulum v3.0.0
tzdata v2024.1
└── pendulum v3.0.0

----- stderr -----
```

## No dedupe

<!-- from pip_tree.rs::no_dedupe -->

Show all dependency paths without deduplication.

```toml title="requirements.txt" snapshot=true
pendulum
time-machine
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + pendulum==3.0.0
 + python-dateutil==2.9.0.post0
 + six==1.16.0
 + time-machine==2.14.1
 + tzdata==2024.1
```

```console
$ uv pip tree --no-dedupe
success: true
exit_code: 0
----- stdout -----
pendulum v3.0.0
├── python-dateutil v2.9.0.post0
│   └── six v1.16.0
├── time-machine v2.14.1
│   └── python-dateutil v2.9.0.post0
│       └── six v1.16.0
└── tzdata v2024.1

----- stderr -----
```

## With editable dependency

<!-- from pip_tree.rs::with_editable -->

Show tree with editable package and git dependency.

```console
$ uv pip install -e [WORKSPACE]/test/packages/hatchling_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + hatchling-editable==0.1.0 (from file://[WORKSPACE]/test/packages/hatchling_editable)
 + iniconfig==2.0.1.dev6+g9cae431 (from git+https://github.com/pytest-dev/iniconfig@9cae43103df70bac6fde7b9f35ad11a9f1be0cb4)
```

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
hatchling-editable v0.1.0
└── iniconfig v2.0.1.dev6+g9cae431

----- stderr -----
```

## Package filter

<!-- from pip_tree.rs::package_flag -->

Filter tree to specific packages with --package.

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

Show tree for werkzeug only:

```console
$ uv pip tree --package werkzeug
success: true
exit_code: 0
----- stdout -----
werkzeug v3.0.1
└── markupsafe v2.1.5

----- stderr -----
```

Show tree for multiple specific packages:

```console
$ uv pip tree --package werkzeug --package jinja2
success: true
exit_code: 0
----- stdout -----
jinja2 v3.1.3
└── markupsafe v2.1.5
werkzeug v3.0.1
└── markupsafe v2.1.5

----- stderr -----
```

## Show version specifiers

<!-- from pip_tree.rs::show_version_specifiers_simple -->

Display version requirements with --show-version-specifiers.

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

```console
$ uv pip tree --show-version-specifiers
success: true
exit_code: 0
----- stdout -----
requests v2.31.0
├── certifi v2024.2.2 [required: >=2017.4.17]
├── charset-normalizer v3.3.2 [required: >=2, <4]
├── idna v3.6 [required: >=2.5, <4]
└── urllib3 v2.2.1 [required: >=1.21.1, <3]

----- stderr -----
```

## Show version specifiers with invert

<!-- from pip_tree.rs::show_version_specifiers_with_invert -->

Combine version specifiers with inverted tree.

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

```console
$ uv pip tree --show-version-specifiers --invert
success: true
exit_code: 0
----- stdout -----
blinker v1.7.0
└── flask v3.0.2 [requires: blinker >=1.6.2]
click v8.1.7
└── flask v3.0.2 [requires: click >=8.1.3]
itsdangerous v2.1.2
└── flask v3.0.2 [requires: itsdangerous >=2.1.2]
markupsafe v2.1.5
├── jinja2 v3.1.3 [requires: markupsafe >=2.0]
│   └── flask v3.0.2 [requires: jinja2 >=3.1.2]
└── werkzeug v3.0.1 [requires: markupsafe >=2.1.1]
    └── flask v3.0.2 [requires: werkzeug >=3.0.0]

----- stderr -----
```

## Show version specifiers with package filter

<!-- from pip_tree.rs::show_version_specifiers_with_package -->

Combine version specifiers with package filtering.

```toml title="requirements.txt" snapshot=true
flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

```console
$ uv pip tree --show-version-specifiers --package werkzeug
success: true
exit_code: 0
----- stdout -----
werkzeug v3.0.1
└── markupsafe v2.1.5 [required: >=2.1.1]

----- stderr -----
```

## Quiet flag ignored

<!-- from pip_tree.rs::print_output_even_with_quite_flag -->

The --quiet flag is ignored for pip tree (tree output is primary purpose).

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

```console
$ uv pip tree --quiet
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Outdated packages

<!-- from pip_tree.rs::outdated -->

Show which packages have newer versions available with --outdated.

```toml title="requirements.txt" snapshot=true
flask==2.0.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 6 packages in [TIME]
Installed 6 packages in [TIME]
 + click==8.1.7
 + flask==2.0.0
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

```console
$ uv pip tree --outdated
success: true
exit_code: 0
----- stdout -----
flask v2.0.0 (latest: v3.0.2)
├── click v8.1.7
├── itsdangerous v2.1.2
├── jinja2 v3.1.3
│   └── markupsafe v2.1.5
└── werkzeug v3.0.1
    └── markupsafe v2.1.5

----- stderr -----
```

## No duplicate dependencies with markers

<!-- from pip_tree.rs::no_duplicate_dependencies_with_markers -->

Dependencies with multiple marker-specific requirements are only shown once.

```toml title="pyproject.toml" snapshot=true
[project]
name = "debug"
version = "0.1.0"
requires-python = ">=3.12.0"
dependencies = [
  "sniffio>=1.0.0; python_version >= '3.11'",
  "sniffio>=1.0.1; python_version >= '3.12'",
  "sniffio>=1.0.2; python_version >= '3.13'",
]

[build-system]
requires = ["uv_build>=0.8.22,<10000"]
build-backend = "uv_build"
```

```toml title="src/debug/__init__.py" snapshot=true

```

```console
$ uv pip install . --strict
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stderr]]
filters = [
  { regex = "Resolved \\d+ packages", replacement = "Resolved [N] packages" },
  { regex = "Prepared \\d+ packages", replacement = "Prepared [N] packages" },
  { regex = "Installed \\d+ packages", replacement = "Installed [N] packages" }
]
```

```toml title="uv.stderr" snapshot=true
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + debug==0.1.0 (from file://[TEMP_DIR])
 + sniffio==1.3.1
```

Only one sniffio dependency shown despite multiple marker requirements:

```console
$ uv pip tree
success: true
exit_code: 0
----- stdout -----
debug v0.1.0
└── sniffio v1.3.1

----- stderr -----
```

Show version specifier (merged from all markers):

```console
$ uv pip tree --show-version-specifiers
success: true
exit_code: 0
----- stdout -----
debug v0.1.0
└── sniffio v1.3.1 [required: >=1.0.1]

----- stderr -----
```
