// Specs validators ported from `apps/rhino-cli/cmd/specs_validate_*.go`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct SpecFinding {
    pub category: String,
    pub criticality: String,
    pub file: String,
    pub evidence: String,
    pub expected: String,
}

pub fn required_spec_folders() -> &'static [&'static str] {
    &[
        "product",
        "system-context",
        "containers",
        "components",
        "behavior",
    ]
}

pub fn walk_feature_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return out,
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
            .map(|s| s.to_lowercase().ends_with(".feature"))
            .unwrap_or(false)
        {
            out.push(p);
        }
    }
    out
}

pub fn walk_md_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return out,
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
            .map(|s| s.to_lowercase().ends_with(".md"))
            .unwrap_or(false)
        {
            out.push(p);
        }
    }
    out
}

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

// ---- validate-counts ----

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

fn markdown_link_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap())
}

pub fn validate_spec_links(repo_root: &Path, folder: &str) -> Vec<SpecFinding> {
    let mut findings = Vec::new();
    let abs = if Path::new(folder).is_absolute() {
        PathBuf::from(folder)
    } else {
        repo_root.join(folder)
    };
    if !abs.exists() {
        findings.push(SpecFinding {
            category: "links".into(),
            criticality: "HIGH".into(),
            file: folder.to_string(),
            evidence: format!("spec folder does not exist: {folder}"),
            expected: "create the spec folder".into(),
        });
        return findings;
    }
    let md_files = walk_md_files(&abs);
    let re = markdown_link_re();
    for md in &md_files {
        let content = match fs::read(md) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let s = String::from_utf8_lossy(&content);
        for cap in re.captures_iter(&s) {
            let target_full = cap[2].to_string();
            let target = match target_full.find('#') {
                Some(i) => target_full[..i].to_string(),
                None => target_full,
            };
            if target.is_empty() {
                continue;
            }
            if target.starts_with("http://") || target.starts_with("https://") {
                continue;
            }
            let resolved = md.parent().unwrap_or(repo_root).join(&target);
            if !resolved.exists() {
                let rel = pathdiff_starts_with(md, repo_root);
                let base = md
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                findings.push(SpecFinding {
                    category: "links".into(),
                    criticality: "HIGH".into(),
                    file: rel,
                    evidence: format!("broken link: {base} -> {target} (file not found)"),
                    expected: format!("fix or remove the link to {target}"),
                });
            }
        }
    }
    findings
}

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

#[cfg(test)]
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
    fn validate_spec_links_broken() {
        let dir = tempdir().unwrap();
        let folder = dir.path().join("specs/apps/x");
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(
            folder.join("a.md"),
            "[bad](./missing.md)\n[good](./other.md)\n",
        )
        .unwrap();
        std::fs::write(folder.join("other.md"), "x").unwrap();
        let f = validate_spec_links(dir.path(), "specs/apps/x");
        assert_eq!(f.len(), 1);
        assert!(f[0].evidence.contains("broken link"));
    }

    #[test]
    fn validate_spec_links_ignores_external() {
        let dir = tempdir().unwrap();
        let folder = dir.path().join("specs/apps/x");
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("a.md"), "[ok](https://example.com)\n").unwrap();
        assert!(validate_spec_links(dir.path(), "specs/apps/x").is_empty());
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
}
