//! Integration tests for `env validate` — exercises the full
//! contract-load → surface-scan → finding-report pipeline.
#![allow(clippy::unwrap_used)]

use rhino_cli::commands::env_validate::{EnvValidateArgs, run_at_root};
use std::fs;
use tempfile::TempDir;

fn write(dir: &TempDir, rel: &str, content: &str) {
    let p = dir.path().join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, content).unwrap();
}

/// Write a minimal `env-injection.yaml` consistent with a single `apps/myapp`
/// `kind: app` surface, so the manifest-consistency pass stays clean and the
/// drift-focused assertions below isolate the behavior under test.
fn write_consistent_manifest(dir: &TempDir) {
    write(
        dir,
        "env-injection.yaml",
        "apps:\n  - app: myapp\n    keys-from: apps/myapp/.env.example\n    runtime: { local: env-local }\nci-harness: []\n",
    );
}

const ARGS_NO_WARN: EnvValidateArgs = EnvValidateArgs { warn_only: false };

// ── matching surface ─────────────────────────────────────────────────────────

#[test]
fn integration_matching_typescript_surface_exits_clean() {
    let tmp = TempDir::new().unwrap();
    write_consistent_manifest(&tmp);

    write(
        &tmp,
        "env-contract.yaml",
        "surfaces:\n  - root: apps/myapp\n    kind: app\n    lang: typescript\n    allowlist: []\n",
    );
    write(&tmp, "apps/myapp/.env.example", "MY_KEY=value\n");
    write(
        &tmp,
        "apps/myapp/src/env.ts",
        "export const env = createEnv({\n  server: {\n    MY_KEY: z.string(),\n  },\n  experimental__runtimeEnv: {},\n});\n",
    );

    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run_at_root(tmp.path(), &ARGS_NO_WARN, &mut out, &mut err);
    assert!(
        result.is_ok(),
        "expected clean exit; got: {}",
        String::from_utf8_lossy(&err)
    );
    assert!(String::from_utf8_lossy(&out).contains("no drift detected"));
}

#[test]
fn integration_matching_rust_surface_exits_clean() {
    let tmp = TempDir::new().unwrap();
    write_consistent_manifest(&tmp);

    write(
        &tmp,
        "env-contract.yaml",
        "surfaces:\n  - root: apps/myapp\n    kind: app\n    lang: rust\n    allowlist: []\n",
    );
    write(
        &tmp,
        "apps/myapp/.env.example",
        "DATABASE_URL=postgres://localhost/db\nAPP_PORT=8080\n",
    );
    write(
        &tmp,
        "apps/myapp/src/config.rs",
        "#[derive(Deserialize)]\npub struct Config {\n    pub database_url: String,\n    pub app_port: u16,\n}\n",
    );

    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run_at_root(tmp.path(), &ARGS_NO_WARN, &mut out, &mut err);
    assert!(
        result.is_ok(),
        "expected clean exit; got: {}",
        String::from_utf8_lossy(&err)
    );
}

// ── seeded mismatch ──────────────────────────────────────────────────────────

#[test]
fn integration_declared_but_unread_exits_nonzero_and_names_key() {
    let tmp = TempDir::new().unwrap();
    write_consistent_manifest(&tmp);

    write(
        &tmp,
        "env-contract.yaml",
        "surfaces:\n  - root: apps/myapp\n    kind: app\n    lang: typescript\n    allowlist: []\n",
    );
    // STALE_KEY declared in .env.example but absent from code
    write(&tmp, "apps/myapp/.env.example", "STALE_KEY=value\n");
    write(
        &tmp,
        "apps/myapp/src/env.ts",
        "export const env = createEnv({ server: {}, experimental__runtimeEnv: {} });\n",
    );

    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run_at_root(tmp.path(), &ARGS_NO_WARN, &mut out, &mut err);
    assert!(result.is_err(), "expected non-zero exit on drift");
    let stderr = String::from_utf8_lossy(&err);
    assert!(
        stderr.contains("STALE_KEY"),
        "expected STALE_KEY in stderr; got: {stderr}"
    );
    assert!(
        stderr.contains("declared-but-unread"),
        "expected drift kind in stderr; got: {stderr}"
    );
}

