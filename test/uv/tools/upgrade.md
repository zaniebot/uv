# Tool Upgrade

Tests for `uv tool upgrade` to upgrade installed tools.

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false

[filters]
exe-suffix = true
counts = true
```

## Upgrading when nothing is installed

<!-- Derived from [`tool_upgrade::tool_upgrade_empty`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L8-L91) -->

When no tools are installed, `uv tool upgrade --all` reports nothing to upgrade.

```console
$ uv tool upgrade --all
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Nothing to upgrade
```

## Upgrading a non-existing package

<!-- Derived from [`tool_upgrade::tool_upgrade_non_existing_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L395-L431) -->

Attempting to upgrade a tool that is not installed fails with an error.

```console
$ uv tool upgrade black
success: false
exit_code: 1
----- stdout -----

----- stderr -----
error: Failed to upgrade black
  Caused by: `black` is not installed; run `uv tool install black` to install
```

## Upgrading with pinned version shows hint

<!-- Derived from [`tool_upgrade::tool_upgrade_pinned_hint`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L218-L267) -->

When a tool is installed with an exact version pin, upgrade shows a hint about reinstalling.

```console
$ uv tool install black==24.3.0
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
$ uv tool upgrade black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Nothing to upgrade

hint: `black` is pinned to `24.3.0` (installed with an exact version pin); reinstall with `uv tool install black@latest` to upgrade to a new version.
```

## Upgrading a tool by name

<!-- Derived from [`tool_upgrade::tool_upgrade_name`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L93-L141) -->

Install an outdated version from Test PyPI, then upgrade from PyPI.

```console
$ uv tool install babel --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade babel --index-url https://pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Updated babel v2.6.0 -> v2.14.0
 - babel==2.6.0
 + babel==2.14.0
 - pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Upgrading all tools

<!-- Derived from [`tool_upgrade::tool_upgrade_all`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L321-L393) -->

Install outdated versions, then upgrade all at once.

```console
$ uv tool install python-dotenv --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + python-dotenv==0.10.2.post2
Installed 1 executable: dotenv
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool install babel --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade --all --index-url https://pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Updated babel v2.6.0 -> v2.14.0
 - babel==2.6.0
 + babel==2.14.0
 - pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
Updated python-dotenv v0.10.2.post2 -> v1.0.1
 - python-dotenv==0.10.2.post2
 + python-dotenv==1.0.1
Installed 1 executable: dotenv
```

## Upgrading respects constraints

<!-- Derived from [`tool_upgrade::tool_upgrade_respect_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L578-L627) -->

When a tool is installed with a version constraint, upgrade respects it.

```console
$ uv tool install babel<2.10 --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade babel --index-url https://pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Updated babel v2.6.0 -> v2.9.1
 - babel==2.6.0
 + babel==2.9.1
 - pytz==2018.5
 + pytz==2024.1
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Upgrading multiple tools by name

<!-- Derived from [`tool_upgrade::tool_upgrade_multiple_names`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L143-L216) -->

Multiple tools can be upgraded at once by passing multiple names.

```console
$ uv tool install python-dotenv --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + python-dotenv==0.10.2.post2
Installed 1 executable: dotenv
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool install babel --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade babel python-dotenv --index-url https://pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Updated babel v2.6.0 -> v2.14.0
 - babel==2.6.0
 + babel==2.14.0
 - pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
Updated python-dotenv v0.10.2.post2 -> v1.0.1
 - python-dotenv==0.10.2.post2
 + python-dotenv==1.0.1
Installed 1 executable: dotenv
```

## Upgrading with inline constraint

<!-- Derived from [`tool_upgrade::tool_upgrade_constraint`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L629-L740) -->

An inline constraint can be applied during upgrade.

```console
$ uv tool install babel --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade babel<2.12.0 --index-url https://pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Updated babel v2.6.0 -> v2.11.0
 - babel==2.6.0
 + babel==2.11.0
 - pytz==2018.5
 + pytz==2024.1
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Upgrading with mixed constraint shows pinned hint

<!-- Derived from [`tool_upgrade::tool_upgrade_pinned_hint_with_mixed_constraint`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L270-L319) -->

When a tool is installed with a mixed constraint that includes an exact pin (e.g., `>=2.0,==2.6.0`),
upgrading shows the pinned hint while still upgrading dependencies.

