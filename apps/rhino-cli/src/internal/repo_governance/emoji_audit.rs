// Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/emoji_audit.go`.

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{anyhow, Context, Error};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiFinding {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub codepoint: String,
    pub severity: String,
}

const EMOJI_FORBIDDEN_EXTENSIONS: &[&str] = &[
    ".json", ".yaml", ".yml", ".toml", ".go", ".ts", ".tsx", ".js", ".jsx", ".py", ".java", ".kt",
    ".rs", ".fs", ".cs", ".dart", ".exs", ".ex", ".clj",
];

fn emoji_skip_dirs() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        [
            "node_modules",
            ".git",
            ".next",
            "dist",
            "build",
            "target",
            "generated",
            "generated-contracts",
            "generated-sources",
            "generated-test-sources",
            "generated-reports",
            "archived",
            "test-results",
            "playwright-report",
            "coverage",
            ".venv",
            "__pycache__",
            ".pytest_cache",
            ".dart_tool",
            "out",
            ".cache",
            "storybook-static",
            ".playwright-mcp",
            "raw",
        ]
        .into_iter()
        .collect()
    })
}

pub fn audit_emoji(paths: &[String]) -> std::result::Result<Vec<EmojiFinding>, Error> {
    if paths.is_empty() {
        return Err(anyhow!("at least one path is required"));
    }
    let mut findings = Vec::new();
    for root in paths {
        let files = walk_emoji_paths(Path::new(root));
        for f in &files {
            let mut more = scan_emoji_file(f).with_context(|| format!("scan {}", f.display()))?;
            findings.append(&mut more);
        }
    }
    findings.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.cmp(&b.line))
            .then(a.column.cmp(&b.column))
    });
    Ok(findings)
}

fn walk_emoji_paths(root: &Path) -> Vec<std::path::PathBuf> {
    if !root.exists() {
        return Vec::new();
    }
    let mut files: Vec<std::path::PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                !emoji_skip_dirs().contains(name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| has_forbidden_emoji_extension(&e.file_name().to_string_lossy()))
        .map(|e| e.into_path())
        .collect();
    files.sort();
    files
}

fn has_forbidden_emoji_extension(name: &str) -> bool {
    let lower = name.to_lowercase();
    EMOJI_FORBIDDEN_EXTENSIONS
        .iter()
        .any(|ext| lower.ends_with(ext))
}

fn scan_emoji_file(path: &Path) -> std::result::Result<Vec<EmojiFinding>, Error> {
    let file = File::open(path)?;
    let path_s = path.to_string_lossy().to_string();
    let mut findings = Vec::new();
    for (line_idx, raw) in BufReader::new(file).lines().enumerate() {
        let line_num = line_idx + 1;
        let Ok(line) = raw else { continue };
        for (col_idx, r) in line.chars().enumerate() {
            let col = col_idx + 1;
            if is_emoji_rune(r) {
                findings.push(EmojiFinding {
                    file: path_s.clone(),
                    line: line_num,
                    column: col,
                    codepoint: format_codepoint(r),
                    severity: "high".to_string(),
                });
            }
        }
    }
    Ok(findings)
}

fn is_emoji_rune(r: char) -> bool {
    let n = r as u32;
    matches!(n,
        0x2300..=0x23FF
        | 0x2600..=0x27BF
        | 0x200D
        | 0xFE0F
        | 0x1F000..=0x1FFFF
    )
}

fn format_codepoint(r: char) -> String {
    let n = r as u32;
    if n <= 0xFFFF {
        format!("U+{n:04X}")
    } else {
        format!("U+{n:X}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn is_emoji_rune_classifies_known_blocks() {
        assert!(is_emoji_rune('✓')); // U+2713 Misc Symbols
        assert!(is_emoji_rune('⚠')); // U+26A0 Warning
        assert!(is_emoji_rune('❌')); // U+274C Cross
        assert!(is_emoji_rune('🚀')); // U+1F680
        assert!(!is_emoji_rune('a'));
        assert!(!is_emoji_rune('日')); // CJK is not emoji
    }

    #[test]
    fn format_codepoint_pads_under_ffff() {
        assert_eq!(format_codepoint('a'), "U+0061");
        assert_eq!(format_codepoint('✓'), "U+2713");
        assert_eq!(format_codepoint('🚀'), "U+1F680");
    }

    #[test]
    fn audit_emoji_finds_codepoint_in_json_file() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("conf.json");
        fs::write(&p, "{\n  \"label\": \"hi ✓ there\"\n}\n").unwrap();
        let findings = audit_emoji(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 2);
        assert_eq!(findings[0].codepoint, "U+2713");
        assert_eq!(findings[0].severity, "high");
    }

    #[test]
    fn audit_emoji_skips_non_forbidden_extensions() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("readme.md"), "# 🚀 Hello\n").unwrap();
        let findings = audit_emoji(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn audit_emoji_skips_node_modules() {
        let tmp = TempDir::new().unwrap();
        let nm = tmp.path().join("node_modules");
        fs::create_dir(&nm).unwrap();
        fs::write(nm.join("x.json"), "✓\n").unwrap();
        let findings = audit_emoji(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn audit_emoji_empty_paths_errors() {
        let err = audit_emoji(&[]).unwrap_err();
        assert!(err.to_string().contains("at least one path"));
    }

    #[test]
    fn audit_emoji_sorts_findings_by_file_line_column() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("b.json"), "✓\n").unwrap();
        fs::write(tmp.path().join("a.json"), "x\n✓\n").unwrap();
        let findings = audit_emoji(&[tmp.path().to_string_lossy().to_string()]).unwrap();
        assert_eq!(findings.len(), 2);
        assert!(findings[0].file.ends_with("a.json"));
        assert!(findings[1].file.ends_with("b.json"));
    }
}
