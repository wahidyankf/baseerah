//! Java import cleaning for generated contracts.
//!
//! Removes same-package imports, unused imports (class name not referenced in the file
//! body), and duplicate import lines. Files are rewritten atomically (temp file + rename)
//! only when changes are detected.

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Error};
use walkdir::WalkDir;

use super::types::{JavaCleanImportsOptions, JavaCleanImportsResult};

/// Removes unused and same-package imports from all `.java` files in `opts.dir`.
///
///: walks the directory in lexical order, and for
/// each `.java` file applies the two-pass [`process_java_file`] transform.
///
/// # Errors
///
/// Returns an error when the target directory cannot be walked or a `.java`
/// file cannot be read or rewritten.
pub fn clean_java_imports(opts: &JavaCleanImportsOptions) -> Result<JavaCleanImportsResult, Error> {
    let mut result = JavaCleanImportsResult {
        total_files: 0,
        modified_files: 0,
        modified: Vec::new(),
    };

    for entry in WalkDir::new(&opts.dir).sort_by_file_name() {
        let entry = entry.with_context(|| format!("walking directory {}", opts.dir))?;
        let path = entry.path();

        // Go: d.IsDir() || !strings.HasSuffix(path, ".java") → skip.
        if entry.file_type().is_dir() || !path.to_string_lossy().ends_with(".java") {
            continue;
        }

        result.total_files += 1;

        let modified = process_java_file(path)?;
        if modified {
            result.modified_files += 1;
            let rel = rel_path(&opts.dir, &path.to_string_lossy());
            result.modified.push(rel);
        }
    }

    Ok(result)
}

/// Processes a single `.java` file, returning `true` when it was modified.
/// Two-pass: pass 1 gathers the package name and body (non-import lines); pass 2
/// filters imports, dropping same-package, unused, and duplicate imports.
fn process_java_file(path: &Path) -> Result<bool, Error> {
    let data = std::fs::read(path).with_context(|| format!("reading file {}", path.display()))?;
    let original = String::from_utf8_lossy(&data).into_owned();
    let lines: Vec<&str> = original.split('\n').collect();

    // Pass 1: gather context — package name and body text (non-import lines joined).
    let mut pkg_name = String::new();
    let mut body_parts: Vec<&str> = Vec::new();

    for line in &lines {
        if let Some(pkg) = line.strip_prefix("package ") {
            let pkg = pkg.strip_suffix(';').unwrap_or(pkg);
            pkg_name = pkg.trim().to_string();
        } else if !line.starts_with("import ") {
            body_parts.push(line);
        }
    }

    let body = body_parts.join("\n");

    // Pass 2: filter imports.
    let mut seen: HashSet<&str> = HashSet::new();
    let mut kept: Vec<&str> = Vec::new();

    for line in &lines {
        if !line.starts_with("import ") {
            kept.push(line);
            continue;
        }

        // Strip the import statement down to the fully-qualified name.
        let mut fqn = line.strip_prefix("import ").unwrap_or(line);
        fqn = fqn.strip_suffix(';').unwrap_or(fqn);
        fqn = fqn.trim();

        // Handle static imports.
        fqn = fqn.strip_prefix("static ").unwrap_or(fqn);
        fqn = fqn.trim();

        // Split into package + class name.
        let parts: Vec<&str> = fqn.split('.').collect();
        if parts.len() < 2 {
            // Malformed import — keep as-is.
            kept.push(line);
            continue;
        }

        let class_name = parts[parts.len() - 1];
        let import_pkg = parts[..parts.len() - 1].join(".");

        // Skip same-package imports.
        if import_pkg == pkg_name {
            continue;
        }

        // Skip if class name not used in body.
        if !body.contains(class_name) {
            continue;
        }

        // Skip duplicates.
        if seen.contains(line) {
            continue;
        }

        seen.insert(line);
        kept.push(line);
    }

    let cleaned = format!("{}\n", trim_end_newlines(&kept.join("\n")));
    let original_norm = format!("{}\n", trim_end_newlines(&original));

    if cleaned == original_norm {
        return Ok(false);
    }

    // Write atomically via a temp file, matching Go's `path + ".tmp"` then rename.
    let tmp_path = format!("{}.tmp", path.to_string_lossy());
    std::fs::write(&tmp_path, cleaned.as_bytes())
        .with_context(|| format!("writing temp file {tmp_path}"))?;
    std::fs::rename(&tmp_path, path)
        .with_context(|| format!("renaming {} to {}", tmp_path, path.display()))?;

    Ok(true)
}

