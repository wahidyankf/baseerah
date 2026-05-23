// Severity enum ported from `apps/rhino-cli/internal/severity/severity.go`.

use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warn,
}

impl Severity {
    pub fn code(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warn => "warn",
        }
    }
}

pub fn parse(s: &str) -> Severity {
    let trimmed = s.trim().to_lowercase();
    match trimmed.as_str() {
        "warn" | "warning" => Severity::Warn,
        _ => Severity::Error,
    }
}

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

    #[test]
    fn parse_warn() {
        assert_eq!(parse("warn"), Severity::Warn);
        assert_eq!(parse("WARNING"), Severity::Warn);
        assert_eq!(parse(" Warn "), Severity::Warn);
    }

    #[test]
    fn parse_error_default() {
        assert_eq!(parse(""), Severity::Error);
        assert_eq!(parse("error"), Severity::Error);
        assert_eq!(parse("anything"), Severity::Error);
    }

    #[test]
    fn resolve_flag_wins() {
        let mut buf = Vec::new();
        assert_eq!(resolve("warn", "error", &mut buf), Severity::Warn);
        assert!(buf.is_empty());
    }

    #[test]
    fn resolve_env_emits_audit_for_warn() {
        let mut buf: Vec<u8> = Vec::new();
        let sev = resolve("", "warn", &mut buf);
        assert_eq!(sev, Severity::Warn);
        assert!(String::from_utf8_lossy(&buf).contains("downgraded"));
    }

    #[test]
    fn resolve_env_error_silent() {
        let mut buf: Vec<u8> = Vec::new();
        let sev = resolve("", "error", &mut buf);
        assert_eq!(sev, Severity::Error);
        assert!(buf.is_empty());
    }

    #[test]
    fn resolve_default_error() {
        let mut buf: Vec<u8> = Vec::new();
        assert_eq!(resolve("", "", &mut buf), Severity::Error);
    }

    #[test]
    fn code_strings() {
        assert_eq!(Severity::Error.code(), "error");
        assert_eq!(Severity::Warn.code(), "warn");
    }
}
