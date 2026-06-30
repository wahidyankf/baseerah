//! `env validate` — manifest-consistency pass for `env-injection.yaml`.
//!
//! A static, value-free check that the injection manifest (`env-injection.yaml`)
//! stays consistent with the surface registry (`env-contract.yaml`) and the apps'
//! committed `.env.example` files. It never reads real values and never touches
//! GitHub / Vercel / k3s — it only diffs NAMES and paths on disk.
//!
//! # ENV-INJECTION CONFIG: `env-injection.yaml` at repo root, parsed with `serde_norway`.
//! Each `apps[]` entry carries `app`, `keys-from`, and a `runtime` home map.
//! Each `ci-harness[]` entry carries `key`, `class`, and `environments`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error};
use serde::Deserialize;

use super::validate::{Contract, parse_declared_keys};

/// Allowed injection homes for a manifest app's `runtime` map.
///
/// A `runtime` value outside this set is a manifest typo or an undocumented home.
pub const ALLOWED_HOMES: &[&str] = &[
    "env-local",
    "compose",
    "vercel-preview",
    "vercel-production",
    "k3s-coralpolyp",
];

/// A single app injection entry from `env-injection.yaml` (`apps[]`).
#[derive(Debug, Deserialize, Clone)]
pub struct InjectionApp {
    /// Logical app name; must match a `kind: app` surface root tail in `env-contract.yaml`.
    pub app: String,
    /// Repo-relative path to the canonical `.env.example` key source.
    #[serde(rename = "keys-from")]
    pub keys_from: String,
    /// Stage → injection-home map (e.g. `local: env-local`). Must be non-empty.
    #[serde(default)]
    pub runtime: std::collections::BTreeMap<String, String>,
}

/// A single CI test-harness key entry from `env-injection.yaml` (`ci-harness[]`).
#[derive(Debug, Deserialize, Clone)]
pub struct CiHarnessKey {
    /// Test-only key name; must NOT appear in any `apps/*/.env.example`.
    pub key: String,
    /// Injection class label (`var` / `secret` / `literal`) — recorded, not validated here.
    #[serde(default)]
    pub class: String,
}

/// Top-level `env-injection.yaml` structure.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    /// Per-app injection entries.
    #[serde(default)]
    pub apps: Vec<InjectionApp>,
    /// CI test-harness key entries.
    #[serde(default, rename = "ci-harness")]
    pub ci_harness: Vec<CiHarnessKey>,
}

/// Kind of manifest-consistency violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestProblem {
    /// A `kind: app` contract surface has no matching `apps[].app` manifest entry.
    AppMissingFromManifest,
    /// A manifest `apps[].app` entry has no matching `kind: app` contract surface.
    AppNotAContractSurface,
    /// A manifest app's `keys-from` does not equal/exist at `apps/<app>/.env.example`.
    KeysFromMismatch,
    /// A manifest app's `runtime` map is empty.
    EmptyRuntime,
    /// A manifest app's `runtime` declares an unknown injection home.
    UnknownHome,
    /// A `ci-harness[].key` also appears as a declared key in some `apps/*/.env.example`.
    CiHarnessKeyLeaksIntoApp,
}

impl ManifestProblem {
    /// Human-readable label for display.
    pub fn label(self) -> &'static str {
        match self {
            Self::AppMissingFromManifest => "app-missing-from-manifest",
            Self::AppNotAContractSurface => "app-not-a-contract-surface",
            Self::KeysFromMismatch => "keys-from-mismatch",
            Self::EmptyRuntime => "empty-runtime",
            Self::UnknownHome => "unknown-home",
            Self::CiHarnessKeyLeaksIntoApp => "ci-harness-key-leaks-into-app",
        }
    }
}

/// A single manifest-consistency finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestFinding {
    /// The kind of violation.
    pub problem: ManifestProblem,
    /// The offending app, key, or home name (whatever the problem is about).
    pub subject: String,
    /// Optional supporting detail (path, home value, etc.).
    pub detail: String,
}

impl ManifestFinding {
    /// Construct a finding from its problem, subject, and supporting detail.
    fn new(
        problem: ManifestProblem,
        subject: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            problem,
            subject: subject.into(),
            detail: detail.into(),
        }
    }
}

