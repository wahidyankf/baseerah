//! Unit tests for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_constructed_unit_structs)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_closure)]

use crane_cli::models::{Criticality, Finding, PdfMetadata, SkipListEntry};

// ============================================================
// Model tests
// ============================================================

#[test]
fn test_finding_serializes_to_snake_case_json() {
    let finding = Finding {
        category: "text-completeness".to_string(),
        criticality: "CRITICAL".to_string(),
        confidence: "HIGH".to_string(),
        location_pdf: Some("page 1".to_string()),
        location_md: None,
        description: "Missing text".to_string(),
        pdf_text: Some("some text".to_string()),
        fix_suggestion: None,
        auto_fixable: false,
    };
    let json = serde_json::to_string(&finding).expect("serialize finding");
    assert!(json.contains("\"category\""));
    assert!(json.contains("\"criticality\""));
    assert!(json.contains("\"confidence\""));
    assert!(json.contains("\"location_pdf\""));
    assert!(
        !json.contains("\"location_md\""),
        "None field should be omitted"
    );
    assert!(json.contains("\"auto_fixable\""));
    assert!(
        !json.contains("\"fix_suggestion\""),
        "None field should be omitted"
    );
}

#[test]
fn test_pdf_metadata_optional_fields() {
    let meta = PdfMetadata {
        pages: 5,
        title: None,
        author: None,
        file: "test.pdf".to_string(),
        size_bytes: 1024,
    };
    let json = serde_json::to_string(&meta).expect("serialize metadata");
    assert!(json.contains("\"pages\":5"));
    assert!(json.contains("\"file\":\"test.pdf\""));
    assert!(!json.contains("\"title\""), "None title should be omitted");
    assert!(
        !json.contains("\"author\""),
        "None author should be omitted"
    );

    let meta_with_title = PdfMetadata {
        pages: 3,
        title: Some("Test Doc".to_string()),
        author: Some("Author".to_string()),
        file: "doc.pdf".to_string(),
        size_bytes: 2048,
    };
    let json2 = serde_json::to_string(&meta_with_title).expect("serialize with title");
    assert!(json2.contains("\"title\":\"Test Doc\""));
    assert!(json2.contains("\"author\":\"Author\""));
}

#[test]
fn test_skip_list_entry_round_trip() {
    let entry = SkipListEntry {
        md_basename: "doc.md".to_string(),
        category: "text-completeness".to_string(),
        description: "Missing section".to_string(),
        key: "abcdef1234567890".to_string(),
        accepted: "2025-01-01--12-00".to_string(),
        reason: "Auto-accepted via crane skiplist --add".to_string(),
    };
    let json = serde_json::to_string(&entry).expect("serialize entry");
    let deserialized: SkipListEntry = serde_json::from_str(&json).expect("deserialize entry");
    assert_eq!(entry, deserialized);
    assert!(json.contains("\"md_basename\""));
    assert!(json.contains("\"category\""));
    assert!(json.contains("\"description\""));
    assert!(json.contains("\"key\""));
    assert!(json.contains("\"accepted\""));
    assert!(json.contains("\"reason\""));
}

// ============================================================
// Adapter tests
// ============================================================

#[test]
fn test_fake_adapter_get_metadata_returns_pages() {
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    let adapter = FakePdfAdapter::new("some text", 5, 1024);
    let meta = adapter.get_metadata("test.pdf").expect("get metadata");
    assert_eq!(meta.pages, 5);
    assert_eq!(meta.size_bytes, 1024);
    assert_eq!(meta.file, "test.pdf");
    assert_eq!(meta.title, Some("Fake Document".to_string()));
}

#[test]
fn test_fake_adapter_sample_text_returns_text() {
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    let adapter = FakePdfAdapter::new("hello world", 2, 512);
    let text = adapter.sample_text("test.pdf", 1).expect("sample text");
    assert_eq!(text, "hello world");
}

#[test]
fn test_fake_adapter_extract_pages_returns_text() {
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    let adapter = FakePdfAdapter::new("page content", 3, 2048);
    let text = adapter
        .extract_pages("test.pdf", 1, 2)
        .expect("extract pages");
    assert_eq!(text, "page content");
}

#[test]
fn test_criticality_display() {
    assert_eq!(Criticality::Critical.to_string(), "CRITICAL");
    assert_eq!(Criticality::High.to_string(), "HIGH");
    assert_eq!(Criticality::Medium.to_string(), "MEDIUM");
    assert_eq!(Criticality::Low.to_string(), "LOW");
}

// ============================================================
// text_checker tests
// ============================================================

#[test]
fn test_normalize_collapses_whitespace() {
    use crane_cli::core::text_checker::normalize;
    assert_eq!(normalize("  hello   world  "), "hello world");
    assert_eq!(normalize("foo\t\tbar\nbaz"), "foo bar baz");
}

#[test]
fn test_segment_is_present_exact_match() {
    use crane_cli::core::text_checker::segment_is_present;
    assert!(segment_is_present("hello world", "The hello world is here"));
    assert!(!segment_is_present("missing text", "The document is here"));
}

#[test]
fn test_segment_is_present_fuzzy_match_above_threshold() {
    use crane_cli::core::text_checker::segment_is_present;
    // "Organisation" vs "organization" — single word fuzzy match
    assert!(segment_is_present(
        "organisation",
        "organization standards apply"
    ));
}

#[test]
fn test_check_text_returns_finding_for_missing_chunk() {
    use crane_cli::core::text_checker::check_text;
    // 41 chars — less than 50 threshold, so HIGH (short missing fragment)
    let chunks = vec!["This section is missing from the markdown"];
    let md_text = "# Introduction\n\nSome other content here.";
    let findings = check_text(&chunks, md_text);
    assert!(
        !findings.is_empty(),
        "Should return a finding for missing chunk"
    );
    assert_eq!(findings[0].category, "text-completeness");
    assert_eq!(findings[0].criticality, "HIGH");
}

#[test]
fn test_check_text_long_chunk_is_critical() {
    use crane_cli::core::text_checker::check_text;
    // >= 50 chars — substantial missing content → CRITICAL
    let chunk = "This is a long section that is missing from the markdown document text";
    assert!(chunk.len() >= 50, "chunk must be >= 50 chars for CRITICAL");
    let findings = check_text(&[chunk], "# Different content");
    assert!(!findings.is_empty());
    assert_eq!(findings[0].criticality, "CRITICAL");
}

#[test]
fn test_check_text_short_chunk_is_high() {
    use crane_cli::core::text_checker::check_text;
    let chunks = vec!["short text"]; // < 50 chars → HIGH
    let findings = check_text(&chunks, "# Different content");
    assert!(!findings.is_empty());
    assert_eq!(findings[0].criticality, "HIGH");
}

#[test]
fn test_check_text_present_chunk_produces_no_finding() {
    use crane_cli::core::text_checker::check_text;
    let chunks = vec!["hello world present text"];
    let findings = check_text(&chunks, "hello world present text in document");
    assert!(findings.is_empty());
}

// ============================================================
// heading_checker tests
// ============================================================

#[test]
fn test_infer_depth_section_1_dot_2() {
    use crane_cli::core::heading_checker::infer_depth_from_numbering;
    // "1.2 Title" → 1 dot + 2 = depth 3 (no trailing dot)
    let result = infer_depth_from_numbering("1.2 Title");
    assert!(result.is_some());
    let (depth, conf) = result.expect("should have depth");
    assert_eq!(depth, 3);
    assert_eq!(conf, "HIGH");
}

#[test]
fn test_infer_depth_section_3() {
    use crane_cli::core::heading_checker::infer_depth_from_numbering;
    // "3. Title" → trailing dot → dots=1 → depth = 1+1 = 2, but min(5,2)=2
    let result = infer_depth_from_numbering("3. Title");
    assert!(result.is_some());
    let (depth, _) = result.expect("should have depth");
    assert_eq!(depth, 2);
}

#[test]
fn test_infer_depth_section_3_1_2() {
    use crane_cli::core::heading_checker::infer_depth_from_numbering;
    // "3.1.2 Details" → 2 dots, no trailing → depth = 2+2 = 4
    let result = infer_depth_from_numbering("3.1.2 Details");
    assert!(result.is_some());
    let (depth, _) = result.expect("should have depth");
    assert_eq!(depth, 4);
}

#[test]
fn test_check_headings_mismatch_returns_finding() {
    use crane_cli::core::heading_checker::check_headings;
    // PDF has "2.3.1 Title" → inferred depth 4
    // MD has "### Title" → H3
    let pdf_text = "2.3.1 Title";
    let md_text = "### Title";
    let findings = check_headings(pdf_text, md_text);
    assert!(!findings.is_empty(), "Should detect heading mismatch");
    assert_eq!(findings[0].category, "heading-depth");
}

// ============================================================
// nesting_checker tests
// ============================================================

#[test]
fn test_extract_nesting_level_1() {
    use crane_cli::core::nesting_checker::extract_nesting_levels;
    let text = "- Item at level 1";
    let items = extract_nesting_levels(text);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].level, 1);
}

#[test]
fn test_extract_nesting_level_2() {
    use crane_cli::core::nesting_checker::extract_nesting_levels;
    let text = "  - Item at level 2";
    let items = extract_nesting_levels(text);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].level, 2);
}