```console
$ uv tool install "babel>=2.0,==2.6.0" --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade babel --index-url https://pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Modified babel environment
 - pytz==2018.5
 + pytz==2024.1

hint: `babel` is pinned to `2.6.0` (installed with an exact version pin); reinstall with `uv tool install babel@latest` to upgrade to a new version.
```

## Upgrading pinned tool updates dependencies

<!-- Derived from [`tool_upgrade::tool_upgrade_with`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L745-L792) -->

When upgrading a tool installed with an exact version pin, the tool stays pinned but its
dependencies can be upgraded.

```console
$ uv tool install babel==2.6.0 --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade babel --index-url https://pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Modified babel environment
 - pytz==2018.5
 + pytz==2024.1

hint: `babel` is pinned to `2.6.0` (installed with an exact version pin); reinstall with `uv tool install babel@latest` to upgrade to a new version.
```

## Upgrading preserves resolution settings

<!-- Derived from [`tool_upgrade::tool_upgrade_settings`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L514-L576) -->

When a tool is installed with resolution settings, they are preserved during upgrade.

Install `black` with `lowest-direct` resolution:

```console
$ uv tool install black>=23 --resolution=lowest-direct
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==23.1.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

Upgrading preserves the resolution setting, so this is a no-op:

```console
$ uv tool upgrade black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Nothing to upgrade
```

Override the resolution to actually upgrade:

```console
$ uv tool upgrade black --resolution=highest
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Updated black v23.1.0 -> v24.3.0
 - black==23.1.0
 + black==24.3.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Upgrading tool to different Python version

<!-- Derived from [`tool_upgrade::tool_upgrade_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L794-L851) -->

A tool can be upgraded to use a different Python version.

```toml
# mdtest
[environment]
python-versions = ["3.11", "3.12"]
```

```console
$ uv tool install babel==2.6.0 --index-url https://test.pypi.org/simple/ --python 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade babel --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
Upgraded tool environment for `babel` to Python 3.12
```

## Upgrading all tools to different Python version

<!-- Derived from [`tool_upgrade::tool_upgrade_python_with_all`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L853-L942) -->

All tools can be upgraded to use a different Python version with `--all`.

```toml
# mdtest
[environment]
python-versions = ["3.11", "3.12"]
```

```console
$ uv tool install babel==2.6.0 --index-url https://test.pypi.org/simple/ --python 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool install python-dotenv --index-url https://test.pypi.org/simple/ --python 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + python-dotenv==0.10.2.post2
Installed 1 executable: dotenv
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade --all --python 3.12
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + python-dotenv==0.10.2.post2
Installed 1 executable: dotenv
Upgraded tool environments for `babel` and `python-dotenv` to Python 3.12
```

## Upgrading tool with additional entrypoints to different Python

<!-- Derived from [`tool_upgrade::test_tool_upgrade_additional_entrypoints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L947-L1010) -->

When upgrading a tool that was installed with `--with-executables-from`, all entrypoints are
re-installed.

```toml
# mdtest
[environment]
python-versions = ["3.11", "3.12"]
```

```console
$ uv tool install --python 3.11 --with-executables-from black babel==2.14.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.14.0
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables from `black`: black, blackd
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool upgrade --python 3.12 babel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.14.0
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables from `black`: black, blackd
Installed 1 executable: pybabel
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
Upgraded tool environment for `babel` to Python 3.12
```

## Continuing upgrade when one tool fails

<!-- Derived from [`tool_upgrade::tool_upgrade_not_stop_if_upgrade_fails`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_upgrade.rs#L434-L511) -->

When upgrading multiple tools with `--all`, successful upgrades are completed even if one tool fails.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }

[filters]
counts = true
exe-suffix = true
```

Install two tools from Test PyPI:

```console
$ uv tool install python-dotenv --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + python-dotenv==0.10.2.post2
Installed 1 executable: dotenv
```

```console
$ uv tool install babel --index-url https://test.pypi.org/simple/
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + babel==2.6.0
 + pytz==2018.5
Installed 1 executable: pybabel
```

Break the receipt for python-dotenv:

```console
$ echo "Invalid receipt" > tools/python-dotenv/uv-receipt.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Upgrading all tools continues despite the failure:

```console
$ uv tool upgrade --all --index-url https://pypi.org/simple/
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Updated babel v2.6.0 -> v2.14.0
 - babel==2.6.0
 + babel==2.14.0
 - pytz==2018.5
Installed 1 executable: pybabel
error: Failed to upgrade python-dotenv
  Caused by: `python-dotenv` is missing a valid receipt; run `uv tool install --force python-dotenv` to reinstall
```
