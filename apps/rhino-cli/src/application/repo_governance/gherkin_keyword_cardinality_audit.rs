//! Gherkin step-keyword cardinality audit for `.feature` files.
//!
//! Enforces the HARD rule from
//! `repo-governance/development/infra/acceptance-criteria.md`: every
//! `Scenario` uses exactly one primary `Given`, one `When`, and one `Then`
//! keyword line, with all extra steps chained via `And`/`But`. `Background`
//! blocks and `Scenario Outline` `Examples` tables are exempt; keyword words
//! inside doc-strings (`"""`) and comments (`#`) are ignored.

use std::path::Path;

use anyhow::{Context, Error, anyhow};

use crate::application::fs::port::Fs;

/// A single step-keyword cardinality violation found in a `.feature` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GherkinCardinalityFinding {
    /// Path of the `.feature` file containing the violation.
    pub file: String,
    /// 1-based line number of the scenario declaration.
    pub line: usize,
    /// Name of the offending scenario (text after `Scenario:`).
    pub scenario: String,
    /// Primary keyword that appears more than once (`Given`, `When`, `Then`).
    pub keyword: String,
    /// Number of primary occurrences of `keyword` in the scenario.
    pub count: usize,
    /// Severity; currently always `"high"`.
    pub severity: String,
}

/// Walks each directory in `paths` and reports any scenario in a `.feature`
/// file that uses a primary `Given`/`When`/`Then` keyword more than once.
///
/// Findings are sorted by `file`, then `line`, then `keyword`.
///
/// # Errors
///
/// Returns an error when `paths` is empty or when a file cannot be read
/// during the scan.
pub fn audit_gherkin_keyword_cardinality(
    fs: &dyn Fs,
    paths: &[String],
) -> std::result::Result<Vec<GherkinCardinalityFinding>, Error> {
    if paths.is_empty() {
        return Err(anyhow!("at least one path is required"));
    }
    let mut findings = Vec::new();
    for root in paths {
        for file in walk_feature_paths(fs, Path::new(root)) {
            let content = fs
                .read_to_string(&file)
                .with_context(|| format!("read {}", file.display()))?;
            let mut more = scan_feature_content(&file.to_string_lossy(), &content);
            findings.append(&mut more);
        }
    }
    findings.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.cmp(&b.line))
            .then(a.keyword.cmp(&b.keyword))
    });
    Ok(findings)
}

/// Primary Gherkin step keywords subject to the cardinality rule.
const PRIMARY_KEYWORDS: [&str; 3] = ["Given", "When", "Then"];

/// Directory names skipped during the walk (build outputs, vendored code,
/// worktrees, and archived sources).
const GHERKIN_SKIP_DIRS: &[&str] = &[
    "node_modules",
    ".git",
    "bin",
    "build",
    "target",
    "dist",
    "worktrees",
    "archived",
];

/// Path fragments excluded from the scan: BDD-library self-test fixtures
/// that deliberately use non-conforming Gherkin shapes.
const EXCLUDED_PATH_FRAGMENTS: [&str; 2] = [
    "libs/elixir-cabbage/test/features/",
    "libs/elixir-gherkin/test/fixtures/",
];

/// Returns `true` when the slash-normalised `path` falls inside one of the
/// [`EXCLUDED_PATH_FRAGMENTS`].
fn is_excluded_feature_path(path: &str) -> bool {
    let slashed = path.replace('\\', "/");
    EXCLUDED_PATH_FRAGMENTS
        .iter()
        .any(|frag| slashed.contains(frag))
}

/// Recursively walks `root` and returns sorted paths of `.feature` files,
/// skipping directories in [`GHERKIN_SKIP_DIRS`] and excluded fixture paths.
fn walk_feature_paths(fs: &dyn Fs, root: &Path) -> Vec<std::path::PathBuf> {
    let mut files: Vec<std::path::PathBuf> = fs
        .walk_files(root, GHERKIN_SKIP_DIRS)
        .into_iter()
        .filter(|p| {
            p.file_name()
                .is_some_and(|n| n.to_string_lossy().to_lowercase().ends_with(".feature"))
        })
        .filter(|p| !is_excluded_feature_path(&p.to_string_lossy()))
        .collect();
    files.sort();
    files
}

