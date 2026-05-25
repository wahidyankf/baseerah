//! OCR subcommands for crane-cli.
//!
//! Note: `run_extract` uses `pdftoppm` and `tesseract` system tools and is
//! excluded from unit test coverage. Only `run_quality` is unit-tested.
#![allow(clippy::missing_docs_in_private_items)]

use crate::core::ocr_assessor::check_ocr_quality;
use std::io::Write;

/// Runs the `crane ocr quality` command, writing JSON findings to `writer`.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_quality_inner(md_text: &str, writer: &mut dyn Write) -> i32 {
    let findings = check_ocr_quality(md_text);
    let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    i32::from(!findings.is_empty())
}

/// Runs the `crane ocr quality` command, writing to stdout.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_quality(md_text: &str) -> i32 {
    run_quality_inner(md_text, &mut std::io::stdout())
}

/// Runs the `crane ocr extract` command, using pdftoppm + tesseract.
///
/// Outputs extracted text to stdout. Returns 0 on success, 1 on error.
pub fn run_extract(pdf_path: &str) -> i32 {
    // Create a unique temp directory in the system temp dir
    let tmp_base = std::env::temp_dir().join(format!(
        "crane-ocr-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos()
    ));

    if let Err(e) = std::fs::create_dir_all(&tmp_base) {
        eprintln!("Error: failed to create temp dir: {e}");
        return 1;
    }

    let prefix = tmp_base.join("page").to_string_lossy().into_owned();

    let status = std::process::Command::new("pdftoppm")
        .args(["-r", "300", "-png", pdf_path, &prefix])
        .status();

    match status {
        Ok(s) if !s.success() => {
            eprintln!("Error: pdftoppm failed");
            let _ = std::fs::remove_dir_all(&tmp_base);
            return 1;
        }
        Err(e) => {
            eprintln!("Error: failed to run pdftoppm: {e}");
            let _ = std::fs::remove_dir_all(&tmp_base);
            return 1;
        }
        Ok(_) => {}
    }

    let mut pages: Vec<std::path::PathBuf> = match std::fs::read_dir(&tmp_base) {
        Ok(entries) => entries
            .filter_map(|e| e.ok().map(|d| d.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "png"))
            .collect(),
        Err(e) => {
            eprintln!("Error: failed to read tmpdir: {e}");
            let _ = std::fs::remove_dir_all(&tmp_base);
            return 1;
        }
    };
    pages.sort();

    let mut texts = Vec::new();
    for page_path in &pages {
        let Some(path_str) = page_path.to_str() else {
            eprintln!("Error: invalid path");
            let _ = std::fs::remove_dir_all(&tmp_base);
            return 1;
        };
        match tesseract::ocr(path_str, "eng") {
            Ok(text) => texts.push(text),
            Err(e) => {
                eprintln!("Error: tesseract failed: {e}");
                let _ = std::fs::remove_dir_all(&tmp_base);
                return 1;
            }
        }
    }

    let _ = std::fs::remove_dir_all(&tmp_base);
    println!("{}", texts.join("\n\n"));
    0
}
