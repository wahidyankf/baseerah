//! CLI entry point: argument parsing, subcommand dispatch, and help output.

use clap::{Parser, Subcommand};

use crate::commands::{
    agents_detect_duplication, agents_emit_bindings, agents_sync, agents_validate_bindings,
    agents_validate_claude, agents_validate_naming, agents_validate_sync, ddd_bc, ddd_ul,
    docs_validate_frontmatter, docs_validate_heading_hierarchy, docs_validate_links,
    docs_validate_mermaid, docs_validate_naming, doctor, env_backup, env_init, env_restore,
    git_pre_commit, governance_agents_md_size, governance_audit, governance_emoji_audit,
    governance_frontmatter_audit, governance_layer_coherence, governance_license_audit,
    governance_readme_index_audit, governance_traceability_audit, governance_vendor_audit,
    spec_coverage_validate, specs_validate_adoption, specs_validate_counts, specs_validate_links,
    specs_validate_tree, test_coverage_diff, test_coverage_merge, test_coverage_validate,
    workflows_validate_naming,
};
use crate::internal::cliout::OutputFormat;

#[derive(Parser, Debug)]
#[command(
    name = "rhino-cli",
    version = "0.16.1",
    about = "CLI tools for repository management",
    long_about = "Command-line tools for repository management and automation.",
    disable_help_flag = true
)]
/// Root CLI arguments shared across all subcommands.
pub struct Cli {
    /// Enable verbose output with timestamps.
    #[arg(
        long,
        short = 'v',
        global = true,
        help = "verbose output with timestamps"
    )]
    pub verbose: bool,

    /// Suppress all output except errors.
    #[arg(long, short = 'q', global = true, help = "quiet mode (errors only)")]
    pub quiet: bool,

    /// Output format: `text`, `json`, or `markdown`.
    #[arg(
        long,
        short = 'o',
        global = true,
        default_value = "text",
        help = "output format: text, json, markdown"
    )]
    pub output: String,

    /// Disable ANSI color codes in output.
    #[arg(long = "no-color", global = true, help = "disable colored output")]
    pub no_color: bool,

    /// Echo a literal message to stdout and exit.
    #[arg(
        long,
        global = true,
        default_value = "",
        help = "echo a message to stdout"
    )]
    pub say: String,

    /// Print help and exit.
    #[arg(long, short = 'h', global = true, help = "Print help")]
    pub help: bool,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Top-level CLI subcommands dispatched by the root `Cli` parser.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Test coverage commands (validate, diff, merge).
    #[command(name = "test-coverage", subcommand)]
    TestCoverage(TestCoverageCommands),
    /// BDD spec coverage commands.
    #[command(name = "spec-coverage", subcommand)]
    SpecCoverage(SpecCoverageCommands),
    /// Repository governance audits.
    #[command(name = "repo-governance", subcommand)]
    RepoGovernance(RepoGovernanceCommands),
    /// Documentation validators.
    #[command(name = "docs", subcommand)]
    Docs(DocsCommands),
    /// Agent definition validators.
    #[command(name = "agents", subcommand)]
    Agents(AgentsCommands),
    /// Workflow file validators.
    #[command(name = "workflows", subcommand)]
    Workflows(WorkflowsCommands),
    /// Spec tree validators (adoption, counts, links, tree).
    #[command(name = "specs", subcommand)]
    Specs(SpecsCommands),
    /// DDD validators (bounded-context, ubiquitous-language).
    #[command(name = "ddd", subcommand)]
    Ddd(DddCommands),
    /// Git hook helpers (pre-commit).
    #[command(name = "git", subcommand)]
    Git(GitCommands),
    /// Environment file helpers (init, backup, restore).
    #[command(name = "env", subcommand)]
    Env(EnvCommands),
    /// Check required tool versions are installed and correct.
    #[command(name = "doctor")]
    Doctor(doctor::DoctorArgs),
}

/// Git hook helper subcommands.
#[derive(Subcommand, Debug)]
pub enum GitCommands {
    /// Run all pre-commit checks (config, lint, format, docs).
    #[command(name = "pre-commit")]
    PreCommit(git_pre_commit::PreCommitArgs),
}

