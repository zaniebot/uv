use anyhow::Result;
use assert_fs::prelude::*;
use indoc::indoc;
use insta::assert_snapshot;

use crate::common::{TestContext, uv_snapshot};

#[test]
/// Check warning message for <https://github.com/astral-sh/uv/issues/6998>
/// if no `build-system` section is defined.
fn sync_scripts_without_build_system() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [project.scripts]
        entry = "foo:custom_entry"
        "#,
    )?;

    let test_script = context.temp_dir.child("src/__init__.py");
    test_script.write_str(
        r#"
        def custom_entry():
            print!("Hello")
       "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: Skipping installation of entry points (`project.scripts`) because this project is not packaged; to install entry points, set `tool.uv.package = true` or define a `build-system`
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    Ok(())
}

#[test]
/// Check warning message for <https://github.com/astral-sh/uv/issues/6998>
/// if the project is marked as `package = false`.
fn sync_scripts_project_not_packaged() -> Result<()> {
    let context = TestContext::new("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "foo"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = []

        [project.scripts]
        entry = "foo:custom_entry"

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.uv]
        package = false
        "#,
    )?;

    let test_script = context.temp_dir.child("src/__init__.py");
    test_script.write_str(
        r#"
        def custom_entry():
            print!("Hello")
       "#,
    )?;

    uv_snapshot!(context.filters(), context.sync(), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: Skipping installation of entry points (`project.scripts`) because this project is not packaged; to install entry points, set `tool.uv.package = true` or define a `build-system`
    Resolved 1 package in [TIME]
    Audited in [TIME]
    ");

    Ok(())
}

#[test]
fn sync_script() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.9", "3.12"]);

    let script = context.temp_dir.child("script.py");
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.11"
        # dependencies = [
        #   "anyio",
        # ]
        # ///

        import anyio
       "#
    })?;

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Creating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // If a lockfile didn't exist already, `uv sync --script` shouldn't create one.
    assert!(!context.temp_dir.child("uv.lock").exists());

    // Modify the script's dependencies.
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.11"
        # dependencies = [
        #   "anyio",
        #   "iniconfig",
        # ]
        # ///

        import anyio
       "#
    })?;

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 4 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    // Remove a dependency.
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.11"
        # dependencies = [
        #   "anyio",
        # ]
        # ///

        import anyio
       "#
    })?;

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 3 packages in [TIME]
    Uninstalled 1 package in [TIME]
     - iniconfig==2.0.0
    ");

    // Modify the `requires-python`.
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.8, <3.11"
        # dependencies = [
        #   "anyio",
        # ]
        # ///

        import anyio
       "#
    })?;

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Updating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 5 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 5 packages in [TIME]
     + anyio==4.3.0
     + exceptiongroup==1.2.0
     + idna==3.6
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    // `--locked` and `--frozen` should fail with helpful error messages.
    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").arg("--locked"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    error: `uv sync --locked` requires a script lockfile; run `uv lock --script script.py` to lock the script
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").arg("--frozen"), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    error: `uv sync --frozen` requires a script lockfile; run `uv lock --script script.py` to lock the script
    ");

    Ok(())
}

