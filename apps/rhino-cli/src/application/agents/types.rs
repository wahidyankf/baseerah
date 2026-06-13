//! Shared agent types ported from `apps/rhino-cli/internal/agents/types.go`.
//
// Sub-set covering validate-claude path: ClaudeAgentFull, ClaudeSkill,
// ValidationCheck, ValidationResult, ValidateClaudeOptions, plus the
// allow-list maps. Sync/converter-only types remain in Go-source-of-truth
// until the sync command is ported.

use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;
use std::time::Duration;

use regex::Regex;

/// Full Claude Code agent definition parsed from a `.claude/agents/*.md` file.
#[derive(Debug, Clone, Default)]
pub struct ClaudeAgentFull {
    /// The `name` frontmatter field — must match the filename stem.
    pub name: String,
    /// The `description` frontmatter field.
    pub description: String,
    /// Tool names from the `tools` frontmatter field.
    pub tools: Vec<String>,
    /// Model alias or full ID from the `model` frontmatter field.
    pub model: String,
    /// Color token from the `color` frontmatter field.
    pub color: String,
    /// Skill names from the `skills` frontmatter sequence.
    pub skills: Vec<String>,
}

/// Minimal Claude Code skill definition parsed from a `SKILL.md` file.
#[derive(Debug, Clone, Default)]
pub struct ClaudeSkill {
    /// The `name` frontmatter field — must match the directory name.
    pub name: String,
    /// The `description` frontmatter field.
    pub description: String,
}

/// One validation check result with status, expected/actual pair, and message.
#[derive(Debug, Clone)]
pub struct ValidationCheck {
    /// Human-readable check identifier (e.g. `"Agent: foo.md - Valid Tools"`).
    pub name: String,
    /// Result: `"passed"`, `"warning"`, or `"failed"`.
    pub status: String,
    /// What was expected (empty when not applicable).
    pub expected: String,
    /// What was observed (empty when not applicable).
    pub actual: String,
    /// Human-readable explanation of the failure or outcome.
    pub message: String,
}

impl ValidationCheck {
    /// Create a `passed` check with the given name and message.
    pub fn passed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: "passed".to_string(),
            expected: String::new(),
            actual: String::new(),
            message: message.into(),
        }
    }
    /// Create a `warning` check with name, expected, actual, and message.
    pub fn warning(
        name: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            status: "warning".to_string(),
            expected: expected.into(),
            actual: actual.into(),
            message: message.into(),
        }
    }
    /// Create a `failed` check with name, expected, actual, and message.
    pub fn failed(
        name: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            status: "failed".to_string(),
            expected: expected.into(),
            actual: actual.into(),
            message: message.into(),
        }
    }
    /// Create a `failed` check with only name and message (no expected/actual).
    pub fn failed_msg(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: "failed".to_string(),
            expected: String::new(),
            actual: String::new(),
            message: message.into(),
        }
    }
}

/// Aggregated result of running a set of validation checks.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// Total number of checks tallied.
    pub total_checks: usize,
    /// Number of checks with status `"passed"`.
    pub passed_checks: usize,
    /// Number of checks with status `"warning"`.
    pub warning_checks: usize,
    /// Number of checks with status `"failed"`.
    pub failed_checks: usize,
    /// Ordered list of individual check results.
    pub checks: Vec<ValidationCheck>,
    /// Wall time for the full validation run.
    pub duration: Duration,
}

impl ValidationResult {
    /// Append `check` and increment the appropriate status counter.
    pub fn tally(&mut self, check: ValidationCheck) {
        match check.status.as_str() {
            "passed" => self.passed_checks += 1,
            "warning" => self.warning_checks += 1,
            _ => self.failed_checks += 1,
        }
        self.total_checks += 1;
        self.checks.push(check);
    }
}

/// Options controlling which parts of the Claude binding to validate.
#[derive(Debug, Clone, Default)]
pub struct ValidateClaudeOptions {
    /// Absolute path to the repository root.
    pub repo_root: std::path::PathBuf,
    /// When true, only agent checks are run (skill checks are still used for skill-exist resolution).
    pub agents_only: bool,
    /// When true, only skill checks are run; agent checks are skipped.
    pub skills_only: bool,
}

/// Return the allow-list of known Claude Code tool names.
pub fn valid_tools() -> &'static HashMap<&'static str, bool> {
    static M: OnceLock<HashMap<&'static str, bool>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        for t in [
            "Read",
            "Write",
            "Edit",
            "Glob",
            "Grep",
            "Bash",
            "BashOutput",
            "KillShell",
            "NotebookEdit",
            "TodoWrite",
            "WebFetch",
            "WebSearch",
            "Agent",
            "Task",
            "SlashCommand",
            "ExitPlanMode",
            "EnterPlanMode",
            "ListMcpResourcesTool",
            "ReadMcpResourceTool",
            "AskUserQuestion",
        ] {
            m.insert(t, true);
        }
        m
    })
}

/// Return the allow-list of accepted Claude model alias strings (empty string means default).
pub fn valid_model_alias() -> &'static HashMap<&'static str, bool> {
    static M: OnceLock<HashMap<&'static str, bool>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        for k in ["", "sonnet", "opus", "haiku", "inherit"] {
            m.insert(k, true);
        }
        m
    })
}

