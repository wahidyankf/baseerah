// Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/license_audit.go`.

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Error};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LicenseFinding {
    pub path: String,
    pub kind: String,
    pub message: String,
}

const LICENSE_EXEMPT_APPS: &[&str] = &["rhino-cli"];

pub fn audit_license(repo_root: &Path) -> std::result::Result<Vec<LicenseFinding>, Error> {
    let mut findings = Vec::new();
    let dirs = required_license_dirs(repo_root)?;
    let mut license_by_dir: HashMap<String, String> = HashMap::new();

    for rel in &dirs {
        let license_path = repo_root.join(rel).join("LICENSE");
        match extract_spdx(&license_path) {
            Ok(spdx) => {
                license_by_dir.insert(rel.clone(), spdx);
            }
            Err(e) => {
                let is_not_found = e
                    .downcast_ref::<std::io::Error>()
                    .map(|io| io.kind() == std::io::ErrorKind::NotFound)
                    .unwrap_or(false);
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
    let claims = match parse_licensing_notice(&notice_path) {
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
        let identified = match license_by_dir.get(&normalised) {
            Some(v) => v,
            None => continue,
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

fn required_license_dirs(repo_root: &Path) -> std::result::Result<Vec<String>, Error> {
    let mut dirs = Vec::new();
    let apps = read_non_hidden_dirs(&repo_root.join("apps"))?;
    for name in &apps {
        if LICENSE_EXEMPT_APPS.contains(&name.as_str()) {
            continue;
        }
        if name.ends_with("-e2e") {
            continue;
        }
        dirs.push(format!("apps/{name}"));
    }
    let libs = read_non_hidden_dirs(&repo_root.join("libs"))?;
    for name in &libs {
        dirs.push(format!("libs/{name}"));
    }
    let specs = repo_root.join("specs");
    match fs::metadata(&specs) {
        Ok(m) if m.is_dir() => dirs.push("specs".to_string()),
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(Error::msg(format!("stat {}: {e}", specs.display())));
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn read_non_hidden_dirs(dir: &Path) -> std::result::Result<Vec<String>, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(Error::msg(format!("read {}: {e}", dir.display()))),
    };
    let mut names = Vec::new();
    for entry in entries.flatten() {
        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        names.push(name);
    }
    names.sort();
    Ok(names)
}

fn extract_spdx(path: &Path) -> std::result::Result<String, Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
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

fn strip_prefix_fold<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.len() < prefix.len() {
        return None;
    }
    if !s[..prefix.len()].eq_ignore_ascii_case(prefix) {
        return None;
    }
    Some(&s[prefix.len()..])
}

#[derive(Debug, Clone)]
struct LicenseClaim {
    path: String,
    license: String,
}

fn parse_licensing_notice(path: &Path) -> std::result::Result<Vec<LicenseClaim>, Error> {
    let data = fs::read_to_string(path)?;
    let lines: Vec<&str> = data.split('\n').collect();
    let mut claims = Vec::new();
    let mut path_col: i64 = -1;
    let mut license_col: i64 = -1;
    let mut in_table = false;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if !line.starts_with('|') {
            path_col = -1;
            license_col = -1;
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
            if path_col >= 0 && license_col >= 0 {
                in_table = true;
            }
            i += 2; // skip header + separator
            continue;
        }
        if (path_col as usize) >= cells.len() || (license_col as usize) >= cells.len() {
            i += 1;
            continue;
        }
        let raw_path = cells[path_col as usize].trim();
        let raw_license = cells[license_col as usize].trim();
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

fn find_columns(cells: &[String]) -> (i64, i64) {
    let mut path_col: i64 = -1;
    let mut license_col: i64 = -1;
    for (i, c) in cells.iter().enumerate() {
        let h = c.trim().to_lowercase();
        match h.as_str() {
            "path" | "directory" => {
                if path_col == -1 {
                    path_col = i as i64;
                }
            }
            "license" => {
                if license_col == -1 {
                    license_col = i as i64;
                }
            }
            _ => {}
        }
    }
    (path_col, license_col)
}

fn normalise_claim_path(raw: &str) -> String {
    let s = raw.trim();
    let s = s.trim_matches('`');
    let s = s.trim();
    let s = s.strip_prefix("./").unwrap_or(s);
    let s = s.strip_suffix('/').unwrap_or(s);
    s.replace('\\', "/")
}

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

fn licenses_equal(identified: &str, claim: &str) -> bool {
    if identified.eq_ignore_ascii_case(claim) {
        return true;
    }
    let ni = classify_license_line(identified);
    let nc = classify_license_line(claim);
    ni.eq_ignore_ascii_case(&nc)
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let findings = audit_license(tmp.path()).unwrap();
        assert!(findings
            .iter()
            .any(|f| f.kind == "missing-license" && f.path == "apps/foo"));
    }

    #[test]
    fn skips_exempt_apps_and_e2e() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("apps/rhino-cli")).unwrap();
        fs::create_dir_all(tmp.path().join("apps/foo-e2e")).unwrap();
        let findings = audit_license(tmp.path()).unwrap();
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
        let findings = audit_license(tmp.path()).unwrap();
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
        let findings = audit_license(tmp.path()).unwrap();
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
        assert_eq!(p, 0);
        assert_eq!(l, 1);
    }
}
