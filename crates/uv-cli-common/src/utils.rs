use std::fmt::Display;
use std::fmt::Write;
use std::time::Duration;

use anyhow::Context;
use owo_colors::OwoColorize;
use uv_cache::Cache;
use uv_configuration::Concurrency;
use uv_distribution_types::InstalledMetadata;
use uv_fs::{CWD, Simplified};
use uv_installer::compile_tree;
use uv_normalize::PackageName;
use uv_python::PythonEnvironment;

use crate::printer::Printer;

/// Format a duration as a human-readable string, Cargo-style.
pub fn elapsed(duration: Duration) -> String {
    let secs = duration.as_secs();
    let ms = duration.subsec_millis();

    if secs >= 60 {
        format!("{}m {:02}s", secs / 60, secs % 60)
    } else if secs > 0 {
        format!("{}.{:02}s", secs, duration.subsec_nanos() / 10_000_000)
    } else if ms > 0 {
        format!("{ms}ms")
    } else {
        format!("0.{:02}ms", duration.subsec_nanos() / 10_000)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangeEventKind {
    /// The package was removed from the environment.
    Removed,
    /// The package was added to the environment.
    Added,
    /// The package was reinstalled without changing versions.
    Reinstalled,
}

#[derive(Debug)]
pub struct ChangeEvent<'a, T: InstalledMetadata> {
    pub dist: &'a T,
    pub kind: ChangeEventKind,
}

#[derive(Debug)]
pub struct DryRunEvent<T: Display> {
    pub name: PackageName,
    pub version: T,
    pub kind: ChangeEventKind,
}

impl<'a, T: InstalledMetadata> ChangeEvent<'a, T> {
    /// Returns `true` if the [`PackageName`] was not re-installed.
    pub fn is_removal(&self) -> bool {
        matches!(self.kind, ChangeEventKind::Removed)
    }

    /// Returns the [`PackageName`] for the [`ChangeEvent`].
    pub fn name(&self) -> &PackageName {
        self.dist.name()
    }
}

impl<'a, T: InstalledMetadata> From<ChangeEvent<'a, T>> for DryRunEvent<String> {
    fn from(value: ChangeEvent<'a, T>) -> Self {
        Self {
            name: value.dist.name().clone(),
            version: value.dist.installed_version().to_string(),
            kind: value.kind,
        }
    }
}

/// Returns a human-readable representation of a number of bytes.
pub fn human_readable_bytes(bytes: u64) -> (f32, &'static str) {
    static UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let bytes = bytes as f32;
    let i = ((bytes.log2() / 10.0) as usize).min(UNITS.len() - 1);
    (bytes / 1024_f32.powi(i as i32), UNITS[i])
}

/// Compile all Python source files in site-packages to bytecode, to speed up the
/// initial run of any subsequent executions.
///
/// See the `--compile` option on `pip sync` and `pip install`.
pub async fn compile_bytecode(
    venv: &PythonEnvironment,
    concurrency: &Concurrency,
    cache: &Cache,
    printer: Printer,
) -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    let mut files = 0;
    for site_packages in venv.site_packages() {
        let site_packages = CWD.join(site_packages);
        files += compile_tree(
            &site_packages,
            venv.python_executable(),
            concurrency,
            cache.root(),
        )
        .await
        .with_context(|| {
            format!(
                "Failed to bytecode-compile Python file in: {}",
                site_packages.user_display()
            )
        })?;
    }
    let s = if files == 1 { "" } else { "s" };
    writeln!(
        printer.stderr(),
        "{}",
        format!(
            "Bytecode compiled {} {}",
            format!("{files} file{s}").bold(),
            format!("in {}", elapsed(start.elapsed())).dimmed()
        )
        .dimmed()
    )?;
    Ok(())
}