//! Shared state-diagram golden corpus harness (P8c).
//!
//! Iterates over every `*.md` fixture in `tests/fixtures/state/`, runs
//! `validate_blocks`, and byte-compares the serialised violation list against
//! the companion `*.expected.json` file.
//!
//! Running:
//!   npx nx run rhino-cli:test:integration
//!
//! Refreshing a single expected file:
//!   cargo run … -- docs validate-mermaid <file> --format=json > *.expected.json
#![allow(clippy::panic, clippy::unwrap_used)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use rhino_cli::domain::mermaid::{
    ViolationKind, default_validate_options, extract_blocks, validate_blocks,
};
use serde::{Deserialize, Serialize};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/state")
}

/// Compact representation stored in the `*.expected.json` files.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ExpectedViolation {
    kind: String,
}

impl ExpectedViolation {
    fn from_kind(k: ViolationKind) -> Self {
        Self {
            kind: k.code().to_string(),
        }
    }
}

#[test]
fn state_diagram_corpus_violations_match_expected() {
    let dir = fixtures_dir();
    assert!(
        dir.exists(),
        "fixture directory missing: {} — run Phase 8c GREEN to create it",
        dir.display()
    );

    let mut md_files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    md_files.sort();

    assert!(
        !md_files.is_empty(),
        "no *.md fixtures found in {} — run Phase 8c GREEN to populate fixtures",
        dir.display()
    );

    let mut any_mismatch = false;
    for md_path in &md_files {
        let stem = md_path.file_stem().unwrap().to_string_lossy();
        let expected_path = dir.join(format!("{stem}.expected.json"));

        let content = fs::read_to_string(md_path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", md_path.display()));
        let blocks = extract_blocks(&md_path.to_string_lossy(), &content);
        let result = validate_blocks(blocks, default_validate_options());

        let actual: Vec<ExpectedViolation> = result
            .violations
            .iter()
            .map(|v| ExpectedViolation::from_kind(v.kind))
            .collect();
        let actual_json = serde_json::to_string_pretty(&actual)
            .unwrap_or_else(|e| panic!("serialisation failed: {e}"));

        let expected_json = fs::read_to_string(&expected_path)
            .unwrap_or_else(|e| panic!("missing expected file {}: {e}", expected_path.display()));
        let expected: Vec<ExpectedViolation> = serde_json::from_str(&expected_json)
            .unwrap_or_else(|e| panic!("bad expected JSON in {}: {e}", expected_path.display()));

        if actual != expected {
            eprintln!(
                "MISMATCH for {}:\n  actual:   {actual_json}\n  expected: {expected_json}",
                md_path.display()
            );
            any_mismatch = true;
        }
    }

    assert!(
        !any_mismatch,
        "one or more corpus fixtures had violations mismatch"
    );
}
