use std::env;
use std::path::PathBuf;

use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use assert_fs::fixture::PathChild;

use crate::common::{TestContext, copy_dir_ignore, uv_snapshot};

fn workspaces_dir() -> PathBuf {
    env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("scripts")
        .join("workspaces")
}

/// Test basic metadata output for a simple workspace with one member.
#[test]
fn workspace_metadata_simple() {
    let context = TestContext::new("3.12");

    // Initialize a workspace with one member
    context.init().arg("foo").assert().success();

    let workspace = context.temp_dir.child("foo");

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/foo",
      "workspace_members": [
        "foo"
      ],
      "packages": [
        {
          "name": "foo",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/foo"
          },
          "manifest_path": "[TEMP_DIR]/foo/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );
}

/// Test metadata for a root workspace (workspace with a root package).
#[test]
fn workspace_metadata_root_workspace() -> Result<()> {
    let context = TestContext::new("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        workspaces_dir().join("albatross-root-workspace"),
        &workspace,
    )?;

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace",
      "workspace_members": [
        "albatross",
        "bird-feeder",
        "seeds"
      ],
      "packages": [
        {
          "name": "albatross",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace"
          },
          "manifest_path": "[TEMP_DIR]/workspace/pyproject.toml",
          "dependencies": [
            "bird-feeder",
            "iniconfig>=2,<3"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "bird-feeder",
          "version": "1.0.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace/packages/bird-feeder"
          },
          "manifest_path": "[TEMP_DIR]/workspace/packages/bird-feeder/pyproject.toml",
          "dependencies": [
            "iniconfig>=2,<3",
            "seeds"
          ],
          "metadata": {
            "requires_python": ">=3.8"
          }
        },
        {
          "name": "seeds",
          "version": "1.0.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace/packages/seeds"
          },
          "manifest_path": "[TEMP_DIR]/workspace/packages/seeds/pyproject.toml",
          "dependencies": [
            "idna==3.6"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );

    Ok(())
}

/// Test metadata for a virtual workspace (no root package).
#[test]
fn workspace_metadata_virtual_workspace() -> Result<()> {
    let context = TestContext::new("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        workspaces_dir().join("albatross-virtual-workspace"),
        &workspace,
    )?;

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace",
      "workspace_members": [
        "albatross",
        "bird-feeder",
        "seeds"
      ],
      "packages": [
        {
          "name": "albatross",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace/packages/albatross"
          },
          "manifest_path": "[TEMP_DIR]/workspace/packages/albatross/pyproject.toml",
          "dependencies": [
            "bird-feeder",
            "iniconfig>=2,<3"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "bird-feeder",
          "version": "1.0.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace/packages/bird-feeder"
          },
          "manifest_path": "[TEMP_DIR]/workspace/packages/bird-feeder/pyproject.toml",
          "dependencies": [
            "anyio>=4.3.0,<5",
            "seeds"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "seeds",
          "version": "1.0.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace/packages/seeds"
          },
          "manifest_path": "[TEMP_DIR]/workspace/packages/seeds/pyproject.toml",
          "dependencies": [
            "idna==3.6"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );

    Ok(())
}

/// Test metadata when run from a workspace member directory.
#[test]
fn workspace_metadata_from_member() -> Result<()> {
    let context = TestContext::new("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        workspaces_dir().join("albatross-root-workspace"),
        &workspace,
    )?;

    let member_dir = workspace.join("packages").join("bird-feeder");

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&member_dir), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace",
      "workspace_members": [
        "albatross",
        "bird-feeder",
        "seeds"
      ],
      "packages": [
        {
          "name": "albatross",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace"
          },
          "manifest_path": "[TEMP_DIR]/workspace/pyproject.toml",
          "dependencies": [
            "bird-feeder",
            "iniconfig>=2,<3"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "bird-feeder",
          "version": "1.0.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace/packages/bird-feeder"
          },
          "manifest_path": "[TEMP_DIR]/workspace/packages/bird-feeder/pyproject.toml",
          "dependencies": [
            "iniconfig>=2,<3",
            "seeds"
          ],
          "metadata": {
            "requires_python": ">=3.8"
          }
        },
        {
          "name": "seeds",
          "version": "1.0.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace/packages/seeds"
          },
          "manifest_path": "[TEMP_DIR]/workspace/packages/seeds/pyproject.toml",
          "dependencies": [
            "idna==3.6"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );

    Ok(())
}

/// Test metadata for a workspace with multiple packages.
#[test]
fn workspace_metadata_multiple_members() {
    let context = TestContext::new("3.12");

    // Initialize workspace root
    context.init().arg("pkg-a").assert().success();

    let workspace_root = context.temp_dir.child("pkg-a");

    // Add more members
    context
        .init()
        .arg("pkg-b")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("pkg-c")
        .current_dir(&workspace_root)
        .assert()
        .success();

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace_root), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/pkg-a",
      "workspace_members": [
        "pkg-a",
        "pkg-b",
        "pkg-c"
      ],
      "packages": [
        {
          "name": "pkg-a",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/pkg-a"
          },
          "manifest_path": "[TEMP_DIR]/pkg-a/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "pkg-b",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/pkg-a/pkg-b"
          },
          "manifest_path": "[TEMP_DIR]/pkg-a/pkg-b/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "pkg-c",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/pkg-a/pkg-c"
          },
          "manifest_path": "[TEMP_DIR]/pkg-a/pkg-c/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );
}

