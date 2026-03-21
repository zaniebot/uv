use std::process::Command;

use uv_static::EnvVars;

/// Add shared arguments to a command.
///
/// In particular, remove any user-defined environment variables and set any machine-specific
/// environment variables to static values.
fn add_shared_args(mut command: Command) -> Command {
    command
        .env(EnvVars::UV_LINK_MODE, "clone")
        .env(EnvVars::UV_CONCURRENT_DOWNLOADS, "50")
        .env(EnvVars::UV_CONCURRENT_BUILDS, "16")
        .env(EnvVars::UV_CONCURRENT_INSTALLS, "8")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env_remove(EnvVars::UV_PYTHON_DOWNLOADS);

    if cfg!(unix) {
        // Avoid locale issues in tests
        command.env(EnvVars::LC_ALL, "C");
    }
    command
}

mod config_files;
mod conflicts;
mod index;
mod layering;
mod options;
