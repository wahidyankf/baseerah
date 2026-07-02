# Delivery — Unify rhino-cli, SDLC & Repo Structure (Second Pass)

> Legend: every item is `[AI]`-executable (including git-mechanical steps — worktree create/remove,
> push-to-main — per the all-3-repos [AI]-tag rule). Each item names a file/path, a verbatim
> verification command, and an acceptance criterion. **Every item is verified against the working
> tree — no item is ticked on the strength of a prior "done" note.** Phases are gated: do not start a
> phase until the prior phase's gate line passes.

## Phase 0 — Baseline & Re-Audit (ose-public)

- [ ] [AI] `npm install && npm run doctor -- --fix` in ose-public. Acceptance: doctor reports all tools OK.
- [ ] [AI] Confirm green starting point: `nx affected -t test:quick,lint,typecheck --base=HEAD~1` exits 0 (or run-many if no affected). Acceptance: exit 0; resolve any preexisting failure first (root-cause, per repo policy).
- [ ] [AI] Re-run the three-surface audit and commit the output as evidence under this plan folder (`audit/` subdir): rhino-cli `diff -rq`, `jq` target keys/commands, hook diffs, `namedInputs.specs` counts, mandatory-target `jq` loop, `coverage.projects` vs `nx show projects`. Acceptance: `audit/` contains reproducible command output matching [tech-docs §2](./tech-docs.md#2-current-state-verified-2026-07-02); any drift from §2 updates §2.
- [ ] **GATE 0**: baseline green + audit committed.

## Phase 1 — Canonical rhino-cli Synthesis (ose-public)

> All rhino-cli source changes use RED/GREEN/REFACTOR with a companion `.feature` scenario. Verify by
> running the target, not by inspection alone.

- [ ] [AI] Vendor primer's cucumber harness into public: copy `tests/*.rs` (11 `[[test]]` suites), `tests/fixtures`, `tests/golden-master`, `tests/*.rs` step defs, and `specs/apps/rhino/behavior/rhino-cli/gherkin/**` from ose-primer. Align `Cargo.toml` (`cucumber` at canonical `0.23.0`, `tokio`, `thiserror`, `[[test]]` blocks); adapt harness code from `0.22.1` API if needed. Acceptance: `cargo test -p rhino-cli` runs the cucumber suites green in public.
- [ ] [AI] Pull primer's testcoverage module + richer internal tree into public (the 5-file delta + 15 only-in-primer files identified in the audit). Acceptance: `diff -rq` public↔primer `src` shows only intended remaining deltas (none, once trimmed).
- [ ] [AI] Unify lint policy to public's strict form (`missing_errors_doc="deny"`, `[lints.rustdoc]`) across the merged source. Acceptance: `cargo clippy -p rhino-cli -- -D warnings` passes.
- [ ] [AI] Data-drive ALL repo-specific behaviour: move hard-coded env-validation scan paths / domain-areas / ddd-areas out of `application/repo_config/mod.rs` (and the `env:validation` target) into `repo-config.yml` reads. RED: add a `.feature` scenario asserting behaviour comes from config; GREEN: implement; REFACTOR. Acceptance: scenario passes; no repo-specific literal remains in source or `project.json`.
- [ ] [AI] Fix latent bug: naming-validator trigger path `.opencode/agent/` → `.opencode/agents/` (source + any hook grep). RED: add a regression scenario (rename an agent file → validator fails); GREEN: fix path; REFACTOR. Acceptance: scenario red-before/green-after in same commit.
- [ ] [AI] Canonicalize the `repo-config.yml` header comment block (restore the `env-injection` line; one wording). Acceptance: header comment is the canonical form to be copied to siblings.
- [ ] [AI] Freeze canonical artifacts: regenerate `Cargo.lock`; record the canonical `src/`, `Cargo.toml`, `Cargo.lock`, `project.json` (repo-agnostic — zero carve-outs) as the propagation source. Acceptance: `cargo test -p rhino-cli` + golden-master pass.
- [ ] [AI] Governance/docs: update reference + governance docs to describe the byte-identity standard (zero rhino-cli carve-outs) + the divergence policy. Acceptance: `npm run lint:md` + link/heading/mermaid validators pass.
- [ ] **GATE 1**: public rhino-cli green (unit + cucumber + golden-master + clippy strict); canonical artifacts frozen.

## Phase 2 — public Closeout

- [ ] [AI] Wire `namedInputs.specs` on the 11 public projects lacking it (`ayokoding-cli`, `ose-cli`, and the 9 `*-fe-e2e`/`*-www-be-e2e`/`*-app-web-e2e` runners; note `organiclever-be-e2e`/`ose-be-e2e` already have it, proving e2e projects can). Acceptance: `namedInputs.specs` present in all 27 `project.json` files (16 current + 11 added); a specs-only edit marks the owning project affected.
- [ ] [AI] Complete `coverage.projects`: add `fsharp-crane-core`, `web-ui-token`, `organiclever-contracts`, `ose-contracts` (or record why excluded). Acceptance: registry count reconciles with `nx show projects`.
- [ ] [AI] Delete the stale `specs/libs/golang-commons` orphan. Acceptance: `find specs -type d -name gherkin -not -path '*/behavior/*'` returns nothing.
- [ ] [AI] Add the `gherkin-cardinality` step to `pr-quality-gate.yml`'s specs-gate. Acceptance: `actionlint` passes; the specs-gate job lists gherkin-cardinality.
- [ ] [AI] Run public's full affected pre-push + PR-gate command set on a no-op. Acceptance: exit 0.
- [ ] **GATE 2**: public fully at target (self-diff clean; all gaps closed).

## Phase 3 — Propagate to ose-primer

- [ ] [AI] Copy this plan folder into ose-primer `plans/in-progress/`. Acceptance: present.
- [ ] [AI] `npm install && npm run doctor -- --fix` in ose-primer. Acceptance: tools OK.
- [ ] [AI] Copy canonical `apps/rhino-cli` (`src/`, `Cargo.toml`, `Cargo.lock`, `project.json`) from public into primer — a clean copy, zero carve-outs (env paths are data in `repo-config.yml`). Bump cucumber `0.22.1`→`0.23.0`. Acceptance: `diff -rq` public↔primer `src` empty; `diff` of Cargo/project.json empty; `cargo test -p rhino-cli` green.
- [ ] [AI] Set primer's `repo-config.yml` env-validation scan paths + domain/ddd areas as data (its own values). Acceptance: `env staged-guard`/`env validate` behave as before; schema/header identical to public.
- [ ] [AI] Fix `.opencode/agent/`→`.opencode/agents/` bug in primer. Acceptance: regression scenario passes.
- [ ] [AI] Wire `namedInputs.specs` on primer's 5 lacking projects (`clojure-openapi-codegen`, `elixir-cabbage`, `elixir-gherkin`, `elixir-openapi-codegen`, `ts-ui-tokens`). Acceptance: count == primer total (`nx show projects`).
- [ ] [AI] Converge `*.cs/.clj/.dart` to native-tool formatters (`dotnet csharpier format`/`cljfmt fix`/`dart format`); drop `scripts/format-*.sh` from lint-staged. Acceptance: lint-staged entries identical to public modulo language set.
- [ ] [AI] Run primer's affected pre-push + PR-gate on a no-op. Acceptance: exit 0.
- [ ] **GATE 3**: primer rhino-cli byte-identical to public (zero carve-outs); primer at target.

## Phase 4 — Propagate to ose-infra (largest; gated, descopable)

> If the full rhino-cli port proves too large/risky, it may be descoped to a documented divergence
> (README Confirmed Decisions note) **without** unwinding Phases 1–3; the non-CLI infra convergence
> below still lands.

- [ ] [AI] Copy this plan folder into ose-infra (linked worktree; per the bare-repo layout). Acceptance: present.
- [ ] [AI] `npm install && npm run doctor -- --fix` in an ose-infra worktree. Acceptance: tools OK.
- [ ] [AI] **Regenerate `apps/rhino-cli` to canonical**: replace infra's divergent module-naming + internal tree + `cli.rs` with the canonical source; copy `Cargo.toml`/`Cargo.lock`/`project.json` verbatim — **relicense to MIT** (no license carve-out); env-validation scan paths come from `repo-config.yml` (no project.json carve-out). Acceptance: `diff -rq` public↔infra `src` empty; `diff` of Cargo/project.json empty; `cargo test -p rhino-cli` green in the worktree.
- [ ] [AI] Set infra's `repo-config.yml` env-validation scan paths to its IaC globs (`infra/on-premise` terraform/ansible) + its domain/ddd areas — as data. Acceptance: `env validate` scans the IaC paths; schema/header identical to public.
- [ ] [AI] Wire cucumber in infra (canonical `tests/*.rs` + `.feature` tree). Acceptance: cucumber suites pass.
- [ ] [AI] Hooks: convert every `npx nx run rhino-cli:*` / `npm run *` gate wrapper to direct `cargo run --release -- …`; add `#!/usr/bin/env sh` + `set -e` + numbered Step comments to `pre-commit`; move shellcheck/hadolint/actionlint from inline blocks into lint-staged; converge any `*.cs/.clj/.dart` entries to native formatters. Acceptance: `.husky/pre-commit`/`pre-push` diff vs public shows only infra-only IaC steps.
- [ ] [AI] CI: add standalone `compat-min-version` + `env-validate` jobs to `main-ci.yml`; add `gherkin-cardinality` to `pr-quality-gate.yml`; add the `env:` NX_BASE/NX_HEAD block; remove the extra standalone markdown job (fold into gates); lower-kebab every workflow `name:`. Acceptance: `actionlint` passes; job skeletons diff vs public show only IaC/app-set/runner differences.
- [ ] [AI] Add missing targets to the 5 infra projects (`coralpolyp-be-e2e`, `coralpolyp-fe-e2e`: `deps:audit`+`compat:min-version`; `coralpolyp-fe`: `compat:min-version`; `libs/ts-ui`, `libs/ts-ui-tokens`: both). Acceptance: mandatory-target `jq` loop prints no MISSING.
- [ ] [AI] Wire `namedInputs.specs` on infra's remaining project (`ts-ui-tokens`). Acceptance: count == infra total.
- [ ] [AI] Run infra's affected pre-push + PR-gate from a linked worktree (its sole execution context). Acceptance: exit 0.
- [ ] **GATE 4**: infra rhino-cli byte-identical to public (zero carve-outs); infra at target.

## Phase 5 — Cross-Repo Byte-Identity Verification & Archival

- [ ] [AI] rhino-cli byte-identity matrix: `diff -rq apps/rhino-cli/src` empty for public↔primer, public↔infra; `Cargo.toml`/`Cargo.lock`/`project.json` diffs show **no differences** (zero carve-outs). Acceptance: matrix committed; nothing differs.
- [ ] [AI] Target parity: `jq -r '.targets|keys[]' apps/rhino-cli/project.json|sort` identical across all 3; every command string identical. Acceptance: identical.
- [ ] [AI] cucumber parity: `cargo test -p rhino-cli` cucumber suites pass in all 3; `tests/*.rs` + `.feature` trees identical. Acceptance: pass + identical.
- [ ] [AI] SDLC mechanism parity: `.husky/*` diffs show only IaC-only steps; lint-staged identical modulo language set; canonical workflows identical modulo app/language/runner. Acceptance: **zero `⚠️` rows** in the parity table.
- [ ] [AI] Config/targets/specs parity: `namedInputs.specs` count == total in all 3; mandatory-target loop clean in all 3; `repo-config.yml` schema + header + harness list identical; no orphan spec dir. Acceptance: all green.
- [ ] [AI] No-regression: each repo's affected pre-push + PR-gate pass on a no-op. Acceptance: exit 0 in all 3.
- [ ] [AI] Build the Phase 5 parity table (every standardization row ✅, zero `⚠️`; the only allowed-divergence rows are app/language set + the CI runner label). Acceptance: table complete in this doc.
- [ ] [AI] Push each repo's changes, then verify CI green per [ci-post-push-verification](../../../repo-governance/development/workflow/ci-post-push-verification.md) (poll every 2 min; never `gh run watch`). Acceptance: all three CIs green.
- [ ] [AI] Add a dated entry to each repo's `plans/done/README.md` summarizing this pass. Acceptance: entries present.
- [ ] [AI] `git mv` this plan folder to `done/2026-07-DD__unify-rhino-cli-sdlc-parity/` (completion date) in all 3 repos; update `plans/in-progress/README.md`. Acceptance: moved; in-progress list cleared.
- [ ] **GATE 5**: all three repos' `apps/rhino-cli` byte-identical (zero carve-outs); plan archived.

## Notes

- **Commits**: split by concern/domain. Sibling repos carry unrelated WIP — stage explicit paths
  only, never `git add -A`.
- **CI monitoring**: poll every 2 min; never `gh run watch`.
- **Stale-note discipline**: if any item here turns out already-done when reached, verify with the
  named command and tick with the evidence — do not assume from the first plan's record.
