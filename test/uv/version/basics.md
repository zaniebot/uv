# Version Basics

Tests for basic `uv version` functionality.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Read version

### Get project version

<!-- from version.rs::version_get -->

Display the project version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31

----- stderr -----
```

### Get version short form

<!-- from version.rs::version_get_short -->

The `--short` flag shows only the version number.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --short
success: true
exit_code: 0
----- stdout -----
1.10.31

----- stderr -----
```

## Set version

### Set specific version

<!-- from version.rs::version_set_value -->

Set the project version to a specific value.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version 1.1.1
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 1.1.1

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Set version with --short

<!-- from version.rs::version_set_value_short -->

The `--short` flag shows only the new version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version 1.1.1 --short
success: true
exit_code: 0
----- stdout -----
1.1.1

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

## Bump version

### Bump patch version

<!-- from version.rs::version_bump_patch -->

The `--bump patch` flag increments the patch version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --bump patch
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 1.10.32

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump patch to specific value

<!-- from version.rs::version_bump_patch_value -->

The `--bump patch=N` flag sets the patch version to N.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --bump patch=40
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 1.10.40

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump minor to specific value

<!-- from version.rs::version_bump_minor_value -->

The `--bump minor=N` flag sets the minor version and resets patch.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.2.3"
requires-python = ">=3.12"
```

```console
$ uv version --bump minor=10
success: true
exit_code: 0
----- stdout -----
myproject 1.2.3 => 1.10.0

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump minor version

<!-- from version.rs::version_bump_minor -->

The `--bump minor` flag increments the minor version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --bump minor
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 1.11.0

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump major version

<!-- from version.rs::version_bump_major -->

The `--bump major` flag increments the major version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --bump major
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 2.0.0

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

## Dry run

### Version dry run

<!-- from version.rs::version_dry_run -->

The `--dry-run` flag shows what would change.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --bump major --dry-run
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 2.0.0

----- stderr -----
```

## JSON output

### Get version JSON

<!-- from version.rs::version_get_json -->

The `--output-format json` flag outputs JSON.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --output-format json
success: true
exit_code: 0
----- stdout -----
{
  "package_name": "myproject",
  "version": "1.10.31",
  "commit_info": null
}

----- stderr -----
```

## Bump to specific values

### Bump major to specific value

<!-- from version.rs::version_bump_major_value -->

The `--bump major=N` flag sets the major version to N and resets minor and patch.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump major=7
success: true
exit_code: 0
----- stdout -----
myproject 2.3.4 => 7.0.0

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

## Pre-release bumps

### Bump alpha

<!-- from version.rs::bump_alpha -->

The `--bump alpha` flag increments the alpha pre-release.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "9!2.3.4a5.post6.dev7+deadbeef6"
requires-python = ">=3.12"
```

```console
$ uv version --bump alpha
success: true
exit_code: 0
----- stdout -----
myproject 9!2.3.4a5.post6.dev7+deadbeef6 => 9!2.3.4a6+deadbeef6

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump beta

<!-- from version.rs::bump_beta -->

The `--bump beta` flag transitions to a beta pre-release.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "9!2.3.4a5.post6.dev7+deadbeef6"
requires-python = ">=3.12"
```

```console
$ uv version --bump beta
success: true
exit_code: 0
----- stdout -----
myproject 9!2.3.4a5.post6.dev7+deadbeef6 => 9!2.3.4b1+deadbeef6

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump beta with specific value

<!-- from version.rs::bump_beta_with_value_existing -->

The `--bump beta=N` flag sets the beta number to N.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.2.3b4"
requires-python = ">=3.12"
```

```console
$ uv version --bump beta=42
success: true
exit_code: 0
----- stdout -----
myproject 1.2.3b4 => 1.2.3b42

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump release candidate

<!-- from version.rs::bump_rc -->

The `--bump rc` flag bumps to a release candidate version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "9!2.3.4a5.post6.dev7+deadbeef6"
requires-python = ">=3.12"
```

```console
$ uv version --bump rc
success: true
exit_code: 0
----- stdout -----
myproject 9!2.3.4a5.post6.dev7+deadbeef6 => 9!2.3.4rc1+deadbeef6

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump post release

<!-- from version.rs::bump_post -->

The `--bump post` flag bumps the post-release number.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "9!2.3.4a5.post6.dev7+deadbeef6"
requires-python = ">=3.12"
```

```console
$ uv version --bump post
success: true
exit_code: 0
----- stdout -----
myproject 9!2.3.4a5.post6.dev7+deadbeef6 => 9!2.3.4a5.post7+deadbeef6

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump dev release

<!-- from version.rs::bump_dev -->

The `--bump dev` flag bumps the dev release number.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "9!2.3.4a5.post6.dev7+deadbeef6"
requires-python = ">=3.12"
```

```console
$ uv version --bump dev
success: true
exit_code: 0
----- stdout -----
myproject 9!2.3.4a5.post6.dev7+deadbeef6 => 9!2.3.4a5.post6.dev8+deadbeef6

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

