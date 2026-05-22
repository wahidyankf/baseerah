# Delivery Checklist — Rewrite rhino-cli to Rust

This is the executable phased plan. Each phase ends at a coherent, mergeable state — you can stop after any phase and `main` stays usable.

## Worktree

Worktree path: `worktrees/rhino-cli-rust-rewrite/`

Provision before execution (run from repo root):

```bash
claude --worktree rhino-cli-rust-rewrite
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Environment Setup

- [ ] Provision worktree: `claude --worktree rhino-cli-rust-rewrite` (creates `worktrees/rhino-cli-rust-rewrite/` per the repo `WorktreeCreate` hook). Verify: `test -d worktrees/rhino-cli-rust-rewrite` exits 0.
- [ ] In the **root worktree** (not the new one), initialize toolchain: `npm install && npm run doctor -- --fix`. Verify: `npm run doctor` exits 0 with all required tools reported as PASS.
- [ ] Verify Rust toolchain meets MSRV 1.88: `rustc --version` reports 1.88.0 or higher. If lower, run `rustup update stable`.
- [ ] Verify `cargo-llvm-cov` is installed: `cargo llvm-cov --version` exits 0. If missing: `cargo install cargo-llvm-cov`.
- [ ] Verify existing Go tests pass before any changes: `npx nx run rhino-cli:test:quick` exits 0.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes. This follows the root cause orientation principle — proactively fix preexisting errors encountered during work.

---

## Phase 0 — Foundation

Goal: a buildable `apps/rhino-cli-rs/` crate with an empty CLI surface, wired into Nx and CI. Zero behavior change to the Go binary. End state: `nx run rhino-cli-rs:build` and `nx run rhino-cli-rs:test:quick` exit 0.

- [ ] Create `apps/rhino-cli-rs/` directory. Verify: `test -d apps/rhino-cli-rs` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Create `apps/rhino-cli-rs/Cargo.toml` with the exact `[dependencies]` / `[dev-dependencies]` table from [tech-docs.md §Pinned Dependencies](./tech-docs.md#pinned-dependencies-cargotoml), `edition = "2024"`, `rust-version = "1.88"`, `name = "rhino-cli"`. Verify: `cargo metadata --manifest-path apps/rhino-cli-rs/Cargo.toml --format-version=1` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Create `apps/rhino-cli-rs/rust-toolchain.toml` with `[toolchain] channel = "1.95.0"`, `components = ["clippy", "rustfmt", "llvm-tools"]`, `profile = "minimal"`. Verify: file exists and `rustc --version` inside `apps/rhino-cli-rs/` reports 1.95.0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Create `apps/rhino-cli-rs/src/main.rs` that calls `cli::run()` and exits with the returned code. Verify: `cargo build --manifest-path apps/rhino-cli-rs/Cargo.toml` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Create `apps/rhino-cli-rs/src/cli.rs` with clap derive root command (`name = "rhino-cli"`, version from `Cargo.toml`) and global flags `--verbose`, `--quiet`, `--output`, `--no-color`, `--say` matching `apps/rhino-cli/cmd/root.go:23` [Repo-grounded]. Implement `PersistentPreRun` that validates `--output` ∈ {text, json, markdown}. Verify: `cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- --help` prints the command tree and `cargo run -- --output xml --help` exits 1 with `unknown output format "xml"` on stderr.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/internal/cliout/mod.rs` as a compile-only stub (empty `pub enum OutputFormat {}` with no `parse()` impl). Write the test module inline at the bottom of the file with `#[cfg(test)] mod tests { use super::*; #[test] fn parse_known_formats() { todo!() } }`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml -- cliout::tests::parse_known_formats` exits **non-zero** (test panics or fails — RED state, implementation not yet present).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Implement `OutputFormat` enum + `parse()` in `apps/rhino-cli-rs/src/internal/cliout/mod.rs` matching the Go sealed-enum contract from [tech-docs.md §Output Sealed-Enum](./tech-docs.md#output-sealed-enum-cliout). Replace the `todo!()` stub with real assertions. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml -- cliout::tests::parse_known_formats` exits 0 (text/json/markdown parse correctly, unknown variant errors).
  - _Suggested executor: `swe-rust-dev`_
