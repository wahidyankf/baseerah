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

> _Diagram considered and deliberately omitted_: a Mermaid flowchart for this strictly-linear
> Phase 0 → Phase 19 sequence was drafted during plan review, but every candidate placement (a
> top-level fenced block ahead of this section) triggers markdownlint's MD046 "consistent code-block
> style" rule against this file's pre-existing list-embedded Gherkin and step-definition fences in
> Phase 6, Phase 10, Phase 11, and Phase 15 (those fences are indented deeper than CommonMark's
> list-continuation width, so markdownlint parses them as indented-style blocks; introducing any
> correctly-recognized top-level fenced block then
> flags a style mismatch). Fixing that latent formatting issue is out of scope for this change. The
> table below, plus the serial-dependency rationale in the prose above, remains the documentation of
> record for phase ordering and the two `[HUMAN]` phases.

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
| 14    | agent fleet + skills (`.amazonq` binding deferred to Phase 15)        | yes                                  |
| 15    | `rhino-cli` functional couplings + `.amazonq` binding                 | yes                                  |
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

- [x] [AI] Confirm the working tree is clean: run `git status --porcelain` from the repo root —
      acceptance: no output. If output exists, stop and surface it; do not stash or discard.
- [x] [AI] Confirm the checkout is on `main` and level with the remote: run
      `git rev-parse --abbrev-ref HEAD && git fetch origin && git status -sb` — acceptance: branch
      is `main`, status line shows no `ahead`/`behind` counts.
- [x] [AI] Record the pre-plan commit SHA into
      `plans/in-progress/beaver-nest-rebrand/evidence/phase-0-baseline.txt`: run `git rev-parse HEAD`
      and write it under a `## Pre-plan HEAD` heading — acceptance: file exists, contains a
      40-character SHA.
- [x] [AI] Install dependencies: `npm install` — acceptance: exits 0.
- [x] [AI] Converge the polyglot toolchain: `npm run doctor -- --fix` — acceptance: exits 0, no
      missing tools reported.
- [x] [AI] Record the residual-reference baseline: run
      `git grep -liE "baseerah" -- . ':!plans/done' ':!generated-reports' | wc -l` and append the
      count to `evidence/phase-0-baseline.txt` under a `## Baseline residual count` heading —
      acceptance: the file records `246` (or the current count if the repo has changed since
      authoring; any material deviation from 246 is surfaced to the maintainer before proceeding).
- [x] [AI] Record the baseline quality state: run
      `npx nx run-many -t typecheck,lint,test:quick --all --parallel=$(( $(sysctl -n hw.ncpu) - 1 ))`
      and append the summary line to `evidence/phase-0-baseline.txt` under a `## Baseline
test:quick` heading — acceptance: the summary line is recorded verbatim, whether it passed
      or failed.
- [x] [AI] If the baseline run reported failures, fix each preexisting failure now, per the
      [Root Cause Orientation principle](../../../repo-governance/principles/general/root-cause-orientation.md)
      — acceptance: a re-run of the same command exits 0. Record any fix in `learnings.md`.
- [x] [AI] Confirm the only GitHub Environments configured are the two harmless auto-created ones
      (as last verified 2026-08-01): run `gh api
repos/wahidyankf/baseerah/environments --jq '.environments[].name'` — acceptance: output is
      exactly `baseerah-app-local` and `baseerah-app-staging` (both auto-created by earlier workflow
      runs referencing an `environment:` key, with empty `protection_rules` and no secrets — Phase 13
      deletes both once the workflows are cut over). If any OTHER/unexpected environment name
      appears, stop and surface it to the maintainer before Phase 13 — that one would need explicit
      investigation, not just deletion.
