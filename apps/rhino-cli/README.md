# rhino-cli

**RHINO** – Repository Hygiene & INtegration Orchestrator

Command-line tools for repository management and automation. The canonical implementation is Rust (this crate); the predecessor Go binary lives at [`archived/rhino-cli/`](../../archived/rhino-cli/) for git-history archaeology only. The Rust rewrite completed 2026-05-23.

## What is rhino-cli?

A Rust CLI binary delivering the same observable contract as the original Go implementation — same commands, same flags, same exit codes, same output formats (text / json / markdown). Built with `clap` (derive macros) and consuming the Gherkin specs in [`specs/apps/rhino/behavior/cli/gherkin/`](../../specs/apps/rhino/behavior/cli/gherkin/).

## Status

Production. All commands ported and byte-identical to the original Go binary across shadow-diff corpora.

## Quick Start

```bash
# Build the release binary (Nx)
nx build rhino-cli-rs

# Run the binary
cargo run --manifest-path apps/rhino-cli/Cargo.toml -- --help

# Echo a message
cargo run --manifest-path apps/rhino-cli/Cargo.toml -- --say "hello world"

# Reject invalid output format (exits 1)
cargo run --manifest-path apps/rhino-cli/Cargo.toml -- --output xml --help
```

## Installation

The crate is local to this monorepo. To produce a standalone binary:

```bash
cd apps/rhino-cli
cargo build --release
# Binary at apps/rhino-cli/target/release/rhino-cli
# Or via Nx: nx build rhino-cli-rs → apps/rhino-cli/dist/rhino-cli
```

Toolchain is pinned to Rust 1.95.0 via `rust-toolchain.toml`; the first `cargo` call inside this crate auto-bootstraps the toolchain through `rustup`. MSRV is 1.88 (`cucumber 0.23.0` bound).

## Nx Targets

| Target             | Command                                                                                 |
| ------------------ | --------------------------------------------------------------------------------------- |
| `build`            | `cargo build --release` → `dist/rhino-cli`                                              |
| `lint`             | `cargo clippy --all-targets -- -D warnings`                                             |
| `typecheck`        | `cargo check --all-targets`                                                             |
| `test:unit`        | `cargo test --lib` (in-source `#[cfg(test)]` modules)                                   |
| `test:integration` | `cargo test --tests` (integration tests under `tests/`)                                 |
| `test:quick`       | `cargo llvm-cov --lib --lcov --fail-under-lines 90` (Phase 1 swaps to native validator) |
| `spec-coverage`    | Phase 0 stub; Phase 1 wires cucumber-rs spec consumption                                |
| `run`              | `cargo run --`                                                                          |
| `install`          | `cargo fetch`                                                                           |

## Global Flags

Mirror the Go binary's Cobra root command (`apps/rhino-cli/cmd/root.go:23`):

- `--verbose, -v` — verbose output with timestamps
- `--quiet, -q` — quiet mode (errors only)
- `--output, -o text|json|markdown` — output format (default: text). Invalid values exit 1.
- `--no-color` — disable colored output
- `--say <msg>` — echo a message to stdout
- `--help, -h` — print help

## See also

- Go implementation (archived at cutover): [`apps/rhino-cli/README.md`](../rhino-cli/README.md)
- Migration plan: [`plans/in-progress/rhino-cli-rust-rewrite/`](../../plans/in-progress/rhino-cli-rust-rewrite/README.md)
- Gherkin specs (shared with Go binary): [`specs/apps/rhino/behavior/cli/gherkin/`](../../specs/apps/rhino/behavior/cli/gherkin/)