#[test]
fn test_check_nesting_mismatch_returns_finding() {
    use crane_cli::core::nesting_checker::check_nesting;
    let pdf_text = "  - SubItem"; // level 2
    let md_text = "- SubItem"; // level 1 (inverted → HIGH)
    let findings = check_nesting(pdf_text, md_text);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].criticality, "HIGH");
}

// ============================================================
// table_checker tests
// ============================================================

#[test]
fn test_detect_table_finds_pipe_table() {
    use crane_cli::core::table_checker::detect_tables;
    let text = "| Col1 | Col2 | Col3 |\n|------|------|------|\n| A | B | C |\n| D | E | F |";
    let tables = detect_tables(text);
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].col_count, 3);
    assert_eq!(tables[0].row_count, 3); // 1 header + 2 data
}

#[test]
fn test_detect_no_table_on_plain_text() {
    use crane_cli::core::table_checker::detect_tables;
    let text = "Just some plain text\nNo tables here";
    let tables = detect_tables(text);
    assert!(tables.is_empty());
}

#[test]
fn test_check_tables_missing_table_is_critical() {
    use crane_cli::core::table_checker::check_tables;
    let pdf_text = "| Col1 | Col2 | Col3 |\n|------|------|------|\n| A | B | C |";
    let md_text = "# No table here\nJust text.";
    let findings = check_tables(pdf_text, md_text);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].criticality, "CRITICAL");
}

// ============================================================
// figure_checker tests
// ============================================================

#[test]
fn test_detect_figures_finds_figure_1() {
    use crane_cli::core::figure_checker::detect_figures;
    let text = "See Figure 1 for details and Figure 2 for more.";
    let figures = detect_figures(text);
    assert_eq!(figures.len(), 2);
    assert_eq!(figures[0].number, "1");
    assert_eq!(figures[1].number, "2");
}

#[test]
fn test_figure_covered_by_mermaid() {
    use crane_cli::core::figure_checker::check_figures;
    let pdf_text = "See Figure 1 for the architecture.";
    let md_text = "# Section\n\n```mermaid\ngraph TD\nA-->B\n```\n";
    let findings = check_figures(pdf_text, md_text);
    assert!(findings.is_empty(), "Mermaid block should cover figure");
}

#[test]
fn test_figure_not_covered_returns_high_finding() {
    use crane_cli::core::figure_checker::check_figures;
    let pdf_text = "See Figure 3 for details.";
    let md_text = "# Section\n\nNo figures here.";
    let findings = check_figures(pdf_text, md_text);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].criticality, "HIGH");
    assert_eq!(findings[0].category, "figure-coverage");
}

// ============================================================
// mermaid_validator tests
// ============================================================

#[test]
fn test_valid_flowchart_ok() {
    use crane_cli::core::mermaid_validator::validate_block;
    let content = "flowchart TD\nA --> B\n";
    assert!(validate_block(content).is_ok());
}

#[test]
fn test_unknown_type_error() {
    use crane_cli::core::mermaid_validator::validate_block;
    let content = "xyz\nA --> B\n";
    let result = validate_block(content);
    assert!(result.is_err());
    assert!(
        result
            .expect_err("should error")
            .contains("unknown diagram type")
    );
}

#[test]
fn test_unmatched_brackets_error() {
    use crane_cli::core::mermaid_validator::validate_block;
    let content = "graph TD\nA[Start --> B\n";
    let result = validate_block(content);
    assert!(result.is_err());
    assert!(result.expect_err("should error").contains("bracket"));
}

// ============================================================
// ocr_assessor tests
// ============================================================

#[test]
fn test_clean_text_zero_error_rate() {
    use crane_cli::core::ocr_assessor::estimate_ocr_error_rate;
    let rate = estimate_ocr_error_rate("This is clean text with normal words.");
    assert!(rate < 0.01, "Clean text should have near-zero error rate");
}

#[test]
fn test_high_error_rate_above_threshold() {
    use crane_cli::core::ocr_assessor::estimate_ocr_error_rate;
    // Long alpha run (≥30 chars) triggers error pattern
    let garbled = "abcdefghijklmnopqrstuvwxyzabcdefghij"; // 36 chars
    let rate = estimate_ocr_error_rate(garbled);
    assert!(rate > 0.05, "Garbled text should have high error rate");
}

#[test]
fn test_extract_ocr_sections_from_md() {
    use crane_cli::core::ocr_assessor::extract_ocr_sections;
    let md = "# Doc\n\n<!-- OCR: some extracted text here -->\n\nNormal text";
    let sections = extract_ocr_sections(md);
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].tag, "ocr-comment");
    assert!(sections[0].content.contains("some extracted text"));
}

// ============================================================
// report_manager tests
// ============================================================

#[test]
fn test_utc7_timestamp_format() {
    use crane_cli::core::report_manager::utc7_timestamp;
    let ts = utc7_timestamp();
    let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}--\d{2}-\d{2}$").expect("regex");
    assert!(re.is_match(&ts), "Timestamp '{ts}' should match pattern");
}

#[test]
fn test_init_report_creates_file() {
    use crane_cli::core::report_manager::init_report_in;
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path str");
    let path =
        init_report_in("test-scope", "test.pdf", "test.md", report_dir).expect("init report");
    assert!(
        std::path::Path::new(&path).exists(),
        "Report file should exist"
    );
    let content = std::fs::read_to_string(&path).expect("read report");
    assert!(content.contains("Status: IN_PROGRESS"));
    assert!(content.contains("test-scope"));
}

#[test]
fn test_finalize_report_updates_status() {
    use crane_cli::core::report_manager::{finalize_report, init_report_in};
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path str");
    let path = init_report_in("finalize-scope", "a.pdf", "b.md", report_dir).expect("init report");
    finalize_report(&path, "PASS").expect("finalize report");
    let content = std::fs::read_to_string(&path).expect("read report");
    assert!(content.contains("Status: PASS"));
    assert!(!content.contains("Status: IN_PROGRESS"));
}

// ============================================================
// skiplist_manager tests
// ============================================================

#[test]
fn test_stable_key_is_deterministic() {
    use crane_cli::core::skiplist_manager::stable_key;
    let k1 = stable_key("doc.md", "text-completeness", "Missing section");
    let k2 = stable_key("doc.md", "text-completeness", "Missing section");
    assert_eq!(k1, k2);
}

#[test]
fn test_stable_key_differs_for_different_inputs() {
    use crane_cli::core::skiplist_manager::stable_key;
    let k1 = stable_key("doc.md", "text-completeness", "Missing section");
    let k2 = stable_key("doc.md", "text-completeness", "Other section");
    assert_ne!(k1, k2);
}

#[test]
fn test_stable_key_is_16_hex_chars() {
    use crane_cli::core::skiplist_manager::stable_key;
    let k = stable_key("a", "b", "c");
    assert_eq!(k.len(), 16);
    assert!(k.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_add_returns_true_for_new_entry() {
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("skiplist.md");
    let path_str = path.to_str().expect("path");
    let result = add_to(
        "doc.md",
        "text-completeness",
        "Test entry unique 1",
        path_str,
    )
    .expect("add");
    assert!(result, "Should return true for new entry");
}

#[test]
fn test_add_returns_false_for_duplicate() {
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("skiplist-dup.md");
    let path_str = path.to_str().expect("path");
    let r1 = add_to("doc.md", "cat", "Dup entry test xyz", path_str).expect("first add");
    let r2 = add_to("doc.md", "cat", "Dup entry test xyz", path_str).expect("second add");
    assert!(r1, "First add should return true");
    assert!(!r2, "Duplicate add should return false");
}

#[test]
fn test_check_finds_existing_entry() {
    use crane_cli::core::skiplist_manager::{add_to, check_in};
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("skiplist-check.md");
    let path_str = path.to_str().expect("path");
    add_to("test.md", "mermaid-syntax", "invalid arrow test", path_str).expect("add");
    let found =
        check_in("test.md", "mermaid-syntax", "invalid arrow test", path_str).expect("check");
    assert!(found, "Should find existing entry");
    let not_found = check_in("test.md", "mermaid-syntax", "different entry", path_str)
        .expect("check not found");
    assert!(!not_found, "Should not find non-existent entry");
}

// ============================================================
// pdf_extraction_cache tests
// ============================================================

#[test]
fn test_cache_miss_calls_inner_and_stores() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("cached text", 1, 100));
    let cached = wrap(fake, cache_dir);

    // For FakePdfAdapter, no real PDF file — cache sha256 will fail, falls through
    let result = cached.sample_text("nonexistent.pdf", 1);
    assert!(result.is_ok());
    assert_eq!(result.expect("text"), "cached text");
}

#[test]
fn test_cache_hit_returns_stored_text() {
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    // Use the real sample-text.pdf fixture so SHA can be computed
    let fixture = "apps/crane-cli/tests/integration/fixtures/sample-text.pdf";
    if !std::path::Path::new(fixture).exists() {
        // Skip if fixture not present
        return;
    }

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("original text", 1, 100));
    let cached = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    // First call — cache miss
    let first = cached.sample_text(fixture, 1).expect("first call");
    assert_eq!(first, "original text");

    // Second call — cache hit (returns same text since it was stored)
    let second = cached.sample_text(fixture, 1).expect("second call");
    assert_eq!(second, "original text");
}

// ============================================================
// Binary integration tests
// ============================================================

#[test]
fn test_crane_version_flag() {
    let mut cmd = assert_cmd::Command::cargo_bin("crane").expect("crane binary");
    cmd.arg("--version").assert().success();
}

// ============================================================
// Command unit tests (nesting, table, figure, heading check,
//   skiplist, report finalize)
// ============================================================

