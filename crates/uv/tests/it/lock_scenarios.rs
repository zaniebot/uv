//! DO NOT EDIT
//!
//! Generated with `uv run scripts/scenarios/generate.py`
//! Scenarios from <test/scenarios>
//!
#![cfg(feature = "test-python")]
#![allow(clippy::needless_raw_string_hashes)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::doc_lazy_continuation)]

use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use assert_fs::prelude::*;
use insta::assert_snapshot;

use uv_static::EnvVars;

use uv_test::packse::PackseServer;
use uv_test::uv_snapshot;

/// There are two packages, `a` and `b`. We select `a` with `a==2.0.0` first, and then `b`, but `a==2.0.0` conflicts with all new versions of `b`, so we backtrack through versions of `b`.
///
/// We need to detect this conflict and prioritize `b` over `a` instead of backtracking down to the too old version of `b==1.0.0` that doesn't depend on `a` anymore.
///
/// ```text
/// wrong-backtracking-basic
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a
/// │   │   ├── satisfied by a-1.0.0
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires b
/// │       ├── satisfied by b-1.0.0
/// │       ├── satisfied by b-2.0.0
/// │       ├── satisfied by b-2.0.1
/// │       ├── satisfied by b-2.0.2
/// │       ├── satisfied by b-2.0.3
/// │       ├── satisfied by b-2.0.4
/// │       ├── satisfied by b-2.0.5
/// │       ├── satisfied by b-2.0.6
/// │       ├── satisfied by b-2.0.7
/// │       ├── satisfied by b-2.0.8
/// │       └── satisfied by b-2.0.9
/// ├── a
/// │   ├── a-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   ├── b-1.0.0
/// │   │   └── requires too-old
/// │   │       └── satisfied by too-old-1.0.0
/// │   ├── b-2.0.0
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.1
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.2
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.3
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.4
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.5
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.6
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.7
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-2.0.8
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   └── b-2.0.9
/// │       └── requires a==1.0.0
/// │           └── satisfied by a-1.0.0
/// └── too-old
///     └── too-old-1.0.0
/// ```
#[test]
fn wrong_backtracking_basic() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("backtracking/wrong-backtracking-basic.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a''',
          '''b''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:10eb37b5137fa0770faf91f55610eb2555ff2b6d6b0fe56c53ea9374173ea099" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:e96075bbdd8879e0f5be27c91dd01388085eab3b08e586138c34a5ee2c45091e" },
        ]

        [[package]]
        name = "b"
        version = "2.0.9"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "a" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.0.9.tar.gz", hash = "sha256:0c1a519f428133aa80e506cfc69086d9b5a503b41a8fd413a286f00b4e82b11f" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.0.9-py3-none-any.whl", hash = "sha256:cf0a6ec142cd3b3c5d0bd0366c2eea2d8a88e524fcbacb01648cc7a1751d26b7" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
            { name = "b" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a" },
            { name = "b" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// There are three packages, `a`, `b` and `b-inner`. Unlike wrong-backtracking-basic, `b` depends on `b-inner` and `a` and `b-inner` conflict, to add a layer of indirection.
///
/// We select `a` with `a==2.0.0` first, then `b`, and then `b-inner`, but `a==2.0.0` conflicts with all new versions of `b-inner`, so we backtrack through versions of `b-inner`.
///
/// We need to detect this conflict and prioritize `b` and `b-inner` over `a` instead of backtracking down to the too old version of `b-inner==1.0.0` that doesn't depend on `a` anymore.
///
/// ```text
/// wrong-backtracking-indirect
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a
/// │   │   ├── satisfied by a-1.0.0
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires b
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires b-inner
/// │           ├── satisfied by b-inner-1.0.0
/// │           ├── satisfied by b-inner-2.0.0
/// │           ├── satisfied by b-inner-2.0.1
/// │           ├── satisfied by b-inner-2.0.2
/// │           ├── satisfied by b-inner-2.0.3
/// │           ├── satisfied by b-inner-2.0.4
/// │           ├── satisfied by b-inner-2.0.5
/// │           ├── satisfied by b-inner-2.0.6
/// │           ├── satisfied by b-inner-2.0.7
/// │           ├── satisfied by b-inner-2.0.8
/// │           └── satisfied by b-inner-2.0.9
/// ├── b-inner
/// │   ├── b-inner-1.0.0
/// │   │   └── requires too-old
/// │   │       └── satisfied by too-old-1.0.0
/// │   ├── b-inner-2.0.0
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.1
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.2
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.3
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.4
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.5
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.6
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.7
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   ├── b-inner-2.0.8
/// │   │   └── requires a==1.0.0
/// │   │       └── satisfied by a-1.0.0
/// │   └── b-inner-2.0.9
/// │       └── requires a==1.0.0
/// │           └── satisfied by a-1.0.0
/// └── too-old
///     └── too-old-1.0.0
/// ```
#[test]
fn wrong_backtracking_indirect() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("backtracking/wrong-backtracking-indirect.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a''',
          '''b''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "b-inner" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:c78348f855cb755e9003ee46a36f0d48677845bb9a20c9092fd50b8ae5923455" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:4a1a47b145aa8a7b213acccfa5221fd2c0c5af88dab6f9ee4103e78897541f8c" },
        ]

        [[package]]
        name = "b-inner"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "too-old" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b_inner-1.0.0.tar.gz", hash = "sha256:2113fba5b7afa06b5350bbbd6e1a87701f040b5c4c2fd4ffb01f334da053b213" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b_inner-1.0.0-py3-none-any.whl", hash = "sha256:cc9eda0f3ba1d9fe8ef9411d26d6dc9b253b916bc5f2a888ab6fdc87d3d6a39b" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
            { name = "b" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a" },
            { name = "b" },
        ]

        [[package]]
        name = "too-old"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/too_old-1.0.0.tar.gz", hash = "sha256:3b98fc1f4649e5a8e1f3d0562760138a7fc913315c4e187ac6d975b3ad9b8790" }
        wheels = [
            { url = "http://[LOCALHOST]/files/too_old-1.0.0-py3-none-any.whl", hash = "sha256:863ad632e67f7690b1f7c7905218c9d22d0c0c213d96e90f16fe4c0693a5bc45" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This test ensures that multiple non-conflicting but also
/// non-overlapping dependency specifications with the same package name
/// are allowed and supported.
///
/// At time of writing, this provokes a fork in the resolver, but it
/// arguably shouldn't since the requirements themselves do not conflict
/// with one another. However, this does impact resolution. Namely, it
/// leaves the `a>=1` fork free to choose `a==2.0.0` since it behaves as if
/// the `a<2` constraint doesn't exist.
///
/// ```text
/// fork-allows-non-conflicting-non-overlapping-dependencies
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=1; sys_platform == "linux"
/// │   │   ├── satisfied by a-1.0.0
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// └── a
///     ├── a-1.0.0
///     └── a-2.0.0
/// ```
#[test]
fn fork_allows_non_conflicting_non_overlapping_dependencies() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/allows-non-conflicting-non-overlapping-dependencies.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=1 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:10eb37b5137fa0770faf91f55610eb2555ff2b6d6b0fe56c53ea9374173ea099" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:e96075bbdd8879e0f5be27c91dd01388085eab3b08e586138c34a5ee2c45091e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", marker = "sys_platform == 'darwin' or sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=1" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This test ensures that multiple non-conflicting dependency
/// specifications with the same package name are allowed and supported.
///
/// This test exists because the universal resolver forks itself based on
/// duplicate dependency specifications by looking at package name. So at
/// first glance, a case like this could perhaps cause an errant fork.
/// While it's difficult to test for "does not create a fork" (at time of
/// writing, the implementation does not fork), we can at least check that
/// this case is handled correctly without issue. Namely, forking should
/// only occur when there are duplicate dependency specifications with
/// disjoint marker expressions.
///
/// ```text
/// fork-allows-non-conflicting-repeated-dependencies
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=1
/// │   │   ├── satisfied by a-1.0.0
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2
/// │       └── satisfied by a-1.0.0
/// └── a
///     ├── a-1.0.0
///     └── a-2.0.0
/// ```
#[test]
fn fork_allows_non_conflicting_repeated_dependencies() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/allows-non-conflicting-repeated-dependencies.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=1''',
          '''a<2''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:10eb37b5137fa0770faf91f55610eb2555ff2b6d6b0fe56c53ea9374173ea099" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:e96075bbdd8879e0f5be27c91dd01388085eab3b08e586138c34a5ee2c45091e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", specifier = "<2" },
            { name = "a", specifier = ">=1" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// An extremely basic test of universal resolution. In this case, the resolution
/// should contain two distinct versions of `a` depending on `sys_platform`.
///
/// ```text
/// fork-basic
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// └── a
///     ├── a-1.0.0
///     └── a-2.0.0
/// ```
#[test]
fn fork_basic() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/basic.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:10eb37b5137fa0770faf91f55610eb2555ff2b6d6b0fe56c53ea9374173ea099" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:e96075bbdd8879e0f5be27c91dd01388085eab3b08e586138c34a5ee2c45091e" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// We have a conflict after forking. This scenario exists to test the error message.
///
/// ```text
/// conflict-in-fork
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "os1"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "os2"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   ├── requires b
/// │   │   │   └── satisfied by b-1.0.0
/// │   │   └── requires c
/// │   │       └── satisfied by c-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires d==1
/// │           └── satisfied by d-1.0.0
/// ├── c
/// │   └── c-1.0.0
/// │       └── requires d==2
/// │           └── satisfied by d-2.0.0
/// └── d
///     ├── d-1.0.0
///     └── d-2.0.0
/// ```
#[test]
fn conflict_in_fork() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/conflict-in-fork.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'os1'''',
          '''a<2 ; sys_platform == 'os2'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving dependencies for split (markers: sys_platform == 'os2'):
      ╰─▶ Because only b==1.0.0 is available and b==1.0.0 depends on d==1, we can conclude that all versions of b depend on d==1.
          And because c==1.0.0 depends on d==2 and only c==1.0.0 is available, we can conclude that all versions of b and all versions of c are incompatible.
          And because a==1.0.0 depends on b and c, we can conclude that a==1.0.0 cannot be used.
          And because only the following versions of a{sys_platform == 'os2'} are available:
              a{sys_platform == 'os2'}==1.0.0
              a{sys_platform == 'os2'}>=2
          and your project depends on a{sys_platform == 'os2'}<2, we can conclude that your project's requirements are unsatisfiable.
    "
    );

    Ok(())
}

/// This test ensures that conflicting dependency specifications lead to an
/// unsatisfiable result.
///
/// In particular, this is a case that should not fork even though there
/// are conflicting requirements because their marker expressions are
/// overlapping. (Well, there aren't any marker expressions here, which
/// means they are both unconditional.)
///
/// ```text
/// fork-conflict-unsatisfiable
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2
/// │   │   ├── satisfied by a-2.0.0
/// │   │   └── satisfied by a-3.0.0
/// │   └── requires a<2
/// │       └── satisfied by a-1.0.0
/// └── a
///     ├── a-1.0.0
///     ├── a-2.0.0
///     └── a-3.0.0
/// ```
#[test]
fn fork_conflict_unsatisfiable() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/conflict-unsatisfiable.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2''',
          '''a<2''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving dependencies:
      ╰─▶ Because your project depends on a>=2 and a<2, we can conclude that your project's requirements are unsatisfiable.
    "
    );

    Ok(())
}

/// This tests that sibling dependencies of a package that provokes a
/// fork are correctly filtered out of forks where they are otherwise
/// impossible.
///
/// In this case, a previous version of the universal resolver would
/// include both `b` and `c` in *both* of the forks produced by the
/// conflicting dependency specifications on `a`. This in turn led to
/// transitive dependency specifications on both `d==1.0.0` and `d==2.0.0`.
/// Since the universal resolver only forks based on local conditions, this
/// led to a failed resolution.
///
/// The correct thing to do here is to ensure that `b` is only part of the
/// `a==4.4.0` fork and `c` is only par of the `a==4.3.0` fork.
///
/// ```text
/// fork-filter-sibling-dependencies
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==4.4.0; sys_platform == "linux"
/// │   │   └── satisfied by a-4.4.0
/// │   ├── requires a==4.3.0; sys_platform == "darwin"
/// │   │   └── satisfied by a-4.3.0
/// │   ├── requires b==1.0.0; sys_platform == "linux"
/// │   │   └── satisfied by b-1.0.0
/// │   └── requires c==1.0.0; sys_platform == "darwin"
/// │       └── satisfied by c-1.0.0
/// ├── a
/// │   ├── a-4.3.0
/// │   └── a-4.4.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires d==1.0.0
/// │           └── satisfied by d-1.0.0
/// ├── c
/// │   └── c-1.0.0
/// │       └── requires d==2.0.0
/// │           └── satisfied by d-2.0.0
/// └── d
///     ├── d-1.0.0
///     └── d-2.0.0
/// ```
#[test]
fn fork_filter_sibling_dependencies() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/filter-sibling-dependencies.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==4.4.0 ; sys_platform == 'linux'''',
          '''a==4.3.0 ; sys_platform == 'darwin'''',
          '''b==1.0.0 ; sys_platform == 'linux'''',
          '''c==1.0.0 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 7 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'linux'",
            "sys_platform == 'darwin'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "4.3.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-4.3.0.tar.gz", hash = "sha256:fee0fc6bcee55d9ae0b70fb7b8df0d2ef97bc78caac7f9f4eade28d1e85886f7" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-4.3.0-py3-none-any.whl", hash = "sha256:4d447edb05d0780bd0985ac7c9699adedc47d6d9052e7748138171ce01cab4bc" },
        ]

        [[package]]
        name = "a"
        version = "4.4.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-4.4.0.tar.gz", hash = "sha256:1806430d31c2f245ab059f21aeb037a3edd949bb2398e13a651cf3b66a72e418" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-4.4.0-py3-none-any.whl", hash = "sha256:734dfa67dfc6048065c5e1f458ef5a80c4c6d3a6d8c03b86191f6c93739aec60" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "d", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:2efaebd818ac206dcf6b00ba6a1b7cdf647e85a9a919ab816d4e96938d02dfbf" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:77fa0217262d13e1b213fb431339699f6bb815fc35d71bb6673e1922426a237d" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "d", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/c-1.0.0.tar.gz", hash = "sha256:3ca9b232b061a08def356df7074c9c9c65ffde39d6f8da16161c85ca7fc42cd7" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-py3-none-any.whl", hash = "sha256:1acd23810c16332fcae9e805255f0293a45f85c2db26a0000b925e2eff7b8d88" },
        ]

        [[package]]
        name = "d"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/d-1.0.0.tar.gz", hash = "sha256:92a96d88da0f35142034d41ad49d3c5270f29ded946134a1cbfd9aecb57e3cc6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/d-1.0.0-py3-none-any.whl", hash = "sha256:21e5ce053320968a39e4b648f3ff04b838a8a8e623c72729adc78bcc6981ecae" },
        ]

        [[package]]
        name = "d"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/d-2.0.0.tar.gz", hash = "sha256:122c5c68004e59ea46a6bf02db4f271c36dec9782e877e979f8567819187d1bf" }
        wheels = [
            { url = "http://[LOCALHOST]/files/d-2.0.0-py3-none-any.whl", hash = "sha256:8ae5768d43bc22a23099bcc8f988094d7fe367d547b6cfbbcb34c164c03b758d" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "4.3.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "4.4.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "b", marker = "sys_platform == 'linux'" },
            { name = "c", marker = "sys_platform == 'darwin'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "==4.3.0" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = "==4.4.0" },
            { name = "b", marker = "sys_platform == 'linux'", specifier = "==1.0.0" },
            { name = "c", marker = "sys_platform == 'darwin'", specifier = "==1.0.0" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This test checks that we discard fork markers when using `--upgrade`.
///
/// ```text
/// fork-upgrade
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires foo
/// │       ├── satisfied by foo-1.0.0
/// │       └── satisfied by foo-2.0.0
/// ├── bar
/// │   ├── bar-1.0.0
/// │   └── bar-2.0.0
/// └── foo
///     ├── foo-1.0.0
///     │   ├── requires bar==1; sys_platform == "linux"
///     │   │   └── satisfied by bar-1.0.0
///     │   └── requires bar==2; sys_platform != "linux"
///     │       └── satisfied by bar-2.0.0
///     └── foo-2.0.0
///         └── requires bar==2
///             └── satisfied by bar-2.0.0
/// ```
#[test]
fn fork_upgrade() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/fork-upgrade.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''foo''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "bar"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/bar-2.0.0.tar.gz", hash = "sha256:eedc005a3c6d9bd8c2677125d0724958350264ad7f010f5b141ba2c48a4e536b" }
        wheels = [
            { url = "http://[LOCALHOST]/files/bar-2.0.0-py3-none-any.whl", hash = "sha256:c452f4a8f924b1c86b9b70b6f75eaf724f2a611e62027c7886fcc502738298e7" },
        ]

        [[package]]
        name = "foo"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "bar" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/foo-2.0.0.tar.gz", hash = "sha256:d4ad264471a6aa74112173324e8d70ff3713d419676a276d7ec26c18cc6ee1c8" }
        wheels = [
            { url = "http://[LOCALHOST]/files/foo-2.0.0-py3-none-any.whl", hash = "sha256:5c3dbb5a959abef5d1ea0b862df7e0b3c649735053f03ae1f4ff024e07a422e0" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "foo" },
        ]

        [package.metadata]
        requires-dist = [{ name = "foo" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// The root cause the resolver to fork over `a`, but the markers on the variant
/// of `a` don't cover the entire marker space, they are missing Python 3.13.
/// Later, we have a dependency this very hole, which we still need to select,
/// instead of having two forks around but without Python 3.13 and omitting
/// `c` from the solution.
///
/// ```text
/// fork-incomplete-markers
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==1; python_version < "3.13"
/// │   │   └── satisfied by a-1.0.0
/// │   ├── requires a==2; python_version >= "3.14"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires b
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires c; python_version == "3.13"
/// │           └── satisfied by c-1.0.0
/// └── c
///     └── c-1.0.0
/// ```
#[test]
fn fork_incomplete_markers() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/incomplete-markers.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1; python_version < '3.13'''',
          '''a==2; python_version >= '3.14'''',
          '''b''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "python_full_version >= '3.14'",
            "python_full_version == '3.13.*'",
            "python_full_version < '3.13'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "python_full_version < '3.13'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:10eb37b5137fa0770faf91f55610eb2555ff2b6d6b0fe56c53ea9374173ea099" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:e96075bbdd8879e0f5be27c91dd01388085eab3b08e586138c34a5ee2c45091e" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "python_full_version >= '3.14'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "python_full_version == '3.13.*'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:3c858c566dd15f5f8aa6e65014c60143190f55947b8f841e37c05c636b9e4e29" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:7ed5c7b529ab9ea5263a81ba18bd0194d47be9c7abe79cc2c01c06f5406218a8" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-1.0.0.tar.gz", hash = "sha256:1a0dc3013c4de679411df70712ff3a4cd23b873fff1ee8ac1f7f57630bb74f86" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-py3-none-any.whl", hash = "sha256:636e6119ddfb2f285fdd5ce9af209265d7209d9838976bbf86e461300ecd850e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "python_full_version < '3.13'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "python_full_version >= '3.14'" },
            { name = "b" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "python_full_version < '3.13'", specifier = "==1" },
            { name = "a", marker = "python_full_version >= '3.14'", specifier = "==2" },
            { name = "b" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This is actually a non-forking test case that tests the tracking of marker
/// expressions in general. In this case, the dependency on `c` should have its
/// marker expressions automatically combined. In this case, it's `linux OR
/// darwin`, even though `linux OR darwin` doesn't actually appear verbatim as a
/// marker expression for any dependency on `c`.
///
/// ```text
/// fork-marker-accrue
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==1.0.0; implementation_name == "cpython"
/// │   │   └── satisfied by a-1.0.0
/// │   └── requires b==1.0.0; implementation_name == "pypy"
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   └── a-1.0.0
/// │       └── requires c==1.0.0; sys_platform == "linux"
/// │           └── satisfied by c-1.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires c==1.0.0; sys_platform == "darwin"
/// │           └── satisfied by c-1.0.0
/// └── c
///     └── c-1.0.0
/// ```
#[test]
fn fork_marker_accrue() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-accrue.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0 ; implementation_name == 'cpython'''',
          '''b==1.0.0 ; implementation_name == 'pypy'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "sys_platform == 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:ca88475be303b1f1c70b44d2080b11521f96f3a73f01e249372186ce7b389e26" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:79965fe99b0470601d9c340c4266c43956835bbadcf6848fb1584b489b99790a" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:27528fbc2f705135e56206d947ee3307af6034f385107dfa454eb55c01348d97" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:8d0694f857dc75ce93603205331e6dbef49c01338ebb6683b1e5408f360e2f3e" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-1.0.0.tar.gz", hash = "sha256:1a0dc3013c4de679411df70712ff3a4cd23b873fff1ee8ac1f7f57630bb74f86" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-py3-none-any.whl", hash = "sha256:636e6119ddfb2f285fdd5ce9af209265d7209d9838976bbf86e461300ecd850e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", marker = "implementation_name == 'cpython'" },
            { name = "b", marker = "implementation_name == 'pypy'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "implementation_name == 'cpython'", specifier = "==1.0.0" },
            { name = "b", marker = "implementation_name == 'pypy'", specifier = "==1.0.0" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// A basic test that ensures, at least in this one basic case, that forking in
/// universal resolution happens only when the corresponding marker expressions are
/// completely disjoint. Here, we provide two completely incompatible dependency
/// specifications with equivalent markers. Thus, they are trivially not disjoint,
/// and resolution should fail.
///
/// NOTE: This acts a regression test for the initial version of universal
/// resolution that would fork whenever a package was repeated in the list of
/// dependency specifications. So previously, this would produce a resolution with
/// both `1.0.0` and `2.0.0` of `a`. But of course, the correct behavior is to fail
/// resolving.
///
/// ```text
/// fork-marker-disjoint
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "linux"
/// │       └── satisfied by a-1.0.0
/// └── a
///     ├── a-1.0.0
///     └── a-2.0.0
/// ```
#[test]
fn fork_marker_disjoint() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-disjoint.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'linux'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving dependencies:
      ╰─▶ Because your project depends on a{sys_platform == 'linux'}>=2 and a{sys_platform == 'linux'}<2, we can conclude that your project's requirements are unsatisfiable.
    "
    );

    Ok(())
}

/// This test builds on `fork-marker-inherit-combined`. Namely, we add
/// `or implementation_name == 'pypy'` to the dependency on `c`. While
/// `sys_platform == 'linux'` cannot be true because of the first fork,
/// the second fork which includes `b==1.0.0` happens precisely when
/// `implementation_name == 'pypy'`. So in this case, `c` should be
/// included.
///
/// ```text
/// fork-marker-inherit-combined-allowed
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   ├── requires b>=2; implementation_name == "cpython"
/// │   │   │   └── satisfied by b-2.0.0
/// │   │   └── requires b<2; implementation_name == "pypy"
/// │   │       └── satisfied by b-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   ├── b-1.0.0
/// │   │   └── requires c; sys_platform == "linux" or implementation_name == "pypy"
/// │   │       └── satisfied by c-1.0.0
/// │   └── b-2.0.0
/// └── c
///     └── c-1.0.0
/// ```
#[test]
fn fork_marker_inherit_combined_allowed() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-inherit-combined-allowed.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
            "implementation_name != 'cpython' and implementation_name != 'pypy' and sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
            "implementation_name != 'cpython' and implementation_name != 'pypy' and sys_platform == 'darwin'",
        ]
        dependencies = [
            { name = "b", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "implementation_name == 'pypy' and sys_platform == 'darwin'" },
            { name = "b", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "implementation_name == 'cpython' and sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:f87d0a15c41acae3fe0c174fa2ff728202d63f84f5a0863b83b26191b0cb75e9" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:bf960a4508ee61472df3c04d75d4b9640b914c51cbbfb6f4c562b6b010d2b404" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
        ]
        dependencies = [
            { name = "c", marker = "implementation_name == 'pypy' and sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:1b5da2df7c7966dcca30f4329f376bdd8f1bcbda88f9107d1ef08e076655b44b" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:5b5be6a5798e877e2251b7947fa85904cbd58a466994ba0e52b9d6ca03318936" },
        ]

        [[package]]
        name = "b"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.0.0.tar.gz", hash = "sha256:55f2897a25930102575ec735a42948a3f8e62169de5960fbf79ecfc7cf72c002" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.0.0-py3-none-any.whl", hash = "sha256:3d4da1cbab02b8c7b731955206703409e616461cc399c827644b74bcd510a7b7" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-1.0.0.tar.gz", hash = "sha256:1a0dc3013c4de679411df70712ff3a4cd23b873fff1ee8ac1f7f57630bb74f86" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-py3-none-any.whl", hash = "sha256:636e6119ddfb2f285fdd5ce9af209265d7209d9838976bbf86e461300ecd850e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This test builds on `fork-marker-inherit-combined`. Namely, we add
/// `or implementation_name == 'cpython'` to the dependency on `c`.
/// While `sys_platform == 'linux'` cannot be true because of the first
/// fork, the second fork which includes `b==1.0.0` happens precisely
/// when `implementation_name == 'pypy'`, which is *also* disjoint with
/// `implementation_name == 'cpython'`. Therefore, `c` should not be
/// included here.
///
/// ```text
/// fork-marker-inherit-combined-disallowed
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   ├── requires b>=2; implementation_name == "cpython"
/// │   │   │   └── satisfied by b-2.0.0
/// │   │   └── requires b<2; implementation_name == "pypy"
/// │   │       └── satisfied by b-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   ├── b-1.0.0
/// │   │   └── requires c; sys_platform == "linux" or implementation_name == "cpython"
/// │   │       └── satisfied by c-1.0.0
/// │   └── b-2.0.0
/// └── c
///     └── c-1.0.0
/// ```
#[test]
fn fork_marker_inherit_combined_disallowed() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-inherit-combined-disallowed.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
            "implementation_name != 'cpython' and implementation_name != 'pypy' and sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
            "implementation_name != 'cpython' and implementation_name != 'pypy' and sys_platform == 'darwin'",
        ]
        dependencies = [
            { name = "b", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "implementation_name == 'pypy' and sys_platform == 'darwin'" },
            { name = "b", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "implementation_name == 'cpython' and sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:f87d0a15c41acae3fe0c174fa2ff728202d63f84f5a0863b83b26191b0cb75e9" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:bf960a4508ee61472df3c04d75d4b9640b914c51cbbfb6f4c562b6b010d2b404" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:d241f66dc9de74c0ca92c4efa15c63850b596057eeb06e81019bf92840281405" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:291080959e6c633c35620507aaba7a973641796dca5b1763b509f9d582c4c60d" },
        ]

        [[package]]
        name = "b"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.0.0.tar.gz", hash = "sha256:55f2897a25930102575ec735a42948a3f8e62169de5960fbf79ecfc7cf72c002" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.0.0-py3-none-any.whl", hash = "sha256:3d4da1cbab02b8c7b731955206703409e616461cc399c827644b74bcd510a7b7" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// In this test, we check that marker expressions which provoke a fork
/// are carried through to subsequent forks. Here, the `a>=2` and `a<2`
/// dependency specifications create a fork, and then the `a<2` fork leads
/// to `a==1.0.0` with dependency specifications on `b>=2` and `b<2` that
/// provoke yet another fork. Finally, in the `b<2` fork, a dependency on
/// `c` is introduced whose marker expression is disjoint with the marker
/// expression that provoked the *first* fork. Therefore, `c` should be
/// entirely excluded from the resolution.
///
/// ```text
/// fork-marker-inherit-combined
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   ├── requires b>=2; implementation_name == "cpython"
/// │   │   │   └── satisfied by b-2.0.0
/// │   │   └── requires b<2; implementation_name == "pypy"
/// │   │       └── satisfied by b-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   ├── b-1.0.0
/// │   │   └── requires c; sys_platform == "linux"
/// │   │       └── satisfied by c-1.0.0
/// │   └── b-2.0.0
/// └── c
///     └── c-1.0.0
/// ```
#[test]
fn fork_marker_inherit_combined() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-inherit-combined.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
            "implementation_name != 'cpython' and implementation_name != 'pypy' and sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
            "implementation_name != 'cpython' and implementation_name != 'pypy' and sys_platform == 'darwin'",
        ]
        dependencies = [
            { name = "b", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "implementation_name == 'pypy' and sys_platform == 'darwin'" },
            { name = "b", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "implementation_name == 'cpython' and sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:f87d0a15c41acae3fe0c174fa2ff728202d63f84f5a0863b83b26191b0cb75e9" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:bf960a4508ee61472df3c04d75d4b9640b914c51cbbfb6f4c562b6b010d2b404" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'pypy' and sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:893c28aa573190378af3b91c816db2b2ee67201a544d87b87ccb0a8167c03a51" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:7d9b580488e074a843b7a3d5c4109396bce7d76495501731ac4bae8969c3efd3" },
        ]

        [[package]]
        name = "b"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "implementation_name == 'cpython' and sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.0.0.tar.gz", hash = "sha256:55f2897a25930102575ec735a42948a3f8e62169de5960fbf79ecfc7cf72c002" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.0.0-py3-none-any.whl", hash = "sha256:3d4da1cbab02b8c7b731955206703409e616461cc399c827644b74bcd510a7b7" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This is like `fork-marker-inherit`, but where both `a>=2` and `a<2`
/// have a conditional dependency on `b`. For `a>=2`, the conditional
/// dependency on `b` has overlap with the `a>=2` marker expression, and
/// thus, `b` should be included *only* in the dependencies for `a==2.0.0`.
/// As with `fork-marker-inherit`, the `a<2` path should exclude `b==1.0.0`
/// since their marker expressions are disjoint.
///
/// ```text
/// fork-marker-inherit-isolated
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   └── requires b; sys_platform == "linux"
/// │   │       └── satisfied by b-1.0.0
/// │   └── a-2.0.0
/// │       └── requires b; sys_platform == "linux"
/// │           └── satisfied by b-1.0.0
/// └── b
///     └── b-1.0.0
/// ```
#[test]
fn fork_marker_inherit_isolated() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-inherit-isolated.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:b48b4f28fbc32cf7391e6ff98e0b516f225cfc6502539283e04098fb192a9e4a" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:e61e8d56ebf2bb0040d28532ecd81c4f1865fddd6455088b6a224053ab731576" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        dependencies = [
            { name = "b", marker = "sys_platform == 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:4fda3519ad2e8272e7aff8651cb08da7aa1e082a15d77b6281292cdf7060a4ae" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:7b813a7fd717a0c9fb3f6d81b7a94ae38cc67fb2281bf0ff5056a3e1a2db65c7" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:444108175a344c7a5c435b365246b1460e85f8243b9da7143de631c88fe649b0" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:f400cd1e5f2cf257b093df1df6ef025412f92e22a8dc275f5df4ea43b0620974" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This is like `fork-marker-inherit`, but tests that the marker
/// expressions that provoke a fork are carried transitively through the
/// dependency graph. In this case, `a<2 -> b -> c -> d`, but where the
/// last dependency on `d` requires a marker expression that is disjoint
/// with the initial `a<2` dependency. Therefore, it ought to be completely
/// excluded from the resolution.
///
/// ```text
/// fork-marker-inherit-transitive
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   └── requires b
/// │   │       └── satisfied by b-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires c
/// │           └── satisfied by c-1.0.0
/// ├── c
/// │   └── c-1.0.0
/// │       └── requires d; sys_platform == "linux"
/// │           └── satisfied by d-1.0.0
/// └── d
///     └── d-1.0.0
/// ```
#[test]
fn fork_marker_inherit_transitive() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-inherit-transitive.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        dependencies = [
            { name = "b", marker = "sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:b9a50f9a7e0ebf5a3cca32a40ec260b7a56fd6cad409031c2b2f74575976b38d" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:5130162ce8802776aa01628619f0024ad745e6c5c3227785b81a92cc65bbef0e" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:6d1cc41f8872b849239fbb43617a9db469c237cd79d1ef591721a839fab51895" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:bf31718603b577e182edf2d33cecfd0b3dda9df0f0e6ab60bd4fc47923631df1" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-1.0.0.tar.gz", hash = "sha256:5d4d1b19d7001cd5b2a3d620efda99e8f91850805d2c982f20bb93cd08346cec" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-py3-none-any.whl", hash = "sha256:306a2cb058d8abdf1654a01ba6080ffa1bddc4f9120aa934c233f91b9703ce01" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This tests that markers which provoked a fork in the universal resolver
/// are used to ignore dependencies which cannot possibly be installed by a
/// resolution produced by that fork.
///
/// In this example, the `a<2` dependency is only active on Darwin
/// platforms. But the `a==1.0.0` distribution has a dependency on `b`
/// that is only active on Linux, where as `a==2.0.0` does not. Therefore,
/// when the fork provoked by the `a<2` dependency considers `b`, it should
/// ignore it because it isn't possible for `sys_platform == 'linux'` and
/// `sys_platform == 'darwin'` to be simultaneously true.
///
/// ```text
/// fork-marker-inherit
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "darwin"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   └── requires b; sys_platform == "linux"
/// │   │       └── satisfied by b-1.0.0
/// │   └── a-2.0.0
/// └── b
///     └── b-1.0.0
/// ```
#[test]
fn fork_marker_inherit() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-inherit.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:b48b4f28fbc32cf7391e6ff98e0b516f225cfc6502539283e04098fb192a9e4a" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:e61e8d56ebf2bb0040d28532ecd81c4f1865fddd6455088b6a224053ab731576" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This is like `fork-marker-inherit`, but it tests that dependency
/// filtering only occurs in the context of a fork.
///
/// For example, as in `fork-marker-inherit`, the `c` dependency of
/// `a<2` should be entirely excluded here since it is possible for
/// `sys_platform` to be simultaneously equivalent to Darwin and Linux.
/// However, the unconditional dependency on `b`, which in turn depends on
/// `c` for Linux only, should still incorporate `c` as the dependency is
/// not part of any fork.
///
/// ```text
/// fork-marker-limited-inherit
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "linux"
/// │   │   └── satisfied by a-2.0.0
/// │   ├── requires a<2; sys_platform == "darwin"
/// │   │   └── satisfied by a-1.0.0
/// │   └── requires b
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   └── requires c; sys_platform == "linux"
/// │   │       └── satisfied by c-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires c; sys_platform == "linux"
/// │           └── satisfied by c-1.0.0
/// └── c
///     └── c-1.0.0
/// ```
#[test]
fn fork_marker_limited_inherit() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-limited-inherit.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'linux'''',
          '''a<2 ; sys_platform == 'darwin'''',
          '''b''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:9f1816f865745d2fa4515cef553b4f251b9d2eb9d91804c46c9812ae150aacc6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:45ac575905e7cc1d9751ecda3ad8bfbff5da0d3197f2751f1ccf067872e6106c" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "sys_platform == 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:893c28aa573190378af3b91c816db2b2ee67201a544d87b87ccb0a8167c03a51" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:7d9b580488e074a843b7a3d5c4109396bce7d76495501731ac4bae8969c3efd3" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-1.0.0.tar.gz", hash = "sha256:1a0dc3013c4de679411df70712ff3a4cd23b873fff1ee8ac1f7f57630bb74f86" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-py3-none-any.whl", hash = "sha256:636e6119ddfb2f285fdd5ce9af209265d7209d9838976bbf86e461300ecd850e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "b" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'linux'", specifier = ">=2" },
            { name = "b" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This tests a case where the resolver forks because of non-overlapping marker
/// expressions on `b`. In the original universal resolver implementation, this
/// resulted in multiple versions of `a` being unconditionally included in the lock
/// file. So this acts as a regression test to ensure that only one version of `a`
/// is selected.
///
/// ```text
/// fork-marker-selection
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a
/// │   │   ├── satisfied by a-0.1.0
/// │   │   └── satisfied by a-0.2.0
/// │   ├── requires b>=2; sys_platform == "linux"
/// │   │   └── satisfied by b-2.0.0
/// │   └── requires b<2; sys_platform == "darwin"
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   ├── a-0.1.0
/// │   └── a-0.2.0
/// │       └── requires b>=2.0.0
/// │           └── satisfied by b-2.0.0
/// └── b
///     ├── b-1.0.0
///     └── b-2.0.0
/// ```
#[test]
fn fork_marker_selection() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-selection.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a''',
          '''b>=2 ; sys_platform == 'linux'''',
          '''b<2 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "0.1.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-0.1.0.tar.gz", hash = "sha256:95ea3bf6ab98343c3af41c7cb01804aca90e66aceeb269e7e35988fd4626a92c" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-0.1.0-py3-none-any.whl", hash = "sha256:bc03a2d2180e65e4e73ccaa7249b3ddc1016774efbad3b3b486deadbaca49613" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:444108175a344c7a5c435b365246b1460e85f8243b9da7143de631c88fe649b0" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:f400cd1e5f2cf257b093df1df6ef025412f92e22a8dc275f5df4ea43b0620974" },
        ]

        [[package]]
        name = "b"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.0.0.tar.gz", hash = "sha256:55f2897a25930102575ec735a42948a3f8e62169de5960fbf79ecfc7cf72c002" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.0.0-py3-none-any.whl", hash = "sha256:3d4da1cbab02b8c7b731955206703409e616461cc399c827644b74bcd510a7b7" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
            { name = "b", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "b", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a" },
            { name = "b", marker = "sys_platform == 'darwin'", specifier = "<2" },
            { name = "b", marker = "sys_platform == 'linux'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// fork-marker-track
///
/// ```text
/// fork-marker-track
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a
/// │   │   ├── satisfied by a-1.3.1
/// │   │   ├── satisfied by a-2.0.0
/// │   │   ├── satisfied by a-3.1.0
/// │   │   └── satisfied by a-4.3.0
/// │   ├── requires b>=2.8; sys_platform == "linux"
/// │   │   └── satisfied by b-2.8
/// │   └── requires b<2.8; sys_platform == "darwin"
/// │       └── satisfied by b-2.7
/// ├── a
/// │   ├── a-1.3.1
/// │   │   └── requires c; implementation_name == "iron"
/// │   │       └── satisfied by c-1.10
/// │   ├── a-2.0.0
/// │   │   ├── requires b>=2.8
/// │   │   │   └── satisfied by b-2.8
/// │   │   └── requires c; implementation_name == "cpython"
/// │   │       └── satisfied by c-1.10
/// │   ├── a-3.1.0
/// │   │   ├── requires b>=2.8
/// │   │   │   └── satisfied by b-2.8
/// │   │   └── requires c; implementation_name == "pypy"
/// │   │       └── satisfied by c-1.10
/// │   └── a-4.3.0
/// │       └── requires b>=2.8
/// │           └── satisfied by b-2.8
/// ├── b
/// │   ├── b-2.7
/// │   └── b-2.8
/// └── c
///     └── c-1.10
/// ```
#[test]
fn fork_marker_track() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/marker-track.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a''',
          '''b>=2.8 ; sys_platform == 'linux'''',
          '''b<2.8 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'darwin'",
            "sys_platform == 'linux'",
            "sys_platform != 'darwin' and sys_platform != 'linux'",
        ]

        [[package]]
        name = "a"
        version = "1.3.1"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "implementation_name == 'iron'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.3.1.tar.gz", hash = "sha256:d619d40d696f148bd83aea8741d4c200e52724ee26c8ca92d7839e82f08635fa" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.3.1-py3-none-any.whl", hash = "sha256:e16058679bf4e8104f74c14833637e85e2521f3eafe93a2e88c4f63cbef96a94" },
        ]

        [[package]]
        name = "b"
        version = "2.7"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'darwin'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.7.tar.gz", hash = "sha256:d768eca6b4468008376430d1704db3c7519959e5f0985455fcf17b761481cd42" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.7-py3-none-any.whl", hash = "sha256:2805a9e34c2223875f7d141ae5e834e3bf52624231fbae11ae15d34dfa51590d" },
        ]

        [[package]]
        name = "b"
        version = "2.8"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.8.tar.gz", hash = "sha256:2e6b546ca893410451b318ce2d7c0f69c41b602d916892f4f3d83147199c4045" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.8-py3-none-any.whl", hash = "sha256:03c21d4304d11442246b563042e5ef753f2dde8a1543e777db059c6c16e1adad" },
        ]

        [[package]]
        name = "c"
        version = "1.10"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-1.10.tar.gz", hash = "sha256:8ef0155e958db107593f5af7524172b32c7bc02133ec32e78ca0c4b8d94007f8" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.10-py3-none-any.whl", hash = "sha256:9dff4662032849867c7d0c86997da8a67bcdeb0b76319a03d4d3d6fb00ef597e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
            { name = "b", version = "2.7", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'darwin'" },
            { name = "b", version = "2.8", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a" },
            { name = "b", marker = "sys_platform == 'darwin'", specifier = "<2.8" },
            { name = "b", marker = "sys_platform == 'linux'", specifier = ">=2.8" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This is the same setup as `non-local-fork-marker-transitive`, but the disjoint
/// dependency specifications on `c` use the same constraints and thus depend on
/// the same version of `c`. In this case, there is no conflict.
///
/// ```text
/// fork-non-fork-marker-transitive
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==1.0.0
/// │   │   └── satisfied by a-1.0.0
/// │   └── requires b==1.0.0
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   └── a-1.0.0
/// │       └── requires c>=2.0.0; sys_platform == "linux"
/// │           └── satisfied by c-2.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires c>=2.0.0; sys_platform == "darwin"
/// │           └── satisfied by c-2.0.0
/// └── c
///     ├── c-1.0.0
///     └── c-2.0.0
/// ```
#[test]
fn fork_non_fork_marker_transitive() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/non-fork-marker-transitive.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0''',
          '''b==1.0.0''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "sys_platform == 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:c6d5e3c11fdd3296b2659d4cbe670db8c4c0e0d51acfa08bbe0da6abe02066e3" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:aa3544062c4e2c5037a9c847283af433ed079fc3fa4dd8c1f35efcdebee4550f" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", marker = "sys_platform == 'darwin'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:ff78b225ca376755fb1c84944b1ecfbb55b14dc18c6321abd902e7618e57782a" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:57d646ea071b91aa984c4fd4f5cadb375e9da2befb0e3365495864c20830b619" },
        ]

        [[package]]
        name = "c"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-2.0.0.tar.gz", hash = "sha256:37a07411ed3ed6b9cb796ae510bea299cb271a38fc9c64763a2efc920625a5e5" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-2.0.0-py3-none-any.whl", hash = "sha256:44d2299422cb766c80a8fc55892d1d73c090481641e084936ed993baa128b7f9" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
            { name = "b" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", specifier = "==1.0.0" },
            { name = "b", specifier = "==1.0.0" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This is like `non-local-fork-marker-transitive`, but the marker expressions are
/// placed on sibling dependency specifications. However, the actual dependency on
/// `c` is indirect, and thus, there's no fork detected by the universal resolver.
/// This in turn results in an unresolvable conflict on `c`.
///
/// ```text
/// fork-non-local-fork-marker-direct
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==1.0.0; sys_platform == "linux"
/// │   │   └── satisfied by a-1.0.0
/// │   └── requires b==1.0.0; sys_platform == "darwin"
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   └── a-1.0.0
/// │       └── requires c<2.0.0
/// │           └── satisfied by c-1.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires c>=2.0.0
/// │           └── satisfied by c-2.0.0
/// └── c
///     ├── c-1.0.0
///     └── c-2.0.0
/// ```
#[test]
fn fork_non_local_fork_marker_direct() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/non-local-fork-marker-direct.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0 ; sys_platform == 'linux'''',
          '''b==1.0.0 ; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving dependencies:
      ╰─▶ Because a==1.0.0 depends on c<2.0.0 and b==1.0.0 depends on c>=2.0.0, we can conclude that b==1.0.0 and a{sys_platform == 'linux'}==1.0.0 are incompatible.
          And because your project depends on a{sys_platform == 'linux'}==1.0.0 and b{sys_platform == 'darwin'}==1.0.0, we can conclude that your project's requirements are unsatisfiable.
    "
    );

    Ok(())
}

/// This setup introduces dependencies on two distinct versions of `c`, where
/// each such dependency has a marker expression attached that would normally
/// make them disjoint. In a non-universal resolver, this is no problem. But in a
/// forking resolver that tries to create one universal resolution, this can lead
/// to two distinct versions of `c` in the resolution. This is in and of itself
/// not a problem, since that is an expected scenario for universal resolution.
/// The problem in this case is that because the dependency specifications for
/// `c` occur in two different points (i.e., they are not sibling dependency
/// specifications) in the dependency graph, the forking resolver does not "detect"
/// it, and thus never forks and thus this results in "no resolution."
///
/// ```text
/// fork-non-local-fork-marker-transitive
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==1.0.0
/// │   │   └── satisfied by a-1.0.0
/// │   └── requires b==1.0.0
/// │       └── satisfied by b-1.0.0
/// ├── a
/// │   └── a-1.0.0
/// │       └── requires c<2.0.0; sys_platform == "linux"
/// │           └── satisfied by c-1.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires c>=2.0.0; sys_platform == "darwin"
/// │           └── satisfied by c-2.0.0
/// └── c
///     ├── c-1.0.0
///     └── c-2.0.0
/// ```
#[test]
fn fork_non_local_fork_marker_transitive() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/non-local-fork-marker-transitive.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0''',
          '''b==1.0.0''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
      × No solution found when resolving dependencies:
      ╰─▶ Because a==1.0.0 depends on c{sys_platform == 'linux'}<2.0.0 and b==1.0.0 depends on c{sys_platform == 'darwin'}>=2.0.0, we can conclude that a==1.0.0 and b==1.0.0 are incompatible.
          And because your project depends on a==1.0.0 and b==1.0.0, we can conclude that your project's requirements are unsatisfiable.
    "
    );

    Ok(())
}

/// This scenario tests a very basic case of overlapping markers. Namely,
/// it emulates a common pattern in the ecosystem where marker expressions
/// are used to progressively increase the version constraints of a package
/// as the Python version increases.
///
/// In this case, there is actually a split occurring between
/// `python_version < '3.13'` and the other marker expressions, so this
/// isn't just a scenario with overlapping but non-disjoint markers.
///
/// In particular, this serves as a regression test. uv used to create a
/// lock file with a dependency on `a` with the following markers:
///
///     python_version < '3.13' or python_version >= '3.14'
///
/// But this implies that `a` won't be installed for Python 3.13, which is
/// clearly wrong.
///
/// The issue was that uv was intersecting *all* marker expressions. So
/// that `a>=1.1.0` and `a>=1.2.0` fork was getting `python_version >=
/// '3.13' and python_version >= '3.14'`, which, of course, simplifies
/// to `python_version >= '3.14'`. But this is wrong! It should be
/// `python_version >= '3.13' or python_version >= '3.14'`, which of course
/// simplifies to `python_version >= '3.13'`. And thus, the resulting forks
/// are not just disjoint but complete in this case.
///
/// Since there are no other constraints on `a`, this causes uv to select
/// `1.2.0` unconditionally. (The marker expressions get normalized out
/// entirely.)
///
/// ```text
/// fork-overlapping-markers-basic
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=1.0.0; python_version < "3.13"
/// │   │   ├── satisfied by a-1.0.0
/// │   │   ├── satisfied by a-1.1.0
/// │   │   └── satisfied by a-1.2.0
/// │   ├── requires a>=1.1.0; python_version >= "3.13"
/// │   │   ├── satisfied by a-1.1.0
/// │   │   └── satisfied by a-1.2.0
/// │   └── requires a>=1.2.0; python_version >= "3.14"
/// │       └── satisfied by a-1.2.0
/// └── a
///     ├── a-1.0.0
///     ├── a-1.1.0
///     └── a-1.2.0
/// ```
#[test]
fn fork_overlapping_markers_basic() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/overlapping-markers-basic.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=1.0.0 ; python_version < '3.13'''',
          '''a>=1.1.0 ; python_version >= '3.13'''',
          '''a>=1.2.0 ; python_version >= '3.14'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "python_full_version >= '3.14'",
            "python_full_version == '3.13.*'",
            "python_full_version < '3.13'",
        ]

        [[package]]
        name = "a"
        version = "1.2.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.2.0.tar.gz", hash = "sha256:72100f17c6cb3fb139367d2b6e9e95d6259083018244c3a7bbbac8ad786f49a5" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.2.0-py3-none-any.whl", hash = "sha256:d2ccc0c7325055926a90702bf46c3c3e80be881299fa78b8b57f296b477623b1" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "python_full_version < '3.13'", specifier = ">=1.0.0" },
            { name = "a", marker = "python_full_version >= '3.13'", specifier = ">=1.1.0" },
            { name = "a", marker = "python_full_version >= '3.14'", specifier = ">=1.2.0" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This test contains a bistable resolution scenario when not using ahead-of-time
/// splitting of resolution forks: We meet one of two fork points depending on the
/// preferences, creating a resolution whose preferences lead us the other fork
/// point.
///
/// In the first case, we are in cleaver 2 and fork on `sys_platform`, in the
/// second case, we are in foo 1 or bar 1 amd fork over `os_name`.
///
/// First case: We select cleaver 2, fork on `sys_platform`, we reject cleaver 2
/// (missing fork `os_name`), we select cleaver 1 and don't fork on `os_name` in
/// `fork-if-not-forked`, done.
/// Second case: We have preference cleaver 1, fork on `os_name` in
/// `fork-if-not-forked`, we reject cleaver 1, we select cleaver 2, we fork on
/// `sys_platform`, we accept cleaver 2 since we forked on `os_name`, done.
///
/// ```text
/// preferences-dependent-forking-bistable
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires cleaver
/// │       ├── satisfied by cleaver-2.0.0
/// │       └── satisfied by cleaver-1.0.0
/// ├── cleaver
/// │   ├── cleaver-2.0.0
/// │   │   ├── requires fork-sys-platform==1; sys_platform == "linux"
/// │   │   │   └── satisfied by fork-sys-platform-1.0.0
/// │   │   ├── requires fork-sys-platform==2; sys_platform != "linux"
/// │   │   │   └── satisfied by fork-sys-platform-2.0.0
/// │   │   ├── requires reject-cleaver2==1; os_name == "posix"
/// │   │   │   └── satisfied by reject-cleaver2-1.0.0
/// │   │   └── requires reject-cleaver2-proxy
/// │   │       └── satisfied by reject-cleaver2-proxy-1.0.0
/// │   └── cleaver-1.0.0
/// │       ├── requires fork-if-not-forked!=2; sys_platform == "linux"
/// │       │   ├── satisfied by fork-if-not-forked-1.0.0
/// │       │   └── satisfied by fork-if-not-forked-3.0.0
/// │       ├── requires fork-if-not-forked-proxy; sys_platform != "linux"
/// │       │   └── satisfied by fork-if-not-forked-proxy-1.0.0
/// │       ├── requires reject-cleaver1==1; sys_platform == "linux"
/// │       │   └── satisfied by reject-cleaver1-1.0.0
/// │       └── requires reject-cleaver1-proxy
/// │           └── satisfied by reject-cleaver1-proxy-1.0.0
/// ├── fork-if-not-forked
/// │   ├── fork-if-not-forked-1.0.0
/// │   │   ├── requires fork-os-name==1; os_name == "posix"
/// │   │   │   └── satisfied by fork-os-name-1.0.0
/// │   │   ├── requires fork-os-name==2; os_name != "posix"
/// │   │   │   └── satisfied by fork-os-name-2.0.0
/// │   │   └── requires reject-cleaver1-proxy
/// │   │       └── satisfied by reject-cleaver1-proxy-1.0.0
/// │   ├── fork-if-not-forked-2.0.0
/// │   └── fork-if-not-forked-3.0.0
/// ├── fork-if-not-forked-proxy
/// │   └── fork-if-not-forked-proxy-1.0.0
/// │       └── requires fork-if-not-forked!=3
/// │           ├── satisfied by fork-if-not-forked-1.0.0
/// │           └── satisfied by fork-if-not-forked-2.0.0
/// ├── fork-os-name
/// │   ├── fork-os-name-1.0.0
/// │   └── fork-os-name-2.0.0
/// ├── fork-sys-platform
/// │   ├── fork-sys-platform-1.0.0
/// │   └── fork-sys-platform-2.0.0
/// ├── reject-cleaver1
/// │   ├── reject-cleaver1-1.0.0
/// │   └── reject-cleaver1-2.0.0
/// ├── reject-cleaver1-proxy
/// │   └── reject-cleaver1-proxy-1.0.0
/// │       └── requires reject-cleaver1==2; sys_platform != "linux"
/// │           └── satisfied by reject-cleaver1-2.0.0
/// ├── reject-cleaver2
/// │   ├── reject-cleaver2-1.0.0
/// │   └── reject-cleaver2-2.0.0
/// └── reject-cleaver2-proxy
///     └── reject-cleaver2-proxy-1.0.0
///         └── requires reject-cleaver2==2; os_name != "posix"
///             └── satisfied by reject-cleaver2-2.0.0
/// ```
#[test]
fn preferences_dependent_forking_bistable() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/preferences-dependent-forking-bistable.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''cleaver''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 8 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'linux'",
            "sys_platform != 'linux'",
        ]

        [[package]]
        name = "cleaver"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "fork-if-not-forked", version = "3.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "fork-if-not-forked-proxy", marker = "sys_platform != 'linux'" },
            { name = "reject-cleaver1", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "reject-cleaver1-proxy" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/cleaver-1.0.0.tar.gz", hash = "sha256:0ac64b5eb8a4b10d979ee2f7bbd5f54f42ad1210a7e003dfaf85a196ffd5f602" }
        wheels = [
            { url = "http://[LOCALHOST]/files/cleaver-1.0.0-py3-none-any.whl", hash = "sha256:085facdafb888ebff451d7895a9db9a0a9660c8d1f0be4ff0cc1ef574d948b75" },
        ]

        [[package]]
        name = "fork-if-not-forked"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform != 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/fork_if_not_forked-2.0.0.tar.gz", hash = "sha256:f972a3d25abdb71db3585126be721e8d98f87945584869be8df10def4815c374" }
        wheels = [
            { url = "http://[LOCALHOST]/files/fork_if_not_forked-2.0.0-py3-none-any.whl", hash = "sha256:e9904eb3c300785ae58902bb33fb16e9226734537a33d71fca9a0706e4275e75" },
        ]

        [[package]]
        name = "fork-if-not-forked"
        version = "3.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/fork_if_not_forked-3.0.0.tar.gz", hash = "sha256:b3ca82e1fd2d2e2df3b9d33ec2f87a4fcf109c4cd69324888d82047d8622cac3" }
        wheels = [
            { url = "http://[LOCALHOST]/files/fork_if_not_forked-3.0.0-py3-none-any.whl", hash = "sha256:63305e229c07a2f21f4388c30e3c2bf7622f1d0ea0e99ff6cde2dd2700822fc9" },
        ]

        [[package]]
        name = "fork-if-not-forked-proxy"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "fork-if-not-forked", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/fork_if_not_forked_proxy-1.0.0.tar.gz", hash = "sha256:235c381ca9079a02426179e3767662c5f9603db281a7f9eef8ee147575598b18" }
        wheels = [
            { url = "http://[LOCALHOST]/files/fork_if_not_forked_proxy-1.0.0-py3-none-any.whl", hash = "sha256:9e54d363a8d1ce771614712b24debd7dac0a7161526d1423dd2134a3898fbfca" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "cleaver" },
        ]

        [package.metadata]
        requires-dist = [{ name = "cleaver" }]

        [[package]]
        name = "reject-cleaver1"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/reject_cleaver1-1.0.0.tar.gz", hash = "sha256:1407a50a2d354fb77d273965db766d4df07b7b24e0fd2419477b894e18cf7a67" }
        wheels = [
            { url = "http://[LOCALHOST]/files/reject_cleaver1-1.0.0-py3-none-any.whl", hash = "sha256:eb0a092ed719f4bead3b604d63db7f84a3711e202fbab98b70ef4b3f417b9dcc" },
        ]

        [[package]]
        name = "reject-cleaver1"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform != 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/reject_cleaver1-2.0.0.tar.gz", hash = "sha256:eba7ff63f7158bf323b9389aff54f53e5e8652d232b064a308adbefe2b2496bd" }
        wheels = [
            { url = "http://[LOCALHOST]/files/reject_cleaver1-2.0.0-py3-none-any.whl", hash = "sha256:2d2181261e6d5a0d521ba357625d49503cb373e5790cc7392c701ebc4c1ab5f6" },
        ]

        [[package]]
        name = "reject-cleaver1-proxy"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "reject-cleaver1", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/reject_cleaver1_proxy-1.0.0.tar.gz", hash = "sha256:fcfd9ed9c8cf4a04bf3e13ce5ff6eab983fefbea87c71b6f04b0a9918fb5d8be" }
        wheels = [
            { url = "http://[LOCALHOST]/files/reject_cleaver1_proxy-1.0.0-py3-none-any.whl", hash = "sha256:c7759e5a3d5697253404454ff5ce202dcb710233a674c2f2e9bdb560b8eff98f" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// Like `preferences-dependent-forking`, but when we don't fork the resolution fails.
///
/// Consider a fresh run without preferences:
/// * We start with cleaver 2
/// * We fork
/// * We reject cleaver 2
/// * We find cleaver solution in fork 1 with foo 2 with bar 1
/// * We find cleaver solution in fork 2 with foo 1 with bar 2
/// * We write cleaver 1, foo 1, foo 2, bar 1 and bar 2 to the lockfile
///
/// In a subsequent run, we read the preference cleaver 1 from the lockfile (the preferences for foo and bar don't matter):
/// * We start with cleaver 1
/// * We're in universal mode, cleaver requires foo 1, bar 1
/// * foo 1 requires bar 2, conflict
///
/// Design sketch:
/// ```text
/// root -> clear, foo, bar
/// # Cause a fork, then forget that version.
/// cleaver 2 -> unrelated-dep==1; fork==1
/// cleaver 2 -> unrelated-dep==2; fork==2
/// cleaver 2 -> reject-cleaver-2
/// # Allow different versions when forking, but force foo 1, bar 1 in universal mode without forking.
/// cleaver 1 -> foo==1; fork==1
/// cleaver 1 -> bar==1; fork==2
/// # When we selected foo 1, bar 1 in universal mode for cleaver, this causes a conflict, otherwise we select bar 2.
/// foo 1 -> bar==2
/// ```
///
/// ```text
/// preferences-dependent-forking-conflicting
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires bar
/// │   │   ├── satisfied by bar-1.0.0
/// │   │   └── satisfied by bar-2.0.0
/// │   ├── requires cleaver
/// │   │   ├── satisfied by cleaver-2.0.0
/// │   │   └── satisfied by cleaver-1.0.0
/// │   └── requires foo
/// │       ├── satisfied by foo-1.0.0
/// │       └── satisfied by foo-2.0.0
/// ├── bar
/// │   ├── bar-1.0.0
/// │   └── bar-2.0.0
/// ├── cleaver
/// │   ├── cleaver-2.0.0
/// │   │   ├── requires reject-cleaver-2
/// │   │   │   └── satisfied by reject-cleaver-2-1.0.0
/// │   │   ├── requires unrelated-dep==1; sys_platform == "linux"
/// │   │   │   └── satisfied by unrelated-dep-1.0.0
/// │   │   └── requires unrelated-dep==2; sys_platform != "linux"
/// │   │       └── satisfied by unrelated-dep-2.0.0
/// │   └── cleaver-1.0.0
/// │       ├── requires bar==1; sys_platform != "linux"
/// │       │   └── satisfied by bar-1.0.0
/// │       └── requires foo==1; sys_platform == "linux"
/// │           └── satisfied by foo-1.0.0
/// ├── foo
/// │   ├── foo-1.0.0
/// │   │   └── requires bar==2
/// │   │       └── satisfied by bar-2.0.0
/// │   └── foo-2.0.0
/// ├── reject-cleaver-2
/// │   └── reject-cleaver-2-1.0.0
/// │       └── requires unrelated-dep==3
/// │           └── satisfied by unrelated-dep-3.0.0
/// └── unrelated-dep
///     ├── unrelated-dep-1.0.0
///     ├── unrelated-dep-2.0.0
///     └── unrelated-dep-3.0.0
/// ```
#[test]
fn preferences_dependent_forking_conflicting() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/preferences-dependent-forking-conflicting.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''cleaver''',
          '''foo''',
          '''bar''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 6 packages in [TIME]
    "
    );

    Ok(())
}

/// This test case is like "preferences-dependent-forking-bistable", but with three
/// states instead of two. The first two locks are in a different state, then we
/// enter the tristable state.
///
/// It's not polished, but it's useful to have something with a higher period
/// than 2 in our test suite.
///
/// ```text
/// preferences-dependent-forking-tristable
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires bar
/// │   │   ├── satisfied by bar-1.0.0
/// │   │   └── satisfied by bar-2.0.0
/// │   ├── requires cleaver
/// │   │   ├── satisfied by cleaver-2.0.0
/// │   │   └── satisfied by cleaver-1.0.0
/// │   └── requires foo
/// │       ├── satisfied by foo-1.0.0
/// │       └── satisfied by foo-2.0.0
/// ├── a
/// │   └── a-1.0.0
/// │       └── requires unrelated-dep3==1; os_name == "posix"
/// │           └── satisfied by unrelated-dep3-1.0.0
/// ├── b
/// │   └── b-1.0.0
/// │       └── requires unrelated-dep3==2; os_name != "posix"
/// │           └── satisfied by unrelated-dep3-2.0.0
/// ├── bar
/// │   ├── bar-1.0.0
/// │   │   ├── requires c!=3; sys_platform == "linux"
/// │   │   │   ├── satisfied by c-1.0.0
/// │   │   │   └── satisfied by c-2.0.0
/// │   │   ├── requires d; sys_platform != "linux"
/// │   │   │   └── satisfied by d-1.0.0
/// │   │   └── requires reject-cleaver-1
/// │   │       └── satisfied by reject-cleaver-1-1.0.0
/// │   └── bar-2.0.0
/// ├── c
/// │   ├── c-1.0.0
/// │   │   ├── requires reject-cleaver-1
/// │   │   │   └── satisfied by reject-cleaver-1-1.0.0
/// │   │   ├── requires unrelated-dep2==1; os_name == "posix"
/// │   │   │   └── satisfied by unrelated-dep2-1.0.0
/// │   │   └── requires unrelated-dep2==2; os_name != "posix"
/// │   │       └── satisfied by unrelated-dep2-2.0.0
/// │   ├── c-2.0.0
/// │   └── c-3.0.0
/// ├── cleaver
/// │   ├── cleaver-2.0.0
/// │   │   ├── requires a
/// │   │   │   └── satisfied by a-1.0.0
/// │   │   ├── requires b
/// │   │   │   └── satisfied by b-1.0.0
/// │   │   ├── requires unrelated-dep==1; sys_platform == "linux"
/// │   │   │   └── satisfied by unrelated-dep-1.0.0
/// │   │   └── requires unrelated-dep==2; sys_platform != "linux"
/// │   │       └── satisfied by unrelated-dep-2.0.0
/// │   └── cleaver-1.0.0
/// │       ├── requires bar==1; sys_platform != "linux"
/// │       │   └── satisfied by bar-1.0.0
/// │       └── requires foo==1; sys_platform == "linux"
/// │           └── satisfied by foo-1.0.0
/// ├── d
/// │   └── d-1.0.0
/// │       └── requires c!=2
/// │           ├── satisfied by c-1.0.0
/// │           └── satisfied by c-3.0.0
/// ├── foo
/// │   ├── foo-1.0.0
/// │   │   ├── requires c!=3; sys_platform == "linux"
/// │   │   │   ├── satisfied by c-1.0.0
/// │   │   │   └── satisfied by c-2.0.0
/// │   │   ├── requires c!=2; sys_platform != "linux"
/// │   │   │   ├── satisfied by c-1.0.0
/// │   │   │   └── satisfied by c-3.0.0
/// │   │   └── requires reject-cleaver-1
/// │   │       └── satisfied by reject-cleaver-1-1.0.0
/// │   └── foo-2.0.0
/// ├── reject-cleaver-1
/// │   └── reject-cleaver-1-1.0.0
/// │       ├── requires unrelated-dep2==1; sys_platform == "linux"
/// │       │   └── satisfied by unrelated-dep2-1.0.0
/// │       └── requires unrelated-dep2==2; sys_platform != "linux"
/// │           └── satisfied by unrelated-dep2-2.0.0
/// ├── reject-cleaver-2
/// │   └── reject-cleaver-2-1.0.0
/// │       └── requires unrelated-dep3==3
/// │           └── satisfied by unrelated-dep3-3.0.0
/// ├── unrelated-dep
/// │   ├── unrelated-dep-1.0.0
/// │   ├── unrelated-dep-2.0.0
/// │   └── unrelated-dep-3.0.0
/// ├── unrelated-dep2
/// │   ├── unrelated-dep2-1.0.0
/// │   ├── unrelated-dep2-2.0.0
/// │   └── unrelated-dep2-3.0.0
/// └── unrelated-dep3
///     ├── unrelated-dep3-1.0.0
///     ├── unrelated-dep3-2.0.0
///     └── unrelated-dep3-3.0.0
/// ```
#[test]
fn preferences_dependent_forking_tristable() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/preferences-dependent-forking-tristable.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''cleaver''',
          '''foo''',
          '''bar''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 11 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'linux'",
            "sys_platform != 'linux'",
        ]

        [[package]]
        name = "bar"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform != 'linux'",
        ]
        dependencies = [
            { name = "d", marker = "sys_platform != 'linux'" },
            { name = "reject-cleaver-1", marker = "sys_platform != 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/bar-1.0.0.tar.gz", hash = "sha256:4151678d8f7377c94432c9ed27788e552879325b149fdb3b18618d5630ee15f7" }
        wheels = [
            { url = "http://[LOCALHOST]/files/bar-1.0.0-py3-none-any.whl", hash = "sha256:dd66d3fe1b685138f4bf865e8434cd9884828ab4e8493142a71c8ba8c779f6d0" },
        ]

        [[package]]
        name = "bar"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/bar-2.0.0.tar.gz", hash = "sha256:eedc005a3c6d9bd8c2677125d0724958350264ad7f010f5b141ba2c48a4e536b" }
        wheels = [
            { url = "http://[LOCALHOST]/files/bar-2.0.0-py3-none-any.whl", hash = "sha256:c452f4a8f924b1c86b9b70b6f75eaf724f2a611e62027c7886fcc502738298e7" },
        ]

        [[package]]
        name = "c"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/c-2.0.0.tar.gz", hash = "sha256:37a07411ed3ed6b9cb796ae510bea299cb271a38fc9c64763a2efc920625a5e5" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-2.0.0-py3-none-any.whl", hash = "sha256:44d2299422cb766c80a8fc55892d1d73c090481641e084936ed993baa128b7f9" },
        ]

        [[package]]
        name = "c"
        version = "3.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform != 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/c-3.0.0.tar.gz", hash = "sha256:db45dd7065945d754e2a1d93241f4f644eee060b14b8415b425bd1ef3fd65287" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-3.0.0-py3-none-any.whl", hash = "sha256:5c404b05ef2b47ae36b90df54b991f5c346248c313b092033b7a8df8e0bc5ba5" },
        ]

        [[package]]
        name = "cleaver"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "bar", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
            { name = "foo", marker = "sys_platform == 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/cleaver-1.0.0.tar.gz", hash = "sha256:98193466000a0b94d44ba4be48753fab4120c6e175ca9e641156aa4a198aa9c7" }
        wheels = [
            { url = "http://[LOCALHOST]/files/cleaver-1.0.0-py3-none-any.whl", hash = "sha256:ece9315741c7079295f9e72a8da6c926b1b6fdaceb0ee8eafcbd63d8a27ba07d" },
        ]

        [[package]]
        name = "d"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", version = "3.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/d-1.0.0.tar.gz", hash = "sha256:90531d7d09ed078587257b6bd0a0438b680752e8dae1d61458d3e3d62312d8b5" }
        wheels = [
            { url = "http://[LOCALHOST]/files/d-1.0.0-py3-none-any.whl", hash = "sha256:b9d54684d9e7218a3daa50d91064153fb9534831d483d4bcb20aa87a7a108161" },
        ]

        [[package]]
        name = "foo"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "c", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "c", version = "3.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
            { name = "reject-cleaver-1" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/foo-1.0.0.tar.gz", hash = "sha256:454a12781f68d01a574c127a16980bdab69c7950c167587ec9ed71718cd33b8b" }
        wheels = [
            { url = "http://[LOCALHOST]/files/foo-1.0.0-py3-none-any.whl", hash = "sha256:adcb65f6437f2298d103ce24f32a85ab827687f33c8b27ff9757ea217351b81e" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "bar", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
            { name = "bar", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "cleaver" },
            { name = "foo" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "bar" },
            { name = "cleaver" },
            { name = "foo" },
        ]

        [[package]]
        name = "reject-cleaver-1"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "unrelated-dep2", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "unrelated-dep2", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/reject_cleaver_1-1.0.0.tar.gz", hash = "sha256:9765fcc0deac35004cec02298aaf6e55a5b87946d20cd85c7d7fb87c20c0fc5d" }
        wheels = [
            { url = "http://[LOCALHOST]/files/reject_cleaver_1-1.0.0-py3-none-any.whl", hash = "sha256:cd525e2507a4fc65c10fab8bab56bc053a00bbf9d6750a9c23197d42aadf636a" },
        ]

        [[package]]
        name = "unrelated-dep2"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/unrelated_dep2-1.0.0.tar.gz", hash = "sha256:781e96f7046b120770fa678a14ba5aad4fb3eaf2265d0a96a6445e100e11b921" }
        wheels = [
            { url = "http://[LOCALHOST]/files/unrelated_dep2-1.0.0-py3-none-any.whl", hash = "sha256:b99d2adcf733f52581011bed0d9f0effdb7824548476a847a7a054b14f46bce5" },
        ]

        [[package]]
        name = "unrelated-dep2"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform != 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/unrelated_dep2-2.0.0.tar.gz", hash = "sha256:3172315b97d250b7b5bba5f601810c2d8517c005caf97f2f23602408cc988ee8" }
        wheels = [
            { url = "http://[LOCALHOST]/files/unrelated_dep2-2.0.0-py3-none-any.whl", hash = "sha256:18d4f9dd426cd0652d3cb1080caeee0166b8c5b4e03249e9f87293d7858cd961" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This test contains a scenario where the solution depends on whether we fork, and whether we fork depends on the
/// preferences.
///
/// Consider a fresh run without preferences:
/// * We start with cleaver 2
/// * We fork
/// * We reject cleaver 2
/// * We find cleaver solution in fork 1 with foo 2 with bar 1
/// * We find cleaver solution in fork 2 with foo 1 with bar 2
/// * We write cleaver 1, foo 1, foo 2, bar 1 and bar 2 to the lockfile
///
/// In a subsequent run, we read the preference cleaver 1 from the lockfile (the preferences for foo and bar don't matter):
/// * We start with cleaver 1
/// * We're in universal mode, we resolve foo 1 and bar 1
/// * We write cleaver 1 and bar 1 to the lockfile
///
/// We call a resolution that's different on the second run to the first unstable.
///
/// Design sketch:
/// ```text
/// root -> clear, foo, bar
/// # Cause a fork, then forget that version.
/// cleaver 2 -> unrelated-dep==1; fork==1
/// cleaver 2 -> unrelated-dep==2; fork==2
/// cleaver 2 -> reject-cleaver-2
/// # Allow different versions when forking, but force foo 1, bar 1 in universal mode without forking.
/// cleaver 1 -> foo==1; fork==1
/// cleaver 1 -> bar==1; fork==2
/// ```
///
/// ```text
/// preferences-dependent-forking
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires bar
/// │   │   ├── satisfied by bar-1.0.0
/// │   │   └── satisfied by bar-2.0.0
/// │   ├── requires cleaver
/// │   │   ├── satisfied by cleaver-2.0.0
/// │   │   └── satisfied by cleaver-1.0.0
/// │   └── requires foo
/// │       ├── satisfied by foo-1.0.0
/// │       └── satisfied by foo-2.0.0
/// ├── bar
/// │   ├── bar-1.0.0
/// │   └── bar-2.0.0
/// ├── cleaver
/// │   ├── cleaver-2.0.0
/// │   │   ├── requires reject-cleaver-2
/// │   │   │   └── satisfied by reject-cleaver-2-1.0.0
/// │   │   ├── requires unrelated-dep==1; sys_platform == "linux"
/// │   │   │   └── satisfied by unrelated-dep-1.0.0
/// │   │   └── requires unrelated-dep==2; sys_platform != "linux"
/// │   │       └── satisfied by unrelated-dep-2.0.0
/// │   └── cleaver-1.0.0
/// │       ├── requires bar==1; sys_platform != "linux"
/// │       │   └── satisfied by bar-1.0.0
/// │       └── requires foo==1; sys_platform == "linux"
/// │           └── satisfied by foo-1.0.0
/// ├── foo
/// │   ├── foo-1.0.0
/// │   └── foo-2.0.0
/// ├── reject-cleaver-2
/// │   └── reject-cleaver-2-1.0.0
/// │       └── requires unrelated-dep==3
/// │           └── satisfied by unrelated-dep-3.0.0
/// └── unrelated-dep
///     ├── unrelated-dep-1.0.0
///     ├── unrelated-dep-2.0.0
///     └── unrelated-dep-3.0.0
/// ```
#[test]
fn preferences_dependent_forking() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/preferences-dependent-forking.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''cleaver''',
          '''foo''',
          '''bar''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "sys_platform == 'linux'",
            "sys_platform != 'linux'",
        ]

        [[package]]
        name = "bar"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform != 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/bar-1.0.0.tar.gz", hash = "sha256:b1eb94dbefaf3aadbea9d91c638802767c7f82daa1950db752b10eb9916fcd10" }
        wheels = [
            { url = "http://[LOCALHOST]/files/bar-1.0.0-py3-none-any.whl", hash = "sha256:097aa87344f94bd8955a8ecf76cfc7bcd5b4c0d40c6528f1f43bff7108b5421d" },
        ]

        [[package]]
        name = "bar"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'linux'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/bar-2.0.0.tar.gz", hash = "sha256:eedc005a3c6d9bd8c2677125d0724958350264ad7f010f5b141ba2c48a4e536b" }
        wheels = [
            { url = "http://[LOCALHOST]/files/bar-2.0.0-py3-none-any.whl", hash = "sha256:c452f4a8f924b1c86b9b70b6f75eaf724f2a611e62027c7886fcc502738298e7" },
        ]

        [[package]]
        name = "cleaver"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "bar", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
            { name = "foo", marker = "sys_platform == 'linux'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/cleaver-1.0.0.tar.gz", hash = "sha256:98193466000a0b94d44ba4be48753fab4120c6e175ca9e641156aa4a198aa9c7" }
        wheels = [
            { url = "http://[LOCALHOST]/files/cleaver-1.0.0-py3-none-any.whl", hash = "sha256:ece9315741c7079295f9e72a8da6c926b1b6fdaceb0ee8eafcbd63d8a27ba07d" },
        ]

        [[package]]
        name = "foo"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/foo-1.0.0.tar.gz", hash = "sha256:14f7c4a961a74f6877aaa920d30e19d5545fc540152a3595bd4eef2b3edd9450" }
        wheels = [
            { url = "http://[LOCALHOST]/files/foo-1.0.0-py3-none-any.whl", hash = "sha256:5cbb733199a4e568066e20a70f9ce37045abbeea78c572409e3de04ffa3d6d88" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "bar", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform != 'linux'" },
            { name = "bar", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'linux'" },
            { name = "cleaver" },
            { name = "foo" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "bar" },
            { name = "cleaver" },
            { name = "foo" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This scenario tries to check that the "remaining universe" handling in
/// the universal resolver is correct. Namely, whenever we create forks
/// from disjoint markers that don't union to the universe, we need to
/// create *another* fork corresponding to the difference between the
/// universe and the union of the forks.
///
/// But when we do this, that remaining universe fork needs to be created
/// like any other fork: it should start copying whatever set of forks
/// existed by the time we got to this point, intersecting the markers with
/// the markers describing the remaining universe and then filtering out
/// any dependencies that are disjoint with the resulting markers.
///
/// This test exercises that logic by ensuring that a package `z` in the
/// remaining universe is excluded based on the combination of markers
/// from a parent fork. That is, if the remaining universe fork does not
/// pick up the markers from the parent forks, then `z` would be included
/// because the remaining universe for _just_ the `b` dependencies of `a`
/// is `os_name != 'linux' and os_name != 'darwin'`, which is satisfied by
/// `z`'s marker of `sys_platform == 'windows'`. However, `a 1.0.0` is only
/// selected in the context of `a < 2 ; sys_platform == 'illumos'`, so `z`
/// should never appear in the resolution.
///
/// ```text
/// fork-remaining-universe-partitioning
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a>=2; sys_platform == "windows"
/// │   │   └── satisfied by a-2.0.0
/// │   └── requires a<2; sys_platform == "illumos"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   ├── requires b>=2; os_name == "linux"
/// │   │   │   └── satisfied by b-2.0.0
/// │   │   ├── requires b<2; os_name == "darwin"
/// │   │   │   └── satisfied by b-1.0.0
/// │   │   └── requires z; sys_platform == "windows"
/// │   │       └── satisfied by z-1.0.0
/// │   └── a-2.0.0
/// ├── b
/// │   ├── b-1.0.0
/// │   └── b-2.0.0
/// └── z
///     └── z-1.0.0
/// ```
#[test]
fn fork_remaining_universe_partitioning() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/remaining-universe-partitioning.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a>=2 ; sys_platform == 'windows'''',
          '''a<2 ; sys_platform == 'illumos'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "os_name == 'darwin' and sys_platform == 'illumos'",
            "os_name == 'linux' and sys_platform == 'illumos'",
            "os_name != 'darwin' and os_name != 'linux' and sys_platform == 'illumos'",
            "sys_platform == 'windows'",
            "sys_platform != 'illumos' and sys_platform != 'windows'",
        ]

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "os_name == 'darwin' and sys_platform == 'illumos'",
            "os_name == 'linux' and sys_platform == 'illumos'",
            "os_name != 'darwin' and os_name != 'linux' and sys_platform == 'illumos'",
        ]
        dependencies = [
            { name = "b", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "os_name == 'darwin' and sys_platform == 'illumos'" },
            { name = "b", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "os_name == 'linux' and sys_platform == 'illumos'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:f70b33a09ae5a06017cbb7742a5c08f8377fdbba1e6c5bb19b4e2de2fbe5f4d2" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:908ee99b22087aca0347e3e797f7536d30ae10b5520172d217783dde6ab38a8b" },
        ]

        [[package]]
        name = "a"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "sys_platform == 'windows'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-2.0.0.tar.gz", hash = "sha256:b5d23816137e4a895c5fdc25c482cc192a6e976397dad1826ce6969997ef2cd6" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-2.0.0-py3-none-any.whl", hash = "sha256:d1d3f7fdfd025cb4ac8f19f39a58d97b6daa7a0cdf66a328757ac1003f769a6b" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "os_name == 'darwin' and sys_platform == 'illumos'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:444108175a344c7a5c435b365246b1460e85f8243b9da7143de631c88fe649b0" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:f400cd1e5f2cf257b093df1df6ef025412f92e22a8dc275f5df4ea43b0620974" },
        ]

        [[package]]
        name = "b"
        version = "2.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        resolution-markers = [
            "os_name == 'linux' and sys_platform == 'illumos'",
        ]
        sdist = { url = "http://[LOCALHOST]/files/b-2.0.0.tar.gz", hash = "sha256:55f2897a25930102575ec735a42948a3f8e62169de5960fbf79ecfc7cf72c002" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-2.0.0-py3-none-any.whl", hash = "sha256:3d4da1cbab02b8c7b731955206703409e616461cc399c827644b74bcd510a7b7" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", version = "1.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'illumos'" },
            { name = "a", version = "2.0.0", source = { registry = "http://[LOCALHOST]/simple/" }, marker = "sys_platform == 'windows'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'illumos'", specifier = "<2" },
            { name = "a", marker = "sys_platform == 'windows'", specifier = ">=2" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This tests that a `Requires-Python` specifier will result in the
/// exclusion of dependency specifications that cannot possibly satisfy it.
///
/// In particular, this is tested via the `python_full_version` marker with
/// a pre-release version.
///
/// ```text
/// fork-requires-python-full-prerelease
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires a==1.0.0; python_full_version == "3.9b1"
/// │       └── satisfied by a-1.0.0
/// └── a
///     └── a-1.0.0
/// ```
#[test]
fn fork_requires_python_full_prerelease() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/requires-python-full-prerelease.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0 ; python_full_version == '3.9b1'''',
        ]
        requires-python = ">=3.10"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.10"

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }

        [package.metadata]
        requires-dist = [{ name = "a", marker = "python_full_version == '3.9'", specifier = "==1.0.0" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This tests that a `Requires-Python` specifier will result in the
/// exclusion of dependency specifications that cannot possibly satisfy it.
///
/// In particular, this is tested via the `python_full_version` marker
/// instead of the more common `python_version` marker.
///
/// ```text
/// fork-requires-python-full
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires a==1.0.0; python_full_version == "3.9"
/// │       └── satisfied by a-1.0.0
/// └── a
///     └── a-1.0.0
/// ```
#[test]
fn fork_requires_python_full() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/requires-python-full.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0 ; python_full_version == '3.9'''',
        ]
        requires-python = ">=3.10"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.10"

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }

        [package.metadata]
        requires-dist = [{ name = "a", marker = "python_full_version == '3.9'", specifier = "==1.0.0" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This tests that a `Requires-Python` specifier that includes a Python
/// patch version will not result in excluded a dependency specification
/// with a `python_version == '3.10'` marker.
///
/// This is a regression test for the universal resolver where it would
/// convert a `Requires-Python: >=3.10.1` specifier into a
/// `python_version >= '3.10.1'` marker expression, which would be
/// considered disjoint with `python_version == '3.10'`. Thus, the
/// dependency `a` below was erroneously excluded. It should be included.
///
/// ```text
/// fork-requires-python-patch-overlap
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires a==1.0.0; python_version == "3.10"
/// │       └── satisfied by a-1.0.0
/// └── a
///     └── a-1.0.0
///         └── requires python>=3.10
/// ```
#[test]
fn fork_requires_python_patch_overlap() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/requires-python-patch-overlap.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0 ; python_version == '3.10'''',
        ]
        requires-python = ">=3.10.1"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.10.1"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:6be143220747f2153423e58696f7f279b095232c3eefef94872a5c2ebc4ef3a0" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:a2f2d5279ab96537eff29d380ec69c12b31ad5b7ede3e21426c2c2046bca0a93" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", marker = "python_full_version < '3.11'" },
        ]

        [package.metadata]
        requires-dist = [{ name = "a", marker = "python_full_version == '3.10.*'", specifier = "==1.0.0" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// This tests that a `Requires-Python` specifier will result in the
/// exclusion of dependency specifications that cannot possibly satisfy it.
///
/// ```text
/// fork-requires-python
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires a==1.0.0; python_version == "3.9"
/// │       └── satisfied by a-1.0.0
/// └── a
///     └── a-1.0.0
/// ```
#[test]
fn fork_requires_python() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("fork/requires-python.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0 ; python_version == '3.9'''',
        ]
        requires-python = ">=3.10"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 1 package in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.10"

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }

        [package.metadata]
        requires-dist = [{ name = "a", marker = "python_full_version == '3.9.*'", specifier = "==1.0.0" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// Check that we only include wheels that match the required Python version
///
/// ```text
/// requires-python-wheels
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires a==1.0.0
/// │       └── satisfied by a-1.0.0
/// └── a
///     └── a-1.0.0
///         └── requires python>=3.10
/// ```
#[test]
fn requires_python_wheels() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("tag_and_markers/requires-python-wheels.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0''',
        ]
        requires-python = ">=3.10"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.10"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:6be143220747f2153423e58696f7f279b095232c3eefef94872a5c2ebc4ef3a0" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-cp310-cp310-any.whl", hash = "sha256:a2b097267bd9b8037256d7081be26d93726408da008af7ce290b4e5bb5211005" },
            { url = "http://[LOCALHOST]/files/a-1.0.0-cp311-cp311-any.whl", hash = "sha256:ccb40857ba58482ae092494854b1e48335ebec555ab021cefdb7baeb2e21e029" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
        ]

        [package.metadata]
        requires-dist = [{ name = "a", specifier = "==1.0.0" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// `c` is not reachable due to the markers, it should be excluded from the lockfile
///
/// ```text
/// unreachable-package
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires a==1.0.0; sys_platform == "win32"
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   └── a-1.0.0
/// │       └── requires b==1.0.0; sys_platform == "linux"
/// │           └── satisfied by b-1.0.0
/// └── b
///     └── b-1.0.0
/// ```
#[test]
fn unreachable_package() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("tag_and_markers/unreachable-package.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0; sys_platform == 'win32'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:1793c8e1a459466196c97d0663a1f6d588b66037f9430f34a70a10550424cdbf" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:67dfcbda3a82fbabf4dd86009e05691f1d52512246502ccc49703fce4d789729" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", marker = "sys_platform == 'win32'" },
        ]

        [package.metadata]
        requires-dist = [{ name = "a", marker = "sys_platform == 'win32'", specifier = "==1.0.0" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// Check that we only include wheels that match the platform markers
///
/// ```text
/// unreachable-wheels
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==1.0.0; sys_platform == "win32"
/// │   │   └── satisfied by a-1.0.0
/// │   ├── requires b==1.0.0; sys_platform == "linux"
/// │   │   └── satisfied by b-1.0.0
/// │   └── requires c==1.0.0; sys_platform == "darwin"
/// │       └── satisfied by c-1.0.0
/// ├── a
/// │   └── a-1.0.0
/// ├── b
/// │   └── b-1.0.0
/// └── c
///     └── c-1.0.0
/// ```
#[test]
fn unreachable_wheels() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("tag_and_markers/unreachable-wheels.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1.0.0; sys_platform == 'win32'''',
          '''b==1.0.0; sys_platform == 'linux'''',
          '''c==1.0.0; sys_platform == 'darwin'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:10eb37b5137fa0770faf91f55610eb2555ff2b6d6b0fe56c53ea9374173ea099" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-cp312-cp312-win_amd64.whl", hash = "sha256:3f0fb3bc62ccf01a29e5d737a6ac5fa1403e3c449cec2569801d145953898699" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:444108175a344c7a5c435b365246b1460e85f8243b9da7143de631c88fe649b0" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-cp312-cp312-manylinux_2_17_x86_64.manylinux2014_x86_64.whl", hash = "sha256:dde7100a02b9c71f3d1657ffded665195de7b8558f69284bf23b36a610fa9a6c" },
            { url = "http://[LOCALHOST]/files/b-1.0.0-cp312-cp312-musllinux_1_1_armv7l.whl", hash = "sha256:68948176f3dceb40718502540b3be37c8ac355eb1a12e69a7d8327c9c565bdba" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/c-1.0.0.tar.gz", hash = "sha256:1a0dc3013c4de679411df70712ff3a4cd23b873fff1ee8ac1f7f57630bb74f86" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-cp312-cp312-macosx_14_0_x86_64.whl", hash = "sha256:656075d61958336816380a5c5527d7dbeab759794a1a20f860f7557bb2888751" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a", marker = "sys_platform == 'win32'" },
            { name = "b", marker = "sys_platform == 'linux'" },
            { name = "c", marker = "sys_platform == 'darwin'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "sys_platform == 'win32'", specifier = "==1.0.0" },
            { name = "b", marker = "sys_platform == 'linux'", specifier = "==1.0.0" },
            { name = "c", marker = "sys_platform == 'darwin'", specifier = "==1.0.0" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// Check the prioritization for virtual extra and marker packages
///
/// ```text
/// marker-variants-have-different-extras
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires psycopg[binary]; platform_python_implementation != "PyPy"
/// │   │   ├── satisfied by psycopg-1.0.0
/// │   │   └── satisfied by psycopg-1.0.0[binary]
/// │   └── requires psycopg; platform_python_implementation == "PyPy"
/// │       ├── satisfied by psycopg-1.0.0
/// │       └── satisfied by psycopg-1.0.0[binary]
/// ├── psycopg
/// │   ├── psycopg-1.0.0
/// │   │   └── requires tzdata; sys_platform == "win32"
/// │   │       └── satisfied by tzdata-1.0.0
/// │   └── psycopg-1.0.0[binary]
/// │       └── requires psycopg-binary; implementation_name != "pypy"
/// │           └── satisfied by psycopg-binary-1.0.0
/// ├── psycopg-binary
/// │   └── psycopg-binary-1.0.0
/// └── tzdata
///     └── tzdata-1.0.0
/// ```
#[test]
fn marker_variants_have_different_extras() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("tag_and_markers/virtual-package-extra-priorities.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''psycopg[binary] ; platform_python_implementation != 'PyPy'''',
          '''psycopg ; platform_python_implementation == 'PyPy'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 4 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        resolution-markers = [
            "platform_python_implementation != 'PyPy'",
            "platform_python_implementation == 'PyPy'",
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "psycopg" },
            { name = "psycopg", extra = ["binary"], marker = "platform_python_implementation != 'PyPy'" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "psycopg", marker = "platform_python_implementation == 'PyPy'" },
            { name = "psycopg", extras = ["binary"], marker = "platform_python_implementation != 'PyPy'" },
        ]

        [[package]]
        name = "psycopg"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "tzdata", marker = "sys_platform == 'win32'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/psycopg-1.0.0.tar.gz", hash = "sha256:d46769e8d414b1cc3ca04740d42be1df8535b387b9266d6ae4ad7ffbd514d08a" }
        wheels = [
            { url = "http://[LOCALHOST]/files/psycopg-1.0.0-py3-none-any.whl", hash = "sha256:ca8442f7460d8cfba2e205a4048afa2ddbe6f5aa1e101c91794e403814ed8f18" },
        ]

        [package.optional-dependencies]
        binary = [
            { name = "psycopg-binary", marker = "implementation_name != 'pypy' and platform_python_implementation != 'PyPy'" },
        ]

        [[package]]
        name = "psycopg-binary"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/psycopg_binary-1.0.0.tar.gz", hash = "sha256:ab34aa0d2cb6cdf38c46832556ece754d940a7699981914aecc31ed96edb2ebe" }
        wheels = [
            { url = "http://[LOCALHOST]/files/psycopg_binary-1.0.0-py3-none-any.whl", hash = "sha256:5ed5c025e24ce3b2e31c5930a729fed9625fc9919c6fb2b798344cd93fc72b5a" },
        ]

        [[package]]
        name = "tzdata"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/tzdata-1.0.0.tar.gz", hash = "sha256:6d0474e2abdf2c801bad4b96eecc4d0ed484832199a3eb813b70890de0737774" }
        wheels = [
            { url = "http://[LOCALHOST]/files/tzdata-1.0.0-py3-none-any.whl", hash = "sha256:1d0f0c1a6fbb1695d93dd9cae2b1991c12eca49e3c3362c6fd65be5250871777" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// Check the prioritization for virtual marker packages
///
/// ```text
/// virtual-package-extra-priorities
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   ├── requires a==1; python_version >= "3.8"
/// │   │   └── satisfied by a-1.0.0
/// │   └── requires b; python_version >= "3.9"
/// │       ├── satisfied by b-1.0.0
/// │       └── satisfied by b-2.0.0
/// ├── a
/// │   ├── a-1.0.0
/// │   │   └── requires b==1; python_version >= "3.10"
/// │   │       └── satisfied by b-1.0.0
/// │   └── a-2.0.0
/// │       └── requires b==1; python_version >= "3.10"
/// │           └── satisfied by b-1.0.0
/// └── b
///     ├── b-1.0.0
///     └── b-2.0.0
/// ```
#[test]
fn virtual_package_extra_priorities() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("tag_and_markers/virtual-package-marker-priorities.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a==1; python_version >= '3.8'''',
          '''b; python_version >= '3.9'''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 3 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "b" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:e8ab556fd01bcf15f43d98d90cae81822cd97b5bc8128c72d4b3ee6a61023f60" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:9629fefaceab9ae432a078ffc1e4950b6ed8895f23305aa9ab2b8ce16eeeb6a4" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        sdist = { url = "http://[LOCALHOST]/files/b-1.0.0.tar.gz", hash = "sha256:444108175a344c7a5c435b365246b1460e85f8243b9da7143de631c88fe649b0" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-py3-none-any.whl", hash = "sha256:f400cd1e5f2cf257b093df1df6ef025412f92e22a8dc275f5df4ea43b0620974" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
            { name = "b" },
        ]

        [package.metadata]
        requires-dist = [
            { name = "a", marker = "python_full_version >= '3.8'", specifier = "==1" },
            { name = "b", marker = "python_full_version >= '3.9'" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}
/// While both Linux and Windows are required and `win-only` has only a Windows wheel, `win-only` is also used only on Windows.
///
/// ```text
/// requires-python-subset
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires win-only; sys_platform == "win32"
/// │       └── satisfied by win-only-1.0.0
/// └── win-only
///     └── win-only-1.0.0
/// ```
#[test]
fn requires_python_subset() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("wheels/requires-python-subset.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''win-only; sys_platform == 'win32'''',
        ]
        requires-python = ">=3.12"
        [tool.uv]
        required-environments = [
          '''sys_platform == "linux"''',
          '''sys_platform == "win32"''',
        ]
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 2 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"
        required-markers = [
            "sys_platform == 'linux'",
            "sys_platform == 'win32'",
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "win-only", marker = "sys_platform == 'win32'" },
        ]

        [package.metadata]
        requires-dist = [{ name = "win-only", marker = "sys_platform == 'win32'" }]

        [[package]]
        name = "win-only"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        wheels = [
            { url = "http://[LOCALHOST]/files/win_only-1.0.0-cp312-abi3-win_amd64.whl", hash = "sha256:859843f5bfcc49626980a2ad685ebf93ccfd33452d942513c4fd53659024763a" },
        ]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}

/// When a dependency is only required on a specific platform (like x86_64), omit wheels that target other platforms (like aarch64).
///
/// ```text
/// specific-architecture
/// ├── environment
/// │   └── python3.12
/// ├── root
/// │   └── requires a
/// │       └── satisfied by a-1.0.0
/// ├── a
/// │   └── a-1.0.0
/// │       ├── requires b; platform_machine == "x86_64"
/// │       │   └── satisfied by b-1.0.0
/// │       ├── requires c; platform_machine == "aarch64"
/// │       │   └── satisfied by c-1.0.0
/// │       └── requires d; platform_machine == "i686"
/// │           └── satisfied by d-1.0.0
/// ├── b
/// │   └── b-1.0.0
/// ├── c
/// │   └── c-1.0.0
/// └── d
///     └── d-1.0.0
/// ```
#[test]
fn specific_architecture() -> Result<()> {
    let context = uv_test::test_context!("3.12");
    let server = PackseServer::new("wheels/specific-architecture.toml");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r###"
        [project]
        name = "project"
        version = "0.1.0"
        dependencies = [
          '''a''',
        ]
        requires-python = ">=3.12"
        "###,
    )?;

    let mut filters = context.filters();
    // The "hint" about non-current environments is platform-dependent, so filter it out.
    filters.push((r"\n\s+hint: .*", ""));

    let mut cmd = context.lock();
    cmd.env_remove(EnvVars::UV_EXCLUDE_NEWER);
    cmd.arg("--index-url").arg(server.index_url());
    uv_snapshot!(filters, cmd, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved 5 packages in [TIME]
    "
    );

    let lock = context.read("uv.lock");
    insta::with_settings!({
        filters => filters,
    }, {
        assert_snapshot!(
            lock, @r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [[package]]
        name = "a"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        dependencies = [
            { name = "b", marker = "platform_machine == 'x86_64'" },
            { name = "c", marker = "platform_machine == 'aarch64'" },
            { name = "d", marker = "platform_machine == 'i686'" },
        ]
        sdist = { url = "http://[LOCALHOST]/files/a-1.0.0.tar.gz", hash = "sha256:d0ce2948885c68c5629c3506325cb99eb092b0905a32d06561ea95447d27b370" }
        wheels = [
            { url = "http://[LOCALHOST]/files/a-1.0.0-py3-none-any.whl", hash = "sha256:6ad7e0e4d86bd30a78ed026332dd8049683faf5accadb4eae6e0579573acc22e" },
        ]

        [[package]]
        name = "b"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        wheels = [
            { url = "http://[LOCALHOST]/files/b-1.0.0-cp313-cp313-freebsd_13_aarch64.whl", hash = "sha256:414c9cc09c5ae9ead899fefcad4a0a478b56cf66ceb4d6d1bd39e0d1d14000d7" },
            { url = "http://[LOCALHOST]/files/b-1.0.0-cp313-cp313-freebsd_13_x86_64.whl", hash = "sha256:f4574728c30ea7854065894c92fd0b83cec2b82accbba0643e91893cf6c08604" },
            { url = "http://[LOCALHOST]/files/b-1.0.0-cp313-cp313-macosx_10_9_x86_64.whl", hash = "sha256:5e6f900122ce6b89d9aec3c6b1a070fcdef4de57f204a3a7c63def7f1de3f407" },
            { url = "http://[LOCALHOST]/files/b-1.0.0-cp313-cp313-manylinux2010_x86_64.whl", hash = "sha256:c919faf2b0ab8d638e2213ae10369295b52eca2389e24164e67238c701605f1b" },
        ]

        [[package]]
        name = "c"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        wheels = [
            { url = "http://[LOCALHOST]/files/c-1.0.0-cp313-cp313-freebsd_13_aarch64.whl", hash = "sha256:946c0a8289c4b416d96b921be7aa9a17156a8823c62c5d0c30785a125ca1a22c" },
            { url = "http://[LOCALHOST]/files/c-1.0.0-cp313-cp313-freebsd_13_x86_64.whl", hash = "sha256:17d3a7ad0680bc577b774d3176339428023cb7dde3a134cdd149bb42b087450f" },
            { url = "http://[LOCALHOST]/files/c-1.0.0-cp313-cp313-macosx_10_9_arm64.whl", hash = "sha256:0349c5260b13abed37831fa7d42ce9d178d75f3a40ccaa1245fcf51a6561489a" },
            { url = "http://[LOCALHOST]/files/c-1.0.0-cp313-cp313-manylinux2010_aarch64.whl", hash = "sha256:60ebf9938e327cc2a0344cea50d6a9ed42723d7e1659f768141c923daf89150c" },
        ]

        [[package]]
        name = "d"
        version = "1.0.0"
        source = { registry = "http://[LOCALHOST]/simple/" }
        wheels = [
            { url = "http://[LOCALHOST]/files/d-1.0.0-cp313-cp313-freebsd_13_aarch64.whl", hash = "sha256:0ef97a69826697d506e5cedfc80da19f4dc12dc9f700da4b97b94160b7f3b150" },
            { url = "http://[LOCALHOST]/files/d-1.0.0-cp313-cp313-freebsd_13_x86_64.whl", hash = "sha256:835432e4c5d823fc63eed3ef25a8153a44c01d996e5cba061b19e315271832f3" },
            { url = "http://[LOCALHOST]/files/d-1.0.0-cp313-cp313-manylinux2010_i686.whl", hash = "sha256:f8dbb12bb95d90cac2b9dbb8adc7f015a33fafc28f8765121ea3a1086d3e46b2" },
        ]

        [[package]]
        name = "project"
        version = "0.1.0"
        source = { virtual = "." }
        dependencies = [
            { name = "a" },
        ]

        [package.metadata]
        requires-dist = [{ name = "a" }]
        "#
        );
    });

    // Assert the idempotence of `uv lock` when resolving from the lockfile (`--locked`).
    context
        .lock()
        .arg("--locked")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("--index-url")
        .arg(server.index_url())
        .assert()
        .success();

    Ok(())
}
