# Tool Install

Tests for `uv tool install` to install Python tools.

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false

[filters]
exe-suffix = true
counts = true
```

## Installing a tool

<!-- Derived from [`tool_install::tool_install`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L18-L180) -->

Install a tool and verify it's available.

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

## Already installed tool

<!-- Derived from [`tool_install::tool_install_already_installed`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1039-L1199) -->

Installing a tool that is already installed reports it's already installed.

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
$ uv tool install black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
`black` is already installed
```

## Upgrading a tool

<!-- Derived from [`tool_install::tool_install_upgrade`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2679-L2781) -->

Installing a tool with different requirements upgrades the existing installation.

First, install black with a specific version:

```console
$ uv tool install black==24.1.1
success: true
exit_code: 0
----- stdout -----

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
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

The receipt should contain the version constraint:

```toml file="tools/black/uv-receipt.toml"
[tool]
requirements = [{ name = "black", specifier = "==24.1.1" }]
entrypoints = [
    { name = "black", install-path = "[TEMP_DIR]/bin/black", from = "black" },
    { name = "blackd", install-path = "[TEMP_DIR]/bin/blackd", from = "black" },
]

[tool.options]
exclude-newer = "2024-03-25T00:00:00Z"
```

Install again without the version constraint. The receipt is updated but packages aren't reinstalled
since the existing version satisfies the new requirement:

```console
$ uv tool install black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Audited [N] packages in [TIME]
Installed 2 executables: black, blackd
```

The receipt should no longer contain the version specifier:

```toml file="tools/black/uv-receipt.toml"
[tool]
requirements = [{ name = "black" }]
entrypoints = [
    { name = "black", install-path = "[TEMP_DIR]/bin/black", from = "black" },
    { name = "blackd", install-path = "[TEMP_DIR]/bin/blackd", from = "black" },
]

[tool.options]
exclude-newer = "2024-03-25T00:00:00Z"
```

Install again with `--with` to add an additional dependency:

```console
$ uv tool install black --with iniconfig@https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0 (from https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl)
Installed 2 executables: black, blackd
```

## Installing with --from and @latest

<!-- Derived from [`tool_install::tool_install_from_at_latest`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3580-L3623) -->

Install a tool using `--from package@latest` to get the latest version.

```console
$ uv tool install app --from executable-application@latest
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + executable-application==0.3.0
Installed 1 executable: app
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with --from and @version

