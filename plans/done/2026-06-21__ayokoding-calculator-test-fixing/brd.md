# Business Requirements — Calculator Test-Fixing

## Business Goal

Bring the AyoKoding cost-of-living calculator to a defensible quality bar by closing the
still-valid correctness, usability, and design findings surfaced by the three-tester
re-validation, so the tool is trustworthy, accessible, and consistent with the rest of the site.

## Business Rationale

The calculator is a flagship interactive tool on `ayokoding.com`. Each open finding erodes user
trust or accessibility:

- A correctness gap (EWT-001: the minimum-role divider vanishes at a zero savings target) makes
  the tool silently misrepresent its own result, the most damaging class of defect for a
  decision-support calculator. [Repo-grounded]
- Accessibility gaps (sub-44 px touch targets, `sr-only` tab descriptions, a missing
  `#tab-desc-cost`, horizontal overflow at 320 px) put the tool below WCAG AA, which the repo
  mandates as a first-class principle. [Repo-grounded: WCAG AA is a stated repo principle in
  `AGENTS.md`]
- Design inconsistency (a bespoke breadcrumb with literal `/` separators instead of the shared
  `ChevronRight` primitive) signals an unpolished product and duplicates maintenance surface.
  [Repo-grounded]

Fixing them together — with companion specs that lock the behaviour — converts a list of ad-hoc
tester observations into durable, regression-protected guarantees.

## Business Impact

- **Pain points addressed**: incorrect result display, accessibility non-compliance, inconsistent
  UI, undiscoverable affordances.
- **Expected benefits**: a correct and accessible calculator; reduced maintenance via primitive
  reuse; regression protection through new Gherkin coverage.

## Affected Roles

This is a solo-maintainer repository; no sign-off ceremonies apply. The maintainer wears:

- **Frontend engineer** — implements the TSX/TS fixes (`swe-typescript-dev`).
- **E2E engineer** — proves runtime behaviour with Playwright (`swe-e2e-dev`).
- **Spec author** — writes companion Gherkin (`specs-maker`).

Consuming agents: `plan-checker` (authoring validation), `plan-execution-checker` (post-execution),
`swe-code-checker` (specs/Gherkin two-path enforcement), the live-site tester triad (rule-15
retest).

## Business-Level Success Metrics

- **Correctness**: the minimum-role divider renders whenever a baseline is set and qualifying rows
  exist, including a zero savings target. _Observable: the existing spec scenario "Zero savings
  target marks the lowest role as the minimum" passes against the implementation._ [Repo-grounded]
- **Accessibility**: all interactive geo-filter controls meet the 44 px minimum target; no
  horizontal scroll at 320 px; every tab has a discoverable, associated description. _Observable:
  Playwright assertions across both locales at 320/375/768/1280 px._ [Judgment call: the 44 px
  target is the repo's established control height, matching the existing `min-h-[44px]` controls.]
- **Consistency**: the calculator breadcrumb uses the shared `Breadcrumb` primitive with
  `ChevronRight`; the final crumb equals the page H1 in both locales. _Observable: unit test
  asserting the rendered component is the shared primitive and the crumb label equals `calcTitle`._
- **Regression protection**: every fixed behaviour and still-relevant proposal has a Gherkin
  scenario; `specs:coverage` exits 0. _Observable: `nx run ayokoding-www:specs:coverage`._
  [Repo-grounded]

## Business-Scope Non-Goals

- Adding new calculator capabilities beyond the listed findings.
- Reworking the dataset, FX table, or tax model.
- Changing deployment or the production branch flow.
- Re-litigating findings the re-validation already marked resolved.

## Business Risks and Mitigations

| Risk                                                           | Mitigation                                                                                |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| A "fix" regresses an already-correct behaviour                 | TDD (RED first) + companion specs; full `specs:coverage` gate each phase.                 |
| A finding rests on a stale/incorrect assumption (e.g. UWT-007) | `tech-docs.md §Assumptions` documents the verified ground truth before any code change.   |
| Breadcrumb consolidation breaks the id-locale label fix        | Phase gate asserts both-locale crumb labels; preserve existing translation keys.          |
| Touch-target change shifts layout and reintroduces overflow    | UWT-008 (overflow) and UWT-016 (touch target) live in the same phase; gate checks 320 px. |
