use std::env;
use std::fs;

use assert_fs::prelude::*;
use predicates::prelude::*;

use crate::common::{TestContext, uv_snapshot};

#[test]
fn test_self_install_to_custom_bin_dir() {
    let context = TestContext::new("3.12");
    let install_dir = context.temp_dir.child("custom-bin");

    uv_snapshot!(context.filters(), context.self_install().arg("--bin-dir").arg(install_dir.path()),
    @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: Installing uv to [TEMP_DIR]/custom-bin/uv
    success: Installed uv to [TEMP_DIR]/custom-bin/uv
    success: Created configuration file: [HOME]/.zshenv
    info: Restart your shell to apply changes
    ");

    // Check that the binary was installed
    install_dir.child("uv").assert(predicate::path::exists());
    
    // On Unix, check that it's executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(install_dir.child("uv").path()).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o755, 0o755);
    }

    // Note: Shell configuration testing is done in shell-specific tests below
}

#[test]
fn test_self_install_default_location() {
    let context = TestContext::new("3.12");

    // Set XDG_BIN_HOME to control where it installs
    let install_dir = context.temp_dir.child("bin");
    
    uv_snapshot!(context.filters(), context.self_install().env("XDG_BIN_HOME", install_dir.path()),
    @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: Installing uv to [TEMP_DIR]/bin/uv
    success: Installed uv to [TEMP_DIR]/bin/uv
    success: Created configuration file: [HOME]/.zshenv
    info: Restart your shell to apply changes
    ");

    // Check that the binary was installed
    install_dir.child("uv").assert(predicate::path::exists());

    // Note: Shell configuration testing is done in shell-specific tests below
}

#[test]
fn test_self_install_already_at_location() {
    let context = TestContext::new("3.12");
    let install_dir = context.temp_dir.child("already-here");
    install_dir.create_dir_all().unwrap();
    
    // First, copy the uv binary to the target location
    let uv_binary = crate::common::get_bin();
    fs::copy(&uv_binary, install_dir.child("uv").path()).unwrap();
    
    // Try to install again - should detect it's already there
    uv_snapshot!(context.filters(), context.self_install()
        .arg("--bin-dir")
        .arg(install_dir.path())
        .env("PATH", env::var("PATH").unwrap()),
    @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: Installing uv to [TEMP_DIR]/already-here/uv
    success: Installed uv to [TEMP_DIR]/already-here/uv
    success: Created configuration file: [HOME]/.zshenv
    info: Restart your shell to apply changes
    ");

    // Note: Shell configuration testing is done in shell-specific tests below
}

#[test]
fn test_self_install_nonexistent_parent_dir() {
    let context = TestContext::new("3.12");
    let install_dir = context.temp_dir.child("deeply").child("nested").child("bin");
    
    // Should create the parent directories
    uv_snapshot!(context.filters(), context.self_install().arg("--bin-dir").arg(install_dir.path()),
    @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: Installing uv to [TEMP_DIR]/deeply/nested/bin/uv
    success: Installed uv to [TEMP_DIR]/deeply/nested/bin/uv
    success: Created configuration file: [HOME]/.zshenv
    info: Restart your shell to apply changes
    ");

    install_dir.child("uv").assert(predicate::path::exists());

    // Note: Shell configuration testing is done in shell-specific tests below
}

#[test]
fn test_self_install_help() {
    let context = TestContext::new("3.12");
    
    uv_snapshot!(context.filters(), context.self_install().arg("--help"),
    @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Install uv to a specific directory and update PATH

    Usage: uv self install [OPTIONS]

    Options:
          --bin-dir <BIN_DIR>  The directory to install uv into

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory [env: UV_CACHE_DIR=]

    Python options:
          --managed-python       Require use of uv-managed Python versions [env: UV_MANAGED_PYTHON=]
          --no-managed-python    Disable use of uv-managed Python versions [env: UV_NO_MANAGED_PYTHON=]
          --no-python-downloads  Disable automatic downloads of Python. [env:
                                 "UV_PYTHON_DOWNLOADS=never"]

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --native-tls
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_NATIVE_TLS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command
          --project <PROJECT>
              Run the command within the given project directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command

    ----- stderr -----
    "#);
}

// Shell-specific tests with environment variable mocking

#[test]
fn test_self_install_bash_shell() {
    let context = TestContext::new("3.12");
    let install_dir = context.temp_dir.child("bash-bin");

    uv_snapshot!(context.filters(), context.self_install()
        .arg("--bin-dir").arg(install_dir.path())
        .env("BASH_VERSION", "5.0")  // Force bash detection
        .env("SHELL", "/bin/bash"),
    @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: Installing uv to [TEMP_DIR]/bash-bin/uv
    success: Installed uv to [TEMP_DIR]/bash-bin/uv
    success: Created configuration file: [HOME]/.bash_profile
    success: Created configuration file: [HOME]/.bashrc
    info: Restart your shell to apply changes
    ");

    // Check that the binary was installed
    install_dir.child("uv").assert(predicate::path::exists());

    // Check that bash configuration files were created
    let bashrc = context.home_dir.child(".bashrc");
    let bash_profile = context.home_dir.child(".bash_profile");
    bashrc.assert(predicate::path::exists());
    bash_profile.assert(predicate::path::exists());
    
    // Check .bashrc content
    let bashrc_content = fs::read_to_string(bashrc.path()).unwrap();
    insta::with_settings!({
        filters => context.filters(),
    }, {
        insta::assert_snapshot!(bashrc_content, @r###"
        # uv
        export PATH="[TEMP_DIR]/bash-bin:$PATH"
        "###);
    });
    
    // Check .bash_profile content
    let bash_profile_content = fs::read_to_string(bash_profile.path()).unwrap();
    insta::with_settings!({
        filters => context.filters(),
    }, {
        insta::assert_snapshot!(bash_profile_content, @r###"
        # uv
        export PATH="[TEMP_DIR]/bash-bin:$PATH"
        "###);
    });
}

#[test]
fn test_self_install_zsh_shell() {
    let context = TestContext::new("3.12");
    let install_dir = context.temp_dir.child("zsh-bin");

    uv_snapshot!(context.filters(), context.self_install()
        .arg("--bin-dir").arg(install_dir.path())
        .env("SHELL", "/bin/zsh")  // Force zsh detection
        .env_remove("BASH_VERSION") // Remove bash detection
        .env_remove("FISH_VERSION"), // Remove fish detection
    @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: Installing uv to [TEMP_DIR]/zsh-bin/uv
    success: Installed uv to [TEMP_DIR]/zsh-bin/uv
    success: Created configuration file: [HOME]/.zshenv
    info: Restart your shell to apply changes
    ");

    // Check that the binary was installed
    install_dir.child("uv").assert(predicate::path::exists());

    // Check that zsh configuration file was created
    let zshenv = context.home_dir.child(".zshenv");
    zshenv.assert(predicate::path::exists());
    
    let shell_content = fs::read_to_string(zshenv.path()).unwrap();
    insta::with_settings!({
        filters => context.filters(),
    }, {
        insta::assert_snapshot!(shell_content, @r###"
        # uv
        export PATH="[TEMP_DIR]/zsh-bin:$PATH"
        "###);
    });
}

#[test]
fn test_self_install_fish_shell() {
    let context = TestContext::new("3.12");
    let install_dir = context.temp_dir.child("fish-bin");

    uv_snapshot!(context.filters(), context.self_install()
        .arg("--bin-dir").arg(install_dir.path())
        .env("FISH_VERSION", "3.0")  // Force fish detection
        .env_remove("BASH_VERSION") // Remove bash detection
        .env_remove("SHELL"), // Remove shell fallback
    @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: Installing uv to [TEMP_DIR]/fish-bin/uv
    success: Installed uv to [TEMP_DIR]/fish-bin/uv
    success: Created configuration file: [USER_CONFIG_DIR]/fish/config.fish
    info: Restart your shell to apply changes
    ");

    // Check that the binary was installed
    install_dir.child("uv").assert(predicate::path::exists());

    // Check that fish configuration file was created  
    // Fish uses XDG_CONFIG_HOME if set, or ~/.config/fish/config.fish otherwise
    let fish_config = context.home_dir.child(".config/fish/config.fish");
    fish_config.assert(predicate::path::exists());
    
    let shell_content = fs::read_to_string(fish_config.path()).unwrap();
    insta::with_settings!({
        filters => context.filters(),
    }, {
        insta::assert_snapshot!(shell_content, @r###"
        # uv
        fish_add_path "[TEMP_DIR]/fish-bin"
        "###);
    });
}
