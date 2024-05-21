use std::path::PathBuf;

/// Returns the path to the user configuration directory.
///
/// This is similar to the `config_dir()` returned by the `dirs` crate, but it uses the
/// `XDG_CONFIG_HOME` environment variable on both Linux _and_ macOS, rather than the
/// `Application Support` directory on macOS.
pub fn config_dir() -> Option<PathBuf> {
    // On Windows, use, e.g., C:\Users\Alice\AppData\Roaming
    #[cfg(windows)]
    {
        dirs_sys::known_folder_roaming_app_data()
    }

    // On Linux and macOS, use, e.g., /home/alice/.config.
    #[cfg(not(windows))]
    {
        std::env::var_os("XDG_CONFIG_HOME")
            .and_then(dirs_sys::is_absolute_path)
            .or_else(|| dirs_sys::home_dir().map(|path| path.join(".config")))
    }
}

/// Returns the path to the user data directory.
///
/// This is the same as the `data_dir()` returned by the `dirs` crate.
pub fn user_data_dir() -> Option<PathBuf> {
    dirs::data_dir()
}
