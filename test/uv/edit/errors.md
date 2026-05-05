# Edit Errors

Tests for error handling in `uv add` and `uv remove`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Add errors

### Git reference without Git source

<!-- from edit.rs::add_git_raw -->

Providing a Git reference (--tag) without a Git source produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add flask --tag 0.0.1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a Git repository, but a Git reference (`--tag 0.0.1`) was provided.
```

### Git branch on URL source

<!-- from edit.rs::add_git_raw -->

Providing a Git reference (--branch) with a non-Git URL source produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add "flask @ https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl" --branch 0.0.1
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a Git repository, but a Git reference (`--branch 0.0.1`) was provided.
```

### Editable on non-local source

<!-- from edit.rs::add_raw_editable -->

Using --editable with a non-local source produces an error.

```toml
# file: pyproject.toml
[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

```console
$ uv add "flask @ https://files.pythonhosted.org/packages/61/80/ffe1da13ad9300f87c93af113edd0638c75138c42a0994becfacac078c06/flask-3.0.3-py3-none-any.whl" --editable
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `flask` did not resolve to a local directory, but the `--editable` flag was provided. Editable installs are only supported for local directories.
```
