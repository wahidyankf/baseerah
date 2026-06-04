//! Bounded-context registry loader and validator.
//!
//! Port of `apps/rhino-cli/internal/bcregistry/`.
//!
//! This module reads `specs/apps/<app>/ddd/bounded-contexts.yaml`, validates
//! its schema, and checks that every declared code directory, glossary file,
//! and Gherkin directory exists on the filesystem with the expected layer
//! structure.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Error, anyhow};
use serde_norway::Value;

use crate::internal::severity::Severity;

/// The only schema version this tool understands for `bounded-contexts.yaml`.
pub const SCHEMA_VERSION: i64 = 2;

/// In-memory representation of a parsed `bounded-contexts.yaml` registry.
#[derive(Debug, Clone, Default)]
pub struct Registry {
    /// Schema version declared in the YAML file.
    pub version: i64,
    /// Application identifier (e.g. `"organiclever"`).
    pub app: String,
    /// All bounded contexts declared in the registry.
    pub contexts: Vec<BcContext>,
}

/// A single bounded context entry within the registry.
#[derive(Debug, Clone, Default)]
pub struct BcContext {
    /// Unique name of this bounded context within the application.
    pub name: String,
    /// Short human-readable summary of the context's responsibility.
    pub summary: String,
    /// DDD layer directory names expected inside each code directory.
    pub layers: Vec<String>,
    /// Relative paths (from the repo root) to code directories for this context.
    pub code: Vec<String>,
    /// Language identifiers for source files in the code directories.
    pub code_lang: Vec<String>,
    /// Relative path to the glossary markdown file for this context.
    pub glossary: String,
    /// Relative paths (from the repo root) to Gherkin feature directories.
    pub gherkin: Vec<String>,
    /// Declared relationships to other bounded contexts.
    pub relationships: Vec<Relationship>,
}

/// A directional relationship between two bounded contexts.
#[derive(Debug, Clone, Default)]
pub struct Relationship {
    /// Name of the target bounded context.
    pub to: String,
    /// Relationship kind (e.g. `"customer-supplier"`, `"anticorruption-layer"`).
    pub kind: String,
    /// Role of the declaring context in this relationship.
    pub role: String,
}

/// A validation finding produced by the registry checker.
#[derive(Debug, Clone)]
pub struct Finding {
    /// File path (relative or absolute) where the finding originates.
    pub file: String,
    /// Human-readable description of the issue.
    pub message: String,
    /// Severity level of this finding.
    pub severity: Severity,
}

/// Options that control how [`validate_all`] runs.
#[derive(Debug, Clone, Default)]
pub struct ValidateOptions {
    /// Absolute path to the repository root.
    pub repo_root: std::path::PathBuf,
    /// Application identifier to validate (e.g. `"organiclever"`).
    pub app: String,
    /// Severity override; defaults to [`Severity::Error`] when `None`.
    pub severity: Option<Severity>,
}

/// Returns a map from language identifier to file-glob patterns for that language.
///
/// The map is initialised once and reused for all subsequent calls.
pub fn supported_lang_globs() -> &'static HashMap<&'static str, &'static [&'static str]> {
    static M: OnceLock<HashMap<&'static str, &'static [&'static str]>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("ts", &["*.ts"][..]);
        m.insert("tsx", &["*.tsx"][..]);
        m.insert("fs", &["*.fs"][..]);
        m.insert("go", &["*.go"][..]);
        m.insert("py", &["*.py"][..]);
        m.insert("java", &["*.java"][..]);
        m.insert("kt", &["*.kt"][..]);
        m.insert("rs", &["*.rs"][..]);
        m.insert("ex", &["*.ex", "*.exs"][..]);
        m.insert("exs", &["*.exs"][..]);
        m.insert("cs", &["*.cs"][..]);
        m.insert("clj", &["*.clj", "*.cljc"][..]);
        m.insert("dart", &["*.dart"][..]);
        m
    })
}

/// Returns an error if any element of `langs` is not a recognised language identifier.
///
/// # Errors
///
/// Returns an error when a language string is not present in [`supported_lang_globs`].
fn validate_code_lang(langs: &[String]) -> Result<(), Error> {
    let m = supported_lang_globs();
    for l in langs {
        if !m.contains_key(l.as_str()) {
            return Err(anyhow!(
                "unsupported code_lang \"{l}\" (supported: ts, tsx, fs, go, py, java, kt, rs, ex, exs, cs, clj, dart)"
            ));
        }
    }
    Ok(())
}

