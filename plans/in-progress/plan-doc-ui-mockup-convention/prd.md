# PRD — Plan-Doc UI Mockup Convention

## Overview

Define WHAT the convention says and HOW an author represents draft UI in a plan. The supporting
research and citations live in [tech-docs.md](./tech-docs.md).

## Design process — the funnel

A UI-bearing plan does not jump straight to one mockup. After goals are clear (BRD/PRD) and the
existing design system has been surveyed (R5), each screen goes through a **diverge → narrow →
select → justify** funnel, and the plan docs record every stage:

```
Goals clear (BRD/PRD)  +  existing UI surveyed (R5)
        │
        ▼
[1] DIVERGE — Low-fi alternatives        ≥ 2 (aim 3) named ASCII wireframes: Option A / B / C
        │     explore genuinely different layouts, not cosmetic variants
        ▼
[2] NARROW — Hi-fi shortlist             carry the 2 strongest forward as .excalidraw.png mockups
        │     (drop the rest; note in one line why each was dropped)
        ▼
[3] SELECT — Choose + NAME the winner    e.g. "Selected: Option B — Ranked Table"
        │     (more than one may be selected if the screen needs variants)
        ▼
[4] JUSTIFY — Rationale in the plan       why the chosen design won; why the runners-up lost
```

Every stage is visible in the plan: the low-fi alternatives, the hi-fi shortlist, the **named**
selection, and a short **rationale** (a decision record). Nothing is silently discarded — a dropped
alternative gets a one-line reason so the reasoning survives review.

## Core rule — both tiers, separated

Within the funnel, each screen is represented at **two** fidelities, in **separate, labelled
sections** — low-fi for divergence, hi-fi for the shortlist/selection:

1. **Low-fidelity mockup (required)** — ASCII / Unicode **wireframe in a fenced code block**.
   Captures layout, structure, control placement, and flow. Inline in the `.md`, perfectly diffable,
   renders identically in VSCode and GitHub. This is the structural source of truth that reviewers
   comment on line-by-line.
2. **High-fidelity version (required)** — Excalidraw **`.excalidraw.png`** referenced via
   `![](./file.excalidraw.png)` (plain `.png` screenshot as fallback once design is final). Conveys
   real spacing, grouping, color, typography, and visual hierarchy. Renders on GitHub and in VSCode;
   stays re-editable via the embedded scene.

Keep them in distinct subsections (e.g. `### Low-Fidelity Wireframe` and `### High-Fidelity Mockup`)
so the diffable structural intent and the visual intent are each reviewable on their own terms.

```
For each screen in a UI-bearing plan:
│
├─ Low-Fidelity Wireframe   → ASCII / Unicode in a fenced code block        (REQUIRED)
│
└─ High-Fidelity Mockup     → Excalidraw .excalidraw.png via ![](./…)       (REQUIRED)
                              (plain .png screenshot = fallback when final)
```

**Never use in plan docs:** inline HTML+CSS, MDX, Mermaid-as-wireframe, or `.excalidraw.svg`
(see the ruled-out table in [tech-docs.md](./tech-docs.md) for the per-option reason).

## Requirements

### R1 — Convention document

A convention section/document MUST:

- State the **both-tiers rule**: every screen in a UI-bearing plan gets a **low-fidelity wireframe**
  (ASCII/Unicode in a fenced code block) AND a **high-fidelity version** (`.excalidraw.png`), in
  separate labelled subsections — each with a copy-paste example.
- Define the role of each tier (low-fi = structure/flow/diffable; hi-fi = spacing/color/hierarchy).
- Include the rendering-support matrix (VSCode built-in / VSCode + extension / GitHub.com / diffable
  / lint-safe) for every candidate.
- Include a **ruled-out** table (inline HTML+CSS, MDX, Mermaid-as-wireframe, `.excalidraw.svg`) with
  a one-line reason each.
- State the hard fact that **GitHub strips `style=`, `class`, `id`, `<style>`, `<script>`**, so
  inline-CSS mockups do not render on GitHub.
