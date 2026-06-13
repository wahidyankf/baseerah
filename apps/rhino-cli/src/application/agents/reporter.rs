//! Validation reporter ported from
//! `apps/rhino-cli/internal/agents/reporter.go` (validation-only paths).

use std::fmt::Write as _;
use std::time::Duration;

use anyhow::Error;
use chrono::Local;
use serde::Serialize;

use super::sync::SyncResult;
use super::types::ValidationResult;

/// Return the human-readable status banner string for a validation result.
fn status_banner(result: &ValidationResult) -> &'static str {
    if result.failed_checks > 0 {
        "\u{274C} VALIDATION FAILED"
    } else if result.warning_checks > 0 {
        "\u{26A0} VALIDATION PASSED WITH WARNINGS"
    } else {
        "\u{2713} VALIDATION PASSED"
    }
}

/// Return the machine-readable status string (`"success"`, `"warning"`, or `"failure"`) for JSON output.
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

/// Format a sub-second nanosecond value as a decimal fraction for `format_go_duration`.
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
        let frac_str = format!("{frac:0width$}");
        let trimmed = trim_trailing_zeros(&frac_str);
        if !trimmed.is_empty() {
            s.push('.');
            s.push_str(&trimmed);
        }
    }
    s
}

/// Remove trailing `'0'` characters from a numeric string.
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

/// JSON envelope for a full validation result.
#[derive(Serialize)]
struct JsonOut<'a> {
    /// Overall status: `"success"`, `"warning"`, or `"failure"`.
    status: &'a str,
    /// ISO-8601 timestamp of when the report was generated.
    timestamp: String,
    /// Total number of checks run.
    total_checks: usize,
    /// Number of passed checks.
    passed_checks: usize,
    /// Number of warning checks.
    warning_checks: usize,
    /// Number of failed checks.
    failed_checks: usize,
    /// Duration of the validation run in milliseconds.
    duration_ms: i64,
    /// Per-check detail records.
    checks: Vec<JsonCheck<'a>>,
}

/// JSON representation of a single validation check.
#[derive(Serialize)]
struct JsonCheck<'a> {
    /// Check identifier string.
    name: &'a str,
    /// Check status: `"passed"`, `"warning"`, or `"failed"`.
    status: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    /// What was expected (omitted when empty).
    expected: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    /// What was observed (omitted when empty).
    actual: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    /// Human-readable message (omitted when empty).
    message: &'a str,
}

/// Format a validation result as pretty-printed JSON.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
///
/// # Panics
///
/// Panics if the validation duration in milliseconds overflows `i64`, which
/// cannot happen for any realistic duration.
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
        duration_ms: i64::try_from(result.duration.as_millis()).expect("duration fits in i64"),
        checks,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

/// Format a validation result as a Markdown report.
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

/// Format a sync result as plain text.
pub fn format_sync_text(result: &SyncResult, verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();
    if !quiet {
        sb.push_str("Sync Complete\n");
        sb.push_str(&"=".repeat(50));
        sb.push_str("\n\n");
    }
    let _ = write!(sb, "Agents: {} converted", result.agents_converted);
    if result.agents_failed > 0 {
        let _ = write!(sb, ", {} failed", result.agents_failed);
    }
    sb.push('\n');
    let _ = write!(sb, "Skills: {} copied", result.skills_copied);
    if result.skills_failed > 0 {
        let _ = write!(sb, ", {} failed", result.skills_failed);
    }
    sb.push('\n');
    let _ = writeln!(sb, "Duration: {}", format_go_duration(result.duration));

    if !result.failed_files.is_empty() {
        sb.push_str("\nFailed Files:\n");
        for f in &result.failed_files {
            let _ = writeln!(sb, "  - {f}");
        }
    }

    if !quiet {
        sb.push('\n');
        if result.failed_files.is_empty() {
            sb.push_str("Status: \u{2713} SUCCESS\n");
        } else {
            sb.push_str("Status: \u{274C} FAILED\n");
        }
    }

    if verbose && !result.warnings.is_empty() {
        sb.push_str("\nWarnings:\n");
        for w in &result.warnings {
            let _ = writeln!(
                sb,
                "  \u{26A0} {}: dropped field \"{}\" ({})",
                w.agent_name, w.field, w.reason
            );
        }
    }
    sb
}

/// JSON envelope for a sync operation result.
#[derive(Serialize)]
struct SyncJsonOut<'a> {
    /// Overall status: `"success"` or `"failure"`.
    status: &'a str,
    /// ISO-8601 timestamp of when the report was generated.
    timestamp: String,
    /// Number of agents successfully converted.
    agents_converted: usize,
    /// Number of agents that failed to convert.
    agents_failed: usize,
    /// Number of skills successfully copied (always 0 — skills are not copied).
    skills_copied: usize,
    /// Number of skills that failed to copy (always 0).
    skills_failed: usize,
    /// Filenames of files that failed during sync.
    failed_files: &'a [String],
    /// Per-field conversion warnings.
    warnings: Vec<SyncJsonWarning<'a>>,
    /// Duration of the sync operation in milliseconds.
    duration_ms: i64,
}

/// JSON representation of a single conversion warning.
#[derive(Serialize)]
struct SyncJsonWarning<'a> {
    /// Name of the agent the warning belongs to.
    agent: &'a str,
    /// Field key that was dropped or translated.
    field: &'a str,
    /// Human-readable reason for the warning.
    reason: &'a str,
}

