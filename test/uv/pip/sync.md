# pip sync

Tests for `uv pip sync`.

## Error handling

### Missing requirements file

<!-- from pip_sync.rs::missing_requirements_txt -->

Running with a missing requirements file shows an error.

```console
$ uv pip sync requirements.txt --strict
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: File not found: `requirements.txt`
```

### Missing venv

<!-- from pip_sync.rs::missing_venv -->

Running with an active virtual environment that doesn't exist shows an error.

Note: The original test also validates the case where VIRTUAL_ENV is not set, which produces a
different error ("No virtual environment found; run `uv venv` to create an environment"). This
scenario cannot be tested in mdtest without framework changes.

```toml
# mdtest

[environment]
create-venv = false

[filters]
virtualenv-bin = true
python-names = true
```

```toml
# file: requirements.txt
anyio
```

```console
$ uv pip sync requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to inspect Python interpreter from active virtual environment at `.venv/[BIN]/[PYTHON]`
  Caused by: Python interpreter not found at `[VENV]/[BIN]/[PYTHON]`
```

## Basic installation

### Install a package

<!-- from pip_sync.rs::install -->

Install a package from a requirements file.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

### Install with copy semantics

<!-- from pip_sync.rs::install_copy -->

The `--link-mode=copy` flag forces copy semantics.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict --link-mode=copy
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

### Install with hardlink semantics

<!-- from pip_sync.rs::install_hardlink -->

The `--link-mode=hardlink` flag forces hardlink semantics.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict --link-mode=hardlink
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

## Empty requirements

### Sync empty requirements

<!-- from pip_sync.rs::pip_sync_empty -->

Syncing an empty requirements file warns by default.

```toml
# file: requirements.txt

```

```console
$ uv pip sync requirements.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Requirements file `requirements.txt` does not contain any dependencies
No requirements found (hint: use `--allow-empty-requirements` to clear the environment)
```

### Sync empty with allow flag

<!-- from pip_sync.rs::pip_sync_empty -->

The `--allow-empty-requirements` flag allows empty requirements.

```toml
# file: requirements.txt

```

```console
$ uv pip sync requirements.txt --allow-empty-requirements
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Requirements file `requirements.txt` does not contain any dependencies
Resolved in [TIME]
Audited in [TIME]
```

## Adding and removing packages

### Add and remove packages

<!-- from pip_sync.rs::add_remove -->

Syncing replaces packages with those in the requirements file.

```toml
# file: requirements.txt
iniconfig==2.0.0
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

```toml
# file: requirements.txt
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 - iniconfig==2.0.0
 + tomli==2.0.1
```

### Add a package sequentially

<!-- from pip_sync.rs::install_sequential -->

Adding a package to requirements adds it to the environment.

```toml
# file: requirements.txt
iniconfig==2.0.0
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

```toml
# file: requirements.txt
iniconfig==2.0.0
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + tomli==2.0.1
```

## Version upgrade

### Upgrade a package

<!-- from pip_sync.rs::upgrade -->

Changing the version in requirements upgrades the package.

```toml
# file: requirements.txt
tomli==2.0.0
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + tomli==2.0.0
```

```toml
# file: requirements.txt
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 - tomli==2.0.0
 + tomli==2.0.1
```

## Reinstall

### Reinstall all packages

<!-- from pip_sync.rs::reinstall -->

The `--reinstall` flag reinstalls all packages.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + markupsafe==2.1.3
 + tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --reinstall --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Uninstalled 2 packages in [TIME]
Installed 2 packages in [TIME]
 ~ markupsafe==2.1.3
 ~ tomli==2.0.1
```

### Reinstall specific package

<!-- from pip_sync.rs::reinstall_package -->

The `--reinstall-package` flag reinstalls a specific package.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + markupsafe==2.1.3
 + tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --reinstall-package tomli --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 ~ tomli==2.0.1
```

## Multiple packages

### Install many packages

<!-- from pip_sync.rs::install_many -->

Install multiple packages from a requirements file.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
tomli==2.0.1
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Installed 2 packages in [TIME]
 + markupsafe==2.1.3
 + tomli==2.0.1
```

## No-op sync

### Noop when already installed

<!-- from pip_sync.rs::noop -->

Syncing when packages are already installed is a no-op.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Audited 1 package in [TIME]
```

## Link mode

### Verify link mode

<!-- from pip_sync.rs::link -->

Verify that package linking uses the expected mode.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

## Yanked versions

### Warn on yanked

<!-- from pip_sync.rs::warn_on_yanked -->

Sync warns when installing yanked versions.

```toml
# file: requirements.txt
colorama==0.4.2
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + colorama==0.4.2
warning: `colorama==0.4.2` is yanked (reason: "Bad build, missing files, will not install")
```

