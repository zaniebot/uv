# pip show

Display package information including name, version, location, requirements, and reverse
dependencies.

## Empty

<!-- from pip_show.rs::show_empty -->

Error when no package name is provided:

```console
$ uv pip show
success: false
exit_code: 1
----- stdout -----

----- stderr -----
warning: Please provide a package name or names.
```

## Requires multiple

<!-- from pip_show.rs::show_requires_multiple -->

Show package with multiple requirements.

Create requirements file:

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

Install:

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

Show package information:

```console
$ uv pip show requests
success: true
exit_code: 0
----- stdout -----
Name: requests
Version: 2.31.0
Location: [SITE_PACKAGES]/
Requires: certifi, charset-normalizer, idna, urllib3
Required-by:

----- stderr -----
```

## Python version marker

<!-- from pip_show.rs::show_python_version_marker -->

`click` v8.1.7 requires `importlib-metadata`, but only when `python_version < "3.8"`. This test
asserts that the Python version marker in the metadata is correctly evaluated.

Create requirements file:

```toml title="requirements.txt" snapshot=true
click==8.1.7
```

Install:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + click==8.1.7
```

Show package information (no requirements shown for Python 3.12):

```console
$ uv pip show click
success: true
exit_code: 0
```

```toml title="uv.filter" snapshot=true
[[assert.stdout]]
# On Windows, click requires colorama
filters = [
  { name = "windows-colorama", regex = "Requires: colorama", replacement = "Requires:" }
]
```

```toml title="uv.stdout" snapshot=true
Name: click
Version: 8.1.7
Location: [SITE_PACKAGES]/
Requires:
Required-by:

```

## Found single package

<!-- from pip_show.rs::show_found_single_package -->

Create requirements file:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
```

Install:

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

Show package:

```console
$ uv pip show markupsafe
success: true
exit_code: 0
----- stdout -----
Name: markupsafe
Version: 2.1.3
Location: [SITE_PACKAGES]/
Requires:
Required-by:

----- stderr -----
```

## Found multiple packages

<!-- from pip_show.rs::show_found_multiple_packages -->

Create requirements file:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
pip==21.3.1
```

Install:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + markupsafe==2.1.3
 + pip==21.3.1
```

Show multiple packages:

```console
$ uv pip show markupsafe pip
success: true
exit_code: 0
----- stdout -----
Name: markupsafe
Version: 2.1.3
Location: [SITE_PACKAGES]/
Requires:
Required-by:
---
Name: pip
Version: 21.3.1
Location: [SITE_PACKAGES]/
Requires:
Required-by:

----- stderr -----
```

## Found one out of three

<!-- from pip_show.rs::show_found_one_out_of_three -->

Create requirements file:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
pip==21.3.1
```

Install:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + markupsafe==2.1.3
 + pip==21.3.1
```

Show three packages, only one is installed:

```console
$ uv pip show markupsafe flask django
success: true
exit_code: 0
----- stdout -----
Name: markupsafe
Version: 2.1.3
Location: [SITE_PACKAGES]/
Requires:
Required-by:

----- stderr -----
warning: Package(s) not found for: django, flask
```

## Found one out of two quiet

<!-- from pip_show.rs::show_found_one_out_of_two_quiet -->

Flask isn't installed, but markupsafe is, so the command should succeed with --quiet.

Create requirements file:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
pip==21.3.1
```

Install:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + markupsafe==2.1.3
 + pip==21.3.1
```

