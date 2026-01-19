# Export to CycloneDX

Tests for `uv export` to CycloneDX SBOM format.

```toml
# mdtest

[environment]
python-version = "3.12"

[filters]
cyclonedx = true
```

## Basic export

<!-- Derived from [`export::cyclonedx_export_basic`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L4881-L4953) -->

Exporting a simple project to CycloneDX format.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3==2.2.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "urllib3-2@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    }
  ],
  "dependencies": [
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "urllib3-2@2.2.0"
      ]
    },
    {
      "ref": "urllib3-2@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 2 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## No dependencies

<!-- Derived from [`export::cyclonedx_export_no_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs) -->

Exporting a project with no dependencies.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 1 package in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [],
  "dependencies": [
    {
      "ref": "project-1@0.1.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 1 package in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Project extra

<!-- Derived from [`export::cyclonedx_export_project_extra`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs) -->

Exporting a project with optional dependencies.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig"]

[project.optional-dependencies]
async = ["urllib3==2.2.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

Default export (without extra):

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "iniconfig-2@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    }
  ],
  "dependencies": [
    {
      "ref": "iniconfig-2@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "iniconfig-2@2.0.0"
      ]
    }
  ]
}
----- stderr -----
Resolved 3 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

Export with the `async` extra:

```console
$ uv export --format cyclonedx1.5 --extra async
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "iniconfig-2@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-3@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    }
  ],
  "dependencies": [
    {
      "ref": "iniconfig-2@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "iniconfig-2@2.0.0",
        "urllib3-3@2.2.0"
      ]
    },
    {
      "ref": "urllib3-3@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 3 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Dependency marker

<!-- Derived from [`export::cyclonedx_export_dependency_marker`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L6525-L6618) -->

Exporting a project with environment markers includes the marker in the output.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "urllib3==2.2.1 ; sys_platform == 'darwin'",
    "iniconfig==2.0.0",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "iniconfig-2@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-3@2.2.1",
      "name": "urllib3",
      "version": "2.2.1",
      "purl": "pkg:pypi/urllib3@2.2.1",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "sys_platform == 'darwin'"
        }
      ]
    }
  ],
  "dependencies": [
    {
      "ref": "iniconfig-2@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "iniconfig-2@2.0.0",
        "urllib3-3@2.2.1"
      ]
    },
    {
      "ref": "urllib3-3@2.2.1",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 3 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Dependency extra

<!-- Derived from [`export::cyclonedx_export_dependency_extra`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L6745-L6867) -->

Exporting a project with a dependency that has extras.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["cryptography[ssh]==42.0.5"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "bcrypt-2@4.1.2",
      "name": "bcrypt",
      "version": "4.1.2",
      "purl": "pkg:pypi/bcrypt@4.1.2"
    },
    {
      "type": "library",
      "bom-ref": "cffi-3@1.16.0",
      "name": "cffi",
      "version": "1.16.0",
      "purl": "pkg:pypi/cffi@1.16.0",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "platform_python_implementation != 'PyPy'"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "cryptography-4@42.0.5",
      "name": "cryptography",
      "version": "42.0.5",
      "purl": "pkg:pypi/cryptography@42.0.5"
    },
    {
      "type": "library",
      "bom-ref": "pycparser-5@2.21",
      "name": "pycparser",
      "version": "2.21",
      "purl": "pkg:pypi/pycparser@2.21",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "platform_python_implementation != 'PyPy'"
        }
      ]
    }
  ],
  "dependencies": [
    {
      "ref": "bcrypt-2@4.1.2",
      "dependsOn": []
    },
    {
      "ref": "cffi-3@1.16.0",
      "dependsOn": [
        "pycparser-5@2.21"
      ]
    },
    {
      "ref": "cryptography-4@42.0.5",
      "dependsOn": [
        "bcrypt-2@4.1.2",
        "cffi-3@1.16.0"
      ]
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "cryptography-4@42.0.5"
      ]
    },
    {
      "ref": "pycparser-5@2.21",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 5 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Dev dependencies

<!-- Derived from [`export::cyclonedx_export_dev_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L7856-L8035) -->

Exporting a project with dev dependencies.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions==4.10.0"]

[tool.uv]
dev-dependencies = ["urllib3==2.2.1"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 3 packages in [TIME]
```

Default export includes dev dependencies:

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "typing-extensions-2@4.10.0",
      "name": "typing-extensions",
      "version": "4.10.0",
      "purl": "pkg:pypi/typing-extensions@4.10.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-3@2.2.1",
      "name": "urllib3",
      "version": "2.2.1",
      "purl": "pkg:pypi/urllib3@2.2.1"
    }
  ],
  "dependencies": [
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "typing-extensions-2@4.10.0",
        "urllib3-3@2.2.1"
      ]
    },
    {
      "ref": "typing-extensions-2@4.10.0",
      "dependsOn": []
    },
    {
      "ref": "urllib3-3@2.2.1",
      "dependsOn": []
    }
  ]
}
----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 3 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

