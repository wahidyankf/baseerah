//! Skill validator ported from
//! `apps/rhino-cli/internal/agents/skill_validator.go`.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde_norway::Value;

use super::frontmatter::extract_frontmatter;
use super::types::{
    ClaudeSkill, ValidationCheck, valid_claude_skill_fields, valid_skill_name_pattern,
};
use super::yaml_formatting::validate_yaml_formatting_raw;

/// Validate a single skill directory at `skill_path` and return all check results.
pub fn validate_skill(skill_path: &Path, skill_name: &str) -> Vec<ValidationCheck> {
    let mut checks: Vec<ValidationCheck> = Vec::new();
    let skill_file = skill_path.join("SKILL.md");

    if !skill_file.exists() {
        checks.push(ValidationCheck::failed(
            format!("Skill: {skill_name} - SKILL.md Exists"),
            "SKILL.md file present",
            "SKILL.md file not found",
            "SKILL.md file missing",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - SKILL.md Exists"),
        "SKILL.md file exists",
    ));

    let content = match fs::read(&skill_file) {
        Ok(c) => c,
        Err(e) => {
            checks.push(ValidationCheck::failed_msg(
                format!("Skill: {skill_name} - Read SKILL.md"),
                format!("Failed to read SKILL.md: {e}"),
            ));
            return checks;
        }
    };

    let formatting_check =
        validate_yaml_formatting_raw(&format!("Skill: {skill_name} - YAML Formatting"), &content);
    let formatting_failed = formatting_check.status == "failed";
    checks.push(formatting_check);
    if formatting_failed {
        return checks;
    }

    let (frontmatter, _) = match extract_frontmatter(&content) {
        Ok(v) => v,
        Err(e) => {
            checks.push(ValidationCheck::failed_msg(
                format!("Skill: {skill_name} - YAML Syntax"),
                format!("Invalid frontmatter: {e}"),
            ));
            return checks;
        }
    };
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - YAML Syntax"),
        "Valid YAML frontmatter",
    ));

    let frontmatter_str = String::from_utf8_lossy(&frontmatter).into_owned();
    let skill = match parse_skill_yaml(&frontmatter_str) {
        Ok(s) => s,
        Err(e) => {
            checks.push(ValidationCheck::failed_msg(
                format!("Skill: {skill_name} - YAML Parse"),
                format!("Failed to parse YAML: {e}"),
            ));
            return checks;
        }
    };

    checks.extend(validate_skill_fields(&skill, &frontmatter_str, skill_name));
    checks
}

