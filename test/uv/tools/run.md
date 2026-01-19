# Tool Run

Tests for `uv tool run` (also available as `uvx`) to run tools in ephemeral environments.

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false

[filters]
exe-suffix = true
counts = true
```

## Running a tool

<!-- Derived from [`tool_run::tool_run_args`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L9-L62) -->

Run a tool with arguments. Note that when packages are cached, only the installation is shown.

```console
$ uv tool run pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Installed [N] packages in [TIME]
```

## Running with invalid version syntax

<!-- Derived from [`tool_run::tool_run_at_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L64-L157) -->

Invalid version syntax fails.

```console
$ uv tool run pytest@ --version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pytest@`
  Caused by: Expected URL
pytest@
       ^
```

## Suggesting valid commands

<!-- Derived from [`tool_run::tool_run_suggest_valid_commands`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L252-L301) -->

When an executable is not found, available executables are suggested.

```console
$ uv tool run --from black orange
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Installed [N] packages in [TIME]
An executable named `orange` is not provided by package `black`.
The following executables are available:
- black
- blackd
```

## Running with @version syntax

<!-- Derived from [`tool_run::tool_run_at_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L64-L157) -->

Run a specific version of a tool using `package@version` syntax.

```console
$ uv tool run pytest@8.0.0 --version
success: true
exit_code: 0
----- stdout -----
pytest 8.0.0

----- stderr -----
Installed [N] packages in [TIME]
```

## Running with --from and version

<!-- Derived from [`tool_run::tool_run_from_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L159-L186) -->

Run a tool from a specific version using `--from`.

```console
$ uv tool run --from pytest==8.0.0 pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.0.0

----- stderr -----
Installed [N] packages in [TIME]
```

## Running with @latest syntax

<!-- Derived from [`tool_run::tool_run_latest`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1928-L1991) -->

Run the latest version of a tool using `package@latest`.

```console
$ uv tool run pytest@latest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Installed [N] packages in [TIME]
```

## Running with extras

<!-- Derived from [`tool_run::tool_run_latest_extra`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1993-L2051) -->

Run a tool with extras using `package[extra]@version` syntax.

```console
$ uv tool run flask[dotenv]@latest --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.2
Werkzeug 3.0.1

----- stderr -----
Installed [N] packages in [TIME]
```

## Running with constraints file

<!-- Derived from [`tool_run::tool_run_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L188-L218) -->

Run a tool with a constraints file limiting dependency versions.

```text
# file: constraints.txt

pluggy<1.4.0
```

```console
$ uv tool run --constraints constraints.txt pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.0.2

----- stderr -----
Installed [N] packages in [TIME]
```

## Running with overrides file

<!-- Derived from [`tool_run::tool_run_overrides`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L220-L250) -->

Run a tool with an overrides file.

```text
# file: overrides.txt

pluggy<1.4.0
```

```console
$ uv tool run --override overrides.txt pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Installed [N] packages in [TIME]
```

## Running without command shows installed tools

<!-- Derived from [`tool_run::tool_run_list_installed`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1287-L1332) -->

Running without a command shows a help message, and if tools are installed, lists them.

```console
$ uv tool run
success: false
exit_code: 2
----- stdout -----
Provide a command to run with `uv tool run <command>`.

See `uv tool run --help` for more information.

----- stderr -----
```

After installing a tool:

```console
$ uv tool install black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool run
success: false
exit_code: 2
----- stdout -----
Provide a command to run with `uv tool run <command>`.

The following tools are installed:

- black v24.3.0

See `uv tool run --help` for more information.

----- stderr -----
```

## Running with requirements file

<!-- Derived from [`tool_run::tool_run_requirements_txt`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1199-L1239) -->

Run a tool with dependencies from a requirements file.

```text
# file: requirements.txt

iniconfig
```

```console
$ uv tool run --with-requirements requirements.txt pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Installed [N] packages in [TIME]
```

## Running with version specifier

<!-- Derived from [`tool_run::tool_run_specifier`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2086-L2115) -->

Run a tool with a version constraint (not using `@` syntax).

```console
$ uv tool run flask<3.0.0 --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 2.3.3
Werkzeug 3.0.1

----- stderr -----
Installed [N] packages in [TIME]
```

## Resolution error

<!-- Derived from [`tool_run::tool_run_resolution_error`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1909-L1926) -->

