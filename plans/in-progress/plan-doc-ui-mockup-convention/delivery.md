# Delivery — Plan-Doc UI Mockup Convention

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

Phased execution checklist. This is a governance/docs change — no `apps/`/`libs/` code, so no
specs/Gherkin steps; markdown quality gates apply. This plan is **not UI-bearing** (it ships a
convention + example assets, not app/lib screens), so the new design-funnel checker step it
introduces exempts it.

## Worktree

Worktree path: `worktrees/plan-doc-ui-mockup-convention/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree plan-doc-ui-mockup-convention
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0 — Setup & baseline

- [ ] `[AI]` Enter worktree: navigate to `worktrees/plan-doc-ui-mockup-convention/` (auto-provisioned
      from `origin/main` by plan-execution Step 0 if absent). Verify with `git status --short` —
      output must be empty (clean working tree) before proceeding.
- [ ] `[AI]` Run `npm run lint:md` to confirm a green markdown baseline before edits — exits 0 with
      no errors reported.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] `[AI]` `npm run lint:md` exits 0 — no violations reported.
- [ ] `[AI]` `git status --short` produces no output — working tree is clean.

> **Pause Safety**: Markdown baseline is green and scope is confirmed. Safe to stop.
> To resume: re-run `npm run lint:md` to confirm baseline is still clean.

## Phase 1 — Plan self-validation (plan-quality-gate)

- [ ] `[AI]` Run the [`plan-quality-gate`](../../../repo-governance/workflows/plan/plan-quality-gate.md)
      workflow in **strict** mode, scope `plans/in-progress/plan-doc-ui-mockup-convention/`:
      invoke `plan-checker` → `plan-fixer` and iterate to **two consecutive zero-finding** validations.
      Its Step 5g harness-neutrality scan fires (the plan touches rules/`repo-governance/`),
      confirming integration with current rules. (R9)
- [ ] `[AI]` Apply any `plan-fixer` changes; re-read the plan docs after fixes.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] `[AI]` `plan-quality-gate` workflow returns `pass` — two consecutive zero-finding strict
      validations confirmed — AC10 met.

> **Pause Safety**: Plan is self-validated and compliant with current rules. Safe to stop.
> To resume: re-run `plan-quality-gate` (strict) on this plan's scope.

## Phase 2 — Convention authored & propagated via repo-rules-maker

- [ ] `[AI]` Invoke **`repo-rules-maker`** to author the convention (do not hand-edit a single file).
      Confirm host: extend `repo-governance/conventions/formatting/diagrams.md` with a
      **"UI Mockups in Plan Docs"** section (per [tech-docs.md](./tech-docs.md)); fall back to a new
      `repo-governance/conventions/formatting/ui-mockups-in-plan-docs.md` (_New file — created only
      if diagrams.md is too large_) only if that file is too large. (R1, R8)
- [ ] `[AI]` The section states: the **both-tiers rule** (R1), the **grounding rule** (R5), the
      **design funnel** (R6: ≥2 named low-fi → 2 hi-fi finalists → named selection → rationale), and
      the **prior-art** recommendation (R7, `web-research-maker`) — each with a copy-paste example.
      Acceptance: `grep -c "both-tiers rule" repo-governance/conventions/formatting/diagrams.md`
      returns ≥ 1. (R1)
- [ ] `[AI]` Add the **rendering-support matrix**, the **ruled-out table** (inline HTML+CSS, MDX,
      Mermaid-as-wireframe, `.excalidraw.svg`) with reasons, the GitHub-strips-`style=` fact, and the
      `.png`-over-`.svg` Excalidraw rule. Acceptance:
      `grep -c "rendering-support matrix" repo-governance/conventions/formatting/diagrams.md` ≥ 1
      AND `grep -c "ruled-out" repo-governance/conventions/formatting/diagrams.md` ≥ 1. (R1)
- [ ] `[AI]` **Propagation sweep** (R8): `repo-rules-maker` updates every in-repo rule surface — the
      convention index/README (`repo-governance/conventions/README.md` + formatting index), the
      `repo-rules-checker` register, and any governance-architecture index enumerating conventions —
      then re-sync bindings: `npm run generate:bindings`.
- [ ] `[AI]` Run `repo-rules-checker`; resolve any governance contradiction/inconsistency it reports.
      Acceptance: `repo-rules-checker` reports 0 governance contradictions.
- [ ] `[AI]` Run `npm run lint:md` — exits 0 (markdown lint clean).
- [ ] `[AI]` Run `npx nx run rhino-cli:links:validation` — exits 0 (no broken links/anchors).

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] `[AI]` `grep -c "both-tiers rule" repo-governance/conventions/formatting/diagrams.md` returns
      ≥ 1 — AC4 met.
- [ ] `[AI]` `grep -c "rendering-support matrix" repo-governance/conventions/formatting/diagrams.md`
      returns ≥ 1 — matrix present.
- [ ] `[AI]` `grep -c "ruled-out" repo-governance/conventions/formatting/diagrams.md` returns ≥ 1 —
      ruled-out table present — AC3 met.
- [ ] `[AI]` `npx nx run rhino-cli:links:validation` exits 0 — no broken links — AC5 partially met.
- [ ] `[AI]` `npm run lint:md` exits 0 — no markdown violations — AC5 met.
- [ ] `[AI]` `repo-rules-checker` exits clean — no governance contradictions — bindings synced — AC9
      met.

> **Pause Safety**: Convention document authored and all in-repo rule surfaces propagated. Safe to
> stop. To resume: re-run `repo-rules-checker` and `npm run lint:md` to verify surfaces are still
> clean.

## Phase 3 — Enforcement wiring (plan maker / checker / fixer / workflow)

- [ ] `[AI]` Edit `.claude/skills/plan-creating-project-plans/SKILL.md`: add the design-funnel rule
      for UI-bearing plans and the design-funnel grilling questions (alternatives / prior art /
      selection + why) using standard multiple-choice options. Acceptance:
      `grep -c "UI-design-funnel" .claude/skills/plan-creating-project-plans/SKILL.md` returns ≥ 1.
      (R2)
- [ ] `[AI]` Edit `.claude/agents/plan-maker.md`: add requirement that on a UI-bearing plan, the
      agent requires the funnel artefacts and emits delivery steps that produce them (as it already
      does for specs/Gherkin). Acceptance:
      `grep -c "UI-bearing" .claude/agents/plan-maker.md` returns ≥ 1. (R2)
- [ ] `[AI]` Edit `.claude/agents/plan-checker.md`: add a **UI-design-funnel completeness**
      step (sibling to its specs/Gherkin Step 5j) that FLAGS (HIGH) a UI-bearing plan missing any
      funnel artefact (alternatives, hi-fi finalists, named selection, rationale, grounding/prior-art
      note); exempt pure-refactor / no-UI plans. Acceptance:
      `grep -c "UI-design-funnel" .claude/agents/plan-checker.md` returns ≥ 1. (R2, AC7b)
- [ ] `[AI]` Edit `.claude/agents/plan-fixer.md`: add remediation logic that scaffolds the missing
      funnel sections, re-validating before applying. Acceptance:
      `grep -c "UI-design-funnel" .claude/agents/plan-fixer.md` returns ≥ 1. (R2)
- [ ] `[AI]` Edit `repo-governance/workflows/plan/plan-quality-gate.md`: list the new checker step
      in the validation scope so the gate fails when a UI-bearing plan skips the funnel. Acceptance:
      `grep -c "UI-design-funnel" repo-governance/workflows/plan/plan-quality-gate.md` returns ≥ 1.
      (R2)
- [ ] `[AI]` Run `npm run generate:bindings` to sync `.opencode/` / `.amazonq/` mirrors for the
      changed agents. Exits 0 with no errors.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] `[AI]` `grep -c "UI-design-funnel" .claude/agents/plan-checker.md` returns ≥ 1 — checker step
      added — AC7b met.
- [ ] `[AI]` `grep -c "UI-bearing" .claude/agents/plan-maker.md` returns ≥ 1 — maker requires funnel
      artefacts — AC7 met.
- [ ] `[AI]` `npm run generate:bindings` exits 0 — bindings synced.
- [ ] `[AI]` `npm run lint:md` exits 0 — no violations.

> **Pause Safety**: Enforcement chain wired: maker requires funnel, checker flags gaps, fixer
> scaffolds, workflow lists the step, bindings re-synced. Safe to stop. To resume: verify
> `grep -c "UI-design-funnel" .claude/agents/plan-checker.md` still returns ≥ 1.

## Phase 4 — Worked example (full funnel)

- [ ] `[AI]` **Prior-art research** (R7): invoke `web-research-maker` for how comparable
      salary/cost-of-living or savings calculators present a multi-city comparison; capture cited
      findings to inform the alternatives. Acceptance: `web-research-maker` returns a written summary
      citing ≥2 named prior-art patterns (e.g. tool names, layouts, or interaction models).
- [ ] `[AI]` **Survey existing UI** (R5): read `libs/web-ui` components/tokens (+ Storybook) and
      `apps/ayokoding-www` pages/theme/i18n shell; note reusable components and any net-new primitive
      (e.g. `Table`). Acceptance: a component-inventory note lists ≥3 reusable `libs/web-ui`
      components and names any net-new primitive.
- [ ] `[AI]` Add the **full funnel** for the compare-all screen to
      `plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md`: ≥2 named low-fi ASCII
      alternatives, 2 hi-fi `.excalidraw.png` finalists, a **named** selection, and a rationale —
      reusing the surveyed design system and citing prior art. Acceptance:
      `grep -c "Selected:" plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md`
      returns ≥ 1. (R3)
- [ ] `[HUMAN]` Open `plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md` in VSCode
      Markdown Preview (Ctrl+Shift+V). Confirm: (a) each low-fi wireframe code block renders as a
      monospace block with correct spacing; (b) both hi-fi `.excalidraw.png` images render as images
      (not broken). After push, confirm the same file renders correctly at the GitHub.com URL. (AC6)
      Observable resume signal: GitHub-rendered view shows both images and monospace blocks without
      errors. Verify by visiting the file URL on GitHub.com after push.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] `[AI]` `grep -c "Selected:" plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md`
      returns ≥ 1 — full funnel with named selection present — AC6 partially met.
- [ ] `[AI]` `grep -c "excalidraw.png" plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md`
      returns ≥ 2 — two hi-fi finalists present.
- [ ] `[HUMAN]` Confirm visual rendering in VSCode preview and GitHub — AC6 fully met.

> **Pause Safety**: Full design funnel exemplar is present in the salary-savings-calculator plan.
> Safe to stop. To resume: check
> `grep -c "Selected:" plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md` returns ≥ 1.

## Phase 5 — Cross-repo parallel plans

The convention is adopted across **all three sibling repos** (ose-public, ose-infra, ose-primer) via
**parallel `plan-doc-ui-mockup-convention` plans** that carry the same convention text,
rendering-matrix, ruled-out table, funnel, and enforcement — differing only in grounding references
(each repo's own UI lib: ose-public uses `libs/web-ui`; ose-infra and ose-primer use `libs/ts-ui` +
`libs/ts-ui-tokens`) and the worked-example exemplar. ose-primer self-adopts via its own plan,
pushed directly to its `origin main` (an explicit owner decision overriding the usual ose-primer
PR-only rule).

- [ ] `[AI]` Create the parallel `plan-doc-ui-mockup-convention` plan in the `ose-infra` repo
      (mirror structure: `brd.md`, `prd.md`, `tech-docs.md`, `delivery.md`), grounding references in
      ose-infra's `libs/ts-ui` and `libs/ts-ui-tokens`. Acceptance: plan folder exists at
      `ose-infra:plans/in-progress/plan-doc-ui-mockup-convention/`.
- [ ] `[AI]` Create the parallel `plan-doc-ui-mockup-convention` plan in the `ose-primer` repo,
      grounding references in `libs/ts-ui` and `libs/ts-ui-tokens`. Push directly to
      `ose-primer:origin/main` (owner-decision override of PR-only rule). Acceptance: plan folder
      exists at `ose-primer:plans/in-progress/plan-doc-ui-mockup-convention/`.
- [ ] `[AI]` Run `plan-quality-gate` (strict) on the ose-infra parallel plan — reaches two
      consecutive zero-finding validations. Acceptance: `plan-quality-gate` returns `pass`.
- [ ] `[AI]` Run `plan-quality-gate` (strict) on the ose-primer parallel plan — reaches two
      consecutive zero-finding validations. Acceptance: `plan-quality-gate` returns `pass`.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] `[HUMAN]` Parallel plan folder exists in ose-infra — verify with
      `ls <local-ose-infra-clone>/plans/in-progress/plan-doc-ui-mockup-convention/` and confirm the
      folder contains `brd.md`, `prd.md`, `tech-docs.md`, `delivery.md` — AC8 partially met.
- [ ] `[HUMAN]` Parallel plan folder exists in ose-primer (pushed to origin main) — verify with
      `gh api repos/wahidyankf/ose-primer/contents/plans/in-progress/plan-doc-ui-mockup-convention --silent`
      returning HTTP 200 — AC8 partially met.
- [ ] `[AI]` Both parallel plans pass `plan-quality-gate` strict — AC8 fully met.

> **Pause Safety**: Parallel plans exist in all three sibling repos and pass quality gates. Safe to
> stop. To resume: re-run `plan-quality-gate` (strict) on the ose-infra and ose-primer plan folders.

## Phase 6 — Quality gates & archival

### Local Quality Gates (Before Push)

- [ ] `[AI]` Run `npm run lint:md` across all changed Markdown — exits 0 with no errors.
- [ ] `[AI]` Run `npx nx run rhino-cli:links:validation` — exits 0, no broken links.
- [ ] `[AI]` Run `npx nx run rhino-cli:mermaid:validation` — exits 0, no Mermaid violations.
- [ ] `[AI]` Run `npm run validate:sync` — exits 0, agent/binding parity confirmed.
- [ ] `[AI]` Re-run `repo-rules-checker` and the `plan-quality-gate` workflow (strict) once more after
      all edits; resolve any finding.
- [ ] `[HUMAN]` Review the convention wording, examples, and enforcement wiring. Observable resume
      signal: reviewer has approved; verify by confirming this step is explicitly ticked by the human.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work.

### Commit Guidelines

- [ ] `[AI]` Commit changes thematically — group related changes into logically cohesive commits,
      only when the user asks.
- [ ] `[AI]` Follow Conventional Commits format: `<type>(<scope>): <description>` — split by concern
      (`docs(governance):` for the convention, `feat(governance):` for the agent/workflow
      enforcement).
- [ ] `[AI]` Do NOT bundle unrelated fixes into a single commit.

### Post-Push Verification

- [ ] `[AI]` After push to `main`, monitor GitHub Actions workflows; verify relevant CI
      (markdown-validate, validate:sync) passes — fix any failure at root cause.
- [ ] `[AI]` Move plan to `plans/done/YYYY-MM-DD__plan-doc-ui-mockup-convention/` via `git mv`;
      update the in-progress and done index READMEs.

### Phase 6 Gate

> All checks below must pass before archiving.

- [ ] `[AI]` `npm run lint:md` exits 0 — no violations.
- [ ] `[AI]` CI (markdown-validate, validate:sync) is green on GitHub Actions.
- [ ] `[AI]` All acceptance-criteria scenarios in `prd.md` verified — plan archived in `plans/done/`.

> **Pause Safety**: All quality gates green, convention live, enforcement wired, exemplar present,
> parallel plans adopted, plan archived. Safe to stop. To resume: verify CI is still green and plan
> folder is in `plans/done/`.