- State that `.excalidraw.png` is required over `.excalidraw.svg` for GitHub-visible mockups
  (Excalidraw custom fonts are blocked by GitHub's CSP on SVG).
- Note the tooling: Excalidraw VSCode extension (`pomdtr.excalidraw-editor`) is needed to **edit**
  but not to **view** `.excalidraw.png`; ASCII needs nothing.
- State the **grounding rule** (R5): before drafting either tier, the author surveys existing UI in
  the related app(s) and lib(s) so mockups reuse the real design system.

### R2 — Enforcement across the plan maker / checker / fixer / workflows

The design rules (both-tiers R1, grounding R5, funnel R6, prior-art R7) are not advisory prose — they
are **enforced** by the same maker → checker → fixer chain that already governs plans, so a UI-bearing
plan cannot pass quality gates without them. A plan is "UI-bearing" when it adds/changes user-facing
screens or components under `apps/` or `libs/` (e.g. `libs/web-ui`).

- **`plan-creating-project-plans` skill** — documents the design-funnel rule as part of plan content
  for UI-bearing plans, and the grilling gates ask the design-funnel questions (which alternatives,
  what prior art, which selection + why) using the standard multiple-choice options.
- **`plan-maker`** — when a plan is UI-bearing, MUST require the funnel artefacts (≥2 named low-fi
  alternatives, 2 hi-fi finalists, a named selection, a rationale, the R5 grounding note, and R7
  prior-art citations) and MUST emit delivery steps that produce them, exactly as it already emits
  specs/Gherkin steps for feature changes.
- **`plan-checker`** — gains a **UI-design-funnel completeness** validation step (sibling to its
  specs/Gherkin Step 5j): for a UI-bearing plan it FLAGS, at HIGH criticality, any missing funnel
  artefact — no alternatives, no hi-fi finalists, an unnamed selection, a missing rationale, or a
  missing grounding/prior-art note. Pure-refactor / no-UI plans are exempt.
- **`plan-fixer`** — remediates the flagged gaps by scaffolding the missing funnel sections
  (alternatives stubs, selection/rationale skeleton) for the author to fill, re-validating before
  applying.
- **`plan-quality-gate`** (and the plan-establishment/execution workflows that compose it) — list the
  new checker step in their validation scope so the gate fails on a UI-bearing plan that skips the
  funnel.

This mirrors the existing **Specs & Gherkin completeness (both paths)** binding: just as app/lib code
never lands without companion Gherkin enforced by `swe-code-checker` + `plan-checker`, a UI-bearing
plan never passes without its design funnel enforced by `plan-checker` + `plan-fixer`.

### R3 — Worked example

- `plans/in-progress/ayokoding-www-salary-savings-calculator/prd.md` MUST gain the **full funnel** for
  the **compare-all** screen as the reference exemplar: prior-art-informed **≥2 named low-fi ASCII
  alternatives** → **2 hi-fi `.excalidraw.png` finalists** → a **named selection** → a **rationale**,
  all reusing the surveyed design system (R5) and rendering correctly in both VSCode and GitHub.
- The **single-city** screen MAY be documented more lightly (at minimum both tiers — one low-fi
  wireframe + one hi-fi mockup); the compare-all screen is the canonical full-funnel demonstration.

### R4 — Propagation

- The convention MUST be propagated to the `ose-primer` downstream template via the standard
  propagation maker (PR, not direct commit).

### R5 — Ground mockups in existing UI

- Before drafting **either** tier, the author MUST read the existing UI / design of the **related
  app(s) and lib(s)**:
  - the shared `libs/web-ui` component kit (component inventory + its Storybook) and its design
    tokens;
  - the target app's existing pages, layout, theme, and locale/i18n shell (e.g. `apps/ayokoding-www`
    for the salary-savings plan);
  - any existing sibling tool/page the new screen should match.
- Mockups MUST reuse components, spacing, color, and patterns that **already exist** in the design
  system rather than inventing them; the `swe-developing-frontend-ui` skill / web-ui kit is the
  reference for tokens and component inventory.
- Any **net-new** component the mockup introduces MUST be called out explicitly (as the salary-savings
  plan already does for the `Table` primitive), so the gap is visible before build.

### R6 — Design funnel (diverge → narrow → select → justify)

The convention MUST require, and `plan-maker` MUST enforce, the staged design process for each
UI-bearing screen:

- **Diverge (low-fi)** — present **≥ 2 (aim for 3) genuinely different** named low-fidelity ASCII
  alternatives (Option A / B / C), not cosmetic variants.
- **Narrow (hi-fi)** — carry the **2 strongest** forward as high-fidelity `.excalidraw.png` mockups;
  give a one-line reason for each low-fi alternative dropped at this gate.
- **Select** — **name** the chosen design explicitly (e.g. "Selected: Option B — Ranked Table").
  More than one may be selected when the screen legitimately needs variants.
- **Justify** — include a short **rationale / decision record** in the plan: why the chosen design
  won and why each runner-up lost (a small table is enough).

