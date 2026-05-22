# Technical Documentation — Rewrite rhino-cli to Rust

## Architecture

### Crate Layout

Single binary crate during migration, mirroring the Go `internal/` layout to make porting mechanical:

```
apps/rhino-cli-rs/                                  # New during migration; renamed to apps/rhino-cli/ at cutover
├── Cargo.toml                                      # Pinned versions, edition 2024, MSRV 1.88
├── rust-toolchain.toml                             # Pin rustc 1.95.0 (see Toolchain Pinning)
├── project.json                                    # Nx targets parity with current rhino-cli/project.json
├── src/
│   ├── main.rs                                     # Entry point — calls cli::run()
│   ├── cli.rs                                      # clap derive root; PersistentPreRun output-format validator
│   ├── commands/                                   # Mirrors apps/rhino-cli/cmd/*.go (one file per command)
│   │   ├── mod.rs
│   │   ├── agents_detect_duplication.rs
│   │   ├── agents_sync.rs
│   │   ├── ...                                     # ~30 files, one per Go cmd/*.go (excluding tests)
│   ├── internal/                                   # Mirrors apps/rhino-cli/internal/<pkg>/
│   │   ├── agents/        mod.rs + ...
│   │   ├── allowlist/     mod.rs + ...
│   │   ├── bcregistry/    mod.rs + ...
│   │   ├── cliout/        mod.rs                   # Output sealed-enum (Text|Json|Markdown)
│   │   ├── docs/          mod.rs + ...
│   │   ├── doctor/        mod.rs + ...
│   │   ├── envbackup/     mod.rs + ...
│   │   ├── fileutil/      mod.rs + ...
│   │   ├── git/           mod.rs + ...
│   │   ├── glossary/      mod.rs + ...
│   │   ├── mermaid/       mod.rs + ...             # tree-sitter integration
│   │   ├── naming/        mod.rs + ...
│   │   ├── repo_governance/ mod.rs + ...
│   │   ├── severity/      mod.rs + ...
│   │   ├── speccoverage/  mod.rs + ...
│   │   └── testcoverage/  mod.rs + ...             # The line-coverage validator (critical path)
│   └── lib.rs                                      # Re-exports for integration-test crate
└── tests/
    ├── cli/                                        # assert_cmd-based integration tests per command
    └── cucumber/                                   # cucumber-rs step definitions consuming specs/apps/rhino/...
        ├── unit_world.rs                           # Mocked-I/O test world
        └── integration_world.rs                    # Real /tmp fixtures test world
```

