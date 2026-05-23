// Sync validator ported from
// `apps/rhino-cli/internal/agents/sync_validator.go`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_norway::Value;

use super::converter::{convert_model, convert_tools, OPENCODE_AGENT_DIR};
use super::frontmatter::{extract_frontmatter, parse_claude_tools};
use super::types::{ValidationCheck, ValidationResult};

pub fn validate_sync(repo_root: &Path) -> ValidationResult {
    let start = Instant::now();
    let mut result = ValidationResult::default();

    result.tally(validate_no_stale_agent_dir(repo_root));
    result.tally(validate_agent_count(repo_root));

    let equivalence = validate_agent_equivalence(repo_root);
    for c in equivalence {
        result.tally(c);
    }

    result.tally(validate_no_synced_skills(repo_root));

    result.duration = start.elapsed();
    result
}

fn validate_no_stale_agent_dir(repo_root: &Path) -> ValidationCheck {
    let stale = repo_root.join(".opencode").join("agent");
    match fs::metadata(&stale) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => ValidationCheck::passed(
            "No Stale Agent Directory",
            "Legacy singular .opencode/agent/ does not exist",
        ),
        Err(e) => ValidationCheck::failed_msg(
            "No Stale Agent Directory",
            format!("Failed to stat .opencode/agent/: {e}"),
        ),
        Ok(info) if info.is_dir() => ValidationCheck::failed(
            "No Stale Agent Directory",
            ".opencode/agent/ does not exist",
            ".opencode/agent/ exists as a directory",
            "Stale singular .opencode/agent/ reappeared; canonical OpenCode path is .opencode/agents/ (plural). Remove the stale directory.",
        ),
        Ok(_) => ValidationCheck::failed(
            "No Stale Agent Directory",
            ".opencode/agent/ does not exist",
            ".opencode/agent/ exists",
            "Stale .opencode/agent/ entry reappeared; canonical OpenCode path is .opencode/agents/ (plural). Remove the stale entry.",
        ),
    }
}

fn validate_agent_count(repo_root: &Path) -> ValidationCheck {
    let claude_dir = repo_root.join(".claude").join("agents");
    let opencode_dir = repo_root.join(OPENCODE_AGENT_DIR);

    let claude_count = count_markdown_files(&claude_dir);
    let opencode_count = count_markdown_files(&opencode_dir);

    if opencode_count >= claude_count {
        ValidationCheck::passed(
            "Agent Count",
            "OpenCode agents directory contains every Claude agent",
        )
        .with_expected(format!(">= {claude_count} agents"))
        .with_actual(format!("{opencode_count} agents"))
    } else {
        ValidationCheck::failed(
            "Agent Count",
            format!(">= {claude_count} agents"),
            format!("{opencode_count} agents"),
            "OpenCode agents directory missing one or more Claude agents",
        )
    }
}

trait WithExpectedActual {
    fn with_expected(self, e: String) -> Self;
    fn with_actual(self, a: String) -> Self;
}

impl WithExpectedActual for ValidationCheck {
    fn with_expected(mut self, e: String) -> Self {
        self.expected = e;
        self
    }
    fn with_actual(mut self, a: String) -> Self {
        self.actual = a;
        self
    }
}

fn validate_agent_equivalence(repo_root: &Path) -> Vec<ValidationCheck> {
    let mut checks = Vec::new();
    let claude_dir = repo_root.join(".claude").join("agents");
    let opencode_dir = repo_root.join(OPENCODE_AGENT_DIR);

    let entries = match fs::read_dir(&claude_dir) {
        Ok(e) => e,
        Err(e) => {
            checks.push(ValidationCheck::failed_msg(
                "Agent Equivalence",
                format!("Failed to read Claude agents directory: {e}"),
            ));
            return checks;
        }
    };

    let mut files: Vec<(PathBuf, String)> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        if !name.ends_with(".md") || name == "README.md" {
            continue;
        }
        files.push((entry.path(), name));
    }
    files.sort_by(|a, b| a.1.cmp(&b.1));

    for (claude_path, name) in files {
        let opencode_path = opencode_dir.join(&name);
        checks.push(validate_agent_file(&name, &claude_path, &opencode_path));
    }

    checks
}

