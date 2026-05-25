//! Severity level enum and resolution helpers.
//!
//! Port of `apps/rhino-cli/internal/severity/severity.go`.
//!
//! The severity level controls how validation findings are treated by the CLI:
//! `Error` causes a non-zero exit code, while `Warn` produces advisory output only.

use std::io::Write;

/// Two-level severity scale used by all rhino-cli validators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// The finding must block the pipeline (non-zero exit).
    Error,
    /// The finding is advisory and does not block the pipeline.
    Warn,
}

impl Severity {
    /// Returns the canonical lowercase string code for this severity level.
    ///
    /// - `"error"` for [`Severity::Error`]
    /// - `"warn"` for [`Severity::Warn`]
    pub fn code(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warn => "warn",
        }
    }
}

/// Parses a severity string into a [`Severity`] variant.
///
/// Accepts `"warn"` and `"warning"` (case-insensitive) as [`Severity::Warn`].
/// Everything else — including an empty string — maps to [`Severity::Error`].
pub fn parse(s: &str) -> Severity {
    let trimmed = s.trim().to_lowercase();
    match trimmed.as_str() {
        "warn" | "warning" => Severity::Warn,
        _ => Severity::Error,
    }
}

/// Resolves the effective severity from a CLI flag value and an environment
/// variable value, writing an audit message to `stderr` when the env var
/// downgrades severity to `Warn`.
///
/// Priority:
/// 1. Non-empty `flag_val` is used as-is.
/// 2. Non-empty `env_val` is used and an audit line is emitted on downgrade.
/// 3. Default: [`Severity::Error`].
pub fn resolve(flag_val: &str, env_val: &str, stderr: &mut dyn Write) -> Severity {
    if !flag_val.is_empty() {
        return parse(flag_val);
    }
    if !env_val.is_empty() {
        let sev = parse(env_val);
        if sev == Severity::Warn {
            let _ = writeln!(
                stderr,
                "WARN: severity downgraded to \"warn\" via OSE_RHINO_DDD_SEVERITY env var"
            );
        }
        return sev;
    }
    Severity::Error
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `"warn"` and `"WARNING"` (and variants) parse to [`Severity::Warn`].
    #[test]
    fn parse_warn() {
        assert_eq!(parse("warn"), Severity::Warn);
        assert_eq!(parse("WARNING"), Severity::Warn);
        assert_eq!(parse(" Warn "), Severity::Warn);
    }

    /// Verifies that an empty string and `"error"` both parse to [`Severity::Error`].
    #[test]
    fn parse_error_default() {
        assert_eq!(parse(""), Severity::Error);
        assert_eq!(parse("error"), Severity::Error);
        assert_eq!(parse("anything"), Severity::Error);
    }

    /// Verifies that a non-empty `flag_val` takes precedence over `env_val`
    /// and produces no output on `stderr`.
    #[test]
    fn resolve_flag_wins() {
        let mut buf = Vec::new();
        assert_eq!(resolve("warn", "error", &mut buf), Severity::Warn);
        assert!(buf.is_empty());
    }

    /// Verifies that a `"warn"` env var emits an audit message on `stderr`.
    #[test]
    fn resolve_env_emits_audit_for_warn() {
        let mut buf: Vec<u8> = Vec::new();
        let sev = resolve("", "warn", &mut buf);
        assert_eq!(sev, Severity::Warn);
        assert!(String::from_utf8_lossy(&buf).contains("downgraded"));
    }

    /// Verifies that an `"error"` env var does not emit anything on `stderr`.
    #[test]
    fn resolve_env_error_silent() {
        let mut buf: Vec<u8> = Vec::new();
        let sev = resolve("", "error", &mut buf);
        assert_eq!(sev, Severity::Error);
        assert!(buf.is_empty());
    }

    /// Verifies that empty flag and env values both absent default to [`Severity::Error`].
    #[test]
    fn resolve_default_error() {
        let mut buf: Vec<u8> = Vec::new();
        assert_eq!(resolve("", "", &mut buf), Severity::Error);
    }

    /// Verifies that [`Severity::code`] returns the expected string for each variant.
    #[test]
    fn code_strings() {
        assert_eq!(Severity::Error.code(), "error");
        assert_eq!(Severity::Warn.code(), "warn");
    }
}
