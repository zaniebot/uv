# pip list

List installed packages in the environment.

## Empty columns

<!-- from pip_list.rs::list_empty_columns -->

```console
$ uv pip list --format columns
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Empty freeze

<!-- from pip_list.rs::list_empty_freeze -->

```console
$ uv pip list --format freeze
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Empty json

<!-- from pip_list.rs::list_empty_json -->

```console
$ uv pip list --format json
success: true
exit_code: 0
----- stdout -----
[]

----- stderr -----
```

## Single no editable

<!-- from pip_list.rs::list_single_no_editable -->

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

```console
$ uv pip list
success: true
exit_code: 0
----- stdout -----
Package    Version
---------- -------
markupsafe 2.1.3

----- stderr -----
```

## Outdated columns

<!-- from pip_list.rs::list_outdated_columns -->

```toml
# file: requirements.txt
anyio==3.0.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.0.0
 + idna==3.6
 + sniffio==1.3.1
```

```console
$ uv pip list --outdated
success: true
exit_code: 0
----- stdout -----
Package Version Latest Type
------- ------- ------ -----
anyio   3.0.0   4.3.0  wheel

----- stderr -----
```

## Outdated json

<!-- from pip_list.rs::list_outdated_json -->

```toml
# file: requirements.txt
anyio==3.0.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.0.0
 + idna==3.6
 + sniffio==1.3.1
```

```console
$ uv pip list --outdated --format json
success: true
exit_code: 0
----- stdout -----
[{"name":"anyio","version":"3.0.0","latest_version":"4.3.0","latest_filetype":"wheel"}]

----- stderr -----
```

## Outdated freeze

<!-- from pip_list.rs::list_outdated_freeze -->

Error when using --outdated with freeze format:

```console
$ uv pip list --outdated --format freeze
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--outdated` cannot be used with `--format freeze`
```

## Outdated git

<!-- from pip_list.rs::list_outdated_git -->

Git dependencies should not appear in outdated list:

```toml
# file: requirements.txt
iniconfig==1.0.0
uv-public-pypackage @ git+https://github.com/astral-test/uv-public-pypackage@0.0.1
```

```toml
[test]
required-features = ["git"]
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + iniconfig==1.0.0
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

```console
$ uv pip list --outdated
success: true
exit_code: 0
----- stdout -----
Package   Version Latest Type
--------- ------- ------ -----
iniconfig 1.0.0   2.0.0  wheel

----- stderr -----
```

## Outdated index

<!-- from pip_list.rs::list_outdated_index -->

Check outdated packages against a different index:

```toml
# file: requirements.txt
anyio==3.0.0
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.0.0
 + idna==3.6
 + sniffio==1.3.1
```

```console
$ uv pip list --outdated --index-url https://test.pypi.org/simple
success: true
exit_code: 0
----- stdout -----
Package Version Latest Type
------- ------- ------ -----
anyio   3.0.0   3.5.0  wheel

----- stderr -----
```

## Editable

<!-- from pip_list.rs::list_editable -->

List with editable package:

```console
$ uv pip install -e [WORKSPACE]/test/packages/poetry_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + poetry-editable==0.1.0 (from file://[WORKSPACE]/test/packages/poetry_editable)
 + sniffio==1.3.1
```

```console
$ uv pip list
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
filters = [
  { regex = "-+.*", replacement = "[UNDERLINE]" },
  { regex = "  +", replacement = " " }
]
```

```toml title="uv.stdout" snapshot=true
Package Version Editable project location
[UNDERLINE]
anyio 4.3.0
idna 3.6
poetry-editable 0.1.0 [WORKSPACE]/test/packages/poetry_editable
sniffio 1.3.1

```

## Editable only

<!-- from pip_list.rs::list_editable_only -->

Install editable package:

```console
$ uv pip install -e [WORKSPACE]/test/packages/poetry_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + poetry-editable==0.1.0 (from file://[WORKSPACE]/test/packages/poetry_editable)
 + sniffio==1.3.1
```

List only editable packages:

```console
$ uv pip list --editable
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
filters = [
  { regex = "-+.*", replacement = "[UNDERLINE]" },
  { regex = "  +", replacement = " " }
]
```