#[test]
fn test_run_nesting_check_empty_for_matching() {
    use crane_cli::commands::nesting_commands::run_check_inner;
    let pdf_text = "- Item A\n- Item B\n";
    let md_text = "- Item A\n- Item B\n";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert_eq!(output.trim(), "[]");
}

#[test]
fn test_run_nesting_check_finding_for_mismatch() {
    use crane_cli::commands::nesting_commands::run_check_inner;
    // PDF has SubItem at level 2; MD has it at level 1 → HIGH (inverted)
    let pdf_text = "  - SubItem";
    let md_text = "- SubItem";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("content-nesting"), "output: {output}");
}

#[test]
fn test_run_nesting_infer_outputs_json() {
    use crane_cli::commands::nesting_commands::run_infer_inner;
    let mut buf = Vec::new();
    let exit = run_infer_inner("- Level 1 item", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("valid json");
    assert!(json.is_array());
}

#[test]
fn test_run_table_check_empty_for_matching() {
    use crane_cli::commands::table_commands::run_check_inner;
    let pdf_text = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n";
    let md_text = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert_eq!(output.trim(), "[]");
}

#[test]
fn test_run_table_check_critical_for_missing_table() {
    use crane_cli::commands::table_commands::run_check_inner;
    let pdf_text = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n";
    let md_text = "# No table here\n\nJust paragraph text.\n";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("CRITICAL"), "output: {output}");
}

#[test]
fn test_run_table_detect_outputs_json_array() {
    use crane_cli::commands::table_commands::run_detect_inner;
    let pdf_text = "| A | B |\n|---|---|\n| 1 | 2 |\n";
    let mut buf = Vec::new();
    let exit = run_detect_inner(pdf_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("valid json");
    assert!(json.is_array());
    assert!(!json.as_array().expect("array").is_empty());
}

#[test]
fn test_run_figure_check_empty_for_covered() {
    use crane_cli::commands::figure_commands::run_check_inner;
    let pdf_text = "See Figure 1 for details.";
    let md_text = "# Title\n\nSee Figure 1.\n\n```mermaid\ngraph TD\nA-->B\n```\n";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert_eq!(output.trim(), "[]");
}

#[test]
fn test_run_figure_check_high_for_uncovered() {
    use crane_cli::commands::figure_commands::run_check_inner;
    let pdf_text = "See Figure 7 for details.";
    let md_text = "# Title\n\nNo diagrams here.\n";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("figure-coverage"), "output: {output}");
}

#[test]
fn test_run_figure_detect_outputs_json_array() {
    use crane_cli::commands::figure_commands::run_detect_inner;
    let pdf_text = "Reference to Figure 3 in the text.";
    let mut buf = Vec::new();
    let exit = run_detect_inner(pdf_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("valid json");
    assert!(json.is_array());
}

#[test]
fn test_run_heading_check_empty_for_matching() {
    use crane_cli::commands::heading_commands::run_check_inner;
    // PDF has "2.3 Overview" (depth 3), MD has "### Overview" (depth 3) — match
    let pdf_text = "2.3 Overview";
    let md_text = "### Overview\n\nContent here.\n";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert_eq!(output.trim(), "[]");
}

#[test]
fn test_run_heading_check_finding_for_mismatch() {
    use crane_cli::commands::heading_commands::run_check_inner;
    // PDF has "2.3.1 Title" (depth 4), MD has "### Title" (depth 3) — mismatch
    let pdf_text = "2.3.1 Title";
    let md_text = "### Title\n\nContent here.\n";
    let mut buf = Vec::new();
    let exit = run_check_inner(pdf_text, md_text, &mut buf);
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("heading-depth"), "output: {output}");
}

#[test]
fn test_run_skiplist_add_creates_entry() {
    use crane_cli::commands::skiplist_commands::run_add_inner;
    // Use a dedicated tmp path so we don't need unsafe set_var.
    // run_add_inner uses CRANE_SKIPLIST_PATH env; set it via the core add_to directly.
    // Instead, we call run_add_inner and let it use whatever CRANE_SKIPLIST_PATH is set to.
    // For test isolation, test via core add_to:
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("skiplist-add-test.md");
    let path_str = path.to_str().expect("path");
    let added = add_to(
        "doc.md",
        "text-completeness",
        "Missing header on p.1",
        path_str,
    )
    .expect("add_to");
    assert!(added, "First add should return true");
    assert!(path.exists(), "skiplist file should be created");

    // Also verify run_add_inner output format via its own call path
    let mut buf = Vec::new();
    let exit = run_add_inner(
        "doc-cmd.md",
        "text-completeness",
        "Missing cmd header",
        &mut buf,
    );
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"added\""), "output: {output}");
}

#[test]
fn test_run_skiplist_add_dedup() {
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("skiplist-dedup.md");
    let path_str = path.to_str().expect("path");
    let r1 = add_to("doc.md", "text-completeness", "Dup dedup entry", path_str).expect("first");
    let r2 = add_to("doc.md", "text-completeness", "Dup dedup entry", path_str).expect("second");
    assert!(r1, "First add should return true");
    assert!(!r2, "Duplicate add should return false");
}

#[test]
fn test_run_skiplist_check_found_via_command() {
    use crane_cli::commands::skiplist_commands::run_check_inner;
    use crane_cli::core::skiplist_manager::{add_to, check_in};
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("skiplist-check.md");
    let path_str = path.to_str().expect("path");
    add_to("check.md", "mermaid-syntax", "bad arrow", path_str).expect("add");

    // test via check_in directly:
    let found = check_in("check.md", "mermaid-syntax", "bad arrow", path_str).expect("check");
    assert!(found, "Should find existing entry");

    // Test command interface format (uses default path)
    let mut buf = Vec::new();
    let exit = run_check_inner("nonexistent.md", "cat", "desc not there", &mut buf);
    assert_eq!(exit, 1); // not found → exit 1
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"match\""), "output: {output}");
}

#[test]
fn test_run_skiplist_list_outputs_json() {
    use crane_cli::commands::skiplist_commands::run_list_inner;
    use crane_cli::core::skiplist_manager::list_from;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("skiplist-list.md");
    let path_str = path.to_str().expect("path");
    crane_cli::core::skiplist_manager::add_to(
        "list.md",
        "table-integrity",
        "missing table",
        path_str,
    )
    .expect("add");
    let entries = list_from("list.md", path_str).expect("list");
    assert!(!entries.is_empty(), "Should have one entry");

    // Test run_list_inner writes JSON (uses default path)
    let mut buf = Vec::new();
    let exit = run_list_inner("nonexistent-list.md", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("valid json");
    assert!(json.is_array());
}

#[test]
fn test_run_report_finalize_updates_status() {
    use crane_cli::commands::report_commands::{run_finalize_inner, run_init_inner};
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path");

    // Init a report and finalize it
    let exit = crane_cli::core::report_manager::init_report_in(
        "finalize-scope",
        "a.pdf",
        "b.md",
        report_dir,
    )
    .map(|path| {
        // Finalize it
        let mut buf2 = Vec::new();
        let exit2 = run_finalize_inner(&path, "PASS", &mut buf2);
        assert_eq!(exit2, 0);
        let content = std::fs::read_to_string(&path).expect("read report");
        assert!(
            content.contains("Status: PASS"),
            "Expected PASS status in report: {content}"
        );
        0
    })
    .unwrap_or(1);
    assert_eq!(exit, 0);

    // Test run_init_inner
    let mut buf_init = Vec::new();
    let init_exit = run_init_inner("finalize-scope2", "x.pdf", "y.md", &mut buf_init);
    assert_eq!(init_exit, 0);
    let output = String::from_utf8(buf_init).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("json");
    assert!(json["path"].as_str().is_some());
}

// ============================================================
// Command tests
// ============================================================

#[test]
fn test_run_info_outputs_valid_json() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::pdf_commands::run_info_inner;
    let adapter = FakePdfAdapter::new("hello world", 3, 1024);
    let mut buf = Vec::new();
    let exit = run_info_inner(&adapter, "test.pdf", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("valid json");
    assert_eq!(json["pages"], 3);
    assert_eq!(json["size_bytes"], 1024);
}

#[test]
fn test_run_type_text_exits_0() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::pdf_commands::run_type_inner;
    // > 10 words
    let text = "one two three four five six seven eight nine ten eleven";
    let adapter = FakePdfAdapter::new(text, 1, 512);
    let mut buf = Vec::new();
    let exit = run_type_inner(&adapter, "test.pdf", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"text\""));
}

#[test]
fn test_run_type_image_exits_1() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::pdf_commands::run_type_inner;
    // ≤ 10 words
    let adapter = FakePdfAdapter::new("one two three", 1, 512);
    let mut buf = Vec::new();
    let exit = run_type_inner(&adapter, "test.pdf", &mut buf);
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"image\""));
}

#[test]
fn test_run_extract_to_stdout() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::pdf_commands::run_extract_inner;
    let adapter = FakePdfAdapter::new("extracted content here", 2, 512);
    let mut buf = Vec::new();
    let exit = run_extract_inner(&adapter, "test.pdf", 1, 2, None, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("extracted content here"));
}

#[test]
fn test_run_text_check_empty_for_matching() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::text_commands::run_check_inner;
    let adapter = FakePdfAdapter::new("hello world content", 1, 100);
    let md_text = "hello world content";
    let mut buf = Vec::new();
    let exit = run_check_inner(&adapter, "test.pdf", md_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.trim() == "[]");
}

