# Tool List

Tests for `uv tool list` to display installed tools.

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false

[filters]
exe-suffix = true
counts = true
```

## Empty tool list

<!-- Derived from [`tool_list::tool_list_empty`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L99-L115) -->

When no tools are installed, `uv tool list` shows a message.

```console
$ uv tool list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
No tools installed
```

## Listing installed tools

<!-- Derived from [`tool_list::tool_list`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L9-L36) -->

After installing a tool, `uv tool list` shows it with its executables.

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
$ uv tool list
success: true
exit_code: 0
----- stdout -----
black v24.3.0
- black
- blackd

----- stderr -----
```

## Listing with paths

<!-- Derived from [`tool_list::tool_list_paths`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L38-L65) -->

The `--show-paths` flag displays the installation paths.

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
$ uv tool list --show-paths
success: true
exit_code: 0
----- stdout -----
black v24.3.0 ([HOME]/data/uv/tools/black)
- black ([HOME]/data/../bin/black)
- blackd ([HOME]/data/../bin/blackd)

----- stderr -----
```

## Listing with paths on Windows

<!-- Derived from [`tool_list::tool_list_paths_windows`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L67-L97) -->

On Windows, paths are displayed with Windows-style backslashes.

```toml
# mdtest

[environment]
target-family = "windows"

[filters]
exe-suffix = false
```

```console
$ uv tool install black==24.2.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

```console
$ uv tool list --show-paths
success: true
exit_code: 0
----- stdout -----
black v24.2.0 ([TEMP_DIR]\tools\black)
- black ([TEMP_DIR]\bin\black.exe)
- blackd ([TEMP_DIR]\bin\blackd.exe)

----- stderr -----
```

## Listing with Python version

<!-- Derived from [`tool_list::tool_list_show_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L567-L595) -->

The `--show-python` flag shows the Python version used by each tool.

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
$ uv tool list --show-python
success: true
exit_code: 0
----- stdout -----
black v24.3.0 [CPython 3.12.[X]]
- black
- blackd

----- stderr -----
```

## Listing with version specifiers

<!-- Derived from [`tool_list::tool_list_show_version_specifiers`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L285-L339) -->

The `--show-version-specifiers` flag shows the version constraints used when installing.

```console
$ uv tool install black<24.3.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.2.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool list --show-version-specifiers
success: true
exit_code: 0
----- stdout -----
black v24.2.0 [required: <24.3.0]
- black
- blackd

----- stderr -----
```

## Listing with extras

<!-- Derived from [`tool_list::tool_list_show_extras`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L456-L565) -->

The `--show-extras` flag shows any extras that were used when installing.

```console
$ uv tool install flask[async,dotenv]
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + asgiref==3.8.1
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + python-dotenv==1.0.1
 + werkzeug==3.0.1
Installed 1 executable: flask
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool list --show-extras
success: true
exit_code: 0
----- stdout -----
flask v3.0.2 [extras: async, dotenv]
- flask

----- stderr -----
```

## Listing with additional dependencies

<!-- Derived from [`tool_list::tool_list_show_with`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L341-L454) -->

The `--show-with` flag shows any additional dependencies installed with `--with`.

```console
$ uv tool install flask --with requests
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + blinker==1.7.0
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + click==8.1.7
 + flask==3.0.2
 + idna==3.6
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + requests==2.31.0
 + urllib3==2.2.1
 + werkzeug==3.0.1
Installed 1 executable: flask
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool list --show-with
success: true
exit_code: 0
----- stdout -----
flask v3.0.2 [with: requests]
- flask

----- stderr -----
```

## Listing with all flags

<!-- Derived from [`tool_list::tool_list_show_all`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L597-L643) -->

Multiple display flags can be combined.

```console
$ uv tool install flask[async,dotenv] --with requests
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + asgiref==3.8.1
 + blinker==1.7.0
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + click==8.1.7
 + flask==3.0.2
 + idna==3.6
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + python-dotenv==1.0.1
 + requests==2.31.0
 + urllib3==2.2.1
 + werkzeug==3.0.1
Installed 1 executable: flask
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool list --show-extras --show-with --show-version-specifiers --show-paths --show-python
success: true
exit_code: 0
----- stdout -----
flask v3.0.2 [extras: async, dotenv] [with: requests] [CPython 3.12.[X]] ([HOME]/data/uv/tools/flask)
- flask ([HOME]/data/../bin/flask)

----- stderr -----
```

## Listing with missing receipt

<!-- Derived from [`tool_list::tool_list_missing_receipt`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L118-L144) -->

Tools with missing receipts should show a warning and not be listed.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
```

```console
$ uv tool install black==24.2.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.2.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

Delete the receipt to simulate corruption:

```console
$ rm tools/black/uv-receipt.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Listing should warn about the malformed tool:

```console
$ uv tool list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Ignoring malformed tool `black` (run `uv tool uninstall black` to remove)
```

## Listing with bad environment

<!-- Derived from [`tool_list::tool_list_bad_environment`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L147-L196) -->

Tools with corrupted environments should show a warning and not be listed.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }

[filters]
python-names = true
virtualenv-bin = true
```

```console
$ uv tool install black==24.2.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.2.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../[BIN]` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../[BIN]:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool install ruff==0.3.4
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + ruff==0.3.4
Installed 1 executable: ruff
warning: `[HOME]/data/../[BIN]` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../[BIN]:$PATH"` or `uv tool update-shell`.
```

Remove the Python interpreter for black to corrupt its environment:

```console
$ rm -rf tools/black/${VENV_BIN}
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Listing should warn about the invalid environment and only show ruff:

```console
$ uv tool list
success: true
exit_code: 0
----- stdout -----
ruff v0.3.4
- ruff

----- stderr -----
warning: Invalid environment at `tools/black`: missing Python executable at `tools/black/[BIN]/[PYTHON]` (run `uv tool install black --reinstall` to reinstall)
```

## Listing with deprecated receipt format

<!-- Derived from [`tool_list::tool_list_deprecated`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_list.rs#L197-L281) -->

Tools with legacy receipt formats are still listed correctly, but invalid receipts show a warning.

First, install black normally:

```console
$ uv tool install black==24.2.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.2.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

Replace with a legacy receipt format (requirements as array of strings):

```toml file="tools/black/uv-receipt.toml"

[tool]
requirements = ["black==24.2.0"]
entrypoints = [
    { name = "black", install-path = "[TEMP_DIR]/bin/black", from = "black" },
    { name = "blackd", install-path = "[TEMP_DIR]/bin/blackd", from = "black" },
]
```

Listing still works with the legacy format:

```console
$ uv tool list
success: true
exit_code: 0
----- stdout -----
black v24.2.0
- black
- blackd

----- stderr -----
```

Replace with an invalid receipt (bad version specifier):

```toml file="tools/black/uv-receipt.toml"

[tool]
requirements = ["black<>24.2.0"]
entrypoints = [
    { name = "black", install-path = "[TEMP_DIR]/bin/black", from = "black" },
    { name = "blackd", install-path = "[TEMP_DIR]/bin/blackd", from = "black" },
]
```

Invalid receipts are ignored with a warning:

```console
$ uv tool list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Ignoring malformed tool `black` (run `uv tool uninstall black` to remove)
```
