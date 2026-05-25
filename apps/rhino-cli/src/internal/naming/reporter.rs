//! Shared naming-violation reporter used by `agents validate-naming` and
//! `workflows validate-naming`.
//!
//! Mirrors `formatNamingText/JSON/Markdown` in
//! `apps/rhino-cli/cmd/agents_validate_naming.go`.

use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use crate::internal::naming::Violation;

/// Formats a list of [`Violation`]s as human-readable plain text.
///
/// When `violations` is empty and `quiet` is `false` a "VALIDATION PASSED"
/// line is emitted.  When `quiet` is `true` and there are no violations the
/// returned string is empty.  When `verbose` is `true` a normative-reference
/// note is appended after the violation list.
pub fn format_text(label: &str, violations: &[Violation], verbose: bool, quiet: bool) -> String {
    let mut b = String::new();
    if violations.is_empty() {
        if !quiet {
            let _ = writeln!(
                b,
                "{label} naming validation: VALIDATION PASSED (0 violations)"
            );
        }
        return b;
    }
    let _ = writeln!(
        b,
        "{label} naming validation: {} violation(s)",
        violations.len()
    );
    for v in violations {
        let _ = writeln!(b, "  [{}] {} — {}", v.kind, v.path, v.message);
    }
    if verbose {
        b.push_str("\nSee repo-governance/conventions/structure/agent-naming.md (or workflow-naming.md) for the normative rule.\n");
    }
    b
}

/// JSON serialisation shape for a single [`Violation`].
#[derive(Serialize)]
struct JsonViolation<'a> {
    /// Relative path of the offending file.
    #[serde(rename = "Path")]
    path: &'a str,
    /// Machine-readable violation category.
    #[serde(rename = "Kind")]
    kind: &'a str,
    /// Human-readable violation description.
    #[serde(rename = "Message")]
    message: &'a str,
}

/// Top-level JSON output envelope.
#[derive(Serialize)]
struct JsonOut<'a> {
    /// The audit kind (e.g., `"agents"` or `"workflows"`).
    kind: &'a str,
    /// All violations found.
    violations: Vec<JsonViolation<'a>>,
    /// Total violation count (convenience field).
    count: usize,
}

/// Serialises `violations` to a pretty-printed JSON string.
///
/// The returned string is terminated with a newline.
///
/// # Errors
///
/// Returns an error when [`serde_json`] fails to serialise the output
/// envelope — in practice this should not occur for the types involved.
pub fn format_json(kind: &str, violations: &[Violation]) -> std::result::Result<String, Error> {
    let jv: Vec<JsonViolation> = violations
        .iter()
        .map(|v| JsonViolation {
            path: &v.path,
            kind: &v.kind,
            message: &v.message,
        })
        .collect();
    let out = JsonOut {
        kind,
        violations: jv,
        count: violations.len(),
    };
    let mut s = serde_json::to_string_pretty(&out)?;
    s.push('\n');
    Ok(s)
}

/// Formats `violations` as a Markdown section.
///
/// Emits a `## <label> naming validation` heading followed either by a clean
/// message (no violations) or a GFM table with `Kind`, `Path`, and `Message`
/// columns.
pub fn format_markdown(label: &str, violations: &[Violation]) -> String {
    let mut b = String::new();
    let _ = writeln!(b, "## {label} naming validation\n");
    if violations.is_empty() {
        b.push_str("All files conform to the naming convention.\n");
        return b;
    }
    b.push_str("| Kind | Path | Message |\n");
    b.push_str("| --- | --- | --- |\n");
    for v in violations {
        let _ = writeln!(b, "| {} | {} | {} |", v.kind, v.path, v.message);
    }
    b
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> Violation {
        Violation {
            path: ".claude/agents/foo.md".to_string(),
            kind: "role-suffix".to_string(),
            message: "bad".to_string(),
        }
    }

    #[test]
    fn format_text_passed_non_quiet() {
        let s = format_text("Agents", &[], false, false);
        assert!(s.contains("VALIDATION PASSED"));
    }

    #[test]
    fn format_text_passed_quiet_empty() {
        let s = format_text("Agents", &[], false, true);
        assert!(s.is_empty());
    }

    #[test]
    fn format_text_failed_lists() {
        let s = format_text("Agents", std::slice::from_ref(&sample()), false, false);
        assert!(s.contains("Agents naming validation: 1 violation"));
        assert!(s.contains("role-suffix"));
    }

    #[test]
    fn format_text_verbose_adds_normative_note() {
        let s = format_text("Agents", std::slice::from_ref(&sample()), true, false);
        assert!(s.contains("normative rule"));
    }

    #[test]
    fn format_json_carries_kind_and_count() {
        let s = format_json("agents", std::slice::from_ref(&sample())).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["kind"], "agents");
        assert_eq!(v["count"], 1);
        assert_eq!(v["violations"][0]["Path"], ".claude/agents/foo.md");
    }

    #[test]
    fn format_markdown_clean_passes() {
        let s = format_markdown("Workflows", &[]);
        assert!(s.contains("All files conform"));
    }

    #[test]
    fn format_markdown_violations_emits_table() {
        let s = format_markdown("Agents", std::slice::from_ref(&sample()));
        assert!(s.contains("| Kind | Path | Message |"));
        assert!(s.contains("role-suffix"));
    }
}