/// Environment file subcommands (`init`, `backup`, `restore`).
#[derive(Subcommand, Debug)]
pub enum EnvCommands {
    /// Create .env files from .env.example templates.
    #[command(name = "init")]
    Init(env_init::EnvInitArgs),
    /// Back up .env files from the repository.
    #[command(name = "backup")]
    Backup(env_backup::EnvBackupArgs),
    /// Restore .env files from a backup.
    #[command(name = "restore")]
    Restore(env_restore::EnvRestoreArgs),
}

/// Spec tree subcommands (`validate-adoption`, `validate-counts`, `validate-links`, `validate-tree`).
#[derive(Subcommand, Debug)]
pub enum SpecsCommands {
    /// Verify an app has adopted BDD and DDD practices.
    #[command(name = "validate-adoption")]
    ValidateAdoption(specs_validate_adoption::ValidateAdoptionArgs),
    /// Validate each required spec subfolder contains at least one spec file.
    #[command(name = "validate-counts")]
    ValidateCounts(specs_validate_counts::ValidateCountsArgs),
    /// Check that markdown links in spec files resolve.
    #[command(name = "validate-links")]
    ValidateLinks(specs_validate_links::ValidateLinksArgs),
    /// Validate canonical C4-aware five-folder spec tree.
    #[command(name = "validate-tree")]
    ValidateTree(specs_validate_tree::ValidateTreeArgs),
}

/// DDD subcommands (`bc`, `ul`).
#[derive(Subcommand, Debug)]
pub enum DddCommands {
    /// Validate bounded-context structural parity against the registry.
    #[command(name = "bc")]
    Bc(ddd_bc::DddBcArgs),
    /// Validate ubiquitous-language glossary parity against the registry.
    #[command(name = "ul")]
    Ul(ddd_ul::DddUlArgs),
}

/// Agent definition subcommands.
#[derive(Subcommand, Debug)]
pub enum AgentsCommands {
    /// Validate agent filename suffixes and mirror parity.
    #[command(name = "validate-naming")]
    ValidateNaming(agents_validate_naming::ValidateNamingArgs),
    /// Detect verbatim duplication across agent and skill files.
    #[command(name = "detect-duplication")]
    DetectDuplication(agents_detect_duplication::DetectDuplicationArgs),
    /// Validate Claude Code agent and skill format in .claude/ directory.
    #[command(name = "validate-claude")]
    ValidateClaude(agents_validate_claude::ValidateClaudeArgs),
    /// Validate that .claude/ and .opencode/ are in sync.
    #[command(name = "validate-sync")]
    ValidateSync(agents_validate_sync::ValidateSyncArgs),
    /// Sync Claude Code agents to `OpenCode` format.
    #[command(name = "sync")]
    Sync(agents_sync::SyncArgs),
    /// Emit the Amazon Q Developer binding bridge files (idempotent).
    #[command(name = "emit-bindings")]
    EmitBindings(agents_emit_bindings::EmitBindingsArgs),
    /// Validate the Amazon Q binding bridge files and catalog coverage.
    #[command(name = "validate-bindings")]
    ValidateBindings(agents_validate_bindings::ValidateBindingsArgs),
}

/// Workflow file subcommands.
#[derive(Subcommand, Debug)]
pub enum WorkflowsCommands {
    /// Validate workflow filename suffixes and frontmatter name consistency.
    #[command(name = "validate-naming")]
    ValidateNaming(workflows_validate_naming::ValidateNamingArgs),
}

/// Documentation validator subcommands.
#[derive(Subcommand, Debug)]
pub enum DocsCommands {
    /// Validate markdown filenames against the lowercase-kebab-case rule.
    #[command(name = "validate-naming")]
    ValidateNaming(docs_validate_naming::ValidateNamingArgs),
    /// Validate documentation YAML frontmatter against area-specific schemas.
    #[command(name = "validate-frontmatter")]
    ValidateFrontmatter(docs_validate_frontmatter::ValidateFrontmatterArgs),
    /// Validate markdown heading hierarchy (one H1, no skipped levels).
    #[command(name = "validate-heading-hierarchy")]
    ValidateHeadingHierarchy(docs_validate_heading_hierarchy::ValidateHeadingHierarchyArgs),
    /// Validate markdown links (relative paths exist on disk).
    #[command(name = "validate-links")]
    ValidateLinks(docs_validate_links::ValidateLinksArgs),
    /// Validate Mermaid flowchart diagrams (label length, span, single-diagram).
    #[command(name = "validate-mermaid")]
    ValidateMermaid(docs_validate_mermaid::ValidateMermaidArgs),
}

