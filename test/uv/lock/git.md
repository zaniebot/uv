# Lock Git Dependencies

Tests for locking packages from Git repositories.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = ["pypi", "git"]
```

## Git with tag

### Lock Git dependency with tag

<!-- from lock.rs::lock_sdist_git -->

Lock a Git dependency using `tool.uv.sources` with a tag.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["uv-public-pypackage"]

[tool.uv.sources]
uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify offline works.

```console
$ uv lock --locked --offline --no-cache
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-public-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-public-pypackage@0dacfd662c64cb4ceb16e6cf65a157a8b715b979)
```

## Git with subdirectory

### Lock Git dependency with subdirectory

<!-- from lock.rs::lock_sdist_git_subdirectory -->

Lock a Git dependency with a subdirectory using PEP 508 syntax.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["example-pkg-a @ git+https://github.com/pypa/sample-namespace-packages.git@df7530eeb8fa0cb7dbb8ecb28363e8e36bfa2f45#subdirectory=pkg_resources/pkg_a"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + example-pkg-a==1 (from git+https://github.com/pypa/sample-namespace-packages.git@df7530eeb8fa0cb7dbb8ecb28363e8e36bfa2f45#subdirectory=pkg_resources/pkg_a)
```

## Git with PEP 508

### Lock Git dependency with PEP 508 tag

<!-- from lock.rs::lock_sdist_git_pep508 -->

Lock a Git dependency using PEP 508 syntax with a tag reference.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["uv-public-pypackage @ git+https://github.com/astral-test/uv-public-pypackage.git@0.0.1"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Git SHA

### Lock Git dependency with full SHA

<!-- from lock.rs::lock_git_sha -->

Lock a Git dependency using a full commit SHA.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["uv-public-pypackage"]

[tool.uv.sources]
uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", rev = "0dacfd662c64cb4ceb16e6cf65a157a8b715b979" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Git short rev

### Lock Git dependency with short rev

<!-- from lock.rs::lock_sdist_git_short_rev -->

Lock a Git dependency using a short revision.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["uv-public-pypackage"]

[tool.uv.sources]
uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", rev = "0dacfd6" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Git with plus prefix

### Lock git+ prefix is handled

<!-- from lock.rs::lock_git_plus_prefix -->

The `git+` prefix in URLs is handled correctly.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["uv-public-pypackage"]

[tool.uv.sources]
uv-public-pypackage = { git = "git+https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Partial Git resolution

### Partial Git resolution is disallowed

<!-- from lock.rs::lock_partial_git -->

Providing both a tag and a commit SHA is not allowed.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["uv-public-pypackage"]

[tool.uv.sources]
uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1", rev = "0dacfd662c64cb4ceb16e6cf65a157a8b715b979" }
```

```console
$ uv lock
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 9, column 24
  |
9 | uv-public-pypackage = { git = "https://github.com/astral-test/uv-public-pypackage", tag = "0.0.1", rev = "0dacfd662c64cb4ceb16e6cf65a157a8b715b979" }
  |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
cannot specify both `tag` and `rev`
```
