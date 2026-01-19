# Cache Clean

Tests for `uv cache clean`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = ["pypi"]

[filters]
counts = true
cache-size = true
```

## Clean all

<!-- Derived from [`cache_clean::clean_all`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_clean.rs#L11-L39) -->

`uv cache clean` should remove all packages from the cache.

```toml
# file: requirements.txt

typing-extensions
iniconfig
```

Install packages to populate the cache:

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
 + typing-extensions==4.10.0
```

Clean the entire cache:

```console
$ uv cache clean --verbose
success: true
exit_code: 0
----- stdout -----

----- stderr -----
DEBUG uv [VERSION] ([COMMIT] DATE)
DEBUG Acquired exclusive lock for `[CACHE_DIR]/`
Clearing cache at: [CACHE_DIR]/
DEBUG Released lock at `[CACHE_DIR]/.lock`
Removed [N] files ([SIZE])
```

## Clean single package (PyPI)

<!-- Derived from [`cache_clean::clean_package_pypi`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_clean.rs#L96-L170) -->

`uv cache clean <package>` should remove only the specified package from the cache.

```toml
# mdtest

[filters]
cache-entry = true
```

```toml
# file: requirements.txt

anyio
iniconfig
```

Install packages to populate the cache:

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + anyio==4.3.0
 + iniconfig==2.0.0
```

Clean only iniconfig:

```console
$ uv cache clean --verbose iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
DEBUG uv [VERSION] ([COMMIT] DATE)
DEBUG Acquired exclusive lock for `[CACHE_DIR]/`
DEBUG Removing dangling cache entry: [CACHE_DIR]/archive-v0/[HASH]
Removed [N] files ([SIZE])
DEBUG Released lock at `[CACHE_DIR]/.lock`
```

Running `uv cache prune` should have no effect (no unused entries):

```console
$ uv cache prune --verbose
success: true
exit_code: 0
----- stdout -----

----- stderr -----
DEBUG uv [VERSION] ([COMMIT] DATE)
DEBUG Acquired exclusive lock for `[CACHE_DIR]/`
Pruning cache at: [CACHE_DIR]/
No unused entries found
DEBUG Released lock at `[CACHE_DIR]/.lock`
```

## Clean single package (custom index)

<!-- Derived from [`cache_clean::clean_package_index`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_clean.rs#L172-L235) -->

`uv cache clean <package>` should also work for packages from a custom index.

```toml
# mdtest

[filters]
cache-entry = true
```

```toml
# file: requirements.txt

anyio
iniconfig
```

Install packages from TestPyPI to populate the cache:

```console
$ uv pip sync requirements.txt --index-url https://test.pypi.org/simple
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + anyio==3.5.0
 + iniconfig==2.0.0
```

Clean only iniconfig:

```console
$ uv cache clean --verbose iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
DEBUG uv [VERSION] ([COMMIT] DATE)
DEBUG Acquired exclusive lock for `[CACHE_DIR]/`
DEBUG Removing dangling cache entry: [CACHE_DIR]/archive-v0/[HASH]
Removed [N] files ([SIZE])
DEBUG Released lock at `[CACHE_DIR]/.lock`
```
