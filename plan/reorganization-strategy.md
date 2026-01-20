# Test Reorganization Strategy

## 🎯 Core Principle

**Organize by FEATURE, not by COMMAND**

Many tests in `pip_install.rs`, `pip_compile.rs`, `lock.rs`, and `pip_sync.rs` are testing universal
dependency resolution features, not command-specific behavior. We should extract these into shared
feature categories.

---

## 📂 Proposed Directory Structure

```
test/uv/
├── resolution/                    # Universal resolver features
│   ├── constraints.md             # ~45 tests from compile + install
│   ├── extras.md                  # ~50 tests from compile + install
│   ├── git.md                     # ~35 tests from compile + install
│   ├── hashes.md                  # ~33 tests from compile + install
│   ├── indexes.md                 # ~33 tests from compile + install
│   ├── markers.md                 # ~50 tests from compile + lock
│   ├── overrides.md               # ~48 tests from compile + lock
│   ├── python-platform.md         # ~57 tests from compile + install
│   ├── strategy.md                # ~30 tests from compile
│   ├── urls.md                    # ~18 tests from compile + install
│   ├── wheels.md                  # ~25 tests from compile
│   ├── yanked.md                  # ~20 tests from compile
│   └── forking.md                 # ~45 tests from compile + lock
│
├── pip/
│   ├── check.md                   # ✅ 4 tests (complete)
│   ├── compile/
│   │   ├── basic.md               # ~20 compile-specific tests
│   │   ├── output.md              # ~15 annotation/header tests
│   │   └── update.md              # ~15 -U flag tests
│   ├── debug.md                   # 1 test
│   ├── freeze.md                  # ✅ 15 tests (complete)
│   ├── install/
│   │   ├── basic.md               # ~25 install-specific tests
│   │   ├── archive-validation.md # ~11 zip security tests
│   │   ├── caching.md             # ~7 cache tests
│   │   ├── dry-run.md             # ~7 dry run tests
│   │   ├── executables.md         # ~10 launcher/symlink tests
│   │   └── formats.md             # ~15 file format tests
│   ├── list.md                    # ✅ 19 tests (complete)
│   ├── show.md                    # ✅ 13 tests (complete)
│   ├── sync/
│   │   ├── basic.md               # ~40 sync-specific tests
│   │   ├── workspace.md           # ~30 workspace sync tests
│   │   └── editable.md            # ~20 editable sync tests
│   ├── tree.md                    # 22 tests
│   └── uninstall.md               # ✅ 13 tests (complete)
│
├── lock/
│   ├── basic.md                   # ~50 lock-specific tests
│   ├── workspace.md               # ~40 workspace locking
│   ├── update.md                  # ~30 lock update scenarios
│   └── conflicts.md               # ~30 conflict resolution
│
└── run/
    ├── basic.md                   # ~30 basic run tests
    ├── scripts.md                 # ~25 script handling
    └── with.md                    # ~20 --with flag tests
```

---

## 🔄 Cross-Cutting Features

### Features Used by Multiple Commands

| Feature              | Used By                              | Test Count |
| -------------------- | ------------------------------------ | ---------- |
| **Constraints**      | compile, install, sync, lock         | ~45        |
| **Extras**           | compile, install, sync, lock         | ~50        |
| **Git Dependencies** | compile, install, sync, lock, freeze | ~35        |
| **Hashes**           | compile, install, sync               | ~33        |
| **Indexes**          | compile, install, sync               | ~33        |
| **Markers**          | compile, install, sync, lock         | ~50        |
| **Overrides**        | compile, lock                        | ~48        |
| **Python/Platform**  | compile, install, lock               | ~57        |
| **URLs**             | compile, install, sync               | ~18        |
| **Yanked**           | compile, install                     | ~20        |
| **Forking**          | compile, lock                        | ~45        |
| **Wheels**           | compile, install                     | ~25        |

### Command-Specific Features

| Command         | Specific Features                                  | Test Count |
| --------------- | -------------------------------------------------- | ---------- |
| **pip compile** | Annotations, Headers, Output format                | ~30        |
| **pip install** | Archive validation, Executables, Editable installs | ~54        |
| **pip sync**    | Exact sync, Removal logic                          | ~40        |
| **lock**        | Lock file format, Update modes                     | ~50        |
| **run**         | Script execution, --with flag                      | ~50        |

---

## 📊 Test Count Breakdown

### Before Reorganization

- `pip_compile.rs`: 372 tests (1 huge file)
- `pip_install.rs`: 271 tests (1 huge file)
- `lock.rs`: 280 tests (1 huge file)
- `pip_sync.rs`: 160 tests (1 large file)
- `run.rs`: 86 tests (1 file)

**Total**: 1,169 tests in 5 files (avg 234 tests/file)

### After Reorganization

**Universal Features** (~430 tests):

- `resolution/constraints.md`: ~45 tests
- `resolution/extras.md`: ~50 tests
- `resolution/git.md`: ~35 tests
- `resolution/hashes.md`: ~33 tests
- `resolution/indexes.md`: ~33 tests
- `resolution/markers.md`: ~50 tests
- `resolution/overrides.md`: ~48 tests
- `resolution/python-platform.md`: ~57 tests
- `resolution/strategy.md`: ~30 tests
- `resolution/urls.md`: ~18 tests
- `resolution/wheels.md`: ~25 tests
- `resolution/yanked.md`: ~20 tests
- `resolution/forking.md`: ~45 tests

