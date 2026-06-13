//! CLI entry point: argument parsing, subcommand dispatch, and help output.

use clap::{Parser, Subcommand};

use crate::commands::{
    convention_audit, convention_validate_agents_md_size, convention_validate_emoji,
    convention_validate_license, doctor, env_backup, env_init, env_restore, env_validate,
    git_pre_commit, governance_audit, governance_layer_coherence, governance_traceability_audit,
    governance_vendor_audit, harness_audit, harness_emit_bindings, harness_generate_bindings,
    harness_sync, harness_validate_bindings, harness_validate_claude, harness_validate_duplication,
    harness_validate_naming, harness_validate_sync, lang_java_validate_null_safety, md_audit,
    md_validate_frontmatter, md_validate_frontmatter_dates, md_validate_heading_hierarchy,
    md_validate_links, md_validate_mermaid, md_validate_naming, md_validate_readme_index,
    specs_audit, specs_bc, specs_clean_java_imports, specs_coverage, specs_gherkin_cardinality,
    specs_scaffold_dart, specs_ul, specs_validate_adoption, specs_validate_counts,
    specs_validate_links, specs_validate_tree, test_coverage_validate, workflows_validate_naming,
};
use crate::domain::cliout::OutputFormat;

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
    /// Test coverage commands (validate only).
    #[command(name = "test-coverage", subcommand)]
    TestCoverage(TestCoverageCommands),
    /// Repository governance audits.
    #[command(name = "repo-governance", subcommand)]
    RepoGovernance(RepoGovernanceCommands),
    /// Markdown validators (naming, frontmatter, heading-hierarchy, links, mermaid, frontmatter-dates, readme-index).
    #[command(name = "md", subcommand)]
    Md(MdCommands),
    /// Convention validators (emoji, license, agents-md-size).
    #[command(name = "convention", subcommand)]
    Convention(ConventionCommands),
    /// Harness (agent binding) validators.
    #[command(name = "harness", subcommand)]
    Harness(HarnessCommands),
    /// Workflow file validators.
    #[command(name = "workflows", subcommand)]
    Workflows(WorkflowsCommands),
    /// Spec tree validators and contract codegen helpers.
    #[command(name = "specs", subcommand)]
    Specs(SpecsCommands),
    /// Language-source correctness checks, nested by language.
    #[command(name = "lang", subcommand)]
    Lang(LangCommands),
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

/// Environment file subcommands (`init`, `backup`, `restore`, `validate`).
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
    /// Check code↔config drift for all surfaces in env-contract.yaml.
    #[command(name = "validate")]
    Validate(env_validate::EnvValidateArgs),
}

// ---------------------------------------------------------------------------
// specs
// ---------------------------------------------------------------------------

/// Spec tree subcommands.
#[derive(Subcommand, Debug)]
pub enum SpecsCommands {
    /// Run spec validation rules.
    #[command(name = "validate", subcommand)]
    Validate(SpecsValidateCommands),
    /// Clean generated contract files (e.g. strip unused Java imports).
    #[command(name = "clean", subcommand)]
    Clean(SpecsCleanCommands),
    /// Scaffold generated contract package structure (e.g. Dart pubspec).
    #[command(name = "scaffold", subcommand)]
    Scaffold(SpecsScaffoldCommands),
    /// Run all specs validators in sequence and aggregate findings.
    #[command(name = "audit")]
    Audit(specs_audit::AuditArgs),
}

/// Specs clean subcommands.
#[derive(Subcommand, Debug)]
pub enum SpecsCleanCommands {
    /// Strip unused/same-package imports from generated Java contract files (dormant in ose-public).
    #[command(name = "java-imports")]
    JavaImports(specs_clean_java_imports::CleanJavaImportsArgs),
}

/// Specs scaffold subcommands.
#[derive(Subcommand, Debug)]
pub enum SpecsScaffoldCommands {
    /// Generate Dart package scaffolding around generated contract types (dormant in ose-public).
    #[command(name = "dart")]
    Dart(specs_scaffold_dart::ScaffoldDartArgs),
}

