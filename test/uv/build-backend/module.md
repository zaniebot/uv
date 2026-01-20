# Build Backend - Module Discovery

Tests for module name configuration and discovery in the uv build backend.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Rename

<!-- Derived from [`build_backend::rename_module`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L282-L359) -->

The `tool.uv.build-backend.module-name` setting controls which module is included in the build,
ignoring the project name.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"

[tool.uv.build-backend]
module-name = "bar"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

This module would normally be included based on project name, but is ignored:

```python
# file: src/foo/__init__.py
print("Hi from foo")
```

This module is selected due to the module-name override:

```python
# file: src/bar/__init__.py
print("Hi from bar")
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
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/dist/foo-1.0.0-py3-none-any.whl)
```

Importing with the module-name succeeds:

```console
$ python -c "import bar"
success: true
exit_code: 0
----- stdout -----
Hi from bar

----- stderr -----
```

Importing with the package name fails:

```console
$ python -c "import foo"
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Traceback (most recent call last):
  File "<string>", line 1, in <module>
ModuleNotFoundError: No module named 'foo'
```

## Rename editable

<!-- Derived from [`build_backend::rename_module_editable_build`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L363-L419) -->

The `tool.uv.build-backend.module-name` setting works for editable builds.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"

[tool.uv.build-backend]
module-name = "bar"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

```python
# file: src/bar/__init__.py
print("Hi from bar")
```

Install as editable:

```console
$ uv pip install -e .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==1.0.0 (from file://[TEMP_DIR]/)
```

Importing with the module-name succeeds:

```console
$ python -c "import bar"
success: true
exit_code: 0
----- stdout -----
Hi from bar

----- stderr -----
```

## Normalization

<!-- Derived from [`build_backend::build_module_name_normalization`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L423-L534) -->

Module name matching is case-insensitive, allowing builds even when the module name case doesn't
match exactly.

```toml
# file: pyproject.toml
[project]
name = "django-plugin"
version = "1.0.0"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"

[tool.uv.build-backend]
module-name = "Django_plugin"
```

Create src directory:

```console
$ mkdir -p src
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Building without any module fails:

```console
$ uv build --wheel
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Expected a Python module at: src/Django_plugin/__init__.py
```

Create the module directory:

```console
$ mkdir -p src/Django_plugin
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Building with module directory but no **init**.py fails:

```console
$ uv build --wheel
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
  × Failed to build `[TEMP_DIR]/`
  ╰─▶ Expected a Python module at: src/Django_plugin/__init__.py
```

Create **init**.py with the exact case from module-name:

```python
# file: src/Django_plugin/__init__.py
print("Hi from bar")
```

Now building succeeds:

```console
$ uv build --wheel
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
Successfully built dist/django_plugin-1.0.0-py3-none-any.whl
```

Install and test:

```console
$ uv pip install --no-index --find-links dist django-plugin
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + django-plugin==1.0.0 (from file://[TEMP_DIR]/dist/django_plugin-1.0.0-py3-none-any.whl)
```

Import works with the original case:

```console
$ python -c "import Django_plugin"
success: true
exit_code: 0
----- stdout -----
Hi from bar

----- stderr -----
```

## Complex namespace

<!-- Derived from [`build_backend::complex_namespace_packages`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L627-L761) -->

Namespace packages work correctly, allowing multiple packages to contribute to the same namespace.

Create first part of namespace package:

```toml
# file: complex-project-part_a/pyproject.toml
[project]
name = "complex-project-part_a"
version = "1.0.0"

[tool.uv.build-backend]
module-name = "complex_project.part_a"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

```python
# file: complex-project-part_a/src/complex_project/part_a/__init__.py
def one():
    return 1
```

Create second part that depends on the first:

```toml
# file: complex-project-part_b/pyproject.toml
[project]
name = "complex-project-part_b"
version = "1.0.0"

[tool.uv.build-backend]
module-name = "complex_project.part_b"

[build-system]
requires = ["uv_build>=0.7,<10000"]
build-backend = "uv_build"
```

```python
# file: complex-project-part_b/src/complex_project/part_b/__init__.py
from complex_project.part_a import one

def two():
    return one() + one()
```

Build both packages:

```console
$ uv build complex-project-part_a --out-dir dist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built dist/complex_project_part_a-1.0.0.tar.gz
Successfully built dist/complex_project_part_a-1.0.0-py3-none-any.whl
```

```console
$ uv build complex-project-part_b --out-dir dist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built dist/complex_project_part_b-1.0.0.tar.gz
Successfully built dist/complex_project_part_b-1.0.0-py3-none-any.whl
```

Install both packages:

```console
$ uv pip install complex-project-part-a complex-project-part-b --offline --find-links dist
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + complex-project-part-a==1.0.0
 + complex-project-part-b==1.0.0
```

Test that the namespace package works:

```console
$ python -c "from complex_project.part_b import two; print(two())"
success: true
exit_code: 0
----- stdout -----
2

----- stderr -----
```

Test editable installs:

```console
$ uv pip install -e complex-project-part_a -e complex-project-part_b --offline
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Uninstalled 2 packages in [TIME]
Installed 2 packages in [TIME]
 - complex-project-part-a==1.0.0
 + complex-project-part-a==1.0.0 (from file://[TEMP_DIR]/complex-project-part_a)
 - complex-project-part-b==1.0.0
 + complex-project-part-b==1.0.0 (from file://[TEMP_DIR]/complex-project-part_b)
```

Editable install also works:

```console
$ python -c "from complex_project.part_b import two; print(two())"
success: true
exit_code: 0
----- stdout -----
2

----- stderr -----
```
