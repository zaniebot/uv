mod wrap;

use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::iter;

use owo_colors::{AnsiColors, OwoColorize};

use wrap::{get_wrap_width, wrap_text};

/// An error that may carry user-facing hints.
///
/// Implement this on error types that want to surface contextual suggestions
/// (e.g., "try `--prerelease=allow`") to the diagnostics layer. Hints are
/// rendered after the error output, each prefixed with `hint:`.
pub trait Hint {
    /// Return any hints associated with this error.
    fn hints(&self) -> Hints<'_> {
        Hints::none()
    }
}

/// A collection of user-facing hint messages.
///
/// Each hint is rendered on its own line, prefixed with the styled `hint:` label.
pub struct Hints<'a>(Vec<Cow<'a, str>>);

impl Hints<'_> {
    /// No hints.
    pub fn none() -> Self {
        Self(Vec::new())
    }

    /// Add a single owned hint.
    pub fn push(&mut self, hint: String) {
        self.0.push(Cow::Owned(hint));
    }

    /// Convert all borrowed hints to owned, extending the lifetime to `'static`.
    pub fn into_owned(self) -> Hints<'static> {
        Hints(
            self.0
                .into_iter()
                .map(|cow| Cow::Owned(cow.into_owned()))
                .collect(),
        )
    }

    /// Whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterate over the hints by reference.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(AsRef::as_ref)
    }

    /// Extend with another set of hints, converting borrowed hints to owned.
    pub fn extend(&mut self, other: Hints<'_>) {
        self.0
            .extend(other.0.into_iter().map(|cow| Cow::Owned(cow.into_owned())));
    }
}

impl<'a> From<&'a str> for Hints<'a> {
    fn from(hint: &'a str) -> Self {
        Self(vec![Cow::Borrowed(hint)])
    }
}

impl From<String> for Hints<'_> {
    fn from(hint: String) -> Self {
        Self(vec![Cow::Owned(hint)])
    }
}

impl FromIterator<String> for Hints<'_> {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(iter.into_iter().map(Cow::Owned).collect())
    }
}

impl fmt::Display for Hints<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for hint in &self.0 {
            write!(f, "\n{HintPrefix} {hint}")?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for Hints<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = std::vec::IntoIter<Cow<'a, str>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A styled `hint:` prefix for use in user-facing messages.
pub struct HintPrefix;

impl fmt::Display for HintPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", "hint".bold().cyan(), ":".bold())
    }
}

/// The severity level for an error chain.
///
/// Determines the label text and color used for the level prefix and
/// "Caused by" labels.
#[derive(Debug, Clone, Copy, Default)]
pub enum ErrorChainLevel {
    /// Red `error:` prefix.
    #[default]
    Error,
    /// Yellow `warning:` prefix.
    Warning,
}

impl ErrorChainLevel {
    /// The label text for this level.
    fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }

    /// The color for this level.
    fn color(self) -> AnsiColors {
        match self {
            Self::Error => AnsiColors::Red,
            Self::Warning => AnsiColors::Yellow,
        }
    }
}

/// Options for formatting error chains via [`write_error_chain_with_options`].
///
/// Use the builder methods to customize the output. Defaults to
/// [`ErrorChainLevel::Error`], no hints, and automatic terminal width detection.
///
/// # Example
///
/// ```
/// use uv_errors::{ErrorChainLevel, ErrorChainOptions};
///
/// let _options = ErrorChainOptions::default()
///     .level(ErrorChainLevel::Warning)
///     .width(80);
/// ```
pub struct ErrorChainOptions<'a> {
    /// The severity level (determines label text and color).
    level: ErrorChainLevel,
    /// Hints to render after the error chain.
    ///
    /// Each hint is displayed on its own line, prefixed with `hint:`.
    /// Callers are responsible for extracting hints from the error
    /// (e.g., via `hints_for_error` in the diagnostics layer).
    hints: Hints<'a>,
    /// Override the terminal width for wrapping (primarily for testing).
    width_override: Option<usize>,
}

impl Default for ErrorChainOptions<'_> {
    fn default() -> Self {
        Self {
            level: ErrorChainLevel::default(),
            hints: Hints::none(),
            width_override: None,
        }
    }
}

impl<'a> ErrorChainOptions<'a> {
    /// Set the severity level (default: [`ErrorChainLevel::Error`]).
    #[must_use]
    pub fn level(mut self, level: ErrorChainLevel) -> Self {
        self.level = level;
        self
    }

    /// Set hints to render after the error chain.
    #[must_use]
    pub fn hints(mut self, hints: Hints<'a>) -> Self {
        self.hints = hints;
        self
    }

    /// Override the terminal width for wrapping (default: auto-detect).
    #[must_use]
    pub fn width(mut self, width: usize) -> Self {
        self.width_override = Some(width);
        self
    }
}

