// Port of `stepMatcher` from `apps/rhino-cli/internal/speccoverage/checker.go`.
// Same `entries: [stepMatcherEntry]` canonical store + O(1) `exactIndex` lookup
// + legacy `exact` / `patterns` write-through views consumed by per-language
// extractors and unit tests.

use std::collections::HashMap;

use regex::Regex;

use super::cucumber_expr::{
    convert_python_parsers_expr, cucumber_expr_to_regex, has_cucumber_expressions,
    is_python_parsers_expr, unescape_cucumber_expr,
};
use super::util::normalize_ws;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatcherKind {
    Exact,
    Pattern,
}

#[derive(Debug, Clone)]
pub struct StepMatcherEntry {
    pub kind: MatcherKind,
    pub exact_text: String,   // populated when kind == Exact (ws-normalized)
    pub pattern_text: String, // raw regex source (kind == Pattern)
    pub file: String,         // origin file (absolute path; reporter resolves)
    pub(crate) compiled: Option<Regex>,
}

#[derive(Debug, Default)]
pub struct StepMatcher {
    pub(crate) entries: Vec<StepMatcherEntry>,
    pub(crate) exact_index: HashMap<String, usize>,
    /// Legacy: derived view of exact strings present in entries.
    pub(crate) exact: HashMap<String, bool>,
    /// Legacy: derived view of compiled patterns.
    pub(crate) patterns: Vec<Regex>,
}

impl StepMatcher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if `step_text` matches either an exact entry or a compiled
    /// regex pattern. O(1) exact-lookup → linear-scan over patterns (mirrors Go).
    pub fn matches(&self, step_text: &str) -> bool {
        let normalized = normalize_ws(step_text);
        if self.exact.contains_key(&normalized) {
            return true;
        }
        for re in &self.patterns {
            if re.is_match(&normalized) {
                return true;
            }
        }
        false
    }

    /// Records an exact-text step entry, normalizing whitespace.
    pub fn add_exact_with_origin(&mut self, text: &str, origin_file: &str) {
        let normalized = normalize_ws(text);
        if normalized.is_empty() {
            return;
        }
        let idx = self.entries.len();
        self.entries.push(StepMatcherEntry {
            kind: MatcherKind::Exact,
            exact_text: normalized.clone(),
            pattern_text: String::new(),
            file: origin_file.to_string(),
            compiled: None,
        });
        self.exact_index.insert(normalized.clone(), idx);
        self.exact.insert(normalized, true);
    }

    /// Records a regex-pattern entry compiled from `pattern_text`.
    pub fn add_pattern_with_origin(&mut self, re: Regex, pattern_text: &str, origin_file: &str) {
        self.entries.push(StepMatcherEntry {
            kind: MatcherKind::Pattern,
            exact_text: String::new(),
            pattern_text: pattern_text.to_string(),
            file: origin_file.to_string(),
            compiled: Some(re.clone()),
        });
        self.patterns.push(re);
    }
}

/// Generic step-text → matcher inserter.
/// - Text starting with `^` → traditional regex.
/// - Text containing `{...}` → Cucumber expression (compiled with `^…$` anchors).
/// - Otherwise → exact literal (Cucumber escapes unescaped first).
pub fn add_step_to_matcher_with_origin(sm: &mut StepMatcher, text: &str, origin_file: &str) {
    let text = normalize_ws(text);
    if text.is_empty() {
        return;
    }
    if text.starts_with('^') {
        if let Ok(re) = Regex::new(&text) {
            sm.add_pattern_with_origin(re, &text, origin_file);
        }
        return;
    }
    if has_cucumber_expressions(&text) {
        let pattern = format!("^{}$", cucumber_expr_to_regex(&text));
        if let Ok(re) = Regex::new(&pattern) {
            sm.add_pattern_with_origin(re, &text, origin_file);
        }
        return;
    }
    sm.add_exact_with_origin(&unescape_cucumber_expr(&text), origin_file);
}

/// Python-specific variant — handles `parsers.parse({name:d})` format strings before
/// falling back to the generic Cucumber path.
pub fn add_python_step_to_matcher_with_origin(sm: &mut StepMatcher, text: &str, origin_file: &str) {
    let text = normalize_ws(text);
    if text.is_empty() {
        return;
    }
    if text.starts_with('^') {
        if let Ok(re) = Regex::new(&text) {
            sm.add_pattern_with_origin(re, &text, origin_file);
        }
        return;
    }
    if is_python_parsers_expr(&text) {
        let pattern = format!("^{}$", convert_python_parsers_expr(&text));
        if let Ok(re) = Regex::new(&pattern) {
            sm.add_pattern_with_origin(re, &text, origin_file);
        }
        return;
    }
    if has_cucumber_expressions(&text) {
        let pattern = format!("^{}$", cucumber_expr_to_regex(&text));
        if let Ok(re) = Regex::new(&pattern) {
            sm.add_pattern_with_origin(re, &text, origin_file);
        }
        return;
    }
    sm.add_exact_with_origin(&text, origin_file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exact_lookup_via_matches() {
        let mut sm = StepMatcher::new();
        sm.add_exact_with_origin("user logs in", "x.rs");
        assert!(sm.matches("user logs in"));
        assert!(sm.matches("user  logs  in")); // ws normalized
        assert!(!sm.matches("user logs out"));
    }

    #[test]
    fn add_pattern_via_cucumber_expression() {
        let mut sm = StepMatcher::new();
        add_step_to_matcher_with_origin(&mut sm, "user enters {string}", "x.rs");
        assert!(sm.matches(r#"user enters "alice""#));
        assert!(!sm.matches("user enters alice"));
    }

    #[test]
    fn add_pattern_via_raw_caret_regex() {
        let mut sm = StepMatcher::new();
        add_step_to_matcher_with_origin(&mut sm, r"^count is (\d+)$", "x.rs");
        assert!(sm.matches("count is 42"));
        assert!(!sm.matches("count is forty-two"));
    }

    #[test]
    fn add_empty_text_is_skipped() {
        let mut sm = StepMatcher::new();
        sm.add_exact_with_origin("", "x.rs");
        assert!(sm.entries.is_empty());
    }

    #[test]
    fn python_parsers_d_compiles_correctly() {
        let mut sm = StepMatcher::new();
        add_python_step_to_matcher_with_origin(&mut sm, "ratio {n:d}", "x.py");
        assert!(sm.matches("ratio 42"));
        assert!(!sm.matches("ratio abc"));
    }
}
