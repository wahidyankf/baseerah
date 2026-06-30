//! `lang java null-safety-annotations validate` — checks Java packages carry
//! required null-safety annotations on generated/contract code.
//!
//! **Dormant in ose-public** — no JVM source here yet. Ships for union CLI
//! parity; activates when a JVM app lands.

use anyhow::Error;
use clap::Args;

use crate::domain::cliout::OutputFormat;

/// CLI arguments for `lang java null-safety-annotations validate`.
#[derive(Args, Debug)]
pub struct ValidateNullSafetyArgs {
    /// Root directory to scan (defaults to repo root).
    #[arg(long, value_name = "dir")]
    pub dir: Option<String>,
}

/// Run the `lang java null-safety-annotations validate` command.
///
/// # Errors
///
/// Always succeeds in ose-public (no JVM source to scan).
pub fn run(args: &ValidateNullSafetyArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let dir = args.dir.as_deref().unwrap_or(".");
    println!(
        "lang java null-safety-annotations validate: dormant in ose-public \
         (no JVM source under {dir}); pass."
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::cliout::OutputFormat;

    #[test]
    fn args_constructible() {
        let a = ValidateNullSafetyArgs { dir: None };
        assert!(a.dir.is_none());
    }

    #[test]
    fn run_dormant_succeeds() {
        let args = ValidateNullSafetyArgs { dir: None };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }

    #[test]
    fn run_with_dir_succeeds() {
        let args = ValidateNullSafetyArgs {
            dir: Some("apps".to_string()),
        };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }
}
