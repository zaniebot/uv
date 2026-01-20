# Hash Verification

Tests for hash verification during `uv pip install`.

## Require hashes

### Install with valid hashes

<!-- from pip_install.rs::require_hashes -->

The `--require-hashes` flag requires hashes for all packages.

```toml
# file: requirements.txt
anyio==4.0.0 \
    --hash=sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f \
    --hash=sha256:f7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a
idna==3.6 \
    --hash=sha256:9ecdbbd083b06798ae1e86adcbfe8ab1479cf864e4ee30fe4e46a003d12491ca \
    --hash=sha256:c05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f
    # via anyio
sniffio==1.3.1 \
    --hash=sha256:2f6da418d1f1e0fddd844478f41680e794e6051915791a034ff65e5f100525a2 \
    --hash=sha256:f4324edc670a0f49750a81b895f35c3adb843cca46f0530f79fc1babb23789dc
    # via anyio
```

```console
$ uv pip install -r requirements.txt --require-hashes
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
Prepared 3 packages in [TIME]
Installed 3 packages in [TIME]
 + anyio==4.0.0
 + idna==3.6
 + sniffio==1.3.1
```

### Require hashes with no deps

<!-- from pip_install.rs::require_hashes_no_deps -->

With `--no-deps`, only top-level packages need hashes.

```toml
# file: requirements.txt
anyio==4.0.0 \
    --hash=sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f \
    --hash=sha256:f7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a
```

```console
$ uv pip install -r requirements.txt --no-deps --require-hashes
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + anyio==4.0.0
```

### Require hashes mismatch

<!-- from pip_install.rs::require_hashes_mismatch -->

Hash mismatch raises an error.

```toml
# file: requirements.txt
anyio==4.0.0 \
    --hash=sha256:afdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f \
    --hash=sha256:a7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a
idna==3.6 \
    --hash=sha256:9ecdbbd083b06798ae1e86adcbfe8ab1479cf864e4ee30fe4e46a003d12491ca \
    --hash=sha256:c05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f
    # via anyio
sniffio==1.3.1 \
    --hash=sha256:2f6da418d1f1e0fddd844478f41680e794e6051915791a034ff65e5f100525a2 \
    --hash=sha256:f4324edc670a0f49750a81b895f35c3adb843cca46f0530f79fc1babb23789dc
    # via anyio
```

```console
$ uv pip install -r requirements.txt --require-hashes
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
  × Failed to download `anyio==4.0.0`
  ╰─▶ Hash mismatch for `anyio==4.0.0`

      Expected:
        sha256:afdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
        sha256:a7ed51751b2c2add651e5747c891b47e26d2a21be5d32d9311dfe9692f3e5d7a

      Computed:
        sha256:cfdb2b588b9fc25ede96d8db56ed50848b0b649dca3dd1df0b11f683bb9e0b5f
```