/// Spec tree validate subcommands.
#[derive(Subcommand, Debug)]
pub enum SpecsValidateCommands {
    /// Verify an app has adopted BDD and DDD practices.
    #[command(name = "adoption")]
    Adoption(specs_validate_adoption::ValidateAdoptionArgs),
    /// Validate each required spec subfolder contains at least one spec file.
    #[command(name = "counts")]
    Counts(specs_validate_counts::ValidateCountsArgs),
    /// Check that markdown links in spec files resolve.
    #[command(name = "links")]
    Links(specs_validate_links::ValidateLinksArgs),
    /// Validate canonical C4-aware five-folder spec tree.
    #[command(name = "tree")]
    Tree(specs_validate_tree::ValidateTreeArgs),
    /// Validate that all BDD spec files have matching test implementations.
    #[command(name = "coverage")]
    Coverage(specs_coverage::ValidateArgs),
    /// Validate bounded-context structural parity against the registry.
    #[command(name = "bc")]
    Bc(specs_bc::DddBcArgs),
    /// Validate ubiquitous-language glossary parity against the registry.
    #[command(name = "ul")]
    Ul(specs_ul::DddUlArgs),
    /// Audit `.feature` scenarios for repeated primary Given/When/Then keywords.
    #[command(name = "gherkin-cardinality")]
    GherkinCardinality(specs_gherkin_cardinality::GherkinKeywordCardinalityArgs),
}

// ---------------------------------------------------------------------------
// harness
// ---------------------------------------------------------------------------

/// Harness (agent binding) subcommands.
#[derive(Subcommand, Debug)]
pub enum HarnessCommands {
    /// Run harness validation rules.
    #[command(name = "validate", subcommand)]
    Validate(HarnessValidateCommands),
    /// Sync Claude Code agents to `OpenCode` format.
    #[command(name = "sync", subcommand)]
    Sync(HarnessSyncCommands),
    /// Emit platform-specific binding bridge files.
    #[command(name = "emit", subcommand)]
    Emit(HarnessEmitCommands),
    /// Generate all platform bindings.
    #[command(name = "generate", subcommand)]
    Generate(HarnessGenerateCommands),
    /// Run all harness validators in sequence and aggregate findings.
    #[command(name = "audit")]
    Audit(harness_audit::AuditArgs),
}

/// Harness validate subcommands.
#[derive(Subcommand, Debug)]
pub enum HarnessValidateCommands {
    /// Validate agent filename suffixes and mirror parity.
    #[command(name = "naming")]
    Naming(harness_validate_naming::ValidateNamingArgs),
    /// Detect verbatim duplication across agent and skill files.
    #[command(name = "duplication")]
    Duplication(harness_validate_duplication::DetectDuplicationArgs),
    /// Validate Claude Code agent and skill format in .claude/ directory.
    #[command(name = "claude")]
    Claude(harness_validate_claude::ValidateClaudeArgs),
    /// Validate that .claude/ and .opencode/ are in sync.
    #[command(name = "sync")]
    Sync(harness_validate_sync::ValidateSyncArgs),
    /// Validate the Amazon Q binding bridge files and catalog coverage.
    #[command(name = "bindings")]
    Bindings(harness_validate_bindings::ValidateBindingsArgs),
}

/// Harness sync subcommands.
#[derive(Subcommand, Debug)]
pub enum HarnessSyncCommands {
    /// Sync Claude Code agents to `OpenCode` format.
    #[command(name = "opencode")]
    Opencode(harness_sync::SyncArgs),
}

/// Harness emit subcommands.
#[derive(Subcommand, Debug)]
pub enum HarnessEmitCommands {
    /// Emit the Amazon Q Developer binding bridge files (idempotent).
    #[command(name = "amazonq")]
    Amazonq(harness_emit_bindings::EmitBindingsArgs),
}

/// Harness generate subcommands.
#[derive(Subcommand, Debug)]
pub enum HarnessGenerateCommands {
    /// Generate all platform bindings: sync `OpenCode` agents and emit Amazon Q bridge files.
    #[command(name = "bindings")]
    Bindings(harness_generate_bindings::GenerateBindingsArgs),
}

// ---------------------------------------------------------------------------
// workflows
// ---------------------------------------------------------------------------

/// Workflow file subcommands.
#[derive(Subcommand, Debug)]
pub enum WorkflowsCommands {
    /// Run workflow validation rules.
    #[command(name = "validate", subcommand)]
    Validate(WorkflowsValidateCommands),
}

