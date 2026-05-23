// Port of `apps/rhino-cli/internal/doctor/reporter.go`.

use std::fmt::Write as _;

use serde::Serialize;

use super::{DoctorResult, Scope, ToolCheck, ToolStatus};

fn symbol_for(status: ToolStatus) -> &'static str {
    match status {
        ToolStatus::Ok => "\u{2713}",      // ✓
        ToolStatus::Warning => "\u{26A0}", // ⚠
        ToolStatus::Missing => "\u{2717}", // ✗
    }
}

fn display_version(c: &ToolCheck) -> String {
    if c.status == ToolStatus::Missing {
        return "not found".into();
    }
    if c.installed_version.is_empty() {
        return "(unknown)".into();
    }
    format!("v{}", c.installed_version)
}

fn overall_status(r: &DoctorResult) -> &'static str {
    if r.missing_count > 0 {
        "missing"
    } else if r.warn_count > 0 {
        "warning"
    } else {
        "ok"
    }
}

/// Mirror Go's time.Now().Format(time.RFC3339): local-zone, second precision, with offset.
fn rfc3339_now() -> String {
    chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string()
}

/// Mirror Go's `d.Round(time.Millisecond)` formatted with `%v`.
fn format_go_duration_ms_rounded(d: std::time::Duration) -> String {
    let nanos_total = d.as_nanos() as i128;
    // Banker-free half-up rounding to the nearest millisecond, like Go's time.Duration.Round.
    let ms_rounded = (nanos_total + 500_000) / 1_000_000;
    let rounded = std::time::Duration::from_nanos((ms_rounded * 1_000_000) as u64);
    crate::internal::agents::reporter::format_go_duration(rounded)
}

/// Human-readable text.
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

#[derive(Serialize)]
struct JsonToolItem<'a> {
    name: &'a str,
    binary: &'a str,
    status: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    installed_version: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    required_version: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    source: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    note: &'a str,
}

#[derive(Serialize)]
struct JsonOutput<'a> {
    status: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    scope: &'a str,
    timestamp: String,
    ok_count: usize,
    warn_count: usize,
    missing_count: usize,
    duration_ms: u64,
    tools: Vec<JsonToolItem<'a>>,
}

/// JSON output. Returns serde error path through anyhow.
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
        duration_ms: result.duration.as_millis() as u64,
        tools,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

/// Markdown report.
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
    let _ = writeln!(sb, "| Total | {} |", total);
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
mod tests {
    use super::*;
    use std::time::Duration;

    fn sample() -> DoctorResult {
        DoctorResult {
            checks: vec![
                ToolCheck {
                    name: "git".into(),
                    binary: "git".into(),
                    status: ToolStatus::Ok,
                    installed_version: "2.42.0".into(),
                    required_version: "".into(),
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
                    installed_version: "".into(),
                    required_version: "".into(),
                    source: "".into(),
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
