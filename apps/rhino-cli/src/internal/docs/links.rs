// Byte-for-byte port of `apps/rhino-cli/internal/docs/links_*.go`.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Instant;

use anyhow::{Context, Error};
use chrono::Local;
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrokenLink {
    pub line_number: usize,
    pub source_file: String,
    pub link_text: String,
    pub target_path: String,
    pub category: String,
}

pub struct LinkValidationResult {
    pub total_files: usize,
    pub total_links: usize,
    pub broken_links: Vec<BrokenLink>,
    pub broken_by_category: HashMap<String, Vec<BrokenLink>>,
    pub scan_duration_ms: i64,
}

pub struct ScanOptions {
    pub repo_root: PathBuf,
    pub staged_only: bool,
    pub skip_paths: Vec<String>,
}

#[derive(Debug, Clone)]
struct LinkInfo {
    line_number: usize,
    url: String,
}

fn link_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap())
}

fn bracket_placeholder_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\[[\w-]+\]").unwrap())
}

pub fn validate_all_links(opts: &ScanOptions) -> std::result::Result<LinkValidationResult, Error> {
    let start = Instant::now();
    let files = get_markdown_files(opts)?;
    let mut result = LinkValidationResult {
        total_files: files.len(),
        total_links: 0,
        broken_links: Vec::new(),
        broken_by_category: HashMap::new(),
        scan_duration_ms: 0,
    };

    for path in &files {
        let links = match extract_links(path) {
            Ok(l) => l,
            Err(_) => continue,
        };
        result.total_links += links.len();

        let broken = match validate_file(path, opts, &links) {
            Ok(b) => b,
            Err(_) => continue,
        };
        for b in broken {
            result
                .broken_by_category
                .entry(b.category.clone())
                .or_default()
                .push(b.clone());
            result.broken_links.push(b);
        }
    }
    result.scan_duration_ms = start.elapsed().as_millis() as i64;
    Ok(result)
}

fn get_markdown_files(opts: &ScanOptions) -> std::result::Result<Vec<PathBuf>, Error> {
    let files = if opts.staged_only {
        get_staged_markdown_files(&opts.repo_root)?
    } else {
        get_all_markdown_files(&opts.repo_root)?
    };
    Ok(filter_skip_paths(files, &opts.repo_root, &opts.skip_paths))
}

fn get_staged_markdown_files(repo_root: &Path) -> std::result::Result<Vec<PathBuf>, Error> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .current_dir(repo_root)
        .output()
        .context("git diff --cached")?;
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text
        .trim()
        .split('\n')
        .filter(|l| !l.is_empty() && l.ends_with(".md"))
        .map(|l| repo_root.join(l))
        .collect())
}

