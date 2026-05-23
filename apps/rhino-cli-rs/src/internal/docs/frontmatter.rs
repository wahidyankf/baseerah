// Byte-for-byte port of `apps/rhino-cli/internal/docs/frontmatter.go`.

use std::fs;
use std::path::Path;

use anyhow::{anyhow, Error};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsFrontmatterFinding {
    pub file: String,
    pub severity: String,
    pub kind: String,
    pub message: String,
}

const SEVERITY_FAIL: &str = "fail";
const SEVERITY_WARN: &str = "warn";

const KIND_INVALID_YAML: &str = "invalid-yaml";
const KIND_MISSING_FRONTMATTER: &str = "missing-frontmatter";
const KIND_MISSING_TITLE: &str = "missing-title";
const KIND_MISSING_DESCRIPTION: &str = "missing-description";
const KIND_MISSING_CATEGORY: &str = "missing-category";
const KIND_MISSING_SUBCATEGORY: &str = "missing-subcategory";
const KIND_MISSING_TAGS: &str = "missing-tags";
const KIND_WRONG_CATEGORY_VALUE: &str = "wrong-category-value";
const KIND_CATEGORY_DEPRECATED: &str = "category-deprecated";

const VALID_CATEGORIES: &[&str] = &["tutorial", "how-to", "reference", "explanation"];

const SOFTWARE_DOC_PREFIX: &str = "docs/explanation/software-engineering/";

const GOVERNANCE_DOC_PREFIXES: &[&str] = &[
    "repo-governance/conventions/",
    "repo-governance/principles/",
    "repo-governance/development/",
    "repo-governance/workflows/",
];

const SKIP_DIRS: &[&str] = &["node_modules", ".git", ".next", "dist", "build", "target"];

#[derive(Clone, Copy)]
enum DocArea {
    Unknown,
    Software,
    Governance,
}

pub fn validate_docs_frontmatter(
    paths: &[String],
) -> std::result::Result<Vec<DocsFrontmatterFinding>, Error> {
    if paths.is_empty() {
        return Err(anyhow!("at least one path is required"));
    }
    let mut findings = Vec::new();
    for root in paths {
        findings.extend(walk_frontmatter_path(root)?);
    }
    findings.sort_by(|a, b| a.file.cmp(&b.file).then(a.kind.cmp(&b.kind)));
    Ok(findings)
}

fn walk_frontmatter_path(root: &str) -> std::result::Result<Vec<DocsFrontmatterFinding>, Error> {
    let root_p = Path::new(root);
    if !root_p.exists() {
        return Ok(Vec::new());
    }
    let mut findings = Vec::new();
    let walker = WalkDir::new(root_p).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy().to_string();
            !SKIP_DIRS.contains(&name.as_str())
        } else {
            true
        }
    });
    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".md") {
            continue;
        }
        let path = entry.path();
        let area = classify_doc_area(&path.to_string_lossy());
        if matches!(area, DocArea::Unknown) {
            continue;
        }
        findings.extend(scan_frontmatter_file(&path.to_string_lossy(), area)?);
    }
    Ok(findings)
}

fn classify_doc_area(path: &str) -> DocArea {
    let slashed = path.replace('\\', "/");
    if slashed.contains(SOFTWARE_DOC_PREFIX) {
        return DocArea::Software;
    }
    for prefix in GOVERNANCE_DOC_PREFIXES {
        if slashed.contains(prefix) {
            return DocArea::Governance;
        }
    }
    DocArea::Unknown
}

fn scan_frontmatter_file(
    path: &str,
    area: DocArea,
) -> std::result::Result<Vec<DocsFrontmatterFinding>, Error> {
    let data = fs::read_to_string(path)?;
    let frontmatter = match extract_frontmatter(&data) {
        Some(fm) => fm,
        None => {
            return Ok(vec![DocsFrontmatterFinding {
                file: path.to_string(),
                severity: SEVERITY_FAIL.to_string(),
                kind: KIND_MISSING_FRONTMATTER.to_string(),
                message: "file has no YAML frontmatter (delimited by `---` fences)".to_string(),
            }]);
        }
    };
    let parsed: serde_norway::Value = match serde_norway::from_str(&frontmatter) {
        Ok(v) => v,
        Err(e) => {
            return Ok(vec![DocsFrontmatterFinding {
                file: path.to_string(),
                severity: SEVERITY_FAIL.to_string(),
                kind: KIND_INVALID_YAML.to_string(),
                message: format!("frontmatter is not valid YAML: {e}"),
            }]);
        }
    };
    Ok(match area {
        DocArea::Software => validate_software_schema(path, &parsed),
        DocArea::Governance => validate_governance_schema(path, &parsed),
        DocArea::Unknown => Vec::new(),
    })
}

