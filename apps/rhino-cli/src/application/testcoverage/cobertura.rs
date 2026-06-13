//! Cobertura XML coverage format parser and result computer.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/cobertura_coverage.go`.
//!
//! Cobertura XML schema:
//! `<coverage><packages><package><classes><class filename="…"><lines><line number hits branch condition-coverage/>`
//!
//! Line classification:
//!
//! - `hits > 0` and (`!branch` or all branches covered) → **covered**
//! - `hits > 0` and `branch` and some uncovered branches → **partial**
//! - `hits == 0` → **missed**

use std::collections::BTreeMap;
use std::fs;

use anyhow::{Error, anyhow};
use serde::Deserialize;

use super::types::{FileResult, Format, Result as CoverageResult};

/// Top-level deserialization target for a Cobertura XML report.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct CoberturaReport {
    /// Optional `<packages>` element; absent in empty reports.
    #[serde(default)]
    pub packages: Option<CoberturaPackages>,
}

/// The `<packages>` container element in a Cobertura XML report.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct CoberturaPackages {
    /// List of `<package>` child elements.
    #[serde(rename = "package", default)]
    pub package: Vec<CoberturaPackage>,
}

/// A `<package>` element grouping related classes.
#[derive(Debug, Deserialize)]
pub(crate) struct CoberturaPackage {
    /// The `name` XML attribute (unused in coverage computation; kept for completeness).
    #[serde(rename = "@name", default)]
    pub _name: String,
    /// Optional `<classes>` child element.
    #[serde(default)]
    pub classes: Option<CoberturaClasses>,
}

/// The `<classes>` container element inside a `<package>`.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct CoberturaClasses {
    /// List of `<class>` child elements.
    #[serde(rename = "class", default)]
    pub class: Vec<CoberturaClass>,
}

/// A `<class>` element representing a single source file.
#[derive(Debug, Deserialize)]
pub(crate) struct CoberturaClass {
    /// The `filename` XML attribute — path to the source file.
    #[serde(rename = "@filename")]
    pub filename: String,
    /// Optional `<lines>` child element containing line-level data.
    #[serde(default)]
    pub lines: Option<CoberturaLines>,
}

/// The `<lines>` container element inside a `<class>`.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct CoberturaLines {
    /// List of `<line>` child elements.
    #[serde(rename = "line", default)]
    pub line: Vec<CoberturaLine>,
}

/// A single `<line>` element with hit-count and optional branch data.
#[derive(Debug, Deserialize)]
pub(crate) struct CoberturaLine {
    /// Line number in the source file.
    #[serde(rename = "@number", default)]
    pub number: i64,
    /// Number of times this line was executed.
    #[serde(rename = "@hits", default)]
    pub hits: i64,
    /// `true` when this line has branch coverage data.
    #[serde(rename = "@branch", default)]
    pub branch: bool,
    /// Raw `condition-coverage` attribute string, e.g. `"50% (1/2)"`.
    #[serde(rename = "@condition-coverage", default)]
    pub condition_coverage: String,
}

/// Reads and parses a Cobertura XML file from `filename`.
///
/// # Errors
///
/// Returns an error when the file cannot be read or the XML is malformed.
pub(crate) fn parse_cobertura(filename: &str) -> std::result::Result<CoberturaReport, Error> {
    let data = fs::read_to_string(filename).map_err(|_| anyhow!("file not found: {filename}"))?;
    let report: CoberturaReport =
        quick_xml::de::from_str(&data).map_err(|e| anyhow!("invalid Cobertura XML: {e}"))?;
    Ok(report)
}

/// Extracts (covered, total) from a `condition-coverage` attribute like "50% (1/2)".
pub(crate) fn parse_branch_coverage(cond_cov: &str) -> (i64, i64) {
    let Some(start) = cond_cov.find('(') else {
        return (0, 0);
    };
    let Some(end) = cond_cov.find(')') else {
        return (0, 0);
    };
    if end <= start {
        return (0, 0);
    }
    let fraction = &cond_cov[start + 1..end];
    let parts: Vec<&str> = fraction.splitn(2, '/').collect();
    if parts.len() != 2 {
        return (0, 0);
    }
    let c = parts[0].parse::<i64>().unwrap_or(0);
    let t = parts[1].parse::<i64>().unwrap_or(0);
    if c == 0 && parts[0] != "0" {
        return (0, 0);
    }
    if t == 0 && parts[1] != "0" {
        return (0, 0);
    }
    (c, t)
}

/// Accumulator for covered/partial/missed line counts within a single source file.
#[derive(Default)]
struct FileCounts {
    /// Number of fully covered lines.
    c: usize,
    /// Number of partially covered lines (executed but with uncovered branches).
    p: usize,
    /// Number of missed (unexecuted) lines.
    m: usize,
}