Export without dev dependencies:

```console
$ uv export --format cyclonedx1.5 --no-dev
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "typing-extensions-2@4.10.0",
      "name": "typing-extensions",
      "version": "4.10.0",
      "purl": "pkg:pypi/typing-extensions@4.10.0"
    }
  ],
  "dependencies": [
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "typing-extensions-2@4.10.0"
      ]
    },
    {
      "ref": "typing-extensions-2@4.10.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
warning: The `tool.uv.dev-dependencies` field (used in `pyproject.toml`) is deprecated and will be removed in a future release; use `dependency-groups.dev` instead
Resolved 3 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Cyclic dependencies

<!-- Derived from [`export::cyclonedx_export_cyclic_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L7663-L7852) -->

Export requirements in the presence of a cycle (`testtools` and `fixtures` depend on each other).

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "testtools==2.3.0",
    "fixtures==3.0.0",
]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 11 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "argparse-2@1.4.0",
      "name": "argparse",
      "version": "1.4.0",
      "purl": "pkg:pypi/argparse@1.4.0"
    },
    {
      "type": "library",
      "bom-ref": "extras-3@1.0.0",
      "name": "extras",
      "version": "1.0.0",
      "purl": "pkg:pypi/extras@1.0.0"
    },
    {
      "type": "library",
      "bom-ref": "fixtures-4@3.0.0",
      "name": "fixtures",
      "version": "3.0.0",
      "purl": "pkg:pypi/fixtures@3.0.0"
    },
    {
      "type": "library",
      "bom-ref": "linecache2-5@1.0.0",
      "name": "linecache2",
      "version": "1.0.0",
      "purl": "pkg:pypi/linecache2@1.0.0"
    },
    {
      "type": "library",
      "bom-ref": "pbr-6@6.0.0",
      "name": "pbr",
      "version": "6.0.0",
      "purl": "pkg:pypi/pbr@6.0.0"
    },
    {
      "type": "library",
      "bom-ref": "python-mimeparse-7@1.6.0",
      "name": "python-mimeparse",
      "version": "1.6.0",
      "purl": "pkg:pypi/python-mimeparse@1.6.0"
    },
    {
      "type": "library",
      "bom-ref": "six-8@1.16.0",
      "name": "six",
      "version": "1.16.0",
      "purl": "pkg:pypi/six@1.16.0"
    },
    {
      "type": "library",
      "bom-ref": "testtools-9@2.3.0",
      "name": "testtools",
      "version": "2.3.0",
      "purl": "pkg:pypi/testtools@2.3.0"
    },
    {
      "type": "library",
      "bom-ref": "traceback2-10@1.4.0",
      "name": "traceback2",
      "version": "1.4.0",
      "purl": "pkg:pypi/traceback2@1.4.0"
    },
    {
      "type": "library",
      "bom-ref": "unittest2-11@1.1.0",
      "name": "unittest2",
      "version": "1.1.0",
      "purl": "pkg:pypi/unittest2@1.1.0"
    }
  ],
  "dependencies": [
    {
      "ref": "argparse-2@1.4.0",
      "dependsOn": []
    },
    {
      "ref": "extras-3@1.0.0",
      "dependsOn": []
    },
    {
      "ref": "fixtures-4@3.0.0",
      "dependsOn": [
        "pbr-6@6.0.0",
        "six-8@1.16.0",
        "testtools-9@2.3.0"
      ]
    },
    {
      "ref": "linecache2-5@1.0.0",
      "dependsOn": []
    },
    {
      "ref": "pbr-6@6.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "fixtures-4@3.0.0",
        "testtools-9@2.3.0"
      ]
    },
    {
      "ref": "python-mimeparse-7@1.6.0",
      "dependsOn": []
    },
    {
      "ref": "six-8@1.16.0",
      "dependsOn": []
    },
    {
      "ref": "testtools-9@2.3.0",
      "dependsOn": [
        "extras-3@1.0.0",
        "fixtures-4@3.0.0",
        "pbr-6@6.0.0",
        "python-mimeparse-7@1.6.0",
        "six-8@1.16.0",
        "traceback2-10@1.4.0",
        "unittest2-11@1.1.0"
      ]
    },
    {
      "ref": "traceback2-10@1.4.0",
      "dependsOn": [
        "linecache2-5@1.0.0"
      ]
    },
    {
      "ref": "unittest2-11@1.1.0",
      "dependsOn": [
        "argparse-2@1.4.0",
        "six-8@1.16.0",
        "traceback2-10@1.4.0"
      ]
    }
  ]
}
----- stderr -----
Resolved 11 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Prune

<!-- Derived from [`export::cyclonedx_export_prune`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L6871-L7050) -->

