// Port of `apps/rhino-cli/internal/doctor/checker.go`.

#![allow(
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_split_once,
    clippy::needless_splitn,
    clippy::trim_split_whitespace
)]

use std::path::Path;
use std::process::Command;
use std::time::Instant;

use super::tools::{build_tool_defs, ToolDef};
use super::{
    is_minimal_tool, CheckOptions, CommandOutput, CommandRunner, DoctorResult, Scope, ToolCheck,
    ToolStatus,
};

/// Strip a leading "v" from a version string.
pub(super) fn normalize_simple_version(s: &str) -> String {
    s.strip_prefix('v').unwrap_or(s).to_string()
}

/// Trim whitespace then strip leading "v".
pub(super) fn parse_trim_version(s: &str) -> String {
    normalize_simple_version(s.trim())
}

/// Return the `word_idx`-th space-separated token from the first line that
/// starts with `line_prefix` (after trimming whitespace). If `token_prefix`
/// is non-empty, it is stripped from the matched token.
pub(super) fn parse_line_word(
    output: &str,
    line_prefix: &str,
    word_idx: usize,
    token_prefix: &str,
) -> String {
    for line in output.split('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with(line_prefix) {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if word_idx < parts.len() {
                let p = parts[word_idx];
                return p.strip_prefix(token_prefix).unwrap_or(p).to_string();
            }
        }
    }
    String::new()
}

// --- Version readers ---

pub(super) fn read_node_version(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let v: serde_json::Value = serde_json::from_slice(&data).ok()?;
    v.get("volta")?.get("node")?.as_str().map(|s| s.to_string())
}

pub(super) fn read_npm_version(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let v: serde_json::Value = serde_json::from_slice(&data).ok()?;
    v.get("volta")?.get("npm")?.as_str().map(|s| s.to_string())
}

pub(super) fn read_java_version(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    // Minimal XML scan for <java.version>X</java.version> under <properties>.
    let needle_open = "<java.version>";
    let needle_close = "</java.version>";
    let s = data.find(needle_open)?;
    let e = data[s + needle_open.len()..].find(needle_close)?;
    Some(data[s + needle_open.len()..s + needle_open.len() + e].to_string())
}

pub(super) fn read_go_version(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    for line in data.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("go ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if !parts.is_empty() {
                return Some(parts[0].to_string());
            }
        }
    }
    None
}

pub(super) fn read_python_version(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    Some(data.trim().to_string())
}

pub(super) fn read_tool_versions_entry(path: &Path, tool: &str) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == tool {
            return Some(parts[1].to_string());
        }
    }
    None
}

pub(super) fn read_dotnet_version(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let v: serde_json::Value = serde_json::from_slice(&data).ok()?;
    v.get("sdk")?
        .get("version")?
        .as_str()
        .map(|s| s.to_string())
}

