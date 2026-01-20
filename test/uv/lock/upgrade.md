# Lock Upgrade

Tests for `uv lock --upgrade` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Upgrade log

### Show upgrade changes

<!-- from lock.rs::lock_upgrade_log -->

The `--upgrade` flag shows what packages changed.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["markupsafe<2", "iniconfig"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

Run with `--upgrade`, no changes expected since requirements are constrained.

```console
$ uv lock --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

Loosen the requirements, drop iniconfig, and add typing-extensions.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["markupsafe", "typing-extensions"]
```

```console
$ uv lock --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Removed iniconfig v2.0.0
Updated markupsafe v1.1.1 -> v2.1.5
Added typing-extensions v4.10.0
```

### Upgrade with multi-version package

<!-- from lock.rs::lock_upgrade_log_multi_version -->

Test upgrading a package that resolved to multiple versions due to markers.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["markupsafe<2 ; sys_platform != 'win32'", "markupsafe==2.0.0 ; sys_platform == 'win32'"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

```console
$ uv lock --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

Loosen the requirement to allow upgrade.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["markupsafe"]
```

```console
$ uv lock --upgrade
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Updated markupsafe v1.1.1, v2.0.0 -> v2.1.5
```

## Preference

### Respect locked versions

<!-- from lock.rs::lock_preference -->

Re-locking respects existing locked versions unless explicitly upgraded.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig<2"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Widen the requirement. Re-lock without upgrade keeps the old version.

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

Check that iniconfig 1.1.1 is kept from the original lock.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==1.1.1
```