Pruning a package from the export.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "jupyter-client==8.6.1"
]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 12 packages in [TIME]
```

Export with pruning `jupyter-core`:

```console
$ uv export --format cyclonedx1.5 --prune jupyter-core
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "cffi-2@1.16.0",
      "name": "cffi",
      "version": "1.16.0",
      "purl": "pkg:pypi/cffi@1.16.0",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "implementation_name == 'pypy'"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "jupyter-client-3@8.6.1",
      "name": "jupyter-client",
      "version": "8.6.1",
      "purl": "pkg:pypi/jupyter-client@8.6.1"
    },
    {
      "type": "library",
      "bom-ref": "pycparser-4@2.21",
      "name": "pycparser",
      "version": "2.21",
      "purl": "pkg:pypi/pycparser@2.21",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "implementation_name == 'pypy'"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "python-dateutil-5@2.9.0.post0",
      "name": "python-dateutil",
      "version": "2.9.0.post0",
      "purl": "pkg:pypi/python-dateutil@2.9.0.post0"
    },
    {
      "type": "library",
      "bom-ref": "pyzmq-6@25.1.2",
      "name": "pyzmq",
      "version": "25.1.2",
      "purl": "pkg:pypi/pyzmq@25.1.2"
    },
    {
      "type": "library",
      "bom-ref": "six-7@1.16.0",
      "name": "six",
      "version": "1.16.0",
      "purl": "pkg:pypi/six@1.16.0"
    },
    {
      "type": "library",
      "bom-ref": "tornado-8@6.4",
      "name": "tornado",
      "version": "6.4",
      "purl": "pkg:pypi/tornado@6.4"
    },
    {
      "type": "library",
      "bom-ref": "traitlets-9@5.14.2",
      "name": "traitlets",
      "version": "5.14.2",
      "purl": "pkg:pypi/traitlets@5.14.2"
    }
  ],
  "dependencies": [
    {
      "ref": "cffi-2@1.16.0",
      "dependsOn": [
        "pycparser-4@2.21"
      ]
    },
    {
      "ref": "jupyter-client-3@8.6.1",
      "dependsOn": [
        "python-dateutil-5@2.9.0.post0",
        "pyzmq-6@25.1.2",
        "tornado-8@6.4",
        "traitlets-9@5.14.2"
      ]
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "jupyter-client-3@8.6.1"
      ]
    },
    {
      "ref": "pycparser-4@2.21",
      "dependsOn": []
    },
    {
      "ref": "python-dateutil-5@2.9.0.post0",
      "dependsOn": [
        "six-7@1.16.0"
      ]
    },
    {
      "ref": "pyzmq-6@25.1.2",
      "dependsOn": [
        "cffi-2@1.16.0"
      ]
    },
    {
      "ref": "six-7@1.16.0",
      "dependsOn": []
    },
    {
      "ref": "tornado-8@6.4",
      "dependsOn": []
    },
    {
      "ref": "traitlets-9@5.14.2",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 12 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Group

<!-- Derived from [`export::cyclonedx_export_group`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L7053-L7270) -->

Exporting with dependency groups.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions==4.10.0"]

[dependency-groups]
foo = ["urllib3==2.2.1 ; sys_platform == 'darwin'"]
bar = ["iniconfig==2.0.0"]
dev = ["sniffio==1.3.1"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Default export includes dev group:

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "sniffio-2@1.3.1",
      "name": "sniffio",
      "version": "1.3.1",
      "purl": "pkg:pypi/sniffio@1.3.1"
    },
    {
      "type": "library",
      "bom-ref": "typing-extensions-3@4.10.0",
      "name": "typing-extensions",
      "version": "4.10.0",
      "purl": "pkg:pypi/typing-extensions@4.10.0"
    }
  ],
  "dependencies": [
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "sniffio-2@1.3.1",
        "typing-extensions-3@4.10.0"
      ]
    },
    {
      "ref": "sniffio-2@1.3.1",
      "dependsOn": []
    },
    {
      "ref": "typing-extensions-3@4.10.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 5 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

Export only specific group:

```console
$ uv export --format cyclonedx1.5 --only-group bar
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "iniconfig-2@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    }
  ],
  "dependencies": [
    {
      "ref": "iniconfig-2@2.0.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 5 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

Export with additional group:

```console
$ uv export --format cyclonedx1.5 --group foo
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "sniffio-2@1.3.1",
      "name": "sniffio",
      "version": "1.3.1",
      "purl": "pkg:pypi/sniffio@1.3.1"
    },
    {
      "type": "library",
      "bom-ref": "typing-extensions-3@4.10.0",
      "name": "typing-extensions",
      "version": "4.10.0",
      "purl": "pkg:pypi/typing-extensions@4.10.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-4@2.2.1",
      "name": "urllib3",
      "version": "2.2.1",
      "purl": "pkg:pypi/urllib3@2.2.1",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "sys_platform == 'darwin'"
        }
      ]
    }
  ],
  "dependencies": [
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "sniffio-2@1.3.1",
        "typing-extensions-3@4.10.0",
        "urllib3-4@2.2.1"
      ]
    },
    {
      "ref": "sniffio-2@1.3.1",
      "dependsOn": []
    },
    {
      "ref": "typing-extensions-3@4.10.0",
      "dependsOn": []
    },
    {
      "ref": "urllib3-4@2.2.1",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 5 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Multiple dependency markers

<!-- Derived from [`export::cyclonedx_export_multiple_dependency_markers`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L6622-L6742) -->

Multiple markers on the same dependency get combined.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = [
    "cryptography==42.0.5 ; python_version > '3.11'",
    "cryptography==42.0.5 ; sys_platform == 'win32'",
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "cffi-2@1.16.0",
      "name": "cffi",
      "version": "1.16.0",
      "purl": "pkg:pypi/cffi@1.16.0",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "(python_full_version >= '3.12' and platform_python_implementation != 'PyPy') or (platform_python_implementation != 'PyPy' and sys_platform == 'win32')"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "cryptography-3@42.0.5",
      "name": "cryptography",
      "version": "42.0.5",
      "purl": "pkg:pypi/cryptography@42.0.5",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "python_full_version >= '3.12' or sys_platform == 'win32'"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "pycparser-4@2.21",
      "name": "pycparser",
      "version": "2.21",
      "purl": "pkg:pypi/pycparser@2.21",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "(python_full_version >= '3.12' and platform_python_implementation != 'PyPy') or (platform_python_implementation != 'PyPy' and sys_platform == 'win32')"
        }
      ]
    }
  ],
  "dependencies": [
    {
      "ref": "cffi-2@1.16.0",
      "dependsOn": [
        "pycparser-4@2.21"
      ]
    },
    {
      "ref": "cryptography-3@42.0.5",
      "dependsOn": [
        "cffi-2@1.16.0"
      ]
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "cryptography-3@42.0.5"
      ]
    },
    {
      "ref": "pycparser-4@2.21",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 4 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Non-project

<!-- Derived from [`export::cyclonedx_export_non_project`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L7273-L7361) -->

Exporting from a workspace without a project section.

```toml
# file: pyproject.toml

[tool.uv.workspace]
members = []

[dependency-groups]
url = ["urllib3==2.2.1"]
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 1 package in [TIME]
```

Default export with no project section:

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ]
  },
  "components": [],
  "dependencies": []
}
----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 1 package in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