/// Format an error chain with default settings (`"error"` level, red, no hints).
///
/// # Example
///
/// ```text
/// error: Failed to install app
///   Caused by: Failed to install dependency
///   Caused by: Permission denied
/// ```
pub fn write_error_chain(err: &dyn Error, stream: impl fmt::Write) -> fmt::Result {
    write_error_chain_with_options(err, stream, &ErrorChainOptions::default())
}

/// Format an error chain with hints (`"error"` level, red).
///
/// Shortcut for the common case of rendering an error with pre-extracted hints.
///
/// # Example
///
/// ```text
/// error: No solution found when resolving dependencies
///   Caused by: package `foo` was not found
///
/// hint: Packages were unavailable because the network was disabled.
/// ```
pub fn write_error_chain_with_hints(
    err: &dyn Error,
    stream: impl fmt::Write,
    hints: Hints<'_>,
) -> fmt::Result {
    write_error_chain_with_options(err, stream, &ErrorChainOptions::default().hints(hints))
}

/// Format a warning chain with default settings (`"warning"` level, yellow, no hints).
///
/// # Example
///
/// ```text
/// warning: Failed to create registry entry for Python 3.12
///   Caused by: Security policy forbids chaining registry entries
/// ```
pub fn write_warning_chain(err: &dyn Error, stream: impl fmt::Write) -> fmt::Result {
    write_error_chain_with_options(
        err,
        stream,
        &ErrorChainOptions::default().level(ErrorChainLevel::Warning),
    )
}

