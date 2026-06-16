# Delivery — Plan-Doc UI Mockup Convention

Phased execution checklist. This is a governance/docs change — no `apps/`/`libs/` code, so no
specs/Gherkin steps; markdown quality gates apply. This plan is **not UI-bearing** (it ships a
convention + example assets, not app/lib screens), so the new design-funnel checker step it
introduces exempts it. Steps tagged `[AI]` (agent-executable) or `[HUMAN]` (needs a person). Gate
each phase before moving on.

## Phase 0 — Setup & baseline

- [ ] `[AI]` Confirm working tree clean; create branch/worktree if the user wants isolation
      (`worktrees/plan-doc-ui-mockup-convention/`).
- [ ] `[AI]` Run `npm run lint:md` to confirm a green markdown baseline before edits.
- [ ] **Gate**: baseline green, scope confirmed.

## Phase 1 — Plan self-validation (plan-quality-gate)

- [ ] `[AI]` Run the [`plan-quality-gate`](../../../repo-governance/workflows/plan/plan-quality-gate.md)
      workflow in **strict** mode, scope `plans/in-progress/plan-doc-ui-mockup-convention/`:
      invoke `plan-checker` → `plan-fixer` and iterate to **two consecutive zero-finding** validations.
      Its Step 5g harness-neutrality scan fires (the plan touches rules/`repo-governance/`),
      confirming integration with current rules. (R9)
- [ ] `[AI]` Apply any `plan-fixer` changes; re-read the plan docs after fixes.
- [ ] **Gate**: `plan-quality-gate` returns `pass` (double-zero, strict) → AC10 met.

## Phase 2 — Convention authored & propagated via repo-rules-maker

- [ ] `[AI]` Invoke **`repo-rules-maker`** to author the convention (do not hand-edit a single file).
      Confirm host: extend `repo-governance/conventions/formatting/diagrams.md` with a
      **"UI Mockups in Plan Docs"** section (per [tech-docs.md](./tech-docs.md)); fall back to a new
      `repo-governance/conventions/formatting/ui-mockups-in-plan-docs.md` only if that file is too
      large. (R1, R8)
- [ ] `[AI]` The section states: the **both-tiers rule** (R1), the **grounding rule** (R5), the
      **design funnel** (R6: ≥2 named low-fi → 2 hi-fi finalists → named selection → rationale), and
      the **prior-art** recommendation (R7, `web-research-maker`) — each with a copy-paste example. (R1)
- [ ] `[AI]` Add the **rendering-support matrix**, the **ruled-out table** (inline HTML+CSS, MDX,
      Mermaid-as-wireframe, `.excalidraw.svg`) with reasons, the GitHub-strips-`style=` fact, and the
      `.png`-over-`.svg` Excalidraw rule. (R1)
- [ ] `[AI]` **Propagation sweep** (R8): `repo-rules-maker` updates every in-repo rule surface — the
      convention index/README (`repo-governance/conventions/README.md` + formatting index), the
      `repo-rules-checker` register, and any governance-architecture index enumerating conventions —
      then re-sync bindings: `npm run generate:bindings`.
- [ ] `[AI]` Run `repo-rules-checker`; resolve any governance contradiction/inconsistency it reports.
- [ ] `[AI]` Run `npm run lint:md` and link validation; fix any violations.
- [ ] **Gate**: doc renders in GitHub preview (AC1) and VSCode preview (AC2); both-tiers (AC4),
      grounding (AC4b), funnel (AC4c), prior-art (AC4d) stated; `repo-rules-checker` clean + bindings
      synced (AC9); lint green (AC5).

## Phase 3 — Enforcement wiring (plan maker / checker / fixer / workflow)

- [ ] `[AI]` `plan-creating-project-plans` skill: document the design-funnel rule for UI-bearing
      plans; add the design-funnel grilling questions (alternatives / prior art / selection + why)
      using standard multiple-choice options. (R2)
- [ ] `[AI]` `plan-maker` (`.claude/agents/plan-maker.md`): on a UI-bearing plan, require the funnel
      artefacts and emit delivery steps that produce them (as it already does for specs/Gherkin). (R2)
- [ ] `[AI]` `plan-checker` (`.claude/agents/plan-checker.md`): add a **UI-design-funnel completeness**
      step (sibling to its specs/Gherkin Step 5j) that FLAGS (HIGH) a UI-bearing plan missing any
      funnel artefact (alternatives, hi-fi finalists, named selection, rationale, grounding/prior-art
      note); exempt pure-refactor / no-UI plans. (R2, AC7b)
- [ ] `[AI]` `plan-fixer` (`.claude/agents/plan-fixer.md`): remediate the flagged gaps by scaffolding
      the missing funnel sections, re-validating before applying. (R2)
- [ ] `[AI]` `plan-quality-gate` (and the plan-establishment/execution workflows): list the new
      checker step in the validation scope so the gate fails when a UI-bearing plan skips the funnel. (R2)
- [ ] `[AI]` Run `npm run generate:bindings` to sync `.opencode/` / `.amazonq/` mirrors for the
      changed agents.
- [ ] **Gate**: maker requires + emits steps (AC7); checker flags / fixer scaffolds / workflow lists
      the step (AC7b); bindings synced.

## Phase 4 — Worked example (full funnel)

- [ ] `[AI]` **Prior-art research** (R7): invoke `web-research-maker` for how comparable
      salary/cost-of-living or savings calculators present a multi-city comparison; capture cited
      findings to inform the alternatives.
- [ ] `[AI]` **Survey existing UI** (R5): read `libs/web-ui` components/tokens (+ Storybook) and
      `apps/ayokoding-www` pages/theme/i18n shell; note reusable components and any net-new primitive
      (e.g. `Table`).
- [ ] `[AI]` Add the **full funnel** for the compare-all screen to
      `plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md`: ≥2 named low-fi ASCII
      alternatives, 2 hi-fi `.excalidraw.png` finalists, a **named** selection, and a rationale —
      reusing the surveyed design system and citing prior art. (R3)
- [ ] `[AI]` Verify low-fi and both hi-fi finalists render in VSCode preview and (after push) GitHub. (AC6)
- [ ] **Gate**: full funnel present in the salary-savings plan and rendering in both surfaces (AC6).

## Phase 5 — Propagation to ose-primer

- [ ] `[AI]` Run `repo-ose-primer-propagation-maker` in `apply` mode for the new convention content
      (opens a draft PR against `ose-primer:main` — never a direct commit). (R4)
- [ ] `[HUMAN]` Review/merge the `ose-primer` PR.
- [ ] **Gate**: propagation PR exists (AC8).

## Phase 6 — Quality gates & archival

- [ ] `[AI]` Final `npm run lint:md` + links validation across all changed Markdown.
- [ ] `[AI]` Re-run `repo-rules-checker` and the `plan-quality-gate` workflow (strict) once more after
      all edits; resolve any finding.
- [ ] `[HUMAN]` Review the convention wording, examples, and enforcement wiring.
- [ ] `[AI]` Commit per Conventional Commits, splitting by concern (`docs(governance):` for the
      convention, `feat(governance):` for the agent/workflow enforcement), only when the user asks.
- [ ] `[AI]` After push to `main`, verify relevant CI (markdown-validate, validate:sync) passes; fix
      any failure at root cause.
- [ ] `[AI]` Move plan to `plans/done/YYYY-MM-DD__plan-doc-ui-mockup-convention/`; update the
      in-progress and done index READMEs.
- [ ] **Gate**: all ACs satisfied; CI green; plan archived.