/// Returns whether `kind` is a symmetric relationship kind.
///
/// - `Some(true)` — the kind requires a reciprocal declaration by the target context.
/// - `Some(false)` — the kind is unidirectional and does not require a reciprocal.
/// - `None` — the kind is not recognised.
pub fn relationship_kind_is_asymmetric(kind: &str) -> Option<bool> {
    match kind {
        "customer-supplier" | "conformist" | "partnership" | "shared-kernel" => Some(true),
        "anticorruption-layer" | "open-host-service" => Some(false),
        _ => None,
    }
}

/// Loads and parses the bounded-context registry for `app` under `repo_root`.
///
/// The function reads `specs/apps/<app>/ddd/bounded-contexts.yaml`, validates
/// the schema version, and checks that every context has non-empty `code` and
/// `gherkin` lists.  Default `code_lang` values (`["ts", "tsx"]`) are applied
/// when a context omits the field.
///
/// # Errors
///
/// Returns an error when:
/// - The YAML file cannot be read or parsed.
/// - The declared schema version does not equal [`SCHEMA_VERSION`].
/// - Any context has an empty `code` or `gherkin` list.
/// - Any language in `code_lang` is not supported.
pub fn load(repo_root: &Path, app: &str) -> Result<Registry, Error> {
    let path = repo_root
        .join("specs")
        .join("apps")
        .join(app)
        .join("ddd")
        .join("bounded-contexts.yaml");
    let data = fs::read(&path)
        .with_context(|| format!("registry not found for app \"{app}\" at {}", path.display()))?;
    let s = String::from_utf8_lossy(&data);
    let v: Value = serde_norway::from_str(&s)
        .with_context(|| format!("failed to parse registry for app \"{app}\""))?;
    let mut reg = parse_registry(&v)?;
    if reg.version != SCHEMA_VERSION {
        return Err(anyhow!(
            "registry for app \"{app}\" has unsupported version {} (expected {SCHEMA_VERSION}) at {}",
            reg.version,
            path.display()
        ));
    }
    for ctx in &mut reg.contexts {
        if ctx.code.is_empty() {
            return Err(anyhow!(
                "registry for app \"{app}\" context \"{}\" has empty code list at {}",
                ctx.name,
                path.display()
            ));
        }
        if ctx.gherkin.is_empty() {
            return Err(anyhow!(
                "registry for app \"{app}\" context \"{}\" has empty gherkin list at {}",
                ctx.name,
                path.display()
            ));
        }
        if ctx.code_lang.is_empty() {
            ctx.code_lang = vec!["ts".into(), "tsx".into()];
        }
        validate_code_lang(&ctx.code_lang)
            .with_context(|| format!("registry context \"{}\"", ctx.name))?;
    }
    Ok(reg)
}

