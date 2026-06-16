# BRD — Plan-Doc UI Mockup Convention

## Problem

Plans increasingly carry UI work, but the repo has no agreed way to show a **draft of that UI**
inside the plan `.md` files. Authors improvise — inline HTML, a screenshot, an ASCII sketch — and
the result often renders inconsistently: a mockup that looks right in the VSCode preview can collapse
to an unstyled blob on GitHub.com (and vice versa). Reviewers on GitHub then cannot see what was
designed, defeating the purpose of putting the mockup in the plan.

## Goal

Give plan authors one documented, low-friction way to embed draft UI that renders **the same in both
the VSCode editor and the GitHub.com rendered view**, stays diff-friendly in git, and survives the
repo's Prettier + markdownlint pipeline unchanged.

## Affected Roles

- **Plan authors (human + `plan-maker` agent)** — get a clear staged design process (diverge →
  narrow → select → justify), grounded in the real design system and informed by prior art, instead
  of guessing a single mockup.
- **Plan reviewers (on GitHub PRs and in VSCode)** — see identical mockups in both surfaces, plus the
  alternatives considered and the rationale for the choice.
- **`plan-checker` / `plan-fixer`** — gain a UI-design-funnel completeness check (checker flags a
  UI-bearing plan missing funnel artefacts; fixer scaffolds them) — enforcement, not just guidance.
- **`web-research-maker`** — supplies prior-art research feeding the divergent alternatives.
- **Downstream `ose-primer` consumers** — inherit the same convention via propagation.

## Why Now

The active `ayokoding-www-salary-savings-calculator` plan is a UI-heavy plan with no mockup. It is
the natural first consumer and worked example, so the convention lands while there is a concrete
plan to apply it to.

## Success Criteria

- A convention document states the **both-tiers rule**: every screen in a UI-bearing plan carries a
  low-fidelity ASCII wireframe AND a high-fidelity `.excalidraw.png`, in separate labelled
  subsections — each with a rendering-support matrix and a copy-paste example.
- The convention requires authors to **ground mockups in the existing design system** (survey
  `libs/web-ui`, the target app, sibling screens; reuse real components/tokens; flag net-new
  components) and to consult **prior art** via `web-research-maker` when crafting designs.
- The convention defines the **design funnel**: ≥2 named low-fi alternatives → 2 hi-fi finalists →
  a **named** selection → a **rationale** decision record; no alternative silently discarded.
- The convention explicitly rules out inline HTML+CSS, MDX, and Mermaid-as-wireframe **for plan
  docs**, each with a one-line reason.
- The rules are **enforced** across the plan maker → checker → fixer chain and the
  `plan-quality-gate` workflow: a UI-bearing plan cannot pass the gate without its design funnel.
- The rule is authored/propagated across in-repo surfaces via **`repo-rules-maker`** (with
  `repo-rules-checker` clean), and this plan passes the **`plan-quality-gate`** workflow.
- The salary-savings-calculator `prd.md` carries the **full funnel** (prior art → alternatives →
  finalists → named selection → rationale) rendering in both VSCode and GitHub, as the exemplar.
- The convention is adopted across all three sibling repos (ose-public, ose-infra, ose-primer) via
  **parallel `plan-doc-ui-mockup-convention` plans** in each repo — same convention text, differing
  only in grounding references and worked-example exemplar. ose-primer self-adopts via its own plan
  pushed directly to its `origin main` (explicit owner decision).

## Non-Goals

- A **new** markdown lint rule or bespoke CI gate (enforcement rides the existing plan
  maker/checker/fixer chain + `plan-quality-gate`, not a new linter).
- Any production application UI work.
- A bespoke wireframe tool.

## Risks

| Risk                                                                  | Mitigation                                                                                    |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Authors still reach for inline HTML out of habit                      | Convention leads with the "ruled out + why" table; `plan-maker` nudges the default.           |
| ASCII wireframes drift out of alignment over edits                    | Recommend generators (BareMinimum/Mockdown) and keep wireframes small/low-fidelity.           |
| Excalidraw `.excalidraw.svg` chosen by mistake (GitHub font fallback) | Convention mandates `.excalidraw.png` for GitHub-visible mockups, documents the why.          |
| Binary `.excalidraw.png` adds diff noise                              | Accepted: the diffable structural record lives in the paired Tier-1 ASCII wireframe.          |
| Authors skip one tier (ship only ASCII or only hi-fi)                 | Convention + `plan-maker` state both tiers are required; reviewers check for both.            |
| Mockup invented from scratch, drifts from what the app can render     | Grounding rule (R5): survey `libs/web-ui` + target app UI first; reuse real components.       |
| Funnel checker false-positives on non-UI plans (over-enforcement)     | "UI-bearing" scope mirrors specs/Gherkin binding; pure-refactor/no-UI plans exempt.           |
| Prior-art research balloons plan context                              | Delegate to `web-research-maker` (isolated context); capture only cited, summarised findings. |

## Classification

Governance / documentation change. No `apps/` or `libs/` code touched → **exempt** from the
specs + Gherkin completeness rule. Markdown quality gates (Prettier, markdownlint, links) still apply.
