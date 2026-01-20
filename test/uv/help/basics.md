# Help Basics

Tests for `uv help` command.

```toml
# mdtest

[environment]
python-versions = []
create-venv = false
```

## Help errors

### Version flag with help

<!-- from help.rs::help_with_version -->

Using `--version` with `uv help` suggests the correct flag.

```console
$ uv help --version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: unexpected argument '--version' found

  tip: a similar argument exists: '--verbose'

Usage: uv help --verbose... [COMMAND]...

For more information, try '--help'.
```

### Unknown subcommand

<!-- from help.rs::help_unknown_subcommand -->

Using `uv help` with an unknown subcommand suggests valid commands.

```console
$ uv help foobar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: There is no command `foobar` for `uv`. Did you mean one of:
    auth
    run
    init
    add
    remove
    version
    sync
    lock
    export
    tree
    format
    tool
    python
    pip
    venv
    build
    publish
    cache
    self
    generate-shell-completion
```

### Unknown subsubcommand

<!-- from help.rs::help_unknown_subsubcommand -->

Using `uv help` with an unknown subsubcommand suggests valid commands.

```console
$ uv help python foobar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: There is no command `foobar` for `uv python`. Did you mean one of:
    list
    install
    upgrade
    find
    pin
    dir
    uninstall
    update-shell
```

### Help with help flag

<!-- from help.rs::help_with_help -->

Using `uv help --help` shows the help command's help.

```console
$ uv help --help
success: true
exit_code: 0
----- stdout -----
Display documentation for a command

Usage: uv help [OPTIONS] [COMMAND]...

Options:
  --no-pager Disable pager when printing help

----- stderr -----
```

<!--
Note: The help_with_global_option test has very long output that changes frequently.
It's not practical to include in mdtest. The test just verifies global options work before `help`.
-->
