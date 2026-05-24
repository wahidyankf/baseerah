// Glossary validator — port of `apps/rhino-cli/internal/glossary/`.

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::Error;
use regex::Regex;
use walkdir::WalkDir;

use crate::internal::bcregistry::{self, BcContext};
use crate::internal::severity::Severity;

#[derive(Debug, Clone, Default)]
pub struct Glossary {
    pub path: String,
    pub frontmatter: HashMap<String, String>,
    pub terms: Vec<Term>,
    pub forbidden_synonyms: Vec<Forbidden>,
    pub parse_errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Default)]
pub struct Term {
    pub term: String,
    pub definition: String,
    pub code_identifiers: Vec<String>,
    pub used_in_features: Vec<String>,
    pub source_line: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Forbidden {
    pub term: String,
    pub reason: String,
    pub source_line: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub file: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Default)]
pub struct ValidateOptions {
    pub repo_root: PathBuf,
    pub app: String,
    pub severity: Option<Severity>,
}

fn re_frontmatter() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^\*\*([^*]+)\*\*:\s*(.+)$").expect("valid hardcoded regex"))
}

fn re_backtick_idents() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"`([^`]+)`").expect("valid hardcoded regex"))
}

fn required_frontmatter_keys() -> &'static [&'static str] {
    &["Bounded context", "Maintainer", "Last reviewed"]
}

fn expected_table_columns() -> &'static [&'static str] {
    &["Term", "Code identifier(s)", "Used in features"]
}

pub fn parse(path: &Path) -> Glossary {
    let mut g = Glossary {
        path: path.to_string_lossy().into_owned(),
        ..Default::default()
    };
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            g.parse_errors.push(ParseError {
                line: 0,
                message: e.to_string(),
            });
            return g;
        }
    };
    parse_content(&mut g, &String::from_utf8_lossy(&data));
    g
}

fn parse_content(g: &mut Glossary, content: &str) {
    let mut line_num = 0usize;
    let mut in_terms = false;
    let mut header_parsed = false;
    let mut in_forbidden = false;
    let fm = re_frontmatter();

    for line in content.split('\n') {
        line_num += 1;
        if let Some(cap) = fm.captures(line) {
            g.frontmatter
                .insert(cap[1].trim().to_string(), cap[2].trim().to_string());
            continue;
        }
        if line == "## Terms" || line == "## Term index" {
            in_terms = true;
            in_forbidden = false;
            header_parsed = false;
            continue;
        }
        if line.starts_with("## Forbidden synonyms") {
            in_terms = false;
            in_forbidden = true;
            continue;
        }
        if line.starts_with("## ") {
            in_terms = false;
            in_forbidden = false;
            continue;
        }
        if in_terms && line.starts_with('|') {
            let cells = split_table_row(line);
            if !header_parsed {
                header_parsed = true;
                g.parse_errors
                    .extend(validate_table_header(&cells, line_num));
                continue;
            }
            if is_separator_row(&cells) {
                continue;
            }
            if cells.len() >= 3 {
                let t = Term {
                    term: strip_markup(&cells[0]),
                    definition: String::new(),
                    code_identifiers: parse_backtick_list(&cells[1]),
                    used_in_features: parse_feature_refs(&cells[2]),
                    source_line: line_num,
                };
                g.terms.push(t);
            }
        }
        if in_forbidden {
            let trimmed = line.trim().trim_start_matches("- ").to_string();
            if trimmed.is_empty() || trimmed == line {
                continue;
            }
            let (term, reason) = parse_forbidden_entry(&trimmed);
            if !term.is_empty() {
                g.forbidden_synonyms.push(Forbidden {
                    term,
                    reason,
                    source_line: line_num,
                });
            }
        }
    }
}

fn validate_table_header(cells: &[String], line_num: usize) -> Vec<ParseError> {
    let expected = expected_table_columns();
    if cells.len() < expected.len() {
        return vec![ParseError {
            line: line_num,
            message: "malformed terms table header: too few columns".to_string(),
        }];
    }
    for (i, exp) in expected.iter().enumerate() {
        let got = strip_markup(&cells[i]);
        if got != *exp {
            return vec![ParseError {
                line: line_num,
                message: format!("malformed terms table header: column {got} expected {exp}"),
            }];
        }
    }
    Vec::new()
}

fn split_table_row(line: &str) -> Vec<String> {
    let line = line.trim();
    let line = line.strip_prefix('|').unwrap_or(line);
    let line = line.strip_suffix('|').unwrap_or(line);
    line.split('|').map(|p| p.trim().to_string()).collect()
}

