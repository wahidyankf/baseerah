//! License presence and SPDX-consistency audit.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/license_audit.go`.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Error};
use serde::Serialize;

use crate::application::fs::port::Fs;

/// A single finding from the license audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LicenseFinding {
    /// Relative path of the directory or entry that triggered the finding.
    pub path: String,
    /// Machine-readable category: `"missing-license"`, `"unreadable-license"`,
    /// or `"spdx-mismatch"`.
    pub kind: String,
    /// Human-readable description of the finding.
    pub message: String,
}

/// App directories that are intentionally exempt from the LICENSE requirement.
const LICENSE_EXEMPT_APPS: &[&str] = &["rhino-cli"];

/// Audits every required `apps/` and `libs/` subdirectory for a `LICENSE`
/// file and cross-checks identified SPDX identifiers against
/// `LICENSING-NOTICE.md`.
///
/// Findings are sorted by `path`, then by `kind`.
///
/// # Errors
///
/// Returns an error when the repository directory structure cannot be read or
/// when `LICENSING-NOTICE.md` exists but cannot be parsed.
pub fn audit_license(
    fs: &dyn Fs,
    repo_root: &Path,
) -> std::result::Result<Vec<LicenseFinding>, Error> {
    let mut findings = Vec::new();
    let dirs = required_license_dirs(fs, repo_root)?;
    let mut license_by_dir: HashMap<String, String> = HashMap::new();

    for rel in &dirs {
        let license_path = repo_root.join(rel).join("LICENSE");
        match extract_spdx(fs, &license_path) {
            Ok(spdx) => {
                license_by_dir.insert(rel.clone(), spdx);
            }
            Err(e) => {
                let is_not_found = e
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|io| io.kind() == std::io::ErrorKind::NotFound);
                if is_not_found {
                    findings.push(LicenseFinding {
                        path: rel.clone(),
                        kind: "missing-license".to_string(),
                        message: format!("required directory \"{rel}\" has no LICENSE file"),
                    });
                    continue;
                }
                findings.push(LicenseFinding {
                    path: rel.clone(),
                    kind: "unreadable-license".to_string(),
                    message: format!("read LICENSE in \"{rel}\": {e}"),
                });
            }
        }
    }

    let notice_path = repo_root.join("LICENSING-NOTICE.md");
    let claims = match parse_licensing_notice(fs, &notice_path) {
        Ok(c) => c,
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    Vec::new()
                } else {
                    return Err(e);
                }
            } else {
                return Err(e);
            }
        }
    };

    for claim in claims {
        let normalised = normalise_claim_path(&claim.path);
        if !owned_by_license_audit(&normalised) {
            continue;
        }
        let Some(identified) = license_by_dir.get(&normalised) else {
            continue;
        };
        if !licenses_equal(identified, &claim.license) {
            findings.push(LicenseFinding {
                path: normalised.clone(),
                kind: "spdx-mismatch".to_string(),
                message: format!(
                    "LICENSING-NOTICE.md claims \"{}\" for \"{normalised}\" but LICENSE identifies \"{identified}\"",
                    claim.license
                ),
            });
        }
    }

    findings.sort_by(|a, b| a.path.cmp(&b.path).then(a.kind.cmp(&b.kind)));
    Ok(findings)
}

/// Returns a sorted list of relative directory paths that must contain a
/// `LICENSE` file.
///
/// Includes non-exempt, non-`-e2e` subdirectories of `apps/`, all
/// subdirectories of `libs/`, and the `specs/` directory when it exists.
///
/// # Errors
///
/// Returns an error when the `apps/` or `libs/` directories cannot be listed
/// or when `specs/` metadata cannot be read.
fn required_license_dirs(fs: &dyn Fs, repo_root: &Path) -> std::result::Result<Vec<String>, Error> {
    let mut dirs = Vec::new();
    let apps = read_non_hidden_dirs(fs, &repo_root.join("apps"))?;
    for name in &apps {
        if LICENSE_EXEMPT_APPS.contains(&name.as_str()) {
            continue;
        }
        if name.ends_with("-e2e") {
            continue;
        }
        dirs.push(format!("apps/{name}"));
    }
    let libs = read_non_hidden_dirs(fs, &repo_root.join("libs"))?;
    for name in &libs {
        dirs.push(format!("libs/{name}"));
    }
    let specs = repo_root.join("specs");
    if fs.exists(&specs) && fs.is_dir(&specs) {
        dirs.push("specs".to_string());
    }
    dirs.sort();
    Ok(dirs)
}

