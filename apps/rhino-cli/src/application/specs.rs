//! Spec-tree validators for OSE Platform spec conventions.
//!
//! Ports `apps/rhino-cli/cmd/specs_validate_*.go`. Each public function
//! validates one aspect of the spec-tree structure and returns a
//! (potentially empty) list of [`SpecFinding`] values.

use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// A single validation finding produced by one of the `validate_spec_*`
/// functions.
#[derive(Debug, Clone)]
pub struct SpecFinding {
    /// Validation category (e.g. `"adoption"`, `"count"`, `"links"`,
    /// `"tree-shape"`).
    pub category: String,
    /// Severity level: `"HIGH"`, `"MEDIUM"`, or `"LOW"`.
    pub criticality: String,
    /// Repo-relative path to the offending file or directory.
    pub file: String,
    /// Human-readable description of what was found.
    pub evidence: String,
    /// Suggested remediation step.
    pub expected: String,
}

/// Returns the ordered list of subfolder names that every spec tree is required
/// to contain.
pub fn required_spec_folders() -> &'static [&'static str] {
    &[
        "product",
        "system-context",
        "containers",
        "components",
        "behavior",
    ]
}

/// Recursively walks `dir` and returns all `.feature` files in sorted order.
///
/// Returns an empty `Vec` if `dir` cannot be read.
pub fn walk_feature_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    let mut items: Vec<std::path::PathBuf> = entries.flatten().map(|e| e.path()).collect();
    items.sort();
    for p in items {
        if p.is_dir() {
            out.extend(walk_feature_files(&p));
            continue;
        }
        if p.file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.to_lowercase().ends_with(".feature"))
        {
            out.push(p);
        }
    }
    out
}

/// Recursively walks `dir` and returns all `.md` files in sorted order.
///
/// Returns an empty `Vec` if `dir` cannot be read.
pub fn walk_md_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    let mut items: Vec<std::path::PathBuf> = entries.flatten().map(|e| e.path()).collect();
    items.sort();
    for p in items {
        if p.is_dir() {
            out.extend(walk_md_files(&p));
            continue;
        }
        if p.file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.to_lowercase().ends_with(".md"))
        {
            out.push(p);
        }
    }
    out
}

/// Counts `.feature` files and non-`README.md` `.md` files under `dir`
/// recursively.
///
/// `README.md` (case-insensitive) is excluded from the count because it is
/// the required index file for each folder and does not constitute a spec.
pub fn count_non_readme_md_files(dir: &Path) -> usize {
    let mut count = 0;
    for entry in WalkDir::new(dir).into_iter().flatten() {
        if entry.file_type().is_dir() {
            continue;
        }
        let n = entry.file_name().to_string_lossy().into_owned();
        let lower = n.to_lowercase();
        let is_feature = lower.ends_with(".feature");
        let is_non_readme_md = lower.ends_with(".md") && !n.eq_ignore_ascii_case("README.md");
        if is_feature || is_non_readme_md {
            count += 1;
        }
    }
    count
}

// ---- validate-adoption ----

/// Checks that `app` has adopted the OSE spec conventions by verifying that:
///
/// - `specs/apps/<app>/behavior/` exists and contains at least one `.feature`
///   file.
/// - `specs/apps/<app>/ddd/bounded-contexts.yaml` exists.
///
/// Returns a [`SpecFinding`] with `criticality = "HIGH"` for each violation.
pub fn validate_spec_adoption(repo_root: &Path, app: &str) -> Vec<SpecFinding> {
    let mut findings = Vec::new();
    let base = repo_root.join("specs/apps").join(app);
    let behavior_dir = base.join("behavior");
    if !behavior_dir.exists() {
        findings.push(SpecFinding {
            category: "adoption".into(),
            criticality: "HIGH".into(),
            file: format!("specs/apps/{app}/behavior"),
            evidence: format!(
                "no feature files found under specs/apps/{app}/behavior/ (directory does not exist)"
            ),
            expected: format!("create specs/apps/{app}/behavior/ with at least one .feature file"),
        });
    } else if walk_feature_files(&behavior_dir).is_empty() {
        findings.push(SpecFinding {
            category: "adoption".into(),
            criticality: "HIGH".into(),
            file: format!("specs/apps/{app}/behavior"),
            evidence: format!("no feature files found under specs/apps/{app}/behavior/"),
            expected: format!("add at least one .feature file under specs/apps/{app}/behavior/"),
        });
    }
    let bc_yaml = base.join("ddd/bounded-contexts.yaml");
    if !bc_yaml.exists() {
        findings.push(SpecFinding {
            category: "adoption".into(),
            criticality: "HIGH".into(),
            file: format!("specs/apps/{app}/ddd"),
            evidence: format!(
                "missing bounded-contexts.yaml at specs/apps/{app}/ddd/bounded-contexts.yaml"
            ),
            expected: format!("create specs/apps/{app}/ddd/bounded-contexts.yaml"),
        });
    }
    findings
}