fn extract_frontmatter(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.split('\n').collect();
    if lines.is_empty() || lines[0].trim() != "---" {
        return None;
    }
    for i in 1..lines.len() {
        if lines[i].trim() == "---" {
            return Some(lines[1..i].join("\n"));
        }
    }
    None
}

fn validate_software_schema(path: &str, fm: &serde_norway::Value) -> Vec<DocsFrontmatterFinding> {
    let mut findings = Vec::new();
    if !has_non_empty_string(fm, "title") {
        findings.push(mk_fail(
            path,
            KIND_MISSING_TITLE,
            "required field \"title\" is missing or empty",
        ));
    }
    if !has_non_empty_string(fm, "description") {
        findings.push(mk_fail(
            path,
            KIND_MISSING_DESCRIPTION,
            "required field \"description\" is missing or empty",
        ));
    }
    if !has_non_empty_string(fm, "category") {
        findings.push(mk_fail(
            path,
            KIND_MISSING_CATEGORY,
            "required field \"category\" is missing or empty",
        ));
    } else {
        let v = string_value(fm.get("category"));
        if VALID_CATEGORIES.contains(&v.as_str()) {
            // ok
        } else if v == "software" {
            findings.push(DocsFrontmatterFinding {
                file: path.to_string(),
                severity: SEVERITY_WARN.to_string(),
                kind: KIND_CATEGORY_DEPRECATED.to_string(),
                message: "field \"category\" value \"software\" is deprecated; use one of: tutorial, how-to, reference, explanation".to_string(),
            });
        } else {
            findings.push(DocsFrontmatterFinding {
                file: path.to_string(),
                severity: SEVERITY_FAIL.to_string(),
                kind: KIND_WRONG_CATEGORY_VALUE.to_string(),
                message: format!(
                    "field \"category\" must be one of: tutorial, how-to, reference, explanation; found \"{v}\""
                ),
            });
        }
    }
    if !has_non_empty_string(fm, "subcategory") {
        findings.push(mk_fail(
            path,
            KIND_MISSING_SUBCATEGORY,
            "required field \"subcategory\" is missing or empty",
        ));
    }
    if !has_non_empty_list(fm, "tags") {
        findings.push(mk_fail(
            path,
            KIND_MISSING_TAGS,
            "required field \"tags\" must be a non-empty list",
        ));
    }
    findings
}

fn validate_governance_schema(path: &str, fm: &serde_norway::Value) -> Vec<DocsFrontmatterFinding> {
    let mut findings = Vec::new();
    if !has_non_empty_string(fm, "title") {
        findings.push(mk_fail(
            path,
            KIND_MISSING_TITLE,
            "required field \"title\" is missing or empty",
        ));
    }
    if !has_non_empty_string(fm, "description") {
        findings.push(DocsFrontmatterFinding {
            file: path.to_string(),
            severity: SEVERITY_WARN.to_string(),
            kind: KIND_MISSING_DESCRIPTION.to_string(),
            message: "recommended field \"description\" is missing or empty".to_string(),
        });
    }
    findings
}

fn mk_fail(path: &str, kind: &str, message: &str) -> DocsFrontmatterFinding {
    DocsFrontmatterFinding {
        file: path.to_string(),
        severity: SEVERITY_FAIL.to_string(),
        kind: kind.to_string(),
        message: message.to_string(),
    }
}

fn has_non_empty_string(fm: &serde_norway::Value, key: &str) -> bool {
    let v = fm.get(key);
    let s = string_value(v);
    !s.trim().is_empty()
}

