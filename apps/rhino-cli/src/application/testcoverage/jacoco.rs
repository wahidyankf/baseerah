//! JaCoCo XML coverage format parser and result computer.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/jacoco_coverage.go`.
//!
//! JaCoCo XML schema: `<report><package name=…><sourcefile name=…><line nr,mi,ci,mb,cb/>…`
//!
//! Line classification:
//!
//! - `ci > 0` and `mb == 0` → **covered**
//! - `ci > 0` and `mb > 0` → **partial**
//! - `ci == 0` → **missed**

use std::fs;

use anyhow::{Error, anyhow};
use serde::Deserialize;

use super::types::{FileResult, Format, Result as CoverageResult};

/// Top-level deserialization target for a `JaCoCo` XML report.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct JacocoReport {
    /// List of `<package>` child elements.
    #[serde(rename = "package", default)]
    pub packages: Vec<JacocoPackage>,
}

/// A `<package>` element grouping source files belonging to one Java package.
#[derive(Debug, Deserialize)]
pub(crate) struct JacocoPackage {
    /// Package name in slash-separated form (e.g. `com/example`).
    #[serde(rename = "@name")]
    pub name: String,
    /// List of `<sourcefile>` child elements.
    #[serde(rename = "sourcefile", default)]
    pub source_files: Vec<JacocoSourceFile>,
}

/// A `<sourcefile>` element representing one source file in a `JaCoCo` report.
#[derive(Debug, Deserialize)]
pub(crate) struct JacocoSourceFile {
    /// Base filename (e.g. `Foo.java`); prepend the package name to get the full path.
    #[serde(rename = "@name")]
    pub name: String,
    /// List of `<line>` child elements with per-line coverage data.
    #[serde(rename = "line", default)]
    pub lines: Vec<JacocoLine>,
}

/// A `<line>` element in a `JaCoCo` report with instruction and branch counters.
#[derive(Debug, Deserialize)]
pub(crate) struct JacocoLine {
    /// Line number in the source file (`nr` attribute).
    #[serde(rename = "@nr", default)]
    pub nr: i64,
    /// Missed instruction count (`mi` attribute; unused in coverage computation).
    #[serde(rename = "@mi", default)]
    pub _mi: i64,
    /// Covered instruction count (`ci` attribute). `ci > 0` means the line was executed.
    #[serde(rename = "@ci", default)]
    pub ci: i64,
    /// Missed branch count (`mb` attribute). `mb > 0` signals a partially covered branch.
    #[serde(rename = "@mb", default)]
    pub mb: i64,
    /// Covered branch count (`cb` attribute).
    #[serde(rename = "@cb", default)]
    pub cb: i64,
}

/// Reads and parses a `JaCoCo` XML file from `filename`.
///
/// # Errors
///
/// Returns an error when the file cannot be read or the XML is malformed.
pub(crate) fn parse_jacoco(filename: &str) -> std::result::Result<JacocoReport, Error> {
    let data = fs::read_to_string(filename).map_err(|_| anyhow!("file not found: {filename}"))?;
    let report: JacocoReport =
        quick_xml::de::from_str(&data).map_err(|e| anyhow!("invalid JaCoCo XML: {e}"))?;
    Ok(report)
}

/// Parses `filename` as a `JaCoCo` XML report and computes aggregated coverage.
///
/// # Errors
///
/// Returns an error when `parse_jacoco` fails (file not found or invalid XML).
pub fn compute_jacoco_result(
    filename: &str,
    threshold: f64,
) -> std::result::Result<CoverageResult, Error> {
    let report = parse_jacoco(filename)?;

    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    let mut per_file: Vec<FileResult> = Vec::new();

    for pkg in &report.packages {
        for sf in &pkg.source_files {
            let mut fc = 0usize;
            let mut fp = 0usize;
            let mut fm = 0usize;
            for line in &sf.lines {
                if line.ci > 0 {
                    if line.mb > 0 {
                        fp += 1;
                    } else {
                        fc += 1;
                    }
                } else {
                    fm += 1;
                }
            }
            covered += fc;
            partial += fp;
            missed += fm;

            let ft = fc + fp + fm;
            let fpct = if ft > 0 {
                100.0 * fc as f64 / ft as f64
            } else {
                100.0
            };
            per_file.push(FileResult {
                path: format!("{}/{}", pkg.name, sf.name),
                covered: fc,
                partial: fp,
                missed: fm,
                total: ft,
                pct: fpct,
            });
        }
    }

    let total = covered + partial + missed;
    let pct = if total > 0 {
        100.0 * covered as f64 / total as f64
    } else {
        100.0
    };

    Ok(CoverageResult {
        file: filename.to_string(),
        format: Format::Jacoco,
        covered,
        partial,
        missed,
        total,
        pct,
        threshold,
        passed: pct >= threshold,
        files: per_file,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        std::fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn parse_jacoco_file_not_found() {
        let err = parse_jacoco("/nonexistent/jacoco.xml").unwrap_err();
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn parse_jacoco_invalid_xml() {
        let tmp = TempDir::new().unwrap();
        let p = write(tmp.path(), "j.xml", "<not></valid>");
        let err = parse_jacoco(p.to_str().unwrap()).unwrap_err();
        assert!(err.to_string().contains("invalid JaCoCo XML"));
    }

    #[test]
    fn compute_jacoco_all_covered_passes() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<report>
  <package name="com/example">
    <sourcefile name="Foo.java">
      <line nr="10" mi="0" ci="1" mb="0" cb="0"/>
      <line nr="11" mi="0" ci="3" mb="0" cb="0"/>
    </sourcefile>
  </package>
</report>"#;
        let p = write(tmp.path(), "j.xml", xml);
        let result = compute_jacoco_result(p.to_str().unwrap(), 90.0).unwrap();
        assert_eq!(result.format, Format::Jacoco);
        assert_eq!(result.covered, 2);
        assert_eq!(result.missed, 0);
        assert!(result.passed);
        assert_eq!(result.files[0].path, "com/example/Foo.java");
    }

    #[test]
    fn compute_jacoco_partial_when_ci_pos_and_mb_pos() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<report>
  <package name="com/example">
    <sourcefile name="Foo.java">
      <line nr="10" mi="0" ci="1" mb="2" cb="1"/>
    </sourcefile>
  </package>
</report>"#;
        let p = write(tmp.path(), "j.xml", xml);
        let result = compute_jacoco_result(p.to_str().unwrap(), 50.0).unwrap();
        assert_eq!(result.partial, 1);
        assert_eq!(result.covered, 0);
        assert_eq!(result.missed, 0);
    }

    #[test]
    fn compute_jacoco_missed_when_ci_zero() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<report>
  <package name="com/example">
    <sourcefile name="Foo.java">
      <line nr="10" mi="3" ci="0" mb="0" cb="0"/>
    </sourcefile>
  </package>
</report>"#;
        let p = write(tmp.path(), "j.xml", xml);
        let result = compute_jacoco_result(p.to_str().unwrap(), 90.0).unwrap();
        assert_eq!(result.missed, 1);
        assert!(!result.passed);
    }

    #[test]
    fn compute_jacoco_empty_report() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><report></report>"#;
        let p = write(tmp.path(), "j.xml", xml);
        let result = compute_jacoco_result(p.to_str().unwrap(), 90.0).unwrap();
        assert_eq!(result.total, 0);
        assert!((result.pct - 100.0).abs() < 0.001);
    }
}
