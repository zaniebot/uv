pub mod runnable;
mod shlex;
pub mod windows;

pub use shlex::{escape_posix_for_single_quotes, shlex_posix, shlex_windows};

use std::env::home_dir;
use std::path::{Path, PathBuf};

use uv_fs::Simplified;
use uv_static::EnvVars;

#[cfg(unix)]
use tracing::debug;

fn non_empty_env_path(variable: &'static str) -> Option<PathBuf> {
    std::env::var(variable)
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn expand_home_path(path: PathBuf, home_dir: Option<&Path>) -> PathBuf {
    if let Some(home_dir) = home_dir
        && path
            .components()
            .next()
            .map(std::path::Component::as_os_str)
            == Some("~".as_ref())
    {
        home_dir.join(path.components().skip(1).collect::<PathBuf>())
    } else {
        path
    }
}

fn first_existing_path(home_dir: &Path, candidates: &[&str], fallback: &str) -> PathBuf {
    candidates
        .iter()
        .map(|candidate| home_dir.join(candidate))
        .find(|candidate| candidate.is_file())
        .unwrap_or_else(|| home_dir.join(fallback))
}

fn zshenv_path(home_dir: &Path, zsh_dot_dir: Option<&Path>) -> PathBuf {
    if let Some(zsh_dot_dir) = zsh_dot_dir {
        let zshenv = zsh_dot_dir.join(".zshenv");
        if zshenv.is_file() {
            return zshenv;
        }
    }

    let zshenv = home_dir.join(".zshenv");
    if zshenv.is_file() {
        return zshenv;
    }

    zsh_dot_dir
        .map(|zsh_dot_dir| zsh_dot_dir.join(".zshenv"))
        .unwrap_or_else(|| home_dir.join(".zshenv"))
}

/// Shells for which virtualenv activation scripts are available.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[expect(clippy::doc_markdown)]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,
    /// Friendly Interactive SHell (fish)
    Fish,
    /// PowerShell
    Powershell,
    /// Cmd (Command Prompt)
    Cmd,
    /// Z SHell (zsh)
    Zsh,
    /// Nushell
    Nushell,
    /// C SHell (csh)
    Csh,
    /// Korn SHell (ksh)
    Ksh,
}

impl Shell {
    fn from_known_shell_env() -> Option<Self> {
        [
            (EnvVars::NU_VERSION, Self::Nushell),
            (EnvVars::FISH_VERSION, Self::Fish),
            (EnvVars::BASH_VERSION, Self::Bash),
            (EnvVars::ZSH_VERSION, Self::Zsh),
            (EnvVars::KSH_VERSION, Self::Ksh),
            (EnvVars::PS_MODULE_PATH, Self::Powershell),
        ]
        .into_iter()
        .find_map(|(variable, shell)| std::env::var_os(variable).is_some().then_some(shell))
    }

    /// Determine the user's current shell from the environment.
    ///
    /// First checks shell-specific environment variables (`NU_VERSION`, `FISH_VERSION`,
    /// `BASH_VERSION`, `ZSH_VERSION`, `KSH_VERSION`, `PSModulePath`) which are set by the
    /// respective shells. This takes priority over `SHELL` because on Unix, `SHELL` refers
    /// to the user's login shell, not the currently running shell.
    ///
    /// Falls back to parsing the `SHELL` environment variable if no shell-specific variables
    /// are found. On Windows, defaults to PowerShell (or Command Prompt if `PROMPT` is set).
    ///
    /// Returns `None` if the shell cannot be determined.
    pub fn from_env() -> Option<Self> {
        Self::from_known_shell_env()
            .or_else(|| std::env::var_os(EnvVars::SHELL).and_then(Self::from_shell_path))
            .or_else(|| {
                if cfg!(windows) {
                    // Command Prompt relies on PROMPT for its appearance whereas PowerShell does not.
                    // See: https://stackoverflow.com/a/66415037.
                    if std::env::var_os(EnvVars::PROMPT).is_some() {
                        Some(Self::Cmd)
                    } else {
                        // Fallback to PowerShell if the PROMPT environment variable is not set.
                        Some(Self::Powershell)
                    }
                } else {
                    None
                }
            })
            // Fallback to detecting the shell from the parent process.
            .or_else(Self::from_parent_process)
    }

