use std::fmt::Write;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// Install uv to a specified directory and update PATH.
pub(crate) async fn self_install(
    bin_dir: Option<std::path::PathBuf>,
    printer: Printer,
) -> Result<ExitStatus> {
    use std::env::current_exe;
    use std::fs;
    
    use uv_dirs::user_executable_directory;
    use uv_fs::{copy_atomic_sync, with_retry_sync};
    use uv_shell::Shell;
    use tokio::io::AsyncWriteExt;

    // Get the current executable path
    let current_exe = current_exe().context("Failed to get current executable path")?;
    
    // Determine the target directory
    let target_dir = if let Some(dir) = bin_dir {
        dir
    } else {
        // Use uv-dirs to find the appropriate executable directory
        user_executable_directory(None)
            .context("Failed to determine user executable directory")?
    };
    
    // Ensure the target directory exists
    fs::create_dir_all(&target_dir)
        .with_context(|| format!("Failed to create directory {}", target_dir.display()))?;

    // Determine the executable name (uv or uv.exe)
    let exe_name = current_exe
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Failed to get executable name"))?;
    
    let target_path = target_dir.join(exe_name);
    
    // Check if we're trying to install to the same location
    if current_exe.canonicalize().ok() == target_path.canonicalize().ok() {
        writeln!(
            printer.stderr(),
            "{}",
            format_args!(
                "{}{} uv is already installed at {}",
                "info".cyan().bold(),
                ":".bold(),
                target_path.display().to_string().cyan()
            )
        )?;
        return Ok(ExitStatus::Success);
    }

    writeln!(
        printer.stderr(),
        "{}",
        format_args!(
            "{}{} Installing uv to {}",
            "info".cyan().bold(),
            ":".bold(),
            target_path.display().to_string().cyan()
        )
    )?;

    // Copy the current executable to the target location using the safe utilities
    with_retry_sync(
        &current_exe,
        &target_path,
        "copy",
        || copy_atomic_sync(&current_exe, &target_path),
    )?;

    // Make it executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_path, perms)?;
    }

    writeln!(
        printer.stderr(),
        "{}",
        format_args!(
            "{}{} Installed uv to {}",
            "success".green().bold(),
            ":".bold(),
            target_path.display().to_string().cyan()
        )
    )?;

    // Check if the target directory is already in PATH
    if Shell::contains_path(&target_dir) {
        writeln!(
            printer.stderr(),
            "{}",
            format_args!(
                "{}{} Directory {} is already in PATH",
                "info".cyan().bold(),
                ":".bold(),
                target_dir.display().to_string().cyan()
            )
        )?;
        return Ok(ExitStatus::Success);
    }

    // On Windows, try to update the PATH registry
    #[cfg(windows)]
    {
        if uv_shell::windows::prepend_path(&target_dir)? {
            writeln!(
                printer.stderr(),
                "{}",
                format_args!(
                    "{}{} Updated PATH to include {}",
                    "success".green().bold(),
                    ":".bold(),
                    target_dir.display().to_string().cyan()
                )
            )?;
            writeln!(
                printer.stderr(),
                "{}",
                format_args!(
                    "{}{} Restart your shell to apply changes",
                    "info".cyan().bold(),
                    ":".bold()
                )
            )?;
            return Ok(ExitStatus::Success);
        }
    }

    // On Unix, update shell configuration files
    #[cfg(unix)]
    {
        // Determine the current shell
        let Some(shell) = Shell::from_env() else {
            writeln!(
                printer.stderr(),
                "{}",
                format_args!(
                    "{}{} Directory {} is not in PATH, but the current shell could not be determined",
                    "info".cyan().bold(),
                    ":".bold(),
                    target_dir.display().to_string().cyan()
                )
            )?;
            writeln!(
                printer.stderr(),
                "{}",
                format_args!(
                    "{}{} Add {} to your PATH manually to use uv",
                    "info".cyan().bold(),
                    ":".bold(),
                    target_dir.display().to_string().cyan()
                )
            )?;
            return Ok(ExitStatus::Success);
        };

        // Look up the configuration files (e.g., `.bashrc`, `.zshrc`) for the shell
        let files = shell.configuration_files();
        if files.is_empty() {
            writeln!(
                printer.stderr(),
                "{}",
                format_args!(
                    "{}{} Directory {} is not in PATH, but updating {} is currently unsupported",
                    "warning".yellow().bold(),
                    ":".bold(),
                    target_dir.display().to_string().cyan(),
                    shell
                )
            )?;
            return Ok(ExitStatus::Success);
        }

        // Prepare the command (e.g., `export PATH="$HOME/.local/bin:$PATH"`)
        let Some(command) = shell.prepend_path(&target_dir) else {
            writeln!(
                printer.stderr(),
                "{}",
                format_args!(
                    "{}{} Directory {} is not in PATH, but the necessary command to update {} could not be determined",
                    "warning".yellow().bold(),
                    ":".bold(),
                    target_dir.display().to_string().cyan(),
                    shell
                )
            )?;
            return Ok(ExitStatus::Success);
        };

        // Update each file, as necessary
        let mut updated = false;
        for file in files {
            // Search for the command in the file, to avoid redundant updates
            match fs_err::tokio::read_to_string(&file).await {
                Ok(contents) => {
                    if contents
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.starts_with('#'))
                        .any(|line| line.contains(&command))
                    {
                        tracing::debug!(
                            "Skipping already-updated configuration file: {}",
                            file.display()
                        );
                        continue;
                    }

                    // Append the command to the file
                    fs_err::tokio::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(&file)
                        .await?
                        .write_all(format!("{contents}\n# uv\n{command}\n").as_bytes())
                        .await?;

                    writeln!(
                        printer.stderr(),
                        "{}",
                        format_args!(
                            "{}{} Updated configuration file: {}",
                            "success".green().bold(),
                            ":".bold(),
                            file.display().to_string().cyan()
                        )
                    )?;
                    updated = true;
                }
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    // Ensure that the directory containing the file exists
                    if let Some(parent) = file.parent() {
                        fs::create_dir_all(parent).with_context(|| {
                            format!("Failed to create parent directory: {}", parent.display())
                        })?;
                    }

                    // Write the command to the file
                    fs_err::tokio::write(&file, format!("# uv\n{command}\n"))
                        .await
                        .with_context(|| format!("Failed to write to: {}", file.display()))?;

                    writeln!(
                        printer.stderr(),
                        "{}",
                        format_args!(
                            "{}{} Created configuration file: {}",
                            "success".green().bold(),
                            ":".bold(),
                            file.display().to_string().cyan()
                        )
                    )?;
                    updated = true;
                }
                Err(err) => {
                    writeln!(
                        printer.stderr(),
                        "{}",
                        format_args!(
                            "{}{} Failed to read configuration file ({}): {}",
                            "warning".yellow().bold(),
                            ":".bold(),
                            file.display().to_string().cyan(),
                            err
                        )
                    )?;
                }
            }
        }

        if updated {
            writeln!(
                printer.stderr(),
                "{}",
                format_args!(
                    "{}{} Restart your shell to apply changes",
                    "info".cyan().bold(),
                    ":".bold()
                )
            )?;
        }
    }

    Ok(ExitStatus::Success)
}