//! Port of `apps/rhino-cli/internal/doctor/reporter.go`.
//!
//! Formats a [`DoctorResult`] as human-readable text, JSON, or Markdown.
//! All three formatters are exposed as `pub` functions: [`format_text`],
//! [`format_json`], and [`format_markdown`].

use std::fmt::Write as _;

use serde::Serialize;

use super::{DoctorResult, Scope, ToolCheck, ToolStatus};

/// Returns the Unicode status symbol for `status`: U+2713 (ok), U+26A0
/// (warning), or U+2717 (missing).
fn symbol_for(status: ToolStatus) -> &'static str {
    match status {
        ToolStatus::Ok => "\u{2713}",      // check mark
        ToolStatus::Warning => "\u{26A0}", // warning sign
        ToolStatus::Missing => "\u{2717}", // ballot X
    }
}

/// Returns a display-friendly version string for the check.
///
/// Returns `"not found"` for missing tools, `"(unknown)"` when the installed
/// version is empty, or `"v<version>"` otherwise.
fn display_version(c: &ToolCheck) -> String {
    if c.status == ToolStatus::Missing {
        return "not found".into();
    }
    if c.installed_version.is_empty() {
        return "(unknown)".into();
    }
    format!("v{}", c.installed_version)
}

/// Returns the overall status code string for the run.
///
/// Priority: `"missing"` > `"warning"` > `"ok"`.
fn overall_status(r: &DoctorResult) -> &'static str {
    if r.missing_count > 0 {
        "missing"
    } else if r.warn_count > 0 {
        "warning"
    } else {
        "ok"
    }
}

/// Returns the current local time formatted as RFC 3339 with second precision.
///
/// Mirrors Go's `time.Now().Format(time.RFC3339)`: local timezone with UTC offset.
fn rfc3339_now() -> String {
    chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string()
}

/// Rounds `d` to the nearest millisecond and formats it using Go's `%v` duration style.
///
/// Mirrors Go's `d.Round(time.Millisecond)` + `fmt.Sprintf("%v", d)`.
///
/// # Panics
///
/// Panics if the total nanoseconds of `d` do not fit in `i128` or if the
/// rounded nanosecond count does not fit in `u64` — both are impossible for
/// any `Duration` representable in Rust.
fn format_go_duration_ms_rounded(d: std::time::Duration) -> String {
    let nanos_total = i128::try_from(d.as_nanos()).expect("nanos fit in i128");
    // Banker-free half-up rounding to the nearest millisecond, like Go's time.Duration.Round.
    let ms_rounded = (nanos_total + 500_000) / 1_000_000;
    let rounded = std::time::Duration::from_nanos(
        u64::try_from(ms_rounded * 1_000_000).expect("rounded nanos fit in u64"),
    );
    crate::internal::agents::reporter::format_go_duration(rounded)
}

/// Formats `result` as human-readable text.
///
/// When `quiet` is `true`, the `"Doctor Report"` header is suppressed.
/// When `verbose` is `true`, the elapsed duration is appended.
pub fn format_text(result: &DoctorResult, verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();

    if !quiet {
        sb.push_str("Doctor Report\n");
        sb.push_str("=============\n\n");
    }

    for check in &result.checks {
        let sym = symbol_for(check.status);
        let ver = display_version(check);
        let _ = writeln!(sb, "{sym} {:<10} {:<14} ({})", check.name, ver, check.note);
    }

    let total = result.ok_count + result.warn_count + result.missing_count;
    let mut summary = format!(
        "\nSummary: {}/{} tools OK, {} warning, {} missing",
        result.ok_count, total, result.warn_count, result.missing_count
    );
    if result.scope == Scope::Minimal {
        summary.push_str(" (scope: minimal)");
    }
    sb.push_str(&summary);
    sb.push('\n');

    if verbose {
        let _ = writeln!(
            sb,
            "Duration: {}",
            format_go_duration_ms_rounded(result.duration)
        );
    }

    sb
}

/// JSON representation of a single tool check result.
#[derive(Serialize)]
struct JsonToolItem<'a> {
    /// Human-readable tool name.
    name: &'a str,
    /// Binary name that is invoked.
    binary: &'a str,
    /// Status code string (`"ok"`, `"warning"`, or `"missing"`).
    status: &'static str,
    /// Installed version string (omitted when empty).
    #[serde(skip_serializing_if = "str::is_empty")]
    installed_version: &'a str,
    /// Required version string (omitted when empty).
    #[serde(skip_serializing_if = "str::is_empty")]
    required_version: &'a str,
    /// Config file providing the required version (omitted when empty).
    #[serde(skip_serializing_if = "str::is_empty")]
    source: &'a str,
    /// Human-readable status explanation (omitted when empty).
    #[serde(skip_serializing_if = "str::is_empty")]
    note: &'a str,
}

/// Top-level JSON document for the doctor report.
#[derive(Serialize)]
struct JsonOutput<'a> {
    /// Overall status code string.
    status: &'static str,
    /// Scope code string (omitted when empty / `"full"` is the default).
    #[serde(skip_serializing_if = "str::is_empty")]
    scope: &'a str,
    /// RFC 3339 timestamp of the report.
    timestamp: String,
    /// Number of tools with `Ok` status.
    ok_count: usize,
    /// Number of tools with `Warning` status.
    warn_count: usize,
    /// Number of tools with `Missing` status.
    missing_count: usize,
    /// Wall-clock duration of the check in milliseconds.
    duration_ms: u64,
    /// Per-tool results.
    tools: Vec<JsonToolItem<'a>>,
}

