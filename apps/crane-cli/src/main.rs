//! crane-cli binary — Content Retrieval And Normalization Engine.
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![allow(clippy::too_many_lines)]

use clap::{Args, Parser, Subcommand};
use crane_cli::adapters::{LopdfAdapter, PdfAdapter};
use crane_cli::commands::{
    check_all_commands, figure_commands, heading_commands, mermaid_commands, nesting_commands,
    ocr_commands, pdf_commands, report_commands, skiplist_commands, table_commands, text_commands,
};
use crane_cli::core::pdf_extraction_cache;
use std::sync::Arc;

/// Content Retrieval And Normalization Engine — PDF-to-Markdown validation tool.
#[derive(Debug, Parser)]
#[command(name = "crane", version, about)]
struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    command: Commands,
}

/// Top-level subcommands.
#[derive(Debug, Subcommand)]
enum Commands {
    /// PDF operations (info, type, extract).
    Pdf(PdfArgs),
    /// Text completeness checking.
    Text(TextArgs),
    /// Heading depth inference and checking.
    Heading(HeadingArgs),
    /// List nesting analysis.
    Nesting(NestingArgs),
    /// Table detection and checking.
    Table(TableArgs),
    /// Figure coverage checking.
    Figure(FigureArgs),
    /// Mermaid diagram validation.
    Mermaid(MermaidArgs),
    /// OCR quality assessment.
    Ocr(OcrArgs),
    /// Audit report management.
    Report(ReportArgs),
    /// Skip list management.
    Skiplist(SkiplistArgs),
    /// Run all check dimensions in one pass.
    #[command(name = "check-all")]
    CheckAll(CheckAllArgs),
}

/// PDF subcommand arguments.
#[derive(Debug, Args)]
struct PdfArgs {
    /// PDF subcommand.
    #[command(subcommand)]
    command: PdfCommands,
}

/// PDF subcommands.
#[derive(Debug, Subcommand)]
enum PdfCommands {
    /// Get PDF metadata as JSON.
    Info(PdfInfoArgs),
    /// Detect if PDF is text-based or image-based.
    Type(PdfTypeArgs),
    /// Extract text from PDF pages.
    Extract(PdfExtractArgs),
}

/// Arguments for `crane pdf info`.
#[derive(Debug, Args)]
struct PdfInfoArgs {
    /// Path to the PDF file.
    pdf: String,
}

/// Arguments for `crane pdf type`.
#[derive(Debug, Args)]
struct PdfTypeArgs {
    /// Path to the PDF file.
    pdf: String,
}

/// Arguments for `crane pdf extract`.
#[derive(Debug, Args)]
struct PdfExtractArgs {
    /// Path to the PDF file.
    pdf: String,
    /// Start page (1-indexed, default 1).
    #[arg(long, default_value = "1")]
    start_page: usize,
    /// End page (1-indexed, default last page).
    #[arg(long, default_value = "999")]
    end_page: usize,
    /// Output file path (default: stdout).
    #[arg(long)]
    output: Option<String>,
}

/// Text subcommand arguments.
#[derive(Debug, Args)]
struct TextArgs {
    /// Text subcommand.
    #[command(subcommand)]
    command: TextCommands,
}

/// Text subcommands.
#[derive(Debug, Subcommand)]
enum TextCommands {
    /// Check text completeness between PDF and Markdown.
    Check(TextCheckArgs),
    /// Search for a segment in a Markdown file.
    Search(TextSearchArgs),
}

/// Arguments for `crane text check`.
#[derive(Debug, Args)]
struct TextCheckArgs {
    /// Path to the PDF file.
    pdf: String,
    /// Path to the Markdown file.
    md: String,
}

/// Arguments for `crane text search`.
#[derive(Debug, Args)]
struct TextSearchArgs {
    /// Path to the Markdown file.
    md: String,
    /// Segment text to search for.
    segment: String,
}

/// Heading subcommand arguments.
#[derive(Debug, Args)]
struct HeadingArgs {
    /// Heading subcommand.
    #[command(subcommand)]
    command: HeadingCommands,
}

