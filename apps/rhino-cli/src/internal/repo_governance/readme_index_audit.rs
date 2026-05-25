//! Audit that every `README.md` links to all sibling `.md` files and
//! subdirectory `README.md` files, and has no ghost links.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/readme_index_audit.go`.

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Error, anyhow};
use glob::Pattern;
use regex::Regex;
use walkdir::WalkDir;

/// A single finding from the README index audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadmeIndexFinding {
    /// Absolute or relative path of the file implicated in the finding.
    pub file: String,
    /// Severity; currently always `"high"`.
    pub severity: String,
    /// Machine-readable violation category: `"orphan"` or `"ghost"`.
    pub kind: String,
    /// Human-readable description of the finding.
    pub message: String,
}

/// Returns a compiled `Regex` that captures the target of a Markdown link
/// whose href ends with `.md` (optionally with a fragment or query string).
fn readme_link_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"\[[^\]]+\]\(([^)]*\.md(?:[#?][^)]*)?)\)").expect("valid hardcoded regex")
    })
}

/// Directory names skipped during the recursive walk.
const SKIP_DIRS: &[&str] = &["node_modules", "target", "dist", "build", ".next", ".git"];

/// Audits every `README.md` found under each directory in `paths`.
///
/// For each `README.md`, sibling `.md` files and subdirectories (those that
/// contain their own `README.md`) must be linked from the `README.md`.
/// Unlinked siblings are reported as `"orphan"` findings; links to
/// non-existent targets are reported as `"ghost"` findings.
///
/// Paths and globs in `excludes` are skipped.  Findings are sorted by `file`,
/// then by `kind`.
///
/// # Errors
///
/// Returns an error when `paths` is empty or when any file cannot be read.
pub fn audit_readme_index(
    paths: &[String],
    excludes: &[String],
) -> std::result::Result<Vec<ReadmeIndexFinding>, Error> {
    if paths.is_empty() {
        return Err(anyhow!("at least one path is required"));
    }
    let mut findings = Vec::new();
    for root in paths {
        let readmes = find_readmes(root, excludes)?;
        for readme in &readmes {
            findings.extend(audit_one_readme(readme, root, excludes)?);
        }
    }
    findings.sort_by(|a, b| a.file.cmp(&b.file).then(a.kind.cmp(&b.kind)));
    Ok(findings)
}

/// Recursively finds all `README.md` files under `root`, respecting `excludes`
/// and [`SKIP_DIRS`].
///
/// Returns an empty `Vec` when `root` does not exist.
///
/// # Errors
///
/// Returns an error when the directory walk encounters an unrecoverable IO
/// error.
fn find_readmes(root: &str, excludes: &[String]) -> std::result::Result<Vec<String>, Error> {
    let root_p = Path::new(root);
    if !root_p.exists() {
        return Ok(Vec::new());
    }
    let mut readmes = Vec::new();
    let walker = WalkDir::new(root_p).into_iter().filter_entry(|e| {
        let path = e.path();
        if path == root_p {
            return true;
        }
        let name = e.file_name().to_string_lossy().to_string();
        if e.file_type().is_dir() {
            if name.starts_with('.') || SKIP_DIRS.contains(&name.as_str()) {
                return false;
            }
            if path
                .strip_prefix(root_p)
                .is_ok_and(|rel| matches_any_glob(&rel.to_string_lossy(), excludes))
            {
                return false;
            }
        }
        true
    });
    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.file_name() != "README.md" {
            continue;
        }
        let path = entry.path();
        let rel = match path.strip_prefix(root_p) {
            Ok(r) => r.to_string_lossy().to_string(),
            Err(_) => continue,
        };
        if matches_any_glob(&rel, excludes) {
            continue;
        }
        readmes.push(path.to_string_lossy().to_string());
    }
    readmes.sort();
    Ok(readmes)
}

