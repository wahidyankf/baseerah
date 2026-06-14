//! `links` subcommand implementations for `ayokoding-cli`.
//!
//! Provides the `links check` subcommand that validates internal links
//! in `ayokoding-www` markdown content.
use std::path::Path;
use std::time::Instant;

use clap::Args;
use rust_commons::links;

/// Arguments for the `links check` subcommand.
#[derive(Debug, Args)]
pub struct LinksCheckArgs {
    /// Path to the content directory to scan for internal links.
    #[arg(long, default_value = "apps/ayokoding-www/content")]
    pub content: String,
}

/// Run the `links check` subcommand.
///
/// Walks the content directory specified in `args`, checks all internal
/// markdown links, and prints results in the requested `output_format`.
///
/// # Errors
///
/// Returns an error if the content directory cannot be read, or if broken
/// links are found (which causes a non-zero exit via the caller).
pub fn run_links_check(
    args: &LinksCheckArgs,
    output_format: &str,
    quiet: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    if !quiet && output_format == "text" {
        println!("Checking internal links in: {}", args.content);
        println!("---");
    }

    let start = Instant::now();
    let result = links::check_links(Path::new(&args.content))?;
    let elapsed = start.elapsed();

    match output_format {
        "json" => {
            let json = links::output_links_json(&result, elapsed)?;
            println!("{json}");
        }
        "markdown" => {
            links::output_links_markdown(&result, elapsed);
        }
        _ => {
            links::output_links_text(&result, elapsed, quiet, verbose);
        }
    }

    if !result.broken_links.is_empty() {
        return Err(anyhow::anyhow!(
            "{} broken link(s) found",
            result.broken_links.len()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use tempfile::TempDir;

    use super::{LinksCheckArgs, run_links_check};

    fn make_args(dir: &TempDir) -> LinksCheckArgs {
        LinksCheckArgs {
            content: dir.path().to_string_lossy().into_owned(),
        }
    }

    #[test]
    fn test_run_links_check_text_empty_dir() {
        let dir = TempDir::new().expect("create tempdir");
        let args = make_args(&dir);
        run_links_check(&args, "text", false, false).expect("should succeed on empty dir");
    }

    #[test]
    fn test_run_links_check_json_empty_dir() {
        let dir = TempDir::new().expect("create tempdir");
        let args = make_args(&dir);
        run_links_check(&args, "json", false, false).expect("should succeed on empty dir");
    }

    #[test]
    fn test_run_links_check_markdown_empty_dir() {
        let dir = TempDir::new().expect("create tempdir");
        let args = make_args(&dir);
        run_links_check(&args, "markdown", false, false).expect("should succeed on empty dir");
    }

    #[test]
    fn test_run_links_check_quiet_mode() {
        let dir = TempDir::new().expect("create tempdir");
        let args = make_args(&dir);
        run_links_check(&args, "text", true, false).expect("should succeed in quiet mode");
    }

    #[test]
    fn test_run_links_check_verbose_mode() {
        let dir = TempDir::new().expect("create tempdir");
        let args = make_args(&dir);
        run_links_check(&args, "text", false, true).expect("should succeed in verbose mode");
    }

    #[test]
    fn test_run_links_check_broken_link_returns_err() {
        let dir = TempDir::new().expect("create tempdir");
        let md_path = dir.path().join("page.md");
        let mut file = std::fs::File::create(&md_path).expect("create md file");
        writeln!(file, "[broken link](/nonexistent-target)").expect("write md content");
        let args = make_args(&dir);
        let result = run_links_check(&args, "text", false, false);
        assert!(
            result.is_err(),
            "expected error when broken links are found"
        );
    }

    #[test]
    fn test_run_links_check_nonexistent_dir_returns_err() {
        let args = LinksCheckArgs {
            content: "/nonexistent/xyz/does-not-exist-ayokoding-cli-test".to_owned(),
        };
        let result = run_links_check(&args, "text", false, false);
        assert!(
            result.is_err(),
            "expected error for nonexistent content directory"
        );
    }

    #[test]
    fn test_run_links_check_broken_link_json_returns_err() {
        let dir = TempDir::new().expect("create tempdir");
        let md_path = dir.path().join("page.md");
        let mut file = std::fs::File::create(&md_path).expect("create md file");
        writeln!(file, "[broken link](/nonexistent-target)").expect("write md content");
        let args = make_args(&dir);
        let result = run_links_check(&args, "json", false, false);
        assert!(
            result.is_err(),
            "expected error when broken links are found in json mode"
        );
    }

    #[test]
    fn test_run_links_check_broken_link_markdown_returns_err() {
        let dir = TempDir::new().expect("create tempdir");
        let md_path = dir.path().join("page.md");
        let mut file = std::fs::File::create(&md_path).expect("create md file");
        writeln!(file, "[broken link](/nonexistent-target)").expect("write md content");
        let args = make_args(&dir);
        let result = run_links_check(&args, "markdown", false, false);
        assert!(
            result.is_err(),
            "expected error when broken links are found in markdown mode"
        );
    }
}