/// Format an error chain with the given [`ErrorChainOptions`].
///
/// # Example
///
/// ```text
/// warning: Failed to create registry entry for Python 3.12
///   Caused by: Security policy forbids chaining registry entries
///
/// hint: Try running with administrator privileges.
/// ```
pub fn write_error_chain_with_options(
    err: &dyn Error,
    mut stream: impl fmt::Write,
    options: &ErrorChainOptions<'_>,
) -> fmt::Result {
    let width = get_wrap_width(options.width_override);

    // Write main error message.
    let main_msg = err.to_string();
    let wrapped_main = wrap_text(&main_msg, width, "", "");
    let color = options.level.color();
    writeln!(
        &mut stream,
        "{}{} {}",
        options.level.label().color(color).bold(),
        ":".bold(),
        wrapped_main.trim()
    )?;

    for source in iter::successors(err.source(), |&err| err.source()) {
        let msg = source.to_string();
        let padding = "  ";
        let cause = "Caused by";
        let child_padding = " ".repeat(padding.len() + cause.len() + 2);

        // Wrap the message with proper indentation for continuation lines.
        let wrapped = wrap_text(&msg, width, "", &child_padding);

        // Split wrapped output and apply coloring to "Caused by:" prefix.
        let mut lines = wrapped.lines();
        if let Some(first) = lines.next() {
            writeln!(
                &mut stream,
                "{}{}: {}",
                padding,
                cause.color(color).bold(),
                first.trim()
            )?;
            for line in lines {
                if line.trim().is_empty() {
                    // Avoid showing indents on empty lines.
                    writeln!(&mut stream)?;
                } else {
                    writeln!(&mut stream, "{line}")?;
                }
            }
        }
    }

    // Write hints.
    for hint in options.hints.iter() {
        writeln!(&mut stream, "\n{HintPrefix} {hint}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use indoc::indoc;
    use insta::assert_snapshot;

    #[test]
    fn test_error_wrapping_with_columns() {
        #[derive(Debug, thiserror::Error)]
        #[error(
            "Because fiasobfhuasbf was not found in the package registry and you require fiasobfhuasbf, we can conclude that your requirements are unsatisfiable."
        )]
        struct Inner;

        #[derive(Debug, thiserror::Error)]
        #[error("No solution found when resolving dependencies")]
        struct Outer {
            #[source]
            source: Inner,
        }

        let error = Outer { source: Inner };
        let mut output = String::new();
        write_error_chain_with_options(
            &error,
            &mut output,
            &ErrorChainOptions::default().width(80),
        )
        .unwrap();
        let output = anstream::adapter::strip_str(&output);

        assert_snapshot!(output, @r"
        error: No solution found when resolving dependencies
          Caused by: Because fiasobfhuasbf was not found in the package registry and you require
                     fiasobfhuasbf, we can conclude that your requirements are
                     unsatisfiable.
        ");
    }

    #[test]
    fn test_error_chain_with_cause() {
        #[derive(Debug, thiserror::Error)]
        #[error("Permission denied")]
        struct Inner;

        #[derive(Debug, thiserror::Error)]
        #[error("Failed to write file")]
        struct Outer {
            #[source]
            source: Inner,
        }

        let error = Outer { source: Inner };
        let mut output = String::new();
        write_error_chain(&error, &mut output).unwrap();
        let output = anstream::adapter::strip_str(&output);

        assert_snapshot!(output, @r"
        error: Failed to write file
          Caused by: Permission denied
        ");
    }

    #[test]
    fn test_no_hyphenation() {
        #[derive(Debug, thiserror::Error)]
        #[error(
            "Failed to download package from https://files.pythonhosted.org/packages/verylongpackagename"
        )]
        struct LongWord;

        let error = LongWord;
        let mut output = String::new();
        write_error_chain_with_options(
            &error,
            &mut output,
            &ErrorChainOptions::default().width(50),
        )
        .unwrap();
        let output = anstream::adapter::strip_str(&output);
        assert_snapshot!(output, @r"
        error: Failed to download package from
        https://files.pythonhosted.org/packages/verylongpackagename
        ");
    }

    #[test]
    fn test_long_words_not_broken() {
        #[derive(Debug, thiserror::Error)]
        #[error(
            "The package supercalifragilisticexpialidocious-extraordinarily-long-name was not found"
        )]
        struct VeryLongWord;

        let error = VeryLongWord;
        let mut output = String::new();
        write_error_chain_with_options(
            &error,
            &mut output,
            &ErrorChainOptions::default().width(40),
        )
        .unwrap();
        let output = anstream::adapter::strip_str(&output);
        assert_snapshot!(output, @r"
        error: The package
        supercalifragilisticexpialidocious-extraordinarily-long-name
        was not found
        ");
    }

    #[test]
    fn test_multiple_error_sources() {
        #[derive(Debug, thiserror::Error)]
        #[error("Network connection timeout after multiple retry attempts")]
        struct DeepError;

        #[derive(Debug, thiserror::Error)]
        #[error("Failed to fetch package metadata from registry")]
        struct MiddleError {
            #[source]
            source: DeepError,
        }

        #[derive(Debug, thiserror::Error)]
        #[error("Unable to resolve package dependencies")]
        struct TopError {
            #[source]
            source: MiddleError,
        }

        let error = TopError {
            source: MiddleError { source: DeepError },
        };
        let mut output = String::new();
        write_error_chain_with_options(
            &error,
            &mut output,
            &ErrorChainOptions::default().width(60),
        )
        .unwrap();
        let output = anstream::adapter::strip_str(&output);
        assert_snapshot!(output, @r"
        error: Unable to resolve package dependencies
          Caused by: Failed to fetch package metadata from registry
          Caused by: Network connection timeout after multiple retry attempts
        ");
    }

    #[test]
    fn test_wrap_only_on_ascii_space() {
        #[derive(Debug, thiserror::Error)]
        #[error("Path /usr/local/lib/python3.12/site-packages not found in filesystem hierarchy")]
        struct SpecialChars;

        let error = SpecialChars;
        let mut output = String::new();
        write_error_chain_with_options(
            &error,
            &mut output,
            &ErrorChainOptions::default().width(50),
        )
        .unwrap();
        let output = anstream::adapter::strip_str(&output);
        assert_snapshot!(output, @r"
        error: Path /usr/local/lib/python3.12/site-packages not
        found in filesystem hierarchy
        ");
    }

    #[test]
    fn format_with_hints() {
        let err = anyhow!("Permission denied").context("Failed to fetch package");

        let hints: Hints<'_> = [
            "Try running with `--verbose` for more information.".to_string(),
            "Try running without --offline.".to_string(),
        ]
        .into_iter()
        .collect();

        let mut rendered = String::new();
        write_error_chain_with_hints(err.as_ref(), &mut rendered, hints).unwrap();
        let rendered = anstream::adapter::strip_str(&rendered);

        assert_snapshot!(rendered, @r"
        error: Failed to fetch package
          Caused by: Permission denied

        hint: Try running with `--verbose` for more information.

        hint: Try running without --offline.
        ");
    }

    #[test]
    fn format_multiline_message() {
        let err_middle = indoc! {"Failed to fetch https://example.com/upload/python3.13.tar.zst
        Server says: This endpoint only support POST requests.

        For downloads, please refer to https://example.com/download/python3.13.tar.zst"};
        let err = anyhow!("Caused By: HTTP Error 400")
            .context(err_middle)
            .context("Failed to download Python 3.12");

        let mut rendered = String::new();
        write_error_chain(err.as_ref(), &mut rendered).unwrap();
        let rendered = anstream::adapter::strip_str(&rendered);

        assert_snapshot!(rendered, @r"
        error: Failed to download Python 3.12
          Caused by: Failed to fetch https://example.com/upload/python3.13.tar.zst
                     Server says: This endpoint only support POST requests.

                     For downloads, please refer to https://example.com/download/python3.13.tar.zst
          Caused by: Caused By: HTTP Error 400
        ");
    }
}
