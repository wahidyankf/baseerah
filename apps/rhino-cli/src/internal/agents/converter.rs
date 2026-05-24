// Converter ported from `apps/rhino-cli/internal/agents/converter.go`.
//
// Implements:
// - OpenCodeAgent struct (emit shape with omitempty parity)
// - ConversionWarning struct
// - ConvertModel, ConvertTools, ConvertColor
// - ConvertAgent: reads a Claude .md, writes the OpenCode equivalent
// - ConvertAllAgents: iterates .claude/agents/ and writes .opencode/agents/

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_norway::Value;

use super::frontmatter::{extract_frontmatter, parse_claude_tools};

pub const OPENCODE_AGENT_DIR: &str = ".opencode/agents";

#[derive(Debug, Clone)]
pub struct ConversionWarning {
    pub agent_name: String,
    pub field: String,
    pub reason: String,
}

/// OpenCodeAgent — emit shape matching Go's struct.
/// Field order: description, model, tools, color, steps, skills.
/// omitempty fields: color (string), steps (0), skills ([]).
#[derive(Debug, Clone, Default)]
pub struct OpenCodeAgent {
    pub description: String,
    pub model: String,
    pub tools: BTreeMap<String, bool>,
    pub color: String,
    pub steps: i64,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldAction {
    Preserve,
    Translate,
    Drop,
    DropWarn,
}

struct FieldPolicy {
    action: FieldAction,
    reason: &'static str,
}

const FIELD_POLICY_TABLE: &[(&str, FieldAction, &str)] = &[
    ("name", FieldAction::Drop, "filename carries name"),
    ("description", FieldAction::Preserve, ""),
    ("tools", FieldAction::Translate, ""),
    ("model", FieldAction::Translate, ""),
    ("color", FieldAction::Translate, ""),
    ("skills", FieldAction::Preserve, ""),
    ("maxTurns", FieldAction::Translate, ""),
    (
        "disallowedTools",
        FieldAction::DropWarn,
        "no opencode equivalent",
    ),
    (
        "permissionMode",
        FieldAction::DropWarn,
        "use opencode permission block",
    ),
    ("effort", FieldAction::DropWarn, "claude-only"),
    ("memory", FieldAction::DropWarn, "claude-only"),
    ("isolation", FieldAction::DropWarn, "claude-only"),
    ("background", FieldAction::DropWarn, "claude-only"),
    ("initialPrompt", FieldAction::DropWarn, "claude-only"),
    (
        "mcpServers",
        FieldAction::DropWarn,
        "opencode declares mcp at config level",
    ),
    ("hooks", FieldAction::DropWarn, "no opencode equivalent"),
];

fn claude_agent_field_policy() -> &'static HashMap<&'static str, FieldPolicy> {
    static M: OnceLock<HashMap<&'static str, FieldPolicy>> = OnceLock::new();
    M.get_or_init(|| {
        FIELD_POLICY_TABLE
            .iter()
            .map(|(k, action, reason)| {
                (
                    *k,
                    FieldPolicy {
                        action: *action,
                        reason,
                    },
                )
            })
            .collect()
    })
}

fn claude_to_opencode_color() -> &'static HashMap<&'static str, &'static str> {
    static M: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("blue", "primary");
        m.insert("green", "success");
        m.insert("yellow", "warning");
        m.insert("purple", "secondary");
        m.insert("red", "error");
        m.insert("orange", "warning");
        m.insert("pink", "accent");
        m.insert("cyan", "info");
        m
    })
}

pub fn convert_color(c: &str) -> String {
    if c.is_empty() {
        return String::new();
    }
    if let Some(mapped) = claude_to_opencode_color().get(c) {
        return (*mapped).to_string();
    }
    c.to_string()
}

fn agent_name_from_path(p: &Path) -> String {
    let base = p
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    base.strip_suffix(".md").unwrap_or(&base).to_string()
}

/// Convert a single Claude agent file to OpenCode format. Returns conversion
/// warnings; writes to `output_path` unless `dry_run` is true.
pub fn convert_agent(
    input_path: &Path,
    output_path: &Path,
    dry_run: bool,
) -> Result<Vec<ConversionWarning>, String> {
    let content = fs::read(input_path).map_err(|e| format!("failed to read file: {e}"))?;
    let (frontmatter, body) =
        extract_frontmatter(&content).map_err(|e| format!("failed to extract frontmatter: {e}"))?;

    let frontmatter_str = String::from_utf8_lossy(&frontmatter).into_owned();
    let value: Value = serde_norway::from_str(&frontmatter_str)
        .map_err(|e| format!("failed to parse YAML: {e}"))?;

    let Value::Mapping(mapping) = value else {
        return Err("frontmatter is not a mapping".to_string());
    };

    let agent_name = agent_name_from_path(input_path);
    let mut warnings: Vec<ConversionWarning> = Vec::new();
    let mut out = OpenCodeAgent::default();

    let policy_map = claude_agent_field_policy();

    for (k, v) in mapping {
        let Some(s) = k.as_str() else { continue };
        let key = s.to_string();
        let Some(policy) = policy_map.get(key.as_str()) else {
            warnings.push(ConversionWarning {
                agent_name: agent_name.clone(),
                field: key.clone(),
                reason: "unknown claude code field".to_string(),
            });
            continue;
        };
        match policy.action {
            FieldAction::Drop => {}
            FieldAction::DropWarn => {
                warnings.push(ConversionWarning {
                    agent_name: agent_name.clone(),
                    field: key.clone(),
                    reason: policy.reason.to_string(),
                });
            }
            FieldAction::Preserve => apply_preserve(&mut out, &key, &v),
            FieldAction::Translate => apply_translate(&mut out, &key, &v),
        }
    }

    let new_frontmatter = encode_opencode_agent(&out);

    let mut output = Vec::new();
    output.extend_from_slice(b"---\n");
    output.extend_from_slice(new_frontmatter.as_bytes());
    output.extend_from_slice(b"---\n");
    output.extend_from_slice(&body);

    if !dry_run {
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create output directory: {e}"))?;
        }
        fs::write(output_path, &output).map_err(|e| format!("failed to write file: {e}"))?;
    }

    Ok(warnings)
}

