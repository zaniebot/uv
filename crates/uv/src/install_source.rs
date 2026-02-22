use std::fmt::Write;
use std::process::Command;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::Result;
use owo_colors::OwoColorize;
use serde::Deserialize;
use tracing::debug;
use which::which;

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// The Homebrew formulae API endpoint for uv.
const HOMEBREW_FORMULA_URL: &str = "https://formulae.brew.sh/api/formula/uv.json";

/// Known sources for uv installations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InstallSource {
    Homebrew,
}

impl InstallSource {
    /// Attempt to infer the install source for the given executable path.
    fn from_path(path: &Path) -> Option<Self> {
        let canonical = path.canonicalize().unwrap_or_else(|_| PathBuf::from(path));

        let components = canonical
            .components()
            .map(|component| component.as_os_str().to_owned())
            .collect::<Vec<_>>();

        let cellar = OsStr::new("Cellar");
        let formula = OsStr::new("uv");

        if components
            .windows(2)
            .any(|window| window[0] == cellar && window[1] == formula)
        {
            return Some(Self::Homebrew);
        }

        None
    }

    /// Detect how uv was installed by inspecting the current executable path.
    pub(crate) fn detect() -> Option<Self> {
        Self::from_path(&std::env::current_exe().ok()?)
    }

    /// Run the package manager's upgrade command, streaming output to the terminal.
    pub(crate) async fn upgrade(self, dry_run: bool, printer: Printer) -> Result<ExitStatus> {
        match self {
            Self::Homebrew => upgrade_homebrew(dry_run, printer).await,
        }
    }
}

/// Response from the Homebrew formulae API.
#[derive(Deserialize)]
struct HomebrewFormula {
    versions: HomebrewVersions,
}

/// Version information from the Homebrew formulae API.
#[derive(Deserialize)]
struct HomebrewVersions {
    stable: String,
}

/// Query the Homebrew formulae API for the latest stable version of uv.
async fn homebrew_latest_version() -> Result<String> {
    let response = reqwest::get(HOMEBREW_FORMULA_URL).await?;
    let formula: HomebrewFormula = response.json().await?;
    Ok(formula.versions.stable)
}

/// Run `brew upgrade uv`, streaming output to the terminal.
async fn upgrade_homebrew(dry_run: bool, printer: Printer) -> Result<ExitStatus> {
    let current = uv_version::version();

    writeln!(
        printer.stderr(),
        "{}",
        format_args!(
            "{}{} Checking for updates...",
            "info".cyan().bold(),
            ":".bold()
        )
    )?;

    let latest = homebrew_latest_version().await?;

    if current == latest {
        writeln!(
            printer.stderr(),
            "{}",
            format_args!(
                "{}{} You're on the latest version of uv ({})",
                "success".green().bold(),
                ":".bold(),
                format!("v{current}").bold().cyan()
            )
        )?;
        return Ok(ExitStatus::Success);
    }

    if dry_run {
        writeln!(
            printer.stderr(),
            "{}",
            format_args!(
                "Would upgrade uv from {} to {}",
                format!("v{current}").bold().white(),
                format!("v{latest}").bold().white(),
            )
        )?;
        return Ok(ExitStatus::Success);
    }

    let brew = which("brew").map_err(|_| anyhow::anyhow!("`brew` not found in PATH"))?;

    writeln!(
        printer.stderr(),
        "{}",
        format_args!(
            "{}{} Running `{}`",
            "info".cyan().bold(),
            ":".bold(),
            "brew upgrade uv".bold()
        )
    )?;

    // Run `brew upgrade uv` with inherited stdio so the user sees brew's output.
    let status = Command::new(&brew).args(["upgrade", "uv"]).status()?;

    if !status.success() {
        debug!("brew upgrade uv exited with status: {status}");
        anyhow::bail!("`brew upgrade uv` failed");
    }

    writeln!(
        printer.stderr(),
        "{}",
        format_args!(
            "{}{} Upgraded uv from {} to {}! {}",
            "success".green().bold(),
            ":".bold(),
            format!("v{current}").bold().cyan(),
            format!("v{latest}").bold().cyan(),
            format!("https://github.com/astral-sh/uv/releases/tag/{latest}").cyan()
        )
    )?;

    Ok(ExitStatus::Success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_homebrew_cellar() {
        assert_eq!(
            InstallSource::from_path(Path::new("/opt/homebrew/Cellar/uv/0.9.11/bin/uv")),
            Some(InstallSource::Homebrew)
        );
    }

    #[test]
    fn detects_linuxbrew_cellar() {
        assert_eq!(
            InstallSource::from_path(Path::new(
                "/home/linuxbrew/.linuxbrew/Cellar/uv/0.9.11/bin/uv"
            )),
            Some(InstallSource::Homebrew)
        );
    }

    #[test]
    fn ignores_non_cellar_paths() {
        assert_eq!(
            InstallSource::from_path(Path::new("/usr/local/bin/uv")),
            None
        );
    }
}
