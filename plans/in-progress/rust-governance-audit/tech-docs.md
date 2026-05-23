---
title: Rust Governance Audit — Technical Documentation
status: in-progress
created: 2026-05-23
---

# Technical Documentation

## 1. Artefact Inventory

Every Rust-touching artefact in scope, grouped by category. This table is the canonical search space for the audit.

### 1.1 Authoritative standards

| Path                                                                         | Lines (approx) | Audit angle                                                               |
| ---------------------------------------------------------------------------- | -------------- | ------------------------------------------------------------------------- |
| `docs/explanation/software-engineering/programming-languages/rust/README.md` | 200+           | Framework stack currency, MSRV statement                                  |
| `docs/.../rust/coding-standards.md`                                          | 400+           | Style rules, `rust-toolchain.toml` example version                        |
| `docs/.../rust/code-quality-standards.md`                                    | 300+           | Clippy lint set, `forbid(unsafe_code)` clause, `cargo audit`/`cargo deny` |
| `docs/.../rust/type-safety-standards.md`                                     | —              | Newtypes, `unwrap`/`expect` policy                                        |
| `docs/.../rust/memory-management-standards.md`                               | —              | Ownership patterns, `Cow` usage                                           |
| `docs/.../rust/error-handling-standards.md`                                  | —              | `thiserror` vs `anyhow` split                                             |
| `docs/.../rust/concurrency-standards.md`                                     | —              | Tokio patterns; N/A for `rhino-cli`                                       |
| `docs/.../rust/api-standards.md`                                             | —              | Axum patterns; N/A for `rhino-cli`                                        |
| `docs/.../rust/testing-standards.md`                                         | —              | Three-level testing alignment                                             |
| `docs/.../rust/performance-standards.md`                                     | —              | Profile config, benchmarks                                                |
| `docs/.../rust/security-standards.md`                                        | —              | `cargo audit` cadence, supply chain                                       |
| `docs/.../rust/ddd-standards.md`                                             | —              | DDD with types; N/A for tooling CLI                                       |
| `docs/.../rust/build-configuration.md`                                       | —              | Cargo profiles, `target/` layout                                          |
| `docs/.../rust/templates/README.md`                                          | —              | Template index                                                            |

### 1.2 Crate

| Path                                                | Audit angle                                            |
| --------------------------------------------------- | ------------------------------------------------------ |
| `apps/rhino-cli/Cargo.toml`                         | Dependency versions, `rust-version`, profile config    |
| `apps/rhino-cli/Cargo.lock`                         | Transitive integrity (lockfile presence)               |
| `apps/rhino-cli/rust-toolchain.toml`                | Channel pin, components                                |
| `apps/rhino-cli/project.json`                       | Nx target shape, lint command, coverage threshold      |
| `apps/rhino-cli/README.md`                          | Tooling description, dependency status                 |
| `apps/rhino-cli/src/lib.rs`                         | `forbid(unsafe_code)` present                          |
| `apps/rhino-cli/src/main.rs`                        | `forbid(unsafe_code)` present                          |
| `apps/rhino-cli/src/cli/`, `commands/`, `internal/` | Module layout vs `coding-standards.md`                 |
| `apps/rhino-cli/tests/cli/`                         | Integration test shape                                 |
| `apps/rhino-cli/tests/cucumber/`                    | Deferred cucumber harness status                       |
| `apps/rhino-cli/scripts/`                           | Shell-glue scripts (`validate-cross-vendor-parity.sh`) |

### 1.3 Specifications

| Path                                              | Audit angle                                                |
| ------------------------------------------------- | ---------------------------------------------------------- |
| `specs/apps/rhino/README.md`                      | Tooling references (Go residue), command pipeline accuracy |
| `specs/apps/rhino/behavior/cli/gherkin/*.feature` | Scenarios map to live `rhino-cli` commands                 |

### 1.4 Cross-cutting governance

