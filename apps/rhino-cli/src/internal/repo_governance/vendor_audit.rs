// Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/governance_vendor_audit.go`.

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Error};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub path: String,
    pub line: usize,
    pub r#match: String,
    pub replacement: String,
}

const FORBIDDEN_CONVENTION_SUFFIX: &str =
    "repo-governance/conventions/structure/governance-vendor-independence.md";

struct ForbiddenTerm {
    re: Regex,
    display_term: &'static str,
    replacement: &'static str,
}

fn forbidden_terms() -> &'static Vec<ForbiddenTerm> {
    static TERMS: OnceLock<Vec<ForbiddenTerm>> = OnceLock::new();
    TERMS.get_or_init(|| {
        vec![
            mk(r"Claude Code", "Claude Code", "\"the coding agent\""),
            mk(
                r"OpenCode",
                "OpenCode",
                "\"the coding agent\" or drop where redundant",
            ),
            mk(
                r"\bCursor\b",
                "Cursor",
                "\"the coding agent\" or \"AI coding editor\"",
            ),
            mk(
                r"\bWindsurf\b",
                "Windsurf",
                "\"the coding agent\" or \"AI coding editor\"",
            ),
            mk(
                r"\bCodeium\b",
                "Codeium",
                "\"the coding agent\" (legacy Windsurf brand)",
            ),
            mk(
                r"\bCopilot\b",
                "Copilot",
                "\"the coding agent\" or \"AI coding assistant\"",
            ),
            mk(
                r"\bAider\b",
                "Aider",
                "\"the coding agent\" or \"AI coding assistant\"",
            ),
            mk(
                r"\bCline\b",
                "Cline",
                "\"the coding agent\" or \"AI coding assistant\"",
            ),
            mk(
                r"\bDevin\b",
                "Devin",
                "\"the coding agent\" (false-positive risk: personal name; review context)",
            ),
            mk(r"\.claude/", ".claude/", "\"primary binding directory\""),
            mk(
                r"\.opencode/",
                ".opencode/",
                "\"secondary binding directory\"",
            ),
            mk(
                r"\.cursor/",
                ".cursor/",
                "\"the platform binding directory\"",
            ),
            mk(
                r"\.windsurf/",
                ".windsurf/",
                "\"the platform binding directory\"",
            ),
            mk(
                r"\.continue/",
                ".continue/",
                "\"the platform binding directory\"",
            ),
            mk(
                r"\.clinerules/",
                ".clinerules/",
                "\"the platform binding directory\"",
            ),
            mk(r"Anthropic", "Anthropic", "\"the model vendor\" or drop"),
            mk(r"\bOpenAI\b", "OpenAI", "\"the model vendor\" or drop"),
            mk(r"\bxAI\b", "xAI", "\"the model vendor\" or drop"),
            mk(r"\bSonnet\b", "Sonnet", "\"execution-grade\""),
            mk(r"\bOpus\b", "Opus", "\"planning-grade\""),
            mk(r"\bHaiku\b", "Haiku", "\"fast\""),
            mk(r"\bGPT\b", "GPT", "\"AI model\" or capability tier"),
            mk(r"\bGemini\b", "Gemini", "\"AI model\" or capability tier"),
            mk(
                r"\bDeepSeek\b",
                "DeepSeek",
                "\"AI model\" or capability tier",
            ),
            mk(r"\bQwen\b", "Qwen", "\"AI model\" or capability tier"),
            mk(r"\bLlama\b", "Llama", "\"AI model\" or capability tier"),
            mk(r"\bMistral\b", "Mistral", "\"AI model\" or capability tier"),
            mk(
                r"\bGrok\b",
                "Grok",
                "\"AI model\" (false-positive risk: verb \"to grok\"; review context)",
            ),
            mk(r"\bSkills\b", "Skills", "\"agent skills\" (lowercase)"),
        ]
    })
}

fn mk(pattern: &str, term: &'static str, replacement: &'static str) -> ForbiddenTerm {
    ForbiddenTerm {
        re: Regex::new(pattern).unwrap(),
        display_term: term,
        replacement,
    }
}

fn html_comment_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"<!--.*?-->").unwrap())
}

fn inline_code_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"`[^`]*`").unwrap())
}

fn link_url_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\[([^\]]*)\]\([^)]*\)").unwrap())
}

pub fn scan_file(path: &Path) -> std::result::Result<Vec<Finding>, Error> {
    let data = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    Ok(scan_lines(&path.to_string_lossy(), &data))
}

pub fn walk(root: &Path) -> std::result::Result<Vec<Finding>, Error> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut findings = Vec::new();
    for entry in WalkDir::new(root).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".md") {
            continue;
        }
        let p = entry.path();
        let p_slash = p.to_string_lossy().replace('\\', "/");
        if p_slash.ends_with(FORBIDDEN_CONVENTION_SUFFIX) {
            continue;
        }
        findings.extend(scan_file(p)?);
    }
    Ok(findings)
}