fn string_value(v: Option<&serde_norway::Value>) -> String {
    match v {
        None => String::new(),
        Some(serde_norway::Value::Null) => String::new(),
        Some(serde_norway::Value::String(s)) => s.clone(),
        Some(serde_norway::Value::Bool(b)) => b.to_string(),
        Some(serde_norway::Value::Number(n)) => n.to_string(),
        Some(other) => serde_norway::to_string(other)
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}

fn has_non_empty_list(fm: &serde_norway::Value, key: &str) -> bool {
    match fm.get(key) {
        Some(serde_norway::Value::Sequence(list)) => !list.is_empty(),
        _ => false,
    }
}

pub fn has_fail_findings(findings: &[DocsFrontmatterFinding]) -> bool {
    findings.iter().any(|f| f.severity == SEVERITY_FAIL)
}

pub fn count_severity(findings: &[DocsFrontmatterFinding], sev: &str) -> usize {
    findings.iter().filter(|f| f.severity == sev).count()
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
    fn errors_on_empty_paths() {
        let err = validate_docs_frontmatter(&[]).unwrap_err();
        assert!(err.to_string().contains("at least one path"));
    }

    #[test]
    fn missing_frontmatter_emits_fail() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path()
                .join("docs/explanation/software-engineering/foo.md"),
            "# No frontmatter\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.iter().any(|f| f.kind == KIND_MISSING_FRONTMATTER));
    }

    #[test]
    fn software_full_schema_passes() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/explanation/software-engineering/foo.md"),
            "---\ntitle: T\ndescription: D\ncategory: explanation\nsubcategory: S\ntags: [a]\n---\nbody\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn software_missing_required_fields_emit_fails() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path()
                .join("docs/explanation/software-engineering/foo.md"),
            "---\ntitle: T\n---\nbody\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        let kinds: Vec<&str> = findings.iter().map(|f| f.kind.as_str()).collect();
        assert!(kinds.contains(&KIND_MISSING_DESCRIPTION));
        assert!(kinds.contains(&KIND_MISSING_CATEGORY));
        assert!(kinds.contains(&KIND_MISSING_SUBCATEGORY));
        assert!(kinds.contains(&KIND_MISSING_TAGS));
    }

    #[test]
    fn software_deprecated_category_emits_warn() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/explanation/software-engineering/foo.md"),
            "---\ntitle: T\ndescription: D\ncategory: software\nsubcategory: S\ntags: [a]\n---\nbody\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        let f = findings
            .iter()
            .find(|f| f.kind == KIND_CATEGORY_DEPRECATED)
            .unwrap();
        assert_eq!(f.severity, SEVERITY_WARN);
    }

    #[test]
    fn software_wrong_category_emits_fail() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/explanation/software-engineering/foo.md"),
            "---\ntitle: T\ndescription: D\ncategory: random\nsubcategory: S\ntags: [a]\n---\nbody\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.iter().any(|f| f.kind == KIND_WRONG_CATEGORY_VALUE));
    }

    #[test]
    fn governance_title_required_description_warned() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/conventions/foo.md"),
            "---\n---\nbody\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        let kinds: Vec<&str> = findings.iter().map(|f| f.kind.as_str()).collect();
        assert!(kinds.contains(&KIND_MISSING_TITLE));
        // description missing → warn only
        let desc = findings
            .iter()
            .find(|f| f.kind == KIND_MISSING_DESCRIPTION)
            .unwrap();
        assert_eq!(desc.severity, SEVERITY_WARN);
    }

    #[test]
    fn governance_full_schema_passes() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("repo-governance/principles/foo.md"),
            "---\ntitle: T\ndescription: D\n---\nbody\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn unknown_area_passes() {
        let tmp = TempDir::new().unwrap();
        write(&tmp.path().join("random/foo.md"), "no frontmatter\n");
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn invalid_yaml_emits_fail() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path()
                .join("docs/explanation/software-engineering/foo.md"),
            "---\ntitle: T\n  invalid: : :\n---\nbody\n",
        );
        let findings =
            validate_docs_frontmatter(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.iter().any(|f| f.kind == KIND_INVALID_YAML));
    }

    #[test]
    fn has_fail_findings_detects_severity() {
        let f1 = DocsFrontmatterFinding {
            file: "a.md".to_string(),
            severity: SEVERITY_WARN.to_string(),
            kind: "x".to_string(),
            message: "m".to_string(),
        };
        let f2 = DocsFrontmatterFinding {
            file: "b.md".to_string(),
            severity: SEVERITY_FAIL.to_string(),
            kind: "y".to_string(),
            message: "m".to_string(),
        };
        assert!(!has_fail_findings(std::slice::from_ref(&f1)));
        assert!(has_fail_findings(&[f1, f2]));
    }
}
