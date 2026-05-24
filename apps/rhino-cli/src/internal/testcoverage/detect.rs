// Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/detect.go`.
//
// Detection priority:
//   1. Filename-based: .info / contains "lcov" → LCOV; .xml + "jacoco" → JaCoCo; .xml + "cobertura" → Cobertura
//   2. Content-based:  mode: → Go; SF:/TN: → LCOV; <report → JaCoCo; <coverage → Cobertura
//   3. Fallback: Go

use std::fs::File;
use std::io::{BufRead, BufReader};

use super::types::Format;

pub fn detect_format(filename: &str) -> Format {
    let lower = filename.to_lowercase();

    if lower.ends_with(".info") || lower.contains("lcov") {
        return Format::Lcov;
    }
    if lower.ends_with(".xml") && lower.contains("jacoco") {
        return Format::Jacoco;
    }
    if lower.ends_with(".xml") && lower.contains("cobertura") {
        return Format::Cobertura;
    }

    let Ok(file) = File::open(filename) else {
        return Format::Go;
    };

    for raw in BufReader::new(file).lines() {
        let Ok(line) = raw else { continue };
        let mut s = line.trim().to_string();
        if s.is_empty() {
            continue;
        }
        if s.starts_with("mode:") {
            return Format::Go;
        }
        if s.starts_with("SF:") || s.starts_with("TN:") {
            return Format::Lcov;
        }
        if s.starts_with("<!DOCTYPE") {
            continue;
        }
        if s.starts_with("<?xml") {
            if let Some(idx) = s.find("?>") {
                let rest = s[idx + 2..].trim();
                if rest.is_empty() {
                    continue;
                }
                s = rest.to_string();
            } else {
                continue;
            }
        }
        if s.starts_with("<report") {
            return Format::Jacoco;
        }
        if s.starts_with("<coverage") {
            return Format::Cobertura;
        }
        break;
    }

    Format::Go
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
    fn detect_by_filename_lcov_info() {
        assert_eq!(detect_format("/tmp/coverage.info"), Format::Lcov);
        assert_eq!(detect_format("/tmp/lcov-report.dat"), Format::Lcov);
    }

    #[test]
    fn detect_by_filename_jacoco_xml() {
        assert_eq!(detect_format("/tmp/jacoco.xml"), Format::Jacoco);
        assert_eq!(detect_format("/tmp/jacoco-coverage.xml"), Format::Jacoco);
    }

    #[test]
    fn detect_by_filename_cobertura_xml() {
        assert_eq!(detect_format("/tmp/cobertura.xml"), Format::Cobertura);
        assert_eq!(
            detect_format("/tmp/cobertura-report.xml"),
            Format::Cobertura
        );
    }

    #[test]
    fn detect_by_content_go_mode() {
        let tmp = TempDir::new().unwrap();
        let p = write(tmp.path(), "cover.out", "mode: set\nfoo.go:1.1,2.2 1 1\n");
        assert_eq!(detect_format(p.to_str().unwrap()), Format::Go);
    }

    #[test]
    fn detect_by_content_lcov_sf() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "out.txt",
            "SF:src/foo.rs\nDA:1,1\nend_of_record\n",
        );
        assert_eq!(detect_format(p.to_str().unwrap()), Format::Lcov);
    }

    #[test]
    fn detect_by_content_lcov_tn() {
        let tmp = TempDir::new().unwrap();
        let p = write(tmp.path(), "out.txt", "TN:foo\nSF:bar.rs\n");
        assert_eq!(detect_format(p.to_str().unwrap()), Format::Lcov);
    }

    #[test]
    fn detect_by_content_jacoco_root() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "out.txt",
            "<?xml version=\"1.0\"?>\n<report>\n</report>",
        );
        assert_eq!(detect_format(p.to_str().unwrap()), Format::Jacoco);
    }

    #[test]
    fn detect_by_content_cobertura_root() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "out.txt",
            "<?xml version=\"1.0\"?>\n<coverage>\n</coverage>",
        );
        assert_eq!(detect_format(p.to_str().unwrap()), Format::Cobertura);
    }

    #[test]
    fn detect_by_content_root_on_same_line_as_xml_decl() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "out.txt",
            "<?xml version=\"1.0\"?><report></report>",
        );
        assert_eq!(detect_format(p.to_str().unwrap()), Format::Jacoco);
    }

    #[test]
    fn detect_fallback_go_when_unrecognized() {
        let tmp = TempDir::new().unwrap();
        let p = write(tmp.path(), "out.txt", "unknown content\n");
        assert_eq!(detect_format(p.to_str().unwrap()), Format::Go);
    }

    #[test]
    fn detect_fallback_go_when_file_missing() {
        assert_eq!(detect_format("/nonexistent/file"), Format::Go);
    }
}