    /// Attempt to determine the shell from the parent process.
    ///
    /// This is a fallback method for when environment variables don't provide
    /// enough information about the current shell. It looks at the parent process
    /// to try to identify which shell is running.
    ///
    /// This method currently only works on Unix-like systems. On other platforms,
    /// it returns `None`.
    fn from_parent_process() -> Option<Self> {
        #[cfg(unix)]
        {
            // Get the parent process ID
            let ppid = nix::unistd::getppid();
            debug!("Detected parent process ID: {ppid}");

            // Try to read the parent process executable path
            let proc_exe_path = format!("/proc/{ppid}/exe");
            if let Ok(exe_path) = fs_err::read_link(&proc_exe_path) {
                debug!("Parent process executable: {}", exe_path.display());
                if let Some(shell) = Self::from_shell_path(&exe_path) {
                    return Some(shell);
                }
            }

            // If reading exe fails, try reading the comm file
            let proc_comm_path = format!("/proc/{ppid}/comm");
            if let Ok(comm) = fs_err::read_to_string(&proc_comm_path) {
                let comm = comm.trim();
                debug!("Parent process comm: {comm}");
                if let Some(shell) = parse_shell_from_path(Path::new(comm)) {
                    return Some(shell);
                }
            }

            debug!("Could not determine shell from parent process");
            None
        }

        #[cfg(not(unix))]
        {
            None
        }
    }

    /// Parse a shell from a path to the executable for the shell.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crate::shells::Shell;
    ///
    /// assert_eq!(Shell::from_shell_path("/bin/bash"), Some(Shell::Bash));
    /// assert_eq!(Shell::from_shell_path("/usr/bin/zsh"), Some(Shell::Zsh));
    /// assert_eq!(Shell::from_shell_path("/opt/my_custom_shell"), None);
    /// ```
    pub fn from_shell_path(path: impl AsRef<Path>) -> Option<Self> {
        parse_shell_from_path(path.as_ref())
    }

    /// Returns `true` if the shell supports a `PATH` update command.
    pub fn supports_update(self) -> bool {
        match self {
            Self::Powershell | Self::Cmd => true,
            shell => !shell.configuration_files().is_empty(),
        }
    }

    /// Return the configuration files that should be modified to append to a shell's `PATH`.
    ///
    /// Some of the logic here is based on rustup's rc file detection.
    ///
    /// See: <https://github.com/rust-lang/rustup/blob/fede22fea7b160868cece632bd213e6d72f8912f/src/cli/self_update/shell.rs#L197>
    pub fn configuration_files(self) -> Vec<PathBuf> {
        let Some(home_dir) = home_dir() else {
            return vec![];
        };
        match self {
            Self::Bash => {
                // On Bash, we need to update both `.bashrc` and `.bash_profile`. The former is
                // sourced for non-login shells, and the latter is sourced for login shells.
                //
                // In lieu of `.bash_profile`, shells will also respect `.bash_login` and
                // `.profile`, if they exist. So we respect those too.
                vec![
                    first_existing_path(
                        &home_dir,
                        &[".bash_profile", ".bash_login", ".profile"],
                        ".bash_profile",
                    ),
                    home_dir.join(".bashrc"),
                ]
            }
            Self::Ksh => {
                // On Ksh it's standard POSIX `.profile` for login shells, and `.kshrc` for non-login.
                vec![home_dir.join(".profile"), home_dir.join(".kshrc")]
            }
            Self::Zsh => {
                // On Zsh, we only need to update `.zshenv`. This file is sourced for both login and
                // non-login shells. However, we match rustup's logic for determining _which_
                // `.zshenv` to use.
                //
                // See: https://github.com/rust-lang/rustup/blob/fede22fea7b160868cece632bd213e6d72f8912f/src/cli/self_update/shell.rs#L197
                let zsh_dot_dir = non_empty_env_path(EnvVars::ZDOTDIR);
                vec![zshenv_path(&home_dir, zsh_dot_dir.as_deref())]
            }
            Self::Fish => {
                // On Fish, we only need to update `config.fish`. This file is sourced for both
                // login and non-login shells. However, we must respect Fish's logic, which reads
                // from `$XDG_CONFIG_HOME/fish/config.fish` if set, and `~/.config/fish/config.fish`
                // otherwise.
                if let Some(xdg_home_dir) = non_empty_env_path(EnvVars::XDG_CONFIG_HOME) {
                    vec![xdg_home_dir.join("fish/config.fish")]
                } else {
                    vec![home_dir.join(".config/fish/config.fish")]
                }
            }
            Self::Csh => {
                // On Csh, we need to update both `.cshrc` and `.login`, like Bash.
                vec![home_dir.join(".cshrc"), home_dir.join(".login")]
            }
            // TODO(charlie): Add support for Nushell.
            Self::Nushell => vec![],
            // See: [`crate::windows::prepend_path`].
            Self::Powershell => vec![],
            // See: [`crate::windows::prepend_path`].
            Self::Cmd => vec![],
        }
    }

