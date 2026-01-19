# Cache Size

Tests for `uv cache size`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = ["pypi"]

[filters]
counts = true
cache-size = true
```

## Empty cache (raw)

<!-- Derived from [`cache_size::cache_size_empty_raw`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_size.rs#L5-L21) -->

`uv cache size` should return 0 for an empty cache directory.

First, clean the cache to ensure it's empty:

```console
$ uv cache clean
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Clearing cache at: [CACHE_DIR]/
Removed [N] files ([SIZE])
```

Now check the size (the filter replaces `0` with `[SIZE]}):

```console
$ uv cache size --preview
success: true
exit_code: 0
----- stdout -----
[SIZE]

----- stderr -----
```

## Cache size with packages (raw)

<!-- Derived from [`cache_size::cache_size_with_packages_raw`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_size.rs#L23-L40) -->

`uv cache size` should return raw bytes after installing packages.

Install a package to populate the cache:

```console
$ uv pip install iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
```

Check the cache size (should be positive):

```console
$ uv cache size --preview
success: true
exit_code: 0
----- stdout -----
[SIZE]

----- stderr -----
```

## Cache size with packages (human-readable)

<!-- Derived from [`cache_size::cache_size_with_packages_human`](https://github.com/astral-sh/uv/blob/08caf342685dcf72c8fd716efa6bff7db8acbee2/crates/uv/tests/it/cache_size.rs#L42-L59) -->

`uv cache size --human` should return human-readable format after installing packages.

Install a package to populate the cache:

```console
$ uv pip install iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved [N] packages in [TIME]
Prepared [N] packages in [TIME]
Installed [N] packages in [TIME]
 + iniconfig==2.0.0
```

Check the cache size with `--human` flag:

```console
$ uv cache size --preview --human
success: true
exit_code: 0
----- stdout -----
[SIZE]

----- stderr -----
```
