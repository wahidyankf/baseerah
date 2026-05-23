// Port of `apps/rhino-cli/internal/speccoverage/checker.go`.
// Spec coverage scanner — walks .feature trees + source trees, matches step
// definitions to Gherkin scenarios, returns gaps + orphan step impls.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Instant;

use anyhow::Error;
use regex::Regex;
use walkdir::WalkDir;

use super::extractors;
use super::matcher::{add_step_to_matcher_with_origin, MatcherKind, StepMatcher};
use super::parser::{parse_feature_file, ParsedStep};
use super::types::{CheckResult, CoverageGap, OrphanStepImpl, ScanOptions, ScenarioGap, StepGap};
use super::util::{first_non_empty, normalize_ws, unescape_string};

// ============================================================
// TS/JS extraction regexes (live inline in Go checker.go).
// ============================================================

fn scenario_def_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"Scenario\s*\(\s*(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*,"#)
            .expect("valid regex")
    })
}

fn step_def_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?s)(?:Given|When|Then|And|But)\s*\(\s*(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*,"#).expect("valid regex")
    })
}

fn ts_regex_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?s)(?:Given|When|Then|And|But)\s*\(\s*/\^?(.*?)\$?\s*/\s*,")
            .expect("valid regex")
    })
}

fn go_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\.Step\(`([^`]+)`").expect("valid regex"))
}

fn go_scenario_comment_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"//\s*Scenario:\s*(.+?)\s*$").expect("valid regex"))
}

// ============================================================
// Constants
// ============================================================

fn skip_dirs() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        let arr = [
            "node_modules",
            ".next",
            "build",
            "dist",
            "storybook-static",
            "coverage",
            ".git",
            "target",
            "_build",
            "deps",
            "bin",
            "obj",
            "__pycache__",
            ".pytest_cache",
            ".venv",
            "generated-contracts",
            "generated_contracts",
            ".dart_tool",
            ".features-gen",
        ];
        arr.into_iter().collect()
    })
}

// ============================================================
// Public entry point
// ============================================================

pub fn check_all(opts: &ScanOptions) -> std::result::Result<CheckResult, Error> {
    let mut effective = opts.clone();
    if effective.specs_dirs.is_empty() && !effective.specs_dir.as_os_str().is_empty() {
        effective.specs_dirs = vec![effective.specs_dir.clone()];
    }
    if effective.shared_steps {
        check_shared_steps(&effective)
    } else {
        check_one_to_one(&effective)
    }
}

fn collect_feature_files(opts: &ScanOptions) -> std::result::Result<Vec<PathBuf>, Error> {
    let dirs: Vec<PathBuf> = if !opts.specs_dirs.is_empty() {
        opts.specs_dirs.clone()
    } else if !opts.specs_dir.as_os_str().is_empty() {
        vec![opts.specs_dir.clone()]
    } else {
        Vec::new()
    };
    let mut all = Vec::new();
    for dir in &dirs {
        all.extend(walk_feature_files(dir, &opts.exclude_dirs)?);
    }
    Ok(all)
}

fn check_shared_steps(opts: &ScanOptions) -> std::result::Result<CheckResult, Error> {
    let start = Instant::now();
    let spec_files = collect_feature_files(opts)?;
    let all_step_texts = extract_all_step_texts(&opts.app_dir)?;
    let mut step_gaps: Vec<StepGap> = Vec::new();
    let mut all_gherkin_steps: Vec<String> = Vec::new();
    let mut total_scenarios = 0usize;
    let mut total_steps = 0usize;

    for spec_file in &spec_files {
        let rel_spec = rel_to(&opts.repo_root, spec_file);
        let scenarios = parse_feature_file(spec_file)?;
        for sc in &scenarios {
            total_scenarios += 1;
            for step in &sc.steps {
                total_steps += 1;
                all_gherkin_steps.push(step.text.clone());
                all_gherkin_steps.extend(step.variants.iter().cloned());
                if !step_covered(&all_step_texts, step) {
                    step_gaps.push(StepGap {
                        spec_file: rel_spec.clone(),
                        scenario_title: sc.title.clone(),
                        step_keyword: step.keyword.clone(),
                        step_text: step.text.clone(),
                    });
                }
            }
        }
    }

    let orphans = check_orphan_step_impls(&all_step_texts, &all_gherkin_steps, &opts.repo_root);

    Ok(CheckResult {
        total_specs: spec_files.len(),
        total_scenarios,
        total_steps,
        gaps: Vec::new(),
        scenario_gaps: Vec::new(),
        step_gaps,
        orphan_step_impls: orphans,
        duration: start.elapsed(),
    })
}

fn check_one_to_one(opts: &ScanOptions) -> std::result::Result<CheckResult, Error> {
    let start = Instant::now();
    let spec_files = collect_feature_files(opts)?;
    let all_step_texts = extract_all_step_texts(&opts.app_dir)?;
    let mut gaps: Vec<CoverageGap> = Vec::new();
    let mut scenario_gaps: Vec<ScenarioGap> = Vec::new();
    let mut step_gaps: Vec<StepGap> = Vec::new();
    let mut all_gherkin_steps: Vec<String> = Vec::new();
    let mut total_scenarios = 0usize;
    let mut total_steps = 0usize;

    for spec_file in &spec_files {
        let stem = spec_file
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.trim_end_matches(".feature").to_string())
            .unwrap_or_default();

        let test_file_paths = find_all_matching_test_files(&opts.app_dir, &stem)?;

        if test_file_paths.is_empty() {
            let rel_path = rel_to(&opts.repo_root, spec_file);
            gaps.push(CoverageGap {
                spec_file: rel_path,
                stem: stem.clone(),
            });
            // Still collect Gherkin step texts from this file for orphan check.
            let scenarios = parse_feature_file(spec_file)?;
            for sc in &scenarios {
                for step in &sc.steps {
                    all_gherkin_steps.push(step.text.clone());
                }
            }
            continue;
        }

        let rel_spec = rel_to(&opts.repo_root, spec_file);
        let scenarios = parse_feature_file(spec_file)?;

        let mut scenario_titles: HashSet<String> = HashSet::new();
        for test_file in &test_file_paths {
            let titles = extract_scenario_titles(test_file)?;
            scenario_titles.extend(titles);
        }

        for sc in &scenarios {
            total_scenarios += 1;
            let normalized = normalize_ws(&sc.title);
            if !scenario_titles.contains(&normalized) {
                scenario_gaps.push(ScenarioGap {
                    spec_file: rel_spec.clone(),
                    scenario_title: sc.title.clone(),
                });
            }

            for step in &sc.steps {
                total_steps += 1;
                all_gherkin_steps.push(step.text.clone());
                all_gherkin_steps.extend(step.variants.iter().cloned());
                if !step_covered(&all_step_texts, step) {
                    step_gaps.push(StepGap {
                        spec_file: rel_spec.clone(),
                        scenario_title: sc.title.clone(),
                        step_keyword: step.keyword.clone(),
                        step_text: step.text.clone(),
                    });
                }
            }
        }
    }

    let orphans = check_orphan_step_impls(&all_step_texts, &all_gherkin_steps, &opts.repo_root);

    Ok(CheckResult {
        total_specs: spec_files.len(),
        total_scenarios,
        total_steps,
        gaps,
        scenario_gaps,
        step_gaps,
        orphan_step_impls: orphans,
        duration: start.elapsed(),
    })
}

// ============================================================
// Coverage helpers
// ============================================================

fn step_covered(sm: &StepMatcher, step: &ParsedStep) -> bool {
    if sm.matches(&step.text) {
        return true;
    }
    if step.variants.is_empty() {
        return false;
    }
    step.variants.iter().all(|v| sm.matches(v))
}

fn check_orphan_step_impls(
    sm: &StepMatcher,
    all_gherkin_steps: &[String],
    repo_root: &Path,
) -> Vec<OrphanStepImpl> {
    if sm.entries.is_empty() {
        return Vec::new();
    }
    let normalized: Vec<String> = all_gherkin_steps
        .iter()
        .map(|gs| normalize_ws(gs))
        .collect();

    let mut orphans = Vec::new();
    for (i, e) in sm.entries.iter().enumerate() {
        let matched = match e.kind {
            MatcherKind::Exact => normalized.iter().any(|gs| gs == &e.exact_text),
            MatcherKind::Pattern => {
                // entries[i] corresponds to patterns[?]; we tracked pattern compilation
                // via sm.patterns. Walk that list against pattern_text for identity.
                sm.patterns
                    .iter()
                    .filter(|re| re.as_str() == e.pattern_text)
                    .any(|re| normalized.iter().any(|gs| re.is_match(gs)))
                    || sm
                        .patterns
                        .get(pattern_index_for_entry(sm, i))
                        .is_some_and(|re| normalized.iter().any(|gs| re.is_match(gs)))
            }
        };
        if matched {
            continue;
        }
        let text = if matches!(e.kind, MatcherKind::Pattern) {
            e.pattern_text.clone()
        } else {
            e.exact_text.clone()
        };
        let file_path = if !repo_root.as_os_str().is_empty() {
            Path::new(&e.file)
                .strip_prefix(repo_root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| e.file.clone())
        } else {
            e.file.clone()
        };
        orphans.push(OrphanStepImpl {
            file: file_path,
            matcher_kind: match e.kind {
                MatcherKind::Exact => "exact".to_string(),
                MatcherKind::Pattern => "pattern".to_string(),
            },
            matcher_text: text,
        });
    }
    orphans
}

/// Approximates Go's `entries[i].Pattern` direct reference — counts how many
/// pattern entries precede `i` in `entries`, returning that index into
/// `patterns`. Safe because `add_pattern_with_origin` appends to both vectors
/// in lockstep.
fn pattern_index_for_entry(sm: &StepMatcher, i: usize) -> usize {
    sm.entries
        .iter()
        .take(i)
        .filter(|e| matches!(e.kind, MatcherKind::Pattern))
        .count()
}

// ============================================================
// Walking
// ============================================================

fn walk_feature_files(
    dir: &Path,
    exclude_dirs: &[String],
) -> std::result::Result<Vec<PathBuf>, Error> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let excl: HashSet<&str> = exclude_dirs.iter().map(String::as_str).collect();
    let mut files = Vec::new();
    let walker = WalkDir::new(dir).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy();
            !excl.contains(name.as_ref())
        } else {
            true
        }
    });
    for entry in walker {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().to_string_lossy().ends_with(".feature") {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(files)
}

fn to_pascal_case(stem: &str) -> String {
    let mut b = String::new();
    for p in stem.split('-') {
        if p.is_empty() {
            continue;
        }
        let mut chars = p.chars();
        if let Some(c) = chars.next() {
            for u in c.to_uppercase() {
                b.push(u);
            }
            b.push_str(chars.as_str());
        }
    }
    b
}

fn matches_stem(base: &str, stem: &str) -> bool {
    let snake = stem.replace('-', "_");
    let pascal = to_pascal_case(stem);
    let test_snake = format!("test_{snake}");

    let prefixes = [
        format!("{stem}."),
        format!("{stem}_"),
        format!("{snake}."),
        format!("{snake}_"),
        pascal.clone(),
        format!("{test_snake}."),
        format!("{test_snake}_"),
    ];
    for prefix in &prefixes {
        if base.starts_with(prefix) {
            return true;
        }
    }
    base == stem || base == snake
}

fn is_test_file(path: &Path) -> bool {
    let base = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    match ext {
        "" => true, // exact stem match w/o extension
        "go" => base.ends_with("_test.go"),
        "ts" | "tsx" | "js" | "jsx" => {
            base.contains(".test.")
                || base.contains(".spec.")
                || base.contains(".steps.")
                || base.contains(".integration.")
                || base.contains("_test.")
        }
        "java" | "kt" => is_in_test_dir(path),
        "py" => base.starts_with("test_") || base.ends_with("_test.py") || is_in_test_dir(path),
        "exs" => base.ends_with("_test.exs") || base.ends_with("_steps.exs"),
        "rs" => base.ends_with("_test.rs") || is_in_test_dir(path),
        "fs" | "cs" => {
            is_in_test_dir(path)
                || base.ends_with("Steps.cs")
                || base.ends_with("Tests.cs")
                || base.ends_with("Steps.fs")
                || base.ends_with("Tests.fs")
        }
        "clj" => base.ends_with("_test.clj") || base.ends_with("_steps.clj"),
        "dart" => base.ends_with("_test.dart") || is_in_test_dir(path),
        _ => false,
    }
}

fn is_in_test_dir(path: &Path) -> bool {
    path.components().any(|comp| {
        matches!(
            comp.as_os_str().to_str(),
            Some("test") | Some("tests") | Some("Tests")
        )
    })
}

fn find_all_matching_test_files(
    app_dir: &Path,
    stem: &str,
) -> std::result::Result<Vec<PathBuf>, Error> {
    if !app_dir.exists() {
        return Ok(Vec::new());
    }
    let mut matches = Vec::new();
    let walker = WalkDir::new(app_dir).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy();
            !skip_dirs().contains(name.as_ref())
        } else {
            true
        }
    });
    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let base = entry.file_name().to_string_lossy();
        if matches_stem(&base, stem) && is_test_file(entry.path()) {
            matches.push(entry.path().to_path_buf());
        }
    }
    Ok(matches)
}

// ============================================================
// Scenario title extraction (dispatch by ext)
// ============================================================

fn extract_scenario_titles(test_file_path: &Path) -> std::result::Result<HashSet<String>, Error> {
    let ext = test_file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match ext {
        "go" | "java" | "kt" | "cs" | "rs" | "dart" => extract_go_scenario_titles(test_file_path),
        "py" => extractors::extract_python_scenario_titles(test_file_path),
        "exs" | "fs" | "clj" => Ok(HashSet::new()), // auto-bind frameworks
        _ => extract_ts_scenario_titles(test_file_path),
    }
}

fn extract_ts_scenario_titles(p: &Path) -> std::result::Result<HashSet<String>, Error> {
    let content = fs::read_to_string(p)?;
    let mut titles = HashSet::new();
    for line in content.lines() {
        for caps in scenario_def_re().captures_iter(line) {
            let dq = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let sq = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let title = unescape_string(first_non_empty(dq, sq));
            titles.insert(normalize_ws(&title));
        }
    }
    Ok(titles)
}

fn extract_go_scenario_titles(p: &Path) -> std::result::Result<HashSet<String>, Error> {
    let content = fs::read_to_string(p)?;
    let mut titles = HashSet::new();
    for line in content.lines() {
        if let Some(caps) = go_scenario_comment_re().captures(line) {
            titles.insert(normalize_ws(caps.get(1).unwrap().as_str()));
        }
    }
    Ok(titles)
}

// ============================================================
// Whole-app step extraction (walks + per-ext dispatch)
// ============================================================

pub fn extract_all_step_texts(app_dir: &Path) -> std::result::Result<StepMatcher, Error> {
    let mut sm = StepMatcher::new();
    if !app_dir.exists() {
        return Ok(sm);
    }

    let walker = WalkDir::new(app_dir).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy();
            !skip_dirs().contains(name.as_ref())
        } else {
            true
        }
    });

    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let _ = match ext {
            "ts" | "tsx" | "js" | "jsx" => extract_ts_step_texts(path, &mut sm),
            "go" => extract_go_step_texts(path, &mut sm),
            "java" | "kt" => extractors::extract_jvm_step_texts(path, &mut sm),
            "py" => extractors::extract_python_step_texts(path, &mut sm),
            "ex" | "exs" => extractors::extract_elixir_step_texts(path, &mut sm),
            "rs" => extractors::extract_rust_step_texts(path, &mut sm),
            "cs" => extractors::extract_csharp_step_texts(path, &mut sm),
            "fs" => extractors::extract_fsharp_step_texts(path, &mut sm),
            "clj" => extractors::extract_clojure_step_texts(path, &mut sm),
            "dart" => extractors::extract_dart_step_texts(path, &mut sm),
            _ => Ok(()),
        };
    }
    Ok(sm)
}

fn extract_ts_step_texts(path: &Path, sm: &mut StepMatcher) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let src = strip_js_comments(&content);
    let path_s = path.to_string_lossy();

    for caps in step_def_re().captures_iter(&src) {
        let dq = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let sq = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let text = unescape_string(first_non_empty(dq, sq));
        add_step_to_matcher_with_origin(sm, &text, &path_s);
    }
    for caps in ts_regex_step_re().captures_iter(&src) {
        let pattern = caps.get(1).unwrap().as_str();
        if let Ok(re) = Regex::new(pattern) {
            sm.add_pattern_with_origin(re, pattern, &path_s);
        }
    }
    Ok(())
}

fn extract_go_step_texts(path: &Path, sm: &mut StepMatcher) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    for line in content.lines() {
        for caps in go_step_re().captures_iter(line) {
            let pattern = caps.get(1).unwrap().as_str();
            if let Ok(re) = Regex::new(pattern) {
                sm.add_pattern_with_origin(re, pattern, &path_s);
            }
        }
    }
    Ok(())
}

/// Strip JS/TS comments from source.
/// - `/* ... */` anywhere (multi-line aware).
/// - `// ...` only when after whitespace at line start.
/// - Preserves string/template literals verbatim.
fn strip_js_comments(src: &str) -> String {
    let bytes = src.as_bytes();
    let n = bytes.len();
    let mut out = String::with_capacity(n);
    let mut i = 0usize;
    let mut at_line_start = true;
    while i < n {
        let c = bytes[i];
        if c == b'\n' {
            out.push('\n');
            i += 1;
            at_line_start = true;
            continue;
        }
        if c == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
            let mut j = i + 2;
            while j + 1 < n && !(bytes[j] == b'*' && bytes[j + 1] == b'/') {
                if bytes[j] == b'\n' {
                    out.push('\n');
                }
                j += 1;
            }
            i = j + 2;
            continue;
        }
        if at_line_start && c == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            let mut j = i + 2;
            while j < n && bytes[j] != b'\n' {
                j += 1;
            }
            i = j;
            continue;
        }
        if c == b'"' || c == b'\'' || c == b'`' {
            let quote = c;
            out.push(c as char);
            i += 1;
            while i < n {
                if bytes[i] == b'\\' && i + 1 < n {
                    out.push(bytes[i] as char);
                    out.push(bytes[i + 1] as char);
                    i += 2;
                    continue;
                }
                out.push(bytes[i] as char);
                if bytes[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            at_line_start = false;
            continue;
        }
        out.push(c as char);
        if c != b' ' && c != b'\t' {
            at_line_start = false;
        }
        i += 1;
    }
    out
}

