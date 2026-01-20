# Python Module - find_uv_bin

Tests for `python -m uv` functionality and the `uv.find_uv_bin()` Python module function.

```toml
# mdtest

[environment]
python-versions = ["3.12"]
required-features = ["python-managed"]
```

## Find uv bin venv

<!-- Derived from [`python_module::find_uv_bin_venv`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L33-L70) -->

Finding uv binary installed in a virtual environment.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv package in venv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find the binary in the virtual environment:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```

## Find uv bin target

<!-- Derived from [`python_module::find_uv_bin_target`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L73-L115) -->

Finding uv binary installed with --target.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install in target directory:

```console
$ uv pip install ../../packages/fake-uv --target target
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: .venv/[BIN]/[PYTHON]
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find the binary in the target directory:

```console
$ PYTHONPATH=target uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/target/[BIN]/uv

----- stderr -----
```

## Find uv bin prefix

<!-- Derived from [`python_module::find_uv_bin_prefix`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L118-L164) -->

Finding uv binary installed with --prefix.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install in prefix directory:

```console
$ uv pip install ../../packages/fake-uv --prefix prefix
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: .venv/[BIN]/[PYTHON]
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find the binary in the prefix directory:

```console
$ PYTHONPATH=prefix/lib/python3.12/site-packages uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/prefix/[BIN]/uv

----- stderr -----
```

## Find uv bin base prefix

<!-- Derived from [`python_module::find_uv_bin_base_prefix`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L167-L215) -->

Finding uv binary by falling back to sys.base_prefix.

Create base venv:

```console
$ uv venv base-venv
success: true
exit_code: 0
```

Install fake-uv in the base venv:

```console
$ uv pip install ../../packages/fake-uv --python base-venv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using Python 3.12.[X] environment at: base-venv
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Create regular venv:

```console
$ uv venv
success: true
exit_code: 0
```

Mutate base_prefix to find uv in base environment:

```console
$ PYTHONPATH=base-venv/lib/python3.12/site-packages uv run python -c "import sys, uv; sys.base_prefix = '[TEMP_DIR]/base-venv'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[TEMP_DIR]/base-venv/[BIN]/uv

----- stderr -----
```

## Find uv bin ephemeral

<!-- Derived from [`python_module::find_uv_bin_in_ephemeral_environment`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L218-L262) -->

Finding uv binary in an ephemeral --with environment.

Create project:

```toml
# file: pyproject.toml

[project]
name = "test-project"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = []
```

Find binary in ephemeral environment:

```console
$ uv run --with ../../packages/fake-uv python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[CACHE_DIR]/archive-v0/[HASH]/[BIN]/uv

----- stderr -----
Reading inline script metadata from: -
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

## Find uv bin parent ephemeral

<!-- Derived from [`python_module::find_uv_bin_in_parent_of_ephemeral_environment`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L265-L319) -->

Finding uv binary in parent of ephemeral environment.

Create project with uv dependency:

```toml
# file: pyproject.toml

[project]
name = "test-project"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["uv"]

[tool.uv.sources]
uv = { path = "../../packages/fake-uv" }
```

Sync to install dependencies:

```console
$ uv sync
success: true
exit_code: 0
```

Find binary in parent environment when using --with:

```console
$ uv run --with rich python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
Reading inline script metadata from: -
Resolved 1 package in [TIME]
Installed 1 package in [TIME]
 + rich==13.7.1
```

## Find uv bin user bin

<!-- Derived from [`python_module::find_uv_bin_user_bin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L322-L395) -->

Finding uv binary installed with --user.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install with --user:

```console
$ uv pip install ../../packages/fake-uv --user
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: .venv/[BIN]/[PYTHON]
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary in user scheme:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[USER_SCHEME]/[BIN]/uv

----- stderr -----
```

## Find uv bin error message

<!-- Derived from [`python_module::find_uv_bin_error_message`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L398-L474) -->

Error message when uv binary cannot be found.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install a package (not uv):

```console
$ uv pip install ../../packages/black_editable
success: true
exit_code: 0
```

Error when uv not found:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; uv.find_uv_bin()"
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Traceback (most recent call last):
[TRACEBACK]
FileNotFoundError: Could not find a `uv` binary in any of the following locations:
    - Searching for `uv[EXE]` within virtual environment at: [VENV]
    - Searching for `uv[EXE]` within base prefix at: /dev/null
    - Searching for `uv[EXE]` within user scheme at: [USER_SCHEME]

Consider installing `uv` with `pip install uv`, or use `uvx` to run commands with `uv` without installing it.
```

## Find uv bin py38

<!-- Derived from [`python_module::find_uv_bin_py38`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L477-L514) -->

```toml
# mdtest

[environment]
python-versions = ["3.8"]
required-features = ["python-managed", "python-eol"]
```

Test find_uv_bin() with Python 3.8.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```

## Find uv bin py39

<!-- Derived from [`python_module::find_uv_bin_py39`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L517-L554) -->

```toml
# mdtest

[environment]
python-versions = ["3.9"]
required-features = ["python-managed"]
```

Test find_uv_bin() with Python 3.9.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```

## Find uv bin py310

<!-- Derived from [`python_module::find_uv_bin_py310`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L557-L594) -->

```toml
# mdtest

[environment]
python-versions = ["3.10"]
required-features = ["python-managed"]
```

Test find_uv_bin() with Python 3.10.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```

## Find uv bin py311

<!-- Derived from [`python_module::find_uv_bin_py311`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L597-L634) -->

```toml
# mdtest

[environment]
python-versions = ["3.11"]
required-features = ["python-managed"]
```

Test find_uv_bin() with Python 3.11.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```

## Find uv bin py312

<!-- Derived from [`python_module::find_uv_bin_py312`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L637-L674) -->

```toml
# mdtest

[environment]
python-versions = ["3.12"]
required-features = ["python-managed"]
```

Test find_uv_bin() with Python 3.12.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```

## Find uv bin py313

<!-- Derived from [`python_module::find_uv_bin_py313`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L677-L714) -->

```toml
# mdtest

[environment]
python-versions = ["3.13"]
required-features = ["python-managed"]
```

Test find_uv_bin() with Python 3.13.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```

## Find uv bin py314

<!-- Derived from [`python_module::find_uv_bin_py314`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/python_module.rs#L717-L754) -->

```toml
# mdtest

[environment]
python-versions = ["3.14"]
required-features = ["python-managed"]
```

Test find_uv_bin() with Python 3.14.

Create venv:

```console
$ uv venv
success: true
exit_code: 0
```

Install fake-uv:

```console
$ uv pip install ../../packages/fake-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv==0.1.0 (from file://[WORKSPACE]/test/packages/fake-uv)
```

Find binary:

```console
$ uv run python -c "import sys; import uv; sys.base_prefix = '/dev/null'; print(uv.find_uv_bin())"
success: true
exit_code: 0
----- stdout -----
[VENV]/[BIN]/uv

----- stderr -----
```
