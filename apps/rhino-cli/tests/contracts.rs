//! Cucumber-rs integration tests for the `specs clean java-imports` and
//! `specs scaffold dart` commands.
//!
//! Wires the behavior-contract feature files at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/contracts/` to step definitions that
//! synthesize generated-contracts fixtures inside a fresh temp directory and
//! drive the compiled `rhino-cli` binary, asserting on output, exit code, and
//! the on-disk effects (rewritten Java files / generated Dart scaffold).

// Test step-definition scaffolding: private World state and step fns are
// self-documenting via their #[given]/#[when]/#[then] gherkin strings.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]

use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// Shared scenario state. Each scenario gets a fresh temp directory used as the
/// generated-contracts directory argument.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct ContractsWorld {
    dir: TempDir,
    output: Option<Output>,
}

impl std::fmt::Debug for ContractsWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContractsWorld").finish_non_exhaustive()
    }
}

impl ContractsWorld {
    fn new() -> Self {
        Self {
            dir: TempDir::new().expect("temp dir"),
            output: None,
        }
    }

    fn write(&self, rel: &str, content: &str) {
        let p = self.dir.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk fixture dir");
        }
        std::fs::write(p, content).expect("write fixture");
    }

    fn read(&self, rel: &str) -> String {
        std::fs::read_to_string(self.dir.path().join(rel)).expect("read fixture")
    }

    fn exists(&self, rel: &str) -> bool {
        self.dir.path().join(rel).exists()
    }

    /// Runs `rhino-cli` with `args` followed by `--dir <fixture-dir> --no-color`.
    ///
    /// `args` is the subcommand path, e.g. `["specs", "clean", "java-imports"]`
    /// for `specs clean java-imports --dir <dir> --no-color`.
    fn exec(&mut self, args: &[&str]) {
        let dir = self.dir.path().to_string_lossy().into_owned();
        let out = std::process::Command::new(cargo_bin("rhino-cli"))
            .args(args)
            .args(["--dir", &dir, "--no-color"])
            .output()
            .expect("run rhino-cli");
        self.output = Some(out);
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stdout).into_owned()
    }

    fn exit_code(&self) -> i32 {
        self.output
            .as_ref()
            .expect("ran")
            .status
            .code()
            .unwrap_or(-1)
    }
}

// ===========================================================================
// Given steps — java-clean-imports
// ===========================================================================

#[given("a generated-contracts directory with Java files containing unused imports")]
fn given_unused_imports(w: &mut ContractsWorld) {
    w.write(
        "Foo.java",
        "package com.foo;\nimport java.util.List;\nimport java.util.Map;\n\nclass Foo { List x; }\n",
    );
}

#[given("a generated-contracts directory with Java files containing same-package imports")]
fn given_same_package_imports(w: &mut ContractsWorld) {
    w.write(
        "Bar.java",
        "package com.foo;\nimport com.foo.Helper;\nimport java.util.List;\n\nclass Bar { Helper h; List x; }\n",
    );
}

#[given("a generated-contracts directory with Java files containing duplicate imports")]
fn given_duplicate_imports(w: &mut ContractsWorld) {
    w.write(
        "Dup.java",
        "package com.foo;\nimport java.util.List;\nimport java.util.List;\n\nclass Dup { List x; }\n",
    );
}

#[given("a generated-contracts directory with Java files having only required imports")]
fn given_required_only(w: &mut ContractsWorld) {
    w.write(
        "Clean.java",
        "package com.foo;\nimport java.util.List;\n\nclass Clean { List x; }\n",
    );
}

#[given("an empty generated-contracts directory")]
fn given_empty_dir(_w: &mut ContractsWorld) {
    // The fresh temp dir is already empty.
}

// ===========================================================================
// Given steps — dart-scaffold
// ===========================================================================

#[given("a generated-contracts directory with model Dart files")]
fn given_model_files(w: &mut ContractsWorld) {
    w.write("lib/model/user.dart", "// user model\n");
    w.write("lib/model/account.dart", "// account model\n");
}

#[given("a generated-contracts directory with no model files")]
fn given_no_model_files(_w: &mut ContractsWorld) {
    // Empty temp dir: no lib/model/*.dart.
}

