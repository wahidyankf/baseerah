//! `ose-cli` binary entry point.
#![forbid(unsafe_code)]

fn main() {
    let exit_code = ose_cli::cli::run();
    std::process::exit(exit_code);
}
