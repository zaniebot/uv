# Sync Groups

Tests for `uv sync` with dependency groups.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Dev dependencies

### Sync with dev dependencies

<!-- from sync.rs::sync_dev -->

Tests for `--only-dev`, `--no-dev`, and default dev group behavior.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
dev = ["anyio"]
```

```console
$ uv lock
success: true
exit_code: 0
```

Sync with only dev dependencies.

```console
$ uv sync --only-dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

Sync without dev dependencies.

```console
$ uv sync --no-dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 1 package in [TIME]
Uninstalled 3 packages in [TIME]
Installed 1 package in [TIME]
 - anyio==4.3.0
 - idna==3.6
 - sniffio==1.3.1
 + typing-extensions==4.10.0
```

## Dependency groups

### Sync with groups

<!-- from sync.rs::sync_group -->

Tests for `--group`, `--only-group`, and `--all-groups` flags.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
dev = ["iniconfig"]
foo = ["anyio"]
bar = ["requests"]
```

```console
$ uv lock
success: true
exit_code: 0
```

Default sync installs dev group.

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + iniconfig==2.0.0
 + typing-extensions==4.10.0
```

Sync with additional group.

```console
$ uv sync --group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

Sync with only a specific group.

```console
$ uv sync --only-group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Prepared 4 packages in [TIME]
Uninstalled 4 packages in [TIME]
Installed 4 packages in [TIME]
 - anyio==4.3.0
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 - iniconfig==2.0.0
 + requests==2.31.0
 - sniffio==1.3.1
 - typing-extensions==4.10.0
 + urllib3==2.2.1
```

Sync with multiple groups.

```console
$ uv sync --group foo --group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + iniconfig==2.0.0
 + sniffio==1.3.1
 + typing-extensions==4.10.0
```

Sync all groups.

```console
$ uv sync --all-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Audited 9 packages in [TIME]
```

Exclude a group with `--no-group`.

```console
$ uv sync --all-groups --no-group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Uninstalled 4 packages in [TIME]
 - certifi==2024.2.2
 - charset-normalizer==3.3.2
 - requests==2.31.0
 - urllib3==2.2.1
```

Combine `--all-groups` with `--no-dev`.

```console
$ uv sync --all-groups --no-dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Uninstalled 1 package in [TIME]
Installed 4 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 - iniconfig==2.0.0
 + requests==2.31.0
 + urllib3==2.2.1
```

Sync with `--dev` flag.

```console
$ uv sync --dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Uninstalled 7 packages in [TIME]
Installed 1 package in [TIME]
 - anyio==4.3.0
 - certifi==2024.2.2
 - charset-normalizer==3.3.2
 - idna==3.6
 + iniconfig==2.0.0
 - requests==2.31.0
 - sniffio==1.3.1
 - urllib3==2.2.1
```

Use `--dev` with `--no-group dev` to exclude dev group.

```console
$ uv sync --dev --no-group dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Uninstalled 1 package in [TIME]
 - iniconfig==2.0.0
```

Use `--group dev` with `--no-dev` to include dev group explicitly.

```console
$ uv sync --group dev --no-dev
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Audited 1 package in [TIME]
```

### No default groups

<!-- from sync.rs::sync_group -->

The `--no-default-groups` flag excludes all default groups.

```console
$ uv sync --all-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Installed 8 packages in [TIME]
 + anyio==4.3.0
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + iniconfig==2.0.0
 + requests==2.31.0
 + sniffio==1.3.1
 + urllib3==2.2.1
```

```console
$ uv sync --no-default-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Uninstalled 8 packages in [TIME]
 - anyio==4.3.0
 - certifi==2024.2.2
 - charset-normalizer==3.3.2
 - idna==3.6
 - iniconfig==2.0.0
 - requests==2.31.0
 - sniffio==1.3.1
 - urllib3==2.2.1
```

Reinstall all groups to set up for next test.

```console
$ uv sync --all-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Installed 8 packages in [TIME]
 + anyio==4.3.0
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + iniconfig==2.0.0
 + requests==2.31.0
 + sniffio==1.3.1
 + urllib3==2.2.1
