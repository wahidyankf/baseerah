//! Null-safety annotation validation.

use std::path::Path;

use anyhow::Error;

use super::scanner::scan_packages;
use super::types::{PackageEntry, ValidationOptions, ValidationResult, ViolationType};

/// Validates all Java packages in `opts.source_root` for the required
/// annotation. For each package directory it checks, in order:
///
///  1. `package-info.java` exists — [`ViolationType::MissingPackageInfo`] if absent.
///  2. `package-info.java` contains `@<annotation>` — [`ViolationType::MissingAnnotation`]
///     if missing.
///
/// # Errors
///
/// Returns an error when the source root cannot be scanned or a
/// `package-info.java` file cannot be read.
pub fn validate_all(opts: &ValidationOptions) -> Result<ValidationResult, Error> {
    let packages = scan_packages(&opts.source_root)?;

    let mut result = ValidationResult {
        total_packages: packages.len(),
        valid_packages: 0,
        all_packages: Vec::new(),
        annotation: opts.annotation.clone(),
    };

    let annotation_needle = format!("@{}", opts.annotation);

    for dir in &packages {
        let rel_dir = rel_path(&opts.source_root, dir);

        let pkg_info_path = Path::new(dir).join("package-info.java");

        match std::fs::read(&pkg_info_path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                result.all_packages.push(PackageEntry {
                    package_dir: rel_dir,
                    valid: false,
                    violation_type: Some(ViolationType::MissingPackageInfo),
                });
            }
            Err(e) => return Err(e.into()),
            Ok(content) => {
                if bytes_contains(&content, annotation_needle.as_bytes()) {
                    result.valid_packages += 1;
                    result.all_packages.push(PackageEntry {
                        package_dir: rel_dir,
                        valid: true,
                        violation_type: None,
                    });
                } else {
                    result.all_packages.push(PackageEntry {
                        package_dir: rel_dir,
                        valid: false,
                        violation_type: Some(ViolationType::MissingAnnotation),
                    });
                }
            }
        }
    }

    Ok(result)
}

/// Computes the relative path of `target` from `base`. On failure it returns
/// `target` verbatim (when stripping the prefix fails).
fn rel_path(base: &str, target: &str) -> String {
    match Path::new(target).strip_prefix(base) {
        Ok(rel) => {
            let s = rel.to_string_lossy();
            if s.is_empty() {
                // Go's filepath.Rel returns "." when target == base.
                ".".to_string()
            } else {
                s.into_owned()
            }
        }
        Err(_) => target.to_string(),
    }
}

/// True when `haystack` contains `needle` as a contiguous byte sub-slice,
///.
fn bytes_contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn opts(root: &std::path::Path, annotation: &str) -> ValidationOptions {
        ValidationOptions {
            source_root: root.to_string_lossy().into_owned(),
            annotation: annotation.to_string(),
        }
    }

    #[test]
    fn all_packages_valid() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("com/foo")).unwrap();
        fs::write(root.join("com/foo/Bar.java"), "package com.foo;").unwrap();
        fs::write(
            root.join("com/foo/package-info.java"),
            "@NullMarked\npackage com.foo;",
        )
        .unwrap();

        let r = validate_all(&opts(root, "NullMarked")).unwrap();
        assert_eq!(r.total_packages, 1);
        assert_eq!(r.valid_packages, 1);
        assert!(r.all_packages[0].valid);
        assert_eq!(r.all_packages[0].package_dir, "com/foo");
    }

    #[test]
    fn missing_package_info() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("com/foo")).unwrap();
        fs::write(root.join("com/foo/Bar.java"), "package com.foo;").unwrap();

        let r = validate_all(&opts(root, "NullMarked")).unwrap();
        assert_eq!(r.total_packages, 1);
        assert_eq!(r.valid_packages, 0);
        assert!(!r.all_packages[0].valid);
        assert_eq!(
            r.all_packages[0].violation_type,
            Some(ViolationType::MissingPackageInfo)
        );
    }

    #[test]
    fn missing_annotation() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("com/foo")).unwrap();
        fs::write(root.join("com/foo/Bar.java"), "package com.foo;").unwrap();
        fs::write(root.join("com/foo/package-info.java"), "package com.foo;").unwrap();

        let r = validate_all(&opts(root, "NullMarked")).unwrap();
        assert_eq!(r.valid_packages, 0);
        assert_eq!(
            r.all_packages[0].violation_type,
            Some(ViolationType::MissingAnnotation)
        );
    }

    #[test]
    fn custom_annotation_matches() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("com/foo")).unwrap();
        fs::write(root.join("com/foo/Bar.java"), "package com.foo;").unwrap();
        fs::write(
            root.join("com/foo/package-info.java"),
            "@NonNull\npackage com.foo;",
        )
        .unwrap();

        let r = validate_all(&opts(root, "NonNull")).unwrap();
        assert_eq!(r.valid_packages, 1);
    }

    #[test]
    fn bytes_contains_basic() {
        assert!(bytes_contains(b"hello world", b"world"));
        assert!(!bytes_contains(b"hello", b"xyz"));
        assert!(bytes_contains(b"abc", b""));
    }

    #[test]
    fn rel_path_strips_base() {
        assert_eq!(rel_path("/a/b", "/a/b/c"), "c");
        assert_eq!(rel_path("/a/b", "/a/b"), ".");
    }
}