/// Test metadata for a single project (not a workspace).
#[test]
fn workspace_metadata_single_project() {
    let context = TestContext::new("3.12");

    context.init().arg("my-project").assert().success();

    let project = context.temp_dir.child("my-project");

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&project), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/my-project",
      "workspace_members": [
        "my-project"
      ],
      "packages": [
        {
          "name": "my-project",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/my-project"
          },
          "manifest_path": "[TEMP_DIR]/my-project/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );
}

/// Test metadata with excluded packages.
#[test]
fn workspace_metadata_with_excluded() -> Result<()> {
    let context = TestContext::new("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        workspaces_dir().join("albatross-project-in-excluded"),
        &workspace,
    )?;

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace",
      "workspace_members": [
        "albatross"
      ],
      "packages": [
        {
          "name": "albatross",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace"
          },
          "manifest_path": "[TEMP_DIR]/workspace/pyproject.toml",
          "dependencies": [
            "iniconfig>=2,<3"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );

    Ok(())
}

/// Test metadata error when not in a project.
#[test]
fn workspace_metadata_no_project() {
    let context = TestContext::new("3.12");

    uv_snapshot!(context.filters(), context.workspace_metadata(), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No `pyproject.toml` found in current directory or any parent directory
    "###
    );
}

/// Test metadata with regular workspace dependencies.
#[test]
fn workspace_metadata_with_regular_dependencies() {
    let context = TestContext::new("3.12");

    // Create workspace with multiple members
    context.init().arg("workspace-root").assert().success();

    let workspace_root = context.temp_dir.child("workspace-root");

    // Create a library package
    context
        .init()
        .arg("lib-a")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Create another package that depends on lib-a
    context
        .init()
        .arg("app-b")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Add lib-a as a dependency to app-b
    context
        .add()
        .arg("lib-a")
        .current_dir(workspace_root.child("app-b"))
        .assert()
        .success();

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace_root), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace-root",
      "workspace_members": [
        "app-b",
        "lib-a",
        "workspace-root"
      ],
      "packages": [
        {
          "name": "app-b",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/app-b"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/app-b/pyproject.toml",
          "dependencies": [
            "lib-a"
          ],
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "lib-a",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/lib-a"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/lib-a/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "workspace-root",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ],
      "resolve": {
        "packages": [
          {
            "name": "app-b",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "lib-a",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "workspace-root",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          }
        ]
      }
    }

    ----- stderr -----
    "#
    );
}

/// Test metadata with optional dependencies (extras) on workspace members.
#[test]
fn workspace_metadata_with_extras() {
    let context = TestContext::new("3.12");

    // Create workspace with multiple members
    context.init().arg("workspace-root").assert().success();

    let workspace_root = context.temp_dir.child("workspace-root");

    // Create library packages
    context
        .init()
        .arg("lib-a")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("lib-b")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Create app with optional dependencies on lib-a and lib-b
    context
        .init()
        .arg("app")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Add optional dependencies
    let app_dir = workspace_root.child("app");
    context
        .add()
        .arg("lib-a")
        .arg("--optional")
        .arg("extra-a")
        .current_dir(&app_dir)
        .assert()
        .success();

    context
        .add()
        .arg("lib-b")
        .arg("--optional")
        .arg("extra-b")
        .current_dir(&app_dir)
        .assert()
        .success();

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace_root), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace-root",
      "workspace_members": [
        "app",
        "lib-a",
        "lib-b",
        "workspace-root"
      ],
      "packages": [
        {
          "name": "app",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/app"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/app/pyproject.toml",
          "optional_dependencies": {
            "extra-a": [
              "lib-a"
            ],
            "extra-b": [
              "lib-b"
            ]
          },
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "lib-a",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/lib-a"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/lib-a/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "lib-b",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/lib-b"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/lib-b/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "workspace-root",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ],
      "resolve": {
        "packages": [
          {
            "name": "app",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "lib-a",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "lib-b",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "workspace-root",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          }
        ]
      }
    }

    ----- stderr -----
    "#
    );
}

