use std::error::Error;
use std::iter;

use owo_colors::{DynColor, OwoColorize};

/// Format an error or warning chain.
///
/// # Example
///
/// ```text
/// error: Failed to install app
///   Caused By: Failed to install dependency
///   Caused By: Error writing failed `/home/ferris/deps/foo`: Permission denied
/// ```
///
/// ```text
/// warning: Failed to create registry entry for Python 3.12
///   Caused By: Security policy forbids chaining registry entries
/// ```
pub fn write_error_chain(
    err: &dyn Error,
    mut stream: impl std::fmt::Write,
    level: impl AsRef<str>,
    color: impl DynColor + Copy,
) -> std::fmt::Result {
    writeln!(
        &mut stream,
        "{}{} {}",
        level.as_ref().color(color).bold(),
        ":".bold(),
        err.to_string().trim()
    )?;
    for source in iter::successors(err.source(), |&err| err.source()) {
        let msg = source.to_string();
        let mut lines = msg.lines();
        if let Some(first) = lines.next() {
            let padding = "  ";
            let cause = "Caused by";
            let child_padding = " ".repeat(padding.len() + cause.len() + 2);
            writeln!(
                &mut stream,
                "{}{}: {}",
                padding,
                cause.color(color).bold(),
                first.trim()
            )?;
            for line in lines {
                writeln!(&mut stream, "{}{}", child_padding, line.trim_end())?;
            }
        }
    }
    Ok(())
}