/// Heading subcommands.
#[derive(Debug, Subcommand)]
enum HeadingCommands {
    /// Infer heading depth from a text line (section numbering).
    Infer(HeadingInferArgs),
    /// Check heading consistency between PDF and Markdown.
    Check(HeadingCheckArgs),
}

/// Arguments for `crane heading infer`.
#[derive(Debug, Args)]
struct HeadingInferArgs {
    /// Text to analyze (e.g. "3.1.2 Details").
    pdf: String,
}

/// Arguments for `crane heading check`.
#[derive(Debug, Args)]
struct HeadingCheckArgs {
    /// Path to the PDF file.
    pdf: String,
    /// Path to the Markdown file.
    md: String,
}

/// Nesting subcommand arguments.
#[derive(Debug, Args)]
struct NestingArgs {
    /// Nesting subcommand.
    #[command(subcommand)]
    command: NestingCommands,
}

/// Nesting subcommands.
#[derive(Debug, Subcommand)]
enum NestingCommands {
    /// Infer nesting levels from PDF text.
    Infer(NestingInferArgs),
    /// Check nesting consistency between PDF and Markdown.
    Check(NestingCheckArgs),
}

/// Arguments for `crane nesting infer`.
#[derive(Debug, Args)]
struct NestingInferArgs {
    /// Path to the PDF file.
    pdf: String,
}

/// Arguments for `crane nesting check`.
#[derive(Debug, Args)]
struct NestingCheckArgs {
    /// Path to the PDF file.
    pdf: String,
    /// Path to the Markdown file.
    md: String,
}

/// Table subcommand arguments.
#[derive(Debug, Args)]
struct TableArgs {
    /// Table subcommand.
    #[command(subcommand)]
    command: TableCommands,
}

/// Table subcommands.
#[derive(Debug, Subcommand)]
enum TableCommands {
    /// Detect tables in PDF text.
    Detect(TableDetectArgs),
    /// Check table integrity between PDF and Markdown.
    Check(TableCheckArgs),
}

/// Arguments for `crane table detect`.
#[derive(Debug, Args)]
struct TableDetectArgs {
    /// Path to the PDF file.
    pdf: String,
}

/// Arguments for `crane table check`.
#[derive(Debug, Args)]
struct TableCheckArgs {
    /// Path to the PDF file.
    pdf: String,
    /// Path to the Markdown file.
    md: String,
}

/// Figure subcommand arguments.
#[derive(Debug, Args)]
struct FigureArgs {
    /// Figure subcommand.
    #[command(subcommand)]
    command: FigureCommands,
}

/// Figure subcommands.
#[derive(Debug, Subcommand)]
enum FigureCommands {
    /// Detect figure references in PDF text.
    Detect(FigureDetectArgs),
    /// Check figure coverage between PDF and Markdown.
    Check(FigureCheckArgs),
}

/// Arguments for `crane figure detect`.
#[derive(Debug, Args)]
struct FigureDetectArgs {
    /// Path to the PDF file.
    pdf: String,
}

/// Arguments for `crane figure check`.
#[derive(Debug, Args)]
struct FigureCheckArgs {
    /// Path to the PDF file.
    pdf: String,
    /// Path to the Markdown file.
    md: String,
}

/// Mermaid subcommand arguments.
#[derive(Debug, Args)]
struct MermaidArgs {
    /// Mermaid subcommand.
    #[command(subcommand)]
    command: MermaidCommands,
}

/// Mermaid subcommands.
#[derive(Debug, Subcommand)]
enum MermaidCommands {
    /// Validate Mermaid diagram syntax in a Markdown file.
    Validate(MermaidValidateArgs),
}

/// Arguments for `crane mermaid validate`.
#[derive(Debug, Args)]
struct MermaidValidateArgs {
    /// Path to the Markdown file.
    md: String,
}

/// OCR subcommand arguments.
#[derive(Debug, Args)]
struct OcrArgs {
    /// OCR subcommand.
    #[command(subcommand)]
    command: OcrCommands,
}