    /// Returns `true` if the given path is on the `PATH` in this shell.
    pub fn contains_path(path: &Path) -> bool {
        let home_dir = home_dir();
        std::env::var_os(EnvVars::PATH)
            .as_ref()
            .iter()
            .flat_map(std::env::split_paths)
            .map(|entry| expand_home_path(entry, home_dir.as_deref()))
            .any(|entry| same_file::is_same_file(path, entry).unwrap_or(false))
    }

    fn escaped_path(self, path: &Path) -> String {
        let path = path.simplified_display().to_string();
        match self {
            Self::Powershell => backtick_escape(&path),
            _ => backslash_escape(&path),
        }
    }

    /// Returns the command necessary to prepend a directory to the `PATH` in this shell.
    pub fn prepend_path(self, path: &Path) -> Option<String> {
        match self {
            Self::Nushell => None,
            Self::Bash | Self::Zsh | Self::Ksh => {
                Some(format!("export PATH=\"{}:$PATH\"", self.escaped_path(path)))
            }
            Self::Fish => Some(format!("fish_add_path \"{}\"", self.escaped_path(path))),
            Self::Csh => Some(format!("setenv PATH \"{}:$PATH\"", self.escaped_path(path))),
            Self::Powershell => Some(format!(
                "$env:PATH = \"{};$env:PATH\"",
                self.escaped_path(path)
            )),
            Self::Cmd => Some(format!("set PATH=\"{};%PATH%\"", self.escaped_path(path))),
        }
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bash => write!(f, "Bash"),
            Self::Fish => write!(f, "Fish"),
            Self::Powershell => write!(f, "PowerShell"),
            Self::Cmd => write!(f, "Command Prompt"),
            Self::Zsh => write!(f, "Zsh"),
            Self::Nushell => write!(f, "Nushell"),
            Self::Csh => write!(f, "Csh"),
            Self::Ksh => write!(f, "Ksh"),
        }
    }
}

/// Parse the shell from the name of the shell executable.
fn parse_shell_from_path(path: &Path) -> Option<Shell> {
    let name = path.file_stem()?.to_str()?;
    match name {
        "bash" => Some(Shell::Bash),
        "zsh" => Some(Shell::Zsh),
        "fish" => Some(Shell::Fish),
        "csh" => Some(Shell::Csh),
        "ksh" => Some(Shell::Ksh),
        "powershell" | "powershell_ise" | "pwsh" => Some(Shell::Powershell),
        _ => None,
    }
}

/// Escape a string for use in a shell command by inserting backslashes.
fn backslash_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' | '"' => escaped.push('\\'),
            _ => {}
        }
        escaped.push(c);
    }
    escaped
}

