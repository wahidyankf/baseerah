// Sealed Format enum + Result types. Mirrors
// `apps/rhino-cli/internal/testcoverage/types.go`.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Go,
    Lcov,
    Jacoco,
    Cobertura,
    Diff,
}

impl Format {
    pub fn code(self) -> &'static str {
        match self {
            Format::Go => "go",
            Format::Lcov => "lcov",
            Format::Jacoco => "jacoco",
            Format::Cobertura => "cobertura",
            Format::Diff => "diff",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FileResult {
    pub path: String,
    pub covered: usize,
    pub partial: usize,
    pub missed: usize,
    pub total: usize,
    pub pct: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Result {
    pub file: String,
    pub format: Format,
    pub covered: usize,
    pub partial: usize,
    pub missed: usize,
    pub total: usize,
    pub pct: f64,
    pub threshold: f64,
    pub passed: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileResult>,
}