When a tool cannot be resolved, a clear error message is shown.

```console
$ uv tool run add
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving tool dependencies:
  ╰─▶ Because there are no versions of add and you require add, we can conclude that your requirements are unsatisfiable.
```

## Upgrade warning

<!-- Derived from [`tool_run::tool_run_upgrade_warn`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1854-L1905) -->

Using `--upgrade` with `uv tool run` shows a warning since it doesn't work as expected.

```console
$ uv tool run --upgrade pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
warning: Tools cannot be upgraded via `uv tool run`; use `uv tool upgrade --all` to upgrade all installed tools, or `uv tool run package@latest` to run the latest version of a tool.
Installed [N] packages in [TIME]
```

When combined with `--with`, a more specific hint is shown:

```console
$ uv tool run --upgrade --with typing-extensions pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
warning: Tools cannot be upgraded via `uv tool run`; use `uv tool upgrade --all` to upgrade all installed tools, `uv tool run package@latest` to run the latest version of a tool, or `uv tool run --refresh package` to upgrade any `--with` dependencies.
Installed [N] packages in [TIME]
```

## Running with --from and @version syntax

<!-- Derived from [`tool_run::tool_run_from_at`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2575-L2619) -->

Run a tool using `--from package@latest` or `--from package@version`.

```toml
# mdtest
[environment]
exclude-newer = "2025-01-18T00:00:00Z"
```

```console
$ uv tool run --from executable-application@latest app --version
success: true
exit_code: 0
----- stdout -----
app 0.3.0

----- stderr -----
Installed [N] packages in [TIME]
```

```console
$ uv tool run --from executable-application@0.2.0 app --version
success: true
exit_code: 0
----- stdout -----
app 0.2.0

----- stderr -----
Installed [N] packages in [TIME]
```

## Verbatim package name handling

<!-- Derived from [`tool_run::tool_run_verbatim_name`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2622-L2707) -->

When the executable name differs from the package name (underscores vs hyphens), uv handles it
correctly.

The normalized package name is `change-wheel-version`, but the executable is `change_wheel_version`.

```console
$ uv tool run change_wheel_version --help
success: true
exit_code: 0
----- stdout -----
usage: change_wheel_version [-h] [--local-version LOCAL_VERSION] [--version VERSION]
                            [--delete-old-wheel] [--allow-same-version]
                            wheel

positional arguments:
  wheel

options:
  -h, --help            show this help message and exit
  --local-version LOCAL_VERSION
  --version VERSION
  --delete-old-wheel
  --allow-same-version

----- stderr -----
Installed [N] packages in [TIME]
```

Using the hyphenated name fails with a helpful message:

```console
$ uv tool run change-wheel-version --help
success: false
exit_code: 1
----- stdout -----

----- stderr -----
An executable named `change-wheel-version` is not provided by package `change-wheel-version`.
The following executables are available:
- change_wheel_version

Use `uv tool run --from change-wheel-version change_wheel_version` instead.
```

Using `--from` with the correct executable name works:

```console
$ uv tool run --from change-wheel-version change_wheel_version --help
success: true
exit_code: 0
----- stdout -----
usage: change_wheel_version [-h] [--local-version LOCAL_VERSION] [--version VERSION]
                            [--delete-old-wheel] [--allow-same-version]
                            wheel

positional arguments:
  wheel

options:
  -h, --help            show this help message and exit
  --local-version LOCAL_VERSION
  --version VERSION
  --delete-old-wheel
  --allow-same-version

----- stderr -----

```

## Running from Git repository

<!-- Derived from [`tool_run::tool_run_git`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L937-L1024) -->

Run a tool directly from a Git repository.

```toml
# mdtest
[environment]
required-features = "git"
```

```console
$ uv tool run git+https://github.com/psf/black@24.2.0 --version
success: true
exit_code: 0
----- stdout -----
black, 24.2.0 (compiled: no)
Python (CPython) 3.12.[X]

----- stderr -----
Installed [N] packages in [TIME]
```

Using `--from` with a Git URL:

```console
$ uv tool run --from git+https://github.com/psf/black@24.2.0 black --version
success: true
exit_code: 0
----- stdout -----
black, 24.2.0 (compiled: no)
Python (CPython) 3.12.[X]

----- stderr -----

```