#[given("an existing generated-contracts directory with old scaffold files")]
fn given_old_scaffold(w: &mut ContractsWorld) {
    w.write("pubspec.yaml", "name: old\n");
    w.write("lib/crud_contracts.dart", "// stale barrel\n");
    w.write("lib/model/user.dart", "// user model\n");
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the developer runs contracts java-clean-imports on the directory")]
fn when_run_clean_imports(w: &mut ContractsWorld) {
    w.exec(&["specs", "clean", "java-imports"]);
}

#[when("the developer runs contracts dart-scaffold on the directory")]
fn when_run_dart_scaffold(w: &mut ContractsWorld) {
    w.exec(&["specs", "scaffold", "dart"]);
}

// ===========================================================================
// Then steps — shared
// ===========================================================================

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut ContractsWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

// ===========================================================================
// Then steps — java-clean-imports
// ===========================================================================

#[then("unused imports are removed from the Java files")]
fn then_unused_removed(w: &mut ContractsWorld) {
    let f = w.read("Foo.java");
    assert!(f.contains("import java.util.List;"), "List kept: {f}");
    assert!(!f.contains("import java.util.Map;"), "Map removed: {f}");
}

#[then("same-package imports are removed from the Java files")]
fn then_same_package_removed(w: &mut ContractsWorld) {
    let f = w.read("Bar.java");
    assert!(
        !f.contains("import com.foo.Helper;"),
        "same-pkg removed: {f}"
    );
    assert!(f.contains("import java.util.List;"), "List kept: {f}");
}

#[then("only one copy of each import remains")]
fn then_deduplicated(w: &mut ContractsWorld) {
    let f = w.read("Dup.java");
    assert_eq!(
        f.matches("import java.util.List;").count(),
        1,
        "expected single import: {f}"
    );
}

#[then("the Java files are unchanged")]
fn then_unchanged(w: &mut ContractsWorld) {
    let f = w.read("Clean.java");
    assert_eq!(
        f,
        "package com.foo;\nimport java.util.List;\n\nclass Clean { List x; }\n"
    );
}

#[then("the command reports no files modified")]
fn then_reports_none_modified(w: &mut ContractsWorld) {
    assert_eq!(w.stdout(), "No imports needed cleaning.\n");
}

// ===========================================================================
// Then steps — dart-scaffold
// ===========================================================================

#[then("pubspec.yaml is created with correct content")]
#[then("pubspec.yaml is created")]
fn then_pubspec_created(w: &mut ContractsWorld) {
    assert!(w.exists("pubspec.yaml"), "pubspec.yaml missing");
    let p = w.read("pubspec.yaml");
    assert!(p.contains("name: crud_contracts"), "pubspec content: {p}");
}

#[then("the barrel library is created with part directives for each model")]
fn then_barrel_with_parts(w: &mut ContractsWorld) {
    let b = w.read("lib/crud_contracts.dart");
    assert!(b.contains("part 'model/account.dart';"), "barrel: {b}");
    assert!(b.contains("part 'model/user.dart';"), "barrel: {b}");
}

#[then("the barrel library is created without part directives")]
fn then_barrel_without_parts(w: &mut ContractsWorld) {
    let b = w.read("lib/crud_contracts.dart");
    assert!(!b.contains("part 'model/"), "no part directives: {b}");
    assert!(b.contains("library openapi.api;"), "barrel header: {b}");
}

#[then("the existing files are overwritten with fresh scaffold")]
fn then_overwritten(w: &mut ContractsWorld) {
    let p = w.read("pubspec.yaml");
    assert!(!p.contains("name: old"), "pubspec overwritten: {p}");
    assert!(p.contains("name: crud_contracts"), "pubspec content: {p}");
    let b = w.read("lib/crud_contracts.dart");
    assert!(!b.contains("stale barrel"), "barrel overwritten: {b}");
    assert!(b.contains("part 'model/user.dart';"), "barrel: {b}");
}

#[tokio::main]
async fn main() {
    ContractsWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/contracts")
        .canonicalize()
        .expect("feature dir resolvable")
}
