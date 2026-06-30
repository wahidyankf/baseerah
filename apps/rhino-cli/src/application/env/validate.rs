//! `env validate` — code↔config drift detection for `env-contract.yaml` surfaces.
//!
//! Two validator surfaces ship active (`app` kind); `terraform` and `ansible` branches
//! are commented forward-scaffold (`// activate when IaC is added`).
//!
//! # ENV-VALIDATE CONFIG: `env-contract.yaml` at repo root, parsed with `serde_norway`.
//! Each surface entry carries `root`, `kind`, `lang`, and `allowlist`.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error};
use regex::Regex;
use serde::Deserialize;
use walkdir::WalkDir;

/// A single env-validate surface entry from `env-contract.yaml`.
#[derive(Debug, Deserialize, Clone)]
pub struct SurfaceConfig {
    /// Path relative to repo root (e.g. `apps/organiclever-be`).
    pub root: String,
    /// Surface kind: `"app"` (active); `"terraform"` / `"ansible"` are forward-scaffold.
    pub kind: String,
    /// Source language for the app validator: `"rust"` or `"typescript"`.
    #[serde(default)]
    pub lang: String,
    /// Keys intentionally exempt from drift detection (framework-injected, test-only, etc.).
    #[serde(default)]
    pub allowlist: Vec<String>,
}

/// Top-level `env-contract.yaml` structure.
#[derive(Debug, Deserialize)]
pub struct Contract {
    /// Ordered list of surfaces to validate.
    pub surfaces: Vec<SurfaceConfig>,
}

/// A single drift finding produced by the validator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Repo-relative surface root.
    pub root: PathBuf,
    /// Drift direction.
    pub drift: DriftKind,
    /// The divergent env-var key name.
    pub key: String,
}

/// Drift direction for a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftKind {
    /// Key present in `.env.example` but not consumed by any code in the surface.
    DeclaredButUnread,
    /// Key consumed by code but absent from `.env.example`.
    ReadButUndeclared,
}

impl DriftKind {
    /// Human-readable label for display.
    pub fn label(self) -> &'static str {
        match self {
            Self::DeclaredButUnread => "declared-but-unread",
            Self::ReadButUndeclared => "read-but-undeclared",
        }
    }
}

/// Load and parse the `env-contract:` section from `repo-config.yml` at `repo_root`.
///
/// # Errors
///
/// Returns an error when `repo-config.yml` cannot be read, is not valid YAML,
/// or the `env-contract:` section is absent.
pub fn load_contract(repo_root: &Path) -> Result<Contract, Error> {
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(rename = "env-contract")]
        env_contract: Option<Contract>,
    }

    let path = repo_root.join("repo-config.yml");
    let data = fs::read_to_string(&path)
        .with_context(|| format!("cannot read repo-config.yml at {}", path.display()))?;
    let wrapper: Wrapper = serde_norway::from_str(&data)
        .with_context(|| format!("failed to parse repo-config.yml at {}", path.display()))?;
    wrapper.env_contract.ok_or_else(|| {
        anyhow::anyhow!(
            "env-contract: section missing from repo-config.yml at {}",
            path.display()
        )
    })
}

/// Parse declared keys from a `.env.example` file.
///
/// Both active lines (`KEY=…`) and commented-out var lines (`# KEY=`) count as declared.
/// Pure annotation comments (no `KEY=` pattern) and blank lines are ignored.
///
/// # Errors
///
/// Returns an error when the file cannot be read.
pub fn parse_declared_keys(env_example: &Path) -> Result<Vec<String>, Error> {
    let content = fs::read_to_string(env_example)
        .with_context(|| format!("cannot read {}", env_example.display()))?;
    let mut keys = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let effective = if let Some(rest) = trimmed.strip_prefix('#') {
            rest.trim()
        } else {
            trimmed
        };
        if effective.is_empty() {
            continue;
        }
        if let Some(eq_pos) = effective.find('=') {
            let key = effective[..eq_pos].trim();
            if is_env_var_name(key) {
                keys.push(key.to_string());
            }
        }
    }
    Ok(keys)
}

