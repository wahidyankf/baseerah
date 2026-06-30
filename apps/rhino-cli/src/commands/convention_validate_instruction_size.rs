//! `harness instruction-size validate` (cross-domain moved from `convention` domain in §2a-names) — checks instruction files against
//! their byte budgets defined in the `instruction-size:` section of `repo-config.yml`.
//!
//! Reads the per-surface and resolved-tree budgets, globs for each surface
//! file, classifies sizes, and returns exit code 1 when any file exceeds its
//! fail threshold.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::application::repo_config;
use crate::application::repo_governance::instruction_size::{
    BudgetConfig, Finding, ResolvedTree, Severity, Surface, check_instruction_sizes,
    check_resolved_tree, severity_label,
};
use crate::domain::cliout::OutputFormat;
use crate::internal::git;

/// JSON output schema identifier for this command.
pub const SCHEMA: &str = "rhino-cli/instruction-size/v1";

/// CLI arguments for `harness instruction-size validate` (cross-domain moved from `convention` domain in §2a-names) (none required).
#[derive(Args, Debug)]
pub struct InstructionSizeArgs;

// ---------------------------------------------------------------------------
// JSON serialization types
// ---------------------------------------------------------------------------

/// A single finding in the JSON envelope.
#[derive(Serialize)]
struct FindingPayload<'a> {
    /// Repo-relative path of the instruction file (or `"resolved-tree"`).
    path: &'a str,
    /// Measured size in bytes.
    size: u64,
    /// Target budget in bytes.
    target: u64,
    /// Warning threshold in bytes.
    warn: u64,
    /// Fail threshold in bytes.
    fail: u64,
    /// Severity label: `"ok"`, `"warn"`, or `"fail"`.
    severity: &'a str,
    /// Human-readable description.
    message: &'a str,
}

/// JSON envelope wrapping the instruction-size audit result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Summary counts.
    total_findings: usize,
    /// Individual findings.
    findings: Vec<FindingPayload<'a>>,
}

// ---------------------------------------------------------------------------
// Public command entry-point
// ---------------------------------------------------------------------------

/// Run the `harness instruction-size validate` (cross-domain moved from `convention` domain in §2a-names) command.
///
/// Discovers the git repository root, loads the budget configuration from
/// the `instruction-size:` section of `repo-config.yml`, and checks all surfaces.
///
/// # Errors
///
/// Returns an error when the git root cannot be found, the budget config
/// cannot be loaded, or any instruction file exceeds its fail budget.
pub fn run(
    _args: &InstructionSizeArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_for_root(&repo_root, output_format)
}

/// Default budget applied to registry instruction surfaces not covered by `instruction-size:` globs.
const REGISTRY_DEFAULT_TARGET: u64 = 10_000;
/// Warning threshold for registry surfaces without an explicit budget.
const REGISTRY_DEFAULT_WARN: u64 = 13_000;
/// Hard-fail threshold for registry surfaces without an explicit budget.
const REGISTRY_DEFAULT_FAIL: u64 = 16_000;

