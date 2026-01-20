# Run with Requirements Files

Tests for `uv run` with requirements files using `--with-requirements`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Basic --with-requirements

### Install from requirements file

<!-- from run.rs::run_requirements_txt -->

The `--with-requirements` flag installs dependencies from a requirements file.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["anyio", "sniffio==1.3.1"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```python main.py
import sniffio
```

```toml
# file: requirements.txt
iniconfig
```

```console
$ uv run --with-requirements requirements.txt main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + foo==1.0.0 (from file://[TEMP_DIR]/)
 + idna==3.6
 + sniffio==1.3.1
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Satisfied requirement uses base environment

<!-- from run.rs::run_requirements_txt -->

A satisfied requirement uses the base environment.

```toml
# file: requirements.txt
sniffio
```

```console
$ uv run --with-requirements requirements.txt main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Audited 4 packages in [TIME]
```

### Different version installs separately

<!-- from run.rs::run_requirements_txt -->

A different version is installed in the ephemeral environment.

```toml
# file: requirements.txt
sniffio<1.3.1
```

```console
$ uv run --with-requirements requirements.txt main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Audited 4 packages in [TIME]
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + sniffio==1.3.0
```

### Combine with --with

<!-- from run.rs::run_requirements_txt -->

The `--with-requirements` flag can be combined with `--with`.

```toml
# file: requirements.txt
sniffio
```

```console
$ uv run --with-requirements requirements.txt --with iniconfig main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Audited 4 packages in [TIME]
Resolved 2 packages in [TIME]
Installed 2 packages in [TIME]
 + iniconfig==2.0.0
 + sniffio==1.3.1
```

### Empty requirements file

<!-- from run.rs::run_empty_requirements_txt -->

An empty requirements file shows a warning.

```toml
# file: pyproject.toml
[project]
name = "foo"
version = "1.0.0"
requires-python = ">=3.8"
dependencies = ["anyio", "sniffio==1.3.1"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: requirements.txt

```

```python main.py
import sniffio
```

```console
$ uv run --with-requirements requirements.txt main.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
Prepared 4 packages in [TIME]
Installed 4 packages in [TIME]
 + anyio==4.3.0
 + foo==1.0.0 (from file://[TEMP_DIR]/)
 + idna==3.6
 + sniffio==1.3.1
warning: Requirements file `requirements.txt` does not contain any dependencies
```
