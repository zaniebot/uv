YOU ARE MIGRATING THE TEST SUITE TO THE NEW MDTEST FRAMEWORK. REVIEW THE PLANS FOR THE MIGRATION. DO
NOT STOP UNTIL YOU HAVE MIGRATED ALL TESTS THAT DO NOT REQUIRE NEW MDTEST FEATURES. DOCUMENT WHICH
MDTEST FEATURES ARE MISSING. SOME TESTS MAY NOT BE MIGRATABLE, BUT MOST ARE. KEEP GOING UNTIL YOU
ARE EXPLICITLY TOLD TO STOP. IF THE CONTEXT IS CLOSE TO FULL, THAT IS OKAY YOU CAN COMPACT AND KEEP
GOING.

## MDTEST STATE PERSISTENCE - IMPORTANT

See `test/uv/readme.md` for full documentation of mdtest features.

**Within a single `###` section:**

- Multiple commands run IN SEQUENCE and SHARE STATE
- File modifications persist between commands
- You can: create file → run command → modify file → run another command

Example pattern for stateful tests:

````markdown
### Install then reinstall

```toml
# file: requirements.txt
Flask==2.3.2
```
````

```console
$ uv pip install -r requirements.txt
success: true
...
```

Now modify the requirements and run again:

```toml
# file: requirements.txt
Flask
```

```console
$ uv pip install -r requirements.txt
success: true
...
```

````

**Between `###` sections:**
- Each section is INDEPENDENT - files don't carry over
- Each section starts with a fresh temp directory

**Most tests CAN be migrated** using this pattern, including tests that:
- Modify pyproject.toml between commands
- Install then reinstall packages
- Lock then sync
- Create lockfile then verify with --locked

## CURRENT MIGRATION STATUS

- **Passed: 1038 tests**
- **Failed: 461 tests**
- **Ignored: 16 tests**

Many remaining failures are due to:
1. Tests using packse test packages (e.g., `executable-application`) not available on public PyPI
2. Tests using shell pipes (`| head -1`) that need debugging
3. Tests needing `[SITE_PACKAGES]` file creation

## MISSING MDTEST FEATURES (Tests requiring these are not migratable)

1. **Stdin redirection** (`< file.txt`) - Used by authentication tests like `uv auth login --password - < password.txt`
2. **Writing to [SITE_PACKAGES]** - Tests that need to create files in the venv's site-packages directory (pip/freeze with egg-info tests)
3. **title= syntax** - The `title="filename"` code fence attribute is NOT supported. Use `# file: filename` inside the code block instead.
4. **working-dir= for non-existent dirs** - Fixed! Now auto-creates directories.
5. **Shell compound commands** (`&&`, `||`, `|`) - Added support for these. Run through `sh -c`.
6. **uv_build package** - Tests using `uv init --lib` can't run `uv run` because `uv_build` package isn't on PyPI

## COMMON MIGRATION ISSUES

1. **Incomplete `uv lock` output** - Always add `stdout/stderr` sections:
   ```console
   $ uv lock
   success: true
   exit_code: 0
   ----- stdout -----

   ----- stderr -----
   Resolved X packages in [TIME]
````

2. **Separate sections expecting shared state** - Merge into single `###` section

3. **Command header format changed** - Export commands show `#    uv --cache-dir [CACHE_DIR] export`
   not `#    uv export --cache-dir [CACHE_DIR]`

4. **Enable filters for JSON output** - Add to document config:
   ```toml
   [filters]
   python-names = true
   virtualenv-bin = true
   ```

- Read CONTRIBUTING.md for guidelines on how to run tools
- ALWAYS attempt to add a test case for changed behavior
- PREFER integration tests, e.g., at `it/...` over unit tests
- When making changes for Windows from Unix, use `cargo xwin clippy` to check compilation
- NEVER perform builds with the release profile, unless asked or reproducing performance issues
- PREFER running specific tests over running the entire test suite
- AVOID using `panic!`, `unreachable!`, `.unwrap()`, unsafe code, and clippy rule ignores
- PREFER patterns like `if let` to handle fallibility
- ALWAYS write `SAFETY` comments following our usual style when writing `unsafe` code
- PREFER `#[expect()]` over `[allow()]` if clippy must be disabled
- PREFER let chains (`if let` combined with `&&`) over nested `if let` statements

```

```
