// Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/jacoco_coverage.go`.
// JaCoCo XML schema: <report><package name=...><sourcefile name=...><line nr,mi,ci,mb,cb/>...
// Classification: ci > 0 && mb == 0 → covered; ci > 0 && mb > 0 → partial; ci == 0 → missed.

use std::fs;

use anyhow::{Error, anyhow};
use serde::Deserialize;

use super::types::{FileResult, Format, Result as CoverageResult};

#[derive(Debug, Deserialize, Default)]
pub(crate) struct JacocoReport {
    #[serde(rename = "package", default)]
    pub packages: Vec<JacocoPackage>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct JacocoPackage {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "sourcefile", default)]
    pub source_files: Vec<JacocoSourceFile>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct JacocoSourceFile {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "line", default)]
    pub lines: Vec<JacocoLine>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct JacocoLine {
    #[serde(rename = "@nr", default)]
    pub nr: i64,
    #[serde(rename = "@mi", default)]
    pub _mi: i64,
    #[serde(rename = "@ci", default)]
    pub ci: i64,
    #[serde(rename = "@mb", default)]
    pub mb: i64,
    #[serde(rename = "@cb", default)]
    pub cb: i64,
}

pub(crate) fn parse_jacoco(filename: &str) -> std::result::Result<JacocoReport, Error> {
    let data = fs::read_to_string(filename).map_err(|_| anyhow!("file not found: {filename}"))?;
    let report: JacocoReport =
        quick_xml::de::from_str(&data).map_err(|e| anyhow!("invalid JaCoCo XML: {e}"))?;
    Ok(report)
}

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
