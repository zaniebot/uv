use std::fmt::Write;

use anyhow::Context;
use owo_colors::OwoColorize;
use tracing::debug;

use uv_cache::Cache;
use uv_cli_output::format::elapsed;
use uv_cli_output::printer::Printer;
use uv_configuration::Concurrency;
use uv_fs::{CWD, Simplified};
use uv_installer::compile_tree;
use uv_python::PythonEnvironment;

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
        if !site_packages.exists() {
            debug!(
                "Skipping non-existent site-packages directory: {}",
                site_packages.display()
            );
            continue;
        }
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