/// Audits a single `README.md` at `readme_path` against the sibling targets
/// present under the same directory within `root`.
///
/// # Errors
///
/// Returns an error when `readme_path` has no parent component or when the
/// file or its sibling directory cannot be read.
fn audit_one_readme(
    readme_path: &str,
    root: &str,
    excludes: &[String],
) -> std::result::Result<Vec<ReadmeIndexFinding>, Error> {
    let dir = Path::new(readme_path)
        .parent()
        .ok_or_else(|| anyhow!("readme has no parent"))?;
    let data = fs::read_to_string(readme_path).with_context(|| format!("read {readme_path}"))?;
    let linked = extract_readme_links(&data);
    let actual = list_sibling_targets(dir, Path::new(root), excludes)?;

    let mut findings = Vec::new();

    // Orphans: file on disk but not in README.
    for name in actual.sorted_names() {
        if !linked.contains(&name) {
            let full = dir.join(&name);
            findings.push(ReadmeIndexFinding {
                file: full.to_string_lossy().to_string(),
                severity: "high".to_string(),
                kind: "orphan".to_string(),
                message: format!("orphan: {name} exists but is not linked from {readme_path}"),
            });
        }
    }

    // Ghosts: README links target absent from disk.
    let mut sorted_links: Vec<String> = linked.into_iter().collect();
    sorted_links.sort();
    for link in sorted_links {
        if !actual.present(&link) {
            let full = dir.join(&link);
            findings.push(ReadmeIndexFinding {
                file: full.to_string_lossy().to_string(),
                severity: "high".to_string(),
                kind: "ghost".to_string(),
                message: format!(
                    "ghost: {readme_path} references {link} but the target does not exist"
                ),
            });
        }
    }

    Ok(findings)
}

/// Extracts all relative `.md` link targets from `content`, stripping fragment
/// and query suffixes, leading `./`, and ignoring absolute paths, parent paths,
/// and URL-like hrefs.
fn extract_readme_links(content: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    for cap in readme_link_re().captures_iter(content) {
        let raw = cap[1].trim();
        if raw.is_empty() {
            continue;
        }
        let raw = raw.strip_prefix("./").unwrap_or(raw);
        let raw = match raw.find(['#', '?']) {
            Some(i) => &raw[..i],
            None => raw,
        };
        if raw.is_empty() || raw.starts_with('/') || raw.starts_with("..") {
            continue;
        }
        // Skip URLs: leading scheme followed by ":" before first "/".
        let url_like = match raw.find(':') {
            Some(colon) if colon > 0 => {
                let slash = raw.find('/');
                slash.is_none_or(|s| colon < s)
            }
            _ => false,
        };
        if url_like {
            continue;
        }
        out.insert(raw.replace('\\', "/"));
    }
    out
}

/// The set of linkable targets adjacent to a `README.md`.
struct SiblingTargets {
    /// Sibling `.md` files (excluding `README.md` itself).
    files: HashSet<String>,
    /// Subdirectory `README.md` paths relative to the parent directory.
    sub_dirs: HashSet<String>,
}

impl SiblingTargets {
    /// Creates an empty `SiblingTargets`.
    fn new() -> Self {
        Self {
            files: HashSet::new(),
            sub_dirs: HashSet::new(),
        }
    }

    /// Returns a sorted `Vec` of all linkable target names.
    fn sorted_names(&self) -> Vec<String> {
        let mut all: BTreeSet<String> = BTreeSet::new();
        all.extend(self.files.iter().cloned());
        all.extend(self.sub_dirs.iter().cloned());
        all.into_iter().collect()
    }

    /// Returns `true` when `link` refers to a file or subdirectory that exists
    /// on disk, including bare-directory links (e.g., `"structure"` resolves to
    /// `"structure/README.md"`).
    fn present(&self, link: &str) -> bool {
        let normalized = link.replace('\\', "/");
        let normalized = normalized.trim_end_matches('/').to_string();
        if self.files.contains(&normalized) {
            return true;
        }
        if self.sub_dirs.contains(&normalized) {
            return true;
        }
        // Allow bare-directory: "structure" → "structure/README.md".
        let bare = format!("{normalized}/README.md");
        if self.sub_dirs.contains(&bare) {
            return true;
        }
        false
    }
}

/// Lists the sibling `.md` files and subdirectories that contain a `README.md`
/// adjacent to a `README.md` at `dir`, relative to `root`.
///
/// Hidden entries and those in [`SKIP_DIRS`] are excluded.  Paths matching
/// `excludes` globs are also excluded.
///
/// # Errors
///
/// Returns an error when `dir` cannot be read.
fn list_sibling_targets(
    dir: &Path,
    root: &Path,
    excludes: &[String],
) -> std::result::Result<SiblingTargets, Error> {
    let mut out = SiblingTargets::new();
    let entries = fs::read_dir(dir).with_context(|| format!("read dir {}", dir.display()))?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let full = dir.join(&name);
        let rel = match full.strip_prefix(root) {
            Ok(r) => r.to_string_lossy().to_string(),
            Err(_) => name.clone(),
        };
        if matches_any_glob(&rel, excludes) {
            continue;
        }
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_dir() {
            if name.starts_with('.') || SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            let sub_readme = full.join("README.md");
            if sub_readme.exists() {
                out.sub_dirs
                    .insert(format!("{name}/README.md").replace('\\', "/"));
            }
            continue;
        }
        if name.starts_with('.') {
            continue;
        }
        if name == "README.md" {
            continue;
        }
        if !name.ends_with(".md") {
            continue;
        }
        out.files.insert(name);
    }
    Ok(out)
}

