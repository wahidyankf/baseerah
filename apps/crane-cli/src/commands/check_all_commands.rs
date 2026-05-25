//! Check-all aggregator command for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::adapters::PdfAdapter;
use crate::core::figure_checker::check_figures;
use crate::core::heading_checker::check_headings;
use crate::core::mermaid_validator::validate_md;
use crate::core::nesting_checker::check_nesting;
use crate::core::table_checker::check_tables;
use crate::core::text_checker::check_text;
use std::io::Write;

/// Runs all check dimensions on a PDF+MD pair, writing JSON findings to `writer`.
///
/// Returns 0 if no findings across all dimensions, 1 if any findings exist or
/// on error.
pub fn run_check_all_inner(
    adapter: &dyn PdfAdapter,
    pdf_path: &str,
    md_text: &str,
    writer: &mut dyn Write,
) -> i32 {
    match adapter.sample_text(pdf_path, 999) {
        Ok(pdf_text) => {
            let chunks: Vec<&str> = pdf_text
                .split('\n')
                .filter(|s| s.trim().len() > 10)
                .collect();

            let mut findings = Vec::new();
            findings.extend(check_text(&chunks, md_text));
            findings.extend(check_headings(&pdf_text, md_text));
            findings.extend(check_nesting(&pdf_text, md_text));
            findings.extend(check_tables(&pdf_text, md_text));
            findings.extend(check_figures(&pdf_text, md_text));
            findings.extend(validate_md(md_text));

            let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
            let _ = writeln!(writer, "{json}");
            i32::from(!findings.is_empty())
        }
        Err(msg) => {
            eprintln!("Error: {msg}");
            1
        }
    }
}

/// Runs all check dimensions on a PDF+MD pair, writing to stdout.
///
/// Returns 0 if no findings, 1 if any findings exist or on error.
pub fn run_check_all(adapter: &dyn PdfAdapter, pdf_path: &str, md_text: &str) -> i32 {
    run_check_all_inner(adapter, pdf_path, md_text, &mut std::io::stdout())
}
