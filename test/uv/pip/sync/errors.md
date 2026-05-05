# pip sync Errors

Tests for error handling in `uv pip sync`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Missing files

### Missing requirements.txt

<!-- from pip_sync.rs::missing_requirements_txt -->

Syncing from a missing requirements.txt shows an error.

```console
$ uv pip sync requirements.txt --strict
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: File not found: `requirements.txt`
```

### Empty requirements.txt

<!-- from pip_sync.rs::pip_sync_empty -->

Syncing an empty requirements.txt shows a warning.

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

## Link mode errors

### Symlink with no-cache

<!-- from pip_sync.rs::install_symlink_no_cache -->

Using symlink link mode with --no-cache produces an error.

```toml
# file: requirements.txt
MarkupSafe==2.1.3
```

```console
$ uv pip sync requirements.txt --link-mode symlink --no-cache --strict
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
error: Symlink-based installation is not supported with `--no-cache`. The created environment will be rendered unusable by the removal of the cache.
```