/// Test metadata with dependency groups containing workspace members.
#[test]
fn workspace_metadata_with_dependency_groups() {
    let context = TestContext::new("3.12");

    // Create workspace with multiple members
    context.init().arg("workspace-root").assert().success();

    let workspace_root = context.temp_dir.child("workspace-root");

    // Create library packages
    context
        .init()
        .arg("test-utils")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("dev-tools")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Create app with dependency groups
    context
        .init()
        .arg("app")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Add dependency groups
    let app_dir = workspace_root.child("app");
    context
        .add()
        .arg("test-utils")
        .arg("--group")
        .arg("test")
        .current_dir(&app_dir)
        .assert()
        .success();

    context
        .add()
        .arg("dev-tools")
        .arg("--group")
        .arg("dev")
        .current_dir(&app_dir)
        .assert()
        .success();

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace_root), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace-root",
      "workspace_members": [
        "app",
        "dev-tools",
        "test-utils",
        "workspace-root"
      ],
      "packages": [
        {
          "name": "app",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/app"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/app/pyproject.toml",
          "dependency_groups": {
            "dev": [
              "dev-tools"
            ],
            "test": [
              "test-utils"
            ]
          },
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "dev-tools",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/dev-tools"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/dev-tools/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "test-utils",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/test-utils"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/test-utils/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "workspace-root",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ],
      "resolve": {
        "packages": [
          {
            "name": "app",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "dev-tools",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "test-utils",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "workspace-root",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          }
        ]
      }
    }

    ----- stderr -----
    "#
    );
}

/// Test metadata with workspace members that have no dependencies on each other.
#[test]
fn workspace_metadata_no_workspace_dependencies() {
    let context = TestContext::new("3.12");

    // Create workspace with multiple members that don't depend on each other
    context.init().arg("workspace-root").assert().success();

    let workspace_root = context.temp_dir.child("workspace-root");

    // Create independent packages
    context
        .init()
        .arg("package-a")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("package-b")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("package-c")
        .current_dir(&workspace_root)
        .assert()
        .success();

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace_root), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace-root",
      "workspace_members": [
        "package-a",
        "package-b",
        "package-c",
        "workspace-root"
      ],
      "packages": [
        {
          "name": "package-a",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/package-a"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/package-a/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "package-b",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/package-b"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/package-b/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "package-c",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/package-c"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/package-c/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "workspace-root",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ]
    }

    ----- stderr -----
    "#
    );
}

/// Test metadata with mixed workspace dependencies (regular, extras, and groups).
#[test]
fn workspace_metadata_mixed_dependencies() {
    let context = TestContext::new("3.12");

    // Create workspace with multiple members
    context.init().arg("workspace-root").assert().success();

    let workspace_root = context.temp_dir.child("workspace-root");

    // Create library packages
    context
        .init()
        .arg("core")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("utils")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("testing")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Create app with all types of dependencies
    context
        .init()
        .arg("app")
        .current_dir(&workspace_root)
        .assert()
        .success();

    // Add regular dependency, optional dependency, and dependency group
    let app_dir = workspace_root.child("app");

    // Add regular dependency
    context
        .add()
        .arg("core")
        .current_dir(&app_dir)
        .assert()
        .success();

    // Add optional dependency
    context
        .add()
        .arg("utils")
        .arg("--optional")
        .arg("utils")
        .current_dir(&app_dir)
        .assert()
        .success();

    // Add dependency group
    context
        .add()
        .arg("testing")
        .arg("--group")
        .arg("test")
        .current_dir(&app_dir)
        .assert()
        .success();

    uv_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace_root), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "version": 1,
      "requires_python": ">=3.12",
      "workspace_root": "[TEMP_DIR]/workspace-root",
      "workspace_members": [
        "app",
        "core",
        "testing",
        "utils",
        "workspace-root"
      ],
      "packages": [
        {
          "name": "app",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/app"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/app/pyproject.toml",
          "dependencies": [
            "core"
          ],
          "optional_dependencies": {
            "utils": [
              "utils"
            ]
          },
          "dependency_groups": {
            "test": [
              "testing"
            ]
          },
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "core",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/core"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/core/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "testing",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/testing"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/testing/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "utils",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root/utils"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/utils/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        },
        {
          "name": "workspace-root",
          "version": "0.1.0",
          "source": {
            "type": "directory",
            "path": "[TEMP_DIR]/workspace-root"
          },
          "manifest_path": "[TEMP_DIR]/workspace-root/pyproject.toml",
          "metadata": {
            "requires_python": ">=3.12"
          }
        }
      ],
      "resolve": {
        "packages": [
          {
            "name": "app",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "core",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "testing",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "utils",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          },
          {
            "name": "workspace-root",
            "version": "0.1.0",
            "source": {
              "type": "registry"
            }
          }
        ]
      }
    }

    ----- stderr -----
    "#
    );
}