/// OCR subcommands.
#[derive(Debug, Subcommand)]
enum OcrCommands {
    /// Assess OCR quality in a Markdown file.
    Quality(OcrQualityArgs),
    /// Extract text from a PDF using OCR (pdftoppm + tesseract).
    Extract(OcrExtractArgs),
}

/// Arguments for `crane ocr quality`.
#[derive(Debug, Args)]
struct OcrQualityArgs {
    /// Path to the Markdown file.
    md: String,
}

/// Arguments for `crane ocr extract`.
#[derive(Debug, Args)]
struct OcrExtractArgs {
    /// Path to the PDF file.
    pdf: String,
}

/// Report subcommand arguments.
#[derive(Debug, Args)]
struct ReportArgs {
    /// Report subcommand.
    #[command(subcommand)]
    command: ReportCommands,
}

/// Report subcommands.
#[derive(Debug, Subcommand)]
enum ReportCommands {
    /// Initialize a new audit report.
    Init(ReportInitArgs),
    /// Finalize an audit report with a status.
    Finalize(ReportFinalizeArgs),
}

/// Arguments for `crane report init`.
#[derive(Debug, Args)]
struct ReportInitArgs {
    /// Scope identifier for the report.
    scope: String,
    /// Path to the PDF file.
    pdf: String,
    /// Path to the Markdown file.
    md: String,
}

/// Arguments for `crane report finalize`.
#[derive(Debug, Args)]
struct ReportFinalizeArgs {
    /// Path to the report file.
    report_path: String,
    /// Status string (e.g. "PASS", "FAIL").
    status: String,
}

/// Skiplist subcommand arguments.
#[derive(Debug, Args)]
struct SkiplistArgs {
    /// Skiplist subcommand.
    #[command(subcommand)]
    command: SkiplistCommands,
}

/// Skiplist subcommands.
#[derive(Debug, Subcommand)]
enum SkiplistCommands {
    /// Add an entry to the skip list.
    Add(SkiplistAddArgs),
    /// Check if an entry exists in the skip list.
    Check(SkiplistCheckArgs),
    /// List all skip list entries for a Markdown file.
    List(SkiplistListArgs),
}

/// Arguments for `crane skiplist add`.
#[derive(Debug, Args)]
struct SkiplistAddArgs {
    /// Markdown file basename.
    md_basename: String,
    /// Finding category.
    category: String,
    /// Finding description.
    description: String,
}

/// Arguments for `crane skiplist check`.
#[derive(Debug, Args)]
struct SkiplistCheckArgs {
    /// Markdown file basename.
    md_basename: String,
    /// Finding category.
    category: String,
    /// Finding description.
    description: String,
}

/// Arguments for `crane skiplist list`.
#[derive(Debug, Args)]
struct SkiplistListArgs {
    /// Markdown file basename.
    md_basename: String,
}

/// Arguments for `crane check-all`.
#[derive(Debug, Args)]
struct CheckAllArgs {
    /// Path to the PDF file.
    pdf: String,
    /// Path to the Markdown file.
    md: String,
    /// Optional cache directory for PDF extractions.
    #[arg(long)]
    cache_dir: Option<String>,
}

/// Builds a `PdfAdapter` (real lopdf, optionally wrapped with caching).
fn build_adapter(cache_dir: Option<&str>) -> Arc<dyn PdfAdapter> {
    let real: Arc<dyn PdfAdapter> = Arc::new(LopdfAdapter::new());
    match cache_dir {
        Some(dir) => pdf_extraction_cache::wrap(real, dir),
        None => real,
    }
}

/// Reads a Markdown file from disk, printing an error and returning `None` on failure.
fn read_md(md_path: &str) -> Option<String> {
    match std::fs::read_to_string(md_path) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Error: failed to read MD file '{md_path}': {e}");
            None
        }
    }
}

