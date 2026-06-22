# Delivery — Standardize rhino-cli Checks & SDLC Commands

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root-cause orientation — fix preexisting errors encountered during work).

> **Multi-repo note**: This plan is authored in `ose-public`. Phases 0–2 execute here. Phases 3–4
> execute in `ose-primer` and `ose-infra` respectively — each begins by propagating this plan folder
> and the two reference docs into the sibling repo (per the
> [multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)),
> then converging that repo in its own working tree. ose-infra is a bare repo + worktree (commit to
> `main` via its worktree).

## Worktree

Worktree path: `worktrees/standardize-rhino-cli-sdlc-parity/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-rhino-cli-sdlc-parity
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline (ose-public)

- [ ] [AI] Provision worktree: `claude --worktree standardize-rhino-cli-sdlc-parity` — acceptance: `worktrees/standardize-rhino-cli-sdlc-parity/` exists.
- [ ] [AI] Initialize toolchain in the root worktree: `npm install && npm run doctor -- --fix` — acceptance: doctor reports all required tools present (rust, node, shellcheck, hadolint, actionlint).
- [ ] [AI] Build rhino-cli: `npx nx build rhino-cli` — acceptance: exits 0.
- [ ] [AI] Record baseline: run `npx nx affected -t typecheck lint test:quick specs:coverage` on a clean tree — acceptance: passes (or preexisting failures noted in implementation notes).

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npx nx build rhino-cli` — exits 0.
- [ ] [AI] `git status` — clean working tree (no stray edits).

> **Pause Safety**: clean baseline recorded, no edits applied. Safe to stop. To resume: `npx nx build rhino-cli`.

---

## Phase 1: Author Standard + Triage Reference Docs + Extend Canonical Nx Naming (ose-public)

