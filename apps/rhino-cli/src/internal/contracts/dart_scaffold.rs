//! Dart package scaffolding for generated contracts.
//!
//! Writes `pubspec.yaml`, ensures `lib/` exists, globs `lib/model/*.dart`, and builds a
//! barrel library with `part` directives for each model file plus the shared utility
//! functions.

use std::fmt::Write as _;

use anyhow::{Context, Error};

use super::types::{DartScaffoldOptions, DartScaffoldResult};

/// `pubspec.yaml` content emitted for the generated package.
const PUBSPEC_CONTENT: &str = "name: crud_contracts\npublish_to: \"none\"\nversion: 1.0.0\nenvironment:\n  sdk: ^3.11.1\ndependencies:\n  collection: ^1.18.0\n";

/// Barrel library header.
const BARREL_HEADER: &str = "// AUTO-GENERATED — do not edit. Recreated by rhino-cli contracts dart-scaffold.\n// @dart=2.18\n// ignore_for_file: type=lint\nlibrary openapi.api;\n\nimport 'package:collection/collection.dart';\n";

/// Barrel library utility functions (note the leading newline).
const BARREL_UTILS: &str = "\nconst _deepEquality = DeepCollectionEquality();\nfinal _dateFormatter = _DateFormatter();\n\nclass _DateFormatter {\n  String format(DateTime dt) =>\n      '${dt.year.toString().padLeft(4, '0')}'\n      '-${dt.month.toString().padLeft(2, '0')}'\n      '-${dt.day.toString().padLeft(2, '0')}';\n}\n\nT? mapValueOfType<T>(Map<String, dynamic> map, String key) {\n  final v = map[key];\n  return v is T ? v : null;\n}\n\nDateTime? mapDateTime(Map<String, dynamic> map, String key, String? f) {\n  final v = map[key];\n  return v is String && v.isNotEmpty ? DateTime.tryParse(v) : null;\n}\n\nMap<K, V>? mapCastOfType<K, V>(Map<String, dynamic> map, String key) {\n  final v = map[key];\n  return v is Map ? v.cast<K, V>() : null;\n}\n";

/// Creates `pubspec.yaml` and a barrel library for the Dart generated-contracts
/// package.
///
/// # Errors
///
/// Returns an error when the target directory cannot be resolved or walked, or
/// when `pubspec.yaml` or the barrel library cannot be written.
pub fn scaffold_dart(opts: &DartScaffoldOptions) -> Result<DartScaffoldResult, Error> {
    let mut result = DartScaffoldResult {
        pubspec_created: false,
        barrel_created: false,
        model_files: Vec::new(),
    };

    let dir = std::path::Path::new(&opts.dir);

    // Step 1: Write pubspec.yaml.
    let pubspec_path = dir.join("pubspec.yaml");
    std::fs::write(&pubspec_path, PUBSPEC_CONTENT.as_bytes()).context("writing pubspec.yaml")?;
    result.pubspec_created = true;

    // Step 2: Ensure lib directory exists.
    let lib_dir = dir.join("lib");
    std::fs::create_dir_all(&lib_dir).context("creating lib directory")?;

    // Step 3: Glob model files (lib/model/*.dart).
    let model_pattern = lib_dir.join("model").join("*.dart");
    let matches = glob::glob(&model_pattern.to_string_lossy()).context("globbing model files")?;

    let mut basenames: Vec<String> = Vec::new();
    for m in matches {
        // Per-entry IO errors are surfaced as the walk error.
        let p = m.context("globbing model files")?;
        if let Some(name) = p.file_name() {
            basenames.push(name.to_string_lossy().into_owned());
        }
    }
    basenames.sort();
    result.model_files.clone_from(&basenames);

    // Step 4: Build barrel file content.
    let mut sb = String::new();
    sb.push_str(BARREL_HEADER);
    for base in &basenames {
        let _ = writeln!(sb, "part 'model/{base}';");
    }
    sb.push_str(BARREL_UTILS);

    // Step 5: Write barrel file.
    let barrel_path = lib_dir.join("crud_contracts.dart");
    std::fs::write(&barrel_path, sb.as_bytes()).context("writing barrel library")?;
    result.barrel_created = true;

    Ok(result)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn opts(dir: &std::path::Path) -> DartScaffoldOptions {
        DartScaffoldOptions {
            dir: dir.to_string_lossy().into_owned(),
        }
    }

    #[test]
    fn scaffold_with_model_files() {
        let tmp = TempDir::new().unwrap();
        let model_dir = tmp.path().join("lib/model");
        fs::create_dir_all(&model_dir).unwrap();
        fs::write(model_dir.join("user.dart"), "// model").unwrap();
        fs::write(model_dir.join("account.dart"), "// model").unwrap();

        let r = scaffold_dart(&opts(tmp.path())).unwrap();
        assert!(r.pubspec_created);
        assert!(r.barrel_created);
        assert_eq!(
            r.model_files,
            vec!["account.dart".to_string(), "user.dart".to_string()]
        );

        let pubspec = fs::read_to_string(tmp.path().join("pubspec.yaml")).unwrap();
        assert_eq!(pubspec, PUBSPEC_CONTENT);

        let barrel = fs::read_to_string(tmp.path().join("lib/crud_contracts.dart")).unwrap();
        assert!(barrel.starts_with(BARREL_HEADER));
        assert!(barrel.contains("part 'model/account.dart';\n"));
        assert!(barrel.contains("part 'model/user.dart';\n"));
        assert!(barrel.ends_with(BARREL_UTILS));
    }

    #[test]
    fn scaffold_with_no_model_files() {
        let tmp = TempDir::new().unwrap();
        let r = scaffold_dart(&opts(tmp.path())).unwrap();
        assert!(r.pubspec_created);
        assert!(r.barrel_created);
        assert!(r.model_files.is_empty());

        let barrel = fs::read_to_string(tmp.path().join("lib/crud_contracts.dart")).unwrap();
        // No part directives: header immediately followed by utils.
        assert_eq!(barrel, format!("{BARREL_HEADER}{BARREL_UTILS}"));
    }

    #[test]
    fn scaffold_overwrites_existing() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("pubspec.yaml"), "old content").unwrap();
        let lib = tmp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(lib.join("crud_contracts.dart"), "old barrel").unwrap();

        let r = scaffold_dart(&opts(tmp.path())).unwrap();
        assert!(r.pubspec_created);
        assert_eq!(
            fs::read_to_string(tmp.path().join("pubspec.yaml")).unwrap(),
            PUBSPEC_CONTENT
        );
        let barrel = fs::read_to_string(lib.join("crud_contracts.dart")).unwrap();
        assert!(barrel.starts_with(BARREL_HEADER));
        assert!(!barrel.contains("old barrel"));
    }

    #[test]
    fn pubspec_constant_matches_expected_bytes() {
        assert_eq!(
            PUBSPEC_CONTENT,
            "name: crud_contracts\npublish_to: \"none\"\nversion: 1.0.0\nenvironment:\n  sdk: ^3.11.1\ndependencies:\n  collection: ^1.18.0\n"
        );
    }
}
