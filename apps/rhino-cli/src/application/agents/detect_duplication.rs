//! Byte-for-byte port of `apps/rhino-cli/internal/agents/detect_duplication.go`.

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use anyhow::{Context, Error};
use sha2::{Digest, Sha256};

/// Number of consecutive normalized lines used as a duplication window.
pub const DUPLICATION_WINDOW_SIZE: usize = 10;

/// A single duplication finding: same 10-line window in two or more files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicationFinding {
    /// Sorted list of absolute file paths where the duplicated window appears.
    pub files: Vec<String>,
    /// First line number (1-based) of the window in each corresponding file.
    pub start_lines: Vec<usize>,
    /// Always `DUPLICATION_WINDOW_SIZE`.
    pub window_size: usize,
    /// Severity of the finding (always `"high"`).
    pub severity: String,
    /// Human-readable description of the finding.
    pub message: String,
}

/// A file + start-line pair in the rolling window index.
#[derive(Debug, Clone)]
struct WindowRef {
    /// Absolute path of the file.
    file: String,
    /// 1-based line number where this window begins.
    start_line: usize,
}

/// Scan `.claude/agents/` and `.claude/skills/*/SKILL.md` for 10-line verbatim duplications.
///
/// # Errors
///
/// Returns an error if a file listed in the agent or skill directories cannot be read.
pub fn detect_duplication(repo_root: &Path) -> std::result::Result<Vec<DuplicationFinding>, Error> {
    let files = enumerate_agent_and_skill_files(repo_root)?;
    let mut hash_index: HashMap<String, Vec<WindowRef>> = HashMap::new();

    for path in &files {
        let raw = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
        let lines = normalize_lines(&strip_frontmatter(&raw));
        if lines.len() < DUPLICATION_WINDOW_SIZE {
            continue;
        }
        for i in 0..=lines.len() - DUPLICATION_WINDOW_SIZE {
            let window = &lines[i..i + DUPLICATION_WINDOW_SIZE];
            if is_excluded_window(window) {
                continue;
            }
            let h = hash_window(window);
            hash_index.entry(h).or_default().push(WindowRef {
                file: path.clone(),
                start_line: i + 1,
            });
        }
    }

    let mut findings = Vec::new();
    for refs in hash_index.values() {
        // Distinct file → first matching start_line
        let mut distinct_files: Vec<(String, usize)> = Vec::new();
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        for r in refs {
            if seen.insert(r.file.clone()) {
                distinct_files.push((r.file.clone(), r.start_line));
            }
        }
        if distinct_files.len() < 2 {
            continue;
        }
        distinct_files.sort_by(|a, b| a.0.cmp(&b.0));
        let paths: Vec<String> = distinct_files.iter().map(|(p, _)| p.clone()).collect();
        let starts: Vec<usize> = distinct_files.iter().map(|(_, s)| *s).collect();
        findings.push(DuplicationFinding {
            files: paths,
            start_lines: starts,
            window_size: DUPLICATION_WINDOW_SIZE,
            severity: "high".to_string(),
            message: format!(
                "{DUPLICATION_WINDOW_SIZE}-line verbatim duplication across {} files",
                distinct_files.len()
            ),
        });
    }

    findings.sort_by(|a, b| {
        if a.files[0] == b.files[0] {
            a.start_lines[0].cmp(&b.start_lines[0])
        } else {
            a.files[0].cmp(&b.files[0])
        }
    });
    Ok(findings)
}

/// Collect all agent `.md` and skill `SKILL.md` file paths under `repo_root`.
///
/// # Errors
///
/// Returns an error if `.claude/agents/` or `.claude/skills/` exists but cannot be read.
fn enumerate_agent_and_skill_files(repo_root: &Path) -> std::result::Result<Vec<String>, Error> {
    let mut files = Vec::new();

    let agents_dir = repo_root.join(".claude").join("agents");
    match fs::read_dir(&agents_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if entry.file_type().is_ok_and(|t| t.is_dir()) {
                    continue;
                }
                let name = entry.file_name().to_string_lossy().to_string();
                if !name.ends_with(".md") || name == "README.md" {
                    continue;
                }
                files.push(entry.path().to_string_lossy().to_string());
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(Error::msg(format!("read {}: {e}", agents_dir.display())));
        }
    }

    let skills_dir = repo_root.join(".claude").join("skills");
    match fs::read_dir(&skills_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if !entry.file_type().is_ok_and(|t| t.is_dir()) {
                    continue;
                }
                let skill_file = entry.path().join("SKILL.md");
                if skill_file.exists() {
                    files.push(skill_file.to_string_lossy().to_string());
                }
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(Error::msg(format!("read {}: {e}", skills_dir.display())));
        }
    }

    files.sort();
    Ok(files)
}