/// Repository governance subcommands.
#[derive(Subcommand, Debug)]
pub enum RepoGovernanceCommands {
    /// Audit AGENTS.md size against the 30/35/40 KB thresholds.
    #[command(name = "agents-md-size")]
    AgentsMdSize(governance_agents_md_size::AgentsMdSizeArgs),
    /// Run all 11 deterministic governance audits and emit a JSON envelope.
    #[command(name = "audit")]
    Audit(governance_audit::AuditArgs),
    /// Audit forbidden file types for emoji codepoints.
    #[command(name = "emoji-audit")]
    EmojiAudit(governance_emoji_audit::EmojiAuditArgs),
    /// Audit markdown files for forbidden manual date metadata.
    #[command(name = "frontmatter-audit")]
    FrontmatterAudit(governance_frontmatter_audit::FrontmatterAuditArgs),
    /// Audit governance docs for layer numbering/naming coherence.
    #[command(name = "layer-coherence")]
    LayerCoherence(governance_layer_coherence::LayerCoherenceArgs),
    /// Verify per-directory LICENSE files match the licensing convention.
    #[command(name = "license-audit")]
    LicenseAudit(governance_license_audit::LicenseAuditArgs),
    /// Audit directory README.md indexes against sibling markdown files.
    #[command(name = "readme-index-audit")]
    ReadmeIndexAudit(governance_readme_index_audit::ReadmeIndexAuditArgs),
    /// Audit governance documents for required traceability sections.
    #[command(name = "traceability-audit")]
    TraceabilityAudit(governance_traceability_audit::TraceabilityAuditArgs),
    /// Scan governance markdown for forbidden vendor-specific terms.
    #[command(name = "vendor-audit")]
    VendorAudit(governance_vendor_audit::VendorAuditArgs),
}

/// Test coverage subcommands (`validate`, `diff`, `merge`).
#[derive(Subcommand, Debug)]
pub enum TestCoverageCommands {
    /// Check test coverage against a threshold (standard line-based algorithm).
    Validate(test_coverage_validate::ValidateArgs),
    /// Show coverage for changed lines only (diff coverage).
    Diff(test_coverage_diff::DiffArgs),
    /// Merge multiple coverage files into one LCOV output.
    Merge(test_coverage_merge::MergeArgs),
}

/// BDD spec coverage subcommands.
#[derive(Subcommand, Debug)]
pub enum SpecCoverageCommands {
    /// Validate that all BDD spec files have matching test implementations.
    Validate(spec_coverage_validate::ValidateArgs),
}

/// Parse CLI arguments and dispatch to the appropriate subcommand.
///
/// Returns the process exit code: `0` on success, `1` on command error, `2` on
/// argument parse error.
///
/// # Errors
///
/// Returns a non-zero exit code when the selected subcommand reports an error.
pub fn run() -> i32 {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            e.print().ok();
            return 2;
        }
    };

    let output_format = match OutputFormat::parse(&cli.output) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error: {err}");
            return 1;
        }
    };

    if cli.help {
        return print_help_and_exit();
    }

    if let Some(cmd) = &cli.command {
        return dispatch(cmd, output_format);
    }

    if !cli.say.is_empty() {
        println!("{}", cli.say);
        return 0;
    }

    print_help_and_exit()
}