/// Returns `true` if `s` is a valid env var name (uppercase letters, digits, underscores only).
fn is_env_var_name(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

/// Scan Rust source for environment variable keys consumed by the code.
///
/// Detects:
/// - `envy`-derived struct field names in `#[derive(Deserialize)]` structs
///   (`field_name` → `FIELD_NAME`).
/// - `std::env::var("KEY")` / `env::var("KEY")` string literals.
///
/// # Panics
///
/// Panics if the internal static regexes fail to compile (compile-time invariant).
///
/// # Errors
///
/// Returns an error when a source file cannot be read.
pub fn scan_rust_reads(root: &Path) -> Result<Vec<String>, Error> {
    let mut keys: HashSet<String> = HashSet::new();

    let env_var_re =
        Regex::new(r#"(?:std::)?env::var\s*\(\s*"([A-Z][A-Z0-9_]*)"\s*\)"#).expect("static regex");
    // Only scan the envy-mapped `Config` struct — not every Deserialize struct.
    let config_struct_re = Regex::new(r"^\s*pub\s+struct\s+Config\b").expect("static regex");
    let pub_field_re = Regex::new(r"^\s+pub\s+([a-z][a-z0-9_]*)\s*:").expect("static regex");

    let src_dir = root.join("src");
    for entry in WalkDir::new(&src_dir).into_iter().flatten() {
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        if !path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().ends_with(".rs"))
        {
            continue;
        }
        let content =
            fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;

        for cap in env_var_re.captures_iter(&content) {
            keys.insert(cap[1].to_string());
        }

        let mut in_config_struct = false;
        let mut brace_depth: i32 = 0;
        for line in content.lines() {
            if !in_config_struct && config_struct_re.is_match(line) {
                in_config_struct = true;
                brace_depth = 0;
            }
            if in_config_struct {
                for ch in line.chars() {
                    match ch {
                        '{' => brace_depth += 1,
                        '}' => {
                            brace_depth -= 1;
                            if brace_depth <= 0 {
                                in_config_struct = false;
                            }
                        }
                        _ => {}
                    }
                }
                if in_config_struct
                    && brace_depth > 0
                    && let Some(cap) = pub_field_re.captures(line)
                {
                    keys.insert(cap[1].to_ascii_uppercase());
                }
            }
        }
    }

    Ok(keys.into_iter().collect())
}

/// Scan TypeScript source for environment variable keys consumed by the code.
///
/// Detects:
/// - Keys declared in `createEnv` schema blocks in `env.ts` (`UPPER_CASE_KEY:` lines).
/// - `env.KEY` property accesses in any `.ts` / `.tsx` file.
///
/// # Panics
///
/// Panics if the internal static regexes fail to compile (compile-time invariant).
///
/// # Errors
///
/// Returns an error when a source file cannot be read.
pub fn scan_ts_reads(root: &Path) -> Result<Vec<String>, Error> {
    let mut keys: HashSet<String> = HashSet::new();

    let env_prop_re = Regex::new(r"\benv\.([A-Z][A-Z0-9_]+)\b").expect("static regex");
    let schema_key_re = Regex::new(r"^\s+([A-Z][A-Z0-9_]+)\s*:").expect("static regex");

    let src_dir = root.join("src");
    for entry in WalkDir::new(&src_dir).into_iter().flatten() {
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if !name.ends_with(".ts") && !name.ends_with(".tsx") {
            continue;
        }
        // Skip test files — they set process.env for mocking, not for production reads.
        if name.contains(".test.") || name.contains(".spec.") {
            continue;
        }
        let content =
            fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;

        for cap in env_prop_re.captures_iter(&content) {
            keys.insert(cap[1].to_string());
        }

        if name == "env.ts" {
            for line in content.lines() {
                if let Some(cap) = schema_key_re.captures(line) {
                    keys.insert(cap[1].to_string());
                }
            }
        }
    }

    Ok(keys.into_iter().collect())
}

/// Scan F# source for environment variable keys consumed by the code.
///
/// Detects:
/// - `Environment.GetEnvironmentVariable("VAR_NAME")` calls
/// - `System.Environment.GetEnvironmentVariable("VAR_NAME")` calls
///
/// # Panics
///
/// Panics if the internal static regexes fail to compile (compile-time invariant).
///
/// # Errors
///
/// Returns an error when a source file cannot be read.
pub fn scan_fsharp_reads(root: &Path) -> Result<Vec<String>, Error> {
    let mut keys: HashSet<String> = HashSet::new();

    let env_var_re = Regex::new(
        r#"(?:System\.)?Environment\.GetEnvironmentVariable\s*\(\s*"([A-Z][A-Z0-9_]*)"\s*\)"#,
    )
    .expect("static regex");

    let src_dir = root.join("src");
    for entry in WalkDir::new(&src_dir).into_iter().flatten() {
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        if !path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().ends_with(".fs"))
        {
            continue;
        }
        let content =
            fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
        for cap in env_var_re.captures_iter(&content) {
            keys.insert(cap[1].to_string());
        }
    }

    Ok(keys.into_iter().collect())
}

