//! Skip list manager — manages known false positives for crane checks.
#![allow(clippy::missing_docs_in_private_items)]

use crate::domain::SkipListEntry;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::io::Write;

const DEFAULT_PATH: &str = "generated-reports/.known-false-positives.md";
const FALSE_POSITIVE_PREFIX: &str = "## FALSE_POSITIVE:";
const DEFAULT_REASON: &str = "Auto-accepted via crane skiplist --add";

/// Resolves the skip list path from `CRANE_SKIPLIST_PATH` env var or default.
pub fn resolve_skiplist_path() -> String {
    std::env::var("CRANE_SKIPLIST_PATH").unwrap_or_else(|_| DEFAULT_PATH.to_string())
}

/// Computes a stable 16-hex-char SHA-256-based key.
///
/// The input is `"<md_basename>|<category>|<description>"` encoded as UTF-8.
/// This produces byte-identical output to the F# implementation.
pub fn stable_key(md_basename: &str, category: &str, description: &str) -> String {
    use std::fmt::Write as FmtWrite;
    let combined = format!("{md_basename}|{category}|{description}");
    let hash = Sha256::digest(combined.as_bytes());
    let mut hex = String::with_capacity(64);
    for b in &hash {
        let _ = write!(hex, "{b:02x}");
    }
    hex[..16].to_string()
}

fn now_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d--%H-%M").to_string()
}

fn parse_heading(line: &str) -> Option<(String, String, String)> {
    let body = line[FALSE_POSITIVE_PREFIX.len()..].trim_start();
    let parts: Vec<&str> = body.splitn(3, " | ").collect();
    if parts.len() == 3 {
        Some((
            parts[0].trim().to_string(),
            parts[1].trim().to_string(),
            parts[2].trim().to_string(),
        ))
    } else {
        None
    }
}

fn parse_metadata(block: &[&str]) -> std::collections::HashMap<String, String> {
    block
        .iter()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let stripped = trimmed.strip_prefix("**")?;
            let end_marker = stripped.find("**:")?;
            let key = stripped[..end_marker].trim().to_string();
            let value = stripped[end_marker + 3..].trim().to_string();
            Some((key, value))
        })
        .collect()
}

fn parse_entries(path: &str) -> Vec<SkipListEntry> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return vec![];
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut entries = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].starts_with(FALSE_POSITIVE_PREFIX) {
            let heading_line = lines[i];
            let meta_lines: Vec<&str> = lines[i + 1..]
                .iter()
                .copied()
                .take_while(|l| !l.starts_with("## ") && l.trim() != "---")
                .collect();

            if let Some((category, md_basename, description)) = parse_heading(heading_line) {
                let meta = parse_metadata(&meta_lines);
                let accepted = meta.get("Accepted").cloned().unwrap_or_default();
                let reason = meta.get("Reason").cloned().unwrap_or_default();
                let key = meta
                    .get("Key")
                    .cloned()
                    .unwrap_or_else(|| stable_key(&md_basename, &category, &description));

                entries.push(SkipListEntry {
                    md_basename,
                    category,
                    description,
                    key,
                    accepted,
                    reason,
                });
            } else {
                eprintln!("Warning: skipping malformed FALSE_POSITIVE heading: {heading_line}");
            }
            i += meta_lines.len() + 1;
        } else {
            i += 1;
        }
    }

    entries
}

fn render_entry(entry: &SkipListEntry) -> String {
    format!(
        "## FALSE_POSITIVE: {} | {} | {}\n\n**Accepted**: {}\n**Category**: {}\n**File**: {}\n**Finding**: {}\n**Key**: {}\n**Reason**: {}\n\n---\n\n",
        entry.category,
        entry.md_basename,
        entry.description,
        entry.accepted,
        entry.category,
        entry.md_basename,
        entry.description,
        entry.key,
        entry.reason,
    )
}

fn append_entry(path: &str, entry: &SkipListEntry) -> Result<()> {
    let text = render_entry(entry);

    if std::path::Path::new(path).exists() {
        let existing = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read skiplist: {path}"))?;
        let needs_blank_line = !existing.ends_with("\n\n") && !existing.is_empty();
        let prefix = if needs_blank_line { "\n" } else { "" };
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .with_context(|| format!("Failed to open skiplist: {path}"))?;
        file.write_all(format!("{prefix}{text}").as_bytes())
            .with_context(|| format!("Failed to write skiplist: {path}"))?;
    } else {
        if let Some(dir) = std::path::Path::new(path)
            .parent()
            .filter(|d| !d.as_os_str().is_empty() && !d.exists())
        {
            std::fs::create_dir_all(dir)
                .with_context(|| format!("Failed to create dir for skiplist: {path}"))?;
        }
        std::fs::write(path, text).with_context(|| format!("Failed to write skiplist: {path}"))?;
    }

    Ok(())
}

/// Adds an entry to the skip list at the given path.
///
/// Returns `Ok(true)` if the entry was added, `Ok(false)` if it was a duplicate.
///
/// # Errors
///
/// Returns an error if the skip list file cannot be read or written.
pub fn add_to(md_basename: &str, category: &str, description: &str, path: &str) -> Result<bool> {
    let key = stable_key(md_basename, category, description);
    let existing = parse_entries(path);

    if existing.iter().any(|e| e.key == key) {
        return Ok(false);
    }

    let entry = SkipListEntry {
        md_basename: md_basename.to_string(),
        category: category.to_string(),
        description: description.to_string(),
        key,
        accepted: now_timestamp(),
        reason: DEFAULT_REASON.to_string(),
    };

    append_entry(path, &entry)?;
    Ok(true)
}

/// Adds an entry to the skip list.
///
/// Returns `Ok(true)` if the entry was added, `Ok(false)` if it was a duplicate.
///
/// # Errors
///
/// Returns an error if the skip list file cannot be read or written.
pub fn add(md_basename: &str, category: &str, description: &str) -> Result<bool> {
    let path = resolve_skiplist_path();
    add_to(md_basename, category, description, &path)
}

/// Checks whether an entry exists in the skip list at the given path.
///
/// # Errors
///
/// Returns an error if the skip list file cannot be read.
pub fn check_in(md_basename: &str, category: &str, description: &str, path: &str) -> Result<bool> {
    let key = stable_key(md_basename, category, description);
    let existing = parse_entries(path);
    Ok(existing.iter().any(|e| e.key == key))
}

/// Checks whether an entry exists in the skip list.
///
/// # Errors
///
/// Returns an error if the skip list file cannot be read.
pub fn check(md_basename: &str, category: &str, description: &str) -> Result<bool> {
    let path = resolve_skiplist_path();
    check_in(md_basename, category, description, &path)
}

/// Lists all skip list entries for the given Markdown basename from the given path.
///
/// # Errors
///
/// Returns an error if the skip list file cannot be read.
pub fn list_from(md_basename: &str, path: &str) -> Result<Vec<SkipListEntry>> {
    let all = parse_entries(path);
    Ok(all
        .into_iter()
        .filter(|e| e.md_basename == md_basename)
        .collect())
}

/// Lists all skip list entries for the given Markdown basename.
///
/// # Errors
///
/// Returns an error if the skip list file cannot be read.
pub fn list(md_basename: &str) -> Result<Vec<SkipListEntry>> {
    let path = resolve_skiplist_path();
    list_from(md_basename, &path)
}
