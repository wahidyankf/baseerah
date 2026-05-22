use clap::{Parser, Subcommand};

use crate::commands::test_coverage_validate;
use crate::internal::cliout::OutputFormat;

#[derive(Parser, Debug)]
#[command(
    name = "rhino-cli",
    version = "0.16.1",
    about = "CLI tools for repository management",
    long_about = "Command-line tools for repository management and automation.",
    disable_help_flag = true
)]
pub struct Cli {
    #[arg(
        long,
        short = 'v',
        global = true,
        help = "verbose output with timestamps"
    )]
    pub verbose: bool,

    #[arg(long, short = 'q', global = true, help = "quiet mode (errors only)")]
    pub quiet: bool,

    #[arg(
        long,
        short = 'o',
        global = true,
        default_value = "text",
        help = "output format: text, json, markdown"
    )]
    pub output: String,

    #[arg(long = "no-color", global = true, help = "disable colored output")]
    pub no_color: bool,

    #[arg(
        long,
        global = true,
        default_value = "",
        help = "echo a message to stdout"
    )]
    pub say: String,

    #[arg(long, short = 'h', global = true, help = "Print help")]
    pub help: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Test coverage commands (validate, diff, merge).
    #[command(name = "test-coverage", subcommand)]
    TestCoverage(TestCoverageCommands),
}

#[derive(Subcommand, Debug)]
pub enum TestCoverageCommands {
    /// Check test coverage against a threshold (standard line-based algorithm).
    Validate(test_coverage_validate::ValidateArgs),
}

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

fn dispatch(cmd: &Commands, output_format: OutputFormat) -> i32 {
    let result = match cmd {
        Commands::TestCoverage(tc) => match tc {
            TestCoverageCommands::Validate(args) => {
                test_coverage_validate::run(args, output_format)
            }
        },
    };
    match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn print_help_and_exit() -> i32 {
    let mut cmd = <Cli as clap::CommandFactory>::command();
    cmd.print_help().ok();
    println!();
    0
}
