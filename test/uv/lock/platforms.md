# Lock Platforms

Tests for locking with platform-specific constraints using `tool.uv.environments`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Android platform

### Lock for Android

<!-- from lock.rs::lock_android -->

Lock packages for Android platform using `tool.uv.environments`.

```toml
# mdtest

[environment]
exclude-newer = "2025-06-01T00:00:00Z"
```

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "deltachat-rpc-server",
]

[tool.uv]
environments = ["sys_platform == 'android'"]
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

Verify offline with `--locked`.

```console
$ uv lock --locked --offline --no-cache
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## Platform intersection

### Lock with environment intersection

<!-- from lock.rs::lock_required_intersection -->

Lock packages with multiple platform constraints.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "numpy",
]

[tool.uv]
environments = [
  "(sys_platform=='linux' and platform_machine=='x86_64')",
  "(platform_machine=='arm64' and sys_platform=='darwin')"
]
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

Verify offline with `--locked`.

```console
$ uv lock --locked --offline --no-cache
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## ARM64 platform

### Lock for ARM64

<!-- from lock.rs::lock_arm -->

Lock packages for ARM64 platform.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["numpy"]

[tool.uv]
environments = ["platform_machine == 'arm64'"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## x86_64 platform

### Lock for x86_64

<!-- from lock.rs::lock_x86_64 -->

Lock packages for x86_64 platform.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["numpy"]

[tool.uv]
environments = ["platform_machine == 'x86_64'"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

## x86 (32-bit) platform

### Lock for x86

<!-- from lock.rs::lock_x86 -->

Lock packages for 32-bit x86 platform.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["numpy"]

[tool.uv]
environments = ["platform_machine == 'i386'"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```