/// Escape a string for use in a `PowerShell` command by inserting backticks.
fn backtick_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            // Need to also escape unicode double quotes that PowerShell treats
            // as the ASCII double quote.
            '"' | '`' | '\u{201C}' | '\u{201D}' | '\u{201E}' | '$' => escaped.push('`'),
            _ => {}
        }
        escaped.push(c);
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs_err::File;
    use temp_env::with_vars;
    use tempfile::tempdir;

    // First option used by std::env::home_dir.
    const HOME_DIR_ENV_VAR: &str = if cfg!(windows) {
        EnvVars::USERPROFILE
    } else {
        EnvVars::HOME
    };

    #[test]
    fn configuration_files_bash_prefers_existing_login_file() {
        let tmp_home_dir = tempdir().unwrap();
        File::create(tmp_home_dir.path().join(".bash_login")).unwrap();

        with_vars([(HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str())], || {
            assert_eq!(
                Shell::Bash.configuration_files(),
                vec![
                    tmp_home_dir.path().join(".bash_login"),
                    tmp_home_dir.path().join(".bashrc")
                ]
            );
        });
    }

    #[test]
    fn configuration_files_zsh_no_existing_zshenv() {
        let tmp_home_dir = tempdir().unwrap();
        let tmp_zdotdir = tempdir().unwrap();

        with_vars(
            [
                (EnvVars::ZDOTDIR, None),
                (HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str()),
            ],
            || {
                assert_eq!(
                    Shell::Zsh.configuration_files(),
                    vec![tmp_home_dir.path().join(".zshenv")]
                );
            },
        );

        with_vars(
            [
                (EnvVars::ZDOTDIR, tmp_zdotdir.path().to_str()),
                (HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str()),
            ],
            || {
                assert_eq!(
                    Shell::Zsh.configuration_files(),
                    vec![tmp_zdotdir.path().join(".zshenv")]
                );
            },
        );
    }

    #[test]
    fn configuration_files_zsh_existing_home_zshenv() {
        let tmp_home_dir = tempdir().unwrap();
        File::create(tmp_home_dir.path().join(".zshenv")).unwrap();

        let tmp_zdotdir = tempdir().unwrap();

        with_vars(
            [
                (EnvVars::ZDOTDIR, None),
                (HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str()),
            ],
            || {
                assert_eq!(
                    Shell::Zsh.configuration_files(),
                    vec![tmp_home_dir.path().join(".zshenv")]
                );
            },
        );

        with_vars(
            [
                (EnvVars::ZDOTDIR, tmp_zdotdir.path().to_str()),
                (HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str()),
            ],
            || {
                assert_eq!(
                    Shell::Zsh.configuration_files(),
                    vec![tmp_home_dir.path().join(".zshenv")]
                );
            },
        );
    }

    #[test]
    fn configuration_files_zsh_existing_zdotdir_zshenv() {
        let tmp_home_dir = tempdir().unwrap();

        let tmp_zdotdir = tempdir().unwrap();
        File::create(tmp_zdotdir.path().join(".zshenv")).unwrap();

        with_vars(
            [
                (EnvVars::ZDOTDIR, tmp_zdotdir.path().to_str()),
                (HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str()),
            ],
            || {
                assert_eq!(
                    Shell::Zsh.configuration_files(),
                    vec![tmp_zdotdir.path().join(".zshenv")]
                );
            },
        );
    }

    #[test]
    fn from_env_prefers_shell_specific_variables_over_shell_path() {
        with_vars(
            [
                (EnvVars::BASH_VERSION, Some("5.2")),
                (EnvVars::SHELL, Some("/bin/zsh")),
            ],
            || {
                assert_eq!(Shell::from_env(), Some(Shell::Bash));
            },
        );
    }

    #[test]
    fn from_env_uses_shell_path_when_no_shell_specific_variable_is_set() {
        with_vars(
            [
                (EnvVars::BASH_VERSION, None),
                (EnvVars::SHELL, Some("/bin/zsh")),
            ],
            || {
                assert_eq!(Shell::from_env(), Some(Shell::Zsh));
            },
        );
    }

    #[test]
    fn configuration_files_fish_respects_xdg_config_home() {
        let tmp_home_dir = tempdir().unwrap();
        let tmp_xdg_dir = tempdir().unwrap();

        with_vars(
            [
                (EnvVars::XDG_CONFIG_HOME, tmp_xdg_dir.path().to_str()),
                (HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str()),
            ],
            || {
                assert_eq!(
                    Shell::Fish.configuration_files(),
                    vec![tmp_xdg_dir.path().join("fish/config.fish")]
                );
            },
        );
    }

    #[test]
    fn contains_path_expands_tilde_entries() {
        let tmp_home_dir = tempdir().unwrap();
        let bin_dir = tmp_home_dir.path().join("bin");
        fs_err::create_dir_all(&bin_dir).unwrap();

        let path = std::env::join_paths([PathBuf::from("~/bin")]).unwrap();

        with_vars(
            [
                (EnvVars::PATH, path.to_str()),
                (HOME_DIR_ENV_VAR, tmp_home_dir.path().to_str()),
            ],
            || {
                assert!(Shell::contains_path(&bin_dir));
            },
        );
    }

    #[test]
    fn prepend_path_uses_shell_specific_escaping() {
        let path = Path::new("/tmp/path with \"quotes\" and $dollars");

        assert_eq!(
            Shell::Bash.prepend_path(path),
            Some("export PATH=\"/tmp/path with \\\"quotes\\\" and $dollars:$PATH\"".to_string())
        );
        assert_eq!(
            Shell::Powershell.prepend_path(path),
            Some("$env:PATH = \"/tmp/path with `\"quotes`\" and `$dollars;$env:PATH\"".to_string())
        );
    }
}
