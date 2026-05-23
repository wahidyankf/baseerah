// Shared agent types ported from `apps/rhino-cli/internal/agents/types.go`.
//
// Sub-set covering validate-claude path: ClaudeAgentFull, ClaudeSkill,
// ValidationCheck, ValidationResult, ValidateClaudeOptions, plus the
// allow-list maps. Sync/converter-only types remain in Go-source-of-truth
// until the sync command is ported.

use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;
use std::time::Duration;

use regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct ClaudeAgentFull {
    pub name: String,
    pub description: String,
    pub tools: Vec<String>,
    pub model: String,
    pub color: String,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClaudeSkill {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ValidationCheck {
    pub name: String,
    pub status: String, // "passed" | "warning" | "failed"
    pub expected: String,
    pub actual: String,
    pub message: String,
}

impl ValidationCheck {
    pub fn passed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: "passed".to_string(),
            expected: String::new(),
            actual: String::new(),
            message: message.into(),
        }
    }
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

#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    pub total_checks: usize,
    pub passed_checks: usize,
    pub warning_checks: usize,
    pub failed_checks: usize,
    pub checks: Vec<ValidationCheck>,
    pub duration: Duration,
}

impl ValidationResult {
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

#[derive(Debug, Clone, Default)]
pub struct ValidateClaudeOptions {
    pub repo_root: std::path::PathBuf,
    pub agents_only: bool,
    pub skills_only: bool,
}

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

pub fn valid_model_id_pattern() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^claude-[a-z0-9.-]+$").unwrap())
}

pub fn agent_tool_pattern() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\(.*\)$").unwrap())
}

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

pub fn valid_skill_name_pattern() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^[a-z0-9-]{1,64}$").unwrap())
}

pub fn required_fields() -> &'static [&'static str] {
    &["name", "description"]
}

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
    v.sort();
    v
}

/// Build a BTreeMap view of the agent field allow-list for deterministic
/// iteration (used in tests).
pub fn valid_claude_agent_fields_sorted() -> BTreeMap<&'static str, bool> {
    valid_claude_agent_fields()
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect()
}

#[cfg(test)]
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
