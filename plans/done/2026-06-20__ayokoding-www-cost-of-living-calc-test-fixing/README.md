# Cost-of-Living Calculator — Test-Fixing Plan

Fix-ready plan that turns the combined exploratory + usability testing pass against the
ayokoding-www **Cost-of-Living Calculator** into an executable, TDD-shaped remediation.

## Status

- **Stage**: in-progress (authoring complete; ready for execution)
- **Source tool**: `http://localhost:3101/{en,id}/tools/cost-of-living-calculator`
- **App**: `apps/ayokoding-www` (Next.js 16, App Router, TypeScript; functional core / imperative
  shell)
- **Testing date**: 2026-06-20 · breakpoints 375 px / 768 px / 1280 px · locales en + id
- **Findings**: 15 exploratory (`EWT-001..015`) + 14 usability (`UWT-001..014`)
- **Spec proposals**: 7 exploratory spec-gaps (`SG-001..007`) + 6 usability spec-suggestions
  (`USS-001..006`)

## Document map

| File                               | Purpose                                                                                                                       |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| [README.md](./README.md)           | This overview, document map, risk summary                                                                                     |
| [findings.md](./findings.md)       | Raw combined findings (exploratory `EWT-###` + usability `UWT-###`) and the cross-reference note (READ-ONLY: do not renumber) |
| [spec-gaps.md](./spec-gaps.md)     | Proposed Gherkin coverage: exploratory `SG-###` + usability `USS-###` (READ-ONLY: do not renumber)                            |
| [walkthrough.md](./walkthrough.md) | Cognitive-walkthrough transcript behind the usability findings (READ-ONLY)                                                    |
| [brd.md](./brd.md)                 | Business rationale: why these defects matter, affected roles, success metrics, business risks                                 |
| [prd.md](./prd.md)                 | Product requirements: personas, user stories, Gherkin acceptance criteria per finding cluster, scope                          |
| [tech-docs.md](./tech-docs.md)     | Root-cause analysis + chosen fix approach per finding cluster, affected files, mockup references, diagrams                    |
| [delivery.md](./delivery.md)       | Phased TDD delivery checklist (Phase 0 baseline first; RED/GREEN/REFACTOR; specs folding; Rule-15 retest)                     |
| [assets/](./assets/)               | UI-design-funnel mockups for the two changed screens (comparison table, city detail) — low-fi + hi-fi at three breakpoints    |

### Assets inventory

- `assets/ui-comparison-table-low-fi-alternatives.md` — low-fi funnel for the comparison table:
  Option A (reorder summary-first, CHOSEN) vs Option B (sticky summary cols) vs Option C (affordance
  only).
- `assets/ui-comparison-table-option-a-summary-first.{svg,png}` (desktop ~1180w) — chosen layout.
- `assets/ui-comparison-table-option-a-summary-first-tablet.{svg,png}` (~768w).
- `assets/ui-comparison-table-option-a-summary-first-mobile.{svg,png}` (~375w).
- `assets/ui-city-detail-option-a-dual-currency.{svg,png}` (desktop) — household-adjusted rows +
  dual-currency relocation.
- `assets/ui-city-detail-option-a-dual-currency-tablet.{svg,png}` (~768w).
- `assets/ui-city-detail-option-a-dual-currency-mobile.{svg,png}` (~375w).

## What this plan fixes

Every exploratory `EWT-###` and usability `UWT-###` finding is scoped. Shared-root-cause pairs (per
the cross-reference note in `findings.md`) are fixed once:

- **`html lang`** (`EWT-001` ⇄ `UWT-006`) → single locale-aware `<html lang={locale}>` fix.
- **URL ⇄ filter sync** (`EWT-003` ⇄ `UWT-003`) → one bidirectional sync feature.
- **Relocation/Liquidity columns** (`EWT-002` no-USD ⇄ `UWT-005` no-definition) → one pass adding
  dual currency + definition tooltips.
- **Household-scaling** (`EWT-006` table columns ⇄ `EWT-007` city-detail rows) → one shared
  scaling-multiplier fix applied in both shells.

## UWT-001 conflict — read this first

> **`UWT-001` (the "Savings" / "Minimum role" tabs appear non-functional) CONFLICTS with the
> exploratory evidence.** The spec-aware exploratory pass actively _used_ both tabs (entered a salary
> on Savings, exercised the sort button per `EWT-012`/`EWT-014`, tested the Minimum-role divider per
> `SG-006`). A fully non-functional reading is therefore inconsistent with the evidence.
>
> **The plan's FIRST `UWT-001` step is a re-verification, not a tab rewrite.** If re-verification
> shows the tabs work (the expected outcome — likely a spec-blind observation artifact such as a
> lazy panel mount or Radix transition-timing snapshot), the remediation reduces to the
> information-scent label fix (`UWT-012`), and the tab-rewrite (and the conditioned `USS-001`
> "disable + Coming soon" suggestion) is recorded as **void**. Do **not** disable or rewrite working
> tabs.

## Risk summary

Every top risk is labelled with the lens that surfaced it.

| #   | Risk                                                                                                                 | Lens                                 | Mitigation                                                                                                                |
| --- | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| R1  | Acting on `UWT-001` (rewrite/disable the tabs) when the tabs in fact work — would break a working feature            | `[Usability]`                        | First `UWT-001` step is mandatory re-verification before any tab change; tab-rewrite recorded void if tabs confirmed OK   |
| R2  | Indonesian pages mislabelled `lang="en"` — assistive tech and translation tooling misread the page language          | `[Exploratory]` (also `[Usability]`) | Single locale-aware `<html lang={locale}>` fix; Gherkin scenarios assert both `id` and `en`                               |
| R3  | Household-scaling inconsistency (`EWT-006`/`EWT-007`) lets category columns and rows disagree with subtotals         | `[Exploratory]`                      | One shared scaling-multiplier helper applied to both table columns and city-detail rows; value-bearing tests assert sums  |
| R4  | Comparison-table reorder (`UWT-004`) changes the column order users may have learned — relearning friction           | `[Usability]`                        | Reorder is the chosen funnel finalist (Total + Essentials moved left, after City); breakdown columns retained, just later |
| R5  | Negative salary input (`EWT-005`) produces nonsensical negative net — erodes trust in the core math                  | `[Exploratory]`                      | Clamp at the input boundary (`Math.max(0, …)`) with a value-bearing test for the negative-input path                      |
| R6  | URL ⇄ filter desync (`EWT-003`/`UWT-003`) — deep links and shared views silently lose state                          | `[Exploratory]` (also `[Usability]`) | Bidirectional URL ⇄ filter sync implemented once; Gherkin asserts hydrate-from-URL and write-to-URL halves                |
| R7  | Relocation/Liquidity columns (`EWT-002`/`UWT-005`) — undefined jargon and missing USD mislead relocation users       | `[Exploratory]` (also `[Usability]`) | One pass adds USD equivalents and definition tooltips to both columns                                                     |
| R8  | Spec drift if `USS-###` suggestions are folded without spec-aware reconciliation — duplicate/contradictory scenarios | `[Usability]`                        | Reconcile every `USS-###` against the existing feature file first; drop duplicates, keep only net-new (see `delivery.md`) |

See [delivery.md](./delivery.md) for the executable checklist and the mandatory `## Worktree`
section.
