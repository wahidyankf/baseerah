// Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/traceability_audit.go`.

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Error};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityFinding {
    pub path: String,
    pub line: usize,
    pub kind: String,
    pub message: String,
}

pub const KIND_MISSING_VISION_SUPPORTED: &str = "missing-vision-supported";
pub const KIND_MISSING_PRINCIPLES_IMPLEMENTED: &str = "missing-principles-implemented";
pub const KIND_MISSING_CONVENTIONS_IMPLEMENTED: &str = "missing-conventions-implemented";
pub const KIND_MISSING_AGENT_REFERENCE: &str = "missing-agent-reference";

fn vision_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^##\s+Vision Supported\s*$").unwrap())
}

fn principles_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^##\s+Principles Implemented/Respected\s*$").unwrap())
}

fn conventions_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^##\s+Conventions Implemented/Respected\s*$").unwrap())
}

fn agent_ref_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\.claude/agents/[a-z0-9-]+\.md").unwrap())
}

const META_EXEMPT: &[&str] = &["meta/execution-modes.md", "meta/workflow-identifier.md"];

pub fn audit_traceability(
    repo_root: &Path,
) -> std::result::Result<Vec<TraceabilityFinding>, Error> {
    let mut findings = Vec::new();
    findings.extend(audit_principles(
        &repo_root.join("repo-governance/principles"),
    )?);
    findings.extend(audit_conventions(
        &repo_root.join("repo-governance/conventions"),
    )?);
    findings.extend(audit_development(
        &repo_root.join("repo-governance/development"),
    )?);
    findings.extend(audit_workflows(
        &repo_root.join("repo-governance/workflows"),
    )?);

    findings.sort_by(|a, b| a.path.cmp(&b.path).then(a.line.cmp(&b.line)));
    Ok(findings)
}

fn audit_principles(root: &Path) -> std::result::Result<Vec<TraceabilityFinding>, Error> {
    let files = list_governance_markdown(root)?;
    let mut findings = Vec::new();
    for path in files {
        let data = fs::read_to_string(&path).with_context(|| format!("read {path}"))?;
        if !vision_re().is_match(&data) {
            findings.push(TraceabilityFinding {
                path: path.clone(),
                line: 1,
                kind: KIND_MISSING_VISION_SUPPORTED.to_string(),
                message: "principle is missing required \"## Vision Supported\" heading"
                    .to_string(),
            });
        }
    }
    Ok(findings)
}

fn audit_conventions(root: &Path) -> std::result::Result<Vec<TraceabilityFinding>, Error> {
    let files = list_governance_markdown(root)?;
    let mut findings = Vec::new();
    for path in files {
        let data = fs::read_to_string(&path).with_context(|| format!("read {path}"))?;
        if !principles_re().is_match(&data) {
            findings.push(TraceabilityFinding {
                path: path.clone(),
                line: 1,
                kind: KIND_MISSING_PRINCIPLES_IMPLEMENTED.to_string(),
                message:
                    "convention is missing required \"## Principles Implemented/Respected\" heading"
                        .to_string(),
            });
        }
    }
    Ok(findings)
}

fn audit_development(root: &Path) -> std::result::Result<Vec<TraceabilityFinding>, Error> {
    let files = list_governance_markdown(root)?;
    let mut findings = Vec::new();
    for path in files {
        let data = fs::read_to_string(&path).with_context(|| format!("read {path}"))?;
        if !principles_re().is_match(&data) {
            findings.push(TraceabilityFinding {
                path: path.clone(),
                line: 1,
                kind: KIND_MISSING_PRINCIPLES_IMPLEMENTED.to_string(),
                message: "development doc is missing required \"## Principles Implemented/Respected\" heading"
                    .to_string(),
            });
        }
        if !conventions_re().is_match(&data) {
            findings.push(TraceabilityFinding {
                path: path.clone(),
                line: 1,
                kind: KIND_MISSING_CONVENTIONS_IMPLEMENTED.to_string(),
                message: "development doc is missing required \"## Conventions Implemented/Respected\" heading"
                    .to_string(),
            });
        }
    }
    Ok(findings)
}

fn audit_workflows(root: &Path) -> std::result::Result<Vec<TraceabilityFinding>, Error> {
    let files = list_governance_markdown(root)?;
    let mut findings = Vec::new();
    for path in files {
        let rel = Path::new(&path)
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        if META_EXEMPT.contains(&rel.as_str()) {
            continue;
        }
        let data = fs::read_to_string(&path).with_context(|| format!("read {path}"))?;
        if !agent_ref_re().is_match(&data) {
            let line = first_non_empty_line(&data);
            findings.push(TraceabilityFinding {
                path: path.clone(),
                line,
                kind: KIND_MISSING_AGENT_REFERENCE.to_string(),
                message: "workflow does not reference any .claude/agents/<name>.md file"
                    .to_string(),
            });
        }
    }
    Ok(findings)
}

fn first_non_empty_line(data: &str) -> usize {
    for (idx, line) in data.split('\n').enumerate() {
        if !line.trim().is_empty() {
            return idx + 1;
        }
    }
    1
}

fn list_governance_markdown(root: &Path) -> std::result::Result<Vec<String>, Error> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut files: Vec<String> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            let n = e.file_name().to_string_lossy();
            n.ends_with(".md") && n != "README.md"
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(p: &Path, content: &str) {
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
    }

    #[test]
    fn principle_passes_when_vision_section_present() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/principles/p.md"),
            "# P\n\n## Vision Supported\n\nx\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn principle_missing_vision_emits_finding() {
        let tmp = TempDir::new().unwrap();
        write(&tmp.path().join("repo-governance/principles/p.md"), "# P\n");
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings
            .iter()
            .any(|f| f.kind == KIND_MISSING_VISION_SUPPORTED));
    }

    #[test]
    fn convention_missing_principles_emits_finding() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/conventions/c.md"),
            "# C\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings
            .iter()
            .any(|f| f.kind == KIND_MISSING_PRINCIPLES_IMPLEMENTED));
    }

    #[test]
    fn development_requires_both_sections() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/development/d.md"),
            "# D\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        let kinds: Vec<&str> = findings.iter().map(|f| f.kind.as_str()).collect();
        assert!(kinds.contains(&KIND_MISSING_PRINCIPLES_IMPLEMENTED));
        assert!(kinds.contains(&KIND_MISSING_CONVENTIONS_IMPLEMENTED));
    }

    #[test]
    fn development_passes_with_both_sections() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/development/d.md"),
            "# D\n\n## Principles Implemented/Respected\n\n## Conventions Implemented/Respected\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn workflow_missing_agent_ref_emits_finding() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/workflows/w.md"),
            "# W\n\nno agent here\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings
            .iter()
            .any(|f| f.kind == KIND_MISSING_AGENT_REFERENCE));
    }

    #[test]
    fn workflow_passes_when_agent_referenced() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/workflows/w.md"),
            "# W\n\nSee `.claude/agents/foo-bar.md`\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn meta_exempt_paths_skip_agent_check() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path()
                .join("repo-governance/workflows/meta/execution-modes.md"),
            "# meta\n\nno agent ref needed\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn readme_files_are_exempt() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/principles/README.md"),
            "# Index\n",
        );
        let findings = audit_traceability(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn first_non_empty_line_skips_blanks() {
        assert_eq!(first_non_empty_line("\n\nhello\n"), 3);
        assert_eq!(first_non_empty_line("hello\n"), 1);
        assert_eq!(first_non_empty_line(""), 1);
    }
}
