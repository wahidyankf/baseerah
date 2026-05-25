//! Claude validator entry point ported from
//! `apps/rhino-cli/internal/agents/claude_validator.go`.

use std::time::Instant;

use super::agent_validator::validate_all_agents;
use super::skill_validator::validate_all_skills;
use super::types::{ValidateClaudeOptions, ValidationResult};

/// Run the full Claude binding validation (skills + agents) according to `opts`.
pub fn validate_claude(opts: &ValidateClaudeOptions) -> ValidationResult {
    let start = Instant::now();
    let mut result = ValidationResult::default();

    let skill_names;
    if opts.agents_only {
        // agents-only — still need skill names for skills-exist validation
        let (_, names) = validate_all_skills(&opts.repo_root);
        skill_names = names;
    } else {
        let (skill_checks, names) = validate_all_skills(&opts.repo_root);
        skill_names = names;
        for check in skill_checks {
            result.tally(check);
        }
    }

    if !opts.skills_only {
        let agent_checks = validate_all_agents(&opts.repo_root, &skill_names);
        for check in agent_checks {
            result.tally(check);
        }
    }

    result.duration = start.elapsed();
    result
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &std::path::Path, content: &str) {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    fn corpus() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let agents = root.join(".claude/agents");
        let skills = root.join(".claude/skills");
        std::fs::create_dir_all(&agents).unwrap();
        std::fs::create_dir_all(&skills).unwrap();
        write(
            &agents.join("foo.md"),
            "---\nname: foo\ndescription: desc\ntools: Read, Write\nmodel: sonnet\ncolor: blue\nskills:\n  - my-skill\n---\nBody\n",
        );
        write(
            &skills.join("my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: ok\n---\nbody\n",
        );
        dir
    }

    #[test]
    fn full_pass() {
        let dir = corpus();
        let opts = ValidateClaudeOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        let r = validate_claude(&opts);
        assert_eq!(r.failed_checks, 0, "result: {r:#?}");
        assert!(r.passed_checks > 0);
    }

    #[test]
    fn agents_only_skips_skill_checks() {
        let dir = corpus();
        let opts = ValidateClaudeOptions {
            repo_root: dir.path().to_path_buf(),
            agents_only: true,
            ..Default::default()
        };
        let r = validate_claude(&opts);
        assert!(r.checks.iter().all(|c| !c.name.starts_with("Skill:")));
    }

    #[test]
    fn skills_only_skips_agent_checks() {
        let dir = corpus();
        let opts = ValidateClaudeOptions {
            repo_root: dir.path().to_path_buf(),
            skills_only: true,
            ..Default::default()
        };
        let r = validate_claude(&opts);
        assert!(r.checks.iter().all(|c| !c.name.starts_with("Agent:")));
    }

    #[test]
    fn duration_recorded() {
        let dir = corpus();
        let opts = ValidateClaudeOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        let r = validate_claude(&opts);
        assert!(r.duration.as_nanos() > 0);
    }
}
