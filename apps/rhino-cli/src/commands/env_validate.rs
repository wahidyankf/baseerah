//! `env validate` — check `env-contract.yaml` surfaces for code↔config drift.
//!
//! # ENV-VALIDATE CONFIG: `env-contract.yaml` at repo root, parsed with `serde_norway`.
//! Each surface entry carries `root`, `kind`, `lang`, and `allowlist`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::envinjection;
use crate::internal::envvalidate;
use crate::internal::git;

/// CLI arguments for `env validate`.
#[derive(Args, Debug)]
pub struct EnvValidateArgs {
    /// Exit 0 even when drift is detected (report only; no gate enforcement).
    #[arg(long = "warn-only")]
    pub warn_only: bool,
}

/// Run the `env validate` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, `env-contract.yaml` cannot
/// be read, or any surface validation fails hard (missing file, unsupported lang).
pub fn run(args: &EnvValidateArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_at_root(
        &repo_root,
        args,
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    )
}

/// Run `env validate` from a known `repo_root` (testable entry point).
///
/// # Errors
///
/// Returns an error when contract loading or surface validation fails.
pub fn run_at_root(
    repo_root: &std::path::Path,
    args: &EnvValidateArgs,
    stdout: &mut dyn std::io::Write,
    stderr: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let contract = envvalidate::load_contract(repo_root)?;
    let findings = envvalidate::validate_all(repo_root, &contract)?;

    // Manifest-consistency pass: static, value-free check of env-injection.yaml
    // against env-contract.yaml and the apps' .env.example files.
    let manifest_findings = envinjection::validate_manifest(repo_root, &contract)?;

    let total = findings.len() + manifest_findings.len();

    if total == 0 {
        writeln!(
            stdout,
            "env validate: no drift detected across all surfaces; env-injection manifest consistent"
        )?;
        return Ok(());
    }

    for f in &findings {
        writeln!(
            stderr,
            "DRIFT  {}  {}  {}",
            f.root.display(),
            f.drift.label(),
            f.key
        )?;
    }

    for f in &manifest_findings {
        writeln!(
            stderr,
            "MANIFEST  {}  {}  {}",
            f.problem.label(),
            f.subject,
            f.detail
        )?;
    }

    if args.warn_only {
        writeln!(
            stderr,
            "env validate: {total} finding(s) — warn-only mode, not failing"
        )?;
        return Ok(());
    }

    Err(anyhow!(
        "env validate: {total} finding(s); fix the divergent keys/manifest entries listed above"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_constructible() {
        let _ = EnvValidateArgs { warn_only: false };
    }
}