#[test]
fn sync_locked_script() -> Result<()> {
    let context = TestContext::new_with_versions(&["3.9", "3.12"]);

    let script = context.temp_dir.child("script.py");
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.11"
        # dependencies = [
        #   "anyio",
        # ]
        # ///

        import anyio
       "#
    })?;

    // Lock the script.
    uv_snapshot!(context.filters(), context.lock().arg("--script").arg("script.py"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "###);

    let lock = context.read("script.py.lock");

    insta::with_settings!({
        filters => context.filters(),
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.11"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [manifest]
        requirements = [{ name = "anyio" }]

        [[package]]
        name = "anyio"
        version = "4.3.0"
        source = { registry = "https://pypi.org/simple" }
        dependencies = [
            { name = "idna" },
            { name = "sniffio" },
        ]
        sdist = { url = "https://files.pythonhosted.org/packages/db/4d/3970183622f0330d3c23d9b8a5f52e365e50381fd484d08e3285104333d3/anyio-4.3.0.tar.gz", hash = "sha256:f75253795a87df48568485fd18cdd2a3fa5c4f7c5be8e5e36637733fce06fed6", size = 159642, upload-time = "2024-02-19T08:36:28.641Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/14/fd/2f20c40b45e4fb4324834aea24bd4afdf1143390242c0b33774da0e2e34f/anyio-4.3.0-py3-none-any.whl", hash = "sha256:048e05d0f6caeed70d731f3db756d35dcc1f35747c8c403364a8332c630441b8", size = 85584, upload-time = "2024-02-19T08:36:26.842Z" },
        ]

        [[package]]
        name = "idna"
        version = "3.6"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/bf/3f/ea4b9117521a1e9c50344b909be7886dd00a519552724809bb1f486986c2/idna-3.6.tar.gz", hash = "sha256:9ecdbbd083b06798ae1e86adcbfe8ab1479cf864e4ee30fe4e46a003d12491ca", size = 175426, upload-time = "2023-11-25T15:40:54.902Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl", hash = "sha256:c05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f", size = 61567, upload-time = "2023-11-25T15:40:52.604Z" },
        ]

        [[package]]
        name = "sniffio"
        version = "1.3.1"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/a2/87/a6771e1546d97e7e041b6ae58d80074f81b7d5121207425c964ddf5cfdbd/sniffio-1.3.1.tar.gz", hash = "sha256:f4324edc670a0f49750a81b895f35c3adb843cca46f0530f79fc1babb23789dc", size = 20372, upload-time = "2024-02-25T23:20:04.057Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/e9/44/75a9c9421471a6c4805dbf2356f7c181a29c1879239abab1ea2cc8f38b40/sniffio-1.3.1-py3-none-any.whl", hash = "sha256:2f6da418d1f1e0fddd844478f41680e794e6051915791a034ff65e5f100525a2", size = 10235, upload-time = "2024-02-25T23:20:01.196Z" },
        ]
        "#
        );
    });

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Creating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 3 packages in [TIME]
    Prepared 3 packages in [TIME]
    Installed 3 packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + sniffio==1.3.1
    ");

    // Modify the script's dependencies.
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.11"
        # dependencies = [
        #   "anyio",
        #   "iniconfig",
        # ]
        # ///

        import anyio
       "#
    })?;

    // Re-run with `--locked`.
    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").arg("--locked"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 4 packages in [TIME]
    The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 4 packages in [TIME]
    Prepared 1 package in [TIME]
    Installed 1 package in [TIME]
     + iniconfig==2.0.0
    ");

    let lock = context.read("script.py.lock");

    insta::with_settings!({
        filters => context.filters(),
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.11"

        [options]
        exclude-newer = "2024-03-25T00:00:00Z"

        [manifest]
        requirements = [
            { name = "anyio" },
            { name = "iniconfig" },
        ]

        [[package]]
        name = "anyio"
        version = "4.3.0"
        source = { registry = "https://pypi.org/simple" }
        dependencies = [
            { name = "idna" },
            { name = "sniffio" },
        ]
        sdist = { url = "https://files.pythonhosted.org/packages/db/4d/3970183622f0330d3c23d9b8a5f52e365e50381fd484d08e3285104333d3/anyio-4.3.0.tar.gz", hash = "sha256:f75253795a87df48568485fd18cdd2a3fa5c4f7c5be8e5e36637733fce06fed6", size = 159642, upload-time = "2024-02-19T08:36:28.641Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/14/fd/2f20c40b45e4fb4324834aea24bd4afdf1143390242c0b33774da0e2e34f/anyio-4.3.0-py3-none-any.whl", hash = "sha256:048e05d0f6caeed70d731f3db756d35dcc1f35747c8c403364a8332c630441b8", size = 85584, upload-time = "2024-02-19T08:36:26.842Z" },
        ]

        [[package]]
        name = "idna"
        version = "3.6"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/bf/3f/ea4b9117521a1e9c50344b909be7886dd00a519552724809bb1f486986c2/idna-3.6.tar.gz", hash = "sha256:9ecdbbd083b06798ae1e86adcbfe8ab1479cf864e4ee30fe4e46a003d12491ca", size = 175426, upload-time = "2023-11-25T15:40:54.902Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/c2/e7/a82b05cf63a603df6e68d59ae6a68bf5064484a0718ea5033660af4b54a9/idna-3.6-py3-none-any.whl", hash = "sha256:c05567e9c24a6b9faaa835c4821bad0590fbb9d5779e7caa6e1cc4978e7eb24f", size = 61567, upload-time = "2023-11-25T15:40:52.604Z" },
        ]

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3", size = 4646, upload-time = "2023-01-07T11:08:11.254Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beeac6173dc9db836a5b46/iniconfig-2.0.0-py3-none-any.whl", hash = "sha256:b6a85871a79d2e3b22d2d1b94ac2824226a63c6b741c88f7ae975f18b6778374", size = 5892, upload-time = "2023-01-07T11:08:09.864Z" },
        ]

        [[package]]
        name = "sniffio"
        version = "1.3.1"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/a2/87/a6771e1546d97e7e041b6ae58d80074f81b7d5121207425c964ddf5cfdbd/sniffio-1.3.1.tar.gz", hash = "sha256:f4324edc670a0f49750a81b895f35c3adb843cca46f0530f79fc1babb23789dc", size = 20372, upload-time = "2024-02-25T23:20:04.057Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/e9/44/75a9c9421471a6c4805dbf2356f7c181a29c1879239abab1ea2cc8f38b40/sniffio-1.3.1-py3-none-any.whl", hash = "sha256:2f6da418d1f1e0fddd844478f41680e794e6051915791a034ff65e5f100525a2", size = 10235, upload-time = "2024-02-25T23:20:01.196Z" },
        ]
        "#
        );
    });

    // Modify the `requires-python`.
    script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.8, <3.11"
        # dependencies = [
        #   "anyio",
        #   "iniconfig",
        # ]
        # ///

        import anyio
       "#
    })?;

    // Re-run with `--locked`.
    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py").arg("--locked"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Updating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    warning: Resolving despite existing lockfile due to fork markers being disjoint with `requires-python`: `python_full_version >= '3.11'` vs `python_full_version >= '3.8' and python_full_version < '3.11'`
    Resolved 6 packages in [TIME]
    The lockfile at `uv.lock` needs to be updated, but `--locked` was provided. To update the lockfile, run `uv lock`.
    ");

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    warning: Resolving despite existing lockfile due to fork markers being disjoint with `requires-python`: `python_full_version >= '3.11'` vs `python_full_version >= '3.8' and python_full_version < '3.11'`
    Resolved 6 packages in [TIME]
    Prepared 2 packages in [TIME]
    Installed 6 packages in [TIME]
     + anyio==4.3.0
     + exceptiongroup==1.2.0
     + idna==3.6
     + iniconfig==2.0.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    Ok(())
}

