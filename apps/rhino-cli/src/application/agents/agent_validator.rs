//! Agent validator ported from
//! `apps/rhino-cli/internal/agents/agent_validator.go`.

use std::collections::HashSet;
use std::fs;
use std::hash::BuildHasher;
use std::path::Path;

use serde_norway::Value;

use super::frontmatter::{extract_frontmatter, parse_claude_tools};
use super::types::{
    ClaudeAgentFull, ValidationCheck, agent_tool_pattern, required_fields,
    valid_claude_agent_fields, valid_colors, valid_model_alias, valid_model_id_pattern,
    valid_tools, valid_tools_sorted,
};
use super::yaml_formatting::validate_yaml_formatting_raw;

/// Validate a single agent file at `agent_path` and return a list of check results.
/// Inserts the agent name into `agent_names` on success for uniqueness tracking.
pub fn validate_agent<S1: BuildHasher, S2: BuildHasher>(
    agent_path: &Path,
    filename: &str,
    agent_names: &mut HashSet<String, S1>,
    skill_names: &HashSet<String, S2>,
) -> Vec<ValidationCheck> {
    let mut checks: Vec<ValidationCheck> = Vec::new();

    let content = match fs::read(agent_path) {
        Ok(c) => c,
        Err(e) => {
            checks.push(ValidationCheck::failed_msg(
                format!("Agent: {filename} - Read File"),
                format!("Failed to read file: {e}"),
            ));
            return checks;
        }
    };

    let formatting_check =
        validate_yaml_formatting_raw(&format!("Agent: {filename} - YAML Formatting"), &content);
    let formatting_failed = formatting_check.status == "failed";
    checks.push(formatting_check);
    if formatting_failed {
        return checks;
    }

    let (frontmatter, _) = match extract_frontmatter(&content) {
        Ok(v) => v,
        Err(e) => {
            checks.push(ValidationCheck::failed_msg(
                format!("Agent: {filename} - YAML Syntax"),
                format!("Invalid frontmatter: {e}"),
            ));
            return checks;
        }
    };
    checks.push(ValidationCheck::passed(
        format!("Agent: {filename} - YAML Syntax"),
        "Valid YAML frontmatter",
    ));

    let frontmatter_str = String::from_utf8_lossy(&frontmatter).into_owned();
    let agent = match parse_agent_yaml(&frontmatter_str) {
        Ok(a) => a,
        Err(e) => {
            checks.push(ValidationCheck::failed_msg(
                format!("Agent: {filename} - YAML Parse"),
                format!("Failed to parse YAML: {e}"),
            ));
            return checks;
        }
    };

    let required_check = validate_required_fields(filename, &agent);
    let required_failed = required_check.status == "failed";
    checks.push(required_check);
    if required_failed {
        return checks;
    }

    checks.extend(validate_field_order(filename, &frontmatter_str));
    checks.push(validate_tools_check(filename, &agent.tools));
    checks.push(validate_model_check(filename, &agent.model));
    if !agent.color.is_empty() {
        checks.push(validate_color_check(filename, &agent.color));
    }
    checks.push(validate_filename_check(filename, &agent.name));
    let unique_check = validate_uniqueness(filename, &agent.name, agent_names);
    let unique_passed = unique_check.status == "passed";
    checks.push(unique_check);
    if unique_passed {
        agent_names.insert(agent.name.clone());
    }
    checks.push(validate_skills_exist(filename, &agent.skills, skill_names));
    checks.push(validate_no_comments(filename, &frontmatter));

    if agent_path.to_string_lossy().contains("generated-reports") {
        checks.push(validate_generated_reports_tools(filename, &agent.tools));
    }

    checks
}

/// Parse a YAML frontmatter string into a `ClaudeAgentFull`.
fn parse_agent_yaml(frontmatter: &str) -> Result<ClaudeAgentFull, String> {
    let v: Value =
        serde_norway::from_str(frontmatter).map_err(|e| format!("yaml parse error: {e}"))?;
    let mut agent = ClaudeAgentFull::default();
    if let Value::Mapping(m) = v {
        for (k, val) in m {
            let key = k.as_str().unwrap_or("").to_string();
            match key.as_str() {
                "name" => agent.name = val.as_str().unwrap_or("").to_string(),
                "description" => agent.description = val.as_str().unwrap_or("").to_string(),
                "model" => agent.model = val.as_str().unwrap_or("").to_string(),
                "color" => agent.color = val.as_str().unwrap_or("").to_string(),
                "tools" => agent.tools = parse_claude_tools(&val),
                "skills" => {
                    if let Value::Sequence(seq) = val {
                        agent.skills = seq
                            .into_iter()
                            .filter_map(|x| x.as_str().map(std::string::ToString::to_string))
                            .collect();
                    }
                }
                _ => {}
            }
        }
    }
    Ok(agent)
}