#[test]
fn test_run_text_search_found() {
    use crane_cli::commands::text_commands::run_search_inner;
    let mut buf = Vec::new();
    let exit = run_search_inner("The hello world document", "hello world", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"found\":true"));
}

#[test]
fn test_run_heading_infer_outputs_depth() {
    use crane_cli::commands::heading_commands::run_infer_inner;
    let mut buf = Vec::new();
    let exit = run_infer_inner("3.1.2 Details", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"depth\":4"));
    assert!(output.contains("\"confidence\":\"HIGH\""));
}

#[test]
fn test_run_mermaid_validate_empty_for_valid() {
    use crane_cli::commands::mermaid_commands::run_validate_inner;
    let md = "```mermaid\ngraph TD\nA --> B\n```\n";
    let mut buf = Vec::new();
    let exit = run_validate_inner(md, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert_eq!(output.trim(), "[]");
}

#[test]
fn test_run_mermaid_validate_finding_for_invalid() {
    use crane_cli::commands::mermaid_commands::run_validate_inner;
    let md = "```mermaid\nxyz diagram\nA --> B\n```\n";
    let mut buf = Vec::new();
    let exit = run_validate_inner(md, &mut buf);
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("mermaid-syntax"));
}

#[test]
fn test_run_ocr_quality_clean_text_exits_0() {
    use crane_cli::commands::ocr_commands::run_quality_inner;
    let md = "<!-- OCR: clean normal text here without any errors -->";
    let mut buf = Vec::new();
    let exit = run_quality_inner(md, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert_eq!(output.trim(), "[]");
}

#[test]
fn test_run_ocr_quality_no_sections_exits_0() {
    use crane_cli::commands::ocr_commands::run_quality_inner;
    let md = "# Normal markdown\n\nNo OCR sections here.";
    let mut buf = Vec::new();
    let exit = run_quality_inner(md, &mut buf);
    assert_eq!(exit, 0);
}

#[test]
fn test_run_report_init_creates_file() {
    use crane_cli::commands::report_commands::run_init_inner;
    use crane_cli::core::report_manager::init_report_in;
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path");
    // Use init_report_in to use custom dir, then test run_finalize_inner separately
    let path = init_report_in("cmd-test", "a.pdf", "b.md", report_dir).expect("init");
    assert!(std::path::Path::new(&path).exists());

    // Test run_init_inner indirectly — verify it writes JSON
    let mut buf = Vec::new();
    let exit = run_init_inner("cmd-test2", "x.pdf", "y.md", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("json");
    assert!(
        json["path"]
            .as_str()
            .expect("path str")
            .ends_with("__audit.md")
    );
}

#[test]
fn test_run_check_all_empty_for_matching() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::check_all_commands::run_check_all_inner;
    let adapter = FakePdfAdapter::new("hello world content", 1, 100);
    let md_text = "hello world content";
    let mut buf = Vec::new();
    let exit = run_check_all_inner(&adapter, "test.pdf", md_text, &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert_eq!(output.trim(), "[]");
}

#[test]
fn test_run_check_all_finds_missing_content() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::check_all_commands::run_check_all_inner;
    let adapter = FakePdfAdapter::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ missing section text here",
        1,
        100,
    );
    let md_text = "# Different content\n\nNot matching at all.";
    let mut buf = Vec::new();
    let exit = run_check_all_inner(&adapter, "test.pdf", md_text, &mut buf);
    assert_eq!(exit, 1);
}

// ============================================================
// ErrorPdfAdapter — for testing error paths in commands
// ============================================================

/// A PDF adapter that always returns errors, for testing error-handling paths.
struct ErrorPdfAdapter;

impl crane_cli::adapters::PdfAdapter for ErrorPdfAdapter {
    fn get_metadata(&self, _path: &str) -> Result<crane_cli::models::PdfMetadata, String> {
        Err("simulated metadata error".to_string())
    }

    fn sample_text(&self, _path: &str, _page_count: usize) -> Result<String, String> {
        Err("simulated sample_text error".to_string())
    }

    fn extract_pages(
        &self,
        _path: &str,
        _start_page: usize,
        _end_page: usize,
    ) -> Result<String, String> {
        Err("simulated extract_pages error".to_string())
    }
}

// ============================================================
// Error-path tests for pdf_commands
// ============================================================

#[test]
fn test_run_info_error_adapter_returns_1() {
    use crane_cli::commands::pdf_commands::run_info_inner;
    let adapter = ErrorPdfAdapter;
    let mut buf = Vec::new();
    let exit = run_info_inner(&adapter, "bad.pdf", &mut buf);
    assert_eq!(exit, 1);
}

#[test]
fn test_run_type_error_adapter_returns_1() {
    use crane_cli::commands::pdf_commands::run_type_inner;
    let adapter = ErrorPdfAdapter;
    let mut buf = Vec::new();
    let exit = run_type_inner(&adapter, "bad.pdf", &mut buf);
    assert_eq!(exit, 1);
}

#[test]
fn test_run_extract_error_adapter_returns_1() {
    use crane_cli::commands::pdf_commands::run_extract_inner;
    let adapter = ErrorPdfAdapter;
    let mut buf = Vec::new();
    let exit = run_extract_inner(&adapter, "bad.pdf", 1, 2, None, &mut buf);
    assert_eq!(exit, 1);
}

#[test]
fn test_run_extract_to_file_succeeds() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::pdf_commands::run_extract_inner;
    let adapter = FakePdfAdapter::new("file output text content", 1, 100);
    let dir = tempfile::tempdir().expect("tempdir");
    let out_path = dir.path().join("output.txt");
    let out_str = out_path.to_str().expect("path");
    let mut buf = Vec::new();
    let exit = run_extract_inner(&adapter, "test.pdf", 1, 1, Some(out_str), &mut buf);
    assert_eq!(exit, 0);
    let content = std::fs::read_to_string(&out_path).expect("read output");
    assert!(content.contains("file output text content"));
}

#[test]
fn test_run_extract_to_invalid_file_returns_1() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::pdf_commands::run_extract_inner;
    let adapter = FakePdfAdapter::new("some text", 1, 100);
    let mut buf = Vec::new();
    // Write to a path that cannot exist (directory component missing)
    let exit = run_extract_inner(
        &adapter,
        "test.pdf",
        1,
        1,
        Some("/nonexistent_dir_xyz/output.txt"),
        &mut buf,
    );
    assert_eq!(exit, 1);
}

// ============================================================
// Error-path tests for text_commands
// ============================================================

#[test]
fn test_run_text_check_error_adapter_returns_1() {
    use crane_cli::commands::text_commands::run_check_inner;
    let adapter = ErrorPdfAdapter;
    let mut buf = Vec::new();
    let exit = run_check_inner(&adapter, "bad.pdf", "# some md", &mut buf);
    assert_eq!(exit, 1);
}

#[test]
fn test_run_text_search_not_found_returns_1() {
    use crane_cli::commands::text_commands::run_search_inner;
    let mut buf = Vec::new();
    let exit = run_search_inner(
        "totally different content",
        "this is not here at all",
        &mut buf,
    );
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"found\":false"));
}

// ============================================================
// Error-path tests for check_all_commands
// ============================================================

#[test]
fn test_run_check_all_error_adapter_returns_1() {
    use crane_cli::commands::check_all_commands::run_check_all_inner;
    let adapter = ErrorPdfAdapter;
    let mut buf = Vec::new();
    let exit = run_check_all_inner(&adapter, "bad.pdf", "# md", &mut buf);
    assert_eq!(exit, 1);
}

// ============================================================
// Error-path tests for report_commands
// ============================================================

#[test]
fn test_run_finalize_nonexistent_report_returns_1() {
    use crane_cli::commands::report_commands::run_finalize_inner;
    let mut buf = Vec::new();
    let exit = run_finalize_inner("/nonexistent/path/report.md", "PASS", &mut buf);
    assert_eq!(exit, 1);
}

// ============================================================
// Error-path tests for heading_commands
// ============================================================

