// Sealed OutputFormat enum. Mirrors `apps/rhino-cli/internal/cliout/format.go`
// — same canonical codes, same "" → Text default, same error string on unknown.

use anyhow::{Error, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

impl OutputFormat {
    /// Canonical lowercase code (mirrors Go `Code()`).
    pub fn code(self) -> &'static str {
        match self {
            OutputFormat::Text => "text",
            OutputFormat::Json => "json",
            OutputFormat::Markdown => "markdown",
        }
    }

    /// Parse a raw flag value into the sealed enum. Empty string and "text"
    /// both produce `Text` (matches Go `Parse("")` → `FormatText{}, true`).
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

    #[test]
    fn parse_unknown_format_errors() {
        let err = OutputFormat::parse("xml").unwrap_err();
        assert_eq!(
            err.to_string(),
            "unknown output format \"xml\": must be text, json, or markdown"
        );
    }

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
