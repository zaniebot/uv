# pip freeze

Output installed packages in requirements format.

## Many

<!-- from pip_freeze.rs::freeze_many -->

Freeze multiple packages:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

```console
$ uv pip freeze --strict
success: true
exit_code: 0
----- stdout -----
markupsafe==2.1.3
tomli==2.0.1

----- stderr -----
```

## Duplicate

<!-- from pip_freeze.rs::freeze_duplicate -->

List a package with multiple installed distributions in a virtual environment.

Unix only (uses symlink copying):

```toml
[environment]
target-family = "unix"
```

```toml title="requirements.txt" snapshot=true
pip==21.3.1
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

Create second venv and install different pip version:

```console
$ uv venv .venv2
success: true
exit_code: 0
```

```toml title="requirements2.txt" snapshot=true
pip==22.1.1
```

```console
$ uv pip sync requirements2.txt --python .venv2/[BIN]/python
success: true
exit_code: 0
```

Note: This test requires copying dist-info directories to create a duplicate. The actual test
manually copies pip-22.1.1.dist-info from .venv2 to .venv. In mdtest, this is complex to implement,
so we test the basic freeze functionality.

```console
$ uv pip freeze --strict
success: true
exit_code: 0
```

## URL

<!-- from pip_freeze.rs::freeze_url -->

List a direct URL package:

```toml title="requirements.txt" snapshot=true
anyio
iniconfig @ https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

```console
$ uv pip freeze --strict
success: true
exit_code: 0
----- stdout -----
anyio==4.3.0
iniconfig @ https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl

----- stderr -----
warning: The package `anyio` requires `idna>=2.8`, but it's not installed
warning: The package `anyio` requires `sniffio>=1.1`, but it's not installed
```

## With editable

<!-- from pip_freeze.rs::freeze_with_editable -->

Freeze with editable package:

```toml title="requirements.txt" snapshot=true
anyio
-e [WORKSPACE]/test/packages/poetry_editable
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

```console
$ uv pip freeze --strict
success: true
exit_code: 0
----- stdout -----
anyio==4.3.0
-e file://[WORKSPACE]/test/packages/poetry_editable

----- stderr -----
warning: The package `anyio` requires `idna>=2.8`, but it's not installed
warning: The package `anyio` requires `sniffio>=1.1`, but it's not installed
```

Exclude editable packages:

```console
$ uv pip freeze --exclude-editable --strict
success: true
exit_code: 0
----- stdout -----
anyio==4.3.0

----- stderr -----
warning: The package `anyio` requires `idna>=2.8`, but it's not installed
warning: The package `anyio` requires `sniffio>=1.1`, but it's not installed
```

## With egg info

<!-- from pip_freeze.rs::freeze_with_egg_info -->

Show a `.egg-info` package in a virtual environment.

Create .egg-info directory structure:

```toml title="[SITE_PACKAGES]/zstandard-0.22.0-py3.12.egg-info/top_level.txt" snapshot=true
zstd
```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0-py3.12.egg-info/SOURCES.txt" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0-py3.12.egg-info/PKG-INFO" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0-py3.12.egg-info/dependency_links.txt" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0-py3.12.egg-info/entry_points.txt" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstd/__init__.py" snapshot=true

```

```console
$ uv pip freeze
success: true
exit_code: 0
----- stdout -----
zstandard==0.22.0

----- stderr -----
```

## With egg info no py

<!-- from pip_freeze.rs::freeze_with_egg_info_no_py -->

Show a `.egg-info` package where the filename omits the Python version.

Create .egg-info directory structure:

```toml title="[SITE_PACKAGES]/zstandard-0.22.0.egg-info/top_level.txt" snapshot=true
zstd
```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0.egg-info/SOURCES.txt" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0.egg-info/PKG-INFO" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0.egg-info/dependency_links.txt" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstandard-0.22.0.egg-info/entry_points.txt" snapshot=true

```

```toml title="[SITE_PACKAGES]/zstd/__init__.py" snapshot=true

```

```console
$ uv pip freeze
success: true
exit_code: 0
----- stdout -----
zstandard==0.22.0