/// Config-aware adoption validator: requires `ddd/` only when `is_ddd_area` is true, and
/// flags an unexpected `ddd/` directory when the area is NOT a ddd-area.
pub fn validate_spec_adoption_ddd_aware(
    repo_root: &Path,
    app: &str,
    is_ddd_area: bool,
) -> Vec<SpecFinding> {
    let mut findings = Vec::new();
    let base = repo_root.join("specs/apps").join(app);
    let behavior_dir = base.join("behavior");
    if !behavior_dir.exists() {
        findings.push(SpecFinding {
            category: "adoption".into(),
            criticality: "HIGH".into(),
            file: format!("specs/apps/{app}/behavior"),
            evidence: format!(
                "no feature files found under specs/apps/{app}/behavior/ (directory does not exist)"
            ),
            expected: format!("create specs/apps/{app}/behavior/ with at least one .feature file"),
        });
    } else if walk_feature_files(&behavior_dir).is_empty() {
        findings.push(SpecFinding {
            category: "adoption".into(),
            criticality: "HIGH".into(),
            file: format!("specs/apps/{app}/behavior"),
            evidence: format!("no feature files found under specs/apps/{app}/behavior/"),
            expected: format!("add at least one .feature file under specs/apps/{app}/behavior/"),
        });
    }
    let ddd_dir = base.join("ddd");
    let bc_yaml = ddd_dir.join("bounded-contexts.yaml");
    if is_ddd_area {
        if !bc_yaml.exists() {
            findings.push(SpecFinding {
                category: "adoption".into(),
                criticality: "HIGH".into(),
                file: format!("specs/apps/{app}/ddd"),
                evidence: format!(
                    "missing bounded-contexts.yaml at specs/apps/{app}/ddd/bounded-contexts.yaml"
                ),
                expected: format!("create specs/apps/{app}/ddd/bounded-contexts.yaml"),
            });
        }
    } else if ddd_dir.exists() {
        findings.push(SpecFinding {
            category: "adoption".into(),
            criticality: "HIGH".into(),
            file: format!("specs/apps/{app}/ddd"),
            evidence: format!(
                "unexpected ddd/ at specs/apps/{app}/ddd — area not listed in specs.ddd-areas"
            ),
            expected: format!(
                "remove specs/apps/{app}/ddd/ or add {app} to specs.ddd-areas in repo-config.yml"
            ),
        });
    }
    findings
}

// ---- validate-counts ----

/// Checks that `folder` (resolved against `repo_root` if relative) exists and
/// that each of the [`required_spec_folders`] is present and non-empty.
///
/// Returns a `"HIGH"` finding if the root folder or a required subfolder is
/// missing, and a `"MEDIUM"` finding if a subfolder exists but contains no
/// non-`README.md` spec files.
pub fn validate_spec_counts(repo_root: &Path, folder: &str) -> Vec<SpecFinding> {
    let mut findings = Vec::new();
    let abs = if Path::new(folder).is_absolute() {
        PathBuf::from(folder)
    } else {
        repo_root.join(folder)
    };
    if !abs.exists() {
        findings.push(SpecFinding {
            category: "count".into(),
            criticality: "HIGH".into(),
            file: folder.to_string(),
            evidence: format!("spec folder does not exist: {folder}"),
            expected: "create the spec folder with required subfolders".into(),
        });
        return findings;
    }
    for sub in required_spec_folders() {
        let sub_path = abs.join(sub);
        let rel = format!("{folder}/{sub}");
        if !sub_path.exists() {
            findings.push(SpecFinding {
                category: "count".into(),
                criticality: "HIGH".into(),
                file: rel.clone(),
                evidence: format!("missing required folder: {sub}"),
                expected: format!("create {rel}/README.md plus at least one spec .md file"),
            });
            continue;
        }
        let n = count_non_readme_md_files(&sub_path);
        if n == 0 {
            findings.push(SpecFinding {
                category: "count".into(),
                criticality: "MEDIUM".into(),
                file: rel.clone(),
                evidence: format!(
                    "empty subfolder: {sub} contains no spec files (only README.md or nothing)"
                ),
                expected: format!("add at least one non-README .md spec file to {rel}/"),
            });
        }
    }
    findings
}

// ---- validate-links ----