/// Returns `true` when `rel` matches at least one of the `patterns` using
/// `glob::Pattern`.
///
/// Matching is attempted against the full path, the basename, and each path
/// component.
fn matches_any_glob(rel: &str, patterns: &[String]) -> bool {
    if rel.is_empty() || rel == "." {
        return false;
    }
    let slashed = rel.replace('\\', "/");
    let components: Vec<&str> = slashed.split('/').collect();
    let basename = PathBuf::from(&slashed)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    for p in patterns {
        if p.is_empty() {
            continue;
        }
        let Ok(pat) = Pattern::new(p) else {
            continue;
        };
        if pat.matches(&slashed) {
            return true;
        }
        if pat.matches(&basename) {
            return true;
        }
        for c in &components {
            if pat.matches(c) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn errors_on_empty_paths() {
        let err = audit_readme_index(&[], &[]).unwrap_err();
        assert!(err.to_string().contains("at least one path"));
    }

    #[test]
    fn detects_orphan_md_file() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "# Title\n").unwrap();
        fs::write(tmp.path().join("other.md"), "x\n").unwrap();
        let findings =
            audit_readme_index(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert!(findings.iter().any(|f| f.kind == "orphan"));
    }

    #[test]
    fn detects_ghost_link() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "[ghost](nonexistent.md)\n").unwrap();
        let findings =
            audit_readme_index(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert!(findings.iter().any(|f| f.kind == "ghost"));
    }

    #[test]
    fn clean_when_all_linked() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "[other](other.md)\n").unwrap();
        fs::write(tmp.path().join("other.md"), "x\n").unwrap();
        let findings =
            audit_readme_index(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn subdir_readme_treated_as_target() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "[sub](structure/README.md)\n").unwrap();
        fs::create_dir(tmp.path().join("structure")).unwrap();
        fs::write(tmp.path().join("structure/README.md"), "# Sub\n").unwrap();
        let findings =
            audit_readme_index(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn bare_dir_link_recognized() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "[sub](structure)\n").unwrap();
        fs::create_dir(tmp.path().join("structure")).unwrap();
        fs::write(tmp.path().join("structure/README.md"), "x\n").unwrap();
        let findings =
            audit_readme_index(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        // No orphan/ghost because README links subdir as bare dir.
        assert!(findings.iter().all(|f| f.kind != "ghost"));
    }

    #[test]
    fn extract_links_strips_fragments() {
        let links = extract_readme_links("[a](foo.md#anchor) [b](bar.md?x=y)");
        assert!(links.contains("foo.md"));
        assert!(links.contains("bar.md"));
    }

    #[test]
    fn extract_links_skips_urls() {
        let links = extract_readme_links("[a](https://example.com/foo.md) [b](mailto:x.md)");
        assert!(links.is_empty());
    }

    #[test]
    fn extract_links_skips_parent_paths() {
        let links = extract_readme_links("[a](../foo.md) [b](/abs/foo.md)");
        assert!(links.is_empty());
    }

    #[test]
    fn matches_glob_basename_full_and_component() {
        assert!(matches_any_glob("foo/bar.md", &["*.md".to_string()]));
        assert!(matches_any_glob(
            "node_modules/foo",
            &["node_modules".to_string()]
        ));
        assert!(matches_any_glob("a/scratch/b.md", &["scratch".to_string()]));
        assert!(!matches_any_glob("foo/bar.md", &["*.txt".to_string()]));
    }

    #[test]
    fn excludes_filter_out_files() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "# x\n").unwrap();
        fs::write(tmp.path().join("scratch.tmp.md"), "x\n").unwrap();
        let findings = audit_readme_index(
            &[tmp.path().to_string_lossy().to_string()],
            &["*.tmp.md".to_string()],
        )
        .unwrap();
        assert!(findings.is_empty());
    }
}