fn apply_preserve(out: &mut OpenCodeAgent, key: &str, value: &Value) {
    match key {
        "description" => {
            if let Some(s) = value.as_str() {
                out.description = s.to_string();
            }
        }
        "skills" => {
            if let Value::Sequence(seq) = value {
                out.skills = seq
                    .iter()
                    .filter_map(|v| v.as_str().map(std::string::ToString::to_string))
                    .collect();
            }
        }
        _ => {}
    }
}

// maxTurns may arrive as f64 from YAML; cast to i64 is safe for small whole-number turn counts.
#[allow(clippy::cast_possible_truncation)]
fn apply_translate(out: &mut OpenCodeAgent, key: &str, value: &Value) {
    match key {
        "tools" => {
            let tools = parse_claude_tools(value);
            out.tools = convert_tools(&tools);
        }
        "model" => {
            let s = value.as_str().unwrap_or("");
            out.model = convert_model(s);
        }
        "maxTurns" => {
            if let Some(n) = value.as_i64() {
                out.steps = n;
            } else if let Some(f) = value.as_f64() {
                out.steps = f as i64;
            }
        }
        "color" => {
            if let Some(s) = value.as_str() {
                out.color = convert_color(s);
            }
        }
        _ => {}
    }
}

/// Emit OpenCodeAgent as YAML matching Go's gopkg.in/yaml.v3 output:
/// - 2-space indent
/// - description, model, tools always emit
/// - color, steps, skills are omitempty (skip when empty/0/empty-vec)
/// - tools is a map (each entry on own line)
/// - skills is a sequence (each entry on own line, "- skill-name")
fn encode_opencode_agent(a: &OpenCodeAgent) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "description: {}", yaml_string(&a.description));
    let _ = writeln!(s, "model: {}", yaml_string(&a.model));
    if a.tools.is_empty() {
        s.push_str("tools: {}\n");
    } else {
        s.push_str("tools:\n");
        for (k, v) in &a.tools {
            let _ = writeln!(s, "  {k}: {v}");
        }
    }
    if !a.color.is_empty() {
        let _ = writeln!(s, "color: {}", yaml_string(&a.color));
    }
    if a.steps != 0 {
        let _ = writeln!(s, "steps: {}", a.steps);
    }
    if !a.skills.is_empty() {
        s.push_str("skills:\n");
        for sk in &a.skills {
            let _ = writeln!(s, "  - {}", yaml_string(sk));
        }
    }
    s
}

/// Render a Go yaml.v3 plain scalar — bare if no special chars, otherwise
/// double-quoted. Matches Go's default behaviour for most agent string values.
fn yaml_string(s: &str) -> String {
    if needs_quoting(s) {
        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
        format!("\"{escaped}\"")
    } else {
        s.to_string()
    }
}

#[allow(clippy::collapsible_if, clippy::collapsible_match)]
fn needs_quoting(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    // Reserved leading indicator chars per YAML 1.2 plain-scalar rules.
    if let Some(c) = s.chars().next() {
        if matches!(
            c,
            '-' | '?'
                | ':'
                | ','
                | '['
                | ']'
                | '{'
                | '}'
                | '#'
                | '&'
                | '*'
                | '!'
                | '|'
                | '>'
                | '\''
                | '"'
                | '%'
                | '@'
                | '`'
        ) {
            return true;
        }
    }
    // Trailing whitespace forces quotation.
    if s.ends_with(' ') || s.ends_with('\t') {
        return true;
    }
    // ": " inside a value is ambiguous (mapping marker).
    if s.contains(": ") || s.ends_with(':') {
        return true;
    }
    // " #" inside a value would start a comment.
    if s.contains(" #") {
        return true;
    }
    if s.contains('\n') {
        return true;
    }
    false
}

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

#[derive(Debug, Clone, Default)]
pub struct ConvertAllResult {
    pub converted: usize,
    pub failed: usize,
    pub failed_files: Vec<String>,
    pub warnings: Vec<ConversionWarning>,
}

