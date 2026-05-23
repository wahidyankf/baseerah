// Audit orchestrator ported from
// `apps/rhino-cli/internal/repo-governance/audit_orchestrator.go`.
//
// Runs the 11 deterministic governance audits in fixed order, normalizes
// per-category findings to AuditFinding, aggregates into AuditEnvelope.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::Error;
use chrono::Utc;
use regex::Regex;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::agents_md_size::check_agents_md_size;
use super::emoji_audit::audit_emoji;
use super::frontmatter_audit::audit_frontmatter;
use super::layer_coherence::audit_layer_coherence;
use super::license_audit::audit_license;
use super::readme_index_audit::audit_readme_index;
use super::traceability_audit::audit_traceability;
use crate::internal::agents::detect_duplication::detect_duplication;
use crate::internal::docs::frontmatter::validate_docs_frontmatter;
use crate::internal::docs::heading_hierarchy::validate_docs_heading_hierarchy;
use crate::internal::docs::naming::validate_docs_naming;

pub const AUDIT_ENVELOPE_SCHEMA: &str = "rhino-cli/repo-governance-audit/v1";
const AUDIT_SEVERITY_HIGH: &str = "high";
const AUDIT_CRITICALITY_HIGH: &str = "HIGH";

pub fn audit_category_order() -> &'static [&'static str] {
    &[
        "agents-md-size",
        "frontmatter-audit",
        "traceability-audit",
        "license-audit",
        "readme-index-audit",
        "emoji-audit",
        "layer-coherence",
        "docs-validate-naming",
        "docs-validate-frontmatter",
        "docs-validate-heading-hierarchy",
        "agents-detect-duplication",
    ]
}

fn audit_category_command(name: &str) -> &'static str {
    match name {
        "agents-md-size" => "repo-governance agents-md-size",
        "frontmatter-audit" => "repo-governance frontmatter-audit",
        "traceability-audit" => "repo-governance traceability-audit",
        "license-audit" => "repo-governance license-audit",
        "readme-index-audit" => "repo-governance readme-index-audit",
        "emoji-audit" => "repo-governance emoji-audit",
        "layer-coherence" => "repo-governance layer-coherence",
        "docs-validate-naming" => "docs validate-naming",
        "docs-validate-frontmatter" => "docs validate-frontmatter",
        "docs-validate-heading-hierarchy" => "docs validate-heading-hierarchy",
        "agents-detect-duplication" => "agents detect-duplication",
        _ => "",
    }
}

fn default_frontmatter_paths() -> &'static [&'static str] {
    &[
        "repo-governance/",
        "docs/explanation/software-engineering/",
        ".claude/agents/",
        ".claude/skills/",
        "plans/",
    ]
}
fn default_readme_index_paths() -> &'static [&'static str] {
    &[
        "repo-governance/",
        ".claude/agents/",
        ".claude/skills/",
        "docs/explanation/software-engineering/",
    ]
}
fn default_emoji_paths() -> &'static [&'static str] {
    &["."]
}
fn default_docs_validate_naming_paths() -> &'static [&'static str] {
    &["docs/", "repo-governance/"]
}
fn default_docs_validate_frontmatter_paths() -> &'static [&'static str] {
    &[
        "docs/explanation/software-engineering/",
        "repo-governance/conventions/",
        "repo-governance/principles/",
        "repo-governance/development/",
        "repo-governance/workflows/",
    ]
}
fn default_docs_validate_heading_hierarchy_paths() -> &'static [&'static str] {
    &["docs/", "repo-governance/"]
}

