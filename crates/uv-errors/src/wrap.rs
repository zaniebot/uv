use uv_static::EnvVars;

/// Checks if line wrapping should be enabled.
///
/// Returns `false` if `UV_NO_WRAP` is set.
fn should_wrap_lines() -> bool {
    std::env::var_os(EnvVars::UV_NO_WRAP).is_none()
}

/// Gets the terminal width for wrapping.
///
/// Checks `width_override`, then `COLUMNS` env var, then terminal size detection.
/// Returns `None` if width cannot be determined (no wrapping should occur).
pub(crate) fn get_wrap_width(width_override: Option<usize>) -> Option<usize> {
    if !should_wrap_lines() {
        return None;
    }

    // Use override if provided (for testing).
    if let Some(width) = width_override {
        return Some(width);
    }

    // Check COLUMNS environment variable.
    if let Ok(cols) = std::env::var(EnvVars::COLUMNS) {
        if let Ok(width) = cols.parse::<usize>() {
            return Some(width);
        }
    }

    // Try to detect terminal width.
    if let Some((terminal_size::Width(width), _)) = terminal_size::terminal_size() {
        return Some(width as usize);
    }

    // No width detected — don't wrap.
    None
}

/// Wraps text at word boundaries with proper indentation.
///
/// Based on miette's `wrap()` implementation from:
/// <https://github.com/zkat/miette/blob/v7.2.0/src/handlers/graphical.rs#L876-L909>
pub(crate) fn wrap_text(
    text: &str,
    width: Option<usize>,
    initial_indent: &str,
    subsequent_indent: &str,
) -> String {
    let Some(width) = width else {
        // If not wrapping, apply indentation while preserving line breaks.
        let mut result = String::with_capacity(2 * text.len());

        for (idx, line) in text.split_terminator('\n').enumerate() {
            if idx == 0 {
                result.push_str(initial_indent);
            } else {
                result.push('\n');
                // Don't add indent to empty lines (avoid trailing whitespace).
                if !line.is_empty() {
                    result.push_str(subsequent_indent);
                }
            }
            result.push_str(line);
        }

        return result;
    };

    let options = textwrap::Options::new(width)
        .initial_indent(initial_indent)
        .subsequent_indent(subsequent_indent)
        .break_words(false)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(textwrap::WordSplitter::NoHyphenation);

    textwrap::fill(text, options)
}