```toml title="uv.stdout" snapshot=true
Package Version Editable project location
[UNDERLINE]
poetry-editable 0.1.0 [WORKSPACE]/test/packages/poetry_editable

```

Exclude editable packages:

```console
$ uv pip list --exclude-editable
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
filters = [
  { regex = "-+.*", replacement = "[UNDERLINE]" },
  { regex = "  +", replacement = " " }
]
```

```toml title="uv.stdout" snapshot=true
Package Version
[UNDERLINE]
anyio 4.3.0
idna 3.6
sniffio 1.3.1

```

Cannot use both --editable and --exclude-editable:

```console
$ uv pip list --editable --exclude-editable
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--editable' cannot be used with '--exclude-editable'

Usage: uv pip list --cache-dir [CACHE_DIR] --editable --exclude-newer <EXCLUDE_NEWER>

For more information, try '--help'.
```

## Exclude

<!-- from pip_list.rs::list_exclude -->

Install editable package:

```console
$ uv pip install -e [WORKSPACE]/test/packages/poetry_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + poetry-editable==0.1.0 (from file://[WORKSPACE]/test/packages/poetry_editable)
 + sniffio==1.3.1
```

Exclude non-existent package:

```console
$ uv pip list --exclude numpy
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
filters = [
  { regex = "-+.*", replacement = "[UNDERLINE]" },
  { regex = "  +", replacement = " " }
]
```

```toml title="uv.stdout" snapshot=true
Package Version Editable project location
[UNDERLINE]
anyio 4.3.0
idna 3.6
poetry-editable 0.1.0 [WORKSPACE]/test/packages/poetry_editable
sniffio 1.3.1

```

Exclude editable package:

```console
$ uv pip list --exclude poetry-editable
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
filters = [
  { regex = "-+.*", replacement = "[UNDERLINE]" },
  { regex = "  +", replacement = " " }
]
```

```toml title="uv.stdout" snapshot=true
Package Version
[UNDERLINE]
anyio 4.3.0
idna 3.6
sniffio 1.3.1

```

Exclude multiple packages:

```console
$ uv pip list --exclude numpy --exclude poetry-editable
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
filters = [
  { regex = "-+.*", replacement = "[UNDERLINE]" },
  { regex = "  +", replacement = " " }
]
```

```toml title="uv.stdout" snapshot=true
Package Version
[UNDERLINE]
anyio 4.3.0
idna 3.6
sniffio 1.3.1

```

## Format json

<!-- from pip_list.rs::list_format_json -->

Unix only (path normalization):

```toml
[environment]
target-family = "unix"
```

Install editable package:

```console
$ uv pip install -e [WORKSPACE]/test/packages/poetry_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + poetry-editable==0.1.0 (from file://[WORKSPACE]/test/packages/poetry_editable)
 + sniffio==1.3.1
```

```console
$ uv pip list --format=json
success: true
exit_code: 0
----- stdout -----
[{"name":"anyio","version":"4.3.0"},{"name":"idna","version":"3.6"},{"name":"poetry-editable","version":"0.1.0","editable_project_location":"[WORKSPACE]/test/packages/poetry_editable"},{"name":"sniffio","version":"1.3.1"}]

----- stderr -----
```

List only editable in JSON:

```console
$ uv pip list --format=json --editable
success: true
exit_code: 0
----- stdout -----
[{"name":"poetry-editable","version":"0.1.0","editable_project_location":"[WORKSPACE]/test/packages/poetry_editable"}]

----- stderr -----
```

Exclude editable in JSON:

```console
$ uv pip list --format=json --exclude-editable
success: true
exit_code: 0
----- stdout -----
[{"name":"anyio","version":"4.3.0"},{"name":"idna","version":"3.6"},{"name":"sniffio","version":"1.3.1"}]

----- stderr -----
```

## Format freeze

<!-- from pip_list.rs::list_format_freeze -->

Install editable package:

```console
$ uv pip install -e [WORKSPACE]/test/packages/poetry_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + poetry-editable==0.1.0 (from file://[WORKSPACE]/test/packages/poetry_editable)
 + sniffio==1.3.1
```

