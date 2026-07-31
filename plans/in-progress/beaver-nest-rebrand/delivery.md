# Delivery — BeaverNest Rebrand

Executable checklist for the [BeaverNest Rebrand](./README.md) plan. Read
[tech-docs.md](./tech-docs.md) first — its **Canonical Substitution Vocabulary** and **Decision Log**
sections are referenced by every phase below via the shorthand `<CANONICAL-SED>` (the perl script in
[tech-docs.md §Canonical Substitution Vocabulary](./tech-docs.md#canonical-substitution-vocabulary)).

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Delivery Mode: `main-to-origin-main`

Work happens in the **primary checkout** on branch `main`. Each phase commits and pushes directly to
`origin main`. No PR opens, so the
[PRs Open at Delivery Boundaries](../../../repo-governance/conventions/structure/plans.md#prs-open-at-delivery-boundaries-not-every-phase-hard-rule)
rule and its `### Delivery Boundaries` table do not bind this plan — a per-phase commit-and-push
checkpoint cadence is the correct and explicitly sanctioned form under this mode (Decision 7). The
PR-Review Maker→Fixer Cycle does not run.

## Worktree

**None.** `main-to-origin-main` works in the primary checkout. Do not run `git worktree add` for
this plan. `worktrees/` remains empty of this plan's name throughout.

## Parallelization Model

**Chosen N: 1** — the main thread does the work, no background fan-out.

This plan is a **serial spine with no independent nodes**: every phase renames a directory or file
that a later phase's content sweep references by path (e.g. Phase 6 renames `specs/apps/baseerah/`
to `specs/apps/beaver-nest/` before Phase 8 rewrites `apps/baseerah-be`'s `codegen` target, which
points at that spec path), and `main-to-origin-main` gives every phase the same single write target
(`origin main` from one checkout), so two agents pushing concurrently would be a write conflict by
construction regardless of file-level independence.

### Push Checkpoints

| Phase | Produces                                                              | Pushes to `origin main`              |
| ----- | --------------------------------------------------------------------- | ------------------------------------ |
| 0     | baseline evidence only                                                | no — evidence rides Phase 1's commit |
| 1     | root identity files + vision doc                                      | yes                                  |
| 2     | `repo-governance/` sweep                                              | yes                                  |
| 3     | `docs/` sweep                                                         | yes                                  |
| 4     | `plans/backlog/`, `plans/ideas/`, `plans/in-progress/README.md` sweep | yes                                  |
| 5     | `repo-config.yml` + `.gitignore`                                      | yes                                  |
| 6     | `specs/apps/beaver-nest/` + `beaver-nest-contracts`                   | yes                                  |
| 7     | `libs/web-ui-token` brand palette file                                | yes                                  |
| 8     | `beaver-nest-be`                                                      | yes                                  |
| 9     | `beaver-nest-be-e2e`                                                  | yes                                  |
| 10    | `beaver-nest-fe` + brand-chip removal                                 | yes                                  |
| 11    | `beaver-nest-fe-e2e`                                                  | yes                                  |
| 12    | `infra/dev/beaver-nest-app/` + root `package.json` scripts            | yes                                  |
| 13    | `.github/workflows/` + GHCR cutover                                   | yes                                  |
| 14    | agent fleet + skills + `.amazonq` binding                             | yes                                  |
| 15    | `rhino-cli` functional couplings                                      | yes                                  |
| 16    | residual sweep + full quality gates + manual verification             | yes                                  |
| 17    | GitHub repo rename (`[HUMAN]`)                                        | n/a — no content commit              |
| 18    | local folder rename + remote re-point (`[HUMAN]`)                     | n/a — no content commit              |
| 19    | knowledge capture                                                     | yes                                  |

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_
>
> **No PR for this phase.** Phase 0 is local setup and baseline only: it opens no PR, pushes no
> branch, runs no PR-Review Maker→Fixer Cycle, and merges nothing. The earliest phase that commits
> content is Phase 1; this phase's evidence file rides Phase 1's commit.

- [ ] [AI] Confirm the working tree is clean: run `git status --porcelain` from the repo root —
      acceptance: no output. If output exists, stop and surface it; do not stash or discard.
- [ ] [AI] Confirm the checkout is on `main` and level with the remote: run
      `git rev-parse --abbrev-ref HEAD && git fetch origin && git status -sb` — acceptance: branch
      is `main`, status line shows no `ahead`/`behind` counts.
- [ ] [AI] Record the pre-plan commit SHA into
      `plans/in-progress/beaver-nest-rebrand/evidence/phase-0-baseline.txt`: run `git rev-parse HEAD`
      and write it under a `## Pre-plan HEAD` heading — acceptance: file exists, contains a
      40-character SHA.
- [ ] [AI] Install dependencies: `npm install` — acceptance: exits 0.
- [ ] [AI] Converge the polyglot toolchain: `npm run doctor -- --fix` — acceptance: exits 0, no
      missing tools reported.
- [ ] [AI] Record the residual-reference baseline: run
      `git grep -liE "baseerah" -- . ':!plans/done' ':!generated-reports' | wc -l` and append the
      count to `evidence/phase-0-baseline.txt` under a `## Baseline residual count` heading —
      acceptance: the file records `246` (or the current count if the repo has changed since
      authoring; any material deviation from 246 is surfaced to the maintainer before proceeding).
- [ ] [AI] Record the baseline quality state: run
      `npx nx run-many -t typecheck,lint,test:quick --all --parallel=$(( $(sysctl -n hw.ncpu) - 1 ))`
      and append the summary line to `evidence/phase-0-baseline.txt` under a `## Baseline
test:quick` heading — acceptance: the summary line is recorded verbatim, whether it passed
      or failed.
- [ ] [AI] If the baseline run reported failures, fix each preexisting failure now, per the
      [Root Cause Orientation principle](../../../repo-governance/principles/general/root-cause-orientation.md)
      — acceptance: a re-run of the same command exits 0. Record any fix in `learnings.md`.
- [ ] [AI] Confirm no GitHub Environments are configured (Decision 11's premise): run
      `gh api repos/wahidyankf/baseerah/environments --jq '.environments[].name'` — acceptance:
      empty output. If non-empty, stop and surface the environment names to the maintainer before
      Phase 13 (they would need explicit renaming via `gh api`, not just text substitution).
- [ ] [AI] Confirm no `stag-*`/`prod-*` branches exist (Decision 11's premise): run
      `git branch -r` — acceptance: only `origin/main` is listed.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `git status --porcelain` — output contains only the new `evidence/` file and this plan's
      own folder (already tracked as part of this plan's authoring).
- [ ] [AI] `evidence/phase-0-baseline.txt` records the pre-plan SHA, the residual count, and the
      baseline test:quick summary.
- [ ] [AI] `npx nx run rhino-cli:test:quick` exits 0 (independent green check before any rename
      touches `rhino-cli`).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no rename has
> started. Safe to stop indefinitely. To resume: re-run the baseline commands and confirm they are
> still clean.

---

## Phase 1: Root Identity Files and Vision Doc

- [ ] [AI] Rename the vision doc: `git mv repo-governance/vision/baseerah.md
repo-governance/vision/beaver-nest.md` — acceptance: `test -f
repo-governance/vision/beaver-nest.md` succeeds and the old path no longer exists.
- [ ] [AI] Apply `<CANONICAL-SED>` to `repo-governance/vision/beaver-nest.md`, then hand-edit its
      title/body to state plainly (per Decision 9/Q7) that "BeaverNest" is a chosen product name
      with no etymological parallel to `بصيرة` — remove the "means insight, inner vision" sentence
      rather than translating it — acceptance: `grep -c "بصيرة" repo-governance/vision/beaver-nest.md`
      returns `0`.
- [ ] [AI] Apply `<CANONICAL-SED>` to `repo-governance/vision/README.md` (updates its cross-link to
      the renamed file) — acceptance: `grep -c "baseerah" repo-governance/vision/README.md` returns
      `0`.
- [ ] [AI] Apply `<CANONICAL-SED>` to `README.md`, `CONTRIBUTING.md`, `ROADMAP.md`, `AGENTS.md` —
      acceptance: `git grep -lic baseerah README.md CONTRIBUTING.md ROADMAP.md AGENTS.md` returns no
      matches (command exits with no output listed, since `-l` with `-c` lists only matching files).
- [ ] [AI] Edit `package.json`: change `"name": "baseerah"` to `"name": "beaver-nest"` (line ~2) and
      rename the `baseerah:dev`/`baseerah:dev:restart` scripts to `beaver-nest:dev`/
      `beaver-nest:dev:restart`, updating their `infra/dev/baseerah-app/` path references to
      `infra/dev/beaver-nest-app/` (the directory itself is renamed in Phase 12; record the new path
      now so Phase 12's `git mv` matches) — acceptance: `grep -c "baseerah" package.json` returns `0`.
- [ ] [AI] Run `npm install` to regenerate `package-lock.json`'s root `name` field consistently —
      acceptance: `grep -c '"name": "beaver-nest"' package-lock.json` returns at least `1`.
- [ ] [AI] Edit `.gitignore` line 159 (`specs/apps/baseerah/containers/contracts/generated/`) to
      `specs/apps/beaver-nest/containers/contracts/generated/` — acceptance: `grep -c "baseerah"
.gitignore` returns `0`.
- [ ] [AI] Apply `<CANONICAL-SED>` to `SECURITY.md` and `LICENSING-NOTICE.md` — acceptance:
      `git grep -lic baseerah SECURITY.md LICENSING-NOTICE.md` returns no matches.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures
      (including preexisting) before proceeding.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename root identity files and vision doc to BeaverNest`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main.
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by this push; verify ALL checks pass; fix
      and re-push if any fail.

### Phase 1 Gate

- [ ] [AI] `git grep -lic baseerah README.md CONTRIBUTING.md ROADMAP.md AGENTS.md package.json
package-lock.json .gitignore SECURITY.md LICENSING-NOTICE.md repo-governance/vision/` returns
      no matches.
- [ ] [AI] `test -f repo-governance/vision/beaver-nest.md` succeeds; `test -f
repo-governance/vision/baseerah.md` fails.

> **Pause Safety**: root identity is fully renamed and pushed; every later phase can proceed
> independently of this one. Safe to stop. To resume: `git status -sb` shows level with
> `origin/main`, then start Phase 2.

---

## Phase 2: `repo-governance/` Sweep

- [ ] [AI] Apply `<CANONICAL-SED>` to every git-tracked file under `repo-governance/` except
      `repo-governance/vision/beaver-nest.md` (already done in Phase 1): run
      `git ls-files -z repo-governance/ | grep -zv 'vision/beaver-nest.md' | xargs -0 perl -pi -e
'<CANONICAL-SED-BODY>'` — acceptance: `git grep -lic baseerah repo-governance/ | wc -l`
      returns `0`.
- [ ] [AI] Spot-check `repo-governance/development/agents/ai-agents.md` and
      `repo-governance/conventions/structure/agent-naming.md` for illustrative agent-name examples
      that used `apps-baseerah-fe-*` — confirm they now read `apps-beaver-nest-fe-*` and still parse
      as valid examples (no broken sentence structure from the mechanical swap) — acceptance: manual
      read confirms coherent prose.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah references across repo-governance/`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 2 Gate

- [ ] [AI] `git grep -lic baseerah repo-governance/` returns no matches.
- [ ] [AI] `npx nx run rhino-cli:test:quick` still exits 0 (governance-doc changes never touch code,
      but this confirms no accidental cross-contamination).

> **Pause Safety**: `repo-governance/` is fully clean of `baseerah` residue and pushed. Safe to stop.
> To resume: confirm level with `origin/main`, then start Phase 3.

---

## Phase 3: `docs/` Sweep

- [ ] [AI] Apply `<CANONICAL-SED>` to every git-tracked file under `docs/`: run
      `git ls-files -z docs/ | xargs -0 perl -pi -e '<CANONICAL-SED-BODY>'` — acceptance:
      `git grep -lic baseerah docs/` returns no matches.
- [ ] [AI] Spot-check `docs/reference/system-architecture/applications.md` and
      `docs/reference/monorepo-structure.md` for any diagram or table listing app names — confirm
      renamed entries read `beaver-nest-be`/`beaver-nest-fe` — acceptance: manual read confirms.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah references across docs/`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 3 Gate

- [ ] [AI] `git grep -lic baseerah docs/` returns no matches.

> **Pause Safety**: `docs/` is fully clean and pushed. Safe to stop. To resume: confirm level with
> `origin/main`, then start Phase 4.

---

## Phase 4: `plans/backlog/`, `plans/ideas/`, and `plans/in-progress/README.md` Sweep

- [ ] [AI] Rename the three idea briefs: `git mv plans/ideas/baseerah-first-deploy.md
plans/ideas/beaver-nest-first-deploy.md && git mv plans/ideas/baseerah-first-llm-integration.md
plans/ideas/beaver-nest-first-llm-integration.md && git mv
plans/ideas/baseerah-persistence-layer.md plans/ideas/beaver-nest-persistence-layer.md` —
      acceptance: all three new paths exist, old paths do not.
- [ ] [AI] Apply `<CANONICAL-SED>` to every git-tracked file under `plans/backlog/` and
      `plans/ideas/`, plus `plans/in-progress/README.md`: run `git ls-files -z plans/backlog/
plans/ideas/ plans/in-progress/README.md | xargs -0 perl -pi -e '<CANONICAL-SED-BODY>'` —
      acceptance: `git grep -lic baseerah plans/backlog/ plans/ideas/ plans/in-progress/README.md`
      returns no matches.
- [ ] [AI] Update `plans/ideas/README.md`'s three bullet links to point at the renamed filenames —
      acceptance: `grep -c "beaver-nest-first-deploy.md\]" plans/ideas/README.md` returns at least
      `1` and the equivalent checks pass for the other two renamed files.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah references across active plans/`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 4 Gate

- [ ] [AI] `git grep -lic baseerah plans/backlog/ plans/ideas/ plans/in-progress/` returns no
      matches.
- [ ] [AI] `test -f plans/ideas/beaver-nest-first-deploy.md && test -f
plans/ideas/beaver-nest-first-llm-integration.md && test -f
plans/ideas/beaver-nest-persistence-layer.md` all succeed.

> **Pause Safety**: active plan content is fully renamed and pushed; `plans/done/` remains untouched
> per the historical-citation exemption (Decision 6). Safe to stop. To resume: confirm level with
> `origin/main`, then start Phase 5.

---

## Phase 5: `repo-config.yml` and Registry Consistency

- [ ] [AI] Edit `repo-config.yml`: rename the `coverage.projects` entries `baseerah-be`,
      `baseerah-be-e2e`, `baseerah-fe`, `baseerah-fe-e2e` to their `beaver-nest-*` equivalents, and
      update each entry's `specs:` glob from `specs/apps/baseerah/behavior/...` to
      `specs/apps/beaver-nest/behavior/...` — acceptance: `grep -c "baseerah" repo-config.yml`
      returns `0` after this step and the two below.
- [ ] [AI] Edit `repo-config.yml`'s `env-contract.surfaces` entries: rename `root: apps/baseerah-be`
      to `root: apps/beaver-nest-be` and its `BASEERAH_BE_CORS_ORIGINS` allowlist entry to
      `BEAVER_NEST_BE_CORS_ORIGINS`; rename `root: apps/baseerah-fe` to `root: apps/beaver-nest-fe`.
- [ ] [AI] Edit `repo-config.yml`'s `env-injection.apps` entries: rename `app: baseerah-be` to
      `app: beaver-nest-be` and `app: baseerah-fe` to `app: beaver-nest-fe`; rename the
      `ci-harness.environments` values `baseerah-app-staging` to `beaver-nest-app-staging` in both
      `API_BASE_URL`/`WEB_BASE_URL`/`VERCEL_AUTOMATION_BYPASS_SECRET` entries.
- [ ] [AI] Verify the YAML still parses: run `cargo run --release --quiet --manifest-path
apps/rhino-cli/Cargo.toml -- specs structure validate` — acceptance: exits 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah entries in repo-config.yml`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 5 Gate

- [ ] [AI] `grep -c "baseerah" repo-config.yml` returns `0`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- specs structure
validate` exits 0.

> **Pause Safety**: `repo-config.yml` is fully renamed and validated; downstream phases (6, 8-11)
> depend on this being correct first. Safe to stop. To resume: confirm level with `origin/main`,
> then start Phase 6.

---

## Phase 6: `specs/apps/beaver-nest/` and `beaver-nest-contracts`

- [ ] [AI] Rename the top-level spec directory: `git mv specs/apps/baseerah specs/apps/beaver-nest`
      — acceptance: `test -d specs/apps/beaver-nest` succeeds, `test -d specs/apps/baseerah` fails.
- [ ] [AI] Rename the two behavior subdirectories: `git mv
specs/apps/beaver-nest/behavior/baseerah-be specs/apps/beaver-nest/behavior/beaver-nest-be &&
git mv specs/apps/beaver-nest/behavior/baseerah-fe
specs/apps/beaver-nest/behavior/beaver-nest-fe` — acceptance: both new paths exist.
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `specs/apps/beaver-nest/` — acceptance:
      `git grep -lic baseerah specs/apps/beaver-nest/` returns no matches.
- [ ] [AI] RED (first half of a cycle spanning Phases 6/10/11 — GREEN lands in Phase 10, REFACTOR in
      Phase 11): in
      `specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature`, the
      `<CANONICAL-SED>` pass already updated "Baseerah" → "BeaverNest" text; now DELETE the entire
      "The multilingual brand chip is understandable to a non-Arabic, non-Indonesian reader"
      scenario (per [tech-docs.md Decision 3](./tech-docs.md#decision-log)).
      **Gherkin (binds) →** "The homepage no longer renders a brand-chip etymology gloss"

      ```gherkin
      Scenario: The homepage no longer renders a brand-chip etymology gloss
        Given a first-time visitor viewing the rendered homepage
        When they inspect the page for a hoverable multilingual term chip
        Then no برصيرة/wawasan-style etymology chip is present
        And no automated test or Gherkin scenario asserts one exists
      ```

      Acceptance: run `npx nx run beaver-nest-fe:specs:behavior:coverage` — rhino-cli's coverage
      validator fails with `ERROR: Found N orphan step implementation(s) (no Gherkin step matches
      them)` (`[Repo-grounded]`: confirmed against `apps/rhino-cli/src/commands/specs_coverage.rs`'s
      `orphan_step_impls` check) because the step files still implement the deleted scenario's steps
      — a deliberate, expected RED state resolved by Phase 10's GREEN and Phase 11's REFACTOR steps.

- [ ] [AI] Edit `specs/apps/beaver-nest/containers/contracts/project.json`: rename `"name":
"baseerah-contracts"` to `"name": "beaver-nest-contracts"` and its `tags` entry
      `"domain:baseerah"` to `"domain:beaver-nest"` — acceptance: `grep -c "baseerah"
specs/apps/beaver-nest/containers/contracts/project.json` returns `0`.
- [ ] [AI] Confirm `specs structure validate` still passes: run `cargo run --release --quiet
--manifest-path apps/rhino-cli/Cargo.toml -- specs structure validate` — acceptance: exits 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures. (`specs:coverage` is
      expected RED per the step above; do not fix it here — Phase 10/11 resolve it.)

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename specs/apps/baseerah to specs/apps/beaver-nest`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI (the `beaver-nest-fe`/`beaver-nest-be`
      `specs:coverage` targets are expected to still fail here — record that in the push notes, it
      resolves by Phase 11's gate).

### Phase 6 Gate

- [ ] [AI] `git grep -lic baseerah specs/apps/beaver-nest/` returns no matches.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- specs structure
validate` exits 0.

> **Pause Safety**: the spec tree is fully renamed; `specs:coverage` for the FE project is
> deliberately RED and will stay that way until Phase 11. Safe to stop with this known RED state
> recorded here. To resume: confirm level with `origin/main`, then start Phase 7.

---

## Phase 7: `libs/web-ui-token` Brand Palette File

- [ ] [AI] Rename: `git mv libs/web-ui-token/src/baseerah.css libs/web-ui-token/src/beaver-nest.css`
      — acceptance: `test -f libs/web-ui-token/src/beaver-nest.css` succeeds.
- [ ] [AI] Edit `libs/web-ui-token/src/beaver-nest.css`: reword the two brand-meaning comments
      (lines 3-4 and 43) to plainly describe this as "BeaverNest's palette" with no برصيرة reference
      (per Decision 8/Q7) — leave every `oklch(...)` numeric value byte-identical — acceptance:
      `diff <(grep -o 'oklch([^)]*)' libs/web-ui-token/src/beaver-nest.css | sort) <(git show
HEAD:libs/web-ui-token/src/baseerah.css | grep -o 'oklch([^)]*)' | sort)` reports no
      differences.
- [ ] [AI] Apply `<CANONICAL-SED>` to `libs/web-ui-token/README.md` and `libs/README.md` — for
      `libs/README.md`, preserve the literal `baseerah-repo-reset` citation in its link to
      `../plans/done/2026-07-31__baseerah-repo-reset/README.md` per Decision 6 (do not let the
      catch-all rule touch that one link target) — acceptance: `grep -c "baseerah" libs/README.md`
      returns exactly `1` (the preserved citation) and `grep -c "baseerah"
libs/web-ui-token/README.md` returns `0`.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename brand palette file to beaver-nest.css`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 7 Gate

- [ ] [AI] `test -f libs/web-ui-token/src/beaver-nest.css` succeeds; the OKLCH-value diff check
      above reports zero differences.
- [ ] [AI] `grep -c "baseerah" libs/README.md` returns exactly `1` (the historical citation).

> **Pause Safety**: the brand palette file is renamed with byte-identical color values. Safe to
> stop. To resume: confirm level with `origin/main`, then start Phase 8.

---

## Phase 8: `beaver-nest-be` (F#)

- [ ] [AI] Rename the app directory: `git mv apps/baseerah-be apps/beaver-nest-be` — acceptance:
      `test -d apps/beaver-nest-be` succeeds.
- [ ] [AI] Rename the F# source directory: `git mv apps/beaver-nest-be/src/BaseerahBe
apps/beaver-nest-be/src/BeaverNestBe` — acceptance: `test -d
apps/beaver-nest-be/src/BeaverNestBe` succeeds.
- [ ] [AI] Rename project files: `git mv
apps/beaver-nest-be/src/BeaverNestBe/BaseerahBe.fsproj
apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj && git mv
apps/beaver-nest-be/tests/unit/BaseerahBe.UnitTests.fsproj
apps/beaver-nest-be/tests/unit/BeaverNestBe.UnitTests.fsproj && git mv
apps/beaver-nest-be/tests/integration/BaseerahBe.IntegrationTests.fsproj
apps/beaver-nest-be/tests/integration/BeaverNestBe.IntegrationTests.fsproj` — acceptance: all
      three new paths exist.
- [ ] [AI] Rename the solution file: `git mv baseerah.sln beaver-nest.sln` — acceptance: `test -f
beaver-nest.sln` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `apps/beaver-nest-be/` and to
      `beaver-nest.sln` — this rewrites `BaseerahBe` → `BeaverNestBe` in every `.fs`/`.fsproj` file
      (namespace declarations, `open` statements, project references), `BASEERAH_BE_PORT`/
      `BASEERAH_BE_CORS_ORIGINS` → `BEAVER_NEST_BE_PORT`/`BEAVER_NEST_BE_CORS_ORIGINS` in
      `Program.fs`, `.env.example`, `Dockerfile`, and `README.md`, and the `project.json` `"name"`
      and `"domain:baseerah"` tag — acceptance: `git grep -lic baseerah apps/beaver-nest-be/
beaver-nest.sln` returns no matches.
- [ ] [AI] Edit `apps/beaver-nest-be/project.json`'s `codegen` target: confirm the
      `openapi-generator-cli` invocation now reads
      `specs/apps/beaver-nest/containers/contracts/generated/openapi-bundled.yaml` and
      `--model-package BeaverNestBe.Contracts` (both should already be correct from the sed pass;
      this step is a manual verification, not a fix) — acceptance: manual read confirms both.
- [ ] [AI] Regenerate the OpenAPI contracts under the new package name (this is a rename-refactor
      verification, not a TDD cycle — no new behavior is introduced): run `npx nx run
beaver-nest-be:codegen` — acceptance: exits 0 and
      `apps/beaver-nest-be/generated-contracts/OpenAPI/src/BeaverNestBe.Contracts/` exists
      (gitignored, not committed).
- [ ] [AI] Verify the existing unit test suite stays green under the renamed namespace: run `npx nx
run beaver-nest-be:test:unit` — acceptance: exits 0, all prior `BaseerahBe` test assertions now
      pass under `BeaverNestBe`.
- [ ] [AI] Run `npx nx run beaver-nest-be:test:integration` — acceptance: exits 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah-be to beaver-nest-be`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 8 Gate

- [ ] [AI] `git grep -lic baseerah apps/beaver-nest-be/ beaver-nest.sln` returns no matches.
- [ ] [AI] `npx nx run beaver-nest-be:test:quick` exits 0.

> **Pause Safety**: `beaver-nest-be` builds, tests, and boots under its new name. Safe to stop. To
> resume: confirm level with `origin/main`, then start Phase 9.

---

## Phase 9: `beaver-nest-be-e2e`

- [ ] [AI] Rename the app directory: `git mv apps/baseerah-be-e2e apps/beaver-nest-be-e2e` —
      acceptance: `test -d apps/beaver-nest-be-e2e` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `apps/beaver-nest-be-e2e/` — acceptance:
      `git grep -lic baseerah apps/beaver-nest-be-e2e/` returns no matches.
- [ ] [AI] Run `npx nx run beaver-nest-be-e2e:typecheck` (or the project's equivalent lint/typecheck
      target) — acceptance: exits 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah-be-e2e to beaver-nest-be-e2e`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 9 Gate

- [ ] [AI] `git grep -lic baseerah apps/beaver-nest-be-e2e/` returns no matches.

> **Pause Safety**: `beaver-nest-be-e2e` is fully renamed. Safe to stop. To resume: confirm level
> with `origin/main`, then start Phase 10.

---

## Phase 10: `beaver-nest-fe` (Next.js) and Brand-Chip Removal

- [ ] [AI] Rename the app directory: `git mv apps/baseerah-fe apps/beaver-nest-fe` — acceptance:
      `test -d apps/beaver-nest-fe` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `apps/beaver-nest-fe/` — acceptance:
      `git grep -lic baseerah apps/beaver-nest-fe/` returns no matches after the remaining steps in
      this phase also complete.
- [ ] [AI] Update the brand copy (a rename-refactor, not a TDD cycle — this substitutes text, it
      does not introduce new behavior): edit `apps/beaver-nest-fe/src/components/AppShell.tsx`,
      confirming the brand-name heading and the one-line description now read "BeaverNest" (from the
      sed pass), then hand-edit the description sentence so it reads as a plain description with no
      Arabic/Indonesian etymology gloss (per Decision 3) — acceptance: `grep -c "wawasan\|بصيرة"
apps/beaver-nest-fe/src/components/AppShell.tsx` returns `0`.
- [ ] [AI] GREEN (second half of the cycle whose RED landed in Phase 6; REFACTOR continues below and
      in Phase 11): delete the multilingual brand-chip JSX block from
      `apps/beaver-nest-fe/src/components/AppShell.tsx` (the `title="insight (English) · wawasan
(Indonesian) · بصيرة (Arabic)"` element and its containing wrapper).
      **Gherkin (binds) →** "The homepage no longer renders a brand-chip etymology gloss" (same
      scenario Phase 6's RED step embedded) — acceptance: `grep -c 'title="insight'
apps/beaver-nest-fe/src/components/AppShell.tsx` returns `0`.
- [ ] [AI] REFACTOR: edit `apps/beaver-nest-fe/src/app/page.test.tsx`: delete the chip-specific
      assertions (the `screen.getByText("بصيرة")` lines and the `toHaveAttribute("title", ...)`
      assertion), keeping the heading/greeting/description assertions (updated to "BeaverNest" by
      the sed pass) — acceptance: `npx nx run beaver-nest-fe:test:unit` exits 0.
- [ ] [AI] REFACTOR: edit `apps/beaver-nest-fe/src/test/landing.steps.ts`: delete the
      `When('they read or hover the "بصيرة" and "wawasan" terms', ...)` step definition (now
      orphaned since Phase 6 deleted its scenario) — acceptance: `npx nx run
beaver-nest-fe:specs:behavior:coverage` exits 0 (this resolves Phase 6's deliberate RED for the
      FE project; Phase 11 resolves the same RED for the FE-E2E project).
- [ ] [AI] Run `npx nx run beaver-nest-fe:codegen` then `npx nx run beaver-nest-fe:test:unit` —
      acceptance: both exit 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures.

### Specs & Gherkin Delivery

- [ ] [AI] Confirm `specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature`
      (edited in Phase 6) has no remaining reference to the deleted chip scenario and that its other
      four scenarios read "BeaverNest" — acceptance: `grep -c "BeaverNest" specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature` returns at least `4`.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah-fe to beaver-nest-fe, remove brand-chip feature`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 10 Gate

- [ ] [AI] `git grep -lic baseerah apps/beaver-nest-fe/` returns no matches.
- [ ] [AI] `npx nx run beaver-nest-fe:test:quick` exits 0.

> **Pause Safety**: `beaver-nest-fe` is fully renamed, the brand-chip feature is removed end to end
> (component, unit test, Gherkin scenario, one step definition), and the FE `specs:coverage` RED
> from Phase 6 is now resolved. Safe to stop. To resume: confirm level with `origin/main`, then start
> Phase 11.

---

## Phase 11: `beaver-nest-fe-e2e`

- [ ] [AI] Rename the app directory: `git mv apps/baseerah-fe-e2e apps/beaver-nest-fe-e2e` —
      acceptance: `test -d apps/beaver-nest-fe-e2e` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `apps/beaver-nest-fe-e2e/` — acceptance:
      `git grep -lic baseerah apps/beaver-nest-fe-e2e/` returns no matches after the step below.
- [ ] [AI] REFACTOR: edit `apps/beaver-nest-fe-e2e/steps/landing.steps.ts`: delete the
      `When('they read or hover the "بصيرة" and "wawasan" terms', ...)` step implementation and its
      two `@covers` comment lines referencing the deleted scenario (per Decision 3, mirroring
      Phase 10's FE-side removal) — acceptance: `grep -c "بصيرة"
apps/beaver-nest-fe-e2e/steps/landing.steps.ts` returns `0`.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah-fe-e2e to beaver-nest-fe-e2e`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 11 Gate

- [ ] [AI] `git grep -lic baseerah apps/beaver-nest-fe-e2e/` returns no matches.

> **Pause Safety**: every application is now fully renamed with the brand-chip feature removed on
> both the FE unit and E2E sides. Safe to stop. To resume: confirm level with `origin/main`, then
> start Phase 12.

---

## Phase 12: `infra/dev/beaver-nest-app/` and Root npm Scripts

- [ ] [AI] Rename the infra directory: `git mv infra/dev/baseerah-app infra/dev/beaver-nest-app` —
      acceptance: `test -d infra/dev/beaver-nest-app` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to `infra/dev/beaver-nest-app/docker-compose.yml`,
      `infra/dev/beaver-nest-app/docker-compose.ci.yml`, `infra/dev/beaver-nest-app/README.md`
      (renames the `baseerah-be`/`baseerah-fe` service names, the `BASEERAH_*` env vars, and the
      `Dockerfile.be.dev` build context comment) — acceptance: `git grep -lic baseerah
infra/dev/beaver-nest-app/` returns no matches.
- [ ] [AI] Confirm `package.json`'s `beaver-nest:dev`/`beaver-nest:dev:restart` scripts (renamed in
      Phase 1) now point at `infra/dev/beaver-nest-app/docker-compose.yml` — acceptance:
      `grep -c "infra/dev/beaver-nest-app" package.json` returns `2`.
- [ ] [AI] Run `npm run beaver-nest:dev:restart` locally (or `docker compose -f
infra/dev/beaver-nest-app/docker-compose.yml config` as a non-destructive syntax check if a
      full stack boot is impractical in this environment) — acceptance: the compose config parses
      with no error and references only `beaver-nest-be`/`beaver-nest-fe` service names.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename infra/dev/baseerah-app to infra/dev/beaver-nest-app`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 12 Gate

- [ ] [AI] `git grep -lic baseerah infra/dev/beaver-nest-app/` returns no matches.
- [ ] [AI] `docker compose -f infra/dev/beaver-nest-app/docker-compose.yml config` exits 0.

> **Pause Safety**: the local dev stack is fully renamed and its compose files parse cleanly. Safe
> to stop. To resume: confirm level with `origin/main`, then start Phase 13.

---

## Phase 13: `.github/workflows/` and GHCR Cutover

- [ ] [AI] Rename the three caller workflow files: `git mv
.github/workflows/baseerah-app-test-local-deploy-stag.yml
.github/workflows/beaver-nest-app-test-local-deploy-stag.yml && git mv
.github/workflows/baseerah-app-test-stag.yml
.github/workflows/beaver-nest-app-test-stag.yml && git mv
.github/workflows/baseerah-be-build-deploy-stag.yml
.github/workflows/beaver-nest-be-build-deploy-stag.yml` — acceptance: all three new paths
      exist.
- [ ] [AI] Apply `<CANONICAL-SED>` to the three renamed workflow files, `.github/workflows/README.md`,
      and `.github/workflows/publish-images.yml` — this renames the `build-baseerah-be` output/job
      names to `build-beaver-nest-be`, the `stag-baseerah-fe`/`stag-baseerah-be` branch-name strings
      to `stag-beaver-nest-fe`/`stag-beaver-nest-be`, the `baseerah-app-staging`/`baseerah-app-local`
      environment strings to `beaver-nest-app-staging`/`beaver-nest-app-local`, and (per Q9, hard
      cutover) the GHCR image name `ghcr.io/wahidyankf/baseerah-be` to
      `ghcr.io/wahidyankf/beaver-nest-be` with no dual-publish — acceptance: `git grep -lic baseerah
.github/workflows/` returns no matches.
- [ ] [AI] Validate the renamed workflows: run `actionlint .github/workflows/*.yml` — acceptance:
      exits 0, no new findings introduced by the rename.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename CI workflows and cut over GHCR image to beaver-nest-be`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor ALL GitHub Actions workflows including
      `publish-images` (which will attempt to build and push `ghcr.io/wahidyankf/beaver-nest-be` on
      this push, since `apps/beaver-nest-be` is now affected) — verify the push succeeds; fix and
      re-push on any failure.
- [ ] [AI] Verify the new image landed: run `gh api
/users/wahidyankf/packages/container/beaver-nest-be/versions --jq '.[0].id'` — acceptance:
      returns a numeric version id (confirms at least one image was pushed under the new name).

### Phase 13 Gate

- [ ] [AI] `git grep -lic baseerah .github/workflows/` returns no matches.
- [ ] [AI] `actionlint .github/workflows/*.yml` exits 0.
- [ ] [AI] The GHCR package-version check above returns a numeric id.

> **Pause Safety**: CI workflows and the GHCR image name are fully cut over with no dual-publish
> bridge. Safe to stop. To resume: confirm level with `origin/main`, then start Phase 14.

---

## Phase 14: Agent Fleet, Skills, and the `.amazonq` Binding

- [ ] [AI] Rename the five `.claude/agents/` files: `git mv
.claude/agents/apps-baseerah-fe-content-checker.md
.claude/agents/apps-beaver-nest-fe-content-checker.md && git mv
.claude/agents/apps-baseerah-fe-content-fixer.md
.claude/agents/apps-beaver-nest-fe-content-fixer.md && git mv
.claude/agents/apps-baseerah-fe-content-maker.md
.claude/agents/apps-beaver-nest-fe-content-maker.md && git mv
.claude/agents/apps-baseerah-fe-deployer.md .claude/agents/apps-beaver-nest-fe-deployer.md &&
git mv .claude/agents/apps-baseerah-be-deployer.md
.claude/agents/apps-beaver-nest-be-deployer.md` — acceptance: all five new paths exist.
- [ ] [AI] Apply `<CANONICAL-SED>` to the five renamed files and to `.claude/agents/README.md`'s
      catalog (name, description, and table-row references) — acceptance: `git grep -lic baseerah
.claude/agents/` returns no matches.
- [ ] [AI] Rename the skill directory: `git mv
.claude/skills/apps-baseerah-fe-developing-content
.claude/skills/apps-beaver-nest-fe-developing-content` — acceptance: `test -d
.claude/skills/apps-beaver-nest-fe-developing-content` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to
      `.claude/skills/apps-beaver-nest-fe-developing-content/SKILL.md` (including its `name:`
      frontmatter field) and to
      `.claude/skills/swe-developing-frontend-ui/reference/brand-context.md` — acceptance:
      `git grep -lic baseerah .claude/skills/` returns no matches.
- [ ] [AI] Mirror the same five renames into `.opencode/agents/` and `.cursor/agents/`: `git mv
.opencode/agents/apps-baseerah-fe-content-checker.md
.opencode/agents/apps-beaver-nest-fe-content-checker.md` (repeat for the remaining four in
      each of `.opencode/agents/` and `.cursor/agents/`), then apply `<CANONICAL-SED>` to all ten
      resulting files — acceptance: `git grep -lic baseerah .opencode/agents/ .cursor/agents/`
      returns no matches.
- [ ] [AI] Rename the Amazon Q binding: `git mv .amazonq/cli-agents/baseerah-default.json
.amazonq/cli-agents/beaver-nest-default.json` and edit its `"name"` field to
      `"beaver-nest-default"` — acceptance: `jq -r .name .amazonq/cli-agents/beaver-nest-default.json`
      returns `beaver-nest-default`.
- [ ] [AI] Verify the generator agrees with the manual renames: run `npm run generate:bindings` —
      acceptance: `git status --porcelain` reports no diff beyond what this phase already staged
      (confirms no drift between the hand-renamed mirrors and what `rhino-cli` would regenerate;
      any diff here is a defect to fix before the phase gate, not a file to blindly accept).

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah agent fleet, skill, and amazonq binding`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 14 Gate

- [ ] [AI] `git grep -lic baseerah .claude/ .opencode/ .cursor/ .amazonq/` returns no matches.
- [ ] [AI] `npm run generate:bindings` followed by `git status --porcelain` shows a clean tree (no
      drift).

> **Pause Safety**: the agent fleet, skill, and generated-binding mirrors are fully renamed and
> internally consistent. Safe to stop. To resume: confirm level with `origin/main`, then start
> Phase 15.

---

## Phase 15: `rhino-cli` Functional Couplings

- [ ] [AI] RED: edit `apps/rhino-cli/tests/agents.rs` — update the five assertions referencing
      `.amazonq/cli-agents/baseerah-default.json` and `"baseerah-default"` to
      `.amazonq/cli-agents/beaver-nest-default.json` and `"beaver-nest-default"`.
      **Gherkin (binds) →** "rhino-cli's Amazon Q binding constant points at the renamed file"

      ```gherkin
      Scenario: rhino-cli's Amazon Q binding constant points at the renamed file
        Given apps/rhino-cli's AMAZONQ_AGENT_DEFINITION constant after the rhino-cli rename phase
        When nx run rhino-cli:test:unit runs
        Then the test asserting the constant's path value passes against ".amazonq/cli-agents/beaver-nest-default.json"
        And the generated file's "name" field reads "beaver-nest-default"
      ```

      Acceptance: run `npx nx run rhino-cli:test:integration` (or the project's equivalent test
      target covering `tests/agents.rs`) and confirm it now fails against the still-unrenamed source
      constant (a deliberate, expected RED state).

- [ ] [AI] GREEN: edit `apps/rhino-cli/src/application/agents/bindings.rs` — rename the
      `AMAZONQ_AGENT_DEFINITION` constant's value to `".amazonq/cli-agents/beaver-nest-default.json"`
      and `AGENT_DEFINITION_CONTENT`'s embedded `"name": "baseerah-default"` to `"name":
"beaver-nest-default"`, and update its own three in-file test assertions to match — acceptance:
      `npx nx run rhino-cli:test:integration` now passes.
- [ ] [AI] REFACTOR: apply `<CANONICAL-SED>` to
      `apps/rhino-cli/src/application/domain_coverage/mod.rs`,
      `apps/rhino-cli/src/commands/specs_validate_counts.rs`, and
      `apps/rhino-cli/src/application/repo_governance/frontmatter_audit.rs`'s self-contained test
      fixtures (`"baseerah-be"` → `"beaver-nest-be"`, `"baseerah"` → `"beaver-nest"`,
      `"apps/baseerah-fe/content/post.md"` → `"apps/beaver-nest-fe/content/post.md"`) — acceptance:
      `npx nx run rhino-cli:test:unit` exits 0.
- [ ] [AI] Preserve the historical citation (Decision 6): confirm
      `apps/rhino-cli/src/commands/specs_coverage.rs`'s comment citing "the baseerah-repo-reset plan"
      is left unchanged — acceptance: `grep -c "baseerah-repo-reset"
apps/rhino-cli/src/commands/specs_coverage.rs` returns `1`.
- [ ] [AI] Update the comment in `apps/rhino-cli/src/application/docs/naming.rs` referencing "a
      Baseerah-identity rewrite" to "a BeaverNest-identity rewrite" (prose only, no functional
      change) — acceptance: `grep -c "Baseerah" apps/rhino-cli/src/application/docs/naming.rs`
      returns `0`.
- [ ] [AI] Run the full `rhino-cli` quality gate: `npx nx run rhino-cli:test:quick` — acceptance:
      exits 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah references in rhino-cli source and tests`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 15 Gate

- [ ] [AI] `npx nx run rhino-cli:test:quick` exits 0.
- [ ] [AI] `git grep -lic baseerah apps/rhino-cli/ | grep -v specs_coverage.rs` returns no matches
      (the one preserved historical citation in `specs_coverage.rs` is the sole expected exception).

> **Pause Safety**: `rhino-cli`'s functional couplings to the old name are fully resolved and its own
> test suite is green. Safe to stop. To resume: confirm level with `origin/main`, then start
> Phase 16.

---

## Phase 16: Cross-Cutting Docs, Repo-Wide Residual Sweep, and Verification

- [ ] [AI] Apply `<CANONICAL-SED>` to `apps/README.md` (which references every renamed app by name
      and the deployment-branch prose) and re-read it for coherence, since it was intentionally
      deferred until every app rename landed — acceptance: `grep -c "baseerah" apps/README.md`
      returns `0`.
- [ ] [AI] Run the full repo-wide residual check: `git grep -liE "baseerah" -- . ':!plans/done'
':!generated-reports'` — acceptance: the only remaining matches are `libs/README.md` (the one
      preserved `baseerah-repo-reset` citation, Decision 6) and
      `apps/rhino-cli/src/commands/specs_coverage.rs` (the one preserved historical comment,
      Decision 6). Any other match is a defect — fix it and re-run this check until only those two
      files remain.
- [ ] [AI] Run the full quality gate across every renamed project: `npx nx run-many -t typecheck,
lint,test:quick,specs:coverage --projects=beaver-nest-be,beaver-nest-be-e2e,beaver-nest-fe,
beaver-nest-fe-e2e,beaver-nest-contracts,rhino-cli` — acceptance: exits 0 for every project.
- [ ] [AI] Run `npx nx affected -t typecheck lint test:quick specs:coverage` across the whole
      affected graph (catches any file outside the explicit project list above that still references
      a renamed path) — acceptance: exits 0.

### Manual UI Verification (Playwright MCP) — single-locale app

- [ ] [AI] Start the local stack: `npm run beaver-nest:dev` (docker compose, per Phase 12) —
      acceptance: both `beaver-nest-be` (port 19320) and `beaver-nest-fe` (port 19310) report
      healthy.
- [ ] [AI] Navigate to `http://localhost:19310/` via `browser_navigate`; resize to 375px, 768px,
      1280px via `browser_resize` at each breakpoint — acceptance: page renders at all three.
- [ ] [AI] Inspect via `browser_snapshot` — verify the level-one heading reads "BeaverNest", the
      greeting text reads "Hello from BeaverNest", the one-line description contains no
      Arabic/Indonesian etymology text, and no hoverable brand-chip element is present — acceptance:
      all four conditions hold.
- [ ] [AI] Navigate to a non-existent path (e.g. `/does-not-exist`) — acceptance: the 404 page shows
      "BeaverNest" branding and a link back to `/`.
- [ ] [AI] Check for JS errors via `browser_console_messages` — acceptance: zero errors.
- [ ] [AI] Verify API integration via `browser_network_requests` — acceptance: the greeting fetch
      request targets `beaver-nest-be`'s renamed base URL.
- [ ] [AI] Capture one screenshot per breakpoint via `browser_take_screenshot`, saved to
      `evidence/phase-16-landing-page-{375,768,1280}px.png` — acceptance: three files exist.
- [ ] [AI] Reference each screenshot in this checklist: `![BeaverNest landing page at 375px]
(./evidence/phase-16-landing-page-375px.png)` (repeat for 768px and 1280px).

### Manual API Verification (curl)

- [ ] [AI] Verify the health endpoint: `curl -s http://localhost:19320/api/v1/health | jq .` —
      acceptance: 200 status, response pasted inline below as `>` blockquote lines.
- [ ] [AI] Verify the greeting endpoint: `curl -s http://localhost:19320/api/v1/greeting | jq .` —
      acceptance: 200 status, response body contains "BeaverNest", pasted inline below as `>`
      blockquote lines.

> _(paste curl output for /api/v1/health and /api/v1/greeting here during execution)_

### Rule-15 Three-Tester Retest (before archival)

- [ ] [AI] Run the three live-site testers (`web-exploratory-tester` + `web-usability-tester` +
      `web-design-tester`) against `http://localhost:19310/` — acceptance: EWT/UWT/DWT findings and
      spec-gaps recorded.
- [ ] [AI] Append each finding here as a new unchecked checkbox, source-attributed
      (`- [ ] EWT-NNN:` / `- [ ] UWT-NNN:` / `- [ ] DWT-NNN: <defect> — fix before archival`).
- [ ] [AI] Fix every rule-15 EWT/UWT/DWT defect finding before archival — deferral requires explicit
      user permission (only when genuinely impossible); `SG-###`/`USS-###` proposals may be triaged
      or deferred.

### Rule-16 API Exploratory Retest (before archival)

- [ ] [AI] Run `api-exploratory-tester` (`output-mode: delivery`, this plan's `plan-path`) against
      `http://localhost:19320/`, with `specs/apps/beaver-nest/containers/contracts/openapi.yaml` as
      ground truth — acceptance: AET findings + spec-gaps recorded.
- [ ] [AI] Append each finding here as a new unchecked checkbox (`- [ ] AET-NNN: <defect> — fix
before archival`) and fix every defect finding before archival (deferral requires explicit
      user permission, only when genuinely impossible).

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename apps/README.md, close out residual sweep and evidence`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 16 Gate

- [ ] [AI] `git grep -liE "baseerah" -- . ':!plans/done' ':!generated-reports'` returns exactly the
      two allowlisted files (`libs/README.md`, `apps/rhino-cli/src/commands/specs_coverage.rs`).
- [ ] [AI] Every rule-15 and rule-16 defect finding is fixed (ticked) or explicitly deferred with
      recorded user permission.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick,specs:coverage --all` exits 0.

> **Pause Safety**: the repository is fully rebranded in content, every quality gate is green, and
> manual verification evidence is committed. Only the two `[HUMAN]` external-identity phases and
> Knowledge Capture remain. Safe to stop indefinitely. To resume: confirm level with `origin/main`,
> then start Phase 17.

---

## Phase 17: GitHub Repository Rename (`[HUMAN]`)

> Per Decision 4/Q3: this happens last among the content-adjacent phases, after every prior phase
> has merged to `origin main` under the OLD repository URL (`github.com/wahidyankf/baseerah`).

- [ ] [HUMAN] Rename the GitHub repository from `baseerah` to `beaver-nest`: run
      `gh repo rename beaver-nest --repo wahidyankf/baseerah` (requires the maintainer's own
      authenticated `gh` session — an agent must not hold or exercise this authority per this plan's
      explicit Q3 decision) — acceptance: `gh repo view wahidyankf/beaver-nest --json name --jq
.name` returns `beaver-nest`. **Resume signal**: the maintainer confirms the rename completed
      (GitHub's redirect from `wahidyankf/baseerah` to `wahidyankf/beaver-nest` is automatic and
      verified by `gh repo view wahidyankf/baseerah --json name --jq .name` also returning
      `beaver-nest`, following the redirect).

### Phase 17 Gate

- [ ] [HUMAN] `gh repo view wahidyankf/beaver-nest --json name --jq .name` returns `beaver-nest`.

> **Pause Safety**: the GitHub repository is renamed; the local checkout's `origin` remote still
> points at the old URL, which GitHub's redirect keeps working. Safe to stop. To resume: proceed to
> Phase 18 whenever the maintainer is ready.

---

## Phase 18: Local Checkout Folder Rename and Remote Re-point (`[HUMAN]`)

> Per Decision 5/Q4: this happens after the GitHub rename, once everything else has merged.

- [ ] [HUMAN] Rename the local checkout folder (e.g. `mv /Users/wkf/ose-projects/baseerah
/Users/wkf/ose-projects/beaver-nest`) — acceptance: the new path exists and contains the `.git`
      directory. **Resume signal**: the maintainer confirms the folder move completed and has
      changed their working directory into the new path.
- [ ] [HUMAN] Re-point the `origin` remote to the renamed GitHub URL: `git remote set-url origin
git@github.com:wahidyankf/beaver-nest.git` (or the equivalent HTTPS URL, matching whichever
      protocol the maintainer's existing remote used) — acceptance: `git remote -v` shows `origin`
      pointing at the `beaver-nest` URL.
- [ ] [HUMAN] Verify the re-pointed remote works: `git fetch origin` — acceptance: exits 0 with no
      error.

### Phase 18 Gate

- [ ] [HUMAN] `git remote -v` shows `origin` pointing at `wahidyankf/beaver-nest`; `git fetch origin`
      exits 0.

> **Pause Safety**: the local checkout folder and remote are fully re-pointed; every subsequent git
> operation in this repo now happens under the new name and path. Safe to stop. To resume: proceed
> to Phase 19 from the renamed folder.

---

## Phase 19: Knowledge Capture

> _Triage every surviving `learnings.md` entry before archival. See the
> [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md)._

- [ ] [AI] Apply the litmus test to every `learnings.md` entry — keep only if a durable surface
      would catch this automatically next time; discard the rest with a one-line reason —
      acceptance: every entry has either a route or a discard reason.
- [ ] [AI] Apply the **secret/sensitivity gate** to every surviving entry — sanitize any secret,
      credential, token, or private hostname to a `<placeholder>` token, or discard if
      unsanitizable — acceptance: `learnings.md` contains no raw secret.
- [ ] [AI] Apply the **repo-relevance gate** to every surviving entry — acceptance: no infra-private
      content appears in this repo's routed output (not applicable here — this repo has no
      `ose-private`-style infra content, but the check is still run for completeness).
- [ ] [AI] Route each surviving learning to exactly one durable home per the open-ended routing
      matrix — non-code homes may land inline (small edit) or as a `plans/backlog/` follow-up
      (large); code homes (`apps/`, `libs/`, tests) are ALWAYS filed as a separate
      `plans/backlog/<slug>/` plan and NEVER landed inline — acceptance: every `learnings.md` entry
      records its terminal routing state.
- [ ] [AI] If no generalizable learning surfaced, record the explicit escape in `learnings.md`:
      `No generalizable learnings — <one-line reason>` — acceptance: `learnings.md` is never
      silently empty.

### Phase 19 Gate

- [ ] [AI] Every `learnings.md` entry is in a terminal state (routed inline, filed as backlog, or
      discarded with reason), or the file records the explicit "none" escape.
- [ ] [AI] No code-homed learning landed inline in this plan's own commits.

### Commit Guidelines

- [ ] [AI] Commit: `docs(plans): triage learnings for beaver-nest-rebrand`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

> **Pause Safety**: `learnings.md` is fully triaged; no future process depends on querying it later.
> Safe to stop. To resume: re-read `learnings.md` and confirm every entry is terminal, then proceed
> to Plan Archival.

---

### Plan Archival

- [ ] Verify ALL delivery checklist items above are ticked.
- [ ] Verify the Knowledge Capture phase is complete — every `learnings.md` entry reached a terminal
      state or the file records the explicit `No generalizable learnings — <reason>` escape.
- [ ] Verify ALL quality gates pass (local + CI) for every renamed project.
- [ ] Verify ALL manual assertions pass (Playwright MCP / curl) with committed evidence in
      `evidence/`.
- [ ] Verify every rule-15 EWT/UWT/DWT defect finding is fixed (ticked) — deferral requires explicit
      user permission (only when genuinely impossible) for defect findings; `SG-###`/`USS-###`
      proposals may be triaged or deferred.
- [ ] Verify every rule-16 AET defect finding is fixed (ticked) — deferral requires explicit user
      permission (only when genuinely impossible); `SG-###` spec-gap proposals may be triaged or
      deferred.
- [ ] Verify Phase 17 and Phase 18's `[HUMAN]` gates both passed (GitHub repo renamed, local folder
      renamed and remote re-pointed).
- [ ] Rename and move: `git mv plans/in-progress/beaver-nest-rebrand/
plans/done/YYYY-MM-DD__beaver-nest-rebrand/` using today's date as the completion date (NOT
      the creation date) — run this from whatever the checkout's current path is post-Phase-18.
- [ ] Update `plans/in-progress/README.md` — remove this plan's entry.
- [ ] Update `plans/done/README.md` — add this plan's entry with its completion date.
- [ ] Update any other READMEs that reference this plan (e.g., `plans/README.md`).
- [ ] Commit the archival (the `evidence/` subfolder moves with the plan):
      `chore(plans): move beaver-nest-rebrand to done`, then commit and push to origin main.