/// Load and parse the `env-injection:` section from `repo-config.yml` at `repo_root`.
///
/// # Errors
///
/// Returns an error when `repo-config.yml` cannot be read, is not valid YAML,
/// or the `env-injection:` section is absent.
pub fn load_manifest(repo_root: &Path) -> Result<Manifest, Error> {
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(rename = "env-injection")]
        env_injection: Option<Manifest>,
    }

    let path = repo_root.join("repo-config.yml");
    let data = fs::read_to_string(&path)
        .with_context(|| format!("cannot read repo-config.yml at {}", path.display()))?;
    let wrapper: Wrapper = serde_norway::from_str(&data)
        .with_context(|| format!("failed to parse repo-config.yml at {}", path.display()))?;
    wrapper.env_injection.ok_or_else(|| {
        anyhow::anyhow!(
            "env-injection: section missing from repo-config.yml at {}",
            path.display()
        )
    })
}

/// Tail of a contract surface root: `apps/ose-www` → `ose-www`.
fn surface_app_name(root: &str) -> &str {
    root.rsplit('/').next().unwrap_or(root)
}

/// Pure consistency check between a parsed manifest and contract.
///
/// `app_declared_keys` maps each `app` name to its declared `.env.example` keys
/// (already parsed by the caller — keeps this function free of filesystem I/O for
/// the key-leak rule). `existing_keys_from` is the set of `keys-from` paths that
/// actually exist on disk (caller-checked). This keeps the function a pure fold
/// over data the imperative shell gathered.
///
/// Returns findings sorted deterministically by `(label, subject, detail)`.
#[must_use]
pub fn check_consistency(
    manifest: &Manifest,
    contract: &Contract,
    app_declared_keys: &std::collections::BTreeMap<String, BTreeSet<String>>,
    existing_keys_from: &BTreeSet<String>,
) -> Vec<ManifestFinding> {
    let mut findings = Vec::new();

    // Contract `kind: app` surfaces, by app-name tail.
    let contract_apps: BTreeSet<String> = contract
        .surfaces
        .iter()
        .filter(|s| s.kind == "app")
        .map(|s| surface_app_name(&s.root).to_string())
        .collect();

    let manifest_apps: BTreeSet<String> = manifest.apps.iter().map(|a| a.app.clone()).collect();

    // Rule 1: coverage, both directions.
    for app in contract_apps.difference(&manifest_apps) {
        findings.push(ManifestFinding::new(
            ManifestProblem::AppMissingFromManifest,
            app.clone(),
            "contract surface has no apps[] entry in env-injection.yaml",
        ));
    }
    for app in manifest_apps.difference(&contract_apps) {
        findings.push(ManifestFinding::new(
            ManifestProblem::AppNotAContractSurface,
            app.clone(),
            "manifest app is not a kind: app surface in env-contract.yaml",
        ));
    }

    for app in &manifest.apps {
        // Rule 2: keys-from integrity (canonical path + exists on disk).
        let canonical = format!("apps/{}/.env.example", app.app);
        if app.keys_from != canonical {
            findings.push(ManifestFinding::new(
                ManifestProblem::KeysFromMismatch,
                app.app.clone(),
                format!("keys-from '{}' != canonical '{canonical}'", app.keys_from),
            ));
        } else if !existing_keys_from.contains(&app.keys_from) {
            findings.push(ManifestFinding::new(
                ManifestProblem::KeysFromMismatch,
                app.app.clone(),
                format!("keys-from path '{}' does not exist", app.keys_from),
            ));
        }

        // Rule 3: runtime non-empty + known homes only.
        if app.runtime.is_empty() {
            findings.push(ManifestFinding::new(
                ManifestProblem::EmptyRuntime,
                app.app.clone(),
                "runtime map is empty",
            ));
        }
        for (stage, home) in &app.runtime {
            if !ALLOWED_HOMES.contains(&home.as_str()) {
                findings.push(ManifestFinding::new(
                    ManifestProblem::UnknownHome,
                    app.app.clone(),
                    format!("stage '{stage}' uses unknown home '{home}'"),
                ));
            }
        }
    }

    // Rule 4: ci-harness keys must not be declared in any app .env.example.
    for ci in &manifest.ci_harness {
        for (app, keys) in app_declared_keys {
            if keys.contains(&ci.key) {
                findings.push(ManifestFinding::new(
                    ManifestProblem::CiHarnessKeyLeaksIntoApp,
                    ci.key.clone(),
                    format!("ci-harness key declared in apps/{app}/.env.example"),
                ));
            }
        }
    }

    findings.sort_by(|a, b| {
        a.problem
            .label()
            .cmp(b.problem.label())
            .then_with(|| a.subject.cmp(&b.subject))
            .then_with(|| a.detail.cmp(&b.detail))
    });
    findings
}