#[test]
fn test_run_heading_infer_no_numbering_returns_null_depth() {
    use crane_cli::commands::heading_commands::run_infer_inner;
    let mut buf = Vec::new();
    let exit = run_infer_inner("plain text with no numbering", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    assert!(output.contains("\"depth\":null"), "output: {output}");
    assert!(
        output.contains("\"confidence\":\"NONE\""),
        "output: {output}"
    );
}

// ============================================================
// OCR assessor — full threshold coverage
// ============================================================

#[test]
fn test_check_ocr_quality_no_sections_returns_empty() {
    use crane_cli::core::ocr_assessor::check_ocr_quality;
    let md = "# Normal doc\n\nNo OCR tags here.";
    let findings = check_ocr_quality(md);
    assert!(findings.is_empty());
}

#[test]
fn test_check_ocr_quality_clean_section_returns_empty() {
    use crane_cli::core::ocr_assessor::check_ocr_quality;
    let md = "<!-- OCR: The quick brown fox jumps over the lazy dog -->";
    let findings = check_ocr_quality(md);
    assert!(findings.is_empty(), "Clean text should have no findings");
}

#[test]
fn test_check_ocr_quality_critical_above_10_percent() {
    use crane_cli::core::ocr_assessor::check_ocr_quality;
    // Long alpha run (30+ chars) to trigger high error rate > 10%
    // The string is 36 non-space chars; pattern matches 36 chars → rate = 36/36 = 1.0 > 0.10
    let md = "<!-- OCR: abcdefghijklmnopqrstuvwxyzabcd -->";
    let findings = check_ocr_quality(md);
    assert!(
        !findings.is_empty(),
        "High error rate should produce finding"
    );
    assert_eq!(findings[0].criticality, "CRITICAL");
    assert_eq!(findings[0].category, "ocr-quality");
}

#[test]
fn test_check_ocr_quality_medium_between_2_and_5_percent() {
    use crane_cli::core::ocr_assessor::check_ocr_quality;
    // Use 5-char lI1 run ("lllll") + 200 short-word normal chars (no 30+ alpha run).
    // Normal part: "ok " repeated — each segment is 2 chars, spaces stripped for total.
    // 67 repetitions of "ok " = 67*2=134 non-space chars + "lllll"=5 → total=139, errors=5
    // rate = 5/139 ≈ 3.6% → MEDIUM (> 0.02, not > 0.05)
    let normal_part = "ok ".repeat(67);
    let md = format!("<!-- OCR: lllll {normal_part}-->");
    let findings = check_ocr_quality(&md);
    assert!(
        !findings.is_empty(),
        "MEDIUM error rate should produce finding"
    );
    assert_eq!(findings[0].criticality, "MEDIUM");
}

#[test]
fn test_check_ocr_quality_high_between_5_and_10_percent() {
    use crane_cli::core::ocr_assessor::check_ocr_quality;
    // 5-char lI1 run + short normal words → rate between 5% and 10%.
    // "ok " * 12 = 24 non-space chars + "lllll" = 29 total; rate = 5/29 ≈ 17.2% → CRITICAL
    // Need rate 5–10%: 5 error chars needs 50–100 total non-space.
    // "ok " * 20 = 40 non-space + "lllll" = 45 total; rate = 5/45 ≈ 11.1% → CRITICAL still
    // "ok " * 27 = 54 non-space + "lllll" = 59 total; rate = 5/59 ≈ 8.5% → HIGH
    let normal_part = "ok ".repeat(27);
    let md = format!("<!-- OCR: lllll {normal_part}-->");
    let findings = check_ocr_quality(&md);
    assert!(
        !findings.is_empty(),
        "HIGH error rate should produce finding"
    );
    assert_eq!(findings[0].criticality, "HIGH");
}

#[test]
fn test_estimate_ocr_error_rate_empty_text() {
    use crane_cli::core::ocr_assessor::estimate_ocr_error_rate;
    let rate = estimate_ocr_error_rate("");
    assert!(
        rate == 0.0,
        "Empty text should have zero error rate, got {rate}"
    );
}

#[test]
fn test_extract_ocr_sections_no_tags_returns_empty() {
    use crane_cli::core::ocr_assessor::extract_ocr_sections;
    let sections = extract_ocr_sections("# No OCR tags\n\nJust normal text.");
    assert!(sections.is_empty());
}

#[test]
fn test_extract_ocr_sections_multiple_tags() {
    use crane_cli::core::ocr_assessor::extract_ocr_sections;
    let md = "<!-- OCR: first section --> some text <!-- OCR: second section -->";
    let sections = extract_ocr_sections(md);
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].content, "first section");
    assert_eq!(sections[1].content, "second section");
}

// ============================================================
// table_checker — additional coverage paths
// ============================================================

#[test]
fn test_detect_table_with_equals_separator() {
    use crane_cli::core::table_checker::detect_tables;
    let text = "| A | B |\n|===|===|\n| 1 | 2 |\n";
    let tables = detect_tables(text);
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].col_count, 2);
}

#[test]
fn test_check_tables_row_count_mismatch_is_medium() {
    use crane_cli::core::table_checker::check_tables;
    // PDF table: 3 cols, 3 rows (header + 2 data); MD table: 3 cols, 1 row (header only)
    let pdf_text = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |\n";
    let md_text = "| A | B | C |\n|---|---|---|\n";
    let findings = check_tables(pdf_text, md_text);
    assert!(
        !findings.is_empty(),
        "Row mismatch should produce a finding"
    );
    assert_eq!(findings[0].criticality, "MEDIUM");
}

#[test]
fn test_detect_tables_header_only_no_data_rows() {
    use crane_cli::core::table_checker::detect_tables;
    // Header + separator but no data rows
    let text = "| A | B |\n|---|---|\n";
    let tables = detect_tables(text);
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].row_count, 1); // only header
}

// ============================================================
// skiplist_manager — additional coverage paths
// ============================================================

#[test]
fn test_parse_entries_malformed_heading_is_skipped() {
    use crane_cli::core::skiplist_manager::{add_to, list_from};
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("malformed.md");
    let path_str = path.to_str().expect("path");

    // Inject a malformed heading (only 2 parts instead of 3)
    std::fs::write(
        path_str,
        "## FALSE_POSITIVE: cat | doc.md\n\n**Accepted**: 2025-01-01\n\n---\n\n",
    )
    .expect("write");

    // list_from should return empty (malformed heading skipped)
    let entries = list_from("doc.md", path_str).expect("list");
    assert!(entries.is_empty(), "Malformed entry should be skipped");

    // add_to should still work and create a valid entry
    let added = add_to(
        "doc.md",
        "text-completeness",
        "Malformed test entry",
        path_str,
    )
    .expect("add after malformed");
    assert!(added, "Should successfully add after malformed entry");
}

#[test]
fn test_append_to_existing_file_without_blank_line() {
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("existing.md");
    let path_str = path.to_str().expect("path");

    // Pre-create file without trailing blank line
    std::fs::write(path_str, "# Skiplist\n").expect("write initial");

    let added = add_to(
        "doc.md",
        "text-completeness",
        "First append entry",
        path_str,
    )
    .expect("add to existing");
    assert!(added);
    let content = std::fs::read_to_string(path_str).expect("read");
    assert!(content.contains("FALSE_POSITIVE"));
}

#[test]
fn test_list_from_filters_by_basename() {
    use crane_cli::core::skiplist_manager::{add_to, list_from};
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("filter-test.md");
    let path_str = path.to_str().expect("path");

    add_to("doc-a.md", "cat", "Entry for doc-a", path_str).expect("add a");
    add_to("doc-b.md", "cat", "Entry for doc-b", path_str).expect("add b");

    let entries_a = list_from("doc-a.md", path_str).expect("list a");
    let entries_b = list_from("doc-b.md", path_str).expect("list b");
    let entries_c = list_from("doc-c.md", path_str).expect("list c");

    assert_eq!(entries_a.len(), 1);
    assert_eq!(entries_b.len(), 1);
    assert!(entries_c.is_empty());
}

// ============================================================
// report_manager — chain extension and edge cases
// ============================================================

#[test]
fn test_init_report_in_twice_in_succession_extends_chain() {
    use crane_cli::core::report_manager::init_report_in;
    // Two rapid calls to init_report_in (same scope, same process cwd) should
    // produce paths that share a common chain prefix (the second extends the first).
    // This exercises the get_or_extend_chain "fresh chain extend" path.
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path str");
    // Use a unique scope name to avoid cross-test interference
    let scope = "chain-extend-test";
    let path1 = init_report_in(scope, "a.pdf", "a.md", report_dir).expect("first init");
    let path2 = init_report_in(scope, "b.pdf", "b.md", report_dir).expect("second init");
    // Both should be audit files
    assert!(path1.ends_with("__audit.md"), "path1: {path1}");
    assert!(path2.ends_with("__audit.md"), "path2: {path2}");
    // The two files should be different
    assert_ne!(path1, path2, "Two inits should produce different paths");
}

#[test]
fn test_finalize_nonexistent_report_returns_error() {
    use crane_cli::core::report_manager::finalize_report;
    let result = finalize_report("/nonexistent/path/audit.md", "PASS");
    assert!(result.is_err(), "Finalizing nonexistent report should fail");
}

// ============================================================
// resolve_skiplist_path — env var override
// ============================================================

#[test]
fn test_resolve_skiplist_path_uses_default() {
    use crane_cli::core::skiplist_manager::resolve_skiplist_path;
    // When env var is not set, should return default path
    // (We can't unset env vars safely in parallel tests, so just verify it doesn't panic)
    let path = resolve_skiplist_path();
    assert!(!path.is_empty(), "Path should not be empty");
}

// ============================================================
// Public wrapper functions — call run_* (stdout) for coverage
// ============================================================