```console
$ uv pip list --format=freeze
success: true
exit_code: 0
----- stdout -----
anyio==4.3.0
idna==3.6
poetry-editable==0.1.0
sniffio==1.3.1

----- stderr -----
```

List only editable in freeze format:

```console
$ uv pip list --format=freeze --editable
success: true
exit_code: 0
----- stdout -----
poetry-editable==0.1.0

----- stderr -----
```

Exclude editable in freeze format:

```console
$ uv pip list --format=freeze --exclude-editable
success: true
exit_code: 0
----- stdout -----
anyio==4.3.0
idna==3.6
sniffio==1.3.1

----- stderr -----
```

## Legacy editable

<!-- from pip_list.rs::list_legacy_editable -->

Create legacy .egg-link editable:

```toml title="[TEMP_DIR]/zstandard_project/zstd/__init__.py" snapshot=true

```

```toml title="[TEMP_DIR]/zstandard_project/zstandard.egg-info/PKG-INFO" snapshot=true
Metadata-Version: 2.1
Name: zstandard
Version: 0.22.0
```

```toml title="[SITE_PACKAGES]/zstandard.egg-link" snapshot=true
[TEMP_DIR]/zstandard_project
```

```toml title="[SITE_PACKAGES]/easy-install.pth" snapshot=true
something
[TEMP_DIR]/zstandard_project
another thing
```

```console
$ uv pip list --editable
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
filters = [
  { regex = "-+.*", replacement = "[UNDERLINE]" },
  { regex = "  +", replacement = " " }
]
```

```toml title="uv.stdout" snapshot=true
Package Version Editable project location
[UNDERLINE]
zstandard 0.22.0 [TEMP_DIR]/zstandard_project

```

## Legacy editable invalid version

<!-- from pip_list.rs::list_legacy_editable_invalid_version -->

Create legacy .egg-link with invalid version:

```toml title="[TEMP_DIR]/paramiko_project/paramiko.egg-info/PKG-INFO" snapshot=true
Metadata-Version: 1.0
Name: paramiko
Version: 0.1-bulbasaur
```

```toml title="[SITE_PACKAGES]/paramiko.egg-link" snapshot=true
[TEMP_DIR]/paramiko_project
```

```console
$ uv pip list --editable
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to read metadata from: `[SITE_PACKAGES]/paramiko.egg-link`
 Caused by: after parsing `0.1-b`, found `ulbasaur`, which is not part of a valid version
```

## Ignores quiet flag format freeze

<!-- from pip_list.rs::list_ignores_quiet_flag_format_freeze -->

Install editable package:

```console
$ uv pip install -e [WORKSPACE]/test/packages/poetry_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + poetry-editable==0.1.0 (from file://[WORKSPACE]/test/packages/poetry_editable)
 + sniffio==1.3.1
```

```console
$ uv pip list --format=freeze --quiet
success: true
exit_code: 0
----- stdout -----
anyio==4.3.0
idna==3.6
poetry-editable==0.1.0
sniffio==1.3.1

----- stderr -----
```

```console
$ uv pip list --format=freeze --editable --quiet
success: true
exit_code: 0
----- stdout -----
poetry-editable==0.1.0

----- stderr -----
```

```console
$ uv pip list --format=freeze --exclude-editable --quiet
success: true
exit_code: 0
----- stdout -----
anyio==4.3.0
idna==3.6
sniffio==1.3.1

----- stderr -----
```

## Target

<!-- from pip_list.rs::list_target -->

```toml
# file: requirements.txt
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip install -r requirements.txt --target [TEMP_DIR]/target
success: true
exit_code: 0
```

```console
$ uv pip list --target [TEMP_DIR]/target
success: true
exit_code: 0
----- stdout -----
Package    Version
---------- -------
markupsafe 2.1.3
tomli      2.0.1

----- stderr -----
```

Without --target, packages should not be visible:

```console
$ uv pip list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Prefix

<!-- from pip_list.rs::list_prefix -->

```toml
# file: requirements.txt
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip install -r requirements.txt --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
```

```console
$ uv pip list --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
----- stdout -----
Package    Version
---------- -------
markupsafe 2.1.3
tomli      2.0.1

----- stderr -----
```

Without --prefix, packages should not be visible:

```console
$ uv pip list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```