#[test]
fn sync_script_with_compatible_build_constraints() -> Result<()> {
    let context = TestContext::new("3.9");

    let test_script = context.temp_dir.child("script.py");

    // Compatible build constraints.
    test_script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.9"
        # dependencies = [
        #   "anyio>=3",
        #   "requests==1.2"
        # ]
        #
        # [tool.uv]
        # build-constraint-dependencies = ["setuptools>=40"]
        # ///

        import anyio
       "#
    })?;

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Creating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
    Resolved 6 packages in [TIME]
    Prepared 6 packages in [TIME]
    Installed 6 packages in [TIME]
     + anyio==4.3.0
     + exceptiongroup==1.2.0
     + idna==3.6
     + requests==1.2.0
     + sniffio==1.3.1
     + typing-extensions==4.10.0
    ");

    Ok(())
}

#[test]
fn sync_script_with_incompatible_build_constraints() -> Result<()> {
    let context = TestContext::new("3.9");

    let test_script = context.temp_dir.child("script.py");

    // Incompatible build constraints.
    test_script.write_str(indoc! { r#"
        # /// script
        # requires-python = ">=3.9"
        # dependencies = [
        #   "anyio>=3",
        #   "requests==1.2"
        # ]
        #
        # [tool.uv]
        # build-constraint-dependencies = ["setuptools==1"]
        # ///

        import anyio
       "#
    })?;

    uv_snapshot!(context.filters(), context.sync().arg("--script").arg("script.py"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Creating script environment at: [CACHE_DIR]/environments-v2/script-[HASH]
      × Failed to download and build `requests==1.2.0`
      ├─▶ Failed to resolve requirements from `setup.py` build
      ├─▶ No solution found when resolving: `setuptools>=40.8.0`
      ╰─▶ Because you require setuptools>=40.8.0 and setuptools==1, we can conclude that your requirements are unsatisfiable.
    ");

    Ok(())
}