### Warn on yanked dry run

<!-- from pip_sync.rs::warn_on_yanked_dry_run -->

Dry run also warns on yanked versions.

```toml
# file: requirements.txt
colorama==0.4.2
```

```console
$ uv pip sync requirements.txt --strict --dry-run
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Would download 1 package
Would install 1 package
 + colorama==0.4.2
warning: `colorama==0.4.2` is yanked (reason: "Bad build, missing files, will not install")
```

## Git dependencies

```toml
# mdtest

[environment]
required-features = ["pypi", "git"]
```

### Install from Git commit

<!-- from pip_sync.rs::install_git_commit -->

Install a package from a Git repository using a commit SHA.

```toml
# file: requirements.txt
uv-public-pypackage @ git+https://github.com/astral-test/uv-public-pypackage@b270df1a2fb5d012294e9aaf05e7e0bab1e6a389
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@b270df1a2fb5d012294e9aaf05e7e0bab1e6a389)
```

### Install from Git tag

<!-- from pip_sync.rs::install_git_tag -->

Install a package from a Git repository using a tag.

```toml
# file: requirements.txt
uv-public-pypackage @ git+https://github.com/astral-test/uv-public-pypackage@test-tag
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

## URL dependencies

### Install from URL

<!-- from pip_sync.rs::install_url -->

Install a package from a direct URL.

```toml
# file: requirements.txt
werkzeug @ https://files.pythonhosted.org/packages/ff/1d/960bb4017c68674a1cb099534840f18d3def3ce44aed12b5ed8b78e0153e/Werkzeug-2.0.0-py3-none-any.whl
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + werkzeug==2.0.0 (from https://files.pythonhosted.org/packages/ff/1d/960bb4017c68674a1cb099534840f18d3def3ce44aed12b5ed8b78e0153e/Werkzeug-2.0.0-py3-none-any.whl)
```

## No index

### Install with no index

<!-- from pip_sync.rs::install_no_index -->

The `--no-index` flag disables package index lookups.

```toml
# file: requirements.txt
iniconfig==2.0.0
```

```console
$ uv pip sync requirements.txt --no-index --strict
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because iniconfig was not found in the provided package locations and you require iniconfig==2.0.0, we can conclude that your requirements are unsatisfiable.

      hint: Packages were unavailable because index lookups were disabled and no additional package locations were provided (try: `--find-links <uri>`)
```

## Constraints

### Compatible constraint

<!-- from pip_sync.rs::compatible_constraint -->

Sync with a compatible constraint succeeds.

```toml
# file: requirements.txt
anyio==3.7.0
```

```toml
# file: constraints.txt
anyio==3.7.0
```

```console
$ uv pip sync requirements.txt --constraint constraints.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==3.7.0
```

### Incompatible constraint

<!-- from pip_sync.rs::incompatible_constraint -->

Sync with an incompatible constraint fails.

```toml
# file: requirements.txt
anyio==3.7.0
```

```toml
# file: constraints.txt
anyio==3.6.0
```

```console
$ uv pip sync requirements.txt --constraint constraints.txt
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because you require anyio==3.7.0 and anyio==3.6.0, we can conclude that your requirements are unsatisfiable.
```

### Irrelevant constraint

<!-- from pip_sync.rs::irrelevant_constraint -->

Sync with an irrelevant constraint succeeds.

```toml
# file: requirements.txt
anyio==3.7.0
```

```toml
# file: constraints.txt
black==23.10.1
```

```console
$ uv pip sync requirements.txt --constraint constraints.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==3.7.0
```

## Repeated requirements

### Repeated identical requirements

<!-- from pip_sync.rs::repeat_requirement_identical -->

Sync with repeated identical requirements succeeds.

```toml
# file: requirements.in
anyio
anyio
```

```console
$ uv pip sync requirements.in
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==4.3.0
```

## Symlink mode

### Install with symlink semantics

<!-- from pip_sync.rs::install_symlink -->

The `--link-mode=symlink` flag uses symlink semantics for installation.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict --link-mode=symlink
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

### Symlink with no-cache fails

<!-- from pip_sync.rs::install_symlink_no_cache -->

The `--link-mode=symlink` flag is incompatible with `--no-cache`.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --strict --link-mode=symlink --no-cache
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
error: Symlink-based installation is not supported with `--no-cache`. The created environment will be rendered unusable by the removal of the cache.
```

## Source distributions

### Install from source distribution

<!-- from pip_sync.rs::install_sdist -->

Installing a package from a source distribution.

```toml
# file: requirements.txt
source-distribution==0.0.1
```

```console
$ uv pip sync requirements.txt --strict --exclude-newer 2025-01-29T00:00:00Z
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + source-distribution==0.0.1
```

### Install from source distribution URL

<!-- from pip_sync.rs::install_sdist_url -->

Installing from a direct source distribution URL.

```toml
# file: requirements.txt
source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
```

### Repeated compatible requirements

<!-- from pip_sync.rs::repeat_requirement_compatible -->

Sync with repeated compatible requirements succeeds.

```toml
# file: requirements.in
anyio
anyio==4.0.0
```

```console
$ uv pip sync requirements.in
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==4.0.0
```

### Repeated incompatible requirements

<!-- from pip_sync.rs::repeat_requirement_incompatible -->

Sync with repeated incompatible requirements fails.

```toml
# file: requirements.in
anyio<4.0.0
anyio==4.0.0
```

```console
$ uv pip sync requirements.in
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because you require anyio<4.0.0 and anyio==4.0.0, we can conclude that your requirements are unsatisfiable.
```

## Duplicate packages

### Duplicate package with overlapping versions

<!-- from pip_sync.rs::duplicate_package_overlap -->

Sync with overlapping conflicting versions fails.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
MarkupSafe==2.1.2
```

```console
$ uv pip sync requirements.txt --strict
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because you require markupsafe==2.1.3 and markupsafe==2.1.2, we can conclude that your requirements are unsatisfiable.
```

### Duplicate package with disjoint markers

<!-- from pip_sync.rs::duplicate_package_disjoint -->

Sync with disjoint marker-based versions succeeds.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
MarkupSafe==2.1.2 ; python_version < '3.6'
```

```console
$ uv pip sync requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + markupsafe==2.1.3
```

### Incompatible wheel format

<!-- from pip_sync.rs::incompatible_wheel -->

Sync with an invalid wheel file fails with a clear error.

```toml
# file: foo-1.2.3-py3-none-any.whl

```

```toml
# file: requirements.txt
foo @ ./foo-1.2.3-py3-none-any.whl
```

```console
$ uv pip sync requirements.txt --strict
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because foo has an invalid package format and you require foo, we can conclude that your requirements are unsatisfiable.

      hint: The structure of `foo` was invalid
        Caused by: Failed to read from zip file
        Caused by: unable to locate the end of central directory record
```

## Hash validation

### Unknown hash algorithm

<!-- from pip_sync.rs::require_hashes_unknown_algorithm -->

Using an unknown hash algorithm fails with a clear error.

```toml
# file: requirements.txt
anyio==4.0.0 --hash=foo:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unsupported hash algorithm (expected one of: `md5`, `sha256`, `sha384`, `sha512`, or `blake2b`) on: `foo`
```

### Missing hash with require-hashes

<!-- from pip_sync.rs::require_hashes_missing_hash -->

Omitting the hash with `--require-hashes` fails.

```toml
# file: requirements.txt
anyio==4.0.0
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: In `--require-hashes` mode, all requirements must have a hash, but none were provided for: anyio==4.0.0
```

### Missing version with require-hashes

<!-- from pip_sync.rs::require_hashes_missing_version -->

Omitting the version with `--require-hashes` fails.

```toml
# file: requirements.txt
anyio --hash=sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: In `--require-hashes` mode, all requirements must have their versions pinned with `==`, but found: anyio
```

### Invalid operator with require-hashes

<!-- from pip_sync.rs::require_hashes_invalid_operator -->

Using a non-`==` operator with `--require-hashes` fails.

```toml
# file: requirements.txt
anyio>4.0.0 --hash=sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: In `--require-hashes` mode, all requirements must have their versions pinned with `==`, but found: anyio>4.0.0
```

### Wheel hash with only-binary

<!-- from pip_sync.rs::require_hashes_wheel_only_binary -->

Include the hash for the wheel with `--only-binary`.

```toml
# file: requirements.txt
anyio==4.0.0 --hash=sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --only-binary :all: --require-hashes
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==4.0.0
```

### Wheel hash with no-binary

<!-- from pip_sync.rs::require_hashes_wheel_no_binary -->

Include the hash for the wheel with `--no-binary` fails (uses source distribution).

```toml
# file: requirements.txt
anyio==4.0.0 --hash=sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --no-binary :all: --require-hashes
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
  × Failed to download and build `anyio==4.0.0`
  ╰─▶ Hash mismatch for `anyio==4.0.0`

      Expected:
        sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f

      Computed:
        sha256:f7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a
```

### Source hash with no-binary

<!-- from pip_sync.rs::require_hashes_source_no_binary -->

Include the hash for the source distribution with `--no-binary`.

```toml
# file: requirements.txt
source-distribution==0.0.1 --hash=sha256:1f83ed7498336c7f2ab9b002cf22583d91115ebc624053dc4eb3a45694490106
```

```console
$ uv pip sync requirements.txt --no-binary :all: --require-hashes --exclude-newer 2025-01-29T00:00:00Z
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + source-distribution==0.0.1
```

### Source hash with only-binary

<!-- from pip_sync.rs::require_hashes_source_only_binary -->

Include the hash for the source distribution with `--only-binary` fails (requires wheel).

```toml
# file: requirements.txt
anyio==4.0.0 --hash=sha256:f7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a
```

```console
$ uv pip sync requirements.txt --only-binary :all: --require-hashes
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
  × Failed to download `anyio==4.0.0`
  ╰─▶ Hash mismatch for `anyio==4.0.0`

      Expected:
        sha256:f7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a

      Computed:
        sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

### Wrong hash digest

<!-- from pip_sync.rs::require_hashes_wrong_digest -->

Using the correct algorithm but wrong digest fails.

```toml
# file: requirements.txt
anyio==4.0.0 --hash=sha256:afdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
  × Failed to download `anyio==4.0.0`
  ╰─▶ Hash mismatch for `anyio==4.0.0`

      Expected:
        sha256:afdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f

      Computed:
        sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

### Wrong hash algorithm

<!-- from pip_sync.rs::require_hashes_wrong_algorithm -->

Using the wrong hash algorithm (digest correct for a different algorithm) fails.

```toml
# file: requirements.txt
anyio==4.0.0 --hash=sha512:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
  × Failed to download `anyio==4.0.0`
  ╰─▶ Hash mismatch for `anyio==4.0.0`

      Expected:
        sha512:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f

      Computed:
        sha512:f30761c1e8725b49c498273b90dba4b05c0fd157811994c806183062cb6647e773364ce45f0e1ff0b10e32fe6d0232ea5ad39476ccf37109d6b49603a09c11c2
```

### Source URL with hash

<!-- from pip_sync.rs::require_hashes_source_url -->

Include the hash for a source distribution specified as a direct URL dependency.

```toml
# file: requirements.txt
source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz --hash=sha256:1f83ed7498336c7f2ab9b002cf22583d91115ebc624053dc4eb3a45694490106
```

```console
$ uv pip sync requirements.txt --require-hashes
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + source-distribution==0.0.1 (from https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz)
```

### Source URL with wrong hash

<!-- from pip_sync.rs::require_hashes_source_url_mismatch -->

Include the wrong hash for a source distribution specified as a direct URL dependency.

```toml
# file: requirements.txt
source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz --hash=sha256:a7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to download and build `source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz`
  ╰─▶ Hash mismatch for `source-distribution @ https://files.pythonhosted.org/packages/10/1f/57aa4cce1b1abf6b433106676e15f9fa2c92ed2bd4cf77c3b50a9e9ac773/source_distribution-0.0.1.tar.gz`

      Expected:
        sha256:a7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a

      Computed:
        sha256:1f83ed7498336c7f2ab9b002cf22583d91115ebc624053dc4eb3a45694490106
```

### Wheel URL with hash

<!-- from pip_sync.rs::require_hashes_wheel_url -->

Include the hash for a wheel specified as a direct URL dependency.

```toml
# file: requirements.txt
anyio @ https://files.pythonhosted.org/packages/36/55/ad4de788d84a630656ece71059665e01ca793c04294c463fd84132f40fe6/anyio-4.0.0-py3-none-any.whl --hash=sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --require-hashes
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==4.0.0 (from https://files.pythonhosted.org/packages/36/55/ad4de788d84a630656ece71059665e01ca793c04294c463fd84132f40fe6/anyio-4.0.0-py3-none-any.whl)
```

### Wheel URL with wrong hash

<!-- from pip_sync.rs::require_hashes_wheel_url_mismatch -->

Include the wrong hash for a wheel specified as a direct URL dependency.

```toml
# file: requirements.txt
anyio @ https://files.pythonhosted.org/packages/36/55/ad4de788d84a630656ece71059665e01ca793c04294c463fd84132f40fe6/anyio-4.0.0-py3-none-any.whl --hash=sha256:afdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```

```console
$ uv pip sync requirements.txt --require-hashes
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
  × Failed to download `anyio @ https://files.pythonhosted.org/packages/36/55/ad4de788d84a630656ece71059665e01ca793c04294c463fd84132f40fe6/anyio-4.0.0-py3-none-any.whl`
  ╰─▶ Hash mismatch for `anyio @ https://files.pythonhosted.org/packages/36/55/ad4de788d84a630656ece71059665e01ca793c04294c463fd84132f40fe6/anyio-4.0.0-py3-none-any.whl`

      Expected:
        sha256:afdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f

      Computed:
        sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```
