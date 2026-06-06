# Delivery Checklist — Gherkin Step-Keyword Cardinality Rule

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/gherkin-step-keyword-cardinality/`

Provision before execution (run from repo root):

```bash
claude --worktree gherkin-step-keyword-cardinality
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the full polyglot toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
- [ ] [AI] Confirm the Rust toolchain builds rhino-cli: `npx nx run rhino-cli:build`
      — acceptance: exits 0.
- [ ] [AI] Record the current `.feature` inventory: `find specs -name '*.feature' | wc -l`
      — acceptance: count recorded (expected 124 at authoring; record actual).
- [ ] [AI] Establish the test baseline for affected projects:
      `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: baseline pass/fail recorded; every preexisting failure documented.
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npx nx run rhino-cli:build` exits 0.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` baseline recorded and
      every preexisting failure resolved (zero unresolved).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run
> `npx nx affected -t typecheck lint test:quick spec-coverage` and confirm it is still clean.

## Phase 1: Author the HARD rule in the canonical convention (via repo-rules-maker)

_Suggested executor: `repo-rules-maker`_

- [ ] [AI] Edit `repo-governance/development/infra/acceptance-criteria.md`: add a HARD rule
      stating that every `Scenario` uses exactly one primary `Given`, one `When`, and one `Then`,
      with all extras chained via `And`/`But`, and that `Background` blocks and `Scenario Outline`
      `Examples` tables are exempt. Include the conforming example and the non-conforming
      (multi-`When`) example from `prd.md` §"The HARD Rule".
      — acceptance: the rule text and both examples are present; `grep -n "exactly one" repo-governance/development/infra/acceptance-criteria.md` returns the rule line.
- [ ] [AI] In the same file, normalize every illustrative Gherkin snippet that currently repeats
      a primary `Given`/`When`/`Then` keyword so it uses `And`/`But` instead.
      — acceptance: no snippet in the file has two `When` or two `Then` lines in the same scenario
      (verify by manual scan; the Phase 14 linter is the authoritative check).

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx affected -t lint` exits 0 (markdown lint passes for the edited convention).
- [ ] [AI] The rule and both examples are present in `acceptance-criteria.md`.

> **Pause Safety**: only the canonical convention changed; the repo is coherent (docs-only edit).
> Safe to stop. To resume: re-run `npx nx affected -t lint`.

## Phase 2: Broad governance sweep + agent prompts (via repo-rules-maker)

_Suggested executor: `repo-rules-maker`_

- [ ] [AI] Edit `repo-governance/development/infra/bdd-spec-test-mapping.md`: reference the new
      HARD rule where scenario structure / step mapping is discussed.
      — acceptance: file references the one-each keyword rule and links to `acceptance-criteria.md`.
- [ ] [AI] Edit `repo-governance/conventions/structure/plans.md`: reference the rule where Gherkin
      acceptance criteria are discussed.
      — acceptance: file references the rule.
- [ ] [AI] Edit `repo-governance/development/infra/best-practices.md`: add the one-each keyword
      shape to the Gherkin best-practices guidance.
      — acceptance: file references the rule.
- [ ] [AI] Edit `repo-governance/development/infra/anti-patterns.md`: add "multiple primary
      `When`/`Then` keyword lines in one scenario" as an explicit anti-pattern.
      — acceptance: file lists the multi-keyword anti-pattern.
- [ ] [AI] Edit `.claude/agents/plan-maker.md`: add the rule to the Gherkin-authoring guidance so
      plan `prd.md` criteria conform.
      — acceptance: file references the rule.
- [ ] [AI] Edit `.claude/agents/plan-checker.md`: add the rule to the AI judgment criteria so
      plan Gherkin is reviewed for keyword cardinality.
      — acceptance: file references the rule as a checked criterion.
- [ ] [AI] Edit `.claude/agents/repo-rules-checker.md`: add the rule to its judgment criteria.
      — acceptance: file references the rule as a checked criterion.
- [ ] [AI] Sweep for any other Gherkin-referencing `repo-governance/` doc and add a reference:
      `grep -rln -i gherkin repo-governance/` — review each hit and reference the rule where a
      scenario-structure discussion exists.
      — acceptance: every Gherkin-discussing governance doc references the rule (no orphan surface).