/// Parses a raw YAML [`Value`] into a [`Registry`].
///
/// # Errors
///
/// Returns an error if the YAML structure is malformed in a way that prevents
/// construction of a valid [`Registry`].
fn parse_registry(v: &Value) -> Result<Registry, Error> {
    let mut r = Registry::default();
    if let Value::Mapping(m) = v {
        for (k, val) in m {
            match k.as_str().unwrap_or("") {
                "version" => r.version = val.as_i64().unwrap_or(0),
                "app" => r.app = val.as_str().unwrap_or("").to_string(),
                "contexts" => {
                    if let Value::Sequence(seq) = val {
                        for c in seq {
                            r.contexts.push(parse_context(c));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(r)
}

/// Parses a single context YAML node into a [`BcContext`].
fn parse_context(v: &Value) -> BcContext {
    let mut c = BcContext::default();
    if let Value::Mapping(m) = v {
        for (k, val) in m {
            match k.as_str().unwrap_or("") {
                "name" => c.name = val.as_str().unwrap_or("").to_string(),
                "summary" => c.summary = val.as_str().unwrap_or("").to_string(),
                "glossary" => c.glossary = val.as_str().unwrap_or("").to_string(),
                "layers" => c.layers = parse_string_seq(val),
                "code" => c.code = parse_string_seq(val),
                "code_lang" => c.code_lang = parse_string_seq(val),
                "gherkin" => c.gherkin = parse_gherkin(val),
                "relationships" => {
                    if let Value::Sequence(seq) = val {
                        for r in seq {
                            c.relationships.push(parse_relationship(r));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    c
}

/// Parses a YAML sequence of strings into a `Vec<String>`.
fn parse_string_seq(v: &Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Value::Sequence(seq) = v {
        for item in seq {
            if let Some(s) = item.as_str() {
                out.push(s.to_string());
            }
        }
    }
    out
}

/// Parses the `gherkin` field which may be a scalar string or a sequence.
///
/// A scalar string is wrapped into a single-element `Vec`.
fn parse_gherkin(v: &Value) -> Vec<String> {
    match v {
        Value::String(s) => vec![s.clone()],
        Value::Sequence(_) => parse_string_seq(v),
        _ => Vec::new(),
    }
}

/// Parses a single relationship YAML node into a [`Relationship`].
fn parse_relationship(v: &Value) -> Relationship {
    let mut r = Relationship::default();
    if let Value::Mapping(m) = v {
        for (k, val) in m {
            match k.as_str().unwrap_or("") {
                "to" => r.to = val.as_str().unwrap_or("").to_string(),
                "kind" => r.kind = val.as_str().unwrap_or("").to_string(),
                "role" => r.role = val.as_str().unwrap_or("").to_string(),
                _ => {}
            }
        }
    }
    r
}

/// Loads the registry for `opts.app` and validates all contexts against the filesystem.
///
/// Returns a sorted list of [`Finding`]s.  An empty list means no issues were detected.
///
/// # Errors
///
/// Returns an error when the registry file cannot be loaded (see [`load`]).
pub fn validate_all(opts: &ValidateOptions) -> Result<Vec<Finding>, Error> {
    let sev = opts.severity.unwrap_or(Severity::Error);
    let reg = load(&opts.repo_root, &opts.app)?;
    Ok(validate_registry(&opts.repo_root, &reg, sev))
}

/// Runs all registry validation checks and returns a deduplicated, sorted list of findings.
fn validate_registry(repo_root: &Path, reg: &Registry, sev: Severity) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Vec::new();
    let mut registered_code: HashMap<String, bool> = HashMap::new();
    let mut registered_glossary: HashMap<String, bool> = HashMap::new();
    let mut registered_gherkin: HashMap<String, bool> = HashMap::new();
    let mut context_by_name: HashMap<String, &BcContext> = HashMap::new();

    for ctx in &reg.contexts {
        context_by_name.insert(ctx.name.clone(), ctx);
        for c in &ctx.code {
            registered_code.insert(repo_root.join(c).to_string_lossy().into_owned(), true);
        }
        registered_glossary.insert(
            repo_root.join(&ctx.glossary).to_string_lossy().into_owned(),
            true,
        );
        for g in &ctx.gherkin {
            registered_gherkin.insert(repo_root.join(g).to_string_lossy().into_owned(), true);
        }
    }

    for ctx in &reg.contexts {
        findings.extend(check_context(repo_root, ctx, sev));
    }

    if !reg.contexts.is_empty() {
        findings.extend(detect_orphans(
            repo_root,
            reg,
            &registered_code,
            &registered_glossary,
            &registered_gherkin,
            sev,
        ));
    }

    findings.extend(check_relationship_symmetry(reg, &context_by_name, sev));
    findings.extend(check_relationship_kinds(reg, sev));

    findings.sort_by(|a, b| a.file.cmp(&b.file));
    findings
}

/// Validates a single bounded context: code directories, layer structure, glossary, and Gherkin.
fn check_context(repo_root: &Path, ctx: &BcContext, sev: Severity) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Vec::new();
    for code_rel in &ctx.code {
        let code_path = repo_root.join(code_rel);
        if !code_path.exists() {
            findings.push(Finding {
                file: code_rel.clone(),
                message: format!("missing code directory for context \"{}\"", ctx.name),
                severity: sev,
            });
            continue;
        }
        findings.extend(check_layers_at_path(repo_root, ctx, code_rel, sev));
    }
    let glossary_path = repo_root.join(&ctx.glossary);
    if !glossary_path.exists() {
        findings.push(Finding {
            file: ctx.glossary.clone(),
            message: format!("missing glossary for context \"{}\"", ctx.name),
            severity: sev,
        });
    }
    findings.extend(check_gherkin(repo_root, ctx, sev));
    findings
}

/// Checks that the declared layers exist inside `code_rel` and that no undeclared
/// layer directories are present on the filesystem.
fn check_layers_at_path(
    repo_root: &Path,
    ctx: &BcContext,
    code_rel: &str,
    sev: Severity,
) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Vec::new();
    let code_path = repo_root.join(code_rel);
    let entries = match fs::read_dir(&code_path) {
        Ok(e) => e,
        Err(e) => {
            return vec![Finding {
                file: code_rel.to_string(),
                message: format!(
                    "cannot read code directory for context \"{}\": {e}",
                    ctx.name
                ),
                severity: sev,
            }];
        }
    };
    let mut actual: HashMap<String, bool> = HashMap::new();
    for entry in entries.flatten() {
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            actual.insert(entry.file_name().to_string_lossy().into_owned(), true);
        }
    }
    let declared: std::collections::HashSet<String> = ctx.layers.iter().cloned().collect();
    for l in &ctx.layers {
        if !actual.contains_key(l) {
            findings.push(Finding {
                file: format!("{code_rel}/{l}"),
                message: format!("missing layer \"{l}\" for context \"{}\"", ctx.name),
                severity: sev,
            });
        }
    }
    let mut extras: Vec<String> = actual
        .keys()
        .filter(|n| !declared.contains(n.as_str()))
        .cloned()
        .collect();
    extras.sort();
    for name in extras {
        findings.push(Finding {
            file: format!("{code_rel}/{name}"),
            message: format!(
                "extra layer \"{name}\" found on filesystem but not declared in registry for context \"{}\"",
                ctx.name
            ),
            severity: sev,
        });
    }
    findings
}

/// Verifies that every Gherkin directory exists and contains at least one `.feature` file.
fn check_gherkin(repo_root: &Path, ctx: &BcContext, sev: Severity) -> Vec<Finding> {
    let mut findings = Vec::new();
    for gh in &ctx.gherkin {
        let gpath = repo_root.join(gh);
        if !gpath.exists() {
            findings.push(Finding {
                file: gh.clone(),
                message: format!("missing gherkin directory for context \"{}\"", ctx.name),
                severity: sev,
            });
            continue;
        }
        let entries = match fs::read_dir(&gpath) {
            Ok(e) => e,
            Err(e) => {
                findings.push(Finding {
                    file: gh.clone(),
                    message: format!(
                        "cannot read gherkin directory for context \"{}\": {e}",
                        ctx.name
                    ),
                    severity: sev,
                });
                continue;
            }
        };
        let mut has = false;
        for entry in entries.flatten() {
            let n = entry.file_name().to_string_lossy().into_owned();
            if entry.file_type().is_ok_and(|t| !t.is_dir()) && n.ends_with(".feature") {
                has = true;
                break;
            }
        }
        if !has {
            findings.push(Finding {
                file: gh.clone(),
                message: format!(
                    "no feature files found in gherkin directory for context \"{}\"",
                    ctx.name
                ),
                severity: sev,
            });
        }
    }
    findings
}

/// Scans the filesystem for directories and files that exist on disk but are
/// not declared in the registry (orphans), and reports them as findings.
fn detect_orphans(
    repo_root: &Path,
    reg: &Registry,
    registered_code: &HashMap<String, bool>,
    registered_glossary: &HashMap<String, bool>,
    registered_gherkin: &HashMap<String, bool>,
    sev: Severity,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut code_roots: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for ctx in &reg.contexts {
        for c in &ctx.code {
            let p = repo_root.join(c);
            if let Some(parent) = p.parent() {
                code_roots.insert(parent.to_string_lossy().into_owned());
            }
        }
    }
    for root in &code_roots {
        findings.extend(detect_orphan_dirs(
            root,
            registered_code,
            "orphan code directory",
            "registered in bounded-contexts.yaml",
            sev,
        ));
    }
    let mut glossary_roots: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut gherkin_roots: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for ctx in &reg.contexts {
        if let Some(parent) = repo_root.join(&ctx.glossary).parent() {
            glossary_roots.insert(parent.to_string_lossy().into_owned());
        }
        for gh in &ctx.gherkin {
            if let Some(parent) = repo_root.join(gh).parent() {
                gherkin_roots.insert(parent.to_string_lossy().into_owned());
            }
        }
    }
    for root in &glossary_roots {
        findings.extend(detect_orphan_files(
            root,
            registered_glossary,
            "orphan glossary file",
            "registered in bounded-contexts.yaml",
            sev,
        ));
    }
    for root in &gherkin_roots {
        findings.extend(detect_orphan_dirs(
            root,
            registered_gherkin,
            "orphan gherkin directory",
            "registered in bounded-contexts.yaml",
            sev,
        ));
    }
    findings
}

/// Scans `root` for sub-directories not present in `registered` and emits
/// a finding of type `kind` for each one.
fn detect_orphan_dirs(
    root: &str,
    registered: &HashMap<String, bool>,
    kind: &str,
    not_reason: &str,
    sev: Severity,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return findings;
    };
    let mut items: Vec<(String, std::path::PathBuf)> = Vec::new();
    for entry in entries.flatten() {
        if !entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let n = entry.file_name().to_string_lossy().into_owned();
        items.push((n, entry.path()));
    }
    items.sort_by(|a, b| a.0.cmp(&b.0));
    for (n, p) in items {
        let full = p.to_string_lossy().into_owned();
        if !registered.contains_key(&full) {
            findings.push(Finding {
                file: full,
                message: format!("{kind} \"{n}\" not {not_reason}"),
                severity: sev,
            });
        }
    }
    findings
}

/// Scans `root` for `.md` files (excluding `README.md`) not present in
/// `registered` and emits a finding of type `kind` for each one.
fn detect_orphan_files(
    root: &str,
    registered: &HashMap<String, bool>,
    kind: &str,
    not_reason: &str,
    sev: Severity,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return findings;
    };
    let mut items: Vec<(String, std::path::PathBuf)> = Vec::new();
    for entry in entries.flatten() {
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let n = entry.file_name().to_string_lossy().into_owned();
        if !n.ends_with(".md") || n == "README.md" {
            continue;
        }
        items.push((n, entry.path()));
    }
    items.sort_by(|a, b| a.0.cmp(&b.0));
    for (n, p) in items {
        let full = p.to_string_lossy().into_owned();
        if !registered.contains_key(&full) {
            findings.push(Finding {
                file: full,
                message: format!("{kind} \"{n}\" not {not_reason}"),
                severity: sev,
            });
        }
    }
    findings
}

/// Checks that every symmetric relationship (`customer-supplier`, `conformist`,
/// `partnership`, `shared-kernel`) has a matching reciprocal declaration in the
/// target context.
fn check_relationship_symmetry(
    reg: &Registry,
    by_name: &HashMap<String, &BcContext>,
    sev: Severity,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let yaml_path = format!("specs/apps/{}/ddd/bounded-contexts.yaml", reg.app);
    for ctx in &reg.contexts {
        for rel in &ctx.relationships {
            let Some(asym) = relationship_kind_is_asymmetric(&rel.kind) else {
                continue; // unknown kinds reported elsewhere
            };
            if !asym {
                continue;
            }
            let target = if let Some(t) = by_name.get(&rel.to) {
                *t
            } else {
                findings.push(Finding {
                    file: yaml_path.clone(),
                    message: format!(
                        "relationship target \"{}\" declared by \"{}\" does not exist in registry",
                        rel.to, ctx.name
                    ),
                    severity: sev,
                });
                continue;
            };
            if !has_reciprocal(target, &ctx.name, &rel.kind) {
                findings.push(Finding {
                    file: yaml_path.clone(),
                    message: format!(
                        "relationship asymmetry: \"{}\" → \"{}\" ({}) but \"{}\" has no reciprocal entry",
                        ctx.name, rel.to, rel.kind, rel.to
                    ),
                    severity: sev,
                });
            }
        }
    }
    findings
}

/// Returns `true` when `ctx` has a relationship back to `source` with the same `kind`.
fn has_reciprocal(ctx: &BcContext, source: &str, kind: &str) -> bool {
    ctx.relationships
        .iter()
        .any(|r| r.to == source && r.kind == kind)
}

/// Checks that all declared relationship kinds are among the recognised values.
fn check_relationship_kinds(reg: &Registry, sev: Severity) -> Vec<Finding> {
    let mut findings = Vec::new();
    let yaml_path = format!("specs/apps/{}/ddd/bounded-contexts.yaml", reg.app);
    for ctx in &reg.contexts {
        for rel in &ctx.relationships {
            if relationship_kind_is_asymmetric(&rel.kind).is_none() {
                findings.push(Finding {
                    file: yaml_path.clone(),
                    message: format!(
                        "unknown relationship kind \"{}\" in \"{}\" → \"{}\"",
                        rel.kind, ctx.name, rel.to
                    ),
                    severity: sev,
                });
            }
        }
    }
    findings
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    /// Creates a file at `p`, making parent directories as needed.
    fn write(p: &Path, s: &str) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, s).unwrap();
    }

    /// Verifies that [`supported_lang_globs`] includes a representative set of languages.
    #[test]
    fn supported_lang_includes_known() {
        let m = supported_lang_globs();
        assert!(m.contains_key("ts"));
        assert!(m.contains_key("rs"));
        assert!(m.contains_key("dart"));
    }

    /// Verifies that [`validate_code_lang`] rejects unknown language identifiers.
    #[test]
    fn validate_code_lang_rejects_unknown() {
        assert!(validate_code_lang(&["ts".into(), "bogus".into()]).is_err());
        assert!(validate_code_lang(&["ts".into(), "tsx".into()]).is_ok());
    }

    /// Verifies that [`relationship_kind_is_asymmetric`] returns the correct
    /// symmetry flag for known relationship kinds and `None` for unknown ones.
    #[test]
    fn relationship_kind_asymmetry() {
        assert_eq!(
            relationship_kind_is_asymmetric("customer-supplier"),
            Some(true)
        );
        assert_eq!(
            relationship_kind_is_asymmetric("anticorruption-layer"),
            Some(false)
        );
        assert_eq!(relationship_kind_is_asymmetric("bogus"), None);
    }

    /// Returns a minimal valid `bounded-contexts.yaml` YAML string for testing.
    fn minimal_registry_yaml() -> &'static str {
        r"version: 2
app: testapp
contexts:
  - name: ctx-a
    summary: ok
    layers: [domain]
    code:
      - apps/testapp/src
    glossary: specs/apps/testapp/glossary/ctx-a.md
    gherkin: specs/apps/testapp/behavior/gherkin/ctx-a
    relationships: []
"
    }

    /// Verifies that [`load`] correctly parses a minimal valid registry, including
    /// applying the default `code_lang` when none is specified.
    #[test]
    fn load_parses_minimal_yaml() {
        let dir = tempdir().unwrap();
        let p = dir
            .path()
            .join("specs/apps/testapp/ddd/bounded-contexts.yaml");
        write(&p, minimal_registry_yaml());
        let r = load(dir.path(), "testapp").unwrap();
        assert_eq!(r.version, 2);
        assert_eq!(r.contexts.len(), 1);
        assert_eq!(r.contexts[0].name, "ctx-a");
        assert_eq!(r.contexts[0].code_lang, vec!["ts", "tsx"]);
    }

    /// Verifies that [`load`] returns an error when the schema version is unsupported.
    #[test]
    fn load_rejects_wrong_version() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("specs/apps/x/ddd/bounded-contexts.yaml");
        write(&p, "version: 1\napp: x\ncontexts: []\n");
        let r = load(dir.path(), "x");
        assert!(r.is_err());
    }

    /// Verifies that [`load`] returns an error when a context has an empty `code` list.
    #[test]
    fn load_rejects_empty_code() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("specs/apps/x/ddd/bounded-contexts.yaml");
        write(
            &p,
            "version: 2\napp: x\ncontexts:\n  - name: ctx\n    layers: []\n    code: []\n    glossary: g\n    gherkin: gh\n",
        );
        let r = load(dir.path(), "x");
        assert!(r.is_err());
    }

    /// Verifies that a scalar `gherkin` value is wrapped into a single-element list.
    #[test]
    fn load_gherkin_scalar_becomes_singleton() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("specs/apps/x/ddd/bounded-contexts.yaml");
        write(
            &p,
            "version: 2\napp: x\ncontexts:\n  - name: ctx\n    layers: [domain]\n    code: [\"apps/x/src\"]\n    glossary: g\n    gherkin: behavior/gherkin/x\n",
        );
        let r = load(dir.path(), "x").unwrap();
        assert_eq!(r.contexts[0].gherkin, vec!["behavior/gherkin/x"]);
    }

    /// Verifies that [`validate_all`] reports missing code, glossary, and gherkin paths.
    #[test]
    fn validate_all_with_missing_paths_reports() {
        let dir = tempdir().unwrap();
        let p = dir
            .path()
            .join("specs/apps/testapp/ddd/bounded-contexts.yaml");
        write(&p, minimal_registry_yaml());
        let r = validate_all(&ValidateOptions {
            repo_root: dir.path().to_path_buf(),
            app: "testapp".to_string(),
            severity: Some(Severity::Error),
        })
        .unwrap();
        // missing code dir + missing glossary + missing gherkin → 3 base findings.
        assert!(r.iter().any(|f| f.message.contains("missing code")));
        assert!(r.iter().any(|f| f.message.contains("missing glossary")));
        assert!(r.iter().any(|f| f.message.contains("missing gherkin")));
    }

    /// Verifies that [`validate_all`] returns no findings when the filesystem matches
    /// the registry exactly.
    #[test]
    fn validate_all_clean_corpus() {
        let dir = tempdir().unwrap();
        let p = dir
            .path()
            .join("specs/apps/testapp/ddd/bounded-contexts.yaml");
        write(&p, minimal_registry_yaml());
        // Create matching filesystem.
        std::fs::create_dir_all(dir.path().join("apps/testapp/src/domain")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/testapp/glossary")).unwrap();
        std::fs::write(dir.path().join("specs/apps/testapp/glossary/ctx-a.md"), "x").unwrap();
        let gherk = dir.path().join("specs/apps/testapp/behavior/gherkin/ctx-a");
        std::fs::create_dir_all(&gherk).unwrap();
        std::fs::write(gherk.join("x.feature"), "x").unwrap();
        let r = validate_all(&ValidateOptions {
            repo_root: dir.path().to_path_buf(),
            app: "testapp".to_string(),
            severity: Some(Severity::Error),
        })
        .unwrap();
        assert_eq!(r.len(), 0, "{r:#?}");
    }

    /// Verifies that an extra layer directory on the filesystem (not in the registry)
    /// is reported as a finding.
    #[test]
    fn validate_detects_extra_layer() {
        let dir = tempdir().unwrap();
        let p = dir
            .path()
            .join("specs/apps/testapp/ddd/bounded-contexts.yaml");
        write(&p, minimal_registry_yaml());
        std::fs::create_dir_all(dir.path().join("apps/testapp/src/domain")).unwrap();
        std::fs::create_dir_all(dir.path().join("apps/testapp/src/unexpected")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/testapp/glossary")).unwrap();
        std::fs::write(dir.path().join("specs/apps/testapp/glossary/ctx-a.md"), "x").unwrap();
        let gherk = dir.path().join("specs/apps/testapp/behavior/gherkin/ctx-a");
        std::fs::create_dir_all(&gherk).unwrap();
        std::fs::write(gherk.join("x.feature"), "x").unwrap();
        let r = validate_all(&ValidateOptions {
            repo_root: dir.path().to_path_buf(),
            app: "testapp".to_string(),
            severity: Some(Severity::Error),
        })
        .unwrap();
        assert!(r.iter().any(|f| f.message.contains("extra layer")));
    }

    /// Verifies that an unknown relationship kind is reported as a finding.
    #[test]
    fn validate_detects_unknown_relationship_kind() {
        let dir = tempdir().unwrap();
        let yaml = r#"version: 2
app: x
contexts:
  - name: a
    summary: s
    layers: [domain]
    code: ["apps/x/src"]
    glossary: g
    gherkin: gh
    relationships:
      - to: b
        kind: bogus-kind
        role: customer
  - name: b
    summary: s
    layers: [domain]
    code: ["apps/x/src2"]
    glossary: g2
    gherkin: gh2
    relationships: []
"#;
        let p = dir.path().join("specs/apps/x/ddd/bounded-contexts.yaml");
        write(&p, yaml);
        let r = validate_all(&ValidateOptions {
            repo_root: dir.path().to_path_buf(),
            app: "x".to_string(),
            severity: Some(Severity::Error),
        })
        .unwrap();
        assert!(
            r.iter()
                .any(|f| f.message.contains("unknown relationship kind"))
        );
    }
}
