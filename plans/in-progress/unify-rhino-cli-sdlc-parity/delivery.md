# Delivery — Unify rhino-cli, SDLC & Repo Structure (Second Pass)

> **Legend** — every item in this checklist is `[AI]`-executable, including git-mechanical steps
> (worktree create/remove, commit-and-push-to-main) per the all-3-repos `[AI]`-tag rule — this plan
> has zero `[HUMAN]` gates. Each item names a file/path, a verbatim verification command, and an
> acceptance criterion. **Every item is verified against the working tree — no item is ticked on the
> strength of a prior "done" note.** Phases are gated: do not start a phase until the prior phase's
> `### Phase N Gate` passes.

<!-- -->

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root-cause orientation — proactively fix preexisting errors encountered during work).

<!-- -->

> **Multi-repo note**: this plan is authored in `ose-public`. Phases 0–2 execute here. Phases 3–4
> execute in `ose-primer` and `ose-infra` respectively — each begins by copying this plan folder into
> the sibling repo (per the
> [multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)).
> `ose-infra` is a normal, non-bare repository — `git status`/`git reset --hard`/`git revert` all
> work at the top level exactly as in `ose-public`/`ose-primer`. It is worked via the same
> `worktrees/<name>/` convention as the other two repos (see the `## Worktree` section above); no
> bare-repo-specific handling applies.

## Worktree

