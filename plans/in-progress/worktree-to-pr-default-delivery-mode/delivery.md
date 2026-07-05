# Delivery Checklist — Worktree-to-PR Default Delivery Mode

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.
>
> **This plan's terminal trunk write is a `[HUMAN]` PR merge** (per the mode below). All
> git-mechanical work — worktree create, branch, commit, push, PR open, worktree remove — is `[AI]`.
> The single irreversible action (clicking Merge) is `[HUMAN]`.

## Worktree

Worktree path: `worktrees/worktree-to-pr-default-delivery-mode/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree worktree-to-pr-default-delivery-mode
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting
the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

> This `## Worktree` section governs the **`ose-public`** worktree. The `ose-primer` and `ose-infra`
> replication phases provision their own worktrees at their own repo roots (see Phases 5 and 6).

## Delivery Mode

**Delivery Mode: `worktree-to-pr`** (the new default this plan establishes — dogfooded here).

- **Work location**: git worktree on a plan branch (`worktree-to-pr-default-delivery-mode`).
- **Integration target**: ONE Pull Request per repo, opened at Phase 0, targeting `main`.
- **Merge authority**: `[AI]` opens the PR, pushes every phase's commits to the PR branch (never to
  `main`), and drives all local + CI gates to GREEN; the terminal **PR merge is `[HUMAN]`**.
- **Three-repo sweep**: three worktrees + three PRs (one per repo), each driven green by `[AI]` and
  merged by the human.

Precedence (mirrors work-branch precedence): invocation argument > this `## Delivery Mode` field >
default (`worktree-to-pr`).

## Delivery Flow

```mermaid
%% Phase progression across the three repos
stateDiagram-v2
  direction LR
  [*] --> Phase0
  Phase0: Phase 0 — baseline + open ose-public PR
  Phase1: Phase 1 — ose-public conventions
  Phase2: Phase 2 — ose-public workflows
  Phase3: Phase 3 — ose-public agents/skill/root + bindings
  Phase4: Phase 4 — ose-public PR green + [HUMAN] merge
  Phase5: Phase 5 — ose-primer replicate + PR + [HUMAN] merge
  Phase6: Phase 6 — ose-infra replicate + PR + [HUMAN] merge
  Phase7: Phase 7 — Knowledge Capture (triage + route learnings)
  Phase0 --> Phase1 --> Phase2 --> Phase3 --> Phase4 --> Phase5 --> Phase6 --> Phase7 --> [*]
```

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_
>
> All commands run from the `ose-public` root unless noted. This phase also opens the single
> `ose-public` PR (per `worktree-to-pr` mechanics — one PR per plan, opened at execution start).

- [ ] [AI] Provision the worktree from latest `origin/main` (from `ose-public` root):
      `git fetch origin && git worktree add -b worktree-to-pr-default-delivery-mode worktrees/worktree-to-pr-default-delivery-mode origin/main`
      — acceptance: `git worktree list` shows `worktrees/worktree-to-pr-default-delivery-mode` on branch `worktree-to-pr-default-delivery-mode`.
- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
- [ ] [AI] Establish the docs/governance baseline in the worktree:
      `npx nx affected -t typecheck lint test:quick specs:coverage` (and `npm run lint:md` if present)
      — acceptance: baseline pass/fail recorded; every preexisting failure documented.
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved.
- [ ] [AI] Open the single draft PR for this plan (from the worktree):
      `gh pr create --draft --base main --head worktree-to-pr-default-delivery-mode --title "docs(governance): worktree-to-pr default delivery mode" --body "Establishes the worktree-to-pr default delivery mode and the four-mode vocabulary. Delivered via this PR (dogfooding). See plans/in-progress/worktree-to-pr-default-delivery-mode/."`
      — acceptance: `gh pr view --json number,isDraft` shows a draft PR number for this branch.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` baseline recorded and every
      preexisting failure resolved (zero unresolved).
- [ ] [AI] `gh pr view --json number,state` returns an open draft PR for
      `worktree-to-pr-default-delivery-mode`.

> **Pause Safety**: only the toolchain was verified, the baseline recorded, and an empty draft PR
> opened — no governance edits exist yet. Safe to stop indefinitely. To resume: re-run the baseline
> command and confirm the draft PR still exists (`gh pr view`).

---

## Phase 1: ose-public — Convention Layer

> All edits in the worktree; commits push to the PR branch, never to `main`.

- [ ] [AI] Edit `repo-governance/conventions/structure/plans.md`: add a `## Delivery Mode` section
      (sibling to the existing `## Worktree` section) defining the four modes
      (`worktree-to-pr` [default], `worktree-to-origin-main`, `main-to-origin-main`, `main-to-pr`),
      each mode's three attributes (work location, integration target, merge authority), and the
      three-tier precedence (invocation argument > plan field > default).
      — acceptance: `grep -c "worktree-to-pr" repo-governance/conventions/structure/plans.md` ≥ 1 and
      all four mode names appear in the file.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Edit `repo-governance/conventions/structure/worktree-path.md`: cross-reference the delivery
      mode (a worktree is used by `worktree-to-pr` and `worktree-to-origin-main`); link to the new
      `## Delivery Mode` section in `plans.md`.
      — acceptance: `grep -c "Delivery Mode" repo-governance/conventions/structure/worktree-path.md` ≥ 1.
  - _Suggested executor: `repo-rules-maker`_

### Local Quality Gates (Before Push)

- [ ] [AI] Fix + verify markdown: `npm run lint:md:fix && npm run lint:md`
      — acceptance: exits 0, no violations.
- [ ] [AI] Validate mermaid/links/headings on changed docs:
      `npx nx run rhino-cli:mermaid:validation && npx nx run rhino-cli:links:validation && npx nx run rhino-cli:headings:hierarchy-validation`
      — acceptance: all three exit 0.
- [ ] [AI] Run affected gates: `npx nx affected -t typecheck lint test:quick specs:coverage`
      — acceptance: exits 0. **Fix ALL failures found — including preexisting issues not caused by
      these changes** (root-cause orientation).

### Commit + Push to PR branch

- [ ] [AI] Commit thematically (Conventional Commits):
      `git commit -m "docs(governance): define delivery-mode vocabulary in plans + worktree-path conventions"`
      — acceptance: commit created on branch `worktree-to-pr-default-delivery-mode`.
- [ ] [AI] Push to the PR branch (NOT `main`): `git push origin worktree-to-pr-default-delivery-mode`
      — acceptance: `gh pr view --json commits` shows the new commit on the PR.

### Post-Push CI Verification (on the PR)

- [ ] [AI] Monitor CI on the PR (poll every ~2 min): `gh pr checks --watch` or
      `gh run list --branch worktree-to-pr-default-delivery-mode`
      — acceptance: all PR checks green; if any fail, fix at root and push a follow-up commit; repeat
      until green. Do NOT proceed while any PR check is red.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `grep -l "worktree-to-pr" repo-governance/conventions/structure/plans.md` returns the file
      and all four mode names are present.
- [ ] [AI] `gh pr checks` shows all checks passing for the PR after the Phase 1 push.

> **Pause Safety**: convention-layer edits are committed and pushed to a green PR; `main` is
> untouched. Safe to stop. To resume: `git -C worktrees/worktree-to-pr-default-delivery-mode status`
> (clean) and `gh pr checks` (green).

---

## Phase 2: ose-public — Workflow Layer

