# Lock URLs

Tests for locking packages from direct URLs.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Wheel URLs

### Lock a wheel from URL

<!-- from lock.rs::lock_wheel_url -->

Lock a dependency specified as a direct URL to a wheel.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio @ https://files.pythonhosted.org/packages/14/fd/2f20c40b45e4fb4324834aea24bd4afdf1143390242c0b33774da0e2e34f/anyio-4.3.0-py3-none-any.whl"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

URL-based wheels have mutable metadata, so `--offline` fails without cache.

```console
$ uv lock --locked --offline --no-cache
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to generate package metadata for `anyio==4.3.0 @ direct+https://files.pythonhosted.org/packages/14/fd/2f20c40b45e4fb4324834aea24bd4afdf1143390242c0b33774da0e2e34f/anyio-4.3.0-py3-none-any.whl`
  Caused by: Network connectivity is disabled, but the requested data wasn't found in the cache for: `https://files.pythonhosted.org/packages/14/fd/2f20c40b45e4fb4324834aea24bd4afdf1143390242c0b33774da0e2e34f/anyio-4.3.0-py3-none-any.whl`
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 2 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0 (from https://files.pythonhosted.org/packages/14/fd/2f20c40b45e4fb4324834aea24bd4afdf1143390242c0b33774da0e2e34f/anyio-4.3.0-py3-none-any.whl)
 + idna==3.6
 + sniffio==1.3.1
```

## Source distribution URLs

### Lock an sdist from URL

<!-- from lock.rs::lock_sdist_url -->

Lock a dependency specified as a direct URL to a source distribution.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio @ https://files.pythonhosted.org/packages/db/4d/3970183622f0330d3c23d9b8a5f52e365e50381fd484d08e3285104333d3/anyio-4.3.0.tar.gz"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.3.0 (from https://files.pythonhosted.org/packages/db/4d/3970183622f0330d3c23d9b8a5f52e365e50381fd484d08e3285104333d3/anyio-4.3.0.tar.gz)
 + idna==3.6
 + sniffio==1.3.1
```

## URL with subdirectory

### Lock URL with subdirectory via sources

<!-- from lock.rs::lock_sdist_url_subdirectory -->

Lock a dependency from a URL with a subdirectory using `tool.uv.sources`.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["root"]

[tool.uv.sources]
root = { url = "https://github.com/user-attachments/files/18216295/subdirectory-test.tar.gz", subdirectory = "packages/root" }
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + root==0.0.1 (from https://github.com/user-attachments/files/18216295/subdirectory-test.tar.gz#subdirectory=packages/root)
 + sniffio==1.3.1
```

### Lock URL with subdirectory via PEP 508

<!-- from lock.rs::lock_sdist_url_subdirectory_pep508 -->

Lock a dependency from a URL with a subdirectory using PEP 508 syntax.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["root @ https://github.com/user-attachments/files/18216295/subdirectory-test.tar.gz#subdirectory=packages/root"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Verify with `--locked`.

```console
$ uv lock --locked
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Install from the lockfile.

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + idna==3.6
 + root==0.0.1 (from https://github.com/user-attachments/files/18216295/subdirectory-test.tar.gz#subdirectory=packages/root)
 + sniffio==1.3.1
```