/// Core logic for `harness instruction-size validate` (cross-domain moved from `convention` domain in §2a-names), exposed for testing.
///
/// Merges two surface sources:
/// - `repo-config.yml` `instruction-size:` section (explicit per-surface budgets, optional).
/// - `repo-config.yml` `harness:` `instruction:` lists (registry-derived surfaces, optional).
///
/// Registry surfaces not covered by an `instruction-size:` glob receive default budget thresholds.
/// When neither source is available, the command skips gracefully.
///
/// # Errors
///
/// Returns an error when the budget config cannot be loaded or any instruction
/// file exceeds its fail budget.
pub fn run_for_root(
    repo_root: &Path,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    // Load instruction-size: section and harness registry from repo-config.yml.
    let harness_config = repo_config::load_or_default(repo_root);
    let registry_globs: Vec<String> = harness_config
        .harness
        .iter()
        .flat_map(|e| e.instruction.iter().cloned())
        .collect();
    let yaml_config = harness_config.instruction_size;

    // If neither source has any surfaces, skip gracefully.
    let yaml_has_surfaces = yaml_config.as_ref().is_some_and(|c| !c.surfaces.is_empty());
    if !yaml_has_surfaces && registry_globs.is_empty() {
        if output_format == OutputFormat::Text {
            println!(
                "INSTRUCTION SIZE: SKIPPED (no instruction-size: section in repo-config.yml \
                 and no harness instruction surfaces in repo-config.yml)"
            );
        }
        return Ok(());
    }

    // Build merged surface list: yaml surfaces first, then any registry glob not already covered.
    let mut merged_surfaces: Vec<Surface> = yaml_config
        .as_ref()
        .map(|c| c.surfaces.clone())
        .unwrap_or_default();
    let mut seen_globs: std::collections::HashSet<String> =
        merged_surfaces.iter().map(|s| s.glob.clone()).collect();
    for glob in &registry_globs {
        if seen_globs.insert(glob.clone()) {
            merged_surfaces.push(Surface {
                glob: glob.clone(),
                target: REGISTRY_DEFAULT_TARGET,
                warn: REGISTRY_DEFAULT_WARN,
                fail: REGISTRY_DEFAULT_FAIL,
            });
        }
    }

    // Build merged config (resolved_tree carried from yaml if present, else omit from findings).
    let merged_config = BudgetConfig {
        surfaces: merged_surfaces,
        resolved_tree: yaml_config.as_ref().map_or_else(
            || ResolvedTree {
                root: String::new(),
                target: u64::MAX,
                warn: u64::MAX,
                fail: u64::MAX,
            },
            |c| c.resolved_tree.clone(),
        ),
    };

    let mut findings = check_instruction_sizes(repo_root, &merged_config);
    if yaml_config.is_some()
        && let Some(tree_finding) = check_resolved_tree(repo_root, &merged_config)
    {
        findings.push(tree_finding);
    }

    let has_fail = findings.iter().any(|f| f.severity == Severity::Fail);

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings, &merged_config)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if has_fail {
        let fail_count = findings
            .iter()
            .filter(|f| f.severity == Severity::Fail)
            .count();
        return Err(anyhow!(
            "instruction-size audit failed: {fail_count} Fail finding(s); apply progressive disclosure \
             — see repo-governance/principles/content/progressive-disclosure.md"
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Formatters (pure functions — testable without I/O)
// ---------------------------------------------------------------------------

/// Format instruction-size findings as human-readable text.
fn format_text(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return "INSTRUCTION SIZE: PASSED — all surfaces within budget\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(sb, "INSTRUCTION SIZE: {} finding(s)", findings.len());
    for f in findings {
        let label = match f.severity {
            Severity::Ok => "PASS",
            Severity::Warn => "WARN",
            Severity::Fail => "FAIL",
        };
        let _ = writeln!(sb, "  [{}] {} — {}", label, f.path, f.message);
    }
    sb
}

/// Serialize instruction-size findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[Finding], _config: &BudgetConfig) -> std::result::Result<String, Error> {
    let has_fail = findings.iter().any(|f| f.severity == Severity::Fail);
    let status = if has_fail { "failed" } else { "passed" };
    let payloads: Vec<FindingPayload<'_>> = findings
        .iter()
        .map(|f| FindingPayload {
            path: &f.path,
            size: f.size,
            target: f.target,
            warn: f.warn,
            fail: f.fail,
            severity: severity_label(&f.severity),
            message: &f.message,
        })
        .collect();
    let env = Envelope {
        schema: SCHEMA,
        status,
        total_findings: findings.len(),
        findings: payloads,
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

/// Format instruction-size findings as a Markdown table.
fn format_markdown(findings: &[Finding]) -> String {
    let mut sb = String::new();
    sb.push_str("## Instruction Size Audit\n\n");
    if findings.is_empty() {
        sb.push_str("**PASSED**: all surfaces within budget\n");
        return sb;
    }
    let _ = writeln!(
        sb,
        "**{}**: {} finding(s)\n",
        if findings.iter().any(|f| f.severity == Severity::Fail) {
            "FAILED"
        } else {
            "WARN"
        },
        findings.len()
    );
    sb.push_str("| Path | Size (bytes) | Severity | Message |\n");
    sb.push_str("| --- | --- | --- | --- |\n");
    for f in findings {
        let sev = severity_label(&f.severity);
        let _ = writeln!(
            sb,
            "| `{}` | {} | {} | {} |",
            f.path, f.size, sev, f.message
        );
    }
    sb
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::application::repo_governance::instruction_size as is;
    use std::fs;
    use tempfile::TempDir;

    fn write_agents_md(dir: &Path, bytes: usize) {
        fs::write(dir.join("AGENTS.md"), "x".repeat(bytes)).unwrap();
    }

    fn write_budget_yaml(dir: &Path) {
        let yaml = concat!(
            "harness: []\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
            "instruction-size:\n",
            "  surfaces:\n",
            "    - glob: \"AGENTS.md\"\n",
            "      target: 24000\n",
            "      warn: 27000\n",
            "      fail: 30000\n",
            "  resolved_tree:\n",
            "    root: \"CLAUDE.md\"\n",
            "    target: 30000\n",
            "    warn: 34000\n",
            "    fail: 38000\n",
        );
        fs::write(dir.join("repo-config.yml"), yaml).unwrap();
    }

    fn write_small_claude(dir: &Path) {
        fs::write(dir.join("CLAUDE.md"), "small content\n").unwrap();
    }

    // ---- run_for_root ----

    #[test]
    fn run_returns_ok_when_within_budget() {
        let tmp = TempDir::new().unwrap();
        write_budget_yaml(tmp.path());
        write_agents_md(tmp.path(), 10_000);
        write_small_claude(tmp.path());
        let result = run_for_root(tmp.path(), OutputFormat::Text);
        assert!(result.is_ok(), "within-budget should pass: {result:?}");
    }

    #[test]
    fn run_returns_err_when_agents_md_exceeds_fail() {
        let tmp = TempDir::new().unwrap();
        write_budget_yaml(tmp.path());
        write_agents_md(tmp.path(), 31_000);
        write_small_claude(tmp.path());
        let result = run_for_root(tmp.path(), OutputFormat::Text);
        assert!(result.is_err(), "fail-budget exceeded should return Err");
    }

    #[test]
    fn run_returns_ok_when_no_instruction_size_section() {
        let tmp = TempDir::new().unwrap();
        // No instruction-size: section in repo-config.yml — should skip gracefully
        let result = run_for_root(tmp.path(), OutputFormat::Text);
        assert!(result.is_ok());
    }

    // ---- format_text ----

    #[test]
    fn format_text_passed_when_no_findings() {
        let s = format_text(&[]);
        assert!(s.contains("PASSED"));
    }

    #[test]
    fn format_text_shows_fail_findings() {
        let finding = Finding {
            path: "AGENTS.md".to_string(),
            size: 31_000,
            target: 24_000,
            warn: 27_000,
            fail: 30_000,
            severity: Severity::Fail,
            message: "AGENTS.md is too large; apply progressive disclosure — see repo-governance/principles/content/progressive-disclosure.md".to_string(),
        };
        let s = format_text(&[finding]);
        assert!(s.contains("[FAIL]"));
        assert!(s.contains("AGENTS.md"));
    }

    // ---- format_json ----

    #[test]
    fn format_json_passed() {
        let config = BudgetConfig {
            surfaces: vec![],
            resolved_tree: is::ResolvedTree {
                root: "CLAUDE.md".to_string(),
                target: 30_000,
                warn: 34_000,
                fail: 38_000,
            },
        };
        let s = format_json(&[], &config).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["schema"], SCHEMA);
        assert_eq!(v["status"], "passed");
        assert_eq!(v["total_findings"], 0);
    }

    #[test]
    fn format_json_failed_contains_finding() {
        let finding = Finding {
            path: "AGENTS.md".to_string(),
            size: 31_000,
            target: 24_000,
            warn: 27_000,
            fail: 30_000,
            severity: Severity::Fail,
            message: "too large; apply progressive disclosure — see repo-governance/principles/content/progressive-disclosure.md".to_string(),
        };
        let config = BudgetConfig {
            surfaces: vec![],
            resolved_tree: is::ResolvedTree {
                root: "CLAUDE.md".to_string(),
                target: 30_000,
                warn: 34_000,
                fail: 38_000,
            },
        };
        let s = format_json(&[finding], &config).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["total_findings"], 1);
        assert_eq!(v["findings"][0]["path"], "AGENTS.md");
        assert_eq!(v["findings"][0]["severity"], "fail");
    }

    // ---- format_markdown ----

    #[test]
    fn format_markdown_passed() {
        let s = format_markdown(&[]);
        assert!(s.contains("## Instruction Size Audit"));
        assert!(s.contains("PASSED"));
    }

    #[test]
    fn format_markdown_with_findings() {
        let finding = Finding {
            path: "AGENTS.md".to_string(),
            size: 31_000,
            target: 24_000,
            warn: 27_000,
            fail: 30_000,
            severity: Severity::Fail,
            message: "too large; apply progressive disclosure — see repo-governance/principles/content/progressive-disclosure.md".to_string(),
        };
        let s = format_markdown(&[finding]);
        assert!(s.contains("FAILED"));
        assert!(s.contains("`AGENTS.md`"));
        assert!(s.contains("fail"));
    }

    // ---- fail message contains progressive disclosure ----

    #[test]
    fn fail_message_in_run_contains_progressive_disclosure() {
        let tmp = TempDir::new().unwrap();
        write_budget_yaml(tmp.path());
        write_agents_md(tmp.path(), 31_000);
        write_small_claude(tmp.path());
        let err = run_for_root(tmp.path(), OutputFormat::Text).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("progressive disclosure"),
            "error must mention progressive disclosure: {msg}"
        );
        assert!(
            msg.contains("repo-governance/principles/content/progressive-disclosure.md"),
            "error must include governance path: {msg}"
        );
    }

    #[test]
    fn harness_registry_instruction_surface_is_budgeted() {
        let tmp = TempDir::new().unwrap();
        let repo_config = concat!(
            "harness:\n",
            "  - { name: cursor, tier: native, shadow: .cursor/rules,",
            " instruction: [.cursor/rules] }\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
        );
        fs::write(tmp.path().join("repo-config.yml"), repo_config).unwrap();
        fs::create_dir_all(tmp.path().join(".cursor")).unwrap();
        // Oversized .cursor/rules — must be flagged via registry surface list
        fs::write(tmp.path().join(".cursor/rules"), "x".repeat(50_000)).unwrap();
        // No instruction-size-budget.yaml — surfaces come from harness registry
        let result = run_for_root(tmp.path(), OutputFormat::Text);
        assert!(
            result.is_err(),
            "registry-driven instruction-size must flag oversized .cursor/rules"
        );
    }

    // ── instruction-size: section in repo-config.yml (RED → GREEN) ───────────

    #[test]
    fn run_reads_instruction_size_section_from_repo_config_yml() {
        let tmp = TempDir::new().unwrap();
        let repo_cfg = concat!(
            "harness: []\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
            "instruction-size:\n",
            "  surfaces:\n",
            "    - glob: \"AGENTS.md\"\n",
            "      target: 24000\n",
            "      warn: 27000\n",
            "      fail: 30000\n",
            "  resolved_tree:\n",
            "    root: \"CLAUDE.md\"\n",
            "    target: 30000\n",
            "    warn: 34000\n",
            "    fail: 38000\n",
        );
        fs::write(tmp.path().join("repo-config.yml"), repo_cfg).unwrap();
        // NO standalone instruction-size-budget.yaml
        // AGENTS.md exceeds fail=30000 threshold
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(31_000)).unwrap();
        fs::write(tmp.path().join("CLAUDE.md"), "small\n").unwrap();
        let result = run_for_root(tmp.path(), OutputFormat::Text);
        assert!(
            result.is_err(),
            "should read instruction-size: from repo-config.yml and flag oversized AGENTS.md"
        );
    }
}