/// Remove the YAML frontmatter block from a markdown string, returning only the body.
fn strip_frontmatter(s: &str) -> String {
    if !s.starts_with("---\n") && !s.starts_with("---\r\n") {
        return s.to_string();
    }
    let Some(nl) = s.find('\n') else {
        return s.to_string();
    };
    let body = &s[nl + 1..];
    let Some(idx) = index_of_fence_line(body) else {
        return s.to_string();
    };
    let close_line = &body[idx..];
    let Some(close_nl) = close_line.find('\n') else {
        return String::new();
    };
    body[idx + close_nl + 1..].to_string()
}

/// Return the byte offset of the closing `---` fence line within `body`, if present.
fn index_of_fence_line(body: &str) -> Option<usize> {
    let mut offset = 0;
    while offset <= body.len() {
        let slice = &body[offset..];
        let (line, has_nl) = match slice.find('\n') {
            Some(end) => (&slice[..end], true),
            None => (slice, false),
        };
        let line = line.trim_end_matches('\r');
        if line == "---" {
            return Some(offset);
        }
        if !has_nl {
            break;
        }
        offset += line.len() + slice[line.len()..].find('\n').unwrap_or(0) + 1;
        // Simpler: advance to next newline
        // The above attempts an exact offset jump; let's just do it cleanly.
        // Reset using the slice
    }
    // Fallback: scan again with simpler logic that handles \r properly
    let mut off = 0usize;
    for raw in body.split('\n') {
        let trimmed = raw.trim_end_matches('\r');
        if trimmed == "---" {
            return Some(off);
        }
        off += raw.len() + 1;
        if off > body.len() {
            break;
        }
    }
    None
}

/// Normalize a string into lines: trim trailing whitespace, collapse consecutive blank lines.
fn normalize_lines(s: &str) -> Vec<String> {
    let s = s.replace("\r\n", "\n");
    let mut out: Vec<String> = Vec::new();
    let mut prev_blank = false;
    for line in s.split('\n') {
        let trimmed = line.trim_end_matches([' ', '\t']);
        let blank = trimmed.is_empty();
        if blank && prev_blank {
            continue;
        }
        out.push(trimmed.to_string());
        prev_blank = blank;
    }
    out
}

/// Return true if `lines` consists entirely of blank lines or heading lines — not worth hashing.
fn is_excluded_window(lines: &[String]) -> bool {
    let mut all_blank = true;
    let mut all_heading_or_blank = true;
    for l in lines {
        let t = l.trim();
        if !t.is_empty() {
            all_blank = false;
            if !t.starts_with('#') {
                all_heading_or_blank = false;
            }
        }
    }
    all_blank || all_heading_or_blank
}