```

Use `--no-default-groups` with specific groups to include only those.

```console
$ uv sync --no-default-groups --group foo --group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Uninstalled 1 package in [TIME]
 - iniconfig==2.0.0
```

## Include group

### Group with include-group

<!-- from sync.rs::sync_include_group -->

Tests for groups that include other groups.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio", {include-group = "bar"}]
bar = ["iniconfig"]
```

```console
$ uv lock
success: true
exit_code: 0
```

Default sync only installs project dependencies.

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + typing-extensions==4.10.0
```

Sync with group foo also installs bar (via include-group).

```console
$ uv sync --group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + iniconfig==2.0.0
 + sniffio==1.3.1
```

Only bar group.

```console
$ uv sync --only-group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Uninstalled 4 packages in [TIME]
 - anyio==4.3.0
 - idna==3.6
 - sniffio==1.3.1
 - typing-extensions==4.10.0
```

Multiple groups together.

```console
$ uv sync --group foo --group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
 + typing-extensions==4.10.0
```

Only group foo excludes project dependencies but includes bar.

```console
$ uv sync --only-group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Uninstalled 1 package in [TIME]
 - typing-extensions==4.10.0
```

All groups.

```console
$ uv sync --all-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Installed 1 package in [TIME]
 + typing-extensions==4.10.0
```

No default groups excludes foo and bar.

```console
$ uv sync --no-default-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Uninstalled 4 packages in [TIME]
 - anyio==4.3.0
 - idna==3.6
 - iniconfig==2.0.0
 - sniffio==1.3.1
```

Reinstall all groups.

```console
$ uv sync --all-groups
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + iniconfig==2.0.0
 + sniffio==1.3.1
```

No default groups with explicit foo includes bar.

```console
$ uv sync --no-default-groups --group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Audited 5 packages in [TIME]
```

## Exclude group

### Exclude group with --no-group

<!-- from sync.rs::sync_exclude_group -->

Tests for excluding groups with `--no-group`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = ["anyio", {include-group = "bar"}]
bar = ["iniconfig"]
```

```console
$ uv lock
success: true
exit_code: 0
```

Sync group foo.

```console
$ uv sync --group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + iniconfig==2.0.0
 + sniffio==1.3.1
 + typing-extensions==4.10.0
```

Group with its own exclusion removes that group.

```console
$ uv sync --group foo --no-group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Uninstalled 4 packages in [TIME]
 - anyio==4.3.0
 - idna==3.6
 - iniconfig==2.0.0
 - sniffio==1.3.1
```

Only bar group.

```console
$ uv sync --only-group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
 - typing-extensions==4.10.0
```

Only bar with no-group bar excludes everything.

```console
$ uv sync --only-group bar --no-group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Uninstalled 1 package in [TIME]
 - iniconfig==2.0.0
```

## Non-existent group

### Error on non-existent group

<!-- from sync.rs::sync_non_existent_group -->

Requesting a non-existent group should fail.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
foo = []
bar = ["requests"]
```

```console
$ uv lock
success: true
exit_code: 0
```

```console
$ uv sync --group baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
error: Group `baz` is not defined in the project's `dependency-groups` table
```

```console
$ uv sync --no-group baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
error: Group `baz` is not defined in the project's `dependency-groups` table
```

### Empty group succeeds

<!-- from sync.rs::sync_non_existent_group -->

Requesting an empty group should succeed.

```console
$ uv sync --group foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + typing-extensions==4.10.0
```

### Frozen uses lockfile groups

<!-- from sync.rs::sync_non_existent_group -->

With `--frozen`, groups from the lockfile are used.

```console
$ uv sync --frozen --group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

Modify pyproject.toml to replace bar with baz.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions"]

[dependency-groups]
baz = ["iniconfig"]
```

With `--frozen`, bar still works from lockfile.

```console
$ uv sync --frozen --group bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 6 packages in [TIME]
```

With `--frozen`, baz fails because it's not in the lockfile.

```console
$ uv sync --frozen --group baz
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Group `baz` is not defined in the project's `dependency-groups` table
```
