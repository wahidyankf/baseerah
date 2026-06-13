//! `specs scaffold dart` — generates Dart package scaffolding (pubspec, lib
//! layout) around generated contract types.
//!
//! **Dormant in ose-public** — no generated Dart contracts here yet. Ships for
//! union CLI parity; activates when Dart contract codegen lands.

use anyhow::Error;
use clap::Args;

use crate::domain::cliout::OutputFormat;

/// CLI arguments for `specs scaffold dart`.
#[derive(Args, Debug)]
pub struct ScaffoldDartArgs {
    /// Root directory for the generated Dart package.
    #[arg(long, value_name = "dir")]
    pub dir: Option<String>,
}

/// Run the `specs scaffold dart` command.
///
/// # Errors
///
/// Always succeeds in ose-public (no Dart contract source to scaffold).
pub fn run(args: &ScaffoldDartArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let dir = args.dir.as_deref().unwrap_or(".");
    println!(
        "specs scaffold dart: dormant in ose-public \
         (no Dart contract source under {dir}); pass."
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::cliout::OutputFormat;

    #[test]
    fn args_constructible() {
        let a = ScaffoldDartArgs { dir: None };
        assert!(a.dir.is_none());
    }

    #[test]
    fn run_dormant_succeeds() {
        let args = ScaffoldDartArgs { dir: None };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }

    #[test]
    fn run_with_dir_succeeds() {
        let args = ScaffoldDartArgs {
            dir: Some("specs".to_string()),
        };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }
}
