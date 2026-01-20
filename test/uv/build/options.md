# Package Building - Options

Tests for build command options and flags.

```toml
# mdtest

[environment]
python-version = "3.12"
```

## Constraints

<!-- Derived from [`build::build_constraints`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L837-L895) -->

The `--build-constraint` flag constrains build dependencies.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["hatchling>=1.0"]
build-backend = "hatchling.build"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

```text
# file: project/constraints.txt
hatchling==0.1.0
```

Build with conflicting constraint fails:

```console
$ cd project && uv build --build-constraint constraints.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
  × Failed to build `[TEMP_DIR]/project`
  ├─▶ Failed to resolve requirements from `build-system.requires`
  ├─▶ No solution found when resolving: `hatchling>=1.0`
  ╰─▶ Because you require hatchling>=1.0 and hatchling==0.1.0, we can conclude that your requirements are unsatisfiable.
```

No artifacts are created:

```console
$ test -f project/dist/project-0.1.0.tar.gz && echo "exists" || echo "missing"
success: true
exit_code: 0
----- stdout -----
missing

----- stderr -----
```

## SHA

<!-- Derived from [`build::build_sha`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L898-L1099) -->

Hash validation works with `--build-constraint` and `--require-hashes`.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

Incorrect hash is rejected:

```text
# file: project/constraints.txt
hatchling==1.22.4 \
    --hash=sha256:a248cb506794bececcddeddb1678bc722f9cfcacf02f98f7c0af6b9ed893caf2 \
    --hash=sha256:e16da5bfc396af7b29daa3164851dd04991c994083f56cb054b5003675caecdc
packaging==24.0 \
    --hash=sha256:2ddfb553fdf02fb784c234c7ba6ccc288296ceabec964ad2eae3777778130bc5 \
    --hash=sha256:eb82c5e3e56209074766e6885bb04b8c38a0c015d0a30036ebe7ece34c9989e9
    # via hatchling
pathspec==0.12.1 \
    --hash=sha256:a0d503e138a4c123b27490a4f7beda6a01c6f288df0e4a8b79c7eb0dc7b4cc08 \
    --hash=sha256:a482d51503a1ab33b1c67a6c3813a26953dbdc71c31dacaef9a838c4e29f5712
    # via hatchling
pluggy==1.4.0 \
    --hash=sha256:7db9f7b503d67d1c5b95f59773ebb58a8c1c288129a88665838012cfb07b8981 \
    --hash=sha256:8c85c2876142a764e5b7548e7d9a0e0ddb46f5185161049a79b7e974454223be
    # via hatchling
tomli==2.0.1 \
    --hash=sha256:939de3e7a6161af0c887ef91b7d41a53e7c5a1ca976325f429cb46ea9bc30ecc \
    --hash=sha256:de526c12914f0c550d15924c62d72abc48d6fe7364aa87328337a31007fe8a4f
    # via hatchling
trove-classifiers==2024.3.3 \
    --hash=sha256:3a84096861b385ec422c79995d1f6435dde47a9b63adaa3c886e53232ba7e6e0 \
    --hash=sha256:df7edff9c67ff86b733628998330b180e81d125b1e096536d83ac0fd79673fdc
    # via hatchling
```

```console
$ cd project && uv build --build-constraint constraints.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
  × Failed to build `[TEMP_DIR]/project`
  ├─▶ Failed to install requirements from `build-system.requires`
  ├─▶ Failed to download `hatchling==1.22.4`
  ╰─▶ Hash mismatch for `hatchling==1.22.4`

      Expected:
        sha256:a248cb506794bececcddeddb1678bc722f9cfcacf02f98f7c0af6b9ed893caf2
        sha256:e16da5bfc396af7b29daa3164851dd04991c994083f56cb054b5003675caecdc

      Computed:
        sha256:f56da5bfc396af7b29daa3164851dd04991c994083f56cb054b5003675caecdc
```

Missing hash with `--require-hashes` is rejected:

```text
# file: project/constraints.txt
hatchling==1.22.4
```

```console
$ cd project && uv build --build-constraint constraints.txt --require-hashes
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
  × Failed to build `[TEMP_DIR]/project`
  ├─▶ Failed to resolve requirements from `build-system.requires`
  ├─▶ No solution found when resolving: `hatchling`
  ╰─▶ In `--require-hashes` mode, all requirements must be pinned upfront with `==`, but found: `hatchling`
```

Correct hash succeeds:

```text
# file: project/constraints.txt
hatchling==1.22.4 \
    --hash=sha256:8a2dcec96d7fb848382ef5848e5ac43fdae641f35a08a3fab5116bd495f3416e \
    --hash=sha256:f56da5bfc396af7b29daa3164851dd04991c994083f56cb054b5003675caecdc
packaging==24.0 \
    --hash=sha256:2ddfb553fdf02fb784c234c7ba6ccc288296ceabec964ad2eae3777778130bc5 \
    --hash=sha256:eb82c5e3e56209074766e6885bb04b8c38a0c015d0a30036ebe7ece34c9989e9
    # via hatchling
pathspec==0.12.1 \
    --hash=sha256:a0d503e138a4c123b27490a4f7beda6a01c6f288df0e4a8b79c7eb0dc7b4cc08 \
    --hash=sha256:a482d51503a1ab33b1c67a6c3813a26953dbdc71c31dacaef9a838c4e29f5712
    # via hatchling
pluggy==1.4.0 \
    --hash=sha256:7db9f7b503d67d1c5b95f59773ebb58a8c1c288129a88665838012cfb07b8981 \
    --hash=sha256:8c85c2876142a764e5b7548e7d9a0e0ddb46f5185161049a79b7e974454223be
    # via hatchling
tomli==2.0.1 \
    --hash=sha256:939de3e7a6161af0c887ef91b7d41a53e7c5a1ca976325f429cb46ea9bc30ecc \
    --hash=sha256:de526c12914f0c550d15924c62d72abc48d6fe7364aa87328337a31007fe8a4f
    # via hatchling
trove-classifiers==2024.3.3 \
    --hash=sha256:3a84096861b385ec422c79995d1f6435dde47a9b63adaa3c886e53232ba7e6e0 \
    --hash=sha256:df7edff9c67ff86b733628998330b180e81d125b1e096536d83ac0fd79673fdc
    # via hatchling
```

