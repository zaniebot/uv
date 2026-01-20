# Build Backend - Basic Building

Tests for the uv build backend (`uv_build`) when building packages that use it.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Direct wheel

<!-- Derived from [`build_backend::built_by_uv_direct_wheel`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L28-L76) -->

Building a wheel directly from a project using the uv build backend.

Build wheel from the built-by-uv test package:

```console
$ uv build test/packages/built-by-uv --wheel --out-dir output
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building wheel (uv build backend)...
Successfully built output/built_by_uv-0.1.0-py3-none-any.whl
```

Install and test the wheel:

```console
$ uv pip install output/built_by_uv-0.1.0-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + built-by-uv==0.1.0 (from file://[TEMP_DIR]/output/built_by_uv-0.1.0-py3-none-any.whl)
```

Test that the package works correctly:

```console
$ python -c "from built_by_uv import greet; from built_by_uv.arithmetic.circle import area; print(greet()); print(f'Area of a circle with r=2: {area(2)}')"
success: true
exit_code: 0
----- stdout -----
Hello 👋
Area of a circle with r=2: 12.56636

----- stderr -----
```

Test that console script works:

```console
$ say-hi
success: true
exit_code: 0
----- stdout -----
Hi from a script!

----- stderr -----
```

## Direct

<!-- Derived from [`build_backend::built_by_uv_direct`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L84-L150) -->

Building both sdist and wheel (wheel built from sdist) for a project using the uv build backend.

Build both artifacts:

```console
$ uv build test/packages/built-by-uv --out-dir output
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution (uv build backend)...
Building wheel from source distribution (uv build backend)...
Successfully built output/built_by_uv-0.1.0.tar.gz
Successfully built output/built_by_uv-0.1.0-py3-none-any.whl
```

Install and test the wheel:

```console
$ uv pip install output/built_by_uv-0.1.0-py3-none-any.whl
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + built-by-uv==0.1.0 (from file://[TEMP_DIR]/output/built_by_uv-0.1.0-py3-none-any.whl)
```

Test that the package works correctly:

```console
$ python -c "from built_by_uv import greet; from built_by_uv.arithmetic.circle import area; print(greet()); print(f'Area of a circle with r=2: {area(2)}')"
success: true
exit_code: 0
----- stdout -----
Hello 👋
Area of a circle with r=2: 12.56636

----- stderr -----
```

## Editable

<!-- Derived from [`build_backend::built_by_uv_editable`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build_backend.rs#L158-L213) -->

Editable installs work correctly with the uv build backend.

Install pytest:

```console
$ uv pip install pytest
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
[PACKAGES]
```

Without the editable install, pytest fails:

```console
$ python -m pytest test/packages/built-by-uv --quiet --capture=no
success: false
exit_code: [N]
----- stdout -----
[WILDCARD]
----- stderr -----
[WILDCARD]
```

Install the package as editable:

```console
$ uv pip install -e test/packages/built-by-uv
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + built-by-uv==0.1.0 (from file://[WORKSPACE]/test/packages/built-by-uv)
```

Now pytest passes:

```console
$ python -m pytest test/packages/built-by-uv --quiet --capture=no
success: true
exit_code: 0
----- stdout -----
..
[N] passed in [TIME]

----- stderr -----
```