| Path                                                                  | Specific Rust reference       |
| --------------------------------------------------------------------- | ----------------------------- |
| `repo-governance/development/infra/ci-conventions.md:84`              | Rustfmt row in language table |
| `repo-governance/development/infra/nx-targets.md`                     | Three-level targets for Rust  |
| `repo-governance/development/quality/code.md`                         | Cross-language quality bar    |
| `repo-governance/development/quality/three-level-testing-standard.md` | unit/integration/e2e mapping  |
| `repo-governance/development/quality/plan-anti-hallucination.md`      | Rust example claims           |
| `repo-governance/development/workflow/native-first-toolchain.md`      | `rustup` install path         |
| `repo-governance/development/workflow/dependency-bump-policy.md:88`   | Rust listed as non-LTS        |
| `repo-governance/development/workflow/worktree-setup.md`              | `cargo` boot inside worktree  |
| `repo-governance/development/agents/ai-agents.md`                     | `swe-rust-dev` enumeration    |

### 1.5 Agent / Skill

| Path                                           | Audit angle                                 |
| ---------------------------------------------- | ------------------------------------------- |
| `.claude/agents/swe-rust-dev.md`               | Skill references, model tier, color         |
| `.opencode/agents/swe-rust-dev.md`             | Sync parity with `.claude/` source of truth |
| `.claude/skills/swe-programming-rust/SKILL.md` | Authoritative-source link health            |

## 2. Currency Table (web-verified, 2026-05-23)

Source: kickoff `web-research-maker` invocation; full report archived in `generated-reports/rust-governance-audit__kickoff-research__2026-05-23.md` if generated separately.

### 2.1 Toolchain

| Source                           | Declared             | Latest stable                                 | Action                                                                |
| -------------------------------- | -------------------- | --------------------------------------------- | --------------------------------------------------------------------- |
| `Cargo.toml` `rust-version`      | `1.88`               | `1.95.0`                                      | Keep as MSRV (compiler ≥ MSRV is fine); decide if MSRV should advance |
| `rust-toolchain.toml` channel    | `1.95.0`             | `1.95.0`                                      | **Current** [Verified]                                                |
| `docs/.../rust/README.md` body   | "Rust 1.82+"         | —                                             | **Stale** — bump to current MSRV with link                            |
| `coding-standards.md` example    | `channel = "1.82.0"` | —                                             | **Stale** — bump to current MSRV                                      |
| Edition declared in `Cargo.toml` | `2024`               | edition 2024 stable since 1.85.0 (2025-02-20) | **Current** [Verified]                                                |

### 2.2 Dependencies (production)

Confidence labels: [Verified] = research report cites docs.rs; [Outdated] = declared version older than upstream latest.

| Crate            | Pinned  | Latest  | Delta     | Action                                                              |
| ---------------- | ------- | ------- | --------- | ------------------------------------------------------------------- |
| `clap`           | 4.6.1   | 4.6.1   | —         | Keep [Verified]                                                     |
| `serde`          | 1.0.228 | 1.0.228 | —         | Keep [Verified]                                                     |
| `serde_json`     | 1.0.150 | 1.0.150 | —         | Keep [Verified]                                                     |
| `serde_norway`   | 0.9.42  | 0.9.42  | —         | Keep [Verified]                                                     |
| `walkdir`        | 2.5.0   | 2.5.0   | —         | Keep [Verified]                                                     |
| `ignore`         | 0.4.25  | 0.4.25  | —         | Keep [Verified]                                                     |
| `regex`          | 1.12.3  | 1.12.3  | —         | Keep [Verified]                                                     |
| `pulldown-cmark` | 0.13.4  | 0.13.4  | —         | Keep [Verified]                                                     |
| `tree-sitter`    | 0.26.9  | 0.26.9  | —         | Keep [Verified]                                                     |
| `anyhow`         | 1.0.102 | 1.0.102 | —         | Keep [Verified]                                                     |
| `thiserror`      | 2.0.18  | 2.0.18  | —         | Keep [Verified]                                                     |
| `quick-xml`      | 0.40.1  | 0.40.1  | —         | Keep [Verified]                                                     |
| `chrono`         | 0.4.39  | 0.4.44  | +5 patch  | **Bump** — no breaking changes [Verified]                           |
| `glob`           | 0.3.2   | 0.3.3   | +1 patch  | **Bump** [Verified]                                                 |
| `sha2`           | 0.10.9  | 0.11.0  | **major** | **Decide**: bump with API migration OR waiver (Path B/C) [Verified] |