## Running from Git repository with LFS

<!-- Derived from [`tool_run::tool_run_git_lfs`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1029-L1108) -->

Run a tool from a Git repository that uses Git LFS for large files.

```toml
# mdtest

[environment]
python-version = "3.13"
required-features = "git"
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
counts = true
exe-suffix = true
```

Run directly from a Git URL with LFS:

```console
$ uv tool run --lfs git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f
success: true
exit_code: 0
----- stdout -----
Hello from test-lfs-repo!

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f#lfs=true)
```

Running again uses the cache:

```console
$ uv tool run --lfs test-lfs-repo @ git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f
success: true
exit_code: 0
----- stdout -----
Hello from test-lfs-repo!

----- stderr -----
Resolved [N] packages in [TIME]
```

Running with `--from` and accessing LFS assets:

```console
$ rm -rf .uv-cache

$ uv tool run --from git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f --lfs test-lfs-repo-assets
success: true
exit_code: 0
----- stdout -----
Hello from test-lfs-repo! LFS_TEST=True ANOTHER_LFS_TEST=True

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f#lfs=true)
```

Running again uses the cache:

```console
$ uv tool run --from test-lfs-repo @ git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f --lfs test-lfs-repo-assets
success: true
exit_code: 0
----- stdout -----
Hello from test-lfs-repo! LFS_TEST=True ANOTHER_LFS_TEST=True

----- stderr -----
Resolved [N] packages in [TIME]
```

## Running Python directly

<!-- Derived from [`tool_run::tool_run_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2118-L2145) -->

Run Python directly using `uv tool run python`.

```console
$ uv tool run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----

```

```console
$ uv tool run python -c "print('Hello, world!')"
success: true
exit_code: 0
----- stdout -----
Hello, world!

----- stderr -----

```

## Running Python with version specifier

<!-- Derived from [`tool_run::tool_run_python_at_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2148-L2216) -->

Run a specific Python version using `python@version` syntax.

```toml
# mdtest
[environment]
python-versions = ["3.12", "3.11"]

[filters]
python-sources = true
```

```console
$ uv tool run python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----

```

```console
$ uv tool run python@3.12 --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----

```

```console
$ uv tool run python@3.11 --version
success: true
exit_code: 0
----- stdout -----
Python 3.11.[X]

----- stderr -----

```

The `@` is optional:

```console
$ uv tool run python3.11 --version
success: true
exit_code: 0
----- stdout -----
Python 3.11.[X]

----- stderr -----

```

Dotless syntax also works:

```console
$ uv tool run python311 --version
success: true
exit_code: 0
----- stdout -----
Python 3.11.[X]

----- stderr -----

```

## Running Python with --from

<!-- Derived from [`tool_run::tool_run_python_from`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2400-L2470) -->

Run Python using `--from python` syntax with version specifiers.

```toml
# mdtest
[environment]
python-versions = ["3.12", "3.11"]

[filters]
python-sources = true
```

```console
$ uv tool run --from python python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----

```

```console
$ uv tool run --from python@3.11 python --version
success: true
exit_code: 0
----- stdout -----
Python 3.11.[X]

----- stderr -----

```

```console
$ uv tool run --from python311 python --version
success: true
exit_code: 0
----- stdout -----
Python 3.11.[X]

----- stderr -----

```

Using a version range:

```console
$ uv tool run --from "python>3.11,<3.13" python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----

```

## Running with editable dependency

