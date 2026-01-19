# Tool Directory

Tests for `uv tool dir` to display tool and bin directory paths.

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false
```

## Displaying the tool directory

<!-- Derived from [`tool_dir::tool_dir`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_dir.rs#L7-L23) -->

The `uv tool dir` command shows the configured tool directory.

```console
$ uv tool dir
success: true
exit_code: 0
----- stdout -----
[HOME]/data/uv/tools

----- stderr -----
```

## Displaying the bin directory

<!-- Derived from [`tool_dir::tool_dir_bin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/tool_dir.rs#L25-L41) -->

The `--bin` flag shows the bin directory where tool executables are installed.

```console
$ uv tool dir --bin
success: true
exit_code: 0
----- stdout -----
[HOME]/data/../bin

----- stderr -----
```