fn validate_agent_file(name: &str, claude_path: &Path, opencode_path: &Path) -> ValidationCheck {
    let check_name = format!("Agent: {name}");

    let claude_content = match fs::read(claude_path) {
        Ok(v) => v,
        Err(e) => {
            return ValidationCheck::failed_msg(
                &check_name,
                format!("Failed to read Claude agent: {e}"),
            )
        }
    };
    let opencode_content = match fs::read(opencode_path) {
        Ok(v) => v,
        Err(e) => {
            return ValidationCheck::failed_msg(
                &check_name,
                format!("Failed to read OpenCode agent: {e}"),
            )
        }
    };

    let (claude_front, claude_body) = match extract_frontmatter(&claude_content) {
        Ok(v) => v,
        Err(e) => {
            return ValidationCheck::failed_msg(
                &check_name,
                format!("Failed to parse Claude frontmatter: {e}"),
            )
        }
    };
    let (opencode_front, opencode_body) = match extract_frontmatter(&opencode_content) {
        Ok(v) => v,
        Err(e) => {
            return ValidationCheck::failed_msg(
                &check_name,
                format!("Failed to parse OpenCode frontmatter: {e}"),
            )
        }
    };

    let claude_str = String::from_utf8_lossy(&claude_front).into_owned();
    let opencode_str = String::from_utf8_lossy(&opencode_front).into_owned();

    let claude_yaml: Value = match serde_norway::from_str(&claude_str) {
        Ok(v) => v,
        Err(e) => {
            return ValidationCheck::failed_msg(
                &check_name,
                format!("Failed to parse Claude YAML: {e}"),
            )
        }
    };
    let opencode_yaml: Value = match serde_norway::from_str(&opencode_str) {
        Ok(v) => v,
        Err(e) => {
            return ValidationCheck::failed_msg(
                &check_name,
                format!("Failed to parse OpenCode YAML: {e}"),
            )
        }
    };

    let claude_desc = claude_yaml
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let opencode_desc = opencode_yaml
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if claude_desc != opencode_desc {
        return ValidationCheck::failed(
            &check_name,
            "Matching descriptions",
            "Descriptions differ",
            "Description mismatch",
        );
    }

    let claude_model = claude_yaml
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let expected_model = convert_model(claude_model);
    let opencode_model = opencode_yaml
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if expected_model != opencode_model {
        return ValidationCheck::failed(
            &check_name,
            format!("Model: {expected_model}"),
            format!("Model: {opencode_model}"),
            "Model mismatch",
        );
    }

    let claude_tools = match claude_yaml.get("tools") {
        Some(v) => parse_claude_tools(v),
        None => Vec::new(),
    };
    let expected_tools = convert_tools(&claude_tools);
    let opencode_tools = parse_opencode_tools(opencode_yaml.get("tools"));
    if !tools_match(&expected_tools, &opencode_tools) {
        return ValidationCheck::failed(
            &check_name,
            format!(
                "Tools: {}",
                format_string_slice_owned(&sorted_keys(&expected_tools))
            ),
            format!(
                "Tools: {}",
                format_string_slice_owned(&sorted_keys(&opencode_tools))
            ),
            "Tools mismatch",
        );
    }

    let claude_skills = parse_string_seq(claude_yaml.get("skills"));
    let opencode_skills = parse_string_seq(opencode_yaml.get("skills"));
    if !skills_match(&claude_skills, &opencode_skills) {
        return ValidationCheck::failed(
            &check_name,
            format!("Skills: {}", format_string_slice_owned(&claude_skills)),
            format!("Skills: {}", format_string_slice_owned(&opencode_skills)),
            "Skills mismatch",
        );
    }

    if claude_body != opencode_body {
        return ValidationCheck::failed(
            &check_name,
            "Matching body content",
            "Body content differs",
            "Body mismatch",
        );
    }

    ValidationCheck::passed(&check_name, "Agent is semantically equivalent")
}

fn parse_opencode_tools(v: Option<&Value>) -> BTreeMap<String, bool> {
    let mut out = BTreeMap::new();
    if let Some(Value::Mapping(m)) = v {
        for (k, val) in m {
            if let Some(key) = k.as_str() {
                let b = val.as_bool().unwrap_or(false);
                out.insert(key.to_string(), b);
            }
        }
    }
    out
}

