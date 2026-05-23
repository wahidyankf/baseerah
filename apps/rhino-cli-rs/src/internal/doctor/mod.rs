// Port of `apps/rhino-cli/internal/doctor/` package.
// Checks required tool versions are installed and correct.

mod checker;
mod fixer;
mod reporter;
mod tools;

use std::time::Duration;

pub use checker::{check_all, real_runner};
pub use fixer::{fix_all, format_fix_summary, FixOptions, FixResult};
pub use reporter::{format_json, format_markdown, format_text};

/// Health status of a tool check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolStatus {
    Ok,
    Warning,
    Missing,
}

impl ToolStatus {
    pub fn code(self) -> &'static str {
        match self {
            ToolStatus::Ok => "ok",
            ToolStatus::Warning => "warning",
            ToolStatus::Missing => "missing",
        }
    }
}

/// Controls which tools doctor checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Full,
    Minimal,
}

impl Scope {
    pub fn code(self) -> &'static str {
        match self {
            Scope::Full => "full",
            Scope::Minimal => "minimal",
        }
    }

    /// Empty or "full" → Some(Full). "minimal" → Some(Minimal). Unknown → None.
    pub fn parse(s: &str) -> Option<Scope> {
        match s {
            "" | "full" => Some(Scope::Full),
            "minimal" => Some(Scope::Minimal),
            _ => None,
        }
    }
}

/// Tool names in the minimal scope.
pub fn is_minimal_tool(name: &str) -> bool {
    matches!(
        name,
        "git" | "volta" | "node" | "npm" | "golang" | "docker" | "jq"
    )
}

/// Result of checking a single tool.
#[derive(Debug, Clone)]
pub struct ToolCheck {
    pub name: String,
    pub binary: String,
    pub status: ToolStatus,
    pub installed_version: String,
    pub required_version: String,
    pub source: String,
    pub note: String,
}

/// Aggregated check results.
#[derive(Debug, Clone)]
pub struct DoctorResult {
    pub checks: Vec<ToolCheck>,
    pub ok_count: usize,
    pub warn_count: usize,
    pub missing_count: usize,
    pub duration: Duration,
    pub scope: Scope,
}

/// (stdout, stderr, exit_code). `Err` = binary not found in PATH.
pub type CommandOutput = Result<(String, String, i32), String>;

/// Injectable command runner.
pub type CommandRunner<'a> = &'a dyn Fn(&str, &[&str]) -> CommandOutput;

/// Configuration for a check.
pub struct CheckOptions<'a> {
    pub repo_root: std::path::PathBuf,
    pub runner: Option<CommandRunner<'a>>,
    pub scope: Scope,
}
