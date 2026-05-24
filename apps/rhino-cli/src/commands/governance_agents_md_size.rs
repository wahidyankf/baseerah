// Port of `apps/rhino-cli/cmd/governance_agents_md_size.go`.
// Text/JSON/Markdown output formats matching Go byte-for-byte.

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::repo_governance::agents_md_size::{AgentsMdSizeFinding, check_agents_md_size};

const SCHEMA: &str = "rhino-cli/agents-md-size/v1";

#[derive(Args, Debug)]
pub struct AgentsMdSizeArgs;

#[derive(Serialize)]
struct ResultPayload<'a> {
    file: &'a str,
    size: i64,
    severity: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
struct Envelope<'a> {
    schema: &'a str,
    status: &'a str,
    result: ResultPayload<'a>,
}

pub fn run(
    _args: &AgentsMdSizeArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let path = repo_root.join("AGENTS.md");
    let path_str = path.to_string_lossy().to_string();
    let finding = check_agents_md_size(&path_str).context("agents-md-size audit failed")?;

    match output_format {
        OutputFormat::Text => print_text(&finding),
        OutputFormat::Json => print_json(&finding)?,
        OutputFormat::Markdown => print_markdown(&finding),
    }

    if finding.severity == "fail" {
        return Err(anyhow!(
            "AGENTS.md exceeds hard limit ({} bytes)",
            finding.size
        ));
    }
    Ok(())
}

fn print_text(f: &AgentsMdSizeFinding) {
    println!(
        "AGENTS.MD SIZE: {} — {}",
        status_label(&f.severity),
        f.message
    );
}

fn print_json(f: &AgentsMdSizeFinding) -> std::result::Result<(), Error> {
    let env = Envelope {
        schema: SCHEMA,
        status: &f.severity,
        result: ResultPayload {
            file: &f.file,
            size: f.size,
            severity: &f.severity,
            message: &f.message,
        },
    };
    let s = serde_json::to_string_pretty(&env)?;
    println!("{s}");
    Ok(())
}

fn print_markdown(f: &AgentsMdSizeFinding) {
    println!(
        "## AGENTS.md Size Audit\n\n**Status**: {}\n\n| File | Size (bytes) | Severity | Message |\n|------|--------------|----------|---------|\n| {} | {} | {} | {} |",
        status_label(&f.severity),
        f.file,
        f.size,
        f.severity,
        f.message
    );
}

pub(crate) fn status_label(severity: &str) -> &str {
    match severity {
        "ok" => "PASS",
        "warn" => "WARN",
        "fail" => "FAIL",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_label_maps_severity_to_label() {
        assert_eq!(status_label("ok"), "PASS");
        assert_eq!(status_label("warn"), "WARN");
        assert_eq!(status_label("fail"), "FAIL");
        assert_eq!(status_label("other"), "other");
    }
}