<!-- Derived from [`tool_run::tool_run_with_editable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1690-L1770) -->

Run a tool with an editable dependency using `--with-editable`.

```console
$ uv tool run --with-editable ${WORKSPACE}/test/packages/black_editable --with iniconfig flask --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.2
Werkzeug 3.0.1

----- stderr -----
Installed [N] packages in [TIME]
```

## Running without output

<!-- Derived from [`tool_run::tool_run_without_output`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1334-L1373) -->

By default, resolver and installer output is omitted.

```console
$ uv tool run pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Installed [N] packages in [TIME]
```

Subsequent runs are completely quiet:

```console
$ uv tool run pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
```

## Running with verbose hint

<!-- Derived from [`tool_run::tool_run_verbose_hint`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2824-L2878) -->

When a tool fails and user provides verbose flags to the tool instead of uv, show a helpful hint.

With `--verbose` flag:

```console
$ uv tool run nonexistent-package-foo --verbose
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because nonexistent-package-foo was not found in the package registry and you require nonexistent-package-foo, we can conclude that your requirements are unsatisfiable.
  help: You provided `--verbose` to `nonexistent-package-foo`. Did you mean to provide it to `uv tool run`? e.g., `uv tool run --verbose nonexistent-package-foo`
```

With `-v` flag:

```console
$ uv tool run nonexistent-package-bar -v
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because nonexistent-package-bar was not found in the package registry and you require nonexistent-package-bar, we can conclude that your requirements are unsatisfiable.
  help: You provided `-v` to `nonexistent-package-bar`. Did you mean to provide it to `uv tool run`? e.g., `uv tool run -v nonexistent-package-bar`
```

## Running from URL

<!-- Derived from [`tool_run::tool_run_url`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L851-L927) -->

Tools can be run directly from URLs using `--from`.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
counts = true
```

Run with a named URL:

```console
$ uv tool run --from "flask @ https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl" flask --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.3
Werkzeug 3.0.1

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.3 (from https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl)
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

Run with just a URL:

```console
$ uv tool run --from https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl flask --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.3
Werkzeug 3.0.1

----- stderr -----
Resolved [N] packages in [TIME]
```

## Running with requirements file arguments

<!-- Derived from [`tool_run::tool_run_requirements_txt_arguments`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1241-L1283) -->

Index URL arguments in requirements files are ignored with a warning.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
counts = true
```

Create a requirements file with an index URL:

```toml file="requirements.txt"
--index-url https://test.pypi.org/simple
idna
```

```console
$ uv tool run --with-requirements requirements.txt flask --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.2
Werkzeug 3.0.1

----- stderr -----
warning: Ignoring `--index-url` from requirements file: `https://test.pypi.org/simple`. Instead, use the `--index-url` command-line argument, or set `index-url` in a `uv.toml` or `pyproject.toml` file.
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + idna==3.6
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
```

## Running with compatible build constraints

<!-- Derived from [`tool_run::tool_run_with_compatible_build_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2896-L2930) -->

Build constraints are respected when building dependencies.

```toml
# mdtest

[environment]
python-version = "3.9"
exclude-newer = "2024-05-04T00:00:00Z"

[filters]
counts = true
exe-suffix = true
```

```toml file="build_constraints.txt"
setuptools>=40
```

```console
$ uv tool run --with requests==1.2 --build-constraints build_constraints.txt pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.2.0

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + exceptiongroup==1.2.1
 + iniconfig==2.0.0
 + packaging==24.0
 + pluggy==1.5.0
 + pytest==8.2.0
 + requests==1.2.0
 + tomli==2.0.1
```

## Running with incompatible build constraints