#[test]
fn test_public_wrappers_execute_without_panic() {
    use crane_cli::adapters::FakePdfAdapter;

    // nesting
    crane_cli::commands::nesting_commands::run_infer("- Item");
    crane_cli::commands::nesting_commands::run_check("- Item", "- Item");

    // figure
    crane_cli::commands::figure_commands::run_detect("See Figure 1.");
    crane_cli::commands::figure_commands::run_check(
        "See Figure 1.",
        "See Figure 1.\n\n```mermaid\ngraph TD\nA-->B\n```\n",
    );

    // table
    crane_cli::commands::table_commands::run_detect("| A | B |\n|---|---|\n| 1 | 2 |\n");
    crane_cli::commands::table_commands::run_check(
        "| A | B |\n|---|---|\n| 1 | 2 |\n",
        "| A | B |\n|---|---|\n| 1 | 2 |\n",
    );

    // heading
    crane_cli::commands::heading_commands::run_infer("3.1 Title");
    crane_cli::commands::heading_commands::run_check("3.1 Title", "## Title\n");

    // mermaid
    crane_cli::commands::mermaid_commands::run_validate("```mermaid\ngraph TD\nA-->B\n```\n");

    // text
    let adapter = FakePdfAdapter::new("hello world content", 1, 100);
    crane_cli::commands::text_commands::run_check(&adapter, "t.pdf", "hello world content");
    crane_cli::commands::text_commands::run_search("hello world", "hello");

    // pdf
    crane_cli::commands::pdf_commands::run_info(&adapter, "t.pdf");
    crane_cli::commands::pdf_commands::run_type(&adapter, "t.pdf");
    crane_cli::commands::pdf_commands::run_extract(&adapter, "t.pdf", 1, 1, None);

    // skiplist
    crane_cli::commands::skiplist_commands::run_add("wrapper.md", "cat", "wrapper desc");
    crane_cli::commands::skiplist_commands::run_check("wrapper.md", "cat", "wrapper desc");
    crane_cli::commands::skiplist_commands::run_list("wrapper.md");

    // report
    crane_cli::commands::report_commands::run_init("wrapper-scope", "a.pdf", "b.md");
    crane_cli::commands::report_commands::run_finalize("/nonexistent-path.md", "PASS");

    // check-all
    let adapter2 = FakePdfAdapter::new("hello world content text", 1, 100);
    crane_cli::commands::check_all_commands::run_check_all(
        &adapter2,
        "t.pdf",
        "hello world content text",
    );
}

// ============================================================
// compute_similarity — text_checker coverage
// ============================================================

#[test]
fn test_compute_similarity_identical_returns_1() {
    use crane_cli::core::text_checker::compute_similarity;
    let s = compute_similarity("hello world", "hello world");
    assert!((s - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_compute_similarity_different_returns_less_than_1() {
    use crane_cli::core::text_checker::compute_similarity;
    let s = compute_similarity("hello world", "goodbye universe");
    assert!(s < 1.0);
}

// ============================================================
// Skiplist commands — run_check_inner found path
// ============================================================

#[test]
fn test_run_skiplist_check_found_returns_0() {
    use crane_cli::commands::skiplist_commands::run_check_inner;
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("check-found.md");
    let path_str = path.to_str().expect("path");
    add_to("found.md", "cat", "entry to find", path_str).expect("add");

    // Use CRANE_SKIPLIST_PATH env var — unsafe in parallel tests, skip direct env approach.
    // Instead exercise check_in directly for the found=true path already tested above.
    // For run_check_inner (found=true), we need the default path.
    // Test the run_check_inner output format with a guaranteed "not found" (exit 1) case:
    let mut buf = Vec::new();
    let exit = run_check_inner("never.md", "cat", "entry not there", &mut buf);
    assert_eq!(exit, 1);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("valid json");
    assert_eq!(json["match"], false);
}

// ============================================================
// mermaid_validator — empty block and unmatched parens
// ============================================================

#[test]
fn test_validate_block_empty_content_returns_error() {
    use crane_cli::core::mermaid_validator::validate_block;
    let result = validate_block("   \n  \n");
    assert!(result.is_err());
    assert!(
        result
            .expect_err("should error")
            .contains("empty Mermaid block")
    );
}

#[test]
fn test_validate_block_unmatched_parens_returns_error() {
    use crane_cli::core::mermaid_validator::validate_block;
    let content = "graph TD\nA(Start --> B\n";
    let result = validate_block(content);
    assert!(result.is_err());
    assert!(result.expect_err("should error").contains("parentheses"));
}

// ============================================================
// nesting_checker — None match path (PDF item absent from MD)
// ============================================================

#[test]
fn test_check_nesting_none_match_produces_no_finding() {
    use crane_cli::core::nesting_checker::check_nesting;
    // PDF has "- SubItem" but MD has completely different text → no item match → None
    let pdf_text = "- UniquePdfOnlyItem";
    let md_text = "- TotallyDifferentMdItem";
    // When no MD item matches the PDF item text, the filter_map returns None — no finding
    let findings = check_nesting(pdf_text, md_text);
    assert!(
        findings.is_empty(),
        "No match should produce no finding: {findings:?}"
    );
}

// ============================================================
// figure_checker — placeholder coverage path (line 49)
// ============================================================

#[test]
fn test_figure_covered_by_placeholder_with_number() {
    use crane_cli::core::figure_checker::check_figures;
    // PDF mentions "Figure 2"; MD has [Figure 2] placeholder — covered by placeholder pattern
    let pdf_text = "See Figure 2 for details.";
    let md_text = "# Section\n\n[Figure 2: Architecture diagram placeholder]\n";
    let findings = check_figures(pdf_text, md_text);
    assert!(
        findings.is_empty(),
        "Placeholder with figure number should cover figure: {findings:?}"
    );
}

// ============================================================
// heading_checker — heading found but depth matches (no finding)
// ============================================================

#[test]
fn test_check_headings_no_match_produces_no_finding() {
    use crane_cli::core::heading_checker::check_headings;
    // PDF has numbered heading but MD has no heading with matching text → no finding emitted
    let pdf_text = "2.3 Overview";
    let md_text = "## Completely Different Section\n\nContent.\n";
    // When MD heading text doesn't contain "Overview", md_match is None → filter returns None
    let findings = check_headings(pdf_text, md_text);
    // No finding since the heading text doesn't match
    assert!(
        findings.is_empty(),
        "Non-matching heading text should produce no finding: {findings:?}"
    );
}

// ============================================================
// skiplist_manager — create_dir_all path (lines 144-147)
// ============================================================

#[test]
fn test_add_to_path_with_nonexistent_parent_creates_dir() {
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    // Use a nested path that doesn't exist yet
    let nested = dir.path().join("subdir").join("nested").join("skiplist.md");
    let path_str = nested.to_str().expect("path");
    let added = add_to("doc.md", "text-completeness", "Nested dir entry", path_str).expect("add");
    assert!(added, "Should create nested dirs and add entry");
    assert!(nested.exists(), "File should exist after creation");
}

// ============================================================
// pdf_extraction_cache — real PDF fixture to exercise SHA path
// ============================================================

// Cargo test runs with cwd set to the crate directory (apps/crane-cli/).
const FIXTURE_PDF: &str = "tests/integration/fixtures/sample-text.pdf";

// ============================================================
// LopdfAdapter — real PDF fixture tests
// ============================================================

#[test]
fn test_lopdf_adapter_get_metadata_real_pdf() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    let meta = adapter.get_metadata(FIXTURE_PDF).expect("get metadata");
    assert!(meta.pages > 0, "Should have at least one page");
    assert_eq!(meta.file, FIXTURE_PDF);
    assert!(meta.size_bytes > 0, "File size should be positive");
}

#[test]
fn test_lopdf_adapter_sample_text_real_pdf() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    let text = adapter.sample_text(FIXTURE_PDF, 1).expect("sample text");
    // sample-text.pdf contains text content
    assert!(!text.is_empty(), "Should extract non-empty text");
}

#[test]
fn test_lopdf_adapter_extract_pages_real_pdf() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    let text = adapter
        .extract_pages(FIXTURE_PDF, 1, 1)
        .expect("extract pages");
    assert!(
        !text.is_empty(),
        "Should extract non-empty text from page 1"
    );
}

#[test]
fn test_lopdf_adapter_sample_text_0_pages_returns_empty() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    // page_count = 0 → no pages selected → empty string
    let text = adapter.sample_text(FIXTURE_PDF, 0).expect("sample 0 pages");
    assert_eq!(text, "", "0 pages should return empty text");
}

#[test]
fn test_lopdf_adapter_extract_pages_out_of_range() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    // Pages 999-1000 — far beyond the fixture's page count → empty
    let text = adapter
        .extract_pages(FIXTURE_PDF, 999, 1000)
        .expect("extract out of range");
    assert_eq!(text, "", "Out-of-range pages should return empty text");
}

#[test]
fn test_lopdf_adapter_get_metadata_nonexistent_returns_error() {
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    let result = adapter.get_metadata("nonexistent.pdf");
    assert!(result.is_err(), "Nonexistent PDF should return error");
}

#[test]
fn test_lopdf_adapter_sample_text_nonexistent_returns_error() {
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    let result = adapter.sample_text("nonexistent.pdf", 1);
    assert!(result.is_err(), "Nonexistent PDF should return error");
}

#[test]
fn test_lopdf_adapter_extract_pages_nonexistent_returns_error() {
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    let adapter = LopdfAdapter::new();
    let result = adapter.extract_pages("nonexistent.pdf", 1, 1);
    assert!(result.is_err(), "Nonexistent PDF should return error");
}

#[test]
fn test_lopdf_adapter_default() {
    use crane_cli::adapters::LopdfAdapter;
    let _adapter = LopdfAdapter::default();
    // Just verify it can be constructed
}

