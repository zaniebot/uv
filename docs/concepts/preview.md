# Preview features

uv includes opt-in preview features to provide an opportunity for community feedback and increase
confidence that changes are a net-benefit before enabling them for everyone.

## Enabling preview features

To enable all preview features, use the `--preview` flag:

```console
$ uv run --preview ...
```

Or, set the `UV_PREVIEW` environment variable:

```console
$ UV_PREVIEW=1 uv run ...
```

To enable specific preview features, use the `--preview-features` flag:

```console
$ uv run --preview-features foo ...
```

The `--preview-features` flag can be repeated to enable multiple features:

```console
$ uv run --preview-features foo --preview-features bar ...
```

Or, features can be provided in a comma separated list:

```console
$ uv run --preview-features foo,bar ...
```

The `UV_PREVIEW_FEATURES` environment variable can be used similarly, e.g.:

```console
$ UV_PREVIEW_FEATURES=foo,bar uv run ...
```

For backwards compatibility, enabling preview features that do not exist will warn, but not error.

## Using preview features

Often, preview features can be used without changing any preview settings if the behavior change is
gated by some sort of user interaction, For example, while `pylock.toml` support is in preview, you
can use `uv pip install` with a `pylock.toml` file without additional configuration because
specifying the `pylock.toml` file indicates you want to use the feature. However, a warning will be
displayed that the feature is in preview. The preview feature can be enabled to silence the warning.

Other preview features change behavior without changes to your use of uv. For example, when the
`python-upgrade` feature is enabled, the default behavior of `uv python install` changes to allow uv
to upgrade Python versions transparently. This feature requires enabling the preview flag for proper
usage.

## Available preview features

The following preview features are available:

- `add-bounds`: Allows configuring the
  [default bounds for `uv add`](../reference/settings.md#add-bounds) invocations.
- `adjust-ulimit`: Automatically raises the open file descriptor limit to the hard limit on Unix
  systems, preventing "too many open files" errors.
- `auth-helper`: Enables `uv auth helper`, which implements the
  [Bazel credential helper protocol](https://github.com/bazelbuild/proposals/blob/main/designs/2022-06-07-bazel-credential-helpers.md)
  for providing credentials to external tools.
- `cache-size`: Allows using `uv cache size` to display the total size of the uv cache directory.
- `detect-module-conflicts`: Warns when multiple installed packages provide modules with the same
  name.
- `direct-publish`: Allows using `uv publish --direct` to upload packages directly to storage using
  a two-phase upload protocol, bypassing the registry's upload endpoint.
- `extra-build-dependencies`: Allows
  [augmenting build dependencies](./projects/config.md#augmenting-build-dependencies) for packages
  that assume the presence of undeclared build requirements.
- `format`: Allows using `uv format`.
- `gcs-endpoint`: Enables Google Cloud Storage endpoint support for package indexes, configured via
  the `UV_GCS_ENDPOINT_URL` environment variable.
- `init-project-flag`: Enforces the removal of the deprecated `--project` flag in `uv init`,
  erroring instead of warning. Use `--directory` or a positional path argument instead.
- `json-output`: Allows `--output-format json` for various uv commands.
- `metadata-json`: Generates JSON versions of wheel metadata files (`WHEEL.json` and
  `METADATA.json`) alongside the standard text format when building wheels with the uv build
  backend.
- `native-auth`: Enables storage of credentials in a
  [system-native location](../concepts/authentication/http.md#the-uv-credentials-store).
- `package-conflicts`: Allows defining workspace conflicts at the package level.
- `pylock`: Allows installing from `pylock.toml` files.
- `python-install-default`: Allows
  [installing `python` and `python3` executables](./python-versions.md#installing-python-executables).
- `python-upgrade`: Allows
  [transparent Python version upgrades](./python-versions.md#upgrading-python-versions).
- `s3-endpoint`: Enables AWS S3 endpoint support for package indexes, configured via the
  `UV_S3_ENDPOINT_URL` environment variable.
- `sbom-export`: Allows exporting dependency lockfiles as a
  [Software Bill of Materials (SBOM)](./projects/export.md#cyclonedx-sbom-format) in CycloneDX
  format via `uv export --format cyclonedx1.5`.
- `special-conda-env-names`: Changes Conda environment detection to use path-based logic rather than
  treating "base" and "root" environment names as special cases.
- `target-workspace-discovery`: Changes `uv run` to discover the workspace starting from the target
  script's directory rather than the current working directory.
- `workspace-dir`: Allows using `uv workspace dir`.
- `workspace-list`: Allows using `uv workspace list`.
- `workspace-metadata`: Allows using `uv workspace metadata`.

## Disabling preview features

The `--no-preview` option can be used to disable preview features.