/// Serialises `result` to a pretty-printed JSON string.
///
/// # Errors
///
/// Returns an error when `serde_json` fails to serialise the result
/// (in practice this should not occur for this fixed schema).
///
/// # Panics
///
/// Panics if the elapsed duration in milliseconds does not fit in `u64`,
/// which cannot happen for any `Duration` representable in Rust.
pub fn format_json(result: &DoctorResult) -> anyhow::Result<String> {
    let tools: Vec<JsonToolItem> = result
        .checks
        .iter()
        .map(|c| JsonToolItem {
            name: &c.name,
            binary: &c.binary,
            status: c.status.code(),
            installed_version: &c.installed_version,
            required_version: &c.required_version,
            source: &c.source,
            note: &c.note,
        })
        .collect();

    let out = JsonOutput {
        status: overall_status(result),
        scope: result.scope.code(),
        timestamp: rfc3339_now(),
        ok_count: result.ok_count,
        warn_count: result.warn_count,
        missing_count: result.missing_count,
        duration_ms: u64::try_from(result.duration.as_millis()).expect("duration fits in u64"),
        tools,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

/// Formats `result` as a Markdown report with a summary table and a per-tool table.
pub fn format_markdown(result: &DoctorResult) -> String {
    let mut sb = String::new();
    sb.push_str("## Doctor Report\n\n");
    let _ = writeln!(sb, "**Generated**: {}\n", rfc3339_now());

    let total = result.ok_count + result.warn_count + result.missing_count;
    sb.push_str("### Summary\n\n");
    sb.push_str("| Metric | Value |\n");
    sb.push_str("|--------|-------|\n");
    let _ = writeln!(sb, "| OK | {} |", result.ok_count);
    let _ = writeln!(sb, "| Warning | {} |", result.warn_count);
    let _ = writeln!(sb, "| Missing | {} |", result.missing_count);
    let _ = writeln!(sb, "| Total | {total} |");
    sb.push('\n');

    sb.push_str("### Tools\n\n");
    sb.push_str("| Tool | Status | Installed | Required | Note |\n");
    sb.push_str("|------|--------|-----------|----------|------|\n");

    for c in &result.checks {
        let sym = symbol_for(c.status);
        let ver = display_version(c);
        let _ = writeln!(
            sb,
            "| {} | {} {} | {} | {} | {} |",
            c.name,
            sym,
            c.status.code(),
            ver,
            c.required_version,
            c.note
        );
    }

    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Builds a sample [`DoctorResult`] with one `Ok`, one `Warning`, and one `Missing` check.
    fn sample() -> DoctorResult {
        DoctorResult {
            checks: vec![
                ToolCheck {
                    name: "git".into(),
                    binary: "git".into(),
                    status: ToolStatus::Ok,
                    installed_version: "2.42.0".into(),
                    required_version: String::new(),
                    source: "(no config file)".into(),
                    note: "no version requirement".into(),
                },
                ToolCheck {
                    name: "node".into(),
                    binary: "node".into(),
                    status: ToolStatus::Warning,
                    installed_version: "22.0.0".into(),
                    required_version: "24.11.1".into(),
                    source: "package.json".into(),
                    note: "required: 24.11.1, version mismatch".into(),
                },
                ToolCheck {
                    name: "ghost".into(),
                    binary: "ghost".into(),
                    status: ToolStatus::Missing,
                    installed_version: String::new(),
                    required_version: String::new(),
                    source: String::new(),
                    note: "not found in PATH".into(),
                },
            ],
            ok_count: 1,
            warn_count: 1,
            missing_count: 1,
            duration: Duration::from_millis(42),
            scope: Scope::Full,
        }
    }

    #[test]
    fn text_contains_header_when_not_quiet() {
        let s = format_text(&sample(), false, false);
        assert!(s.contains("Doctor Report"));
        assert!(s.contains("git"));
        assert!(s.contains("Summary: 1/3 tools OK"));
    }

    #[test]
    fn text_omits_header_when_quiet() {
        let s = format_text(&sample(), false, true);
        assert!(!s.contains("Doctor Report"));
    }

    #[test]
    fn text_minimal_scope_marker() {
        let mut r = sample();
        r.scope = Scope::Minimal;
        let s = format_text(&r, false, false);
        assert!(s.contains("(scope: minimal)"));
    }

    #[test]
    fn text_verbose_includes_duration() {
        let s = format_text(&sample(), true, false);
        assert!(s.contains("Duration:"));
    }

    #[test]
    fn json_round_trips() {
        let s = format_json(&sample()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "missing");
        assert_eq!(v["scope"], "full");
        assert_eq!(v["ok_count"], 1);
        assert_eq!(v["tools"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn markdown_has_tables() {
        let s = format_markdown(&sample());
        assert!(s.contains("## Doctor Report"));
        assert!(s.contains("| Metric | Value |"));
        assert!(s.contains("| Tool | Status | Installed |"));
    }

    #[test]
    fn overall_status_priority() {
        let mut r = sample();
        r.missing_count = 0;
        let s = format_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "warning");
        r.warn_count = 0;
        let s2 = format_json(&r).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&s2).unwrap();
        assert_eq!(v2["status"], "ok");
    }
}