/// Returns the lazily-compiled regex that matches a Markdown link of the form
/// `[text](target)`.
/// Returns `path` relative to `base` by stripping the `base` prefix from the
/// string representation.
///
/// Returns the full string of `path` unchanged if it does not start with
/// `base`.
fn pathdiff_starts_with(path: &Path, base: &Path) -> String {
    let p = path.to_string_lossy().to_string();
    let b = base.to_string_lossy().to_string();
    if let Some(rest) = p.strip_prefix(&b) {
        rest.trim_start_matches('/').to_string()
    } else {
        p
    }
}

// ---- validate-tree ----

/// Checks that the spec-tree for `app` has all required top-level subfolders
/// and that each subfolder contains a `README.md`.
///
/// Returns a `"HIGH"` finding for each missing folder and each missing
/// `README.md`.
pub fn validate_spec_tree(repo_root: &Path, app: &str) -> Vec<SpecFinding> {
    let mut findings = Vec::new();
    let base = repo_root.join("specs/apps").join(app);
    for folder in required_spec_folders() {
        let folder_path = base.join(folder);
        if !folder_path.exists() {
            findings.push(SpecFinding {
                category: "tree-shape".into(),
                criticality: "HIGH".into(),
                file: format!("specs/apps/{app}"),
                evidence: format!("missing required folder: {folder}"),
                expected: format!("create specs/apps/{app}/{folder}/ with README.md"),
            });
            continue;
        }
        let readme = folder_path.join("README.md");
        if !readme.exists() {
            findings.push(SpecFinding {
                category: "tree-shape".into(),
                criticality: "HIGH".into(),
                file: format!("specs/apps/{app}/{folder}"),
                evidence: format!("missing README.md in required folder: {folder}"),
                expected: format!("create specs/apps/{app}/{folder}/README.md"),
            });
        }
    }
    findings
}

// ---- validate-gherkin-domains ----