// ============================================================
// Path helpers
// ============================================================

fn rel_to(root: &Path, p: &Path) -> String {
    if root.as_os_str().is_empty() {
        return p.to_string_lossy().to_string();
    }
    p.strip_prefix(root)
        .map(|r| r.to_string_lossy().to_string())
        .unwrap_or_else(|_| p.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn to_pascal_case_handles_kebab() {
        assert_eq!(to_pascal_case("health-check"), "HealthCheck");
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("a-b-c"), "ABC");
    }

    #[test]
    fn matches_stem_kebab_snake_pascal_test_prefix() {
        assert!(matches_stem("user-login.test.ts", "user-login"));
        assert!(matches_stem("user_login.test.ts", "user-login"));
        assert!(matches_stem("UserLogin", "user-login"));
        assert!(matches_stem("test_user_login.py", "user-login"));
        assert!(matches_stem("user-login", "user-login"));
        assert!(!matches_stem("other.ts", "user-login"));
    }

    #[test]
    fn is_test_file_typescript() {
        assert!(is_test_file(Path::new("x.test.ts")));
        assert!(is_test_file(Path::new("x.spec.ts")));
        assert!(is_test_file(Path::new("x.steps.ts")));
        assert!(!is_test_file(Path::new("x.ts")));
    }

    #[test]
    fn is_test_file_go() {
        assert!(is_test_file(Path::new("foo_test.go")));
        assert!(!is_test_file(Path::new("foo.go")));
    }

    #[test]
    fn is_test_file_python() {
        assert!(is_test_file(Path::new("test_foo.py")));
        assert!(is_test_file(Path::new("foo_test.py")));
        assert!(is_test_file(Path::new("tests/foo.py")));
    }

    #[test]
    fn is_in_test_dir_detects_test_segments() {
        assert!(is_in_test_dir(Path::new("a/tests/b.rs")));
        assert!(is_in_test_dir(Path::new("a/test/b.rs")));
        assert!(is_in_test_dir(Path::new("a/Tests/b.cs")));
        assert!(!is_in_test_dir(Path::new("a/b/c.rs")));
    }

    #[test]
    fn strip_js_comments_removes_block_and_line_at_line_start() {
        let s = "/* drop */ keep\n// drop\n  // drop\nreal_code(); // keep\n";
        let out = strip_js_comments(s);
        assert!(!out.contains("drop"));
        assert!(out.contains("real_code()"));
        assert!(out.contains("// keep")); // not at line start → preserved
    }

    #[test]
    fn strip_js_comments_preserves_strings() {
        let s = r#"const x = "// not a comment"; foo();"#;
        let out = strip_js_comments(s);
        assert!(out.contains("// not a comment"));
    }

    #[test]
    fn walk_feature_files_returns_features_recursively() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("a/b")).unwrap();
        std::fs::write(tmp.path().join("x.feature"), "Feature: x").unwrap();
        std::fs::write(tmp.path().join("a/y.feature"), "Feature: y").unwrap();
        std::fs::write(tmp.path().join("a/b/z.feature"), "Feature: z").unwrap();
        std::fs::write(tmp.path().join("not.txt"), "no").unwrap();
        let files = walk_feature_files(tmp.path(), &[]).unwrap();
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn walk_feature_files_skips_excluded_dirs() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("skip-me")).unwrap();
        std::fs::write(tmp.path().join("x.feature"), "Feature: x").unwrap();
        std::fs::write(tmp.path().join("skip-me/y.feature"), "Feature: y").unwrap();
        let files = walk_feature_files(tmp.path(), &["skip-me".to_string()]).unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn extract_all_step_texts_aggregates_across_languages() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("steps.go"),
            "func step(sc *godog.ScenarioContext) {\n  sc.Step(`^user logs in$`, login)\n}\n",
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("Steps.java"),
            "@Given(\"a user\")\nvoid step() {}\n",
        )
        .unwrap();
        let sm = extract_all_step_texts(tmp.path()).unwrap();
        assert!(sm.matches("user logs in"));
        assert!(sm.matches("a user"));
    }

    #[test]
    fn check_all_one_to_one_reports_file_gap() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");
        let app = tmp.path().join("app");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(
            specs.join("user-login.feature"),
            "Feature: x\nScenario: T\n  Given x\n",
        )
        .unwrap();
        // No test file in app.
        let opts = ScanOptions {
            repo_root: tmp.path().to_path_buf(),
            specs_dir: specs.clone(),
            specs_dirs: vec![],
            app_dir: app.clone(),
            verbose: false,
            quiet: false,
            shared_steps: false,
            exclude_dirs: vec![],
        };
        let r = check_all(&opts).unwrap();
        assert_eq!(r.gaps.len(), 1);
        assert_eq!(r.gaps[0].stem, "user-login");
    }

    #[test]
    fn check_all_shared_steps_skips_file_matching() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");
        let app = tmp.path().join("app");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(
            specs.join("foo.feature"),
            "Feature: x\nScenario: T\n  Given user logs in\n",
        )
        .unwrap();
        std::fs::write(
            app.join("steps.go"),
            "// stub\nfunc x(sc *godog.ScenarioContext) {\n  sc.Step(`^user logs in$`, fn)\n}\n",
        )
        .unwrap();
        let opts = ScanOptions {
            repo_root: tmp.path().to_path_buf(),
            specs_dir: specs.clone(),
            specs_dirs: vec![],
            app_dir: app.clone(),
            verbose: false,
            quiet: false,
            shared_steps: true,
            exclude_dirs: vec![],
        };
        let r = check_all(&opts).unwrap();
        assert_eq!(r.gaps.len(), 0); // shared_steps skips file matching
        assert_eq!(r.step_gaps.len(), 0); // step is covered
        assert_eq!(r.total_scenarios, 1);
    }
}
