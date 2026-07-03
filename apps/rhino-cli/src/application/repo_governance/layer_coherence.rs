//! Layer coherence audit across governance documents.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/layer_coherence.go`.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::sync::OnceLock;

use anyhow::Error;
use regex::Regex;

use crate::application::fs::port::Fs;

/// A single finding emitted by the layer coherence audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerCoherenceFinding {
    /// Path (or composite path `a+b`) of the offending file(s).
    pub file: String,
    /// Severity; currently always `"fail"`.
    pub severity: String,
    /// Machine-readable violation category (one of the `KIND_*` constants).
    pub kind: String,
    /// Human-readable description of the violation.
    pub message: String,
}

/// Finding kind: the same layer number is declared with two different names
/// within a single file.
pub const KIND_INTRA_FILE_NAME_CONFLICT: &str = "intra-file-name-conflict";
/// Finding kind: a layer number exists in one governance document but is absent
/// from the other.
pub const KIND_CROSS_FILE_NUMBER_MISMATCH: &str = "cross-file-number-mismatch";
/// Finding kind: the same layer number has different names in the two documents.
pub const KIND_CROSS_FILE_NAME_MISMATCH: &str = "cross-file-name-mismatch";
/// Finding kind: an integer in the range `[0, max_layer]` is not declared in
/// either document.
pub const KIND_NUMBERING_GAP: &str = "numbering-gap";
/// Finding kind: a required governance document does not exist on disk.
pub const KIND_MISSING_DOC: &str = "missing-doc";

/// Relative path to the repository governance architecture document.
const ARCH_PATH: &str = "repo-governance/repository-governance-architecture.md";
/// Relative path to the repo-governance README.
const README_PATH: &str = "repo-governance/README.md";

/// Returns a compiled `Regex` matching bold layer declarations such as
/// `**Layer 0: Vision**`.
fn bold_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"\*\*Layer (\d+):\s*([A-Za-z][A-Za-z0-9 -]+?)\*\*")
            .expect("valid hardcoded regex")
    })
}

/// Returns a compiled `Regex` matching ATX heading layer declarations such as
/// `## Layer 0: Vision (the why)`.
fn head_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?m)^##\s+Layer (\d+):\s*([A-Za-z][A-Za-z0-9 -]+?)\s*\(")
            .expect("valid hardcoded regex")
    })
}

/// Audits that the governance layer declarations in
/// `repo-governance/repository-governance-architecture.md` and
/// `repo-governance/README.md` are mutually consistent and contiguous.
///
/// Findings are sorted by `file`, then by `kind`.
///
/// # Errors
///
/// Returns an error when either document cannot be read (other than
/// `NotFound`, which is reported as a `KIND_MISSING_DOC` finding).
pub fn audit_layer_coherence(
    fs: &dyn Fs,
    repo_root: &Path,
) -> std::result::Result<Vec<LayerCoherenceFinding>, Error> {
    let arch_path = repo_root.join(ARCH_PATH);
    let readme_path = repo_root.join(README_PATH);
    let arch_path_s = arch_path.to_string_lossy().to_string();
    let readme_path_s = readme_path.to_string_lossy().to_string();

    let mut findings = Vec::new();
    let (arch_map, mut arch_findings) = read_layer_map(fs, &arch_path_s)?;
    findings.append(&mut arch_findings);
    let (readme_map, mut readme_findings) = read_layer_map(fs, &readme_path_s)?;
    findings.append(&mut readme_findings);

    if let (Some(am), Some(rm)) = (arch_map.as_ref(), readme_map.as_ref()) {
        findings.extend(compare_layer_maps(am, rm, &arch_path_s, &readme_path_s));
        findings.extend(check_numbering_gap(am, rm, &arch_path_s, &readme_path_s));
    }

    findings.sort_by(|a, b| a.file.cmp(&b.file).then(a.kind.cmp(&b.kind)));
    Ok(findings)
}

/// Map from layer number to layer name extracted from a single document.
type LayerMap = HashMap<i64, String>;
/// Result type for [`read_layer_map`]: the optional map plus any intra-file
/// findings.
type LayerMapResult = std::result::Result<(Option<LayerMap>, Vec<LayerCoherenceFinding>), Error>;