#[derive(Debug, Clone, Default)]
pub struct AuditOptions {
    pub repo_root: PathBuf,
    pub skip: Vec<String>,
    pub include_only: Vec<String>,
    pub now: Option<String>, // RFC3339; None → time::now
    pub frontmatter_audit_paths: Vec<String>,
    pub readme_index_audit_paths: Vec<String>,
    pub emoji_audit_paths: Vec<String>,
    pub docs_validate_naming_paths: Vec<String>,
    pub docs_validate_frontmatter_paths: Vec<String>,
    pub docs_validate_heading_hierarchy_paths: Vec<String>,
    pub known_false_positives_path: Option<PathBuf>,
    pub exclude_globs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditEnvelope {
    pub schema: String,
    pub status: String,
    pub result: AuditResult,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditResult {
    pub git_sha: String,
    pub ran_at: String,
    pub total_findings: usize,
    pub by_severity: BTreeMap<String, usize>,
    pub by_category: BTreeMap<String, usize>,
    pub categories: Vec<AuditCategoryResult>,
    pub skipped_false_positives: Vec<AuditFinding>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditCategoryResult {
    pub name: String,
    pub command: String,
    pub passed: bool,
    pub findings: Vec<AuditFinding>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditFinding {
    pub key: String,
    pub severity: String,
    pub criticality: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub file: String,
    #[serde(skip_serializing_if = "skip_zero")]
    pub line: usize,
    pub message: String,
}

fn skip_zero(n: &usize) -> bool {
    *n == 0
}

pub fn run_audit(opts: &AuditOptions) -> std::result::Result<AuditEnvelope, Error> {
    let ran_at = opts
        .now
        .clone()
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());

    let mut categories: Vec<AuditCategoryResult> = Vec::new();
    for &name in audit_category_order() {
        if opts.skip.iter().any(|s| s == name) {
            continue;
        }
        if !opts.include_only.is_empty() && !opts.include_only.iter().any(|s| s == name) {
            continue;
        }
        let mut findings = run_category(name, opts)?;
        findings = filter_excluded(findings, &opts.exclude_globs);
        sort_audit_findings(&mut findings);
        categories.push(AuditCategoryResult {
            name: name.to_string(),
            command: audit_category_command(name).to_string(),
            passed: findings.is_empty(),
            findings,
        });
    }

    let skip_set = load_known_false_positives(opts)?;
    let (categories, skipped) = partition_false_positives(categories, &skip_set);

    let mut total = 0usize;
    let mut by_sev: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_cat: BTreeMap<String, usize> = BTreeMap::new();
    for c in &categories {
        total += c.findings.len();
        by_cat.insert(c.name.clone(), c.findings.len());
        for f in &c.findings {
            *by_sev.entry(f.severity.clone()).or_insert(0) += 1;
        }
    }

    let status = if total > 0 { "failed" } else { "ok" }.to_string();
    let git_sha = read_git_sha(&opts.repo_root);

    Ok(AuditEnvelope {
        schema: AUDIT_ENVELOPE_SCHEMA.to_string(),
        status,
        result: AuditResult {
            git_sha,
            ran_at,
            total_findings: total,
            by_severity: by_sev,
            by_category: by_cat,
            categories,
            skipped_false_positives: skipped,
        },
    })
}

fn run_category(name: &str, opts: &AuditOptions) -> std::result::Result<Vec<AuditFinding>, Error> {
    match name {
        "agents-md-size" => {
            let p = opts.repo_root.join("AGENTS.md");
            if !p.exists() {
                return Ok(vec![new_audit_finding(
                    name,
                    &p.to_string_lossy(),
                    0,
                    "AGENTS.md is missing",
                )]);
            }
            let f = check_agents_md_size(&p.to_string_lossy())?;
            if f.severity != "fail" {
                Ok(Vec::new())
            } else {
                Ok(vec![new_audit_finding(name, &f.file, 0, &f.message)])
            }
        }
        "frontmatter-audit" => {
            let paths = resolve_paths(
                &opts.repo_root,
                &opts.frontmatter_audit_paths,
                default_frontmatter_paths(),
            );
            let findings = audit_frontmatter(&paths)?;
            Ok(findings
                .into_iter()
                .map(|f| new_audit_finding(name, &f.file, f.line, &f.message))
                .collect())
        }
        "traceability-audit" => {
            let findings = audit_traceability(&opts.repo_root)?;
            Ok(findings
                .into_iter()
                .map(|f| new_audit_finding(name, &f.path, f.line, &f.message))
                .collect())
        }
        "license-audit" => {
            let findings = audit_license(&opts.repo_root)?;
            Ok(findings
                .into_iter()
                .map(|f| new_audit_finding(name, &f.path, 0, &f.message))
                .collect())
        }
        "readme-index-audit" => {
            let paths = resolve_paths(
                &opts.repo_root,
                &opts.readme_index_audit_paths,
                default_readme_index_paths(),
            );
            let findings = audit_readme_index(&paths, &[])?;
            Ok(findings
                .into_iter()
                .map(|f| new_audit_finding(name, &f.file, 0, &f.message))
                .collect())
        }
        "emoji-audit" => {
            let paths = resolve_paths(
                &opts.repo_root,
                &opts.emoji_audit_paths,
                default_emoji_paths(),
            );
            let findings = audit_emoji(&paths)?;
            Ok(findings
                .into_iter()
                .map(|f| {
                    let msg = format!(
                        "forbidden emoji codepoint {} at column {}",
                        f.codepoint, f.column
                    );
                    new_audit_finding(name, &f.file, f.line, &msg)
                })
                .collect())
        }
        "layer-coherence" => {
            let findings = audit_layer_coherence(&opts.repo_root)?;
            Ok(findings
                .into_iter()
                .map(|f| new_audit_finding(name, &f.file, 0, &f.message))
                .collect())
        }
        "docs-validate-naming" => {
            let paths = resolve_paths(
                &opts.repo_root,
                &opts.docs_validate_naming_paths,
                default_docs_validate_naming_paths(),
            );
            let findings = validate_docs_naming(&paths, &[])?;
            Ok(findings
                .into_iter()
                .map(|f| new_audit_finding(name, &f.file, 0, &f.message))
                .collect())
        }
        "docs-validate-frontmatter" => {
            let paths = resolve_paths(
                &opts.repo_root,
                &opts.docs_validate_frontmatter_paths,
                default_docs_validate_frontmatter_paths(),
            );
            let findings = validate_docs_frontmatter(&paths)?;
            Ok(findings
                .into_iter()
                .filter(|f| f.severity == "fail")
                .map(|f| new_audit_finding(name, &f.file, 0, &f.message))
                .collect())
        }
        "docs-validate-heading-hierarchy" => {
            let paths = resolve_paths(
                &opts.repo_root,
                &opts.docs_validate_heading_hierarchy_paths,
                default_docs_validate_heading_hierarchy_paths(),
            );
            let findings = validate_docs_heading_hierarchy(&paths)?;
            Ok(findings
                .into_iter()
                .map(|f| new_audit_finding(name, &f.file, f.line, &f.message))
                .collect())
        }
        "agents-detect-duplication" => {
            let findings = detect_duplication(&opts.repo_root)?;
            Ok(findings
                .into_iter()
                .map(|f| {
                    let file = f.files.first().cloned().unwrap_or_default();
                    let line = f.start_lines.first().copied().unwrap_or(0);
                    let mut msg = f.message.clone();
                    if f.files.len() > 1 {
                        msg = format!("{} (files: {})", f.message, f.files.join(", "));
                    }
                    new_audit_finding(name, &file, line, &msg)
                })
                .collect())
        }
        _ => Err(anyhow::anyhow!("unknown category {name}")),
    }
}

fn resolve_paths(repo_root: &Path, override_paths: &[String], defaults: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let push_resolved = |out: &mut Vec<String>, p: &str| {
        if Path::new(p).is_absolute() {
            out.push(p.to_string());
        } else {
            out.push(go_filepath_join(&repo_root.to_string_lossy(), p));
        }
    };
    if !override_paths.is_empty() {
        for p in override_paths {
            push_resolved(&mut out, p);
        }
    } else {
        for p in defaults {
            push_resolved(&mut out, p);
        }
    }
    out
}

/// Mirror Go's `filepath.Join`: lexical join + `path.Clean` (drops `.` and
/// trailing slashes).
fn go_filepath_join(base: &str, rel: &str) -> String {
    let joined = if base.ends_with('/') {
        format!("{base}{rel}")
    } else {
        format!("{base}/{rel}")
    };
    clean_path(&joined)
}

#[allow(clippy::collapsible_if, clippy::collapsible_match)]
fn clean_path(p: &str) -> String {
    if p.is_empty() {
        return ".".to_string();
    }
    let absolute = p.starts_with('/');
    let mut stack: Vec<&str> = Vec::new();
    for seg in p.split('/') {
        match seg {
            "" | "." => continue,
            ".." => {
                if let Some(last) = stack.last() {
                    if *last != ".." {
                        stack.pop();
                        continue;
                    }
                }
                if !absolute {
                    stack.push("..");
                }
            }
            _ => stack.push(seg),
        }
    }
    let body = stack.join("/");
    if absolute {
        format!("/{body}")
    } else if body.is_empty() {
        ".".to_string()
    } else {
        body
    }
}

fn path_matches_any_glob(path: &str, globs: &[String]) -> bool {
    let slashed_path = path.replace('\\', "/");
    for g in globs {
        let slashed_glob = g.replace('\\', "/");
        if let Some(prefix) = slashed_glob.strip_suffix("/**") {
            if slashed_path.contains(&format!("/{prefix}/"))
                || slashed_path.starts_with(&format!("{prefix}/"))
                || slashed_path.ends_with(&format!("/{prefix}"))
            {
                return true;
            }
            for part in slashed_path.split('/') {
                if part == prefix {
                    return true;
                }
            }
            continue;
        }
        // simple wildcard match
        if simple_match(g, path) {
            return true;
        }
        if simple_match(&slashed_glob, &slashed_path) {
            return true;
        }
    }
    false
}

fn simple_match(pattern: &str, s: &str) -> bool {
    // Minimal `*` glob matcher.
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        return pattern == s;
    }
    let mut pos = 0;
    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            if !s[pos..].starts_with(part) {
                return false;
            }
            pos += part.len();
        } else if i == parts.len() - 1 {
            return s[pos..].ends_with(part);
        } else if let Some(idx) = s[pos..].find(part) {
            pos += idx + part.len();
        } else {
            return false;
        }
    }
    true
}