- [ ] [AI] Edit `repo-governance/workflows/plan/plan-execution.md`:
      (a) Step 0 — add delivery-mode selection with the three-tier precedence alongside the existing
      work-branch precedence; (b) Steps 2b/2c — under `worktree-to-pr` the push target is the PR
      branch and CI is monitored on the PR; (c) Step 8 finalization — archival delivered via PR, the
      `[HUMAN]` merge gate, worktree cleanup AFTER merge. Keep the other three modes documented.
      — acceptance: `grep -c "worktree-to-pr" repo-governance/workflows/plan/plan-execution.md` ≥ 1
      and the precedence phrase (invocation > plan > default) appears near Step 0.
  - _Suggested executor: `repo-workflow-maker`_
- [ ] [AI] Edit `repo-governance/development/workflow/trunk-based-development.md`: reconcile the "all
      development on `main`" posture (decision 6) — frame worktree → PR via short-lived plan branches
      as a valid TBD flavor; update `## Default Push and Worktree Execution` so the default is
      short-lived-branch-via-PR while preserving TBD spirit. Honor the maintenance note listing the
      four duplication sites.
      — acceptance: `grep -ci "short-lived" repo-governance/development/workflow/trunk-based-development.md` ≥ 1
      and the doc no longer states direct-push-to-main as the sole default.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Edit `repo-governance/development/workflow/git-push-default.md` and
      `repo-governance/development/workflow/git-push-safety.md`: reconcile push semantics — default
      integration target is a PR branch; direct push remains available via `*-to-origin-main` modes;
      keep force-push/linear-history rules correct for plan branches.
      — acceptance: both files reference the PR-branch default and the `*-to-origin-main` modes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Edit `repo-governance/development/workflow/pr-merge-protocol.md`: document the
      `worktree-to-pr` terminal step — `[AI]` ensures all gates (local + CI) are GREEN; `[HUMAN]`
      merge performs the trunk write.
      — acceptance: `grep -ci "worktree-to-pr" repo-governance/development/workflow/pr-merge-protocol.md` ≥ 1.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Edit `repo-governance/workflows/plan/plan-planning.md`,
      `repo-governance/workflows/plan/plan-quality-gate.md`,
      `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`, and
      `repo-governance/workflows/plan/plan-multi-repo-parity-planning-and-execution.md`: reference
      delivery-mode selection where each touches worktrees/pushing/plan-structure validation.
      — acceptance: `grep -lc "Delivery Mode" repo-governance/workflows/plan/plan-planning.md repo-governance/workflows/plan/plan-quality-gate.md repo-governance/workflows/plan/plan-multi-repo-parity-planning.md repo-governance/workflows/plan/plan-multi-repo-parity-planning-and-execution.md`
      returns all four files.
  - _Suggested executor: `repo-workflow-maker`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npm run lint:md:fix && npm run lint:md` — acceptance: exits 0.
- [ ] [AI] `npx nx run rhino-cli:mermaid:validation && npx nx run rhino-cli:links:validation && npx nx run rhino-cli:headings:hierarchy-validation`
      — acceptance: all exit 0.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` — acceptance: exits 0.
      **Fix ALL failures, including preexisting.**

### Commit + Push to PR branch

- [ ] [AI] Commit thematically:
      `git commit -m "docs(governance): add delivery-mode selection to plan-execution + reconcile TBD/push semantics"`
      — acceptance: commit created on the plan branch.
- [ ] [AI] Push to the PR branch: `git push origin worktree-to-pr-default-delivery-mode`
      — acceptance: PR shows the new commit.

### Post-Push CI Verification (on the PR)

- [ ] [AI] Monitor CI on the PR until green (`gh pr checks --watch`); fix at root + follow-up commit
      if red; repeat until green.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `grep -c "worktree-to-pr" repo-governance/workflows/plan/plan-execution.md` ≥ 1 and Step 0
      documents the three-tier delivery-mode precedence.
- [ ] [AI] All four plan-workflow docs reference `Delivery Mode`; TBD doc reconciled.
- [ ] [AI] `gh pr checks` all green after the Phase 2 push.

> **Pause Safety**: workflow + development-workflow edits committed to a green PR; `main` untouched.
> Safe to stop. To resume: `git -C worktrees/worktree-to-pr-default-delivery-mode status` clean and
> `gh pr checks` green.