/// Validate a single `app`-kind surface against its `.env.example`.
///
/// Returns zero or more drift findings.
///
/// # Errors
///
/// Returns an error when source files cannot be read or the lang is unsupported.
pub fn validate_app_surface(
    repo_root: &Path,
    surface: &SurfaceConfig,
) -> Result<Vec<Finding>, Error> {
    let root = repo_root.join(&surface.root);
    let env_example = root.join(".env.example");

    let declared: HashSet<String> = parse_declared_keys(&env_example)?.into_iter().collect();

    let read: HashSet<String> = match surface.lang.as_str() {
        "rust" => scan_rust_reads(&root)?.into_iter().collect(),
        "typescript" => scan_ts_reads(&root)?.into_iter().collect(),
        "fsharp" => scan_fsharp_reads(&root)?.into_iter().collect(),
        other => return Err(anyhow::anyhow!("unsupported lang: {other}")),
    };

    let allowlist: HashSet<&str> = surface.allowlist.iter().map(String::as_str).collect();

    let mut findings = Vec::new();

    for key in &declared {
        if !read.contains(key) && !allowlist.contains(key.as_str()) {
            findings.push(Finding {
                root: PathBuf::from(&surface.root),
                drift: DriftKind::DeclaredButUnread,
                key: key.clone(),
            });
        }
    }

    for key in &read {
        if !declared.contains(key) && !allowlist.contains(key.as_str()) {
            findings.push(Finding {
                root: PathBuf::from(&surface.root),
                drift: DriftKind::ReadButUndeclared,
                key: key.clone(),
            });
        }
    }

    findings.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(findings)
}

/// Validate all surfaces declared in `contract`.
///
/// Unknown surface kinds are logged to stderr but do not cause an error.
///
/// # Errors
///
/// Returns an error when any surface validation fails.
pub fn validate_all(repo_root: &Path, contract: &Contract) -> Result<Vec<Finding>, Error> {
    let mut all = Vec::new();
    for surface in &contract.surfaces {
        match surface.kind.as_str() {
            "app" => {
                all.extend(validate_app_surface(repo_root, surface)?);
            }
            // activate when IaC is added
            // "terraform" => { /* diff tfvars.example keys vs variable blocks */ }
            // "ansible" => { /* diff env.example keys vs playbook lookup calls */ }
            other => {
                eprintln!(
                    "env validate: unknown surface kind '{}' for {} — skipped",
                    other, surface.root
                );
            }
        }
    }
    Ok(all)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &TempDir, rel: &str, content: &str) {
        let p = dir.path().join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
    }

    // ── parse_declared_keys ──────────────────────────────────────────────────

    #[test]
    fn parse_active_key() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp,
            ".env.example",
            "DATABASE_URL=postgres://localhost/db\n",
        );
        let keys = parse_declared_keys(&tmp.path().join(".env.example")).unwrap();
        assert_eq!(keys, vec!["DATABASE_URL"]);
    }

    #[test]
    fn parse_commented_optional_key() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp,
            ".env.example",
            "# OPTIONAL | string | desc\n# MY_KEY=\n",
        );
        let keys = parse_declared_keys(&tmp.path().join(".env.example")).unwrap();
        assert_eq!(keys, vec!["MY_KEY"]);
    }

    #[test]
    fn parse_skips_pure_comment() {
        let tmp = TempDir::new().unwrap();
        write(&tmp, ".env.example", "# This is a pure comment\n");
        let keys = parse_declared_keys(&tmp.path().join(".env.example")).unwrap();
        assert!(keys.is_empty());
    }

    #[test]
    fn parse_skips_lowercase_key() {
        let tmp = TempDir::new().unwrap();
        write(&tmp, ".env.example", "not_env_var=value\n");
        let keys = parse_declared_keys(&tmp.path().join(".env.example")).unwrap();
        assert!(keys.is_empty());
    }

    // ── scan_rust_reads ──────────────────────────────────────────────────────

    #[test]
    fn scan_rust_finds_envy_struct_fields() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp,
            "src/config.rs",
            r"use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub app_port: u16,
}
",
        );
        let mut keys = scan_rust_reads(tmp.path()).unwrap();
        keys.sort();
        assert_eq!(keys, vec!["APP_PORT", "DATABASE_URL"]);
    }

    #[test]
    fn scan_rust_finds_env_var_literals() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp,
            "src/main.rs",
            r#"fn main() { let v = std::env::var("SOME_KEY").unwrap(); }