/// Reads PDF text via the adapter, printing an error and returning `None` on failure.
fn read_pdf_text(adapter: &dyn PdfAdapter, pdf: &str) -> Option<String> {
    match adapter.sample_text(pdf, 999) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Error: {e}");
            None
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Commands::Pdf(args) => match args.command {
            PdfCommands::Info(a) => {
                let adapter = LopdfAdapter::new();
                pdf_commands::run_info(&adapter, &a.pdf)
            }
            PdfCommands::Type(a) => {
                let adapter = LopdfAdapter::new();
                pdf_commands::run_type(&adapter, &a.pdf)
            }
            PdfCommands::Extract(a) => {
                let adapter = LopdfAdapter::new();
                pdf_commands::run_extract(
                    &adapter,
                    &a.pdf,
                    a.start_page,
                    a.end_page,
                    a.output.as_deref(),
                )
            }
        },

        Commands::Text(args) => match args.command {
            TextCommands::Check(a) => {
                let adapter = LopdfAdapter::new();
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                text_commands::run_check(&adapter, &a.pdf, &md_text)
            }
            TextCommands::Search(a) => {
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                text_commands::run_search(&md_text, &a.segment)
            }
        },

        Commands::Heading(args) => match args.command {
            HeadingCommands::Infer(a) => {
                let adapter = LopdfAdapter::new();
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(text) => heading_commands::run_infer(&text),
                    None => 1,
                }
            }
            HeadingCommands::Check(a) => {
                let adapter = LopdfAdapter::new();
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(pdf_text) => heading_commands::run_check(&pdf_text, &md_text),
                    None => 1,
                }
            }
        },

        Commands::Nesting(args) => match args.command {
            NestingCommands::Infer(a) => {
                let adapter = LopdfAdapter::new();
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(text) => nesting_commands::run_infer(&text),
                    None => 1,
                }
            }
            NestingCommands::Check(a) => {
                let adapter = LopdfAdapter::new();
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(pdf_text) => nesting_commands::run_check(&pdf_text, &md_text),
                    None => 1,
                }
            }
        },

        Commands::Table(args) => match args.command {
            TableCommands::Detect(a) => {
                let adapter = LopdfAdapter::new();
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(text) => table_commands::run_detect(&text),
                    None => 1,
                }
            }
            TableCommands::Check(a) => {
                let adapter = LopdfAdapter::new();
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(pdf_text) => table_commands::run_check(&pdf_text, &md_text),
                    None => 1,
                }
            }
        },

        Commands::Figure(args) => match args.command {
            FigureCommands::Detect(a) => {
                let adapter = LopdfAdapter::new();
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(text) => figure_commands::run_detect(&text),
                    None => 1,
                }
            }
            FigureCommands::Check(a) => {
                let adapter = LopdfAdapter::new();
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                match read_pdf_text(&adapter, &a.pdf) {
                    Some(pdf_text) => figure_commands::run_check(&pdf_text, &md_text),
                    None => 1,
                }
            }
        },

        Commands::Mermaid(args) => match args.command {
            MermaidCommands::Validate(a) => {
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                mermaid_commands::run_validate(&md_text)
            }
        },

        Commands::Ocr(args) => match args.command {
            OcrCommands::Quality(a) => {
                let Some(md_text) = read_md(&a.md) else {
                    std::process::exit(1);
                };
                ocr_commands::run_quality(&md_text)
            }
            OcrCommands::Extract(a) => ocr_commands::run_extract(&a.pdf),
        },

        Commands::Report(args) => match args.command {
            ReportCommands::Init(a) => report_commands::run_init(&a.scope, &a.pdf, &a.md),
            ReportCommands::Finalize(a) => report_commands::run_finalize(&a.report_path, &a.status),
        },

        Commands::Skiplist(args) => match args.command {
            SkiplistCommands::Add(a) => {
                skiplist_commands::run_add(&a.md_basename, &a.category, &a.description)
            }
            SkiplistCommands::Check(a) => {
                skiplist_commands::run_check(&a.md_basename, &a.category, &a.description)
            }
            SkiplistCommands::List(a) => skiplist_commands::run_list(&a.md_basename),
        },

        Commands::CheckAll(a) => {
            let adapter = build_adapter(a.cache_dir.as_deref());
            let Some(md_text) = read_md(&a.md) else {
                std::process::exit(1);
            };
            check_all_commands::run_check_all(adapter.as_ref(), &a.pdf, &md_text)
        }
    };

    std::process::exit(exit_code);
}