---

## Phase 3: ose-public — Agents, Skill, Root Instructions, Bindings

- [ ] [AI] Edit `.claude/skills/plan-creating-project-plans/SKILL.md`: add a `## Delivery Mode`
      requirement + vocabulary + precedence + template (default `worktree-to-pr`), sibling to the
      existing `## Worktree Specification` section.
      — acceptance: `grep -c "Delivery Mode" .claude/skills/plan-creating-project-plans/SKILL.md` ≥ 1.
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Edit `.claude/agents/plan-maker.md`: instruct authoring of the `## Delivery Mode` section
      (default `worktree-to-pr`).
      — acceptance: `grep -c "Delivery Mode" .claude/agents/plan-maker.md` ≥ 1.
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Edit `.claude/agents/plan-checker.md`: validate `## Delivery Mode` presence + valid
      vocabulary (closed enum); flag missing/invalid as a finding.
      — acceptance: `grep -c "Delivery Mode" .claude/agents/plan-checker.md` ≥ 1.
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Edit `.claude/agents/plan-execution-checker.md`: validate delivery matched the declared
      mode (for `worktree-to-pr`: a PR exists and its gates are green).
      — acceptance: `grep -c "Delivery Mode" .claude/agents/plan-execution-checker.md` ≥ 1.
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Edit `.claude/agents/plan-fixer.md`: scaffold a missing `## Delivery Mode` section.
      — acceptance: `grep -c "Delivery Mode" .claude/agents/plan-fixer.md` ≥ 1.
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Edit `AGENTS.md` (Git Workflow section): update the delivery/TBD description to reflect the
      worktree → PR default and name the four modes.
      — acceptance: `grep -c "worktree-to-pr" AGENTS.md` ≥ 1.
- [ ] [AI] Edit `CLAUDE.md`: align the Claude-specific binding text with the worktree → PR default
      (note `CLAUDE.md` imports `AGENTS.md`).
      — acceptance: delivery description in `CLAUDE.md` is consistent with `AGENTS.md` (no stale
      "direct push to main is the default" wording remains).
- [ ] [AI] Re-sync bindings after the `.claude/**` edits: `npm run generate:bindings`
      — acceptance: exits 0 and `git status --porcelain .opencode .amazonq` shows only intended,
      staged regenerated changes (no unexplained drift).

### Local Quality Gates (Before Push)

- [ ] [AI] `npm run lint:md:fix && npm run lint:md` — acceptance: exits 0.
- [ ] [AI] Validate bindings sync is clean: `npm run validate:claude && npm run validate:opencode`
      (or the repo's binding-validation targets) — acceptance: exits 0, no sync drift reported.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` — acceptance: exits 0.
      **Fix ALL failures, including preexisting.**

### Commit + Push to PR branch

- [ ] [AI] Commit thematically (split agent/skill edits from generated bindings if cleaner):
      `git commit -m "docs(agents): require Delivery Mode field in plan agents/skill + root instructions"`
      then `git commit -m "chore(bindings): re-sync .opencode/.amazonq for delivery-mode changes"`
      — acceptance: commits created on the plan branch.
- [ ] [AI] Push to the PR branch: `git push origin worktree-to-pr-default-delivery-mode`
      — acceptance: PR shows the new commits.

### Post-Push CI Verification (on the PR)

- [ ] [AI] Monitor CI on the PR until green; fix at root + follow-up commit if red; repeat.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] All five `.claude/agents/plan-*.md` + the plan-creating SKILL reference `Delivery Mode`;
      `AGENTS.md` references `worktree-to-pr`.
- [ ] [AI] `npm run generate:bindings` leaves the tree clean (`git status --porcelain .opencode .amazonq`
      empty after staging) and binding validation passes.
- [ ] [AI] `gh pr checks` all green after the Phase 3 push.