## Validation

### Bump must increase version

<!-- from version.rs::version_bump_patch_value_must_increase -->

Bumping to a lower version produces an error.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.0.12"
requires-python = ">=3.12"
```

```console
$ uv version --bump patch=11
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: 0.0.12 => 0.0.11 didn't increase the version; provide the exact version to force an update
```

### Invalid version format

<!-- from version.rs::version_set_invalid -->

Setting an invalid version produces an error.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version abcd
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: expected version to start with a number, but no leading ASCII digits were found
```

### Missing bump flag hint

<!-- from version.rs::version_missing_bump -->

Providing a bump name without `--bump` shows a helpful error.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version minor
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Invalid version `minor`, did you mean to pass `--bump minor`?
```

### Bump patch with short flag

<!-- from version.rs::version_bump_patch_short -->

The `--bump patch` combined with `--short` shows only the new version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --bump patch --short
success: true
exit_code: 0
----- stdout -----
1.10.32

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Set version dry run

<!-- from version.rs::version_set_dry -->

The `--dry-run` flag shows what would change without modifying the file.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version 1.2.3 --dry-run
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 1.2.3

----- stderr -----
```

### Bump stable

<!-- from version.rs::bump_stable -->

The `--bump stable` flag removes pre-release and dev suffixes.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "9!2.3.4a5.post6.dev7+deadbeef6"
requires-python = ">=3.12"
```

```console
$ uv version --bump stable
success: true
exit_code: 0
----- stdout -----
myproject 9!2.3.4a5.post6.dev7+deadbeef6 => 9!2.3.4+deadbeef6

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump stable with value fails

<!-- from version.rs::bump_stable_with_value_fails -->

The `--bump stable` flag does not accept a value.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.2.3"
requires-python = ">=3.12"
```

```console
$ uv version --bump stable=1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--bump stable` does not accept a value
```

### Bump with empty value fails

<!-- from version.rs::bump_empty_value_fails -->

An empty value for `--bump` produces an error.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.2.3"
requires-python = ">=3.12"
```

```console
$ uv version --bump patch=
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--bump` values cannot be empty
```

### Bump with invalid numeric value fails

<!-- from version.rs::bump_invalid_numeric_value_fails -->

A non-numeric value for `--bump` produces an error.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.2.3"
requires-python = ">=3.12"
```

```console
$ uv version --bump dev=foo
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: invalid numeric value `foo` for `--bump dev`
```

### Bump dev with value

<!-- from version.rs::bump_dev_with_value -->

The `--bump dev=N` flag sets the dev number to N.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.1.0.dev4"
requires-python = ">=3.12"
```

```console
$ uv version --bump dev=42
success: true
exit_code: 0
----- stdout -----
myproject 0.1.0.dev4 => 0.1.0.dev42

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump beta with value on new version

<!-- from version.rs::bump_beta_with_value_new -->

Multiple `--bump` flags can be combined to bump patch and add beta.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.2.3"
requires-python = ">=3.12"
```

```console
$ uv version --bump beta=5 --bump patch
success: true
exit_code: 0
----- stdout -----
myproject 1.2.3 => 1.2.4b5

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump post with value clears dev

<!-- from version.rs::bump_post_with_value_clears_dev -->

The `--bump post=N` flag clears the dev release.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.2.3.post4.dev9"
requires-python = ">=3.12"
```

```console
$ uv version --bump post=10
success: true
exit_code: 0
----- stdout -----
myproject 1.2.3.post4.dev9 => 1.2.3.post10

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump stable decreases version

<!-- from version.rs::bump_decrease_stable -->

Bumping stable on a post release decreases the version.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4.post6"
requires-python = ">=3.12"
```

```console
$ uv version --bump stable
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: 2.3.4.post6 => 2.3.4 didn't increase the version; provide the exact version to force an update
```

### Bump double major fails

<!-- from version.rs::bump_double_major -->

Using the same release component twice fails.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump major --bump major
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Only one release version component can be provided to `--bump`, got: major, major
```

### Bump double alpha fails

<!-- from version.rs::bump_double_alpha -->

Using the same pre-release component twice fails.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump alpha --bump alpha
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Only one pre-release version component can be provided to `--bump`, got: alpha, alpha
```

### Bump major with alpha

<!-- from version.rs::bump_alpha_major -->

Combining `--bump major` with `--bump alpha` creates a pre-release of the next major.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump major --bump alpha
success: true
exit_code: 0
----- stdout -----
myproject 2.3.4 => 3.0.0a1

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump alpha with dev fails

<!-- from version.rs::bump_alpha_dev -->