/// Workflow validate subcommands.
#[derive(Subcommand, Debug)]
pub enum WorkflowsValidateCommands {
    /// Validate workflow filename suffixes and frontmatter name consistency.
    #[command(name = "naming")]
    Naming(workflows_validate_naming::ValidateNamingArgs),
}

// ---------------------------------------------------------------------------
// md
// ---------------------------------------------------------------------------

/// Markdown validator subcommands.
#[derive(Subcommand, Debug)]
pub enum MdCommands {
    /// Run markdown validation rules.
    #[command(name = "validate", subcommand)]
    Validate(MdValidateCommands),
    /// Audit markdown files for forbidden manual date metadata.
    #[command(name = "frontmatter-dates")]
    FrontmatterDates(md_validate_frontmatter_dates::FrontmatterAuditArgs),
    /// Audit directory README.md indexes against sibling markdown files.
    #[command(name = "readme-index")]
    ReadmeIndex(md_validate_readme_index::ReadmeIndexAuditArgs),
    /// Run all md validators in sequence and aggregate findings.
    #[command(name = "audit")]
    Audit(md_audit::AuditArgs),
}

/// Markdown validate subcommands.
#[derive(Subcommand, Debug)]
pub enum MdValidateCommands {
    /// Validate markdown filenames against the lowercase-kebab-case rule.
    #[command(name = "naming")]
    Naming(md_validate_naming::ValidateNamingArgs),
    /// Validate documentation YAML frontmatter against area-specific schemas.
    #[command(name = "frontmatter")]
    Frontmatter(md_validate_frontmatter::ValidateFrontmatterArgs),
    /// Validate markdown heading hierarchy (one H1, no skipped levels).
    #[command(name = "heading-hierarchy")]
    HeadingHierarchy(md_validate_heading_hierarchy::ValidateHeadingHierarchyArgs),
    /// Validate markdown links (relative paths exist on disk).
    #[command(name = "links")]
    Links(md_validate_links::ValidateLinksArgs),
    /// Validate Mermaid flowchart diagrams (label length, span, single-diagram).
    #[command(name = "mermaid")]
    Mermaid(md_validate_mermaid::ValidateMermaidArgs),
    /// Audit markdown files for forbidden manual date metadata.
    #[command(name = "frontmatter-dates")]
    FrontmatterDates(md_validate_frontmatter_dates::FrontmatterAuditArgs),
    /// Audit directory README.md indexes against sibling markdown files.
    #[command(name = "readme-index")]
    ReadmeIndex(md_validate_readme_index::ReadmeIndexAuditArgs),
}

// ---------------------------------------------------------------------------
// convention
// ---------------------------------------------------------------------------

/// Convention validator subcommands.
#[derive(Subcommand, Debug)]
pub enum ConventionCommands {
    /// Run convention validation rules.
    #[command(name = "validate", subcommand)]
    Validate(ConventionValidateCommands),
    /// Run all convention validators in sequence and aggregate findings.
    #[command(name = "audit")]
    Audit(convention_audit::AuditArgs),
}

/// Convention validate subcommands.
#[derive(Subcommand, Debug)]
pub enum ConventionValidateCommands {
    /// Audit forbidden file types for emoji codepoints.
    #[command(name = "emoji")]
    Emoji(convention_validate_emoji::EmojiAuditArgs),
    /// Verify per-directory LICENSE files match the licensing convention.
    #[command(name = "license")]
    License(convention_validate_license::LicenseAuditArgs),
    /// Audit AGENTS.md size against the 30/35/40 KB thresholds.
    #[command(name = "agents-md-size")]
    AgentsMdSize(convention_validate_agents_md_size::AgentsMdSizeArgs),
}

// ---------------------------------------------------------------------------
// repo-governance
// ---------------------------------------------------------------------------

/// Repository governance subcommands.
#[derive(Subcommand, Debug)]
pub enum RepoGovernanceCommands {
    /// Run governance validation rules.
    #[command(name = "validate", subcommand)]
    Validate(RepoGovernanceValidateCommands),
    /// Run all deterministic governance audits and emit a JSON envelope.
    #[command(name = "audit")]
    Audit(governance_audit::AuditArgs),
}