pub(super) fn read_dart_sdk_version(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    let mut in_env = false;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed == "environment:" {
            in_env = true;
            continue;
        }
        if in_env {
            if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("sdk:") {
                let mut v = rest.trim().to_string();
                v = v.trim_matches('"').to_string();
                v = v.strip_prefix('^').unwrap_or(&v).to_string();
                v = v.strip_prefix(">=").unwrap_or(&v).to_string();
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

pub(super) fn read_rust_version(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    for line in data.lines() {
        let t = line.trim();
        if t.starts_with("rust-version") {
            if let Some((_, rhs)) = t.split_once('=') {
                let mut v = rhs.trim().to_string();
                v = v.trim_matches('"').to_string();
                return Some(v);
            }
        }
    }
    None
}

pub(super) fn read_flutter_version(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    let mut in_env = false;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed == "environment:" {
            in_env = true;
            continue;
        }
        if in_env {
            if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("flutter:") {
                let mut v = rest.trim().to_string();
                v = v.trim_matches('"').to_string();
                v = v.strip_prefix('^').unwrap_or(&v).to_string();
                v = v.strip_prefix(">=").unwrap_or(&v).to_string();
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

// --- Parsers for tool output ---

pub(super) fn parse_java_version(stderr: &str) -> String {
    for line in stderr.split('\n') {
        if line.contains("version") {
            let start = line.find('"');
            let end = line.rfind('"');
            if let (Some(s), Some(e)) = (start, end) {
                if s != e {
                    let version = &line[s + 1..e];
                    let parts: Vec<&str> = version.split('.').collect();
                    if let Some(first) = parts.first() {
                        if !first.is_empty() {
                            if *first == "1" && parts.len() > 1 {
                                return parts[1].to_string();
                            }
                            return first.to_string();
                        }
                    }
                }
            }
        }
    }
    String::new()
}

pub(super) fn parse_python_version(out: &str) -> String {
    parse_line_word(out, "Python ", 1, "")
}

pub(super) fn parse_rust_version(out: &str) -> String {
    parse_line_word(out, "rustc ", 1, "")
}

pub(super) fn parse_cargo_llvm_cov(out: &str) -> String {
    parse_line_word(out, "cargo-llvm-cov ", 1, "")
}

pub(super) fn parse_elixir_version(out: &str) -> String {
    parse_line_word(out, "Elixir ", 1, "")
}

pub(super) fn parse_erlang_version(out: &str) -> String {
    out.trim().to_string()
}

pub(super) fn parse_dotnet_version(out: &str) -> String {
    out.trim().to_string()
}

pub(super) fn parse_clojure_version(out: &str) -> String {
    parse_line_word(out, "Clojure CLI version ", 3, "")
}

pub(super) fn parse_dart_version(out: &str) -> String {
    for line in out.split('\n') {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("Dart SDK version:") {
            let fields: Vec<&str> = rest.trim().split_whitespace().collect();
            if let Some(first) = fields.first() {
                return first.to_string();
            }
        }
    }
    String::new()
}

pub(super) fn parse_flutter_version(out: &str) -> String {
    parse_line_word(out, "Flutter ", 1, "")
}

pub(super) fn parse_docker_version(out: &str) -> String {
    for line in out.split('\n') {
        let t = line.trim();
        if t.starts_with("Docker version") {
            let fields: Vec<&str> = t.split_whitespace().collect();
            if fields.len() >= 3 {
                return fields[2].trim_end_matches(',').to_string();
            }
        }
    }
    String::new()
}

pub(super) fn parse_golangci_lint_version(out: &str) -> String {
    parse_line_word(out, "golangci-lint", 3, "")
}

pub(super) fn parse_jq_version(out: &str) -> String {
    out.trim()
        .strip_prefix("jq-")
        .unwrap_or(out.trim())
        .to_string()
}

pub(super) fn parse_playwright_version(out: &str) -> String {
    parse_line_word(out, "Version ", 1, "")
}

// --- Comparators ---

pub(super) fn compare_exact(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".into());
    }
    let inst = normalize_simple_version(installed);
    let req = normalize_simple_version(required);
    if inst == req {
        (ToolStatus::Ok, format!("required: {required}"))
    } else {
        (
            ToolStatus::Warning,
            format!("required: {required}, version mismatch"),
        )
    }
}

pub(super) fn compare_major(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".into());
    }
    let inst = normalize_simple_version(installed);
    let req = normalize_simple_version(required);
    let inst_major = inst.splitn(2, '.').next().unwrap_or("");
    let req_major = req.splitn(2, '.').next().unwrap_or("");
    if !inst_major.is_empty() && inst_major == req_major {
        (ToolStatus::Ok, format!("required: {required}"))
    } else {
        (
            ToolStatus::Warning,
            format!("required: {required}, version mismatch"),
        )
    }
}

pub(super) fn parse_version_parts(s: &str) -> Option<(i64, i64, i64)> {
    let s = normalize_simple_version(s);
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    let mut nums = [0i64; 3];
    for (i, p) in parts.iter().enumerate() {
        let n: i64 = p.parse().ok()?;
        nums[i] = n;
    }
    Some((nums[0], nums[1], nums[2]))
}

pub(super) fn compare_major_gte(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".into());
    }
    let inst = normalize_simple_version(installed);
    let req = normalize_simple_version(required);
    let i_major = inst.splitn(2, '.').next().unwrap_or("");
    let r_major = req.splitn(2, '.').next().unwrap_or("");
    let (i_maj, r_maj): (i64, i64) = match (i_major.parse(), r_major.parse()) {
        (Ok(a), Ok(b)) => (a, b),
        _ => return compare_exact(installed, required),
    };
    if i_maj >= r_maj {
        (
            ToolStatus::Ok,
            format!("required: \u{2265}{required} (major)"),
        )
    } else {
        (
            ToolStatus::Warning,
            format!("required: \u{2265}{required} (major), version too old"),
        )
    }
}

pub(super) fn compare_gte(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".into());
    }
    let i = parse_version_parts(installed);
    let r = parse_version_parts(required);
    let ((i_maj, i_min, i_pat), (r_maj, r_min, r_pat)) = match (i, r) {
        (Some(a), Some(b)) => (a, b),
        _ => return compare_exact(installed, required),
    };
    if i_maj > r_maj
        || (i_maj == r_maj && i_min > r_min)
        || (i_maj == r_maj && i_min == r_min && i_pat >= r_pat)
    {
        (ToolStatus::Ok, format!("required: \u{2265}{required}"))
    } else {
        (
            ToolStatus::Warning,
            format!("required: \u{2265}{required}, version too old"),
        )
    }
}

// --- Playwright browser detection ---

pub(super) fn check_playwright_browsers() -> bool {
    let Some(home) = dirs_home() else {
        return false;
    };
    let cache_dir = if cfg!(target_os = "macos") {
        home.join("Library").join("Caches").join("ms-playwright")
    } else {
        home.join(".cache").join("ms-playwright")
    };
    let Ok(entries) = std::fs::read_dir(&cache_dir) else {
        return false;
    };
    for e in entries.flatten() {
        if let Some(name) = e.file_name().to_str() {
            if name.starts_with("chromium-") {
                return true;
            }
        }
    }
    false
}

fn dirs_home() -> Option<std::path::PathBuf> {
    std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
}

pub(super) fn compare_playwright(_installed: &str, _required: &str) -> (ToolStatus, String) {
    if !check_playwright_browsers() {
        return (
            ToolStatus::Warning,
            "browsers not installed \u{2014} run: npx playwright install".into(),
        );
    }
    (ToolStatus::Ok, "no version requirement".into())
}

// --- Runner ---

pub fn real_runner(name: &str, args: &[&str]) -> CommandOutput {
    if !binary_in_path(name) {
        return Err(format!("binary not found in PATH: {name}"));
    }
    let out = Command::new(name)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let code = out.status.code().unwrap_or(-1);
    Ok((stdout, stderr, code))
}

/// Mirror of Go's exec.LookPath. Walk $PATH for an executable file named `name`.
fn binary_in_path(name: &str) -> bool {
    if name.contains('/') {
        return std::fs::metadata(name)
            .map(|m| m.is_file())
            .unwrap_or(false);
    }
    let Some(path_var) = std::env::var_os("PATH") else {
        return false;
    };
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return true;
        }
    }
    false
}

