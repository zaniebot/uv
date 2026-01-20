# Migration Status & Priorities

> **⚠️ IMPORTANT FOR AGENTS:** We're at 39.8% completion (997/2,504 tests). Goal is 90%+ (2,200+
> tests). **Keep working!** Focus on the large files: pip_compile, lock, pip_install, sync,
> pip_sync, edit, run.

## Current Progress

**Overall: 997 of 2,504 tests (39.8%) migrated to mdtest format**

**Mdtest files created:**

- 105 mdtest files
- 1,168 test sections written
- Comprehensive coverage across most command areas

**Files with most progress:**

- init.rs: 73 of 73 tests (100%)
- tool_install.rs: 51 of 51 tests (100%)
- python_install.rs: 49 of 49 tests (100%)
- venv.rs: 41 of 41 tests (100%)
- build.rs: 32 of 32 tests (100%)
- export.rs: 77 of 78 tests (98.7%)
- tool_run.rs: 48 of 49 tests (98.0%)

**Files needing work:**

- pip_compile.rs: 209 tests remaining (163 done, 372 total) - 43.8% complete
- lock.rs: 265 tests remaining (15 done, 280 total) - 5.4% complete
- pip_install.rs: 232 tests remaining (37 done, 269 total) - 13.8% complete
- sync.rs: 142 tests remaining (13 done, 155 total) - 8.4% complete
- pip_sync.rs: 118 tests remaining (13 done, 131 total) - 9.9% complete
- edit.rs: 90 tests remaining (75 done, 165 total) - 45.5% complete
- run.rs: 74 tests remaining (12 done, 86 total) - 14.0% complete

## What CAN Be Migrated (Most Tests!)

The vast majority of tests (>95%) can and should be migrated to mdtest. These include:

### Large Files with Many Migratable Tests

1. **pip_compile.rs** (372 tests)
   - Basic compilation scenarios
   - Output formatting tests
   - Constraint/override tests
   - Index/registry tests
   - See [pip-compile-breakdown.md](pip-compile-breakdown.md) for reorganization plan

2. **pip_install.rs** (269 tests)
   - Package installation scenarios
   - Upgrade/downgrade tests
   - Constraint/requirement tests
   - Index/source tests
   - See [pip-install-breakdown.md](pip-install-breakdown.md) for reorganization plan

3. **lock.rs** (276 tests remaining, 4 done)
   - Lockfile generation
   - Resolution scenarios
   - Platform markers
   - Git dependencies
   - Most are straightforward mdtest conversions

4. **edit.rs** (165 tests)
   - `uv add` tests (adding dependencies)
   - `uv remove` tests (removing dependencies)
   - Workspace/group/optional dependency tests
   - All are good candidates for mdtest

5. **sync.rs** (155 tests remaining, 5 done)
   - Project syncing scenarios
   - Workspace syncing
   - Group/extra handling
   - Environment management
   - Most are migratable

6. **run.rs** (82 tests remaining, 4 done)
   - Script execution
   - Environment handling
   - PEP 723 inline metadata
   - Module execution
   - Most are migratable

7. **pip_sync.rs** (131 tests)
   - Environment syncing
   - Package installation/removal
   - Constraint handling

8. **export.rs** (78 tests)
   - Requirements.txt export
   - CycloneDX export
   - Format conversions

9. **init.rs** (73 tests)
   - Project initialization
   - Template creation
   - Configuration setup

10. **version.rs** (60 tests remaining, 7 done)
    - Version reading/setting
    - Version bumping
    - Workspace version management

## What CANNOT Be Migrated (Small Minority)

Only a small subset of tests truly can't be migrated. See [cannot-migrate.md](cannot-migrate.md) for
details:

1. **Network mocking tests** - Require wiremock/mockserver
   - `publish.rs` - publishing tests with auth
   - `network.rs` - network error scenarios

2. **Async/low-level tests**
   - `extract.rs` - async zip extraction (requires tokio runtime)