fn scan_lines(path: &str, content: &str) -> Vec<Finding> {
    let lines: Vec<&str> = content.split('\n').collect();
    let mut findings = Vec::new();

    let mut in_code_fence_len: usize = 0;
    let mut in_frontmatter = false;
    let mut in_html_comment = false;
    let mut in_platform_binding_section = false;
    let mut platform_binding_heading_level: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let line_num = i + 1;

        // YAML frontmatter.
        if line_num == 1 && line.trim() == "---" {
            in_frontmatter = true;
            continue;
        }
        if in_frontmatter {
            if line.trim() == "---" {
                in_frontmatter = false;
            }
            continue;
        }

        // Multi-line HTML comment.
        if in_html_comment {
            if line.contains("-->") {
                in_html_comment = false;
            }
            continue;
        }
        if line.contains("<!--") && !line.contains("-->") {
            in_html_comment = true;
            if let Some(idx) = line.find("<!--") {
                let before = &line[..idx];
                let stripped = strip_non_prose(before);
                if !stripped.is_empty() {
                    for ft in forbidden_terms() {
                        if ft.re.is_match(&stripped) {
                            findings.push(Finding {
                                path: path.to_string(),
                                line: line_num,
                                r#match: ft.display_term.to_string(),
                                replacement: ft.replacement.to_string(),
                            });
                        }
                    }
                }
            }
            continue;
        }

        // Code fences (length-aware per CommonMark).
        let fl = fence_line_len(line);
        if fl > 0 {
            if in_code_fence_len == 0 {
                in_code_fence_len = fl;
                continue;
            } else if fl >= in_code_fence_len {
                in_code_fence_len = 0;
                continue;
            }
            // Inner fence line (shorter than opener) — falls through.
        }

        if in_code_fence_len > 0 {
            continue;
        }

        // Platform Binding Examples heading scope.
        if let Some(level) = parse_heading(line) {
            if is_platform_binding_heading(line) {
                in_platform_binding_section = true;
                platform_binding_heading_level = level;
                continue;
            }
            if in_platform_binding_section && level <= platform_binding_heading_level {
                in_platform_binding_section = false;
                platform_binding_heading_level = 0;
            }
        }

        if in_platform_binding_section {
            continue;
        }

        // Scan for forbidden terms.
        let stripped = strip_non_prose(line);
        for ft in forbidden_terms() {
            if ft.re.is_match(&stripped) {
                findings.push(Finding {
                    path: path.to_string(),
                    line: line_num,
                    r#match: ft.display_term.to_string(),
                    replacement: ft.replacement.to_string(),
                });
            }
        }
    }
    findings
}

fn fence_line_len(line: &str) -> usize {
    let trimmed = line.trim();
    let mut n = 0;
    for ch in trimmed.chars() {
        if ch == '`' {
            n += 1;
        } else {
            break;
        }
    }
    if n >= 3 {
        n
    } else {
        0
    }
}

fn strip_non_prose(line: &str) -> String {
    let s = html_comment_re().replace_all(line, "");
    let s = inline_code_re().replace_all(&s, "``");
    let s = link_url_re().replace_all(&s, "[$1]");
    s.into_owned()
}

fn parse_heading(line: &str) -> Option<usize> {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return None;
    }
    let mut level = 0;
    for ch in trimmed.chars() {
        if ch == '#' {
            level += 1;
        } else {
            break;
        }
    }
    if level > 6 {
        return None;
    }
    let bytes = trimmed.as_bytes();
    if bytes.len() <= level || bytes[level] != b' ' {
        return None;
    }
    Some(level)
}

fn is_platform_binding_heading(line: &str) -> bool {
    line.to_lowercase().contains("platform binding examples")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn detects_forbidden_brand_in_prose() {
        let findings = scan_lines("x.md", "I use Claude Code.\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].r#match, "Claude Code");
    }

    #[test]
    fn skips_code_fences() {
        let findings = scan_lines("x.md", "```\nClaude Code\n```\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_nested_quad_fences() {
        let findings = scan_lines("x.md", "````md\n```\nClaude Code\n```\n````\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_frontmatter() {
        let findings = scan_lines("x.md", "---\ntitle: Claude Code\n---\n\nBody.\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_html_comments() {
        let findings = scan_lines("x.md", "<!-- Claude Code -->\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_multiline_html_comments() {
        let findings = scan_lines("x.md", "<!--\nClaude Code\n-->\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_inline_code_spans() {
        let findings = scan_lines("x.md", "Use `Claude Code` here.\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_link_urls() {
        let findings = scan_lines("x.md", "[link](https://example.com/Claude-Code/foo)\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_platform_binding_section() {
        let content = "# X\n\n## Platform Binding Examples\n\nClaude Code\n\n## Next\n";
        let findings = scan_lines("x.md", content);
        assert!(findings.is_empty());
    }

    #[test]
    fn platform_binding_scope_ends_at_same_level_heading() {
        // line 1: ## Platform Binding Examples
        // line 2: (blank)
        // line 3: Claude Code (in scope)
        // line 4: ## Other
        // line 5: (blank)
        // line 6: Claude Code (out of scope)
        let content = "## Platform Binding Examples\n\nClaude Code\n## Other\n\nClaude Code\n";
        let findings = scan_lines("x.md", content);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 6);
    }

    #[test]
    fn detects_binding_dir_path() {
        let findings = scan_lines(
            "x.md",
            "Edit `.claude/agents/` is wrong, edit .claude/agents/ instead.\n",
        );
        // The `.claude/` regex matches in stripped prose (inline code already removed).
        assert!(findings.iter().any(|f| f.r#match == ".claude/"));
    }

    #[test]
    fn skip_convention_definition_file() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join(FORBIDDEN_CONVENTION_SUFFIX);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, "Claude Code\n").unwrap();
        let findings = walk(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn parse_heading_recognises_atx_levels() {
        assert_eq!(parse_heading("## Foo"), Some(2));
        assert_eq!(parse_heading("### Bar"), Some(3));
        assert_eq!(parse_heading("####### Too deep"), None);
        assert_eq!(parse_heading("##NoSpace"), None);
        assert_eq!(parse_heading("Foo"), None);
    }

    #[test]
    fn fence_line_len_counts_backticks() {
        assert_eq!(fence_line_len("```"), 3);
        assert_eq!(fence_line_len("```js"), 3);
        assert_eq!(fence_line_len("````"), 4);
        assert_eq!(fence_line_len("``"), 0);
        assert_eq!(fence_line_len("text"), 0);
    }
}