/// Check that `name`, `description`, and `tools` are all non-empty.
fn validate_required_fields(filename: &str, agent: &ClaudeAgentFull) -> ValidationCheck {
    let mut missing: Vec<&str> = Vec::new();
    if agent.name.is_empty() {
        missing.push("name");
    }
    if agent.description.is_empty() {
        missing.push("description");
    }
    if agent.tools.is_empty() {
        missing.push("tools");
    }
    if !missing.is_empty() {
        return ValidationCheck::failed(
            format!("Agent: {filename} - Required Fields"),
            "All required fields present",
            format!("Missing: {}", format_slice(&missing)),
            "Required fields missing",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Required Fields"),
        "All required fields present",
    )
}

/// Check that required fields appear before optional fields, and warn on unknown fields.
fn validate_field_order(filename: &str, frontmatter: &str) -> Vec<ValidationCheck> {
    let v: Result<Value, _> = serde_norway::from_str(frontmatter);
    let v = match v {
        Ok(v) => v,
        Err(e) => {
            return vec![ValidationCheck::failed_msg(
                format!("Agent: {filename} - Field Order"),
                format!("Failed to parse YAML for order check: {e}"),
            )];
        }
    };

    let mut field_names: Vec<String> = Vec::new();
    if let Value::Mapping(m) = &v {
        for (k, _) in m {
            if let Some(s) = k.as_str() {
                field_names.push(s.to_string());
            }
        }
    }

    let required: HashSet<&'static str> = required_fields().iter().copied().collect();

    let mut saw_optional = false;
    let mut out_of_order: Vec<String> = Vec::new();
    for f in &field_names {
        if required.contains(f.as_str()) {
            if saw_optional {
                out_of_order.push(f.clone());
            }
        } else {
            saw_optional = true;
        }
    }

    let mut checks: Vec<ValidationCheck> = Vec::new();
    if out_of_order.is_empty() {
        checks.push(ValidationCheck::passed(
            format!("Agent: {filename} - Field Order"),
            "Required fields appear before optional fields",
        ));
    } else {
        checks.push(ValidationCheck::failed(
            format!("Agent: {filename} - Field Order"),
            format!(
                "Required fields {} appear before any optional field",
                format_string_slice(required_fields())
            ),
            format!(
                "Required field(s) appear after optional field: {}",
                format_string_slice_owned(&out_of_order)
            ),
            "Required fields must appear before optional fields",
        ));
    }

    let allow = valid_claude_agent_fields();
    for f in &field_names {
        if !allow.contains_key(f.as_str()) {
            checks.push(ValidationCheck::warning(
                format!("Agent: {filename} - Unknown Field: {f}"),
                "Field listed in ValidClaudeAgentFields",
                format!("Unknown field: {f}"),
                format!(
                    "Field \"{f}\" is not in the documented Claude Code agent field set; verify it is intentional"
                ),
            ));
        }
    }
    checks
}

/// Check that every tool name (or base name of call-form entries) is in the allow-list.
#[allow(clippy::collapsible_match, clippy::collapsible_if)]
fn validate_tools_check(filename: &str, tools: &[String]) -> ValidationCheck {
    let mut invalid: Vec<String> = Vec::new();
    let allow = valid_tools();
    let pat = agent_tool_pattern();
    for tool in tools {
        let tool = tool.trim();
        if tool.is_empty() {
            continue;
        }
        let mut base = tool.to_string();
        if let Some(m) = pat.captures(tool) {
            if let Some(b) = m.get(1) {
                base = b.as_str().to_string();
            }
        }
        if !allow.contains_key(base.as_str()) {
            invalid.push(tool.to_string());
        }
    }
    if !invalid.is_empty() {
        let valid = valid_tools_sorted();
        return ValidationCheck::failed(
            format!("Agent: {filename} - Valid Tools"),
            format!("Valid tools: {}", format_string_slice(&valid)),
            format!("Invalid tools: {}", format_string_slice_owned(&invalid)),
            "Invalid tool names",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Valid Tools"),
        "All tools valid",
    )
}

/// Check that `model` is a valid alias or a full `claude-*` model ID.
fn validate_model_check(filename: &str, model: &str) -> ValidationCheck {
    let aliases = valid_model_alias();
    let pat = valid_model_id_pattern();
    if aliases.contains_key(model) || pat.is_match(model) {
        return ValidationCheck::passed(format!("Agent: {filename} - Valid Model"), "Model valid");
    }
    ValidationCheck::failed(
        format!("Agent: {filename} - Valid Model"),
        "<empty>|sonnet|opus|haiku|inherit|claude-*",
        format!("Model: {model}"),
        "Invalid model",
    )
}

/// Check that `color` is in the allow-list of named color tokens.
fn validate_color_check(filename: &str, color: &str) -> ValidationCheck {
    if !valid_colors().contains_key(color) {
        let valid_colors_list = [
            "red", "blue", "green", "yellow", "purple", "orange", "pink", "cyan",
        ];
        return ValidationCheck::failed(
            format!("Agent: {filename} - Valid Color"),
            format!("Valid colors: {}", format_string_slice(&valid_colors_list)),
            format!("Color: {color}"),
            "Invalid color",
        );
    }
    ValidationCheck::passed(format!("Agent: {filename} - Valid Color"), "Color valid")
}

/// Check that the filename equals `<name>.md`.
fn validate_filename_check(filename: &str, name: &str) -> ValidationCheck {
    let expected = format!("{name}.md");
    if filename != expected {
        return ValidationCheck::failed(
            format!("Agent: {filename} - Filename Match"),
            format!("Filename: {expected}"),
            format!("Filename: {filename}"),
            "Filename does not match name field",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Filename Match"),
        "Filename matches name",
    )
}

/// Check that `name` has not already been registered in `agent_names`.
fn validate_uniqueness<S: BuildHasher>(
    filename: &str,
    name: &str,
    agent_names: &HashSet<String, S>,
) -> ValidationCheck {
    if agent_names.contains(name) {
        return ValidationCheck::failed(
            format!("Agent: {filename} - Name Uniqueness"),
            "Unique agent name",
            format!("Duplicate name: {name}"),
            "Agent name already used",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Name Uniqueness"),
        "Agent name unique",
    )
}

/// Check that every skill listed in `skills` exists in `skill_names`.
fn validate_skills_exist<S: BuildHasher>(
    filename: &str,
    skills: &[String],
    skill_names: &HashSet<String, S>,
) -> ValidationCheck {
    let mut missing: Vec<String> = Vec::new();
    for s in skills {
        if !skill_names.contains(s) {
            missing.push(s.clone());
        }
    }
    if !missing.is_empty() {
        return ValidationCheck::failed(
            format!("Agent: {filename} - Skills Exist"),
            "All skills exist",
            format!("Missing skills: {}", format_string_slice_owned(&missing)),
            "Referenced skills not found",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Skills Exist"),
        "All skills exist",
    )
}

/// Check that frontmatter contains no YAML comment lines (`#`).
fn validate_no_comments(filename: &str, frontmatter: &[u8]) -> ValidationCheck {
    let s = String::from_utf8_lossy(frontmatter);
    for line in s.split('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            return ValidationCheck::failed(
                format!("Agent: {filename} - No Comments"),
                "No YAML comments",
                "Comments found",
                "YAML comments not allowed in frontmatter",
            );
        }
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - No Comments"),
        "No YAML comments",
    )
}

/// Check that agents in `generated-reports/` declare both `Write` and `Bash` tools.
#[allow(clippy::collapsible_match, clippy::collapsible_if)]
fn validate_generated_reports_tools(filename: &str, tools: &[String]) -> ValidationCheck {
    let mut has_write = false;
    let mut has_bash = false;
    let pat = agent_tool_pattern();
    for tool in tools {
        let tool = tool.trim();
        let mut base = tool.to_string();
        if let Some(m) = pat.captures(tool) {
            if let Some(b) = m.get(1) {
                base = b.as_str().to_string();
            }
        }
        if base == "Write" {
            has_write = true;
        }
        if base == "Bash" {
            has_bash = true;
        }
    }
    if !has_write || !has_bash {
        return ValidationCheck::failed(
            format!("Agent: {filename} - Generated Reports Tools"),
            "Tools must include: Write, Bash",
            format!("Has Write: {has_write}, Has Bash: {has_bash}"),
            "generated-reports/ agents must have Write AND Bash tools",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Generated Reports Tools"),
        "Has required Write and Bash tools",
    )
}

/// Validate all `.md` agent files in `.claude/agents/` and return every check result.
pub fn validate_all_agents<S: BuildHasher>(
    repo_root: &Path,
    skill_names: &HashSet<String, S>,
) -> Vec<ValidationCheck> {
    let agents_dir = repo_root.join(".claude").join("agents");
    let entries = match fs::read_dir(&agents_dir) {
        Ok(e) => e,
        Err(e) => {
            return vec![ValidationCheck::failed_msg(
                "Read Agents Directory",
                format!("Failed to read agents directory: {e}"),
            )];
        }
    };

    let mut agent_names: HashSet<String> = HashSet::new();
    let mut all: Vec<ValidationCheck> = Vec::new();

    let mut paths: Vec<(std::path::PathBuf, String)> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        if !name.ends_with(".md") || name == "README.md" {
            continue;
        }
        paths.push((entry.path(), name));
    }
    paths.sort_by(|a, b| a.1.cmp(&b.1));

    for (path, name) in paths {
        let checks = validate_agent(&path, &name, &mut agent_names, skill_names);
        all.extend(checks);
    }

    all
}

/// Formats a slice of &str into Go's `%v` for a string slice: `[a b c]`.
fn format_string_slice(s: &[&str]) -> String {
    let inner: Vec<String> = s.iter().map(std::string::ToString::to_string).collect();
    format!("[{}]", inner.join(" "))
}

/// Format an owned string slice as `[a b c]` — Go `%v` style.
fn format_string_slice_owned(s: &[String]) -> String {
    format!("[{}]", s.join(" "))
}

/// Formats &[&str] -> "[a b c]" — same shape as Go's `%v`.
fn format_slice(s: &[&str]) -> String {
    format_string_slice(s)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use tempfile::tempdir;

    fn write(path: &Path, content: &str) {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    fn empty_skills() -> HashSet<String> {
        HashSet::new()
    }

    #[test]
    fn validate_required_fields_missing_name() {
        let agent = ClaudeAgentFull {
            description: "desc".to_string(),
            ..Default::default()
        };
        let c = validate_required_fields("x.md", &agent);
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_required_fields_all_present() {
        let agent = ClaudeAgentFull {
            name: "n".to_string(),
            description: "d".to_string(),
            tools: vec!["Read".to_string()],
            ..Default::default()
        };
        let c = validate_required_fields("x.md", &agent);
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_required_fields_missing_tools() {
        let agent = ClaudeAgentFull {
            name: "n".to_string(),
            description: "d".to_string(),
            ..Default::default()
        };
        let c = validate_required_fields("x.md", &agent);
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_tools_passes_known() {
        let c = validate_tools_check("x.md", &["Read".into(), "Write".into()]);
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_tools_passes_agent_form() {
        let c = validate_tools_check("x.md", &["Agent(swe-typescript-dev)".into()]);
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_tools_fails_unknown() {
        let c = validate_tools_check("x.md", &["Nonsense".into()]);
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_model_accepts_empty() {
        assert_eq!(validate_model_check("x.md", "").status, "passed");
    }

    #[test]
    fn validate_model_accepts_alias() {
        assert_eq!(validate_model_check("x.md", "sonnet").status, "passed");
    }

    #[test]
    fn validate_model_accepts_id() {
        assert_eq!(
            validate_model_check("x.md", "claude-opus-4-7").status,
            "passed"
        );
    }

    #[test]
    fn validate_model_rejects_other() {
        assert_eq!(validate_model_check("x.md", "gpt-4").status, "failed");
    }

    #[test]
    fn validate_color_rejects_unknown() {
        assert_eq!(validate_color_check("x.md", "magenta").status, "failed");
    }

    #[test]
    fn validate_color_accepts_known() {
        assert_eq!(validate_color_check("x.md", "blue").status, "passed");
    }

    #[test]
    fn validate_filename_match() {
        assert_eq!(validate_filename_check("foo.md", "foo").status, "passed");
        assert_eq!(validate_filename_check("foo.md", "bar").status, "failed");
    }

    #[test]
    fn validate_uniqueness_dup() {
        let mut set: HashSet<String> = HashSet::new();
        set.insert("foo".to_string());
        let c = validate_uniqueness("foo.md", "foo", &set);
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_skills_missing() {
        let mut set: HashSet<String> = HashSet::new();
        set.insert("a".to_string());
        let c = validate_skills_exist("x.md", &["a".into(), "b".into()], &set);
        assert_eq!(c.status, "failed");
        assert!(c.actual.contains('b'));
    }

    #[test]
    fn validate_no_comments_fails() {
        let c = validate_no_comments("x.md", b"name: x\n# comment\ndescription: y\n");
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_no_comments_passes() {
        let c = validate_no_comments("x.md", b"name: x\ndescription: y\n");
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_generated_reports_tools_passes() {
        let c = validate_generated_reports_tools(
            "x.md",
            &["Read".into(), "Write".into(), "Bash".into()],
        );
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_generated_reports_tools_fails_missing_write() {
        let c = validate_generated_reports_tools("x.md", &["Bash".into()]);
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_field_order_passes() {
        let f = "name: foo\ndescription: bar\ntools:\n  - Read\nmodel: sonnet\n";
        let checks = validate_field_order("x.md", f);
        assert!(
            checks
                .iter()
                .any(|c| c.name.ends_with("Field Order") && c.status == "passed")
        );
    }

    #[test]
    fn validate_field_order_fails_required_after_optional() {
        let f = "description: foo\ntools: Read\nname: x\n";
        let checks = validate_field_order("x.md", f);
        assert!(
            checks
                .iter()
                .any(|c| c.name.ends_with("Field Order") && c.status == "failed")
        );
    }

    #[test]
    fn validate_field_order_warns_unknown_field() {
        let f = "name: x\ndescription: y\nbogus: z\n";
        let checks = validate_field_order("x.md", f);
        assert!(
            checks
                .iter()
                .any(|c| c.status == "warning" && c.name.contains("Unknown Field: bogus"))
        );
    }

    #[test]
    fn validate_agent_end_to_end_pass() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("foo.md");
        write(
            &path,
            "---\nname: foo\ndescription: desc\ntools: Read, Write, Bash\nmodel: sonnet\ncolor: blue\n---\nBody\n",
        );
        let mut names = HashSet::new();
        let skills = empty_skills();
        let checks = validate_agent(&path, "foo.md", &mut names, &skills);
        let failed = checks.iter().filter(|c| c.status == "failed").count();
        assert_eq!(failed, 0, "checks: {checks:#?}");
    }

    #[test]
    fn validate_agent_yaml_formatting_failure_returns_early() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("foo.md");
        write(&path, "---\nname:foo\ndescription: bar\n---\nBody\n");
        let mut names = HashSet::new();
        let skills = empty_skills();
        let checks = validate_agent(&path, "foo.md", &mut names, &skills);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].status, "failed");
    }

    #[test]
    fn validate_agent_read_error() {
        let mut names = HashSet::new();
        let skills = empty_skills();
        let checks = validate_agent(
            Path::new("/nonexistent/path/missing.md"),
            "missing.md",
            &mut names,
            &skills,
        );
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].status, "failed");
        assert!(checks[0].message.contains("Failed to read"));
    }

    #[test]
    fn validate_agent_bad_yaml() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("foo.md");
        write(&path, "---\nname: [unbalanced\n---\nBody\n");
        let mut names = HashSet::new();
        let skills = empty_skills();
        let checks = validate_agent(&path, "foo.md", &mut names, &skills);
        assert!(checks.iter().any(|c| c.status == "failed"));
    }

    #[test]
    fn validate_all_agents_missing_dir() {
        let dir = tempdir().unwrap();
        let skills = empty_skills();
        let checks = validate_all_agents(dir.path(), &skills);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].status, "failed");
    }

    #[test]
    fn validate_all_agents_reads_md_files() {
        let dir = tempdir().unwrap();
        let agents = dir.path().join(".claude").join("agents");
        std::fs::create_dir_all(&agents).unwrap();
        write(
            &agents.join("foo.md"),
            "---\nname: foo\ndescription: ok\ntools: Read\nmodel: sonnet\ncolor: blue\n---\n",
        );
        write(&agents.join("README.md"), "skip me\n");
        let skills = empty_skills();
        let checks = validate_all_agents(dir.path(), &skills);
        assert!(checks.iter().any(|c| c.name.contains("foo.md")));
        assert!(checks.iter().all(|c| !c.name.contains("README.md")));
    }
}