<!-- Derived from [`tool_run::tool_run_with_incompatible_build_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2933-L2966) -->

Incompatible build constraints cause build failures.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }
python-version = "3.9"
exclude-newer = "2024-05-04T00:00:00Z"

[filters]
counts = true
exe-suffix = true
```

```toml file="build_constraints.txt"
setuptools==2
```

```console
$ uv tool run --with requests==1.2 --build-constraints build_constraints.txt pytest --version
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to download and build `requests==1.2.0`
  ├─▶ Failed to resolve requirements from `setup.py` build
  ├─▶ No solution found when resolving: `setuptools>=40.8.0`
  ╰─▶ Because you require setuptools>=40.8.0 and setuptools==2, we can conclude that your requirements are unsatisfiable.
```

## Hinting when Python version not available

<!-- Derived from [`tool_run::tool_run_hint_version_not_available`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2352-L2397) -->

When Python downloads are disabled or unavailable, helpful hints are provided.

```toml
# mdtest

[environment]
python-versions = []

[filters]
counts = true
python-sources = true
```

With Python downloads set to "never":

```console
$ UV_PYTHON_DOWNLOADS=never uv tool run python@3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.12 in [PYTHON SOURCES]

hint: A managed Python download is available for Python 3.12, but Python downloads are set to 'never'
```

In offline mode:

```console
$ UV_PYTHON_DOWNLOADS=auto UV_OFFLINE=true uv tool run python@3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.12 in [PYTHON SOURCES]

hint: A managed Python download is available for Python 3.12, but uv is set to offline mode
```

With managed Python disabled:

```console
$ UV_PYTHON_DOWNLOADS=auto UV_NO_MANAGED_PYTHON=true uv tool run python@3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No interpreter found for Python 3.12 in [PYTHON SOURCES]

hint: A managed Python download is available for Python 3.12, but the Python preference is set to 'only system'
```

## Re-resolving for compatible Python version

<!-- Derived from [`tool_run::tool_run_reresolve_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L3382-L3451) -->

When the default Python version is incompatible, uv re-resolves with a compatible version.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
python-versions = ["3.11", "3.12"]

[filters]
counts = true
```

Create a package requiring Python 3.12+:

```toml file="foo/pyproject.toml"
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = []

[project.scripts]
foo = "foo:run"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatchling.build.targets.wheel]
packages = ["src/foo"]
```

```python file="foo/src/foo/__init__.py"
import sys

def run():
    print(".".join(str(key) for key in sys.version_info[:2]))
```

Run automatically selects Python 3.12:

```console
$ uv tool run --from ./foo foo
success: true
exit_code: 0
----- stdout -----
3.12

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/foo)
```

Explicitly requesting incompatible version fails:

```console
$ uv tool run --from ./foo --python 3.11 foo
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because foo==1.0.0 depends on Python >=3.12 and you require foo==1.0.0, we can conclude that your requirements are incompatible with the available Python version (3.11.[X]).
```

## Warning about executable not in from package

<!-- Derived from [`tool_run::tool_run_warn_executable_not_in_from`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L304-L367) -->

When running an executable not provided by the `--from` package, a helpful warning is shown.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
exclude-newer = "2024-05-04T00:00:00Z"

[filters]
exe-suffix = true
```

```console
$ uv tool run --from fastapi fastapi
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 35 packages in [TIME]
Prepared 35 packages in [TIME]
Installed 35 packages in [TIME]
 + annotated-types==0.6.0
 + anyio==4.3.0
 + certifi==2024.2.2
 + click==8.1.7
 + dnspython==2.6.1
 + email-validator==2.1.1
 + fastapi==0.111.0
 + fastapi-cli==0.0.2
 + h11==0.14.0
 + httpcore==1.0.5
 + httptools==0.6.1
 + httpx==0.27.0
 + idna==3.7
 + jinja2==3.1.3
 + markdown-it-py==3.0.0
 + markupsafe==2.1.5
 + mdurl==0.1.2
 + orjson==3.10.3
 + pydantic==2.7.1
 + pydantic-core==2.18.2
 + pygments==2.17.2
 + python-dotenv==1.0.1
 + python-multipart==0.0.9
 + pyyaml==6.0.1
 + rich==13.7.1
 + shellingham==1.5.4
 + sniffio==1.3.1
 + starlette==0.37.2
 + typer==0.12.3
 + typing-extensions==4.11.0
 + ujson==5.9.0
 + uvicorn==0.29.0
 + uvloop==0.19.0
 + watchfiles==0.21.0
 + websockets==12.0
warning: An executable named `fastapi` is not provided by package `fastapi` but is available via the dependency `fastapi-cli`. Consider using `uv tool run --from fastapi-cli fastapi` instead.
```

## Using installed tool version

<!-- Derived from [`tool_run::tool_run_from_install`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L370-L502) -->

When a tool is already installed, `uv tool run` uses that version unless otherwise specified.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
counts = true
```

First install black at a specific version:

```console
$ uv tool install black==24.1.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.1.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
```

Running uses the installed version:

```console
$ uv tool run black --version
success: true
exit_code: 0
----- stdout -----
black, 24.1.0 (compiled: yes)
Python (CPython) 3.12.[X]

----- stderr -----
```

Using `--isolated` ignores the installed version:

```console
$ uv tool run --isolated black --version
success: true
exit_code: 0
----- stdout -----
black, 24.3.0 (compiled: yes)
Python (CPython) 3.12.[X]

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
```

Specifying a different version with `@` syntax:

```console
$ uv tool run black@24.1.1 --version
success: true
exit_code: 0
----- stdout -----
black, 24.1.1 (compiled: yes)
Python (CPython) 3.12.[X]

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.1.1
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
```

## Using installed tool with constraints

<!-- Derived from [`tool_run::tool_run_from_install_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L505-L601) -->

Compatible constraints use the installed tool, incompatible constraints trigger reinstall.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
counts = true
```

Install flask at a specific version:

```console
$ uv tool install flask==3.0.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.0
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
Installed 1 executable: flask
```

Running uses the installed version:

```console
$ uv tool run flask --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.0
Werkzeug 3.0.1

----- stderr -----
```

Compatible constraint uses installed version:

```toml file="constraints.txt"
werkzeug<4.0.0
```

```console
$ uv tool run --constraints constraints.txt flask --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.0
Werkzeug 3.0.1

----- stderr -----
```

Incompatible constraint triggers reinstall:

```toml file="constraints.txt"
werkzeug<3.0.0
```

```console
$ uv tool run --constraints constraints.txt flask --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]
Flask 3.0.0
Werkzeug 2.3.7

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.0
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==2.3.7
```

## Error on script in from argument

<!-- Derived from [`tool_run::tool_run_with_from_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2784-L2801) -->

Providing a Python script to `--from` is not supported.

```toml
# mdtest

[filters]
counts = true
```

```console
$ uv tool run --from script.py ruff
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: It looks like you provided a Python script to `--from`, which is not supported

hint: If you meant to run a command from the `script-py` package, use the normalized package name instead to disambiguate, e.g., `uv tool run --from script-py ruff`
```

## Error on script and from script

<!-- Derived from [`tool_run::tool_run_with_script_and_from_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2804-L2821) -->

Providing scripts to both command and `--from` is not supported.

```toml
# mdtest

[filters]
counts = true
```

```console
$ uv tool run --from script.py other-script.py
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: It looks like you provided a Python script to `--from`, which is not supported

hint: If you meant to run a command from the `script-py` package, use the normalized package name instead to disambiguate, e.g., `uv tool run --from script-py other-script.py`
```

## Running with dependencies from script

<!-- Derived from [`tool_run::tool_run_with_dependencies_from_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2969-L3030) -->

The `--with-requirements` flag can extract dependencies from PEP 723 script metadata.

```toml
# mdtest

[filters]
counts = true
missing-file-error = true
```

Create a script with PEP 723 metadata:

```python file="script.py"
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "anyio",
# ]
# ///

import anyio
```

```python file="script-no-ext"
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "anyio",
# ]
# ///

import anyio
```

```console
$ uv tool run --with-requirements script.py black script.py -q
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + anyio==4.3.0
 + black==24.3.0
 + click==8.1.7
 + idna==3.6
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
 + sniffio==1.3.1
```

Works with scripts without extension too:

```console
$ uv tool run --with-requirements script-no-ext black script-no-ext -q
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
```

## Error running existing .py script

<!-- Derived from [`tool_run::tool_run_with_existing_py_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2710-L2725) -->

Running a Python script with `.py` extension is not supported.

```toml
# mdtest

[environment]
tree = { "script.py" = { create = true } }

[filters]
counts = true
```

```console
$ uv tool run script.py
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: It looks like you tried to run a Python script at `script.py`, which is not supported by `uv tool run`

hint: Use `uv run script.py` instead
```

## Error running existing .pyw script

<!-- Derived from [`tool_run::tool_run_with_existing_pyw_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2728-L2745) -->

Running a Python script with `.pyw` extension is not supported.

```toml
# mdtest

[environment]
tree = { "script.pyw" = { create = true } }

[filters]
counts = true
```

```console
$ uv tool run script.pyw
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: It looks like you tried to run a Python script at `script.pyw`, which is not supported by `uv tool run`

hint: Use `uv run script.pyw` instead
```

## Error with nonexistent .py script

<!-- Derived from [`tool_run::tool_run_with_nonexistent_py_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2748-L2763) -->

When a `.py` file doesn't exist, suggest using `--from` to disambiguate.

```toml
# mdtest

[filters]
counts = true
```

```console
$ uv tool run script.py
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: It looks like you provided a Python script to run, which is not supported supported by `uv tool run`

hint: We did not find a script at the requested path. If you meant to run a command from the `script-py` package, pass the normalized package name to `--from` to disambiguate, e.g., `uv tool run --from script-py script.py`
```

## Error with nonexistent .pyw script

<!-- Derived from [`tool_run::tool_run_with_nonexistent_pyw_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2766-L2781) -->

When a `.pyw` file doesn't exist, suggest using `--from` to disambiguate.

```toml
# mdtest

[filters]
counts = true
```

```console
$ uv tool run script.pyw
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: It looks like you provided a Python script to run, which is not supported supported by `uv tool run`

hint: We did not find a script at the requested path. If you meant to run a command from the `script-pyw` package, pass the normalized package name to `--from` to disambiguate, e.g., `uv tool run --from script-pyw script.pyw`
```

## Running with environment file

<!-- Derived from [`tool_run::run_with_env_file`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L2481-L2572) -->

The `--env-file` flag loads environment variables from a file.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
counts = true
```

Create a custom project with a script that prints environment variables:

```toml file="foo/pyproject.toml"
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = []

[project.scripts]
script = "foo.main:run"

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python file="foo/src/foo/main.py"
def run():
    import os

    print(os.environ.get('THE_EMPIRE_VARIABLE'))
    print(os.environ.get('REBEL_1'))
    print(os.environ.get('REBEL_2'))
    print(os.environ.get('REBEL_3'))

__name__ == "__main__" and run()
```

Running the script without an env file shows None for all variables:

```console
$ uv tool run --from ./foo script
success: true
exit_code: 0
----- stdout -----
None
None
None
None

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/foo)
```

Create an environment file:

```text file=".file"
THE_EMPIRE_VARIABLE=palpatine
REBEL_1=leia_organa
REBEL_2=obi_wan_kenobi
REBEL_3=C3PO
```

Running with `--env-file` loads the environment variables:

```console
$ uv tool run --env-file .file --from ./foo script
success: true
exit_code: 0
----- stdout -----
palpatine
leia_organa
obi_wan_kenobi
C3PO

----- stderr -----
Resolved [N] packages in [TIME]
```

## Caching tool environments

<!-- Derived from [`tool_run::tool_run_cache`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L688-L754) -->

Tool environments are cached based on the Python version and package versions.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
python-versions = ["3.11", "3.12"]

[filters]
counts = true
```

First run with Python 3.12:

```console
$ uv tool run -p 3.12 black --version
success: true
exit_code: 0
----- stdout -----
black, 24.3.0 (compiled: yes)
Python (CPython) 3.12.[X]

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
```

Running again with Python 3.12 uses the cache:

```console
$ uv tool run -p 3.12 black --version
success: true
exit_code: 0
----- stdout -----
black, 24.3.0 (compiled: yes)
Python (CPython) 3.12.[X]

----- stderr -----
```

Running with Python 3.11 creates a new environment:

```console
$ uv tool run -p 3.11 black --version
success: true
exit_code: 0
----- stdout -----
black, 24.3.0 (compiled: yes)
Python (CPython) 3.11.[X]

----- stderr -----
Resolved [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
```

## Error when package has no executables

<!-- Derived from [`tool_run::warn_no_executables_found`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1826-L1850) -->

When a package provides no executables, an error is shown.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
exe-suffix = true
```

```console
$ uv tool run requests
success: false
exit_code: 1
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
Package `requests` does not provide any executables.
```

## Running with comma-separated dependencies (shorthand)

<!-- Derived from [`tool_run::tool_run_csv_with_shorthand`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1377-L1435) -->

The `-w` shorthand flag accepts comma-separated dependencies.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
target-family = "unix"

[filters]
counts = true
```

```console
$ uv tool run -w iniconfig,typing-extensions pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
 + packaging==24.0
 + pluggy==1.4.0
 + pytest==8.1.1
 + typing-extensions==4.10.0
```

## Running with comma-separated dependencies

<!-- Derived from [`tool_run::tool_run_csv_with`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1439-L1559) -->

The `--with` flag accepts comma-separated dependencies.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
counts = true
```

```console
$ uv tool run --with iniconfig,typing-extensions pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
 + packaging==24.0
 + pluggy==1.4.0
 + pytest==8.1.1
 + typing-extensions==4.10.0
```

## Running with repeated with flags

<!-- Derived from [`tool_run::tool_run_repeated_with`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L1563-L1621) -->

Multiple `--with` flags can be used together.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
target-family = "unix"

[filters]
counts = true
```

```console
$ uv tool run --with iniconfig --with typing-extensions pytest --version
success: true
exit_code: 0
----- stdout -----
pytest 8.1.1

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
 + packaging==24.0
 + pluggy==1.4.0
 + pytest==8.1.1
 + typing-extensions==4.10.0
```

## Windows runnable types

<!-- Derived from [`tool_run::tool_run_windows_runnable_types`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L3070-L3199) -->

On Windows, tools can include .bat, .cmd, and .ps1 script types alongside .exe executables.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }
target-family = "windows"

[filters]
counts = true
```

Create a package with legacy script files:

```toml file="foo/pyproject.toml"
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = []

[project.scripts]
custom_pydoc = "foo.main:run"

[tool.setuptools]
script-files = [
    "misc/custom_pydoc.bat",
    "misc/custom_pydoc.cmd",
    "misc/custom_pydoc.ps1"
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```text file="foo/misc/custom_pydoc.bat"
python.exe -m pydoc %*
```

```text file="foo/misc/custom_pydoc.cmd"
python.exe -m pydoc %*
```

```text file="foo/misc/custom_pydoc.ps1"
python.exe -m pydoc $args
```

```python file="foo/src/foo/main.py"
import pydoc, sys

def run():
    sys.argv[0] = "pydoc"
    pydoc.cli()

__name__ == "__main__" and run()
```

Install the tool:

```console
$ uv tool install foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/foo)
Installed 4 executables: custom_pydoc.bat, custom_pydoc.cmd, custom_pydoc.exe, custom_pydoc.ps1
```

Running with a nonexistent executable shows all available types:

```console
$ uv tool run --from foo does_not_exist
success: false
exit_code: 1
----- stdout -----

----- stderr -----
An executable named `does_not_exist` is not provided by package `foo`.
The following executables are available:
- custom_pydoc.bat
- custom_pydoc.cmd
- custom_pydoc.exe
- custom_pydoc.ps1
```

Running with explicit .bat extension:

```console
$ uv tool run --from foo custom_pydoc.bat
success: true
exit_code: 0
----- stdout -----
pydoc - the Python documentation tool

pydoc <name> ...
    Show text documentation on something.  <name> may be the name of a
    Python keyword, topic, function, module, or package, or a dotted
    reference to a class or function within a module or module in a
    package.  If <name> contains a '\', it is used as the path to a
    Python source file to document. If name is 'keywords', 'topics',
    or 'modules', a listing of these things is displayed.

pydoc -k <keyword>
    Search for a keyword in the synopsis lines of all available modules.

pydoc -n <hostname>
    Start an HTTP server with the given hostname (default: localhost).

pydoc -p <port>
    Start an HTTP server on the given port on the local machine.  Port
    number 0 can be used to get an arbitrary unused port.

pydoc -b
    Start an HTTP server on an arbitrary unused port and open a web browser
    to interactively browse documentation.  This option can be used in
    combination with -n and/or -p.

pydoc -w <name> ...
    Write out the HTML documentation for a module to a file in the current
    directory.  If <name> contains a '\', it is treated as a filename; if
    it names a directory, documentation is written for all the contents.


----- stderr -----
```

## Windows dotted package name

<!-- Derived from [`tool_run::tool_run_windows_dotted_package_name`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L3476-L3508) -->

Windows handles packages with dots in the name.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
target-family = "windows"

[filters]
counts = true
```

```console
$ uv tool run --from ${WORKSPACE}/test/packages/package.name.with.dots package.name.with.dots
success: true
exit_code: 0
----- stdout -----
package.name.with.dots version 0.1.0

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + package-name-with-dots==0.1.0 (from file://[TEMP_DIR]/package.name.with.dots)
```

## Keyring authentication with @latest

<!-- Derived from [`tool_run::tool_run_latest_keyring_auth`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_run.rs#L3512-L3562) -->

The keyring is consulted when resolving @latest versions with authenticated indexes.

```toml
# mdtest

[environment]
exclude-newer = "2025-01-18T00:00:00Z"
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", KEYRING_TEST_CREDENTIALS = '{"pypi-proxy.fly.dev": {"public": "heron"}}' }

[filters]
counts = true
```

```console
$ uv tool install --index https://public@pypi-proxy.fly.dev/basic-auth/simple --keyring-provider subprocess executable-application@latest
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Keyring request for public@https://pypi-proxy.fly.dev/basic-auth/simple
Keyring request for public@pypi-proxy.fly.dev
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + executable-application==0.3.0
Installed 1 executable: app
```