/// Format a sync result as pretty-printed JSON.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
///
/// # Panics
///
/// Panics if the sync duration in milliseconds overflows `i64`, which cannot
/// happen for any realistic duration.
pub fn format_sync_json(result: &SyncResult) -> std::result::Result<String, Error> {
    let status = if result.failed_files.is_empty() {
        "success"
    } else {
        "failure"
    };
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    let warnings: Vec<SyncJsonWarning> = result
        .warnings
        .iter()
        .map(|w| SyncJsonWarning {
            agent: &w.agent_name,
            field: &w.field,
            reason: &w.reason,
        })
        .collect();
    let out = SyncJsonOut {
        status,
        timestamp,
        agents_converted: result.agents_converted,
        agents_failed: result.agents_failed,
        skills_copied: result.skills_copied,
        skills_failed: result.skills_failed,
        failed_files: &result.failed_files,
        warnings,
        duration_ms: i64::try_from(result.duration.as_millis()).expect("duration fits in i64"),
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

/// Format a sync result as a Markdown report.
pub fn format_sync_markdown(result: &SyncResult) -> String {
    let mut sb = String::new();
    sb.push_str("# Sync Results\n\n");
    sb.push_str("## Summary\n\n");
    let _ = writeln!(sb, "- **Agents Converted**: {}", result.agents_converted);
    if result.agents_failed > 0 {
        let _ = writeln!(sb, "- **Agents Failed**: {}", result.agents_failed);
    }
    let _ = writeln!(sb, "- **Skills Copied**: {}", result.skills_copied);
    if result.skills_failed > 0 {
        let _ = writeln!(sb, "- **Skills Failed**: {}", result.skills_failed);
    }
    let _ = writeln!(
        sb,
        "- **Duration**: {}\n",
        format_go_duration(result.duration)
    );
    if !result.failed_files.is_empty() {
        sb.push_str("## Failed Files\n\n");
        for f in &result.failed_files {
            let _ = writeln!(sb, "- `{f}`");
        }
        sb.push('\n');
    }
    if result.failed_files.is_empty() {
        sb.push_str("**Status**: \u{2713} SUCCESS\n");
    } else {
        sb.push_str("**Status**: \u{274C} FAILED\n");
    }
    if !result.warnings.is_empty() {
        sb.push_str("\n## Warnings\n\n");
        for w in &result.warnings {
            let _ = writeln!(
                sb,
                "- \u{26A0} `{}`: dropped field `{}` ({})",
                w.agent_name, w.field, w.reason
            );
        }
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
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

    fn sync_sample(failed: bool) -> SyncResult {
        let mut r = SyncResult {
            agents_converted: 5,
            agents_failed: 0,
            skills_copied: 3,
            skills_failed: 0,
            failed_files: vec![],
            warnings: vec![],
            duration: Duration::from_secs(1),
        };
        if failed {
            r.agents_failed = 1;
            r.skills_failed = 1;
            r.failed_files.push("broken.md".to_string());
        }
        r
    }

    #[test]
    fn format_sync_text_success() {
        let s = format_sync_text(&sync_sample(false), false, false);
        assert!(s.contains("Sync Complete"));
        assert!(s.contains("Agents: 5 converted"));
        assert!(s.contains("Skills: 3 copied"));
        assert!(s.contains("SUCCESS"));
    }

    #[test]
    fn format_sync_text_failed() {
        let s = format_sync_text(&sync_sample(true), false, false);
        assert!(s.contains("FAILED"));
        assert!(s.contains("broken.md"));
        assert!(s.contains("Agents: 5 converted, 1 failed"));
        assert!(s.contains("Skills: 3 copied, 1 failed"));
    }

    #[test]
    fn format_sync_text_quiet_omits_banner() {
        let s = format_sync_text(&sync_sample(false), false, true);
        assert!(!s.contains("Sync Complete"));
        assert!(!s.contains("Status:"));
    }

    #[test]
    fn format_sync_text_verbose_lists_warnings() {
        let mut r = sync_sample(false);
        r.warnings
            .push(crate::internal::agents::converter::ConversionWarning {
                agent_name: "foo".into(),
                field: "mcpServers".into(),
                reason: "no opencode equivalent".into(),
            });
        let s = format_sync_text(&r, true, false);
        assert!(s.contains("Warnings:"));
        assert!(s.contains("foo"));
    }

    #[test]
    fn format_sync_json_success() {
        let s = format_sync_json(&sync_sample(false)).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["agents_converted"], 5);
        assert_eq!(v["duration_ms"], 1000);
    }

    #[test]
    fn format_sync_json_failure() {
        let s = format_sync_json(&sync_sample(true)).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["failed_files"][0], "broken.md");
    }

    #[test]
    fn format_sync_markdown_success() {
        let s = format_sync_markdown(&sync_sample(false));
        assert!(s.contains("# Sync Results"));
        assert!(s.contains("**Agents Converted**: 5"));
        assert!(s.contains("SUCCESS"));
    }

    #[test]
    fn format_sync_markdown_failure_with_warnings() {
        let mut r = sync_sample(true);
        r.warnings
            .push(crate::internal::agents::converter::ConversionWarning {
                agent_name: "agent-a".into(),
                field: "memory".into(),
                reason: "claude-only".into(),
            });
        let s = format_sync_markdown(&r);
        assert!(s.contains("FAILED"));
        assert!(s.contains("## Warnings"));
        assert!(s.contains("agent-a"));
    }

    #[test]
    fn use_super_conversion_warning() {
        // Re-exported via reporter format_sync_* path.
        use crate::internal::agents::converter::ConversionWarning;
        let w = ConversionWarning {
            agent_name: "x".into(),
            field: "y".into(),
            reason: "z".into(),
        };
        assert_eq!(w.field, "y");
    }
}
