# Project Initialization - Git/VCS Integration

Tests for Git and VCS integration during `uv init`.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Git

<!-- Derived from [`init::init_git`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2806-L2843) -->

By default, `uv init` creates a Git repository with .gitignore.

```console
$ uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

A .gitignore file is created:

```text title="foo/.gitignore" snapshot=true
# Python-generated files
__pycache__/
*.py[oc]
build/
dist/
wheels/
*.egg-info

# Virtual environments
.venv
```

A .git directory is created:

```console
$ test -d foo/.git && echo "Git repository exists"
success: true
exit_code: 0
----- stdout -----
Git repository exists

----- stderr -----
```

## VCS none

<!-- Derived from [`init::init_vcs_none`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2845-L2862) -->

The `--vcs none` flag skips Git initialization.

```console
$ uv init --vcs none foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

No .gitignore is created:

```console
$ test -f foo/.gitignore && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

No .git directory is created:

```console
$ test -d foo/.git && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

## Inside Git repo

<!-- Derived from [`init::init_inside_git_repo`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2864-L2900) -->

When inside an existing Git repository, `uv init` does not create a nested repository.

```toml
# mdtest

[environment]
required-features = "git"
```

Initialize a Git repository:

```console
$ git init
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

Initialize a project with explicit `--vcs git`:

```console
$ uv init --vcs git foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

No .gitignore is created in the subdirectory:

```console
$ test -f foo/.gitignore && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

Initialize another project without explicit VCS flag:

```console
$ uv init bar
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `bar` at `[TEMP_DIR]/bar`
```

No .gitignore is created:

```console
$ test -f bar/.gitignore && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

## Git not installed

<!-- Derived from [`init::init_git_not_installed`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L2902-L2929) -->

When Git is not available, `uv init` succeeds without Git initialization by default.

```console
$ PATH="" uv init foo
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `foo` at `[TEMP_DIR]/foo`
```

With explicit `--vcs git`, the command fails:

```console
$ PATH="" uv init --vcs git bar
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Attempted to initialize a Git repository, but `git` was not found in PATH
```

## Git states

<!-- Derived from [`init::git_states`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/init.rs#L4016-L4073) -->

Test different combinations of Git availability and VCS flags.

When Git is available, default init creates a repository:

```console
$ uv init working
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `working` at `[TEMP_DIR]/working`
```

Verify Git repository exists:

```console
$ test -d working/.git && echo "exists"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```

With `--vcs none`, no repository is created:

```console
$ uv init --vcs none working-no-git
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `working-no-git` at `[TEMP_DIR]/working-no-git`
```

Verify no Git repository:

```console
$ test -d working-no-git/.git && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

With explicit `--vcs git`, repository is created:

```console
$ uv init --vcs git working-git
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `working-git` at `[TEMP_DIR]/working-git`
```

Verify Git repository exists:

```console
$ test -d working-git/.git && echo "exists"
success: true
exit_code: 0
----- stdout -----
exists

----- stderr -----
```