/// Reads the layer declarations from the Markdown document at `path` and
/// returns a [`LayerMap`] together with any intra-file name-conflict findings.
///
/// Returns `(None, findings)` when the file does not exist (recorded as a
/// `KIND_MISSING_DOC` finding).
///
/// # Errors
///
/// Returns an error when the file exists but cannot be read.
fn read_layer_map(fs: &dyn Fs, path: &str) -> LayerMapResult {
    let data = match fs.read_to_string(Path::new(path)) {
        Ok(d) => d,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok((
                None,
                vec![LayerCoherenceFinding {
                    file: path.to_string(),
                    severity: "fail".to_string(),
                    kind: KIND_MISSING_DOC.to_string(),
                    message: format!("governance doc \"{path}\" does not exist"),
                }],
            ));
        }
        Err(e) => return Err(Error::msg(format!("read {path}: {e}"))),
    };

    let mut layers: HashMap<i64, String> = HashMap::new();
    let mut findings = Vec::new();

    let add_match = |num_str: &str,
                     name: &str,
                     layers: &mut HashMap<i64, String>,
                     findings: &mut Vec<LayerCoherenceFinding>| {
        let num: i64 = match num_str.parse() {
            Ok(n) => n,
            Err(_) => return,
        };
        let trimmed = name.to_string();
        if let Some(existing) = layers.get(&num) {
            if *existing != trimmed {
                findings.push(LayerCoherenceFinding {
                    file: path.to_string(),
                    severity: "fail".to_string(),
                    kind: KIND_INTRA_FILE_NAME_CONFLICT.to_string(),
                    message: format!(
                        "file declares Layer {num} with two different names: \"{existing}\" and \"{trimmed}\""
                    ),
                });
            }
            return;
        }
        layers.insert(num, trimmed);
    };

    for cap in bold_re().captures_iter(&data) {
        add_match(&cap[1], &cap[2], &mut layers, &mut findings);
    }
    for cap in head_re().captures_iter(&data) {
        add_match(&cap[1], &cap[2], &mut layers, &mut findings);
    }

    Ok((Some(layers), findings))
}

/// Cross-checks `arch` and `readme` layer maps and emits
/// `KIND_CROSS_FILE_NUMBER_MISMATCH` or `KIND_CROSS_FILE_NAME_MISMATCH`
/// findings for any discrepancy.
fn compare_layer_maps(
    arch: &HashMap<i64, String>,
    readme: &HashMap<i64, String>,
    arch_path: &str,
    readme_path: &str,
) -> Vec<LayerCoherenceFinding> {
    let composite = format!("{arch_path}+{readme_path}");
    let mut seen: BTreeSet<i64> = BTreeSet::new();
    seen.extend(arch.keys().copied());
    seen.extend(readme.keys().copied());

    let mut findings = Vec::new();
    for n in seen {
        let arch_name = arch.get(&n);
        let readme_name = readme.get(&n);
        match (arch_name, readme_name) {
            (Some(name), None) => findings.push(LayerCoherenceFinding {
                file: composite.clone(),
                severity: "fail".to_string(),
                kind: KIND_CROSS_FILE_NUMBER_MISMATCH.to_string(),
                message: format!(
                    "Layer {n} (\"{name}\") is declared in {arch_path} but missing from {readme_path}"
                ),
            }),
            (None, Some(name)) => findings.push(LayerCoherenceFinding {
                file: composite.clone(),
                severity: "fail".to_string(),
                kind: KIND_CROSS_FILE_NUMBER_MISMATCH.to_string(),
                message: format!(
                    "Layer {n} (\"{name}\") is declared in {readme_path} but missing from {arch_path}"
                ),
            }),
            (Some(an), Some(rn)) if an != rn => findings.push(LayerCoherenceFinding {
                file: composite.clone(),
                severity: "fail".to_string(),
                kind: KIND_CROSS_FILE_NAME_MISMATCH.to_string(),
                message: format!(
                    "Layer {n} named \"{an}\" in {arch_path} but \"{rn}\" in {readme_path}"
                ),
            }),
            _ => {}
        }
    }
    findings
}