Export with group specified:

```console
$ uv export --format cyclonedx1.5 --group url
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ]
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "urllib3-1@2.2.1",
      "name": "urllib3",
      "version": "2.2.1",
      "purl": "pkg:pypi/urllib3@2.2.1"
    }
  ],
  "dependencies": [
    {
      "ref": "urllib3-1@2.2.1",
      "dependsOn": []
    }
  ]
}
----- stderr -----
warning: No `requires-python` value found in the workspace. Defaulting to `>=3.12`.
Resolved 1 package in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Direct URL

<!-- Derived from [`export::cyclonedx_export_direct_url`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L4956-L5029) -->

Exporting a dependency from a direct URL.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["idna @ https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "idna-2@3.6",
      "name": "idna",
      "version": "3.6",
      "purl": "pkg:pypi/idna@3.6?download_url=https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl"
    }
  ],
  "dependencies": [
    {
      "ref": "idna-2@3.6",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "idna-2@3.6"
      ]
    }
  ]
}
----- stderr -----
Resolved 2 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Git dependency

<!-- Derived from [`export::cyclonedx_export_git_dependency`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L5033-L5106) -->

```toml
# mdtest

[environment]
required-features = ["git"]
```

Exporting a dependency from a git repository.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3 @ git+https://github.com/urllib3/urllib3.git@2.2.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "urllib3-2@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0?vcs_url=https://github.com/urllib3/urllib3.git%3Frev%3D2.2.0%2304df048cf4b1c3790c56e26c659db764aad62d6f"
    }
  ],
  "dependencies": [
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "urllib3-2@2.2.0"
      ]
    },
    {
      "ref": "urllib3-2@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 2 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Mixed source types

<!-- Derived from [`export::cyclonedx_export_mixed_source_types`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L5172-L5273) -->

```toml
# mdtest

[environment]
required-features = ["git"]
```

Exporting dependencies from mixed sources (PyPI, Git, direct URL).

```toml
# file: pyproject.toml

