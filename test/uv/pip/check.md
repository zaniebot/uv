# pip check

Check for package incompatibilities in the current environment.

## Compatible packages

<!-- from pip_check.rs::check_compatible_packages -->

Create requirements file:

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

Install compatible packages:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

Check for package incompatibilities:

```console
$ uv pip check
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Checked 5 packages in [TIME]
All installed packages are compatible
```

## Incompatible packages

<!-- from pip_check.rs::check_incompatible_packages -->

requests 2.31.0 requires idna (<4,>=2.5). This test force-installs idna 2.4 to trigger a failure.

Create requirements file:

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

Install compatible packages:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

Install incompatible idna version:

```toml title="requirements_idna.txt" snapshot=true
idna==2.4
```

```console
$ uv pip install -r requirements_idna.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Uninstalled 1 package in [TIME]
Installed 1 package in [TIME]
 - idna==3.6
 + idna==2.4
warning: The package `requests` requires `idna>=2.5,<4`, but `2.4` is installed
```

Check for incompatibilities:

```console
$ uv pip check
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Checked 5 packages in [TIME]
Found 1 incompatibility
The package `requests` requires `idna>=2.5,<4`, but `2.4` is installed
```

## Multiple incompatible packages

<!-- from pip_check.rs::check_multiple_incompatible_packages -->

requests 2.31.0 requires idna (<4,>=2.5) and urllib3<3,>=1.21.1. This test force-installs idna 2.4
and urllib3 1.20 to trigger multiple incompatibilities.

Create requirements file:

```toml title="requirements.txt" snapshot=true
requests==2.31.0
```

Install compatible packages:

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
Prepared 5 packages in [TIME]
Installed 5 packages in [TIME]
 + certifi==2024.2.2
 + charset-normalizer==3.3.2
 + idna==3.6
 + requests==2.31.0
 + urllib3==2.2.1
```

Install two incompatible package versions:

```toml title="requirements_two.txt" snapshot=true
idna==2.4
urllib3==1.20
```

```console
$ uv pip install -r requirements_two.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
Prepared 2 packages in [TIME]
Uninstalled 2 packages in [TIME]
Installed 2 packages in [TIME]
 - idna==3.6
 + idna==2.4
 - urllib3==2.2.1
 + urllib3==1.20
warning: The package `requests` requires `idna>=2.5,<4`, but `2.4` is installed
warning: The package `requests` requires `urllib3>=1.21.1,<3`, but `1.20` is installed
```

Check for multiple incompatibilities:

```console
$ uv pip check
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Checked 5 packages in [TIME]
Found 2 incompatibilities
The package `requests` requires `idna>=2.5,<4`, but `2.4` is installed
The package `requests` requires `urllib3>=1.21.1,<3`, but `1.20` is installed
```

## Python version

<!-- from pip_check.rs::check_python_version -->

Check Python version compatibility using --python-version flag.

Install urllib3:

```console
$ uv pip install urllib3 --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + urllib3==2.2.1
```

Check with a Python version that's incompatible with urllib3:

```console
$ uv pip check --python-version 3.7
success: false
exit_code: 1
----- stdout -----

----- stderr -----
Checked 1 package in [TIME]
Found 1 incompatibility
The package `urllib3` requires Python >=3.8, but `3.12.[X]` is installed
```