/// Compute a hex-encoded SHA-256 hash of the newline-joined window for index lookup.
fn hash_window(lines: &[String]) -> String {
    let joined = lines.join("\n");
    let mut hasher = Sha256::new();
    hasher.update(joined.as_bytes());
    let sum = hasher.finalize();
    let mut out = String::with_capacity(sum.len() * 2);
    for b in sum {
        let _ = write!(out, "{b:02x}");
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::format_collect)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn strip_frontmatter_removes_yaml_block() {
        let s = "---\nname: foo\n---\nBody\n";
        assert_eq!(strip_frontmatter(s), "Body\n");
    }

    #[test]
    fn strip_frontmatter_returns_unchanged_when_no_fence() {
        let s = "# Title\n";
        assert_eq!(strip_frontmatter(s), s);
    }

    #[test]
    fn strip_frontmatter_handles_crlf() {
        let s = "---\r\nname: foo\r\n---\r\nBody\r\n";
        let out = strip_frontmatter(s);
        assert!(out.contains("Body"));
    }

    #[test]
    fn normalize_lines_collapses_blank_runs() {
        let s = "a\n\n\nb\n";
        let lines = normalize_lines(s);
        // a, blank, b, blank-from-trailing — but trailing \n produces empty + that's collapsed
        assert!(lines.iter().filter(|l| l.is_empty()).count() <= 2);
    }

    #[test]
    fn normalize_lines_strips_trailing_whitespace() {
        let lines = normalize_lines("a  \nb\t\nc");
        assert_eq!(lines[0], "a");
        assert_eq!(lines[1], "b");
        assert_eq!(lines[2], "c");
    }

    #[test]
    fn is_excluded_window_skips_blank_only() {
        let lines: Vec<String> = (0..10).map(|_| String::new()).collect();
        assert!(is_excluded_window(&lines));
    }

    #[test]
    fn is_excluded_window_skips_headings_only() {
        let lines: Vec<String> = (0..10).map(|i| format!("## H{i}")).collect();
        assert!(is_excluded_window(&lines));
    }

    #[test]
    fn is_excluded_window_includes_prose() {
        let mut lines: Vec<String> = (0..9).map(|i| format!("# H{i}")).collect();
        lines.push("body text".to_string());
        assert!(!is_excluded_window(&lines));
    }

    #[test]
    fn hash_window_deterministic() {
        let lines: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        assert_eq!(hash_window(&lines), hash_window(&lines));
    }

    #[test]
    fn detect_duplication_finds_cross_file_match() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join(".claude/agents");
        fs::create_dir_all(&agents).unwrap();
        let dup_body = "---\nname: x\n---\n".to_string()
            + &(0..15)
                .map(|i| format!("Line content {i}\n"))
                .collect::<String>();
        fs::write(agents.join("foo-maker.md"), &dup_body).unwrap();
        fs::write(agents.join("bar-maker.md"), &dup_body).unwrap();
        let findings = detect_duplication(tmp.path()).unwrap();
        assert!(!findings.is_empty());
        assert_eq!(findings[0].window_size, DUPLICATION_WINDOW_SIZE);
        assert_eq!(findings[0].severity, "high");
        assert_eq!(findings[0].files.len(), 2);
    }

    #[test]
    fn detect_duplication_skips_within_single_file() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join(".claude/agents");
        fs::create_dir_all(&agents).unwrap();
        let body = (0..30)
            .map(|i| format!("Line {}\n", i % 10))
            .collect::<String>();
        fs::write(agents.join("foo-maker.md"), body).unwrap();
        let findings = detect_duplication(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn detect_duplication_missing_dirs_is_empty() {
        let tmp = TempDir::new().unwrap();
        let findings = detect_duplication(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn enumerate_includes_skill_files() {
        let tmp = TempDir::new().unwrap();
        let skill_dir = tmp.path().join(".claude/skills/foo");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), "x").unwrap();
        let files = enumerate_agent_and_skill_files(tmp.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("SKILL.md"));
    }

    #[test]
    fn enumerate_skips_readme_md() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join(".claude/agents");
        fs::create_dir_all(&agents).unwrap();
        fs::write(agents.join("README.md"), "x").unwrap();
        fs::write(agents.join("foo-maker.md"), "x").unwrap();
        let files = enumerate_agent_and_skill_files(tmp.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("foo-maker.md"));
    }

    #[test]
    fn enumerate_skips_non_md_files() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join(".claude/agents");
        fs::create_dir_all(&agents).unwrap();
        fs::write(agents.join("foo.txt"), "x").unwrap();
        fs::write(agents.join("bar-maker.md"), "x").unwrap();
        let files = enumerate_agent_and_skill_files(tmp.path()).unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn detect_duplication_three_files_cluster() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join(".claude/agents");
        fs::create_dir_all(&agents).unwrap();
        let body = "---\nname: x\n---\n".to_string()
            + &(0..12)
                .map(|i| format!("Shared content line {i}\n"))
                .collect::<String>();
        fs::write(agents.join("foo-maker.md"), &body).unwrap();
        fs::write(agents.join("bar-maker.md"), &body).unwrap();
        fs::write(agents.join("baz-maker.md"), &body).unwrap();
        let findings = detect_duplication(tmp.path()).unwrap();
        assert!(findings.iter().any(|f| f.files.len() >= 3));
    }

    #[test]
    fn detect_duplication_sorts_by_first_file_then_start_line() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join(".claude/agents");
        fs::create_dir_all(&agents).unwrap();
        // Two distinct duplications across two file pairs.
        let body1 = "---\nname: x\n---\n".to_string()
            + &(0..12)
                .map(|i| format!("Body A line {i}\n"))
                .collect::<String>();
        let body2 = "---\nname: x\n---\n".to_string()
            + &(0..12)
                .map(|i| format!("Body B line {i}\n"))
                .collect::<String>();
        fs::write(agents.join("a-maker.md"), &body1).unwrap();
        fs::write(agents.join("b-maker.md"), &body1).unwrap();
        fs::write(agents.join("c-maker.md"), &body2).unwrap();
        fs::write(agents.join("d-maker.md"), &body2).unwrap();
        let findings = detect_duplication(tmp.path()).unwrap();
        // findings sorted by first file alphabetically
        for w in findings.windows(2) {
            assert!(w[0].files[0] <= w[1].files[0]);
        }
    }

    #[test]
    fn index_of_fence_line_finds_closing() {
        assert_eq!(index_of_fence_line("a\nb\n---\nc\n"), Some(4));
    }

    #[test]
    fn index_of_fence_line_returns_none_when_absent() {
        assert_eq!(index_of_fence_line("a\nb\nc\n"), None);
    }

    #[test]
    fn strip_frontmatter_empty_close_at_eof() {
        // Closing fence at file end with no trailing newline → returns empty
        let s = "---\nfoo: bar\n---";
        let out = strip_frontmatter(s);
        assert_eq!(out, "");
    }

    #[test]
    fn strip_frontmatter_unclosed_returns_unchanged() {
        let s = "---\nfoo: bar\nbody no close fence\n";
        assert_eq!(strip_frontmatter(s), s);
    }
}