[project]
name = "mixed-project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "iniconfig==2.0.0",  # PyPI registry package
    "urllib3 @ git+https://github.com/urllib3/urllib3.git@2.2.0",  # Git package
    "idna @ https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl"  # Direct URL package
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "mixed-project-1@0.1.0",
      "name": "mixed-project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "idna-2@3.6",
      "name": "idna",
      "version": "3.6",
      "purl": "pkg:pypi/idna@3.6?download_url=https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl"
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-3@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-4@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0?vcs_url=https://github.com/urllib3/urllib3.git%3Frev%3D2.2.0%2304df048cf4b1c3790c56e26c659db764aad62d6f"
    }
  ],
  "dependencies": [
    {
      "ref": "idna-2@3.6",
      "dependsOn": []
    },
    {
      "ref": "iniconfig-3@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "mixed-project-1@0.1.0",
      "dependsOn": [
        "idna-2@3.6",
        "iniconfig-3@2.0.0",
        "urllib3-4@2.2.0"
      ]
    },
    {
      "ref": "urllib3-4@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 4 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## workspace-member

Test exporting CycloneDX with workspace members.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3==2.2.0", "child1", "child2"]

[tool.uv.workspace]
members = ["child1", "packages/*"]

[tool.uv.sources]
child1 = { workspace = true }
child2 = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child1/pyproject.toml

[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: packages/child2/pyproject.toml

[project]
name = "child2"
version = "0.2.9"
requires-python = ">=3.11"
dependencies = []

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5 --all-extras
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child1-2@0.1.0",
      "name": "child1",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child1"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "child2-3@0.2.9",
      "name": "child2",
      "version": "0.2.9",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "packages/child2"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-4@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-5@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child1-2@0.1.0",
      "dependsOn": [
        "iniconfig-4@2.0.0"
      ]
    },
    {
      "ref": "child2-3@0.2.9",
      "dependsOn": []
    },
    {
      "ref": "iniconfig-4@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "child1-2@0.1.0",
        "child2-3@0.2.9",
        "urllib3-5@2.2.0"
      ]
    },
    {
      "ref": "urllib3-5@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 5 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## workspace-non-root

<!-- Derived from [`export::cyclonedx_export_workspace_non_root`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L5620-L5714) -->

Test exporting for a non-root workspace package with `--package child`.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3==2.2.0", "child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5 --package child
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "child-1@0.1.0",
      "name": "child",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "iniconfig-2@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child-1@0.1.0",
      "dependsOn": [
        "iniconfig-2@2.0.0"
      ]
    },
    {
      "ref": "iniconfig-2@2.0.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 4 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## workspace-extras

<!-- Derived from [`export::cyclonedx_export_workspace_with_extras`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L5717-L5927) -->

Test with optional dependencies, two commands (one without --all-extras, one with).

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child"]

[project.optional-dependencies]
url = ["urllib3==2.2.0"]
test = ["iniconfig==2.0.0"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions==4.10.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

Export without extras:

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child-2@0.1.0",
      "name": "child",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "typing-extensions-3@4.10.0",
      "name": "typing-extensions",
      "version": "4.10.0",
      "purl": "pkg:pypi/typing-extensions@4.10.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child-2@0.1.0",
      "dependsOn": [
        "typing-extensions-3@4.10.0"
      ]
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "child-2@0.1.0"
      ]
    },
    {
      "ref": "typing-extensions-3@4.10.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 5 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

Export with all extras:

```console
$ uv export --format cyclonedx1.5 --all-extras
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child-2@0.1.0",
      "name": "child",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-3@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "typing-extensions-4@4.10.0",
      "name": "typing-extensions",
      "version": "4.10.0",
      "purl": "pkg:pypi/typing-extensions@4.10.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-5@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child-2@0.1.0",
      "dependsOn": [
        "typing-extensions-4@4.10.0"
      ]
    },
    {
      "ref": "iniconfig-3@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "child-2@0.1.0",
        "iniconfig-3@2.0.0",
        "urllib3-5@2.2.0"
      ]
    },
    {
      "ref": "typing-extensions-4@4.10.0",
      "dependsOn": []
    },
    {
      "ref": "urllib3-5@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 5 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## workspace-frozen

<!-- Derived from [`export::cyclonedx_export_workspace_frozen`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L5930-L6079) -->

Test with --frozen flag, two commands (one failing without frozen, one succeeding with
--all-packages --frozen).

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3==2.2.0", "child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ rm -rf child
success: true
exit_code: 0
----- stdout -----

----- stderr -----
```

```console
$ uv export --format cyclonedx1.5 --all-packages
success: false
exit_code: 1
----- stdout -----

----- stderr -----
  × Failed to build `project @ file://[TEMP_DIR]/`
  ├─▶ Failed to parse entry: `child`
  ╰─▶ `child` references a workspace in `tool.uv.sources` (e.g., `child = { workspace = true }`), but is not a workspace member
```

Export with frozen succeeds:

```console
$ uv export --format cyclonedx1.5 --all-packages --frozen
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-5",
      "name": "project"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child-2@0.1.0",
      "name": "child",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-3@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-4@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    },
    {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child-2@0.1.0",
      "dependsOn": [
        "iniconfig-3@2.0.0"
      ]
    },
    {
      "ref": "iniconfig-3@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "child-2@0.1.0",
        "urllib3-4@2.2.0"
      ]
    },
    {
      "ref": "urllib3-4@2.2.0",
      "dependsOn": []
    },
    {
      "ref": "project-5",
      "dependsOn": [
        "child-2@0.1.0",
        "project-1@0.1.0"
      ]
    }
  ]
}
----- stderr -----
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## workspace-all-packages

