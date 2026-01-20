# Basic Installation

Tests for basic `uv pip install` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
exclude-newer = "2024-03-25T00:00:00Z"
```

## Package installation

### Install a package

<!-- from pip_install.rs::install_package -->

Install a package and its dependencies.

```console
$ uv pip install Flask --strict
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

### Install from requirements file

<!-- from pip_install.rs::install_requirements_txt -->

Install packages from a requirements.txt file.

```toml
# file: requirements.txt
Flask
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

## Editable installation

### Install editable package

<!-- from pip_install.rs::install_editable -->

Install a package in editable mode.

```console
$ uv pip install -e ${WORKSPACE}/test/packages/poetry_editable
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

## Upgrade behavior

### Upgrade a package

<!-- from pip_install.rs::install_upgrade -->

The `--upgrade-package` flag upgrades a specific package.

```console
$ uv pip install anyio==3.6.2 --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.6.2
 + idna==3.6
 + sniffio==1.3.1
```

```console
$ uv pip install anyio --upgrade-package anyio
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 - anyio==3.6.2
 + anyio==4.3.0
```

## Constraints

### Install with constraints

<!-- from pip_install.rs::install_constraints_txt -->

The `--constraint` flag limits package versions.

```toml
# file: requirements.txt
anyio==3.7.0
```

```toml
# file: constraints.txt
idna<3.4
```

```console
$ uv pip install -r requirements.txt --constraint constraints.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.3
 + sniffio==1.3.1
```

## No index

### Install with no index

<!-- from pip_install.rs::install_no_index -->

The `--no-index` flag disables package index lookups.

```console
$ uv pip install Flask --no-index
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because flask was not found in the provided package locations and you require flask, we can conclude that your requirements are unsatisfiable.

      hint: Packages were unavailable because index lookups were disabled and no additional package locations were provided (try: `--find-links <uri>`)
```

### No index with version

<!-- from pip_install.rs::install_no_index_version -->

The version is included in the error message when specified.

```console
$ uv pip install Flask==3.0.0 --no-index
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because flask was not found in the provided package locations and you require flask==3.0.0, we can conclude that your requirements are unsatisfiable.

      hint: Packages were unavailable because index lookups were disabled and no additional package locations were provided (try: `--find-links <uri>`)
```

## Index priority

### Extra index URL has priority

<!-- from pip_install.rs::install_extra_index_url_has_priority -->

When using `--extra-index-url`, the extra index is checked first.

```console
$ uv pip install black==24.2.0 --no-deps --index-url https://test.pypi.org/simple --extra-index-url https://pypi.org/simple --exclude-newer 2024-03-09T00:00:00Z
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + black==24.2.0
```

## Reinstall

### Reinstall a package

<!-- from pip_install.rs::reinstall -->

The `--reinstall` flag reinstalls packages even if they're already installed.

```console
$ uv pip install iniconfig==2.0.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

```console
$ uv pip install iniconfig==2.0.0 --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 ~ iniconfig==2.0.0
```

## Extras

### Install package with extra

<!-- from pip_install.rs::install_extra -->

Install a package with an extra.

```console
$ uv pip install Flask[async]
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
Prepared 8 packages in [TIME]
Installed 8 packages in [TIME]
 + asgiref==3.8.1
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

## Multiple requirements files

### Install from multiple requirements files

<!-- from pip_install.rs::install_requirements_txt_multiple -->

Install from multiple requirements files.

```toml
# file: requirements1.txt
Flask
```

```toml
# file: requirements2.txt
iniconfig
```

```console
$ uv pip install -r requirements1.txt -r requirements2.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 8 packages in [TIME]
Prepared 8 packages in [TIME]
Installed 8 packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + iniconfig==2.0.0
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

## Dry run

### Dry run installation

<!-- from pip_install.rs::dry_run -->

The `--dry-run` flag shows what would be installed without installing.

```console
$ uv pip install Flask --dry-run
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Would download 7 packages
Would install 7 packages
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

## Error handling

### Missing requirements file

<!-- from pip_install.rs::missing_requirements_txt -->

Installing from a non-existent requirements file shows an error.

```console
$ uv pip install -r requirements.txt --strict
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: File not found: `requirements.txt`
```

### No solution

<!-- from pip_install.rs::no_solution -->

Installing incompatible packages shows a resolution error.

```console
$ uv pip install "flask>=3.0.2" "WerkZeug<1.0.0" --strict
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because only flask<=3.0.2 is available and flask==3.0.2 depends on werkzeug>=3.0.0, we can conclude that flask>=3.0.2 depends on werkzeug>=3.0.0.
      And because you require flask>=3.0.2 and werkzeug<1.0.0, we can conclude that your requirements are unsatisfiable.
```

