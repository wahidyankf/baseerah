# rhino-cli (Rust port — work in progress)

**RHINO** – Repository Hygiene & INtegration Orchestrator

Command-line tools for repository management and automation. Rust rewrite of the Go `apps/rhino-cli/` binary. Both implementations coexist on `main` while the port progresses phase-by-phase; downstream callers flip from the Go binary to this Rust crate as each command lands. See [`plans/in-progress/rhino-cli-rust-rewrite/`](../../plans/in-progress/rhino-cli-rust-rewrite/README.md) for the migration plan and motivation (type-safety for fundamental tooling).

## What is rhino-cli (Rust port)?

A Rust CLI binary that delivers the same observable contract as the Go `rhino-cli` — same commands, same flags, same exit codes, same output formats (text / json / markdown). Built with `clap` (derive macros) and consuming the same Gherkin specs in [`specs/apps/rhino/behavior/cli/gherkin/`](../../specs/apps/rhino/behavior/cli/gherkin/) via `cucumber-rs`.

## Status

Phase 0 (foundation): crate scaffold, Nx targets, output-format sealed enum. Most commands still ship from the Go binary at `apps/rhino-cli/`. See the [phased delivery checklist](../../plans/in-progress/rhino-cli-rust-rewrite/delivery.md) for the current frontier.

## Quick Start

```bash
# Build the release binary (Nx)
nx build rhino-cli-rs

# Run the binary
cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- --help

# Echo a message
cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- --say "hello world"

# Reject invalid output format (exits 1)
cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- --output xml --help
```

## Installation

The crate is local to this monorepo. To produce a standalone binary:

```bash
cd apps/rhino-cli-rs
cargo build --release
# Binary at apps/rhino-cli-rs/target/release/rhino-cli
# Or via Nx: nx build rhino-cli-rs → apps/rhino-cli-rs/dist/rhino-cli
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
