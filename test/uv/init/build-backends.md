# Project Initialization - Build Backends

Tests for initializing projects with different build backends using `uv init --build-backend`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Application package with flit

<!-- Derived from [`init::init_application_package_flit`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3064-L3134) -->

The `--app --package --build-backend flit` flags create a packaged application with flit backend.

```console working-dir="foo"
$ uv init --app --package --build-backend flit
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The flit build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.scripts]
foo = "foo:main"

[build-system]
requires = ["flit_core>=3.2,<4"]
build-backend = "flit_core.buildapi"
```

The **init**.py contains the main function:

```python title="foo/src/foo/__init__.py" snapshot=true
def main() -> None:
    print("Hello from foo!")
```

The application can be run:

```console working-dir="foo"
$ uv run foo
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==0.1.0 (from file://[TEMP_DIR]/foo)
```

## Library with flit

<!-- Derived from [`init::init_library_flit`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3136-L3215) -->

The `--lib --build-backend flit` flags create a library project with flit backend.

```console working-dir="foo"
$ uv init --lib --build-backend flit
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The flit build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["flit_core>=3.2,<4"]
build-backend = "flit_core.buildapi"
```

The **init**.py contains the hello function:

```python title="foo/src/foo/__init__.py" snapshot=true
def hello() -> str:
    return "Hello from foo!"
```

An empty py.typed file is created:

```text title="foo/src/foo/py.typed" snapshot=true

```

The library can be imported:

```console working-dir="foo"
$ uv run python -c "import foo; print(foo.hello())"
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==0.1.0 (from file://[TEMP_DIR]/foo)
```

## Library with poetry

<!-- Derived from [`init::init_library_poetry`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3256-L3335) -->

The `--lib --build-backend poetry` flags create a library project with poetry-core backend.

```toml
# mdtest

[environment]
exclude-newer = "2025-04-28T00:00:00Z"
```

```console working-dir="foo"
$ uv init --lib --build-backend poetry
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The poetry-core build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["poetry-core>=2,<3"]
build-backend = "poetry.core.masonry.api"
```

The **init**.py contains the hello function:

```python title="foo/src/foo/__init__.py" snapshot=true
def hello() -> str:
    return "Hello from foo!"
```

An empty py.typed file is created:

```text title="foo/src/foo/py.typed" snapshot=true

```

The library can be imported:

```console working-dir="foo"
$ uv run python -c "import foo; print(foo.hello())"
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==0.1.0 (from file://[TEMP_DIR]/foo)
```

## Application package with maturin

<!-- Derived from [`init::init_app_build_backend_maturin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3337-L3466) -->

The `--app --package --build-backend maturin` flags create a packaged application with maturin
backend for Rust extensions.

```toml
# mdtest

[environment]
required-features = "crates-io"
```

```console working-dir="foo"
$ uv init --app --package --build-backend maturin
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The maturin build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.scripts]
foo = "foo:main"

[tool.maturin]
module-name = "foo._core"
python-packages = ["foo"]
python-source = "src"

[tool.uv]
cache-keys = [{ file = "pyproject.toml" }, { file = "src/**/*.rs" }, { file = "Cargo.toml" }, { file = "Cargo.lock" }]

[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"
```

The **init**.py imports from the Rust extension:

```python title="foo/src/foo/__init__.py" snapshot=true
from foo._core import hello_from_bin


def main() -> None:
    print(hello_from_bin())
```

A type stub is created:

```python title="foo/src/foo/_core.pyi" snapshot=true
def hello_from_bin() -> str: ...
```

The Rust source is created:

```rust title="foo/src/lib.rs" snapshot=true
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
mod _core {
    use pyo3::prelude::*;

    #[pyfunction]
    fn hello_from_bin() -> String {
        "Hello from foo!".to_string()
    }
}
```

The Cargo.toml is created:

```toml title="foo/Cargo.toml" snapshot=true
[package]
name = "foo"
version = "0.1.0"
edition = "2024"

[lib]
name = "_core"
# "cdylib" is necessary to produce a shared library for Python to import from.
crate-type = ["cdylib"]

[dependencies]
# "extension-module" tells pyo3 we want to build an extension module (skips linking against libpython.so)
# "abi3-py39" tells pyo3 (and maturin) to build using the stable ABI with minimum Python version 3.9
pyo3 = { version = "0.27.1", features = ["extension-module", "abi3-py39"] }
```

## Application package with scikit

<!-- Derived from [`init::init_app_build_backend_scikit`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3468-L3590) -->

The `--app --package --build-backend scikit` flags create a packaged application with
scikit-build-core backend for C++ extensions.

```console working-dir="foo"
$ uv init --app --package --build-backend scikit
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The scikit-build-core build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.scripts]
foo = "foo:main"

[tool.scikit-build]
minimum-version = "build-system.requires"
build-dir = "build/{wheel_tag}"

[tool.uv]
cache-keys = [{ file = "pyproject.toml" }, { file = "src/**/*.{h,c,hpp,cpp}" }, { file = "CMakeLists.txt" }]

[build-system]
requires = ["scikit-build-core>=0.10", "pybind11"]
build-backend = "scikit_build_core.build"
```

The **init**.py imports from the C++ extension:

```python title="foo/src/foo/__init__.py" snapshot=true
from foo._core import hello_from_bin


def main() -> None:
    print(hello_from_bin())
```

A type stub is created:

```python title="foo/src/foo/_core.pyi" snapshot=true
def hello_from_bin() -> str: ...
```

The C++ source is created:

```cpp title="foo/src/main.cpp" snapshot=true
#include <pybind11/pybind11.h>

std::string hello_from_bin() { return "Hello from foo!"; }

namespace py = pybind11;

