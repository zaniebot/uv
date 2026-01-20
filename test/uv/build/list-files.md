# Package Building - File Listing

Tests for the `--list` flag that shows files included in builds.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## List files

<!-- Derived from [`build::build_list_files`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1633-L1751) -->

The `--list` flag shows files that will be included in the build without actually building.

List files in default build (sdist + wheel from sdist):

```console
$ uv build test/packages/built-by-uv --out-dir output1 --list
success: true
exit_code: 0
----- stdout -----
Building built_by_uv-0.1.0.tar.gz will include the following files:
built_by_uv-0.1.0/PKG-INFO (generated)
built_by_uv-0.1.0/LICENSE-APACHE (LICENSE-APACHE)
built_by_uv-0.1.0/LICENSE-MIT (LICENSE-MIT)
built_by_uv-0.1.0/README.md (README.md)
built_by_uv-0.1.0/assets/data.csv (assets/data.csv)
built_by_uv-0.1.0/header/built_by_uv.h (header/built_by_uv.h)
built_by_uv-0.1.0/pyproject.toml (pyproject.toml)
built_by_uv-0.1.0/scripts/whoami.sh (scripts/whoami.sh)
built_by_uv-0.1.0/src/built_by_uv/__init__.py (src/built_by_uv/__init__.py)
built_by_uv-0.1.0/src/built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
built_by_uv-0.1.0/src/built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
built_by_uv-0.1.0/src/built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
built_by_uv-0.1.0/src/built_by_uv/build-only.h (src/built_by_uv/build-only.h)
built_by_uv-0.1.0/src/built_by_uv/cli.py (src/built_by_uv/cli.py)
built_by_uv-0.1.0/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
Building built_by_uv-0.1.0-py3-none-any.whl will include the following files:
built_by_uv/__init__.py (src/built_by_uv/__init__.py)
built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
built_by_uv/cli.py (src/built_by_uv/cli.py)
built_by_uv-0.1.0.dist-info/licenses/LICENSE-APACHE (LICENSE-APACHE)
built_by_uv-0.1.0.dist-info/licenses/LICENSE-MIT (LICENSE-MIT)
built_by_uv-0.1.0.dist-info/licenses/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
built_by_uv-0.1.0.data/headers/built_by_uv.h (header/built_by_uv.h)
built_by_uv-0.1.0.data/scripts/whoami.sh (scripts/whoami.sh)
built_by_uv-0.1.0.data/data/data.csv (assets/data.csv)
built_by_uv-0.1.0.dist-info/WHEEL (generated)
built_by_uv-0.1.0.dist-info/entry_points.txt (generated)
built_by_uv-0.1.0.dist-info/METADATA (generated)

----- stderr -----
Building source distribution (uv build backend)...
Successfully built output1/built_by_uv-0.1.0.tar.gz
```

Only the sdist is created:

```console
$ test -f output1/built_by_uv-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

```console
$ test -f output1/built_by_uv-0.1.0-py3-none-any.whl && echo "wheel exists" || echo "wheel missing"
success: true
exit_code: 0
----- stdout -----
wheel missing

----- stderr -----
```

List files with explicit --sdist --wheel:

```console
$ uv build test/packages/built-by-uv --out-dir output2 --list --sdist --wheel
success: true
exit_code: 0
----- stdout -----
Building built_by_uv-0.1.0.tar.gz will include the following files:
built_by_uv-0.1.0/PKG-INFO (generated)
built_by_uv-0.1.0/LICENSE-APACHE (LICENSE-APACHE)
built_by_uv-0.1.0/LICENSE-MIT (LICENSE-MIT)
built_by_uv-0.1.0/README.md (README.md)
built_by_uv-0.1.0/assets/data.csv (assets/data.csv)
built_by_uv-0.1.0/header/built_by_uv.h (header/built_by_uv.h)
built_by_uv-0.1.0/pyproject.toml (pyproject.toml)
built_by_uv-0.1.0/scripts/whoami.sh (scripts/whoami.sh)
built_by_uv-0.1.0/src/built_by_uv/__init__.py (src/built_by_uv/__init__.py)
built_by_uv-0.1.0/src/built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
built_by_uv-0.1.0/src/built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
built_by_uv-0.1.0/src/built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
built_by_uv-0.1.0/src/built_by_uv/build-only.h (src/built_by_uv/build-only.h)
built_by_uv-0.1.0/src/built_by_uv/cli.py (src/built_by_uv/cli.py)
built_by_uv-0.1.0/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
Building built_by_uv-0.1.0-py3-none-any.whl will include the following files:
built_by_uv/__init__.py (src/built_by_uv/__init__.py)
built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
built_by_uv/cli.py (src/built_by_uv/cli.py)
built_by_uv-0.1.0.dist-info/licenses/LICENSE-APACHE (LICENSE-APACHE)
built_by_uv-0.1.0.dist-info/licenses/LICENSE-MIT (LICENSE-MIT)
built_by_uv-0.1.0.dist-info/licenses/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
built_by_uv-0.1.0.data/headers/built_by_uv.h (header/built_by_uv.h)
built_by_uv-0.1.0.data/scripts/whoami.sh (scripts/whoami.sh)
built_by_uv-0.1.0.data/data/data.csv (assets/data.csv)
built_by_uv-0.1.0.dist-info/WHEEL (generated)
built_by_uv-0.1.0.dist-info/entry_points.txt (generated)
built_by_uv-0.1.0.dist-info/METADATA (generated)

----- stderr -----
```

No artifacts are created:

```console
$ test -f output2/built_by_uv-0.1.0.tar.gz && echo "sdist exists" || echo "sdist missing"
success: true
exit_code: 0
----- stdout -----
sdist missing

----- stderr -----
```

```console
$ test -f output2/built_by_uv-0.1.0-py3-none-any.whl && echo "wheel exists" || echo "wheel missing"
success: true
exit_code: 0
----- stdout -----
wheel missing

----- stderr -----
```

## List files errors

<!-- Derived from [`build::build_list_files_errors`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1755-L1800) -->

The `--list` flag has constraints on when it can be used.

Cannot use --list with --force-pep517:

```console
$ uv build test/packages/built-by-uv --out-dir output1 --list --force-pep517
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: the argument '--list' cannot be used with '--force-pep517'

Usage: uv build --cache-dir [CACHE_DIR] --out-dir <OUT_DIR> --exclude-newer <EXCLUDE_NEWER> <SRC>

For more information, try '--help'.
```

Cannot use --list with non-uv build backends:

```console
$ uv build test/packages/anyio_local --out-dir output2 --list
success: false
exit_code: 2
----- stdout -----

----- stderr -----
  × Failed to build `[WORKSPACE]/test/packages/anyio_local`
  ╰─▶ Can only use `--list` with the uv backend
```
