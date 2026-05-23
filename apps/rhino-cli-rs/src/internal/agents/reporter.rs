// Validation reporter ported from
// `apps/rhino-cli/internal/agents/reporter.go` (validation-only paths).

use std::fmt::Write as _;
use std::time::Duration;

use anyhow::Error;
use chrono::Local;
use serde::Serialize;

use super::types::ValidationResult;

fn status_banner(result: &ValidationResult) -> &'static str {
    if result.failed_checks > 0 {
        "\u{274C} VALIDATION FAILED"
    } else if result.warning_checks > 0 {
        "\u{26A0} VALIDATION PASSED WITH WARNINGS"
    } else {
        "\u{2713} VALIDATION PASSED"
    }
}

fn status_json(result: &ValidationResult) -> &'static str {
    if result.failed_checks > 0 {
        "failure"
    } else if result.warning_checks > 0 {
        "warning"
    } else {
        "success"
    }
}

/// Formats a Duration the same way Go's `time.Duration.String()` does:
/// 1s, 100ms, 1.5s, 1µs, 1ns, 1m0s, 1h0m0s, etc. Implementation mirrors
/// the relevant cases in src/time/format.go (Go std lib).
pub fn format_go_duration(d: Duration) -> String {
    let nanos = d.as_nanos() as i128;
    if nanos == 0 {
        return "0s".to_string();
    }
    if nanos < 1_000_000_000 {
        // Sub-second: pick unit by magnitude.
        let (mut frac, unit) = if nanos < 1_000 {
            return format!("{nanos}ns");
        } else if nanos < 1_000_000 {
            (format_fraction(nanos, 1_000), "\u{00B5}s")
        } else {
            (format_fraction(nanos, 1_000_000), "ms")
        };
        if frac.is_empty() {
            frac = "0".to_string();
        }
        return format!("{frac}{unit}");
    }
    // ≥ 1s
    let total_secs = nanos / 1_000_000_000;
    let frac_ns = (nanos % 1_000_000_000) as i64;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    let frac_part = if frac_ns == 0 {
        String::new()
    } else {
        format!(".{}", trim_trailing_zeros(&format!("{frac_ns:09}")))
    };
    if hours > 0 {
        format!("{hours}h{mins}m{secs}{frac_part}s")
    } else if mins > 0 {
        format!("{mins}m{secs}{frac_part}s")
    } else {
        format!("{secs}{frac_part}s")
    }
}

fn format_fraction(nanos: i128, scale: i128) -> String {
    let whole = nanos / scale;
    let frac = nanos % scale;
    let mut s = format!("{whole}");
    if frac != 0 {
        let width = match scale {
            1_000 => 3,
            1_000_000 => 6,
            _ => 9,
        };
        let frac_str = format!("{frac:0width$}", width = width);
        let trimmed = trim_trailing_zeros(&frac_str);
        if !trimmed.is_empty() {
            s.push('.');
            s.push_str(&trimmed);
        }
    }
    s
}

fn trim_trailing_zeros(s: &str) -> String {
    let trimmed = s.trim_end_matches('0');
    trimmed.to_string()
}