fn is_separator_row(cells: &[String]) -> bool {
    if cells.is_empty() {
        return false;
    }
    for c in cells {
        let stripped: String = c.replace('-', "");
        let s = stripped.trim();
        if !s.is_empty() && s != ":" {
            return false;
        }
    }
    true
}

fn strip_markup(s: &str) -> String {
    s.trim().replace('`', "")
}

fn parse_backtick_list(cell: &str) -> Vec<String> {
    re_backtick_idents()
        .captures_iter(cell)
        .map(|c| c[1].trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_feature_refs(cell: &str) -> Vec<String> {
    let cell = cell.replace("<br>", ",");
    cell.split(',')
        .map(|p| {
            let s = p.trim().replace('`', "");
            // Strip trailing parenthetical annotation
            if let Some(idx) = s.find('(') {
                s[..idx].trim().to_string()
            } else {
                s
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_forbidden_entry(line: &str) -> (String, String) {
    let em_dash = "—";
    let idx = line
        .find(em_dash)
        .map(|i| (i, em_dash.len()))
        .or_else(|| line.find('-').map(|i| (i, 1)));
    match idx {
        Some((i, em_len)) => {
            let term = line[..i].trim().trim_matches('"').to_string();
            let reason = line[i + em_len..].trim().to_string();
            (term, reason)
        }
        None => (line.trim().trim_matches('"').to_string(), String::new()),
    }
}

pub fn validate_all(opts: &ValidateOptions) -> Result<Vec<Finding>, Error> {
    let sev = opts.severity.unwrap_or(Severity::Error);
    let reg = bcregistry::load(&opts.repo_root, &opts.app)?;
    let mut findings: Vec<Finding> = Vec::new();
    let mut glossaries: HashMap<String, Glossary> = HashMap::new();

    for ctx in &reg.contexts {
        let glossary_path = opts.repo_root.join(&ctx.glossary);
        let g = parse(&glossary_path);
        for pe in &g.parse_errors {
            findings.push(Finding {
                file: ctx.glossary.clone(),
                message: pe.message.clone(),
                severity: sev,
            });
        }
        findings.extend(check_frontmatter(&ctx.glossary, &g, sev));
        findings.extend(check_table_header(&ctx.glossary, &g, sev));

        let code_paths: Vec<PathBuf> = ctx.code.iter().map(|c| opts.repo_root.join(c)).collect();
        let mut code_exts: Vec<String> = Vec::new();
        let lang_map = bcregistry::supported_lang_globs();
        for lang in &ctx.code_lang {
            if let Some(globs) = lang_map.get(lang.as_str()) {
                for g in *globs {
                    code_exts.push((*g).to_string());
                }
            }
        }
        let gherkin_paths: Vec<PathBuf> =
            ctx.gherkin.iter().map(|g| opts.repo_root.join(g)).collect();
        findings.extend(check_terms(
            &ctx.glossary,
            &g,
            &code_paths,
            &code_exts,
            &gherkin_paths,
            sev,
        ));
        findings.extend(check_forbidden_synonyms(
            &ctx.glossary,
            &g,
            &code_paths,
            &code_exts,
            &gherkin_paths,
            sev,
        ));
        glossaries.insert(ctx.name.clone(), g);
    }

    findings.extend(check_term_collisions(&reg, &glossaries, sev));
    findings.sort_by(|a, b| a.file.cmp(&b.file));
    Ok(findings)
}

fn check_frontmatter(file: &str, g: &Glossary, sev: Severity) -> Vec<Finding> {
    required_frontmatter_keys()
        .iter()
        .filter(|k| !g.frontmatter.contains_key(**k))
        .map(|k| Finding {
            file: file.to_string(),
            message: format!("missing frontmatter key: {k}"),
            severity: sev,
        })
        .collect()
}

fn check_table_header(file: &str, g: &Glossary, sev: Severity) -> Vec<Finding> {
    g.parse_errors
        .iter()
        .filter(|pe| pe.message.contains("malformed terms table header"))
        .map(|pe| Finding {
            file: file.to_string(),
            message: pe.message.clone(),
            severity: sev,
        })
        .collect()
}

fn check_terms(
    file: &str,
    g: &Glossary,
    code_paths: &[PathBuf],
    code_exts: &[String],
    gherkin_paths: &[PathBuf],
    sev: Severity,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for term in &g.terms {
        for id in &term.code_identifiers {
            let mut count = 0usize;
            for cp in code_paths {
                count += grep_files(id, cp, code_exts);
            }
            if count == 0 {
                findings.push(Finding {
                    file: file.to_string(),
                    message: format!(
                        "stale identifier: `{id}` (term \"{}\", not found in {})",
                        term.term,
                        format_paths(code_paths)
                    ),
                    severity: sev,
                });
            }
        }
        for r in &term.used_in_features {
            if !feature_ref_resolves(r, gherkin_paths) {
                findings.push(Finding {
                    file: file.to_string(),
                    message: format!("missing feature reference: {r}"),
                    severity: sev,
                });
            }
        }
    }
    findings
}

fn format_paths(paths: &[PathBuf]) -> String {
    let parts: Vec<String> = paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    format!("[{}]", parts.join(" "))
}

#[allow(clippy::collapsible_if, clippy::collapsible_match)]
fn feature_ref_resolves(reference: &str, gherkin_paths: &[PathBuf]) -> bool {
    for gh in gherkin_paths {
        let mut feature_path = gh.join(Path::new(reference).file_name().unwrap_or_default());
        if reference.contains('/') {
            let parts: Vec<&str> = reference.splitn(2, '/').collect();
            if parts.len() == 2 {
                if let Some(parent) = gh.parent() {
                    feature_path = parent.join(parts[0]).join(parts[1]);
                }
            }
        }
        let fp_str = feature_path.to_string_lossy();
        if fp_str.contains('*') {
            if let Ok(mut matches) = glob::glob(&fp_str) {
                if matches.next().is_some() {
                    return true;
                }
            }
        } else if feature_path.exists() {
            return true;
        }
    }
    false
}

fn check_forbidden_synonyms(
    file: &str,
    g: &Glossary,
    code_paths: &[PathBuf],
    code_exts: &[String],
    gherkin_paths: &[PathBuf],
    sev: Severity,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for fb in &g.forbidden_synonyms {
        let mut count = 0usize;
        for cp in code_paths {
            count += grep_files(&fb.term, cp, code_exts);
        }
        for gh in gherkin_paths {
            count += grep_files(&fb.term, gh, &["*.feature".to_string()]);
        }
        if count > 0 {
            findings.push(Finding {
                file: file.to_string(),
                message: format!("forbidden synonym used in own context: \"{}\"", fb.term),
                severity: sev,
            });
        }
    }
    findings
}

fn check_term_collisions(
    reg: &bcregistry::Registry,
    glossaries: &HashMap<String, Glossary>,
    sev: Severity,
) -> Vec<Finding> {
    let mut term_contexts: HashMap<String, Vec<String>> = HashMap::new();
    for ctx in &reg.contexts {
        if let Some(g) = glossaries.get(&ctx.name) {
            for t in &g.terms {
                term_contexts
                    .entry(t.term.clone())
                    .or_default()
                    .push(ctx.name.clone());
            }
        }
    }
    let mut entries: Vec<(String, Vec<String>)> = term_contexts.into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut findings = Vec::new();
    for (term, contexts) in entries {
        if contexts.len() < 2 {
            continue;
        }
        let mut all_covered = true;
        'outer: for ctx_name in &contexts {
            let Some(g) = glossaries.get(ctx_name) else {
                continue;
            };
            let others: Vec<&String> = contexts.iter().filter(|c| *c != ctx_name).collect();
            for other in others {
                if !has_forbidden_for(g, &term, other) {
                    all_covered = false;
                    break 'outer;
                }
            }
        }
        if !all_covered {
            findings.push(Finding {
                file: format!("specs/apps/{}/ddd/bounded-contexts.yaml", reg.app),
                message: format!(
                    "term collision: \"{term}\" defined in {} without mutual Forbidden-synonyms cross-link",
                    format_string_slice_owned(&contexts)
                ),
                severity: sev,
            });
        }
    }
    findings
}

fn format_string_slice_owned(s: &[String]) -> String {
    format!("[{}]", s.join(" "))
}

fn has_forbidden_for(g: &Glossary, term: &str, _other: &str) -> bool {
    g.forbidden_synonyms
        .iter()
        .any(|fb| fb.term.eq_ignore_ascii_case(term))
}

fn grep_files(pattern: &str, root: &Path, exts: &[String]) -> usize {
    let escaped = regex::escape(pattern);
    let Ok(re) = Regex::new(&format!(r"\b{escaped}\b")) else {
        return 0;
    };
    let mut count = 0usize;
    for entry in WalkDir::new(root).into_iter().flatten() {
        if entry.file_type().is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let mut matched = false;
        for ext in exts {
            let trimmed = ext.trim_start_matches("*.");
            if name.ends_with(&format!(".{trimmed}")) {
                matched = true;
                break;
            }
        }
        if !matched {
            continue;
        }
        let Ok(f) = fs::File::open(entry.path()) else {
            continue;
        };
        let reader = BufReader::new(f);
        for line in reader.lines().map_while(Result::ok) {
            if re.is_match(&line) {
                count += 1;
            }
        }
    }
    count
}

pub fn ctx_first_code_path(_ctx: &BcContext) -> &'static str {
    // unused but kept for parity
    ""
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &Path, content: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn parse_frontmatter_and_terms() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("g.md");
        write(
            &p,
            "**Bounded context**: ctx-a\n**Maintainer**: tester\n**Last reviewed**: 2026-05-23\n\n## Terms\n\n| Term | Code identifier(s) | Used in features |\n|------|--------------------|------------------|\n| Foo  | `Foo`              | feature-a.feature |\n",
        );
        let g = parse(&p);
        assert_eq!(
            g.frontmatter
                .get("Bounded context")
                .map(std::string::String::as_str),
            Some("ctx-a")
        );
        assert_eq!(g.terms.len(), 1);
        assert_eq!(g.terms[0].term, "Foo");
        assert_eq!(g.terms[0].code_identifiers, vec!["Foo"]);
    }

    #[test]
    fn parse_malformed_header_reports_error() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("g.md");
        write(
            &p,
            "## Terms\n\n| Whatever | Wrong |\n|---|---|\n| x | y |\n",
        );
        let g = parse(&p);
        assert!(
            g.parse_errors
                .iter()
                .any(|pe| pe.message.contains("malformed"))
        );
    }

    #[test]
    fn parse_forbidden_synonyms_em_dash() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("g.md");
        write(
            &p,
            "## Forbidden synonyms\n\n- \"OldTerm\" — replaced by Foo\n",
        );
        let g = parse(&p);
        assert_eq!(g.forbidden_synonyms.len(), 1);
        assert_eq!(g.forbidden_synonyms[0].term, "OldTerm");
        assert_eq!(g.forbidden_synonyms[0].reason, "replaced by Foo");
    }

    #[test]
    fn split_table_row_strips_pipes() {
        let cells = split_table_row("| a | b | c |");
        assert_eq!(cells, vec!["a", "b", "c"]);
    }

    #[test]
    fn is_separator_row_detects_dashes() {
        assert!(is_separator_row(&["---".to_string(), "---".to_string()]));
        assert!(!is_separator_row(&["a".to_string(), "---".to_string()]));
    }

    #[test]
    fn parse_backtick_list_extracts_ids() {
        assert_eq!(
            parse_backtick_list("`Foo`, `Bar`"),
            vec!["Foo".to_string(), "Bar".to_string()]
        );
    }

    #[test]
    fn parse_feature_refs_strips_parenthetical() {
        let v = parse_feature_refs("a.feature (Scenario: x), `b.feature`");
        assert_eq!(v, vec!["a.feature".to_string(), "b.feature".to_string()]);
    }

    #[test]
    fn parse_feature_refs_handles_br() {
        let v = parse_feature_refs("a.feature<br>b.feature");
        assert_eq!(v, vec!["a.feature".to_string(), "b.feature".to_string()]);
    }

    #[test]
    fn check_frontmatter_reports_missing() {
        let mut g = Glossary::default();
        let r = check_frontmatter("g.md", &g, Severity::Error);
        assert_eq!(r.len(), 3);
        g.frontmatter.insert("Bounded context".into(), "ok".into());
        let r2 = check_frontmatter("g.md", &g, Severity::Error);
        assert_eq!(r2.len(), 2);
    }

    #[test]
    fn grep_files_counts_matches() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("x.ts"), "const Foo = 1;\nFoo.bar;\n").unwrap();
        let c = grep_files("Foo", dir.path(), &["*.ts".to_string()]);
        assert_eq!(c, 2);
    }

    #[test]
    fn check_table_header_extracts_malformed() {
        let mut g = Glossary::default();
        g.parse_errors.push(ParseError {
            line: 5,
            message: "malformed terms table header: column X".into(),
        });
        let r = check_table_header("g.md", &g, Severity::Error);
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn feature_ref_resolves_simple_path() {
        let dir = tempdir().unwrap();
        let gpath = dir.path().join("gherkin");
        std::fs::create_dir_all(&gpath).unwrap();
        std::fs::write(gpath.join("a.feature"), "x").unwrap();
        assert!(feature_ref_resolves(
            "a.feature",
            std::slice::from_ref(&gpath)
        ));
        assert!(!feature_ref_resolves("missing.feature", &[gpath]));
    }

    #[test]
    fn parse_forbidden_entry_quoted_and_unquoted() {
        let (t, r) = parse_forbidden_entry("\"X\" - reason");
        assert_eq!(t, "X");
        assert_eq!(r, "reason");
        let (t2, r2) = parse_forbidden_entry("BareTerm");
        assert_eq!(t2, "BareTerm");
        assert_eq!(r2, "");
    }

    fn write_min_reg(root: &Path) {
        let yaml = "version: 2\napp: testapp\ncontexts:\n  - name: ctx-a\n    summary: ok\n    layers: [domain]\n    code: [\"apps/testapp/src\"]\n    glossary: specs/apps/testapp/glossary/ctx-a.md\n    gherkin: specs/apps/testapp/behavior/gherkin/ctx-a\n";
        let p = root.join("specs/apps/testapp/ddd/bounded-contexts.yaml");
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, yaml).unwrap();
    }

    #[test]
    fn validate_all_missing_glossary_reports() {
        let dir = tempdir().unwrap();
        write_min_reg(dir.path());
        let r = validate_all(&ValidateOptions {
            repo_root: dir.path().to_path_buf(),
            app: "testapp".to_string(),
            severity: Some(Severity::Error),
        })
        .unwrap();
        // Missing frontmatter keys + parse error for missing file
        assert!(!r.is_empty());
    }

    #[test]
    fn validate_all_clean_glossary() {
        let dir = tempdir().unwrap();
        write_min_reg(dir.path());
        std::fs::create_dir_all(dir.path().join("apps/testapp/src/domain")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/testapp/glossary")).unwrap();
        std::fs::write(
            dir.path().join("specs/apps/testapp/glossary/ctx-a.md"),
            "**Bounded context**: ctx-a\n**Maintainer**: x\n**Last reviewed**: 2026-05-23\n\n## Terms\n\n| Term | Code identifier(s) | Used in features |\n|------|--------------------|------------------|\n",
        )
        .unwrap();
        let gpath = dir.path().join("specs/apps/testapp/behavior/gherkin/ctx-a");
        std::fs::create_dir_all(&gpath).unwrap();
        std::fs::write(gpath.join("x.feature"), "x").unwrap();
        let r = validate_all(&ValidateOptions {
            repo_root: dir.path().to_path_buf(),
            app: "testapp".to_string(),
            severity: Some(Severity::Error),
        })
        .unwrap();
        assert!(r.is_empty(), "{r:#?}");
    }

    #[test]
    fn validate_all_detects_stale_identifier() {
        let dir = tempdir().unwrap();
        write_min_reg(dir.path());
        std::fs::create_dir_all(dir.path().join("apps/testapp/src/domain")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/testapp/glossary")).unwrap();
        std::fs::write(
            dir.path().join("specs/apps/testapp/glossary/ctx-a.md"),
            "**Bounded context**: ctx-a\n**Maintainer**: x\n**Last reviewed**: 2026-05-23\n\n## Terms\n\n| Term | Code identifier(s) | Used in features |\n|------|--------------------|------------------|\n| Foo  | `NonExistentSymbol` | x.feature |\n",
        )
        .unwrap();
        let gpath = dir.path().join("specs/apps/testapp/behavior/gherkin/ctx-a");
        std::fs::create_dir_all(&gpath).unwrap();
        std::fs::write(gpath.join("x.feature"), "x").unwrap();
        let r = validate_all(&ValidateOptions {
            repo_root: dir.path().to_path_buf(),
            app: "testapp".to_string(),
            severity: Some(Severity::Error),
        })
        .unwrap();
        assert!(r.iter().any(|f| f.message.contains("stale identifier")));
    }

    #[test]
    fn parse_feature_refs_empty_cell() {
        assert!(parse_feature_refs("").is_empty());
    }

    #[test]
    fn strip_markup_removes_backticks_and_trims() {
        assert_eq!(strip_markup(" `Foo` "), "Foo");
    }

    #[test]
    fn has_forbidden_for_case_insensitive() {
        let mut g = Glossary::default();
        g.forbidden_synonyms.push(Forbidden {
            term: "OldTerm".into(),
            reason: String::new(),
            source_line: 0,
        });
        assert!(has_forbidden_for(&g, "oldterm", "any"));
        assert!(!has_forbidden_for(&g, "NewTerm", "any"));
    }
}
