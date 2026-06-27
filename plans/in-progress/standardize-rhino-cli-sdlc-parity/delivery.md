# Delivery — Standardize rhino-cli Checks & SDLC Commands

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

<!-- -->

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root-cause orientation — fix preexisting errors encountered during work).

<!-- -->

> **Multi-repo note**: This plan is authored in `ose-public`. Phases 0–2 execute here. Phases 3–4
> execute in `ose-primer` and `ose-infra` respectively — each begins by propagating this plan folder
> and the two reference docs into the sibling repo (per the
> [multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)),
> then converging that repo in its own working tree. ose-infra is a normal repo (not bare) — commit
> to `main` directly.

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
- [ ] [AI] Extend the canonical Nx naming scheme: in `repo-governance/development/infra/nx-targets.md` **drop `format`/`format:check` from the lifecycle target list** (formatting is file-type lint-staged, documented separately) and add `test:coverage`, `specs:behavior:coverage` (renamed from `specs:coverage`), `specs:domain:coverage` (`*-be` only), plus `shell:check` / `dockerfiles:check` / `actions:check` to the `{domain}:{work}` governance/validation table — acceptance: all changes present; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] In `repo-governance/development/infra/nx-target-naming.md` document the `{tool}:check` derivation (domain = tool, work = `check`) **and** that formatting is file-type lint-staged (no per-project `format`/`format:check` target) — acceptance: both documented; `npx nx run rhino-cli:links:validation` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Encode the [§1.2 testing-architecture standard](./tech-docs.md#12-testing-architecture--target-contents-standard) into `repo-governance/development/infra/nx-targets.md`: the mandatory targets + `echo`-placeholder rule, the `test:quick` = typecheck→lint→test:unit (`parallel: false`) composition, the native `test:coverage` ≥ 90% gate (replacing the removed rhino-cli `test-coverage`), BE service-level / FE-DB-only `test:integration`, `*-e2e`-only `test:e2e`, the file-type-based `format` via lint-staged (no per-project `format` target), and the pre-push ≡ PR-gate rule (only `test:quick`; never integration/e2e) — acceptance: all rules present and self-consistent with existing sections (resolve the "expose only needed targets" / no-op-anti-pattern tension explicitly); `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] **RED**: add a feature file under `specs/apps/rhino/behavior/rhino-cli/gherkin/` for the orphan-feature check, and a unit test asserting `specs behavior-coverage validate --require-consumption` fails on an unconsumed feature — command: `npx nx run rhino-cli:test:unit` — acceptance: new test fails (flag/behaviour not yet implemented).
  - **Gherkin (binds) →** "An orphan feature file fails the gate"

    ```gherkin
    Scenario: An orphan feature file fails the gate
      Given a feature file under specs that no test references
      When rhino-cli specs behavior-coverage validate runs with --require-consumption
      Then it fails and names the orphan feature file
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED**: add a unit test asserting `specs behavior-coverage validate --require-consumption` fails on a feature file that is consumed (its file is referenced by a test) but one of its scenarios is not exercised by any eligible unit/integration/e2e test — command: `npx nx run rhino-cli:test:unit` — acceptance: new test fails (per-scenario consumption check not yet implemented).
  - **Gherkin (binds) →** "An uncovered scenario fails the gate"

    ```gherkin
    Scenario: An uncovered scenario fails the gate
      Given a scenario in a binding feature file that no eligible unit, integration, or e2e test exercises
      When rhino-cli specs behavior-coverage validate runs with --require-consumption
      Then it fails and names the uncovered feature and scenario
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: implement the `--require-consumption` behaviour in `specs behavior-coverage validate` (rhino-cli `src/`) — every `.feature` file **and every scenario** under the scanned binding spec dir must be exercised by ≥1 **eligible** test (unit/integration/e2e, per `@tag`); emit `orphan feature: <path> not consumed by any test` for an unconsumed file and `uncovered scenario: <feature>:<scenario> not exercised by any eligible unit/integration/e2e test` for an unbound scenario, exit non-zero otherwise — command: `npx nx run rhino-cli:test:unit` — acceptance: new test passes; `npx nx run rhino-cli:specs:coverage` still exits 0 on the current tree.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: default `--require-consumption` on for the `specs:coverage` Nx target across projects; update `specs/apps/rhino/` Gherkin + `docs/reference/sdlc-gate-standard.md` to document the new check — command: `npx nx run rhino-cli:test:quick` — acceptance: all rhino-cli tests pass; `specs:coverage` gate (soon renamed to `specs:behavior:coverage` in §1b) documents both step-def and consumption checks.

### 1b. Rename `specs validate coverage`→`behavior-coverage` + add `specs domain-coverage validate`

- [ ] [AI] **RED**: Write tests in `apps/rhino-cli/tests/` (or the relevant test module) asserting that `cargo run -- specs behavior-coverage validate` succeeds and `cargo run -- specs validate coverage` fails with "unrecognized subcommand" — command: `npx nx run rhino-cli:test:unit` — acceptance: new tests fail (rename not yet applied).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Rename the CLI dispatch entry from `coverage` to `behavior-coverage` in `apps/rhino-cli/src/cli.rs` (and the application module it resolves to); update `specs/apps/rhino/**` Gherkin and all unit tests to use the new command name — command: `npx nx run rhino-cli:test:unit` — acceptance: new tests pass; no other tests broken.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: Rename the Nx target from `specs:coverage` to `specs:behavior:coverage` in `apps/rhino-cli/project.json`; update all Nx target references (hooks, workflows, `nx-targets.md`) — command: `npx nx run rhino-cli:test:quick` — acceptance: `npx nx run rhino-cli:specs:behavior:coverage` exits 0; `npx nx run rhino-cli:specs:coverage` fails with "target not found"; all tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: add a feature file + unit test asserting `specs domain-coverage validate` fails when a domain entity in `specs/apps/<domain>/domain/**` has no domain unit test — command: `npx nx run rhino-cli:test:unit` — acceptance: test fails (command not yet implemented).
  - **Gherkin (binds) →** "An uncovered domain entity fails the gate"

    ```gherkin
    Scenario: An uncovered domain entity fails the gate
      Given a *-be project with a domain entity that no domain unit test exercises
      When rhino-cli specs domain-coverage validate runs
      Then it fails and names the uncovered domain entity
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: implement `specs domain-coverage validate` (rhino-cli `src/`) — for `*-be` projects, every entity in the bounded-context/ubiquitous-language registry under `specs/apps/<domain>/domain/**` must be exercised by ≥1 domain unit test; emit `uncovered domain entity: <name>` and exit non-zero otherwise — command: `npx nx run rhino-cli:test:unit` — acceptance: new test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: expose the new command as the Nx target `specs:domain:coverage`, wired **only on `*-be` projects**; document it in `docs/reference/sdlc-gate-standard.md` — command: `npx nx run rhino-cli:test:quick` — acceptance: tests pass; the target resolves for `*-be` projects and is absent on others.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run rhino-cli:links:validation` — exits 0.
- [ ] [AI] `npx nx run rhino-cli:mermaid:validation` — exits 0 (validates the plan's mermaid diagrams).
- [ ] [AI] `npm run lint:md` — exits 0.
- [ ] [AI] Commit: `docs(reference): add rhino-cli command triage and SDLC gate standard`.

> **Pause Safety**: standard + triage are published and self-consistent; no hooks/workflows changed yet. Safe to stop. To resume: `npm run lint:md`.

---

## Phase 2: Converge ose-public to the Standard

### 2a. Standardize rhino-cli target names (remove `fmt`/`format:check`, add `{tool}:check`, add `harness:bindings-validation`)

- [ ] [AI] **Remove** the `fmt` and `format:check` targets from `apps/rhino-cli/project.json` (formatting moves to file-type lint-staged, §1.1) — acceptance: `npx nx run rhino-cli:fmt` and `:format:check` both fail with "target not found".
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Ensure the shared lint-staged config (`package.json` `lint-staged` block / `.lintstagedrc`) covers `*.rs`→`rustfmt` (and `*.fs`→`fantomas` where F# ships) so the removed Rust/F# `fmt` is replaced by file-type formatting — acceptance: staging a `*.rs` file and committing reformats it via the hook; `grep -rn 'rhino-cli:fmt\b\|rhino-cli:format:check' --include='*.json' --include='*.md' --include='*.sh' --include='*.yml' .` returns zero hits.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: run `npx nx run rhino-cli:shell:check` BEFORE adding it — acceptance: fails with "target shell:check not found" (documents the gap).
- [ ] [AI] **GREEN**: in `apps/rhino-cli/project.json` add targets `shell:check`, `dockerfiles:check`, `actions:check` (shellcheck `--severity=warning`, hadolint `--failure-threshold warning`, actionlint), and `harness:bindings-validation` (`cargo run -- harness validate bindings`) — acceptance: `npx nx run rhino-cli:shell:check`, `:dockerfiles:check`, `:actions:check`, `:harness:bindings-validation` each exit 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: make the four new targets cacheable and tagged consistently with the existing validation targets in `project.json` — acceptance: `npx nx show project rhino-cli --json | jq '.targets | keys'` lists all four; re-run is a cache-hit.
- [ ] [AI] Replace the `npm run harness:bindings-validation` invocation in `.husky/pre-push` with `npx nx run rhino-cli:harness:bindings-validation` (mechanism parity with primer) — acceptance: the scoped pre-push step invokes the Nx target; it exits 0.

#### 2a-cov. Remove the rhino-cli `test-coverage` command + Nx target (coverage goes native)

Implements the [§1.1 Coverage-enforcement decision](./tech-docs.md#11-nx-target-name-standard-targets-invoked-by-hooksci) — drop the central rhino-cli coverage parser in favour of each project's native ≥ 90% gate.

- [ ] [AI] **RED**: In `apps/rhino-cli/tests/` (or the relevant integration test module), add a test asserting `cargo run -- test-coverage validate` exits non-zero with "unrecognized subcommand `test-coverage`" — command: `npx nx run rhino-cli:test:unit` — acceptance: new test fails (command still present, so the assertion that it is absent fails).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: remove the `test-coverage validate` command from `apps/rhino-cli/src/` (CLI dispatch + application module + adapter), its `specs/apps/rhino/**` Gherkin, and its unit/integration tests; delete the `test-coverage` Nx target from `apps/rhino-cli/project.json` — command: `npx nx run rhino-cli:test:quick` — acceptance: `cargo run -- test-coverage validate` exits non-zero ("unrecognized subcommand"); `jq -e '.targets|has("test-coverage")|not' apps/rhino-cli/project.json` is true; all remaining rhino-cli tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: scrub every `test-coverage` / Codecov-algorithm reference from `apps/rhino-cli/README.md` and any `repo-governance/`/docs that describe the removed command (public) — acceptance: `grep -rin 'test-coverage\|codecov' apps/rhino-cli repo-governance docs` returns only `ExcludeFromCodeCoverage`-attribute hits; `npm run lint:md` passes.

#### 2a-cfg. Merge root config files into `repo-config.yml` (§1.1a)

- [ ] [AI] **RED**: add a unit test asserting the rhino-cli config loader reads the `instruction-size`/`env-contract`/`env-injection` sections from `repo-config.yml`, and errors hard when a section is missing — command: `npx nx run rhino-cli:test:unit` — acceptance: test fails (loader still reads the standalone files).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: create `repo-config.yml` at the repo root with the three namespaced sections (migrate the contents of `instruction-size-budget.yaml`, `env-contract.yaml`, `env-injection.yaml` verbatim under `instruction-size:` / `env-contract:` / `env-injection:`); update the loaders in `apps/rhino-cli/src/` (`convention validate instruction-size` — pre-§2a-names current name; renamed to `convention instruction-size validate` in §2a-names later in this phase, `env validate`/`init`/`backup`/`restore`, env-injection checker) to read `repo-config.yml` sections — command: `npx nx run rhino-cli:test:unit` — acceptance: test passes; `npx nx run rhino-cli:instruction-size:validation` and `:env:validation` exit 0 against `repo-config.yml`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: delete the three standalone root files; repoint every Nx-target `inputs` glob and any doc/reference from them to `repo-config.yml` (keep `apps/rhino-cli/tests/fixtures/**` standalone fixtures untouched) — command: `npx nx run rhino-cli:test:quick` — acceptance: `test ! -f instruction-size-budget.yaml && test ! -f env-contract.yaml && test ! -f env-injection.yaml`; `grep -rn 'instruction-size-budget.yaml\|env-contract.yaml\|env-injection.yaml' --include='*.json' --include='*.md' . | grep -v 'tests/fixtures'` returns nothing; gates exit 0.

#### 2a-names. Standardize rhino-cli command names to verb-last (§2.0)

- [ ] [AI] Document the two naming conventions in `repo-governance/development/infra/nx-target-naming.md` (and a short CLI-command-naming note): CLI commands are `{domain} {sub-domain…} {verb}` (verb last); Nx targets are `:`-separated `{domain}:{work}`/lifecycle — acceptance: both conventions documented with examples; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] **RED**: add a test asserting the verb-last invocation works and the old verb-middle form fails, for a representative sample (`convention emoji validate`, `harness opencode sync`, `repo-governance vendor validate`) — command: `npx nx run rhino-cli:test:unit` — acceptance: test fails (commands still verb-middle).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: rename every rhino-cli leaf command in `apps/rhino-cli/src/cli.rs` (+ dispatch) to the verb-last **target** form in the [§2 triage table](./tech-docs.md#2-rhino-cli-command-triage-wired-vs-not-wired) (`{domain} {noun…} {verb}`); drop the `(alias)` shortcuts (rows 13–14) in favour of canonical verb-last; keep Nx target names (`:`-separated) unchanged — command: `npx nx run rhino-cli:test:unit` — acceptance: every CLI command matches its triage target column; `cargo run -- --help` recursively shows verb-last leaves; tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: update every reference to a renamed command (Nx-target `command:` strings in `project.json`, `.husky/*`, `.github/workflows/*`, `package.json` scripts, `docs/`, `repo-governance/`, `specs/apps/rhino/**`) — command: `grep -rn -E '\b(validate|sync|emit|generate|clean|scaffold) [a-z][a-z-]*' --include='*.json' --include='*.yml' --include='*.sh' --include='*.md' . | grep -v 'rhino-cli-command-triage\.md\|standardize-rhino-cli-sdlc-parity/tech-docs\.md'` returns no verb-middle forms (the two triage docs deliberately preserve old forms in their "current" column and are excluded); `npx nx run rhino-cli:test:quick` and `npm run lint:md` pass.

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
- [ ] [AI] For EACH project's `project.json`, ensure the [§1.2 mandatory-six targets](./tech-docs.md#12-testing-architecture--target-contents-standard) exist — add `echo` placeholders for any missing among `test:unit`, `test:integration`, `test:e2e`, `test:quick`, `lint`, `typecheck` (**no `format` target** — formatting is lint-staged) — acceptance: `npx nx show project <p> --json | jq '.targets|keys'` includes all six for every project; `npx nx affected -t typecheck lint test:unit test:integration test:e2e test:quick` resolves a task (real or echo) for every affected project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Set every project's `test:quick` to the sequential composition (`nx:run-commands`, `"parallel": false`, commands `nx run <p>:typecheck` → `nx run <p>:lint` → `nx run <p>:test:unit`) — acceptance: running `test:quick` executes the three in order and stops at the first failure (verify by temporarily breaking lint in one project).
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Apply the content rules: `test:e2e` real only on `*-e2e` projects (echo elsewhere); BE `test:integration` is service-level (no HTTP); FE `test:integration` is echo unless DB-backed (keep `organiclever-app-web`'s PGlite integration real); `test:unit` includes BDD + non-BDD (coverage gated by the sibling `test:coverage` target, not here) — acceptance: spot-check one BE, one FE-without-DB, `organiclever-app-web`, and one `*-e2e` project match the rules.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] For EACH project with a real `test:unit`, add a native `test:coverage` target (≥ 90% line via the project's own runner — `vitest --coverage` thresholds, `cargo llvm-cov`/`tarpaulin`, `dotnet test` coverage gate) per the [§1.3 matrix](./tech-docs.md#13-per-project-target-matrix-post-implementation-ose-public) `test:coverage` column; `echo` where `test:unit` is `echo` — acceptance: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:coverage")' >/dev/null || echo "NO-COV: $p"; done` prints no `NO-COV`; a project under 90% fails its `test:coverage`.
- [ ] [AI] Wire `specs:domain:coverage` (→ `rhino-cli specs domain-coverage validate`) **only on `*-be` backend projects** (`ose-be`, `organiclever-be`) per the §1.3 matrix `specs:domain:coverage` column — acceptance: `npx nx show project ose-be --json | jq -e '.targets|has("specs:domain:coverage")'` is true; a non-`*-be` project (e.g. `ose-www`) does **not** declare it.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Make pre-push and the PR quality gate run the identical per-project code command — `nx affected -t test:quick` — and confirm **neither** runs `test:integration`/`test:e2e`; those stay only on the CRON pipelines. Edit `.husky/pre-push` and `.github/workflows/pr-quality-gate.yml` accordingly per [§1.2 gate rule](./tech-docs.md#12-testing-architecture--target-contents-standard) — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation; both run `test:quick`.
- [ ] [AI] Run the extended `specs:behavior:coverage` (`--require-consumption`) across affected projects and fix any orphan feature files (add the missing consuming test, or remove the dead feature with justification) — command: `npx nx affected -t specs:behavior:coverage` — acceptance: exits 0 with no orphan-feature errors.

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

- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — exits 0 (fix any preexisting failures).
- [ ] [AI] `npx nx run rhino-cli:shell:check` and `:dockerfiles:check` and `:actions:check` and `:harness:bindings-validation` — each exits 0; `npx nx run rhino-cli:fmt` and `:format:check` both fail (targets removed).
- [ ] [AI] Every project exposes the mandatory-six targets: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:unit") and has("test:integration") and has("test:e2e") and has("test:quick") and has("lint") and has("typecheck")' >/dev/null || echo "MISSING: $p"; done` — acceptance: prints no `MISSING` line.
- [ ] [AI] Coverage went native: `jq -e '.targets|has("test-coverage")|not' apps/rhino-cli/project.json` is true; every project with a real `test:unit` also exposes `test:coverage`; `grep -rin 'test-coverage\|codecov' apps repo-governance docs --include='*.md' --include='*.json' --include='*.rs' | grep -vi 'ExcludeFromCodeCoverage'` returns nothing — acceptance: no stale `test-coverage`/Codecov references remain in ose-public.
- [ ] [AI] `actionlint .github/workflows/main-ci.yml` — exits 0; the per-project matrix + staging-deploy legs are well-formed.
- [ ] [AI] `npm run lint:md` — exits 0.
- [ ] [AI] Commit rhino-cli target-name standardization: `git commit -m "chore(rhino-cli): standardize Nx target names (remove fmt/format:check, add check+bindings targets, remove test-coverage, rename specs:coverage to specs:behavior:coverage, add specs:domain:coverage)"` — acceptance: `git log --oneline -1` shows this commit; `npx nx run rhino-cli:shell:check` exits 0; `npx nx run rhino-cli:fmt` fails with "target not found".
- [ ] [AI] Commit lint-staged formatter map: `git commit -m "chore(config): add *.rs and *.fs file-type formatter entries to lint-staged"` — acceptance: `git log --oneline -1` shows this commit; staging a `*.rs` file and running pre-commit reformats it via rustfmt.
- [ ] [AI] Commit repo-config.yml merge: `git commit -m "chore(config): merge instruction-size-budget.yaml, env-contract.yaml, env-injection.yaml into repo-config.yml"` — acceptance: `git log --oneline -1` shows this commit; `test ! -f instruction-size-budget.yaml && test ! -f env-contract.yaml && test ! -f env-injection.yaml` passes.
- [ ] [AI] Commit hook rewire: `git commit -m "chore(hooks): rewire pre-commit to use Nx {tool}:check targets"` — acceptance: `git log --oneline -1` shows this commit; `bash .husky/pre-commit` on a staged no-op exits 0.
- [ ] [AI] Commit workflow renames + ref updates: `git commit -m "chore(ci): rename workflow files to canonical names (pr-quality-gate, validate-markdown, validate-env)"` — acceptance: `git log --oneline -1` shows this commit; `test -f .github/workflows/pr-quality-gate.yml && test -f .github/workflows/validate-markdown.yml && test -f .github/workflows/validate-env.yml` passes.
- [ ] [AI] Commit markdown-validator addition: `git commit -m "chore(ci): add gherkin-cardinality-validation step to validate-markdown.yml"` — acceptance: `git log --oneline -1` shows this commit; `actionlint .github/workflows/validate-markdown.yml` exits 0.
- [ ] [AI] Commit per-project target-contents: `git commit -m "chore(nx): add mandatory-six targets + test:quick sequential composition + native test:coverage to all projects"` — acceptance: `git log --oneline -1` shows this commit; the mandatory-six `jq` check (Phase 2 gate) prints no `MISSING` line.
- [ ] [AI] Commit gate rule (test:quick-only for pre-push + PR gate): `git commit -m "chore(ci): restrict pre-push and PR gate to test:quick; integration/e2e reserved for CRON"` — acceptance: `git log --oneline -1` shows this commit; `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.
- [ ] [AI] Commit post-merge main-ci + staging-deploy workflow: `git commit -m "chore(ci): add main-ci.yml per-project affected matrix + staging deploy on green"` — acceptance: `git log --oneline -1` shows this commit; `actionlint .github/workflows/main-ci.yml` exits 0.
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
- [ ] [AI] Apply the same rhino-cli source changes to primer (propagated rhino-cli): merge root configs into `repo-config.yml` + delete the 3 standalone files (§2a-cfg); ensure the lint-staged map covers `*.rs`/`*.fs` (§2a) — acceptance: `test -f repo-config.yml`; the 3 old files absent; `npx nx run rhino-cli:instruction-size:validation`/`:env:validation` exit 0.

### 3b. Standardize rhino-cli target names

- [ ] [AI] In primer `apps/rhino-cli/project.json`: **remove `fmt` + `format:check` targets** (formatting → lint-staged); rename `shell:lint`→`shell:check`, `dockerfiles:lint`→`dockerfiles:check`, `actions:lint`→`actions:check`; update every reference (`grep -rn 'rhino-cli:fmt\b\|rhino-cli:format:check\|:shell:lint\|:dockerfiles:lint\|:actions:lint' --include='*.json' --include='*.yml' --include='*.sh' --include='*.md' .`) — acceptance: `npx nx run rhino-cli:shell:check`/`:dockerfiles:check`/`:actions:check` each exit 0; `:fmt`/`:format:check` fail; zero stale references.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Add the structural targets primer's rhino-cli is **missing** so its key set matches public/infra: `specs:adoption-validation`, `specs:counts-validation`, `specs:links-validation`, `specs:tree-validation`, `test:e2e` (echo) — and mirror public's coverage decision: **do not** add `test-coverage` (it is removed everywhere); add a native `test:coverage` target instead — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set (which contains `test:coverage`, not `test-coverage`).
  - _Suggested executor: `swe-rust-dev`_

### 3c. Hook + workflow parity

- [ ] [AI] Add `governance:vendor-audit-validation` to primer's `.husky/pre-push` scoped validators (gated on `^repo-governance/.*\.md$`) — acceptance: editing a `repo-governance/*.md` then running pre-push triggers it; exits 0.
- [ ] [AI] Promote primer's deferred structural specs-gate in `.github/workflows/pr-quality-gate.yml` to the full set (`specs:adoption-validation` + `specs:tree-validation` + `specs:counts-validation` + `specs:links-validation` + `specs:behavior:coverage` + `specs:gherkin-cardinality-validation`) — acceptance: `actionlint` passes; job lists all six.
- [ ] [AI] Extract a standalone `.github/workflows/validate-env.yml` from primer's folded-in PR-gate env job (`npx nx run rhino-cli:env:validation` on `pull_request` + `push:main`); remove the duplicated env logic from the PR gate — acceptance: `actionlint` passes; `validate-env.yml` matches the public/infra shape.
- [ ] [AI] Confirm primer's `validate-markdown.yml` runs the 4-validator set (mermaid + links + headings + gherkin-cardinality) — acceptance: all four present.
- [ ] [AI] Align primer's PR-gate job skeleton to the standard (detect, language gates, markdown, naming, env, specs-gate, quality-gate sentinel; formatting is enforced by lint-staged at commit, not a PR-gate job); **keep** primer's per-language jobs (golang/jvm/dotnet/python/rust/elixir/clojure/dart — allowed divergence) — acceptance: `actionlint` passes; skeleton matches, language jobs preserved.

### 3d. Mandatory-six sweep across all 26 primer projects

- [ ] [AI] For EACH primer project, bring its `project.json` to the [§1.3b matrix](./tech-docs.md#13b-per-project-target-matrix-post-implementation-ose-primer) — biggest gaps: add `test:e2e` (echo) to the 11 `crud-be-*` + `crud-fs-ts-nextjs`; add `test:integration`+`test:e2e` (echo) to `crud-fe-*`; fill the support libs (`ts-ui-tokens` needs 4: `test:unit`/`test:integration`/`test:e2e` echo + `test:quick`; `golang-commons`/`clojure-openapi-codegen` need `typecheck` echo + more; `elixir-*` + `ts-ui` need `test:integration`/`test:e2e` echo); add `specs:behavior:coverage` to libs lacking it; add `specs:domain:coverage` to the 11 `crud-be-*` backends (**no `format` target anywhere** — lint-staged handles it) — acceptance: the mandatory-six `jq` check (Phase 2 gate) prints no `MISSING` for any primer project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Set every primer project's `test:quick` to the sequential typecheck→lint→test:unit composition (`nx:run-commands`, `parallel:false`) — acceptance: order verified by breaking lint in one project.
- [ ] [AI] Add a native `test:coverage` target (≥ 90% line via each project's own runner; `echo` where `test:unit` is `echo`) to every primer project per the [§1.3b matrix](./tech-docs.md#13b-per-project-target-matrix-post-implementation-ose-primer) `test:coverage` column — acceptance: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:coverage")' >/dev/null || echo "NO-COV: $p"; done` prints no `NO-COV`.
- [ ] [AI] Wire `specs:domain:coverage` on the 11 `crud-be-*` backend projects (per §1.3b matrix) — acceptance: `npx nx show project crud-be-rust-axum --json | jq -e '.targets|has("specs:domain:coverage")'` is true; `crud-fe-*`/libs do **not** declare it.
- [ ] [AI] Resolve orphan features: `npx nx run-many -t specs:behavior:coverage` with `--require-consumption` — acceptance: no orphan-feature errors.

### 3e. Post-merge CI (primer is a template — tests only, deploy is a no-op)

- [ ] [AI] Add `.github/workflows/main-ci.yml` mirroring public's per-project affected matrix (`test:quick`→`test:integration`→`test:e2e` per project) — acceptance: `actionlint` passes; a merge dispatches one leg per affected project.
- [ ] [AI] Document that primer's deploy leg is a **no-op** (the `crud-*` demo apps have no live staging env — they are reference scaffolding); keep the `test-and-deploy-*-development` local-stack workflows as the nightly fallback — acceptance: `docs/reference/sdlc-gate-standard.md` in primer states the no-deploy rationale; no deploy step is wired.
- [ ] [AI] Confirm primer pre-push ≡ PR run only `test:quick` (+ validators), never integration/e2e — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] In ose-primer: `npx nx run-many -t typecheck lint test:quick specs:behavior:coverage` — exits 0.
- [ ] [AI] In ose-primer, every project exposes the mandatory-six: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:unit") and has("test:integration") and has("test:e2e") and has("test:quick") and has("lint") and has("typecheck")' >/dev/null || echo "MISSING: $p"; done` — acceptance: prints no `MISSING` line.
- [ ] [AI] In ose-primer: `npm run lint:md` and `actionlint` on changed workflows — exit 0.
- [ ] [AI] Commit propagated artifacts + config merge: `git commit -m "chore(config): propagate standardize-rhino-cli-sdlc-parity plan artifacts and merge repo-config.yml into ose-primer"` — acceptance: `git log --oneline -1` shows this commit; `test -f repo-config.yml` passes.
- [ ] [AI] Commit rhino-cli target-name standardization: `git commit -m "chore(rhino-cli): standardize Nx target names in ose-primer (remove fmt/format:check, add check+bindings, rename specs:coverage to specs:behavior:coverage)"` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set.
- [ ] [AI] Commit hook + workflow parity: `git commit -m "chore(ci): align primer hooks and workflows to canonical standard (validate-env.yml, full specs-gate, governance-vendor in pre-push)"` — acceptance: `actionlint` passes; `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.
- [ ] [AI] Commit mandatory-six sweep: `git commit -m "chore(nx): add mandatory-six targets + sequential test:quick + native test:coverage + specs:domain:coverage to all 26 primer projects"` — acceptance: mandatory-six `jq` check prints no `MISSING`; no `NO-COV` project.
- [ ] [AI] Commit post-merge CI: `git commit -m "chore(ci): add main-ci.yml per-project affected matrix to ose-primer (template — no deploy leg)"` — acceptance: `actionlint .github/workflows/main-ci.yml` exits 0.
- [ ] [AI] Push ose-primer to `origin main` and poll CI: `gh run view --json status,conclusion` every 2 min until complete — acceptance: all checks green (incl. new `validate-env.yml`, promoted specs-gate, `main-ci.yml`).

> **Pause Safety**: ose-public + ose-primer converged and green. Safe to stop. To resume (primer): `npx nx affected -t lint`.

---

## Phase 4: Propagate + Converge ose-infra

> Executes in `ose-infra` (normal repo; commit to `main` directly). Target state =
> the [§1.3c infra matrix](./tech-docs.md#13c-per-project-target-matrix-post-implementation-ose-infra).
> Infra already matches the workflow filenames (`pr-quality-gate.yml`, `validate-markdown.yml`,
> `validate-env.yml`) + governance-vendor pre-push, but (like public) lacks the `{tool}:check` +
> `harness:bindings-validation` Nx targets and still has `fmt`/`format:check` (to be removed → lint-staged). CI runs on the self-hosted runner.

### 4a. Baseline + propagate

- [ ] [AI] In ose-infra: `npm install && npm run doctor -- --fix`; `npx nx build rhino-cli` — acceptance: doctor green; rhino-cli builds.
- [ ] [AI] Propagate the artifacts + the `nx-targets.md`/`nx-target-naming.md` additions into ose-infra; replace the matrix with the §1.3c infra matrix; document infra-only IaC gates (terraform/ansible/yamllint) and the self-hosted runner in the divergence section of `docs/reference/sdlc-gate-standard.md` — acceptance: artifacts exist; `npm run lint:md` passes.
- [ ] [AI] Apply the same rhino-cli source changes to infra (propagated rhino-cli): merge root configs into `repo-config.yml` + delete the 3 standalone files (§2a-cfg); ensure the lint-staged map covers `*.rs`/`*.fs` (§2a) — acceptance: `test -f repo-config.yml`; the 3 old files absent; `:instruction-size:validation`/`:env:validation` exit 0.

### 4b. Standardize rhino-cli target names

- [ ] [AI] In infra `apps/rhino-cli/project.json`: **remove `fmt` + `format:check` targets** (formatting → lint-staged); add `shell:check`/`dockerfiles:check`/`actions:check` + `harness:bindings-validation` Nx targets (same defs as public); **remove the `test-coverage` target** (the command is gone from the propagated rhino-cli source); update references; rewire `.husky/pre-commit` to the `{tool}:check` targets and `.husky/pre-push` to `npx nx run rhino-cli:harness:bindings-validation` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set (no `fmt`/`format:check`/`test-coverage`); each new target exits 0.
  - _Suggested executor: `swe-rust-dev`_

### 4c-codecov. Remove Codecov residue (infra — last repo still carrying it)

- [ ] [AI] Delete `ose-infra/codecov.yml` (the last live Codecov config across the three repos; public + primer already removed it) — acceptance: `test ! -f codecov.yml`.
- [ ] [AI] Scrub stale Codecov references from infra governance docs + `apps/rhino-cli/README.md`: remove the `codecov-upload.yml` CRON row/bullets from `repo-governance/development/quality/three-level-testing-standard.md`, the `codecov-upload.yml` upload step from `repo-governance/development/infra/ci-conventions.md`, the "Codecov algorithm"/`test-coverage validate` text from `repo-governance/development/infra/nx-targets.md` and `apps/rhino-cli/README.md` — acceptance: `grep -rin codecov . | grep -vi 'ExcludeFromCodeCoverage'` returns nothing in ose-infra; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-fixer`_

### 4c. Confirm workflow + hook parity (record IaC divergence)

- [ ] [AI] Verify infra's `pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml` match the standard filenames + validator sets (markdown 4-validator; specs-gate full set) — acceptance: filenames identical; validator sets match; any gap recorded as a fix step.
- [ ] [AI] Confirm infra's pre-commit/pre-push step order matches the standard, with terraform/ansible/yamllint as **documented allowed additions** (not drift) and the `[self-hosted, linux, ose-infra-runner]` label retained — acceptance: order matches; IaC + runner appear only in the divergence section.
- [ ] [AI] Fix any gaps found above — acceptance: each fixed gate exits 0 locally.

### 4d. Mandatory-six sweep across all 7 infra projects

- [ ] [AI] Bring each infra project to the [§1.3c matrix](./tech-docs.md#13c-per-project-target-matrix-post-implementation-ose-infra): `coralpolyp-be` keeps service-level `test:integration` **and gains `specs:domain:coverage`**; `coralpolyp-fe` integration real only if DB-backed else echo; `ts-ui-tokens` gains its 4 missing targets; `ts-ui` gains `test:integration`/`test:e2e` echo; `*-e2e` keep real `test:e2e`, echo `test:unit`/`test:integration` (**no `format` target anywhere** — lint-staged handles it) — acceptance: the mandatory-six `jq` check prints no `MISSING` for any infra project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Set every infra project's `test:quick` to the sequential typecheck→lint→test:unit composition; resolve orphan features via `specs:behavior:coverage --require-consumption` — acceptance: order verified; no orphan-feature errors.
- [ ] [AI] Add a native `test:coverage` target (≥ 90% line; `echo` where `test:unit` is `echo`) to every infra project per the [§1.3c matrix](./tech-docs.md#13c-per-project-target-matrix-post-implementation-ose-infra) `test:coverage` column — acceptance: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:coverage")' >/dev/null || echo "NO-COV: $p"; done` prints no `NO-COV`.
- [ ] [AI] Wire `specs:domain:coverage` on `coralpolyp-be` (the only infra `*-be` backend) per §1.3c matrix — acceptance: `npx nx show project coralpolyp-be --json | jq -e '.targets|has("specs:domain:coverage")'` is true; `coralpolyp-fe`/libs do **not** declare it.

### 4e. Post-merge CI + coralpolyp staging deploy

- [ ] [AI] Add `.github/workflows/main-ci.yml` (self-hosted) with the per-project affected matrix (`test:quick`→`test:integration`→`test:e2e`); on a green `coralpolyp-be`/`coralpolyp-fe` leg, deploy to coralpolyp staging by reusing the `test-and-deploy-coralpolyp-development` build/deploy logic (merge-triggered) — acceptance: `actionlint` passes; a green coralpolyp leg deploys to staging.
- [ ] [AI] Reduce the existing coralpolyp dev/stag CRON cadence to nightly fallback; keep `test-coralpolyp-staging.yml` → prod promotion scheduled — acceptance: `actionlint` passes; crons run once/day; prod promotion unchanged.
- [ ] [AI] Confirm infra pre-push ≡ PR run only `test:quick` (+ validators + IaC), never integration/e2e — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] In ose-infra: `npx nx run-many -t typecheck lint test:quick specs:behavior:coverage` — exits 0.
- [ ] [AI] In ose-infra, every project exposes the mandatory-six: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:unit") and has("test:integration") and has("test:e2e") and has("test:quick") and has("lint") and has("typecheck")' >/dev/null || echo "MISSING: $p"; done` — acceptance: prints no `MISSING` line.
- [ ] [AI] In ose-infra, coverage went native + Codecov gone: `jq -e '.targets|has("test-coverage")|not' apps/rhino-cli/project.json` is true; every real-`test:unit` project also exposes `test:coverage`; `test ! -f codecov.yml`; `grep -rin codecov . | grep -vi 'ExcludeFromCodeCoverage'` returns nothing — acceptance: no `test-coverage`/Codecov residue in ose-infra.
- [ ] [AI] In ose-infra: `npm run lint:md` and `actionlint` on changed workflows — exit 0.
- [ ] [AI] Commit propagated artifacts + config merge: `git commit -m "chore(config): propagate standardize-rhino-cli-sdlc-parity plan artifacts and merge repo-config.yml into ose-infra"` — acceptance: `git log --oneline -1` shows this commit; `test -f repo-config.yml` passes; 3 standalone config files absent.
- [ ] [AI] Commit rhino-cli target-name standardization: `git commit -m "chore(rhino-cli): standardize Nx target names in ose-infra (remove fmt/format:check/test-coverage, add check+bindings, specs:behavior:coverage)"` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set; `npx nx run rhino-cli:shell:check` exits 0.
- [ ] [AI] Commit Codecov removal + workflow parity: `git commit -m "chore(ci): remove Codecov residue and confirm workflow+hook parity in ose-infra"` — acceptance: `test ! -f codecov.yml`; `grep -rin codecov . | grep -vi 'ExcludeFromCodeCoverage'` returns nothing; `actionlint` passes.
- [ ] [AI] Commit mandatory-six sweep: `git commit -m "chore(nx): add mandatory-six targets + sequential test:quick + native test:coverage + specs:domain:coverage to all 7 infra projects"` — acceptance: mandatory-six `jq` check prints no `MISSING`; no `NO-COV` project.
- [ ] [AI] Commit post-merge CI + coralpolyp staging: `git commit -m "chore(ci): add main-ci.yml per-project affected matrix + coralpolyp staging deploy to ose-infra"` — acceptance: `actionlint .github/workflows/main-ci.yml` exits 0; a green coralpolyp leg deploys to staging.
- [ ] [AI] Push ose-infra to `origin main` and poll CI: `gh run view --json status,conclusion` every 2 min until complete — acceptance: all checks green on the self-hosted runner (incl. `main-ci.yml`).

> **Pause Safety**: all three repos converged and green. Safe to stop. To resume (infra): `npx nx affected -t lint`.

---

## Phase 5: Cross-Repo Parity Verification & Archival

- [ ] [AI] Build the parity table comparing all three repos across every mechanics row (PR-gate filename, markdown filename, env filename, markdown validator set, specs-gate set, lint invocation mechanism, pre-push governance-vendor presence, hook step order, rhino-cli target-key set, **rhino-cli command set verb-last + identical**, **`repo-config.yml` section schema identical**, mandatory targets on every project, `test:quick` = typecheck→lint→test:unit composition, native `test:coverage` ≥ 90% gate on every real-`test:unit` project, **no** `test-coverage` target + **no** Codecov anywhere, `format` via file-type lint-staged (no per-project `format` target), pre-push ≡ PR runs only `test:quick`, `specs:behavior:coverage --require-consumption` (feature **+ scenario** eligible coverage) enabled, canonical CI workflow names present) — acceptance: a table with a ✅/❌ per repo per row is produced; every mechanics row is ✅ across all three (allowed-divergence rows excluded); the standardization layer is confirmed **identical** cross-repo.
- [ ] [AI] Record the parity table in each repo's `docs/reference/sdlc-gate-standard.md` under a "Parity Status" heading — acceptance: present in all three; lint:md passes.

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` (each repo).
- [ ] [AI] Run affected linting: `npx nx affected -t lint` (each repo).
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` (each repo).
- [ ] [AI] Run affected spec coverage: `npx nx affected -t specs:behavior:coverage` (each repo).
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

### Phase 5 Gate

> All checks below must pass before archival.

- [ ] [AI] The parity table shows ✅ on every mechanics row across all three repos (allowed-divergence rows excluded) — acceptance: no ❌ in any mechanics row.
- [ ] [AI] **CI standardization complete** (plan scope boundary): in each repo the four canonical workflows exist with the exact ose-public names (`pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml`, `main-ci.yml`) and every project resolves into `main-ci.yml`'s affected matrix — acceptance: `for w in pr-quality-gate validate-markdown validate-env main-ci; do test -f .github/workflows/$w.yml || echo "MISSING-CI: $w"; done` prints nothing in each repo; the affected matrix lists every project.
- [ ] [AI] **Config + coverage cleanup complete**: `repo-config.yml` exists and the 3 standalone config files are absent; no `format`/`format:check`/`test-coverage` targets; `grep -ri codecov` returns only `ExcludeFromCodeCoverage` — in all three repos.
- [ ] [AI] `docs/reference/sdlc-gate-standard.md` (with the Parity Status table) and `rhino-cli-command-triage.md` exist in all three repos — acceptance: `npm run lint:md` passes in each.
- [ ] [AI] All three repos green on local gates (`npx nx affected -t typecheck lint test:quick specs:behavior:coverage`) and on CI for the latest `main` push — acceptance: each repo's latest `gh run view --json conclusion` reports `success`.

> **Pause Safety**: all three repos converged, parity-verified, and green; nothing half-applied. Safe to stop. To resume: re-run the cross-repo parity verification table (Phase 5 step 1) and confirm all-green.

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