### 2.3 Dependencies (dev)

| Crate        | Pinned | Latest | Delta                            | Action                                            |
| ------------ | ------ | ------ | -------------------------------- | ------------------------------------------------- |
| `cucumber`   | 0.23.0 | 0.23.0 | —                                | Keep [Verified]                                   |
| `assert_cmd` | 2.2.2  | 2.2.2  | —                                | Keep [Verified]                                   |
| `predicates` | 3.1.4  | 3.1.4  | —                                | Keep [Verified]                                   |
| `tempfile`   | 3.14.0 | 3.27.0 | +13 patches, **breaking rename** | **Bump + rename `into_path` → `keep`** [Verified] |

### 2.4 Security advisories

`RustSec` database scan (2026-05-23): **no open advisories** affect any pinned version. `RUSTSEC-2020-0159` (chrono `localtime_r`) was patched in 0.4.20; pinned 0.4.39 is clean.

### 2.5 Style guidance

| Topic                                    | Current consensus                                                                                             | Confidence                                                    |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| `#![forbid(unsafe_code)]` for app crates | Yes; cannot be overridden by `#[allow(unsafe_code)]` (distinguishes from `deny`)                              | [Likely] — community consensus, no first-party normative page |
| Clippy baseline                          | `clippy::all` + opt-in `clippy::pedantic` with targeted `#[allow]`; do NOT enable `clippy::nursery` wholesale | [Verified] — official Clippy docs                             |
| `cargo audit` cadence                    | Per-CI run + dependabot-style watch                                                                           | [Verified]                                                    |

## 3. Contradiction Catalogue

Discovered before delivery; each one becomes a delivery item.

| ID   | Statement A                                                   | Statement B                                                    | Resolution                                                               |
| ---- | ------------------------------------------------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------ |
| C-01 | `docs/.../rust/README.md`: "Rust 1.82+"                       | `Cargo.toml`: `rust-version = "1.88"`                          | Replace prose with link to `Cargo.toml`                                  |
| C-02 | `coding-standards.md`: `channel = "1.82.0"` example           | `rust-toolchain.toml`: `channel = "1.95.0"`                    | Update example to current pin                                            |
| C-03 | `specs/apps/rhino/README.md`: `go test`, `godog`, `.go` files | `apps/rhino-cli/`: Rust crate, `cargo test`, deferred cucumber | Full rewrite to Rust                                                     |
| C-04 | `code-quality-standards.md:246`: "MUST forbid(unsafe_code)"   | No discoverable cross-reference from `quality/code.md`         | Add link from `code.md`                                                  |
| C-05 | `apps/rhino-cli/README.md`: no dependency status section      | Three deps stale per research                                  | Add "Dependency Status" section OR resolve all stale deps                |
| C-06 | `Cargo.toml` `rust-version = "1.88"`                          | `rust-toolchain.toml` `channel = "1.95.0"`                     | **Not** a contradiction; MSRV ≤ pin. Document the distinction in README. |

## 4. Code Structure Best-Practice Checklist

Applied during the structural audit phase against `apps/rhino-cli/src/`.

### 4.1 Module layout

- [ ] `src/lib.rs` exposes only intended public modules
- [ ] `src/main.rs` is a thin binary shim (verified — currently 4 lines)
- [ ] `src/cli/` contains argument parsing only
- [ ] `src/commands/` contains command handlers, each in its own file
- [ ] `src/internal/` contains private implementation; not part of public API
- [ ] No circular module dependencies (`cargo modules` or visual review)
- [ ] No `mod.rs` antipattern for new modules (prefer `module.rs` + `module/` directory per 2018+ edition style)

### 4.2 Public API

