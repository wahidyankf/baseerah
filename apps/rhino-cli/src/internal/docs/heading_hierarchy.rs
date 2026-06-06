//! Markdown heading-hierarchy validator.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/docs/heading_hierarchy.go`.
//!
//! Validates two rules across every `.md` file reachable from the supplied root
//! paths:
//! 1. Each file must have exactly one H1 heading.
//! 2. Heading levels must not skip (e.g. H1 → H3 without an intervening H2).
//!
//! Headings inside fenced code blocks are ignored.

use std::fs;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use walkdir::WalkDir;

use super::naming::SKIP_DIRS as NAMING_SKIP_DIRS;

/// A single heading-hierarchy finding for a markdown file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsHeadingFinding {
    /// Path to the file that contains the finding.
    pub file: String,
    /// One-based line number where the problematic heading appears.
    pub line: usize,
    /// Severity string (currently always `"high"`).
    pub severity: String,
    /// Machine-readable kind: `"missing-h1"`, `"duplicate-h1"`, or `"skipped-level"`.
    pub kind: String,
    /// Human-readable description of the heading issue.
    pub message: String,
}

/// Validates heading hierarchy in every markdown file reachable from `paths`.
///
/// Findings are sorted by file path, then by line number.
///
/// # Errors
///
/// Returns an error when `paths` is empty, or when a file cannot be read.
pub fn validate_docs_heading_hierarchy(
    paths: &[String],
) -> std::result::Result<Vec<DocsHeadingFinding>, Error> {
    if paths.is_empty() {
        return Err(anyhow!("at least one path is required"));
    }
    let mut findings = Vec::new();
    for root in paths {
        findings.extend(walk_heading_hierarchy_path(root)?);
    }
    findings.sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
    Ok(findings)
}

/// Returns `true` when the repository-relative path `repo_rel` is in the prose
/// allowlist.
///
/// The prose allowlist consists of:
/// - `docs/`
/// - `repo-governance/`
/// - `plans/` **except** `plans/done/`
/// - `specs/`
/// - Root-level `*.md` files (no directory component)
/// - `apps/<name>/README.md` and `libs/<name>/README.md` (project-root READMEs only)
/// - `apps/<name>/docs/**` and `libs/<name>/docs/**` (project docs subtrees)
///
/// Everything else — including `.claude/`, `.opencode/`, deep `apps/`/`libs/`
/// internals, `plans/done/`, and noise directories — is **default-deny**.
pub fn is_prose_allowlisted(repo_rel: &str) -> bool {
    // Normalise path separators to forward slash for consistent matching.
    let r = repo_rel.replace('\\', "/");
    // plans/done/ is explicitly excluded even though plans/ is allowlisted.
    if r.starts_with("plans/done/") || r == "plans/done" {
        return false;
    }
    if r.starts_with("docs/")
        || r.starts_with("repo-governance/")
        || r.starts_with("plans/")
        || r.starts_with("specs/")
    {
        return true;
    }
    // Root-level *.md (no slash present at all)
    if !r.contains('/') && r.ends_with(".md") {
        return true;
    }
    // apps/<name>/README.md, libs/<name>/README.md, apps/<name>/docs/**, libs/<name>/docs/**
    if let Some(rest) = r.strip_prefix("apps/").or_else(|| r.strip_prefix("libs/"))
        && let Some((_project, tail)) = rest.split_once('/')
    {
        return tail == "README.md" || tail.starts_with("docs/");
    }
    false
}

/// Performs an allowlisted heading-hierarchy scan rooted at `repo_root`.
///
/// Only files whose repository-relative path satisfies [`is_prose_allowlisted`]
/// are checked.  `exclude_prefixes` are additional prefixes (on top of the
/// default-deny allowlist) to skip — applied after the allowlist filter.
///
/// Findings are sorted by file path, then by line number.
///
/// # Errors
///
/// Returns an error when a file cannot be read.
pub fn validate_docs_heading_hierarchy_allowlisted(
    repo_root: &std::path::Path,
    exclude_prefixes: &[String],
) -> std::result::Result<Vec<DocsHeadingFinding>, Error> {
    let mut findings = Vec::new();
    let walker = WalkDir::new(repo_root).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy().to_string();
            !NAMING_SKIP_DIRS.contains(&name.as_str())
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
        // Compute repository-relative path for allowlist check.
        let rel = entry.path().strip_prefix(repo_root).map_or_else(
            |_| entry.path().to_string_lossy().to_string(),
            |p| p.to_string_lossy().replace('\\', "/"),
        );
        if !is_prose_allowlisted(&rel) {
            continue;
        }
        // Apply caller-supplied exclude prefixes after the allowlist.
        if exclude_prefixes
            .iter()
            .any(|pfx| rel.starts_with(pfx.as_str()))
        {
            continue;
        }
        findings.extend(scan_file_heading_hierarchy(
            &entry.path().to_string_lossy(),
        )?);
    }
    findings.sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
    Ok(findings)
}

