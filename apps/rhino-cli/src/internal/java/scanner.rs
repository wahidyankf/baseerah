//! Package discovery for the Java validator.

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Error;
use walkdir::WalkDir;

/// Walks `source_root` and returns absolute paths of directories that contain
/// at least one `.java` file (`package-info.java` counts). The returned vector
/// is sorted lexicographically, matching Go's `sort.Strings` on absolute paths.
///
///: it uses a set keyed on the parent directory of
/// each `.java` file, then sorts the resulting paths.
///
/// # Errors
///
/// Returns an error when `source_root` cannot be walked.
pub fn scan_packages(source_root: &str) -> Result<Vec<String>, Error> {
    let mut package_set: BTreeSet<String> = BTreeSet::new();

    for entry in WalkDir::new(source_root).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();
        // Go: !d.IsDir() && filepath.Ext(path) == ".java"
        if !entry.file_type().is_dir()
            && has_java_ext(path)
            && let Some(parent) = path.parent()
        {
            package_set.insert(parent.to_string_lossy().into_owned());
        }
    }

    // BTreeSet already yields lexicographically sorted unique entries, matching
    // Go's `sort.Strings` over the deduplicated directory paths.
    Ok(package_set.into_iter().collect())
}

/// Returns true when `path`'s extension is exactly `.java` (case-sensitive).
fn has_java_ext(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "java")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn collects_unique_sorted_package_dirs() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("a/b")).unwrap();
        fs::create_dir_all(root.join("a/c")).unwrap();
        fs::write(root.join("a/b/X.java"), "package a.b;").unwrap();
        fs::write(root.join("a/b/Y.java"), "package a.b;").unwrap();
        fs::write(root.join("a/c/Z.java"), "package a.c;").unwrap();
        fs::write(root.join("a/notes.txt"), "ignore me").unwrap();

        let pkgs = scan_packages(&root.to_string_lossy()).unwrap();
        let b = root.join("a/b").to_string_lossy().into_owned();
        let c = root.join("a/c").to_string_lossy().into_owned();
        assert_eq!(pkgs, vec![b, c]);
    }

    #[test]
    fn empty_tree_yields_no_packages() {
        let tmp = TempDir::new().unwrap();
        let pkgs = scan_packages(&tmp.path().to_string_lossy()).unwrap();
        assert!(pkgs.is_empty());
    }

    #[test]
    fn non_java_files_are_ignored() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("README.md"), "# hi").unwrap();
        fs::write(tmp.path().join("Foo.javax"), "nope").unwrap();
        let pkgs = scan_packages(&tmp.path().to_string_lossy()).unwrap();
        assert!(pkgs.is_empty());
    }
}
