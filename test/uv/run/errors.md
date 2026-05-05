# Run Errors

Tests for error handling in `uv run`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Locked mode

### Locked without lockfile

<!-- from run.rs::run_locked -->

Running with `--locked` should error if no lockfile is present.

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

## Script errors

### Unclosed PEP 723 tag

<!-- from run.rs::run_pep723_script -->

An unclosed PEP 723 script tag produces an error.

```python
# file: main.py
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

### Script with missing package

<!-- from run.rs::run_pep723_script -->

A script depending on a non-existent package produces a resolution error.

```python
# file: main.py
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