> **Pause Safety**: all ose-public content edits are committed to a green PR with synced bindings;
> `main` untouched. Safe to stop. To resume: `gh pr checks` green and `git status` clean.

---

## Phase 4: ose-public — Deliver via worktree-to-pr (PR green → [HUMAN] merge)

- [ ] [AI] Final PR sweep — ensure the full diff is coherent and all gates are green:
      `gh pr view --json mergeable,mergeStateStatus,statusCheckRollup` and `gh pr checks`
      — acceptance: `mergeable` is `MERGEABLE`, all checks passing.
- [ ] [AI] Flip the PR from draft to ready for review: `gh pr ready`
      — acceptance: `gh pr view --json isDraft` shows `false`.
- [ ] [HUMAN] Review the PR and click **Merge** in GitHub (the irreversible trunk write).
      — handoff: `[AI]` has driven all gates green and marked the PR ready; the human performs the
      merge. Observable resume signal: `gh pr view --json state` returns `MERGED`.
- [ ] [AI] After merge, remove the ose-public worktree:
      `git worktree remove worktrees/worktree-to-pr-default-delivery-mode`
      — acceptance: `git worktree list` no longer lists the path.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `gh pr view --json state` returns `MERGED` for the ose-public PR.
- [ ] [AI] `git -C /Users/wkf/ose-projects/ose-public fetch origin && git log origin/main --oneline -1`
      shows the merge on `main`; post-merge `main-ci` (if any) is green (`gh run list --branch main -L 1`).
- [ ] [AI] The ose-public worktree is removed.

> **Pause Safety**: ose-public change is fully delivered on `main` via merged PR; worktree cleaned up.
> Safe to stop indefinitely before starting the sibling-repo replication. To resume: begin Phase 5.

---

## Phase 5: ose-primer — Replicate the Change (own worktree + PR)

> Repo root: `/Users/wkf/ose-primer` → `/Users/wkf/ose-projects/ose-primer` [Repo-grounded]. Apply the
> conceptually identical change (not necessarily byte-identical — governance prose is not under the
> rhino-cli byte-identity mandate). Use the merged ose-public files as the canonical reference.

- [ ] [AI] Provision the primer worktree from latest `origin/main` (from the ose-primer root):
      `git -C /Users/wkf/ose-projects/ose-primer fetch origin && git -C /Users/wkf/ose-projects/ose-primer worktree add -b worktree-to-pr-default-delivery-mode worktrees/worktree-to-pr-default-delivery-mode origin/main`
      — acceptance: `git -C /Users/wkf/ose-projects/ose-primer worktree list` shows the path.
- [ ] [AI] Initialize toolchain: `npm install && npm run doctor -- --fix` in the ose-primer root
      — acceptance: both exit 0.
- [ ] [AI] Open the single draft PR for primer:
      `gh pr create --draft --base main --head worktree-to-pr-default-delivery-mode --title "docs(governance): worktree-to-pr default delivery mode" --body "Parity port of the ose-public delivery-mode change."`
      (run from the primer worktree) — acceptance: `gh pr view --json number` returns a PR number.
