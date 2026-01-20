# Project Initialization - Scripts

Tests for initializing Python scripts with PEP 723 inline metadata using `uv init --script`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Script

<!-- Derived from [`init::init_script`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L652-L701) -->

The `--script` flag creates a Python script with inline metadata.

```console working-dir="foo"
$ uv init --script main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized script at `main.py`
```

The script contains PEP 723 metadata:

```python title="foo/main.py" snapshot=true
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///


def main() -> None:
    print("Hello from main.py!")


if __name__ == "__main__":
    main()
```

The script can be run:

```console working-dir="foo"
$ uv run python main.py
success: true
exit_code: 0
----- stdout -----
Hello from main.py!

----- stderr -----
```

## Script bare

<!-- Derived from [`init::init_script_bare`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L705-L737) -->

Using `--bare` with `--script` omits the default script content.

```console working-dir="foo"
$ uv init --script --bare main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized script at `main.py`
```

Only the metadata block is created:

```python title="foo/main.py" snapshot=true
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
```

## Script Python version

<!-- Derived from [`init::init_script_python_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L741-L781) -->

Python versions passed as arguments are present in file metadata.

```toml
# mdtest

[environment]
python-version = "3.11"
```

```console working-dir="foo"
$ uv init --script version.py --python 3.11
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized script at `version.py`
```

The specified Python version is in the metadata:

```python title="foo/version.py" snapshot=true
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///


def main() -> None:
    print("Hello from version.py!")


if __name__ == "__main__":
    main()
```

## Script create directory

<!-- Derived from [`init::init_script_create_directory`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L785-L827) -->

Init script creates parent directories if they don't exist.

```toml
# mdtest

[environment]
python-version = "3.12"
```

```console working-dir="foo"
$ uv init --script test/dir.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized script at `test/dir.py`
```

The script is created in the nested directory:

```python title="foo/test/dir.py" snapshot=true
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///


def main() -> None:
    print("Hello from dir.py!")


if __name__ == "__main__":
    main()
```

## Script file conflicts

<!-- Derived from [`init::init_script_file_conflicts`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L829-L879) -->

When initializing a script that already exists, metadata is added preserving content.

```python
# file: foo/script.py
print("Hello, world!")
```

```console working-dir="foo"
$ uv init --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized script at `script.py`
```

The metadata is added to the existing file:

```python title="foo/script.py" snapshot=true
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

print("Hello, world!")
```

## Script shebang

<!-- Derived from [`init::init_script_shebang`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L883-L938) -->

Init script does not trash an existing shebang.

```python
# file: script.py
#! /usr/bin/env python3
print("Hello, world!")
```

```console
$ uv init --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: If you execute script.py directly, it might ignore its inline metadata.
Consider replacing its shebang with: #!/usr/bin/env -S uv run --script
Initialized script at `script.py`
```

The shebang is preserved:

```python title="script.py" snapshot=true
#! /usr/bin/env python3
#
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

print("Hello, world!")
```

When the shebang already contains `uv`, no warning is shown:

```python
# file: script.py
#!/usr/bin/env -S uv run --script
print("Hello, world!")
```

```console
$ uv init --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized script at `script.py`
```

The uv shebang is preserved:

```python title="script.py" snapshot=true
#!/usr/bin/env -S uv run --script
#
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

print("Hello, world!")
```

## Script picks latest stable version

<!-- Derived from [`init::init_script_picks_latest_stable_version`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L944-L981) -->

`uv init --script` picks the latest non-pre-release version of Python for `requires-python`.

```toml
# mdtest

[environment]
python-versions = ["3.14.0rc2", "3.13", "3.12"]
managed-python-versions = ["3.14.0rc2", "3.13", "3.12"]
required-features = "python-patch"
```

```console
$ uv init --script main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized script at `main.py`
```

The latest stable version is used (3.13, not 3.14.0rc2):

```python title="main.py" snapshot=true
# /// script
# requires-python = ">=3.13"
# dependencies = []
# ///


def main() -> None:
    print("Hello from main.py!")


if __name__ == "__main__":
    main()
```
