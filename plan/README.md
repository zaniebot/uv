# Test Migration Plan

This directory contains planning documents for migrating uv's integration tests from Rust to mdtest
(markdown-based testing).

## 🚀 Quick Start for Agents

**New to this migration?** Read these in order:

1. **[PRIORITIES.md](PRIORITIES.md)** ⭐ **START HERE!**
   - Current: 997/2,504 (39.8%) | Goal: 90%+ (2,200+ tests)
   - Top 7 files to work on with exact instructions
   - What to skip (only ~150-200 tests)

2. **[migration-status.md](migration-status.md)** - Detailed status
   - Complete progress breakdown by file
   - What CAN vs CANNOT be migrated
   - Strategic priorities

3. **[style.md](style.md)** - How to write mdtest files
   - Formatting guidelines
   - Naming conventions
   - Test organization patterns

4. **[MIGRATION.md](MIGRATION.md)** - Complete test checklist
   - All 2,504 tests with checkboxes
   - 997 marked as `[x]` (migrated)
   - Auto-updated by `update-migration-status.py`

## 📁 Directory Structure

### Essential Documents (Read These)

- **migration-status.md** - Migration priorities and status (READ THIS FIRST!)
- **style.md** - How to write mdtest files
- **MIGRATION.md** - Complete test checklist
- **cannot-migrate.md** - Tests that should stay in Rust

### Reorganization Plans (For Large Files)

These files have detailed plans for breaking them into smaller, focused test files:

- **pip-compile-breakdown.md** - Break 372 tests into ~20 files
- **pip-install-breakdown.md** - Break 269 tests into ~19 files
- **reorganization-strategy.md** - Overall approach for large files

### Feature Area Documents

These organize tests by feature area (useful for understanding test scope):

- **pip.md** - pip command tests
- **resolution.md** - Dependency resolution tests
- **workspaces.md** - Workspace tests
- **tools.md** - Tool management tests
- **python-management.md** - Python version tests
- **project-lifecycle.md** - Project lifecycle tests (init, add, remove, etc.)
- **building.md** - Package building tests
- **build-backend.md** - Build backend tests
- **export.md** - Export format tests
- **virtual-environments.md** - venv tests
- **git-dependencies.md** - Git dependency tests
- **authentication.md** - Auth tests
- **caching.md** - Cache tests

### Reference

- **rm-unblocked-tests.md** - Tests that were blocking but are now fixed

## 🎯 What to Work On

### Highest Priority (Most Remaining Tests)

1. **lock.rs** - 265 tests remaining (5.4% done) ⚠️ HUGE OPPORTUNITY
2. **pip_install.rs** - 232 tests remaining (13.8% done)
3. **pip_compile.rs** - 209 tests remaining (43.8% done)
4. **sync.rs** - 142 tests remaining (8.4% done)
5. **pip_sync.rs** - 118 tests remaining (9.9% done)
6. **edit.rs** - 90 tests remaining (45.5% done)
7. **run.rs** - 74 tests remaining (14.0% done)

### Quick Wins (Nearly Complete)

- **export.rs** - 1 test remaining (98.7% done)
- **tool_run.rs** - 1 test remaining (98.0% done)

See **[PRIORITIES.md](PRIORITIES.md)** for detailed instructions on each file!

## 📝 Migration Workflow

1. **Pick a file** from the priorities in migration-status.md
2. **Read the Rust test** in `crates/uv/tests/it/<file>.rs`
3. **Create mdtest file** in `test/uv/<area>/<feature>.md`
4. **Write the test** following style.md guidelines
5. **Mark as done** in MIGRATION.md by changing `[ ]` to `[x]`
6. **Commit** with message like: "Migrate <test-name> to mdtest"

## 💡 Tips for Agents

- **Don't be conservative!** Most tests CAN be migrated
- **The 3.8% completion rate is misleading** - we're early, keep going!
- **Use the reorganization plans** for pip_compile and pip_install
- **Group related tests** into focused files (e.g., all constraint tests together)
- **Reference the Rust test** with `<!-- from file.rs::test_name -->` comments
- **Test early** - run `cargo nextest run mdtest` to verify your tests

## ⚠️ Critical: Maintain Test Parity

**Do NOT simplify or reduce test output!** Migrated tests must be functionally equivalent to the
original Rust tests:

- **Full output matching** - If the Rust test checks a full JSON response, check the full JSON
  response in mdtest. Use placeholder filters like `[TEMP_DIR]`, `[VENV]`, `[BIN]`, `[PYTHON]`,
  `[TIME]` for dynamic values.
- **No shell workarounds** - Don't use pipes, `2>/dev/null`, or `python -c` to extract subsets. The
  mdtest framework handles dynamic values through filters.
- **All test phases** - If a test has lock, sync, then check phases, include all of them.

See **[style.md](style.md)** for detailed parity guidelines.

## ❌ What NOT to Migrate

Only skip tests that:

- Require network mocking (wiremock/mockserver)
- Are async/low-level (extract.rs)
- Modify system state (self_update.rs)
- Use complex packse fixtures (lock_scenarios.rs, pip_install_scenarios.rs)
- Dump internal structures (show_settings.rs)

See **cannot-migrate.md** for the full list (~150-200 tests, or 6-8% of total).

## 📊 Current Status

**Current status: 997 of 2,504 tests (39.8%) migrated**

**Mdtest files created:**

- 105 mdtest files
- 1,168 test sections written

**Files at 100%:** init, tool_install, python_install, venv, build (5 files complete!)

**Files needing work:**

- pip_compile.rs: 209 remaining (163 done) - 43.8% complete
- lock.rs: 265 remaining (15 done) - 5.4% complete
- pip_install.rs: 232 remaining (37 done) - 13.8% complete
- sync.rs: 142 remaining (13 done) - 8.4% complete
- pip_sync.rs: 118 remaining (13 done) - 9.9% complete
- edit.rs: 90 remaining (75 done) - 45.5% complete
- run.rs: 74 remaining (12 done) - 14.0% complete

**Goal:** Migrate 90%+ (2,200+ tests), leaving only ~150-200 complex tests in Rust

## 🔄 Updating Migration Status

After creating new mdtest files, run this script to sync MIGRATION.md:

```bash
python3 plan/update-migration-status.py
```

This script:

- Scans all mdtest files for test provenance comments
- Extracts test names from `<!-- from file.rs::test_name -->` comments
- Extracts test names from `<!-- Derived from [`file::test_name`] -->` comments
- Marks corresponding tests as `[x]` in MIGRATION.md
- Reports progress statistics

**Always run this after migrating tests to keep tracking accurate!**

## 🔍 Finding Tests

### By file

```bash
grep "pip_compile.rs::" plan/MIGRATION.md | head -20
```

### Count remaining in a file

```bash
grep "pip_compile.rs::" plan/MIGRATION.md | grep "^\- \[ \]" | wc -l
```

### Overall progress

```bash
grep -c "^\- \[x\]" plan/MIGRATION.md  # Done
grep -c "^\- \[ \]" plan/MIGRATION.md  # Remaining
```

## 📚 More Information

- Main project: `/Users/zb/workspace/uv`
- Rust tests: `crates/uv/tests/it/`
- Mdtest files: `test/uv/`
- Migration guide: `test/uv/readme.md` (mdtest features)
