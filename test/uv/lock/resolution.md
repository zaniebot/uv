# Lock Resolution

Tests for lock resolution modes and strategies.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Resolution modes

### Default highest resolution

<!-- from lock.rs::lock_resolution_mode -->

By default, uv resolves to the highest compatible version.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio>=3"]
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

### Lowest-direct resolution

<!-- from lock.rs::lock_resolution_mode -->

Using `--resolution lowest-direct` ignores the existing lockfile.

```console
$ uv lock --resolution lowest-direct
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Ignoring existing lockfile due to change in resolution mode: `highest` vs. `lowest-direct`
Resolved 4 packages in [TIME]
```

## Version specifiers

### Prefix match version

<!-- from lock.rs::lock_prefix_match -->

Using a prefix match version specifier that doesn't exist should fail.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==5.4.*"]
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because only anyio<=4.3.0 is available and your project depends on anyio==5.4.*, we can conclude that your project's requirements are unsatisfiable.
```

## Lowest resolution warnings

### Warn about unpinned transitive dependencies

<!-- from lock.rs::lock_warn_missing_transitive_lower_bounds -->

Using `--resolution lowest` warns about unpinned transitive dependencies.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["pytest>8"]
```

```console
$ uv lock --resolution lowest
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
warning: The transitive dependency `packaging` is unpinned. Consider setting a lower bound with a constraint when using `--resolution lowest` to avoid using outdated versions.
warning: The transitive dependency `iniconfig` is unpinned. Consider setting a lower bound with a constraint when using `--resolution lowest` to avoid using outdated versions.
warning: The transitive dependency `colorama` is unpinned. Consider setting a lower bound with a constraint when using `--resolution lowest` to avoid using outdated versions.
```