<!-- Derived from [`export::cyclonedx_export_workspace_all_packages`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L6082-L6259) -->

Test --all-packages with 2 child workspaces.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3==2.2.0"]

[tool.uv.workspace]
members = ["child1", "child2"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child1/pyproject.toml

[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child2/pyproject.toml

[project]
name = "child2"
version = "0.2.0"
requires-python = ">=3.12"
dependencies = ["sniffio==1.3.1"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5 --all-packages
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-7",
      "name": "project"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child1-2@0.1.0",
      "name": "child1",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child1"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "child2-3@0.2.0",
      "name": "child2",
      "version": "0.2.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child2"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-4@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "sniffio-5@1.3.1",
      "name": "sniffio",
      "version": "1.3.1",
      "purl": "pkg:pypi/sniffio@1.3.1"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-6@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    },
    {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child1-2@0.1.0",
      "dependsOn": [
        "iniconfig-4@2.0.0"
      ]
    },
    {
      "ref": "child2-3@0.2.0",
      "dependsOn": [
        "sniffio-5@1.3.1"
      ]
    },
    {
      "ref": "iniconfig-4@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "urllib3-6@2.2.0"
      ]
    },
    {
      "ref": "sniffio-5@1.3.1",
      "dependsOn": []
    },
    {
      "ref": "urllib3-6@2.2.0",
      "dependsOn": []
    },
    {
      "ref": "project-7",
      "dependsOn": [
        "child1-2@0.1.0",
        "child2-3@0.2.0",
        "project-1@0.1.0"
      ]
    }
  ]
}
----- stderr -----
Resolved 6 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## all-packages-non-workspace-root

<!-- Derived from [`export::cyclonedx_export_all_packages_non_workspace_root_dependency`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L6262-L6346) -->

Test --all-packages for non-workspace project.

```toml
# file: pyproject.toml

[project]
name = "my-project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3==2.2.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 2 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5 --all-packages
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "my-project-3",
      "name": "my-project"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "urllib3-2@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    },
    {
      "type": "library",
      "bom-ref": "my-project-1@0.1.0",
      "name": "my-project",
      "version": "0.1.0"
    }
  ],
  "dependencies": [
    {
      "ref": "my-project-1@0.1.0",
      "dependsOn": [
        "urllib3-2@2.2.0"
      ]
    },
    {
      "ref": "urllib3-2@2.2.0",
      "dependsOn": []
    },
    {
      "ref": "my-project-3",
      "dependsOn": [
        "my-project-1@0.1.0"
      ]
    }
  ]
}
----- stderr -----
Resolved 2 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## workspace-mixed-dependencies

<!-- Derived from [`export::cyclonedx_export_workspace_mixed_dependencies`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L6350-L6519) -->

Test workspace with mixed dependencies (combination of workspace and registry deps, with another
workspace dep not depended on by the root).

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child1", "urllib3==2.2.0"]

[tool.uv.workspace]
members = ["child1", "child2"]

[tool.uv.sources]
child1 = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child1/pyproject.toml

[project]
name = "child1"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["child2", "iniconfig==2.0.0"]

[tool.uv.sources]
child2 = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child2/pyproject.toml

[project]
name = "child2"
version = "0.2.0"
requires-python = ">=3.12"
dependencies = ["sniffio==1.3.1"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 6 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child1-2@0.1.0",
      "name": "child1",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child1"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "child2-3@0.2.0",
      "name": "child2",
      "version": "0.2.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child2"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-4@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "sniffio-5@1.3.1",
      "name": "sniffio",
      "version": "1.3.1",
      "purl": "pkg:pypi/sniffio@1.3.1"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-6@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child1-2@0.1.0",
      "dependsOn": [
        "child2-3@0.2.0",
        "iniconfig-4@2.0.0"
      ]
    },
    {
      "ref": "child2-3@0.2.0",
      "dependsOn": [
        "sniffio-5@1.3.1"
      ]
    },
    {
      "ref": "iniconfig-4@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "child1-2@0.1.0",
        "urllib3-6@2.2.0"
      ]
    },
    {
      "ref": "sniffio-5@1.3.1",
      "dependsOn": []
    },
    {
      "ref": "urllib3-6@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 6 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Project extra with all-extras flag

<!-- Derived from [`export::cyclonedx_export_project_extra_with_optional_flag`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L5356-L5454) -->

Exporting a project with optional dependencies using --all-extras flag.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["typing-extensions==4.10.0"]

[project.optional-dependencies]
url = ["urllib3==2.2.0"]
pytest = ["iniconfig==2.0.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5 --all-extras
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "iniconfig-2@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "typing-extensions-3@4.10.0",
      "name": "typing-extensions",
      "version": "4.10.0",
      "purl": "pkg:pypi/typing-extensions@4.10.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-4@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    }
  ],
  "dependencies": [
    {
      "ref": "iniconfig-2@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "iniconfig-2@2.0.0",
        "typing-extensions-3@4.10.0",
        "urllib3-4@2.2.0"
      ]
    },
    {
      "ref": "typing-extensions-3@4.10.0",
      "dependsOn": []
    },
    {
      "ref": "urllib3-4@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 4 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## No emit package and project

<!-- Derived from [`export::cyclonedx_export_no_emit`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L7364-L7551) -->

Using --no-emit-package and --no-emit-project flags to exclude packages from export.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["urllib3==2.2.0", "child"]

[tool.uv.workspace]
members = ["child"]

[tool.uv.sources]
child = { workspace = true }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 4 packages in [TIME]
```