/// Execute one tool check.
pub(super) fn run_one_def(runner: CommandRunner<'_>, def: &ToolDef) -> ToolCheck {
    let required_version = (def.read_req)();
    let args_strs: Vec<&str> = def.args.iter().map(|s| s.as_str()).collect();
    match runner(&def.binary, &args_strs) {
        Err(_) => ToolCheck {
            name: def.name.clone(),
            binary: def.binary.clone(),
            status: ToolStatus::Missing,
            installed_version: String::new(),
            required_version,
            source: def.source.clone(),
            note: "not found in PATH".into(),
        },
        Ok((stdout, stderr, _code)) => {
            let output = if def.use_stderr { &stderr } else { &stdout };
            let installed = (def.parse_ver)(output);
            let (status, note) = (def.compare)(&installed, &required_version);
            ToolCheck {
                name: def.name.clone(),
                binary: def.binary.clone(),
                status,
                installed_version: installed,
                required_version,
                source: def.source.clone(),
                note,
            }
        }
    }
}

/// Run all tool checks.
pub fn check_all(opts: &CheckOptions<'_>) -> DoctorResult {
    let start = Instant::now();

    let runner: CommandRunner<'_> = opts.runner.unwrap_or(&real_runner);

    let mut defs = build_tool_defs(&opts.repo_root);

    if opts.scope == Scope::Minimal {
        defs.retain(|d| is_minimal_tool(&d.name));
    }

    let mut checks = Vec::with_capacity(defs.len());
    for def in &defs {
        checks.push(run_one_def(runner, def));
    }

    let mut ok = 0usize;
    let mut warn = 0usize;
    let mut missing = 0usize;
    for c in &checks {
        match c.status {
            ToolStatus::Ok => ok += 1,
            ToolStatus::Warning => warn += 1,
            ToolStatus::Missing => missing += 1,
        }
    }

    DoctorResult {
        checks,
        ok_count: ok,
        warn_count: warn,
        missing_count: missing,
        duration: start.elapsed(),
        scope: opts.scope,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_exact_no_req() {
        assert_eq!(compare_exact("1.0", "").0, ToolStatus::Ok);
    }

    #[test]
    fn compare_exact_match() {
        let (s, n) = compare_exact("1.2.3", "1.2.3");
        assert_eq!(s, ToolStatus::Ok);
        assert!(n.contains("required: 1.2.3"));
    }

    #[test]
    fn compare_exact_mismatch() {
        let (s, _) = compare_exact("1.2.3", "1.2.4");
        assert_eq!(s, ToolStatus::Warning);
    }

    #[test]
    fn compare_exact_strips_v() {
        let (s, _) = compare_exact("v1.2.3", "1.2.3");
        assert_eq!(s, ToolStatus::Ok);
    }

    #[test]
    fn compare_major_match() {
        let (s, _) = compare_major("21.0.1", "21");
        assert_eq!(s, ToolStatus::Ok);
    }

    #[test]
    fn compare_major_mismatch() {
        let (s, _) = compare_major("17", "21");
        assert_eq!(s, ToolStatus::Warning);
    }

    #[test]
    fn compare_gte_higher_ok() {
        let (s, _) = compare_gte("1.25.0", "1.24.0");
        assert_eq!(s, ToolStatus::Ok);
    }

    #[test]
    fn compare_gte_equal_ok() {
        let (s, _) = compare_gte("1.24.0", "1.24.0");
        assert_eq!(s, ToolStatus::Ok);
    }

    #[test]
    fn compare_gte_lower_warn() {
        let (s, _) = compare_gte("1.22.0", "1.24.0");
        assert_eq!(s, ToolStatus::Warning);
    }

    #[test]
    fn compare_gte_falls_back_on_parse_fail() {
        let (s, _) = compare_gte("abc", "1.24.0");
        // Non-numeric installed: falls back to exact comparison → mismatch.
        assert_eq!(s, ToolStatus::Warning);
    }

    #[test]
    fn compare_major_gte_higher_ok() {
        let (s, _) = compare_major_gte("28", "27");
        assert_eq!(s, ToolStatus::Ok);
    }

    #[test]
    fn compare_major_gte_lower_warn() {
        let (s, _) = compare_major_gte("26", "27");
        assert_eq!(s, ToolStatus::Warning);
    }

    #[test]
    fn parse_line_word_match() {
        let s = parse_line_word("git version 2.42.0", "git version ", 2, "");
        assert_eq!(s, "2.42.0");
    }

    #[test]
    fn parse_line_word_strips_prefix() {
        let s = parse_line_word("go version go1.25.0 darwin", "go version ", 2, "go");
        assert_eq!(s, "1.25.0");
    }

    #[test]
    fn parse_trim_strips_v() {
        assert_eq!(parse_trim_version("  v24.11.1\n"), "24.11.1");
    }

    #[test]
    fn parse_java_old_style() {
        let stderr = "openjdk version \"1.8.0_292\"";
        assert_eq!(parse_java_version(stderr), "8");
    }

    #[test]
    fn parse_java_new_style() {
        let stderr = "openjdk version \"21.0.1\"";
        assert_eq!(parse_java_version(stderr), "21");
    }

    #[test]
    fn parse_docker_strips_comma() {
        assert_eq!(
            parse_docker_version("Docker version 29.2.1, build abc"),
            "29.2.1"
        );
    }

    #[test]
    fn parse_jq_strips_prefix() {
        assert_eq!(parse_jq_version("jq-1.8.1"), "1.8.1");
    }

    #[test]
    fn parse_dart_version_first_field() {
        assert_eq!(
            parse_dart_version("Dart SDK version: 3.11.3 (stable) on host"),
            "3.11.3"
        );
    }

    #[test]
    fn parse_clojure_version_word_index() {
        assert_eq!(
            parse_clojure_version("Clojure CLI version 1.12.4.1582"),
            "1.12.4.1582"
        );
    }

    #[test]
    fn parse_playwright_word() {
        assert_eq!(parse_playwright_version("Version 1.58.2"), "1.58.2");
    }

    #[test]
    fn read_go_version_finds_directive() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("go.mod");
        std::fs::write(&p, "module x\n\ngo 1.25.0\n").unwrap();
        assert_eq!(read_go_version(&p).as_deref(), Some("1.25.0"));
    }

    #[test]
    fn read_node_version_reads_volta_node() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("package.json");
        std::fs::write(&p, r#"{"volta":{"node":"24.11.1","npm":"10.9.0"}}"#).unwrap();
        assert_eq!(read_node_version(&p).as_deref(), Some("24.11.1"));
        assert_eq!(read_npm_version(&p).as_deref(), Some("10.9.0"));
    }

    #[test]
    fn read_tool_versions_entry_finds_value() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join(".tool-versions");
        std::fs::write(&p, "elixir 1.19.5-otp-27\nerlang 27.3.4\n").unwrap();
        assert_eq!(
            read_tool_versions_entry(&p, "elixir").as_deref(),
            Some("1.19.5-otp-27")
        );
        assert_eq!(
            read_tool_versions_entry(&p, "erlang").as_deref(),
            Some("27.3.4")
        );
    }

    #[test]
    fn read_dart_sdk_version_with_caret() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("pubspec.yaml");
        std::fs::write(
            &p,
            "name: x\nenvironment:\n  sdk: ^3.11.3\n  flutter: ^3.41.5\n",
        )
        .unwrap();
        assert_eq!(read_dart_sdk_version(&p).as_deref(), Some("3.11.3"));
        assert_eq!(read_flutter_version(&p).as_deref(), Some("3.41.5"));
    }

    #[test]
    fn read_rust_version_from_cargo() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("Cargo.toml");
        std::fs::write(&p, "[package]\nname = \"x\"\nrust-version = \"1.88\"\n").unwrap();
        assert_eq!(read_rust_version(&p).as_deref(), Some("1.88"));
    }

    #[test]
    fn read_dotnet_version_from_global_json() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("global.json");
        std::fs::write(&p, r#"{"sdk":{"version":"8.0.401"}}"#).unwrap();
        assert_eq!(read_dotnet_version(&p).as_deref(), Some("8.0.401"));
    }

    #[test]
    fn run_one_def_missing() {
        let def = ToolDef {
            name: "ghosttool".into(),
            binary: "ghosttool-binary-that-does-not-exist".into(),
            source: "".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: |s| s.trim().to_string(),
            compare: compare_exact,
            read_req: || "".into(),
            install_cmd: None,
        };
        let runner: CommandRunner = &|_, _| Err("not found".into());
        let c = run_one_def(runner, &def);
        assert_eq!(c.status, ToolStatus::Missing);
        assert_eq!(c.note, "not found in PATH");
    }

    #[test]
    fn run_one_def_ok_with_fake_runner() {
        let def = ToolDef {
            name: "fake".into(),
            binary: "fake".into(),
            source: "".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: |s| s.trim().to_string(),
            compare: compare_exact,
            read_req: || "1.0.0".into(),
            install_cmd: None,
        };
        let runner: CommandRunner = &|_, _| Ok(("1.0.0\n".into(), String::new(), 0));
        let c = run_one_def(runner, &def);
        assert_eq!(c.status, ToolStatus::Ok);
        assert_eq!(c.installed_version, "1.0.0");
    }

    #[test]
    fn check_all_runs_and_aggregates() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("package.json"), "{}").unwrap();
        let runner: CommandRunner = &|name, _args| match name {
            "git" => Ok(("git version 2.42.0\n".into(), String::new(), 0)),
            "volta" => Ok(("2.0.2\n".into(), String::new(), 0)),
            _ => Err("not found".into()),
        };
        let opts = CheckOptions {
            repo_root: dir.path().to_path_buf(),
            runner: Some(runner),
            scope: Scope::Minimal,
        };
        let r = check_all(&opts);
        assert_eq!(r.checks.len(), 7);
        assert!(r.ok_count >= 2);
        assert!(r.missing_count >= 1);
    }
}