Show with --quiet (succeeds even though flask isn't found):

```console
$ uv pip show markupsafe flask --quiet
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Empty quiet

<!-- from pip_show.rs::show_empty_quiet -->

Flask isn't installed, so the command should fail even with --quiet.

Create requirements file:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
pip==21.3.1
```

Install:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + markupsafe==2.1.3
 + pip==21.3.1
```

Show non-existent package with --quiet:

```console
$ uv pip show flask --quiet
success: false
exit_code: 1
----- stdout -----

----- stderr -----
```

## Editable

<!-- from pip_show.rs::show_editable -->

Show editable package installation.

Install editable package:

```console
$ uv pip install -e ../../test/packages/poetry_editable
success: true
exit_code: 0
env:
  CARGO_TARGET_DIR: ../../../target/target_install_editable
```

Show editable package:

```console
$ uv pip show poetry-editable
success: true
exit_code: 0
----- stdout -----
Name: poetry-editable
Version: 0.1.0
Location: [SITE_PACKAGES]/
Editable project location: [WORKSPACE]/test/packages/poetry_editable
Requires: anyio
Required-by:

----- stderr -----
```

## Required by multiple

<!-- from pip_show.rs::show_required_by_multiple -->

Show package that is required by multiple other packages.

Create requirements file:

```toml title="requirements.txt" snapshot=true
anyio==4.0.0
requests==2.31.0
```

Install:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + anyio==4.0.0
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + sniffio==1.3.1
 + urllib3==2.2.1
```

Show idna which is required by both anyio and requests:

```console
$ uv pip show idna
success: true
exit_code: 0
----- stdout -----
Name: idna
Version: 3.6
Location: [SITE_PACKAGES]/
Requires:
Required-by: anyio, requests

----- stderr -----
```

## Files

<!-- from pip_show.rs::show_files -->

Show package with --files flag to list all installed files.

Install:

```console
$ uv pip install requests==2.31.0 --strict
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

Show with files list:

```toml
[environment]
target-family = "unix"
```

```console
$ uv pip show requests --files
success: true
exit_code: 0
----- stdout -----
Name: requests
Version: 2.31.0
Location: [SITE_PACKAGES]/
Requires: certifi, charset-normalizer, idna, urllib3
Required-by:
Files:
  requests-2.31.0.dist-info/INSTALLER
  requests-2.31.0.dist-info/LICENSE
  requests-2.31.0.dist-info/METADATA
  requests-2.31.0.dist-info/RECORD
  requests-2.31.0.dist-info/REQUESTED
  requests-2.31.0.dist-info/WHEEL
  requests-2.31.0.dist-info/top_level.txt
  requests/__init__.py
  requests/__version__.py
  requests/_internal_utils.py
  requests/adapters.py
  requests/api.py
  requests/auth.py
  requests/certs.py
  requests/compat.py
  requests/cookies.py
  requests/exceptions.py
  requests/help.py
  requests/hooks.py
  requests/models.py
  requests/packages.py
  requests/sessions.py
  requests/status_codes.py
  requests/structures.py
  requests/utils.py

----- stderr -----
```

## Target

<!-- from pip_show.rs::show_target -->

Show package installed in a --target directory.

Create requirements file:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
```

Install to target directory:

```console
$ uv pip install -r requirements.txt --target [TEMP_DIR]/target
success: true
exit_code: 0
```

Show package in target directory:

```console
$ uv pip show markupsafe --target [TEMP_DIR]/target
success: true
exit_code: 0
----- stdout -----
Name: markupsafe
Version: 2.1.3
Location: [TEMP_DIR]/target
Requires:
Required-by:

----- stderr -----
```

Without --target, package should not be found:

```console
$ uv pip show markupsafe
success: false
exit_code: 1
----- stdout -----

----- stderr -----
warning: Package(s) not found for: markupsafe
```

## Prefix

<!-- from pip_show.rs::show_prefix -->

Show package installed in a --prefix directory.

Create requirements file:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
```

Install to prefix directory:

```console
$ uv pip install -r requirements.txt --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
```

Show package in prefix directory:

```console
$ uv pip show markupsafe --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
----- stdout -----
Name: markupsafe
Version: 2.1.3
Location: [TEMP_DIR]/prefix/[PYTHON-LIB]/site-packages
Requires:
Required-by:

----- stderr -----
```

Without --prefix, package should not be found:

```console
$ uv pip show markupsafe
success: false
exit_code: 1
----- stdout -----

----- stderr -----
warning: Package(s) not found for: markupsafe
```