### Empty requirements file

<!-- from pip_install.rs::empty_requirements_txt -->

Installing from an empty requirements file is a warning, not an error.

```toml
# file: requirements.txt

```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Requirements file `requirements.txt` does not contain any dependencies
Audited in [TIME]
```

## Respecting installed packages

### Respect installed and reinstall

<!-- from pip_install.rs::respect_installed_and_reinstall -->

When installing a package that's already installed, the existing version is respected.

```toml
# file: requirements.txt
Flask==2.3.2
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
 + flask==2.3.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

Re-install Flask without version constraint. The existing version is respected.

```toml
# file: requirements.txt
Flask
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 1 package in [TIME]
```

## No dependencies

### Install without dependencies

<!-- from pip_install.rs::no_deps -->

The `--no-deps` flag installs without dependencies.

```console
$ uv pip install Flask --no-deps --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + flask==3.0.2
warning: The package `flask` requires `werkzeug>=3.0.0`, but it's not installed
warning: The package `flask` requires `jinja2>=3.1.2`, but it's not installed
warning: The package `flask` requires `itsdangerous>=2.1.2`, but it's not installed
warning: The package `flask` requires `click>=8.1.3`, but it's not installed
warning: The package `flask` requires `blinker>=1.6.2`, but it's not installed
```

## Find links

### Install from find links

<!-- from pip_install.rs::find_links -->

The `--find-links` flag allows installation from a local directory.

```console
$ uv pip install tqdm --find-links ${WORKSPACE}/test/links/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + tqdm==1000.0.0
```

### Find links with no binary

<!-- from pip_install.rs::find_links_no_binary -->

With `--no-binary :all:`, only source distributions are used.

```console
$ uv pip install tqdm --no-binary :all: --find-links ${WORKSPACE}/test/links/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + tqdm==999.0.0
```

## All extras

### All extras requires project file

<!-- from pip_install.rs::install_extras -->

The `--all-extras` flag requires a pyproject.toml file.

```toml
# file: requirements.txt
anyio==3.7.0
```

```console
$ uv pip install --all-extras -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Requesting extras requires a `pylock.toml`, `pyproject.toml`, `setup.cfg`, or `setup.py` file. Use `package[extra]` syntax instead.
```

### All extras with pyproject.toml

<!-- from pip_install.rs::install_extras -->

The `--all-extras` flag works with pyproject.toml.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
dependencies = ["anyio==3.7.0"]
```

```console
$ uv pip install --all-extras -r pyproject.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

## Exact installation

### Exact install removes extraneous packages

<!-- from pip_install.rs::exact_install_removes_extraneous_packages -->

The `--exact` flag removes packages not in the requirements file.

```toml
# file: requirements.txt
anyio==3.7.0
```

```console
$ uv pip install --exact -r requirements.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + sniffio==1.3.1
```

Install flask as well.

```console
$ uv pip install flask
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

Using `--exact` again removes flask and its dependencies.

```console
$ uv pip install --exact -r requirements.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Uninstalled 7 packages in [TIME]
 - blinker==1.7.0
 - click==8.1.7
 - flask==3.0.2
 - itsdangerous==2.1.2
 - jinja2==3.1.3
 - markupsafe==2.1.5
 - werkzeug==3.0.1
```

## Reinstall with extras

### Install package then add extra

<!-- from pip_install.rs::reinstall_extras -->

Installing a package then re-installing with an extra should add the extra dependencies.

```toml
# file: requirements.txt
httpx
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
 + anyio==4.3.0
 + certifi==2024.2.2
 + h11==0.14.0
 + httpcore==1.0.4
 + httpx==0.27.0
 + idna==3.6
 + sniffio==1.3.1
```

Re-install httpx with an extra:

```toml
# file: requirements.txt
httpx[http2]
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + h2==4.1.0
 + hpack==4.0.0
 + hyperframe==6.0.1
```

## Poetry projects

### Install from Poetry pyproject.toml

<!-- from pip_install.rs::install_pyproject_toml_poetry -->

Installing from a Poetry-style pyproject.toml works with extras.

```toml
# file: pyproject.toml
[tool.poetry]
name = "poetry-editable"
version = "0.1.0"
description = ""
authors = ["Astral Software Inc. <hey@astral.sh>"]

[tool.poetry.dependencies]
python = "^3.10"
anyio = "^3"
iniconfig = { version = "*", optional = true }

[tool.poetry.extras]
test = ["iniconfig"]

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

