# Tests Unblocked by `rm` Support

With the addition of `$ rm` command support in mdtest, the following tests can now be migrated from
Rust integration tests to markdown tests.

## Previously Blocked Tests (5 total)

### Tools - Uninstall (2 tests)

**Location:** `plan/tools.md` lines 299-302

1. **`tool_uninstall_missing_receipt`** (from `tool_uninstall.rs`)
   - **What it does:** Installs a tool, deletes its `uv-receipt.toml`, then uninstalls
   - **File deletion needed:** `rm tools/black/uv-receipt.toml`
   - **Example:** `test/uv/tools/uninstall-missing-receipt-example.md`

2. **`tool_uninstall_all_missing_receipt`** (from `tool_uninstall.rs`)
   - **What it does:** Installs multiple tools, deletes one receipt, then uninstalls all
   - **File deletion needed:** `rm tools/TOOL_NAME/uv-receipt.toml`

### Tools - List (2 tests)

**Location:** `plan/tools.md` lines 180-183

3. **`tool_list_missing_receipt`** (from `tool_list.rs`)
   - **What it does:** Installs tools, deletes a receipt, then lists tools
   - **File deletion needed:** `rm tools/TOOL_NAME/uv-receipt.toml`

4. **`tool_list_bad_environment`** (from `tool_list.rs`)
   - **What it does:** Installs tools, removes a tool's bin directory, then lists
   - **File deletion needed:** `rm -rf tools/TOOL_NAME/[BIN]`
   - **Example:** `test/uv/tools/list-bad-environment-example.md`

### Tools - Upgrade (1 test)

**Location:** `plan/tools.md` line 340

5. **`tool_upgrade_not_stop_if_upgrade_fails`** (from `tool_upgrade.rs`)
   - **Note:** This test actually _writes_ invalid content to corrupt the receipt, not delete it
   - **Can use:** `Write` operation to create invalid receipt (already supported)
   - **May not actually need `rm`** - should be re-evaluated

## Migration Strategy

For each test:

1. Create markdown file in appropriate location under `test/uv/tools/`
2. Use `$ rm <file>` for single file deletion
3. Use `$ rm -rf <directory>` for recursive directory deletion
4. Set up environment with `UV_TOOL_DIR` and `XDG_BIN_HOME` in mdtest config
5. Apply appropriate filters (exe-suffix, python-names, virtualenv-bin, etc.)

## Common Patterns

### Delete a receipt file

```markdown
$ rm tools/black/uv-receipt.toml success: true exit_code: 0 ----- stdout -----

----- stderr -----
```

### Delete a directory recursively

```markdown
$ rm -rf tools/black/[BIN] success: true exit_code: 0 ----- stdout -----

----- stderr -----
```

### Delete multiple files

```markdown
$ rm tools/black/uv-receipt.toml tools/ruff/uv-receipt.toml success: true exit_code: 0 ----- stdout
-----

----- stderr -----
```

## Impact

- **Previously blocked:** 5 tests (listed as blockers in `plan/tools.md`)
- **Now unblocked:** 4-5 tests (depending on `tool_upgrade_not_stop_if_upgrade_fails`)
- **Tools migration progress:** Will increase from 64/136 (47.1%) to 68-69/136 (50.0-50.7%)

## Next Steps

1. Migrate `tool_uninstall_missing_receipt` and `tool_uninstall_all_missing_receipt`
2. Migrate `tool_list_missing_receipt` and `tool_list_bad_environment`
3. Re-evaluate `tool_upgrade_not_stop_if_upgrade_fails` (may not need `rm`)
4. Update `plan/tools.md` to remove "File deletion" from blockers section
5. Delete corresponding Rust test files once migration is complete
