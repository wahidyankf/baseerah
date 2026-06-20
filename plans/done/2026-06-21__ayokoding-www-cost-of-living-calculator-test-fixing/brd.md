# Business Requirements — Cost-of-Living Calculator Fix

## Why this matters

The cost-of-living calculator is a flagship tool on ayokoding-www's `/tools` surface — the section that
demonstrates the site's data-tooling credibility. It targets a high-stakes audience: people comparing
relocation options and salary competitiveness across cities, often a once-in-a-career decision. Polish and
trustworthiness on this surface directly shape whether a first-time visitor believes the numbers.

The three-tester pass found the tool is **functionally strong but presentation-degraded**: the calculation
engine is correct (verified across bands, OECD sub-linear housing, rural/private multipliers, clamping), yet
the rendered surface drops a headline feature (dual currency), contradicts its own name, and degrades the
Indonesian locale and the empty/mobile states.

## Who is affected

- **Indonesian-locale users** (`/id/…`) — the second supported language, explicitly branded ("Kalkulator
  Biaya Hidup"). They see Indonesian UI chrome but English city/country names in the primary desktop tables,
  and an English-labelled mobile nav. The localization promise breaks on the most-used surfaces.
- **First-time visitors (all locales)** — hit comprehension blockers within ~30 seconds: a heading that
  disagrees with the tab/title, all-red pre-populated tables that read as "you can't afford anywhere," fused
  tab labels, and a raw-i18n-key `/tools` landing page.
- **Mobile users** (majority of sessions) — cost cards omit the country; at 320 px the household controls
  wrap so labels detach from their inputs.

## Cost of not fixing

| Open finding                                  | Business cost                                                                                                                                     |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Dual-currency absent (DWT-001)                | The tool's marketed differentiator is missing; users must guess each row's currency, undermining cross-city comparison — the core job-to-be-done. |
| H1 ↔ title mismatch (DWT-004/UWT-001)         | A returning/bookmarking user doubts they're on the right page; brand identity reads as inconsistent.                                              |
| `id` English names (EWT-002/003, DWT-008/009) | Indonesian users distrust a half-translated tool; localization investment is visibly incomplete.                                                  |
| Pre-populated red empty states (UWT-003/007)  | The most damaging trust failure: newcomers conclude the tool is broken or that no city is affordable, and abandon.                                |
| Raw i18n keys on `/tools` (UWT-004)           | Any user landing on the section index sees code strings — signals a broken site before they reach the tool.                                       |
| Unstyled input/select (DWT-003/006)           | Bare browser defaults inside a polished page read as "integration unfinished," lowering perceived quality.                                        |

## Success metrics

- All 29 findings resolved and **re-verified at every breakpoint (320–1440 px) and both locales**.
- Every monetary cell across all three tabs shows local currency **and** USD; no bare-integer money cell.
- `id`-locale desktop tables render Indonesian city/country names where translations exist (English fallback
  only where no translation exists).
- H1, page `<title>`, and the active tab label all name the tool consistently in each locale.
- Savings and Minimum-role tabs show an instructional empty state (no negative figures) until valid input.
- All text inputs/selectors use `libs/web-ui` primitives; `/tools` index renders localized text.
- A near-end **Rule-15 three-tester retest** (per
  [User-Facing Delivery Hardening](../../../repo-governance/development/quality/user-facing-delivery-hardening.md))
  finds no new Critical/Major regressions before archival.

## Non-Goals and Constraints

**Non-goals** (explicitly out of business scope):

- Improving or extending the calculator engine, data model, or city/country dataset.
- Adding new calculator tabs, comparison dimensions, or data sources.
- Redesigning any surface beyond restoring fidelity to the already-committed hi-fi mockups.
- Expanding localization to languages other than `en` and `id`.
- Changes to Vercel deployment configuration beyond the HSTS header verify-and-add step.
- Architectural refactors not required to fix a finding (e.g., moving the whole feature to a new
  directory structure).

**Constraints**:

- Non-destructive scope: the engine/data model is correct and must not regress (existing unit tests
  lock it; new spec scenarios extend coverage without weakening it).
- Localization must fall back to English where an `id` translation is genuinely absent.
- Fixes restore fidelity to the **already-committed** mockups; no net-new screen design except the
  two empty-state prompts.

## Business Risks

| Risk                                                                                                    | Likelihood | Impact                                        | Mitigation                                                                                                                    |
| ------------------------------------------------------------------------------------------------------- | ---------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Fix to locale names (`localeName` propagation) regresses the calculation engine output                  | Low        | Critical (wrong numbers)                      | Engine tests locked before Phase 2; `npx nx run ayokoding-www:test:unit` must pass after every GREEN step                     |
| Locale-fallback applied too broadly breaks English output on `en` locale                                | Low        | High (English users see wrong locale strings) | Phase 3 tests explicitly assert English fallback; `localeName` helper already handles this                                    |
| Dual-currency cell width causes table overflow at 768 px, requiring a visual regression fix post-launch | Medium     | Medium (usability regression on tablet)       | Verify against mockup at 768 px in Phase 10 manual check; constrain column width if needed                                    |
| Middleware lowercase-redirect introduces unintended 308 loops on valid paths                            | Low        | High (site-wide 404 cascade)                  | Unit-test edge cases in `middleware.test.ts`; scope the redirect narrowly to the locale path segment only                     |
| Empty-state hi-fi PNGs not produced before Phase 7 code (HUMAN step 7.0 blocked)                        | Medium     | Medium (Phase 7 gate stalls)                  | Phase 7 gate explicitly blocks code until PNGs are committed; plan notes this as the one residual deferred to human execution |
| Rule-15 retest (Phase 11) finds new Critical findings introduced by fix interactions                    | Low        | High (delays archival)                        | Each phase gate enforces full test suite green; manual verification in Phase 10 catches regressions before retest             |