/// Returns the sorted names of non-hidden subdirectories inside `dir`.
///
/// Returns an empty `Vec` when `dir` does not exist.
///
/// # Errors
///
/// Returns an error when `dir` exists but cannot be read.
fn read_non_hidden_dirs(fs: &dyn Fs, dir: &Path) -> std::result::Result<Vec<String>, Error> {
    let entries = match fs.read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(Error::msg(format!("read {}: {e}", dir.display()))),
    };
    let mut names = Vec::new();
    for entry in entries {
        if !entry.is_dir {
            continue;
        }
        if entry.name.starts_with('.') {
            continue;
        }
        names.push(entry.name);
    }
    names.sort();
    Ok(names)
}

/// Reads the first non-blank line of the `LICENSE` file at `path` and
/// classifies it as an SPDX identifier string.
///
/// # Errors
///
/// Returns an error when the file cannot be opened, a line cannot be read, or
/// the file is empty.
fn extract_spdx(fs: &dyn Fs, path: &Path) -> std::result::Result<String, Error> {
    let lines = fs.read_lines(path)?;
    for line in lines {
        let line = line.with_context(|| format!("scan {}", path.display()))?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        return Ok(classify_license_line(trimmed));
    }
    Err(Error::msg(format!(
        "LICENSE file \"{}\" is empty",
        path.display()
    )))
}

/// Maps the first line of a `LICENSE` file to a canonical SPDX identifier.
///
/// Recognises `SPDX-License-Identifier:` headers as well as common prose
/// patterns for MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, MPL-2.0, and
/// GPL.  Returns `line` unchanged when no pattern matches.
fn classify_license_line(line: &str) -> String {
    if let Some(rest) = strip_prefix_fold(line, "SPDX-License-Identifier:") {
        return rest.trim().to_string();
    }
    let lower = line.to_lowercase();
    if lower.contains("mit license") || lower == "mit" {
        return "MIT".to_string();
    }
    if lower.contains("apache license, version 2.0")
        || lower.contains("apache license 2.0")
        || lower.contains("apache-2.0")
    {
        return "Apache-2.0".to_string();
    }
    if lower.contains("bsd 3-clause") || lower.contains("bsd-3-clause") {
        return "BSD-3-Clause".to_string();
    }
    if lower.contains("bsd 2-clause") || lower.contains("bsd-2-clause") {
        return "BSD-2-Clause".to_string();
    }
    if lower.contains("mozilla public license") || lower.contains("mpl-2.0") {
        return "MPL-2.0".to_string();
    }
    if lower.contains("gnu general public license") {
        return "GPL".to_string();
    }
    line.to_string()
}

/// Case-insensitively strips `prefix` from the start of `s`.
///
/// Returns `Some(&s[prefix.len()..])` when `s` starts with `prefix`
/// (ASCII-case-insensitive), or `None` otherwise.
fn strip_prefix_fold<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.len() < prefix.len() {
        return None;
    }
    if !s[..prefix.len()].eq_ignore_ascii_case(prefix) {
        return None;
    }
    Some(&s[prefix.len()..])
}

/// A single row parsed from the `LICENSING-NOTICE.md` table.
#[derive(Debug, Clone)]
struct LicenseClaim {
    /// Relative directory path as it appears in the table.
    path: String,
    /// SPDX identifier claimed in the table.
    license: String,
}