- [x] [AI] Confirm no `stag-*`/`prod-*` branches exist (Decision 11's premise): run
      `git branch -r` — acceptance: only `origin/main` is listed.

**Date**: 2026-08-01. **Status**: All 10 items complete. **Files Changed**:
`plans/in-progress/beaver-nest-rebrand/evidence/phase-0-baseline.txt` (new). **Notes**: pre-plan HEAD
`fec0c3ab70c079a370ab24df563e5a2fc63c896a`; residual `baseerah` count 253 (vs. authored baseline 246 —
+7/~2.8%, non-material drift from ordinary commits landing since plan authoring, proceeded per the
plan's own drift-tolerance clause); baseline `typecheck,lint,test:quick --all` clean on first run, 9
projects, exit 0, no preexisting failures, so `learnings.md` was not touched; GitHub Environments =
exactly `baseerah-app-local` + `baseerah-app-staging`; `git branch -r` = only `origin/main`.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `git status --porcelain` — output contains only the new `evidence/` file and this plan's
      own folder (already tracked as part of this plan's authoring).
- [x] [AI] `evidence/phase-0-baseline.txt` records the pre-plan SHA, the residual count, and the
      baseline test:quick summary.
- [x] [AI] `npx nx run rhino-cli:test:quick` exits 0 (independent green check before any rename
      touches `rhino-cli`).

**Date**: 2026-08-01. **Status**: Gate green — all 3 checks passed. **Files Changed**: none (verification
only). **Notes**: `git status --porcelain` shows only the new untracked `evidence/` file; evidence file
contains all three required headings with real values; `npx nx run rhino-cli:test:quick` exited 0.

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no rename has
> started. Safe to stop indefinitely. To resume: re-run the baseline commands and confirm they are
> still clean.

---

## Phase 1: Root Identity Files and Vision Doc

- [x] [AI] Rename the vision doc: `git mv repo-governance/vision/baseerah.md
repo-governance/vision/beaver-nest.md` — acceptance: `test -f
repo-governance/vision/beaver-nest.md` succeeds and the old path no longer exists.
- [x] [AI] Apply `<CANONICAL-SED>` to `repo-governance/vision/beaver-nest.md`, then hand-edit its
      title/body to state plainly (per Decision 9/Q7) that "BeaverNest" is a chosen product name
      with no etymological parallel to `بصيرة` — remove the ENTIRE etymology sentence, "means
      _insight_, _inner vision_, _ketajaman melihat_ — in Indonesian, _wawasan_ or _kejernihan
      pandang_." (`[Repo-grounded]`: verified 2026-08-01, this live file's sentence has FOUR
      etymology terms, not the two/three quoted elsewhere in this plan — `بصيرة`, `ketajaman
      melihat`, `wawasan`, and `kejernihan pandang` — all four must go together, not just بصيرة),
      rather than translating any of it — acceptance: `grep -c "بصيرة\|ketajaman melihat\|wawasan\|
kejernihan pandang" repo-governance/vision/beaver-nest.md` returns `0`.
- [x] [AI] Apply `<CANONICAL-SED>` to `repo-governance/vision/README.md` (updates its cross-link to
      the renamed file) — acceptance: `grep -c "baseerah" repo-governance/vision/README.md` returns
      `0`.
- [x] [AI] Apply `<CANONICAL-SED>` to `README.md`, `CONTRIBUTING.md`, `ROADMAP.md`, `AGENTS.md`,
      then hand-edit the same بصيرة-etymology sentence in both `README.md` ("**BeaverNest** (Arabic
      بصيرة) means _insight_, _inner vision_ — in Indonesian, _wawasan_ or _kejernihan pandang_.")
      and `AGENTS.md` ("**BeaverNest** (Arabic بصيرة — _insight_, _wawasan_, _kejernihan pandang_)")
      the sed pass alone would leave behind — per Decision 9/Q7, the same treatment already applied
      to the vision doc above: remove the etymology clause entirely rather than translating it,
      keeping each sentence's surrounding structure intact (e.g. README.md's sentence becomes
      "**BeaverNest** is a personal operating layer... See the [BeaverNest
      Vision](./repo-governance/vision/beaver-nest.md) for the full 'why'.", folding the two
      sentences together; AGENTS.md's becomes "**BeaverNest** — a personal operating layer covering
      ..."). Per Decision 12, `CONTRIBUTING.md`'s `git clone https://github.com/wahidyankf/baseerah.git`
      / `cd baseerah` lines are a deliberate exception — revert just those two lines back to
      `baseerah` after the sed pass: `perl -pi -e 's/github\.com\/wahidyankf\/beaver-nest/
github.com\/wahidyankf\/baseerah/; s/^(\s*)cd beaver-nest$/$1cd baseerah/' CONTRIBUTING.md` (note
      the `(\s*)`/`$1` capture — unlike `development-environment-setup.md`'s unindented clone block,
      `CONTRIBUTING.md`'s `cd beaver-nest` line sits inside a numbered list item indented 3 spaces,
      so a bare `^cd beaver-nest$` anchor would silently fail to match; `[Repo-grounded]`: verified
      against the live file, 2026-08-01) (Phase 17 flips them for real once the GitHub rename has
      actually happened) — acceptance: `git grep -lic
baseerah README.md ROADMAP.md AGENTS.md` returns no matches, `grep -c "wahidyankf/baseerah\|^\s*cd
baseerah$" CONTRIBUTING.md` returns `2` (the two preserved GitHub-URL lines, and nothing else —
      `git grep -c baseerah CONTRIBUTING.md` returns exactly `2` too, confirming no OTHER residual
      snuck back in), and `grep -c "بصيرة\|wawasan\|kejernihan pandang" README.md AGENTS.md` returns
      `0` for both.
- [x] [AI] Edit `package.json`: change `"name": "baseerah"` to `"name": "beaver-nest"` (line ~2) and
      rename the `baseerah:dev`/`baseerah:dev:restart` scripts to `beaver-nest:dev`/
      `beaver-nest:dev:restart`, updating their `infra/dev/baseerah-app/` path references to
      `infra/dev/beaver-nest-app/` (the directory itself is renamed in Phase 12; record the new path
      now so Phase 12's `git mv` matches) — acceptance: `grep -c "baseerah" package.json` returns `0`.
- [x] [AI] Run `npm install` to regenerate `package-lock.json`'s root `name` field consistently —
      acceptance: `grep -c '"name": "beaver-nest"' package-lock.json` returns at least `1`.
- [x] [AI] Edit `.gitignore` line 159 (`specs/apps/baseerah/containers/contracts/generated/`) to
      `specs/apps/beaver-nest/containers/contracts/generated/` — acceptance: `grep -c "baseerah"
.gitignore` returns `0`.
- [x] [AI] Apply `<CANONICAL-SED>` to `SECURITY.md` and `LICENSING-NOTICE.md` — acceptance:
      `git grep -lic baseerah SECURITY.md LICENSING-NOTICE.md` returns no matches.

**Date**: 2026-08-01. **Status**: All 8 items complete. **Files Changed**: `repo-governance/vision/beaver-nest.md`
(renamed + sed + etymology removed), `repo-governance/vision/README.md`, `README.md`, `CONTRIBUTING.md`,
`ROADMAP.md`, `AGENTS.md`, `package.json`, `package-lock.json`, `.gitignore`, `SECURITY.md`,
`LICENSING-NOTICE.md`. **Notes**: all acceptance greps verified 0/exact-match as specified. Found and
fixed a preexisting defect in this phase's own acceptance-criterion text (line 188's `grep` pattern used
a bare `^cd baseerah$` anchor that silently fails to match CONTRIBUTING.md's 3-space-indented line —
same bug class as iteration-18's finding; corrected to `^\s*cd baseerah$`). Discovered mid-phase: the
repo's pre-push hook runs `md links validate` repo-wide and blocks the push on ANY broken link, not
scoped to this phase's file set — so renaming `vision/baseerah.md` broke 27 inbound links from files
outside Phase 1's scope (`.claude/agents/`, `.cursor/agents/`, `.opencode/agents/`, `plans/ideas/`,
`specs/apps/baseerah/product/README.md`, and this plan's own `README.md`/`brd.md`). This means the
plan's "each phase pushes independently" assumption needs a small addendum: **every phase that
`git mv`s a path must also repoint (not fully content-sweep) all repo-wide inbound links to that path
in the SAME commit**, even though the referencing file's full `baseerah`→`beaver-nest` content sweep
stays deferred to its own designated phase. Applied here via a targeted single-string sed
(`repo-governance/vision/baseerah\.md` → `repo-governance/vision/beaver-nest.md`) across exactly the
27 files `git grep -l` found, touching nothing else in them. Logged to `learnings.md` as a
generalizable pattern for every later phase that renames a path (Phases 6, 8, 9, 10, 11, 12).

### Local Quality Gates (Before Push)

- [x] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures
      (including preexisting) before proceeding.

**Date**: 2026-08-01. **Status**: `nx affected` reported "No tasks were run" — none of Phase 1's touched
files (root docs/governance prose, package.json, .gitignore) belong to any Nx project's source root, so
zero tasks were affected; trivially green. `markdownlint-cli2` on all 8 touched markdown files: 0 errors.

### Commit Guidelines

- [x] [AI] Commit: `chore(rebrand): rename root identity files and vision doc to BeaverNest`

### Post-Push CI Verification

- [x] [AI] Commit and push to origin main.
- [x] [AI] Monitor ALL GitHub Actions workflows triggered by this push; verify ALL checks pass; fix
      and re-push if any fail.

**Date**: 2026-08-01. **Status**: Pushed as 2 commits (52806c370 body, 06c6c9a1f link-repoint fix
discovered mid-phase — see learnings.md). All 3 triggered workflows on 06c6c9a1f (`pr-quality-gate`,
`publish-images`, `validate-env`) completed with `conclusion: success`. **Files Changed**: none
(verification only).

### Phase 1 Gate

- [x] [AI] `git grep -lic baseerah README.md ROADMAP.md AGENTS.md package.json .gitignore
SECURITY.md LICENSING-NOTICE.md repo-governance/vision/` returns no matches (`CONTRIBUTING.md` is
      checked separately below, per Decision 12's deliberate exception). `package-lock.json` is
      excluded from this blanket check: it legitimately still contains `apps/baseerah-fe`,
      `apps/baseerah-fe-e2e`, and `apps/baseerah-be-e2e` workspace-path entries until those app
      directories are themselves renamed in Phases 8-11 — instead check just the root `name` field:
      `grep -c '"name": "beaver-nest"' package-lock.json` returns at least `1`.
- [x] [AI] `git grep -c baseerah CONTRIBUTING.md` returns exactly `2` (only the two preserved
      GitHub-URL lines Decision 12 defers to Phase 17 — anything else would mean an unrelated
      residual snuck back in).
- [x] [AI] `test -f repo-governance/vision/beaver-nest.md` succeeds; `test -f
repo-governance/vision/baseerah.md` fails.
- [x] [AI] `grep -c "بصيرة\|wawasan\|kejernihan pandang" README.md AGENTS.md` returns `0` for both.
- [x] [AI] `grep -c "بصيرة\|ketajaman melihat\|wawasan\|kejernihan pandang"
repo-governance/vision/beaver-nest.md` returns `0`.

**Date**: 2026-08-01. **Status**: Phase 1 Gate green — all 5 checks passed (verified live: root-file
grep clean, CONTRIBUTING.md=2, vision doc renamed, etymology=0 in both README/AGENTS and vision.md).
Found and fixed a genuine plan-design gap during verification: the gate's original blanket
`package-lock.json` check could never pass at this point in the plan — `apps/baseerah-fe`,
`apps/baseerah-fe-e2e`, `apps/baseerah-be-e2e` workspace-path entries legitimately remain until those
app directories are renamed in Phases 8-11. Fixed by excluding `package-lock.json` from the blanket
grep and checking only its root `name` field instead. **Files Changed**:
`plans/in-progress/beaver-nest-rebrand/delivery.md` (gate-scope fix, not yet committed — rides into
Phase 2's commit).

> **Pause Safety**: root identity is fully renamed and pushed; every later phase can proceed
> independently of this one. Safe to stop. To resume: `git status -sb` shows level with
> `origin/main`, then start Phase 2.

---

## Phase 2: `repo-governance/` Sweep

- [x] [AI] Apply `<CANONICAL-SED>` to every git-tracked file under `repo-governance/` except
      `repo-governance/vision/beaver-nest.md` (already done in Phase 1), preserving historical
      citations per [tech-docs.md Decision 6](./tech-docs.md#decision-log): (1) capture
      `git grep -l "baseerah-repo-reset" -- repo-governance/ >
local-temp/rebrand-citations-phase2.txt`; (2) run `git ls-files -z repo-governance/ | grep -zv
      'vision/beaver-nest.md' | xargs -0 perl -pi -e '<CANONICAL-SED-BODY>'`; (3) revert the
      captured files' mangled citation: `< local-temp/rebrand-citations-phase2.txt xargs -I{} perl
      -pi -e 's/beaver-nest-repo-reset/baseerah-repo-reset/g' {}` (note: `xargs -a` is GNU-only;
      BSD/macOS `xargs` lacks it — redirect the file into stdin instead); (4) per Decision 12, revert the
      two GitHub-URL clone blocks the sed pass just mangled in
      `repo-governance/workflows/infra/development-environment-setup.md`: `perl -pi -e
      's/github\.com\/wahidyankf\/beaver-nest/github.com\/wahidyankf\/baseerah/g;
      s/^cd beaver-nest$/cd baseerah/'
repo-governance/workflows/infra/development-environment-setup.md` (Phase 17 flips these for real
      once the GitHub rename has actually happened) — acceptance:
      `git grep -l "baseerah-repo-reset" -- repo-governance/ | diff -
local-temp/rebrand-citations-phase2.txt` reports no differences (the historical citations are
      intact, unchanged, and unmangled), `git grep -lic baseerah repo-governance/` returns only that
      same captured file set **plus** `development-environment-setup.md`, and `grep -c
      "wahidyankf/baseerah\|^cd baseerah$"
repo-governance/workflows/infra/development-environment-setup.md` returns `4` (two clone-URL lines + two `cd baseerah` lines, and nothing else — `git grep -c baseerah
      repo-governance/workflows/infra/development-environment-setup.md` returns exactly `4` too).
      **Done 2026-08-01**: citation diff clean, `git grep -lic baseerah repo-governance/` returns
      exactly `pdf-to-md-quality-gate.md` + `development-environment-setup.md`, dev-env-setup count
      = 4. Discovered `xargs -a` is BSD/macOS-unsupported (see [learnings.md](./learnings.md)) —
      fixed manually for this phase and proactively rewrote all 5 occurrences of the pattern across
      this file (Phases 2, 3, 4, 6, 12) to the portable `< file xargs -I{}` form.
- [x] [AI] Spot-check `repo-governance/development/agents/ai-agents.md` and
      `repo-governance/conventions/structure/agent-naming.md` for illustrative examples that named
      `baseerah-fe` directly — `[Repo-grounded]`: as of 2026-08-01 these are ai-agents.md's `# Content
Checker for baseerah-fe` title-pattern example, its "`baseerah-fe` matches `apps/baseerah-fe/`"
      naming-parity example, and agent-naming.md's `baseerah-fe` qualifier-token example — confirm
      each now reads `beaver-nest-fe` and still parses as a valid example (no broken sentence
      structure from the mechanical swap) — acceptance: manual read confirms coherent prose.

### Local Quality Gates (Before Push)

- [x] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures.
      **Done 2026-08-01**: nx affected (27 tasks, 26 cached) all succeed; `md links validate`
      clean; `md mermaid validate` clean (only pre-existing unrelated fixture failures + 1
      pre-existing warning, both outside repo-governance/); `markdownlint-cli2` on
      `repo-governance/**/*.md` — 0 errors.

### Commit Guidelines

- [x] [AI] Commit: `chore(rebrand): rename baseerah references across repo-governance/`
      **Done 2026-08-01**: commit `86bdcf60b`.

### Post-Push CI Verification

- [x] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.
      **Done 2026-08-01**: pushed to origin main; all 3 workflows (validate-env, publish-images,
      pr-quality-gate) green on commit `86bdcf60b`, no re-push needed.

### Phase 2 Gate

- [x] [AI] `git grep -l "baseerah-repo-reset" -- repo-governance/ | diff -
      local-temp/rebrand-citations-phase2.txt` reports no differences, and `git grep -lic baseerah
repo-governance/` returns only that same captured file set **plus**
      `repo-governance/workflows/infra/development-environment-setup.md` (Decision 12's preserved
      GitHub-URL file, per the step above) — no other `baseerah` residue.
      **Done 2026-08-01**: diff clean; `git grep -lic baseerah repo-governance/` returns exactly
      `pdf-to-md-quality-gate.md` + `development-environment-setup.md`; dev-env-setup count = 4.
- [x] [AI] `npx nx run rhino-cli:test:quick` still exits 0 (governance-doc changes never touch code,
      but this confirms no accidental cross-contamination).
      **Done 2026-08-01**: exits 0 (373 scenarios, 1552 steps, all covered; specs structure validate
      0 findings).

> **Pause Safety**: `repo-governance/` is fully clean of `baseerah` residue outside the preserved
> historical citations (Decision 6) and pushed. Safe to stop. To resume: confirm level with
> `origin/main`, then start Phase 3.

---

## Phase 3: `docs/` Sweep

- [x] [AI] Apply `<CANONICAL-SED>` to every git-tracked file under `docs/`, preserving historical
      citations per [tech-docs.md Decision 6](./tech-docs.md#decision-log): (1) capture
      `git grep -l "baseerah-repo-reset" -- docs/ > local-temp/rebrand-citations-phase3.txt`; (2) run
      `git ls-files -z docs/ | xargs -0 perl -pi -e '<CANONICAL-SED-BODY>'`; (3) revert the captured
      files' mangled citation: `< local-temp/rebrand-citations-phase3.txt xargs -I{} perl -pi -e
      's/beaver-nest-repo-reset/baseerah-repo-reset/g' {}` (BSD/macOS `xargs` has no `-a`; redirect
      the file into stdin instead) — acceptance: `git grep -l
"baseerah-repo-reset" -- docs/ | diff - local-temp/rebrand-citations-phase3.txt` reports no
      differences and `git grep -lic baseerah docs/` returns only that same captured file set.
      **Done 2026-08-01**: 12 citation files captured, diff clean, residual matches exactly.
- [x] [AI] Spot-check `docs/reference/system-architecture/applications.md` and
      `docs/reference/monorepo-structure.md` for any diagram or table listing app names — confirm
      renamed entries read `beaver-nest-be`/`beaver-nest-fe` — acceptance: manual read confirms.
      **Done 2026-08-01**: both files read coherently — mermaid diagram nodes, app cards, and
      dependency-list entries all show `beaver-nest-fe`/`beaver-nest-be` correctly.

### Local Quality Gates (Before Push)

- [x] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures.
      **Done 2026-08-01**: nx affected (27 tasks, 26 cached) all succeed. `md links validate`
      initially found 1 broken link (`deployment.md` → prematurely-renamed
      `beaver-nest-first-deploy.md`, a new discovery — see [learnings.md](./learnings.md)); reverted
      that one link, re-ran, clean. `markdownlint-cli2` on `docs/**/*.md` — 0 errors.

### Commit Guidelines

- [x] [AI] Commit: `chore(rebrand): rename baseerah references across docs/`
      **Done 2026-08-01**: commit pending push (see Post-Push step below for SHA).

### Post-Push CI Verification

- [x] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 3 Gate

- [x] [AI] `git grep -l "baseerah-repo-reset" -- docs/ | diff -
      local-temp/rebrand-citations-phase3.txt` reports no differences, and `git grep -lic baseerah
docs/` returns only that same captured file set.
      **Done 2026-08-01**: diff clean, residual set unchanged (still exactly the 12 captured
      citation files, including `deployment.md` which now also carries the reverted
      `baseerah-first-deploy.md` link text pending Phase 4's file move).

> **Pause Safety**: `docs/` is fully clean of `baseerah` residue outside the preserved historical
> citations (Decision 6) and pushed. Safe to stop. To resume: confirm level with `origin/main`, then
> start Phase 4.

---

## Phase 4: `plans/backlog/`, `plans/ideas/`, and `plans/in-progress/README.md` Sweep

- [x] [AI] Rename the three idea briefs: `git mv plans/ideas/baseerah-first-deploy.md
plans/ideas/beaver-nest-first-deploy.md && git mv plans/ideas/baseerah-first-llm-integration.md
plans/ideas/beaver-nest-first-llm-integration.md && git mv
plans/ideas/baseerah-persistence-layer.md plans/ideas/beaver-nest-persistence-layer.md` —
      acceptance: all three new paths exist, old paths do not. **Also**: Phase 3's sweep left
      `docs/reference/system-architecture/deployment.md` line 22 pointing at
      `../../../plans/ideas/baseerah-first-deploy.md` (reverted from the sed's premature rename
      since the file didn't exist yet at that path) — repoint this one link to
      `beaver-nest-first-deploy.md` in this same commit now that the file has actually moved (see
      [learnings.md](./learnings.md)).
      **Done 2026-08-01**: all 3 files renamed; `deployment.md`'s link repointed. `md links
      validate` also found 2 MORE repo-wide inbound links to the old path (`apps/README.md`,
      `plans/in-progress/beaver-nest-rebrand/brd.md`) — fixed both with a targeted path-only sed
      (new learnings.md entry: this is the same class of bug as the Phase 1 discovery, this time
      triggered by a `git mv` rather than a content rename).
- [x] [AI] Apply `<CANONICAL-SED>` to every git-tracked file under `plans/backlog/` and
      `plans/ideas/`, plus `plans/in-progress/README.md`, preserving historical citations per
      [tech-docs.md Decision 6](./tech-docs.md#decision-log): (1) capture `git grep -l
"baseerah-repo-reset" -- plans/backlog/ plans/ideas/ plans/in-progress/README.md >
      local-temp/rebrand-citations-phase4.txt` (this captures BOTH `plans/in-progress/README.md`'s
      lowercase `` `baseerah-repo-reset/` `` illustrative-example citation at line 17 and every
      other citing file under `plans/backlog/`/`plans/ideas/` — not a hardcoded 2-file list); (2) run
      `git ls-files -z plans/backlog/ plans/ideas/ plans/in-progress/README.md | xargs -0 perl -pi -e
      '<CANONICAL-SED-BODY>'`; (3) revert the captured files' mangled citation: `< local-temp/rebrand-citations-phase4.txt xargs -I{} perl -pi -e
      's/beaver-nest-repo-reset/baseerah-repo-reset/g' {}` (BSD/macOS `xargs` has no `-a`; redirect the
      file into stdin instead) — acceptance: `git grep -l
"baseerah-repo-reset" -- plans/backlog/ plans/ideas/ plans/in-progress/README.md | diff -
      local-temp/rebrand-citations-phase4.txt` reports no differences (the citing files, including
      `plans/in-progress/README.md`'s line-17 lowercase citation, are intact and unmangled), and
      `git grep -lic baseerah plans/backlog/ plans/ideas/ plans/in-progress/README.md` returns only
      that same captured file set (`plans/in-progress/README.md`'s residual `baseerah` match here is
      exactly this preserved citation; its separate capital-`Baseerah` "from X to Y" sentence is
      fixed by the next step and checked by the Phase 4 Gate below).
- [x] [AI] Manually fix the "from X to Y" narrative sentence(s) the blind sed pass just destroyed:
      `plans/in-progress/README.md`'s `beaver-nest-rebrand` entry describes this plan as renaming
      "the repository's product identity from Baseerah to BeaverNest" — because both the old-name
      and new-name tokens collapse to the same replacement string under `<CANONICAL-SED>`'s catch-all
      rule, the sed pass mangles this into "...from BeaverNest to BeaverNest...", destroying the
      sentence's meaning. Hand-rewrite that one sentence (and any other repo-wide "from Baseerah to
      BeaverNest"-shaped narrative prose this sweep touches, e.g. in `plans/backlog/` or
      `plans/ideas/`) to read "...renames the repository's product identity to BeaverNest (formerly
      Baseerah)..." or equivalent phrasing that preserves both names — do not trust the mechanical
      substitution for narrative sentences describing the rename itself — acceptance: manual read of
      `plans/in-progress/README.md`'s entry confirms it still names both "Baseerah" and "BeaverNest"
      and reads coherently.
      **Done 2026-08-01**: fixed to "...renames the repository's product identity to BeaverNest
      (formerly Baseerah)...". No other "from X to Y"-shaped sentences found elsewhere in
      `plans/backlog/`/`plans/ideas/`.
- [x] [AI] Update `plans/ideas/README.md`'s three bullet links to point at the renamed filenames —
      acceptance: `grep -c "beaver-nest-first-deploy.md\]" plans/ideas/README.md` returns at least
      `1` and the equivalent checks pass for the other two renamed files.
      **Done 2026-08-01**: already correct — the CANONICAL-SED pass renamed these link targets
      alongside the `git mv`, since both happened in this same phase. Verified all 3 links resolve.

### Local Quality Gates (Before Push)

- [x] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures.
      **Done 2026-08-01**: nx affected (27 tasks, 26 cached) all succeed. `md links validate`
      initially found 2 broken links from the idea-brief renames (see notes above); fixed, re-ran,
      clean. `markdownlint-cli2` across all touched paths — 0 errors.

### Commit Guidelines

- [x] [AI] Commit: `chore(rebrand): rename baseerah references across active plans/`

### Post-Push CI Verification

- [x] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 4 Gate

- [x] [AI] `git grep -l "baseerah-repo-reset" -- plans/backlog/ plans/ideas/
plans/in-progress/README.md | diff - local-temp/rebrand-citations-phase4.txt` reports no
      differences, and `git grep -lic baseerah plans/backlog/ plans/ideas/` returns only that
      captured file set restricted to `plans/backlog/`/`plans/ideas/` (no other residue). (This
      intentionally does not scan all of `plans/in-progress/`: this plan's own folder,
      `plans/in-progress/beaver-nest-rebrand/`, necessarily narrates the old name throughout and is
      renamed and moved to `plans/done/` only in the final Plan Archival step — see Phase 16's
      residual-sweep exclusion for the same reasoning.)
      **Done 2026-08-01**: diff clean, residual set unchanged.
- [x] [AI] `grep -c "baseerah-repo-reset" plans/in-progress/README.md` returns exactly `1` (the
      preserved lowercase illustrative-example citation at line 17, restored by the revert step
      above — this is a SEPARATE citation from the "Baseerah" capital-B check below; both must hold
      simultaneously in this one file).
      **Done 2026-08-01**: returns `1`.
- [x] [AI] `grep -c "Baseerah" plans/in-progress/README.md` returns exactly `1` (the intentionally
      preserved "formerly Baseerah" historical framing added by the manual fix above — case-sensitive,
      so it does not double-count the lowercase citation checked above).
      **Done 2026-08-01**: returns `1`.
- [x] [AI] `test -f plans/ideas/beaver-nest-first-deploy.md && test -f
plans/ideas/beaver-nest-first-llm-integration.md && test -f
plans/ideas/beaver-nest-persistence-layer.md` all succeed.
      **Done 2026-08-01**: all three exist.

> **Pause Safety**: active plan content is fully renamed and pushed; `plans/done/` remains untouched
> per the historical-citation exemption (Decision 6). Safe to stop. To resume: confirm level with
> `origin/main`, then start Phase 5.

---

## Phase 5: `repo-config.yml` and Registry Consistency

- [ ] [AI] Edit `repo-config.yml`: rename the `coverage.projects` entries `baseerah-be`,
      `baseerah-be-e2e`, `baseerah-fe`, `baseerah-fe-e2e` to their `beaver-nest-*` equivalents, and
      update each entry's `specs:` glob from `specs/apps/baseerah/behavior/...` to
      `specs/apps/beaver-nest/behavior/...` — acceptance: `grep -c "baseerah" repo-config.yml`
      returns `0` after this step and the three below.
- [ ] [AI] Edit `repo-config.yml`'s `env-contract.surfaces` entries: rename `root: apps/baseerah-be`
      to `root: apps/beaver-nest-be` and its `BASEERAH_BE_CORS_ORIGINS` allowlist entry to
      `BEAVER_NEST_BE_CORS_ORIGINS`; rename `root: apps/baseerah-fe` to `root: apps/beaver-nest-fe`.
- [ ] [AI] Edit `repo-config.yml`'s `env-injection.apps` entries: rename `app: baseerah-be` to
      `app: beaver-nest-be` and `app: baseerah-fe` to `app: beaver-nest-fe`; rename the
      `ci-harness.environments` values `baseerah-app-staging` to `beaver-nest-app-staging` in both
      `API_BASE_URL`/`WEB_BASE_URL`/`VERCEL_AUTOMATION_BYPASS_SECRET` entries.
- [ ] [AI] Sweep the remaining `baseerah` occurrences the three targeted edits above don't touch —
      `[Repo-grounded]`: as of 2026-08-01 these are the `# baseerah domain` comment, the
      `Deliberately excluded, not omitted by oversight: baseerah-contracts's test-level...` comment,
      the `(browser-facing CORS isn't needed yet — baseerah-fe fetches server-side)` comment, and the
      two `keys-from: apps/baseerah-be/.env.example` / `apps/baseerah-fe/.env.example` paths — rename
      each to its `beaver-nest` equivalent (comments reworded in place, `keys-from` paths updated to
      match the Phase 8/Phase 4 `.env.example` renames). Re-run `grep -c "baseerah" repo-config.yml`
      to confirm `0` before moving on — if it isn't, the file still has a residual this step's list
      didn't anticipate; find it and rename it too.
- [ ] [AI] Verify the YAML still parses: run `cargo run --release --quiet --manifest-path
apps/rhino-cli/Cargo.toml -- specs structure validate` — acceptance: exits 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures.

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
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `specs/apps/beaver-nest/`, preserving
      historical citations per [tech-docs.md Decision 6](./tech-docs.md#decision-log): (1) capture
      `git grep -l "baseerah-repo-reset" -- specs/apps/beaver-nest/ >
local-temp/rebrand-citations-phase6.txt` (both `.../beaver-nest-be/gherkin/README.md` and
      `.../beaver-nest-fe/gherkin/README.md` cite the archived plan-id); (2) run `git ls-files -z
      specs/apps/beaver-nest/ | xargs -0 perl -pi -e '<CANONICAL-SED-BODY>'`; (3) revert the captured
      files' mangled citation: `< local-temp/rebrand-citations-phase6.txt xargs -I{} perl -pi -e
      's/beaver-nest-repo-reset/baseerah-repo-reset/g' {}` (BSD/macOS `xargs` has no `-a`; redirect
      the file into stdin instead) — acceptance: `git grep -l
"baseerah-repo-reset" -- specs/apps/beaver-nest/ | diff -
      local-temp/rebrand-citations-phase6.txt` reports no differences, and `git grep -lic baseerah
specs/apps/beaver-nest/` returns only that same captured file set.
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
        Then no بصيرة/wawasan-style etymology chip is present
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

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures. (`specs:behavior:coverage` is
      expected RED per the step above; do not fix it here — Phase 10/11 resolve it.)

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename specs/apps/baseerah to specs/apps/beaver-nest`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI (the `beaver-nest-fe`/`beaver-nest-be`
      `specs:behavior:coverage` targets are expected to still fail here — record that in the push notes, it
      resolves by Phase 11's gate).

### Phase 6 Gate

- [ ] [AI] `git grep -l "baseerah-repo-reset" -- specs/apps/beaver-nest/ | diff -
      local-temp/rebrand-citations-phase6.txt` reports no differences, and `git grep -lic baseerah
specs/apps/beaver-nest/` returns only that same captured file set.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- specs structure
validate` exits 0.

> **Pause Safety**: the spec tree is fully renamed; `specs:behavior:coverage` for the FE project is
> deliberately RED and will stay that way until Phase 11. Safe to stop with this known RED state
> recorded here. To resume: confirm level with `origin/main`, then start Phase 7.

---

## Phase 7: `libs/web-ui-token` Brand Palette File

- [ ] [AI] Rename: `git mv libs/web-ui-token/src/baseerah.css libs/web-ui-token/src/beaver-nest.css`
      — acceptance: `test -f libs/web-ui-token/src/beaver-nest.css` succeeds.
- [ ] [AI] Edit `libs/web-ui-token/src/beaver-nest.css`: reword the two brand-meaning comments
      (lines 3-4 and 43) to plainly describe this as "BeaverNest's palette" with no بصيرة reference
      (per Decision 8/Q7) — leave every `oklch(...)` numeric value byte-identical — acceptance:
      `diff <(grep -o 'oklch([^)]*)' libs/web-ui-token/src/beaver-nest.css | sort) <(git show
HEAD:libs/web-ui-token/src/baseerah.css | grep -o 'oklch([^)]*)' | sort)` reports no
      differences, and `grep -c "بصيرة" libs/web-ui-token/src/beaver-nest.css` returns `0`.
- [ ] [AI] Apply `<CANONICAL-SED>` to `libs/web-ui-token/README.md` and `libs/README.md`, then hand-
      edit `libs/web-ui-token/README.md`'s "`### baseerah.css`" section (now "`### beaver-nest.css`"):
      reword "Indigo-violet OKLCH design system for BeaverNest apps (`beaver-nest-fe`), evoking بصيرة
      (insight, inner vision):" to drop the بصيرة clause per Decision 8/9/Q7, matching the
      `beaver-nest.css` file-comment treatment above — then for `libs/README.md` only, run a scoped
      follow-up revert to restore the one historical citation the catch-all rule just mangled: `perl
-pi -e 's/beaver-nest-repo-reset/baseerah-repo-reset/g' libs/README.md` — this is safe because
      `beaver-nest-repo-reset` cannot appear anywhere else in the file except as the sed-mangled form
      of the preserved `2026-07-31__baseerah-repo-reset` citation link (per Decision 6; do not let
      the catch-all rule's output stand for that one link target) — acceptance: `grep -c "baseerah"
libs/README.md` returns exactly `1` (the preserved citation, correctly restored), `grep -c
"baseerah" libs/web-ui-token/README.md` returns `0`, and `grep -c "بصيرة"
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
- [ ] [AI] `grep -c "بصيرة" libs/web-ui-token/README.md libs/web-ui-token/src/beaver-nest.css`
      returns `0` for both.

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
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `apps/beaver-nest-fe/`, then per Decision 12
      revert the three GitHub-URL citations the sed pass just mangled (Phase 17 flips them for real
      once the GitHub rename has actually happened): `perl -pi -e
's/github\.com\/wahidyankf\/beaver-nest/github.com\/wahidyankf\/baseerah/g'
apps/beaver-nest-fe/Dockerfile apps/beaver-nest-fe/src/components/AppShell.tsx
apps/beaver-nest-fe/src/app/page.test.tsx` — acceptance: `git grep -lic baseerah
apps/beaver-nest-fe/` returns only these three files after the remaining steps in this phase
      also complete, and `grep -c "wahidyankf/baseerah" apps/beaver-nest-fe/Dockerfile
apps/beaver-nest-fe/src/components/AppShell.tsx apps/beaver-nest-fe/src/app/page.test.tsx`
      returns `1` for each of the three.
- [ ] [AI] Confirm the brand copy is already fully converted (no hand-edit needed here — the prior
      `<CANONICAL-SED>` step already rewrote every "Baseerah" occurrence under
      `apps/beaver-nest-fe/`, including the brand-name heading and footer in
      `apps/beaver-nest-fe/src/components/AppFrame.tsx` (`<AppHeader title="BeaverNest" />` and
      `&copy; BeaverNest`) and the one-line description paragraph in
      `apps/beaver-nest-fe/src/components/AppShell.tsx`, which now reads "BeaverNest is a personal
      operating layer — ..." with no Arabic/Indonesian etymology gloss, per Decision 3): the
      brand-chip element a few lines below the description, which still contains
      `wawasan`/`بصيرة`, is untouched by `<CANONICAL-SED>` (it is not a "Baseerah" occurrence) and is
      deleted by the next GREEN step — acceptance: `grep -c "BeaverNest is a personal operating
layer" apps/beaver-nest-fe/src/components/AppShell.tsx` returns `1`.
- [ ] [AI] Hand-edit `apps/beaver-nest-fe/src/app/layout.tsx`'s `metadata.description` field: the
      prior `<CANONICAL-SED>` step already renamed `"Baseerah — insight, wawasan"` to
      `"BeaverNest — insight, wawasan"` (it does contain the literal "Baseerah" token, so the sed
      pass touches it), but leaves the "insight, wawasan" etymology gloss in place in this
      user/search-engine-facing `<meta name="description">` tag — reword to plain product copy with
      no etymology reference (e.g. `"BeaverNest — a personal operating layer"`), per Decision 9
      (`[Repo-grounded]`: this is the one place the etymology gloss survives in rendered/served
      output outside the deleted brand chip, and no other step in this plan touches it) —
      acceptance: `grep -c "wawasan" apps/beaver-nest-fe/src/app/layout.tsx` returns `0`.
- [ ] [AI] GREEN (second half of the cycle whose RED landed in Phase 6; REFACTOR continues below and
      in Phase 11): delete the multilingual brand-chip JSX block from
      `apps/beaver-nest-fe/src/components/AppShell.tsx` (the `title="insight (English) · wawasan
(Indonesian) · بصيرة (Arabic)"` element and its containing wrapper), together with its
      immediately-preceding JSX comment (`{/* Bespoke two-line chip, not the shared \`Badge\`
      primitive... see Rule-15 finding DWT-004. \*/}`) — that comment documents markup rationale for
      the element being deleted here and would otherwise survive as stale dead commentary.
      **Gherkin (binds) →** "The homepage no longer renders a brand-chip etymology gloss" (same
      scenario Phase 6's RED step embedded) — acceptance:`grep -c 'title="insight'
      apps/beaver-nest-fe/src/components/AppShell.tsx`returns`0`and`grep -c "DWT-004"
      apps/beaver-nest-fe/src/components/AppShell.tsx`returns`0`.
- [ ] [AI] GREEN: author the bound scenario into the feature file, so the AC4 binding is a scenario
      `rhino-cli` actually validates, not one merely implied by the old scenario's absence. Add the
      following scenario to
      `specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature`, at the
      same position the "multilingual brand chip is understandable" scenario occupied before Phase 6
      deleted it:

      ```gherkin
      Scenario: The homepage no longer renders a brand-chip etymology gloss
        Given a first-time visitor viewing the rendered homepage
        When they inspect the page for a hoverable multilingual term chip
        Then no بصيرة/wawasan-style etymology chip is present
        And no automated test or Gherkin scenario asserts one exists
      ```

      Acceptance: the scenario text above appears verbatim in the feature file (spot-check with a
      manual read or `grep -A4 "no longer renders a brand-chip"` against the file path above).

- [ ] [AI] GREEN: add the corresponding **no-op** step-definition entries to
      `apps/beaver-nest-fe/src/test/landing.steps.ts`, matching this file's own established
      convention (a literal-text registry whose `Given`/`When`/`Then`/`And` calls are
      locally-shadowed no-op functions — per the file's own header comment, "The real assertions
      live in `src/app/page.test.tsx`; this file exists only so every Gherkin step text has a
      matching Given/When/Then/And call for the coverage checker to find"): add
      `Given("a first-time visitor viewing the rendered homepage", () => {});`,
      `When("they inspect the page for a hoverable multilingual term chip", () => {});`,
      `Then("no بصيرة/wawasan-style etymology chip is present", () => {});`, and
      `And("no automated test or Gherkin scenario asserts one exists", () => {});` — the real DOM
      assertion for this scenario is added to `page.test.tsx` in the REFACTOR step below, per this
      codebase's convention that only `page.test.tsx` executes real assertions (see HIGH-NEW-1 in
      the iteration-2 plan-checker audit) — acceptance: `npx nx run
beaver-nest-fe:specs:behavior:coverage` reports zero step gaps for the new scenario (no "step(s)
      without matching step definitions" for these four steps), but still exits non-zero at this
      point in the sequence, printing exactly `ERROR: Found 1 orphan step implementation(s) (no
      Gherkin step matches them)` (`[Repo-grounded]`: confirmed against
      `apps/rhino-cli/src/commands/specs_coverage.rs`'s `orphan_step_impls` check) for the
      still-present, not-yet-deleted `they read or hover the "بصيرة" and "wawasan" terms` step — this
      orphan is expected here and is resolved by the REFACTOR step immediately below, which deletes
      that step definition and makes the command exit 0.
- [ ] [AI] REFACTOR: edit `apps/beaver-nest-fe/src/app/page.test.tsx`: delete the chip-specific
      assertions (the `screen.getByText("بصيرة")` lines and the `toHaveAttribute("title", ...)`
      assertion, in both the accessibility test and the "tells a first-time visitor" test), keeping
      the heading/greeting/description assertions (updated to "BeaverNest" by the sed pass); then, in
      the "tells a first-time visitor..." test, add the real, executing negative assertion this
      scenario's DOM check requires:
      `expect(screen.queryByTitle(/insight/i)).not.toBeInTheDocument();`; also rename that test's own
      description from `"tells a first-time visitor what Baseerah is, glosses the multilingual chip,
      and offers a way to learn more"` to `"tells a first-time visitor what BeaverNest is and offers
      a way to learn more"` (the sed pass renames "Baseerah" → "BeaverNest" in this string but the
      "glosses the multilingual chip" clause becomes false the moment this REFACTOR step removes the
      chip assertions, so it must be dropped by hand, not left for the sed pass to half-fix) —
      acceptance: `npx nx run beaver-nest-fe:test:unit` exits 0, `grep -c
"queryByTitle(/insight/i)).not.toBeInTheDocument" apps/beaver-nest-fe/src/app/page.test.tsx`
      returns `1`, and `grep -c "glosses the multilingual chip"
apps/beaver-nest-fe/src/app/page.test.tsx` returns `0`.
- [ ] [AI] REFACTOR: edit `apps/beaver-nest-fe/src/test/landing.steps.ts`: delete all three
      now-orphaned step definitions belonging to the scenario Phase 6 deleted —
      `Given("a first-time visitor viewing the homepage brand chip", ...)`,
      `When('they read or hover the "بصيرة" and "wawasan" terms', ...)`, and
      `Then("a plain-language English gloss or tooltip explains what each term means", ...)` (all
      three, not just the `When` line — orphan detection is per-step-text, so leaving the
      `Given`/`Then` lines would still fail) — acceptance: `npx nx run
beaver-nest-fe:specs:behavior:coverage` exits 0 (this resolves Phase 6's deliberate RED for the
      FE project; Phase 11 resolves the same RED for the FE-E2E project).
- [ ] [AI] Run `npx nx run beaver-nest-fe:codegen` then `npx nx run beaver-nest-fe:test:unit` —
      acceptance: both exit 0.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures.

### Specs & Gherkin Delivery

- [ ] [AI] Confirm `specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature`
      (edited in Phase 6, scenario authored above) has no remaining reference to the deleted "brand
      chip is understandable" scenario, contains the new "no longer renders a brand-chip etymology
      gloss" scenario, and its other four preexisting scenarios read "BeaverNest" — acceptance:
      `grep -c "Scenario:" specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature`
      returns `5` and `grep -c "BeaverNest" specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature`
      returns at least `4`.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah-fe to beaver-nest-fe, remove brand-chip feature`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 10 Gate

- [ ] [AI] `git grep -lic baseerah apps/beaver-nest-fe/` returns exactly the three Decision-12
      GitHub-URL files (`Dockerfile`, `src/components/AppShell.tsx`, `src/app/page.test.tsx`) and no
      others.
- [ ] [AI] `npx nx run beaver-nest-fe:test:quick` exits 0.
- [ ] [AI] `grep -c "wawasan" apps/beaver-nest-fe/src/app/layout.tsx` returns `0`.
- [ ] [AI] `grep -c "DWT-004" apps/beaver-nest-fe/src/components/AppShell.tsx` returns `0`.

> **Pause Safety**: `beaver-nest-fe` is fully renamed, the brand-chip feature is removed end to end
> (component, unit test, Gherkin scenario, one step definition), and the FE `specs:behavior:coverage` RED
> from Phase 6 is now resolved. Safe to stop. To resume: confirm level with `origin/main`, then start
> Phase 11.

---

## Phase 11: `beaver-nest-fe-e2e`

- [ ] [AI] Rename the app directory: `git mv apps/baseerah-fe-e2e apps/beaver-nest-fe-e2e` —
      acceptance: `test -d apps/beaver-nest-fe-e2e` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to every file under `apps/beaver-nest-fe-e2e/` — acceptance:
      `git grep -lic baseerah apps/beaver-nest-fe-e2e/` returns no matches after the steps below.
- [ ] [AI] GREEN: `apps/beaver-nest-fe-e2e/steps/landing.steps.ts` is the E2E **aggregate BDD
      binder** for the entire feature file (its own header comment states "Covers:
      specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature"; and
      `repo-config.yml`'s `coverage.projects` registry points both `beaver-nest-fe` and
      `beaver-nest-fe-e2e` at the identical specs glob — but at different `levels` ([unit] vs.
      [e2e]), so the two projects are checked by two DIFFERENT mechanisms, not the same one: see the
      GREEN acceptance below). Add real Playwright step implementations for the four new Gherkin
      lines Phase 10 authored ("The homepage no longer renders a brand-chip etymology gloss"):

      ```typescript
      Given("a first-time visitor viewing the rendered homepage", async ({ page }) => {
        await page.goto("/");
      });

      When("they inspect the page for a hoverable multilingual term chip", async () => {
        // No-op: the assertion below directly queries for the chip's absence.
      });

      Then("no بصيرة/wawasan-style etymology chip is present", async ({ page }) => {
        await expect(page.getByTitle(/insight/i)).toHaveCount(0);
      });

      And("no automated test or Gherkin scenario asserts one exists", async () => {
        // No-op: the scenario's own presence as a negative check satisfies this clause.
      });
      ```

      — acceptance: run `npx bddgen && npx nx run beaver-nest-fe-e2e:specs:e2e:coverage` —
      `[Repo-grounded]`: confirmed against `apps/baseerah-fe-e2e/project.json` that this project's
      real coverage signal is `specs:e2e:coverage` (the bddgen-generated-spec `test.fixme`/baseline
      diff check in `apps/rhino-cli/src/application/e2e_coverage/`), not `specs:behavior:coverage` —
      that target is a documented no-op stub for this project (`"echo 'no-op: target not applicable
      for this project'"`), consistent with its `levels: [e2e]` entry in `repo-config.yml` (unlike
      `beaver-nest-fe`'s `levels: [unit]` entry, whose `specs:behavior:coverage` target is the real
      orphan-step-checking one used in Phase 10 above). Expect `specs:e2e:coverage` to exit 0 with
      `0 new unbound scenario(s) beyond baseline`, confirming the four new steps correctly bind the
      new Gherkin scenario (bddgen generates no `test.fixme` for it). The three now-orphaned step
      implementations from the scenario Phase 6 deleted remain as harmless dead code at this
      point — no Nx target for this project detects orphaned (unmatched) step implementations; their
      removal is verified directly by `grep` in the REFACTOR step immediately below, not by a
      coverage target.

- [ ] [AI] REFACTOR: edit `apps/beaver-nest-fe-e2e/steps/landing.steps.ts`: delete all three
      now-orphaned step implementations belonging to the scenario Phase 6 deleted —
      `Given("a first-time visitor viewing the homepage brand chip", ...)`,
      `When('they read or hover the "بصيرة" and "wawasan" terms', ...)`, and
      `Then("a plain-language English gloss or tooltip explains what each term means", ...)` (all
      three, not just the `When` line — orphan detection is per-step-text) — plus its one `@covers`
      comment line referencing the deleted scenario (per Decision 3, mirroring Phase 10's FE-side
      removal) — acceptance: `grep -c "they read or hover"
apps/beaver-nest-fe-e2e/steps/landing.steps.ts` returns `0` (confirms the orphaned step is gone;
      note a bare `grep -c "بصيرة"` count is NOT a valid check here — the GREEN step above
      deliberately leaves ONE legitimate بصيرة reference in the surviving `Then("no بصيرة/wawasan-style
      etymology chip is present", ...)` step, so `grep -c "بصيرة"` correctly returns `1`, not `0`,
      after this REFACTOR step).

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah-fe-e2e to beaver-nest-fe-e2e`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 11 Gate

- [ ] [AI] `git grep -lic baseerah apps/beaver-nest-fe-e2e/` returns no matches.
- [ ] [AI] `npx bddgen && npx nx run beaver-nest-fe-e2e:specs:e2e:coverage` exits 0 (this project's
      real coverage signal — `specs:behavior:coverage` is a no-op stub here, see the GREEN step
      above).
- [ ] [AI] `grep -c "they read or hover" apps/beaver-nest-fe-e2e/steps/landing.steps.ts` returns `0`,
      and `grep -c "بصيرة" apps/beaver-nest-fe-e2e/steps/landing.steps.ts` returns exactly `1` (the
      one legitimate surviving reference in the new scenario's `Then` step — not `0`, since that step
      is meant to stay).

> **Pause Safety**: every application is now fully renamed with the brand-chip feature removed on
> both the FE unit and E2E sides. Safe to stop. To resume: confirm level with `origin/main`, then
> start Phase 12.

---

## Phase 12: `infra/dev/beaver-nest-app/` and Root npm Scripts

- [ ] [AI] Rename the infra directory: `git mv infra/dev/baseerah-app infra/dev/beaver-nest-app` —
      acceptance: `test -d infra/dev/beaver-nest-app` succeeds.
- [ ] [AI] Apply `<CANONICAL-SED>` to `infra/dev/beaver-nest-app/docker-compose.yml`,
      `infra/dev/beaver-nest-app/docker-compose.ci.yml`, `infra/dev/beaver-nest-app/README.md`,
      **and** `infra/dev/beaver-nest-app/Dockerfile.be.dev` (renames the `baseerah-be`/`baseerah-fe`
      service names, the `BASEERAH_*` env vars, and — in `Dockerfile.be.dev` — the `CMD`'s
      `src/BaseerahBe/BaseerahBe.fsproj` project path to `src/BeaverNestBe/BeaverNestBe.fsproj`,
      matching the F# project rename from Phase 8), preserving the historical citation in
      `README.md` per [tech-docs.md Decision 6](./tech-docs.md#decision-log): (1) capture `git grep
      -l "baseerah-repo-reset" -- infra/dev/beaver-nest-app/ >
local-temp/rebrand-citations-phase12.txt`; (2) run `perl -pi -e '<CANONICAL-SED-BODY>'
      infra/dev/beaver-nest-app/docker-compose.yml infra/dev/beaver-nest-app/docker-compose.ci.yml
      infra/dev/beaver-nest-app/README.md infra/dev/beaver-nest-app/Dockerfile.be.dev`; (3) revert
      the captured file(s)' mangled citation:
      `xargs -a
      local-temp/rebrand-citations-phase12.txt -I{} perl -pi -e
      's/beaver-nest-repo-reset/baseerah-repo-reset/g' {}` — acceptance: `git grep -l
"baseerah-repo-reset" -- infra/dev/beaver-nest-app/ | diff -
      local-temp/rebrand-citations-phase12.txt` reports no differences, and `git grep -lic baseerah
infra/dev/beaver-nest-app/` returns only that same captured file set.
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

- [ ] [AI] `git grep -l "baseerah-repo-reset" -- infra/dev/beaver-nest-app/ | diff -
      local-temp/rebrand-citations-phase12.txt` reports no differences, and `git grep -lic baseerah
infra/dev/beaver-nest-app/` returns only that same captured file set.
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
- [ ] [AI] Delete the two stale auto-created GitHub Environment objects Phase 0 confirmed
      (`baseerah-app-local`, `baseerah-app-staging` — empty `protection_rules`, no secrets, safe to
      remove since the workflows above now reference `beaver-nest-app-local`/`beaver-nest-app-staging`
      instead): run `gh api -X DELETE repos/wahidyankf/baseerah/environments/baseerah-app-local &&
gh api -X DELETE repos/wahidyankf/baseerah/environments/baseerah-app-staging` — acceptance: both
      calls return no error, and `gh api repos/wahidyankf/baseerah/environments --jq
'.environments[].name'` returns empty output (new `beaver-nest-app-*` Environment objects
      auto-create themselves on the next workflow run that references them, per GitHub's own
      behavior — this step doesn't need to pre-create them).

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
- [ ] [AI] `gh api repos/wahidyankf/baseerah/environments --jq '.environments[].name'` returns empty
      output (the two stale `baseerah-app-*` Environment objects are deleted).

> **Pause Safety**: CI workflows and the GHCR image name are fully cut over with no dual-publish
> bridge. Safe to stop. To resume: confirm level with `origin/main`, then start Phase 14.

---

## Phase 14: Agent Fleet and Skills (`.amazonq` Binding Rename Deferred to Phase 15)

> **Note**: the `.amazonq/cli-agents/baseerah-default.json` rename, and its
> `npm run generate:bindings` re-verification, are deferred to Phase 15 rather than done here. That
> generated file's content is produced from `apps/rhino-cli/src/application/agents/bindings.rs`'s
> `AMAZONQ_AGENT_DEFINITION` / `AGENT_DEFINITION_CONTENT` source constants, which are not renamed
> until Phase 15's GREEN step. Running the generator in this phase — before that source rename lands
> — would regenerate the just-`git mv`'d-away `baseerah-default.json` from the still-old constant,
> producing an unavoidable diff. Phase 15 renames the `.amazonq` file and the source constant
> together, in the same phase, so the generator and the manual rename never drift apart even
> momentarily. (brd.md's risk table already documents Phase 15 as the home for this constant.)

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
- [ ] [AI] Hand-edit the brand-chip-specific content guidance the `<CANONICAL-SED>` steps above
      cannot touch (none of it contains the literal string `baseerah`, so no sed pass catches it —
      `[Repo-grounded]`: verified 2026-08-01, this is the complete list): in
      `.claude/agents/apps-beaver-nest-fe-content-maker.md`, drop "brand chip" from the
      `description:` frontmatter's parenthetical content list (Decision 3 deleted the feature), AND
      reword its "Model Selection Justification" section (the "bilingual/trilingual brand moment
      (Arabic/English/Indonesian)" clause in the opening sentence, and the "chip phrasing" sub-item
      in its first bullet) to drop both references to the deleted chip — this is a differently-worded
      spot the "brand chip"/"brand-chip" grep below does not match on its own, which is why it needs
      its own check; in
      `.claude/agents/apps-beaver-nest-fe-content-checker.md` and `...-content-fixer.md`, remove the
      "Understanding of the bilingual/trilingual brand-chip constraint" bullet (and, in the fixer,
      its "and shared-chrome (`AppFrame`) rule" clause — keep the shared-chrome half if it's still
      accurate, drop only the chip half); in `.claude/agents/README.md`'s catalog entry for
      `apps-beaver-nest-fe-content-maker`, drop the "brand chip" item from its parenthetical content
      list (mirrors the content-maker.md description edit above — the sed pass renames
      `baseerah-fe`→`beaver-nest-fe` in this same line but leaves "brand chip" untouched, same blind
      spot); in `.claude/skills/apps-beaver-nest-fe-developing-content/SKILL.md`, drop "brand chip"
      from the `description:` frontmatter, remove "the trilingual brand chip (بصيرة / insight /
      wawasan)" from the file-purpose table row, and remove the "English-first, with deliberate
      bilingual/trilingual moments" bullet describing the chip's Arabic/Indonesian text; in
      `.claude/skills/swe-developing-frontend-ui/reference/brand-context.md`, reword the
      "**Personality**: Clear, insightful, self-owned — بصيرة (insight, inner vision)" line to drop
      the بصيرة etymology per Decision 9 (BeaverNest has no invented meaning) — acceptance: `grep -lc
"brand.chip\|بصيرة\|wawasan"
.claude/agents/apps-beaver-nest-fe-content-maker.md
.claude/agents/apps-beaver-nest-fe-content-checker.md
.claude/agents/apps-beaver-nest-fe-content-fixer.md
.claude/agents/README.md
.claude/skills/apps-beaver-nest-fe-developing-content/SKILL.md
.claude/skills/swe-developing-frontend-ui/reference/brand-context.md` reports zero matches
      across all six files (note the `.` in `brand.chip` matches either a space or a hyphen — the
      content-checker/fixer bullets use "brand-chip", hyphenated, not "brand chip"), and `grep -c
      "brand moment\|chip phrasing" .claude/agents/apps-beaver-nest-fe-content-maker.md` returns `0`
      (this differently-worded pair doesn't match the pattern above, hence the separate check). Then
      run `npm run generate:bindings` to regenerate the `.opencode/agents/` and `.cursor/agents/`
      mirrors of
      the three edited `.claude/agents/*.md` files from this corrected source — the two edited
      `.claude/skills/**` files have no mirror to regenerate (per AGENTS.md's Multi-harness
      configuration section, skills are NOT mirrored; OpenCode reads
      `.claude/skills/{name}/SKILL.md` natively) — acceptance: `git grep -lic "brand.chip"
      .opencode/agents/ .cursor/agents/` returns no matches.

### Local Quality Gates (Before Push)

- [ ] Run `npx nx affected -t typecheck lint test:quick` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah agent fleet and skill directory`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 14 Gate

- [ ] [AI] `git grep -lic baseerah .claude/ .opencode/ .cursor/` returns no matches.
- [ ] [AI] `git grep -lic "brand.chip" .claude/agents/ .claude/skills/apps-beaver-nest-fe-developing-content/
.claude/skills/swe-developing-frontend-ui/reference/brand-context.md .opencode/agents/
.cursor/agents/` returns no matches (the `.` matches either the space or hyphen variant), and
      `grep -c "بصيرة" .claude/skills/swe-developing-frontend-ui/reference/brand-context.md` returns
      `0`.
- [ ] [AI] `grep -c "brand moment\|chip phrasing" .claude/agents/apps-beaver-nest-fe-content-maker.md`
      returns `0`.

> **Pause Safety**: the agent fleet and skill directories are fully renamed and internally
> consistent. The `.amazonq/cli-agents/baseerah-default.json` binding is deliberately left
> unrenamed at this pause point (see the note at the top of this phase) — that is expected, not a
> gap. Safe to stop. To resume: confirm level with `origin/main`, then start Phase 15.

---

## Phase 15: `rhino-cli` Functional Couplings and the `.amazonq` Binding

- [ ] [AI] RED: edit `apps/rhino-cli/tests/agents.rs` — update all seven assertions referencing
      `.amazonq/cli-agents/baseerah-default.json` and/or `"baseerah-default"` to
      `.amazonq/cli-agents/beaver-nest-default.json` and `"beaver-nest-default"`: two fixture-setup
      path-string assertions (~line 698, ~line 731), two path-string assertions inside
      step-function bodies (~line 790, ~line 804), the generated file's `name`-field assertion
      (~line 794), the drift-detection output-string assertion (~line 847), and the `cucumber`
      `#[then(...)]` step-binding literal itself (~line 784). Because `cucumber` matches step text
      verbatim, rename that macro literal IN THE SAME EDIT as the corresponding Gherkin step text in
      `specs/apps/rhino/behavior/rhino-cli/gherkin/harness/agents-bindings.feature:15` (`And the
file .amazonq/cli-agents/baseerah-default.json is written as a valid Amazon Q agent
definition` → `...beaver-nest-default.json...`) — leaving either one unrenamed on its own
      breaks step-binding resolution ("step doesn't match any function") rather than producing the
      intended assertion-level RED failure. This is a pure rename/refactor step (no new behavior),
      exempt from the Gherkin-binds tagging convention per the Feature-Change-Completeness policy;
      the illustrative scenario below documents intent but is not a companion-`.feature` binding —
      the actual scenario this step touches keeps its original title, "Emitting writes the rules
      pointer and the agent definition" (`specs/apps/rhino/behavior/rhino-cli/gherkin/harness/agents-bindings.feature:10-15`),
      with only its step text changing.

                                                                            ```gherkin
                                                                            Scenario: rhino-cli's Amazon Q binding constant points at the renamed file
                                                                              Given apps/rhino-cli's AMAZONQ_AGENT_DEFINITION constant after the rhino-cli rename phase
                                                                              When nx run rhino-cli:test:integration runs
                                                                              Then the test asserting the constant's path value passes against ".amazonq/cli-agents/beaver-nest-default.json"
                                                                              And the generated file's "name" field reads "beaver-nest-default"
                                                                            ```

                                                                            Acceptance: run `npx nx run rhino-cli:test:integration` (or the project's equivalent test
                                                                            target covering `tests/agents.rs`) and confirm the suite runs without a step-binding-mismatch
                                                                            error (the renamed macro literal and the renamed Gherkin step text resolve to each other) but
                                                                            the "Emitting writes the rules pointer and the agent definition" scenario's assertions fail
                                                                            against the still-unrenamed source constant in `bindings.rs` (a deliberate, expected RED
                                                                            state).

- [ ] [AI] GREEN: edit `apps/rhino-cli/src/application/agents/bindings.rs` — rename the
      `AMAZONQ_AGENT_DEFINITION` constant's value to `".amazonq/cli-agents/beaver-nest-default.json"`
      and `AGENT_DEFINITION_CONTENT`'s embedded `"name": "baseerah-default"` to `"name":
"beaver-nest-default"`, and update its own three in-file test assertions to match — acceptance:
      `npx nx run rhino-cli:test:integration` now passes.
- [ ] [AI] Rename the Amazon Q binding file to match the just-renamed constant (deferred here from
      Phase 14 — see the note at the top of Phase 14): `git mv
.amazonq/cli-agents/baseerah-default.json .amazonq/cli-agents/beaver-nest-default.json` and
      edit its `"name"` field to `"beaver-nest-default"` — acceptance: `jq -r .name
.amazonq/cli-agents/beaver-nest-default.json` returns `beaver-nest-default`.
- [ ] [AI] Verify the generator agrees with the manual rename: run `npm run generate:bindings` —
      acceptance: `git status --porcelain` reports no diff beyond what this phase already staged
      (confirms no drift between the hand-renamed `.amazonq` file and what `rhino-cli` regenerates
      from the now-renamed `bindings.rs` constants; any diff here is a defect to fix before the
      phase gate, not a file to blindly accept).
- [ ] [AI] REFACTOR: apply `<CANONICAL-SED>` to
      `apps/rhino-cli/src/application/domain_coverage/mod.rs`,
      `apps/rhino-cli/src/commands/specs_validate_counts.rs`,
      `apps/rhino-cli/src/application/repo_governance/frontmatter_audit.rs`, and
      `apps/rhino-cli/tests/docs.rs`'s self-contained test fixtures (`"baseerah-be"` →
      `"beaver-nest-be"`, `"baseerah"` → `"beaver-nest"`, `"apps/baseerah-fe/content/post.md"` →
      `"apps/beaver-nest-fe/content/post.md"`) — acceptance: `npx nx run rhino-cli:test:unit` exits 0
      (covers the first three, `src/`-embedded fixtures) and `npx nx run rhino-cli:test:integration`
      exits 0 (covers `tests/docs.rs`, which `test:unit`'s explicit `--test` list does not include).
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

- [ ] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — fix ALL failures.

### Commit Guidelines

- [ ] [AI] Commit: `chore(rebrand): rename baseerah references in rhino-cli source, tests, and the
amazonq binding`

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main; monitor CI; fix and re-push on any failure.

### Phase 15 Gate

- [ ] [AI] `npx nx run rhino-cli:test:quick` exits 0.
- [ ] [AI] `npx nx run rhino-cli:test:integration` exits 0 (this is the target that actually
      executes `tests/agents.rs`'s cucumber suite; `test:quick`'s constituent targets do not run it).
- [ ] [AI] `git grep -lic baseerah apps/rhino-cli/ specs/apps/rhino/ .amazonq/ | grep -v
specs_coverage.rs` returns no matches (the one preserved historical citation in
      `specs_coverage.rs` is the sole expected exception).

> **Pause Safety**: `rhino-cli`'s functional couplings to the old name are fully resolved and its own
> test suite is green. Safe to stop. To resume: confirm level with `origin/main`, then start
> Phase 16.

---

## Phase 16: Cross-Cutting Docs, Repo-Wide Residual Sweep, and Verification

- [ ] [AI] Apply `<CANONICAL-SED>` to `apps/README.md` (which references every renamed app by name
      and the deployment-branch prose, and was intentionally deferred until every app rename
      landed), then run the same scoped follow-up revert Phase 7 used for `libs/README.md` to
      restore the one historical citation the catch-all rule just mangled:
      `perl -pi -e 's/beaver-nest-repo-reset/baseerah-repo-reset/g' apps/README.md` — this is safe
      because `beaver-nest-repo-reset` cannot appear anywhere else in the file except as the
      sed-mangled form of the preserved `2026-07-31__baseerah-repo-reset` citation link (per
      Decision 6) — acceptance: `grep -c "baseerah" apps/README.md` returns exactly `1` (the
      preserved citation).
- [ ] [AI] Run the full repo-wide residual check: `git grep -liE "baseerah" -- . ':!plans/done'
':!generated-reports' ':!plans/in-progress/beaver-nest-rebrand'` — this plan's own folder is
      excluded because its narrative prose necessarily discusses the old name it is renaming from;
      it is renamed and moved to `plans/done/` only in the final Plan Archival step, after this
      phase's gate has already passed — acceptance: every remaining match falls into one of TWO
      expected-residual classes: (a) `git grep -l "baseerah-repo-reset" -- . ':!plans/done'
':!generated-reports' ':!plans/in-progress/beaver-nest-rebrand'` (the historical-citation
      preservation rule, [tech-docs.md Decision 6](./tech-docs.md#decision-log) — generalizes across
      however many files legitimately cite the archived plan-id, not a fixed list), or (b) exactly
      the five Decision-12 GitHub-URL files — `CONTRIBUTING.md`,
      `repo-governance/workflows/infra/development-environment-setup.md`,
      `apps/beaver-nest-fe/Dockerfile`, `apps/beaver-nest-fe/src/components/AppShell.tsx`,
      `apps/beaver-nest-fe/src/app/page.test.tsx` (deferred to Phase 17's flip, not a residual to fix
      here). Any match NOT in class (a) or (b) is a defect — fix it and re-run until the condition
      holds.
- [ ] [AI] Run `md links validate` as an independent cross-check the `git grep`-based mechanism
      above cannot fully replace: `cargo run --release --quiet --manifest-path
apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done` — acceptance: exits 0 with
      zero broken-link findings (a broken relative link here would indicate a historical citation
      whose target _shape_ changed without still matching the literal `baseerah-repo-reset`
      substring — a class of breakage the grep-based revert mechanism cannot detect on its own).
- [ ] [AI] Run the full quality gate across every renamed project: `npx nx run-many -t typecheck,
lint,test:quick,specs:behavior:coverage --projects=beaver-nest-be,beaver-nest-be-e2e,beaver-nest-fe,
beaver-nest-fe-e2e,beaver-nest-contracts,rhino-cli` — acceptance: exits 0 for every project.
- [ ] [AI] Run `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` across the whole
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
- [ ] [AI] Verify the greeting endpoint: `curl -s http://localhost:19320/api/v1/hello | jq .` —
      acceptance: 200 status, response body contains "BeaverNest", pasted inline below as `>`
      blockquote lines.

> _(paste curl output for /api/v1/health and /api/v1/hello here during execution)_

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

- [ ] [AI] `git grep -liE "baseerah" -- . ':!plans/done' ':!generated-reports'
':!plans/in-progress/beaver-nest-rebrand'` returns only files falling into one of the two
      expected-residual classes defined in the step above: (a) files also appearing in `git grep -l
"baseerah-repo-reset" -- . ':!plans/done' ':!generated-reports'
':!plans/in-progress/beaver-nest-rebrand'` (Decision 6's historical citations), or (b) exactly
      `CONTRIBUTING.md`, `repo-governance/workflows/infra/development-environment-setup.md`,
      `apps/beaver-nest-fe/Dockerfile`, `apps/beaver-nest-fe/src/components/AppShell.tsx`, and
      `apps/beaver-nest-fe/src/app/page.test.tsx` (Decision 12's preserved GitHub-URL files, deferred
      to Phase 17). Any match outside both (a) and (b) is a defect.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links
validate --exclude plans/done` exits 0.
- [ ] [AI] Every rule-15 and rule-16 defect finding is fixed (ticked) or explicitly deferred with
      recorded user permission.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick,specs:behavior:coverage --all` exits 0.

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
- [ ] [AI] Now that the GitHub rename above is real and the new URL resolves, flip the five
      Decision-12 GitHub-URL citations deferred since Phases 1/2/10: `git grep -l
      "wahidyankf/baseerah" -- . ':!plans/done' ':!generated-reports'
':!plans/in-progress/beaver-nest-rebrand' | xargs perl -pi -e
      's/github\.com\/wahidyankf\/baseerah/github.com\/wahidyankf\/beaver-nest/g'`, then fix the two
      unindented `cd baseerah` lines in `repo-governance/workflows/infra/development-environment-setup.md`:
      `perl -pi -e 's/^cd baseerah$/cd beaver-nest/'
repo-governance/workflows/infra/development-environment-setup.md`, and the one 3-space-indented
      `cd baseerah` line in `CONTRIBUTING.md` (same indentation caveat as Phase 1's revert — use the
      capture group, not a bare anchor): `perl -pi -e 's/^(\s*)cd baseerah$/$1cd beaver-nest/'
CONTRIBUTING.md` — acceptance: `git grep -lic "wahidyankf/baseerah\|^\s*cd baseerah$" -- .
      ':!plans/done' ':!generated-reports' ':!plans/in-progress/beaver-nest-rebrand'` returns no
      matches, and `git grep -c "wahidyankf/beaver-nest" CONTRIBUTING.md
      repo-governance/workflows/infra/development-environment-setup.md apps/beaver-nest-fe/Dockerfile
      apps/beaver-nest-fe/src/components/AppShell.tsx apps/beaver-nest-fe/src/app/page.test.tsx`
      shows the expected count in each (2, 2, 1, 1, 1 respectively, matching Phase 2/10's preserved
      counts).
- [ ] [AI] Commit and push this flip: `chore(rebrand): flip preserved GitHub URL citations now that
the repository rename is live` — monitor CI; fix and re-push on any failure.

### Phase 17 Gate

- [ ] [HUMAN] `gh repo view wahidyankf/beaver-nest --json name --jq .name` returns `beaver-nest`.
- [ ] [AI] `git grep -lic "wahidyankf/baseerah" -- . ':!plans/done' ':!generated-reports'
':!plans/in-progress/beaver-nest-rebrand'` returns no matches.

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