- [ ] [AI] Confirm triage rows 25–27: read the `git pre-commit` implementation in `apps/rhino-cli/src/` (grep for `pre_commit` / `generate bindings` / `sync`) — acceptance: determine whether binding sync is auto-run by the hook; update the triage status from `[Unverified]` to wired/not-wired with the cited source line.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Cross-check the triage against the CLI surface: `cargo run -p rhino-cli -- --help` recursively (or read `apps/rhino-cli/src/cli.rs`) — acceptance: every leaf subcommand in the CLI appears exactly once in the triage table; no command is missing.
- [ ] [AI] Create `docs/reference/rhino-cli-command-triage.md` containing the [tech-docs §2 triage table](./tech-docs.md#2-rhino-cli-command-triage-wired-vs-not-wired) (every command, its description, wired/not-wired status, and invocation site), with a short intro and a "wired = invoked by lifecycle automation" definition — acceptance: file exists; `npx nx run rhino-cli:links:validation` passes for it; `npm run lint:md` passes.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Create `docs/reference/sdlc-gate-standard.md` containing [tech-docs §1 standard](./tech-docs.md#1-target-standard-best-of-three-synthesis) + [§3 divergence policy](./tech-docs.md#3-divergence-policy-allowed-vs-drift) — acceptance: file exists; lint:md passes; links:validation passes.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Add both new docs to `docs/reference/README.md` index — acceptance: both linked; `npx nx run rhino-cli:headings:hierarchy-validation` and `links:validation` pass.
- [ ] [AI] Extend the canonical Nx naming scheme to close the two gaps: in `repo-governance/development/infra/nx-targets.md` add `format` to the lifecycle target list (paired with the existing `format:check`) and add `shell:check` / `dockerfiles:check` / `actions:check` to the `{domain}:{work}` governance/validation table — acceptance: both additions present; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] In `repo-governance/development/infra/nx-target-naming.md` document the `format` (write/`format:check` verify) pairing and the `{tool}:check` derivation (domain = tool, work = `check`) — acceptance: both derivations documented; `npx nx run rhino-cli:links:validation` passes.
  - _Suggested executor: `repo-rules-maker`_

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run rhino-cli:links:validation` — exits 0.
- [ ] [AI] `npx nx run rhino-cli:mermaid:validation` — exits 0 (validates the plan's mermaid diagrams).
- [ ] [AI] `npm run lint:md` — exits 0.
- [ ] [AI] Commit: `docs(reference): add rhino-cli command triage and SDLC gate standard`.

> **Pause Safety**: standard + triage are published and self-consistent; no hooks/workflows changed yet. Safe to stop. To resume: `npm run lint:md`.

---

## Phase 2: Converge ose-public to the Standard

### 2a. Standardize rhino-cli target names (`fmt`→`format`, add `{tool}:check`, add `harness:bindings-validation`)

- [ ] [AI] Rename the format/write target in `apps/rhino-cli/project.json`: `fmt` → `format` (keep `format:check` as-is) — acceptance: `npx nx run rhino-cli:format` runs `cargo fmt`; `npx nx run rhino-cli:fmt` now fails with "target fmt not found".
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Grep and update every reference to the old `fmt` target: `grep -rn 'rhino-cli:fmt\b\|nx run rhino-cli:fmt' --include='*.json' --include='*.md' --include='*.sh' --include='*.yml' .` (package.json scripts, hooks, workflows, docs) — acceptance: zero `rhino-cli:fmt` references remain; `npm run lint:md` passes.
- [ ] [AI] **RED**: run `npx nx run rhino-cli:shell:check` BEFORE adding it — acceptance: fails with "target shell:check not found" (documents the gap).
- [ ] [AI] **GREEN**: in `apps/rhino-cli/project.json` add targets `shell:check`, `dockerfiles:check`, `actions:check` (shellcheck `--severity=warning`, hadolint `--failure-threshold warning`, actionlint), and `harness:bindings-validation` (`cargo run -- harness validate bindings`) — acceptance: `npx nx run rhino-cli:shell:check`, `:dockerfiles:check`, `:actions:check`, `:harness:bindings-validation` each exit 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: make the four new targets cacheable and tagged consistently with the existing validation targets in `project.json` — acceptance: `npx nx show project rhino-cli --json | jq '.targets | keys'` lists all four; re-run is a cache-hit.
- [ ] [AI] Replace the `npm run harness:bindings-validation` invocation in `.husky/pre-push` with `npx nx run rhino-cli:harness:bindings-validation` (mechanism parity with primer) — acceptance: the scoped pre-push step invokes the Nx target; it exits 0.

### 2b. Rewire pre-commit to use the Nx `{tool}:check` targets

- [ ] [AI] Edit `.husky/pre-commit`: replace the inline `shellcheck` / `hadolint` / `actionlint` blocks with `npx nx run rhino-cli:shell:check` / `:dockerfiles:check` / `:actions:check`, preserving the tool-gated graceful-skip behaviour and the standard step order (identity → no-env → check → `git pre-commit` → `nx affected test:quick`) per [tech-docs §1](./tech-docs.md#1-target-standard-best-of-three-synthesis) — acceptance: `bash .husky/pre-commit` on a staged no-op runs without error; step order matches the standard.

### 2c. Rename workflow files + fix all references

- [ ] [AI] `git mv .github/workflows/commons-quality-gate.yml .github/workflows/pr-quality-gate.yml` — acceptance: file moved; `git status` shows a rename.
- [ ] [AI] `git mv .github/workflows/markdown-validate.yml .github/workflows/validate-markdown.yml` — acceptance: rename shown.
- [ ] [AI] `git mv .github/workflows/commons-env-validate.yml .github/workflows/validate-env.yml` — acceptance: rename shown.
- [ ] [AI] Update the `name:` field inside each renamed workflow to match its new role — acceptance: `actionlint` passes on all three.
- [ ] [AI] Grep for old filenames repo-wide and update every reference: `grep -rn 'commons-quality-gate\|markdown-validate\|commons-env-validate' --include='*.md' --include='*.yml' .` — acceptance: zero hits remain except in this plan's drift catalog; `.github/workflows/README.md`, `repo-governance/development/quality/*.md`, and root `AGENTS.md`/`CLAUDE.md` updated as needed.
  - _Suggested executor: `repo-rules-fixer`_

### 2d. Add gherkin-cardinality to the markdown workflow

- [ ] [AI] Edit `.github/workflows/validate-markdown.yml`: add a step `npx nx run rhino-cli:specs:gherkin-cardinality-validation` after the heading-hierarchy step — acceptance: `actionlint` passes; the workflow now runs the 4-validator standard set.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` — exits 0 (fix any preexisting failures).
- [ ] [AI] `npx nx run rhino-cli:format` and `:shell:check` and `:dockerfiles:check` and `:actions:check` and `:harness:bindings-validation` — each exits 0.
- [ ] [AI] `npm run lint:md` — exits 0.
- [ ] [AI] Commit thematically: one commit for the rhino-cli target-name standardization (`fmt`→`format`, `:check` targets, bindings target), one for the hook rewire, one for the workflow renames+refs, one for the markdown-validator addition.
- [ ] [AI] Push to `origin main`; monitor GitHub Actions; verify the renamed `pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml` all run green — acceptance: all CI checks pass.

> **Pause Safety**: ose-public is fully converged and green on CI. Safe to stop. To resume: `npx nx affected -t lint`.

---

## Phase 3: Propagate + Converge ose-primer

> Executes in the `ose-primer` repo (`/Users/wkf/ose-projects/ose-primer`). Begins by copying this
> plan folder and the two reference docs across, then converging primer. Use primer's own worktree.

- [ ] [AI] Propagate: copy `plans/in-progress/standardize-rhino-cli-sdlc-parity/`, `docs/reference/rhino-cli-command-triage.md`, `docs/reference/sdlc-gate-standard.md`, and the `nx-targets.md`/`nx-target-naming.md` additions into `ose-primer` (adjust the triage/standard docs for primer's app+language set per the divergence policy) — acceptance: the artifacts exist in primer; `npm run lint:md` passes there.
- [ ] [AI] Standardize primer's rhino-cli target names in `apps/rhino-cli/project.json`: rename `fmt`→`format`; rename `shell:lint`→`shell:check`, `dockerfiles:lint`→`dockerfiles:check`, `actions:lint`→`actions:check`; update every reference (`grep -rn 'rhino-cli:fmt\b\|:shell:lint\|:dockerfiles:lint\|:actions:lint'`) — acceptance: `npx nx run rhino-cli:format`, `:shell:check`, `:dockerfiles:check`, `:actions:check` each exit 0; zero stale references.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Add the missing structural targets to primer's `apps/rhino-cli/project.json` so the target set matches public/infra: `specs:adoption-validation`, `specs:counts-validation`, `specs:links-validation`, `specs:tree-validation`, `test-coverage`, `test:e2e` (no-op echo where no e2e) — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json` matches public's sorted key set.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Add `governance:vendor-audit-validation` to primer's `.husky/pre-push` scoped validators (gated on `^repo-governance/.*\.md$`), matching the standard — acceptance: editing a `repo-governance/*.md` file then running pre-push triggers the validator; it exits 0.
- [ ] [AI] Promote primer's deferred structural specs-gate set: in `.github/workflows/pr-quality-gate.yml` change the `specs-gate` job to run the full set (`specs:adoption-validation` + `specs:tree-validation` + `specs:counts-validation` + `specs:links-validation` + `specs:coverage` + `specs:gherkin-cardinality-validation`) — acceptance: `actionlint` passes; the job lists all six validators.
- [ ] [AI] Extract a standalone `.github/workflows/validate-env.yml` from primer's folded-in PR-gate env job, running `npx nx run rhino-cli:env:validation` on `pull_request`+`push:main`, and remove the now-duplicated env logic from the PR gate — acceptance: `actionlint` passes; `validate-env.yml` exists and matches the public/infra shape.
- [ ] [AI] Reconcile primer's `validate-markdown.yml` to the 4-validator standard (it should already include gherkin-cardinality — confirm) — acceptance: workflow runs mermaid + links + headings + gherkin-cardinality.
- [ ] [AI] Align the PR-gate job skeleton names to the standard (detect, format, language gates, markdown, naming, env or removed-if-standalone, specs-gate, quality-gate sentinel); keep primer's extra per-language jobs (allowed divergence) — acceptance: `actionlint` passes; skeleton matches the standard, language jobs preserved.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] In ose-primer: `npx nx affected -t typecheck lint test:quick specs:coverage` — exits 0.
- [ ] [AI] In ose-primer: `npm run lint:md` — exits 0.
- [ ] [AI] Commit thematically in ose-primer; push to `origin main`; verify CI green (including the new `validate-env.yml` and the promoted specs-gate).

> **Pause Safety**: ose-public + ose-primer converged and green. Safe to stop. To resume (primer): `npx nx affected -t lint`.

---

## Phase 4: Propagate + Converge ose-infra

> Executes in `ose-infra` (bare repo + worktree at `/Users/wkf/ose-projects/ose-infra`; commit to
> `main` via its worktree). Infra already matches most of the workflow standard (`pr-quality-gate.yml`,
> `validate-markdown.yml`, `validate-env.yml`, governance-vendor in pre-push) but, like public, lacks
> the `{tool}:check` Nx targets and the `harness:bindings-validation` Nx target, and uses `fmt`.
> This phase confirms workflow parity and converges the target names.

- [ ] [AI] Propagate: copy the plan folder + the two reference docs + the `nx-targets.md`/`nx-target-naming.md` additions into ose-infra, adapting the triage/standard for infra's app set (coralpolyp) and adding the infra-only IaC gates to the divergence section — acceptance: artifacts exist; `npm run lint:md` passes.
- [ ] [AI] Standardize infra's rhino-cli target names in `apps/rhino-cli/project.json` (same as public): rename `fmt`→`format`; add `shell:check`/`dockerfiles:check`/`actions:check` + `harness:bindings-validation` Nx targets; update references; rewire `.husky/pre-commit` to the `{tool}:check` targets and `.husky/pre-push` to `npx nx run rhino-cli:harness:bindings-validation` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json` matches public's sorted key set; each new target exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Verify infra's `pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml` already match the standard filenames + validator sets — acceptance: filenames identical; markdown workflow runs the 4-validator set; specs-gate runs the full set; record any gap as a fix step.
- [ ] [AI] Confirm infra's pre-commit/pre-push step order matches the standard, with IaC steps (terraform/ansible/yamllint) documented as allowed additions in `docs/reference/sdlc-gate-standard.md` — acceptance: order matches; IaC additions appear only in the divergence section, not flagged as drift.
- [ ] [AI] Fix any gaps found in the two steps above — acceptance: each fixed gate exits 0 locally.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] In ose-infra: `npx nx affected -t typecheck lint test:quick specs:coverage` — exits 0.
- [ ] [AI] In ose-infra: `npm run lint:md` — exits 0.
- [ ] [AI] Commit thematically in ose-infra (via worktree); push to `origin main`; verify CI green on the self-hosted runner.

> **Pause Safety**: all three repos converged and green. Safe to stop. To resume (infra): `npx nx affected -t lint`.

---

## Phase 5: Cross-Repo Parity Verification & Archival

- [ ] [AI] Build the parity table comparing all three repos across every mechanics row (PR-gate filename, markdown filename, env filename, markdown validator set, specs-gate set, lint invocation mechanism, pre-push governance-vendor presence, hook step order) — acceptance: a table with a ✅/❌ per repo per row is produced; every mechanics row is ✅ across all three (allowed-divergence rows excluded).
- [ ] [AI] Record the parity table in each repo's `docs/reference/sdlc-gate-standard.md` under a "Parity Status" heading — acceptance: present in all three; lint:md passes.

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` (each repo).
- [ ] [AI] Run affected linting: `npx nx affected -t lint` (each repo).
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` (each repo).
- [ ] [AI] Run affected spec coverage: `npx nx affected -t specs:coverage` (each repo).
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes.

### Post-Push Verification

- [ ] [AI] Push final changes to `main` in each repo.
- [ ] [AI] Monitor GitHub Actions for each push (poll every 2 minutes; one `gh run view --json status,conclusion` per wakeup).
- [ ] [AI] Verify all CI checks pass in all three repos.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; do NOT archive until all three are green.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group by surface (docs / hooks / workflows / Nx targets).
- [ ] [AI] Follow Conventional Commits: `<type>(<scope>): <description>`.
- [ ] [AI] Split per repo and per concern; do NOT bundle unrelated fixes.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI) in all three repos.
- [ ] [AI] Move plan folder from `plans/in-progress/` to `plans/done/` via `git mv` in each repo: `git mv plans/in-progress/standardize-rhino-cli-sdlc-parity plans/done/2026-MM-DD__standardize-rhino-cli-sdlc-parity` (use the actual completion date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Commit: `chore(plans): move standardize-rhino-cli-sdlc-parity to done`.

## Validation Checklist

- [ ] [AI] All TDD cycles complete (the rhino-cli Nx-target additions in Phase 2a).
- [ ] [AI] All tests pass (`npx nx affected -t test:quick`) in all three repos.
- [ ] [AI] Command triage doc covers every leaf subcommand.
- [ ] [AI] SDLC standard doc + parity table present in all three repos.
- [ ] [AI] Divergence policy documents every retained difference.
- [ ] [AI] Acceptance criteria in [prd.md](./prd.md) verified.
