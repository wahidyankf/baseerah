//! Pure filter helpers for staged-file lists — no I/O.

/// Directories skipped by the staged-mermaid and staged-heading steps.
pub const STAGED_SKIP_PREFIXES: &[&str] = &[
    "plans/done",
    "apps/ayokoding-web/content",
    "apps/ose-web/content",
    "apps/rhino-cli/tests/fixtures",
];

/// Returns the subset of `staged` files that end with `.md` and whose
/// repo-relative path does not start with any of the named `skip_prefixes`.
pub fn staged_md_files<'a>(staged: &'a [String], skip_prefixes: &[&str]) -> Vec<&'a str> {
    staged
        .iter()
        .filter(|f| f.ends_with(".md"))
        .filter(|f| skip_prefixes.iter().all(|pfx| !f.starts_with(pfx)))
        .map(String::as_str)
        .collect()
}

/// Returns `true` when at least one staged file path satisfies `pred`.
pub fn has_match(staged: &[String], pred: impl Fn(&str) -> bool) -> bool {
    staged.iter().any(|f| pred(f))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn has_match_basic() {
        let s = vec![".claude/agents/x.md".to_string()];
        assert!(has_match(&s, |f| f.starts_with(".claude/")));
        assert!(!has_match(&s, |f| f.starts_with(".opencode/")));
    }

    #[test]
    fn staged_md_files_filters_by_extension_and_prefix() {
        let staged = vec![
            "docs/foo.md".to_string(),
            "plans/done/old.md".to_string(),
            "apps/ayokoding-web/content/page.md".to_string(),
            "src/main.rs".to_string(),
        ];
        let result = staged_md_files(&staged, STAGED_SKIP_PREFIXES);
        assert_eq!(result, vec!["docs/foo.md"]);
    }
}