/// Plain-text formatter.
pub fn format_validation_text(result: &ValidationResult, verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();
    if !quiet {
        sb.push_str("Validation Complete\n");
        sb.push_str(&"=".repeat(50));
        sb.push_str("\n\n");
    }
    let _ = writeln!(sb, "Total Checks: {}", result.total_checks);
    let _ = writeln!(sb, "Passed: {}", result.passed_checks);
    if result.warning_checks > 0 {
        let _ = writeln!(sb, "Warnings: {}", result.warning_checks);
    }
    let _ = writeln!(sb, "Failed: {}", result.failed_checks);
    let _ = writeln!(sb, "Duration: {}", format_go_duration(result.duration));

    if result.failed_checks > 0 {
        sb.push_str("\nFailed Checks:\n");
        for c in &result.checks {
            if c.status == "failed" {
                let _ = writeln!(sb, "\n  \u{274C} {}", c.name);
                if !c.expected.is_empty() {
                    let _ = writeln!(sb, "     Expected: {}", c.expected);
                }
                if !c.actual.is_empty() {
                    let _ = writeln!(sb, "     Actual: {}", c.actual);
                }
                if !c.message.is_empty() {
                    let _ = writeln!(sb, "     Message: {}", c.message);
                }
            }
        }
    }

    if result.warning_checks > 0 {
        sb.push_str("\nWarnings:\n");
        for c in &result.checks {
            if c.status == "warning" {
                let _ = writeln!(sb, "\n  \u{26A0} {}", c.name);
                if !c.expected.is_empty() {
                    let _ = writeln!(sb, "     Expected: {}", c.expected);
                }
                if !c.actual.is_empty() {
                    let _ = writeln!(sb, "     Actual: {}", c.actual);
                }
                if !c.message.is_empty() {
                    let _ = writeln!(sb, "     Message: {}", c.message);
                }
            }
        }
    }

    if verbose {
        sb.push_str("\nAll Checks:\n");
        for c in &result.checks {
            let marker = match c.status.as_str() {
                "passed" => "\u{2713}",
                "warning" => "\u{26A0}",
                _ => "\u{274C}",
            };
            let _ = writeln!(sb, "  {marker} {}", c.name);
            if !c.message.is_empty() {
                let _ = writeln!(sb, "     {}", c.message);
            }
        }
    }

    if !quiet {
        sb.push('\n');
        let _ = writeln!(sb, "Status: {}", status_banner(result));
    }

    sb
}

#[derive(Serialize)]
struct JsonOut<'a> {
    status: &'a str,
    timestamp: String,
    total_checks: usize,
    passed_checks: usize,
    warning_checks: usize,
    failed_checks: usize,
    duration_ms: i64,
    checks: Vec<JsonCheck<'a>>,
}

#[derive(Serialize)]
struct JsonCheck<'a> {
    name: &'a str,
    status: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    expected: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    actual: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    message: &'a str,
}

