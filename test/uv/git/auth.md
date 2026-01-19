# Git Authentication

Tests for Git repository authentication.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = ["git", "git-token"]
```

## Private source

<!-- Derived from [`edit::add_git_private_source`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git requirement from a private repository with credentials. The resolution should succeed,
but the `pyproject.toml` should omit the credentials.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add uv-private-pypackage@git+https://${GITHUB_TOKEN}@github.com/astral-test/uv-private-pypackage
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-private-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-private-pypackage@d780faf0ac91257d4d5a4f0c5a0e4509608c0071)
```

The `pyproject.toml` should have the source configured without credentials:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "uv-private-pypackage",
]

[tool.uv.sources]
uv-private-pypackage = { git = "https://github.com/astral-test/uv-private-pypackage" }
```

Install from the lockfile:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 1 package in [TIME]
```

## Private raw

<!-- Derived from [`edit::add_git_private_raw`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git requirement from a private repository with `--raw-sources`. The `pyproject.toml` should
retain the credentials (redacted in output).

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add uv-private-pypackage@git+https://${GITHUB_TOKEN}@github.com/astral-test/uv-private-pypackage --raw-sources
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + uv-private-pypackage==0.1.0 (from git+https://github.com/astral-test/uv-private-pypackage@d780faf0ac91257d4d5a4f0c5a0e4509608c0071)
```

Install from the lockfile:

```console
$ uv sync --frozen
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Audited 1 package in [TIME]
```
