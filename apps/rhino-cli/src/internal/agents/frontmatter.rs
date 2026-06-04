//! Frontmatter extraction, YAML normalization, and tools parsing.
//! Ported from `apps/rhino-cli/internal/agents/converter.go` (subset).

use std::sync::OnceLock;

use regex::Regex;
use serde_norway::Value;

/// Pattern: word/hyphen colon non-whitespace — matches "name:value" without
/// space-after-colon. Replaces with "name: value".
fn yaml_colon_norm() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^([a-zA-Z0-9_-]+):([^\s])").expect("valid hardcoded regex"))
}

/// Normalize a YAML snippet by inserting a space after colons without one.
pub fn normalize_yaml(content: &[u8]) -> Vec<u8> {
    let s = String::from_utf8_lossy(content).into_owned();
    let out = yaml_colon_norm().replace_all(&s, "$1: $2");
    out.into_owned().into_bytes()
}

/// Extracts YAML frontmatter and body from markdown content. Returns
/// (frontmatter, body) with the frontmatter normalized (space-after-colon).
///
/// # Errors
///
/// Returns an error string if the content is not valid UTF-8, if the file is
/// too short to contain frontmatter, if it does not begin with `---`, or if
/// the closing `---` delimiter is not found.
pub fn extract_frontmatter(content: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let s = std::str::from_utf8(content).map_err(|e| format!("invalid UTF-8: {e}"))?;
    let lines: Vec<&str> = s.split('\n').collect();
    if lines.len() < 3 {
        return Err("file too short to contain frontmatter".to_string());
    }
    if lines[0].trim() != "---" {
        return Err("frontmatter does not start with ---".to_string());
    }
    let Some(end) = lines
        .iter()
        .enumerate()
        .skip(1)
        .find_map(|(i, line)| (line.trim() == "---").then_some(i))
    else {
        return Err("frontmatter closing --- not found".to_string());
    };
    let front = lines[1..end].join("\n");
    let body = if end + 1 < lines.len() {
        lines[end + 1..].join("\n")
    } else {
        String::new()
    };
    Ok((normalize_yaml(front.as_bytes()), body.into_bytes()))
}

/// Parses tools from Claude format (comma-separated string or YAML sequence).
pub fn parse_claude_tools(tools_raw: &Value) -> Vec<String> {
    match tools_raw {
        Value::Sequence(seq) => seq
            .iter()
            .filter_map(|v| v.as_str().map(std::string::ToString::to_string))
            .collect(),
        Value::String(s) => s
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn normalize_yaml_adds_space() {
        let out = normalize_yaml(b"name:value\ndescription: ok\n");
        assert_eq!(out, b"name: value\ndescription: ok\n");
    }

    #[test]
    fn normalize_yaml_skips_list_items() {
        let out = normalize_yaml(b"tools:\n  - Read\n  - Write\n");
        assert_eq!(out, b"tools:\n  - Read\n  - Write\n");
    }

    #[test]
    fn extract_frontmatter_simple() {
        let c = b"---\nname: foo\ndescription: bar\n---\nbody here\n";
        let (front, body) = extract_frontmatter(c).unwrap();
        assert_eq!(front, b"name: foo\ndescription: bar");
        assert_eq!(body, b"body here\n");
    }

    #[test]
    fn extract_frontmatter_no_open_marker() {
        let r = extract_frontmatter(b"name: foo\n---\nbody\n");
        assert!(r.is_err());
    }

    #[test]
    fn extract_frontmatter_no_close_marker() {
        let r = extract_frontmatter(b"---\nname: foo\nbody\nstuff\n");
        assert!(r.is_err());
    }

    #[test]
    fn extract_frontmatter_too_short() {
        let r = extract_frontmatter(b"---\n");
        assert!(r.is_err());
    }

    #[test]
    fn parse_claude_tools_from_string() {
        let v: Value = serde_norway::from_str("Read, Write, Edit").unwrap();
        let tools = parse_claude_tools(&v);
        assert_eq!(tools, vec!["Read", "Write", "Edit"]);
    }

    #[test]
    fn parse_claude_tools_from_sequence() {
        let v: Value = serde_norway::from_str("[Read, Write, Bash]").unwrap();
        let tools = parse_claude_tools(&v);
        assert_eq!(tools, vec!["Read", "Write", "Bash"]);
    }

    #[test]
    fn parse_claude_tools_empty_string() {
        let v: Value = serde_norway::from_str("\"\"").unwrap();
        let tools = parse_claude_tools(&v);
        assert!(tools.is_empty());
    }
}