/// Walks `root` recursively and validates each markdown file.
///
/// Returns an empty list if `root` does not exist on the filesystem.
///
/// # Errors
///
/// Returns an error when a markdown file cannot be read.
fn walk_heading_hierarchy_path(root: &str) -> std::result::Result<Vec<DocsHeadingFinding>, Error> {
    let root_p = Path::new(root);
    if !root_p.exists() {
        return Ok(Vec::new());
    }
    let mut findings = Vec::new();
    let walker = WalkDir::new(root_p).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy().to_string();
            !NAMING_SKIP_DIRS.contains(&name.as_str())
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
        findings.extend(scan_file_heading_hierarchy(
            &entry.path().to_string_lossy(),
        )?);
    }
    Ok(findings)
}

/// Internal representation of a parsed markdown heading.
#[derive(Debug, Clone, Copy)]
struct Heading {
    /// One-based source line number.
    line: usize,
    /// ATX heading level (1–6).
    level: usize,
}

/// Reads `path`, extracts headings (ignoring fenced code blocks), and
/// applies the hierarchy rules.
///
/// # Errors
///
/// Returns an error when `path` cannot be read.
fn scan_file_heading_hierarchy(path: &str) -> std::result::Result<Vec<DocsHeadingFinding>, Error> {
    let data = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
    let headings = collect_headings(&data);
    Ok(analyze_headings(path, &headings))
}

/// Parses all ATX headings from `content`, skipping lines inside fenced code blocks.
fn collect_headings(content: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut in_fence = false;
    let mut fence_char: char = ' ';
    let mut fence_len: usize = 0;
    for (i, line) in content.split('\n').enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim_start_matches([' ', '\t']);
        if let Some((ch, length)) = parse_fence_open(trimmed) {
            if !in_fence {
                in_fence = true;
                fence_char = ch;
                fence_len = length;
            } else if ch == fence_char && length >= fence_len {
                in_fence = false;
                fence_char = ' ';
                fence_len = 0;
            }
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some(level) = parse_heading_level(trimmed) {
            headings.push(Heading {
                line: line_num,
                level,
            });
        }
    }
    headings
}

/// Public re-export of [`parse_fence_open`] for use by sibling modules.
pub(crate) fn parse_fence_open_pub(s: &str) -> Option<(char, usize)> {
    parse_fence_open(s)
}

/// Public re-export of [`parse_heading_level`] for use by sibling modules.
pub(crate) fn parse_heading_level_pub(s: &str) -> Option<usize> {
    parse_heading_level(s)
}

/// Parses the opening of a fenced code block from the start of a line.
///
/// Returns `Some((fence_char, length))` when the line begins with three or
/// more identical `` ` `` or `~` characters; otherwise returns `None`.
///
/// # Panics
///
/// Panics if `s` is non-empty but has no first character (impossible in practice).
fn parse_fence_open(s: &str) -> Option<(char, usize)> {
    if s.is_empty() {
        return None;
    }
    let first = s.chars().next().expect("s is non-empty — checked above");
    if first != '`' && first != '~' {
        return None;
    }
    let mut n = 0;
    for c in s.chars() {
        if c == first {
            n += 1;
        } else {
            break;
        }
    }
    if n < 3 {
        return None;
    }
    Some((first, n))
}