3. **System modification tests**
   - `self_update.rs` - modifies CARGO_HOME (CI-only)

4. **Complex fixture tests**
   - Some `ecosystem.rs` tests (use large external repositories)
   - Some `workflow.rs` tests (complex multi-step scenarios with fixtures)

5. **Scenario tests using packse fixtures**
   - `lock_scenarios.rs`
   - `pip_install_scenarios.rs`
   - `pip_compile_scenarios.rs`
   - These use generated fixture packages from the packse ecosystem

6. **Debug/introspection tests**
   - `show_settings.rs` - dumps internal settings structure

**Estimate:** ~150-200 tests out of 2,504 cannot be easily migrated (~6-8%)

## Why Only 3.8% Complete?

We're early in the process! We've focused on:

- Setting up infrastructure
- Creating example patterns
- Testing edge cases
- Documenting the migration approach

The completed tests are spread across:

- pip_list.rs (19), pip_freeze.rs (15), pip_uninstall.rs (13), pip_show.rs (13)
- tree.rs (8), version.rs (7), sync.rs (5), run.rs (4), lock.rs (4), format.rs (4)
- pip_check.rs (4)

## Migration Priorities

### High Priority (Most Impact)

1. **pip_compile.rs** (372 tests)
   - Most commonly used command
   - Clear reorganization plan exists
   - Break into ~20 focused files

2. **pip_install.rs** (269 tests)
   - Core functionality
   - Clear reorganization plan exists
   - Break into ~19 focused files

3. **edit.rs** (165 tests)
   - `uv add` and `uv remove` are key workflows
   - Tests are straightforward to migrate
   - Can organize by feature (registry, git, workspace, etc.)

### Medium Priority (Finish What We Started)

4. **lock.rs** (276 remaining)
   - We have 4 basic tests done
   - Many more straightforward scenarios remain

5. **sync.rs** (155 remaining)
   - We have 5 basic tests done
   - Workspace, group, and extra handling remain

6. **run.rs** (82 remaining)
   - We have 4 basic tests done
   - PEP 723, workspace, and isolation tests remain

### Lower Priority (But Still Migratable)

7. **pip_sync.rs** (131 tests)
8. **export.rs** (78 tests)
9. **init.rs** (73 tests)
10. **version.rs** (60 remaining)
11. **Other files** with <50 remaining tests each

## Recommended Approach

### For Large Files (pip_compile, pip_install)

Use the reorganization plans:

1. Read the breakdown document (pip-compile-breakdown.md or pip-install-breakdown.md)
2. Create the new directory structure (e.g., `test/uv/pip/compile/`)
3. Migrate tests into focused files (constraints.md, indexes.md, output.md, etc.)
4. Each file should focus on one feature/category

### For Medium Files (edit, lock, sync, run)

Continue the pattern we've established:

1. Create feature-focused files (e.g., `edit/add-registry.md`, `lock/upgrade.md`)
2. Group related tests together
3. Add clear section headers and test descriptions

### For Small Files (<50 tests)

Can often migrate in a few focused files:

1. Create a `basics.md` for common cases
2. Create additional files for special scenarios if needed

## Progress Tracking

Update [MIGRATION.md](MIGRATION.md) by:

1. Mark tests as `[x]` when migrated
2. Add a comment with the mdtest file path when useful
3. Run periodic status checks:
   ```bash
   grep -c "^\- \[ \]" plan/MIGRATION.md  # Remaining
   grep -c "^\- \[x\]" plan/MIGRATION.md  # Complete
   ```

## Next Steps

The migration has plenty of runway! We should continue with:

1. **Immediate:** Continue migrating straightforward tests from the large files
2. **Short-term:** Focus on high-impact files (pip_compile, pip_install, edit)
3. **Medium-term:** Complete the medium files we've started (lock, sync, run)
4. **Long-term:** Fill in the remaining straightforward tests

**Goal:** Migrate ~90%+ of tests (2,200+ tests) to mdtest format, leaving only the truly
complex/fixture-dependent tests in Rust.