Worktree path: `worktrees/unify-rhino-cli-sdlc-parity/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree unify-rhino-cli-sdlc-parity
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed. This worktree hosts Phases 0–2
(`ose-public` execution); Phases 3–4 operate in their own sibling-repo worktrees per the multi-repo
note above.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0 — Baseline & Re-Audit (ose-public)

- [ ] [AI] `npm install && npm run doctor -- --fix` in ose-public. Acceptance: doctor reports all tools OK.
- [ ] [AI] Confirm green starting point: `nx affected -t test:quick,lint,typecheck,specs:behavior:coverage --base=HEAD~1` exits 0 (or run-many if no affected). Acceptance: exit 0; resolve any preexisting failure first (root-cause, per repo policy).
- [ ] [AI] Re-run the three-surface audit and commit the output as evidence under this plan folder (`audit/` subdir): rhino-cli `diff -rq`, `jq` target keys/commands, hook diffs, `namedInputs.specs` counts, mandatory-target `jq` loop, `coverage.projects` vs `nx show projects`. Acceptance: `audit/` contains reproducible command output matching [tech-docs §2](./tech-docs.md#2-current-state-verified-2026-07-02); any drift from §2 updates §2.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `nx affected -t test:quick,lint,typecheck,specs:behavior:coverage --base=HEAD~1` — exits 0 (or the `run-many` equivalent on a fresh clone).
- [ ] [AI] `test -d plans/in-progress/unify-rhino-cli-sdlc-parity/audit` — the committed audit evidence directory exists with reproducible command output.
- [ ] [AI] `git status` — clean working tree (no stray edits beyond the committed audit evidence).

> **Pause Safety**: baseline green, audit evidence committed, no source changes applied yet. Safe to stop. To resume: `nx affected -t test:quick,lint,typecheck,specs:behavior:coverage --base=HEAD~1`.

## Phase 1 — Canonical rhino-cli Synthesis (ose-public)

> All rhino-cli source changes use RED/GREEN/REFACTOR with a companion `.feature` scenario. Verify by
> running the target, not by inspection alone.

- [ ] [AI] Copy `apps/rhino-cli/tests/*.rs` (11 `[[test]]` suites + their step-def files) from `ose-primer` into `ose-public`'s `apps/rhino-cli/tests/`. Acceptance: `find apps/rhino-cli/tests -name '*.rs' | wc -l` in public matches the count copied from primer.
- [ ] [AI] Copy `apps/rhino-cli/tests/fixtures/**` from `ose-primer` into `ose-public`. Acceptance: `diff -rq apps/rhino-cli/tests/fixtures <path-to-ose-primer>/apps/rhino-cli/tests/fixtures` is empty.
- [ ] [AI] Copy `apps/rhino-cli/tests/golden-master/**` from `ose-primer` into `ose-public`. Acceptance: `diff -rq apps/rhino-cli/tests/golden-master <path-to-ose-primer>/apps/rhino-cli/tests/golden-master` is empty.
- [ ] [AI] Copy `specs/apps/rhino/behavior/rhino-cli/gherkin/**` from `ose-primer` into `ose-public`. Acceptance: `diff -rq specs/apps/rhino/behavior/rhino-cli/gherkin <path-to-ose-primer>/specs/apps/rhino/behavior/rhino-cli/gherkin` is empty.
- [ ] [AI] Align `apps/rhino-cli/Cargo.toml`: set `cucumber` to canonical `0.23.0`, add `tokio`/`thiserror` if missing, add the copied `[[test]]` blocks; adapt harness code from primer's `0.22.1` cucumber API where the API differs. Acceptance: `cargo test -p rhino-cli` runs the cucumber suites green in public.
- [ ] [AI] Pull primer's testcoverage module + richer internal tree into public (the 5-file delta + 14 only-in-primer files identified in the audit, under `apps/rhino-cli/src/` — **excluding** `commands/specs_validate_links.rs`, which is dead code: undeclared by any `mod specs_validate_links;` anywhere in primer's `apps/rhino-cli/src/`, and referenced only by the `specs_validate_links_no_longer_parses` test asserting the `specs validate links` CLI command was already removed; drop it rather than carry it into the canonical). Acceptance: `diff -rq` public↔primer `apps/rhino-cli/src` shows only the intended remaining deltas (`specs_validate_links.rs` excluded by design, not copied; zero unintended deltas); `grep -rl 'specs_validate_links' apps/rhino-cli/src` in public returns nothing.
- [ ] [AI] Unify lint policy to public's strict form (`missing_errors_doc="deny"`, `[lints.rustdoc]`) in `apps/rhino-cli/Cargo.toml` across the merged source. Acceptance: `cargo clippy -p rhino-cli -- -D warnings` passes.
- [ ] [AI] **RED**: add a `.feature` scenario (+ step def) in `specs/apps/rhino/behavior/rhino-cli/gherkin/` asserting `env validate` runs `validate_terraform`/`validate_ansible` when a repo declares `kind: terraform`/`kind: ansible` surfaces in `repo-config.yml`, and skips them by data (not by stub) when no such surfaces are declared — command: `cargo test -p rhino-cli` — acceptance: new scenario fails (public's `application/env/validate.rs` dispatcher only matches a hard-coded `"app"` string and `eprintln!`s+skips any other kind; no `validate_terraform`/`validate_ansible` functions exist yet).
  - **Gherkin (binds) →** "IaC env-validation is preserved in the canonical"

    ```gherkin
    Scenario: IaC env-validation is preserved in the canonical
      Given ose-infra declares terraform and ansible surfaces in repo-config.yml
      When env validate runs
      Then validate_terraform and validate_ansible execute and report drift
      And ose-public and ose-primer, which declare no such surfaces, skip validation by data, not by stub
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: port `validate_terraform` and `validate_ansible` (+ their `#[cfg(test)] mod {terraform,ansible}_validator` unit-test modules, ~90 lines each) from `ose-infra`'s `apps/rhino-cli/src/application/env/validate.rs` into public's copy of the same file; replace public's bare `kind: String` matched via `.as_str()` (`"app"`-only) with infra's typed `SurfaceKind` enum (`App`/`Terraform`/`Ansible`) and generalize `validate_all`'s dispatch to match all three variants — command: `cargo test -p rhino-cli` — acceptance: new scenario passes; `cargo test -p rhino-cli terraform_validator::` and `cargo test -p rhino-cli ansible_validator::` (the ported test modules) both pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: remove the now-superseded `// activate when IaC is added` comment stubs in `application/env/validate.rs` and align doc-comments/naming between the ported `SurfaceKind` dispatch and the surrounding module — command: `cargo clippy -p rhino-cli -- -D warnings` — acceptance: clippy passes; all unit + cucumber tests still green; a manual check confirms public's/primer's `repo-config.yml` (declaring zero `terraform`/`ansible` surfaces) still produce zero findings for those kinds — i.e., the real validators no-op there by data, not by stub.
- [ ] [AI] **RED**: add a `.feature` scenario (+ step def) in `specs/apps/rhino/behavior/rhino-cli/gherkin/` asserting rhino-cli's env-validation scan paths / domain-areas / ddd-areas are read from `repo-config.yml`, not hard-coded — command: `cargo test -p rhino-cli` — acceptance: new scenario fails (behaviour is still hard-coded in `apps/rhino-cli/src/application/repo_config/mod.rs`).
  - **Gherkin (binds) →** "Repo-specific behaviour is data-driven, not hard-coded"

    ```gherkin
    Scenario: Repo-specific behaviour is data-driven, not hard-coded
      Given rhino-cli's repo-specific behaviour (env globs, domain/ddd areas)
      When rhino-cli runs
      Then it reads that behaviour from repo-config.yml, not from source hard-coded per repo
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: implement the config read in `apps/rhino-cli/src/application/repo_config/mod.rs` and the `env:validation` target, moving env-validation scan paths / domain-areas / ddd-areas out of hard-coded literals into `repo-config.yml` reads — command: `cargo test -p rhino-cli` — acceptance: new scenario passes; a grep for the removed hard-coded literal in `apps/rhino-cli/src` and `apps/rhino-cli/project.json` returns nothing.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: clean up `apps/rhino-cli/src/application/repo_config/mod.rs` (remove dead code left over from the old hard-coded implementation, tidy naming) — command: `cargo clippy -p rhino-cli -- -D warnings` — acceptance: clippy passes; all unit + cucumber tests still green.
- [ ] [AI] **RED**: add a regression `.feature` scenario (+ step def) asserting the naming validator fires on an invalid agent-file rename and that no trigger path references the singular `.opencode/agent/` — command: `cargo test -p rhino-cli` — acceptance: new scenario fails (the trigger path is currently the buggy singular form).
  - **Gherkin (binds) →** "The agent-naming validator fires"

    ```gherkin
    Scenario: The agent-naming validator fires
      Given an agent file renamed to an invalid suffix
      When the naming validator runs (triggered on .opencode/agents/ changes)
      Then it detects the invalid name and fails
      And no trigger path references the singular .opencode/agent/
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: fix the trigger path in `apps/rhino-cli/src/` (the naming-validator's watched-path config) and the hook grep in `.husky/pre-push` (`.opencode/agent/` → `.opencode/agents/`) — command: `cargo test -p rhino-cli` — acceptance: new scenario passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: grep the full source tree for any other stray singular `.opencode/agent/` reference and correct it — command: `grep -rn '\.opencode/agent/' apps/rhino-cli/src .husky/` — acceptance: zero matches (only the plural `.opencode/agents/` remains).
- [ ] [AI] Canonicalize the `repo-config.yml` header comment block (restore the `env-injection` line; one wording). Acceptance: header comment is the canonical form to be copied to siblings.
- [ ] [AI] Freeze canonical artifacts: regenerate `Cargo.lock`; record the canonical `src/`, `Cargo.toml`, `Cargo.lock`, `project.json` (repo-agnostic — zero carve-outs) as the propagation source. Acceptance: `cargo test -p rhino-cli` + golden-master pass.
- [ ] [AI] Update `docs/reference/sdlc-gate-standard.md` §Divergence Policy (and §Target Standard) to describe the rhino-cli byte-identity standard (zero carve-outs — `src/`/`Cargo.toml`/`Cargo.lock`/`project.json` identical across all three repos) and the updated divergence-policy boundary (app/language set + the CI runner label are the only sanctioned divergence). Acceptance: `npm run lint:md` exits 0; `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md mermaid validate docs/reference/sdlc-gate-standard.md` and `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md heading-hierarchy validate docs/reference/sdlc-gate-standard.md` both exit 0.
- [ ] [AI] Author the rhino-cli byte-identity + zero-carve-out standard in `repo-governance/development/infra/nx-targets.md` (new subsection under "Cache and Inputs Convention" titled "Cross-Repo rhino-cli Byte-Identity Standard"): state verbatim that (1) `apps/rhino-cli`'s `src/`, `Cargo.toml`, `Cargo.lock`, and `project.json` MUST be byte-identical across `ose-public`/`ose-primer`/`ose-infra` with zero carve-outs, (2) every Nx-registered project in every repo (per `nx show projects`, including the contracts projects under `specs/apps/*/containers/contracts/`) MUST declare `namedInputs.specs`, and (3) rhino-cli's own behaviour MUST be cucumber-covered in all three repos. Acceptance: `npm run lint:md` exits 0; `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md heading-hierarchy validate repo-governance/development/infra/nx-targets.md` exits 0; the new subsection states all three rules verbatim.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Add a one-line pointer to `AGENTS.md`'s "Related Repositories" section noting `apps/rhino-cli` is required to be byte-identical (zero carve-outs) across all three repos per `docs/reference/sdlc-gate-standard.md`, then run `npm run generate:bindings` to re-sync `.opencode/`/`.amazonq/`. Acceptance: `npm run lint:md` exits 0; `git status --porcelain` after `generate:bindings` shows no drift beyond the intended `AGENTS.md`/binding-mirror changes.
  - _Suggested executor: `repo-rules-maker`_

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `cargo test -p rhino-cli` — unit + cucumber + golden-master suites all pass.
- [ ] [AI] `cargo clippy -p rhino-cli -- -D warnings` — exits 0 (strict lint policy).
- [ ] [AI] `nx run rhino-cli:specs:behavior:coverage` — exits 0.
- [ ] [AI] `diff -rq apps/rhino-cli/src <path-to-ose-primer>/apps/rhino-cli/src` — shows only the one intended remaining delta (`commands/specs_validate_links.rs`, deliberately excluded as dead code — see Phase 1's pull-forward item — which Phase 3's full-copy propagation naturally removes from primer); zero unintended deltas; confirms the canonical artifacts are frozen and ready to propagate.
- [ ] [AI] `cargo test -p rhino-cli terraform_validator::` and `cargo test -p rhino-cli ansible_validator::` — both pass, confirming infra's real Terraform/Ansible env-drift validators (ported into the canonical below) are present and functional, not just aggregate-green.

> **Pause Safety**: public's rhino-cli synthesis is complete and green; canonical artifacts (`src/`, `Cargo.toml`, `Cargo.lock`, `project.json`) are frozen and ready to copy into the siblings. Safe to stop. To resume: `cargo test -p rhino-cli`.

## Phase 2 — public Closeout

- [ ] [AI] Wire `namedInputs.specs` on the 13 public projects lacking it (`ayokoding-cli`, `ose-cli`, the 9 `*-fe-e2e`/`*-www-be-e2e`/`*-app-web-e2e` runners, plus the 2 contracts projects `organiclever-contracts` (`specs/apps/organiclever/containers/contracts/project.json`) and `ose-contracts` (`specs/apps/ose/containers/contracts/project.json`) — both Nx-registered but outside `apps/`/`libs/`, invisible to a `find apps libs` scan; note `organiclever-be-e2e`/`ose-be-e2e` already have it, proving e2e projects can). Acceptance: `for p in $(npx nx show projects --json | jq -r '.[]'); do npx nx show project "$p" --json | jq -e '.namedInputs.specs' >/dev/null || echo "MISSING: $p"; done` prints nothing (all 29 Nx-registered projects, including both contracts projects, carry `namedInputs.specs`); a specs-only edit marks the owning project affected.
- [ ] [AI] Complete `coverage.projects` in `repo-config.yml`: add `fsharp-crane-core`, `web-ui-token`, `organiclever-contracts`, `ose-contracts` (or record why excluded). Acceptance: the entry count under `coverage.projects` reconciles with `nx show projects` (29 total, minus any documented exclusion).
- [ ] [AI] Delete the stale `specs/libs/golang-commons` orphan directory. Acceptance: `find specs -type d -name gherkin -not -path '*/behavior/*'` returns nothing.
- [ ] [AI] Add the `gherkin-cardinality` step to `.github/workflows/pr-quality-gate.yml`'s specs-gate job. Acceptance: `actionlint .github/workflows/pr-quality-gate.yml` passes; the specs-gate job lists `specs gherkin-cardinality validate`.
- [ ] [AI] Run `sh .husky/pre-push` from the repo root on the closed-out tree (simulates the full local pre-push gate; the PR-gate CI workflow runs the equivalent `nx affected` command set, verified separately in Phase 5's CI monitoring step). Acceptance: exits 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `for p in $(npx nx show projects --json | jq -r '.[]'); do npx nx show project "$p" --json | jq -e '.namedInputs.specs' >/dev/null || echo "MISSING: $p"; done` — prints nothing (29/29, including `organiclever-contracts`/`ose-contracts`).
- [ ] [AI] `nx affected -t typecheck,lint,test:quick,specs:behavior:coverage --base=HEAD~1` — exits 0.
- [ ] [AI] `sh .husky/pre-push` — exits 0.
- [ ] [AI] `find specs -type d -name gherkin -not -path '*/behavior/*'` — returns nothing.

> **Pause Safety**: ose-public is fully at target — self-diff clean, all Phase 2 gaps closed. Safe to stop. To resume: `sh .husky/pre-push`.

## Phase 3 — Propagate to ose-primer

- [ ] [AI] Copy this plan folder into ose-primer `plans/in-progress/`. Acceptance: present.
- [ ] [AI] `npm install && npm run doctor -- --fix` in ose-primer. Acceptance: tools OK.
- [ ] [AI] Copy canonical `apps/rhino-cli` (`src/`, `Cargo.toml`, `Cargo.lock`, `project.json`) from public into primer — a clean copy, zero carve-outs (env paths are data in `repo-config.yml`). Bump cucumber `0.22.1`→`0.23.0`. Acceptance: `diff -rq` public↔primer `src` empty; `diff` of Cargo/project.json empty; `cargo test -p rhino-cli` green.
- [ ] [AI] Set primer's `repo-config.yml` env-validation scan paths + domain/ddd areas as data (its own values). Acceptance: `env staged-guard`/`env validate` behave as before; schema/header identical to public.
- [ ] [AI] Fix `.opencode/agent/`→`.opencode/agents/` bug in primer. Acceptance: the regression scenario bound in Phase 1's RED step passes.
- [ ] [AI] Wire `namedInputs.specs` on primer's 6 lacking projects (`clojure-openapi-codegen`, `elixir-cabbage`, `elixir-gherkin`, `elixir-openapi-codegen`, `ts-ui-tokens`, plus the contracts project `crud-contracts` at `specs/apps/crud/containers/contracts/project.json` — Nx-registered but outside `apps/`/`libs/`, invisible to a `find apps libs` scan). Acceptance: `for p in $(npx nx show projects --json | jq -r '.[]'); do npx nx show project "$p" --json | jq -e '.namedInputs.specs' >/dev/null || echo "MISSING: $p"; done` prints nothing (count == primer's full `nx show projects` total, 26).
- [ ] [AI] Converge `*.cs/.clj/.dart` to native-tool formatters (`dotnet csharpier format`/`cljfmt fix`/`dart format`) in primer's `package.json` lint-staged config; drop `scripts/format-*.sh`. Acceptance: lint-staged entries identical to public modulo language set.
- [ ] [AI] Copy the canonicalized `repo-governance/development/infra/nx-targets.md` "Cross-Repo rhino-cli Byte-Identity Standard" subsection and the `AGENTS.md` "Related Repositories" pointer (both authored in Phase 1) into primer, substituting only repo-name references, then run `npm run generate:bindings`. Acceptance: `diff` of the subsection's prose against public's (modulo repo-name substitution) shows no unintended wording drift; `npm run lint:md` exits 0 in primer.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Run `sh .husky/pre-push` from the primer repo root on the propagated tree (simulates the full local pre-push gate; the PR-gate CI workflow runs the equivalent `nx affected` command set, verified separately in Phase 5's CI monitoring step). Acceptance: exits 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `diff -rq apps/rhino-cli/src` (public vs primer) — empty; `diff` of `Cargo.toml`/`Cargo.lock`/`project.json` — empty (zero carve-outs).
- [ ] [AI] `cargo test -p rhino-cli` in primer — cucumber suites pass.
- [ ] [AI] `sh .husky/pre-push` in primer — exits 0.
- [ ] [AI] `nx run rhino-cli:specs:behavior:coverage` in primer — exits 0.

> **Pause Safety**: primer's rhino-cli is byte-identical to public and primer is at target. Safe to stop. To resume: `sh .husky/pre-push` (primer repo root).

## Phase 4 — Propagate to ose-infra (largest; gated, descopable)

> If the full rhino-cli port proves too large/risky, it may be descoped to a documented divergence
> (README Confirmed Decisions note) **without** unwinding Phases 1–3; the non-CLI infra convergence
> below still lands.

- [ ] [AI] Copy this plan folder into ose-infra `plans/in-progress/`. Acceptance: present.
- [ ] [AI] `npm install && npm run doctor -- --fix` in ose-infra. Acceptance: tools OK.
- [ ] [AI] **Regenerate `apps/rhino-cli` to canonical**: replace infra's divergent module-naming + internal tree + `cli.rs` with the canonical source (which now includes infra's own `validate_terraform`/`validate_ansible` implementations, ported into the canonical in Phase 1 — this is a like-for-like replacement, not a deletion); copy `Cargo.toml`/`Cargo.lock`/`project.json` verbatim — **relicense to MIT** (no license carve-out); env-validation scan paths come from `repo-config.yml` (no project.json carve-out). Acceptance: `diff -rq` public↔infra `src` empty; `diff` of Cargo/project.json empty; `cargo test -p rhino-cli` green in infra; `cargo test -p rhino-cli terraform_validator::` and `cargo test -p rhino-cli ansible_validator::` (the canonical IaC validator test modules) both pass in infra, confirming the real Terraform/Ansible drift-detection logic is present and functional post-regeneration, not silently replaced by the pre-Phase-1 stub.
- [ ] [AI] Set infra's `repo-config.yml` env-validation scan paths to its IaC globs (`infra/on-premise` terraform/ansible) + its domain/ddd areas — as data (the `kind: terraform`/`kind: ansible` surfaces are already declared in infra's `repo-config.yml` today). Acceptance: `env validate` scans the IaC paths and `validate_terraform`/`validate_ansible` execute against them (per the now-canonical data-driven `SurfaceKind` dispatch from Phase 1); schema/header identical to public.
- [ ] [AI] Wire cucumber in infra (canonical `tests/*.rs` + `.feature` tree, copied verbatim from the now-canonical public tree). Acceptance: cucumber suites pass.
- [ ] [AI] Convert every `npx nx run rhino-cli:*` / `npm run *` gate wrapper in `.husky/pre-commit` and `.husky/pre-push` to a direct `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- <command>` invocation. Acceptance: a grep for rhino-cli `npx nx run`/`npm run` wrapper lines in `.husky/pre-commit` and `.husky/pre-push` returns nothing (IaC-only lines excluded).
- [ ] [AI] Add `#!/usr/bin/env sh` + `set -e` + numbered Step comments to `.husky/pre-commit` (matching public's format). Acceptance: `head -2 .husky/pre-commit` shows the shebang + `set -e`; each stage is a numbered `# Step N:` comment.
- [ ] [AI] Move shellcheck/hadolint/actionlint from inline `.husky/pre-commit` shell blocks into `package.json`'s lint-staged file-type entries (`*.sh`, `**/Dockerfile*`, `.github/workflows/*.{yml,yaml}`). Acceptance: `grep -c 'shellcheck\|hadolint\|actionlint' .husky/pre-commit` returns 0; the three lint-staged entries exist in `package.json`.
- [ ] [AI] Converge `*.cs/.clj/.dart` lint-staged entries in infra's `package.json` to native-tool formatters (`dotnet csharpier format`/`cljfmt fix`/`dart format`); drop `scripts/format-*.sh`. Acceptance: lint-staged entries identical to public modulo language set.
- [ ] [AI] Add a standalone `compat-min-version` job to `.github/workflows/main-ci.yml`. Acceptance: `actionlint .github/workflows/main-ci.yml` passes; the job is present and named lower-kebab.
- [ ] [AI] Add a standalone `env-validate` job to `.github/workflows/main-ci.yml`. Acceptance: `actionlint .github/workflows/main-ci.yml` passes; the job is present and named lower-kebab.
- [ ] [AI] Verify `.github/workflows/pr-quality-gate.yml`'s specs-gate job already runs gherkin-cardinality validation (confirmed present as `npx nx run rhino-cli:specs:gherkin-cardinality-validation`); align only if its invocation form diverges from the canonical form (the raw `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- specs gherkin-cardinality validate` form appears in this workflow's separate `markdown-per-file` job, not in `specs-gate`). Acceptance: `grep -n gherkin-cardinality .github/workflows/pr-quality-gate.yml` shows the step present; `actionlint .github/workflows/pr-quality-gate.yml` passes.
- [ ] [AI] Convert infra's 6 duplicated per-job `env: NX_BASE`/`NX_HEAD` blocks in `.github/workflows/pr-quality-gate.yml` (`detect`, `shellcheck`, `hadolint`, `actionlint`, `typescript`, `rust` jobs) into a single workflow-level `env:` block, matching public's mechanism (the values are already present per-job today — this converges the invocation _mechanism_, not the values, which were never actually missing). Acceptance: `.github/workflows/pr-quality-gate.yml` has exactly one top-level `env:` block declaring `NX_BASE`/`NX_HEAD`; `grep -c '^      NX_BASE:' .github/workflows/pr-quality-gate.yml` returns 0 (no remaining per-job duplicates); `actionlint` passes.
- [ ] [AI] Remove the extra standalone markdown workflow job from `.github/workflows/` (fold into the existing gates, matching public). Acceptance: `test ! -f .github/workflows/validate-markdown.yml` (or the equivalent extra file is absent); `actionlint` passes on the remaining workflows.
- [ ] [AI] Lower-kebab every workflow `name:` value across `.github/workflows/*.yml`. Acceptance: every `name:` value in `.github/workflows/*.yml` is lower-kebab (no Title Case).
- [ ] [AI] Add missing targets to the 6 infra projects (`coralpolyp-contracts` at `specs/apps/coralpolyp/containers/contracts/project.json`: `deps:audit`+`compat:min-version`; `coralpolyp-be-e2e`, `coralpolyp-fe-e2e`: `deps:audit`+`compat:min-version`; `coralpolyp-fe`: `compat:min-version`; `libs/ts-ui`, `libs/ts-ui-tokens`: both). Acceptance: `for p in $(npx nx show projects --json | jq -r '.[]'); do npx nx show project "$p" --json | jq -e '.targets|has("deps:audit") and has("compat:min-version")' >/dev/null || echo "MISSING: $p"; done` prints no `MISSING` line.
- [ ] [AI] Wire `namedInputs.specs` on infra's remaining 2 projects (`ts-ui-tokens`, plus the contracts project `coralpolyp-contracts` at `specs/apps/coralpolyp/containers/contracts/project.json` — Nx-registered but outside `apps/`/`libs/`, invisible to a `find apps libs` scan). Acceptance: `for p in $(npx nx show projects --json | jq -r '.[]'); do npx nx show project "$p" --json | jq -e '.namedInputs.specs' >/dev/null || echo "MISSING: $p"; done` prints nothing (count == infra's full `nx show projects` total, 8).
- [ ] [AI] Copy the canonicalized `repo-governance/development/infra/nx-targets.md` "Cross-Repo rhino-cli Byte-Identity Standard" subsection and the `AGENTS.md` "Related Repositories" pointer (both authored in Phase 1) into infra, substituting only repo-name references, then run `npm run generate:bindings`. Acceptance: `diff` of the subsection's prose against public's (modulo repo-name substitution) shows no unintended wording drift; `npm run lint:md` exits 0 in infra.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Run `sh .husky/pre-push` from an ose-infra worktree (`worktrees/<name>/`) to prove the hook is worktree-safe — the same guardrail check applied to public and primer, not a bare-repo requirement. Acceptance: exits 0.

### Phase 4 Gate

> All checks below must pass before starting Phase 5. If Phase 4 was descoped per the preamble
> above, this gate is replaced by a single check: the documented-divergence entry exists in the
> README's Confirmed Decisions and the non-CLI infra convergence items above are green.

- [ ] [AI] `diff -rq apps/rhino-cli/src` (public vs infra) — empty; `diff` of `Cargo.toml`/`Cargo.lock`/`project.json` — empty (zero carve-outs, license relicensed to MIT).
- [ ] [AI] `cargo test -p rhino-cli` in infra — cucumber suites pass.
- [ ] [AI] `cargo test -p rhino-cli terraform_validator::` and `cargo test -p rhino-cli ansible_validator::` in infra — both pass, proving infra's real Terraform/Ansible env-drift validators are present and functional post-regeneration (guards against the CRITICAL silent-loss risk identified in tech-docs §11).
- [ ] [AI] `sh .husky/pre-push` in infra — exits 0.
- [ ] [AI] `nx run rhino-cli:specs:behavior:coverage` in infra — exits 0.
- [ ] [AI] `for p in $(npx nx show projects --json | jq -r '.[]'); do npx nx show project "$p" --json | jq -e '.targets|has("deps:audit") and has("compat:min-version")' >/dev/null || echo "MISSING: $p"; done` in infra — prints no `MISSING` line.

> **Pause Safety**: infra's rhino-cli is byte-identical to public and infra is at target (or the descope path is documented and the non-CLI convergence is green). Safe to stop. To resume: `sh .husky/pre-push` (infra repo root).

## Phase 5 — Cross-Repo Byte-Identity Verification & Archival

- [ ] [AI] rhino-cli byte-identity matrix: `diff -rq apps/rhino-cli/src` empty for public↔primer, public↔infra; `Cargo.toml`/`Cargo.lock`/`project.json` diffs show **no differences** (zero carve-outs). Acceptance: matrix committed under this plan folder's `audit/` subdir; nothing differs.
- [ ] [AI] Target parity: `jq -r '.targets|keys[]' apps/rhino-cli/project.json|sort` identical across all 3 repos; every command string identical. Acceptance: identical.
- [ ] [AI] cucumber parity: `cargo test -p rhino-cli` cucumber suites pass in all 3 repos; `tests/*.rs` + `.feature` trees identical. Acceptance: pass + identical.
- [ ] [AI] SDLC mechanism parity: `.husky/*` diffs show only IaC-only steps in infra; lint-staged identical modulo language set; canonical workflows identical modulo app/language/runner. Acceptance: **zero `⚠️` rows** in the parity table (built below).
- [ ] [AI] Config/targets/specs parity: `for p in $(npx nx show projects --json | jq -r '.[]'); do npx nx show project "$p" --json | jq -e '.namedInputs.specs' >/dev/null || echo "MISSING: $p"; done` prints nothing in all 3 repos (29/29 public, 26/26 primer, 8/8 infra — including all 4 contracts projects `organiclever-contracts`, `ose-contracts`, `crud-contracts`, `coralpolyp-contracts`); mandatory-target loop (same `nx show projects` enumeration) clean in all 3; `repo-config.yml` schema + header + harness list identical (`diff` the header comment block + top-level keys); no orphan spec dir. Acceptance: all green.
- [ ] [AI] Governance/docs convergence check: `diff` the `repo-governance/development/infra/nx-targets.md` "Cross-Repo rhino-cli Byte-Identity Standard" subsection and the `AGENTS.md` "Related Repositories" pointer across all 3 repos (substitute repo-name tokens before diffing). Acceptance: no unintended wording drift beyond the expected repo-name substitution.
- [ ] [AI] Binding-mirror-sync check (harness-neutrality, per Phase 1's governance-docs update + the `.opencode/agent/` bug fix): run `npm run generate:bindings` in each of the 3 repos and confirm `git status --porcelain` reports no diff afterward. Acceptance: clean `git status --porcelain` in all 3 repos post-generation.
- [ ] [AI] No-regression: `sh .husky/pre-push` passes in all 3 repos on a no-op (nothing staged). Acceptance: exit 0 in all 3.
- [ ] [AI] Build the Phase 5 parity table in this section (every standardization row ✅, zero `⚠️`; the only allowed-divergence rows are app/language set + the CI runner label). Acceptance: table complete in this doc.

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck in each repo: `nx affected -t typecheck`.
- [ ] [AI] Run affected linting in each repo: `nx affected -t lint`.
- [ ] [AI] Run affected quick tests in each repo: `nx affected -t test:quick`.
- [ ] [AI] Run affected spec coverage in each repo: `nx affected -t specs:behavior:coverage`.
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by this plan's changes (root-cause orientation).
- [ ] [AI] Verify all checks above pass before pushing any of the 3 repos.

### Post-Push Verification

- [ ] [AI] Push each repo's changes to `origin main`.
- [ ] [AI] Monitor GitHub Actions per [ci-post-push-verification](../../../repo-governance/development/workflow/ci-post-push-verification.md) — poll every 2 minutes, one `gh run view --json status,conclusion` per wakeup, never `gh run watch`. Watch `pr-quality-gate.yml` and `main-ci.yml` in ose-public and ose-primer; watch `pr-quality-gate.yml`, `main-ci.yml`, and infra's IaC-specific jobs in ose-infra.
- [ ] [AI] Verify all watched workflows report `success` in all three repos.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; do NOT archive until all three are green.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — split by concern/domain (rhino-cli source, hooks, workflows, Nx targets, docs).
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`.
- [ ] [AI] Sibling repos carry unrelated WIP — stage explicit paths only, never `git add -A`.
- [ ] [AI] Do NOT bundle unrelated fixes into a single commit.

### Phase 5 Gate

> All checks below must pass before archival.

- [ ] [AI] The parity table (built above) shows ✅ on every mechanics row across all three repos (allowed-divergence rows excluded) — no ❌ or ⚠️ in any mechanics row.
- [ ] [AI] `npm run generate:bindings && git status --porcelain` — clean (no drift) in all 3 repos.
- [ ] [AI] All 3 repos' latest push shows `success` on every watched CI workflow — no red.

> **Pause Safety**: all three repos converged, parity-verified, bindings clean, and CI-green; nothing half-applied. Safe to stop. To resume: re-run the byte-identity matrix (this phase's first item) and confirm all-green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items above are ticked (or explicitly deferred with a documented reason, e.g. the Phase 4 descope path).
- [ ] [AI] Verify ALL quality gates pass (local + CI) in all three repos.
- [ ] [AI] `git mv` this plan folder to `done/2026-07-DD__unify-rhino-cli-sdlc-parity/` (actual completion date) in all 3 repos.
- [ ] [AI] Update `plans/in-progress/README.md` in each repo — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` in each repo — add the plan entry with completion date.
- [ ] [AI] Commit: `chore(plans): move unify-rhino-cli-sdlc-parity to done` in each repo.

## Notes

- **Stale-note discipline**: if any item here turns out already-done when reached, verify with the
  named command and tick with the evidence — do not assume from the first plan's record.
