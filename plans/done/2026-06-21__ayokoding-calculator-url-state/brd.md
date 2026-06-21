# Business Requirements — Calculator URL State Reflection

## Business Goal

Make every interaction with the AyoKoding cost-of-living calculator **shareable, bookmarkable, and
back-navigable** by reflecting the full filter state in the URL. A user who configures the
calculator — picks a tab, narrows to a region/country/city, and sets household parameters — should
be able to copy the URL, send it to a friend, or bookmark it, and have the exact same view restored.

## Business Rationale (Why This Exists)

The calculator is a flagship interactive tool on `ayokoding.com`, an educational platform. Its
value is in letting people compare the cost of living across tech-hub cities for **their** household
shape. That value is undercut today because the configured view is ephemeral:

- A user who drills into Singapore, switches to the Savings tab, and bookmarks the page gets sent
  back to the default "Cost of living / all cities" view on return — the configuration is lost
  `[Repo-grounded: handleTabChange never writes the URL; cost-basis controls never serialized]`.
- A user who shares a link expecting to show a friend "Jakarta vs Singapore for a family of four"
  shares a URL that does not encode the household size or the comparison scope.
- Invalid or stale deep links (e.g. an old `?country=Indonesia` link using the full country name
  instead of the `id` country code) silently fail to restore, leaving a confusing blank filter
  `[Repo-grounded: country id is "id", not "Indonesia"]`.

This is a **trust and shareability** problem: an interactive tool that forgets the user's input
feels broken, and a tool whose links do not work erodes confidence in the whole site.

## Business Impact

### Pain points addressed

- **Lost work on navigation** — the browser Back button and bookmarks reset most selections.
  _(qualitative reasoning: documented as UWT-005, a "Major usability problem", in the usability
  tester findings — `plans/backlog/2026-06-21__ayokoding-calculator-usability-findings/findings.md`)_
- **Broken sharing** — shared links do not carry the household/cost-basis configuration, so the
  recipient sees a different calculation. _(Judgment call: derived from the gap between the three
  control groups and the two params currently serialized.)_
- **Confusing deep-link failures** — invalid params leave a stale or empty filter rather than a
  clean default. _(Observable fact: `[DEEPLINK-COUNTRY] ?country=Indonesia (full name): select=""`
  in the exploratory tester log
  `plans/backlog/2026-06-21__ayokoding-calculator-exploratory-findings/evidence/test-log2.txt`.)_

### Expected benefits

- Every calculator view becomes a stable, shareable artifact (a clean URL with only the
  non-default params).
- Browser Back/Forward becomes a meaningful undo/redo across filter states.
- Invalid links degrade gracefully to a sensible canonical state instead of breaking.

## Affected Roles

This is a solo-maintainer repository — no sign-off ceremonies. The hats worn for this change:

- **Frontend engineer** — implements the URL-state refactor (FCIS core + shell glue).
- **QA / tester** — the three live-site advocate agents (`web-exploratory-tester`,
  `web-usability-tester`, `web-design-tester`) whose prior findings this plan absorbs and whose
  retest closes it.
- **Spec author** — reconciles `cost-of-living-calculator.feature` with the new behavior.

Consuming agents: `swe-typescript-dev` (core + shell), `swe-e2e-dev` (Playwright steps),
`specs-maker`/`specs-checker` (Gherkin), `plan-execution-checker` (final validation).

## Business-Level Success Metrics

- **Shareability**: a configured calculator URL, opened in a fresh tab, restores the identical view
  (tab + geo scope + cost-basis controls). _(Observable check: the e2e deep-link-restore test
  passes.)_
- **No silent state loss**: changing any of the nine controls updates the URL within the same
  interaction. _(Observable check: the e2e round-trip test asserts the URL after each change.)_
- **Graceful degradation**: every invalid/out-of-range/contradictory param resolves to a clean
  canonical URL on load. _(Observable check: the sanitize/canonicalize unit + e2e tests pass.)_
- **Zero regressions**: existing calculator behavior (calculations, translations, tab content) is
  unchanged. _(Observable check: the full `cost-of-living-calculator.feature` suite plus
  `nx affected -t test:quick specs:coverage` stay green.)_

These are pass/fail observable checks, not numeric targets — no usage telemetry exists in-repo to
ground a quantitative KPI `[Judgment call]`.

## Business-Scope Non-Goals

- No change to the calculation model, the dataset, or the displayed figures.
- No new persistence layer (no localStorage, no server-side state) — the URL is the only store.
- No new third-party dependency (e.g. `nuqs`) — hand-rolled pure functions only.
- No visual redesign — design-tier findings are owned by a separate plan.

## Business Risks and Mitigations

| Risk                                                                                | Likelihood | Mitigation                                                                                                                                     |
| ----------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Refactor to URL-as-source-of-truth regresses existing calculations or tab content   | Medium     | Full existing `.feature` suite + unit tests run as a gate every phase; TDD ensures behavior is pinned before refactor.                         |
| `router.push` per change creates a deep, hard-to-escape history stack ("Back trap") | Medium     | Add explicit on-page nav escape links (breadcrumb to Home / Tools); documented as an explicit deliverable.                                     |
| Existing deep links break (back-compat)                                             | Low        | Keep the existing `tab`/`country`/`city` param key names; sanitize drops only genuinely invalid values.                                        |
| Canonicalization `replace` loop or flicker on load                                  | Low        | Canonicalize once on mount via `router.replace` (not `push`); unit-test the idempotency of `sanitize` (sanitize(sanitize(x)) === sanitize(x)). |
