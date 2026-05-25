//! Report manager — creates and finalizes audit report files.
#![allow(clippy::missing_docs_in_private_items)]

use anyhow::{Context, Result};
use chrono::{FixedOffset, Utc};
use std::io::Write;

/// Chain window in seconds — if the chain file is older than this, start fresh.
const CHAIN_WINDOW_SECONDS: i64 = 30;

/// UTC+7 offset in seconds.
const UTC7_OFFSET_SECS: i32 = 7 * 3600;

fn chain_file_path(scope: &str) -> String {
    format!(".execution-chain-{scope}")
}

fn new_id() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    format!("{:06x}", nanos % 0x100_0000)
}

/// Gets or extends the execution chain for a given scope.
///
/// Reads `.execution-chain-<scope>` from the current directory. If the file
/// exists and is fresh (< 30 seconds old), extends the chain with a new 6-hex
/// ID. Otherwise starts a new chain.
pub fn get_or_extend_chain(scope: &str) -> String {
    let chain_file = chain_file_path(scope);
    let new_id = new_id();

    let existing_chain = std::fs::read_to_string(&chain_file)
        .ok()
        .and_then(|content| {
            let content = content.trim().to_string();
            let (ts_str, chain_str) = content.split_once(' ')?;
            let ts: i64 = ts_str.parse().ok()?;
            let now = Utc::now().timestamp();
            if now - ts < CHAIN_WINDOW_SECONDS {
                Some(format!("{chain_str}__{new_id}"))
            } else {
                None
            }
        });

    let chain = existing_chain.unwrap_or_else(|| new_id.clone());
    let now_ts = Utc::now().timestamp();
    let _ = std::fs::write(&chain_file, format!("{now_ts} {chain}"));
    chain
}

/// Returns the current time formatted in UTC+7 as `yyyy-MM-dd--HH-mm`.
///
/// # Panics
///
/// Panics if the UTC+0 offset cannot be constructed (unreachable in practice).
pub fn utc7_timestamp() -> String {
    let offset = FixedOffset::east_opt(UTC7_OFFSET_SECS)
        .unwrap_or_else(|| FixedOffset::east_opt(0).expect("zero offset"));
    let now = Utc::now().with_timezone(&offset);
    now.format("%Y-%m-%d--%H-%M").to_string()
}

/// Initializes an audit report in `report_dir`.
///
/// Creates `<report_dir>/<scope>__<chain>__<ts>__audit.md` and writes a header.
/// Returns the path to the created file.
///
/// # Errors
///
/// Returns an error if the directory cannot be created or the file cannot be
/// written.
pub fn init_report_in(scope: &str, pdf: &str, md: &str, report_dir: &str) -> Result<String> {
    let chain = get_or_extend_chain(scope);
    let ts = utc7_timestamp();
    let report_path = format!("{report_dir}/{scope}__{chain}__{ts}__audit.md");

    std::fs::create_dir_all(report_dir)
        .with_context(|| format!("Failed to create report dir: {report_dir}"))?;

    let header =
        format!("# Audit Report\n\nScope: {scope}\nPDF: {pdf}\nMD: {md}\nStatus: IN_PROGRESS\n");
    std::fs::write(&report_path, header)
        .with_context(|| format!("Failed to write report: {report_path}"))?;

    Ok(report_path)
}

/// Initializes an audit report in `generated-reports/`.
///
/// # Errors
///
/// Returns an error if the report cannot be created.
pub fn init_report(scope: &str, pdf: &str, md: &str) -> Result<String> {
    init_report_in(scope, pdf, md, "generated-reports")
}

/// Finalizes an audit report by replacing `Status: IN_PROGRESS` with the given status.
///
/// # Errors
///
/// Returns an error if the report file cannot be read or written.
pub fn finalize_report(report_path: &str, status: &str) -> Result<()> {
    if !std::path::Path::new(report_path).exists() {
        anyhow::bail!("Report not found: {report_path}");
    }

    let content = std::fs::read_to_string(report_path)
        .with_context(|| format!("Failed to read report: {report_path}"))?;
    let updated = content.replace("Status: IN_PROGRESS", &format!("Status: {status}"));

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(report_path)
        .with_context(|| format!("Failed to open report for writing: {report_path}"))?;
    file.write_all(updated.as_bytes())
        .with_context(|| format!("Failed to write report: {report_path}"))?;

    Ok(())
}
