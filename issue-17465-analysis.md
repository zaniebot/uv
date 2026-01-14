# Issue #17465 Analysis: UV_PYTHON_INSTALL_DIR Not Respected in v0.9.25

## Summary

This document captures the root cause analysis for
[GitHub Issue #17465](https://github.com/astral-sh/uv/issues/17465), which reports that
`UV_PYTHON_INSTALL_DIR` is not respected in v0.9.25 when running in Docker containers.

## Symptoms

- Error: `failed to create directory /.cache/uv: Permission denied (os error 13)`
- Occurs when running `uv python install --install-dir .` in Docker containers
- Worked in v0.9.24, broken in v0.9.25
- The error is about the cache directory (`/.cache/uv`), not the Python install directory

## Root Cause

The issue was introduced by **PR #17088** ("Add `--compile-bytecode` to `uv python install` and
`uv python upgrade`"), which was merged into v0.9.25.

### What Changed

**v0.9.24** (`crates/uv/src/lib.rs`):

```rust
// No cache initialization for python install
commands::python_install(
    &project_dir,
    args.install_dir,
    // ... other args, but NO cache parameter
    globals.preview,
    printer,
)
```

**v0.9.25** (`crates/uv/src/lib.rs:1648-1672`):

```rust
// Initialize the cache - NEW!
let cache = cache.init().await?;

commands::python_install(
    &project_dir,
    args.install_dir,
    // ... other args
    args.compile_bytecode,
    &globals.concurrency,
    &cache,  // NEW - cache is now passed
    globals.preview,
    printer,
)
```

### Why This Breaks Docker Builds

1. When `cache.init()` is called, it creates the cache directory structure
2. The cache directory is computed via `uv_dirs::user_cache_dir()`
   (`crates/uv-dirs/src/lib.rs:43-47`)
3. This uses `etcetera::base_strategy::choose_base_strategy()` which relies on `$XDG_CACHE_HOME` or
   `$HOME/.cache`
4. In Docker containers (especially those built with Kaniko), `HOME` is often unset or set to `/`
5. This results in the cache directory being `/.cache/uv`
6. Creating `/.cache/uv` requires root permissions
7. Error: "Permission denied (os error 13)"

### Key Code Locations

| File                                       | Line      | Description                                               |
| ------------------------------------------ | --------- | --------------------------------------------------------- |
| `crates/uv/src/lib.rs`                     | 1648-1649 | Cache initialization added for `PythonCommand::Install`   |
| `crates/uv/src/lib.rs`                     | 1680-1681 | Cache initialization added for `PythonCommand::Upgrade`   |
| `crates/uv/src/commands/python/install.rs` | 194-254   | New `install()` wrapper that handles bytecode compilation |
| `crates/uv-cache/src/lib.rs`               | 405-452   | `create_base_files()` which calls `create_dir_all(root)`  |
| `crates/uv-cache/src/cli.rs`               | 43-75     | `Cache::from_settings()` which computes cache directory   |
| `crates/uv-dirs/src/lib.rs`                | 43-47     | `user_cache_dir()` which falls back to `$HOME/.cache/uv`  |

## Suggested Fixes

### Option 1: Lazy Cache Initialization

Only initialize the cache when `--compile-bytecode` is actually specified:

```rust
let cache = if args.compile_bytecode {
    Some(cache.init().await?)
} else {
    None
};
```

### Option 2: Graceful Fallback

Handle the case where no suitable cache directory can be found:

- Skip bytecode compilation if cache cannot be initialized
- Use a temporary directory instead

### Option 3: Better Error Message

At minimum, provide a clearer error message explaining that `UV_CACHE_DIR` should be set when `HOME`
is not available.

## Environment Details from Issue

- **Affected Version**: v0.9.25
- **Platform**: Ubuntu 24.04 in Docker
- **Container Builder**: Kaniko
- **Previous Working Version**: v0.9.24

## Commits Between v0.9.24 and v0.9.25

The relevant commit is:

- `4513797f4` - Add `--compile-bytecode` to `uv python install` and `uv python upgrade` (#17088)
