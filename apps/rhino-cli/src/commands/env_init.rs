// Port of `apps/rhino-cli/cmd/env_init.go`.

use std::fs;

use anyhow::{Error, anyhow};
use clap::Args;
use walkdir::WalkDir;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;

#[derive(Args, Debug)]
pub struct EnvInitArgs {
    /// Overwrite existing .env files.
    #[arg(long = "force")]
    pub force: bool,
}

pub fn run(args: &EnvInitArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let infra_dev = repo_root.join("infra/dev");
    let mut created = 0usize;
    let mut skipped = 0usize;
    let mut errs: Vec<String> = Vec::new();

    for entry in WalkDir::new(&infra_dev).into_iter().flatten() {
        if entry.file_type().is_dir() {
            continue;
        }
        if entry.file_name() != ".env.example" {
            continue;
        }
        let path = entry.path();
        let env_path = path
            .parent()
            .ok_or_else(|| anyhow!("invalid path"))?
            .join(".env");
        let rel = env_path.strip_prefix(&repo_root).unwrap_or(&env_path);
        if !args.force && env_path.exists() {
            println!(
                "Skipped: {} (already exists, use --force to overwrite)",
                rel.display()
            );
            skipped += 1;
            continue;
        }
        let data = match fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                errs.push(format!("failed to read {}: {e}", path.display()));
                continue;
            }
        };
        if let Err(e) = fs::write(&env_path, data) {
            errs.push(format!("failed to write {}: {e}", env_path.display()));
            continue;
        }
        println!(
            "Created: {} (from {})",
            rel.display(),
            path.file_name()
                .expect("walkdir entry always has file_name")
                .to_string_lossy()
        );
        created += 1;
    }

    println!("\nSummary: {created} created, {skipped} skipped");
    for e in &errs {
        eprintln!("Error: {e}");
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn args_default() {
        let _ = EnvInitArgs { force: false };
    }
}
