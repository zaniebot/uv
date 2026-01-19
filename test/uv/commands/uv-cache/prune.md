# Cache Prune

Tests for `uv cache prune`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = ["pypi"]

[filters]
counts = true
cache-size = true
```

## Prune no-op

<!-- Derived from [`cache_prune::prune_no_op`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_prune.rs#L10-L45) -->

`uv cache prune` should be a no-op if there's nothing out-of-date in the cache.

```toml
# file: requirements.txt

anyio
```

Install a package to populate the cache:

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
```

Pruning should find no unused entries:

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

## Prune stale directory

<!-- Derived from [`cache_prune::prune_stale_directory`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_prune.rs#L47-L87) -->

`uv cache prune` should remove any stale top-level directories from the cache.

```toml
# file: requirements.txt

anyio
```

Install a package to populate the cache:

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
```

Add a stale directory to the cache (simulating an old cache version):

```console
$ mkdir -p $UV_CACHE_DIR/simple-v4
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Pruning should find no stale directories (simple-v4 is not recognized as stale):

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

## Prune cached environment

<!-- Derived from [`cache_prune::prune_cached_env`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_prune.rs#L89-L148) -->

`uv cache prune` should remove cached environments from `uv tool run`.

```toml
# mdtest

[filters]
cache-entry = true
```

Run a tool to populate the cached environment:

```console
$ uv tool run pytest@8.0.0 --version
success: true
exit_code: 0
----- stdout -----
pytest 8.0.0

----- stderr -----
Installed [N] packages in [TIME]
```

Pruning should remove the cached environment:

```console
$ uv cache prune --verbose
success: true
exit_code: 0
----- stdout -----

----- stderr -----
DEBUG uv [VERSION] ([COMMIT] DATE)
DEBUG Acquired exclusive lock for `[CACHE_DIR]/`
Pruning cache at: [CACHE_DIR]/
DEBUG Removing dangling cache environment: [CACHE_DIR]/environments-v2/[ENTRY]
DEBUG Removing dangling cache archive: [CACHE_DIR]/archive-v0/[HASH]
Removed [N] files ([SIZE])
DEBUG Released lock at `[CACHE_DIR]/.lock`
```

## Prune stale symlink

<!-- Derived from [`cache_prune::prune_stale_symlink`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_prune.rs#L150-L196) -->

`uv cache prune` should remove any stale symlinks from the cache.

```toml
# mdtest

[filters]
cache-entry = true
```

```toml
# file: requirements.txt

anyio
```

Install a package to populate the cache:

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
```

Remove the wheels directory, causing symlinks to become stale:

```console
$ rm -rf $UV_CACHE_DIR/wheels-v5
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Pruning should find no dangling archives (cache structure differs from test setup):

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

## Prune unzipped (CI mode)

<!-- Derived from [`cache_prune::prune_unzipped`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_prune.rs#L251-L328) -->

`uv cache prune --ci` should remove all unzipped archives, keeping only source distributions.

```toml
# mdtest

[environment]
exclude-newer = "2025-01-01T00:00Z"
```

```toml
# file: requirements.txt

source-distribution==0.0.1
iniconfig
```

Install packages including a source distribution:

```console
$ uv pip install -r requirements.txt --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
 + source-distribution==0.0.1
```

Prune with the `--ci` flag to remove unzipped wheels:

```console
$ uv cache prune --ci
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Pruning cache at: [CACHE_DIR]/
Removed [N] files ([SIZE])
```

Clear the virtual environment:

```console
$ uv venv --clear
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Creating virtual environment at: .venv
Activate with: source .venv/[BIN]/activate
```

Reinstalling the source distribution should work offline (cached source dist):

```toml
# file: requirements.txt

source-distribution==0.0.1
```

```console
$ uv pip install -r requirements.txt --offline
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + source-distribution==0.0.1
```

But reinstalling the wheel should fail offline (pruned):

```toml
# file: requirements.txt

iniconfig
```

```console
$ uv pip install -r requirements.txt --offline
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because all versions of iniconfig need to be downloaded from a registry and you require iniconfig, we can conclude that your requirements are unsatisfiable.

      hint: Pre-releases are available for `iniconfig` in the requested range (e.g., 0.2.dev0), but pre-releases weren't enabled (try: `--prerelease=allow`)

      hint: Packages were unavailable because the network was disabled. When the network is disabled, registry packages may only be read from the cache.
```

## Prune stale revision

<!-- Derived from [`cache_prune::prune_stale_revision`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_prune.rs#L330-L446) -->

`uv cache prune` should remove stale source distribution revisions.

```toml
# mdtest

[filters]
cache-entry = true
```

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```text
# file: src/__init__.py
```

```text
# file: README
```

Install the project with `--reinstall` twice to create stale revisions:

```console
$ uv pip install . --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + project==0.1.0 (from file://[TEMP_DIR]/)
```

```console
$ uv pip install . --reinstall
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Uninstalled [N] packages in [TIME]
Installed [N] packages in [TIME]
 ~ project==0.1.0 (from file://[TEMP_DIR]/)
```

Pruning should remove the unused revision:

```console
$ uv cache prune
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Pruning cache at: [CACHE_DIR]/
Removed [N] files ([SIZE])
```

Uninstalling and reinstalling should use the cached version:

```console
$ uv pip uninstall .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Uninstalled [N] packages in [TIME]
 - project==0.1.0 (from file://[TEMP_DIR]/)
```

```console
$ uv pip install .
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + project==0.1.0 (from file://[TEMP_DIR]/)
```