pub fn convert_all_agents(repo_root: &Path, dry_run: bool) -> Result<ConvertAllResult, String> {
    let claude_dir = repo_root.join(".claude").join("agents");
    let opencode_dir = repo_root.join(OPENCODE_AGENT_DIR);

    let entries = fs::read_dir(&claude_dir)
        .map_err(|e| format!("failed to read .claude/agents directory: {e}"))?;

    let mut paths: Vec<(PathBuf, String)> = Vec::new();
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

    let mut result = ConvertAllResult::default();
    for (input, name) in paths {
        let output = opencode_dir.join(&name);
        if let Ok(w) = convert_agent(&input, &output, dry_run) {
            result.converted += 1;
            result.warnings.extend(w);
        } else {
            result.failed += 1;
            result.failed_files.push(name);
        }
    }

    Ok(result)
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
    fn convert_color_translates_known() {
        assert_eq!(convert_color("blue"), "primary");
        assert_eq!(convert_color("green"), "success");
        assert_eq!(convert_color("yellow"), "warning");
        assert_eq!(convert_color("purple"), "secondary");
        assert_eq!(convert_color("red"), "error");
        assert_eq!(convert_color("orange"), "warning");
        assert_eq!(convert_color("pink"), "accent");
        assert_eq!(convert_color("cyan"), "info");
    }

    #[test]
    fn convert_color_passes_through_unknown() {
        assert_eq!(convert_color("primary"), "primary");
        assert_eq!(convert_color("#ff00aa"), "#ff00aa");
        assert_eq!(convert_color(""), "");
    }

    #[test]
    fn agent_name_from_path_strips_extension() {
        assert_eq!(agent_name_from_path(Path::new("/x/foo.md")), "foo");
        assert_eq!(agent_name_from_path(Path::new("/x/bar.md")), "bar");
    }

    #[test]
    fn convert_agent_writes_opencode_file() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("foo.md");
        let output = dir.path().join(".opencode/agents/foo.md");
        write(
            &input,
            "---\nname: foo\ndescription: desc\ntools: Read, Write\nmodel: sonnet\ncolor: blue\nskills:\n  - my-skill\n---\nBody text\n",
        );
        let warnings = convert_agent(&input, &output, false).unwrap();
        assert!(warnings.is_empty());
        let content = std::fs::read_to_string(&output).unwrap();
        assert!(content.starts_with("---\n"));
        assert!(content.contains("description: desc"));
        assert!(content.contains("model: opencode-go/minimax-m2.7"));
        assert!(content.contains("read: true"));
        assert!(content.contains("write: true"));
        assert!(content.contains("color: primary"));
        assert!(content.contains("- my-skill"));
        assert!(content.ends_with("Body text\n"));
    }

    #[test]
    fn convert_agent_dry_run_does_not_write() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("foo.md");
        let output = dir.path().join(".opencode/agents/foo.md");
        write(
            &input,
            "---\nname: foo\ndescription: desc\ntools: Read\nmodel: sonnet\n---\nBody\n",
        );
        convert_agent(&input, &output, true).unwrap();
        assert!(!output.exists());
    }

    #[test]
    fn convert_agent_drops_warn_known_field() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("foo.md");
        let output = dir.path().join("out.md");
        write(
            &input,
            "---\nname: foo\ndescription: d\ntools: Read\nmodel: sonnet\nmcpServers:\n  one: two\n---\nBody\n",
        );
        let warnings = convert_agent(&input, &output, true).unwrap();
        assert!(warnings.iter().any(|w| w.field == "mcpServers"));
    }

    #[test]
    fn convert_agent_warns_unknown_field() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("foo.md");
        let output = dir.path().join("out.md");
        write(
            &input,
            "---\nname: foo\ndescription: d\ntools: Read\nmodel: sonnet\nbogus: yes\n---\nBody\n",
        );
        let warnings = convert_agent(&input, &output, true).unwrap();
        assert!(
            warnings
                .iter()
                .any(|w| w.field == "bogus" && w.reason == "unknown claude code field")
        );
    }

    #[test]
    fn convert_all_agents_iterates() {
        let dir = tempdir().unwrap();
        let claude = dir.path().join(".claude/agents");
        std::fs::create_dir_all(&claude).unwrap();
        write(
            &claude.join("a.md"),
            "---\nname: a\ndescription: a\ntools: Read\nmodel: sonnet\n---\nBody A\n",
        );
        write(
            &claude.join("b.md"),
            "---\nname: b\ndescription: b\ntools: Write\nmodel: sonnet\n---\nBody B\n",
        );
        write(&claude.join("README.md"), "skip me\n");
        let r = convert_all_agents(dir.path(), false).unwrap();
        assert_eq!(r.converted, 2);
        assert_eq!(r.failed, 0);
        assert!(dir.path().join(".opencode/agents/a.md").exists());
        assert!(dir.path().join(".opencode/agents/b.md").exists());
        assert!(!dir.path().join(".opencode/agents/README.md").exists());
    }

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
        let in_tools = vec![String::new(), "  ".to_string(), "Read".to_string()];
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
