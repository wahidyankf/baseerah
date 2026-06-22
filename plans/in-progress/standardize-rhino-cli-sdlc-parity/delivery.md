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
- [ ] [AI] Encode the [§1.2 testing-architecture standard](./tech-docs.md#12-testing-architecture--target-contents-standard) into `repo-governance/development/infra/nx-targets.md`: the mandatory-seven targets + `echo`-placeholder rule, the `test:quick` = typecheck→lint→test:unit (`parallel: false`) composition, BE service-level / FE-DB-only `test:integration`, `*-e2e`-only `test:e2e`, and the pre-push ≡ PR-gate rule (only `test:quick`; never integration/e2e) — acceptance: all rules present and self-consistent with existing sections (resolve the "expose only needed targets" / no-op-anti-pattern tension explicitly); `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] **RED**: add a feature file under `specs/apps/rhino/behavior/rhino-cli/gherkin/` for the orphan-feature check, and a unit test asserting `specs validate coverage --require-consumption` fails on an unconsumed feature — command: `npx nx run rhino-cli:test:unit` — acceptance: new test fails (flag/behaviour not yet implemented).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: implement the `--require-consumption` behaviour in `specs validate coverage` (rhino-cli `src/`) — every `.feature` under the scanned spec dir must be referenced by ≥1 test; emit `orphan feature: <path> not consumed by any test` and exit non-zero otherwise — command: `npx nx run rhino-cli:test:unit` — acceptance: new test passes; `npx nx run rhino-cli:specs:coverage` still exits 0 on the current tree.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: default `--require-consumption` on for the `specs:coverage` Nx target across projects; update `specs/apps/rhino/` Gherkin + `docs/reference/sdlc-gate-standard.md` to document the new check — command: `npx nx run rhino-cli:test:quick` — acceptance: all rhino-cli tests pass; `specs:coverage` documents both step-def and consumption checks.

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

### 2e. Apply the testing-architecture target contents to every project (ose-public)

- [ ] [AI] Enumerate projects: `npx nx show projects` — acceptance: the list matches the rows of the [§1.3 per-project target matrix](./tech-docs.md#13-per-project-target-matrix-post-implementation-ose-public); reconcile any new/removed project against the matrix before converging.
- [ ] [AI] For EACH project's `project.json`, ensure the [§1.2 mandatory-seven targets](./tech-docs.md#12-testing-architecture--target-contents-standard) exist — add `echo` placeholders for any missing among `test:unit`, `test:integration`, `test:e2e`, `test:quick`, `lint`, `format`, `typecheck` — acceptance: `npx nx show project <p> --json | jq '.targets|keys'` includes all seven for every project; `npx nx affected -t format typecheck lint test:unit test:integration test:e2e test:quick` resolves a task (real or echo) for every affected project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Set every project's `test:quick` to the sequential composition (`nx:run-commands`, `"parallel": false`, commands `nx run <p>:typecheck` → `nx run <p>:lint` → `nx run <p>:test:unit`) — acceptance: running `test:quick` executes the three in order and stops at the first failure (verify by temporarily breaking lint in one project).
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Apply the content rules: `test:e2e` real only on `*-e2e` projects (echo elsewhere); BE `test:integration` is service-level (no HTTP); FE `test:integration` is echo unless DB-backed (keep `organiclever-app-web`'s PGlite integration real); `test:unit` includes BDD + non-BDD with coverage — acceptance: spot-check one BE, one FE-without-DB, `organiclever-app-web`, and one `*-e2e` project match the rules.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Make pre-push and the PR quality gate run the identical per-project code command — `nx affected -t test:quick` — and confirm **neither** runs `test:integration`/`test:e2e`; those stay only on the CRON pipelines. Edit `.husky/pre-push` and `.github/workflows/pr-quality-gate.yml` accordingly per [§1.2 gate rule](./tech-docs.md#12-testing-architecture--target-contents-standard) — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation; both run `test:quick`.
- [ ] [AI] Run the extended `specs:coverage` (`--require-consumption`) across affected projects and fix any orphan feature files (add the missing consuming test, or remove the dead feature with justification) — command: `npx nx affected -t specs:coverage` — acceptance: exits 0 with no orphan-feature errors.

### 2f. Post-merge CI + per-project staging deploy (ose-public)

Implements the [§1.4 standard](./tech-docs.md#14-post-merge-main-ci--per-project-staging-deploy).

- [ ] [AI] Add a `push: branches: [main]` trigger to a new `.github/workflows/main-ci.yml` that computes the affected project list (`npx nx show projects --affected --json`) and fans out a **per-project matrix** — acceptance: `actionlint` passes; a merge to main dispatches one matrix leg per affected project.
- [ ] [AI] Each matrix leg runs, in order, `nx run <p>:test:quick` → `nx run <p>:test:integration` → `nx run <p>:test:e2e` (its paired `*-e2e` runner where applicable), failing that leg only (no cross-project blocking) — acceptance: a forced failure in one project's integration test does not fail other projects' legs.
- [ ] [AI] On a passing leg for an **app-tier** deployable (`*-app-web`, `*-be`), trigger the staging deploy by reusing `_reusable-app-test-local-deploy-stag.yml` (force-push the project's `stag-*` branch) — acceptance: a green app-tier leg updates its `stag-*` branch; Vercel/k8s staging picks it up.
- [ ] [AI+HUMAN] Provision a staging environment for each `*-www` site: create the `stag-<app>-www` branch (`[AI]`, git-mechanical) and a Vercel **staging** project bound to it (`[HUMAN]` or via Vercel MCP — dashboard credentials) — acceptance: each `*-www` has a `stag-*-www` branch and a Vercel staging project; a push to that branch deploys to the staging URL.
- [ ] [AI] On a passing leg for a `*-www` site, deploy it to its new staging env (force-push `stag-<app>-www`); add a `_reusable-www-test-local-deploy-stag.yml` mirroring the prod one but targeting staging — acceptance: a green www leg updates its staging branch and deploys.
- [ ] [AI] Reduce the existing `*-test-local-deploy-stag.yml` CRON cadence to a single nightly run (retain as fallback), and leave `*-test-stag.yml` → deploy-prod scheduled as-is — acceptance: `actionlint` passes; the stag-deploy crons run once/day; prod promotion unchanged.
- [ ] [AI] Confirm pre-push and `pr-quality-gate.yml` still run **only** `test:quick` (+ governance validators) and **not** integration/e2e — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` — exits 0 (fix any preexisting failures).
- [ ] [AI] `npx nx run rhino-cli:format` and `:shell:check` and `:dockerfiles:check` and `:actions:check` and `:harness:bindings-validation` — each exits 0.
- [ ] [AI] Every project exposes the mandatory-seven targets: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:unit") and has("test:integration") and has("test:e2e") and has("test:quick") and has("lint") and has("format") and has("typecheck")' >/dev/null || echo "MISSING: $p"; done` — acceptance: prints no `MISSING` line.
- [ ] [AI] `actionlint .github/workflows/main-ci.yml` — exits 0; the per-project matrix + staging-deploy legs are well-formed.
- [ ] [AI] `npm run lint:md` — exits 0.
- [ ] [AI] Commit thematically: one commit for the rhino-cli target-name standardization (`fmt`→`format`, `:check` targets, bindings target), one for the hook rewire, one for the workflow renames+refs, one for the markdown-validator addition, one for the per-project target-contents (`test:quick` composition + mandatory-seven), one for the gate rule (test:quick-only), one for the post-merge main-ci + staging-deploy workflow.
- [ ] [AI] Push to `origin main`; monitor GitHub Actions; verify the renamed `pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml` all run green — acceptance: all CI checks pass.

> **Pause Safety**: ose-public is fully converged and green on CI. Safe to stop. To resume: `npx nx affected -t lint`.

---

## Phase 3: Propagate + Converge ose-primer

> Executes in the `ose-primer` repo (`/Users/wkf/ose-projects/ose-primer`). Target state = the
> [§1.3b primer matrix](./tech-docs.md#13b-per-project-target-matrix-post-implementation-ose-primer).
> Use primer's own worktree; commit to its `main`.

### 3a. Baseline + propagate

- [ ] [AI] Provision primer worktree + toolchain: `npm install && npm run doctor -- --fix` in ose-primer; `npx nx build rhino-cli` — acceptance: doctor green; rhino-cli builds.
- [ ] [AI] Record primer baseline: `npx nx run-many -t typecheck lint test:quick specs:coverage` — acceptance: pass, or preexisting failures noted.
- [ ] [AI] Propagate the artifacts: copy `plans/in-progress/standardize-rhino-cli-sdlc-parity/`, `docs/reference/rhino-cli-command-triage.md`, `docs/reference/sdlc-gate-standard.md`, and the `nx-targets.md`/`nx-target-naming.md` additions into ose-primer; replace the §1.3 matrix with the §1.3b primer matrix; adjust triage/standard for primer's app+language set per the divergence policy — acceptance: artifacts exist; `npm run lint:md` passes.

### 3b. Standardize rhino-cli target names

- [ ] [AI] In primer `apps/rhino-cli/project.json`: rename `fmt`→`format`; rename `shell:lint`→`shell:check`, `dockerfiles:lint`→`dockerfiles:check`, `actions:lint`→`actions:check`; update every reference (`grep -rn 'rhino-cli:fmt\b\|:shell:lint\|:dockerfiles:lint\|:actions:lint' --include='*.json' --include='*.yml' --include='*.sh' --include='*.md' .`) — acceptance: `npx nx run rhino-cli:format`/`:shell:check`/`:dockerfiles:check`/`:actions:check` each exit 0; zero stale references.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Add the structural targets primer's rhino-cli is **missing** so its key set matches public/infra: `specs:adoption-validation`, `specs:counts-validation`, `specs:links-validation`, `specs:tree-validation`, `test-coverage`, `test:e2e` (echo) — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set.
  - _Suggested executor: `swe-rust-dev`_

### 3c. Hook + workflow parity

- [ ] [AI] Add `governance:vendor-audit-validation` to primer's `.husky/pre-push` scoped validators (gated on `^repo-governance/.*\.md$`) — acceptance: editing a `repo-governance/*.md` then running pre-push triggers it; exits 0.
- [ ] [AI] Promote primer's deferred structural specs-gate in `.github/workflows/pr-quality-gate.yml` to the full set (`specs:adoption-validation` + `specs:tree-validation` + `specs:counts-validation` + `specs:links-validation` + `specs:coverage` + `specs:gherkin-cardinality-validation`) — acceptance: `actionlint` passes; job lists all six.
- [ ] [AI] Extract a standalone `.github/workflows/validate-env.yml` from primer's folded-in PR-gate env job (`npx nx run rhino-cli:env:validation` on `pull_request` + `push:main`); remove the duplicated env logic from the PR gate — acceptance: `actionlint` passes; `validate-env.yml` matches the public/infra shape.
- [ ] [AI] Confirm primer's `validate-markdown.yml` runs the 4-validator set (mermaid + links + headings + gherkin-cardinality) — acceptance: all four present.
- [ ] [AI] Align primer's PR-gate job skeleton to the standard (detect, format, language gates, markdown, naming, env, specs-gate, quality-gate sentinel); **keep** primer's per-language jobs (golang/jvm/dotnet/python/rust/elixir/clojure/dart — allowed divergence) — acceptance: `actionlint` passes; skeleton matches, language jobs preserved.

### 3d. Mandatory-seven sweep across all 26 primer projects

- [ ] [AI] For EACH primer project, bring its `project.json` to the [§1.3b matrix](./tech-docs.md#13b-per-project-target-matrix-post-implementation-ose-primer) — biggest gaps: **add `format` everywhere** (none have it); add `test:e2e` (echo) to the 11 `crud-be-*` + `crud-fs-ts-nextjs`; add `test:integration`+`test:e2e` (echo) to `crud-fe-*`; fill the support libs (`ts-ui-tokens` needs 5: `test:unit`/`test:integration`/`test:e2e` echo + `test:quick` + `format`; `golang-commons`/`clojure-openapi-codegen` need `typecheck` echo + more; `elixir-*` + `ts-ui` need `test:integration`/`test:e2e` echo + `format`); add `specs:coverage` to libs lacking it — acceptance: the mandatory-seven `jq` check (Phase 2 gate) prints no `MISSING` for any primer project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Set every primer project's `test:quick` to the sequential typecheck→lint→test:unit composition (`nx:run-commands`, `parallel:false`) — acceptance: order verified by breaking lint in one project.
- [ ] [AI] Resolve orphan features: `npx nx run-many -t specs:coverage` with `--require-consumption` — acceptance: no orphan-feature errors.

### 3e. Post-merge CI (primer is a template — tests only, deploy is a no-op)

- [ ] [AI] Add `.github/workflows/main-ci.yml` mirroring public's per-project affected matrix (`test:quick`→`test:integration`→`test:e2e` per project) — acceptance: `actionlint` passes; a merge dispatches one leg per affected project.
- [ ] [AI] Document that primer's deploy leg is a **no-op** (the `crud-*` demo apps have no live staging env — they are reference scaffolding); keep the `test-and-deploy-*-development` local-stack workflows as the nightly fallback — acceptance: `docs/reference/sdlc-gate-standard.md` in primer states the no-deploy rationale; no deploy step is wired.
- [ ] [AI] Confirm primer pre-push ≡ PR run only `test:quick` (+ validators), never integration/e2e — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] In ose-primer: `npx nx run-many -t typecheck lint test:quick specs:coverage` — exits 0.
- [ ] [AI] In ose-primer: mandatory-seven present on every project (Phase 2 gate `jq` loop prints no `MISSING`).
- [ ] [AI] In ose-primer: `npm run lint:md` and `actionlint` on changed workflows — exit 0.
- [ ] [AI] Commit thematically in ose-primer; push to `origin main`; verify CI green (incl. new `validate-env.yml`, promoted specs-gate, `main-ci.yml`).

> **Pause Safety**: ose-public + ose-primer converged and green. Safe to stop. To resume (primer): `npx nx affected -t lint`.

---

## Phase 4: Propagate + Converge ose-infra

> Executes in `ose-infra` (bare repo + worktree; commit to `main` via its worktree). Target state =
> the [§1.3c infra matrix](./tech-docs.md#13c-per-project-target-matrix-post-implementation-ose-infra).
> Infra already matches the workflow filenames (`pr-quality-gate.yml`, `validate-markdown.yml`,
> `validate-env.yml`) + governance-vendor pre-push, but (like public) lacks the `{tool}:check` +
> `harness:bindings-validation` Nx targets and uses `fmt`. CI runs on the self-hosted runner.

### 4a. Baseline + propagate

- [ ] [AI] Create/enter infra worktree on `main` (bare-repo layout); `npm install && npm run doctor -- --fix`; `npx nx build rhino-cli` — acceptance: doctor green; rhino-cli builds.
- [ ] [AI] Propagate the artifacts + the `nx-targets.md`/`nx-target-naming.md` additions into ose-infra; replace the matrix with the §1.3c infra matrix; document infra-only IaC gates (terraform/ansible/yamllint) and the self-hosted runner in the divergence section of `docs/reference/sdlc-gate-standard.md` — acceptance: artifacts exist; `npm run lint:md` passes.

### 4b. Standardize rhino-cli target names

- [ ] [AI] In infra `apps/rhino-cli/project.json`: rename `fmt`→`format`; add `shell:check`/`dockerfiles:check`/`actions:check` + `harness:bindings-validation` Nx targets (same defs as public); update references; rewire `.husky/pre-commit` to the `{tool}:check` targets and `.husky/pre-push` to `npx nx run rhino-cli:harness:bindings-validation` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set; each new target exits 0.
  - _Suggested executor: `swe-rust-dev`_

### 4c. Confirm workflow + hook parity (record IaC divergence)

- [ ] [AI] Verify infra's `pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml` match the standard filenames + validator sets (markdown 4-validator; specs-gate full set) — acceptance: filenames identical; validator sets match; any gap recorded as a fix step.
- [ ] [AI] Confirm infra's pre-commit/pre-push step order matches the standard, with terraform/ansible/yamllint as **documented allowed additions** (not drift) and the `[self-hosted, linux, ose-infra-runner]` label retained — acceptance: order matches; IaC + runner appear only in the divergence section.
- [ ] [AI] Fix any gaps found above — acceptance: each fixed gate exits 0 locally.

### 4d. Mandatory-seven sweep across all 7 infra projects

- [ ] [AI] Bring each infra project to the [§1.3c matrix](./tech-docs.md#13c-per-project-target-matrix-post-implementation-ose-infra): **add `format` everywhere**; `coralpolyp-be` keeps service-level `test:integration`; `coralpolyp-fe` integration real only if DB-backed else echo; `ts-ui-tokens` gains its 5 missing targets; `ts-ui` gains `test:integration`/`test:e2e` echo + `format`; `*-e2e` keep real `test:e2e`, echo `test:unit`/`test:integration` — acceptance: the mandatory-seven `jq` check prints no `MISSING` for any infra project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Set every infra project's `test:quick` to the sequential typecheck→lint→test:unit composition; resolve orphan features via `specs:coverage --require-consumption` — acceptance: order verified; no orphan-feature errors.

### 4e. Post-merge CI + coralpolyp staging deploy

- [ ] [AI] Add `.github/workflows/main-ci.yml` (self-hosted) with the per-project affected matrix (`test:quick`→`test:integration`→`test:e2e`); on a green `coralpolyp-be`/`coralpolyp-fe` leg, deploy to coralpolyp staging by reusing the `test-and-deploy-coralpolyp-development` build/deploy logic (merge-triggered) — acceptance: `actionlint` passes; a green coralpolyp leg deploys to staging.
- [ ] [AI] Reduce the existing coralpolyp dev/stag CRON cadence to nightly fallback; keep `test-coralpolyp-staging.yml` → prod promotion scheduled — acceptance: `actionlint` passes; crons run once/day; prod promotion unchanged.
- [ ] [AI] Confirm infra pre-push ≡ PR run only `test:quick` (+ validators + IaC), never integration/e2e — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] In ose-infra: `npx nx run-many -t typecheck lint test:quick specs:coverage` — exits 0.
- [ ] [AI] In ose-infra: mandatory-seven present on every project (`jq` loop prints no `MISSING`).
- [ ] [AI] In ose-infra: `npm run lint:md` and `actionlint` on changed workflows — exit 0.
- [ ] [AI] Commit thematically in ose-infra (via worktree); push to `origin main`; verify CI green on the self-hosted runner (incl. `main-ci.yml`).

> **Pause Safety**: all three repos converged and green. Safe to stop. To resume (infra): `npx nx affected -t lint`.

---

## Phase 5: Cross-Repo Parity Verification & Archival

- [ ] [AI] Build the parity table comparing all three repos across every mechanics row (PR-gate filename, markdown filename, env filename, markdown validator set, specs-gate set, lint invocation mechanism, pre-push governance-vendor presence, hook step order, rhino-cli target-key set, mandatory-seven on every project, `test:quick` = typecheck→lint→test:unit composition, pre-push ≡ PR runs only `test:quick`, `specs:coverage --require-consumption` enabled) — acceptance: a table with a ✅/❌ per repo per row is produced; every mechanics row is ✅ across all three (allowed-divergence rows excluded).
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
