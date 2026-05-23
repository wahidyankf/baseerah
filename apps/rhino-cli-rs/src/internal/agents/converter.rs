// Converter helpers (subset needed for validate-sync) ported from
// `apps/rhino-cli/internal/agents/converter.go`.

use std::collections::BTreeMap;

pub const OPENCODE_AGENT_DIR: &str = ".opencode/agents";

/// Converts a Claude tools array to an OpenCode tools map.
/// Lower-cases each entry; empty entries are dropped.
pub fn convert_tools(claude_tools: &[String]) -> BTreeMap<String, bool> {
    let mut m = BTreeMap::new();
    for t in claude_tools {
        let lower = t.trim().to_lowercase();
        if !lower.is_empty() {
            m.insert(lower, true);
        }
    }
    m
}

/// Converts a Claude model alias to the corresponding OpenCode model ID.
pub fn convert_model(claude_model: &str) -> String {
    let m = claude_model.trim();
    if m == "haiku" {
        "opencode-go/glm-5".to_string()
    } else {
        "opencode-go/minimax-m2.7".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_tools_lowercases() {
        let in_tools = vec!["Read".to_string(), "Write".to_string(), "BASH".to_string()];
        let out = convert_tools(&in_tools);
        assert_eq!(out.get("read"), Some(&true));
        assert_eq!(out.get("write"), Some(&true));
        assert_eq!(out.get("bash"), Some(&true));
    }

    #[test]
    fn convert_tools_skips_empty() {
        let in_tools = vec!["".to_string(), "  ".to_string(), "Read".to_string()];
        let out = convert_tools(&in_tools);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn convert_model_haiku() {
        assert_eq!(convert_model("haiku"), "opencode-go/glm-5");
    }

    #[test]
    fn convert_model_default() {
        assert_eq!(convert_model("sonnet"), "opencode-go/minimax-m2.7");
        assert_eq!(convert_model(""), "opencode-go/minimax-m2.7");
        assert_eq!(convert_model("inherit"), "opencode-go/minimax-m2.7");
    }
}