```console
$ cd project && uv build --build-constraint constraints.txt
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built dist/project-0.1.0.tar.gz
Successfully built dist/project-0.1.0-py3-none-any.whl
```

Artifacts are created:

```console
$ test -f project/dist/project-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

## Quiet

<!-- Derived from [`build::build_quiet`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1102-L1138) -->

The `-q`/`--quiet` flag suppresses all output.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

Quiet build produces no output:

```console
$ uv build project -q
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

## No build logs

<!-- Derived from [`build::build_no_build_logs`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1141-L1181) -->

The `--no-build-logs` flag suppresses build backend output but shows uv messages.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

Build with no build logs:

```console
$ uv build project --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

## Hide output env var

<!-- Derived from [`build::build_hide_build_output_env_var`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1185-L1225) -->

The `UV_HIDE_BUILD_OUTPUT` environment variable suppresses build output.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.0"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

```python
# file: project/src/project/__init__.py
```

```text
# file: project/README
```

Build with UV_HIDE_BUILD_OUTPUT:

```console
$ UV_HIDE_BUILD_OUTPUT=1 uv build project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

## Hide output on failure

<!-- Derived from [`build::build_hide_build_output_on_failure`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L1229-L1284) -->

`UV_HIDE_BUILD_OUTPUT` hides output even when the build fails.

```toml
# file: project/pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"

[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"
```

```python
# file: project/setup.py
import os
import sys
print("FOO=" + os.environ.get("FOO", "not-set"), file=sys.stderr)
sys.stderr.flush()
raise Exception("Build failed intentionally!")
```

Without the environment variable, the output is shown:

```console
$ FOO=bar uv build project
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
FOO=bar
  × Failed to build `[TEMP_DIR]/project`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `setuptools.build_meta.build_sdist` failed (exit status: 1)
      hint: This usually indicates a problem with the package or the build environment.
```

With the environment variable, the output is hidden:

```console
$ FOO=bar UV_HIDE_BUILD_OUTPUT=1 uv build project
success: false
exit_code: 2
----- stdout -----

----- stderr -----
Building source distribution...
  × Failed to build `[TEMP_DIR]/project`
  ├─▶ The build backend returned an error
  ╰─▶ Call to `setuptools.build_meta.build_sdist` failed (exit status: 1)
      hint: This usually indicates a problem with the package or the build environment.
```

## Clear

<!-- Derived from [`build::build_clear`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L2248-L2311) -->

The `--clear` flag removes the output directory before building.

Initialize and build a project:

```console
$ uv init project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project` at `[TEMP_DIR]/project`
```

```console
$ uv build project --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

Artifacts are created:

```console
$ test -f project/dist/project-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

Add a marker file:

```console
$ echo "marker" > project/dist/marker.txt && test -f project/dist/marker.txt && echo "marker exists"
success: true
exit_code: 0
----- stdout -----
marker exists

----- stderr -----
```

Build with `--clear` removes the marker:

```console
$ uv build project --clear --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

Marker is gone but artifacts are recreated:

```console
$ test -f project/dist/marker.txt && echo "marker exists" || echo "marker missing"
success: true
exit_code: 0
----- stdout -----
marker missing

----- stderr -----
```

```console
$ test -f project/dist/project-0.1.0.tar.gz && echo "sdist exists"
success: true
exit_code: 0
----- stdout -----
sdist exists

----- stderr -----
```

## No gitignore

<!-- Derived from [`build::build_no_gitignore`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/build.rs#L2315-L2362) -->

The `--no-create-gitignore` flag skips creating .gitignore in the dist directory.

Initialize and build a project with default behavior:

```console
$ uv init project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Initialized project `project` at `[TEMP_DIR]/project`
```

```console
$ uv build project --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

.gitignore is created by default:

```console
$ test -f project/dist/.gitignore && echo "gitignore exists"
success: true
exit_code: 0
----- stdout -----
gitignore exists

----- stderr -----
```

Remove and rebuild without .gitignore:

```console
$ rm -rf project/dist && uv build project --no-create-gitignore --no-build-logs
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Building source distribution...
Building wheel from source distribution...
Successfully built project/dist/project-0.1.0.tar.gz
Successfully built project/dist/project-0.1.0-py3-none-any.whl
```

.gitignore is not created:

```console
$ test -f project/dist/.gitignore && echo "gitignore exists" || echo "gitignore missing"
success: true
exit_code: 0
----- stdout -----
gitignore missing

----- stderr -----
```
