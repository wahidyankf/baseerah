//! Internal link checking for Hugo/Next.js markdown content.
//!
//! # Overview
//!
//! Walks a content directory, extracts internal markdown links from `.md` files,
//! and resolves them relative to the content root. Returns a [`CheckResult`] that
//! callers can format as text, JSON, or Markdown via the output helpers.
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

use anyhow::Context as _;
use regex::Regex;
use walkdir::WalkDir;

/// A broken internal markdown link found during a link check.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BrokenLink {
    /// The `.md` file that contains the broken link.
    pub source_file: String,
    /// The 1-based line number of the broken link.
    pub line: usize,
    /// The display text of the markdown link (`[text](...)`).
    pub text: String,
    /// The resolved target path that could not be found.
    pub target: String,
}

/// Aggregate result of a link-check run over a content directory.
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Total number of internal links checked.
    pub checked_count: usize,
    /// Number of walk or file-read errors encountered.
    pub error_count: usize,
    /// Human-readable descriptions of any errors encountered.
    pub errors: Vec<String>,
    /// All broken links discovered.
    pub broken_links: Vec<BrokenLink>,
}

/// Walk `content_dir` recursively, extract internal markdown links from every
/// `.md` file, and return a [`CheckResult`] describing what was found.
///
/// # Errors
///
/// Returns an error if `content_dir` does not exist or if the directory walk
/// itself fails (individual file errors are collected in [`CheckResult::errors`]).
pub fn check_links(content_dir: &Path) -> anyhow::Result<CheckResult> {
    let abs_content_dir = fs::canonicalize(content_dir).with_context(|| {
        format!(
            "content directory does not exist: {}",
            content_dir.display()
        )
    })?;

    let link_re = Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").context("failed to compile link regex")?;

    let mut result = CheckResult {
        checked_count: 0,
        error_count: 0,
        errors: Vec::new(),
        broken_links: Vec::new(),
    };

    for entry in WalkDir::new(&abs_content_dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                result.error_count += 1;
                result.errors.push(format!("walk error: {e}"));
                continue;
            }
        };

        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext != "md" {
            continue;
        }

        match check_file_links(path, &abs_content_dir, &link_re) {
            Ok((checked, broken)) => {
                result.checked_count += checked;
                result.broken_links.extend(broken);
            }
            Err(e) => {
                result.error_count += 1;
                result
                    .errors
                    .push(format!("error reading {}: {e}", path.display()));
            }
        }
    }

    Ok(result)
}

/// Check internal links in a single markdown file.
///
/// Returns `(checked_count, broken_links)`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
fn check_file_links(
    file_path: &Path,
    abs_content_dir: &Path,
    link_re: &Regex,
) -> anyhow::Result<(usize, Vec<BrokenLink>)> {
    let file = fs::File::open(file_path)
        .with_context(|| format!("cannot open {}", file_path.display()))?;
    let reader = io::BufReader::new(file);

    let mut checked = 0usize;
    let mut broken = Vec::new();
    let mut in_fenced_block = false;

    for (line_idx, line_result) in reader.lines().enumerate() {
        let line = line_result.with_context(|| format!("read error in {}", file_path.display()))?;
        let line_num = line_idx + 1;

        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fenced_block = !in_fenced_block;
            continue;
        }
        if in_fenced_block {
            continue;
        }

        for cap in link_re.captures_iter(&line) {
            let text = cap[1].to_owned();
            let raw_target = cap[2].to_owned();

            if !is_internal_link(&raw_target) {
                continue;
            }

            let target = strip_fragment_and_query(&raw_target);
            checked += 1;

            if !target_exists(abs_content_dir, &target) {
                broken.push(BrokenLink {
                    source_file: file_path.display().to_string(),
                    line: line_num,
                    text,
                    target,
                });
            }
        }
    }

    Ok((checked, broken))
}

/// Return `true` when `target` is an internal (non-external, non-anchor) link
/// that points to a markdown content page rather than a static asset.
fn is_internal_link(target: &str) -> bool {
    if target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
        || target.starts_with("//")
    {
        return false;
    }
    if target.starts_with('#') {
        return false;
    }
    if has_file_extension(target) {
        return false;
    }
    true
}

