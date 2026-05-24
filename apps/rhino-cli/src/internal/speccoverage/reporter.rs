// Byte-for-byte port of `apps/rhino-cli/internal/speccoverage/reporter.go`.
// Text / JSON / Markdown output formats — same exact strings so shadow-diff
// against the Go binary passes.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use super::types::{CheckResult, StepGap};

pub fn format_text(r: &CheckResult, _verbose: bool, quiet: bool) -> String {
    let has_gaps = !r.gaps.is_empty()
        || !r.scenario_gaps.is_empty()
        || !r.step_gaps.is_empty()
        || !r.orphan_step_impls.is_empty();

    let mut out = String::new();
    if !has_gaps {
        if !quiet {
            let _ = writeln!(
                out,
                "✓ Spec coverage valid! {} specs, {} scenarios, {} steps — all covered.",
                r.total_specs, r.total_scenarios, r.total_steps
            );
        }
        return out;
    }

    out.push_str("✗ Spec coverage gaps found!\n\n");

    if !r.gaps.is_empty() {
        let _ = writeln!(out, "Missing test files ({}):", r.gaps.len());
        for gap in &r.gaps {
            let _ = writeln!(
                out,
                "  - {}\n    (expected test file with stem: {})",
                gap.spec_file, gap.stem
            );
        }
    }

    if !r.scenario_gaps.is_empty() {
        if !r.gaps.is_empty() {
            out.push('\n');
        }
        let _ = writeln!(out, "Missing scenarios ({}):", r.scenario_gaps.len());
        for sg in &r.scenario_gaps {
            let _ = writeln!(
                out,
                "  - {}\n    → Scenario: \"{}\"",
                sg.spec_file, sg.scenario_title
            );
        }
    }

    if !r.step_gaps.is_empty() {
        if !r.gaps.is_empty() || !r.scenario_gaps.is_empty() {
            out.push('\n');
        }
        let _ = writeln!(out, "Missing steps ({}):", r.step_gaps.len());

        // Group by (spec_file, scenario_title) preserving first-seen order.
        let mut order: Vec<(String, String)> = Vec::new();
        let mut groups: BTreeMap<(String, String), Vec<&StepGap>> = BTreeMap::new();
        for sg in &r.step_gaps {
            let k = (sg.spec_file.clone(), sg.scenario_title.clone());
            if !groups.contains_key(&k) {
                order.push(k.clone());
            }
            groups.entry(k).or_default().push(sg);
        }
        for k in &order {
            let _ = writeln!(out, "  - {}\n    → Scenario: \"{}\"", k.0, k.1);
            if let Some(gs) = groups.get(k) {
                for sg in gs {
                    let _ = writeln!(out, "      · {} {}", sg.step_keyword, sg.step_text);
                }
            }
        }
    }

    if !r.orphan_step_impls.is_empty() {
        if !r.gaps.is_empty() || !r.scenario_gaps.is_empty() || !r.step_gaps.is_empty() {
            out.push('\n');
        }
        let _ = writeln!(
            out,
            "Orphan step implementations ({}) — no Gherkin step matches:",
            r.orphan_step_impls.len()
        );
        for o in &r.orphan_step_impls {
            let _ = writeln!(
                out,
                "  - {}\n      [{}] {}",
                o.file, o.matcher_kind, o.matcher_text
            );
        }
    }

    out
}

#[derive(Serialize)]
struct JsonOutput<'a> {
    status: &'static str,
    timestamp: String,
    total_specs: usize,
    total_scenarios: usize,
    total_steps: usize,
    gap_count: usize,
    scenario_gap_count: usize,
    step_gap_count: usize,
    orphan_step_impl_count: usize,
    duration_ms: i64,
    gaps: Vec<JsonGap<'a>>,
    scenario_gaps: Vec<JsonScenGap<'a>>,
    step_gaps: Vec<JsonStepGap<'a>>,
    orphan_step_impls: Vec<JsonOrphanImpl<'a>>,
}

#[derive(Serialize)]
struct JsonGap<'a> {
    spec_file: &'a str,
    stem: &'a str,
}

#[derive(Serialize)]
struct JsonScenGap<'a> {
    spec_file: &'a str,
    scenario_title: &'a str,
}

#[derive(Serialize)]
struct JsonStepGap<'a> {
    spec_file: &'a str,
    scenario_title: &'a str,
    keyword: &'a str,
    step_text: &'a str,
}

#[derive(Serialize)]
struct JsonOrphanImpl<'a> {
    file: &'a str,
    matcher_kind: &'a str,
    matcher_text: &'a str,
}