/// Route a top-level [`Commands`] variant to its subcommand handler.
///
/// Returns `0` on success or `1` on error.
fn dispatch(cmd: &Commands, output_format: OutputFormat) -> i32 {
    let result = match cmd {
        Commands::TestCoverage(tc) => match tc {
            TestCoverageCommands::Validate(args) => {
                test_coverage_validate::run(args, output_format)
            }
            TestCoverageCommands::Diff(args) => test_coverage_diff::run(args, output_format),
            TestCoverageCommands::Merge(args) => test_coverage_merge::run(args, output_format),
        },
        Commands::SpecCoverage(sc) => match sc {
            SpecCoverageCommands::Validate(args) => {
                spec_coverage_validate::run(args, output_format)
            }
        },
        Commands::RepoGovernance(rg) => match rg {
            RepoGovernanceCommands::AgentsMdSize(args) => {
                governance_agents_md_size::run(args, output_format)
            }
            RepoGovernanceCommands::Audit(args) => governance_audit::run(args, output_format),
            RepoGovernanceCommands::EmojiAudit(args) => {
                governance_emoji_audit::run(args, output_format)
            }
            RepoGovernanceCommands::FrontmatterAudit(args) => {
                governance_frontmatter_audit::run(args, output_format)
            }
            RepoGovernanceCommands::LayerCoherence(args) => {
                governance_layer_coherence::run(args, output_format)
            }
            RepoGovernanceCommands::LicenseAudit(args) => {
                governance_license_audit::run(args, output_format)
            }
            RepoGovernanceCommands::ReadmeIndexAudit(args) => {
                governance_readme_index_audit::run(args, output_format)
            }
            RepoGovernanceCommands::TraceabilityAudit(args) => {
                governance_traceability_audit::run(args, output_format)
            }
            RepoGovernanceCommands::VendorAudit(args) => {
                governance_vendor_audit::run(args, output_format)
            }
        },
        Commands::Docs(dc) => match dc {
            DocsCommands::ValidateNaming(args) => docs_validate_naming::run(args, output_format),
            DocsCommands::ValidateFrontmatter(args) => {
                docs_validate_frontmatter::run(args, output_format)
            }
            DocsCommands::ValidateHeadingHierarchy(args) => {
                docs_validate_heading_hierarchy::run(args, output_format)
            }
            DocsCommands::ValidateLinks(args) => docs_validate_links::run(args, output_format),
            DocsCommands::ValidateMermaid(args) => docs_validate_mermaid::run(args, output_format),
        },
        Commands::Agents(ac) => dispatch_agents(ac, output_format),
        Commands::Workflows(wc) => match wc {
            WorkflowsCommands::ValidateNaming(args) => {
                workflows_validate_naming::run(args, output_format)
            }
        },
        Commands::Specs(sc) => match sc {
            SpecsCommands::ValidateAdoption(args) => {
                specs_validate_adoption::run(args, output_format)
            }
            SpecsCommands::ValidateCounts(args) => specs_validate_counts::run(args, output_format),
            SpecsCommands::ValidateLinks(args) => specs_validate_links::run(args, output_format),
            SpecsCommands::ValidateTree(args) => specs_validate_tree::run(args, output_format),
        },
        Commands::Ddd(dc) => match dc {
            DddCommands::Bc(args) => ddd_bc::run(args, output_format),
            DddCommands::Ul(args) => ddd_ul::run(args, output_format),
        },
        Commands::Git(gc) => match gc {
            GitCommands::PreCommit(args) => git_pre_commit::run_cmd(args, output_format),
        },
        Commands::Env(ec) => match ec {
            EnvCommands::Init(args) => env_init::run(args, output_format),
            EnvCommands::Backup(args) => env_backup::run(args, output_format),
            EnvCommands::Restore(args) => env_restore::run(args, output_format),
        },
        Commands::Doctor(args) => doctor::run(args, output_format),
    };
    match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

/// Route an [`AgentsCommands`] variant to its handler.
///
/// # Errors
///
/// Propagates any error returned by the selected agent subcommand.
fn dispatch_agents(
    ac: &AgentsCommands,
    output_format: OutputFormat,
) -> std::result::Result<(), anyhow::Error> {
    match ac {
        AgentsCommands::ValidateNaming(args) => agents_validate_naming::run(args, output_format),
        AgentsCommands::DetectDuplication(args) => {
            agents_detect_duplication::run(args, output_format)
        }
        AgentsCommands::ValidateClaude(args) => agents_validate_claude::run(args, output_format),
        AgentsCommands::ValidateSync(args) => agents_validate_sync::run(args, output_format),
        AgentsCommands::Sync(args) => agents_sync::run(args, output_format),
        AgentsCommands::EmitBindings(args) => agents_emit_bindings::run(args, output_format),
        AgentsCommands::ValidateBindings(args) => {
            agents_validate_bindings::run(args, output_format)
        }
    }
}

/// Print the top-level help message to stdout and return exit code `0`.
fn print_help_and_exit() -> i32 {
    let mut cmd = <Cli as clap::CommandFactory>::command();
    cmd.print_help().ok();
    println!();
    0
}