fn parse_string_seq(v: Option<&Value>) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(Value::Sequence(seq)) = v {
        for item in seq {
            if let Some(s) = item.as_str() {
                out.push(s.to_string());
            }
        }
    }
    out
}

fn validate_no_synced_skills(repo_root: &Path) -> ValidationCheck {
    let claude_dir = repo_root.join(".claude").join("skills");
    let mut claude_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(entries) = fs::read_dir(&claude_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let skill_file = entry.path().join("SKILL.md");
                if skill_file.exists() {
                    claude_names.insert(entry.file_name().to_string_lossy().into_owned());
                }
            }
        }
    }

    let mirror_dirs = [
        repo_root.join(".opencode").join("skill"),
        repo_root.join(".opencode").join("skills"),
    ];

    let mut offenders: Vec<String> = Vec::new();
    for dir in &mirror_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
                    && claude_names.contains(&name)
                {
                    let skill_file = entry.path().join("SKILL.md");
                    if skill_file.exists() {
                        offenders.push(entry.path().to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    if offenders.is_empty() {
        ValidationCheck::passed(
            "No Synced Skill Mirror",
            "No rhino-cli-managed skill copies under .opencode/skill or .opencode/skills",
        )
    } else {
        ValidationCheck::failed(
            "No Synced Skill Mirror",
            "No skill copy mirroring .claude/skills/<name>",
            format!(
                "Found {} mirrored skill dir(s): {}",
                offenders.len(),
                format_string_slice_owned(&offenders)
            ),
            "OpenCode reads .claude/skills/ natively; remove the mirror copies",
        )
    }
}

fn count_markdown_files(dir: &Path) -> usize {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut count = 0;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type().map(|t| !t.is_dir()).unwrap_or(false)
            && name.ends_with(".md")
            && name != "README.md"
        {
            count += 1;
        }
    }
    count
}

fn tools_match(a: &BTreeMap<String, bool>, b: &BTreeMap<String, bool>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (k, v) in a {
        if b.get(k) != Some(v) {
            return false;
        }
    }
    true
}

fn skills_match(a: &[String], b: &[String]) -> bool {
    a == b
}

fn sorted_keys(m: &BTreeMap<String, bool>) -> Vec<String> {
    m.keys().cloned().collect()
}

fn format_string_slice_owned(s: &[String]) -> String {
    format!("[{}]", s.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &Path, content: &str) {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    fn setup() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let claude = root.join(".claude/agents");
        let opencode = root.join(".opencode/agents");
        std::fs::create_dir_all(&claude).unwrap();
        std::fs::create_dir_all(&opencode).unwrap();
        write(
            &claude.join("foo.md"),
            "---\nname: foo\ndescription: desc\ntools: Read, Write\nmodel: sonnet\nskills:\n  - my-skill\n---\nBody\n",
        );
        write(
            &opencode.join("foo.md"),
            "---\ndescription: desc\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n  write: true\nskills:\n  - my-skill\n---\nBody\n",
        );
        dir
    }

    #[test]
    fn validate_no_stale_agent_dir_passes_when_absent() {
        let dir = tempdir().unwrap();
        let c = validate_no_stale_agent_dir(dir.path());
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_no_stale_agent_dir_fails_when_present_as_dir() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".opencode/agent")).unwrap();
        let c = validate_no_stale_agent_dir(dir.path());
        assert_eq!(c.status, "failed");
        assert!(c.actual.contains("exists as a directory"));
    }

    #[test]
    fn validate_no_stale_agent_dir_fails_when_present_as_file() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".opencode")).unwrap();
        std::fs::write(dir.path().join(".opencode/agent"), b"x").unwrap();
        let c = validate_no_stale_agent_dir(dir.path());
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_agent_count_passes() {
        let dir = setup();
        let c = validate_agent_count(dir.path());
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_agent_count_fails_with_missing() {
        let dir = setup();
        std::fs::remove_file(dir.path().join(".opencode/agents/foo.md")).unwrap();
        let c = validate_agent_count(dir.path());
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_agent_equivalence_pass() {
        let dir = setup();
        let checks = validate_agent_equivalence(dir.path());
        assert!(!checks.is_empty());
        assert!(checks.iter().all(|c| c.status == "passed"), "{checks:#?}");
    }

    #[test]
    fn validate_agent_equivalence_fails_on_desc_mismatch() {
        let dir = setup();
        write(
            &dir.path().join(".opencode/agents/foo.md"),
            "---\ndescription: NOPE\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n  write: true\nskills:\n  - my-skill\n---\nBody\n",
        );
        let checks = validate_agent_equivalence(dir.path());
        assert!(checks
            .iter()
            .any(|c| c.status == "failed" && c.message == "Description mismatch"));
    }

    #[test]
    fn validate_agent_equivalence_fails_on_model_mismatch() {
        let dir = setup();
        write(
            &dir.path().join(".opencode/agents/foo.md"),
            "---\ndescription: desc\nmodel: opencode-go/wrong\ntools:\n  read: true\n  write: true\nskills:\n  - my-skill\n---\nBody\n",
        );
        let checks = validate_agent_equivalence(dir.path());
        assert!(checks
            .iter()
            .any(|c| c.status == "failed" && c.message == "Model mismatch"));
    }

    #[test]
    fn validate_agent_equivalence_fails_on_tools_mismatch() {
        let dir = setup();
        write(
            &dir.path().join(".opencode/agents/foo.md"),
            "---\ndescription: desc\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\nskills:\n  - my-skill\n---\nBody\n",
        );
        let checks = validate_agent_equivalence(dir.path());
        assert!(checks
            .iter()
            .any(|c| c.status == "failed" && c.message == "Tools mismatch"));
    }

    #[test]
    fn validate_agent_equivalence_fails_on_skills_mismatch() {
        let dir = setup();
        write(
            &dir.path().join(".opencode/agents/foo.md"),
            "---\ndescription: desc\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n  write: true\nskills:\n  - other-skill\n---\nBody\n",
        );
        let checks = validate_agent_equivalence(dir.path());
        assert!(checks
            .iter()
            .any(|c| c.status == "failed" && c.message == "Skills mismatch"));
    }

    #[test]
    fn validate_agent_equivalence_fails_on_body_mismatch() {
        let dir = setup();
        write(
            &dir.path().join(".opencode/agents/foo.md"),
            "---\ndescription: desc\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n  write: true\nskills:\n  - my-skill\n---\nDifferent Body\n",
        );
        let checks = validate_agent_equivalence(dir.path());
        assert!(checks
            .iter()
            .any(|c| c.status == "failed" && c.message == "Body mismatch"));
    }

    #[test]
    fn validate_no_synced_skills_passes() {
        let dir = setup();
        // Create .claude/skills/my-skill/SKILL.md but no mirror
        write(
            &dir.path().join(".claude/skills/my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: ok\n---\n",
        );
        let c = validate_no_synced_skills(dir.path());
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn validate_no_synced_skills_fails_with_mirror() {
        let dir = setup();
        write(
            &dir.path().join(".claude/skills/my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: ok\n---\n",
        );
        write(
            &dir.path().join(".opencode/skills/my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: ok\n---\n",
        );
        let c = validate_no_synced_skills(dir.path());
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn validate_no_synced_skills_tolerates_extra_skill() {
        let dir = setup();
        write(
            &dir.path().join(".claude/skills/my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: ok\n---\n",
        );
        write(
            &dir.path().join(".opencode/skills/other-skill/SKILL.md"),
            "---\nname: other-skill\ndescription: ok\n---\n",
        );
        let c = validate_no_synced_skills(dir.path());
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn count_markdown_files_excludes_readme() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path()).unwrap();
        write(&dir.path().join("a.md"), "x");
        write(&dir.path().join("README.md"), "x");
        assert_eq!(count_markdown_files(dir.path()), 1);
    }

    #[test]
    fn tools_match_basic() {
        let mut a = BTreeMap::new();
        a.insert("read".to_string(), true);
        let mut b = BTreeMap::new();
        b.insert("read".to_string(), true);
        assert!(tools_match(&a, &b));
        b.insert("write".to_string(), true);
        assert!(!tools_match(&a, &b));
    }

    #[test]
    fn full_validate_sync_pass() {
        let dir = setup();
        let r = validate_sync(dir.path());
        assert_eq!(r.failed_checks, 0, "result: {r:#?}");
    }
}