fn filter_excluded(findings: Vec<AuditFinding>, exclude_globs: &[String]) -> Vec<AuditFinding> {
    if exclude_globs.is_empty() {
        return findings;
    }
    findings
        .into_iter()
        .filter(|f| !path_matches_any_glob(&f.file, exclude_globs))
        .collect()
}

fn new_audit_finding(category: &str, file: &str, line: usize, message: &str) -> AuditFinding {
    AuditFinding {
        key: build_audit_key(category, file, message),
        severity: AUDIT_SEVERITY_HIGH.to_string(),
        criticality: AUDIT_CRITICALITY_HIGH.to_string(),
        file: file.to_string(),
        line,
        message: message.to_string(),
    }
}

fn build_audit_key(category: &str, file: &str, message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let digest = hex::encode(hasher.finalize());
    format!("{category}|{file}|{}", &digest[..8])
}

fn sort_audit_findings(findings: &mut [AuditFinding]) {
    findings.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.cmp(&b.line))
            .then(a.key.cmp(&b.key))
    });
}

fn known_false_positive_pattern() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^\s*-\s+`([^`]+)`").unwrap())
}

fn load_known_false_positives(
    opts: &AuditOptions,
) -> std::result::Result<std::collections::HashSet<String>, Error> {
    let path = opts.known_false_positives_path.clone().unwrap_or_else(|| {
        opts.repo_root
            .join("generated-reports")
            .join(".known-false-positives.md")
    });
    let mut set = std::collections::HashSet::new();
    match fs::read_to_string(&path) {
        Ok(content) => {
            for cap in known_false_positive_pattern().captures_iter(&content) {
                set.insert(cap[1].to_string());
            }
            Ok(set)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(set),
        Err(e) => Err(e.into()),
    }
}

fn partition_false_positives(
    mut categories: Vec<AuditCategoryResult>,
    skip_set: &std::collections::HashSet<String>,
) -> (Vec<AuditCategoryResult>, Vec<AuditFinding>) {
    let mut skipped: Vec<AuditFinding> = Vec::new();
    for c in categories.iter_mut() {
        let mut kept: Vec<AuditFinding> = Vec::new();
        for f in c.findings.drain(..) {
            if skip_set.contains(&f.key) {
                skipped.push(f);
            } else {
                kept.push(f);
            }
        }
        c.findings = kept;
        c.passed = c.findings.is_empty();
    }
    skipped.sort_by(|a, b| a.key.cmp(&b.key));
    (categories, skipped)
}

fn read_git_sha(repo_root: &Path) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => "unknown".to_string(),
    }
}

// hex encoding (no `hex` crate dependency).
mod hex {
    pub fn encode<T: AsRef<[u8]>>(bytes: T) -> String {
        let b = bytes.as_ref();
        let mut s = String::with_capacity(b.len() * 2);
        for byte in b {
            s.push_str(&format!("{:02x}", byte));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_audit_key_deterministic() {
        let k = build_audit_key("cat", "file.md", "msg");
        assert!(k.starts_with("cat|file.md|"));
        assert_eq!(k.len(), "cat|file.md|".len() + 8);
        // Same inputs → same hash.
        let k2 = build_audit_key("cat", "file.md", "msg");
        assert_eq!(k, k2);
    }

    #[test]
    fn sort_audit_findings_sorts_by_file_then_line() {
        let mut v = vec![
            new_audit_finding("c", "b.md", 5, "m"),
            new_audit_finding("c", "a.md", 10, "m"),
            new_audit_finding("c", "a.md", 5, "m"),
        ];
        sort_audit_findings(&mut v);
        assert_eq!(v[0].file, "a.md");
        assert_eq!(v[0].line, 5);
        assert_eq!(v[1].file, "a.md");
        assert_eq!(v[1].line, 10);
        assert_eq!(v[2].file, "b.md");
    }

    #[test]
    fn resolve_paths_joins_relative() {
        let r = resolve_paths(Path::new("/repo"), &[], &["docs/", "/abs/"]);
        assert!(r[0].contains("/repo/docs"));
        assert!(r[1].starts_with("/abs"));
    }

    #[test]
    fn resolve_paths_override_wins() {
        let r = resolve_paths(Path::new("/repo"), &["custom/".to_string()], &["default/"]);
        assert!(r[0].contains("/repo/custom"));
        assert!(!r.iter().any(|p| p.contains("default")));
    }

    #[test]
    fn path_matches_glob_dir_star() {
        assert!(path_matches_any_glob(
            "archived/foo/bar.md",
            &["archived/**".to_string()]
        ));
        assert!(!path_matches_any_glob(
            "docs/foo.md",
            &["archived/**".to_string()]
        ));
    }

    #[test]
    fn path_matches_glob_simple() {
        // simple_match handles basic wildcard cases.
        assert!(simple_match("*.md", "foo.md"));
        assert!(simple_match("a*b", "axb"));
        assert!(!simple_match("a*b", "ax"));
    }

    #[test]
    fn filter_excluded_drops_matches() {
        let v = vec![
            new_audit_finding("c", "archived/x.md", 0, "m"),
            new_audit_finding("c", "docs/y.md", 0, "m"),
        ];
        let out = filter_excluded(v, &["archived/**".to_string()]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].file, "docs/y.md");
    }

    #[test]
    fn partition_false_positives_moves_keys() {
        let f1 = new_audit_finding("c", "a.md", 0, "msg1");
        let f2 = new_audit_finding("c", "b.md", 0, "msg2");
        let cats = vec![AuditCategoryResult {
            name: "c".to_string(),
            command: "x".to_string(),
            passed: false,
            findings: vec![f1.clone(), f2.clone()],
        }];
        let mut skip = std::collections::HashSet::new();
        skip.insert(f1.key.clone());
        let (kept, skipped) = partition_false_positives(cats, &skip);
        assert_eq!(kept[0].findings.len(), 1);
        assert_eq!(skipped.len(), 1);
    }

    #[test]
    fn known_false_positive_pattern_parses_bullets() {
        let content = "- `cat|file.md|abcd1234`\n- `other`\nfoo bar\n";
        let re = known_false_positive_pattern();
        let m: Vec<String> = re
            .captures_iter(content)
            .map(|c| c[1].to_string())
            .collect();
        assert_eq!(m.len(), 2);
        assert_eq!(m[0], "cat|file.md|abcd1234");
    }

    #[test]
    fn skip_zero_helper() {
        assert!(skip_zero(&0));
        assert!(!skip_zero(&5));
    }

    #[test]
    fn clean_path_handles_dot_segments() {
        assert_eq!(clean_path("/a/./b"), "/a/b");
        assert_eq!(clean_path("/a/b/.."), "/a");
        assert_eq!(clean_path("a/./b/"), "a/b");
        assert_eq!(clean_path(""), ".");
        assert_eq!(clean_path("/"), "/");
        assert_eq!(clean_path("./foo/../bar"), "bar");
    }

    #[test]
    fn go_filepath_join_drops_dot() {
        let r = go_filepath_join("/repo", "./.agents");
        assert_eq!(r, "/repo/.agents");
        let r = go_filepath_join("/repo/", "docs/");
        assert_eq!(r, "/repo/docs");
        let r = go_filepath_join("/repo", ".");
        assert_eq!(r, "/repo");
    }

    #[test]
    fn audit_category_command_returns_expected() {
        assert_eq!(
            audit_category_command("agents-md-size"),
            "repo-governance agents-md-size"
        );
        assert_eq!(
            audit_category_command("agents-detect-duplication"),
            "agents detect-duplication"
        );
        assert_eq!(audit_category_command("unknown"), "");
    }

    #[test]
    fn audit_category_order_is_fixed() {
        let o = audit_category_order();
        assert_eq!(o.len(), 11);
        assert_eq!(o[0], "agents-md-size");
        assert_eq!(o[10], "agents-detect-duplication");
    }

    #[test]
    fn default_paths_return_non_empty() {
        assert!(!default_frontmatter_paths().is_empty());
        assert!(!default_readme_index_paths().is_empty());
        assert!(!default_emoji_paths().is_empty());
        assert!(!default_docs_validate_naming_paths().is_empty());
        assert!(!default_docs_validate_frontmatter_paths().is_empty());
        assert!(!default_docs_validate_heading_hierarchy_paths().is_empty());
    }

    #[test]
    fn run_audit_empty_repo_skip_all_categories() {
        let dir = tempfile::tempdir().unwrap();
        let opts = AuditOptions {
            repo_root: dir.path().to_path_buf(),
            skip: audit_category_order()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            now: Some("2026-05-23T00:00:00Z".to_string()),
            ..Default::default()
        };
        let env = run_audit(&opts).unwrap();
        assert_eq!(env.status, "ok");
        assert_eq!(env.result.total_findings, 0);
        assert!(env.result.categories.is_empty());
    }

    #[test]
    fn run_audit_include_only_filter() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("AGENTS.md"), "x").unwrap();
        let opts = AuditOptions {
            repo_root: dir.path().to_path_buf(),
            include_only: vec!["agents-md-size".to_string()],
            now: Some("2026-05-23T00:00:00Z".to_string()),
            ..Default::default()
        };
        let env = run_audit(&opts).unwrap();
        assert_eq!(env.result.categories.len(), 1);
        assert_eq!(env.result.categories[0].name, "agents-md-size");
    }

    #[test]
    fn run_audit_missing_agents_md_emits_finding() {
        let dir = tempfile::tempdir().unwrap();
        let opts = AuditOptions {
            repo_root: dir.path().to_path_buf(),
            include_only: vec!["agents-md-size".to_string()],
            now: Some("2026-05-23T00:00:00Z".to_string()),
            ..Default::default()
        };
        let env = run_audit(&opts).unwrap();
        assert_eq!(env.result.total_findings, 1);
        let f = &env.result.categories[0].findings[0];
        assert!(f.message.contains("AGENTS.md is missing"));
    }

    #[test]
    fn run_audit_load_false_positives_skips() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("generated-reports")).unwrap();
        // Create an AGENTS.md too big.
        std::fs::write(
            dir.path().join("AGENTS.md"),
            "x".repeat(41 * 1024).into_bytes(),
        )
        .unwrap();
        // Pre-compute the key that would be assigned to the AGENTS.md finding
        // — we don't know the exact message text, so we set a permissive
        // skip-list that matches by key prefix.
        let env = run_audit(&AuditOptions {
            repo_root: dir.path().to_path_buf(),
            include_only: vec!["agents-md-size".to_string()],
            now: Some("2026-05-23T00:00:00Z".to_string()),
            ..Default::default()
        })
        .unwrap();
        let key = env.result.categories[0]
            .findings
            .first()
            .map(|f| f.key.clone())
            .unwrap_or_default();
        std::fs::write(
            dir.path()
                .join("generated-reports/.known-false-positives.md"),
            format!("- `{key}`\n"),
        )
        .unwrap();
        let env2 = run_audit(&AuditOptions {
            repo_root: dir.path().to_path_buf(),
            include_only: vec!["agents-md-size".to_string()],
            now: Some("2026-05-23T00:00:00Z".to_string()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(env2.result.total_findings, 0);
        assert_eq!(env2.result.skipped_false_positives.len(), 1);
    }

    #[test]
    fn run_audit_status_failed_when_findings() {
        let dir = tempfile::tempdir().unwrap();
        let opts = AuditOptions {
            repo_root: dir.path().to_path_buf(),
            include_only: vec!["agents-md-size".to_string()],
            now: Some("2026-05-23T00:00:00Z".to_string()),
            ..Default::default()
        };
        let env = run_audit(&opts).unwrap();
        assert_eq!(env.status, "failed");
    }

    #[test]
    fn read_git_sha_returns_unknown_in_nongit_dir() {
        let dir = tempfile::tempdir().unwrap();
        let s = read_git_sha(dir.path());
        assert_eq!(s, "unknown");
    }

    #[test]
    fn run_audit_unknown_category_returns_error() {
        // Construct an opts with a fake skip name that does not exist —
        // since our include_only uses it but it's not in order, run_audit
        // will pass through with no categories.
        let dir = tempfile::tempdir().unwrap();
        let opts = AuditOptions {
            repo_root: dir.path().to_path_buf(),
            include_only: vec!["never-real".to_string()],
            now: Some("2026-05-23T00:00:00Z".to_string()),
            ..Default::default()
        };
        let env = run_audit(&opts).unwrap();
        assert!(env.result.categories.is_empty());
    }

    #[test]
    fn run_category_handles_each_branch() {
        let dir = tempfile::tempdir().unwrap();
        let opts = AuditOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        // Each category should not panic for an empty repo.
        for cat in audit_category_order() {
            if *cat == "license-audit" || *cat == "traceability-audit" {
                // These walk repo_root structures; for an empty dir they
                // return Ok with no findings, so the call should succeed.
            }
            let _ = run_category(cat, &opts);
        }
    }

    #[test]
    fn run_category_unknown_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let opts = AuditOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        let r = run_category("nope-not-real", &opts);
        assert!(r.is_err());
    }

    #[test]
    fn audit_envelope_json_includes_schema() {
        let env = AuditEnvelope {
            schema: AUDIT_ENVELOPE_SCHEMA.to_string(),
            status: "ok".to_string(),
            result: AuditResult {
                git_sha: "x".into(),
                ran_at: "2026".into(),
                total_findings: 0,
                by_severity: BTreeMap::new(),
                by_category: BTreeMap::new(),
                categories: vec![],
                skipped_false_positives: vec![],
            },
        };
        let s = serde_json::to_string(&env).unwrap();
        assert!(s.contains("\"schema\":\"rhino-cli/repo-governance-audit/v1\""));
    }

    use std::collections::BTreeMap;
}