/// Parses `filename` as a Cobertura XML report and computes aggregated coverage.
///
/// # Errors
///
/// Returns an error when `parse_cobertura` fails (file not found or invalid XML).
pub fn compute_cobertura_result(
    filename: &str,
    threshold: f64,
) -> std::result::Result<CoverageResult, Error> {
    let report = parse_cobertura(filename)?;

    let mut file_map: BTreeMap<String, FileCounts> = BTreeMap::new();

    if let Some(packages) = report.packages.as_ref() {
        for pkg in &packages.package {
            if let Some(classes) = pkg.classes.as_ref() {
                for cls in &classes.class {
                    let counts = file_map.entry(cls.filename.clone()).or_default();
                    if let Some(lines) = cls.lines.as_ref() {
                        for line in &lines.line {
                            if line.hits > 0 {
                                if line.branch {
                                    let (br_cov, br_total) =
                                        parse_branch_coverage(&line.condition_coverage);
                                    if br_total > 0 && br_cov < br_total {
                                        counts.p += 1;
                                    } else {
                                        counts.c += 1;
                                    }
                                } else {
                                    counts.c += 1;
                                }
                            } else {
                                counts.m += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    let mut per_file: Vec<FileResult> = Vec::new();
    for (path, fc) in file_map {
        covered += fc.c;
        partial += fc.p;
        missed += fc.m;
        let ft = fc.c + fc.p + fc.m;
        let fpct = if ft > 0 {
            100.0 * fc.c as f64 / ft as f64
        } else {
            100.0
        };
        per_file.push(FileResult {
            path,
            covered: fc.c,
            partial: fc.p,
            missed: fc.m,
            total: ft,
            pct: fpct,
        });
    }

    let total = covered + partial + missed;
    let pct = if total > 0 {
        100.0 * covered as f64 / total as f64
    } else {
        100.0
    };

    Ok(CoverageResult {
        file: filename.to_string(),
        format: Format::Cobertura,
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
    fn parse_cobertura_file_not_found() {
        let err = parse_cobertura("/nonexistent/cob.xml").unwrap_err();
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn parse_branch_coverage_typical() {
        assert_eq!(parse_branch_coverage("50% (1/2)"), (1, 2));
        assert_eq!(parse_branch_coverage("100% (4/4)"), (4, 4));
        assert_eq!(parse_branch_coverage("0% (0/3)"), (0, 3));
    }

    #[test]
    fn parse_branch_coverage_malformed() {
        assert_eq!(parse_branch_coverage(""), (0, 0));
        assert_eq!(parse_branch_coverage("50%"), (0, 0));
        assert_eq!(parse_branch_coverage("(invalid)"), (0, 0));
    }

    #[test]
    fn compute_cobertura_all_covered_passes() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0"?>
<coverage>
  <packages>
    <package name="pkg">
      <classes>
        <class filename="src/foo.py">
          <lines>
            <line number="1" hits="5" branch="false"/>
            <line number="2" hits="3" branch="false"/>
          </lines>
        </class>
      </classes>
    </package>
  </packages>
</coverage>"#;
        let p = write(tmp.path(), "c.xml", xml);
        let result = compute_cobertura_result(p.to_str().unwrap(), 90.0).unwrap();
        assert_eq!(result.format, Format::Cobertura);
        assert_eq!(result.covered, 2);
        assert!(result.passed);
    }

    #[test]
    fn compute_cobertura_branch_partial() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0"?>
<coverage>
  <packages>
    <package name="pkg">
      <classes>
        <class filename="src/foo.py">
          <lines>
            <line number="10" hits="5" branch="true" condition-coverage="50% (1/2)"/>
          </lines>
        </class>
      </classes>
    </package>
  </packages>
</coverage>"#;
        let p = write(tmp.path(), "c.xml", xml);
        let result = compute_cobertura_result(p.to_str().unwrap(), 50.0).unwrap();
        assert_eq!(result.partial, 1);
        assert_eq!(result.covered, 0);
    }

    #[test]
    fn compute_cobertura_missed_when_hits_zero() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0"?>
<coverage>
  <packages>
    <package name="pkg">
      <classes>
        <class filename="src/foo.py">
          <lines>
            <line number="10" hits="0" branch="false"/>
          </lines>
        </class>
      </classes>
    </package>
  </packages>
</coverage>"#;
        let p = write(tmp.path(), "c.xml", xml);
        let result = compute_cobertura_result(p.to_str().unwrap(), 90.0).unwrap();
        assert_eq!(result.missed, 1);
        assert!(!result.passed);
    }

    #[test]
    fn compute_cobertura_branch_full_coverage_is_covered() {
        let tmp = TempDir::new().unwrap();
        let xml = r#"<?xml version="1.0"?>
<coverage>
  <packages>
    <package name="pkg">
      <classes>
        <class filename="src/foo.py">
          <lines>
            <line number="10" hits="5" branch="true" condition-coverage="100% (2/2)"/>
          </lines>
        </class>
      </classes>
    </package>
  </packages>
</coverage>"#;
        let p = write(tmp.path(), "c.xml", xml);
        let result = compute_cobertura_result(p.to_str().unwrap(), 50.0).unwrap();
        assert_eq!(result.covered, 1);
        assert_eq!(result.partial, 0);
    }
}