/// Repository governance validate subcommands.
#[derive(Subcommand, Debug)]
pub enum RepoGovernanceValidateCommands {
    /// Scan governance markdown for forbidden vendor-specific terms.
    #[command(name = "vendor")]
    Vendor(governance_vendor_audit::VendorAuditArgs),
    /// Audit governance docs for layer numbering/naming coherence.
    #[command(name = "layer-coherence")]
    LayerCoherence(governance_layer_coherence::LayerCoherenceArgs),
    /// Audit governance documents for required traceability sections.
    #[command(name = "traceability")]
    Traceability(governance_traceability_audit::TraceabilityAuditArgs),
}

// ---------------------------------------------------------------------------
// lang
// ---------------------------------------------------------------------------

/// Language-source correctness subcommands, nested by language.
#[derive(Subcommand, Debug)]
pub enum LangCommands {
    /// Java language checks.
    #[command(name = "java", subcommand)]
    Java(LangJavaCommands),
}

/// Java language subcommands.
#[derive(Subcommand, Debug)]
pub enum LangJavaCommands {
    /// Validate Java source correctness.
    #[command(name = "validate", subcommand)]
    Validate(LangJavaValidateCommands),
}

/// Java validate subcommands.
#[derive(Subcommand, Debug)]
pub enum LangJavaValidateCommands {
    /// Check Java packages carry required null-safety annotations (dormant in ose-public).
    #[command(name = "null-safety-annotations")]
    NullSafetyAnnotations(lang_java_validate_null_safety::ValidateNullSafetyArgs),
}

// ---------------------------------------------------------------------------
// test-coverage
// ---------------------------------------------------------------------------

/// Test coverage subcommands (`validate` only).
#[derive(Subcommand, Debug)]
pub enum TestCoverageCommands {
    /// Check test coverage against a threshold (standard line-based algorithm).
    Validate(test_coverage_validate::ValidateArgs),
}

