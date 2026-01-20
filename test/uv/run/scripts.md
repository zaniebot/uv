# Scripts

Tests for running Python scripts with `uv run`.

## PEP 723 inline script metadata

### Basic PEP 723 script

<!-- from run.rs::run_pep723_script -->

Scripts with PEP 723 metadata install their own dependencies.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["anyio"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python main.py
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "iniconfig",
# ]
# ///

import iniconfig
```

```console
$ uv run main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Script reuses environment

<!-- from run.rs::run_pep723_script -->

Running the same script again reuses the existing environment.

```console
$ uv run main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

### Script without dependencies field

<!-- from run.rs::run_pep723_script -->

The dependencies field can be omitted in PEP 723 metadata.

```python main.py
# /// script
# requires-python = ">=3.11"
# ///

print("Hello, world!")
```

```console
$ uv run main.py
success: true
exit_code: 0
----- stdout -----
Hello, world!

----- stderr -----
```

### Script with locked warns

<!-- from run.rs::run_pep723_script -->

Running a script with `--locked` warns when no lockfile exists.

```console
$ uv run --locked main.py
success: true
exit_code: 0
----- stdout -----
Hello, world!

----- stderr -----
warning: No lockfile found for Python script (ignoring `--locked`); run `uv lock --script` to generate a lockfile
```

### Script with missing package

<!-- from run.rs::run_pep723_script -->

Scripts with unresolvable dependencies fail with an appropriate error.

```python main.py
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "add",
# ]
# ///
```

```console
$ uv run --no-project main.py
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving script dependencies:
  ╰─▶ Because there are no versions of add and you require add, we can conclude that your requirements are unsatisfiable.
```

### Script with unclosed tag

<!-- from run.rs::run_pep723_script -->

Scripts with unclosed PEP 723 tags fail with an error.

```python main.py
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "iniconfig",
# ]

# ///

import iniconfig
```

```console
$ uv run --no-project main.py
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: An opening tag (`# /// script`) was found without a closing tag (`# ///`). Ensure that every line between the opening and closing tags (including empty lines) starts with a leading `#`.
```

## Script Python version requirements

### Incompatible Python version

<!-- from run.rs::run_pep723_script_requires_python -->

Scripts with incompatible Python requirements warn but still run.

```toml .python-version
3.9
```

```python main.py
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "iniconfig",
# ]
# ///

import iniconfig

x: str | int = "hello"
print(x)
```

```console
$ uv run main.py
success: false
exit_code: 1
----- stdout -----

----- stderr -----
warning: The Python request from `.python-version` resolved to Python 3.9.[X], which is incompatible with the script's Python requirement: `>=3.11`
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
Traceback (most recent call last):
  File "[TEMP_DIR]/main.py", line 10, in <module>
    x: str | int = "hello"
TypeError: unsupported operand type(s) for |: 'type' and 'type'
```

## Exit codes

### Script exit code propagates

<!-- from run.rs::run_exit_code -->

Exit codes from scripts are propagated correctly.

```python script.py
# /// script
# requires-python = ">=3.11"
# ///

exit(42)
```

```console
$ uv run script.py
success: false
exit_code: 42
----- stdout -----

----- stderr -----
```