```console
$ uv pip install -r pyproject.toml --extra test
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==3.7.1
 + idna==3.6
 + iniconfig==2.0.0
 + sniffio==1.3.1
```

## Binary options

### Only binary with no binary fails

<!-- from pip_install.rs::install_only_binary_all_and_no_binary_all -->

Using `--only-binary :all:` and `--no-binary :all:` together prevents installation.

```console
$ uv pip install anyio --no-binary :all: --only-binary :all: --strict
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because all versions of anyio have no usable wheels and you require anyio, we can conclude that your requirements are unsatisfiable.

      hint: Pre-releases are available for `anyio` in the requested range (e.g., 4.0.0rc1), but pre-releases weren't enabled (try: `--prerelease=allow`)

      hint: Wheels are required for `anyio` because building from source is disabled for all packages (i.e., with `--no-build`)
```

### No binary overrides only binary all

<!-- from pip_install.rs::install_no_binary_overrides_only_binary_all -->

A specific `--no-binary` overrides the less specific `--only-binary :all:`.

```console
$ uv pip install anyio --only-binary :all: --no-binary idna --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

### Only binary overrides no binary all

<!-- from pip_install.rs::install_only_binary_overrides_no_binary_all -->

A specific `--only-binary` overrides the less specific `--no-binary :all:`.

```console
$ uv pip install anyio --no-binary :all: --only-binary idna --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

### No binary comma-separated

<!-- from pip_install.rs::install_no_binary_comma_separated -->

The `--no-binary` flag accepts comma-separated values.

```console
$ uv pip install anyio --no-binary=idna,sniffio --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

### Only binary comma-separated

<!-- from pip_install.rs::install_only_binary_comma_separated -->

The `--only-binary` flag accepts comma-separated values.

```console
$ uv pip install anyio --only-binary=idna,sniffio --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

## UV sources

### Install from pyproject.toml with tool.uv.sources

<!-- from pip_install.rs::tool_uv_sources_is_in_preview -->

The `tool.uv.sources` section can override dependency sources for pip install.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "0.0.0"
dependencies = [
  "iniconfig>1,<=2",
]

