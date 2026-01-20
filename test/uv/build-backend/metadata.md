# Build Backend - Metadata

Tests for metadata generation in the uv build backend.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## All metadata

<!-- Derived from [`build_backend::build_with_all_metadata`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L1228-L1447) -->

The uv build backend correctly handles projects with all possible metadata fields.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
description = "A Python package with all metadata fields"
readme = "Readme.md"
requires-python = ">=3.12"
license = "MIT OR Apache-2.0"
license-files = ["License*"]
authors = [
    {name = "Jane Doe", email = "jane@example.com"},
    {name = "John Doe"},
    {email = "info@example.com"},
]
maintainers = [
    {name = "ferris", email = "ferris@example.com"},
]
keywords = ["example", "test", "metadata"]
classifiers = [
    "Development Status :: 4 - Beta",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.12",
]
dependencies = [
    "anyio>=4,<5",
]

[project.optional-dependencies]
dev = ["pytest>=7.0"]

[project.urls]
Homepage = "https://octocat.github.io/spoon-knife"
Repository = "https://github.com/octocat/Spoon-Knife"
Changelog = "https://github.com/octocat/Spoon-Knife/blob/main/CHANGELOG.md"

[project.scripts]
foo-cli = "foo:main"

[project.gui-scripts]
foo-gui = "foo:gui_main"

[project.entry-points."foo.plugins"]
bar = "foo:bar_plugin"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

```python
# file: src/foo/__init__.py
def main():
    print("Hello from foo!")

def gui_main():
    print("GUI main")

def bar_plugin():
    pass
```

```text
# file: License.txt
MIT License
```

```text
# file: Readme.md
Hello World!
```

Build the wheel:

```console
$ uv build --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
Successfully built dist/foo-1.0.0-py3-none-any.whl
```

Install the wheel:

```console
$ uv pip install dist/foo-1.0.0-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
[PACKAGES]
```

Check the METADATA file:

```console
$ cat .venv/lib/python*/site-packages/foo-1.0.0.dist-info/METADATA
success: true
exit_code: 0
----- stdout -----
Metadata-Version: 2.4
Name: foo
Version: 1.0.0
Summary: A Python package with all metadata fields
Keywords: example,test,metadata
Author: Jane Doe, John Doe
Author-email: Jane Doe <jane@example.com>, info@example.com
License-Expression: MIT OR Apache-2.0
License-File: License.txt
Classifier: Development Status :: 4 - Beta
Classifier: Programming Language :: Python :: 3
Classifier: Programming Language :: Python :: 3.12
Requires-Dist: anyio>=4,<5
Requires-Dist: pytest>=7.0 ; extra == 'dev'
Maintainer: ferris
Maintainer-email: ferris <ferris@example.com>
Requires-Python: >=3.12
Project-URL: Homepage, https://octocat.github.io/spoon-knife
Project-URL: Repository, https://github.com/octocat/Spoon-Knife
Project-URL: Changelog, https://github.com/octocat/Spoon-Knife/blob/main/CHANGELOG.md
Provides-Extra: dev
Description-Content-Type: text/markdown

Hello World!

----- stderr -----
```

Check the WHEEL file:

```console
$ cat .venv/lib/python*/site-packages/foo-1.0.0.dist-info/WHEEL
success: true
exit_code: 0
----- stdout -----
Wheel-Version: 1.0
Generator: uv [VERSION]
Root-Is-Purelib: true
Tag: py3-none-any

----- stderr -----
```

Test the console script:

```console
$ foo-cli
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
```

Test the GUI script:

```console
$ foo-gui
success: true
exit_code: 0
----- stdout -----
GUI main

----- stderr -----
```
