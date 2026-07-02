//! Cucumber-rs integration tests for the `lang java null-safety-annotations
//! validate` command.
//!
//! Wires the behavior-contract feature file at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/java/` to step definitions that
//! synthesize Java source trees inside a fresh temp directory and drive the
//! compiled `rhino-cli` binary, asserting on output and exit code.

// Test step-definition scaffolding: private World state and step fns are
// self-documenting via their #[given]/#[when]/#[then] gherkin strings.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]

use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// Shared scenario state. Each scenario gets a fresh temp source tree.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct JavaWorld {
    root: TempDir,
    /// Annotation name to require (set by the `--annotation` scenario).
    annotation: Option<String>,
    output: Option<Output>,
}

impl std::fmt::Debug for JavaWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JavaWorld")
            .field("annotation", &self.annotation)
            .finish_non_exhaustive()
    }
}

impl JavaWorld {
    fn new() -> Self {
        Self {
            root: TempDir::new().expect("temp dir"),
            annotation: None,
            output: None,
        }
    }

    fn write(&self, rel: &str, content: &str) {
        let p = self.root.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk fixture dir");
        }
        std::fs::write(p, content).expect("write fixture");
    }

    fn exec(&mut self) {
        let root = self.root.path().to_string_lossy().into_owned();
        let mut args = vec![
            "lang".to_string(),
            "java".to_string(),
            "null-safety-annotations".to_string(),
            "validate".to_string(),
            "--dir".to_string(),
            root,
        ];
        if let Some(a) = &self.annotation {
            args.push("--annotation".to_string());
            args.push(a.clone());
        }
        args.push("--no-color".to_string());
        let out = std::process::Command::new(cargo_bin("rhino-cli"))
            .args(&args)
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
// Given steps
// ===========================================================================

#[given("a Java source tree where every package has a @NullMarked-annotated package-info.java")]
fn given_all_annotated(w: &mut JavaWorld) {
    w.write("com/a/A.java", "package com.a;\n");
    w.write("com/a/package-info.java", "@NullMarked\npackage com.a;\n");
    w.write("com/b/B.java", "package com.b;\n");
    w.write("com/b/package-info.java", "@NullMarked\npackage com.b;\n");
}

#[given("a Java source tree where one package has no package-info.java file")]
fn given_missing_package_info(w: &mut JavaWorld) {
    w.write("com/a/A.java", "package com.a;\n");
    w.write("com/a/package-info.java", "@NullMarked\npackage com.a;\n");
    // com/b has a .java file but no package-info.java.
    w.write("com/b/B.java", "package com.b;\n");
}

#[given("a Java source tree where one package has a package-info.java without @NullMarked")]
fn given_missing_annotation(w: &mut JavaWorld) {
    w.write("com/a/A.java", "package com.a;\n");
    w.write("com/a/package-info.java", "@NullMarked\npackage com.a;\n");
    // com/b has package-info.java but no @NullMarked.
    w.write("com/b/B.java", "package com.b;\n");
    w.write("com/b/package-info.java", "package com.b;\n");
}

#[given("a Java source tree where every package has a @NonNull-annotated package-info.java")]
fn given_all_nonnull(w: &mut JavaWorld) {
    w.annotation = Some("NonNull".to_string());
    w.write("com/a/A.java", "package com.a;\n");
    w.write("com/a/package-info.java", "@NonNull\npackage com.a;\n");
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the developer runs java validate-annotations on the source root")]
#[when("the developer runs java validate-annotations with --annotation NonNull")]
fn when_run_validate(w: &mut JavaWorld) {
    w.exec();
}

// ===========================================================================
// Then steps
// ===========================================================================

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut JavaWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut JavaWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[then("the output reports zero violations")]
fn then_zero_violations(w: &mut JavaWorld) {
    let out = w.stdout();
    assert!(out.contains("0 violations found."), "got: {out}");
}

#[then("the output identifies the package missing package-info.java")]
fn then_identifies_missing_package_info(w: &mut JavaWorld) {
    let out = w.stdout();
    assert!(out.contains("com/b"), "got: {out}");
    assert!(out.contains("package-info.java missing"), "got: {out}");
}

#[then("the output identifies the package with the missing annotation")]
fn then_identifies_missing_annotation(w: &mut JavaWorld) {
    let out = w.stdout();
    assert!(out.contains("com/b"), "got: {out}");
    assert!(out.contains("@NullMarked missing"), "got: {out}");
}

#[tokio::main]
async fn main() {
    JavaWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/java")
        .canonicalize()
        .expect("feature dir resolvable")
}