- [ ] Every `pub` item in `lib.rs` is intentional (audit each `pub` keyword)
- [ ] No accidental re-exports
- [ ] Public functions have `#[must_use]` where they return a `Result` or significant value
- [ ] No abbreviations in public names (per `coding-standards.md`)

### 4.3 Error handling

- [ ] Library code returns `Result<T, ThisErrorEnum>`
- [ ] Binary entry returns `anyhow::Result<()>` (or sets exit code explicitly — current pattern)
- [ ] No `unwrap()` or `expect()` outside `#[cfg(test)]`
- [ ] No `panic!()` outside `unreachable!()` justified cases

### 4.4 Safety

- [ ] `#![forbid(unsafe_code)]` present in `src/lib.rs` line 1 (done 2026-05-23)
- [ ] `#![forbid(unsafe_code)]` present in `src/main.rs` line 1 (done 2026-05-23)
- [ ] `grep -rE '\bunsafe\b' src/` returns zero matches
- [ ] `cargo clippy --all-targets -- -D warnings -D unsafe_code` exits 0

### 4.5 Lints

- [ ] `[lints.rust]` and `[lints.clippy]` blocks added to `Cargo.toml` with `clippy::all = "deny"` and `clippy::pedantic = "warn"` (cherry-picked allow list documented inline)
- [ ] Or: equivalent `#![warn(...)]` block at crate root with rationale
- [ ] `rustfmt.toml` (or absence) matches `coding-standards.md` guidance

### 4.6 Testing

- [ ] `tests/cli/` contains binary integration tests via `assert_cmd`
- [ ] Unit test coverage ≥ 90% (per `project.json` `test:quick` `--fail-under-lines 90`)
- [ ] Cucumber harness status (deferred) explicitly noted in `apps/rhino-cli/README.md`

### 4.7 Performance

- [ ] `[profile.release]` matches `build-configuration.md` recommendations (current: `opt-level = 3`, `lto = "thin"`, `codegen-units = 1`, `strip = "symbols"` — verify against doc)

### 4.8 Build / Nx

- [ ] All ten `validate:*` Nx targets execute against the current binary surface
- [ ] `nx run rhino-cli:test:quick` covers spec-coverage gate
- [ ] `cargo audit` invocation present (CI or pre-merge target) OR explicit deferral with date

## 5. Methodology

### 5.1 Pair-wise contradiction sweep

For the 14 standards docs × 7 cross-cutting governance files = 98 pair comparisons. Reduce to the ~25 pairs that share at least one keyword (Rust version, framework name, lint set, unsafe policy). Manual review; record findings in a checker-style report at `generated-reports/rust-governance-audit__YYYY-MM-DD.md` with UUID chain.

### 5.2 Re-validation loop

After each remediation commit, re-run:

```bash
nx run rhino-cli:typecheck && \
nx run rhino-cli:lint && \
nx run rhino-cli:test:quick && \
npm run lint:md
```

All four must pass before pushing.

### 5.3 Web-research re-verification

If any dependency action takes more than a week, re-invoke `web-research-maker` to refresh the currency table before committing.

## 6. Out-of-Band Considerations

- **AyoKoding Rust learning content** is separate platform; if any tutorial there contradicts the standards, file a finding linking to this plan but do not edit cross-platform.
- **`archived/rhino-cli/`** (Go preserved copy) is read-only history; do not edit during this audit.
- **Future Rust crates** beyond `rhino-cli` should adopt the same audit checklist; consider promoting Section 4 into a `repo-governance/development/quality/rust-crate-structural-checklist.md` document — flagged as a Phase 5 enhancement.

## 7. References

- [Rust 1.95.0 release blog](https://blog.rust-lang.org/releases/latest/)
- [Rust 1.85.0 / Edition 2024 announcement](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
- [RustSec Advisory Database](https://rustsec.org/advisories/)
- [Clippy Usage docs](https://doc.rust-lang.org/clippy/usage.html)
- [Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md)
- [Rust Coding Standards](../../../docs/explanation/software-engineering/programming-languages/rust/README.md)
- [rhino-cli Rust port plan (done)](../../done/2026-05-23__rhino-cli-rust-rewrite/README.md)