[tool.uv.sources]
iniconfig = { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl" }
```

```console
$ uv pip install -r pyproject.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0 (from https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl)
```

## Dry run

### Dry run install

<!-- from pip_install.rs::dry_run_install -->

The `--dry-run` flag shows what would be installed without actually installing.

```toml
# file: requirements.txt
httpx==0.25.1
```

```console
$ uv pip install -r requirements.txt --dry-run --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Would download 7 packages
Would install 7 packages
 + anyio==4.3.0
 + certifi==2024.2.2
 + h11==0.14.0
 + httpcore==1.0.4
 + httpx==0.25.1
 + idna==3.6
 + sniffio==1.3.1
```

### Dry run install URL dependency

<!-- from pip_install.rs::dry_run_install_url_dependency -->

The `--dry-run` flag works with URL dependencies.

```toml
# file: requirements.txt
anyio @ https://files.pythonhosted.org/packages/2d/b8/7333d87d5f03247215d86a86362fd3e324111788c6cdd8d2e6196a6ba833/anyio-4.2.0.tar.gz
```

```console
$ uv pip install -r requirements.txt --dry-run
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Would download 3 packages
Would install 3 packages
 + anyio @ https://files.pythonhosted.org/packages/2d/b8/7333d87d5f03247215d86a86362fd3e324111788c6cdd8d2e6196a6ba833/anyio-4.2.0.tar.gz
 + idna==3.6
 + sniffio==1.3.1
```

### Dry run install already installed

<!-- from pip_install.rs::dry_run_install_already_installed -->

Dry run when packages are already installed shows no changes.

```toml
# file: requirements.txt
httpx==0.25.1
```

Install the packages first:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 7 packages in [TIME]
Prepared 7 packages in [TIME]
Installed 7 packages in [TIME]
 + anyio==4.3.0
 + certifi==2024.2.2
 + h11==0.14.0
 + httpcore==1.0.4
 + httpx==0.25.1
 + idna==3.6
 + sniffio==1.3.1
```

Then dry run shows no changes:

```console
$ uv pip install -r requirements.txt --dry-run --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 1 package in [TIME]
Would make no changes
```

## Invalid input

### Invalid pyproject.toml syntax

<!-- from pip_install.rs::invalid_pyproject_toml_syntax -->

Invalid TOML syntax produces a parse error.

```toml
# file: pyproject.toml
123 - 456
```

```console
$ uv pip install -r pyproject.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 1, column 5
    |
  1 | 123 - 456
    |     ^
  key with no value, expected `=`

error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 5
  |
1 | 123 - 456
  |     ^
key with no value, expected `=`
```

### Invalid pyproject.toml project schema

<!-- from pip_install.rs::invalid_pyproject_toml_project_schema -->

A pyproject.toml with a `[project]` table but no name produces an error.

```toml
# file: pyproject.toml
[project]
```

```console
$ uv pip install -r pyproject.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 1
  |
1 | [project]
  | ^^^^^^^^^
`pyproject.toml` is using the `[project]` table, but the required `project.name` field is not set
```

### Invalid TOML filename

<!-- from pip_install.rs::invalid_toml_filename -->

A TOML file that doesn't follow PEP 751 naming conventions produces an error.

```toml
# file: test.toml

```

```console
$ uv pip install -r test.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `test.toml` is not a valid PEP 751 filename: expected TOML file to start with `pylock.` and end with `.toml` (e.g., `pylock.toml`, `pylock.dev.toml`)
```

### Conflicting pins in requirements file

<!-- from pip_install.rs::install_requirements_txt_conflicting_pins -->

Installing from a requirements file with conflicting pins fails.

```toml
# file: requirements.txt
blinker==1.7.0
click==7.0.0
flask==3.0.2
itsdangerous==2.1.2
jinja2==3.1.3
markupsafe==2.1.5
werkzeug==3.0.1
```

```console
$ uv pip install -r requirements.txt --strict
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because flask==3.0.2 depends on click>=8.1.3 and you require click==7.0.0, we can conclude that your requirements and flask==3.0.2 are incompatible.
      And because you require flask==3.0.2, we can conclude that your requirements are unsatisfiable.
```

### Unsupported flags in requirements file

<!-- from pip_install.rs::install_unsupported_flag -->

Unsupported flags in a requirements file are ignored with a warning.

```toml
# file: requirements.txt
--pre
--prefer-binary :all:
iniconfig
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Ignoring unsupported option in `requirements.txt`: `--pre` (hint: pass `--pre` on the command line instead)
warning: Ignoring unsupported option in `requirements.txt`: `--prefer-binary`
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Invalid editable without URL

<!-- from pip_install.rs::invalid_editable_no_url -->

Editable installs must refer to a local directory, not a versioned package.

```toml
# file: requirements.txt
-e black==0.1.0
```

```console
$ uv pip install -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unsupported editable requirement in `requirements.txt`
  Caused by: Editable `black` must refer to a local directory, not a versioned package
```

### Invalid editable with unnamed HTTPS URL

<!-- from pip_install.rs::invalid_editable_unnamed_https_url -->

Editable installs must refer to a local directory, not an HTTPS URL.

```toml
# file: requirements.txt
-e https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl
```

```console
$ uv pip install -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unsupported editable requirement in `requirements.txt`
  Caused by: Editable must refer to a local directory, not an HTTPS URL: `https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl`
```

### Invalid editable with named HTTPS URL

<!-- from pip_install.rs::invalid_editable_named_https_url -->

Named editable installs must refer to a local directory, not an HTTPS URL.

```toml
# file: requirements.txt
-e black @ https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl
```

```console
$ uv pip install -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unsupported editable requirement in `requirements.txt`
  Caused by: Editable `black` must refer to a local directory, not an HTTPS URL: `https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl`
```

### Invalid file extension

<!-- from pip_install.rs::invalid_extension -->

A URL with an unsupported file extension fails.

```console
$ uv pip install "ruff @ https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6.tar.baz"
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `ruff @ https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6.tar.baz`
  Caused by: Expected direct URL (`https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6.tar.baz`) to end in a supported file extension: `.whl`, `.tar.gz`, `.zip`, `.tar.bz2`, `.tar.lz`, `.tar.lzma`, `.tar.xz`, `.tar.zst`, `.tar`, `.tbz`, `.tgz`, `.tlz`, or `.txz`
ruff @ https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6.tar.baz
       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

### No file extension

<!-- from pip_install.rs::no_extension -->

A URL without a file extension fails.

```console
$ uv pip install "ruff @ https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6"
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `ruff @ https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6`
  Caused by: Expected direct URL (`https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6`) to end in a supported file extension: `.whl`, `.tar.gz`, `.zip`, `.tar.bz2`, `.tar.lz`, `.tar.lzma`, `.tar.xz`, `.tar.zst`, `.tar`, `.tbz`, `.tgz`, `.tlz`, or `.txz`
ruff @ https://files.pythonhosted.org/packages/f7/69/96766da2cdb5605e6a31ef2734aff0be17901cefb385b885c2ab88896d76/ruff-0.5.6
       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