/// Parses the ATX heading level (1–6) from the start of a trimmed line.
///
/// Returns `None` when the line is not a valid ATX heading (wrong prefix,
/// no space after `#` characters, or empty heading text).
fn parse_heading_level(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes[0] != b'#' {
        return None;
    }
    let mut level = 0;
    while level < bytes.len() && bytes[level] == b'#' {
        level += 1;
    }
    if !(1..=6).contains(&level) {
        return None;
    }
    if level >= bytes.len() {
        return None;
    }
    let next = bytes[level];
    if next != b' ' && next != b'\t' {
        return None;
    }
    let rest = s[level + 1..].trim();
    if rest.is_empty() {
        return None;
    }
    Some(level)
}

/// Applies the H1-uniqueness and no-level-skipping rules to a list of headings.
///
/// Returns an empty list when `headings` is empty (file has no headings at all).
fn analyze_headings(file: &str, headings: &[Heading]) -> Vec<DocsHeadingFinding> {
    if headings.is_empty() {
        return Vec::new();
    }
    let mut findings = Vec::new();
    let mut h1_count = 0usize;
    let mut first_h1_line = 0usize;
    let mut second_h1_line = 0usize;
    for h in headings {
        if h.level == 1 {
            h1_count += 1;
            if h1_count == 1 {
                first_h1_line = h.line;
            } else if h1_count == 2 {
                second_h1_line = h.line;
            }
        }
    }
    if h1_count == 0 {
        findings.push(DocsHeadingFinding {
            file: file.to_string(),
            line: headings[0].line,
            severity: "high".to_string(),
            kind: "missing-h1".to_string(),
            message:
                "markdown file has no H1 heading; every documented file must have exactly one H1"
                    .to_string(),
        });
    } else if h1_count >= 2 {
        findings.push(DocsHeadingFinding {
            file: file.to_string(),
            line: second_h1_line,
            severity: "high".to_string(),
            kind: "duplicate-h1".to_string(),
            message: format!(
                "markdown file has {h1_count} H1 headings (first at line {first_h1_line}); every file must have exactly one H1"
            ),
        });
    }
    for i in 1..headings.len() {
        let prev = headings[i - 1].level;
        let cur = headings[i].level;
        if cur > prev + 1 {
            findings.push(DocsHeadingFinding {
                file: file.to_string(),
                line: headings[i].line,
                severity: "high".to_string(),
                kind: "skipped-level".to_string(),
                message: format!(
                    "H{cur} heading follows H{prev}, skipping H{}; heading levels must not skip",
                    prev + 1
                ),
            });
        }
    }
    findings
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Verifies that an empty paths slice returns an error.
    #[test]
    fn errors_on_empty_paths() {
        let err = validate_docs_heading_hierarchy(&[]).unwrap_err();
        assert!(err.to_string().contains("at least one path"));
    }

    /// Verifies that a file with one H1 and properly nested headings passes.
    #[test]
    fn passes_when_one_h1_no_skips() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.md"), "# T\n\n## A\n\n### B\n").unwrap();
        let findings =
            validate_docs_heading_hierarchy(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that a file with no H1 emits a `missing-h1` finding.
    #[test]
    fn detects_missing_h1() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.md"), "## H2\n").unwrap();
        let findings =
            validate_docs_heading_hierarchy(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.iter().any(|f| f.kind == "missing-h1"));
    }

    /// Verifies that a file with two H1 headings emits a `duplicate-h1` finding.
    #[test]
    fn detects_duplicate_h1() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.md"), "# T\n\n# Another\n").unwrap();
        let findings =
            validate_docs_heading_hierarchy(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.iter().any(|f| f.kind == "duplicate-h1"));
    }

    /// Verifies that H1 → H3 (skipping H2) emits a `skipped-level` finding.
    #[test]
    fn detects_skipped_level() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.md"), "# T\n\n### Skip\n").unwrap();
        let findings =
            validate_docs_heading_hierarchy(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.iter().any(|f| f.kind == "skipped-level"));
    }

    /// Verifies that headings inside a fenced code block are not counted.
    #[test]
    fn ignores_headings_inside_code_fence() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("a.md"),
            "# T\n\n```\n## Inside fence\n```\n",
        )
        .unwrap();
        let findings =
            validate_docs_heading_hierarchy(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that a nested fence (e.g. inside a four-backtick fence) does not
    /// prematurely close the outer fence.
    #[test]
    fn nested_fence_does_not_close_outer() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("a.md"),
            "# T\n\n````md\n```\n## Inner\n```\n````\n",
        )
        .unwrap();
        let findings =
            validate_docs_heading_hierarchy(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that a completely empty file produces no findings.
    #[test]
    fn empty_file_yields_no_findings() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.md"), "").unwrap();
        let findings =
            validate_docs_heading_hierarchy(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that [`parse_heading_level`] returns the correct level for valid
    /// ATX headings and `None` for invalid inputs.
    #[test]
    fn parse_heading_level_returns_levels() {
        assert_eq!(parse_heading_level("# A"), Some(1));
        assert_eq!(parse_heading_level("##### Five"), Some(5));
        assert_eq!(parse_heading_level("####### Too deep"), None);
        assert_eq!(parse_heading_level("#NoSpace"), None);
        assert_eq!(parse_heading_level("# "), None);
        assert_eq!(parse_heading_level("Not heading"), None);
    }

    /// Verifies that [`parse_fence_open`] correctly identifies and measures fence openers.
    #[test]
    fn parse_fence_open_counts_chars() {
        assert_eq!(parse_fence_open("```"), Some(('`', 3)));
        assert_eq!(parse_fence_open("```rust"), Some(('`', 3)));
        assert_eq!(parse_fence_open("~~~~"), Some(('~', 4)));
        assert_eq!(parse_fence_open("``"), None);
        assert_eq!(parse_fence_open("text"), None);
    }

    // ── Phase 2 RED tests ─────────────────────────────────────────────────────

    /// (a) A `docs/` file with two H1s yields a `duplicate-h1` finding (allowlist runs).
    #[test]
    fn allowlist_runs_for_docs_file() {
        let tmp = TempDir::new().unwrap();
        let docs_dir = tmp.path().join("docs");
        fs::create_dir_all(&docs_dir).unwrap();
        fs::write(docs_dir.join("page.md"), "# First\n\n# Second\n").unwrap();
        // Use the full-repo allowlisted walk rooted at tmp — the docs/ subdir must be scanned.
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.iter().any(|f| f.kind == "duplicate-h1"),
            "docs/ file should be in the prose allowlist and yield a duplicate-h1 finding"
        );
    }

    /// (b) A `.claude/agents/` file with zero H1s yields NO finding (default-deny).
    #[test]
    fn agent_file_is_default_deny() {
        let tmp = TempDir::new().unwrap();
        let agents_dir = tmp.path().join(".claude/agents");
        fs::create_dir_all(&agents_dir).unwrap();
        fs::write(
            agents_dir.join("my-agent.md"),
            "## Not H1\n\n### Also not H1\n",
        )
        .unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.is_empty(),
            ".claude/agents/ file must be default-deny (no findings expected)"
        );
    }

    /// (c) A `SKILL.md` under `.claude/skills/` with many H1s yields NO finding.
    #[test]
    fn skill_file_is_default_deny() {
        let tmp = TempDir::new().unwrap();
        let skill_dir = tmp.path().join(".claude/skills/my-skill");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), "# One\n\n# Two\n\n# Three\n").unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.is_empty(),
            ".claude/skills/ SKILL.md must be default-deny"
        );
    }

    /// (d) A `plans/done/` file with a skipped level yields NO finding (`done` excluded).
    #[test]
    fn plans_done_is_excluded() {
        let tmp = TempDir::new().unwrap();
        let done_dir = tmp.path().join("plans/done/2024-01-01__old-plan");
        fs::create_dir_all(&done_dir).unwrap();
        fs::write(done_dir.join("delivery.md"), "# T\n\n### Skip\n").unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.is_empty(),
            "plans/done/ must be excluded from allowlist"
        );
    }

    /// (e) A `plans/in-progress/` file with a duplicate H1 yields a finding (in allowlist).
    #[test]
    fn plans_in_progress_is_in_allowlist() {
        let tmp = TempDir::new().unwrap();
        let plan_dir = tmp.path().join("plans/in-progress/my-plan");
        fs::create_dir_all(&plan_dir).unwrap();
        fs::write(plan_dir.join("delivery.md"), "# One\n\n# Two\n").unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.iter().any(|f| f.kind == "duplicate-h1"),
            "plans/in-progress/ must be in the allowlist and yield findings"
        );
    }

    /// (f) An `apps/example/README.md` with a skipped level yields a finding
    /// (app/lib top-level READMEs are in the prose allowlist).
    #[test]
    fn apps_readme_is_allowlisted() {
        let tmp = TempDir::new().unwrap();
        let app_dir = tmp.path().join("apps/example");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("README.md"), "# App\n\n### Skip\n").unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.iter().any(|f| f.kind == "skipped-level"),
            "apps/*/README.md must be in the prose allowlist"
        );
    }

    /// (f2) A deep `apps/example/src/notes.md` file yields NO finding (apps internals
    /// stay default-deny; only top-level READMEs and docs/ subtrees are allowlisted).
    #[test]
    fn apps_internals_stay_default_deny() {
        let tmp = TempDir::new().unwrap();
        let src_dir = tmp.path().join("apps/example/src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("notes.md"), "## No H1\n").unwrap();
        // A nested README is also NOT allowlisted — only the app-root README is.
        fs::write(src_dir.join("README.md"), "## No H1\n").unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.is_empty(),
            "apps/*/src/** must be default-deny (no findings expected)"
        );
    }

    /// (f3) A `specs/` prose file with two H1s yields a finding (specs/ is allowlisted).
    #[test]
    fn specs_dir_is_allowlisted() {
        let tmp = TempDir::new().unwrap();
        let specs_dir = tmp.path().join("specs/apps/foo");
        fs::create_dir_all(&specs_dir).unwrap();
        fs::write(specs_dir.join("overview.md"), "# A\n\n# B\n").unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.iter().any(|f| f.kind == "duplicate-h1"),
            "specs/ must be in the prose allowlist"
        );
    }

    /// (f4) A `libs/example/README.md` and `libs/example/docs/**` file yield findings
    /// (lib READMEs + docs subtrees are allowlisted); `apps/*/docs/**` likewise.
    #[test]
    fn lib_readme_and_docs_subtrees_are_allowlisted() {
        let tmp = TempDir::new().unwrap();
        let lib_dir = tmp.path().join("libs/example");
        fs::create_dir_all(lib_dir.join("docs/deep")).unwrap();
        fs::write(lib_dir.join("README.md"), "# Lib\n\n### Skip\n").unwrap();
        fs::write(lib_dir.join("docs/deep/guide.md"), "# A\n\n# B\n").unwrap();
        let app_docs = tmp.path().join("apps/example/docs");
        fs::create_dir_all(&app_docs).unwrap();
        fs::write(app_docs.join("usage.md"), "## No H1\n").unwrap();
        let findings = validate_docs_heading_hierarchy_allowlisted(tmp.path(), &[]).unwrap();
        assert!(
            findings.iter().any(|f| f.kind == "skipped-level"),
            "libs/*/README.md must be allowlisted"
        );
        assert!(
            findings.iter().any(|f| f.kind == "duplicate-h1"),
            "libs/*/docs/** must be allowlisted"
        );
        assert!(
            findings.iter().any(|f| f.kind == "missing-h1"),
            "apps/*/docs/** must be allowlisted"
        );
    }

    /// (g) `--exclude docs` suppresses findings in the `docs` tree while
    /// other allowlist trees still report.
    #[test]
    fn exclude_suppresses_docs_but_not_repo_governance() {
        let tmp = TempDir::new().unwrap();
        let docs_dir = tmp.path().join("docs");
        fs::create_dir_all(&docs_dir).unwrap();
        fs::write(docs_dir.join("page.md"), "# A\n\n# B\n").unwrap();
        let rg_dir = tmp.path().join("repo-governance");
        fs::create_dir_all(&rg_dir).unwrap();
        fs::write(rg_dir.join("rule.md"), "# X\n\n# Y\n").unwrap();
        // Exclude docs/ — repo-governance/ still reports.
        let findings =
            validate_docs_heading_hierarchy_allowlisted(tmp.path(), &["docs".to_string()]).unwrap();
        assert!(
            findings.iter().all(|f| !f.file.contains("docs/page.md")),
            "docs/ findings must be suppressed by --exclude docs"
        );
        assert!(
            findings.iter().any(|f| f.kind == "duplicate-h1"),
            "repo-governance/ must still yield findings when only docs/ is excluded"
        );
    }
}
