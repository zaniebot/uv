# pip install Errors

Tests for error handling in `uv pip install`.

## Missing files

### Missing pyproject.toml

<!-- from pip_install.rs::missing_pyproject_toml -->

Installing from a missing pyproject.toml shows an error.

```console
$ uv pip install -r pyproject.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: File not found: `pyproject.toml`
```

## Invalid pyproject.toml

### Invalid TOML syntax

<!-- from pip_install.rs::invalid_pyproject_toml_syntax -->

A pyproject.toml with invalid TOML syntax produces a parse error.

```toml
# file: pyproject.toml
123 - 456
```

```console
$ uv pip install -r pyproject.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 1, column 5
    |
  1 | 123 - 456
    |     ^
  key with no value, expected `=`

error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 5
  |
1 | 123 - 456
  |     ^
key with no value, expected `=`
```

### Missing project name

<!-- from pip_install.rs::invalid_pyproject_toml_project_schema -->

A pyproject.toml with [project] but no name produces an error.

```toml
# file: pyproject.toml
[project]
```

```console
$ uv pip install -r pyproject.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `pyproject.toml`
  Caused by: TOML parse error at line 1, column 1
  |
1 | [project]
  | ^^^^^^^^^
`pyproject.toml` is using the `[project]` table, but the required `project.name` field is not set
```

## Invalid Python version

### Invalid major version

<!-- from pip_install.rs::invalid_python_version -->

An invalid Python version format produces an error.

```console
$ uv pip install flask --python-version 311
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: invalid value '311' for '--python-version <PYTHON_VERSION>': Python version `311` has an invalid major version (311)

For more information, try '--help'.
```

## Subcommand errors

### Missing pip subcommand

<!-- from pip_install.rs::missing_pip -->

Running `uv install` without `pip` suggests the correct command.

```console
$ uv install
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: unrecognized subcommand 'install'

  tip: a similar subcommand exists: 'uv pip install'

Usage: uv [OPTIONS] <COMMAND>

For more information, try '--help'.
```

### Invalid tool.uv option type

<!-- from pip_install.rs::invalid_pyproject_toml_option_schema -->

Using an invalid type for a tool.uv option produces a warning.

```toml
# file: pyproject.toml
[tool.uv]
index-url = true
```

```console
$ uv pip install iniconfig
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 2, column 13
    |
  2 | index-url = true
    |             ^^^^
  invalid type: boolean `true`, expected a string

Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

### Invalid TOML filename

<!-- from pip_install.rs::invalid_toml_filename -->

Using a non-standard .toml filename produces an error.

```toml test.toml
# empty
```

```console
$ uv pip install -r test.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: `test.toml` is not a valid PEP 751 filename: expected TOML file to start with `pylock.` and end with `.toml` (e.g., `pylock.toml`, `pylock.dev.toml`)
```

### Unsupported flags in requirements.txt

<!-- from pip_install.rs::install_unsupported_flag -->

Using unsupported flags in requirements.txt produces warnings.

```toml
# file: requirements.txt
--pre
--prefer-binary :all:
iniconfig
```

```console
$ uv pip install -r requirements.txt --strict
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Ignoring unsupported option in `requirements.txt`: `--pre` (hint: pass `--pre` on the command line instead)
warning: Ignoring unsupported option in `requirements.txt`: `--prefer-binary`
Resolved 1 package in [TIME]
Prepared 1 package in [TIME]
Installed 1 package in [TIME]
 + iniconfig==2.0.0
```

## Editable errors

### Editable with version specifier

<!-- from pip_install.rs::invalid_editable_no_url -->

An editable requirement with a version specifier produces an error.

```toml
# file: requirements.txt
-e black==0.1.0
```

```console
$ uv pip install -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unsupported editable requirement in `requirements.txt`
  Caused by: Editable `black` must refer to a local directory, not a versioned package
```

### Editable with unnamed HTTPS URL

<!-- from pip_install.rs::invalid_editable_unnamed_https_url -->

An editable requirement with an HTTPS URL produces an error.

```toml
# file: requirements.txt
-e https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl
```

```console
$ uv pip install -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unsupported editable requirement in `requirements.txt`
  Caused by: Editable must refer to a local directory, not an HTTPS URL: `https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl`
```

### Editable with named HTTPS URL

<!-- from pip_install.rs::invalid_editable_named_https_url -->

An editable requirement with a named HTTPS URL produces an error.

```toml
# file: requirements.txt
-e black @ https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl
```

```console
$ uv pip install -r requirements.txt
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Unsupported editable requirement in `requirements.txt`
  Caused by: Editable `black` must refer to a local directory, not an HTTPS URL: `https://files.pythonhosted.org/packages/0f/89/294c9a6b6c75a08da55e9d05321d0707e9418735e3062b12ef0f54c33474/black-24.4.2-py3-none-any.whl`
```

## Resolution errors

### Conflicting pinned versions

<!-- from pip_install.rs::install_requirements_txt_conflicting_pins -->

Installing with conflicting version pins produces an error.

```toml
# file: requirements.txt
blinker==1.7.0
click==7.0.0
flask==3.0.2
itsdangerous==2.1.2
jinja2==3.1.3
markupsafe==2.1.5
werkzeug==3.0.1
```

```console
$ uv pip install -r requirements.txt --strict
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × No solution found when resolving dependencies:
  ╰─▶ Because flask==3.0.2 depends on click>=8.1.3 and you require click==7.0.0, we can conclude that your requirements and flask==3.0.2 are incompatible.
      And because you require flask==3.0.2, we can conclude that your requirements are unsatisfiable.
```

### Unknown tool.uv field

<!-- from pip_install.rs::invalid_pyproject_toml_option_unknown_field -->

An unknown field in `[tool.uv]` produces a warning.

```toml
# file: pyproject.toml
[tool.uv]
unknown = "field"

[build-system]
requires = ["setuptools"]
build-backend = "setuptools.build_meta"
```

```console
$ uv pip install -r pyproject.toml
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Failed to parse `pyproject.toml` during settings discovery:
  TOML parse error at line 2, column 1
    |
  2 | unknown = "field"
    | ^^^^^^^
  unknown field `unknown`, expected one of [OPTIONS]

Resolved in [TIME]
Audited in [TIME]
```

### Disallowed managed field in uv.toml (auto-discovery)

<!-- from pip_install.rs::invalid_uv_toml_option_disallowed_automatic_discovery -->

The `managed` field is not allowed in a `uv.toml` file.

```toml uv.toml
managed = true
```

```console
$ uv pip install iniconfig
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `uv.toml`. The `managed` field is not allowed in a `uv.toml` file. `managed` is only applicable in the context of a project, and should be placed in a `pyproject.toml` file instead.
```

### Disallowed managed field in config file (command-line)

<!-- from pip_install.rs::invalid_uv_toml_option_disallowed_command_line -->

The `managed` field is not allowed in a config file passed via `--config-file`.

```toml foo.toml
managed = true
```

```console
$ uv pip install iniconfig --config-file foo.toml
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Failed to parse: `foo.toml`. The `managed` field is not allowed in a `uv.toml` file. `managed` is only applicable in the context of a project, and should be placed in a `pyproject.toml` file instead.
```
