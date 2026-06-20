# ayokoding-www Cost-of-Living Calculator — Three-Tester Fix Plan

**Status**: Done
**Stage**: done
**Type**: web-UI feature-change (test-fixing)
**App**: `ayokoding-www` — `/[locale]/tools/cost-of-living-calculator`
**Origin**: [web-ux-test-fixing-planning workflow](../../../repo-governance/workflows/web/web-ux-test-fixing-planning.md), run 2026-06-20

## What this is

A single, fix-ready plan consolidating one three-tester pass over the running cost-of-living calculator
(both `en` and `id` locales, viewports 320–1440 px). Three complementary live-site lenses ran
sequentially and their results were folded into one plan with sources kept attributed:

- **Exploratory** (`web-exploratory-tester`, spec-aware) — 5 `EWT-###` correctness/edge/localization defects.
- **Usability** (`web-usability-tester`, spec-blind) — 13 `UWT-###` first-time-user heuristic violations.
- **Design** (`web-design-tester`, design-aware) — 11 `DWT-###` mockup/token/primitive fidelity defects.

**29 findings total.** Several share a single root cause across lenses — see the
[cross-reference note](./findings.md#cross-reference-note) so each is fixed once.

> **This plan is a proposal, not the fix.** It modifies no `apps/` or `libs/` source. The fixes happen
> later via the [Plan Execution workflow](../../../repo-governance/workflows/plan/plan-execution.md), with
> `delivery.md` as the executable checklist. The plan is a **snapshot of the site as tested on
> 2026-06-20** — re-run all three testers if the site changes materially before execution.

## Scope (grilled, locked 2026-06-20)

- **Dual-currency restore (DWT-001): IN SCOPE, full** — every money figure across all three tabs shows
  local currency + USD, restoring the committed-mockup/PRD promise.
- **All 29 findings: IN SCOPE** — Critical through Trivial, including the HSTS prod-config check (EWT-005)
  and the Minor/Trivial responsive nits.

## Top risks

| #   | Risk                                                                                                                                      | Lens                                                                   | Severity      |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- | ------------- |
| 1   | Core dual-currency display (local + USD) absent from cost/savings tables — contradicts the tool's headline PRD feature                    | **[Design]** DWT-001 / [Usability] UWT-009                             | Critical      |
| 2   | H1 "Salary Savings Calculator" contradicts the page title + tab "Cost of Living Calculator" (stale name after the tool was renamed)       | **[Design]** DWT-004 / [Usability] UWT-001                             | Critical      |
| 3   | `id`-locale desktop tables render English city/country names while mobile renders Indonesian — broken localization on the primary surface | **[Exploratory]** EWT-002/003 / [Design] DWT-008/009                   | Major         |
| 4   | Savings & Minimum-role tabs show pre-populated all-negative tables before any input — read as "the tool is broken"                        | **[Usability]** UWT-003/007                                            | Major (sev-4) |
| 5   | Savings gross-salary input and Min-role baseline selector are unstyled browser defaults, not `libs/web-ui` primitives                     | **[Design]** DWT-003/006                                               | Major         |
| 6   | `/{en,id}/tools` index renders raw i18n keys (`toolsPageTitle`)                                                                           | **[Usability]** UWT-004                                                | Major (sev-4) |
| 7   | Mobile cost card omits country; 320 px household controls wrap mid-pair                                                                   | **[Exploratory]** EWT-001 / [Design] DWT-002/011 / [Usability] UWT-010 | Major         |

## Design ground truth

This is a **fidelity-restoration** plan: most fixes bring the running page back into line with already-committed
hi-fi mockups. The design is established — see
[`plans/done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/`](../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets)
(cost-of-living, savings, and min-role tables across desktop/tablet/mobile). The only **net-new** design
elements are the two empty-state prompts (Savings, Minimum-role); lo-fi wireframes for those are in
[`assets/`](./assets). See [tech-docs.md](./tech-docs.md) for the per-cluster design-asset strategy.

## Document map

| File                               | Purpose                                                                         |
| ---------------------------------- | ------------------------------------------------------------------------------- |
| [README.md](./README.md)           | This overview, scope, risk summary                                              |
| [brd.md](./brd.md)                 | Business/user/brand impact, success metrics                                     |
| [prd.md](./prd.md)                 | Personas, user stories, Gherkin acceptance criteria                             |
| [findings.md](./findings.md)       | All 29 findings, source-attributed (EWT / UWT / DWT) + cross-reference note     |
| [walkthrough.md](./walkthrough.md) | Usability cognitive-walkthrough transcript                                      |
| [spec-gaps.md](./spec-gaps.md)     | `SG-###` / `USS-###` / `SG-D-###` Gherkin proposals for `specs/**`              |
| [tech-docs.md](./tech-docs.md)     | Root-cause analysis + fix approach per cluster                                  |
| [delivery.md](./delivery.md)       | TDD-shaped delivery checklist (Phase 0 first; `[AI]`/`[HUMAN]`; Rule-15 retest) |
| [assets/](./assets)                | Design ground-truth pointer + lo-fi wireframes for the net-new empty states     |

## Out of scope

- Translating city names that have **no** Indonesian translation (correct English fallback retained).
- Cross-browser (only Chrome exercised), dark mode, Lighthouse/Core-Web-Vitals (deferred; localhost HTTP).
- Minimum-role "reference role" / "my salary" baseline sources and dual-currency display selector were not
  deeply exercised by the testers; only the segmented-control styling (DWT-006) is in scope here.
