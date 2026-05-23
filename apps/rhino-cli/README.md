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
nx build rhino-cli

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
# Or via Nx: nx build rhino-cli → apps/rhino-cli/dist/rhino-cli
```

Toolchain is pinned to Rust 1.95.0 via `rust-toolchain.toml`; the first `cargo` call inside this crate auto-bootstraps the toolchain through `rustup`. MSRV is 1.88 (`cucumber 0.23.0` bound).

> **Note (C-06):** `rust-version = "1.88"` in `Cargo.toml` is the _minimum_ compiler version that can build this crate (MSRV). `channel = "1.95.0"` in `rust-toolchain.toml` is the _installed_ toolchain version used by developers and CI. Both are correct — installed ≥ MSRV is the invariant.

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

Global flags (see `src/cli.rs`):

- `--verbose, -v` — verbose output with timestamps
- `--quiet, -q` — quiet mode (errors only)
- `--output, -o text|json|markdown` — output format (default: text). Invalid values exit 1.
- `--no-color` — disable colored output
- `--say <msg>` — echo a message to stdout
- `--help, -h` — print help

## Dependency Status

Reviewed 2026-05-23. Policy paths per [Dependency Bump Stability & Safety Policy](../../repo-governance/development/workflow/dependency-bump-policy.md).

| Dependency | Pinned | Latest | Path | Decision                                         |
| ---------- | ------ | ------ | ---- | ------------------------------------------------ |
| `chrono`   | 0.4.44 | 0.4.44 | A    | Bumped from 0.4.39; patch-only                   |
| `glob`     | 0.3.3  | 0.3.3  | A    | Bumped from 0.3.2; patch-only                    |
| `sha2`     | 0.11.0 | 0.11.0 | A    | Bumped from 0.10.9; only `{Digest, Sha256}` used |
| `tempfile` | 3.27.0 | 3.27.0 | A    | Bumped from 3.14.0; only `TempDir::new()` used   |

## See also

- Go implementation (archived at cutover): [`archived/rhino-cli/README.md`](../../archived/rhino-cli/README.md)
- Migration plan (completed 2026-05-23): [`plans/done/2026-05-23__rhino-cli-rust-rewrite/`](../../plans/done/2026-05-23__rhino-cli-rust-rewrite/README.md)
- Gherkin specs (shared with Go binary): [`specs/apps/rhino/behavior/cli/gherkin/`](../../specs/apps/rhino/behavior/cli/gherkin/)