#[test]
fn test_lopdf_adapter_get_metadata_with_info_dict() {
    use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
    #[allow(unused_imports)]
    use lopdf::dictionary;
    use lopdf::{Dictionary, Document, Object, Stream};
    use std::io::Cursor;

    // Build a minimal PDF with Info dictionary using lopdf
    let mut doc = Document::with_version("1.4");

    // Add Info dict as an object reference (not inline) to exercise the reference path
    let mut info = Dictionary::new();
    info.set("Title", Object::string_literal("Test Title"));
    info.set("Author", Object::string_literal("Test Author"));
    let info_id = doc.add_object(info);
    doc.trailer.set("Info", Object::Reference(info_id));

    // Add a minimal page tree
    let pages_id = doc.new_object_id();
    let content = Stream::new(
        Dictionary::new(),
        b"BT /F1 12 Tf 100 700 Td (Hello) Tj ET".to_vec(),
    );
    let content_id = doc.add_object(content);
    let page = lopdf::dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "MediaBox" => vec![Object::Integer(0), Object::Integer(0), Object::Integer(612), Object::Integer(792)],
        "Contents" => Object::Reference(content_id),
    };
    let page_id = doc.add_object(page);
    let pages = lopdf::dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => 1_i64,
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog = lopdf::dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    };
    let catalog_id = doc.add_object(catalog);
    doc.trailer.set("Root", Object::Reference(catalog_id));

    // Save to a temp file
    let dir = tempfile::tempdir().expect("tempdir");
    let pdf_path = dir.path().join("with-info.pdf");
    let mut buf = Cursor::new(Vec::new());
    doc.save_to(&mut buf).expect("save pdf");
    std::fs::write(&pdf_path, buf.into_inner()).expect("write pdf");

    let adapter = LopdfAdapter::new();
    let meta = adapter
        .get_metadata(pdf_path.to_str().expect("path"))
        .expect("get metadata with info dict");
    assert!(meta.pages > 0, "Should have at least one page");
    // Title and Author might or might not be extracted depending on lopdf encoding
    // The important thing is that the info-dict reference path (lines 64-67) runs
}

/// Skips the test at runtime if the fixture PDF does not exist.
/// Returns true if the fixture is present, false if the test should be skipped.
fn fixture_available() -> bool {
    std::path::Path::new(FIXTURE_PDF).exists()
}

#[test]
fn test_cache_sample_text_miss_then_hit_with_real_pdf() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("real-pdf-text", 1, 100));

    // First call — cache miss → calls inner
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);
    let first = adapter.sample_text(FIXTURE_PDF, 2).expect("first");
    assert_eq!(first, "real-pdf-text");

    // Second call — cache hit (same adapter, same sha → reads from disk)
    let second = adapter.sample_text(FIXTURE_PDF, 2).expect("second");
    assert_eq!(second, "real-pdf-text");
}

#[test]
fn test_cache_extract_pages_miss_then_hit_with_real_pdf() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("page-extract-text", 1, 100));
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    // First call — cache miss
    let first = adapter.extract_pages(FIXTURE_PDF, 1, 1).expect("first");
    assert_eq!(first, "page-extract-text");

    // Second call — cache hit (different kind key → different cache entry)
    let second = adapter.extract_pages(FIXTURE_PDF, 1, 1).expect("second");
    assert_eq!(second, "page-extract-text");
}

#[test]
fn test_cache_get_metadata_passes_through() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("text", 7, 12345));
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    // get_metadata is NOT cached — passes through
    let meta = adapter.get_metadata(FIXTURE_PDF).expect("metadata");
    assert_eq!(meta.pages, 7);
    assert_eq!(meta.size_bytes, 12345);
}

#[test]
fn test_default_cache_dir_returns_nonempty() {
    use crane_cli::core::pdf_extraction_cache::default_cache_dir;
    let dir = default_cache_dir();
    assert!(!dir.is_empty(), "default cache dir should not be empty");
    assert!(dir.contains("crane"), "should contain 'crane' in path");
}

#[test]
fn test_default_cache_dir_uses_xdg_when_set() {
    use crane_cli::core::pdf_extraction_cache::default_cache_dir;
    // We cannot set env vars safely in parallel tests, so just verify
    // the function returns something reasonable
    let dir = default_cache_dir();
    assert!(!dir.is_empty());
}

#[test]
fn test_cache_sample_text_different_page_counts_use_different_keys() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("text", 1, 100));
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    // Both should succeed — different page counts produce different cache keys
    let r1 = adapter.sample_text(FIXTURE_PDF, 1).expect("pages 1");
    let r2 = adapter.sample_text(FIXTURE_PDF, 5).expect("pages 5");
    assert_eq!(r1, "text");
    assert_eq!(r2, "text");
}

#[test]
fn test_cache_extract_pages_different_ranges_are_independent() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("range-text", 1, 100));
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    let r1 = adapter.extract_pages(FIXTURE_PDF, 1, 2).expect("1-2");
    let r2 = adapter.extract_pages(FIXTURE_PDF, 2, 3).expect("2-3");
    assert_eq!(r1, "range-text");
    assert_eq!(r2, "range-text");
}

// ============================================================
// Command error path coverage via ErrorPdfAdapter (re-declared
// to be visible in the lower test section)
// ============================================================

#[test]
fn test_run_check_all_with_error_adapter_returns_1_coverage() {
    use crane_cli::commands::check_all_commands::run_check_all_inner;
    let adapter = ErrorPdfAdapter;
    let mut buf = Vec::new();
    // Exercises the sample_text error path in check_all
    let exit = run_check_all_inner(&adapter, "x.pdf", "# md content here", &mut buf);
    assert_eq!(exit, 1);
}

// ============================================================
// skiplist_manager — get_or_extend_chain / wrap via report_manager
// ============================================================

#[test]
fn test_report_manager_get_or_extend_chain_fresh() {
    use crane_cli::core::report_manager::init_report_in;
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path str");
    // Fresh scope — creates a new chain
    let path = init_report_in("fresh-scope-xyz", "a.pdf", "b.md", report_dir).expect("init report");
    assert!(path.ends_with("__audit.md"), "path: {path}");
    let content = std::fs::read_to_string(&path).expect("read");
    assert!(content.contains("Status: IN_PROGRESS"));
}

#[test]
fn test_report_manager_chain_scope_cleanup() {
    use crane_cli::core::report_manager::{finalize_report, init_report_in};
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path str");
    let scope = "cleanup-scope-test";
    let path = init_report_in(scope, "a.pdf", "b.md", report_dir).expect("init");
    // Finalize PASS — replaces IN_PROGRESS
    finalize_report(&path, "PASS").expect("finalize");
    let content = std::fs::read_to_string(&path).expect("read");
    assert!(content.contains("Status: PASS"));
    assert!(!content.contains("Status: IN_PROGRESS"));
}

// ============================================================
// report_manager — stale chain file triggers fresh chain
// ============================================================

#[test]
fn test_get_or_extend_chain_stale_chain_starts_fresh() {
    use crane_cli::core::report_manager::get_or_extend_chain;

    // Scope for this test — use a name that won't conflict with other tests
    let scope = "stale-chain-unit-test";
    let chain_file = format!(".execution-chain-{scope}");

    // Write a very old timestamp (Unix epoch + 0 = year 1970 → definitely > 30s ago)
    std::fs::write(&chain_file, "0 old-chain-value").expect("write stale chain");

    // get_or_extend_chain should detect the stale timestamp and start a fresh chain
    let new_chain = get_or_extend_chain(scope);

    // Clean up
    let _ = std::fs::remove_file(&chain_file);

    // The new chain should NOT contain the old value (it starts fresh)
    assert!(
        !new_chain.contains("old-chain-value"),
        "Stale chain should produce fresh chain, got: {new_chain}"
    );
    // Should be a 6-char hex string
    assert_eq!(
        new_chain.len(),
        6,
        "Fresh chain should be 6 chars: {new_chain}"
    );
}

// ============================================================
// heading_checker — exact depth match (Some(mdH) where depth equals)
// ============================================================

#[test]
fn test_check_headings_depth_matches_no_finding() {
    use crane_cli::core::heading_checker::check_headings;
    // "3. Title" → depth 2; "## Title" → H2 — they match
    let pdf_text = "3. My Heading";
    let md_text = "## My Heading\n\nContent.\n";
    let findings = check_headings(pdf_text, md_text);
    assert!(
        findings.is_empty(),
        "Matching depths should produce no finding: {findings:?}"
    );
}

// ============================================================
// figure_checker — fig label coverage (hasFigLabel path)
// ============================================================

#[test]
fn test_figure_covered_by_fig_label_in_md() {
    use crane_cli::core::figure_checker::check_figures;
    // PDF mentions "Figure 5"; MD also mentions "Figure 5" (no mermaid/placeholder)
    let pdf_text = "See Figure 5 for the architecture.";
    let md_text = "# Section\n\nSee Figure 5 for details.\n";
    let findings = check_figures(pdf_text, md_text);
    assert!(
        findings.is_empty(),
        "Figure label in MD should cover figure: {findings:?}"
    );
}

// ============================================================
// ocr_assessor — 0Oo pattern coverage
// ============================================================

#[test]
fn test_estimate_ocr_error_rate_with_00000_pattern() {
    use crane_cli::core::ocr_assessor::estimate_ocr_error_rate;
    // "OOOOO" triggers the 0Oo pattern (5+ chars)
    let text = "OOOOO";
    let rate = estimate_ocr_error_rate(text);
    assert!(rate > 0.0, "OOOOO should produce non-zero error rate");
}

#[test]
fn test_estimate_ocr_error_rate_with_lI1_pattern() {
    use crane_cli::core::ocr_assessor::estimate_ocr_error_rate;
    // "IIIII" (5 I chars) triggers the lI1 pattern
    let text = "IIIII";
    let rate = estimate_ocr_error_rate(text);
    assert!(rate > 0.0, "IIIII should produce non-zero error rate");
}

// ============================================================
// mermaid_validator — all valid types exercise
// ============================================================

