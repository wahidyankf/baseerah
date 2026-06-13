//! Sealed `OutputFormat` enum for CLI output formatting.
//!
//! Mirrors `apps/rhino-cli/internal/cliout/format.go` — same canonical codes,
//! same `""` → `Text` default, same error string on unknown format.

use anyhow::{Error, anyhow};

/// The set of output formats supported by the CLI.
///
/// The enum is intentionally non-exhaustive in concept: adding a new format
/// requires updating [`OutputFormat::code`] and [`OutputFormat::parse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Plain-text output (the default).
    Text,
    /// Machine-readable JSON output.
    Json,
    /// GitHub-flavoured Markdown output.
    Markdown,
}

impl OutputFormat {
    /// Returns the canonical lowercase code string for this format.
    ///
    /// Mirrors the Go `Code()` method.
    pub fn code(self) -> &'static str {
        match self {
            OutputFormat::Text => "text",
            OutputFormat::Json => "json",
            OutputFormat::Markdown => "markdown",
        }
    }

    /// Parses a raw CLI flag value into a [`OutputFormat`] variant.
    ///
    /// An empty string and `"text"` both produce [`OutputFormat::Text`],
    /// matching the Go `Parse("") → FormatText{}, true` behaviour.
    ///
    /// # Errors
    ///
    /// Returns an error when `s` is not one of `""`, `"text"`, `"json"`, or
    /// `"markdown"`.
    pub fn parse(s: &str) -> Result<Self, Error> {
        match s {
            "" | "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "markdown" => Ok(OutputFormat::Markdown),
            other => Err(anyhow!(
                "unknown output format {other:?}: must be text, json, or markdown"
            )),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    /// Verifies that all documented format strings parse to the expected variants.
    #[test]
    fn parse_known_formats() {
        assert_eq!(OutputFormat::parse("text").unwrap(), OutputFormat::Text);
        assert_eq!(OutputFormat::parse("json").unwrap(), OutputFormat::Json);
        assert_eq!(
            OutputFormat::parse("markdown").unwrap(),
            OutputFormat::Markdown
        );
        assert_eq!(OutputFormat::parse("").unwrap(), OutputFormat::Text);
    }

    /// Verifies that an unknown format string returns the expected error message.
    #[test]
    fn parse_unknown_format_errors() {
        let err = OutputFormat::parse("xml").unwrap_err();
        assert_eq!(
            err.to_string(),
            "unknown output format \"xml\": must be text, json, or markdown"
        );
    }

    /// Verifies that `code()` and `parse()` are inverses of each other for all variants.
    #[test]
    fn code_round_trip() {
        for f in [
            OutputFormat::Text,
            OutputFormat::Json,
            OutputFormat::Markdown,
        ] {
            assert_eq!(OutputFormat::parse(f.code()).unwrap(), f);
        }
    }
}