#[test]
fn integration_read_but_undeclared_exits_nonzero_and_names_key() {
    let tmp = TempDir::new().unwrap();
    write_consistent_manifest(&tmp);

    write(
        &tmp,
        "env-contract.yaml",
        "surfaces:\n  - root: apps/myapp\n    kind: app\n    lang: typescript\n    allowlist: []\n",
    );
    // .env.example is empty; code reads NEW_KEY
    write(&tmp, "apps/myapp/.env.example", "");
    write(
        &tmp,
        "apps/myapp/src/env.ts",
        "export const env = createEnv({\n  server: {\n    NEW_KEY: z.string(),\n  },\n  experimental__runtimeEnv: {},\n});\n",
    );

    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run_at_root(tmp.path(), &ARGS_NO_WARN, &mut out, &mut err);
    assert!(result.is_err(), "expected non-zero exit on drift");
    let stderr = String::from_utf8_lossy(&err);
    assert!(
        stderr.contains("NEW_KEY"),
        "expected NEW_KEY in stderr; got: {stderr}"
    );
    assert!(
        stderr.contains("read-but-undeclared"),
        "expected drift kind in stderr; got: {stderr}"
    );
}

// ── warn-only mode ────────────────────────────────────────────────────────────

#[test]
fn integration_warn_only_does_not_fail_on_drift() {
    let tmp = TempDir::new().unwrap();
    write_consistent_manifest(&tmp);

    write(
        &tmp,
        "env-contract.yaml",
        "surfaces:\n  - root: apps/myapp\n    kind: app\n    lang: typescript\n    allowlist: []\n",
    );
    write(&tmp, "apps/myapp/.env.example", "STALE_KEY=value\n");
    write(
        &tmp,
        "apps/myapp/src/env.ts",
        "export const env = createEnv({ server: {}, experimental__runtimeEnv: {} });\n",
    );

    let args = EnvValidateArgs { warn_only: true };
    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run_at_root(tmp.path(), &args, &mut out, &mut err);
    assert!(result.is_ok(), "warn-only should exit 0 even with drift");
    let stderr = String::from_utf8_lossy(&err);
    assert!(
        stderr.contains("warn-only"),
        "expected warn-only notice in stderr"
    );
}

// ── env-injection.yaml manifest-consistency pass (committed fixtures) ──────────

fn fixture_root(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/env-injection")
        .join(name)
}

#[test]
fn integration_matched_manifest_fixture_exits_clean() {
    let root = fixture_root("matched");
    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run_at_root(&root, &ARGS_NO_WARN, &mut out, &mut err);
    assert!(
        result.is_ok(),
        "expected clean exit on matched fixture; got: {}",
        String::from_utf8_lossy(&err)
    );
    assert!(
        String::from_utf8_lossy(&out).contains("env-injection.yaml consistent"),
        "expected consistency notice; got: {}",
        String::from_utf8_lossy(&out)
    );
}

#[test]
fn integration_mismatched_ci_leak_fixture_fails_and_names_key() {
    // The ci-harness key WEB_BASE_URL is wrongly declared in apps/demo-www/.env.example.
    let root = fixture_root("mismatched-ci-leak");
    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run_at_root(&root, &ARGS_NO_WARN, &mut out, &mut err);
    assert!(
        result.is_err(),
        "expected non-zero exit on manifest inconsistency"
    );
    let stderr = String::from_utf8_lossy(&err);
    assert!(
        stderr.contains("MANIFEST"),
        "expected MANIFEST finding line; got: {stderr}"
    );
    assert!(
        stderr.contains("ci-harness-key-leaks-into-app"),
        "expected leak problem label; got: {stderr}"
    );
    assert!(
        stderr.contains("WEB_BASE_URL"),
        "expected offending key named; got: {stderr}"
    );
    assert!(
        stderr.contains("demo-www"),
        "expected offending app named; got: {stderr}"
    );
}