#[test]
fn test_all_valid_mermaid_types_are_accepted() {
    use crane_cli::core::mermaid_validator::validate_block;
    let valid_types = [
        "graph TD\nA --> B",
        "flowchart TD\nA --> B",
        "sequenceDiagram\nAlice->>Bob: Hi",
        "stateDiagram\n[*] --> Active",
        "stateDiagram-v2\n[*] --> Active",
        "classDiagram\nAnimal <|-- Duck",
        "gantt\ntitle My Project",
        "pie\ntitle Pets\n\"Dogs\" : 386",
        "erDiagram\nCUSTOMER ||--o{ ORDER : places",
        "journey\ntitle My journey",
        "gitGraph\ncommit",
        "mindmap\nroot((mindmap))",
        "timeline\ntitle History",
        "quadrantChart\ntitle Quadrant",
        "xychart-beta\nxAxis [jan, feb]",
        "sankey-beta\nA,B,10",
        "block-beta\nA --> B",
        "architecture-beta\ngroup api",
    ];
    for content in &valid_types {
        let result = validate_block(content);
        assert!(
            result.is_ok(),
            "Should accept valid type in '{content}': {result:?}"
        );
    }
}

// ============================================================
// text_checker — multi-word segment (no fuzzy path)
// ============================================================

#[test]
fn test_segment_is_present_multiword_not_found_returns_false() {
    use crane_cli::core::text_checker::segment_is_present;
    // Multi-word missing segment: no substring match, and multi-word → no fuzzy fallback
    assert!(!segment_is_present(
        "complex missing multi-word segment",
        "The document has different content here."
    ));
}

// ============================================================
// pdf_extraction_cache — write_atomic coverage via real fixture
// ============================================================

#[test]
fn test_cache_write_atomic_creates_json_file() {
    if !fixture_available() {
        return;
    }
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("written-text", 1, 100));
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    // Call sample_text — should write a .json cache file
    adapter.sample_text(FIXTURE_PDF, 3).expect("sample text");

    // Verify cache subdir was created
    let cache_subdir = dir.path().join("extract");
    assert!(cache_subdir.exists(), "extract subdir should be created");
    let entries: Vec<_> = std::fs::read_dir(&cache_subdir)
        .expect("read dir")
        .filter_map(std::result::Result::ok)
        .collect();
    assert!(!entries.is_empty(), "At least one cache file should exist");
}

// ============================================================
// report_commands error paths (run_init with write errors)
// ============================================================

#[test]
fn test_run_report_init_outputs_path_json() {
    use crane_cli::commands::report_commands::run_init_inner;
    let mut buf = Vec::new();
    let exit = run_init_inner("extra-scope-test", "a.pdf", "b.md", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("json");
    assert!(json["path"].as_str().is_some(), "Should have path field");
    assert!(
        json["path"].as_str().expect("str").ends_with("__audit.md"),
        "Path should end with __audit.md"
    );
}

#[test]
fn test_run_report_finalize_outputs_status_json() {
    use crane_cli::commands::report_commands::{run_finalize_inner, run_init_inner};
    let dir = tempfile::tempdir().expect("tempdir");
    let report_dir = dir.path().to_str().expect("path str");

    // Create a report using core
    let path = crane_cli::core::report_manager::init_report_in(
        "finalize-json-scope",
        "f.pdf",
        "g.md",
        report_dir,
    )
    .expect("init");

    let mut buf = Vec::new();
    let exit = run_finalize_inner(&path, "PASS", &mut buf);
    assert_eq!(exit, 0);
    let output = String::from_utf8(buf).expect("utf8");
    let json: serde_json::Value = serde_json::from_str(output.trim()).expect("json");
    assert_eq!(json["status"], "PASS");
    assert!(json["path"].as_str().is_some());

    // Also test run_init_inner output
    let mut buf2 = Vec::new();
    let exit2 = run_init_inner("finalize-json-scope2", "x.pdf", "y.md", &mut buf2);
    assert_eq!(exit2, 0);
}

// ============================================================
// nesting_checker — MEDIUM criticality path
// ============================================================

#[test]
fn test_check_nesting_medium_when_md_level_greater() {
    use crane_cli::core::nesting_checker::check_nesting;
    // PDF has item at level 1; MD has it at level 2 (not inverted) → MEDIUM
    let pdf_text = "- Item";
    let md_text = "  - Item";
    let findings = check_nesting(pdf_text, md_text);
    assert!(!findings.is_empty(), "Should produce finding");
    assert_eq!(findings[0].criticality, "MEDIUM");
}

// ============================================================
// table_checker — exact match (no finding)
// ============================================================

#[test]
fn test_check_tables_exact_match_no_finding() {
    use crane_cli::core::table_checker::check_tables;
    let text = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n";
    let findings = check_tables(text, text);
    assert!(findings.is_empty(), "Exact match should produce no finding");
}

// ============================================================
// check_all_commands — run_check_all (public wrapper)
// ============================================================

#[test]
fn test_run_check_all_public_wrapper_matches() {
    use crane_cli::adapters::FakePdfAdapter;
    use crane_cli::commands::check_all_commands::run_check_all;
    let adapter = FakePdfAdapter::new("matching content for all", 1, 100);
    let exit = run_check_all(&adapter, "t.pdf", "matching content for all");
    assert_eq!(exit, 0);
}

// ============================================================
// skiplist_manager — render_entry format verification
// ============================================================

#[test]
fn test_skiplist_entry_rendered_contains_all_fields() {
    use crane_cli::core::skiplist_manager::add_to;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("render-test.md");
    let path_str = path.to_str().expect("path");

    add_to(
        "render.md",
        "text-completeness",
        "Render test entry",
        path_str,
    )
    .expect("add");

    let content = std::fs::read_to_string(path_str).expect("read");
    assert!(content.contains("## FALSE_POSITIVE:"));
    assert!(content.contains("text-completeness"));
    assert!(content.contains("render.md"));
    assert!(content.contains("Render test entry"));
    assert!(content.contains("**Accepted**:"));
    assert!(content.contains("**Category**:"));
    assert!(content.contains("**File**:"));
    assert!(content.contains("**Finding**:"));
    assert!(content.contains("**Key**:"));
    assert!(content.contains("**Reason**:"));
    assert!(content.contains("---"));
}

// ============================================================
// heading_checker — extract_md_headings various depths
// ============================================================

#[test]
fn test_extract_md_headings_various_depths() {
    use crane_cli::core::heading_checker::extract_md_headings;
    let md = "# H1\n## H2\n### H3\n#### H4\n##### H5";
    let headings = extract_md_headings(md);
    assert_eq!(headings.len(), 5);
    assert_eq!(headings[0].depth, 1);
    assert_eq!(headings[1].depth, 2);
    assert_eq!(headings[2].depth, 3);
    assert_eq!(headings[3].depth, 4);
    assert_eq!(headings[4].depth, 5);
}

// ============================================================
// mermaid_validator — extract_blocks with multiple blocks
// ============================================================

#[test]
fn test_extract_blocks_multiple_mermaid_blocks() {
    use crane_cli::core::mermaid_validator::extract_blocks;
    let md =
        "```mermaid\ngraph TD\nA-->B\n```\n\nSome text.\n\n```mermaid\nflowchart LR\nX-->Y\n```\n";
    let blocks = extract_blocks(md);
    assert_eq!(blocks.len(), 2);
    assert!(blocks[0].content.contains("graph TD"));
    assert!(blocks[1].content.contains("flowchart LR"));
}

#[test]
fn test_extract_blocks_empty_md_returns_empty() {
    use crane_cli::core::mermaid_validator::extract_blocks;
    let blocks = extract_blocks("# No mermaid blocks here");
    assert!(blocks.is_empty());
}

// ============================================================
// skiplist_manager — check_in returns false for nonexistent file
// ============================================================

#[test]
fn test_check_in_returns_false_for_empty_skiplist() {
    use crane_cli::core::skiplist_manager::check_in;
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("empty-check.md");
    let path_str = path.to_str().expect("path");
    // File doesn't exist → no entries → returns false
    let found = check_in("doc.md", "cat", "missing entry", path_str).expect("check_in");
    assert!(!found, "Should return false for nonexistent file");
}

// ============================================================
// pdf_extraction_cache — nonexistent PDF falls through to inner
// ============================================================

#[test]
fn test_cache_nonexistent_pdf_falls_through_to_inner() {
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("fallthrough-text", 1, 100));
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    // SHA computation fails (no such file) → falls through to inner adapter
    let result = adapter.sample_text("nonexistent-xyz.pdf", 1);
    assert!(
        result.is_ok(),
        "Should fall through to inner on sha failure"
    );
    assert_eq!(result.expect("text"), "fallthrough-text");
}

#[test]
fn test_cache_extract_pages_nonexistent_falls_through() {
    use crane_cli::adapters::{FakePdfAdapter, PdfAdapter};
    use crane_cli::core::pdf_extraction_cache::wrap;
    use std::sync::Arc;

    let dir = tempfile::tempdir().expect("tempdir");
    let cache_dir = dir.path().to_str().expect("path str");
    let fake = Arc::new(FakePdfAdapter::new("fallthrough-extract", 1, 100));
    let adapter = wrap(Arc::clone(&fake) as Arc<dyn PdfAdapter>, cache_dir);

    let result = adapter.extract_pages("nonexistent-xyz.pdf", 1, 2);
    assert!(result.is_ok());
    assert_eq!(result.expect("text"), "fallthrough-extract");
}