/// Parses `LICENSING-NOTICE.md` at `path` and extracts all `LicenseClaim`
/// rows from GFM tables that have both a `Path`/`Directory` column and a
/// `License` column.
///
/// # Errors
///
/// Returns an error when the file cannot be read.
fn parse_licensing_notice(
    fs: &dyn Fs,
    path: &Path,
) -> std::result::Result<Vec<LicenseClaim>, Error> {
    let data = fs.read_to_string(path)?;
    let lines: Vec<&str> = data.split('\n').collect();
    let mut claims = Vec::new();
    let mut path_col: Option<usize> = None;
    let mut license_col: Option<usize> = None;
    let mut in_table = false;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if !line.starts_with('|') {
            path_col = None;
            license_col = None;
            in_table = false;
            i += 1;
            continue;
        }
        let cells = split_markdown_row(line);
        if !in_table {
            if i + 1 >= lines.len() {
                i += 1;
                continue;
            }
            let sep = lines[i + 1].trim();
            if !is_markdown_table_separator(sep) {
                i += 1;
                continue;
            }
            let (pc, lc) = find_columns(&cells);
            path_col = pc;
            license_col = lc;
            if path_col.is_some() && license_col.is_some() {
                in_table = true;
            }
            i += 2; // skip header + separator
            continue;
        }
        let (Some(pc), Some(lc)) = (path_col, license_col) else {
            i += 1;
            continue;
        };
        if pc >= cells.len() || lc >= cells.len() {
            i += 1;
            continue;
        }
        let raw_path = cells[pc].trim();
        let raw_license = cells[lc].trim();
        if raw_path.is_empty() || raw_license.is_empty() {
            i += 1;
            continue;
        }
        claims.push(LicenseClaim {
            path: raw_path.to_string(),
            license: raw_license.to_string(),
        });
        i += 1;
    }
    Ok(claims)
}

/// Splits a GFM table row `line` into individual cell strings, respecting
/// backslash-escaped pipe characters.
fn split_markdown_row(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);
    let mut cells = Vec::new();
    let mut current = String::new();
    let mut escaped = false;
    for r in trimmed.chars() {
        if escaped {
            current.push(r);
            escaped = false;
            continue;
        }
        if r == '\\' {
            escaped = true;
            continue;
        }
        if r == '|' {
            cells.push(current.clone());
            current.clear();
            continue;
        }
        current.push(r);
    }
    cells.push(current);
    cells
}

/// Returns `true` when `line` is a GFM table separator row (e.g., `| --- | :---: |`).
fn is_markdown_table_separator(line: &str) -> bool {
    if !line.starts_with('|') {
        return false;
    }
    let cells = split_markdown_row(line);
    if cells.is_empty() {
        return false;
    }
    for c in &cells {
        let c = c.trim();
        let c = c.trim_matches(':');
        if c.is_empty() {
            return false;
        }
        for r in c.chars() {
            if r != '-' {
                return false;
            }
        }
    }
    true
}

/// Finds the column indices for the `path`/`directory` and `license` headers
/// in a GFM table header row.
///
/// Returns `(path_col, license_col)` where each is `None` when the
/// corresponding header is absent.
fn find_columns(cells: &[String]) -> (Option<usize>, Option<usize>) {
    let mut path_col: Option<usize> = None;
    let mut license_col: Option<usize> = None;
    for (i, c) in cells.iter().enumerate() {
        let h = c.trim().to_lowercase();
        match h.as_str() {
            "path" | "directory" if path_col.is_none() => {
                path_col = Some(i);
            }
            "license" if license_col.is_none() => {
                license_col = Some(i);
            }
            _ => {}
        }
    }
    (path_col, license_col)
}

/// Normalises a raw path value from `LICENSING-NOTICE.md` by stripping
/// surrounding whitespace, backticks, leading `./`, and trailing `/`, and
/// converting backslashes to forward slashes.
fn normalise_claim_path(raw: &str) -> String {
    let s = raw.trim();
    let s = s.trim_matches('`');
    let s = s.trim();
    let s = s.strip_prefix("./").unwrap_or(s);
    let s = s.strip_suffix('/').unwrap_or(s);
    s.replace('\\', "/")
}

/// Returns `true` when the path `p` falls within the scope of this audit
/// (immediate children of `apps/` or `libs/`, or the `specs` root).
fn owned_by_license_audit(p: &str) -> bool {
    if p == "specs" {
        return true;
    }
    if p.starts_with("apps/") || p.starts_with("libs/") {
        let rest = if let Some(r) = p.strip_prefix("apps/") {
            r
        } else {
            p.strip_prefix("libs/").unwrap_or(p)
        };
        if rest.is_empty() || rest.contains('/') {
            return false;
        }
        return true;
    }
    false
}