"#,
        );
        let keys = scan_rust_reads(tmp.path()).unwrap();
        assert!(
            keys.contains(&"SOME_KEY".to_string()),
            "expected SOME_KEY; got {keys:?}"
        );
    }

    // ── scan_ts_reads ────────────────────────────────────────────────────────

    #[test]
    fn scan_ts_finds_create_env_schema_keys() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp,
            "src/env.ts",
            r#"import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";
export const env = createEnv({
  server: {
    MY_SERVER_KEY: z.string().optional(),
    ANOTHER_KEY: z.string(),
  },
  experimental__runtimeEnv: {},
});
"#,
        );
        let mut keys = scan_ts_reads(tmp.path()).unwrap();
        keys.sort();
        assert!(keys.contains(&"MY_SERVER_KEY".to_string()), "got {keys:?}");
        assert!(keys.contains(&"ANOTHER_KEY".to_string()), "got {keys:?}");
    }

    #[test]
    fn scan_ts_finds_env_property_accesses() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp,
            "src/app/page.tsx",
            r#"import { env } from "@/env";
const url = env.ORGANICLEVER_BE_URL ?? "default";
"#,
        );
        let keys = scan_ts_reads(tmp.path()).unwrap();
        assert!(
            keys.contains(&"ORGANICLEVER_BE_URL".to_string()),
            "got {keys:?}"
        );
    }

    // ── scan_fsharp_reads ────────────────────────────────────────────────────

    #[test]
    fn scan_fsharp_finds_env_var_literals() {
        let dir = tempfile::tempdir().unwrap();
        let src_dir = dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(
            src_dir.join("Config.fs"),
            r#"module Config
let load () =
    let port = Environment.GetEnvironmentVariable("CRANE_BE_PORT")
    let url = System.Environment.GetEnvironmentVariable("CRANE_BE_NATS_URL")
    (port, url)
"#,
        )
        .unwrap();
        let keys = scan_fsharp_reads(dir.path()).unwrap();
        assert!(
            keys.contains(&"CRANE_BE_PORT".to_string()),
            "expected CRANE_BE_PORT; got {keys:?}"
        );
        assert!(
            keys.contains(&"CRANE_BE_NATS_URL".to_string()),
            "expected CRANE_BE_NATS_URL; got {keys:?}"
        );
    }

    // ── validate_app_surface ────────────────────────────────────────────────

    #[test]
    fn validate_matching_app_returns_no_findings() {
        let tmp = TempDir::new().unwrap();
        write(&tmp, "apps/myapp/.env.example", "MY_KEY=value\n");
        write(
            &tmp,
            "apps/myapp/src/env.ts",
            "export const env = createEnv({\n  server: {\n    MY_KEY: z.string(),\n  },\n  experimental__runtimeEnv: {},\n});\n",
        );
        let surface = SurfaceConfig {
            root: "apps/myapp".to_string(),
            kind: "app".to_string(),
            lang: "typescript".to_string(),
            allowlist: vec![],
        };
        let findings = validate_app_surface(tmp.path(), &surface).unwrap();
        assert!(
            findings.is_empty(),
            "expected no findings; got {findings:?}"
        );
    }

    #[test]
    fn validate_declared_but_unread_key_is_reported() {
        let tmp = TempDir::new().unwrap();
        write(&tmp, "apps/myapp/.env.example", "STALE_KEY=value\n");
        write(
            &tmp,
            "apps/myapp/src/env.ts",
            r"export const env = createEnv({ server: {}, experimental__runtimeEnv: {} });
",
        );
        let surface = SurfaceConfig {
            root: "apps/myapp".to_string(),
            kind: "app".to_string(),
            lang: "typescript".to_string(),
            allowlist: vec![],
        };
        let findings = validate_app_surface(tmp.path(), &surface).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].drift, DriftKind::DeclaredButUnread);
        assert_eq!(findings[0].key, "STALE_KEY");
    }

    #[test]
    fn validate_read_but_undeclared_key_is_reported() {
        let tmp = TempDir::new().unwrap();
        write(&tmp, "apps/myapp/.env.example", "");
        write(
            &tmp,
            "apps/myapp/src/env.ts",
            "export const env = createEnv({\n  server: {\n    NEW_KEY: z.string(),\n  },\n  experimental__runtimeEnv: {},\n});\n",
        );
        let surface = SurfaceConfig {
            root: "apps/myapp".to_string(),
            kind: "app".to_string(),
            lang: "typescript".to_string(),
            allowlist: vec![],
        };
        let findings = validate_app_surface(tmp.path(), &surface).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].drift, DriftKind::ReadButUndeclared);
        assert_eq!(findings[0].key, "NEW_KEY");
    }

    #[test]
    fn validate_allowlisted_key_is_not_reported() {
        let tmp = TempDir::new().unwrap();
        write(&tmp, "apps/myapp/.env.example", "# PORT=3100\n");
        write(
            &tmp,
            "apps/myapp/src/env.ts",
            r"export const env = createEnv({ server: {}, experimental__runtimeEnv: {} });
",
        );
        let surface = SurfaceConfig {
            root: "apps/myapp".to_string(),
            kind: "app".to_string(),
            lang: "typescript".to_string(),
            allowlist: vec!["PORT".to_string()],
        };
        let findings = validate_app_surface(tmp.path(), &surface).unwrap();
        assert!(
            findings.is_empty(),
            "expected no findings; got {findings:?}"
        );
    }

    #[test]
    fn validate_rust_matching_config_returns_no_findings() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp,
            "apps/myapp/.env.example",
            "DATABASE_URL=postgres://localhost/db\nAPP_PORT=8080\n",
        );
        write(
            &tmp,
            "apps/myapp/src/config.rs",
            r"use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub app_port: u16,
}
",
        );
        let surface = SurfaceConfig {
            root: "apps/myapp".to_string(),
            kind: "app".to_string(),
            lang: "rust".to_string(),
            allowlist: vec![],
        };
        let findings = validate_app_surface(tmp.path(), &surface).unwrap();
        assert!(
            findings.is_empty(),
            "expected no findings; got {findings:?}"
        );
    }

    // ── load_contract from repo-config.yml (RED → GREEN) ─────────────────────

    #[test]
    fn load_contract_reads_env_contract_section_from_repo_config_yml() {
        let tmp = TempDir::new().unwrap();
        let yaml = concat!(
            "harness: []\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
            "env-contract:\n",
            "  surfaces:\n",
            "    - root: apps/myapp\n",
            "      kind: app\n",
            "      lang: typescript\n",
            "      allowlist: []\n",
        );
        fs::write(tmp.path().join("repo-config.yml"), yaml).unwrap();
        // NO standalone env-contract.yaml — loader must read from repo-config.yml
        let result = load_contract(tmp.path());
        assert!(
            result.is_ok(),
            "should read env-contract: from repo-config.yml without standalone file: {result:?}"
        );
        let contract = result.unwrap();
        assert_eq!(contract.surfaces.len(), 1);
        assert_eq!(contract.surfaces[0].root, "apps/myapp");
    }

    #[test]
    fn load_contract_errors_when_env_contract_section_missing_from_repo_config_yml() {
        let tmp = TempDir::new().unwrap();
        let yaml = concat!(
            "harness: []\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
        );
        fs::write(tmp.path().join("repo-config.yml"), yaml).unwrap();
        // NO standalone env-contract.yaml, NO env-contract: section in repo-config.yml
        let result = load_contract(tmp.path());
        assert!(
            result.is_err(),
            "should error when env-contract: section is absent from repo-config.yml"
        );
    }
}