// ---------------------------------------------------------------------------
// dispatch
// ---------------------------------------------------------------------------

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
        },
        Commands::RepoGovernance(rg) => match rg {
            RepoGovernanceCommands::Validate(vc) => match vc {
                RepoGovernanceValidateCommands::Vendor(args) => {
                    governance_vendor_audit::run(args, output_format)
                }
                RepoGovernanceValidateCommands::LayerCoherence(args) => {
                    governance_layer_coherence::run(args, output_format)
                }
                RepoGovernanceValidateCommands::Traceability(args) => {
                    governance_traceability_audit::run(args, output_format)
                }
            },
            RepoGovernanceCommands::Audit(args) => governance_audit::run(args, output_format),
        },
        Commands::Md(mc) => dispatch_md(mc, output_format),
        Commands::Convention(cc) => match cc {
            ConventionCommands::Validate(vc) => match vc {
                ConventionValidateCommands::Emoji(args) => {
                    convention_validate_emoji::run(args, output_format)
                }
                ConventionValidateCommands::License(args) => {
                    convention_validate_license::run(args, output_format)
                }
                ConventionValidateCommands::AgentsMdSize(args) => {
                    convention_validate_agents_md_size::run(args, output_format)
                }
            },
            ConventionCommands::Audit(args) => convention_audit::run(args, output_format),
        },
        Commands::Harness(hc) => dispatch_harness(hc, output_format),
        Commands::Workflows(wc) => match wc {
            WorkflowsCommands::Validate(vc) => match vc {
                WorkflowsValidateCommands::Naming(args) => {
                    workflows_validate_naming::run(args, output_format)
                }
            },
        },
        Commands::Specs(sc) => dispatch_specs(sc, output_format),
        Commands::Lang(lc) => dispatch_lang(lc, output_format),
        Commands::Git(gc) => match gc {
            GitCommands::PreCommit(args) => git_pre_commit::run_cmd(args, output_format),
        },
        Commands::Env(ec) => match ec {
            EnvCommands::Init(args) => env_init::run(args, output_format),
            EnvCommands::Backup(args) => env_backup::run(args, output_format),
            EnvCommands::Restore(args) => env_restore::run(args, output_format),
            EnvCommands::Validate(args) => env_validate::run(args, output_format),
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

/// Route a [`MdCommands`] variant to its handler.
///
/// # Errors
///
/// Propagates any error returned by the selected md subcommand.
fn dispatch_md(
    mc: &MdCommands,
    output_format: OutputFormat,
) -> std::result::Result<(), anyhow::Error> {
    match mc {
        MdCommands::Validate(vc) => match vc {
            MdValidateCommands::Naming(args) => md_validate_naming::run(args, output_format),
            MdValidateCommands::Frontmatter(args) => {
                md_validate_frontmatter::run(args, output_format)
            }
            MdValidateCommands::HeadingHierarchy(args) => {
                md_validate_heading_hierarchy::run(args, output_format)
            }
            MdValidateCommands::Links(args) => md_validate_links::run(args, output_format),
            MdValidateCommands::Mermaid(args) => md_validate_mermaid::run(args, output_format),
            MdValidateCommands::FrontmatterDates(args) => {
                md_validate_frontmatter_dates::run(args, output_format)
            }
            MdValidateCommands::ReadmeIndex(args) => {
                md_validate_readme_index::run(args, output_format)
            }
        },
        MdCommands::FrontmatterDates(args) => {
            md_validate_frontmatter_dates::run(args, output_format)
        }
        MdCommands::ReadmeIndex(args) => md_validate_readme_index::run(args, output_format),
        MdCommands::Audit(args) => md_audit::run(args, output_format),
    }
}

/// Route a [`HarnessCommands`] variant to its handler.
///
/// # Errors
///
/// Propagates any error returned by the selected harness subcommand.
fn dispatch_harness(
    hc: &HarnessCommands,
    output_format: OutputFormat,
) -> std::result::Result<(), anyhow::Error> {
    match hc {
        HarnessCommands::Validate(vc) => match vc {
            HarnessValidateCommands::Naming(args) => {
                harness_validate_naming::run(args, output_format)
            }
            HarnessValidateCommands::Duplication(args) => {
                harness_validate_duplication::run(args, output_format)
            }
            HarnessValidateCommands::Claude(args) => {
                harness_validate_claude::run(args, output_format)
            }
            HarnessValidateCommands::Sync(args) => harness_validate_sync::run(args, output_format),
            HarnessValidateCommands::Bindings(args) => {
                harness_validate_bindings::run(args, output_format)
            }
        },
        HarnessCommands::Sync(sc) => match sc {
            HarnessSyncCommands::Opencode(args) => harness_sync::run(args, output_format),
        },
        HarnessCommands::Emit(ec) => match ec {
            HarnessEmitCommands::Amazonq(args) => harness_emit_bindings::run(args, output_format),
        },
        HarnessCommands::Generate(gc) => match gc {
            HarnessGenerateCommands::Bindings(args) => {
                harness_generate_bindings::run(args, output_format)
            }
        },
        HarnessCommands::Audit(args) => harness_audit::run(args, output_format),
    }
}

/// Route a [`SpecsCommands`] variant to its handler.
///
/// # Errors
///
/// Propagates any error returned by the selected specs subcommand.
fn dispatch_specs(
    sc: &SpecsCommands,
    output_format: OutputFormat,
) -> std::result::Result<(), anyhow::Error> {
    match sc {
        SpecsCommands::Validate(vc) => match vc {
            SpecsValidateCommands::Adoption(args) => {
                specs_validate_adoption::run(args, output_format)
            }
            SpecsValidateCommands::Counts(args) => specs_validate_counts::run(args, output_format),
            SpecsValidateCommands::Links(args) => specs_validate_links::run(args, output_format),
            SpecsValidateCommands::Tree(args) => specs_validate_tree::run(args, output_format),
            SpecsValidateCommands::Coverage(args) => specs_coverage::run(args, output_format),
            SpecsValidateCommands::Bc(args) => specs_bc::run(args, output_format),
            SpecsValidateCommands::Ul(args) => specs_ul::run(args, output_format),
            SpecsValidateCommands::GherkinCardinality(args) => {
                specs_gherkin_cardinality::run(args, output_format)
            }
        },
        SpecsCommands::Clean(cc) => match cc {
            SpecsCleanCommands::JavaImports(args) => {
                specs_clean_java_imports::run(args, output_format)
            }
        },
        SpecsCommands::Scaffold(sc) => match sc {
            SpecsScaffoldCommands::Dart(args) => specs_scaffold_dart::run(args, output_format),
        },
        SpecsCommands::Audit(args) => specs_audit::run(args, output_format),
    }
}

/// Route a [`LangCommands`] variant to its handler.
///
/// # Errors
///
/// Propagates any error returned by the selected lang subcommand.
fn dispatch_lang(
    lc: &LangCommands,
    output_format: OutputFormat,
) -> std::result::Result<(), anyhow::Error> {
    match lc {
        LangCommands::Java(jc) => match jc {
            LangJavaCommands::Validate(vc) => match vc {
                LangJavaValidateCommands::NullSafetyAnnotations(args) => {
                    lang_java_validate_null_safety::run(args, output_format)
                }
            },
        },
    }
}

/// Print the top-level help message to stdout and return exit code `0`.
fn print_help_and_exit() -> i32 {
    let mut cmd = <Cli as clap::CommandFactory>::command();
    cmd.print_help().ok();
    println!();
    0
}

#[cfg(test)]
mod cli_uniform_grammar_tests {
    use super::Cli;
    use clap::Parser;

    // --- OLD forms must FAIL to parse after P9b ---

    #[test]
    fn old_md_validate_mermaid_fails() {
        let result = Cli::try_parse_from(["rhino-cli", "md", "validate-mermaid"]);
        assert!(
            result.is_err(),
            "md validate-mermaid should no longer parse"
        );
    }

    #[test]
    fn old_harness_validate_naming_fails() {
        let result = Cli::try_parse_from(["rhino-cli", "harness", "validate-naming"]);
        assert!(
            result.is_err(),
            "harness validate-naming should no longer parse"
        );
    }

    #[test]
    fn old_harness_detect_duplication_fails() {
        let result = Cli::try_parse_from(["rhino-cli", "harness", "detect-duplication"]);
        assert!(
            result.is_err(),
            "harness detect-duplication should no longer parse"
        );
    }

    #[test]
    fn old_repo_governance_vendor_audit_fails() {
        let result = Cli::try_parse_from(["rhino-cli", "repo-governance", "vendor-audit"]);
        assert!(
            result.is_err(),
            "repo-governance vendor-audit should no longer parse"
        );
    }

    #[test]
    fn old_specs_coverage_fails() {
        let result = Cli::try_parse_from(["rhino-cli", "specs", "coverage"]);
        assert!(result.is_err(), "specs coverage should no longer parse");
    }

    // --- NEW uniform forms must PASS after P9b ---

    #[test]
    fn new_md_validate_mermaid_passes() {
        let result = Cli::try_parse_from(["rhino-cli", "md", "validate", "mermaid"]);
        assert!(
            result.is_ok(),
            "md validate mermaid must parse: {:?}",
            result.err()
        );
    }

    #[test]
    fn new_harness_validate_naming_passes() {
        let result = Cli::try_parse_from(["rhino-cli", "harness", "validate", "naming"]);
        assert!(
            result.is_ok(),
            "harness validate naming must parse: {:?}",
            result.err()
        );
    }

    #[test]
    fn new_harness_sync_opencode_passes() {
        let result = Cli::try_parse_from(["rhino-cli", "harness", "sync", "opencode"]);
        assert!(
            result.is_ok(),
            "harness sync opencode must parse: {:?}",
            result.err()
        );
    }

    #[test]
    fn new_repo_governance_validate_vendor_passes() {
        let result = Cli::try_parse_from(["rhino-cli", "repo-governance", "validate", "vendor"]);
        assert!(
            result.is_ok(),
            "repo-governance validate vendor must parse: {:?}",
            result.err()
        );
    }

    #[test]
    fn new_specs_validate_coverage_passes() {
        // coverage requires ≥2 positional args; supply dummy paths so parse succeeds.
        let result = Cli::try_parse_from([
            "rhino-cli",
            "specs",
            "validate",
            "coverage",
            "specs/",
            "apps/",
        ]);
        assert!(
            result.is_ok(),
            "specs validate coverage must parse: {:?}",
            result.err()
        );
    }
}