/// Mutable parse state for one `.feature` file scan.
struct ScanState {
    /// `true` while inside a doc-string (`"""` or `` ``` `` fences).
    in_doc_string: bool,
    /// `true` while inside an `Examples:` table of a `Scenario Outline`.
    in_examples: bool,
    /// Current countable scenario: `(name, declaration line)`.
    scenario: Option<(String, usize)>,
    /// Primary keyword counts for the current scenario, indexed in step with
    /// [`PRIMARY_KEYWORDS`].
    counts: [usize; 3],
}

/// Scans the `content` of a single `.feature` file (identified as `file` in
/// findings) and returns all step-keyword cardinality violations.
fn scan_feature_content(file: &str, content: &str) -> Vec<GherkinCardinalityFinding> {
    let mut findings = Vec::new();
    let mut state = ScanState {
        in_doc_string: false,
        in_examples: false,
        scenario: None,
        counts: [0; 3],
    };
    for (idx, raw) in content.lines().enumerate() {
        let line_num = idx + 1;
        let class = classify_line(raw.trim());
        if state.in_doc_string {
            if matches!(class, LineClass::DocStringDelimiter) {
                state.in_doc_string = false;
            }
            continue;
        }
        match class {
            LineClass::DocStringDelimiter => state.in_doc_string = true,
            LineClass::Comment | LineClass::Other => {}
            LineClass::Header(header) => {
                apply_block_header(&mut state, &mut findings, file, header, line_num);
            }
            LineClass::PrimaryStep(k) => {
                if state.scenario.is_some() && !state.in_examples {
                    state.counts[k] += 1;
                }
            }
        }
    }
    flush_scenario(&mut state, &mut findings, file);
    findings
}

/// Classification of a single trimmed `.feature` line.
enum LineClass {
    /// Doc-string fence delimiter (`"""` or `` ``` ``).
    DocStringDelimiter,
    /// Comment line starting with `#`.
    Comment,
    /// Recognised Gherkin block header.
    Header(BlockHeader),
    /// Primary step line; the payload indexes [`PRIMARY_KEYWORDS`].
    PrimaryStep(usize),
    /// Anything else: `And`/`But`/`*` continuations, table rows, tags, prose.
    Other,
}

/// Classifies a trimmed `.feature` line for the cardinality scanner.
fn classify_line(trimmed: &str) -> LineClass {
    if is_doc_string_delimiter(trimmed) {
        return LineClass::DocStringDelimiter;
    }
    if trimmed.starts_with('#') {
        return LineClass::Comment;
    }
    if let Some(header) = parse_block_header(trimmed) {
        return LineClass::Header(header);
    }
    if let Some(k) = primary_keyword_index(trimmed) {
        return LineClass::PrimaryStep(k);
    }
    LineClass::Other
}

/// A Gherkin block header recognised by the scanner.
enum BlockHeader {
    /// `Background:` — exempt from the cardinality rule.
    Background,
    /// `Scenario:` / `Scenario Outline:` / `Scenario Template:` with its name.
    Scenario(String),
    /// `Examples:` / `Scenarios:` — table of a `Scenario Outline`, exempt.
    Examples,
    /// `Feature:` / `Rule:` — structural headers that end any open scenario.
    Structural,
}

/// Parses `trimmed` as a Gherkin block header, if it is one.
fn parse_block_header(trimmed: &str) -> Option<BlockHeader> {
    if trimmed.starts_with("Background:") {
        return Some(BlockHeader::Background);
    }
    for prefix in ["Scenario Outline:", "Scenario Template:", "Scenario:"] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return Some(BlockHeader::Scenario(rest.trim().to_string()));
        }
    }
    if trimmed.starts_with("Examples:") || trimmed.starts_with("Scenarios:") {
        return Some(BlockHeader::Examples);
    }
    if trimmed.starts_with("Feature:") || trimmed.starts_with("Rule:") {
        return Some(BlockHeader::Structural);
    }
    None
}

/// Applies a parsed block `header` to the scan `state`, flushing any open
/// scenario when the header ends it.
fn apply_block_header(
    state: &mut ScanState,
    findings: &mut Vec<GherkinCardinalityFinding>,
    file: &str,
    header: BlockHeader,
    line_num: usize,
) {
    match header {
        BlockHeader::Scenario(name) => {
            flush_scenario(state, findings, file);
            state.scenario = Some((name, line_num));
            state.in_examples = false;
        }
        BlockHeader::Background | BlockHeader::Structural => {
            flush_scenario(state, findings, file);
            state.in_examples = false;
        }
        BlockHeader::Examples => {
            state.in_examples = true;
        }
    }
}

