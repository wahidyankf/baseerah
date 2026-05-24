// Port of `apps/rhino-cli/internal/testcoverage/merge.go`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error};

use super::cobertura::{parse_branch_coverage, parse_cobertura};
use super::detect::detect_format;
use super::go_coverage::parse_cover_out;
use super::jacoco::parse_jacoco;
use super::lcov::parse_lcov;
use super::types::{FileResult, Format, Result as CoverageResult};

/// Per-line coverage data.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LineCoverage {
    pub hit_count: i64,
    pub branches: Vec<BranchCoverage>,
}

/// Per-branch coverage data.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BranchCoverage {
    pub block_id: i64,
    pub branch_id: i64,
    pub hit_count: i64,
}

/// filepath → line_number → LineCoverage.
/// BTreeMap to mirror Go's `sort.Strings(files)` + `sort.Ints(lineNos)` deterministic output.
pub type CoverageMap = BTreeMap<String, BTreeMap<i64, LineCoverage>>;

/// Union multiple CoverageMaps. Max hit count per line; branches unioned by (block, branch).
pub fn merge_coverage_maps(maps: &[CoverageMap]) -> CoverageMap {
    let mut result: CoverageMap = BTreeMap::new();
    for m in maps {
        for (file_path, lines) in m {
            let entry = result.entry(file_path.clone()).or_default();
            for (line_no, lc) in lines {
                match entry.get(line_no).cloned() {
                    None => {
                        entry.insert(*line_no, lc.clone());
                    }
                    Some(mut existing) => {
                        if lc.hit_count > existing.hit_count {
                            existing.hit_count = lc.hit_count;
                        }
                        existing.branches = merge_branches(&existing.branches, &lc.branches);
                        entry.insert(*line_no, existing);
                    }
                }
            }
        }
    }
    result
}

fn merge_branches(a: &[BranchCoverage], b: &[BranchCoverage]) -> Vec<BranchCoverage> {
    let mut m: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    for br in a {
        m.insert((br.block_id, br.branch_id), br.hit_count);
    }
    for br in b {
        let key = (br.block_id, br.branch_id);
        let current = m.get(&key).copied().unwrap_or(0);
        if br.hit_count > current {
            m.insert(key, br.hit_count);
        }
    }
    m.into_iter()
        .map(|((block_id, branch_id), hit_count)| BranchCoverage {
            block_id,
            branch_id,
            hit_count,
        })
        .collect()
}

/// Format the CoverageMap as LCOV text. Deterministic order via BTreeMap.
pub fn format_lcov_string(cm: &CoverageMap) -> String {
    let mut sb = String::new();
    for (file_path, lines) in cm {
        sb.push_str("TN:\n");
        let _ = writeln!(sb, "SF:{file_path}");
        // BRDA records first.
        for (ln, lc) in lines {
            for br in &lc.branches {
                let _ = writeln!(
                    sb,
                    "BRDA:{ln},{block},{branch},{hits}",
                    block = br.block_id,
                    branch = br.branch_id,
                    hits = br.hit_count
                );
            }
        }
        // Then DA records.
        for (ln, lc) in lines {
            let _ = writeln!(sb, "DA:{ln},{}", lc.hit_count);
        }
        sb.push_str("end_of_record\n");
    }
    sb
}

/// Write the CoverageMap as an LCOV file.
pub fn write_lcov(cm: &CoverageMap, out_path: &Path) -> Result<(), Error> {
    let content = format_lcov_string(cm);
    std::fs::write(out_path, content).with_context(|| format!("write {}", out_path.display()))?;
    Ok(())
}

/// Whether any branch in the list has hit_count <= 0.
pub fn has_missed_branch(branches: &[BranchCoverage]) -> bool {
    branches.iter().any(|b| b.hit_count <= 0)
}