**Command-Specific** (~739 tests):

- `pip/compile/*.md`: ~50 tests (3 files)
- `pip/install/*.md`: ~189 tests (6-8 files)
- `pip/sync/*.md`: ~90 tests (3 files)
- `lock/*.md`: ~150 tests (4 files)
- `run/*.md`: ~75 tests (3 files)

**Total**: ~1,169 tests in ~40 files (avg ~29 tests/file)

---

## ✨ Benefits

### 1. Reusability

Tests for "git dependencies" don't need to be duplicated across compile, install, and sync. Write
once, validate everywhere.

### 2. Discoverability

Want to know how constraints work? Look in `resolution/constraints.md`, not scattered across 4
different files.

### 3. Coverage Clarity

Easy to see:

- "We have 45 constraint tests" ✓
- "Git dependencies are well-tested" ✓
- "Only 18 URL tests, might need more" ⚠️

### 4. Maintenance

Change to marker evaluation? Update `resolution/markers.md`, and all commands benefit.

### 5. Logical Grouping

Files stay manageable (20-50 tests each) vs. monolithic (200-400 tests).

---

## 🚀 Migration Strategy

### Phase 1: Extract Universal Features (priority)

Start with features that are clearly reusable:

1. **Git Dependencies** (`resolution/git.md`)
   - Pull from: pip_compile, pip_install
   - ~35 tests
   - Clear, self-contained

2. **Hashes** (`resolution/hashes.md`)
   - Pull from: pip_compile, pip_install
   - ~33 tests
   - Well-defined behavior

3. **Extras** (`resolution/extras.md`)
   - Pull from: pip_compile, pip_install, lock
   - ~50 tests
   - Important feature

### Phase 2: Command-Specific Tests

After extracting universal features:

4. **pip compile basic** (`pip/compile/basic.md`)
   - ~20 tests
   - Quick wins

5. **pip tree** (`pip/tree.md`)
   - 22 tests
   - Standalone command

6. **pip debug** (`pip/debug.md`)
   - 1 test
   - Trivial

### Phase 3: Medium Categories

7. **run** (~75-86 tests split into 3 files)
8. **pip sync basic** (~40 tests)

### Phase 4: Large Remaining

9. **pip install** remaining command-specific tests
10. **lock** file-specific tests
11. **resolution** remaining universal features

---

## 📋 Implementation Notes

### Test Attribution

Keep provenance comments but reference the logical feature:

```markdown
<!-- from pip_compile.rs::compile_git_branch_https_dependency -->
<!-- Also covers: pip install, pip sync, lock -->
```

### Shared Test Patterns

Universal feature tests should demonstrate behavior that applies to all commands:

````markdown
## Git branch dependency

Test git branch references in dependency resolution.

### Via pip compile

```console
$ echo "package @ git+https://github.com/user/repo@branch" > requirements.in
$ uv pip compile requirements.in
...
```
````

### Via pip install

```console
$ uv pip install "package @ git+https://github.com/user/repo@branch"
...
```

````

### Cross-References
Link related tests:

```markdown
For git authentication, see: [Git Authentication](../git/auth.md)
For git with constraints, see: [Constraints](constraints.md#git-dependencies)
````

---

## 🎓 Example: Constraints Category

### Before

- `pip_compile.rs`: 15 constraint tests (lines 245-892)
- `pip_install.rs`: 20 constraint tests (lines 1234-2100)
- `pip_sync.rs`: 10 constraint tests (lines 456-789)

**Total**: 45 tests scattered across 3 files

### After

- `resolution/constraints.md`: 45 tests organized by scenario
  - Basic constraints
  - Conflicting constraints
  - Constraints with markers
  - Constraints with extras
  - Build constraints
  - Override interactions

**Total**: 45 tests in 1 logical file, clearly showing all constraint behavior

---

## 📈 Expected Outcomes

### Before

- 5 massive files (160-372 tests each)
- Hard to find specific feature tests
- Unclear what's tested where
- Duplication across command tests

### After

- ~40 focused files (15-50 tests each)
- Clear feature documentation
- Easy to find "how do git deps work?"
- Single source of truth for each feature
- Manageable file sizes

---

## 🤔 Open Questions

1. **Should authentication tests be in `resolution/` or keep in `pip/install/auth.md`?**
   - Lean toward `resolution/auth.md` since it applies to compile, install, sync

2. **How deep to nest?**
   - `resolution/git.md` vs. `resolution/git/basic.md` + `resolution/git/auth.md`
   - Prefer flat structure until a category exceeds ~60 tests

3. **What about lock-specific features?**
   - Keep `lock/` directory for lock file format specifics
   - Move universal resolution to `resolution/`

4. **Edge cases that test multiple features?**
   - Place in most relevant category
   - Add cross-references to related features

---

## 🎯 Decision

This reorganization should be done DURING migration, not after. As we migrate tests:

1. Read the test
2. Identify if it's universal (resolution) or command-specific
3. Place in the appropriate feature/command category
4. Update plan files with the new organization

This way we build the better structure from the start rather than migrating to a flawed structure
and reorganizing later.
