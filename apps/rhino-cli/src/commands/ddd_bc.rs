// Port of `apps/rhino-cli/cmd/ddd_bc.go` + `ddd_runner.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::internal::bcregistry;
use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::severity::{Severity, resolve};

#[derive(Args, Debug)]
pub struct DddBcArgs {
    #[arg(value_name = "app")]
    pub app: String,
    /// Override finding severity: warn|error.
    #[arg(long = "severity", default_value = "")]
    pub severity: String,
}

pub fn run(args: &DddBcArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_at_root(
        &repo_root,
        args,
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    )
}

pub fn run_at_root(
    repo_root: &std::path::Path,
    args: &DddBcArgs,
    stdout: &mut dyn std::io::Write,
    stderr: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let env = std::env::var("OSE_RHINO_DDD_SEVERITY").unwrap_or_default();
    let sev = resolve(&args.severity, &env, stderr);
    let findings = bcregistry::validate_all(&bcregistry::ValidateOptions {
        repo_root: repo_root.to_path_buf(),
        app: args.app.clone(),
        severity: Some(sev),
    })?;
    for f in &findings {
        writeln!(stdout, "{}: {}: {}", f.file, f.severity.code(), f.message)?;
    }
    let err_count = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    if err_count > 0 {
        return Err(anyhow!("{err_count} error finding(s) found by ddd bc"));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn args_constructible() {
        let a = DddBcArgs {
            app: "x".to_string(),
            severity: "warn".to_string(),
        };
        assert_eq!(a.app, "x");
        assert_eq!(a.severity, "warn");
    }

    #[test]
    fn run_at_root_missing_registry_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut out: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        let args = DddBcArgs {
            app: "missing".to_string(),
            severity: String::new(),
        };
        assert!(run_at_root(dir.path(), &args, &mut out, &mut err).is_err());
    }
}
