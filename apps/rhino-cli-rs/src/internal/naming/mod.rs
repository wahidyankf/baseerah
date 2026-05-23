// Byte-for-byte port of `apps/rhino-cli/internal/naming/naming.go`.
//
// Pure validators for agent and workflow naming conventions. Filesystem-agnostic:
// callers collect file lists (and content bytes for frontmatter checks) and pass them in.

use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub path: String,
    pub kind: String,
    pub message: String,
}

pub fn basename_sans_ext(path: &str) -> String {
    let p = Path::new(path);
    let stem = p.file_stem().map(|s| s.to_string_lossy().to_string());
    stem.unwrap_or_else(|| {
        Path::new(path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default()
    })
}

pub fn validate_suffix(path: &str, allowed_suffixes: &[&str], kind: &str) -> Option<Violation> {
    let name = basename_sans_ext(path);
    for suffix in allowed_suffixes {
        if name == *suffix {
            // Bare suffix (e.g. "maker.md") has no scope and is invalid.
            continue;
        }
        if name.ends_with(&format!("-{suffix}")) {
            return None;
        }
    }
    Some(Violation {
        path: path.to_string(),
        kind: kind.to_string(),
        message: format!(
            "filename \"{name}\" does not end with any allowed suffix ({})",
            allowed_suffixes.join(", ")
        ),
    })
}

pub fn extract_frontmatter_name(content: &[u8]) -> String {
    let text = String::from_utf8_lossy(content);
    if !text.starts_with("---\n") && !text.starts_with("---\r\n") {
        return String::new();
    }
    let rest = &text[4..];
    let end = match rest.find("\n---") {
        Some(i) => i,
        None => return String::new(),
    };
    let frontmatter = &rest[..end];
    for line in frontmatter.split('\n') {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("name:") {
            let value = rest.trim();
            let value = value.trim_matches(|c| c == '"' || c == '\'');
            return value.to_string();
        }
    }
    String::new()
}

pub fn validate_frontmatter_name(path: &str, content: &[u8]) -> Option<Violation> {
    let name = extract_frontmatter_name(content);
    if name.is_empty() {
        return None;
    }
    let expected = basename_sans_ext(path);
    if name == expected {
        return None;
    }
    Some(Violation {
        path: path.to_string(),
        kind: "frontmatter-mismatch".to_string(),
        message: format!("frontmatter name \"{name}\" does not match filename \"{expected}\""),
    })
}

pub fn validate_mirror(claude_files: &[String], opencode_files: &[String]) -> Vec<Violation> {
    use std::collections::HashMap;
    let mut claude_set: HashMap<String, String> = HashMap::new();
    for p in claude_files {
        claude_set.insert(basename_sans_ext(p), p.clone());
    }
    let mut opencode_set: HashMap<String, String> = HashMap::new();
    for p in opencode_files {
        opencode_set.insert(basename_sans_ext(p), p.clone());
    }
    let mut violations = Vec::new();
    for (name, path) in &claude_set {
        if !opencode_set.contains_key(name) {
            violations.push(Violation {
                path: path.clone(),
                kind: "mirror-drift".to_string(),
                message: format!(
                    "{}.md exists in .claude/agents/ but not in .opencode/agents/",
                    name
                ),
            });
        }
    }
    for (name, path) in &opencode_set {
        if !claude_set.contains_key(name) {
            violations.push(Violation {
                path: path.clone(),
                kind: "mirror-drift".to_string(),
                message: format!(
                    "{}.md exists in .opencode/agents/ but not in .claude/agents/",
                    name
                ),
            });
        }
    }
    violations.sort_by(|a, b| a.path.cmp(&b.path));
    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basename_strips_md_extension() {
        assert_eq!(basename_sans_ext("a/b/foo.md"), "foo");
        assert_eq!(basename_sans_ext("foo.md"), "foo");
        assert_eq!(basename_sans_ext("foo"), "foo");
    }

    #[test]
    fn validate_suffix_accepts_matching_role() {
        assert!(
            validate_suffix("apps-foo-maker.md", &["maker", "checker"], "role-suffix").is_none()
        );
        assert!(validate_suffix("foo-checker.md", &["maker", "checker"], "role-suffix").is_none());
    }

    #[test]
    fn validate_suffix_rejects_mismatch() {
        let v = validate_suffix("foo-bar.md", &["maker", "checker"], "role-suffix").unwrap();
        assert_eq!(v.kind, "role-suffix");
        assert!(v.message.contains("does not end with"));
    }

    #[test]
    fn validate_suffix_rejects_bare_suffix() {
        let v = validate_suffix("maker.md", &["maker"], "role-suffix");
        assert!(v.is_some());
    }

    #[test]
    fn validate_suffix_matches_multi_word() {
        assert!(
            validate_suffix("plan-quality-gate.md", &["quality-gate"], "type-suffix").is_none()
        );
    }

    #[test]
    fn extract_frontmatter_returns_name() {
        let content = b"---\nname: foo-bar\ndescription: D\n---\n# Body";
        assert_eq!(extract_frontmatter_name(content), "foo-bar");
    }

    #[test]
    fn extract_frontmatter_strips_quotes() {
        let content = b"---\nname: \"foo-bar\"\n---\n";
        assert_eq!(extract_frontmatter_name(content), "foo-bar");
    }

    #[test]
    fn extract_frontmatter_no_block_returns_empty() {
        assert_eq!(extract_frontmatter_name(b"# Title\n"), "");
    }

    #[test]
    fn validate_frontmatter_name_matches() {
        assert!(validate_frontmatter_name("foo.md", b"---\nname: foo\n---\n").is_none());
    }

    #[test]
    fn validate_frontmatter_name_mismatch() {
        let v = validate_frontmatter_name("foo.md", b"---\nname: bar\n---\n").unwrap();
        assert_eq!(v.kind, "frontmatter-mismatch");
    }

    #[test]
    fn validate_mirror_finds_missing_pairs() {
        let claude = vec![".claude/agents/foo.md".to_string()];
        let opencode = vec![".opencode/agents/bar.md".to_string()];
        let vs = validate_mirror(&claude, &opencode);
        assert_eq!(vs.len(), 2);
    }

    #[test]
    fn validate_mirror_clean_when_pairs_match() {
        let claude = vec![".claude/agents/foo.md".to_string()];
        let opencode = vec![".opencode/agents/foo.md".to_string()];
        let vs = validate_mirror(&claude, &opencode);
        assert!(vs.is_empty());
    }
}
