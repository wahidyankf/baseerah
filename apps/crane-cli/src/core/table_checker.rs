//! Table integrity checker — verifies tables are present and correctly structured.
#![allow(clippy::missing_docs_in_private_items)]

use crate::models::Finding;
use regex::Regex;
use std::sync::OnceLock;

/// Compiled pipe-column pattern.
fn pipe_pattern() -> &'static Regex {
    static PAT: OnceLock<Regex> = OnceLock::new();
    PAT.get_or_init(|| Regex::new(r"\|[^|]+").expect("static regex"))
}

fn is_separator_line(line: &str) -> bool {
    line.contains("---") || line.contains("===")
}

fn is_table_row(line: &str) -> bool {
    pipe_pattern().find_iter(line).count() >= 2
}

/// A detected table specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TableSpec {
    /// Number of rows including the header row.
    pub row_count: usize,
    /// Number of columns.
    pub col_count: usize,
    /// The header row text.
    pub header_row: String,
}

/// Detects pipe-style Markdown tables in layout text.
///
/// A table is identified by a header row (≥2 pipe-separated columns) followed
/// immediately by a separator line (`---` or `===`).
pub fn detect_tables(layout_text: &str) -> Vec<TableSpec> {
    let lines: Vec<&str> = layout_text.split('\n').collect();
    let mut results = Vec::new();
    let mut i = 0;

    while i + 1 < lines.len() {
        let line = lines[i];
        let next = lines[i + 1];
        let cols = pipe_pattern().find_iter(line).count();

        if cols >= 2 && is_separator_line(next) {
            let mut data_rows = 0;
            let mut j = i + 2;
            while j < lines.len() && is_table_row(lines[j]) {
                data_rows += 1;
                j += 1;
            }
            results.push(TableSpec {
                row_count: data_rows + 1, // header counts as 1
                col_count: cols,
                header_row: line.to_string(),
            });
            i = j;
        } else {
            i += 1;
        }
    }

    results
}

/// Checks table integrity between PDF layout text and Markdown.
///
/// Returns findings for tables missing entirely (CRITICAL) or with mismatched
/// row counts (MEDIUM).
pub fn check_tables(pdf_layout_text: &str, md_text: &str) -> Vec<Finding> {
    let pdf_tables = detect_tables(pdf_layout_text);
    let md_tables = detect_tables(md_text);

    pdf_tables
        .iter()
        .filter_map(|pdf_table| {
            let md_match = md_tables
                .iter()
                .find(|t| t.col_count == pdf_table.col_count);

            match md_match {
                None => Some(Finding {
                    category: "table-integrity".to_string(),
                    criticality: "CRITICAL".to_string(),
                    confidence: "HIGH".to_string(),
                    location_pdf: Some(pdf_table.header_row.clone()),
                    location_md: None,
                    description: format!("Missing table with {} columns", pdf_table.col_count),
                    pdf_text: Some(pdf_table.header_row.clone()),
                    fix_suggestion: Some("Add the missing table to the Markdown".to_string()),
                    auto_fixable: false,
                }),
                Some(md_table)
                    if md_table.row_count != pdf_table.row_count && pdf_table.row_count > 1 =>
                {
                    Some(Finding {
                        category: "table-integrity".to_string(),
                        criticality: "MEDIUM".to_string(),
                        confidence: "MEDIUM".to_string(),
                        location_pdf: Some(pdf_table.header_row.clone()),
                        location_md: Some(md_table.header_row.clone()),
                        description: format!(
                            "Table row count mismatch: PDF {}, MD {}",
                            pdf_table.row_count, md_table.row_count
                        ),
                        pdf_text: None,
                        fix_suggestion: Some("Verify row count matches source PDF".to_string()),
                        auto_fixable: false,
                    })
                }
                _ => None,
            }
        })
        .collect()
}