/// Checks that every `.feature` file under
/// `behavior/<surface>/gherkin/` lives inside a domain subdirectory rather
/// than directly at the `gherkin/` root.
///
/// Flat `.feature` files at the `gherkin/` level are reported as `"HIGH"`
/// findings because they violate the required layout:
/// `behavior/<surface>/gherkin/<domain>/<feature>.feature`.
pub fn validate_spec_gherkin_domains(repo_root: &Path, app: &str) -> Vec<SpecFinding> {
    let mut findings = Vec::new();
    let behavior = repo_root.join("specs/apps").join(app).join("behavior");
    if !behavior.exists() {
        return findings;
    }
    let Ok(surfaces) = fs::read_dir(&behavior) else {
        return findings;
    };
    for surface_entry in surfaces.flatten() {
        let surface_path = surface_entry.path();
        if !surface_path.is_dir() {
            continue;
        }
        let gherkin = surface_path.join("gherkin");
        if !gherkin.exists() {
            continue;
        }
        let Ok(gherkin_entries) = fs::read_dir(&gherkin) else {
            continue;
        };
        for entry in gherkin_entries.flatten() {
            let p = entry.path();
            if p.is_file()
                && p.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.to_lowercase().ends_with(".feature"))
            {
                let rel = pathdiff_starts_with(&p, repo_root);
                findings.push(SpecFinding {
                    category: "tree-shape".into(),
                    criticality: "HIGH".into(),
                    file: rel.clone(),
                    evidence: format!(
                        "flat feature file at {rel}; expected behavior/<surface>/gherkin/<domain>/<feature>.feature"
                    ),
                    expected: format!(
                        "move {rel} into a domain subdirectory under the gherkin/ folder"
                    ),
                });
            }
        }
    }
    findings
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn touch(p: &Path) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, "x").unwrap();
    }

    #[test]
    fn required_folders_5() {
        assert_eq!(required_spec_folders().len(), 5);
    }

    #[test]
    fn walk_feature_files_finds_nested() {
        let dir = tempdir().unwrap();
        touch(&dir.path().join("a/b.feature"));
        touch(&dir.path().join("c.feature"));
        touch(&dir.path().join("d.md"));
        let r = walk_feature_files(dir.path());
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn walk_md_files_finds_nested() {
        let dir = tempdir().unwrap();
        touch(&dir.path().join("a/b.md"));
        touch(&dir.path().join("c.md"));
        touch(&dir.path().join("d.feature"));
        let r = walk_md_files(dir.path());
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn count_non_readme_md_files_includes_features() {
        let dir = tempdir().unwrap();
        touch(&dir.path().join("README.md"));
        touch(&dir.path().join("a.md"));
        touch(&dir.path().join("b.feature"));
        assert_eq!(count_non_readme_md_files(dir.path()), 2);
    }

    #[test]
    fn validate_spec_adoption_missing_all() {
        let dir = tempdir().unwrap();
        let f = validate_spec_adoption(dir.path(), "missing");
        // expect 2 findings (behavior missing + bc-yaml missing)
        assert_eq!(f.len(), 2);
    }

    #[test]
    fn validate_spec_adoption_empty_behavior() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x/behavior")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x/ddd")).unwrap();
        std::fs::write(
            dir.path().join("specs/apps/x/ddd/bounded-contexts.yaml"),
            "y",
        )
        .unwrap();
        let f = validate_spec_adoption(dir.path(), "x");
        assert_eq!(f.len(), 1);
        assert!(f[0].evidence.contains("no feature files"));
    }

    #[test]
    fn validate_spec_adoption_clean() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x/behavior")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x/ddd")).unwrap();
        std::fs::write(dir.path().join("specs/apps/x/behavior/a.feature"), "x").unwrap();
        std::fs::write(
            dir.path().join("specs/apps/x/ddd/bounded-contexts.yaml"),
            "y",
        )
        .unwrap();
        let f = validate_spec_adoption(dir.path(), "x");
        assert!(f.is_empty());
    }

    #[test]
    fn validate_spec_counts_missing_folder() {
        let dir = tempdir().unwrap();
        let f = validate_spec_counts(dir.path(), "specs/apps/x");
        assert_eq!(f.len(), 1);
        assert!(f[0].evidence.contains("does not exist"));
    }

    #[test]
    fn validate_spec_counts_reports_each_missing() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x")).unwrap();
        let f = validate_spec_counts(dir.path(), "specs/apps/x");
        assert_eq!(f.len(), 5); // all five required folders missing
    }

    #[test]
    fn validate_spec_counts_empty_subfolder_medium() {
        let dir = tempdir().unwrap();
        for sub in required_spec_folders() {
            std::fs::create_dir_all(dir.path().join("specs/apps/x").join(sub)).unwrap();
            std::fs::write(
                dir.path().join("specs/apps/x").join(sub).join("README.md"),
                "x",
            )
            .unwrap();
        }
        let f = validate_spec_counts(dir.path(), "specs/apps/x");
        assert_eq!(f.len(), 5);
        assert!(f.iter().all(|x| x.criticality == "MEDIUM"));
    }

    #[test]
    fn validate_spec_tree_missing() {
        let dir = tempdir().unwrap();
        let f = validate_spec_tree(dir.path(), "x");
        assert_eq!(f.len(), 5);
    }

    #[test]
    fn validate_spec_tree_complete() {
        let dir = tempdir().unwrap();
        for folder in required_spec_folders() {
            let p = dir.path().join("specs/apps/x").join(folder);
            std::fs::create_dir_all(&p).unwrap();
            std::fs::write(p.join("README.md"), "x").unwrap();
        }
        assert!(validate_spec_tree(dir.path(), "x").is_empty());
    }

    #[test]
    fn validate_spec_tree_build_tools_surface_accepted() {
        // Regression: behavior/build-tools/gherkin/ is a valid surface — validator must not
        // reject unknown surface names under behavior/.
        let dir = tempdir().unwrap();
        for folder in required_spec_folders() {
            let p = dir.path().join("specs/apps/x").join(folder);
            std::fs::create_dir_all(&p).unwrap();
            std::fs::write(p.join("README.md"), "x").unwrap();
        }
        let gherkin = dir.path().join("specs/apps/x/behavior/build-tools/gherkin");
        std::fs::create_dir_all(&gherkin).unwrap();
        std::fs::write(gherkin.join("build-tools.feature"), "Feature: build-tools").unwrap();
        assert!(validate_spec_tree(dir.path(), "x").is_empty());
    }

    #[test]
    fn validate_spec_gherkin_domains_flat_feature_rejected() {
        let dir = tempdir().unwrap();
        let gherkin = dir.path().join("specs/apps/x/behavior/cli/gherkin");
        std::fs::create_dir_all(&gherkin).unwrap();
        std::fs::write(gherkin.join("flat.feature"), "Feature: flat").unwrap();
        let f = validate_spec_gherkin_domains(dir.path(), "x");
        assert!(
            !f.is_empty(),
            "expected HIGH finding for flat feature at gherkin root"
        );
        assert!(f.iter().any(|x| x.criticality == "HIGH"));
    }

    #[test]
    fn validate_spec_gherkin_domains_domain_subdir_accepted() {
        let dir = tempdir().unwrap();
        let domain = dir.path().join("specs/apps/x/behavior/cli/gherkin/links");
        std::fs::create_dir_all(&domain).unwrap();
        std::fs::write(domain.join("links-check.feature"), "Feature: links").unwrap();
        assert!(validate_spec_gherkin_domains(dir.path(), "x").is_empty());
    }
}
