# Lock Hashes

Tests for hash handling during lock operations.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Invalid hash in lockfile

### Lock with invalid hash fails

<!-- from lock.rs::lock_invalid_hash -->

A lockfile with an invalid hash should fail `--locked` and `--frozen` verification.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]
```

Create a lockfile with an invalid hash for `idna`:

```toml
# file: uv.lock
version = 1
revision = 2
requires-python = ">=3.12"

[options]
exclude-newer = "2024-03-25T00:00:00Z"

[[package]]
name = "anyio"
version = "3.7.0"
source = { registry = "https://pypi.org/simple" }
dependencies = [
    { name = "idna" },
    { name = "sniffio" },
]
sdist = { url = "https://files.pythonhosted.org/packages/c6/b3/fefbf7e78ab3b805dec67d698dc18dd505af7a18a8dd08868c9b4fa736b5/anyio-3.7.0.tar.gz", hash = "sha256:275d9973793619a5374e1c89a4f4ad3f4b0a5510a2b5b939444bee8f4c4d37ce", size = 142737 }
wheels = [
    { url = "https://files.pythonhosted.org/packages/68/fe/7ce1926952c8a403b35029e194555558514b365ad77d75125f521a2bec62/anyio-3.7.0-py3-none-any.whl", hash = "sha256:eddca883c4175f14df8aedce21054bfca3adb70ffe76a9f607aef9d7fa2ea7f0", size = 80873 },
]

[[package]]
name = "idna"
version = "3.6"
source = { registry = "https://pypi.org/simple" }
sdist = { url = "https://files.pythonhosted.org/packages/bf/3f/ea4b9117521a1e9c50344b909be7886dd00a519552724809bb1f486986c2/idna-3.6.tar.gz", hash = "sha256:aecdbbd083b06798ae1e86adcbfe8ab1479cf864e4ee30fe4e46a003d12491ca", size = 175426 }
wheels = [
    { url = "https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl", hash = "sha256:d05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f", size = 61567 },
]

[[package]]
name = "project"
version = "0.1.0"
source = { editable = "." }
dependencies = [
    { name = "anyio" },
]
requires-dist = [{ name = "anyio", specifier = "==3.7.0" }]

[[package]]
name = "sniffio"
version = "1.3.1"
source = { registry = "https://pypi.org/simple" }
sdist = { url = "https://files.pythonhosted.org/packages/a2/87/a6771e1546d97e7e041b6ae58d80074f81b7d5121207425c964ddf5cfdbd/sniffio-1.3.1.tar.gz", hash = "sha256:f4324edc670a0f49750a81b895f35c3adb843cca46f0530f79fc1babb23789dc", size = 20372 }
wheels = [
    { url = "https://files.pythonhosted.org/packages/e9/44/75a9c9421471a6c4805dbf2356f7c181a29c1879239abab1ea2cc8f38b40/sniffio-1.3.1-py3-none-any.whl", hash = "sha256:2f6da418d1f1e0fddd844478f41680e794e6051915791a034ff65e5f100525a2", size = 10235 },
]
```

The `--locked` flag should detect the outdated lockfile:

```console
$ uv lock --locked
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
```

### Sync with invalid hash fails

<!-- from lock.rs::lock_invalid_hash -->

Installing from a lockfile with an invalid hash should fail hash verification.

```console
$ uv sync --frozen
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to download `idna==3.6`
  ╰─▶ Hash mismatch for `idna==3.6`

      Expected:
        sha256:d05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f

      Computed:
        sha256:c05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f
      help: `idna` (v3.6) was included because `project` (v0.1.0) depends on `anyio` (v3.7.0) which depends on `idna`
```