pub fn format_json(r: &CheckResult) -> std::result::Result<String, Error> {
    let status = if r.gaps.is_empty()
        && r.scenario_gaps.is_empty()
        && r.step_gaps.is_empty()
        && r.orphan_step_impls.is_empty()
    {
        "success"
    } else {
        "failure"
    };

    let timestamp = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string();

    let gaps: Vec<JsonGap> = r
        .gaps
        .iter()
        .map(|g| JsonGap {
            spec_file: &g.spec_file,
            stem: &g.stem,
        })
        .collect();
    let scenario_gaps: Vec<JsonScenGap> = r
        .scenario_gaps
        .iter()
        .map(|sg| JsonScenGap {
            spec_file: &sg.spec_file,
            scenario_title: &sg.scenario_title,
        })
        .collect();
    let step_gaps: Vec<JsonStepGap> = r
        .step_gaps
        .iter()
        .map(|sg| JsonStepGap {
            spec_file: &sg.spec_file,
            scenario_title: &sg.scenario_title,
            keyword: &sg.step_keyword,
            step_text: &sg.step_text,
        })
        .collect();
    let orphan_step_impls: Vec<JsonOrphanImpl> = r
        .orphan_step_impls
        .iter()
        .map(|o| JsonOrphanImpl {
            file: &o.file,
            matcher_kind: &o.matcher_kind,
            matcher_text: &o.matcher_text,
        })
        .collect();

    let out = JsonOutput {
        status,
        timestamp,
        total_specs: r.total_specs,
        total_scenarios: r.total_scenarios,
        total_steps: r.total_steps,
        gap_count: r.gaps.len(),
        scenario_gap_count: r.scenario_gaps.len(),
        step_gap_count: r.step_gaps.len(),
        orphan_step_impl_count: r.orphan_step_impls.len(),
        duration_ms: i64::try_from(r.duration.as_millis()).expect("duration fits in i64"),
        gaps,
        scenario_gaps,
        step_gaps,
        orphan_step_impls,
    };

    Ok(serde_json::to_string_pretty(&out)?)
}

/// Markdown format intentionally delegates to text — the text format is
/// already markdown-compatible. Mirrors Go's `FormatMarkdown`.
pub fn format_markdown(r: &CheckResult) -> String {
    format_text(r, false, false)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::internal::speccoverage::types::{
        CoverageGap, OrphanStepImpl, ScenarioGap, StepGap as StepGapT,
    };
    use std::time::Duration;

    fn empty_result() -> CheckResult {
        CheckResult {
            total_specs: 3,
            total_scenarios: 10,
            total_steps: 25,
            gaps: Vec::new(),
            scenario_gaps: Vec::new(),
            step_gaps: Vec::new(),
            orphan_step_impls: Vec::new(),
            duration: Duration::from_millis(123),
        }
    }

    #[test]
    fn format_text_no_gaps_emits_check_message() {
        let r = empty_result();
        let s = format_text(&r, false, false);
        assert_eq!(
            s,
            "✓ Spec coverage valid! 3 specs, 10 scenarios, 25 steps — all covered.\n"
        );
    }

    #[test]
    fn format_text_no_gaps_quiet_is_empty() {
        let r = empty_result();
        let s = format_text(&r, false, true);
        assert!(s.is_empty());
    }

    #[test]
    fn format_text_with_gaps_lists_them() {
        let mut r = empty_result();
        r.gaps.push(CoverageGap {
            spec_file: "specs/x.feature".to_string(),
            stem: "x".to_string(),
        });
        r.scenario_gaps.push(ScenarioGap {
            spec_file: "specs/y.feature".to_string(),
            scenario_title: "Some Title".to_string(),
        });
        r.step_gaps.push(StepGapT {
            spec_file: "specs/z.feature".to_string(),
            scenario_title: "Scenario A".to_string(),
            step_keyword: "Given".to_string(),
            step_text: "missing step".to_string(),
        });
        r.orphan_step_impls.push(OrphanStepImpl {
            file: "src/orphan.ts".to_string(),
            matcher_kind: "exact".to_string(),
            matcher_text: "orphaned".to_string(),
        });
        let s = format_text(&r, false, false);
        assert!(s.starts_with("✗ Spec coverage gaps found!"));
        assert!(s.contains("Missing test files (1)"));
        assert!(s.contains("specs/x.feature"));
        assert!(s.contains("Missing scenarios (1)"));
        assert!(s.contains("\"Some Title\""));
        assert!(s.contains("Missing steps (1)"));
        assert!(s.contains("Given missing step"));
        assert!(s.contains("Orphan step implementations (1)"));
        assert!(s.contains("[exact] orphaned"));
    }

    #[test]
    fn format_json_status_success_on_clean() {
        let r = empty_result();
        let s = format_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["total_specs"], 3);
        assert_eq!(v["gap_count"], 0);
    }

    #[test]
    fn format_json_status_failure_on_any_gap() {
        let mut r = empty_result();
        r.scenario_gaps.push(ScenarioGap {
            spec_file: "x".to_string(),
            scenario_title: "T".to_string(),
        });
        let s = format_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["scenario_gap_count"], 1);
    }

    #[test]
    fn format_markdown_delegates_to_text() {
        let r = empty_result();
        let md = format_markdown(&r);
        let txt = format_text(&r, false, false);
        assert_eq!(md, txt);
    }
}