/// Imperative shell: gather filesystem facts, then run the pure consistency check.
///
/// Reads `env-injection.yaml`, each contract app's `.env.example` (for the
/// key-leak rule), and probes `keys-from` paths for existence. Delegates the
/// actual rule evaluation to [`check_consistency`].
///
/// # Errors
///
/// Returns an error when the manifest cannot be loaded or a referenced
/// `.env.example` cannot be read.
pub fn validate_manifest(
    repo_root: &Path,
    contract: &Contract,
) -> Result<Vec<ManifestFinding>, Error> {
    let manifest = load_manifest(repo_root)?;

    // Declared keys per contract app surface (used for the ci-harness leak rule).
    let mut app_declared_keys: std::collections::BTreeMap<String, BTreeSet<String>> =
        std::collections::BTreeMap::new();
    for surface in contract.surfaces.iter().filter(|s| s.kind == "app") {
        let app = surface_app_name(&surface.root).to_string();
        let env_example = repo_root.join(&surface.root).join(".env.example");
        let keys: BTreeSet<String> = if env_example.exists() {
            parse_declared_keys(&env_example)?.into_iter().collect()
        } else {
            BTreeSet::new()
        };
        app_declared_keys.insert(app, keys);
    }

    // Which manifest keys-from paths actually exist on disk.
    let mut existing_keys_from: BTreeSet<String> = BTreeSet::new();
    for app in &manifest.apps {
        let p: PathBuf = repo_root.join(&app.keys_from);
        if p.exists() {
            existing_keys_from.insert(app.keys_from.clone());
        }
    }

    Ok(check_consistency(
        &manifest,
        contract,
        &app_declared_keys,
        &existing_keys_from,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::application::env::validate::SurfaceConfig;
    use std::collections::{BTreeMap, BTreeSet};

    fn surface(root: &str) -> SurfaceConfig {
        SurfaceConfig {
            root: root.to_string(),
            kind: "app".to_string(),
            lang: "typescript".to_string(),
            allowlist: vec![],
        }
    }

    fn app(name: &str, runtime: &[(&str, &str)]) -> InjectionApp {
        InjectionApp {
            app: name.to_string(),
            keys_from: format!("apps/{name}/.env.example"),
            runtime: runtime
                .iter()
                .map(|(s, h)| ((*s).to_string(), (*h).to_string()))
                .collect(),
        }
    }

    fn existing(apps: &[InjectionApp]) -> BTreeSet<String> {
        apps.iter().map(|a| a.keys_from.clone()).collect()
    }

    #[test]
    fn matched_manifest_has_no_findings() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www"), surface("apps/ose-be")],
        };
        let apps = vec![
            app(
                "ose-www",
                &[("local", "env-local"), ("production", "vercel-production")],
            ),
            app("ose-be", &[("staging", "k3s-coralpolyp")]),
        ];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![CiHarnessKey {
                key: "WEB_BASE_URL".to_string(),
                class: "var".to_string(),
            }],
        };
        let declared = BTreeMap::new();
        let findings = check_consistency(&manifest, &contract, &declared, &existing(&apps));
        assert!(findings.is_empty(), "expected none; got {findings:?}");
    }

    #[test]
    fn contract_app_missing_from_manifest_is_reported() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www"), surface("apps/ose-be")],
        };
        let apps = vec![app("ose-www", &[("local", "env-local")])];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![],
        };
        let findings = check_consistency(&manifest, &contract, &BTreeMap::new(), &existing(&apps));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].problem, ManifestProblem::AppMissingFromManifest);
        assert_eq!(findings[0].subject, "ose-be");
    }

    #[test]
    fn manifest_app_not_a_contract_surface_is_reported() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www")],
        };
        let apps = vec![
            app("ose-www", &[("local", "env-local")]),
            app("ghost-app", &[("local", "env-local")]),
        ];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![],
        };
        let findings = check_consistency(&manifest, &contract, &BTreeMap::new(), &existing(&apps));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].problem, ManifestProblem::AppNotAContractSurface);
        assert_eq!(findings[0].subject, "ghost-app");
    }

    #[test]
    fn empty_runtime_is_reported() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www")],
        };
        let apps = vec![app("ose-www", &[])];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![],
        };
        let findings = check_consistency(&manifest, &contract, &BTreeMap::new(), &existing(&apps));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].problem, ManifestProblem::EmptyRuntime);
    }

    #[test]
    fn unknown_home_is_reported() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www")],
        };
        let apps = vec![app("ose-www", &[("local", "fly-io")])];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![],
        };
        let findings = check_consistency(&manifest, &contract, &BTreeMap::new(), &existing(&apps));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].problem, ManifestProblem::UnknownHome);
        assert!(findings[0].detail.contains("fly-io"));
    }

    #[test]
    fn keys_from_mismatch_is_reported() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www")],
        };
        let mut a = app("ose-www", &[("local", "env-local")]);
        a.keys_from = "apps/ose-www/.env.sample".to_string();
        let apps = vec![a];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![],
        };
        let findings = check_consistency(&manifest, &contract, &BTreeMap::new(), &existing(&apps));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].problem, ManifestProblem::KeysFromMismatch);
    }

    #[test]
    fn keys_from_path_missing_is_reported() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www")],
        };
        let apps = vec![app("ose-www", &[("local", "env-local")])];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![],
        };
        // existing set is empty → path does not exist on disk
        let findings = check_consistency(&manifest, &contract, &BTreeMap::new(), &BTreeSet::new());
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].problem, ManifestProblem::KeysFromMismatch);
        assert!(findings[0].detail.contains("does not exist"));
    }

    #[test]
    fn ci_harness_key_leaking_into_app_is_reported() {
        let contract = Contract {
            surfaces: vec![surface("apps/ose-www")],
        };
        let apps = vec![app("ose-www", &[("local", "env-local")])];
        let manifest = Manifest {
            apps: apps.clone(),
            ci_harness: vec![CiHarnessKey {
                key: "WEB_BASE_URL".to_string(),
                class: "var".to_string(),
            }],
        };
        let mut declared = BTreeMap::new();
        declared.insert(
            "ose-www".to_string(),
            BTreeSet::from(["WEB_BASE_URL".to_string()]),
        );
        let findings = check_consistency(&manifest, &contract, &declared, &existing(&apps));
        assert_eq!(findings.len(), 1);
        assert_eq!(
            findings[0].problem,
            ManifestProblem::CiHarnessKeyLeaksIntoApp
        );
        assert_eq!(findings[0].subject, "WEB_BASE_URL");
        assert!(findings[0].detail.contains("ose-www"));
    }

    // ── load_manifest from repo-config.yml (RED → GREEN) ─────────────────────

    #[test]
    fn load_manifest_reads_env_injection_section_from_repo_config_yml() {
        use std::fs;
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let yaml = concat!(
            "harness: []\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
            "env-injection:\n",
            "  apps:\n",
            "    - app: demo\n",
            "      keys-from: apps/demo/.env.example\n",
            "      runtime: { local: env-local }\n",
            "  ci-harness: []\n",
        );
        fs::write(tmp.path().join("repo-config.yml"), yaml).unwrap();
        // NO standalone env-injection.yaml — loader must read from repo-config.yml
        let result = load_manifest(tmp.path());
        assert!(
            result.is_ok(),
            "should read env-injection: from repo-config.yml without standalone file: {result:?}"
        );
        let manifest = result.unwrap();
        assert_eq!(manifest.apps.len(), 1);
        assert_eq!(manifest.apps[0].app, "demo");
    }

    #[test]
    fn load_manifest_errors_when_env_injection_section_missing_from_repo_config_yml() {
        use std::fs;
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let yaml = concat!(
            "harness: []\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
        );
        fs::write(tmp.path().join("repo-config.yml"), yaml).unwrap();
        // NO standalone env-injection.yaml, NO env-injection: section in repo-config.yml
        let result = load_manifest(tmp.path());
        assert!(
            result.is_err(),
            "should error when env-injection: section is absent from repo-config.yml"
        );
    }
}