Exclude urllib3:

```console
$ uv export --format cyclonedx1.5 --no-emit-package urllib3
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child-2@0.1.0",
      "name": "child",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-3@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child-2@0.1.0",
      "dependsOn": [
        "iniconfig-3@2.0.0"
      ]
    },
    {
      "ref": "iniconfig-3@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "child-2@0.1.0"
      ]
    }
  ]
}
----- stderr -----
Resolved 4 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

Exclude project:

```console
$ uv export --format cyclonedx1.5 --no-emit-project
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child-2@0.1.0",
      "name": "child",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-3@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    },
    {
      "type": "library",
      "bom-ref": "urllib3-4@2.2.0",
      "name": "urllib3",
      "version": "2.2.0",
      "purl": "pkg:pypi/urllib3@2.2.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child-2@0.1.0",
      "dependsOn": [
        "iniconfig-3@2.0.0"
      ]
    },
    {
      "ref": "iniconfig-3@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "urllib3-4@2.2.0",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 4 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Relative path dependency

<!-- Derived from [`export::cyclonedx_export_relative_path`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L7556-L7660) -->

Exporting a project with a relative path dependency.

```toml
# file: dependency/pyproject.toml

[project]
name = "dependency"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["iniconfig==2.0.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: project/pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["dependency"]

[tool.uv.sources]
dependency = { path = "../dependency" }

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock --directory project
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 3 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5 --directory project
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "dependency-2@0.1.0",
      "name": "dependency",
      "version": "0.1.0"
    },
    {
      "type": "library",
      "bom-ref": "iniconfig-3@2.0.0",
      "name": "iniconfig",
      "version": "2.0.0",
      "purl": "pkg:pypi/iniconfig@2.0.0"
    }
  ],
  "dependencies": [
    {
      "ref": "dependency-2@0.1.0",
      "dependsOn": [
        "iniconfig-3@2.0.0"
      ]
    },
    {
      "ref": "iniconfig-3@2.0.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "dependency-2@0.1.0"
      ]
    }
  ]
}
----- stderr -----
Using CPython 3.12.[X] interpreter at: [PYTHON-3.12]
Resolved 3 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## All packages with conflicting workspace members

<!-- Derived from [`export::cyclonedx_export_all_packages_conflicting_workspace_members`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L8051-L8189) -->

Exporting with --all-packages when workspace members have conflicting dependencies.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.3.0"]

[tool.uv.workspace]
members = ["child"]

[tool.uv]
conflicts = [
  [
    { package = "project" },
    { package = "child" },
  ],
]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```toml
# file: child/pyproject.toml

[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["sortedcontainers==2.4.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: Declaring conflicts for packages (`package = ...`) is experimental and may change without warning. Pass `--preview-features package-conflicts` to disable this warning.
Resolved 4 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5 --all-packages
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-3",
      "name": "project"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "child-2@0.1.0",
      "name": "child",
      "version": "0.1.0",
      "properties": [
        {
          "name": "uv:workspace:path",
          "value": "child"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  ],
  "dependencies": [
    {
      "ref": "child-2@0.1.0",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": []
    },
    {
      "ref": "project-3",
      "dependsOn": [
        "child-2@0.1.0",
        "project-1@0.1.0"
      ]
    }
  ]
}
----- stderr -----
warning: Declaring conflicts for packages (`package = ...`) is experimental and may change without warning. Pass `--preview-features package-conflicts` to disable this warning.
Resolved 4 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```

## Alternative registry

<!-- Derived from [`export::cyclonedx_export_alternative_registry`](https://github.com/astral-sh/uv/blob/main/crates/uv/tests/it/export.rs#L8192-L8415) -->

Exporting dependencies from an alternative registry.

```toml
# file: pyproject.toml

[project]
name = "project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["torch==2.6.0"]

[build-system]
requires = ["setuptools>=42"]
build-backend = "setuptools.build_meta"

[[tool.uv.index]]
name = "pytorch-cpu"
url = "https://astral-sh.github.io/pytorch-mirror/whl/cpu"
default = true
```

```toml
# mdtest

[environment]
exclude-newer = "2025-01-30T00:00:00Z"
```

```console
$ uv lock
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 12 packages in [TIME]
```

```console
$ uv export --format cyclonedx1.5
success: true
exit_code: 0
----- stdout -----
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "version": 1,
  "serialNumber": "[SERIAL_NUMBER]",
  "metadata": {
    "timestamp": "[TIMESTAMP]",
    "tools": [
      {
        "vendor": "Astral Software Inc.",
        "name": "uv",
        "version": "[VERSION]"
      }
    ],
    "component": {
      "type": "library",
      "bom-ref": "project-1@0.1.0",
      "name": "project",
      "version": "0.1.0"
    }
  },
  "components": [
    {
      "type": "library",
      "bom-ref": "filelock-2@3.13.1",
      "name": "filelock",
      "version": "3.13.1",
      "purl": "pkg:pypi/filelock@3.13.1?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "fsspec-3@2024.6.1",
      "name": "fsspec",
      "version": "2024.6.1",
      "purl": "pkg:pypi/fsspec@2024.6.1?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "jinja2-4@3.1.4",
      "name": "jinja2",
      "version": "3.1.4",
      "purl": "pkg:pypi/jinja2@3.1.4?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "markupsafe-5@3.0.2",
      "name": "markupsafe",
      "version": "3.0.2",
      "purl": "pkg:pypi/markupsafe@3.0.2?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "mpmath-6@1.3.0",
      "name": "mpmath",
      "version": "1.3.0",
      "purl": "pkg:pypi/mpmath@1.3.0?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "networkx-7@3.3",
      "name": "networkx",
      "version": "3.3",
      "purl": "pkg:pypi/networkx@3.3?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "setuptools-8@70.2.0",
      "name": "setuptools",
      "version": "70.2.0",
      "purl": "pkg:pypi/setuptools@70.2.0?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "sympy-9@1.13.1",
      "name": "sympy",
      "version": "1.13.1",
      "purl": "pkg:pypi/sympy@1.13.1?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    },
    {
      "type": "library",
      "bom-ref": "torch-10@2.6.0",
      "name": "torch",
      "version": "2.6.0",
      "purl": "pkg:pypi/torch@2.6.0?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "sys_platform == 'darwin'"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "torch-11@2.6.0+cpu",
      "name": "torch",
      "version": "2.6.0+cpu",
      "purl": "pkg:pypi/torch@2.6.0%2Bcpu?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu",
      "properties": [
        {
          "name": "uv:package:marker",
          "value": "sys_platform != 'darwin'"
        }
      ]
    },
    {
      "type": "library",
      "bom-ref": "typing-extensions-12@4.12.2",
      "name": "typing-extensions",
      "version": "4.12.2",
      "purl": "pkg:pypi/typing-extensions@4.12.2?repository_url=https://astral-sh.github.io/pytorch-mirror/whl/cpu"
    }
  ],
  "dependencies": [
    {
      "ref": "filelock-2@3.13.1",
      "dependsOn": []
    },
    {
      "ref": "fsspec-3@2024.6.1",
      "dependsOn": []
    },
    {
      "ref": "jinja2-4@3.1.4",
      "dependsOn": [
        "markupsafe-5@3.0.2"
      ]
    },
    {
      "ref": "markupsafe-5@3.0.2",
      "dependsOn": []
    },
    {
      "ref": "mpmath-6@1.3.0",
      "dependsOn": []
    },
    {
      "ref": "networkx-7@3.3",
      "dependsOn": []
    },
    {
      "ref": "project-1@0.1.0",
      "dependsOn": [
        "torch-10@2.6.0",
        "torch-11@2.6.0+cpu"
      ]
    },
    {
      "ref": "setuptools-8@70.2.0",
      "dependsOn": []
    },
    {
      "ref": "sympy-9@1.13.1",
      "dependsOn": [
        "mpmath-6@1.3.0"
      ]
    },
    {
      "ref": "torch-10@2.6.0",
      "dependsOn": [
        "filelock-2@3.13.1",
        "fsspec-3@2024.6.1",
        "jinja2-4@3.1.4",
        "networkx-7@3.3",
        "setuptools-8@70.2.0",
        "sympy-9@1.13.1",
        "typing-extensions-12@4.12.2"
      ]
    },
    {
      "ref": "torch-11@2.6.0+cpu",
      "dependsOn": [
        "filelock-2@3.13.1",
        "fsspec-3@2024.6.1",
        "jinja2-4@3.1.4",
        "networkx-7@3.3",
        "setuptools-8@70.2.0",
        "sympy-9@1.13.1",
        "typing-extensions-12@4.12.2"
      ]
    },
    {
      "ref": "typing-extensions-12@4.12.2",
      "dependsOn": []
    }
  ]
}
----- stderr -----
Resolved 12 packages in [TIME]
warning: `uv export --format=cyclonedx1.5` is experimental and may change without warning. Pass `--preview-features sbom-export` to disable this warning.
```