PYBIND11_MODULE(_core, m) {
  m.doc() = "pybind11 hello module";

  m.def("hello_from_bin", &hello_from_bin, R"pbdoc(
      A function that returns a Hello string.
  )pbdoc");
}
```

The CMakeLists.txt is created:

```cmake title="foo/CMakeLists.txt" snapshot=true
cmake_minimum_required(VERSION 3.15)
project(${SKBUILD_PROJECT_NAME} LANGUAGES CXX)

set(PYBIND11_FINDPYTHON ON)
find_package(pybind11 CONFIG REQUIRED)

pybind11_add_module(_core MODULE src/main.cpp)
install(TARGETS _core DESTINATION ${SKBUILD_PROJECT_NAME})
```

## Library with maturin

<!-- Derived from [`init::init_lib_build_backend_maturin`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3592-L3718) -->

The `--lib --build-backend maturin` flags create a library project with maturin backend for Rust
extensions.

```toml
# mdtest

[environment]
required-features = "crates-io"
```

```console working-dir="foo"
$ uv init --lib --build-backend maturin
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The maturin build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[tool.maturin]
module-name = "foo._core"
python-packages = ["foo"]
python-source = "src"

[tool.uv]
cache-keys = [{ file = "pyproject.toml" }, { file = "src/**/*.rs" }, { file = "Cargo.toml" }, { file = "Cargo.lock" }]

[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"
```

The **init**.py imports from the Rust extension:

```python title="foo/src/foo/__init__.py" snapshot=true
from foo._core import hello_from_bin


def hello() -> str:
    return hello_from_bin()
```

A type stub is created:

```python title="foo/src/foo/_core.pyi" snapshot=true
def hello_from_bin() -> str: ...
```

The Rust source is created:

```rust title="foo/src/lib.rs" snapshot=true
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
mod _core {
    use pyo3::prelude::*;

    #[pyfunction]
    fn hello_from_bin() -> String {
        "Hello from foo!".to_string()
    }
}
```

The Cargo.toml is created:

```toml title="foo/Cargo.toml" snapshot=true
[package]
name = "foo"
version = "0.1.0"
edition = "2024"

[lib]
name = "_core"
# "cdylib" is necessary to produce a shared library for Python to import from.
crate-type = ["cdylib"]

[dependencies]
# "extension-module" tells pyo3 we want to build an extension module (skips linking against libpython.so)
# "abi3-py39" tells pyo3 (and maturin) to build using the stable ABI with minimum Python version 3.9
pyo3 = { version = "0.27.1", features = ["extension-module", "abi3-py39"] }
```

## Library with scikit

<!-- Derived from [`init::init_lib_build_backend_scikit`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3720-L3839) -->

The `--lib --build-backend scikit` flags create a library project with scikit-build-core backend for
C++ extensions.

```console working-dir="foo"
$ uv init --lib --build-backend scikit
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The scikit-build-core build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[tool.scikit-build]
minimum-version = "build-system.requires"
build-dir = "build/{wheel_tag}"

[tool.uv]
cache-keys = [{ file = "pyproject.toml" }, { file = "src/**/*.{h,c,hpp,cpp}" }, { file = "CMakeLists.txt" }]

[build-system]
requires = ["scikit-build-core>=0.10", "pybind11"]
build-backend = "scikit_build_core.build"
```

The **init**.py imports from the C++ extension:

```python title="foo/src/foo/__init__.py" snapshot=true
from foo._core import hello_from_bin


def hello() -> str:
    return hello_from_bin()
```

A type stub is created:

```python title="foo/src/foo/_core.pyi" snapshot=true
def hello_from_bin() -> str: ...
```

The C++ source is created:

```cpp title="foo/src/main.cpp" snapshot=true
#include <pybind11/pybind11.h>

std::string hello_from_bin() { return "Hello from foo!"; }

namespace py = pybind11;

PYBIND11_MODULE(_core, m) {
  m.doc() = "pybind11 hello module";

  m.def("hello_from_bin", &hello_from_bin, R"pbdoc(
      A function that returns a Hello string.
  )pbdoc");
}
```

The CMakeLists.txt is created:

```cmake title="foo/CMakeLists.txt" snapshot=true
cmake_minimum_required(VERSION 3.15)
project(${SKBUILD_PROJECT_NAME} LANGUAGES CXX)

set(PYBIND11_FINDPYTHON ON)
find_package(pybind11 CONFIG REQUIRED)

pybind11_add_module(_core MODULE src/main.cpp)
install(TARGETS _core DESTINATION ${SKBUILD_PROJECT_NAME})
```

## Application package with hatchling

<!-- Derived from [`init::init_application_package_hatchling`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3841-L3909) -->

The `--app --package --build-backend hatchling` flags create a packaged application with hatchling
backend.

```console working-dir="foo"
$ uv init --app --package --build-backend hatchling
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo`
```

The hatchling build backend is configured:

```toml title="foo/pyproject.toml" snapshot=true
[project]
name = "foo"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.scripts]
foo = "foo:main"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

The **init**.py contains the main function:

```python title="foo/src/foo/__init__.py" snapshot=true
def main() -> None:
    print("Hello from foo!")
```

The application can be run:

```console working-dir="foo"
$ uv run foo
success: true
exit_code: 0
----- stdout -----
Hello from foo!

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + foo==0.1.0 (from file://[TEMP_DIR]/foo)
```

## Backend implies package

<!-- Derived from [`init::init_backend_implies_package`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L3217-L3254) -->

When using `--build-backend`, the `--package` flag is implied.

```console
$ uv init project --build-backend flit
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project` at `[TEMP_DIR]/project`
```

A packaged application is created:

```toml title="project/pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.scripts]
project = "project:main"

[build-system]
requires = ["flit_core>=3.2,<4"]
build-backend = "flit_core.buildapi"
```