fn get_all_markdown_files(repo_root: &Path) -> std::result::Result<Vec<PathBuf>, Error> {
    let dirs = ["repo-governance", "docs", ".claude"];
    let mut files = Vec::new();
    for dir in &dirs {
        let dir_path = repo_root.join(dir);
        if !dir_path.exists() {
            continue;
        }
        for entry in WalkDir::new(&dir_path).into_iter().flatten() {
            if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "md") {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    // Root-level *.md
    if let Ok(entries) = fs::read_dir(repo_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|e| e == "md") {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn filter_skip_paths(files: Vec<PathBuf>, repo_root: &Path, skip_paths: &[String]) -> Vec<PathBuf> {
    if skip_paths.is_empty() {
        return files;
    }
    files
        .into_iter()
        .filter(|f| {
            let rel = match f.strip_prefix(repo_root) {
                Ok(r) => r.to_string_lossy().to_string(),
                Err(_) => return true,
            };
            for skip in skip_paths {
                if rel.starts_with(skip) {
                    return false;
                }
            }
            true
        })
        .collect()
}

fn extract_links(path: &Path) -> std::result::Result<Vec<LinkInfo>, Error> {
    let data = fs::read_to_string(path)?;
    let mut links = Vec::new();
    let mut in_code_block = false;
    for (i, line) in data.split('\n').enumerate() {
        let line_num = i + 1;
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }
        for cap in link_re().captures_iter(line) {
            let mut url = cap[2].to_string();
            url = url
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_string();
            if url.starts_with("http://")
                || url.starts_with("https://")
                || url.starts_with('#')
                || url.starts_with("mailto:")
            {
                continue;
            }
            if should_skip_link(&url) {
                continue;
            }
            links.push(LinkInfo {
                line_number: line_num,
                url,
            });
        }
    }
    Ok(links)
}

pub fn should_skip_link(link: &str) -> bool {
    if link.starts_with('/') {
        return true;
    }
    if link.contains("{{<") || link.contains("{{%") {
        return true;
    }
    let placeholders = [
        "path.md",
        "target",
        "link",
        "./path/to/",
        "../path/to/",
        "path/to/convention.md",
        "path/to/practice.md",
        "path/to/rule.md",
        "./relative/path/to/",
    ];
    for p in &placeholders {
        if link.contains(p) {
            return true;
        }
    }
    if bracket_placeholder_re().is_match(link) {
        return true;
    }
    if link == "path" || link == "target" || link == "link" {
        return true;
    }
    if link.contains("/images/") && !link.starts_with("../") {
        return true;
    }
    let example_patterns = [
        "./overview",
        "./guide.md",
        "./examples.md",
        "./reference.md",
        "./diagram.png",
        "./image.png",
        "./screenshots/",
        "./auth-guide.md",
        "by-concept/beginner",
        "./by-example/beginner",
        "swe/prog-lang/",
        "../parent",
        "./ai/",
        "../swe/",
        "../../advanced/",
        "url",
        "./LICENSE",
        "../../features.md",
        "../../.opencode/",
    ];
    for p in &example_patterns {
        if link.contains(p) {
            return true;
        }
    }
    false
}

fn validate_file(
    file_path: &Path,
    opts: &ScanOptions,
    links: &[LinkInfo],
) -> std::result::Result<Vec<BrokenLink>, Error> {
    // Skill files: skip validation
    let p_str = file_path.to_string_lossy();
    if p_str.contains(".claude/skills/") {
        return Ok(Vec::new());
    }
    let mut broken = Vec::new();
    for link in links {
        let target = resolve_link(file_path, &link.url);
        if !target.exists() {
            let rel = file_path
                .strip_prefix(&opts.repo_root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| file_path.to_string_lossy().to_string());
            let category = categorize_broken_link(&link.url);
            broken.push(BrokenLink {
                line_number: link.line_number,
                source_file: rel,
                link_text: link.url.clone(),
                target_path: target.to_string_lossy().to_string(),
                category,
            });
        }
    }
    Ok(broken)
}

fn resolve_link(source_file: &Path, link: &str) -> PathBuf {
    let without_anchor = link.split('#').next().unwrap_or("");
    if without_anchor.is_empty() {
        return source_file.to_path_buf();
    }
    let parent = source_file.parent().unwrap_or(Path::new(""));
    let joined = parent.join(without_anchor);
    // filepath.Clean equivalent: normalize . and ..
    clean_path(&joined)
}

fn clean_path(p: &Path) -> PathBuf {
    let mut out = Vec::new();
    let mut is_abs = false;
    for comp in p.components() {
        use std::path::Component;
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                if !matches!(out.last(), Some(s) if s != ".." && s != "/") {
                    out.push("..".to_string());
                } else {
                    out.pop();
                }
            }
            Component::Normal(s) => out.push(s.to_string_lossy().to_string()),
            Component::RootDir => {
                is_abs = true;
                out.clear();
            }
            Component::Prefix(_) => {}
        }
    }
    let mut result = PathBuf::new();
    if is_abs {
        result.push("/");
    }
    for c in out {
        result.push(c);
    }
    if result.as_os_str().is_empty() {
        result.push(".");
    }
    result
}

pub fn categorize_broken_link(link: &str) -> String {
    if link.contains("workflows/") && !link.contains("repo-governance/workflows/") {
        return "workflows/ paths".to_string();
    }
    if link.contains("vision/") && !link.contains("repo-governance/vision/") {
        return "vision/ paths".to_string();
    }
    if link.contains("conventions/README.md") {
        return "conventions README".to_string();
    }
    if link == "CODE_OF_CONDUCT.md" || link == "CHANGELOG.md" {
        return "Missing files".to_string();
    }
    "General/other paths".to_string()
}

pub fn format_link_text(result: &LinkValidationResult, _verbose: bool, quiet: bool) -> String {
    let mut output = String::new();
    if result.broken_links.is_empty() {
        if !quiet {
            output.push_str("✓ All links valid! No broken links found.\n");
        }
        return output;
    }
    output.push_str("# Broken Links Report\n\n");
    let _ = writeln!(
        output,
        "**Total broken links**: {}",
        result.broken_links.len()
    );

    let category_order = [
        "Legacy prefixed paths",
        "Missing files",
        "General/other paths",
        "workflows/ paths",
        "vision/ paths",
        "conventions README",
    ];

    for category in &category_order {
        let links = match result.broken_by_category.get(*category) {
            Some(l) if !l.is_empty() => l,
            _ => continue,
        };
        let _ = write!(output, "\n## {category} ({} links)\n", links.len());

        let mut by_file: HashMap<String, Vec<BrokenLink>> = HashMap::new();
        for link in links {
            by_file
                .entry(link.source_file.clone())
                .or_default()
                .push(link.clone());
        }
        let mut files: Vec<String> = by_file.keys().cloned().collect();
        files.sort();
        for file in files {
            let _ = write!(output, "\n### {file}\n\n");
            let mut file_links = by_file.remove(&file).unwrap_or_default();
            file_links.sort_by_key(|l| l.line_number);
            for link in file_links {
                let _ = writeln!(output, "- Line {}: `{}`", link.line_number, link.link_text);
            }
        }
    }
    output
}

pub fn format_link_json(result: &LinkValidationResult) -> std::result::Result<String, Error> {
    use serde::Serialize;

    #[derive(Serialize)]
    struct JsonBrokenLink<'a> {
        source_file: &'a str,
        line_number: usize,
        link_text: &'a str,
        target_path: &'a str,
    }

    #[derive(Serialize)]
    struct JsonOutput<'a> {
        status: &'a str,
        timestamp: String,
        total_files: usize,
        total_links: usize,
        broken_count: usize,
        duration_ms: i64,
        categories: HashMap<&'a str, Vec<JsonBrokenLink<'a>>>,
    }

    let status = if result.broken_links.is_empty() {
        "success"
    } else {
        "failure"
    };
    let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    let mut categories: HashMap<&str, Vec<JsonBrokenLink>> = HashMap::new();
    for (cat, links) in &result.broken_by_category {
        let jl: Vec<JsonBrokenLink> = links
            .iter()
            .map(|l| JsonBrokenLink {
                source_file: &l.source_file,
                line_number: l.line_number,
                link_text: &l.link_text,
                target_path: &l.target_path,
            })
            .collect();
        categories.insert(cat.as_str(), jl);
    }
    let out = JsonOutput {
        status,
        timestamp,
        total_files: result.total_files,
        total_links: result.total_links,
        broken_count: result.broken_links.len(),
        duration_ms: result.scan_duration_ms,
        categories,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

pub fn format_link_markdown(result: &LinkValidationResult) -> String {
    format_link_text(result, false, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn skip_link_recognises_placeholders() {
        assert!(should_skip_link("/absolute"));
        assert!(should_skip_link("path"));
        assert!(should_skip_link("target"));
        assert!(should_skip_link("link"));
        assert!(should_skip_link("./relative/path/to/foo"));
        assert!(should_skip_link("[placeholder]-link"));
        assert!(should_skip_link("./images/foo.png"));
        assert!(!should_skip_link("real.md"));
    }

    #[test]
    fn categorize_returns_categories() {
        assert_eq!(
            categorize_broken_link("docs/workflows/foo.md"),
            "workflows/ paths"
        );
        assert_eq!(
            categorize_broken_link("docs/vision/foo.md"),
            "vision/ paths"
        );
        assert_eq!(
            categorize_broken_link("conventions/README.md"),
            "conventions README"
        );
        assert_eq!(
            categorize_broken_link("CODE_OF_CONDUCT.md"),
            "Missing files"
        );
        assert_eq!(categorize_broken_link("foo.md"), "General/other paths");
    }

    #[test]
    fn extract_links_finds_valid_links() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("a.md");
        fs::write(&p, "See [link](foo.md) and [code](`bar.md`)\n").unwrap();
        let links = extract_links(&p).unwrap();
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn extract_links_skips_inside_fence() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("a.md");
        fs::write(&p, "```\n[in fence](x.md)\n```\n[outside](y.md)\n").unwrap();
        let links = extract_links(&p).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "y.md");
    }

    #[test]
    fn extract_links_skips_external_urls() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("a.md");
        fs::write(
            &p,
            "[a](https://example.com) [b](http://x.io) [c](#anchor) [d](mailto:x@y) [e](real.md)\n",
        )
        .unwrap();
        let links = extract_links(&p).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "real.md");
    }

    #[test]
    fn validate_all_links_returns_broken() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/a.md"), "[bad](nonexistent.md)\n").unwrap();
        let opts = ScanOptions {
            repo_root: tmp.path().to_path_buf(),
            staged_only: false,
            skip_paths: Vec::new(),
        };
        let result = validate_all_links(&opts).unwrap();
        assert_eq!(result.total_files, 1);
        assert!(!result.broken_links.is_empty());
    }

    #[test]
    fn format_link_text_succeeds_with_no_broken() {
        let result = LinkValidationResult {
            total_files: 5,
            total_links: 20,
            broken_links: Vec::new(),
            broken_by_category: HashMap::new(),
            scan_duration_ms: 100,
        };
        let s = format_link_text(&result, false, false);
        assert!(s.contains("All links valid"));
    }

    #[test]
    fn format_link_text_quiet_is_empty_when_clean() {
        let result = LinkValidationResult {
            total_files: 5,
            total_links: 20,
            broken_links: Vec::new(),
            broken_by_category: HashMap::new(),
            scan_duration_ms: 100,
        };
        let s = format_link_text(&result, false, true);
        assert!(s.is_empty());
    }

    fn broken_link() -> BrokenLink {
        BrokenLink {
            line_number: 5,
            source_file: "docs/foo.md".to_string(),
            link_text: "nonexistent.md".to_string(),
            target_path: "docs/nonexistent.md".to_string(),
            category: "General/other paths".to_string(),
        }
    }

    fn result_with_broken() -> LinkValidationResult {
        let mut by_cat = HashMap::new();
        by_cat.insert("General/other paths".to_string(), vec![broken_link()]);
        LinkValidationResult {
            total_files: 1,
            total_links: 1,
            broken_links: vec![broken_link()],
            broken_by_category: by_cat,
            scan_duration_ms: 50,
        }
    }

    #[test]
    fn format_link_text_with_broken_renders_report() {
        let s = format_link_text(&result_with_broken(), false, false);
        assert!(s.contains("# Broken Links Report"));
        assert!(s.contains("**Total broken links**: 1"));
        assert!(s.contains("General/other paths"));
        assert!(s.contains("docs/foo.md"));
        assert!(s.contains("nonexistent.md"));
    }

    #[test]
    fn format_link_markdown_delegates_to_text() {
        let s = format_link_markdown(&result_with_broken());
        assert!(s.contains("Broken Links Report"));
    }

    #[test]
    fn format_link_json_with_broken_status_failure() {
        let s = format_link_json(&result_with_broken()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["broken_count"], 1);
        assert!(v["categories"]["General/other paths"].is_array());
    }

    #[test]
    fn validate_file_skips_skill_files() {
        let tmp = TempDir::new().unwrap();
        let skill_dir = tmp.path().join(".claude/skills/foo");
        fs::create_dir_all(&skill_dir).unwrap();
        let p = skill_dir.join("SKILL.md");
        fs::write(&p, "[bad](nonexistent.md)\n").unwrap();
        let opts = ScanOptions {
            repo_root: tmp.path().to_path_buf(),
            staged_only: false,
            skip_paths: Vec::new(),
        };
        let links = extract_links(&p).unwrap();
        let broken = validate_file(&p, &opts, &links).unwrap();
        assert!(broken.is_empty());
    }

    #[test]
    fn filter_skip_paths_excludes_listed() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("docs/keep.md");
        let f2 = tmp.path().join("skip/me.md");
        let files = vec![f1.clone(), f2.clone()];
        let filtered = filter_skip_paths(files, tmp.path(), &["skip".to_string()]);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0], f1);
    }

    #[test]
    fn resolve_link_handles_anchors() {
        let source = PathBuf::from("/repo/docs/a.md");
        let resolved = resolve_link(&source, "b.md#section");
        assert_eq!(resolved, PathBuf::from("/repo/docs/b.md"));
    }

    #[test]
    fn resolve_link_pure_anchor_returns_source() {
        let source = PathBuf::from("/repo/docs/a.md");
        let resolved = resolve_link(&source, "");
        assert_eq!(resolved, source);
    }

    #[test]
    fn clean_path_resolves_dotdot() {
        let p = PathBuf::from("/a/b/../c");
        let cleaned = clean_path(&p);
        assert_eq!(cleaned, PathBuf::from("/a/c"));
    }

    #[test]
    fn format_link_json_has_status() {
        let result = LinkValidationResult {
            total_files: 5,
            total_links: 20,
            broken_links: Vec::new(),
            broken_by_category: HashMap::new(),
            scan_duration_ms: 100,
        };
        let s = format_link_json(&result).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["total_files"], 5);
    }
}
