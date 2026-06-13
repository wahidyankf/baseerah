//! `specs clean java-imports` — strips unused and same-package imports from
//! generated Java contract files, cleaning codegen output.
//!
//! **Dormant in ose-public** — no generated Java contracts here yet. Ships for
//! union CLI parity; activates when Java contract codegen lands.

use anyhow::Error;
use clap::Args;

use crate::domain::cliout::OutputFormat;

/// CLI arguments for `specs clean java-imports`.
#[derive(Args, Debug)]
pub struct CleanJavaImportsArgs {
    /// Root directory to scan for generated Java files.
    #[arg(long, value_name = "dir")]
    pub dir: Option<String>,
}

/// Run the `specs clean java-imports` command.
///
/// # Errors
///
/// Always succeeds in ose-public (no generated Java contracts to clean).
pub fn run(args: &CleanJavaImportsArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let dir = args.dir.as_deref().unwrap_or(".");
    println!(
        "specs clean java-imports: dormant in ose-public \
         (no generated Java contracts under {dir}); pass."
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::cliout::OutputFormat;

    #[test]
    fn args_constructible() {
        let a = CleanJavaImportsArgs { dir: None };
        assert!(a.dir.is_none());
    }

    #[test]
    fn run_dormant_succeeds() {
        let args = CleanJavaImportsArgs { dir: None };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }

    #[test]
    fn run_with_dir_succeeds() {
        let args = CleanJavaImportsArgs {
            dir: Some("specs".to_string()),
        };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }
}