/// Return compiled regex matching full Claude model IDs (e.g. `claude-sonnet-4-6`).
///
/// # Panics
///
/// Panics if the hardcoded regex pattern is invalid, which cannot happen.
pub fn valid_model_id_pattern() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^claude-[a-z0-9.-]+$").expect("valid hardcoded regex"))
}

/// Return compiled regex matching agent tool entries in call form (`ToolName(...)`).
///
/// # Panics
///
/// Panics if the hardcoded regex pattern is invalid, which cannot happen.
pub fn agent_tool_pattern() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\(.*\)$").expect("valid hardcoded regex"))
}

/// Return the allow-list of accepted `color` values for agent definitions.
pub fn valid_colors() -> &'static HashMap<&'static str, bool> {
    static M: OnceLock<HashMap<&'static str, bool>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        for c in [
            "red", "blue", "green", "yellow", "purple", "orange", "pink", "cyan",
        ] {
            m.insert(c, true);
        }
        m
    })
}

/// Return compiled regex that valid skill directory names must match.
///
/// # Panics
///
/// Panics if the hardcoded regex pattern is invalid, which cannot happen.
pub fn valid_skill_name_pattern() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[a-z0-9-]{1,64}$").expect("valid hardcoded regex"))
}

/// Return the ordered list of required frontmatter fields for agent definitions.
pub fn required_fields() -> &'static [&'static str] {
    &["name", "description"]
}

/// Return the allow-list of known frontmatter field names for Claude Code agents.
pub fn valid_claude_agent_fields() -> &'static HashMap<&'static str, bool> {
    static M: OnceLock<HashMap<&'static str, bool>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        for f in [
            "name",
            "description",
            "tools",
            "disallowedTools",
            "model",
            "permissionMode",
            "maxTurns",
            "skills",
            "mcpServers",
            "hooks",
            "memory",
            "background",
            "effort",
            "isolation",
            "color",
            "initialPrompt",
        ] {
            m.insert(f, true);
        }
        m
    })
}

/// Return the allow-list of known frontmatter field names for Claude Code skills.
pub fn valid_claude_skill_fields() -> &'static HashMap<&'static str, bool> {
    static M: OnceLock<HashMap<&'static str, bool>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        for f in [
            "name",
            "description",
            "license",
            "compatibility",
            "metadata",
            "when_to_use",
            "argument-hint",
            "arguments",
            "disable-model-invocation",
            "user-invocable",
            "allowed-tools",
            "model",
            "effort",
            "context",
            "agent",
            "hooks",
            "paths",
            "shell",
        ] {
            m.insert(f, true);
        }
        m
    })
}

/// Sorted iteration of the tool allow-list — matches Go's `for t := range
/// ValidTools` iteration order being non-deterministic; we emit sorted output
/// for the validateTools "Expected" string. Go's map iteration is random — to
/// match byte-for-byte we cannot, so reporter avoids embedding the expected
/// list for tool failures by running shadow-diff on success-paths only.
pub fn valid_tools_sorted() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = valid_tools().keys().copied().collect();
    v.sort_unstable();
    v
}

/// Build a `BTreeMap` view of the agent field allow-list for deterministic
/// iteration (used in tests).
pub fn valid_claude_agent_fields_sorted() -> BTreeMap<&'static str, bool> {
    valid_claude_agent_fields()
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn valid_tools_contains_core_tools() {
        let m = valid_tools();
        assert!(m.contains_key("Read"));
        assert!(m.contains_key("Write"));
        assert!(m.contains_key("Bash"));
        assert!(m.contains_key("Agent"));
        assert!(m.contains_key("Task"));
    }

    #[test]
    fn valid_model_alias_empty_string_is_valid() {
        assert!(valid_model_alias().contains_key(""));
    }

    #[test]
    fn valid_model_id_pattern_matches_known_ids() {
        let r = valid_model_id_pattern();
        assert!(r.is_match("claude-opus-4-7"));
        assert!(r.is_match("claude-sonnet-4-6"));
        assert!(!r.is_match("opus"));
    }

    #[test]
    fn agent_tool_pattern_captures_base() {
        let r = agent_tool_pattern();
        let m = r.captures("Agent(swe-typescript-dev)").unwrap();
        assert_eq!(&m[1], "Agent");
    }

    #[test]
    fn valid_skill_name_pattern_rejects_uppercase() {
        let r = valid_skill_name_pattern();
        assert!(r.is_match("valid-name"));
        assert!(!r.is_match("Invalid"));
    }

    #[test]
    fn validation_result_tally_buckets() {
        let mut r = ValidationResult::default();
        r.tally(ValidationCheck::passed("n1", "ok"));
        r.tally(ValidationCheck::warning("n2", "e", "a", "m"));
        r.tally(ValidationCheck::failed("n3", "e", "a", "m"));
        assert_eq!(r.total_checks, 3);
        assert_eq!(r.passed_checks, 1);
        assert_eq!(r.warning_checks, 1);
        assert_eq!(r.failed_checks, 1);
    }
}