**Decision**: single binary crate, not a workspace, because every command shares the `cliout` output sealed-enum and command tree. Workspace introduces friction without benefit at this scale. [Judgment call — matches Go's monolithic `internal/` layout.]

### Toolchain Pinning

- `rust-toolchain.toml`: `channel = "1.95.0"`, `components = ["clippy", "rustfmt", "llvm-tools"]`, `profile = "minimal"` [Web-cited — "Rust 1.95.0 released 2026-04-16" — https://blog.rust-lang.org/2026/04/16/Rust-1.95.0/, accessed 2026-05-22].
- `Cargo.toml`: `edition = "2024"`, `rust-version = "1.88"` (MSRV bound by `cucumber 0.23.0` [Web-cited — "cucumber 0.23.0 requires Rust 1.88+" — https://crates.io/crates/cucumber, accessed 2026-05-22]).
- Doctor already probes `rustc` and `cargo-llvm-cov` [Repo-grounded — `apps/rhino-cli/internal/doctor/checker.go:parseRustVersion`].

### Pinned Dependencies (`Cargo.toml`)

All versions are latest stable as of 2026-05-22 [Web-cited — "latest stable versions confirmed at https://crates.io/ registry search" — https://crates.io/, accessed 2026-05-22].

```toml
[dependencies]
clap            = { version = "4.6.1",  features = ["derive", "env"] }
serde           = { version = "1.0.228", features = ["derive"] }
serde_json      = "1.0.150"
serde_norway    = "0.9.42"   # Drop-in serde_yaml replacement; serde_yaml is DEPRECATED [Web-cited — "serde_yaml deprecated, use serde_norway instead" — https://crates.io/crates/serde_norway, accessed 2026-05-22]
walkdir         = "2.5.0"
ignore          = "0.4.25"
regex           = "1.12.3"
pulldown-cmark  = "0.13.4"
tree-sitter     = "0.26.9"
anyhow          = "1.0.102"
thiserror       = "2.0.18"

[dev-dependencies]
cucumber        = "0.23.0"
assert_cmd      = "2.2.2"
predicates      = "3.1.4"

[build-dependencies]
# (none currently planned; tree-sitter grammar fetched via runtime FFI loader)
```

**Notes**:

- `tokio` intentionally omitted. `rhino-cli` is a sync, single-shot CLI; async runtime is dead weight here. Use `std::process::Command` for shell-outs and `rayon` if parallelism becomes a need.
- `serde_yaml` is deprecated since March 2024; `serde_norway` is the actively-maintained drop-in [Web-cited — "serde_yaml deprecated March 2024, serde_norway is the maintained fork" — https://users.rust-lang.org/t/108868, accessed 2026-05-22].
- `tree-sitter` 0.26.9 is current; the `tree-sitter-markdown` grammar is loaded as a separate crate (TBD: pin to the same upstream SHA the Go binding uses; verified before Phase 3 starts).
- Coverage tooling: `cargo-llvm-cov` invoked from Nx targets — not a Cargo dependency. Doctor probes for it.

### CLI Framework — clap

`clap` 4.6.1 with derive macros mirrors Cobra's command tree pattern. Root command takes the same global flags as the Go version (`--verbose`, `--quiet`, `--output`, `--no-color`, `--say`) and uses a `clap::CommandFactory` hook to validate the `--output` value before any subcommand `run()` executes — equivalent to Cobra's `PersistentPreRunE: parseOutputFormat`.

Subcommands are derive-macro structs with `#[derive(Subcommand)]` mirroring the namespace tree (`agents detect-duplication`, `docs validate-naming`, etc.). The exit-code contract matches Go: 0 on success, 1 on validation/runtime error, 2 on usage error.

### Output Sealed-Enum (`cliout`)

The Go `cliout` package uses a sealed-enum pattern. Rust equivalent:

```rust
// src/internal/cliout/mod.rs
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

impl OutputFormat {
    pub fn parse(s: &str) -> Result<Self, anyhow::Error> {
        match s {
            "text"     => Ok(OutputFormat::Text),
            "json"     => Ok(OutputFormat::Json),
            "markdown" => Ok(OutputFormat::Markdown),
            other      => Err(anyhow!("unknown output format {:?}: must be text, json, or markdown", other)),
        }
    }
}
```

Every command's `RunE` analogue takes `&OutputFormat` and dispatches via match — exhaustive at compile time, matching the Go sealed-enum contract.

## BDD Test Wiring

The Gherkin `.feature` files at `specs/apps/rhino/behavior/cli/gherkin/` [Repo-grounded] are the source of truth and must NOT be modified during the rewrite. The Rust port consumes them at both unit and integration levels via `cucumber-rs`:

| Level       | Cucumber `World`   | Step implementation                              | Nx target          |
| ----------- | ------------------ | ------------------------------------------------ | ------------------ |
| Unit        | `UnitWorld`        | Mocked I/O via dependency-injection traits       | `test:unit`        |
| Integration | `IntegrationWorld` | Real filesystem via `tempfile::TempDir` fixtures | `test:integration` |

`cucumber-rs` reads `.feature` files at relative path `../../specs/apps/rhino/behavior/cli/gherkin/`. Step definitions live in `tests/cucumber/`. The unit world replaces I/O functions with mock implementations matching the Go `testable.go` pattern; the integration world drives the binary against real `/tmp` fixtures.

[Repo-grounded — see `specs/apps/rhino/README.md` §Dual Consumption].

## Coverage Validator Port (Critical Path)

The `testcoverage` package contains the line-coverage algorithm that all of `ayokoding-cli`, `ose-cli`, `crane-cli`, `ayokoding-web`, `organiclever-be`, `organiclever-web`, `ose-app-be`, `ose-app-web`, `ose-web`, `wahidyankf-web`, and `rhino-cli` itself depend on for their `test:quick` gates [Repo-grounded — grep across `apps/*/project.json`]. Its algorithm:

1. Parse `cover.out` blocks using the regex `^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$` [Repo-grounded — `apps/rhino-cli/internal/testcoverage/go_coverage.go:13`].
2. Group blocks by file.
3. For each line in each file, collect ALL counts seen across all blocks covering that line.
4. Look up the source-file line to determine if it's executable Go code (skip blank, comment-only, brace-only).
5. Classify: covered if all counts > 0; partial if mixed; missed if all counts == 0.
6. `pct = covered / (covered + partial + missed)`. Partial counts as missed in the denominator.
7. `Passed = pct >= threshold`.

The Rust port replicates this byte-for-byte. The same `coverBlockRe` regex is reused (Rust `regex` crate accepts identical syntax). The source-file line classifier (`isGoCodeLine`) is ported verbatim. Format auto-detection (Go cover.out vs. LCOV vs. JaCoCo vs. Cobertura) keys on the same file content as today's Go implementation.

**Diff-test corpus**: Phase 1 builds a corpus of `cover.out` / `lcov.info` / `cobertura.xml` / `jacoco.xml` files captured from real CI runs, then asserts byte-identical output from both binaries for each corpus entry. Stored at `apps/rhino-cli-rs/tests/cucumber/fixtures/coverage-corpus/` (gitignored if too large; tracked via stable hash list in `tests/cucumber/coverage-corpus.txt`).

## Shadow-Diff Mechanics

For each command being migrated, the shadow-diff harness runs both binaries with the same arguments and compares stdout, stderr, and exit code. Implementation: a small bash script `apps/rhino-cli-rs/scripts/shadow-diff.sh` that:

1. Captures `go run -C ../rhino-cli main.go $@` to `/tmp/go-out.txt` + `/tmp/go-err.txt` + `/tmp/go-exit.txt`.
2. Captures `cargo run --release --manifest-path ../rhino-cli-rs/Cargo.toml -- $@` to corresponding `/tmp/rs-*` files.
3. `diff` each pair; exit 1 on any difference.

During the soak window for each command, the shadow-diff harness is wired into the relevant Nx target so a regression is caught as a CI failure, not a silent drift. After the soak passes for two consecutive clean weeks (Phase 1) or two consecutive clean runs (later phases), the target switches to the Rust binary outright.

## Nx Target Mapping

The new `apps/rhino-cli-rs/project.json` mirrors the Go version's targets [Repo-grounded — `apps/rhino-cli/project.json`]:

| Go target name                     | Rust equivalent command                                                                                                                                                                                                           |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `build`                            | `cargo build --release --manifest-path apps/rhino-cli-rs/Cargo.toml`                                                                                                                                                              |
| `lint`                             | `cargo clippy --manifest-path apps/rhino-cli-rs/Cargo.toml -- -D warnings`                                                                                                                                                        |
| `typecheck`                        | `cargo check --manifest-path apps/rhino-cli-rs/Cargo.toml`                                                                                                                                                                        |
| `test:unit`                        | `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib`                                                                                                                                                                   |
| `test:integration`                 | `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --test '*'`                                                                                                                                                              |
| `test:quick`                       | `cargo llvm-cov --manifest-path apps/rhino-cli-rs/Cargo.toml --lcov --output-path apps/rhino-cli-rs/cover.out && cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- test-coverage validate apps/rhino-cli-rs/cover.out 90` |
| `spec-coverage`                    | `cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- spec-coverage validate specs/apps/rhino/behavior/cli/gherkin apps/rhino-cli-rs --shared-steps`                                                                         |
| `validate:*` (21 validate targets) | Same `cargo run` invocations with `--` separator                                                                                                                                                                                  |

[Note: `cargo llvm-cov` emits LCOV; the validator's auto-detect already handles `.info`/LCOV inputs. Threshold 90 is the agreed unified floor from the recent CLI alignment commit `d4bacc851`.]

## Command-Specific Risks

| Command                                | Risk                                              | Mitigation                                                                                                      |
| -------------------------------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `docs validate-mermaid`                | Tree-sitter grammar version drift Go ↔ Rust       | Pin same `tree-sitter-markdown` grammar SHA; diff-test corpus of known-good and known-bad mermaid blocks        |
| `repo-governance license-audit`        | Reads LICENSE files with subtle Go string parsing | Port verbatim; diff-test against current `LICENSE` files in `apps/*/LICENSE` and `libs/*/LICENSE`               |
| `agents detect-duplication`            | Diff algorithm for fuzzy agent-skill overlap      | Port verbatim; diff-test against current `.claude/agents/*.md` and `.claude/skills/*/SKILL.md`                  |
| `agents sync`                          | Generates `.opencode/agents/*.md` files           | Run sync side-by-side; assert byte-identical generated `.opencode/` directory                                   |
| `doctor`                               | Cross-platform install logic (macOS + Linux)      | Diff-test `--dry-run` against current Go output for each supported platform                                     |
| `git pre-commit`                       | Invoked by `.husky/pre-commit`                    | Switch hook last (Phase 6); shadow-diff in CI runs against PRs before switching the actual hook                 |
| `repo-governance vendor-audit`         | Skips "Platform Binding Examples" headings        | Port the scanner's heading-detection state machine verbatim; corpus diff-test against `AGENTS.md` + `CLAUDE.md` |
| `test-coverage validate` (LCOV/JaCoCo) | Multi-format auto-detection                       | Port format detector first; diff-test against samples from `apps/organiclever-web/`, `ose-app-web/`, etc.       |

## Caller Graph (Migration Targets)

Files that reference `rhino-cli` and must be updated as the corresponding command ports complete [Repo-grounded — `grep -l "rhino-cli\|rhino" .husky/ .github/workflows/ apps/*/project.json`]:

- `.husky/pre-commit` (calls `git pre-commit`)
- `.husky/pre-push` (calls `nx run rhino-cli:validate:*` — flips when targets flip)
- `.github/workflows/pr-validate-links.yml` (direct `go run -C apps/rhino-cli` invocation)
- `.github/workflows/_reusable-test-and-deploy.yml` (invokes `nx run-many -t validate:specs-* --projects=rhino-cli` + `nx run rhino-cli:validate:naming-*` — needs `setup-rust-toolchain` + `Swatinem/rust-cache@v2` when Phase 2+ targets flip) [Repo-grounded — confirmed via grep]
- `.github/workflows/pr-quality-gate.yml` (invokes `nx run-many -t validate:specs-*` — needs `setup-rust-toolchain` when Phase 2+ targets flip) [Repo-grounded — confirmed via grep]
- `.github/workflows/test-and-deploy-organiclever-web-development.yml` (invokes `nx run-many -t validate:specs-*` — needs `setup-rust-toolchain` when Phase 2+ targets flip) [Repo-grounded — confirmed via grep]
- `.github/workflows/test-and-deploy-ose-app-web-development.yml` (invokes `nx run-many -t validate:specs-*` — needs `setup-rust-toolchain` when Phase 2+ targets flip) [Repo-grounded — confirmed via grep]
- `apps/ayokoding-cli/project.json`
- `apps/ayokoding-web/project.json` (DDD bc/ul + test:quick + spec-coverage)
- `apps/crane-cli/project.json` (test-coverage validate + spec-coverage)
- `apps/organiclever-be/project.json`
- `apps/organiclever-web/project.json`
- `apps/ose-app-be/project.json`
- `apps/ose-app-web/project.json`
- `apps/ose-cli/project.json`
- `apps/ose-web/project.json`
- `apps/wahidyankf-web/project.json`
- `apps/rhino-cli/project.json` (self — replaced at cutover)

## Archival Mechanics

At Phase 8 cutover:

1. **Caller-graph empty check** — `grep -rE "apps/rhino-cli/main\.go|go run -C apps/rhino-cli" .github/workflows/ .husky/ apps/*/project.json` returns no matches.
2. **Move Go implementation**: `git mv apps/rhino-cli archived/rhino-cli-go` — preserves history.
3. **Move Rust implementation**: `git mv apps/rhino-cli-rs apps/rhino-cli`.
4. **Update Cargo manifest name + Nx project name**: `name = "rhino-cli"` everywhere.
5. **Update `archived/README.md`**: add table row `| rhino-cli-go/ | 2026-XX-XX | Replaced by Rust implementation | apps/rhino-cli/ |` [Repo-grounded — pattern from existing `archived/README.md` entry].
6. **Drop `golang-commons` from rhino-cli's implicit deps** — Rust version has no Go dependency. (Lib stays for `ayokoding-cli` / `ose-cli`.)
7. **Regenerate `dist/`** to ensure compiled binary is reproducible.
8. **Run full pre-push gate** and two consecutive clean CI runs on `main` before the plan is considered done.

The Go implementation is NEVER edited after archival — `archived/README.md` line 14 [Repo-grounded] makes this an absolute rule.

## Rollback Plan

If a critical regression surfaces post-cutover:

1. `git revert <cutover-commit>` restores `apps/rhino-cli/` (Go) and removes `archived/rhino-cli-go/`.
2. Nx project.json references flip back automatically because the revert restores their old form.
3. The Rust crate remains buildable under `apps/rhino-cli-rs/` — fix forward and re-cutover when ready.

This works because every phase commit ends at a coherent state. No phase requires a forward-only migration.

## Dependencies and Open Items

- **Open**: confirm exact `tree-sitter-markdown` grammar SHA used by the Go binding before Phase 3 starts. [Unverified — needs Phase 3 spike.]
- **Open**: validate that `cargo llvm-cov` LCOV output's path resolution matches the existing validator's path-stripping logic, OR that Go-cover.out output mode also works under `cargo llvm-cov`. [Unverified — needs Phase 0 spike with a minimal Rust crate.]
- **Open**: decide whether `dist/rhino-cli` binary is checked in (current Go pattern via Nx outputs) or built per-CI run (cleaner; relies on `Swatinem/rust-cache@v2`). Default to "built per-CI run" and revisit if cold-cache times become a problem. [Judgment call.]

See [delivery.md](./delivery.md) for the phased execution checklist.
