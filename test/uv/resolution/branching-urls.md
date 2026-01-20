# Branching URLs

Tests for URL branching with disjoint and overlapping markers.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Disjoint markers

<!-- Derived from [`branching_urls::branching_urls_disjoint`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L16-L39) -->

When dependencies specify different URLs for the same package with disjoint markers, resolution
succeeds.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    # Valid, disjoint split
    "iniconfig @ https://files.pythonhosted.org/packages/9b/dd/b3c12c6d707058fa947864b67f0c4e0c39ef8610988d7baea9578f3c48f3/iniconfig-1.1.1-py2.py3-none-any.whl ; python_version < '3.12'",
    "iniconfig @ https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl ; python_version >= '3.12'",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Overlapping markers

<!-- Derived from [`branching_urls::branching_urls_overlapping`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L50-L76) -->

When dependencies specify different URLs with overlapping markers, resolution fails.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    # Conflicting split
    "iniconfig @ https://files.pythonhosted.org/packages/9b/dd/b3c12c6d707058fa947864b67f0c4e0c39ef8610988d7baea9578f3c48f3/iniconfig-1.1.1-py2.py3-none-any.whl ; python_version < '3.12'",
    "iniconfig @ https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl ; python_version >= '3.11'",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to resolve dependencies for `a` (v0.1.0)
  ╰─▶ Requirements contain conflicting URLs for package `iniconfig` in split `python_full_version == '3.11.*'`:
      - https://files.pythonhosted.org/packages/9b/dd/b3c12c6d707058fa947864b67f0c4e0c39ef8610988d7baea9578f3c48f3/iniconfig-1.1.1-py2.py3-none-any.whl
      - https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl
```

## Root splits with transitive conflict

<!-- Derived from [`branching_urls::root_package_splits_but_transitive_conflict`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L88-L145) -->

When the root package has diverging URLs but transitive dependencies have conflicting URLs,
resolution fails.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    # Force a split
    "anyio==4.3.0 ; python_version >= '3.12'",
    "anyio==4.2.0 ; python_version < '3.12'",
    "b"
]

[tool.uv.sources]
b = { path = "b" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "b1",
    "b2",
]

[tool.uv.sources]
b1 = { path = "../b1" }
b2 = { path = "../b2" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b1/pyproject.toml

[project]
name = "b1"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig @ https://files.pythonhosted.org/packages/9b/dd/b3c12c6d707058fa947864b67f0c4e0c39ef8610988d7baea9578f3c48f3/iniconfig-1.1.1-py2.py3-none-any.whl",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b2/pyproject.toml

[project]
name = "b2"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig @ https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to resolve dependencies for `b2` (v0.1.0)
  ╰─▶ Requirements contain conflicting URLs for package `iniconfig` in split `python_full_version >= '3.12'`:
      - https://files.pythonhosted.org/packages/9b/dd/b3c12c6d707058fa947864b67f0c4e0c39ef8610988d7baea9578f3c48f3/iniconfig-1.1.1-py2.py3-none-any.whl
      - https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl
  help: `b2` (v0.1.0) was included because `a` (v0.1.0) depends on `b` (v0.1.0) which depends on `b2`
```

## Root splits with transitive splits too

<!-- Derived from [`branching_urls::root_package_splits_transitive_too`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L159-L337) -->

When both root and transitive dependencies have marker-based splits, resolution succeeds.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    # Force a split
    "anyio==4.3.0 ; python_version >= '3.12'",
    "anyio==4.2.0 ; python_version < '3.12'",
    "b"
]

[tool.uv.sources]
b = { path = "b" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "b1 ; python_version < '3.12'",
    "b2 ; python_version >= '3.12'",
]

[tool.uv.sources]
b1 = { path = "../b1" }
b2 = { path = "../b2" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b1/pyproject.toml

[project]
name = "b1"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig @ https://files.pythonhosted.org/packages/9b/dd/b3c12c6d707058fa947864b67f0c4e0c39ef8610988d7baea9578f3c48f3/iniconfig-1.1.1-py2.py3-none-any.whl",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b2/pyproject.toml

[project]
name = "b2"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig @ https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 10 packages in [TIME]
```

## Root splits with different dependencies per split

<!-- Derived from [`branching_urls::root_package_splits_other_dependencies_too`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L365-L537) -->

Root package can have different path dependencies per marker split.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    # Force a split
    "anyio==4.3.0 ; python_version >= '3.12'",
    "anyio==4.2.0 ; python_version < '3.12'",
    # These two are currently included in both parts of the split.
    "b1 ; python_version < '3.12'",
    "b2 ; python_version >= '3.12'",
]

[tool.uv.sources]
b1 = { path = "b1" }
b2 = { path = "b2" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b1/pyproject.toml

[project]
name = "b1"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig==1.1.1",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b2/pyproject.toml

[project]
name = "b2"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig==2.0.0"
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 9 packages in [TIME]
```

## Registry and direct URL branching

<!-- Derived from [`branching_urls::branching_between_registry_and_direct_url`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L549-L634) -->

Dependencies can branch between registry and direct URL sources based on markers.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig == 1.1.1 ; python_version < '3.12'",
    "iniconfig @ https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl ; python_version >= '3.12'",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Different URL sources with disjoint markers

<!-- Derived from [`branching_urls::branching_urls_of_different_sources_disjoint`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L635-L718) -->

Dependencies can use different URL sources (git, path) with disjoint markers.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig ; python_version < '3.12'",
    "iniconfig ; python_version >= '3.12'",
]

[tool.uv.sources]
iniconfig = [
    { git = "https://github.com/pytest-dev/iniconfig", tag = "v1.1.1", marker = "python_version < '3.12'" },
    { git = "https://github.com/pytest-dev/iniconfig", tag = "v2.0.0", marker = "python_version >= '3.12'" },
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

## Different URL sources with conflicting markers

<!-- Derived from [`branching_urls::branching_urls_of_different_sources_conflict`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L719-L748) -->

When URL sources have overlapping markers, resolution fails.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    "iniconfig ; python_version < '3.12'",
    "iniconfig ; python_version >= '3.11'",
]

[tool.uv.sources]
iniconfig = [
    { git = "https://github.com/pytest-dev/iniconfig", tag = "v1.1.1", marker = "python_version < '3.12'" },
    { git = "https://github.com/pytest-dev/iniconfig", tag = "v2.0.0", marker = "python_version >= '3.11'" },
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to resolve dependencies for `a` (v0.1.0)
  ╰─▶ Requirements contain conflicting URLs for package `iniconfig`:
      - git+https://github.com/pytest-dev/iniconfig@v1.1.1
      - git+https://github.com/pytest-dev/iniconfig@v2.0.0
```

## Don't pre-visit URL packages

<!-- Derived from [`branching_urls::dont_pre_visit_url_packages`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/branching_urls.rs#L749-L829) -->

URL packages should not be pre-visited as registry distributions.

```toml
# file: pyproject.toml

[project]
name = "a"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
    # This c is not a registry distribution, we must not pre-visit it as such.
    "c==0.1.0",
    "b",
]

[tool.uv.sources]
b = { path = "b" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: b/pyproject.toml

[project]
name = "b"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = [
  "c",
]

[tool.uv.sources]
c = { path = "../c" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: c/pyproject.toml

[project]
name = "c"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock --offline
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```