/// Run field-level checks on a parsed skill (description, name, name format, uniqueness).
fn validate_skill_fields(
    skill: &ClaudeSkill,
    frontmatter_str: &str,
    skill_name: &str,
) -> Vec<ValidationCheck> {
    let mut checks: Vec<ValidationCheck> = Vec::new();

    if skill.description.is_empty() {
        checks.push(ValidationCheck::failed(
            format!("Skill: {skill_name} - Description Field Required"),
            "description field present",
            "description field missing or empty",
            "Required description field missing",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Description Field Required"),
        "Required description field present",
    ));

    if skill.name.is_empty() {
        checks.push(ValidationCheck::failed(
            format!("Skill: {skill_name} - Name Field Required"),
            "name field present",
            "name field missing or empty",
            "Required name field missing",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Name Field Required"),
        "Required name field present",
    ));

    if !valid_skill_name_pattern().is_match(&skill.name) {
        checks.push(ValidationCheck::failed(
            format!("Skill: {skill_name} - Name Format"),
            "Lowercase letters/numbers/hyphens only, max 64 chars",
            format!("Name: {}", skill.name),
            "Invalid skill name format",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Name Format"),
        "Name format valid",
    ));

    if skill.name != skill_name {
        checks.push(ValidationCheck::failed(
            format!("Skill: {skill_name} - Name Match"),
            format!("name field matches directory: {skill_name}"),
            format!("name field: {}", skill.name),
            "Skill name must match directory name",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Name Match"),
        "Name matches directory name",
    ));

    #[allow(clippy::collapsible_if)]
    if let Ok(Value::Mapping(m)) = serde_norway::from_str::<Value>(frontmatter_str) {
        let allow = valid_claude_skill_fields();
        for (k, _) in m {
            if let Some(key) = k.as_str() {
                if !allow.contains_key(key) {
                    checks.push(ValidationCheck::warning(
                        format!("Skill: {skill_name} - Unknown Field: {key}"),
                        "Field listed in ValidClaudeSkillFields",
                        format!("Unknown field: {key}"),
                        format!(
                            "Field \"{key}\" is not in the documented Claude Code skill field set; verify it is intentional"
                        ),
                    ));
                }
            }
        }
    }

    checks
}

/// Parse a YAML frontmatter string into a `ClaudeSkill`.
fn parse_skill_yaml(frontmatter: &str) -> Result<ClaudeSkill, String> {
    let v: Value =
        serde_norway::from_str(frontmatter).map_err(|e| format!("yaml parse error: {e}"))?;
    let mut skill = ClaudeSkill::default();
    if let Value::Mapping(m) = v {
        for (k, val) in m {
            let key = k.as_str().unwrap_or("").to_string();
            match key.as_str() {
                "name" => skill.name = val.as_str().unwrap_or("").to_string(),
                "description" => skill.description = val.as_str().unwrap_or("").to_string(),
                _ => {}
            }
        }
    }
    Ok(skill)
}

/// Validate all skill directories under `.claude/skills/` and return checks + passing skill names.
pub fn validate_all_skills(repo_root: &Path) -> (Vec<ValidationCheck>, HashSet<String>) {
    let skills_dir = repo_root.join(".claude").join("skills");
    let entries = match fs::read_dir(&skills_dir) {
        Ok(e) => e,
        Err(e) => {
            return (
                vec![ValidationCheck::failed_msg(
                    "Read Skills Directory",
                    format!("Failed to read skills directory: {e}"),
                )],
                HashSet::new(),
            );
        }
    };

    let mut skill_names: HashSet<String> = HashSet::new();
    let mut all: Vec<ValidationCheck> = Vec::new();

    let mut dirs: Vec<(std::path::PathBuf, String)> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        if name.starts_with('.') {
            continue;
        }
        dirs.push((entry.path(), name));
    }
    dirs.sort_by(|a, b| a.1.cmp(&b.1));

    for (path, name) in dirs {
        let checks = validate_skill(&path, &name);
        let all_passed = checks.iter().all(|c| c.status != "failed");
        all.extend(checks);
        if all_passed {
            skill_names.insert(name);
        }
    }

    (all, skill_names)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &Path, content: &str) {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn missing_skill_md_fails() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("test-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        let checks = validate_skill(&skill_dir, "test-skill");
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].status, "failed");
    }

    #[test]
    fn valid_skill_passes_all() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        write(
            &skill_dir.join("SKILL.md"),
            "---\nname: my-skill\ndescription: ok\n---\nBody\n",
        );
        let checks = validate_skill(&skill_dir, "my-skill");
        let failed = checks.iter().filter(|c| c.status == "failed").count();
        assert_eq!(failed, 0, "checks: {checks:#?}");
    }

    #[test]
    fn name_mismatch_fails() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("dir-name");
        std::fs::create_dir_all(&skill_dir).unwrap();
        write(
            &skill_dir.join("SKILL.md"),
            "---\nname: other-name\ndescription: ok\n---\n",
        );
        let checks = validate_skill(&skill_dir, "dir-name");
        assert!(
            checks
                .iter()
                .any(|c| c.status == "failed" && c.name.contains("Name Match"))
        );
    }

    #[test]
    fn bad_name_format_fails() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("Bad_Name");
        std::fs::create_dir_all(&skill_dir).unwrap();
        write(
            &skill_dir.join("SKILL.md"),
            "---\nname: Bad_Name\ndescription: ok\n---\n",
        );
        let checks = validate_skill(&skill_dir, "Bad_Name");
        assert!(
            checks
                .iter()
                .any(|c| c.status == "failed" && c.name.contains("Name Format"))
        );
    }

    #[test]
    fn missing_description_fails() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("no-desc");
        std::fs::create_dir_all(&skill_dir).unwrap();
        write(&skill_dir.join("SKILL.md"), "---\nname: no-desc\n---\n");
        let checks = validate_skill(&skill_dir, "no-desc");
        assert!(
            checks
                .iter()
                .any(|c| c.status == "failed" && c.name.contains("Description Field Required"))
        );
    }

    #[test]
    fn missing_name_fails() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("no-name");
        std::fs::create_dir_all(&skill_dir).unwrap();
        write(&skill_dir.join("SKILL.md"), "---\ndescription: ok\n---\n");
        let checks = validate_skill(&skill_dir, "no-name");
        assert!(
            checks
                .iter()
                .any(|c| c.status == "failed" && c.name.contains("Name Field Required"))
        );
    }

    #[test]
    fn unknown_field_warns() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("with-unknown");
        std::fs::create_dir_all(&skill_dir).unwrap();
        write(
            &skill_dir.join("SKILL.md"),
            "---\nname: with-unknown\ndescription: ok\nbogus: yes\n---\n",
        );
        let checks = validate_skill(&skill_dir, "with-unknown");
        assert!(
            checks
                .iter()
                .any(|c| c.status == "warning" && c.name.contains("Unknown Field: bogus"))
        );
    }

    #[test]
    fn validate_all_skills_missing_dir() {
        let dir = tempdir().unwrap();
        let (checks, names) = validate_all_skills(dir.path());
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].status, "failed");
        assert!(names.is_empty());
    }

    #[test]
    fn validate_all_skills_collects_names() {
        let dir = tempdir().unwrap();
        let skills = dir.path().join(".claude").join("skills");
        std::fs::create_dir_all(skills.join("a")).unwrap();
        write(
            &skills.join("a/SKILL.md"),
            "---\nname: a\ndescription: ok\n---\n",
        );
        std::fs::create_dir_all(skills.join("b")).unwrap();
        write(
            &skills.join("b/SKILL.md"),
            "---\nname: b\ndescription: ok\n---\n",
        );
        let (_, names) = validate_all_skills(dir.path());
        assert!(names.contains("a"));
        assert!(names.contains("b"));
    }

    #[test]
    fn yaml_formatting_failure_returns_early() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join("bad");
        std::fs::create_dir_all(&skill_dir).unwrap();
        write(
            &skill_dir.join("SKILL.md"),
            "---\nname:bad\ndescription: ok\n---\n",
        );
        let checks = validate_skill(&skill_dir, "bad");
        assert!(
            checks
                .iter()
                .any(|c| c.status == "failed" && c.name.contains("YAML Formatting"))
        );
    }
}
