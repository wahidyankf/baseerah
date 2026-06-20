# Business Requirements — Cost-of-Living Calculator Test-Fixing

## Business goal

Restore correctness, accessibility, localisation, and first-time-user trust in the ayokoding-www
**Cost-of-Living Calculator** so that a relocation-minded software engineer (the tool's primary
audience) can rely on its figures and navigate it without confusion, in both English and Indonesian.

The combined exploratory + usability testing pass confirmed the **core math** (tax bands, FX, OECD
household scaling, savings) is correct. The defects sit _around_ that correct core: accessibility
gaps, localisation gaps, household-scaling display inconsistencies, a viewport-overflow that hides
the single most important datum (the total), and several first-time-user comprehension failures.
This plan fixes those defects without touching the verified math engine.

## Business rationale (WHY this matters)

- **Trust in a numeric tool is binary.** A calculator that displays a negative net for a negative
  salary input (`EWT-005`), or whose visible category columns do not sum to the stated subtotal
  (`EWT-006`/`EWT-007`), reads as broken even when the underlying engine is correct. A first-time
  user cannot distinguish "display bug" from "wrong math" — both destroy confidence.
- **The Indonesian audience is first-class.** ayokoding-www serves the Indonesian tech community as a
  bilingual platform (per the app description). Indonesian pages currently emit `html lang="en"`
  (`EWT-001`/`UWT-006`), ship untranslated strings (`EWT-008`/`EWT-009`/`EWT-010`/`EWT-011`), and
  mislead assistive tech and translation tooling. This is a correctness gap for half the audience.
- **The answer the user came for is off-screen.** At 1280 px the comparison table overflows its
  container and the "Total" / "Essentials" columns are clipped with no scroll affordance
  (`UWT-004`). A user discovers the total only by accidental horizontal scroll. Reordering the
  summary columns leftward (the chosen fix) puts the answer back in the first viewport.
- **First-time comprehension gates adoption.** The cognitive walkthrough (`walkthrough.md`) shows a
  realistic first-time user (Alex, a Jakarta developer evaluating a Singapore move) hitting friction
  at every step: an H1/URL name mismatch (`UWT-002`), weak tab labels (`UWT-012`), undefined finance
  jargon (`UWT-005`), and a (conflict-flagged) perception that two tabs are non-functional
  (`UWT-001`).

## Business impact

### Pain points addressed

- Mislabelled document language harms Indonesian users' assistive-tech and machine-translation
  experience (`EWT-001`/`UWT-006`).
- Display inconsistencies (`EWT-006`/`EWT-007`) and the negative-input bug (`EWT-005`) read as
  broken math and erode trust in the whole tool.
- The hidden total (`UWT-004`) means users miss the primary output and may abandon the comparison.
- Untranslated strings and English-only dropdowns (`EWT-008`–`EWT-011`) leave the Indonesian build
  feeling half-finished.
- Lost filter state on share/bookmark (`EWT-003`/`UWT-003`) blocks the natural "send this comparison
  to a friend" workflow.

### Expected benefits

- A calculator whose visible numbers are internally consistent and whose total is always reachable.
- A fully localised Indonesian experience with a correct language signal.
- Shareable, bookmarkable filtered views via URL state.
- Reduced first-time-user confusion (clear names, defined jargon, predictive tab labels).
- A reconciled, more complete `specs/**` feature file so future regressions are caught by
  `specs:coverage`.

## Affected roles

This is a solo-maintainer repository — there are no sign-off ceremonies. The roles below name the
hats the maintainer wears and the agents that consume each file:

- **Maintainer-as-frontend-engineer** — implements the shell/layout/i18n fixes; consumes `tech-docs.md`
  and `delivery.md`.
- **Maintainer-as-spec-author** — folds `SG-###` + reconciled `USS-###` into the feature file;
  consumes `spec-gaps.md` and `prd.md`.
- **`swe-typescript-dev`** — suggested executor for the TypeScript/TSX shell and layout changes.
- **`apps-ayokoding-www-*` agents** — content/localisation guardians for the ayokoding-www app
  context.
- **`web-exploratory-tester`** — runs the Rule-15 retest round near the end of delivery.
- **`web-usability-tester`** — authored the usability findings `UWT-###` and the cognitive
  walkthrough; co-produced the inputs this plan remediates.
- **`plan-checker` / `plan-execution-checker`** — validate this plan and the executed result.

## Business-level success metrics

- **All 15 exploratory + 14 usability findings resolved or formally voided.** `[Observable]` —
  every `EWT-###`/`UWT-###` maps to a ticked delivery step or a recorded void (the `UWT-001`
  tab-rewrite case).
- **Indonesian pages emit the correct `lang` attribute.** `[Observable]` —
  `document.documentElement.lang === "id"` on `/id/…` routes, asserted by a Gherkin scenario and a
  unit/e2e test.
- **Visible comparison-table columns sum to the stated subtotal under multi-adult households.**
  `[Observable]` — value-bearing test asserts column-sum equals Essentials for a 2-adult household.
- **The total is visible in the initial desktop viewport without horizontal scroll.** `[Observable]`
  — verified at 1280 px in the Rule-15 retest and a responsive deliverable check.
- **`specs:coverage` passes with the reconciled feature file.** `[Observable]` —
  `npx nx run ayokoding-www:specs:coverage` exits 0.
- **First-time-user friction reduced.** `[Judgment call]` — the named comprehension fixes (H1
  subtitle, defined jargon, predictive tab labels) plausibly reduce the walkthrough friction points;
  confirmed qualitatively by the Rule-15 `web-exploratory-tester` round, not by a measured metric.

## Business-scope non-goals

- **Re-deriving or changing the core math.** The tax-band, FX, OECD-scaling, and savings engines are
  verified correct and are out of scope except where a _display_ reads the wrong (unscaled) value.
- **Renaming the URL slug.** Per the locked decision, the `cost-of-living-calculator` slug stays; the
  H1/`<title>` reconciliation is by subtitle + descriptive title, not a slug rename.
- **Adding new cities, roles, or datasets.** Dataset expansion is unrelated to these findings.
- **A full keyboard-only / screen-reader-order audit.** The usability pass explicitly did not cover
  these dimensions; this plan fixes only the specific a11y findings raised (`EWT-001`, `EWT-010`,
  `EWT-011`, `EWT-012`, `UWT-006`, `UWT-008`).
- **Building a `/tools` index page beyond what `UWT-013` requires.** `UWT-013` (parent URL 404) is
  scoped as a minimal index route, not a tools-hub redesign.

## Business risks and mitigations

| Risk                                                                                     | Mitigation                                                                                                          |
| ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Acting on `UWT-001` and breaking working tabs                                            | Mandatory re-verification first; tab-rewrite recorded void if tabs confirmed functional (see README conflict note)  |
| Folding spec-blind `USS-###` suggestions that duplicate or contradict existing scenarios | Spec-aware reconciliation against the existing feature file before folding; drop duplicates, keep only net-new      |
| Column-reorder (`UWT-004`) imposes relearning cost on returning users                    | Reorder keeps all columns, only moves Total + Essentials left after City; documented in the design-funnel rationale |
| Localisation fixes regress the English build                                             | Gherkin scenarios assert both `en` and `id` outcomes; tests run on both locales                                     |
| Display-scaling fix accidentally changes a correct figure                                | Value-bearing tests pin the expected household-adjusted amounts; the core engine is untouched (display reads it)    |