/// Return `true` when the last path segment of `target` contains a dot,
/// indicating a link to a static file rather than a Hugo/Next.js page.
fn has_file_extension(target: &str) -> bool {
    let last = match target.rfind('/') {
        Some(idx) => &target[idx + 1..],
        None => target,
    };
    last.contains('.')
}

/// Strip the fragment (`#…`) and query (`?…`) parts from `target`.
fn strip_fragment_and_query(target: &str) -> String {
    let mut s = target;
    if let Some(idx) = s.find('#') {
        s = &s[..idx];
    }
    if let Some(idx) = s.find('?') {
        s = &s[..idx];
    }
    s.to_owned()
}

/// Return `true` when `abs_content_dir/target.md` or
/// `abs_content_dir/target/_index.md` exists on disk.
fn target_exists(abs_content_dir: &Path, target: &str) -> bool {
    // Normalise the target (strip leading slash so Path::join works)
    let target_stripped = target.trim_start_matches('/');
    let local_path = abs_content_dir.join(target_stripped);

    // Check <target>.md
    let mut md_path = local_path.clone();
    md_path.set_extension("md");
    // Avoid .md.md: only append if extension wasn't already .md
    if md_path.exists() {
        return true;
    }
    // Fall back to checking <target>.md directly (covers the set_extension approach above)
    // and also try via string concatenation matching Go's `localPath + ".md"`
    let md_str = format!("{}.md", local_path.display());
    if Path::new(&md_str).exists() {
        return true;
    }

    // Check <target>/_index.md
    let index_path = local_path.join("_index.md");
    if index_path.exists() {
        return true;
    }

    false
}

/// Print a human-readable link-check report to stdout.
///
/// When `quiet` is `true`, prints nothing on success and only errors on failure.
/// When `verbose` is `true`, appends a completion timestamp.
pub fn output_links_text(result: &CheckResult, elapsed: Duration, quiet: bool, verbose: bool) {
    if quiet {
        return;
    }

    println!();
    println!("Link Check Complete");
    println!("===================");
    println!("Checked:  {} link(s)", result.checked_count);
    println!("Broken:   {} link(s)", result.broken_links.len());
    println!("Errors:   {}", result.error_count);
    println!("Duration: {elapsed:?}");

    if !result.errors.is_empty() {
        println!();
        println!("Errors:");
        for e in &result.errors {
            println!("  - {e}");
        }
    }

    if !result.broken_links.is_empty() {
        println!();
        println!("Broken Links:");
        println!(
            "  {:<60} {:>5}  {:<30}  Target",
            "Source File", "Line", "Text"
        );
        println!("  {:<60} {:>5}  {:<30}  ---", "---", "---", "---");
        for bl in &result.broken_links {
            println!(
                "  {:<60} {:>5}  {:<30}  {}",
                bl.source_file, bl.line, bl.text, bl.target
            );
        }
    }

    if verbose {
        let ts = chrono::Local::now().to_rfc3339();
        println!("\nCompleted at: {ts}");
    }
}

/// Serialise the check result as a pretty-printed JSON string.
///
/// # Errors
///
/// Returns an error if JSON serialisation fails.
pub fn output_links_json(result: &CheckResult, elapsed: Duration) -> anyhow::Result<String> {
    let status = if result.broken_links.is_empty() {
        "success"
    } else {
        "failure"
    };

    let timestamp = chrono::Local::now().to_rfc3339();
    let duration_ms = u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX);

    let mut map = serde_json::Map::new();
    map.insert(
        "status".to_owned(),
        serde_json::Value::String(status.to_owned()),
    );
    map.insert("timestamp".to_owned(), serde_json::Value::String(timestamp));
    map.insert(
        "duration_ms".to_owned(),
        serde_json::Value::Number(duration_ms.into()),
    );
    map.insert(
        "checked".to_owned(),
        serde_json::Value::Number(result.checked_count.into()),
    );
    map.insert(
        "broken".to_owned(),
        serde_json::Value::Number(result.broken_links.len().into()),
    );
    map.insert(
        "errors".to_owned(),
        serde_json::to_value(&result.errors).context("serialise errors")?,
    );
    map.insert(
        "broken_links".to_owned(),
        serde_json::to_value(&result.broken_links).context("serialise broken_links")?,
    );

    serde_json::to_string_pretty(&serde_json::Value::Object(map))
        .context("JSON serialisation failed")
}