/// Compute a Result from a CoverageMap using the standard line-based algorithm.
pub fn result_from_coverage_map(cm: &CoverageMap, threshold: f64) -> CoverageResult {
    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    let mut per_file: Vec<FileResult> = Vec::new();

    for (file_path, lines) in cm {
        let mut fc = 0usize;
        let mut fp = 0usize;
        let mut fm = 0usize;
        for lc in lines.values() {
            if lc.hit_count > 0 {
                if has_missed_branch(&lc.branches) {
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
            path: file_path.clone(),
            covered: fc,
            partial: fp,
            missed: fm,
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

    CoverageResult {
        file: String::new(),
        format: Format::Lcov,
        covered,
        partial,
        missed,
        total,
        pct,
        threshold,
        passed: pct >= threshold,
        files: per_file,
    }
}

// --- Format-specific CoverageMap converters ---

pub fn to_coverage_map_lcov(filename: &str) -> Result<CoverageMap, Error> {
    let files = parse_lcov(filename)?;
    let mut cm: CoverageMap = BTreeMap::new();
    for f in files {
        let entry = cm.entry(f.path.clone()).or_default();
        for (line_no, count) in &f.da_lines {
            let mut lc = LineCoverage {
                hit_count: *count,
                branches: Vec::new(),
            };
            if let Some(branches) = f.brda_data.get(line_no) {
                for (i, hits) in branches.iter().enumerate() {
                    lc.branches.push(BranchCoverage {
                        block_id: 0,
                        branch_id: i as i64,
                        hit_count: *hits,
                    });
                }
            }
            entry.insert(*line_no, lc);
        }
        // BRDA-only lines.
        for (line_no, branches) in &f.brda_data {
            if !f.da_lines.contains_key(line_no) {
                let mut lc = LineCoverage {
                    hit_count: 0,
                    branches: Vec::new(),
                };
                for (i, hits) in branches.iter().enumerate() {
                    lc.branches.push(BranchCoverage {
                        block_id: 0,
                        branch_id: i as i64,
                        hit_count: *hits,
                    });
                    if *hits > 0 && lc.hit_count == 0 {
                        lc.hit_count = *hits;
                    }
                }
                entry.insert(*line_no, lc);
            }
        }
    }
    Ok(cm)
}

pub fn to_coverage_map_go(filename: &str) -> Result<CoverageMap, Error> {
    let blocks = parse_cover_out(filename)?;
    let mut cm: CoverageMap = BTreeMap::new();
    for b in blocks {
        let entry = cm.entry(b.filepath.clone()).or_default();
        for line_no in b.start_line..=b.end_line {
            let key = line_no as i64;
            match entry.get(&key).cloned() {
                None => {
                    entry.insert(
                        key,
                        LineCoverage {
                            hit_count: b.count as i64,
                            branches: Vec::new(),
                        },
                    );
                }
                Some(mut existing) => {
                    if (b.count as i64) > existing.hit_count {
                        existing.hit_count = b.count as i64;
                    }
                    entry.insert(key, existing);
                }
            }
        }
    }
    Ok(cm)
}

pub fn to_coverage_map_jacoco(filename: &str) -> Result<CoverageMap, Error> {
    let report = parse_jacoco(filename)?;
    let mut cm: CoverageMap = BTreeMap::new();
    for pkg in &report.packages {
        for sf in &pkg.source_files {
            let file_path = format!("{}/{}", pkg.name, sf.name);
            let entry = cm.entry(file_path).or_default();
            for line in &sf.lines {
                let mut lc = LineCoverage {
                    hit_count: line.ci,
                    branches: Vec::new(),
                };
                if line.mb > 0 || line.cb > 0 {
                    for i in 0..line.cb {
                        lc.branches.push(BranchCoverage {
                            block_id: 0,
                            branch_id: i,
                            hit_count: 1,
                        });
                    }
                    for i in 0..line.mb {
                        lc.branches.push(BranchCoverage {
                            block_id: 0,
                            branch_id: line.cb + i,
                            hit_count: 0,
                        });
                    }
                }
                entry.insert(line.nr, lc);
            }
        }
    }
    Ok(cm)
}

pub fn to_coverage_map_cobertura(filename: &str) -> Result<CoverageMap, Error> {
    let report = parse_cobertura(filename)?;
    let mut cm: CoverageMap = BTreeMap::new();
    let Some(packages) = &report.packages else {
        return Ok(cm);
    };
    for pkg in &packages.package {
        let Some(classes) = &pkg.classes else {
            continue;
        };
        for cls in &classes.class {
            let entry = cm.entry(cls.filename.clone()).or_default();
            if let Some(lines) = &cls.lines {
                for line in &lines.line {
                    let mut lc = LineCoverage {
                        hit_count: line.hits,
                        branches: Vec::new(),
                    };
                    if line.branch {
                        let (br_cov, br_total) = parse_branch_coverage(&line.condition_coverage);
                        for i in 0..br_cov {
                            lc.branches.push(BranchCoverage {
                                block_id: 0,
                                branch_id: i,
                                hit_count: 1,
                            });
                        }
                        for i in br_cov..br_total {
                            lc.branches.push(BranchCoverage {
                                block_id: 0,
                                branch_id: i,
                                hit_count: 0,
                            });
                        }
                    }
                    entry.insert(line.number, lc);
                }
            }
        }
    }
    Ok(cm)
}

/// Dispatch by detected format.
pub fn to_coverage_map(filename: &str) -> Result<CoverageMap, Error> {
    match detect_format(filename) {
        Format::Lcov => to_coverage_map_lcov(filename),
        Format::Jacoco => to_coverage_map_jacoco(filename),
        Format::Cobertura => to_coverage_map_cobertura(filename),
        Format::Go | Format::Diff => to_coverage_map_go(filename),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn cm_single(path: &str, line: i64, hit: i64) -> CoverageMap {
        let mut cm: CoverageMap = BTreeMap::new();
        let mut lines = BTreeMap::new();
        lines.insert(
            line,
            LineCoverage {
                hit_count: hit,
                branches: Vec::new(),
            },
        );
        cm.insert(path.into(), lines);
        cm
    }

    #[test]
    fn merge_takes_max_hit_count() {
        let a = cm_single("a.go", 10, 1);
        let b = cm_single("a.go", 10, 5);
        let m = merge_coverage_maps(&[a, b]);
        assert_eq!(m["a.go"][&10].hit_count, 5);
    }

    #[test]
    fn merge_unions_files() {
        let a = cm_single("a.go", 1, 1);
        let b = cm_single("b.go", 2, 1);
        let m = merge_coverage_maps(&[a, b]);
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn merge_branches_takes_max_per_key() {
        let a = vec![BranchCoverage {
            block_id: 0,
            branch_id: 0,
            hit_count: 1,
        }];
        let b = vec![BranchCoverage {
            block_id: 0,
            branch_id: 0,
            hit_count: 3,
        }];
        let m = merge_branches(&a, &b);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].hit_count, 3);
    }

    #[test]
    fn format_lcov_string_includes_tn_sf() {
        let cm = cm_single("a.go", 10, 2);
        let s = format_lcov_string(&cm);
        assert!(s.contains("TN:\n"));
        assert!(s.contains("SF:a.go"));
        assert!(s.contains("DA:10,2"));
        assert!(s.contains("end_of_record"));
    }

    #[test]
    fn result_from_coverage_map_basic() {
        let mut cm: CoverageMap = BTreeMap::new();
        let mut lines = BTreeMap::new();
        lines.insert(
            1,
            LineCoverage {
                hit_count: 1,
                branches: Vec::new(),
            },
        );
        lines.insert(
            2,
            LineCoverage {
                hit_count: 0,
                branches: Vec::new(),
            },
        );
        cm.insert("a.go".into(), lines);
        let r = result_from_coverage_map(&cm, 50.0);
        assert_eq!(r.covered, 1);
        assert_eq!(r.missed, 1);
        assert!((r.pct - 50.0).abs() < 1e-9);
        assert!(r.passed);
    }

    #[test]
    fn has_missed_branch_detects_zero() {
        assert!(has_missed_branch(&[BranchCoverage {
            block_id: 0,
            branch_id: 0,
            hit_count: 0
        }]));
        assert!(!has_missed_branch(&[BranchCoverage {
            block_id: 0,
            branch_id: 0,
            hit_count: 1
        }]));
    }

    #[test]
    fn to_coverage_map_go_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("cover.out");
        std::fs::write(&p, "mode: set\napp/foo.go:1.2,3.4 1 1\n").unwrap();
        let cm = to_coverage_map_go(p.to_str().unwrap()).unwrap();
        assert_eq!(cm["app/foo.go"][&1].hit_count, 1);
    }

    #[test]
    fn write_lcov_writes_file() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("out.info");
        let cm = cm_single("a.go", 1, 3);
        write_lcov(&cm, &p).unwrap();
        let s = std::fs::read_to_string(&p).unwrap();
        assert!(s.contains("DA:1,3"));
    }
}
