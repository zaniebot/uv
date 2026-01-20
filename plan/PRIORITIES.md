# Migration Priorities - What to Work On Next

**Last updated:** Run `python3 plan/update-migration-status.py` to refresh

**Current:** 997 of 2,504 tests (39.8%) | **Goal:** 90%+ (2,200+ tests)

## 🎯 Top Priority Files (Work on These!)

### 1. lock.rs - 265 tests remaining (5.4% complete)

**Why:** Core functionality, many straightforward tests remain

**Status:** Only 15 of 280 done - huge opportunity!

**Where to put tests:**

- `test/uv/lock/` - already has basics.md
- Create more files: upgrade.md, frozen.md, platforms.md, groups.md, git.md, etc.

**How to find tests:**

```bash
grep "lock.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -20
```

### 2. pip_install.rs - 232 tests remaining (13.8% complete)

**Why:** Core pip functionality

**Status:** Only 37 of 269 done

**Where to put tests:**

- `test/uv/pip/install/` - already has basics.md
- See [pip-install-breakdown.md](pip-install-breakdown.md) for organization plan
- Create: reinstall.md, upgrade.md, constraints.md, urls.md, git.md, editable.md, etc.

**How to find tests:**

```bash
grep "pip_install.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -20
```

### 3. pip_compile.rs - 209 tests remaining (43.8% complete)

**Why:** Core pip functionality, already half done

**Status:** 163 of 372 done - good progress, finish it!

**Where to put tests:**

- `test/uv/pip/compile/` - already has output.md, pyproject.md
- See [pip-compile-breakdown.md](pip-compile-breakdown.md) for organization plan
- Create more files for: constraints.md, hashes.md, annotations.md, platforms.md, etc.

**How to find tests:**

```bash
grep "pip_compile.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -20
```

### 4. sync.rs - 142 tests remaining (8.4% complete)

**Why:** Core project sync functionality

**Status:** Only 13 of 155 done

**Where to put tests:**

- `test/uv/sync/` - already has basics.md
- Create: groups.md, extras.md, workspace.md, no-install.md, build-isolation.md, etc.

**How to find tests:**

```bash
grep "sync.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -20
```

### 5. pip_sync.rs - 118 tests remaining (9.9% complete)

**Why:** pip sync functionality

**Status:** Only 13 of 131 done

**Where to put tests:**

- `test/uv/pip/sync.md` - exists but incomplete
- Could split into: sync/basics.md, sync/editable.md, sync/reinstall.md, etc.

**How to find tests:**

```bash
grep "pip_sync.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -20
```

### 6. edit.rs - 90 tests remaining (45.5% complete)

**Why:** `uv add` and `uv remove` are key workflows

**Status:** 75 of 165 done - almost half way!

**Where to put tests:**

- `test/uv/edit/` - already has add.md, remove.md, bounds.md, groups.md, virtual.md
- These files may need expansion or new files for: git.md, workspace.md, markers.md, etc.

**How to find tests:**

```bash
grep "edit.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -20
```

### 7. run.rs - 74 tests remaining (14.0% complete)

**Why:** `uv run` is a primary command

**Status:** Only 12 of 86 done

**Where to put tests:**

- `test/uv/run/` - has basics.md, scripts.md, with.md, groups.md, requirements.md, module.md
- Create more: workspace.md, isolated.md, editable.md, python-version.md, etc.

**How to find tests:**

```bash
grep "run.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -20
```

## ✅ Completed Files (5 files at 100%)

These are done - don't work on these unless fixing issues:

- **init.rs** - 73/73 (100%)
- **tool_install.rs** - 51/51 (100%)
- **python_install.rs** - 49/49 (100%)
- **venv.rs** - 41/41 (100%)
- **build.rs** - 32/32 (100%)

## 🎉 Nearly Complete Files (>95%)

These are almost done - finish them off:

- **export.rs** - 77/78 (98.7%) - just 1 test left!
- **tool_run.rs** - 48/49 (98.0%) - just 1 test left!

## 📋 Workflow for Agents

1. **Pick a file** from priorities above (start with #1-#7)
2. **Find remaining tests:**
   ```bash
   grep "FILENAME.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | head -10
   ```
3. **Read the Rust test** in `crates/uv/tests/it/FILENAME.rs`
4. **Find the test function** - search for `fn test_name`
5. **Create/update mdtest file** in appropriate `test/uv/` location
6. **Add provenance comment:** `<!-- from FILENAME.rs::test_name -->`
7. **Run update script:**
   ```bash
   python3 plan/update-migration-status.py
   ```
8. **Verify** the test is now marked `[x]` in MIGRATION.md

## 💡 Tips

- **Group related tests** into focused files (all constraint tests together, all index tests
  together, etc.)
- **Use the breakdown docs** for guidance on file organization (pip-compile-breakdown.md,
  pip-install-breakdown.md)
- **Don't recreate files** - add to existing mdtest files when appropriate
- **Check existing files first** - many test areas already have mdtest files started

## ❌ Skip These (Cannot Migrate)

See [cannot-migrate.md](cannot-migrate.md) for details (~150-200 tests):

- Network mocking (publish.rs, network.rs)
- Async tests (extract.rs)
- System modification (self_update.rs)
- Packse scenarios (lock_scenarios.rs, pip_install_scenarios.rs, pip_compile_scenarios.rs)
- Debug output (show_settings.rs)
