# Tool Uninstall

Tests for `uv tool uninstall` to remove installed tools.

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false

[filters]
exe-suffix = true
counts = true
```

## Uninstalling a tool

<!-- Derived from [`tool_uninstall::tool_uninstall`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_uninstall.rs#L8-L67) -->

Install a tool, then uninstall it.

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
$ uv tool uninstall black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 2 executables: black, blackd
```

After uninstalling, the tool shouldn't be listed.

```console
$ uv tool list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
No tools installed
```

## Uninstalling multiple tools

<!-- Derived from [`tool_uninstall::tool_uninstall_multiple_names`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_uninstall.rs#L69-L114) -->

Multiple tools can be uninstalled at once.

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
$ uv tool install ruff
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + ruff==0.3.4
Installed 1 executable: ruff
warning: `[HOME]/data/../bin` is not on your PATH. To use installed tools, run `export PATH="[HOME]/data/../bin:$PATH"` or `uv tool update-shell`.
```

```console
$ uv tool uninstall black ruff
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled 3 executables: black, blackd, ruff
```

After uninstalling, no tools should be listed.

```console
$ uv tool list
success: true
exit_code: 0
----- stdout -----

----- stderr -----
No tools installed
```

## Uninstalling a tool that is not installed

<!-- Derived from [`tool_uninstall::tool_uninstall_not_installed`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_uninstall.rs#L116-L132) -->

Uninstalling a tool that is not installed fails with an error.

```console
$ uv tool uninstall black
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `black` is not installed
```

## Uninstalling tool with missing receipt

<!-- Derived from [`tool_uninstall::tool_uninstall_missing_receipt`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_uninstall.rs#L134-L161) -->

If a tool's receipt is missing (corrupted state), uninstall should still clean up the environment.

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

Uninstall should handle the missing receipt gracefully:

```console
$ uv tool uninstall black
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Removed dangling environment for `black`
```

## Uninstalling all tools with missing receipt

<!-- Derived from [`tool_uninstall::tool_uninstall_all_missing_receipt`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_uninstall.rs#L163-L190) -->

Using `--all` should also handle tools with missing receipts.

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

Delete the receipt:

```console
$ rm tools/black/uv-receipt.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Uninstall all should clean up the dangling environment:

```console
$ uv tool uninstall --all
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Removed dangling environment for `black`
```