The funnel artefacts live in the plan (`prd.md` plus the plan's `assets/`); no alternative is
silently discarded.

### R7 — Prior-art research (web-research-maker)

- When crafting designs (low-fi **and** hi-fi), the author SHOULD consult **prior art** — how
  comparable tools/screens are designed in the wild — via the `web-research-maker` agent, so the
  divergent alternatives are informed rather than invented from a blank page.
- This complements the **internal** grounding rule (R5, the repo's own design system) with an
  **external** pattern survey; cited findings inform the Stage 1 alternatives and the rationale.
- The convention MUST mention this as a recommended input to the funnel's diverge stage.

### R8 — Propagate the rule via repo-rules-maker

- The convention MUST be authored/extended and propagated **through `repo-rules-maker`**, which
  sweeps every in-repo rule surface (the convention doc + its index/README, the
  `repo-rules-checker` register, and any governance-architecture index that enumerates conventions)
  and then re-syncs platform bindings — not by hand-editing only the obvious file.
- `repo-rules-checker` MUST report no governance contradictions/inconsistencies after the sweep.

### R9 — Validate plan integration via plan-quality-gate

- This plan MUST pass the [`plan-quality-gate`](../../../repo-governance/workflows/plan/plan-quality-gate.md)
  workflow (strict mode) — `plan-checker` → `plan-fixer` iterating to two consecutive zero-finding
  validations — confirming the plan is complete, hallucination-free, and **integrated with current
  rules** (its Step 5g harness-neutrality scan fires because the plan touches rules/`repo-governance/`).

## Acceptance Criteria

Because this is a governance/docs change (no `apps/`/`libs/` code), acceptance is verified by review
and the markdown quality gates rather than Gherkin specs.

- **AC1** — Opening the convention doc in **GitHub.com rendered view** shows the ASCII wireframe
  example rendering as a monospace block and the matrix/tables rendering correctly.
- **AC2** — Opening the same doc in the **VSCode built-in Markdown preview** shows identical content.
- **AC3** — The ruled-out table names inline HTML+CSS, MDX, Mermaid-as-wireframe, and
  `.excalidraw.svg`, each with a reason.
- **AC4** — The convention states the both-tiers rule (low-fi wireframe + hi-fi version, separate
  subsections) as mandatory for UI-bearing plans.
- **AC4b** — The convention states the grounding rule (R5): survey existing app/lib UI before
  drafting mockups, reuse the real design system, flag net-new components.
- **AC4c** — The convention states the design funnel (R6): ≥2 named low-fi alternatives → 2 hi-fi
  finalists → named selection → rationale/decision record, no alternative silently dropped.
- **AC4d** — The convention recommends `web-research-maker` prior-art research (R7) as an input to
  the funnel's diverge stage.
- **AC5** — `npm run lint:md` passes on all new/edited Markdown; links validate.
- **AC6** — The salary-savings-calculator `prd.md` shows the full funnel for the compare-all screen:
  ≥2 low-fi ASCII alternatives, 2 hi-fi `.excalidraw.png` finalists, a **named** selection, and a
  rationale — all rendering in both VSCode and GitHub.
- **AC7** — `plan-maker` and the `plan-creating-project-plans` skill require the funnel artefacts and
  emit delivery steps for them on a UI-bearing plan; the grilling gates ask the design-funnel
  questions.
- **AC7b** — `plan-checker` gains a UI-design-funnel completeness step that FLAGS (HIGH) a UI-bearing
  plan missing any funnel artefact; `plan-fixer` can scaffold the missing sections; `plan-quality-gate`
  lists the step in its validation scope. A deliberately incomplete UI-bearing test plan is flagged.
- **AC8** — An `ose-primer` PR carrying the convention exists (propagation done).
- **AC9** — The rule was propagated via `repo-rules-maker` (R8): convention doc + index + the
  `repo-rules-checker` register updated, bindings re-synced, and `repo-rules-checker` reports no
  governance contradictions.
- **AC10** — The [`plan-quality-gate`](../../../repo-governance/workflows/plan/plan-quality-gate.md)
  workflow (strict) reaches two consecutive zero-finding validations on this plan (R9).

## Open Questions (resolved)

- _Inline HTML+CSS on GitHub?_ — No, sanitizer strips it. Resolved: ruled out.
- _Mermaid for wireframes?_ — No wireframe type exists; repo validator caps layout. Resolved: ruled
  out.
- _`.svg` vs `.png` for Excalidraw on GitHub?_ — `.png` (SVG font CSP fallback). Resolved.
- _New convention doc vs extend `diagrams.md`?_ — Decision deferred to tech-docs; default is to add a
  section under the existing formatting/diagrams convention to avoid convention sprawl.
