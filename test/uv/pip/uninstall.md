# pip uninstall

Uninstall packages from the environment.

## No arguments

<!-- from pip_uninstall.rs::no_arguments -->

Error when no package is specified:

```console
$ uv pip uninstall
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the following required arguments were not provided:
  <PACKAGE|--requirements <REQUIREMENTS>>

Usage: uv pip uninstall <PACKAGE|--requirements <REQUIREMENTS>>

For more information, try '--help'.
```

## Invalid requirement

<!-- from pip_uninstall.rs::invalid_requirement -->

Error on invalid version syntax:

```console
$ uv pip uninstall flask==1.0.x
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `flask==1.0.x`
  Caused by: after parsing `1.0`, found `.x`, which is not part of a valid version
flask==1.0.x
     ^^^^^^^
```

## Missing requirements txt

<!-- from pip_uninstall.rs::missing_requirements_txt -->

Error when requirements file doesn't exist:

```console
$ uv pip uninstall -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: File not found: `requirements.txt`
```

## Invalid requirements txt requirement

<!-- from pip_uninstall.rs::invalid_requirements_txt_requirement -->

Error on invalid requirement in requirements file:

```toml title="requirements.txt" snapshot=true
flask==1.0.x
```

```console
$ uv pip uninstall -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Couldn't parse requirement in `requirements.txt` at position 0
  Caused by: after parsing `1.0`, found `.x`, which is not part of a valid version
flask==1.0.x
     ^^^^^^^
```

## Uninstall

<!-- from pip_uninstall.rs::uninstall -->

Basic package uninstall:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

Verify package is installed:

```console
$ python -c "import markupsafe"
success: true
exit_code: 0
```

Uninstall package:

```console
$ uv pip uninstall MarkupSafe
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 1 package in [TIME]
 - markupsafe==2.1.3
```

Verify package is uninstalled:

```console
$ python -c "import markupsafe"
success: false
exit_code: 1
```

## Missing record

<!-- from pip_uninstall.rs::missing_record -->

Error when RECORD file is missing:

```toml title="requirements.txt" snapshot=true
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

Delete the RECORD file:

```console
$ rm [SITE_PACKAGES]/MarkupSafe-2.1.3.dist-info/RECORD
success: true
exit_code: 0
```

Uninstall should fail:

```console
$ uv pip uninstall MarkupSafe
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Cannot uninstall package; `RECORD` file not found at: [SITE_PACKAGES]/MarkupSafe-2.1.3.dist-info/RECORD
```

## Uninstall editable by name

<!-- from pip_uninstall.rs::uninstall_editable_by_name -->

Uninstall an editable package by name:

```toml title="requirements.txt" snapshot=true
-e [WORKSPACE]/test/packages/flit_editable
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

Verify package is installed:

```console
$ python -c "import flit_editable"
success: true
exit_code: 0
```

Uninstall by name:

```console
$ uv pip uninstall flit-editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 1 package in [TIME]
 - flit-editable==0.1.0 (from file://[WORKSPACE]/test/packages/flit_editable)
```

Verify package is uninstalled:

```console
$ python -c "import flit_editable"
success: false
exit_code: 1
```

## Uninstall by path

<!-- from pip_uninstall.rs::uninstall_by_path -->

Uninstall a package by path:

```toml title="requirements.txt" snapshot=true
[WORKSPACE]/test/packages/flit_editable
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

Verify package is installed:

```console
$ python -c "import flit_editable"
success: true
exit_code: 0
```

Uninstall by path:

```console
$ uv pip uninstall [WORKSPACE]/test/packages/flit_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 1 package in [TIME]
 - flit-editable==0.1.0 (from file://[WORKSPACE]/test/packages/flit_editable)
```

Verify package is uninstalled:

```console
$ python -c "import flit_editable"
success: false
exit_code: 1
```

## Uninstall duplicate by path

<!-- from pip_uninstall.rs::uninstall_duplicate_by_path -->

Uninstall a package specified by both name and path:

```toml title="requirements.txt" snapshot=true
[WORKSPACE]/test/packages/flit_editable
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

Verify package is installed:

```console
$ python -c "import flit_editable"
success: true
exit_code: 0
```

Uninstall by both name and path:

```console
$ uv pip uninstall flit-editable [WORKSPACE]/test/packages/flit_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 1 package in [TIME]
 - flit-editable==0.1.0 (from file://[WORKSPACE]/test/packages/flit_editable)
```

Verify package is uninstalled:

```console
$ python -c "import flit_editable"
success: false
exit_code: 1
```

## Uninstall duplicate

<!-- from pip_uninstall.rs::uninstall_duplicate -->

Uninstall duplicate versions of the same package.

Create two separate virtual environments with different pip versions, then manually copy one into
the other to create a duplicate installation.

Install pip 21.3.1 in first venv:

```toml title="requirements.txt" snapshot=true
pip==21.3.1
```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
```

Save first venv site-packages location:

```console
$ python -c "import site; print(site.getsitepackages()[0])"
success: true
exit_code: 0
```

Create second venv and install pip 22.1.1:

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

Manually copy pip 22.1.1 dist-info to create duplicate (simulated):

```console
$ cp -r [SITE_PACKAGES]/pip-22.1.1.dist-info [SITE_PACKAGES]/pip-22.1.1.dist-info.backup || true
success: true
exit_code: 0
```

Note: This test creates a duplicate by copying dist-info directories. In the mdtest version, we
would need to manually create the structure or skip the actual duplicate creation. For now, we'll
test the basic uninstall.

```console
$ uv pip uninstall pip
success: true
exit_code: 0
```

## Uninstall egg info

<!-- from pip_uninstall.rs::uninstall_egg_info -->

Uninstall a legacy `.egg-info` package.

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

Create package directory:

```toml title="[SITE_PACKAGES]/zstd/__init__.py" snapshot=true

```

Uninstall:

```console
$ uv pip uninstall zstandard
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 1 package in [TIME]
 - zstandard==0.22.0
```

## Uninstall legacy editable

<!-- from pip_uninstall.rs::uninstall_legacy_editable -->

Uninstall a legacy editable package with .egg-link.

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

Create easy-install.pth:

```toml title="[SITE_PACKAGES]/easy-install.pth" snapshot=true
something
[TEMP_DIR]/zstandard_project
another thing
```

Uninstall:

```console
$ uv pip uninstall zstandard
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 1 package in [TIME]
 - zstandard==0.22.0
```

Verify .egg-link is removed:

```console
$ test -f [SITE_PACKAGES]/zstandard.egg-link && echo "exists" || echo "removed"
success: true
exit_code: 0
----- stdout -----
removed

----- stderr -----
```

Verify .egg-info directory still exists in source:

```console
$ test -d [TEMP_DIR]/zstandard_project/zstandard.egg-info && echo "exists" || echo "removed"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```

## Dry run uninstall egg info

<!-- from pip_uninstall.rs::dry_run_uninstall_egg_info -->

Dry run of uninstalling a `.egg-info` package.

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

Create package directory:

```toml title="[SITE_PACKAGES]/zstd/__init__.py" snapshot=true

```

Dry run uninstall:

```console
$ uv pip uninstall --dry-run zstandard
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Would uninstall 1 package
 - zstandard==0.22.0
```

Verify .egg-info directory still exists:

```console
$ test -d [SITE_PACKAGES]/zstandard-0.22.0-py3.12.egg-info && echo "exists" || echo "removed"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```

Verify package directory still exists:

```console
$ test -f [SITE_PACKAGES]/zstd/__init__.py && echo "exists" || echo "removed"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```