- [ ] [AI] Apply the identical edits to the primer copies of every file in
      [`tech-docs.md` §Surface Inventory](./tech-docs.md#surface-inventory): the two conventions, the
      four development-workflow docs, the five workflow docs, the five `.claude/agents/plan-*.md` +
      the plan-creating SKILL, and `AGENTS.md` + `CLAUDE.md`.
      — acceptance: `grep -rc "worktree-to-pr" repo-governance AGENTS.md .claude` (from primer worktree)
      returns non-zero matches across the same surfaces as ose-public.
  - _Suggested executor: `repo-rules-maker` (conventions/dev-workflow) + `repo-workflow-maker` (workflows) + `agent-maker` (.claude)_
- [ ] [AI] Re-sync bindings: `npm run generate:bindings`
      — acceptance: exits 0; `git status --porcelain .opencode .amazonq` shows only intended staged drift.

### Local Quality Gates (Before Push)

- [ ] [AI] `npm run lint:md:fix && npm run lint:md` — acceptance: exits 0.
- [ ] [AI] `npx nx run rhino-cli:mermaid:validation && npx nx run rhino-cli:links:validation && npx nx run rhino-cli:headings:hierarchy-validation`
      — acceptance: all exit 0.
- [ ] [AI] `npm run validate:claude && npm run validate:opencode` — acceptance: exits 0.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` — acceptance: exits 0.
      **Fix ALL failures, including preexisting.**

### Commit + Push + CI (on the primer PR)

- [ ] [AI] Commit thematically and push to the PR branch:
      `git commit -m "docs(governance): worktree-to-pr default delivery mode (parity port)"` then
      `git push origin worktree-to-pr-default-delivery-mode`
      — acceptance: primer PR shows the commit.
- [ ] [AI] Monitor CI on the primer PR until green; fix at root + follow-up commit if red; repeat.

### Deliver + Cleanup

- [ ] [AI] Flip to ready: `gh pr ready`; confirm `gh pr checks` all green and
      `gh pr view --json mergeable` is `MERGEABLE`.
- [ ] [HUMAN] Review and click **Merge** on the primer PR.
      — handoff: gates green, PR ready. Resume signal: `gh pr view --json state` returns `MERGED`.
- [ ] [AI] After merge, remove the primer worktree:
      `git -C /Users/wkf/ose-projects/ose-primer worktree remove worktrees/worktree-to-pr-default-delivery-mode`
      — acceptance: the path is no longer listed.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] Primer PR `state` is `MERGED`; the four-mode vocabulary + precedence are present in the
      primer surfaces (`grep` confirms parity with ose-public conceptually).
- [ ] [AI] Post-merge primer `main-ci` (if any) is green; primer worktree removed.

> **Pause Safety**: primer change delivered on primer `main` via merged PR; worktree cleaned up. Safe
> to stop before starting ose-infra. To resume: begin Phase 6.

---

## Phase 6: ose-infra — Replicate the Change (own worktree + PR)

> Repo root: `/Users/wkf/ose-projects/ose-infra` [Repo-grounded]. Private repo, outside the parity
> loop, but carries its own copies. Apply the conceptually identical change; confirm the four-mode
> vocabulary lands intact even if some prose phrasing differs (see `tech-docs.md` open question 3).

- [ ] [AI] Provision the infra worktree from latest `origin/main`:
      `git -C /Users/wkf/ose-projects/ose-infra fetch origin && git -C /Users/wkf/ose-projects/ose-infra worktree add -b worktree-to-pr-default-delivery-mode worktrees/worktree-to-pr-default-delivery-mode origin/main`
      — acceptance: `git -C /Users/wkf/ose-projects/ose-infra worktree list` shows the path.
- [ ] [AI] Initialize toolchain: `npm install && npm run doctor -- --fix` in the ose-infra root
      — acceptance: both exit 0.
- [ ] [AI] Open the single draft PR for infra:
      `gh pr create --draft --base main --head worktree-to-pr-default-delivery-mode --title "docs(governance): worktree-to-pr default delivery mode" --body "Port of the delivery-mode change to the private infra repo."`
      — acceptance: `gh pr view --json number` returns a PR number.
- [ ] [AI] Apply the identical edits to the infra copies of every file in
      [`tech-docs.md` §Surface Inventory](./tech-docs.md#surface-inventory).
      — acceptance: `grep -rc "worktree-to-pr" repo-governance AGENTS.md .claude` (from infra worktree)
      returns non-zero matches across the same surfaces.
  - _Suggested executor: `repo-rules-maker` + `repo-workflow-maker` + `agent-maker`_
- [ ] [AI] Re-sync bindings: `npm run generate:bindings`
      — acceptance: exits 0; only intended staged drift under `.opencode`/`.amazonq`.

### Local Quality Gates (Before Push)

- [ ] [AI] `npm run lint:md:fix && npm run lint:md` — acceptance: exits 0.
- [ ] [AI] `npx nx run rhino-cli:mermaid:validation && npx nx run rhino-cli:links:validation && npx nx run rhino-cli:headings:hierarchy-validation`
      — acceptance: all exit 0.
- [ ] [AI] `npm run validate:claude && npm run validate:opencode` — acceptance: exits 0.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` — acceptance: exits 0.
      **Fix ALL failures, including preexisting.**

### Commit + Push + CI (on the infra PR)

- [ ] [AI] Commit thematically and push to the PR branch:
      `git commit -m "docs(governance): worktree-to-pr default delivery mode (infra port)"` then
      `git push origin worktree-to-pr-default-delivery-mode`
      — acceptance: infra PR shows the commit.
- [ ] [AI] Monitor CI on the infra PR until green; fix at root + follow-up commit if red; repeat.

### Deliver + Cleanup

- [ ] [AI] Flip to ready: `gh pr ready`; confirm `gh pr checks` all green and `mergeable` is `MERGEABLE`.
- [ ] [HUMAN] Review and click **Merge** on the infra PR.
      — handoff: gates green, PR ready. Resume signal: `gh pr view --json state` returns `MERGED`.
- [ ] [AI] After merge, remove the infra worktree:
      `git -C /Users/wkf/ose-projects/ose-infra worktree remove worktrees/worktree-to-pr-default-delivery-mode`
      — acceptance: the path is no longer listed.

### Phase 6 Gate

> All checks below must pass before archival.

- [ ] [AI] Infra PR `state` is `MERGED`; the four-mode vocabulary + precedence present in infra surfaces.
- [ ] [AI] Post-merge infra `main-ci` (if any) is green; infra worktree removed.

> **Pause Safety**: all three repos delivered on their respective `main` via merged PRs; all three
> worktrees cleaned up. Safe to stop. To resume: proceed to Phase 7 (Knowledge Capture).

---

## Phase 7: Knowledge Capture (triage + route learnings)

> The sibling plan `plan-execution-knowledge-capture` executes FIRST and lands the Knowledge Capture
> requirement into the repo before this plan runs. Therefore this plan MUST honor it: triage the
> learnings surfaced during Phases 0–6 and route each through **the Knowledge Capture convention's
> triage rubric**, applying the two safety gates first. This phase is the last substantive phase
> before archival.
>
> **Triage rubric (open-ended, non-exhaustive)**: route each kept learning to the most fitting
> destination named by the convention. Candidate destinations include (illustrative, NOT a fixed
> or exhaustive set): `repo-governance/**`, `docs/**`, `.claude/agents/**`, `.claude/skills/**`,
> `apps/`/`libs/` source code, tests, a post-mortem entry (for failures), a `plans/ideas.md` /
> backlog entry, an inline fix in this plan before archival, cross-session auto-memory, or discard
> (noise / not durable). A learning may legitimately fit a destination not listed here — apply the
> exact rubric and destination names from the Knowledge Capture convention as landed by the sibling
> plan (grep the repo for the convention doc at execution time; do not assume its path or a fixed
> destination count).
>
> **Safety gates (apply BEFORE routing each item)**: (a) **repo-relevance gate** — only capture
> learnings relevant to this repo/estate; drop the rest. (b) **secret/sensitivity gate** — never write
> a system secret or sensitive value into any git-tracked destination (hard iron rule); redact or
> reference an env var instead.

- [ ] [AI] Assemble the raw learnings log from Phases 0–6: preexisting failures fixed during baseline
      or gates, any CI-on-PR surprises, per-repo prose-divergence notes (esp. ose-infra), and any
      binding-sync drift observed. — acceptance: a bullet list of candidate learnings exists in the
      execution notes (not yet routed).
- [ ] [AI] Apply the **repo-relevance gate** to each candidate — mark keep/drop with a one-line reason.
      — acceptance: every candidate carries a keep/drop decision.
- [ ] [AI] Apply the **secret/sensitivity gate** to each kept candidate — confirm no secret/sensitive
      value is carried into any destination; redact or replace with an env-var reference where needed.
      — acceptance: no kept item contains a raw secret; `git diff` of any destination shows no secret.
- [ ] [AI] Route each kept learning to the most fitting destination per the convention's triage
      rubric (open-ended — do not force-fit into a fixed list). For any inline-now item, apply the
      small fix in this plan before archival; for any backlog/idea item, add a `plans/ideas.md` entry
      (or open a backlog plan) rather than expanding this plan's scope.
      — acceptance: each kept learning maps to a named destination; inline-now fixes are committed;
      backlog items appear in `plans/ideas.md`.
  - _Suggested executor: `repo-rules-maker` (governance destination) / `agent-maker` (agent-or-skill destination)_
- [ ] [AI] If any routing produced a governance/agent/skill edit, re-run the relevant local gates
      (`npm run lint:md:fix && npm run lint:md`; `npm run generate:bindings` if `.claude/**` changed)
      and deliver via the same `worktree-to-pr` mode (its own small PR + `[HUMAN]` merge, or fold into
      the appropriate repo's PR if still open). — acceptance: gates green; any such change merged.

### Phase 7 Gate

> All checks below must pass before archival.

- [ ] [AI] Every candidate learning has a keep/drop decision and every kept learning has exactly one
      routed destination (no unrouted learnings remain).
- [ ] [AI] Both safety gates were applied and no secret/sensitive value was written to any git-tracked
      destination (`git log -p` spot-check on any Knowledge-Capture commit is clean).
- [ ] [AI] Any inline-now fixes are committed and merged; any backlog items are recorded in
      `plans/ideas.md`.

> **Pause Safety**: all Phase 0–6 learnings are triaged and routed; no learning is dropped silently
> and no secret leaked. Safe to stop. To resume: proceed to Plan Archival.

---

## Plan Archival

> This plan is docs/governance-only: no UI/API, so no Playwright/curl manual assertions, no `evidence/`
> screenshots, no rule-15/16 tester retests apply. Specs/Gherkin two-path completeness and the
> UI-design-funnel are EXEMPT (see [`prd.md` §Exemption Notes](./prd.md#exemption-notes-read-by-plan-checker)).

- [ ] [AI] Verify ALL delivery checklist items are ticked across Phases 0–7.
- [ ] [AI] Verify ALL quality gates passed (local + CI) in each repo, and all three PRs are `MERGED`.
- [ ] [AI] Confirm the four-mode vocabulary + three-tier precedence are present and consistent in all
      three repos (`grep` spot-check on `plans.md` + `plan-execution.md` + `AGENTS.md`).
- [ ] [AI] Rename and move the plan to done using today's completion date:
      `git mv plans/in-progress/worktree-to-pr-default-delivery-mode plans/done/YYYY-MM-DD__worktree-to-pr-default-delivery-mode`
      — acceptance: the folder now lives under `plans/done/`.
- [ ] [AI] Update `plans/in-progress/README.md` — remove this plan's entry (if present).
- [ ] [AI] Update `plans/done/README.md` — add this plan's entry with the completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g., `plans/README.md`).
- [ ] [AI] Deliver the archival move via the same `worktree-to-pr` mode (its own small PR) OR fold it
      into the ose-public delivery per maintainer preference; commit:
      `chore(plans): move worktree-to-pr-default-delivery-mode to done`.

### Note on the upstream dependency

- [ ] [AI] Confirm this plan honored the Knowledge Capture requirement landed by
      `plan-execution-knowledge-capture` (the sibling plan that executes BEFORE this one): Phase 7 ran,
      all Phase 0–6 learnings were triaged and routed through the convention's open-ended triage rubric
      with both safety gates applied. — acceptance: Phase 7 gate is green and no learning was dropped silently.
