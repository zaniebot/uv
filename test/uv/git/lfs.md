# Git LFS

Tests for Git LFS (Large File Storage) support.

```toml
# mdtest

[environment]
python-version = "3.13"
required-features = ["git-lfs", "pypi"]
```

## Basic

<!-- Derived from [`edit::add_git_lfs`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Adding a Git dependency with LFS support.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.13"
dependencies = []
```

```console
$ uv add test-lfs-repo@git+https://github.com/astral-sh/test-lfs-repo --rev 657500f0703dc173ac5d68dfa1d7e8c985c84424 --lfs --no-cache
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + test-lfs-repo==0.1.0 (from git+https://github.com/astral-sh/test-lfs-repo@657500f0703dc173ac5d68dfa1d7e8c985c84424#lfs=true)
```

The `pyproject.toml` should have the source configured with LFS enabled:

```toml title="pyproject.toml" snapshot=true
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.13"
dependencies = [
    "test-lfs-repo",
]

[tool.uv.sources]
test-lfs-repo = { git = "https://github.com/astral-sh/test-lfs-repo", rev = "657500f0703dc173ac5d68dfa1d7e8c985c84424", lfs = true }
```

## Error

<!-- Derived from [`edit::add_git_lfs_error`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/edit.rs) -->

Using `--lfs` flag with a non-Git source is an error.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.13"
dependencies = []
```

```console
$ uv add typing-extensions --lfs
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `typing-extensions` did not resolve to a Git repository, but a Git extension (`--lfs`) was provided.
```
