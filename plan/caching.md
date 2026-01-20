# Caching

Cache structure, pruning stale entries, cleaning specific packages.

## Initialization

- [x] commands/uv-cache/cache.md#init-failure (from cache.rs::cache_init_failure)

## Clean

- [x] commands/uv-cache/clean.md#clean-all (from cache_clean.rs::clean_all)
- [x] commands/uv-cache/clean.md#clean-package-pypi (from cache_clean.rs::clean_package_pypi)
- [x] commands/uv-cache/clean.md#clean-package-index (from cache_clean.rs::clean_package_index)

## Prune

- [x] commands/uv-cache/prune.md#no-op (from cache_prune.rs::prune_no_op)
- [x] commands/uv-cache/prune.md#stale-directory (from cache_prune.rs::prune_stale_directory)
- [x] commands/uv-cache/prune.md#cached-env (from cache_prune.rs::prune_cached_env)
- [x] commands/uv-cache/prune.md#stale-symlink (from cache_prune.rs::prune_stale_symlink)
- [x] commands/uv-cache/prune.md#unzipped (from cache_prune.rs::prune_unzipped)
- [x] commands/uv-cache/prune.md#stale-revision (from cache_prune.rs::prune_stale_revision)

## Size

- [x] commands/uv-cache/size.md#empty-raw (from cache_size.rs::cache_size_empty_raw)
- [x] commands/uv-cache/size.md#with-packages-raw (from cache_size.rs::cache_size_with_packages_raw)
- [x] commands/uv-cache/size.md#with-packages-human (from
      cache_size.rs::cache_size_with_packages_human)