- [ ] Create `apps/rhino-cli-rs/project.json` with the Nx target table from [tech-docs.md §Nx Target Mapping](./tech-docs.md#nx-target-mapping). Tags: `["type:app", "platform:cli", "lang:rust", "domain:tooling"]`. ImplicitDependencies: `[]` (no Go dependency). Verify: `npx nx show project rhino-cli-rs --json | jq '.targets | keys | length'` returns ≥ 9 (build, lint, typecheck, test:unit, test:integration, test:quick, spec-coverage, run, install).
  - _Suggested executor: `swe-rust-dev`_
- [ ] Add `apps/rhino-cli-rs/.gitignore` with `target/`, `dist/`, `cover.out`, `cover_spec.out`. Verify: `git check-ignore apps/rhino-cli-rs/target` reports the file as ignored.
- [ ] Create `apps/rhino-cli-rs/README.md` with the same purpose/quickstart structure as `apps/rhino-cli/README.md` [Repo-grounded] but pointing at Rust commands. Verify: `npm run lint:md` exits 0.
  - _Suggested executor: `readme-maker`_
- [ ] Smoke-test Nx targets:
  - [ ] `npx nx run rhino-cli-rs:build` exits 0 and produces `apps/rhino-cli-rs/dist/rhino-cli`.
  - [ ] `npx nx run rhino-cli-rs:typecheck` exits 0.
  - [ ] `npx nx run rhino-cli-rs:lint` exits 0.
  - [ ] `npx nx run rhino-cli-rs:test:unit` exits 0 (passes on empty test surface).
  - [ ] `npx nx run rhino-cli-rs:test:quick` exits 0 (90% threshold trivially passed on empty surface).
- [ ] Add CI workflow updates: edit each of the five workflows listed in [tech-docs.md §Caller Graph](./tech-docs.md#caller-graph-migration-targets) that invoke `rhino-cli` via Nx targets — specifically `.github/workflows/_reusable-test-and-deploy.yml`, `.github/workflows/pr-quality-gate.yml`, `.github/workflows/test-and-deploy-organiclever-web-development.yml`, `.github/workflows/test-and-deploy-ose-app-web-development.yml` — to add `actions-rust-lang/setup-rust-toolchain@v1` + `Swatinem/rust-cache@v2` steps before any Nx invocation of `rhino-cli-rs` targets. Verify: commit directly to `main` and monitor CI via `gh run list --branch main --limit 5 --json conclusion,headSha` until all listed workflows report `"conclusion": "success"` for your commit SHA.
  - _Suggested executor: `ci-fixer`_
- [ ] Run local quality gates per "Local Quality Gates (Per Phase)" section below; commit and push.

**Phase 0 commit**: `feat(rhino-cli-rs): scaffold Rust crate, Nx targets, and CI integration`

---

## Phase 1 — Critical-path commands (test-coverage validate + spec-coverage validate)

Goal: port the two commands that every other app's `test:quick` / `spec-coverage` depends on. Both are migrated under a shadow-diff gate, soak for one calendar week, then flip every downstream caller in one batch.

- [ ] Build the shadow-diff harness at `apps/rhino-cli-rs/scripts/shadow-diff.sh` per [tech-docs.md §Shadow-Diff Mechanics](./tech-docs.md#shadow-diff-mechanics). Verify: running it against `--help` for both binaries exits 0 and prints "Shadow diff PASS".
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/internal/testcoverage/go_coverage.rs` as a compile-only stub (`pub fn compute_go_result() { todo!() }`). Port all unit test assertions from `apps/rhino-cli/internal/testcoverage/go_coverage_test.go` into a `#[cfg(test)] mod tests` block in the same file. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib testcoverage::go_coverage::tests` exits **non-zero** (RED state — stubs panic).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Implement `apps/rhino-cli-rs/src/internal/testcoverage/go_coverage.rs` with byte-for-byte algorithm parity from `apps/rhino-cli/internal/testcoverage/go_coverage.go:116` [Repo-grounded]. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib testcoverage::go_coverage::tests` exits 0; all ported unit tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/internal/testcoverage/lcov.rs`, `jacoco.rs`, `cobertura.rs` as compile-only stubs. Write failing unit tests for each format's parse and auto-detect logic. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib testcoverage` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Implement LCOV, JaCoCo, Cobertura format detectors and parsers in `apps/rhino-cli-rs/src/internal/testcoverage/`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib testcoverage` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/commands/test_coverage_validate.rs` as an empty stub command that returns `todo!()`. Write a cucumber step definition in `apps/rhino-cli-rs/tests/cucumber/specs/test_coverage_validate.rs` consuming `specs/apps/rhino/behavior/cli/gherkin/test-coverage-validate.feature` [Repo-grounded]. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --test cucumber -- test-coverage-validate` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `cmd/test_coverage.go` + `cmd/test_coverage_validate.go` → `apps/rhino-cli-rs/src/commands/test_coverage_validate.rs`. Verify: `cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- test-coverage validate apps/rhino-cli/cover.out 90` exits 0 with stdout `Line coverage: ... PASS: ...`, matching the Go binary's output byte-for-byte.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/commands/spec_coverage_validate.rs` and `apps/rhino-cli-rs/src/internal/speccoverage/mod.rs` as compile-only stubs. Write cucumber step definitions consuming `specs/apps/rhino/behavior/cli/gherkin/spec-coverage-validate.feature` [Repo-grounded]. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --test cucumber -- spec-coverage-validate` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `cmd/spec_coverage.go` + `cmd/spec_coverage_validate.go` + `internal/speccoverage/` → `apps/rhino-cli-rs/src/commands/spec_coverage_validate.rs` + `internal/speccoverage/mod.rs`. Verify: `cargo run --manifest-path apps/rhino-cli-rs/Cargo.toml -- spec-coverage validate specs/apps/rhino/behavior/cli/gherkin apps/rhino-cli --shared-steps` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Wire the cucumber-rs harness at `apps/rhino-cli-rs/tests/cucumber/mod.rs`. Add `UnitWorld` (mocked I/O) + `IntegrationWorld` (real `tempfile::TempDir`) per [tech-docs.md §BDD Test Wiring](./tech-docs.md#bdd-test-wiring). Verify: `cargo test --test cucumber` exits 0 with the two Phase 1 feature files (`test-coverage-validate.feature`, `spec-coverage-validate.feature`) reported as 100% passing.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Build the coverage-corpus diff-test at `apps/rhino-cli-rs/tests/cucumber/fixtures/coverage-corpus/` per [tech-docs.md §Coverage Validator Port](./tech-docs.md#coverage-validator-port-critical-path). Capture at least 5 `cover.out` files from real CI runs (one each from ayokoding-cli, ose-cli, organiclever-be, ose-app-be, rhino-cli). Verify: `cargo test --test corpus_diff` exits 0; every corpus entry produces identical output across Go and Rust binaries.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Enable shadow-diff in CI for one week: add a non-blocking CI step that runs `shadow-diff.sh test-coverage validate` and `shadow-diff.sh spec-coverage validate` against every push, posting divergence as an annotation. Verify: at least 5 distinct CI runs report no divergence before flipping callers.
- [ ] Flip downstream callers (all in one commit to avoid mixed-binary windows):
  - [ ] Edit `apps/ayokoding-cli/project.json` line ~19: change `cd ../../apps/rhino-cli && CGO_ENABLED=0 go run main.go test-coverage validate ...` to `cd ../../apps/rhino-cli-rs && cargo run --release --quiet -- test-coverage validate ...`. Verify: `npx nx run ayokoding-cli:test:quick` exits 0.
  - [ ] Edit `apps/ayokoding-cli/project.json` line ~71: flip `spec-coverage` invocation. Verify: `npx nx run ayokoding-cli:spec-coverage` exits 0.
  - [ ] Repeat for `apps/ose-cli/project.json` (test:quick + spec-coverage). Verify: both nx targets exit 0.
  - [ ] Repeat for `apps/ayokoding-web/project.json` (test:quick line 95 + spec-coverage line 115).
  - [ ] Repeat for `apps/crane-cli/project.json` (test-coverage validate line 29 + spec-coverage line 80).
  - [ ] Repeat for `apps/organiclever-be/project.json`, `apps/organiclever-web/project.json`.
  - [ ] Repeat for `apps/ose-app-be/project.json`, `apps/ose-app-web/project.json`.
  - [ ] Repeat for `apps/ose-web/project.json`, `apps/wahidyankf-web/project.json`.
  - [ ] Repeat for `apps/rhino-cli/project.json` (self — switch its own `test:quick` to use the Rust binary for validation).
  - [ ] Verify caller-graph step: `grep -rE "go run.*rhino-cli.*test-coverage|go run.*rhino-cli.*spec-coverage" apps/*/project.json .github/ .husky/` returns no matches.
- [ ] Run local quality gates per "Local Quality Gates (Per Phase)"; commit and push.

**Phase 1 commit**: `feat(rhino-cli-rs): port test-coverage + spec-coverage validators, flip downstream callers`

---

## Phase 2 — Governance suite

Goal: port the `repo-governance` namespace and its 9 sub-validators. Each command flips its Nx `validate:*` target as the port lands.

- [ ] RED: Create `apps/rhino-cli-rs/src/internal/repo_governance/mod.rs` as a compile-only stub. Write unit tests porting assertions from the Go `internal/repo-governance/` test files. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib repo_governance` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/repo-governance/` Go package → `apps/rhino-cli-rs/src/internal/repo_governance/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib repo_governance` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] For each command below, follow the RED→GREEN pattern: (1) create a stub command returning `todo!()` + write cucumber step defs consuming the `.feature` file, verify `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --test cucumber -- <feature-name>` exits non-zero; (2) port the command to green; (3) run shadow-diff for at least 2 CI runs; (4) flip the matching `validate:*` target. Acceptance per command: `npx nx run rhino-cli:validate:<target-name>` exits 0 AND `shadow-diff.sh repo-governance <subcmd>` reports no divergence. Feature files: [Repo-grounded — `specs/apps/rhino/behavior/cli/gherkin/repo-governance-*.feature`].
  - [ ] `repo-governance audit` — uses sub-audit aggregator; port `cmd/governance_audit.go` + `internal/repo-governance/aggregator.go`.
  - [ ] `repo-governance agents-md-size` — port `cmd/governance_agents_md_size.go`.
  - [ ] `repo-governance emoji-audit` — port `cmd/governance_emoji_audit.go`.
  - [ ] `repo-governance frontmatter-audit` — port `cmd/governance_frontmatter_audit.go`.
  - [ ] `repo-governance layer-coherence` — port `cmd/governance_layer_coherence.go`.
  - [ ] `repo-governance license-audit` — port `cmd/governance_license_audit.go`. Corpus diff-test against `apps/*/LICENSE` + `libs/*/LICENSE`.
  - [ ] `repo-governance readme-index-audit` — port `cmd/governance_readme_index_audit.go`.
  - [ ] `repo-governance traceability-audit` — port `cmd/governance_traceability_audit.go`.
  - [ ] `repo-governance vendor-audit` — port `cmd/governance_vendor_audit.go`. Heading-state-machine verbatim port per [tech-docs.md §Command-Specific Risks](./tech-docs.md#command-specific-risks).
  - _Suggested executor for each: `swe-rust-dev`_
- [ ] After all governance commands flip, run `grep -E "go run.*rhino-cli.*(repo-governance|governance)" apps/*/project.json .github/ .husky/`. Verify: no matches.
- [ ] Run local quality gates; commit and push.

**Phase 2 commit**: `feat(rhino-cli-rs): port repo-governance suite, flip validate:* targets`

---

## Phase 3 — Docs validators

Goal: port the `docs` namespace (5 commands). The `validate-mermaid` port is the risk node — it depends on `tree-sitter-markdown` grammar parity.

- [ ] Resolve the `tree-sitter-markdown` open question from [tech-docs.md §Dependencies and Open Items](./tech-docs.md#dependencies-and-open-items). Identify the upstream Git SHA the Go tree-sitter binding uses; pin the same SHA in the Rust `tree-sitter-markdown` crate. Verify: `cargo run -- docs validate-mermaid repo-governance/` produces identical output to the Go binary against a 10-file corpus of known-good and known-bad mermaid blocks.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port each docs command following the RED→GREEN pattern per Phase 2: stub command + failing cucumber step defs first (exits non-zero), then port to green, then shadow-diff, then flip target. Acceptance per command: `npx nx run rhino-cli:validate:<target>` exits 0 AND shadow-diff clean.
  - [ ] `docs validate-frontmatter` — port `cmd/docs_validate_frontmatter.go`.
  - [ ] `docs validate-heading-hierarchy` — port `cmd/docs_validate_heading_hierarchy.go`.
  - [ ] `docs validate-links` — port `cmd/docs_validate_links.go`.
  - [ ] `docs validate-mermaid` — port `cmd/docs_validate_mermaid.go`. After flip, run `npx nx affected -t validate:mermaid` on `main` and verify identical output to Go baseline.
  - [ ] `docs validate-naming` — port `cmd/docs_validate_naming.go`.
  - _Suggested executor for each: `swe-rust-dev`_
- [ ] Verify caller-graph: `grep -E "go run.*rhino-cli.*docs " apps/*/project.json .github/ .husky/` returns no matches.
- [ ] Run local quality gates; commit and push.

**Phase 3 commit**: `feat(rhino-cli-rs): port docs validators (frontmatter/headings/links/mermaid/naming)`

---

## Phase 4 — Agents + workflows

Goal: port the `agents` and `workflows` namespaces. `agents sync` generates `.opencode/agents/*.md` — must be byte-identical.

- [ ] RED: Create `apps/rhino-cli-rs/src/internal/agents/mod.rs` as a compile-only stub. Write unit tests porting assertions from Go `internal/agents/` test files. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib agents` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/agents/` → `apps/rhino-cli-rs/src/internal/agents/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib agents` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/internal/naming/mod.rs` as a compile-only stub. Write unit tests porting assertions from Go `internal/naming/` test files. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib naming` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/naming/` → `apps/rhino-cli-rs/src/internal/naming/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib naming` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port each command following the RED→GREEN pattern per Phase 2 (stub + failing tests first, then implement, then shadow-diff, then flip target):
  - [ ] `agents detect-duplication` — port `cmd/agents_detect_duplication.go`. Diff-test against current `.claude/agents/*.md` + `.claude/skills/*/SKILL.md`.
  - [ ] `agents sync` — port `cmd/agents_sync.go`. After flip, run `cargo run -- agents sync` and assert `.opencode/agents/` is byte-identical to the Go-binary baseline. Verify: `diff -r .opencode/agents/ /tmp/opencode-go-baseline/agents/` reports no differences.
  - [ ] `agents validate-claude` — port `cmd/agents_validate_claude.go`.
  - [ ] `agents validate-naming` — port `cmd/agents_validate_naming.go`.
  - [ ] `agents validate-sync` — port `cmd/agents_validate_sync.go`.
  - [ ] `workflows validate-naming` — port `cmd/workflows_validate_naming.go`.
  - _Suggested executor for each: `swe-rust-dev`_
- [ ] Verify caller-graph: `grep -E "go run.*rhino-cli.*(agents|workflows)" apps/*/project.json .github/ .husky/` returns no matches.
- [ ] Run local quality gates; commit and push.

**Phase 4 commit**: `feat(rhino-cli-rs): port agents + workflows commands`

---

## Phase 5 — Specs + DDD

Goal: port the `specs` and `ddd` namespaces.

- [ ] RED: Create `apps/rhino-cli-rs/src/internal/bcregistry/mod.rs` as a compile-only stub. Write unit tests porting assertions from Go `internal/bcregistry/` test files. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib bcregistry` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/bcregistry/` (bounded-context registry) → `apps/rhino-cli-rs/src/internal/bcregistry/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib bcregistry` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/internal/glossary/mod.rs` as a compile-only stub. Write unit tests porting assertions from Go `internal/glossary/` test files. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib glossary` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/glossary/` → `apps/rhino-cli-rs/src/internal/glossary/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib glossary` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port each command following the RED→GREEN pattern per Phase 2 (stub + failing tests first, then implement, then shadow-diff, then flip target):
  - [ ] `specs validate-adoption` — port `cmd/specs_validate_adoption.go` consuming `specs/apps/rhino/behavior/cli/gherkin/specs/validate-adoption.feature` [Repo-grounded — verified file exists].
  - [ ] `specs validate-counts` — port `cmd/specs_validate_counts.go`.
  - [ ] `specs validate-links` — port `cmd/specs_validate_links.go`.
  - [ ] `specs validate-tree` — port `cmd/specs_validate_tree.go`.
  - [ ] `ddd bc` — port `cmd/ddd_bc.go` + `cmd/ddd_runner.go`. (Called by `apps/ayokoding-web/project.json` test:quick line 93 [Repo-grounded].)
  - [ ] `ddd ul` — port `cmd/ddd_ul.go`. (Called by `apps/ayokoding-web/project.json` line 94.)
  - _Suggested executor for each: `swe-rust-dev`_
- [ ] Verify caller-graph: `grep -E "go run.*rhino-cli.*(specs|ddd)" apps/*/project.json .github/ .husky/` returns no matches.
- [ ] Run local quality gates; commit and push.

**Phase 5 commit**: `feat(rhino-cli-rs): port specs + ddd commands`

---

## Phase 6 — Doctor + env + git

Goal: port the `doctor`, `env`, and `git` namespaces. `doctor` is the biggest single port (cross-platform install logic). `git pre-commit` is invoked directly by the Husky hook.

- [ ] RED: Create `apps/rhino-cli-rs/src/internal/doctor/mod.rs` as a compile-only stub. Write unit tests for every ported function from `apps/rhino-cli/internal/doctor/checker.go` + `fixer.go`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib doctor` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/doctor/` → `apps/rhino-cli-rs/src/internal/doctor/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib doctor` exits 0; every ported function has a Rust unit-test equivalent.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/internal/envbackup/mod.rs` as a compile-only stub. Write unit tests porting assertions from Go `internal/envbackup/` test files. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib envbackup` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/envbackup/` → `apps/rhino-cli-rs/src/internal/envbackup/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib envbackup` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] RED: Create `apps/rhino-cli-rs/src/internal/git/mod.rs` as a compile-only stub. Write unit tests porting assertions from Go `internal/git/` test files. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib git` exits **non-zero** (RED state).
  - _Suggested executor: `swe-rust-dev`_
- [ ] GREEN: Port `internal/git/` → `apps/rhino-cli-rs/src/internal/git/mod.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib git` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port each command following the RED→GREEN pattern per Phase 2 (stub + failing tests first, then implement, then shadow-diff, then flip target):
  - [ ] `doctor` — port `cmd/doctor.go`. Cucumber port of `specs/apps/rhino/behavior/cli/gherkin/doctor.feature`. Diff-test `--dry-run` on a clean container (Linux) and on the maintainer's macOS workstation; assert identical install plan.
  - [ ] `env init` — port `cmd/env_init.go`.
  - [ ] `env backup` — port `cmd/env_backup.go`.
  - [ ] `env restore` — port `cmd/env_restore.go`.
  - [ ] `git pre-commit` — port `cmd/git_pre_commit.go`. Acceptance: shadow-diff against the Go binary across 10 representative pre-commit invocations in CI (mixed JS/Go/markdown changes).
  - _Suggested executor for each: `swe-rust-dev`_
- [ ] Edit `.husky/pre-commit` line 2 [Repo-grounded]: change `CGO_ENABLED=0 go run -C apps/rhino-cli main.go git pre-commit` to `cargo run --release --quiet --manifest-path apps/rhino-cli-rs/Cargo.toml -- git pre-commit`. Verify: a manual commit of any change triggers the hook and exits 0.
- [ ] Verify caller-graph: `grep -E "go run.*rhino-cli.*(doctor|env|git)" apps/*/project.json .github/ .husky/` returns no matches.
- [ ] Run local quality gates; commit and push.

**Phase 6 commit**: `feat(rhino-cli-rs): port doctor + env + git pre-commit, flip Husky hook`

---

## Phase 7 — Test-coverage helpers

Goal: port the remaining `test-coverage` helper commands. None are critical-path callers but they round out the namespace.

- [ ] Port each command following the RED→GREEN pattern per Phase 2 (stub + failing tests first, then implement, then shadow-diff, then flip target):
  - [ ] `test-coverage diff` — RED: Create `apps/rhino-cli-rs/src/commands/test_coverage_diff.rs` as a compile-only stub (`pub fn run() { todo!() }`). Write the test module inline at the bottom of the file with `#[cfg(test)] mod tests { use super::*; #[test] fn diff_smoke() { todo!() } }`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib commands::test_coverage_diff::tests` exits **non-zero** (RED state — stub panics). GREEN: port `cmd/test_coverage_diff.go`; replace `todo!()` stubs with real implementation. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib commands::test_coverage_diff::tests` exits 0.
  - [ ] `test-coverage merge` — RED: Create `apps/rhino-cli-rs/src/commands/test_coverage_merge.rs` as a compile-only stub (`pub fn run() { todo!() }`). Write the test module inline at the bottom of the file with `#[cfg(test)] mod tests { use super::*; #[test] fn merge_smoke() { todo!() } }`. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib commands::test_coverage_merge::tests` exits **non-zero** (RED state — stub panics). GREEN: port `cmd/test_coverage_merge.go`; replace `todo!()` stubs with real implementation. Verify: `cargo test --manifest-path apps/rhino-cli-rs/Cargo.toml --lib commands::test_coverage_merge::tests` exits 0.
  - _Suggested executor for each: `swe-rust-dev`_
- [ ] Verify caller-graph: `grep -E "go run.*rhino-cli.*test-coverage (diff|merge)" apps/*/project.json .github/ .husky/` returns no matches.
- [ ] Run local quality gates; commit and push.

**Phase 7 commit**: `feat(rhino-cli-rs): port test-coverage diff + merge helpers`

---

## Phase 8 — Cutover and archival

Goal: zero Go-binary references on `main`; `apps/rhino-cli/` is the Rust crate; `archived/rhino-cli-go/` preserves the original Go implementation.

- [ ] Caller-graph empty check — `grep -rE "apps/rhino-cli/main\.go|go run -C apps/rhino-cli|go run.*apps/rhino-cli" .github/workflows/ .husky/ apps/*/project.json` returns no matches. If any match found, return to the responsible phase and re-flip the caller; do not proceed.
- [ ] Two consecutive clean CI runs on `main` with zero Go-binary references — verify via `gh run list --branch main --limit 6 --json conclusion,headSha` reporting `"conclusion": "success"` for the two most recent commits.
- [ ] Manual archival ceremony (one commit, do not split):
  - [ ] `git mv apps/rhino-cli archived/rhino-cli-go`. Verify: `test -d archived/rhino-cli-go` AND `test ! -d apps/rhino-cli`.
  - [ ] `git mv apps/rhino-cli-rs apps/rhino-cli`. Verify: `test -d apps/rhino-cli` AND `test -f apps/rhino-cli/Cargo.toml`.
  - [ ] Edit `apps/rhino-cli/Cargo.toml` if needed — no path changes required since `name = "rhino-cli"` was already set in Phase 0.
  - [ ] Edit `apps/rhino-cli/project.json`: ensure `name = "rhino-cli"`, `sourceRoot = "apps/rhino-cli"`; replace any `apps/rhino-cli-rs` substring with `apps/rhino-cli` via `sed -i '' 's|apps/rhino-cli-rs|apps/rhino-cli|g' apps/rhino-cli/project.json`. Verify: `npx nx show project rhino-cli --json | jq -r '.sourceRoot'` returns `apps/rhino-cli`.
  - [ ] Edit every downstream `apps/*/project.json` to remove `apps/rhino-cli-rs` references (replace with `apps/rhino-cli`). Verify: `grep -l "apps/rhino-cli-rs" apps/*/project.json` returns nothing.
  - [ ] Edit `.husky/pre-commit` line 2: replace `apps/rhino-cli-rs` with `apps/rhino-cli`. Verify: `grep "apps/rhino-cli-rs" .husky/*` returns nothing.
  - [ ] Edit `archived/README.md` [Repo-grounded — line 7-9 table format]. Append a row matching the existing entry shape (Directory | Archived date | Reason | Successor). The successor cell should link to `../apps/rhino-cli/` from `archived/README.md`'s location. Verify: `npm run lint:md` exits 0.
    - _Suggested executor: `docs-maker`_
  - [ ] Update `apps/rhino-cli/README.md` to reflect that this is the Rust implementation; link `archived/rhino-cli-go/README.md` for history. Verify: `npm run lint:md` exits 0.
    - _Suggested executor: `readme-maker`_
  - [ ] Edit `AGENTS.md` if any line names `apps/rhino-cli` as Go — verify with `grep -n "rhino-cli" AGENTS.md` and update language.
  - [ ] Edit `CLAUDE.md` for the same.
- [ ] Drop `golang-commons` from `apps/rhino-cli/project.json` `implicitDependencies` — Rust crate has no Go dependency. Verify: `jq '.implicitDependencies' apps/rhino-cli/project.json` returns `[]` or no longer contains `"golang-commons"`.
- [ ] Run the full pre-push gate locally: `npx nx affected -t typecheck lint test:quick spec-coverage --base=origin/main`. Verify: exits 0.
- [ ] Run shadow-diff one final time across the full command surface against the archived Go binary (`go run -C archived/rhino-cli-go main.go ...`). Verify: zero divergences on a representative invocation set (each documented command with default flags, and each command with `-o json` + `-o markdown`).
- [ ] Run local quality gates; commit the archival ceremony as a single commit.

**Phase 8 commit**: `refactor(rhino-cli): archive Go implementation, promote Rust port to canonical apps/rhino-cli/`

---

## Local Quality Gates (Per Phase)

Run at the end of every phase before pushing. Adapt the `--projects` flag to the affected projects in that phase.

- [ ] `npx nx affected -t typecheck` exits 0
- [ ] `npx nx affected -t lint` exits 0
- [ ] `npx nx affected -t test:quick` exits 0
- [ ] `npx nx affected -t spec-coverage` exits 0
- [ ] `npx nx affected -t test:integration` exits 0 (Phase 1+ where integration tests exist)
- [ ] `npm run lint:md` exits 0 (if any markdown changed)
- [ ] Fix ALL failures found — including preexisting issues not caused by your changes
- [ ] Verify all checks pass before pushing

## Post-Push Verification

After each phase commit lands on `main`:

- [ ] Push changes to `main` (direct, per Trunk Based Development)
- [ ] Monitor GitHub Actions workflows for the push via `gh run list --branch main --limit 10`
- [ ] Verify all CI checks pass for your commit SHA
- [ ] If any CI check fails, fix immediately and push a follow-up commit on `main`
- [ ] Do NOT proceed to the next phase until CI is green

## Manual API Verification (Phase 6 + Phase 8)

For Phase 6 (Husky hook flip) and Phase 8 (cutover):

- [ ] Make a trivial commit in a scratch worktree (e.g., add a space to `README.md`); verify the pre-commit hook fires and exits 0.
- [ ] Run `npx nx run-many --targets=test:quick --projects=ayokoding-cli,ose-cli,rhino-cli` — verify all three exit 0.
- [ ] Spot-check at least 3 randomly-selected `apps/*/project.json` `spec-coverage` targets — verify each exits 0.

## Commit Guidelines

- [ ] Commit changes thematically — group related changes into logically cohesive commits per the phase plan above
- [ ] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [ ] Split different domains/concerns into separate commits within a phase if a phase produces large diffs
- [ ] Do NOT bundle unrelated fixes into a single commit

## Plan Archival

- [ ] Verify ALL delivery checklist items above are ticked
- [ ] Verify ALL quality gates pass (local + CI) on the final cutover commit
- [ ] Move plan folder from `plans/in-progress/rhino-cli-rust-rewrite/` to `plans/done/YYYY-MM-DD__rhino-cli-rust-rewrite/` via `git mv` using today's completion date
- [ ] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] Update any other READMEs that reference this plan (search via `grep -rl "rhino-cli-rust-rewrite" plans/ docs/ repo-governance/`)
- [ ] Commit: `chore(plans): move rhino-cli-rust-rewrite to done`