pub fn format_validation_json(result: &ValidationResult) -> std::result::Result<String, Error> {
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    let checks: Vec<JsonCheck> = result
        .checks
        .iter()
        .map(|c| JsonCheck {
            name: &c.name,
            status: &c.status,
            expected: &c.expected,
            actual: &c.actual,
            message: &c.message,
        })
        .collect();
    let out = JsonOut {
        status: status_json(result),
        timestamp,
        total_checks: result.total_checks,
        passed_checks: result.passed_checks,
        warning_checks: result.warning_checks,
        failed_checks: result.failed_checks,
        duration_ms: result.duration.as_millis() as i64,
        checks,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

pub fn format_validation_markdown(result: &ValidationResult, verbose: bool) -> String {
    let mut sb = String::new();
    sb.push_str("# Validation Results\n\n");
    sb.push_str("## Summary\n\n");
    let _ = writeln!(sb, "- **Total Checks**: {}", result.total_checks);
    let _ = writeln!(sb, "- **Passed**: {}", result.passed_checks);
    if result.warning_checks > 0 {
        let _ = writeln!(sb, "- **Warnings**: {}", result.warning_checks);
    }
    let _ = writeln!(sb, "- **Failed**: {}", result.failed_checks);
    let _ = writeln!(
        sb,
        "- **Duration**: {}\n",
        format_go_duration(result.duration)
    );

    if result.failed_checks > 0 {
        sb.push_str("## Failed Checks\n\n");
        for c in &result.checks {
            if c.status == "failed" {
                let _ = writeln!(sb, "### \u{274C} {}\n", c.name);
                if !c.expected.is_empty() {
                    let _ = writeln!(sb, "- **Expected**: {}", c.expected);
                }
                if !c.actual.is_empty() {
                    let _ = writeln!(sb, "- **Actual**: {}", c.actual);
                }
                if !c.message.is_empty() {
                    let _ = writeln!(sb, "- **Message**: {}", c.message);
                }
                sb.push('\n');
            }
        }
    }

    if result.warning_checks > 0 {
        sb.push_str("## Warnings\n\n");
        for c in &result.checks {
            if c.status == "warning" {
                let _ = writeln!(sb, "### \u{26A0} {}\n", c.name);
                if !c.expected.is_empty() {
                    let _ = writeln!(sb, "- **Expected**: {}", c.expected);
                }
                if !c.actual.is_empty() {
                    let _ = writeln!(sb, "- **Actual**: {}", c.actual);
                }
                if !c.message.is_empty() {
                    let _ = writeln!(sb, "- **Message**: {}", c.message);
                }
                sb.push('\n');
            }
        }
    }

    if verbose {
        sb.push_str("## All Checks\n\n");
        for c in &result.checks {
            let marker = match c.status.as_str() {
                "passed" => "\u{2713}",
                "warning" => "\u{26A0}",
                _ => "\u{274C}",
            };
            let _ = write!(sb, "- {marker} {}", c.name);
            if !c.message.is_empty() {
                let _ = write!(sb, " - {}", c.message);
            }
            sb.push('\n');
        }
        sb.push('\n');
    }

    let _ = writeln!(sb, "**Status**: {}", status_banner(result));

    sb
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::agents::types::ValidationCheck;
    use std::time::Duration;

    fn sample_result() -> ValidationResult {
        let mut r = ValidationResult::default();
        r.tally(ValidationCheck::passed("Agent: x.md - YAML Syntax", "ok"));
        r.tally(ValidationCheck::warning(
            "Agent: x.md - Unknown Field: bogus",
            "Allow-listed",
            "Unknown field: bogus",
            "Not in allow list",
        ));
        r.tally(ValidationCheck::failed(
            "Agent: x.md - Required Fields",
            "All required fields present",
            "Missing: [name]",
            "Required fields missing",
        ));
        r.duration = Duration::from_secs(1);
        r
    }

    #[test]
    fn format_text_emits_status_failed_banner() {
        let r = sample_result();
        let s = format_validation_text(&r, false, false);
        assert!(s.contains("VALIDATION FAILED"));
        assert!(s.contains("Failed Checks"));
    }

    #[test]
    fn format_text_quiet_omits_banner() {
        let r = sample_result();
        let s = format_validation_text(&r, false, true);
        assert!(!s.contains("Validation Complete"));
        assert!(!s.contains("Status:"));
    }

    #[test]
    fn format_text_verbose_lists_all() {
        let r = sample_result();
        let s = format_validation_text(&r, true, false);
        assert!(s.contains("All Checks:"));
    }

    #[test]
    fn format_json_shape() {
        let r = sample_result();
        let s = format_validation_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["duration_ms"], 1000);
        assert_eq!(v["total_checks"], 3);
    }

    #[test]
    fn format_json_status_success() {
        let mut r = ValidationResult::default();
        r.tally(ValidationCheck::passed("ok", "msg"));
        let s = format_validation_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
    }

    #[test]
    fn format_json_status_warning() {
        let mut r = ValidationResult::default();
        r.tally(ValidationCheck::warning("w", "e", "a", "m"));
        let s = format_validation_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "warning");
    }

    #[test]
    fn format_markdown_banner() {
        let r = sample_result();
        let s = format_validation_markdown(&r, false);
        assert!(s.contains("VALIDATION FAILED"));
        assert!(s.contains("## Failed Checks"));
        assert!(s.contains("## Warnings"));
    }

    #[test]
    fn format_markdown_verbose_includes_all_checks() {
        let r = sample_result();
        let s = format_validation_markdown(&r, true);
        assert!(s.contains("## All Checks"));
    }

    #[test]
    fn format_go_duration_zero() {
        assert_eq!(format_go_duration(Duration::from_secs(0)), "0s");
    }

    #[test]
    fn format_go_duration_second() {
        assert_eq!(format_go_duration(Duration::from_secs(1)), "1s");
    }

    #[test]
    fn format_go_duration_ms() {
        assert_eq!(format_go_duration(Duration::from_millis(100)), "100ms");
    }

    #[test]
    fn format_go_duration_us() {
        assert_eq!(format_go_duration(Duration::from_micros(5)), "5\u{00B5}s");
    }

    #[test]
    fn format_go_duration_ns() {
        assert_eq!(format_go_duration(Duration::from_nanos(7)), "7ns");
    }

    #[test]
    fn format_go_duration_1500ms_renders_as_1_5s() {
        assert_eq!(format_go_duration(Duration::from_millis(1500)), "1.5s");
    }

    #[test]
    fn format_go_duration_minute() {
        assert_eq!(format_go_duration(Duration::from_secs(61)), "1m1s");
    }

    #[test]
    fn format_go_duration_hour() {
        assert_eq!(format_go_duration(Duration::from_secs(3661)), "1h1m1s");
    }
}