/// Emits `KIND_NUMBERING_GAP` findings for any integer in `[0, max]` that
/// appears in neither `arch` nor `readme`.
///
/// # Panics
///
/// Panics if `seen` is somehow non-empty but has no maximum element — guarded
/// by the preceding `is_empty()` check.
fn check_numbering_gap(
    arch: &HashMap<i64, String>,
    readme: &HashMap<i64, String>,
    arch_path: &str,
    readme_path: &str,
) -> Vec<LayerCoherenceFinding> {
    let composite = format!("{arch_path}+{readme_path}");
    let mut seen: BTreeSet<i64> = BTreeSet::new();
    seen.extend(arch.keys().copied());
    seen.extend(readme.keys().copied());
    if seen.is_empty() {
        return Vec::new();
    }
    let max = *seen
        .iter()
        .max()
        .expect("seen is non-empty — is_empty() check above guards this");
    let mut findings = Vec::new();
    for i in 0..=max {
        if !seen.contains(&i) {
            findings.push(LayerCoherenceFinding {
                file: composite.clone(),
                severity: "fail".to_string(),
                kind: KIND_NUMBERING_GAP.to_string(),
                message: format!(
                    "layer numbering is not contiguous: Layer {i} is missing between 0 and {max}"
                ),
            });
        }
    }
    findings
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::real::RealFs;
    use std::fs;
    use tempfile::TempDir;

    fn write_docs(tmp: &TempDir, arch: &str, readme: &str) {
        let arch_p = tmp.path().join(ARCH_PATH);
        let readme_p = tmp.path().join(README_PATH);
        fs::create_dir_all(arch_p.parent().unwrap()).unwrap();
        fs::write(&arch_p, arch).unwrap();
        fs::write(&readme_p, readme).unwrap();
    }

    #[test]
    fn passes_with_matching_docs() {
        let tmp = TempDir::new().unwrap();
        write_docs(
            &tmp,
            "## Layer 0: Vision (the why)\n## Layer 1: Principles (the values)\n",
            "**Layer 0: Vision**\n**Layer 1: Principles**\n",
        );
        let findings = audit_layer_coherence(&RealFs, tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn detects_intra_file_name_conflict() {
        let tmp = TempDir::new().unwrap();
        write_docs(
            &tmp,
            "## Layer 0: Vision (a)\n## Layer 0: Mission (b)\n",
            "**Layer 0: Vision**\n",
        );
        let findings = audit_layer_coherence(&RealFs, tmp.path()).unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.kind == KIND_INTRA_FILE_NAME_CONFLICT)
        );
    }

    #[test]
    fn detects_cross_file_number_mismatch() {
        let tmp = TempDir::new().unwrap();
        write_docs(
            &tmp,
            "## Layer 0: Vision (x)\n## Layer 1: Principles (y)\n",
            "**Layer 0: Vision**\n",
        );
        let findings = audit_layer_coherence(&RealFs, tmp.path()).unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.kind == KIND_CROSS_FILE_NUMBER_MISMATCH)
        );
    }

    #[test]
    fn detects_cross_file_name_mismatch() {
        let tmp = TempDir::new().unwrap();
        write_docs(&tmp, "## Layer 0: Vision (x)\n", "**Layer 0: Mission**\n");
        let findings = audit_layer_coherence(&RealFs, tmp.path()).unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.kind == KIND_CROSS_FILE_NAME_MISMATCH)
        );
    }

    #[test]
    fn detects_numbering_gap() {
        let tmp = TempDir::new().unwrap();
        write_docs(
            &tmp,
            "## Layer 0: Vision (x)\n## Layer 2: Conventions (y)\n",
            "**Layer 0: Vision**\n**Layer 2: Conventions**\n",
        );
        let findings = audit_layer_coherence(&RealFs, tmp.path()).unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.kind == KIND_NUMBERING_GAP && f.message.contains("Layer 1"))
        );
    }

    #[test]
    fn missing_doc_emits_finding() {
        let tmp = TempDir::new().unwrap();
        // Only write README, no arch doc.
        let readme_p = tmp.path().join(README_PATH);
        fs::create_dir_all(readme_p.parent().unwrap()).unwrap();
        fs::write(&readme_p, "**Layer 0: Vision**\n").unwrap();
        let findings = audit_layer_coherence(&RealFs, tmp.path()).unwrap();
        assert!(findings.iter().any(|f| f.kind == KIND_MISSING_DOC));
    }

    #[test]
    fn sorts_findings_by_file_then_kind() {
        let tmp = TempDir::new().unwrap();
        write_docs(
            &tmp,
            "## Layer 0: Vision (x)\n## Layer 2: Conventions (y)\n",
            "**Layer 0: Vision**\n**Layer 1: Principles**\n",
        );
        let findings = audit_layer_coherence(&RealFs, tmp.path()).unwrap();
        // Findings should be in stable sort order.
        for window in findings.windows(2) {
            let (a, b) = (&window[0], &window[1]);
            assert!(a.file < b.file || (a.file == b.file && a.kind <= b.kind));
        }
    }
}
