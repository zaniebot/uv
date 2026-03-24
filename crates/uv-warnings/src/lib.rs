use std::error::Error;
use std::iter;
use std::sync::atomic::AtomicBool;
use std::sync::{LazyLock, Mutex};

// macro hygiene: The user might not have direct dependencies on those crates
#[doc(hidden)]
pub use anstream;
#[doc(hidden)]
pub use owo_colors;
use owo_colors::{DynColor, OwoColorize};
use rustc_hash::FxHashSet;

/// Whether user-facing warnings are enabled.
pub static ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable user-facing warnings.
pub fn enable() {
    ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// Disable user-facing warnings.
pub fn disable() {
    ENABLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

/// The type of callback used to override warning output.
type WarnPrinter = Box<dyn Fn(&str) + Send>;

/// A global callback that, when set, wraps warning output so it doesn't conflict
/// with indicatif progress bars. Typically set to a closure that calls
/// `MultiProgress::suspend(|| eprintln!(...))`.
pub static WARN_PRINTER: LazyLock<Mutex<Option<WarnPrinter>>> = LazyLock::new(|| Mutex::new(None));

/// Register a printer callback for warning output.
///
/// When set, `warn_user!` and `warn_user_once!` will call this callback instead of
/// writing directly to stderr. This allows the caller to wrap the write in
/// `MultiProgress::suspend()` to avoid truncating warnings when progress bars are active.
pub fn set_warn_printer(printer: Box<dyn Fn(&str) + Send>) {
    if let Ok(mut guard) = WARN_PRINTER.lock() {
        *guard = Some(printer);
    }
}

/// Remove the printer callback, reverting to direct stderr writes.
pub fn unset_warn_printer() {
    if let Ok(mut guard) = WARN_PRINTER.lock() {
        *guard = None;
    }
}

/// Warn a user, if warnings are enabled.
#[macro_export]
macro_rules! warn_user {
    ($($arg:tt)*) => {{
        use $crate::anstream::eprintln;
        use $crate::owo_colors::OwoColorize;

        if $crate::ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            let message = format!("{}", format_args!($($arg)*));
            let formatted = format!("{}{} {}", "warning".yellow().bold(), ":".bold(), message.bold());
            if let Ok(guard) = $crate::WARN_PRINTER.lock() {
                if let Some(ref printer) = *guard {
                    printer(&formatted);
                } else {
                    eprintln!("{formatted}");
                }
            } else {
                eprintln!("{formatted}");
            }
        }
    }};
}

pub static WARNINGS: LazyLock<Mutex<FxHashSet<String>>> = LazyLock::new(Mutex::default);

/// Warn a user once, if warnings are enabled, with uniqueness determined by the content of the
/// message.
#[macro_export]
macro_rules! warn_user_once {
    ($($arg:tt)*) => {{
        use $crate::anstream::eprintln;
        use $crate::owo_colors::OwoColorize;

        if $crate::ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            if let Ok(mut states) = $crate::WARNINGS.lock() {
                let message = format!("{}", format_args!($($arg)*));
                if states.insert(message.clone()) {
                    let formatted = format!("{}{} {}", "warning".yellow().bold(), ":".bold(), message.bold());
                    if let Ok(guard) = $crate::WARN_PRINTER.lock() {
                        if let Some(ref printer) = *guard {
                            printer(&formatted);
                        } else {
                            eprintln!("{formatted}");
                        }
                    } else {
                        eprintln!("{formatted}");
                    }
                }
            }
        }
    }};
}

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
///
/// ```text
/// error: Failed to download Python 3.12
///  Caused by: Failed to fetch https://example.com/upload/python3.13.tar.zst
///             Server says: This endpoint only support POST requests.
///
///             For downloads, please refer to https://example.com/download/python3.13.tar.zst
///  Caused by: Caused By: HTTP Error 400
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
        err.to_string().trim().bold()
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
                let line = line.trim_end();
                if line.is_empty() {
                    // Avoid showing indents on empty lines
                    writeln!(&mut stream)?;
                } else {
                    writeln!(&mut stream, "{}{}", child_padding, line.trim_end())?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::write_error_chain;
    use anyhow::anyhow;
    use indoc::indoc;
    use insta::assert_snapshot;
    use owo_colors::AnsiColors;

    #[test]
    fn format_multiline_message() {
        let err_middle = indoc! {"Failed to fetch https://example.com/upload/python3.13.tar.zst
        Server says: This endpoint only support POST requests.

        For downloads, please refer to https://example.com/download/python3.13.tar.zst"};
        let err = anyhow!("Caused By: HTTP Error 400")
            .context(err_middle)
            .context("Failed to download Python 3.12");

        let mut rendered = String::new();
        write_error_chain(err.as_ref(), &mut rendered, "error", AnsiColors::Red).unwrap();
        let rendered = anstream::adapter::strip_str(&rendered);

        assert_snapshot!(rendered, @"
        error: Failed to download Python 3.12
          Caused by: Failed to fetch https://example.com/upload/python3.13.tar.zst
                     Server says: This endpoint only support POST requests.

                     For downloads, please refer to https://example.com/download/python3.13.tar.zst
          Caused by: Caused By: HTTP Error 400
        ");
    }
}