/// Returns `true` when `identified` and `claim` refer to the same SPDX
/// license, either by direct case-insensitive comparison or after normalising
/// both through [`classify_license_line`].
fn licenses_equal(identified: &str, claim: &str) -> bool {
    if identified.eq_ignore_ascii_case(claim) {
        return true;
    }
    let ni = classify_license_line(identified);
    let nc = classify_license_line(claim);
    ni.eq_ignore_ascii_case(&nc)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::real::RealFs;
    use std::fs;
    use tempfile::TempDir;

    fn write_license(tmp: &TempDir, rel: &str, text: &str) {
        let p = tmp.path().join(rel).join("LICENSE");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, text).unwrap();
    }

    #[test]
    fn classify_recognises_known_licenses() {
        assert_eq!(classify_license_line("MIT License"), "MIT");
        assert_eq!(classify_license_line("Apache License 2.0"), "Apache-2.0");
        assert_eq!(
            classify_license_line("BSD 3-Clause License"),
            "BSD-3-Clause"
        );
        assert_eq!(
            classify_license_line("BSD 2-Clause License"),
            "BSD-2-Clause"
        );
        assert_eq!(classify_license_line("SPDX-License-Identifier: MIT"), "MIT");
        assert_eq!(classify_license_line("Mozilla Public License"), "MPL-2.0");
        assert_eq!(classify_license_line("Random License"), "Random License");
    }

    #[test]
    fn licenses_equal_handles_aliases() {
        assert!(licenses_equal("MIT", "MIT License"));
        assert!(licenses_equal("MIT", "MIT"));
        assert!(!licenses_equal("MIT", "Apache-2.0"));
    }

    #[test]
    fn detects_missing_license() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("apps/foo")).unwrap();
        let findings = audit_license(&RealFs, tmp.path()).unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.kind == "missing-license" && f.path == "apps/foo")
        );
    }

    #[test]
    fn skips_exempt_apps_and_e2e() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("apps/rhino-cli")).unwrap();
        fs::create_dir_all(tmp.path().join("apps/foo-e2e")).unwrap();
        let findings = audit_license(&RealFs, tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn detects_spdx_mismatch() {
        let tmp = TempDir::new().unwrap();
        write_license(&tmp, "apps/foo", "MIT License\n");
        fs::write(
            tmp.path().join("LICENSING-NOTICE.md"),
            "# Notice\n\n| Path | License |\n| --- | --- |\n| apps/foo | Apache-2.0 |\n",
        )
        .unwrap();
        let findings = audit_license(&RealFs, tmp.path()).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, "spdx-mismatch");
    }

    #[test]
    fn passes_when_notice_matches() {
        let tmp = TempDir::new().unwrap();
        write_license(&tmp, "apps/foo", "MIT License\n");
        fs::write(
            tmp.path().join("LICENSING-NOTICE.md"),
            "# Notice\n\n| Path | License |\n| --- | --- |\n| apps/foo | MIT |\n",
        )
        .unwrap();
        let findings = audit_license(&RealFs, tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn normalise_strips_backticks_and_slashes() {
        assert_eq!(normalise_claim_path("`apps/foo`"), "apps/foo");
        assert_eq!(normalise_claim_path("./apps/foo/"), "apps/foo");
    }

    #[test]
    fn owned_by_audit_only_immediate_children() {
        assert!(owned_by_license_audit("specs"));
        assert!(owned_by_license_audit("apps/foo"));
        assert!(owned_by_license_audit("libs/bar"));
        assert!(!owned_by_license_audit("apps/foo/src"));
        assert!(!owned_by_license_audit("archived/old"));
    }

    #[test]
    fn split_markdown_row_handles_escape() {
        let cells = split_markdown_row("| a | b\\|c | d |");
        assert_eq!(cells.len(), 3);
        assert_eq!(cells[0].trim(), "a");
        assert_eq!(cells[1].trim(), "b|c");
        assert_eq!(cells[2].trim(), "d");
    }

    #[test]
    fn is_separator_recognises_separators() {
        assert!(is_markdown_table_separator("|---|---|"));
        assert!(is_markdown_table_separator("| --- | :---: |"));
        assert!(!is_markdown_table_separator("| a | b |"));
    }

    #[test]
    fn find_columns_locates_headers() {
        let cells = vec![
            "Path".to_string(),
            "License".to_string(),
            "Notes".to_string(),
        ];
        let (p, l) = find_columns(&cells);
        assert_eq!(p, Some(0));
        assert_eq!(l, Some(1));
    }
}