<!-- Derived from [`tool_install::tool_install_from_at_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3625-L3668) -->

Install a specific version using `--from package@version`.

```console
$ uv tool install app --from executable-application@0.2.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + executable-application==0.2.0
Installed 1 executable: app
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with --from errors

<!-- Derived from [`tool_install::tool_install_from`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L974-L1037) -->

Using `--from` with a different package name fails.

```console
$ uv tool install black --from flask==24.2.0
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package name (`flask`) provided with `--from` does not match install request (`black`)
```

Using `--from` with a conflicting version fails.

```console
$ uv tool install black==24.2.0 --from black==24.3.0
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package requirement (`black==24.3.0`) provided with `--from` conflicts with install request (`black==24.2.0`)
```

## Installing with unnamed URL using --from

<!-- Derived from [`tool_install::tool_install_unnamed_from`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2138-L2226) -->

Install from a bare URL using `--from`.

```console
$ uv tool install black --from https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.4.2 (from https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl)
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with unnamed URL using --with

<!-- Derived from [`tool_install::tool_install_unnamed_with`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2228-L2318) -->

Install with an additional dependency from a bare URL.

```console
$ uv tool install black --with https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + iniconfig==2.0.0 (from https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl)
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing bare URL package

<!-- Derived from [`tool_install::tool_install_unnamed_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1734-L1829) -->

Install a tool directly from a bare URL without specifying a name.

```console
$ uv tool install https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.4.2 (from https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl)
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with --from name conflict

<!-- Derived from [`tool_install::tool_install_unnamed_conflict`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2114-L2136) -->

Using `--from` with a URL that provides a different package name fails.

```console
$ uv tool install black --from https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package name (`iniconfig`) provided with `--from` does not match install request (`black`)
```

## Installing a package without executables

<!-- Derived from [`tool_install::tool_install_no_entrypoints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs) -->

Installing a package that provides no executables fails.

```console
$ uv tool install iniconfig
success: false
exit_code: 2
----- stdout -----
No executables are provided by package `iniconfig`; removing tool

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
error: Failed to install entrypoints for `iniconfig`
```

## Installing with version specifier

<!-- Derived from [`tool_install::tool_install_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L505-L584) -->

Install a tool with a specific version constraint.

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

## Installing with @version syntax

<!-- Derived from [`tool_install::tool_install_at_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3467-L3529) -->

Install a tool using the `package@version` syntax.

```console
$ uv tool install black@24.1.0
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
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

Combining `package@version` with `--from` is an error.

```console
$ uv tool install black@24.1.0 --from black==24.1.0
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package requirement (`black==24.1.0`) provided with `--from` conflicts with install request (`black@24.1.0`)
```

## Installing with @latest syntax

<!-- Derived from [`tool_install::tool_install_at_latest`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3532-L3600) -->

Install the latest version of a tool using the `package@latest` syntax.

```console
$ uv tool install black@latest
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

## Upgrading with @latest syntax

<!-- Derived from [`tool_install::tool_install_at_latest_upgrade`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3670-L3789) -->

Using `package@latest` forces an upgrade to the latest version even if a tool is already installed.

First, install an older version of black:

```console
$ uv tool install black==24.1.1
success: true
exit_code: 0
----- stdout -----

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
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

Installing without a version constraint doesn't upgrade since the existing version satisfies the
requirement:

```console
$ uv tool install black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Audited [N] packages in [TIME]
Installed 2 executables: black, blackd
```

But using `@latest` forces an upgrade to the latest version:

```console
$ uv tool install black@latest
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Uninstalled [N] packages in [TIME]
Installed [N] packages in [TIME]
 - black==24.1.1
 + black==24.3.0
Installed 2 executables: black, blackd
```

## Installing with --with flag

<!-- Derived from [`tool_install::tool_install_unnamed_with`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2230-L2320) -->

Install a tool with additional dependencies using `--with`.

```console
$ uv tool install black --with iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + iniconfig==2.0.0
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with extras

<!-- Derived from [`tool_install::tool_install`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs) -->

Install a tool with extras.

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

## Installing with reinstall flag

<!-- Derived from [`tool_install::tool_install_upgrade`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2681-L2780) -->

Install with `--reinstall` to force reinstallation.

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

```console
$ uv tool install black --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Uninstalled [N] packages in [TIME]
Installed [N] packages in [TIME]
 - black==24.2.0
 + black==24.3.0
 ~ click==8.1.7
 ~ mypy-extensions==1.0.0
 ~ packaging==24.0
 ~ pathspec==0.12.1
 ~ platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with constraints file

<!-- Derived from [`tool_install::tool_install_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3793-L3896) -->

Install a tool with a constraints file limiting dependency versions.

```text
# file: constraints.txt

mypy-extensions<1
```

```console
$ uv tool install black --constraints constraints.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==0.4.4
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

Installing with the same constraints again reports already installed:

```console
$ uv tool install black --constraints constraints.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
`black` is already installed
```

## Installing with mismatched package name from URL

<!-- Derived from [`tool_install::tool_install_mismatched_name`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L4001-L4052) -->

When using `--from` with a URL, the package name must match.

```console
$ uv tool install black --from https://files.pythonhosted.org/packages/af/47/93213ee66ef8fae3b93b3e29206f6b251e65c97bd91d8e1c5596ef15af0a/flask-3.1.0-py3-none-any.whl
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package name (`flask`) provided with `--from` does not match install request (`black`)
```

```console
$ uv tool install black --from "flask @ https://files.pythonhosted.org/packages/af/47/93213ee66ef8fae3b93b3e29206f6b251e65c97bd91d8e1c5596ef15af0a/flask-3.1.0-py3-none-any.whl"
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package name (`flask`) provided with `--from` does not match install request (`black`)
```

```console
$ uv tool install flask --from "black @ https://files.pythonhosted.org/packages/af/47/93213ee66ef8fae3b93b3e29206f6b251e65c97bd91d8e1c5596ef15af0a/flask-3.1.0-py3-none-any.whl"
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Package name (`black`) provided with `--from` does not match install request (`flask`)
```

## Installing with resolution settings

<!-- Derived from [`tool_install::tool_install_settings`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3313-L3464) -->

Install a tool with resolution settings that affect which versions are selected.

```console
$ uv tool install flask>=3 --resolution=lowest-direct
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
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing from Git repository

<!-- Derived from [`tool_install::tool_install_git`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1825-L1915) -->

Install a tool directly from a Git repository.

```toml
# mdtest
[environment]
required-features = "git"
```

```console
$ uv tool install git+https://github.com/psf/black@24.2.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.2.0 (from git+https://github.com/psf/black@6fdf8a4af28071ed1d079c01122b34c5d587207a)
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing from Git repository with LFS

<!-- Derived from [`tool_install::tool_install_git_lfs`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1920-L2013) -->

Install a tool from a Git repository that uses Git LFS for large files.

```toml
# mdtest

[environment]
python-version = "3.13"
required-features = "git"
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }

[filters]
exe-suffix = true
```

```console
$ uv tool install --lfs test-lfs-repo @ git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo@c6d77ab63d91104f32ab5e5ae2943f4d26ff875f#lfs=true)
Installed 2 executables: test-lfs-repo, test-lfs-repo-assets
```

The receipt includes the LFS flag:

```console
$ cat tools/test-lfs-repo/uv-receipt.toml
success: true
exit_code: 0
----- stdout -----
[tool]
requirements = [{ name = "test-lfs-repo", git = "https://github.com/astral-sh/test-lfs-repo?lfs=true&rev=c6d77ab63d91104f32ab5e5ae2943f4d26ff875f" }]
entrypoints = [
    { name = "test-lfs-repo", install-path = "[TEMP_DIR]/bin/test-lfs-repo", from = "test-lfs-repo" },
    { name = "test-lfs-repo-assets", install-path = "[TEMP_DIR]/bin/test-lfs-repo-assets", from = "test-lfs-repo" },
]

[tool.options]
exclude-newer = "[EXCLUDE_NEWER]"

----- stderr -----
```

Verify the executables work and can access LFS assets:

```console
$ test-lfs-repo
success: true
exit_code: 0
----- stdout -----
Hello from test-lfs-repo!

----- stderr -----
```

```console
$ test-lfs-repo-assets
success: true
exit_code: 0
----- stdout -----
Hello from test-lfs-repo! LFS_TEST=True ANOTHER_LFS_TEST=True

----- stderr -----
```

## Installing with existing executable (force required)

<!-- Derived from [`tool_install::tool_install_force`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1226-L1496) -->

When an executable already exists at the target path, installation fails unless `--force` is used.

```toml
# mdtest
[environment]
env = { UV_TOOL_BIN_DIR = "./bin" }
```

Create an empty file to simulate an existing executable:

```tree create=true
.
└── bin/
    └── black
```

```console
$ uv tool install black
success: false
exit_code: 2
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
error: Executable already exists: black (use `--force` to overwrite)
```

Using `--force` overwrites the existing executable:

```console
$ uv tool install black --force
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `./bin` is not on your PATH. To use installed tools, run `export PATH="./bin:$PATH"` or `uv tool update-shell`.
```

## Installing an editable package

<!-- Derived from [`tool_install::tool_install_editable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L593-L750) -->

Install a tool from a local editable package.

```console
$ uv tool install -e ${WORKSPACE}/test/packages/black_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==0.1.0 (from file://[WORKSPACE]/test/packages/black_editable)
Installed 1 executable: black
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with --with-editable

<!-- Derived from [`tool_install::tool_install_with_editable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L288-L327) -->

Install a tool with additional editable dependencies using `--with-editable`.

```console
$ uv tool install executable-application --with-editable ${WORKSPACE}/test/packages/anyio_local --with iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + anyio==4.3.0+foo (from file://[WORKSPACE]/test/packages/anyio_local)
 + executable-application==0.3.0
 + iniconfig==2.0.0
Installed 1 executable: app
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing editable with --from

<!-- Derived from [`tool_install::tool_install_editable_from`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L890-L971) -->

Install a tool as editable using `-e` with `--from`.

```console
$ uv tool install black -e --from ${WORKSPACE}/test/packages/black_editable
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==0.1.0 (from file://[WORKSPACE]/test/packages/black_editable)
Installed 1 executable: black
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with overrides

<!-- Derived from [`tool_install::tool_install_overrides`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3898-L3964) -->

Use `--overrides` to force specific dependency versions.

```toml file="overrides.txt"

click<8
anyio>=3
```

```console
$ uv tool install black --overrides overrides.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==7.1.2
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with compatible build constraints

<!-- Derived from [`tool_install::tool_install_with_compatible_build_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L329-L397) -->

Use `--build-constraints` to specify build-time dependency requirements.

```toml
# mdtest

[environment]
python-version = "3.9"
exclude-newer = "2024-05-04T00:00:00Z"
```

```txt file="build_constraints.txt"
setuptools>=40
```

```console
$ uv tool install black --with requests==1.2 --build-constraints build_constraints.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.4.2
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.1
 + requests==1.2.0
 + tomli==2.0.1
 + typing-extensions==4.11.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with incompatible build constraints

<!-- Derived from [`tool_install::tool_install_with_incompatible_build_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L399-L437) -->

Build constraints that conflict with package requirements cause installation to fail.

```toml
# mdtest

[environment]
python-version = "3.9"
exclude-newer = "2024-05-04T00:00:00Z"
```

```txt file="build_constraints.txt"
setuptools==2
```

```console
$ uv tool install black --with requests==1.2 --build-constraints build_constraints.txt
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to download and build `requests==1.2.0`
  ├─▶ Failed to resolve requirements from `setup.py` build
  ├─▶ No solution found when resolving: `setuptools>=40.8.0`
  ╰─▶ Because you require setuptools>=40.8.0 and setuptools==2, we can conclude that your requirements are unsatisfiable.
```

## Installing with requirements file

<!-- Derived from [`tool_install::tool_install_requirements_txt`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2447-L2549) -->

Use `--with-requirements` to install additional dependencies from a requirements file.

```txt file="requirements.txt"
iniconfig
```

```console
$ uv tool install black --with-requirements requirements.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + iniconfig==2.0.0
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with requirements file arguments

<!-- Derived from [`tool_install::tool_install_requirements_txt_arguments`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2551-L2629) -->

Arguments like `--index-url` in requirements files are ignored with a warning.

```txt file="requirements.txt"

--index-url https://test.pypi.org/simple
idna
```

```console
$ uv tool install black --with-requirements requirements.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Ignoring `--index-url` from requirements file: `https://test.pypi.org/simple`. Instead, use the `--index-url` command-line argument, or set `index-url` in a `uv.toml` or `pyproject.toml` file.
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + idna==3.6
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with dependencies from script

<!-- Derived from [`tool_install::tool_install_with_dependencies_from_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2321-L2445) -->

Install a tool with additional dependencies extracted from a Python script's inline metadata.

```python file="script.py"

# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "anyio",
# ]
# ///

import anyio
```

```console
$ uv tool install black --with-requirements script.py
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
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with global Python version

<!-- Derived from [`tool_install::tool_install_with_global_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L181-L285) -->

Tools respect the global Python version from configuration.

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
```

Set the global Python version:

```txt file="${HOME}/.config/uv/.python-version"
3.11
```

Tools are installed using the configured Python version:

```console
$ uv tool install flask
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + blinker==1.7.0
 + click==8.1.7
 + flask==3.0.2
 + itsdangerous==2.1.2
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + werkzeug==3.0.1
Installed 1 executable: flask
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

## Installing with Python version flag

<!-- Derived from [`tool_install::tool_install_python_requests`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2843-L2920) -->

Use `-p` or `--python` to specify the Python version for tool installation.

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
```

Install with Python 3.12:

```console
$ uv tool install -p 3.12 black
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

Reinstalling with the same Python version is a no-op:

```console
$ uv tool install -p 3.12 black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
`black` is already installed
```

Changing the Python version reinstalls the tool:

```console
$ uv tool install -p 3.11 black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Uninstalled [N] packages in [TIME]
Installed [N] packages in [TIME]
 - black==24.3.0
 + black==24.3.0
Installed 2 executables: black, blackd
```

## Installing with Python preference

<!-- Derived from [`tool_install::tool_install_python_preference`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L2924-L3062) -->

The `--python-preference` flag controls whether uv prefers system or managed Python installations.
When switching between different Python sources (system vs managed), the tool is reinstalled.

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
managed-python-versions = ["3.11"]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }

[filters]
counts = true
exe-suffix = true
```

Install with Python 3.12 (uses default preference):

```console
$ uv tool install -p 3.12 black
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
```

Reinstalling with the same Python version is a no-op:

```console
$ uv tool install -p 3.12 black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
`black` is already installed
```

Installing with Python 3.11 and `only-system` reinstalls due to incompatible Python version:

```console
$ uv tool install -p 3.11 --python-preference only-system black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Ignoring existing environment for `black`: the requested Python interpreter does not match the environment interpreter
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
```

Reinstalling with the same Python 3.11 `only-system` is a no-op:

```console
$ uv tool install -p 3.11 --python-preference only-system black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
`black` is already installed
```

Installing with Python 3.11 and `only-managed` reinstalls due to different Python source:

```console
$ uv tool install -p 3.11 --python-preference only-managed black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Ignoring existing environment for `black`: the requested Python interpreter does not match the environment interpreter
Resolved [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
```

Reinstalling with the same Python 3.11 `only-managed` is a no-op:

```console
$ uv tool install -p 3.11 --python-preference only-managed black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
`black` is already installed
```

## Installing with Python platform

<!-- Derived from [`tool_install::tool_install_python_platform`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L4498-L4560) -->

Use `--python-platform` to install for a specific target platform.

Install for macOS:

```console
$ uv tool install black --python-platform macos
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

Changing the platform reinstalls:

```console
$ uv tool install black --python-platform linux
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Uninstalled [N] packages in [TIME]
Installed [N] packages in [TIME]
 - black==24.3.0
 + black==24.3.0
 - click==8.1.7
 + click==8.1.7
 - mypy-extensions==1.0.0
 + mypy-extensions==1.0.0
 - packaging==24.0
 + packaging==24.0
 - pathspec==0.12.1
 + pathspec==0.12.1
 - platformdirs==4.2.0
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
```

## Installing Python itself is not allowed

<!-- Derived from [`tool_install::tool_install_python`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3962-L3998) -->

Installing Python as a tool is not permitted.

```console
$ uv tool install python
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Cannot install Python with `uv tool install`. Did you mean to use `uv python install`?
```

Even with a version specifier:

```console
$ uv tool install python@3.12
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Cannot install Python with `uv tool install`. Did you mean to use `uv python install`?
```

## Installing with HOME directory

<!-- Derived from [`tool_install::tool_install_home`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1496-L1536) -->

Tools install to `$HOME/.local/bin` on Unix systems.

```toml
# mdtest

[environment]
target-family = "unix"
```

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
warning: `[HOME]/.local/bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/.local/bin:$PATH"` or `uv tool update-shell`.
```

## Installing with XDG_DATA_HOME

<!-- Derived from [`tool_install::tool_install_xdg_data_home`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1538-L1573) -->

The bin directory is inferred from `$XDG_DATA_HOME`.

```console
$ XDG_DATA_HOME="${TEMP_DIR}/data/home" uv tool install black
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
warning: `[TEMP_DIR]/data/bin` is not on your PATH. To use installed tools, run `export PATH="[TEMP_DIR]/data/bin:$PATH"` or `uv tool update-shell`.
```

## Installing with XDG_BIN_HOME

<!-- Derived from [`tool_install::tool_install_xdg_bin_home`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1575-L1608) -->

The bin directory can be set with `$XDG_BIN_HOME`.

```console
$ XDG_BIN_HOME="${TEMP_DIR}/bin" uv tool install black
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
warning: `[TEMP_DIR]/bin` is not on your PATH. To use installed tools, run `export PATH="[TEMP_DIR]/bin:$PATH"` or `uv tool update-shell`.
```

## Installing with UV_TOOL_BIN_DIR

<!-- Derived from [`tool_install::tool_install_tool_bin_dir`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1610-L1643) -->

The bin directory can be set with `$UV_TOOL_BIN_DIR`.

```console
$ UV_TOOL_BIN_DIR="${TEMP_DIR}/bin" uv tool install black
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
warning: `[TEMP_DIR]/bin` is not on your PATH. To use installed tools, run `export PATH="[TEMP_DIR]/bin:$PATH"` or `uv tool update-shell`.
```

## Installing with preserved environment

<!-- Derived from [`tool_install::tool_install_preserve_environment`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3064-L3128) -->

When new incompatible requirements are requested, the existing environment is preserved with a
warning.

First, install black:

```console
$ uv tool install black==24.1.1
success: true
exit_code: 0
----- stdout -----

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
Installed 2 executables: black, blackd
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

Attempting to install with an incompatible requirement preserves the environment:

```console
$ uv tool install black==24.1.1 --with packaging==0.0.1
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
error: Failed to prepare distributions
  Caused by: Failed to fetch wheel: packaging==0.0.1
  Caused by: Failed to download and build: `packaging==0.0.1`
  Caused by: Failed to download: `packaging==0.0.1`
  Caused by: Package is unavailable: packaging==0.0.1
```

## Installing with PATH warning

<!-- Derived from [`tool_install::tool_install_warn_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3130-L3162) -->

When the bin directory is not in PATH, uv shows a warning.

```toml
# mdtest

[environment]
target-family = "unix"
```

```console
$ uv tool install black==24.1.1
success: true
exit_code: 0
----- stdout -----

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
Installed 2 executables: black, blackd
warning: `[TEMP_DIR]/bin` is not on your PATH. To use installed tools, run `export PATH="[TEMP_DIR]/bin:$PATH"` or `uv tool update-shell`.
```

## Installing with executables from multiple packages

<!-- Derived from [`tool_install::tool_install_with_executables_from`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L4258-L4344) -->

The `--with-executables-from` flag allows installing executables from additional packages in the
same environment.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }

[filters]
counts = true
exe-suffix = true
```

```console
$ uv tool install --with-executables-from ansible-core,black ansible==9.3.0
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + ansible==9.3.0
 + ansible-core==2.16.4
 + black==24.3.0
 + cffi==1.16.0
 + click==8.1.7
 + cryptography==42.0.5
 + jinja2==3.1.3
 + markupsafe==2.1.5
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
 + pycparser==2.21
 + pyyaml==6.0.1
 + resolvelib==1.0.1
Installed 11 executables from `ansible-core`: ansible, ansible-config, ansible-connection, ansible-console, ansible-doc, ansible-galaxy, ansible-inventory, ansible-playbook, ansible-pull, ansible-test, ansible-vault
Installed 2 executables from `black`: black, blackd
Installed 1 executable: ansible-community
```

## Installing with executables from package without entrypoints

<!-- Derived from [`tool_install::tool_install_with_executables_from_no_entrypoints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L4348-L4387) -->

When using `--with-executables-from` with a package that has no executables, a helpful message is
shown.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }

[filters]
counts = true
exe-suffix = true
```

```console
$ uv tool install --with-executables-from requests flask
success: true
exit_code: 0
----- stdout -----
No executables are provided by package `requests`
hint: Use `--with requests` to include `requests` as a dependency without installing its executables.

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
```

## Suggesting packages with desired executable

<!-- Derived from [`tool_install::tool_install_suggest_other_packages_with_executable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L440-L520) -->

When installing a package with no executables, uv suggests packages that provide the desired
executable.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin" }
exclude-newer = "2024-05-04T00:00:00Z"

[filters]
exe-suffix = true
```

```console
$ uv tool install fastapi==0.111.0
success: false
exit_code: 2
----- stdout -----
No executables are provided by package `fastapi`; removing tool
hint: An executable with the name `fastapi` is available via dependency `fastapi-cli`.
      Did you mean `uv tool install fastapi-cli`?

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
```

## Installing from authenticated index

<!-- Derived from [`tool_install::tool_install_credentials`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L4056-L4132) -->

When installing from an authenticated index, credentials are omitted from the receipt.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }
exclude-newer = "2025-01-18T00:00:00Z"

[filters]
counts = true
exe-suffix = true
```

```console
$ uv tool install executable-application --index https://public:heron@pypi-proxy.fly.dev/basic-auth/simple
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + executable-application==0.3.0
Installed 1 executable: app
```

## Installing from authenticated default index

<!-- Derived from [`tool_install::tool_install_default_credentials`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L4136-L4231) -->

When installing with a default authenticated index configured via config file, credentials are
omitted from the receipt.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }
exclude-newer = "2025-01-18T00:00:00Z"

[filters]
counts = true
exe-suffix = true
```

First create a config file with authenticated index:

```toml file="uv.toml"
[[index]]
url = "https://public:heron@pypi-proxy.fly.dev/basic-auth/simple"
default = true
authenticate = "always"
```

```console
$ uv tool install executable-application --config-file uv.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + executable-application==0.3.0
Installed 1 executable: app
```

## Installing with find-links

<!-- Derived from [`tool_install::tool_install_find_links`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L4390-L4467) -->

The `--find-links` flag can be used to specify additional package sources.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }
python-version = "3.13"

[filters]
exe-suffix = true
```

```console
$ uv tool install --find-links ${WORKSPACE}/test/links/ basic-app
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Installed 1 package in [TIME]
 + basic-app==0.1.0
Installed 1 executable: basic-app
```

## Installing uninstallable package

<!-- Derived from [`tool_install::tool_install_uninstallable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L1681-L1732) -->

When a package cannot be built, an error is shown and no tool environment is created.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }

[filters]
exe-suffix = true
```

```console
$ uv tool install pyenv
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
  × Failed to build `pyenv==0.0.1`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `setuptools.build_meta:__legacy__.build_wheel` failed (exit status: 1)

      [stdout]
      running bdist_wheel
      running build
      installing to build/bdist.linux-x86_64/wheel
      running install

      [stderr]
      # NOTE #
      We are sorry, but this package is not installable with pip.

      Please read the installation instructions at:

      https://github.com/pyenv/pyenv#installation
      #


      hint: This usually indicates a problem with the package or the build environment.
```

## Removing tool on installation failure

<!-- Derived from [`tool_install::tool_install_remove_on_empty`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L751-L841) -->

When installing fails due to no entrypoints, any existing tool environment is removed.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "${TEMP_DIR}/tools", XDG_BIN_HOME = "${TEMP_DIR}/bin", PATH = "${TEMP_DIR}/bin" }

[filters]
exe-suffix = true
```

First install black normally:

```console
$ uv tool install black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 6 packages in [TIME]
Installed 6 packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
```

Now create a local package named black with no entrypoints:

```toml file="black/pyproject.toml"
[project]
name = "black"
version = "0.1.0"
description = "Black without any entrypoints"
authors = []
dependencies = []
requires-python = ">=3.11,<3.13"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

Installing this editable version should remove the existing tool:

```console
$ uv tool install --editable black
success: false
exit_code: 2
----- stdout -----
No executables are provided by package `black`; removing tool

----- stderr -----
Resolved 1 package in [TIME]
Installed 1 package in [TIME]
 + black==0.1.0 (from file://[TEMP_DIR]/black)
```

## Reinstalling with invalid receipt

<!-- Derived from [`tool_install::tool_install_bad_receipt`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3166-L3231) -->

When a tool has an invalid receipt, reinstalling replaces it with a valid installation.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }

[filters]
counts = true
exe-suffix = true
```

First install black:

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
```

Corrupt the receipt:

```console
$ echo "invalid" > tools/black/uv-receipt.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Reinstalling black should detect and fix the invalid receipt:

```console
$ uv tool install black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Removed existing `black` with invalid receipt
Resolved [N] packages in [TIME]
Installed [N] packages in [TIME]
 + black==24.3.0
 + click==8.1.7
 + mypy-extensions==1.0.0
 + packaging==24.0
 + pathspec==0.12.1
 + platformdirs==4.2.0
Installed 2 executables: black, blackd
```

## Installing package with malformed dist-info

<!-- Derived from [`tool_install::tool_install_malformed_dist_info`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_install.rs#L3236-L3319) -->

uv can install packages with malformed `.dist-info` directory names.

```toml
# mdtest

[environment]
env = { UV_TOOL_DIR = "tools", XDG_BIN_HOME = "bin", PATH = "bin" }
exclude-newer = "2025-01-18T00:00:00Z"

[filters]
counts = true
exe-suffix = true
```

```console
$ uv tool install executable-application
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + executable-application==0.3.0
Installed 1 executable: app
```