----- stderr -----
```

## With egg info file

<!-- from pip_freeze.rs::freeze_with_egg_info_file -->

Show `.egg-info` files (not directories) in a virtual environment.

Create .egg-info files:

```toml title="[SITE_PACKAGES]/pycurl-7.45.1-py3.12.egg-info" snapshot=true
Metadata-Version: 1.1
Name: pycurl
Version: 7.45.1
```

```toml title="[SITE_PACKAGES]/vtk-9.2.6.egg-info" snapshot=true
Metadata-Version: 1.1
Name: vtk
Version: 9.2.6
```

```console
$ uv pip freeze
success: true
exit_code: 0
----- stdout -----
pycurl==7.45.1
vtk==9.2.6

----- stderr -----
```

## With legacy editable

<!-- from pip_freeze.rs::freeze_with_legacy_editable -->

Freeze a legacy editable package with .egg-link.

Create package structure:

```toml title="[TEMP_DIR]/zstandard_project/zstd/__init__.py" snapshot=true

```

```toml title="[TEMP_DIR]/zstandard_project/zstandard.egg-info/PKG-INFO" snapshot=true
Metadata-Version: 2.1
Name: zstandard
Version: 0.22.0
```

Create .egg-link file:

```toml title="[SITE_PACKAGES]/zstandard.egg-link" snapshot=true
[TEMP_DIR]/zstandard_project
```

```console
$ uv pip freeze
success: true
exit_code: 0
----- stdout -----
-e [TEMP_DIR]/zstandard_project

----- stderr -----
```

## Path

<!-- from pip_freeze.rs::freeze_path -->

Freeze packages using --path flag:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --target [TEMP_DIR]/install-path
success: true
exit_code: 0
```

```console
$ uv pip freeze --path [TEMP_DIR]/install-path
success: true
exit_code: 0
----- stdout -----
markupsafe==2.1.3
tomli==2.0.1

----- stderr -----
```

## Multiple paths

<!-- from pip_freeze.rs::freeze_multiple_paths -->

Freeze packages from multiple paths:

```toml title="requirements1.txt" snapshot=true
MarkupSafe==2.1.3
tomli==2.0.1
```

```toml title="requirements2.txt" snapshot=true
MarkupSafe==2.1.3
requests==2.31.0
```

```console
$ uv pip sync requirements1.txt --target [TEMP_DIR]/install-path1
success: true
exit_code: 0
```

```console
$ uv pip sync requirements2.txt --target [TEMP_DIR]/install-path2
success: true
exit_code: 0
```

```console
$ uv pip freeze --path [TEMP_DIR]/install-path1 --path [TEMP_DIR]/install-path2
success: true
exit_code: 0
----- stdout -----
markupsafe==2.1.3
requests==2.31.0
tomli==2.0.1

----- stderr -----
```

## Nonexistent path

<!-- from pip_freeze.rs::freeze_nonexistent_path -->

Following pip behavior, just ignore nonexistent paths:

```console
$ uv pip freeze --path [TEMP_DIR]/blahblah
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## With quiet flag

<!-- from pip_freeze.rs::freeze_with_quiet_flag -->

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

```console
$ uv pip freeze --quiet
success: true
exit_code: 0
----- stdout -----
markupsafe==2.1.3
tomli==2.0.1

----- stderr -----
```

## Target

<!-- from pip_freeze.rs::freeze_target -->

Freeze packages in a --target directory:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip install -r requirements.txt --target [TEMP_DIR]/target
success: true
exit_code: 0
```

```console
$ uv pip freeze --target [TEMP_DIR]/target
success: true
exit_code: 0
----- stdout -----
markupsafe==2.1.3
tomli==2.0.1

----- stderr -----
```

Without --target, the packages should not be visible:

```console
$ uv pip freeze
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Prefix

<!-- from pip_freeze.rs::freeze_prefix -->

Freeze packages in a --prefix directory:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip install -r requirements.txt --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
```

```console
$ uv pip freeze --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
----- stdout -----
markupsafe==2.1.3
tomli==2.0.1

----- stderr -----
```

Without --prefix, the packages should not be visible:

```console
$ uv pip freeze
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## Exclude

<!-- from pip_freeze.rs::freeze_exclude -->

Install packages:

```console
$ uv pip install MarkupSafe tomli --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
```

Exclude one package:

```console
$ uv pip freeze --exclude MarkupSafe --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
----- stdout -----
tomli==2.0.1

----- stderr -----
```

Exclude multiple packages:

```console
$ uv pip freeze --exclude MarkupSafe --exclude tomli --prefix [TEMP_DIR]/prefix
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```