/// Trims trailing `\n` characters.
fn trim_end_newlines(s: &str) -> &str {
    s.trim_end_matches('\n')
}

/// Computes the relative path of `target` from `base`; on failure it falls
/// back to the absolute path.
fn rel_path(base: &str, target: &str) -> String {
    match Path::new(target).strip_prefix(base) {
        Ok(rel) => rel.to_string_lossy().into_owned(),
        Err(_) => target.to_string(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn opts(dir: &std::path::Path) -> JavaCleanImportsOptions {
        JavaCleanImportsOptions {
            dir: dir.to_string_lossy().into_owned(),
        }
    }

    #[test]
    fn removes_unused_imports() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("Foo.java");
        fs::write(
            &f,
            "package com.foo;\nimport java.util.List;\nimport java.util.Map;\n\nclass Foo { List x; }\n",
        )
        .unwrap();

        let r = clean_java_imports(&opts(tmp.path())).unwrap();
        assert_eq!(r.total_files, 1);
        assert_eq!(r.modified_files, 1);
        assert_eq!(r.modified, vec!["Foo.java".to_string()]);

        let after = fs::read_to_string(&f).unwrap();
        assert!(after.contains("import java.util.List;"));
        assert!(!after.contains("import java.util.Map;"));
    }

    #[test]
    fn removes_same_package_imports() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("Bar.java");
        fs::write(
            &f,
            "package com.foo;\nimport com.foo.Helper;\n\nclass Bar { Helper h; }\n",
        )
        .unwrap();

        let r = clean_java_imports(&opts(tmp.path())).unwrap();
        assert_eq!(r.modified_files, 1);
        let after = fs::read_to_string(&f).unwrap();
        assert!(!after.contains("import com.foo.Helper;"));
    }

    #[test]
    fn deduplicates_imports() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("Dup.java");
        fs::write(
            &f,
            "package com.foo;\nimport java.util.List;\nimport java.util.List;\n\nclass Dup { List x; }\n",
        )
        .unwrap();

        let r = clean_java_imports(&opts(tmp.path())).unwrap();
        assert_eq!(r.modified_files, 1);
        let after = fs::read_to_string(&f).unwrap();
        assert_eq!(after.matches("import java.util.List;").count(), 1);
    }

    #[test]
    fn unchanged_when_only_required_imports() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("Clean.java");
        let content = "package com.foo;\nimport java.util.List;\n\nclass Clean { List x; }\n";
        fs::write(&f, content).unwrap();

        let r = clean_java_imports(&opts(tmp.path())).unwrap();
        assert_eq!(r.total_files, 1);
        assert_eq!(r.modified_files, 0);
        assert!(r.modified.is_empty());
        assert_eq!(fs::read_to_string(&f).unwrap(), content);
    }

    #[test]
    fn empty_directory_no_errors() {
        let tmp = TempDir::new().unwrap();
        let r = clean_java_imports(&opts(tmp.path())).unwrap();
        assert_eq!(r.total_files, 0);
        assert_eq!(r.modified_files, 0);
    }

    #[test]
    fn malformed_import_kept() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("M.java");
        // "import foo;" has fewer than 2 dotted parts → kept as-is.
        fs::write(&f, "package com.foo;\nimport foo;\n\nclass M {}\n").unwrap();
        let r = clean_java_imports(&opts(tmp.path())).unwrap();
        assert_eq!(r.modified_files, 0);
        assert!(fs::read_to_string(&f).unwrap().contains("import foo;"));
    }

    #[test]
    fn static_import_handled() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("S.java");
        fs::write(
            &f,
            "package com.foo;\nimport static org.junit.Assert.assertEquals;\n\nclass S { void t() { assertEquals(1, 1); } }\n",
        )
        .unwrap();
        let r = clean_java_imports(&opts(tmp.path())).unwrap();
        // assertEquals IS used in body → kept → unchanged.
        assert_eq!(r.modified_files, 0);
    }
}
