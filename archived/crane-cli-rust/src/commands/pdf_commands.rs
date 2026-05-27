//! PDF subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::infrastructure::PdfAdapter;
use std::io::Write;

/// Serializes a value to JSON and writes it to the given writer.
///
/// # Errors
///
/// Returns `Err(String)` if serialization fails.
fn json_out<T: serde::Serialize>(writer: &mut dyn Write, val: &T) -> Result<(), String> {
    let json = serde_json::to_string(val).map_err(|e| format!("JSON serialization error: {e}"))?;
    writeln!(writer, "{json}").map_err(|e| format!("Write error: {e}"))
}

/// Runs the `crane pdf info` command, writing JSON metadata to `writer`.
///
/// Returns 0 on success, 1 on error.
pub fn run_info_inner(adapter: &dyn PdfAdapter, pdf: &str, writer: &mut dyn Write) -> i32 {
    match adapter.get_metadata(pdf) {
        Ok(meta) => {
            if json_out(writer, &meta).is_err() {
                return 1;
            }
            0
        }
        Err(msg) => {
            eprintln!("Error: {msg}");
            1
        }
    }
}

/// Runs the `crane pdf info` command, writing JSON metadata to stdout.
///
/// Returns 0 on success, 1 on error.
pub fn run_info(adapter: &dyn PdfAdapter, pdf: &str) -> i32 {
    run_info_inner(adapter, pdf, &mut std::io::stdout())
}

/// Runs the `crane pdf type` command, writing `{"type": "text"|"image"}` to `writer`.
///
/// Returns 0 for text PDFs (word count > 10), 1 for image PDFs.
pub fn run_type_inner(adapter: &dyn PdfAdapter, pdf: &str, writer: &mut dyn Write) -> i32 {
    match adapter.sample_text(pdf, 3) {
        Ok(text) => {
            let word_count = text
                .split([' ', '\n', '\t'])
                .filter(|s| !s.is_empty())
                .count();
            let doc_type = if word_count > 10 { "text" } else { "image" };
            if json_out(writer, &serde_json::json!({"type": doc_type})).is_err() {
                return 1;
            }
            i32::from(doc_type != "text")
        }
        Err(msg) => {
            eprintln!("Error: {msg}");
            1
        }
    }
}

/// Runs the `crane pdf type` command, writing to stdout.
///
/// Returns 0 for text PDFs, 1 for image PDFs.
pub fn run_type(adapter: &dyn PdfAdapter, pdf: &str) -> i32 {
    run_type_inner(adapter, pdf, &mut std::io::stdout())
}

/// Runs the `crane pdf extract` command, writing extracted text to `writer` or a file.
///
/// Returns 0 on success, 1 on error.
pub fn run_extract_inner(
    adapter: &dyn PdfAdapter,
    pdf: &str,
    start_page: usize,
    end_page: usize,
    output: Option<&str>,
    writer: &mut dyn Write,
) -> i32 {
    match adapter.extract_pages(pdf, start_page, end_page) {
        Ok(text) => {
            match output {
                Some(out_path) => {
                    if std::fs::write(out_path, &text).is_err() {
                        eprintln!("Error: failed to write output file");
                        return 1;
                    }
                }
                None => {
                    if writeln!(writer, "{text}").is_err() {
                        return 1;
                    }
                }
            }
            0
        }
        Err(msg) => {
            eprintln!("Error: {msg}");
            1
        }
    }
}

/// Runs the `crane pdf extract` command, writing to stdout or a file.
///
/// Returns 0 on success, 1 on error.
pub fn run_extract(
    adapter: &dyn PdfAdapter,
    pdf: &str,
    start_page: usize,
    end_page: usize,
    output: Option<&str>,
) -> i32 {
    run_extract_inner(
        adapter,
        pdf,
        start_page,
        end_page,
        output,
        &mut std::io::stdout(),
    )
}
