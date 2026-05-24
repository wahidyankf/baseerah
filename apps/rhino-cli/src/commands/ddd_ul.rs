// Port of `apps/rhino-cli/cmd/ddd_ul.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::glossary;
use crate::internal::severity::{Severity, resolve};

#[derive(Args, Debug)]
pub struct DddUlArgs {
    #[arg(value_name = "app")]
    pub app: String,
    #[arg(long = "severity", default_value = "")]
    pub severity: String,
}

pub fn run(args: &DddUlArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
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
    args: &DddUlArgs,
    stdout: &mut dyn std::io::Write,
    stderr: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let env = std::env::var("OSE_RHINO_DDD_SEVERITY").unwrap_or_default();
    let sev = resolve(&args.severity, &env, stderr);
    let findings = glossary::validate_all(&glossary::ValidateOptions {
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
        return Err(anyhow!("{err_count} error finding(s) found by ddd ul"));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn args_constructible() {
        let a = DddUlArgs {
            app: "x".to_string(),
            severity: String::new(),
        };
        assert_eq!(a.app, "x");
    }

    #[test]
    fn run_at_root_missing_registry_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut out: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        let args = DddUlArgs {
            app: "missing".to_string(),
            severity: String::new(),
        };
        assert!(run_at_root(dir.path(), &args, &mut out, &mut err).is_err());
    }
}