Combining `--bump alpha` with `--bump dev` fails as both are pre-release.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump alpha --bump dev
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Only one pre-release version component can be provided to `--bump`, got: alpha, dev
```

### Bump major with dev

<!-- from version.rs::bump_dev_major -->

Combining `--bump major` with `--bump dev` creates a dev release of the next major.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump major --bump dev
success: true
exit_code: 0
----- stdout -----
myproject 2.3.4 => 3.0.0.dev1

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump major with post fails

<!-- from version.rs::bump_post_major -->

Combining `--bump major` with `--bump post` fails.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump major --bump post
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--bump post` cannot be used with another `--bump` value, got: major, post
```

### Bump stable with major fails

<!-- from version.rs::bump_stable_major -->

Combining `--bump stable` with `--bump major` fails.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump stable --bump major
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--bump stable` cannot be used with another `--bump` value, got: stable, major
```

### Bump major dry run

<!-- from version.rs::version_major_dry -->

The `--dry-run` flag shows what would change without modifying the file.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31"
requires-python = ">=3.12"
```

```console
$ uv version --bump major --dry-run
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31 => 2.0.0

----- stderr -----
```

## Incomplete versions

### Bump patch on two-part version

<!-- from version.rs::version_patch_uncompleted -->

Bumping patch on a version without patch adds a patch component.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.1"
requires-python = ">=3.12"
```

```console
$ uv version --bump patch
success: true
exit_code: 0
----- stdout -----
myproject 0.1 => 0.1.1

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump minor on two-part version

<!-- from version.rs::version_minor_uncompleted -->

Bumping minor on a two-part version increments the minor.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.1"
requires-python = ">=3.12"
```

```console
$ uv version --bump minor
success: true
exit_code: 0
----- stdout -----
myproject 0.1 => 0.2

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump major on two-part version

<!-- from version.rs::version_major_uncompleted -->

Bumping major on a two-part version increments the major and resets minor.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.1"
requires-python = ">=3.12"
```

```console
$ uv version --bump major
success: true
exit_code: 0
----- stdout -----
myproject 0.1 => 1.0

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

## Pre-release versions

### Bump major on dev version

<!-- from version.rs::version_major_dev -->

Bumping major on a dev version removes the dev suffix.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31.dev10"
requires-python = ">=3.12"
```

```console
$ uv version --bump major
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31.dev10 => 2.0.0

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump major on post version

<!-- from version.rs::version_major_post -->

Bumping major on a post-release version removes the post suffix.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "1.10.31.post10"
requires-python = ">=3.12"
```

```console
$ uv version --bump major
success: true
exit_code: 0
----- stdout -----
myproject 1.10.31.post10 => 2.0.0

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Bump alpha on beta decreases version

<!-- from version.rs::bump_decrease_alpha_beta -->

Bumping to alpha on a beta version fails as it decreases.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4b5"
requires-python = ">=3.12"
```

```console
$ uv version --bump alpha
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: 2.3.4b5 => 2.3.4a1 didn't increase the version; provide the exact version to force an update
```

### Bump alpha on stable decreases version

<!-- from version.rs::bump_decrease_alpha_stable -->

Bumping to alpha on a stable version requires also bumping a release component.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "2.3.4"
requires-python = ">=3.12"
```

```console
$ uv version --bump alpha
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: 2.3.4 => 2.3.4a1 didn't increase the version; when bumping to a pre-release version you also need to increase a release version component, e.g., with `--bump <major|minor|patch>`
```

### Bump patch and dev together

<!-- from version.rs::bump_patch_and_dev_value -->

Combining `--bump patch` with `--bump dev=N` creates a dev release of the next patch.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "0.0.1"
requires-python = ">=3.12"
```

```console
$ uv version --bump patch --bump dev=66463664
success: true
exit_code: 0
----- stdout -----
myproject 0.0.1 => 0.0.2.dev66463664

----- stderr -----
Resolved 1 package in [TIME]
Audited in [TIME]
```

### Many bump flags with incompatible post

<!-- from version.rs::many_bump_complex -->

Using `--bump post` with other bump flags fails.

```toml
# file: pyproject.toml
[project]
name = "myproject"
version = "9!2.3.4a5.post6.dev7+deadbeef6"
requires-python = ">=3.12"
```

```console
$ uv version --bump major --bump patch --bump alpha --bump minor --bump dev --bump minor --bump post --bump post
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `--bump post` cannot be used with another `--bump` value, got: major, patch, alpha, minor, dev, minor, post, post
```

## Missing project

### No pyproject.toml shows hint

<!-- from version.rs::version_get_missing_with_hint -->

Running `uv version` without a pyproject.toml suggests `uv self version`.

```console
$ uv version
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: No `pyproject.toml` found in current directory or any parent directory

hint: If you meant to view uv's version, use `uv self version` instead
```

## Self version

### Get uv version

<!-- from version.rs::self_version -->

`uv self version` shows the uv tool version, not the project version.

```toml
# file: pyproject.toml
[project]
name = "myapp"
version = "0.1.2"
```

```console
$ uv self version
success: true
exit_code: 0
----- stdout -----
uv [VERSION] ([COMMIT] DATE)

----- stderr -----
```

<!--
Note: The following tests cannot be migrated to mdtest:

- self_version_short: Uses [VERSION] placeholder for dynamic version output
- test_self_update_offline_error: Output depends on how uv was installed (package manager vs standalone)
-->