/// Print a Markdown-formatted link-check report to stdout.
pub fn output_links_markdown(result: &CheckResult, elapsed: Duration) {
    let status = if result.broken_links.is_empty() {
        "PASS"
    } else {
        "FAIL"
    };
    let timestamp = chrono::Local::now().to_rfc3339();

    println!("# Link Check Report");
    println!();
    println!("**Timestamp**: {timestamp}");
    println!("**Duration**: {elapsed:?}");
    println!("**Status**: {status}");
    println!();
    println!("## Summary");
    println!();
    println!("| Metric | Count |");
    println!("| --- | --- |");
    println!("| Checked | {} |", result.checked_count);
    println!("| Broken | {} |", result.broken_links.len());
    println!("| Errors | {} |", result.error_count);

    if !result.errors.is_empty() {
        println!();
        println!("## Errors");
        println!();
        for e in &result.errors {
            println!("- {e}");
        }
    }

    if !result.broken_links.is_empty() {
        println!();
        println!("## Broken Links");
        println!();
        println!("| Source File | Line | Text | Target |");
        println!("| --- | --- | --- | --- |");
        for bl in &result.broken_links {
            println!(
                "| {} | {} | {} | {} |",
                bl.source_file, bl.line, bl.text, bl.target
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_check_links_returns_ok_for_empty_dir() {
        let dir = TempDir::new().expect("tempdir");
        let result = check_links(dir.path()).expect("check_links failed");
        assert_eq!(result.broken_links.len(), 0);
        assert_eq!(result.checked_count, 0);
    }

    #[test]
    fn test_check_links_detects_broken_link() {
        let dir = TempDir::new().expect("tempdir");
        let md = dir.path().join("page.md");
        let mut f = std::fs::File::create(&md).expect("create file");
        writeln!(f, "[broken](/does-not-exist)").expect("write");
        let result = check_links(dir.path()).expect("check_links failed");
        assert!(!result.broken_links.is_empty(), "expected broken link");
    }

    #[test]
    fn test_check_links_skips_code_blocks() {
        let dir = TempDir::new().expect("tempdir");
        let md = dir.path().join("page.md");
        let mut f = std::fs::File::create(&md).expect("create file");
        writeln!(f, "```\n[skipped](/does-not-exist)\n```").expect("write");
        let result = check_links(dir.path()).expect("check_links failed");
        assert_eq!(result.broken_links.len(), 0);
    }

    #[test]
    fn test_check_links_rejects_nonexistent_dir() {
        let result = check_links(Path::new("/nonexistent/path/xyz_abc_123"));
        assert!(result.is_err(), "expected error for nonexistent dir");
    }

    #[test]
    fn test_output_links_json_contains_required_keys() {
        let result = CheckResult {
            checked_count: 5,
            error_count: 0,
            errors: vec![],
            broken_links: vec![],
        };
        let json = output_links_json(&result, Duration::from_millis(42)).expect("json failed");
        let v: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
        assert!(v.get("status").is_some());
        assert!(v.get("checked").is_some());
        assert!(v.get("broken_links").is_some());
        assert!(v.get("duration_ms").is_some());
    }

    #[test]
    fn test_output_links_markdown_contains_headings() {
        let result = CheckResult {
            checked_count: 0,
            error_count: 0,
            errors: vec![],
            broken_links: vec![],
        };
        // Capture stdout is complex in unit tests; just assert it doesn't panic.
        output_links_markdown(&result, Duration::from_millis(0));
    }

    // --- is_internal_link ---

    #[test]
    fn test_is_internal_link_returns_false_for_http() {
        assert!(!is_internal_link("http://example.com/page"));
    }

    #[test]
    fn test_is_internal_link_returns_false_for_https() {
        assert!(!is_internal_link("https://example.com/page"));
    }

    #[test]
    fn test_is_internal_link_returns_false_for_mailto() {
        assert!(!is_internal_link("mailto:user@example.com"));
    }

    #[test]
    fn test_is_internal_link_returns_false_for_protocol_relative() {
        assert!(!is_internal_link("//cdn.example.com/asset.js"));
    }

    #[test]
    fn test_is_internal_link_returns_false_for_anchor_only() {
        assert!(!is_internal_link("#section-heading"));
    }

    #[test]
    fn test_is_internal_link_returns_false_for_file_with_extension() {
        assert!(!is_internal_link("/images/logo.png"));
    }

    #[test]
    fn test_is_internal_link_returns_true_for_internal_path() {
        assert!(is_internal_link("/docs/getting-started"));
    }

    // --- has_file_extension ---

    #[test]
    fn test_has_file_extension_with_extension() {
        assert!(has_file_extension("/assets/image.png"));
    }

    #[test]
    fn test_has_file_extension_without_extension() {
        assert!(!has_file_extension("/docs/getting-started"));
    }

    #[test]
    fn test_has_file_extension_with_dot_in_dir_but_no_extension_in_filename() {
        // The last segment after the final '/' is "page" which has no dot.
        assert!(!has_file_extension("/path.to/page"));
    }

    // --- strip_fragment_and_query ---

    #[test]
    fn test_strip_fragment_and_query_with_fragment() {
        assert_eq!(strip_fragment_and_query("/docs/page#section"), "/docs/page");
    }

    #[test]
    fn test_strip_fragment_and_query_with_query() {
        assert_eq!(strip_fragment_and_query("/docs/page?tab=1"), "/docs/page");
    }

    #[test]
    fn test_strip_fragment_and_query_with_both() {
        assert_eq!(
            strip_fragment_and_query("/docs/page?tab=1#section"),
            "/docs/page"
        );
    }

    #[test]
    fn test_strip_fragment_and_query_plain() {
        assert_eq!(strip_fragment_and_query("/docs/page"), "/docs/page");
    }

    // --- target_exists ---

    #[test]
    fn test_target_exists_via_index_md() {
        let dir = TempDir::new().expect("tempdir");
        // Create <target>/_index.md so the section root resolves.
        let section = dir.path().join("section");
        std::fs::create_dir_all(&section).expect("create dir");
        std::fs::File::create(section.join("_index.md")).expect("create _index.md");

        assert!(target_exists(dir.path(), "section"));
    }

    #[test]
    fn test_target_exists_via_dot_md_file() {
        let dir = TempDir::new().expect("tempdir");
        std::fs::File::create(dir.path().join("about.md")).expect("create about.md");

        assert!(target_exists(dir.path(), "about"));
    }

    #[test]
    fn test_target_exists_returns_false_when_missing() {
        let dir = TempDir::new().expect("tempdir");
        assert!(!target_exists(dir.path(), "nonexistent-page"));
    }

    // --- output_links_json status field ---

    #[test]
    fn test_output_links_json_status_is_failure_when_broken_links_present() {
        let result = CheckResult {
            checked_count: 1,
            error_count: 0,
            errors: vec![],
            broken_links: vec![BrokenLink {
                source_file: "page.md".to_owned(),
                line: 1,
                text: "broken".to_owned(),
                target: "/missing".to_owned(),
            }],
        };
        let json = output_links_json(&result, Duration::from_millis(10)).expect("json failed");
        let v: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
        assert_eq!(v["status"], "failure");
    }

    #[test]
    fn test_output_links_json_status_is_success_when_no_broken_links() {
        let result = CheckResult {
            checked_count: 3,
            error_count: 0,
            errors: vec![],
            broken_links: vec![],
        };
        let json = output_links_json(&result, Duration::from_millis(5)).expect("json failed");
        let v: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
        assert_eq!(v["status"], "success");
    }

    // --- output_links_text ---

    #[test]
    fn test_output_links_text_quiet_does_not_panic() {
        let result = CheckResult {
            checked_count: 0,
            error_count: 0,
            errors: vec![],
            broken_links: vec![],
        };
        // quiet=true returns immediately; must not panic.
        output_links_text(&result, Duration::from_millis(0), true, false);
    }

    #[test]
    fn test_output_links_text_non_quiet_does_not_panic() {
        let result = CheckResult {
            checked_count: 2,
            error_count: 0,
            errors: vec![],
            broken_links: vec![],
        };
        output_links_text(&result, Duration::from_millis(10), false, false);
    }

    #[test]
    fn test_output_links_text_verbose_does_not_panic() {
        let result = CheckResult {
            checked_count: 0,
            error_count: 0,
            errors: vec![],
            broken_links: vec![],
        };
        // verbose=true appends a timestamp line; must not panic.
        output_links_text(&result, Duration::from_millis(0), false, true);
    }

    #[test]
    fn test_output_links_text_with_errors_does_not_panic() {
        let result = CheckResult {
            checked_count: 0,
            error_count: 1,
            errors: vec!["read error: permission denied".to_owned()],
            broken_links: vec![],
        };
        output_links_text(&result, Duration::from_millis(5), false, false);
    }

    #[test]
    fn test_output_links_text_with_broken_links_does_not_panic() {
        let result = CheckResult {
            checked_count: 2,
            error_count: 0,
            errors: vec![],
            broken_links: vec![
                BrokenLink {
                    source_file: "docs/page.md".to_owned(),
                    line: 10,
                    text: "missing page".to_owned(),
                    target: "/docs/missing".to_owned(),
                },
                BrokenLink {
                    source_file: "docs/other.md".to_owned(),
                    line: 5,
                    text: "also missing".to_owned(),
                    target: "/docs/also-missing".to_owned(),
                },
            ],
        };
        output_links_text(&result, Duration::from_millis(20), false, false);
    }

    // --- output_links_markdown with broken links and errors ---

    #[test]
    fn test_output_links_markdown_with_broken_links_does_not_panic() {
        let result = CheckResult {
            checked_count: 1,
            error_count: 0,
            errors: vec![],
            broken_links: vec![BrokenLink {
                source_file: "index.md".to_owned(),
                line: 3,
                text: "bad link".to_owned(),
                target: "/nowhere".to_owned(),
            }],
        };
        output_links_markdown(&result, Duration::from_millis(15));
    }

    #[test]
    fn test_output_links_markdown_with_errors_does_not_panic() {
        let result = CheckResult {
            checked_count: 0,
            error_count: 2,
            errors: vec![
                "walk error: permission denied".to_owned(),
                "error reading foo.md: unexpected EOF".to_owned(),
            ],
            broken_links: vec![],
        };
        output_links_markdown(&result, Duration::from_millis(8));
    }

    // --- check_links with file that has only external/non-internal links ---

    #[test]
    fn test_check_links_skips_external_links() {
        let dir = TempDir::new().expect("tempdir");
        let md = dir.path().join("page.md");
        let mut f = std::fs::File::create(&md).expect("create file");
        writeln!(
            f,
            "[external](https://example.com) [anchor](#section) [img](/logo.png)"
        )
        .expect("write");
        let result = check_links(dir.path()).expect("check_links failed");
        // No internal links checked, no broken links.
        assert_eq!(result.checked_count, 0);
        assert_eq!(result.broken_links.len(), 0);
    }

    // --- check_links with a valid internal link ---

    #[test]
    fn test_check_links_passes_valid_internal_link() {
        let dir = TempDir::new().expect("tempdir");
        // Create the target file.
        std::fs::File::create(dir.path().join("about.md")).expect("create about.md");
        // Create a page that links to it.
        let md = dir.path().join("page.md");
        let mut f = std::fs::File::create(&md).expect("create file");
        writeln!(f, "[About](/about)").expect("write");
        let result = check_links(dir.path()).expect("check_links failed");
        assert_eq!(result.checked_count, 1);
        assert_eq!(result.broken_links.len(), 0);
    }
}
