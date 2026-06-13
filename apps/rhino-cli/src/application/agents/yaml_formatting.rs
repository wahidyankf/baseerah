//! YAML formatting validator ported from
//! `apps/rhino-cli/internal/agents/yaml_formatting.go`.
//
// Checks for missing spaces after colons in frontmatter key-value pairs.

use super::types::ValidationCheck;

/// Check that every key-value line in the frontmatter of `content` has a space after its colon.
#[allow(clippy::collapsible_if)]
pub fn validate_yaml_formatting_raw(check_name: &str, content: &[u8]) -> ValidationCheck {
    let s = String::from_utf8_lossy(content);
    let lines: Vec<&str> = s.split('\n').collect();

    if lines.len() < 3 {
        return ValidationCheck::passed(check_name, "File too short to check formatting");
    }

    if lines[0].trim() != "---" {
        return ValidationCheck::failed_msg(check_name, "Frontmatter does not start with ---");
    }

    let Some(end) = lines
        .iter()
        .enumerate()
        .skip(1)
        .find_map(|(i, line)| (line.trim() == "---").then_some(i))
    else {
        return ValidationCheck::failed_msg(check_name, "Frontmatter closing --- not found");
    };

    let mut issues: Vec<String> = Vec::new();
    for (i, line) in lines.iter().enumerate().take(end).skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('-') || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.contains(':') {
            // Equivalent of bytes.SplitN(line, ":", 2)
            let mut parts = trimmed.splitn(2, ':');
            let _key = parts.next();
            if let Some(rest) = parts.next() {
                if !rest.is_empty() && !rest.starts_with(' ') {
                    issues.push(format!(
                        "Line {}: '{}' (missing space after colon)",
                        i + 1,
                        trimmed
                    ));
                }
            }
        }
    }

    if !issues.is_empty() {
        return ValidationCheck::failed(
            check_name,
            "Space after colon in YAML key-value pairs (e.g., 'name: value')",
            format!("Found {} formatting issues", issues.len()),
            format!("YAML formatting errors:\n  {}", issues.join("\n  ")),
        );
    }

    ValidationCheck::passed(check_name, "YAML formatting correct (spaces after colons)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_well_formed() {
        let c = b"---\nname: foo\ndescription: bar\n---\nbody\n";
        let v = validate_yaml_formatting_raw("Agent: foo.md - YAML Formatting", c);
        assert_eq!(v.status, "passed");
    }

    #[test]
    fn fails_missing_space() {
        let c = b"---\nname:foo\ndescription: bar\n---\nbody\n";
        let v = validate_yaml_formatting_raw("Agent: foo.md - YAML Formatting", c);
        assert_eq!(v.status, "failed");
        assert!(v.message.contains("missing space after colon"));
    }

    #[test]
    fn passes_short_file() {
        let c = b"---\n---\n";
        let v = validate_yaml_formatting_raw("X", c);
        assert_eq!(v.status, "passed");
    }

    #[test]
    fn fails_no_open() {
        let v = validate_yaml_formatting_raw("X", b"name: x\ndescription: y\nbody: z\n");
        assert_eq!(v.status, "failed");
        assert!(v.message.contains("does not start with ---"));
    }

    #[test]
    fn fails_no_close() {
        let v = validate_yaml_formatting_raw("X", b"---\nname: x\nbody only\nno close\n");
        assert_eq!(v.status, "failed");
        assert!(v.message.contains("closing --- not found"));
    }
}
