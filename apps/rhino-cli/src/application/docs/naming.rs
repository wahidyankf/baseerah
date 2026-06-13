//! Markdown filename convention validator.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/docs/naming.go`.
//!
//! Enforces the lowercase-kebab-case filename rule (`^[a-z0-9-]+\.md$`) for
//! every `.md` file found under the supplied root paths.  `README.md` is
//! always exempt, and callers may supply additional glob patterns to exclude
//! further filenames.

use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Error, anyhow};
use glob::Pattern;
use regex::Regex;
use walkdir::WalkDir;

/// A single naming-convention finding for a markdown file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsNamingFinding {
    /// Path to the file whose name violates the convention.
    pub file: String,
    /// Severity string (currently always `"high"`).
    pub severity: String,
    /// Human-readable description of the violation including the suggested fix.
    pub message: String,
}

/// Returns the compiled regex that accepts valid lowercase-kebab-case filenames.
fn kebab_case_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z0-9-]+\.md$").expect("valid hardcoded regex"))
}

/// Directory names that are skipped during recursive walks (shared with
/// [`super::frontmatter`] and [`super::heading_hierarchy`]).
pub const SKIP_DIRS: &[&str] = &["node_modules", ".git", ".next", "dist", "build", "target"];

/// Validates the filename convention for every `.md` file reachable from `paths`.
///
/// `exempt_globs` is a list of glob patterns (matched against the bare filename,
/// not the full path).  Files whose name matches any pattern are silently skipped.
/// `README.md` is always exempt regardless of `exempt_globs`.
///
/// Findings are sorted by file path.
///
/// # Errors
///
/// Returns an error when:
/// - `paths` is empty.
/// - Any pattern in `exempt_globs` is not a valid glob expression.
pub fn validate_docs_naming(
    paths: &[String],
    exempt_globs: &[String],
) -> std::result::Result<Vec<DocsNamingFinding>, Error> {
    if paths.is_empty() {
        return Err(anyhow!("at least one path is required"));
    }
    for pat in exempt_globs {
        if Pattern::new(pat).is_err() {
            return Err(anyhow!("invalid exempt glob \"{pat}\""));
        }
    }
    let mut findings = Vec::new();
    for root in paths {
        findings.extend(walk_naming_path(root, exempt_globs));
    }
    findings.sort_by(|a, b| a.file.cmp(&b.file));
    Ok(findings)
}

/// Walks `root` recursively and collects naming findings for non-compliant files.
///
/// Returns an empty list if `root` does not exist on the filesystem.
fn walk_naming_path(root: &str, exempt_globs: &[String]) -> Vec<DocsNamingFinding> {
    let root_p = Path::new(root);
    if !root_p.exists() {
        return Vec::new();
    }
    let mut findings = Vec::new();
    let walker = WalkDir::new(root_p).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy().to_string();
            !SKIP_DIRS.contains(&name.as_str())
        } else {
            true
        }
    });
    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let base = entry.file_name().to_string_lossy().to_string();
        if !base.ends_with(".md") {
            continue;
        }
        if is_naming_exempt(&base, exempt_globs) {
            continue;
        }
        if !kebab_case_re().is_match(&base) {
            findings.push(DocsNamingFinding {
                file: entry.path().to_string_lossy().to_string(),
                severity: "high".to_string(),
                message: format!(
                    "filename \"{base}\" violates lowercase-kebab-case rule (^[a-z0-9-]+\\.md$); rename to lowercase-kebab-case or add an exemption"
                ),
            });
        }
    }
    findings
}

/// Returns `true` when `basename` should be skipped during naming validation.
///
/// `README.md` is always exempt.  Additional exemptions are matched via `exempt_globs`.
fn is_naming_exempt(basename: &str, exempt_globs: &[String]) -> bool {
    if basename == "README.md" {
        return true;
    }
    for pat in exempt_globs {
        let matched = Pattern::new(pat).is_ok_and(|p| p.matches(basename));
        if matched {
            return true;
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

    /// Verifies that an empty paths slice returns an error.
    #[test]
    fn errors_on_empty_paths() {
        let err = validate_docs_naming(&[], &[]).unwrap_err();
        assert!(err.to_string().contains("at least one path"));
    }

    /// Verifies that a file with an uppercase letter in its basename is reported.
    #[test]
    fn detects_uppercase_basename() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("FooBar.md"), "x").unwrap();
        let findings =
            validate_docs_naming(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("lowercase-kebab-case"));
    }

    /// Verifies that a filename with underscores is reported as a violation.
    #[test]
    fn detects_underscore_basename() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("foo_bar.md"), "x").unwrap();
        let findings =
            validate_docs_naming(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert_eq!(findings.len(), 1);
    }

    /// Verifies that `README.md` is always exempt from the naming convention.
    #[test]
    fn readme_always_exempt() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "x").unwrap();
        let findings =
            validate_docs_naming(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that a file matching an exempt glob is not reported.
    #[test]
    fn exempt_glob_excludes_match() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("AGENTS.md"), "x").unwrap();
        let findings = validate_docs_naming(
            &[tmp.path().to_string_lossy().to_string()],
            &["AGENTS.md".to_string()],
        )
        .unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that `node_modules/` directories are skipped during the walk.
    #[test]
    fn skips_node_modules() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join("node_modules")).unwrap();
        fs::write(tmp.path().join("node_modules/Bad.md"), "x").unwrap();
        let findings =
            validate_docs_naming(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that a valid lowercase-kebab-case filename produces no finding.
    #[test]
    fn passes_valid_kebab_basename() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("foo-bar-baz.md"), "x").unwrap();
        let findings =
            validate_docs_naming(&[tmp.path().to_string_lossy().to_string()], &[]).unwrap();
        assert!(findings.is_empty());
    }

    /// Verifies that an invalid glob pattern in `exempt_globs` returns an error.
    #[test]
    fn invalid_glob_errors() {
        let err = validate_docs_naming(&["x".to_string()], &["[unclosed".to_string()]).unwrap_err();
        assert!(err.to_string().contains("invalid exempt glob"));
    }
}