/// Emits findings for the open scenario in `state` (each primary keyword
/// counted more than once) and resets the per-scenario counters.
fn flush_scenario(
    state: &mut ScanState,
    findings: &mut Vec<GherkinCardinalityFinding>,
    file: &str,
) {
    if let Some((name, line)) = state.scenario.take() {
        for (i, keyword) in PRIMARY_KEYWORDS.iter().enumerate() {
            if state.counts[i] > 1 {
                findings.push(GherkinCardinalityFinding {
                    file: file.to_string(),
                    line,
                    scenario: name.clone(),
                    keyword: (*keyword).to_string(),
                    count: state.counts[i],
                    severity: "high".to_string(),
                });
            }
        }
    }
    state.counts = [0; 3];
}

/// Returns `true` when `trimmed` opens or closes a Gherkin doc-string
/// (`"""` or `` ``` `` fence, optionally followed by a content type).
fn is_doc_string_delimiter(trimmed: &str) -> bool {
    trimmed.starts_with("\"\"\"") || trimmed.starts_with("```")
}

/// Returns the [`PRIMARY_KEYWORDS`] index when `trimmed` starts with a
/// primary `Given`/`When`/`Then` keyword followed by whitespace.
///
/// `And`/`But`/`*` continuation lines and non-step lines return `None`.
fn primary_keyword_index(trimmed: &str) -> Option<usize> {
    PRIMARY_KEYWORDS.iter().position(|kw| {
        trimmed
            .strip_prefix(kw)
            .is_some_and(|rest| rest.starts_with(char::is_whitespace))
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::real::RealFs;
    use std::fs;
    use tempfile::TempDir;

    /// Writes `content` to `name` under `dir` and returns the scan-root path.
    fn write_feature(dir: &TempDir, name: &str, content: &str) -> String {
        fs::write(dir.path().join(name), content).unwrap();
        dir.path().to_string_lossy().to_string()
    }

    #[test]
    fn flags_scenario_with_multiple_when_lines() {
        let tmp = TempDir::new().unwrap();
        let root = write_feature(
            &tmp,
            "sample.feature",
            "Feature: Sample\n\n  Scenario: Double when\n    Given a precondition\n    When the first action runs\n    When the second action runs\n    Then the outcome is checked\n",
        );
        let findings = audit_gherkin_keyword_cardinality(&RealFs, &[root]).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].keyword, "When");
        assert_eq!(findings[0].count, 2);
        assert_eq!(findings[0].scenario, "Double when");
        assert_eq!(findings[0].severity, "high");
        assert!(findings[0].file.ends_with("sample.feature"));
    }

    #[test]
    fn exempts_background_block() {
        let tmp = TempDir::new().unwrap();
        // The Background block repeats Given twice (exempt); the scenario
        // repeats When twice (one finding). Exactly one finding expected.
        let root = write_feature(
            &tmp,
            "background.feature",
            "Feature: Background exemption\n\n  Background:\n    Given a shared precondition\n    Given another shared precondition\n\n  Scenario: Offender\n    When the first action runs\n    When the second action runs\n    Then the outcome is checked\n",
        );
        let findings = audit_gherkin_keyword_cardinality(&RealFs, &[root]).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].keyword, "When");
        assert_eq!(findings[0].scenario, "Offender");
    }

    #[test]
    fn exempts_scenario_outline_examples() {
        let tmp = TempDir::new().unwrap();
        // The Scenario Outline conforms and its Examples table is exempt;
        // the second scenario repeats Then twice. Exactly one finding expected.
        let root = write_feature(
            &tmp,
            "outline.feature",
            "Feature: Outline exemption\n\n  Scenario Outline: Conforming outline\n    Given a precondition\n    When the action runs with <input>\n    Then the result is <output>\n\n    Examples:\n      | input | output |\n      | When  | Then   |\n      | one   | two    |\n\n  Scenario: Offender\n    Given a precondition\n    When the action runs\n    Then the first check passes\n    Then the second check passes\n",
        );
        let findings = audit_gherkin_keyword_cardinality(&RealFs, &[root]).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].keyword, "Then");
        assert_eq!(findings[0].scenario, "Offender");
        assert_eq!(findings[0].count, 2);
    }

    #[test]
    fn ignores_keyword_words_in_docstrings_and_comments() {
        let tmp = TempDir::new().unwrap();
        // Doc-string and comment lines contain primary-looking keywords that
        // must be ignored; the second scenario repeats Given twice (one
        // finding). Exactly one finding expected.
        let root = write_feature(
            &tmp,
            "docstring.feature",
            "Feature: Doc-string and comment exemption\n\n  Scenario: Conforming with noise\n    Given a precondition\n    # When this comment must be ignored\n    When the action runs with payload:\n      \"\"\"\n      When embedded in a doc-string\n      Then also embedded in a doc-string\n      \"\"\"\n    Then the outcome is checked\n\n  Scenario: Offender\n    Given the first precondition\n    Given the second precondition\n    When the action runs\n    Then the outcome is checked\n",
        );
        let findings = audit_gherkin_keyword_cardinality(&RealFs, &[root]).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].keyword, "Given");
        assert_eq!(findings[0].scenario, "Offender");
    }

    #[test]
    fn continuation_keywords_are_not_primary() {
        let tmp = TempDir::new().unwrap();
        let root = write_feature(
            &tmp,
            "conforming.feature",
            "Feature: Conforming\n\n  Scenario: One each with continuations\n    Given a precondition\n    And another precondition\n    When the action runs\n    But nothing else happens\n    Then the outcome is checked\n    * the extra outcome is checked\n",
        );
        let findings = audit_gherkin_keyword_cardinality(&RealFs, &[root]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn audit_empty_paths_errors() {
        let err = audit_gherkin_keyword_cardinality(&RealFs, &[]).unwrap_err();
        assert!(err.to_string().contains("at least one path"));
    }

    #[test]
    fn walk_skips_excluded_fixture_and_skip_dirs() {
        let tmp = TempDir::new().unwrap();
        let violation = "Feature: F\n\n  Scenario: S\n    When one\n    When two\n";
        for sub in [
            "libs/elixir-cabbage/test/features",
            "libs/elixir-gherkin/test/fixtures",
            "worktrees/x",
            "archived/y",
            "target/z",
        ] {
            let dir = tmp.path().join(sub);
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("v.feature"), violation).unwrap();
        }
        let findings =
            audit_gherkin_keyword_cardinality(&RealFs, &[tmp.path().to_string_lossy().to_string()])
                .unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn walk_nonexistent_root_yields_no_findings() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does-not-exist");
        let findings =
            audit_gherkin_keyword_cardinality(&RealFs, &[missing.to_string_lossy().to_string()])
                .unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn findings_sorted_by_file_then_line() {
        let tmp = TempDir::new().unwrap();
        let two_offenders = "Feature: F\n\n  Scenario: First\n    When one\n    When two\n\n  Scenario: Second\n    Then one\n    Then two\n";
        write_feature(&tmp, "b.feature", two_offenders);
        let root = write_feature(
            &tmp,
            "a.feature",
            "Feature: F\n\n  Scenario: Only\n    Given one\n    Given two\n",
        );
        let findings = audit_gherkin_keyword_cardinality(&RealFs, &[root]).unwrap();
        assert_eq!(findings.len(), 3);
        assert!(findings[0].file.ends_with("a.feature"));
        assert!(findings[1].file.ends_with("b.feature"));
        assert!(findings[1].line < findings[2].line);
    }

    #[test]
    fn scan_reports_multiple_keywords_in_one_scenario() {
        let findings = scan_feature_content(
            "multi.feature",
            "Feature: F\n\n  Scenario: Multi\n    Given one\n    Given two\n    When one\n    When two\n    Then only\n",
        );
        assert_eq!(findings.len(), 2);
        let keywords: Vec<&str> = findings.iter().map(|f| f.keyword.as_str()).collect();
        assert!(keywords.contains(&"Given"));
        assert!(keywords.contains(&"When"));
    }

    #[test]
    fn scan_flags_scenario_outline_steps_outside_examples() {
        // Scenario Outline steps remain subject to the rule — only the
        // Examples table itself is exempt.
        let findings = scan_feature_content(
            "outline-steps.feature",
            "Feature: F\n\n  Scenario Outline: Offending outline\n    Given a precondition\n    When the first action with <x>\n    When the second action with <x>\n    Then the result is <y>\n\n    Examples:\n      | x | y |\n      | 1 | 2 |\n",
        );
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].keyword, "When");
        assert_eq!(findings[0].scenario, "Offending outline");
    }
}
