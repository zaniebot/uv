# Run Basics

Tests for basic `uv run` functionality.

## Argument handling

### Arguments before command

<!-- from run.rs::run_args -->

Arguments before the command are treated as uv arguments.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv run --help python
success: true
exit_code: 0
----- stdout -----
Run a command or script

[UV RUN HELP]
```

### Arguments after command

<!-- from run.rs::run_args -->

Arguments after the command are passed to the command.

```console
$ uv run python --help
success: true
exit_code: 0
----- stdout -----
usage: [PYTHON HELP]
```

### Separator for arguments

<!-- from run.rs::run_args -->

Use `--` to separate uv arguments from command arguments.

```console
$ uv run -- python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----
Resolved 1 package in [TIME]
Audited 1 package in [TIME]
```

## No arguments

### Run without arguments

<!-- from run.rs::run_no_args -->

Running without arguments shows available commands.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv run
success: false
exit_code: 2
----- stdout -----
Provide a command or script to invoke with `uv run <command>` or `uv run <script>.py`.

The following commands are available in the environment:

- python
- python3
- python3.12

See `uv run --help` for more information.

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/)
```

## Locked and frozen modes

### Run with --locked (no lockfile)

<!-- from run.rs::run_locked -->

Running with `--locked` errors when no lockfile exists.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv run --locked -- python --version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unable to find lockfile at `uv.lock`, but `--locked` was provided. To create a lockfile, run `uv lock` or `uv sync` without the flag.
```

### Run with --locked (outdated lockfile)

<!-- from run.rs::run_locked -->

Running with `--locked` errors when the lockfile is outdated.

```console
$ uv lock
success: true
exit_code: 0
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv run --locked -- python --version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
error: The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
```

### Run with --frozen (no lockfile)

<!-- from run.rs::run_frozen -->

Running with `--frozen` errors when no lockfile exists.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv run --frozen -- python --version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unable to find lockfile at `uv.lock`, but `--frozen` was provided. To create a lockfile, run `uv lock` or `uv sync` without the flag.
```

## No sync mode

### Run with --no-sync

<!-- from run.rs::run_no_sync -->