- [ ] [AI] Re-sync secondary bindings so agent-prompt edits propagate to `.opencode/`:
      `npm run generate:bindings`
      — acceptance: exits 0; `git status` shows regenerated `.opencode/agents/` mirrors, no parity drift.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx affected -t lint` exits 0 (markdown + binding parity).
- [ ] [AI] `npm run generate:bindings` produced no uncommitted drift beyond the intended edits
      (`git status` reviewed).

> **Pause Safety**: docs + agent-prompt + binding edits only; repo is coherent (no code change yet).
> Safe to stop. To resume: re-run `npx nx affected -t lint`.

## Phase 3: Manual skill propagation (without repo-rules-maker)

> Edit the two skill packages by hand — do NOT delegate to `repo-rules-maker`.

- [ ] [AI] Edit `.claude/skills/plan-writing-gherkin-criteria/SKILL.md` by hand: add a dedicated
      "Step-Keyword Cardinality" section stating the HARD rule + exemptions, and normalize every
      example snippet that repeats a primary keyword to use `And`/`But`.
      — acceptance: the rule section is present and no snippet repeats a primary `When`/`Then`.
- [ ] [AI] Edit `.claude/skills/plan-creating-project-plans/SKILL.md` by hand: reference the rule in
      the Gherkin acceptance-criteria guidance.
      — acceptance: file references the rule and links to the canonical convention.
- [ ] [AI] Re-sync secondary bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/` / `.amazonq/` regenerated with no parity drift
      (skills are not mirrored, but bindings must re-sync cleanly).

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx affected -t lint` exits 0.
- [ ] [AI] Both skill files reference the rule; `npm run generate:bindings` left no parity drift.

> **Pause Safety**: docs/skills/bindings only; repo coherent. Safe to stop. To resume:
> re-run `npx nx affected -t lint`.

## Phase 4: Build the deterministic `gherkin-keyword-cardinality` audit (TDD)

_Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED** — Create the audit module
      `apps/rhino-cli/src/internal/repo_governance/gherkin_keyword_cardinality_audit.rs`
      (sibling pattern: `emoji_audit.rs`) with a failing unit test
      _New test_ `flags_scenario_with_multiple_when_lines` asserting a scenario with two primary
      `When` lines yields one finding. Run `npx nx run rhino-cli:test:unit` — acceptance: the new
      test FAILS to compile or fails the assertion (red).
- [ ] [AI] **RED** — Add failing tests _New test_ `exempts_background_block`,
      _New test_ `exempts_scenario_outline_examples`, and
      _New test_ `ignores_keyword_words_in_docstrings_and_comments`
      in the same module. Run `npx nx run rhino-cli:test:unit` — acceptance: the three new tests
      fail (red).
- [ ] [AI] **GREEN** — Implement the audit: parse each `.feature` file, group lines by `Scenario`,
      count primary `Given`/`When`/`Then` keyword lines (a primary keyword starts the trimmed line
      and is not `And`/`But`/`*`), emit a finding when any primary keyword count > 1, and skip lines
      inside `Background:`, `Scenario Outline:` `Examples:` tables, doc-strings (`"""`), and comments
      (`#`). Run `npx nx run rhino-cli:test:unit` — acceptance: all four new tests pass (green).
- [ ] [AI] **REFACTOR** — Extract the line-classification helper for reuse and de-duplicate parsing;
      keep all tests green. Run `npx nx run rhino-cli:test:unit` and `npx nx run rhino-cli:lint`
      — acceptance: tests pass, lint exits 0.
- [ ] [AI] Create the CLI command
      `apps/rhino-cli/src/commands/governance_gherkin_keyword_cardinality_audit.rs`
      (sibling pattern: `governance_emoji_audit.rs`) exposing
      `repo-governance gherkin-keyword-cardinality` that scans `specs/**/*.feature` by default.
      — acceptance: `npx nx run rhino-cli:build` exits 0 and
      `./apps/rhino-cli/dist/rhino-cli repo-governance gherkin-keyword-cardinality --help`
      prints usage (the build target copies the release binary to `apps/rhino-cli/dist/rhino-cli`).
- [ ] [AI] Register the command module in the rhino-cli command registry
      (`apps/rhino-cli/src/commands.rs` — flat module file confirmed present via
      `test -f apps/rhino-cli/src/commands.rs` [Repo-grounded]; secondary wiring in
      `apps/rhino-cli/src/cli.rs` — cross-check via
      `grep -rn "governance_emoji_audit" apps/rhino-cli/src/`).
      — acceptance: the command is dispatchable from the CLI.
- [ ] [AI] Wire the category into
      `apps/rhino-cli/src/internal/repo_governance/audit_orchestrator.rs` (add the module `use`,
      the category id `"gherkin-keyword-cardinality"`, and the dispatch arm — mirror the
      `emoji-audit` references) and into
      `apps/rhino-cli/src/commands/governance_audit.rs`.
      — acceptance: `grep -n "gherkin-keyword-cardinality" apps/rhino-cli/src/internal/repo_governance/audit_orchestrator.rs`
      returns the registration; `npx nx run rhino-cli:test:unit` passes.
- [ ] [AI] Add the new category to the Step 0.5 deterministic preflight enumeration in
      `repo-governance/workflows/repo/repo-rules-quality-gate.md`.
      — acceptance: the file lists `gherkin-keyword-cardinality` among preflight categories.
- [ ] [AI] Wire the category into CI: locate the workflow that runs the rhino-cli governance audit
      (`grep -rln "repo-governance" .github/workflows/` — if none, confirm CI invokes the audit via
      the quality-gate workflow) and ensure the new category is included.
      — acceptance: CI runs the new audit, OR it is documented that CI invokes it transitively via
      the quality-gate preflight (record which).
- [ ] [AI] Add the rule to the AI judgment criteria already edited in Phase 2 for `plan-checker`
      and `repo-rules-checker` (cross-check the deterministic linter complements the AI judgment).
      — acceptance: no-op if already present from Phase 2; otherwise add and re-sync bindings.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run rhino-cli:test:unit` exits 0 with all new tests green.
- [ ] [AI] `npx nx run rhino-cli:lint` and `npx nx run rhino-cli:build` exit 0.
- [ ] [AI] The built binary runs `repo-governance gherkin-keyword-cardinality` against
      `specs/**/*.feature` and prints a finding list (may be non-empty — offenders are fixed in
      Phases 5–13).

> **Pause Safety**: the linter exists and is green on its own tests; the spec corpus may still have
> offenders but nothing is broken (the linter is additive). Safe to stop. To resume:
> re-run `npx nx run rhino-cli:test:unit`.

---

> **Per-app retrofit phases (5–13)** — each phase: (1) run the new linter scoped to that project's
> spec subtree to discover offenders, (2) normalize offending scenarios (replace repeated primary
> keywords with `And`/`But`) AND update step definitions in lockstep, (3) gate on the project's
> tests + spec coverage. If the linter reports **zero offenders** for a project, make no edits but
> still run the gate. Do NOT fabricate offender counts — discover them at execution.

## Phase 5: Retrofit `specs/apps/rhino` (cucumber-rs)

_Suggested executor: `swe-rust-dev`_

- [ ] [AI] Run the linter scoped to rhino specs:
      `./apps/rhino-cli/dist/rhino-cli repo-governance gherkin-keyword-cardinality --path specs/apps/rhino`
      (use the path flag if supported; otherwise grep the full-run output for `specs/apps/rhino`).
      — acceptance: offender list for `specs/apps/rhino` recorded.
- [ ] [AI] For each offender, replace repeated primary `Given`/`When`/`Then` lines with `And`/`But`
      in the `.feature` file. Note: the cucumber-rs step harness for rhino-cli is not yet
      implemented (spec-coverage is stubbed per `project.json`; `apps/rhino-cli/tests/` contains
      only `cli_smoke.rs`). Only `apps/rhino-cli/tests/cli_smoke.rs` needs a grep check to confirm
      no matching step text breaks — run:
      `grep -n "<step phrase>" apps/rhino-cli/tests/cli_smoke.rs`
      If no match, no step-def update is needed; the `.feature` normalization alone is sufficient.
      — acceptance: linter reports zero violations for `specs/apps/rhino`.

### Phase 5 Gate

- [ ] [AI] Linter reports zero `specs/apps/rhino` violations.
- [ ] [AI] `npx nx run rhino-cli:test:quick` and `npx nx run rhino-cli:spec-coverage` exit 0.

> **Pause Safety**: rhino specs conform and rhino tests pass. Safe to stop. To resume: re-run the
> rhino-scoped linter + `npx nx run rhino-cli:test:quick`.

## Phase 6: Retrofit `specs/apps/organiclever` (organiclever-be + organiclever-web)

_Suggested executor: `swe-rust-dev` (be) / `swe-typescript-dev` (web)_

- [ ] [AI] Run the linter scoped to `specs/apps/organiclever`; record offenders.
- [ ] [AI] Normalize offending `.feature` files and update step definitions in lockstep for both
      `organiclever-be` (Rust) and `organiclever-web` (TS) owners.
      Step-definition file globs to update in lockstep: - `organiclever-be` (Rust): `apps/organiclever-be/tests/unit/main.rs` and
      `apps/organiclever-be/tests/integration/main.rs` - `organiclever-web` unit TS steps: `apps/organiclever-web/test/unit/steps/**/*.steps.tsx` - `organiclever-web` e2e TS steps: `apps/organiclever-web-e2e/steps/*.steps.ts`
      and `apps/organiclever-be-e2e/steps/*.steps.ts`
      To discover which specific step file binds to an offending scenario line, run:
      `grep -rln "<step phrase>" apps/organiclever-be/tests/ apps/organiclever-web/test/ apps/organiclever-web-e2e/steps/ apps/organiclever-be-e2e/steps/`
      — acceptance: linter reports zero violations for `specs/apps/organiclever`.

### Phase 6 Gate

- [ ] [AI] Linter reports zero `specs/apps/organiclever` violations.
- [ ] [AI] `npx nx run organiclever-be:test:quick`, `npx nx run organiclever-web:test:quick`, and
      `npx nx run organiclever-be:spec-coverage` + `npx nx run organiclever-web:spec-coverage` exit 0.

> **Pause Safety**: organiclever specs conform and tests pass. Safe to stop. To resume: re-run the
> organiclever-scoped linter + the two `test:quick` targets.

## Phase 7: Retrofit `specs/apps/ayokoding` (ayokoding-cli + ayokoding-web)

_Suggested executor: `swe-rust-dev` (cli) / `swe-typescript-dev` (web)_

- [ ] [AI] Run the linter scoped to `specs/apps/ayokoding`; record offenders.
- [ ] [AI] Normalize offending `.feature` files and update step defs in lockstep.
      Step-definition file globs to update in lockstep: - `ayokoding-cli` (Rust, no Godog — uses inline `#[test]` assertions):
      `apps/ayokoding-cli/tests/cli_smoke.rs` (sole test file; grep for matching step text) - `ayokoding-web` unit TS steps: `apps/ayokoding-web/test/unit/fe-steps/*.steps.tsx` and
      `apps/ayokoding-web/test/unit/be-steps/*.steps.ts` - `ayokoding-web` integration TS steps: `apps/ayokoding-web/test/integration/be-steps/*.steps.ts` - `ayokoding-web` e2e TS steps: `apps/ayokoding-web-fe-e2e/src/steps/*.steps.ts` and
      `apps/ayokoding-web-be-e2e/src/steps/*.steps.ts`
      To discover which specific step file binds to an offending scenario line, run:
      `grep -rln "<step phrase>" apps/ayokoding-cli/tests/ apps/ayokoding-web/test/ apps/ayokoding-web-fe-e2e/src/steps/ apps/ayokoding-web-be-e2e/src/steps/`
      — acceptance: linter reports zero violations for `specs/apps/ayokoding`.

### Phase 7 Gate

- [ ] [AI] Linter reports zero `specs/apps/ayokoding` violations.
- [ ] [AI] `npx nx run ayokoding-cli:test:quick`, `npx nx run ayokoding-web:test:quick`,
      `npx nx run ayokoding-cli:spec-coverage`, `npx nx run ayokoding-web:spec-coverage` exit 0.

> **Pause Safety**: ayokoding specs conform and tests pass. Safe to stop. To resume: re-run the
> ayokoding-scoped linter + the two `test:quick` targets.

## Phase 8: Retrofit `specs/apps/crane` (crane-cli)

_Suggested executor: `swe-rust-dev`_

- [ ] [AI] Run the linter scoped to `specs/apps/crane`; record offenders.
- [ ] [AI] Normalize offending `.feature` files and update crane-cli step definitions in lockstep.
      Step-definition file globs to update in lockstep: - `crane-cli` (F#): `apps/crane-cli/tests/unit/Steps/*.fs` (e.g., `TextSteps.fs`,
      `FigureSteps.fs`, `TableSteps.fs`, `NestingSteps.fs`, `MermaidSteps.fs`, `ReportSteps.fs`,
      `CheckAllSteps.fs`)
      To discover which specific step file binds to an offending scenario line, run:
      `grep -rln "<step phrase>" apps/crane-cli/tests/`
      — acceptance: linter reports zero violations for `specs/apps/crane`.

### Phase 8 Gate

- [ ] [AI] Linter reports zero `specs/apps/crane` violations.
- [ ] [AI] `npx nx run crane-cli:test:quick` and `npx nx run crane-cli:spec-coverage` exit 0.

> **Pause Safety**: crane specs conform and tests pass. Safe to stop. To resume: re-run the
> crane-scoped linter + `npx nx run crane-cli:test:quick`.

## Phase 9: Retrofit `specs/apps/ose-platform` (ose-web + ose-cli)

_Suggested executor: `swe-rust-dev` (cli) / `swe-typescript-dev` (web)_

- [ ] [AI] Run the linter scoped to `specs/apps/ose-platform`; record offenders.
- [ ] [AI] Normalize offending `.feature` files and update step defs in lockstep.
      Step-definition file globs to update in lockstep: - `ose-cli` (Rust, no Godog): `apps/ose-cli/tests/cli_smoke.rs` (sole test file; grep for
      matching step text) - `ose-web` unit TS steps: `apps/ose-web/test/unit/fe-steps/*.steps.tsx` and
      `apps/ose-web/test/unit/be-steps/*.steps.ts` - `ose-web` integration TS steps: `apps/ose-web/test/integration/be-steps/*.steps.ts` - `ose-web` e2e TS steps: `apps/ose-web-fe-e2e/src/steps/*.steps.ts` and
      `apps/ose-web-be-e2e/src/steps/*.steps.ts`
      To discover which specific step file binds to an offending scenario line, run:
      `grep -rln "<step phrase>" apps/ose-cli/tests/ apps/ose-web/test/ apps/ose-web-fe-e2e/src/steps/ apps/ose-web-be-e2e/src/steps/`
      — acceptance: linter reports zero violations for `specs/apps/ose-platform`.

### Phase 9 Gate

- [ ] [AI] Linter reports zero `specs/apps/ose-platform` violations.
- [ ] [AI] `npx nx run ose-web:test:quick`, `npx nx run ose-cli:test:quick`,
      `npx nx run ose-web:spec-coverage`, `npx nx run ose-cli:spec-coverage` exit 0.

> **Pause Safety**: ose-platform specs conform and tests pass. Safe to stop. To resume: re-run the
> ose-platform-scoped linter + the two `test:quick` targets.

## Phase 10: Retrofit `specs/apps/wahidyankf` (wahidyankf-web)

_Suggested executor: `swe-typescript-dev`_

- [ ] [AI] Run the linter scoped to `specs/apps/wahidyankf`; record offenders.
- [ ] [AI] Normalize offending `.feature` files and update wahidyankf-web step defs in lockstep.
      Step-definition file globs to update in lockstep: - `wahidyankf-web` unit TS steps: `apps/wahidyankf-web/test/unit/steps/*.steps.ts` - `wahidyankf-web` e2e TS steps: `apps/wahidyankf-web-fe-e2e/steps/*.steps.ts`
      To discover which specific step file binds to an offending scenario line, run:
      `grep -rln "<step phrase>" apps/wahidyankf-web/test/ apps/wahidyankf-web-fe-e2e/steps/`
      — acceptance: linter reports zero violations for `specs/apps/wahidyankf`.

### Phase 10 Gate

- [ ] [AI] Linter reports zero `specs/apps/wahidyankf` violations.
- [ ] [AI] `npx nx run wahidyankf-web:test:quick` and `npx nx run wahidyankf-web:spec-coverage` exit 0.

> **Pause Safety**: wahidyankf specs conform and tests pass. Safe to stop. To resume: re-run the
> wahidyankf-scoped linter + `npx nx run wahidyankf-web:test:quick`.

## Phase 11: Retrofit `specs/apps/ose-app` (ose-app-be + ose-app-web)

_Suggested executor: `swe-rust-dev` (be) / `swe-typescript-dev` (web)_

- [ ] [AI] Run the linter scoped to `specs/apps/ose-app`; record offenders.
- [ ] [AI] Normalize offending `.feature` files and update step defs in lockstep.
      Step-definition file globs to update in lockstep: - `ose-app-be` (Rust): `apps/ose-app-be/tests/unit/main.rs` and
      `apps/ose-app-be/tests/integration/main.rs` - `ose-app-web` e2e TS steps: `apps/ose-app-web-e2e/steps/*.steps.ts`
      To discover which specific step file binds to an offending scenario line, run:
      `grep -rln "<step phrase>" apps/ose-app-be/tests/ apps/ose-app-web-e2e/steps/`
      — acceptance: linter reports zero violations for `specs/apps/ose-app`.

### Phase 11 Gate

- [ ] [AI] Linter reports zero `specs/apps/ose-app` violations.
- [ ] [AI] `npx nx run ose-app-be:test:quick`, `npx nx run ose-app-web:test:quick`,
      `npx nx run ose-app-be:spec-coverage`, `npx nx run ose-app-web:spec-coverage` exit 0.

> **Pause Safety**: ose-app specs conform and tests pass. Safe to stop. To resume: re-run the
> ose-app-scoped linter + the two `test:quick` targets.

## Phase 12: Retrofit Go-lib specs (`specs/libs/golang-commons`, `specs/libs/golang-link-commons`)

_Suggested executor: `swe-golang-dev`_

- [ ] [AI] Run the linter scoped to `specs/libs/golang-commons` and
      `specs/libs/golang-link-commons`; record offenders.
- [ ] [AI] Normalize offending `.feature` files and update their step definitions in lockstep.
      These specs belong to archived Go libraries; no live Nx project owns them. The step definitions
      live in the archived Go source. To locate the step file binding an offending scenario line, run:
      `grep -rln "<step phrase>" archived/ayokoding-cli/ archived/ose-cli/ archived/rhino-cli/`
      If a match is found in the archived source, update the step text there. If no Nx project covers
      the archived Go tests, note the absence and proceed — zero linter violations is the acceptance
      criterion regardless.
      — acceptance: linter reports zero violations for both Go-lib spec subtrees.

### Phase 12 Gate

- [ ] [AI] Linter reports zero violations for both Go-lib spec subtrees.
- [ ] [AI] Confirm no live Nx project owns these Go-lib specs by running:
      `grep -rln 'golang-commons\|golang-link-commons' apps/*/project.json libs/*/project.json`
      If the grep returns no matches (expected, as no Nx project currently tracks these archived
      Go-lib specs), skip the project `test:quick` run and note the absence. If a project IS found,
      run `npx nx run <project>:test:quick` — exit 0.

> **Pause Safety**: Go-lib specs conform and tests pass. Safe to stop. To resume: re-run the
> Go-lib-scoped linter + the resolved Go-lib tests.

## Phase 13: Retrofit `specs/libs/web-ui` (web-ui lib, 18 component specs)

_Suggested executor: `swe-typescript-dev`_

- [ ] [AI] Run the linter scoped to `specs/libs/web-ui`; record offenders across the 18 component
      feature files.
- [ ] [AI] Normalize offending `.feature` files and update the web-ui step definitions in lockstep.
      — acceptance: linter reports zero violations for `specs/libs/web-ui`.

### Phase 13 Gate

- [ ] [AI] Linter reports zero `specs/libs/web-ui` violations.
- [ ] [AI] `npx nx run web-ui:test:quick` exits 0.
      (`web-ui:spec-coverage` does NOT exist — confirmed: `grep '"spec-coverage"' libs/web-ui/project.json`
      returns no match; skip that target.)

> **Pause Safety**: web-ui specs conform and tests pass. The entire spec corpus now conforms. Safe to
> stop. To resume: run the full-corpus linter (Phase 14 first check).

## Phase 14: Strict repo-rules-quality-gate (double-zero)

- [ ] [AI] Run the full-corpus linter once to confirm zero offenders repo-wide:
      `./apps/rhino-cli/dist/rhino-cli repo-governance gherkin-keyword-cardinality`
      — acceptance: zero findings across all of `specs/**/*.feature`.
- [ ] [AI] Execute the `repo-rules-quality-gate` workflow at **strict** mode per
      `repo-governance/workflows/repo/repo-rules-quality-gate.md` (pin `RHINO_AUDIT_NOW=<RFC3339>`
      for the run as the workflow Step 0.5 requires).
      — acceptance: the workflow terminates with `pass` status; the deterministic preflight reports
      zero `gherkin-keyword-cardinality` findings.
- [ ] [AI] If the gate reports any finding (deterministic or AI-judgment), fix the root cause and
      re-run until double-zero.
      — acceptance: a clean strict run with zero deterministic and zero confirmed AI-judgment findings.

### Phase 14 Gate

> All checks below must pass before starting Phase 15.

- [ ] [AI] Full-corpus linter reports zero findings.
- [ ] [AI] `repo-rules-quality-gate` (strict) terminates `pass` with double-zero.

> **Pause Safety**: rule authored, propagated, enforced, and validated repo-wide; nothing pushed yet.
> Safe to stop. To resume: re-run the full-corpus linter and the strict gate.

## Phase 15: Local quality gates, commit, push, CI verification

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes
- [ ] [AI] Re-run failing checks to confirm resolution
- [ ] [AI] Verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Do not defer or skip existing issues. Commit preexisting fixes separately with
> appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits
      (suggested split: `docs(governance): add Gherkin keyword-cardinality HARD rule`;
      `feat(rhino-cli): add gherkin-keyword-cardinality audit category`;
      `refactor(specs): normalize <project> scenarios to one-each keyword shape` per project;
      `chore(bindings): re-sync skill + agent bindings`).
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [ ] [AI] Split different domains/concerns into separate commits
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work
- [ ] [AI] Do NOT bundle unrelated changes into a single commit

### Post-Push CI Verification

- [ ] [AI] Push changes to `main` (direct push, Trunk Based Development — no PR):
      `git push origin main`
- [ ] [AI] Check which push-triggered GitHub Actions workflows fired:
      `gh run list --branch main --limit 5 --json name,status,conclusion`
      — `validate-markdown.yml` (push to `main`, no path filter) WILL fire and validates
      mermaid + links + heading-hierarchy across the repo; the affected paths
      (`apps/rhino-cli/`, `repo-governance/`, `.claude/`, `specs/`) do NOT match the path
      filters of `crane-cli-integration.yml` (the only other push-triggered workflow in
      `.github/workflows/`); `pr-quality-gate.yml` fires on PRs only (Trunk Based
      Development — no PR is created); scheduled workflows fire independently.
      Poll each triggered run to completion (every 3 minutes;
      one `gh run view --json status,conclusion` per wakeup; never `gh run watch`).
- [ ] [AI] Verify ALL CI checks pass — no exceptions
- [ ] [AI] If any CI check fails, fix the root cause immediately and push a follow-up commit
- [ ] [AI] Repeat until ALL GitHub Actions pass with zero failures
- [ ] [AI] Do NOT proceed to archival until CI is fully green

### Phase 15 Gate

> All checks below must pass before archival.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0 locally.
- [ ] [AI] Changes pushed to `origin main`; all triggered GitHub Actions are green.

> **Pause Safety**: work is committed and pushed; CI is green. Safe to stop. To resume: re-check CI
> status with `gh run view --json status,conclusion`.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Verify the strict `repo-rules-quality-gate` passed with double-zero (Phase 14)
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/gherkin-step-keyword-cardinality/ plans/done/2026-06-05__gherkin-step-keyword-cardinality/`
      (use the actual completion date at execution if later than 2026-06-05)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update `plans/README.md` if it references this plan
- [ ] [AI] Commit the archival: `chore(plans): move gherkin-step-keyword-cardinality to done`
- [ ] [AI] Push the archival commit to `origin main` and confirm CI is green.
