# Style Guide

Guidelines for migrating tests from Rust to mdtest format.

## Tracking

When migrating a test, check the box in the section-specific file (e.g., `workspaces.md`,
`tools.md`). Do not update `MIGRATION.md` - a separate process will sync the overview.

The section files are:

- `workspaces.md` - Workspace discovery, members, virtual vs root workspaces
- `git-dependencies.md` - Git repositories, refs, subdirectories, authentication
- `build-backend.md` - uv's native build backend
- `authentication.md` - Credentials, login/logout, keyring integration
- `caching.md` - Cache structure, pruning, cleaning
- `python-management.md` - Installing, finding, pinning Python versions
- `tools.md` - Tool installation (uvx), running, upgrading
- `resolution.md` - Dependency resolution, branching URLs, markers
- `virtual-environments.md` - Creating and managing venvs
- `project-lifecycle.md` - Init, add/remove dependencies
- `export.md` - requirements.txt, PEP 751, CycloneDX
- `building.md` - Building sdists and wheels
- `cannot-migrate.md` - Generated tests that cannot be migrated

## File Organization

Organize tests by feature area, not by original Rust file:

- `commands/uv-venv.md` - Main venv command tests
- `commands/uv-add.md` - Add command tests
- `python-version-files.md` - `.python-version` file discovery (cross-command)
- `settings/environment-variables.md` - Environment variable parsing (cross-command)
- `projects/custom-environment-path.md` - `UV_PROJECT_ENVIRONMENT` feature

Tests that are specific to a command go in `commands/`. Tests for cross-cutting features go in
feature-specific files.

## Parity with Original Tests

**Critical**: Migrated tests must maintain full parity with the original Rust tests.

- **Do not reduce output to subsets** - If the original test checks a full JSON response, the
  migrated test must also check the full JSON response with appropriate placeholder filters
- **Do not use shell pipes or workarounds** - The mdtest framework supports placeholder filters like
  `[TEMP_DIR]`, `[VENV]`, `[BIN]`, `[PYTHON]`, `[TIME]` that handle dynamic values
- **Do not simplify assertions** - If the original test verifies specific fields or exact output,
  the migrated test must do the same
- **Preserve all test scenarios** - If a test has multiple phases (e.g., lock then sync then check),
  include all phases

The goal is 1:1 functional equivalence. A passing migrated test should verify exactly what the
original Rust test verified.

## Writing Style

Transform code comments into markdown prose. Keep the same tone and level of detail as the original
author. Don't be verbose. Don't sound like an LLM.

### Section Titles

Use descriptive, documentation-style titles instead of test function names:

- Good: "Creating a virtual environment in the current directory"
- Bad: "create_venv_cwd"

## Provenance

At the start of each migrated test section, include a comment with the test name and link:

```markdown
<!-- Derived from [`venv::create_venv`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/venv.rs#L14-L71) -->
```

Use markdown link format with backticks around the test name. Include both the `module::test_name`
and a permalink to the specific lines.

## Code Block Directives

### File Directives

Use `# file: <path>` as the first line inside a code block to specify the file path:

````markdown
```toml
# file: pyproject.toml

[project]
name = "example"
```
````

The directive line and optional blank line after it are stripped from the content before writing the
file.

### Config Blocks

Use `# mdtest` as the first line to mark a configuration block:

````markdown
```toml
# mdtest

[environment]
python-version = "3.12"
```
````

## Configuration

### File-level vs Section-level Config

Use file-level config for shared settings:

```toml
# mdtest

[environment]
python-version = "3.12"
create-venv = false

[tree]
exclude = ["cache"]
```

Override at section level when needed:

```toml
# mdtest

[environment]
python-versions = ["3.11", "3.12"]
```

### Common Patterns

**Platform-specific tests:**

```toml
# mdtest

[environment]
target-family = "unix"  # or "windows"
```

**Feature-gated tests:**

```toml
# mdtest

[environment]
required-features = "python-patch"
```

**Environment variable removal:**

```toml
# mdtest

[environment]
env-remove = ["UV_TEST_PYTHON_PATH"]
```

**Marking Python versions as managed:**

```toml
# mdtest

[environment]
python-versions = ["3.11"]
managed-python-versions = ["3.12"]
```

This marks Python 3.12 as a "managed" installation. Managed versions are automatically included in
the available Python versions (placed first for preference order). This affects output like
`Using CPython 3.12.X` (managed) vs `Using CPython 3.11.X interpreter at: /path/to/python` (system).

**Custom filters:**

```toml
# mdtest

[filters]
python-sources = true
counts = true
```

**Running commands from a subdirectory:**

Add `working-dir="<path>"` to a console block to run commands from a subdirectory:

    ```console working-dir=".venv"
    $ uv venv . --clear
    ```

**Creating empty directories:**

Use `tree create=true` to create directory structures without file content:

    ```tree create=true
    .
    └── .venv/
    ```

## Tree Snapshots

Tree snapshots show directories with trailing slashes:

    ```tree depth=2
    .
    └── .venv/
        ├── [BIN]/
        └── lib/
    ```

## What Can Be Migrated

Most tests can be migrated, including:

- **Multiple file setups** - Workspace tests with many `pyproject.toml` files, source files, etc.
  are fully supported via the `# file: path/to/file` directive
- **Tests with multiple commands** - Sequential `uv lock`, `uv sync`, `uv export` commands work fine
- **Platform-specific tests** - Use `target-family` to gate tests
- **Tests with subdirectory commands** - Use `working-dir` attribute on console blocks

## Known Limitations

### Features Not Yet Supported

- `with_versions_as_managed()` - Testing managed vs system Python preference
- Shell activation tests - Requires running in actual shell
- Some Windows-specific tests - May need platform-specific test infrastructure