Running with `--no-sync` skips syncing even without a lockfile.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv run --no-sync -- python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----
```

### Run with --no-sync after lock

<!-- from run.rs::run_no_sync -->

With a lockfile, `--no-sync` still skips installation.

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

```console
$ uv run --no-sync -- python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----
```

### Run with --no-sync after sync

<!-- from run.rs::run_no_sync -->

After sync, packages are available with `--no-sync`.

```console
$ uv sync
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + anyio==3.7.0
 + idna==3.6
 + project==0.1.0 (from file://[TEMP_DIR]/)
 + sniffio==1.3.1
 + typing-extensions==4.10.0
```

```console
$ uv run --no-sync -- python -c "import anyio; print(anyio.__name__)"
success: true
exit_code: 0
----- stdout -----
anyio

----- stderr -----
```

## Argument parsing

### Arguments before command

<!-- from run.rs::run_args -->

Arguments before the command are treated as uv arguments.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

Arguments before the command go to uv (--help shows uv run help):

```console
$ uv run --help python 2>&1 | head -1
success: true
exit_code: 0
----- stdout -----
Run a command or script

----- stderr -----
```

### Use double-dash separator

<!-- from run.rs::run_args -->

Use `--` to separate uv arguments from the command.

```console
$ uv run -- python --version
success: true
exit_code: 0
----- stdout -----
Python 3.12.[X]

----- stderr -----
Resolved 1 package in [TIME]
Audited 1 package in [TIME]
```

## No arguments

```toml
[environment]
target-family = "unix"
```

### Run without arguments

<!-- from run.rs::run_no_args -->

Running without specifying any arguments lists available scripts.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv run 2>&1 | head -3
success: false
exit_code: 2
----- stdout -----
Provide a command or script to invoke with `uv run <command>` or `uv run <script>.py`.

----- stderr -----
```

## Exact mode

### Run with inexact semantics (default)

<!-- from run.rs::run_exact -->

By default, `uv run` uses inexact semantics - extra packages in the environment are allowed.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["iniconfig"]
```

```console
$ uv run python -c "import iniconfig"
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

Change the dependencies to remove `iniconfig`:

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["anyio"]
```

With inexact semantics (default), both `iniconfig` and `anyio` are available:

```console
$ uv run python -c "import iniconfig; import anyio"
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + sniffio==1.3.1
```

### Run with exact semantics

<!-- from run.rs::run_exact -->

With `--exact`, packages not in dependencies are removed from the environment.

```console
$ uv run --exact python -c "import iniconfig"
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Uninstalled 1 package in [TIME]
 - iniconfig==2.0.0
Traceback (most recent call last):
  File "<string>", line 1, in <module>
ModuleNotFoundError: No module named 'iniconfig'
```

## Running packages

### Run a directory as package

<!-- from run.rs::run_package -->

Running `.` executes the `__main__.py` file in the current directory.

```python __main__.py
print("Hello, world!")
```

```console
$ uv run .
success: true
exit_code: 0
----- stdout -----
Hello, world!

----- stderr -----
```

### Run with stdin args

<!-- from run.rs::run_stdin_args -->

Arguments are passed through to Python when running with `-c`.

```console
$ uv run python -c "import sys; print(sys.argv)" foo bar
success: true
exit_code: 0
----- stdout -----
['-c', 'foo', 'bar']

----- stderr -----
```

## Validation errors

### Run with invalid project table

<!-- from run.rs::run_invalid_project_table -->

Running with a pyproject.toml that has [project.urls] but no [project] name produces an error.

```toml
# file: pyproject.toml
[project.urls]
repository = 'https://github.com/octocat/octocat-python'

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python main.py
print("Hello, world!")
```

```console
$ uv run main.py
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 2
  |
1 | [project.urls]
  |  ^^^^^^^
`pyproject.toml` is using the `[project]` table, but the required `project.name` field is not set
```

## Environment files

### Run with env file

<!-- from run.rs::run_with_env -->

Use `--env-file` to load environment variables from a file.

```python test.py
import os
print(os.environ.get('THE_EMPIRE_VARIABLE'))
print(os.environ.get('REBEL_1'))
```

```toml .env
THE_EMPIRE_VARIABLE=palpatine
REBEL_1=leia_organa
```

Without the env file, variables are not set:

```console
$ uv run test.py
success: true
exit_code: 0
----- stdout -----
None
None

----- stderr -----
```

With the env file, variables are loaded:

```console
$ uv run --env-file .env test.py
success: true
exit_code: 0
----- stdout -----
palpatine
leia_organa

----- stderr -----
```

### Run with custom env file path

<!-- from run.rs::run_with_env_file -->

The env file can have any name.

```python test.py
import os
print(os.environ.get('THE_EMPIRE_VARIABLE'))
print(os.environ.get('REBEL_1'))
```

```toml .custom_env
THE_EMPIRE_VARIABLE=palpatine
REBEL_1=leia_organa
```

```console
$ uv run --env-file .custom_env test.py
success: true
exit_code: 0
----- stdout -----
palpatine
leia_organa

----- stderr -----
```

### Run with malformed env file

<!-- from run.rs::run_with_malformed_env -->

Malformed environment variable names in env files produce warnings.

```python test.py
import os
print(os.environ.get('THE_EMPIRE_VARIABLE'))
```

```toml .env
THE_^EMPIRE_VARIABLE=darth_vader
```

```console
$ uv run --env-file .env test.py
success: true
exit_code: 0
----- stdout -----
None

----- stderr -----
warning: Failed to parse environment file `.env` at position 4: THE_^EMPIRE_VARIABLE=darth_vader
```

### Run with multiple env files

<!-- from run.rs::run_with_multiple_env_files -->

Multiple env files can be specified with repeated `--env-file` flags. Later files override earlier
ones.

```python test.py
import os
print(os.environ.get('THE_EMPIRE_VARIABLE'))
print(os.environ.get('REBEL_1'))
print(os.environ.get('REBEL_2'))
```

```toml .env1
THE_EMPIRE_VARIABLE=palpatine
REBEL_1=leia_organa
```

```toml .env2
THE_EMPIRE_VARIABLE=palpatine
REBEL_1=obi_wan_kenobi
REBEL_2=C3PO
```

```console
$ uv run --env-file .env1 --env-file .env2 test.py
success: true
exit_code: 0
----- stdout -----
palpatine
obi_wan_kenobi
C3PO

----- stderr -----
```

### Run with missing env file

<!-- from run.rs::run_with_not_existing_env_file -->

Specifying a non-existent env file produces an error.

```python test.py
import os
print(os.environ.get('THE_EMPIRE_VARIABLE'))
```

```console
$ uv run --env-file .env.development test.py
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No environment file found at: `.env.development`
```

## Extras and conflicts

### Run with extra conflict

<!-- from run.rs::run_with_extra_conflict -->

When extras have conflicting dependencies, the conflict can be specified in pyproject.toml.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12.0"
dependencies = []

[project.optional-dependencies]
foo = ["iniconfig==2.0.0"]
bar = ["iniconfig==1.1.1"]

[tool.uv]
conflicts = [
  [
    { extra = "foo" },
    { extra = "bar" },
  ],
]
```

```console
$ uv run --extra foo python -c "import iniconfig"
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```
