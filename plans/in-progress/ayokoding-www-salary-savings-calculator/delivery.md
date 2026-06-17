# Delivery Checklist — Salary Savings Calculator

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate`: a must-pass verification
> checklist plus a **Pause Safety** note (the safe-to-stop state after the phase and the
> single command to resume). A phase is **not complete until its gate is green**; do not start
> phase N+1 while any check in phase N's gate is failing.

Commands assume repo root `/Users/wkf/ose-projects/ose-public` unless noted. Each code step uses
RED → GREEN → REFACTOR.

## Worktree

Worktree path: `worktrees/ayokoding-www-salary-savings-calculator/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree ayokoding-www-salary-savings-calculator
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0 — Setup & Baseline

- [ ] **[HUMAN]** Create worktree: `git worktree add worktrees/ayokoding-www-salary-savings-calculator -b ayokoding-www-salary-savings-calculator`.
- [ ] **[AI]** In the worktree, install + converge toolchain: `npm install` then `npm run doctor -- --fix`.
- [ ] **[AI]** Establish green baseline for the app and the shared UI lib (Phase 2 adds a `web-ui` primitive): `npx nx run ayokoding-www:test:quick` and `npx nx run web-ui:test:quick`. Acceptance: both pass before any change.
- [ ] **[AI]** Confirm the functional-core / imperative-shell layout in `apps/ayokoding-www/src/features/<name>/{core,shell}/` and the i18n mechanism in `src/features/i18n/core/`. Confirm whether the new `tools/` route should live under the `(app)` route group (`app/[locale]/(app)/tools/cost-of-living-calculator/page.tsx`) or directly under `[locale]` (`app/[locale]/tools/cost-of-living-calculator/page.tsx`). Record both decisions in `tech-docs.md §Risks / Open Questions` if the chosen layout differs from the proposed one.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npx nx run ayokoding-www:test:quick` and `npx nx run web-ui:test:quick` — both exit 0 (green baseline before any change).
- [ ] [AI] `apps/ayokoding-www/src/features/i18n/core/translations.ts` exists — `test -f apps/ayokoding-www/src/features/i18n/core/translations.ts && echo "OK"`.
- [ ] [AI] Feature-folder convention and route-group placement decision recorded in `tech-docs.md`.

> **Pause Safety**: worktree provisioned, toolchain converged, baseline green, conventions confirmed.
> Safe to stop. To resume: `npx nx run ayokoding-www:test:quick` — must still pass before Phase 1.

## Phase 1 — FX Snapshot + City Data (expenses + tax bands + relocation) + Calculation Core (TDD)

The app stores its currency conversion rates in-repo in `fx.ts` (the **single source** for every
conversion: an ISO-4217 → USD-per-unit table + `fxSnapshotDate`). The city dataset stores, per city,
seven modeled monthly expense categories (incl. childcare), a per-pre-school-child childcare median, a
`{ public, private }` per-school-age-child school median, and a **split** one-time relocation block
(sunk costs incl. key money + a liquidity reserve); plus a per-**country** **federal** banded
effective tax model, per-city **sub-national** rates for US/CA/CH, each country's `healthcareModelType`,
and the shared OECD-modified household/area multipliers. **A city's FX-to-USD is derived from `fx.ts`
via its `currency` — there is no standalone `fxToUsd` field on a city.** All figures are
`web-research-maker`-sourced.

- [ ] **[AI]** Source the FX snapshot via `web-research-maker`: an authoritative **ISO-4217 → USD
      value per 1 unit** rate for **every currency** used by any city/country/role in the datasets
      **plus every supported chosen-display currency** (USD itself = 1), with a single `fxSnapshotDate`.
      Record cited findings + the snapshot date in a research note referenced from `fx.ts` comments.
      Acceptance: a rate exists for each currency the rest of the plan will reference; no fabricated
      rates (each cited or documented). - _Suggested executor: `web-research-maker`_
- [ ] **[AI] RED** Add `fx.test.ts` asserting the FX single-source invariants: `fx.ratesUsdPerUnit`
      has a positive-number entry for **every currency referenced by any city/country/role AND every
      supported chosen-display currency**; `USD` maps to `1`; a `fxSnapshotDate` (ISO date) is present;
      and the `fxToUsd`/`cityFxToUsd` helpers read a city's rate from `fx.ts` via `city.currency` (and
      guard a missing currency rather than returning `NaN`). File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/data/fx.test.ts`. Command:
      `npx nx run ayokoding-www:test:unit`. Acceptance: test fails (no `fx.ts` yet).
- [ ] **[AI] GREEN** Add `fx.ts` — the authoritative `FxTable` (`ratesUsdPerUnit` ISO-4217 → USD per 1
      unit + `fxSnapshotDate`) from the FX research step, with sourced-estimate comments; export the
      `fxToUsd(fx, currency)`, `cityFxToUsd(fx, city)`, and `usdToDisplay(fx, usd, displayCurrency)`
      helpers used by `calc.ts`/`role-lookup.ts`. File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/data/fx.ts`. Acceptance:
      `fx.test.ts` passes. - _Suggested executor: `swe-typescript-dev`_
- [ ] **[AI]** Source the city data via `web-research-maker`: (a) per city, the seven monthly expense
      categories (housing, food, transport-as-transit-pass, utilities, **healthcare as out-of-pocket
      only**, **childcare per pre-school child**, lifestyle) in local currency, a `{ public, private }`
      per-school-age-child school median, a per-pre-school-child `childcareMedianLocal`, and the **split**
      one-time relocation components — **sunk costs** (housing deposit ≈1–3× rent, **key money**
      non-refundable e.g. Japan reikin ≈1–2× rent / 0 where N/A, moving/shipping, visa/admin
      cross-border) and a **liquidity reserve** (cash cushion ≈3–6× essentials, shown separately as a
      reserve the user keeps); (b) per country, a **federal** effective (income tax + mandatory
      contributions) rate at `low`/`mid`/`high` monthly-gross-USD bands, a `healthcareModelType`
      (`oop` | `tax-funded` | `mixed`), **plus the `compulsoryInsurance` flags (`health`,
      `socialSecurity`, optional `note`)**; (c) for **federal/multi-jurisdiction countries (US states,
      Canada provinces, Switzerland cantons)**, a per-city **`subNational` banded effective rate** added
      on top of federal; (d) each city's ISO-4217 `currency` (its USD rate is **derived from `fx.ts`**,
      not stored on the city — ensure every city's `currency` has an `fx.ts` entry). Each value carries a `confidence` tier
      (`high` | `moderate` | `proxy`) and a source note; record cited findings + a `snapshotDate` in a
      research note referenced from `cities.ts` comments. For `tax-funded`/`mixed` countries the
      `healthcare` expense models **only out-of-pocket** costs (mandatory premiums already sit inside
      `effectiveRate`) to avoid double-counting. Acceptance: every city has all seven categories +
      childcare + school + a split relocation block + a resolvable country with federal banded rates +
      `healthcareModelType` + `compulsoryInsurance`, every US/CA/CH city carries `subNational`, no
      fabricated exact figures (gaps documented as `proxy` derivations). - _Suggested executor: `web-research-maker`_
- [ ] **[AI] RED** Add `cities.test.ts` asserting dataset invariants: every city has all seven expense
      categories (`housing`/`food`/`transport`/`utilities`/`healthcare`/`childcare`/`lifestyle`), a
      `childcareMedianLocal`, a `schoolMedianLocal.{public,private}`, a full split `relocation` block
      (`sunkCosts.{deposit,keyMoney,moving,visaAdmin}` + `liquidityReserve.cashCushion`), a `countryId`
      that resolves to a `country`, and an ISO `currency` that **resolves to an entry in `fx.ts`** (the
      city carries **no** standalone `fxToUsd` field); **every city in a US/CA/CH
      country carries `subNational` with banded `effectiveRate`, and unitary-country cities may omit
      it**; every `country` has banded `effectiveRate.{low,mid,high}` with valid `confidence`, a
      `healthcareModelType` of `oop`/`tax-funded`/`mixed`, **and a `compulsoryInsurance` field with
      boolean `health` and `socialSecurity` flags**; dataset has a `snapshotDate`; and **no Israeli
      city / `ILS` currency / Israel country** is present; also assert **at least one city each from
      ASEAN, Japan, Europe (non-Nordic), and the Nordics** via the `region` field. File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/data/cities.test.ts`. Command:
      `npx nx run ayokoding-www:test:unit`. Acceptance: test fails (no dataset yet).
- [ ] **[AI] GREEN** Add `cities.ts` static dataset covering **as many tech-hub cities worldwide as we
      reasonably can** (breadth-first, excl. Israel): per-city seven expense categories (incl.
      childcare), childcare + school medians, split `relocation` block, `countryId`, `currency`
      (USD rate derived from `fx.ts`, not stored on the city), `region`, `subNational` for US/CA/CH
      cities, and sourced-estimate comments; the
      `countries` table with federal banded effective tax rates, `healthcareModelType`, **and
      per-country `compulsoryInsurance` flags**; plus the shared OECD-modified multiplier helpers
      (`equivalisedSize`, `subLinear`, `perCapita`, `SUBLINEAR_DAMPING`) and `AREA_MULTIPLIERS`. File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/data/cities.ts`. Acceptance: `cities.test.ts`
      passes.
- [ ] **[AI] RED** Add `calc.test.ts` covering the per-category expense build (housing/utilities scale
      **sub-linearly**; food/healthcare/childcare scale **near per-capita**; transport/lifestyle flat),
      `childcareLocal`, `schoolLocal`, `essentialsLocal`, `expensesLocal`, `expensesUsd`, the **split**
      `relocationSunkLocal`/`relocationSunkUsd` + `liquidityReserveLocal`/`liquidityReserveUsd`,
      `incomeBand`, `effectiveRate` (federal + sub-national), `netUsd`, **`grossMonthlyToAnnual` /
      `grossAnnualToMonthly` (annual = 12 × monthly and the inverse)**, **`totalCompAnnual`
      (`grossAnnual + nonSalaryCompAnnual`, informational — never alters net or either savings
      figure)**, `costOfLivingRow`, `savingsRow` (with **both** `essentialSavings` and
      `afterLifestyleSavings`), and `sortByEssentialSavings`, including: **every `*Usd` value routes
      through `fxToUsd(fx, …)` so a city's USD figure equals its local value ×
      `fx.ratesUsdPerUnit[city.currency]`**; housing rises sub-linearly and food/healthcare/childcare near per-capita as the
      OECD-modified household grows; `rural` housing < `center` housing; `private` ≥ `public` school
      cost; zero school cost when `schoolKids = 0`; childcare scales with `preschoolKids` and is zero at
      `preschoolKids = 0`; `essentialSavings = net − essentials` and
      `afterLifestyleSavings = essentialSavings − lifestyle`; `effectiveRate` for a US/CA/CH city =
      federal + sub-national (> federal alone) and for a unitary-country city = federal only; `netUsd` <
      gross for a positive rate and rises with band; `incomeBand` classifies at/across thresholds; the
      relocation split — `relocationSunkLocal` = deposit + keyMoney + moving + visaAdmin,
      `liquidityReserveLocal` = cashCushion, **neither** folded into either savings figure and the
      reserve never added to the sunk-cost total; and the deficit (essentials > net → negative savings)
      and zero/negative-salary edge cases. File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/calc.test.ts`. Command:
      `npx nx run ayokoding-www:test:unit`. Acceptance: fails (no calc yet).
- [ ] **[AI] GREEN** Implement pure `calc.ts` functions per `tech-docs.md` (OECD-modified per-category
      household + area scaling, per-pre-school-child childcare add-on, per-school-age-child school
      add-on, federal + sub-national `netUsd`, gross monthly↔annual derivation, `totalCompAnnual`, the
      two savings figures, split relocation totals, **all `*Usd` conversions reading from `fx.ts` via
      the `fxToUsd`/`cityFxToUsd` helpers**). File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/calc.ts`. Acceptance: `calc.test.ts` passes.
- [ ] **[AI] REFACTOR** Tidy types/naming in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/calc.ts` and
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/data/cities.ts` (or equivalent paths
      confirmed in Phase 0); ensure `calc.ts` is React-free and side-effect-free (no imports from
      React, no `console.log`, no module-level mutation). Acceptance: `npx nx run ayokoding-www:test:unit`
      exits 0; `npx nx run ayokoding-www:lint` exits 0 with no errors.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 1 Gate

> All checks below must pass before starting Phase 1b.

- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (all `fx.test.ts`, `cities.test.ts`, and `calc.test.ts` assertions pass).
- [ ] [AI] `npx nx run ayokoding-www:lint` — exits 0 with no errors on the new data/calc files.
- [ ] [AI] FX single-source verified: `fx.ts` has a positive USD-per-unit entry for every currency referenced by `cities.ts` (and `USD` = 1) plus a `fxSnapshotDate`; no city declares its own `fxToUsd` — asserted by `fx.test.ts` + `cities.test.ts`.
- [ ] [AI] Dataset coverage verified: `cities.ts` contains at least one city each from ASEAN, Japan, Europe (non-Nordic), and Nordics regions.
- [ ] [AI] Every city's `countryId` resolves to a `country` with federal banded `effectiveRate`, a `healthcareModelType`, **and a `compulsoryInsurance` field (boolean `health` + `socialSecurity`)**; every US/CA/CH city carries `subNational` — asserted by `cities.test.ts`.
- [ ] [AI] Every city has `childcareMedianLocal` and a split `relocation` block (`sunkCosts` + `liquidityReserve`) — asserted by `cities.test.ts`.
- [ ] [AI] No Israeli city/country in dataset: grep for `ILS` and `Israel` returns 0 results in `cities.ts`.

> **Pause Safety**: `fx.ts` (the authoritative FX snapshot), `cities.ts` (expenses + tax bands +
> relocation, FX derived from `fx.ts`), and `calc.ts` pure functions are complete, unit-tested, and
> lint-clean. No UI code exists yet. Safe to stop. To resume:
> `npx nx run ayokoding-www:test:unit` — must still pass before Phase 1b.

## Phase 1b — Role-Salary Data + Reverse-Lookup Core (TDD)

Adds the second dataset (`roles.ts`), the pure cascading `geo-filter.ts` selectors, and the pure
`role-lookup.ts` search that powers the minimum-role tab. The **software-engineering** role taxonomy +
the role × **country** salary distribution (p25 / median / p75) + non-salary comp are sourced via
`web-research-maker`, then encoded as a static full **country**×role matrix with per-cell confidence
tiers; cities inherit their country's distribution. The lookup runs the **median** role salary through
the **same net→expenses→savings engine** from `calc.ts`. Still no UI.

- [ ] **[AI]** Source the role data via `web-research-maker`: (a) the canonical 15-rung
      **software-engineering** ladder (IC + management, with `rank`/`track`/`label`), and (b) per role
      per **country** present in `cities.ts`, a gross monthly **`{ p25, median, p75 }`** salary
      distribution (bottom 25% / median / top 25%) plus a typical **non-salary comp** (annual
      RSU/equity + bonus), each with a `confidence` tier and a source note. Record cited findings +
      `snapshotDate` in a research note referenced from `roles.ts` comments. Acceptance: a complete
      role list + a `{ p25, median, p75 }` distribution (with `p25 ≤ median ≤ p75`) and a non-salary
      comp (or documented `proxy` derivation) for every country×role pair, no fabricated exact figures. - _Suggested executor: `web-research-maker`_
- [ ] **[AI] RED** Add `roles.test.ts` asserting matrix invariants: `ladder` is the full 15-rung set
      with strictly increasing `rank`; `salaries` keys **exactly match** the **country** set referenced
      by `cities.ts` (full role × country matrix, no holes); every cell carries a `{ p25, median, p75 }`
      distribution with `p25 ≤ median ≤ p75`, each a positive `monthlyGrossLocal` + valid `confidence`,
      plus a `nonSalaryComp` (non-negative `annualLocal` + `confidence`); **no Israeli country/city /
      `ILS`** leaks in; a `snapshotDate` is present. File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/data/roles.test.ts`. Command:
      `npx nx run ayokoding-www:test:unit`. Acceptance: fails (no `roles.ts` yet).
- [ ] **[AI] GREEN** Add `roles.ts` — the `ladder` metadata + the full role × **country** `salaries`
      matrix (each cell `{ p25, median, p75 }` + `nonSalaryComp`) from the research step, with
      sourced-estimate comments and `snapshotDate`. File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/data/roles.ts`. Acceptance: `roles.test.ts`
      passes.
- [ ] **[AI] RED** Add `geo-filter.test.ts` covering the cascading selectors: `countriesForRegion`
      returns only that region's countries; `citiesForCountry` returns only that country's cities;
      `scopedCities(region, country, city)` applies the three levels in order; clearing a higher level
      resets lower ones; no filter returns all cities. File:
      `apps/ayokoding-www/src/features/cost-of-living-calculator/core/geo-filter.test.ts`. Command:
      `npx nx run ayokoding-www:test:unit`. Acceptance: fails (no `geo-filter.ts` yet).
- [ ] **[AI] GREEN** Implement pure `geo-filter.ts` (Region → Country → City cascading selectors over
      `cities.ts`). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/core/geo-filter.ts`.
      Acceptance: `geo-filter.test.ts` passes.
- [ ] **[AI] RED** Add `role-lookup.test.ts` covering `roleMedianGrossUsd` (uses the **median**),
      `roleSalaryDistributionUsd`, `roleNonSalaryCompUsd`, **`roleTotalCompUsd`**,
      `candidateEssentialSavingsUsd`, `bestCityForRole` (filter-scoped via `cityScope`),
      `resolveBaselineUsd` (all three baseline sources, each on `essentialSavings`, the reference source
      using the median), `rankLadder` (best city + country, the p25/median/p75 distribution, non-salary
      comp, **total comp**, `clears` flags), `minimumRole`, `orderForDisplay`, and
      `toDisplayCurrencies` (reading rates from `fx.ts`), including: the no-qualifier case
      (`minimumRole` → `null`); reference-role baseline parity; cost-basis changes shifting candidates;
      **federal + sub-national tax band selection affecting net savings**; **the geographic filter
      scoping changing each role's best city**; **the reorder grouping qualifying roles above the
      minimum and non-qualifying roles below a divider**; **non-salary comp + total comp NOT changing
      the ranking**; **lifestyle changes NOT changing the ranking**; and confidence propagation to the
      chosen row.
      File: `apps/ayokoding-www/src/features/cost-of-living-calculator/core/role-lookup.test.ts`. Command:
      `npx nx run ayokoding-www:test:unit`. Acceptance: fails (no `role-lookup.ts` yet).
- [ ] **[AI] GREEN** Implement pure `role-lookup.ts` per `tech-docs.md` (reuses `calc.ts`
      `savingsRow`; **median**-based salary, USD-normalised qualify, `cityScope` filtering,
      seniority-ordered display, lowest-rank minimum, and the qualifying/non-qualifying `orderForDisplay`
      reorder). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/core/role-lookup.ts`. Acceptance:
      `role-lookup.test.ts` passes.
- [ ] **[AI] REFACTOR** Tidy types/naming in `role-lookup.ts`, `geo-filter.ts`, and `roles.ts`; ensure
      `role-lookup.ts` and `geo-filter.ts` are React-free and side-effect-free (no React imports, no
      `console.log`, no module-level mutation) and `role-lookup.ts` reuses `calc.ts` rather than
      duplicating cost/tax math. Acceptance: `npx nx run ayokoding-www:test:unit` exits 0;
      `npx nx run ayokoding-www:lint` exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 1b Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (all `roles.test.ts`, `geo-filter.test.ts`, and `role-lookup.test.ts` assertions pass).
- [ ] [AI] `npx nx run ayokoding-www:lint` — exits 0 with no errors on the new role data/lookup/geo-filter files.
- [ ] [AI] Full-matrix check: `roles.ts` `salaries` key set equals the **country** set referenced by `cities.ts` (no missing or extra countries); every cell has a `{ p25, median, p75 }` distribution (`p25 ≤ median ≤ p75`) + a `nonSalaryComp` — asserted by `roles.test.ts`.
- [ ] [AI] No Israeli country/city in role matrix: grep for `ILS` and `Israel` returns 0 results in `roles.ts`.

> **Pause Safety**: both datasets and both pure cores (`calc.ts` + `role-lookup.ts`) are complete,
> unit-tested, and lint-clean. No UI code exists yet. Safe to stop. To resume:
> `npx nx run ayokoding-www:test:unit` — must still pass before Phase 2.

## Phase 2 — Interactive Page (TDD)

All three tabs need a `Table` primitive that `libs/web-ui` does not yet ship. Build that primitive in
the shared lib first, then consume it from the app. Changes under `libs/web-ui` are picked up by the
`nx affected` quality gates in Phase 4.

- [ ] **[AI] RED** Add a unit test for a new `Table` primitive in `libs/web-ui` following the existing primitive pattern (e.g. `libs/web-ui/src/primitives/table/table.test.tsx`): assert it renders `Table`/`TableHeader`/`TableBody`/`TableRow`/`TableHead`/`TableCell`/`TableCaption` with correct semantic roles. Command: `npx nx run web-ui:test:unit`. Acceptance: fails (no component yet).
- [ ] **[AI] GREEN** Create the `Table` primitive (delegate to `swe-ui-maker`): `libs/web-ui/src/primitives/table/table.tsx` (shadcn `Table` family, CVA variants, semantic `<table>` markup, AA-contrast tokens), barrel-export it from `libs/web-ui/src/index.ts`, and add `libs/web-ui/src/primitives/table/table.stories.tsx`. Acceptance: `npx nx run web-ui:test:unit` exits 0; `npx nx run web-ui:lint` exits 0; `npx nx run web-ui:build-storybook` succeeds.
  - _Suggested executor: `swe-ui-maker`_
- [ ] **[AI] RED** Add a component test for the shared **geo-filters** component: render the Region / Country / City cascading filter row; assert selecting a Region narrows the Country options, selecting a Country narrows the City options, and clearing a higher level resets the lower ones; assert the selected scope is reported to the parent. File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.test.tsx`. Command: `npx nx run ayokoding-www:test:unit`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `geo-filters.tsx` (Region / Country / City cascading `Command`/dropdown row consuming `geo-filter.ts`). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx`. Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for the **Cost-of-living** tab: render the category table, assert a city row shows a **Country column immediately to the left of the City column**, all seven expense categories (incl. childcare) plus the **school** column, an essentials subtotal, a total (in both local currency and USD), a separate relocation **sunk-cost** total, and a separately labelled **liquidity reserve**; assert the **healthcare funding-scheme badge** is shown; assert the shared geo-filters narrow the rows; assert clicking a city name fires the city-detail navigation (`?tab=cost&city=<id>`). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.test.tsx`. Command: `npx nx run ayokoding-www:test:unit`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `cost-of-living.tsx` (category table consuming `calc.ts` `costOfLivingRow` and the new `Table` primitive, Country column left of City, the **school** column, city-name links to the detail view, the healthcare-scheme badge, and the shared `geo-filters`). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`. Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for the single-city **city-detail** view: render with a deep-linked `city` id; assert the full per-category breakdown (housing/food/transport/utilities/healthcare-OOP/childcare/school/lifestyle), essentials subtotal, total, healthcare scheme badge, and split relocation (sunk + liquidity reserve) are shown in both local currency and USD, and a back affordance returns to the full table. File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `city-detail.tsx` (single-city Cost-of-living detail consuming `calc.ts` `costOfLivingRow`, dual-currency, healthcare badge, split relocation, back affordance; reached via `?tab=cost&city=<id>` or a city-name click). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx`. Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for the **Savings** tab: enter a gross salary, assert the **monthly AND annual** figures are both shown (entering monthly fills annual = 12×), each city row shows a Country column left of City, the informational **non-salary comp** column, a **total compensation** column (base annual + non-salary comp), net (after federal + sub-national tax, lower than gross), essentials, **both savings figures** (`essentialSavings` and `afterLifestyleSavings`) with percentages, and the healthcare-scheme badge, including the deficit case; assert non-salary comp **and total comp** are NOT in the net/savings; assert sort by savings; assert city-name links navigate to the detail. File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `savings.tsx` (gross input accepting monthly or annual with the other derived, non-salary-comp column, **total-comp column (informational, for negotiation context)**, Country+City columns, city-name links, net/essentials/two-savings-figures table consuming `calc.ts` `savingsRow` + `totalCompAnnual`). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`. Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for the **Minimum role** tab: set a savings-target baseline, assert the **"Roles: software-engineering (IC + management)"** caption renders; assert the ladder is **reordered** — qualifying roles grouped above a divider with the lowest qualifier (ranked on **essential savings** via the **median** salary) marked as the minimum, and non-qualifying roles grouped, dimmed, below the divider; assert each row shows a Country column + best city, the role × country **p25 / median / p75** distribution, the **non-salary comp**, a **total compensation** figure (base + non-salary comp), and essential savings in USD + local + display currency; assert switching the baseline source recomputes; assert the geo-filters scope the candidate cities; assert the no-qualifier message shows when the target exceeds every role's best-city essential savings. File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `min-role.tsx` (SE-roles caption, baseline selector, display-currency picker, shared geo-filters scoping candidates, reordered ranked ladder table consuming `role-lookup.ts` + the shared `Table` with the Country column, p25/median/p75 distribution, non-salary comp, **total comp (informational)**, minimum marker + confidence badges + healthcare-scheme badge + summary line). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`. Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for the shared controls: single/married selector, two kids number inputs (**pre-school 0–3**, **school-age 0–3**), area toggle (`center`/`rural`), and a school-type toggle (`public`/`private`) that is **hidden** when there are no school-age children and **shown** once school-age kids are selected; assert modeled expenses recompute (childcare scales with pre-school kids, schooling with school-age kids) when each changes. File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement the shared controls (single/married + pre-school & school-age kid counts, area toggle, conditional school-type toggle). File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx`. Acceptance: test passes.
- [ ] **[AI] RED** Add an integration test for the top-level page at
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/page.test.tsx` (_New file_):
      render the page and assert (a) all three tabs ("Cost of living", "Savings", "Minimum role") are
      reachable via tab click, (b) clicking the "Savings" tab shows the gross-salary input, (c) the URL
      query `?tab=cost&city=<id>` syncs the city-detail view (clicking a city name sets `detailCity` and
      the detail renders), and (d) the shared household state (single/married + pre-school & school-age
      kid counts) is passed down and triggers a recompute. Command: `npx nx run ayokoding-www:test:unit`.
      Acceptance: test fails (page.tsx does not exist yet).
- [ ] **[AI] GREEN** Add `page.tsx` (`'use client'`) at `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/page.tsx` with the **three-tab** toggle wiring `cost-of-living`, `savings`, `min-role`, and the single-city `city-detail` view + the shared household (single/married + pre-school & school-age kid counts), area, school-type state, the shared **Region / Country / City** cascading-filter state, the `detailCity` drill-down synced to the `?tab=cost&city=<id>` URL query, the savings gross-salary input (**monthly with annual derived**), and the minimum-role (baseline source, reference city/role, savings target, display currency) state. Acceptance: `npx nx run ayokoding-www:test:unit` exits 0 (page.test.tsx passes); route renders in dev (`npx nx dev ayokoding-www`, visit `/en/tools/cost-of-living-calculator`) with all three tabs reachable, the cascading filters working, and `?tab=cost&city=<id>` deep-linking to a single-city detail.
- [ ] **[AI] REFACTOR** Extract shared `Intl.NumberFormat` formatting logic into a shared helper (e.g.
      `formatCurrency(amount, currency, locale)`); de-duplicate formatting calls across
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`,
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx`,
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`,
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`,
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx`, and
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/page.tsx` (or equivalent paths
      confirmed in Phase 0). Acceptance: `npx nx run ayokoding-www:test:unit` exits 0; no test
      regressions.
  - _Suggested executor: `swe-ui-maker`_

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx run web-ui:test:unit` and `npx nx run web-ui:lint` — both exit 0 (new `Table` primitive tested and lint-clean); `npx nx run web-ui:build-storybook` succeeds.
- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (all component tests for `geo-filters`, `cost-of-living`, `city-detail`, `savings`, `min-role`, and `controls` pass).
- [ ] [AI] Dev server check: `npx nx dev ayokoding-www` starts; navigate to `/en/tools/cost-of-living-calculator` — page renders without a crash, all three tabs are reachable, the cascading filters work, and `?tab=cost&city=<id>` deep-links to a single-city detail.
- [ ] [AI] `npx nx run ayokoding-www:lint` — exits 0 on all new component files.

> **Pause Safety**: all three calculator tabs render and compute correctly with full component test
> coverage; dev server verified. Bilingual strings and a11y not yet applied. Safe to stop.
> To resume: `npx nx run ayokoding-www:test:unit` — must still pass before Phase 3.

## Phase 3 — Bilingual Strings + Polish

- [ ] **[AI]** Edit `apps/ayokoding-www/src/features/i18n/core/translations.ts` — add all calculator
      UI strings for both `en` and `id`: headings, tab names ("Cost of living", "Savings", "Minimum
      role"), the **eight expense-category names** (housing, food, transport, utilities, healthcare,
      **childcare**, **school**, lifestyle), **essentials subtotal / total** labels, **net / tax**
      wording (incl. "federal" + "state/province/canton" sub-national + income-band labels),
      **healthcare funding-scheme** badge labels ("tax-funded", "mandatory payroll insurance",
      "out-of-pocket"), the **two savings-figure** labels ("Savings after essentials" / "Savings after
      lifestyle"), **relocation** labels split into **sunk costs** (deposit, **key money**, moving,
      visa/admin) and **liquidity reserve** (cash cushion), the **Region / Country / City** filter
      labels, **Country** + **City** column headers, the city-detail **"Back to all cities"** label, the
      gross-salary **monthly** + **annual** labels, the **non-salary comp** ("Typical RSU/equity +
      bonus") label + its informational note, the **total compensation** ("Total comp") label + its
      informational note, the **p25 / median / p75** labels ("Bottom 25%", "Median",
      "Top 25%"), the **"Roles: software-engineering (IC + management)"** caption, the **qualifies /
      below minimum** group labels, single/married labels, the **pre-school children** + **school-age
      children** count labels, area + school-type toggle labels, **baseline-source labels** (my salary /
      reference role / savings target), **display-currency label**, **confidence-tier labels**, the
      **Disclaimers** block (pension-excluded, clothing/personal-care-in-lifestyle, nominal-FX-not-PPP,
      snapshot-staleness, simplified-tax, healthcare-OOP, relocation-reserve, **role-salary-national-level**,
      **non-salary-comp-informational**), the "Gross monthly salary (before tax)" salary label, and the
      "Data last updated" label — following the existing `Record<Locale, Record<string, string>>` shape
      in that file. Wire the new keys into the calculator page and components. Role labels come from
      `roles.ts` (`ladder[].label.en/id`). Acceptance: `/id/tools/cost-of-living-calculator` shows Indonesian
      labels for all calculator UI elements including the three tabs, the eight category names, the
      Region/Country/City filter labels, the SE-roles caption, the tax/net wording, the healthcare-scheme
      badge, and the relocation labels. - _Suggested executor: `apps-ayokoding-www-general-maker`_
- [ ] **[AI]** Label salary inputs "Gross monthly salary (before tax)"; show a prominent, localized
      **"Data last updated: &lt;date&gt;"** label (formatted from `snapshotDate` via `Intl.DateTimeFormat`)
      near the results, plus the **Disclaimers** block covering "estimates only", "savings are net of a
      simplified effective tax rate (federal + sub-national for US/CA/CH only) — not a full bracket
      calculation, and excluding filing status/deductions/benefits-in-kind/contribution caps",
      "household/rural costs use shared OECD-modified multipliers and childcare/school costs are city
      medians", "transport assumes public transport — car ownership not modeled", "relocation sunk
      costs are a one-time estimate kept out of the monthly savings math; the cash cushion is a reserve
      you keep, not a sunk cost", "savings are before voluntary pension/retirement contributions",
      "clothing and personal care are folded into lifestyle", "a positive USD savings figure does not
      mean equal purchasing power — USD uses a nominal FX snapshot, not PPP", "healthcare models
      out-of-pocket only; the funding scheme is shown per country", "**role salary is modeled at the
      national (country) level — cities inherit their country's p25/median/p75 distribution**", and
      "**non-salary comp (RSU/equity + bonus) is informational total-comp context only, not part of the
      savings math**". Acceptance: last-updated date, gross-salary (monthly + annual) labels,
      healthcare-scheme badge, SE-roles caption, and the full disclaimer block clearly visible in both
      locales.

### Manual UI Verification (Playwright MCP)

- [ ] [AI] Start dev server: `npx nx dev ayokoding-www` (port 3101).
- [ ] [AI] `browser_navigate` to `http://localhost:3101/en/tools/cost-of-living-calculator` — acceptance: page loads without JS errors.
- [ ] [AI] `browser_snapshot` — verify the **Cost of living** tab renders with the Country column to the left of the City column, the seven category columns (incl. childcare) plus the school column, essentials subtotal, total, relocation sunk-cost column, the separately labelled liquidity reserve, the healthcare funding-scheme badge, the Region / Country / City cascading filters, the household control (single/married + pre-school & school-age kid counts), and the area toggle all visible.
- [ ] [AI] `browser_click` the Region filter then the Country filter — acceptance: choosing a Region narrows the Country list, choosing a Country narrows the City list and the table to that country's cities; clearing restores all cities.
- [ ] [AI] `browser_click` a city name in the table — acceptance: navigates to the single-city Cost-of-living detail (URL contains `?tab=cost&city=`); the detail shows the full per-category breakdown + healthcare badge + split relocation in local + USD; a back affordance returns to the full table.
- [ ] [AI] `browser_click` the **Savings** tab, `browser_fill_form` the gross monthly salary with `"8000"` — acceptance: the annual gross shows `96,000`; each city row shows the Country+City, the informational non-salary-comp column, the **total compensation** column (base annual + non-salary comp), net (after federal + sub-national tax, lower than 8000), essentials, both savings figures (after essentials, after lifestyle), and savings % columns; `browser_click` a sort trigger sorts by savings.
- [ ] [AI] `browser_click` the **Minimum role** tab, confirm the "Roles: software-engineering (IC + management)" caption is present, set the baseline source to "savings target", and `browser_fill_form` the target with `"2000"` — acceptance: the ladder is reordered — qualifying roles grouped above the marked minimum, non-qualifying roles dimmed below a divider; each row shows the best city + its country, the p25/median/p75 distribution, and the **total compensation** (base + non-salary comp) figure; savings show in USD + local + display currency; selecting a Country in the filters re-scopes the candidate cities. Resize narrow (`browser_resize` to ~375 px) — acceptance: the ladder reflows to stacked cards (responsive mobile layout) without overflow.
- [ ] [AI] `browser_console_messages` — acceptance: zero JS errors.
- [ ] [AI] `browser_navigate` to `http://localhost:3101/id/tools/cost-of-living-calculator`, then `browser_snapshot` — acceptance: all labels, tab names, category names, and the disclaimer are in Indonesian.
- [ ] [AI] `browser_take_screenshot` — save as visual record for this phase.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (no regressions from i18n wiring).
- [ ] [AI] `/en/tools/cost-of-living-calculator` and `/id/tools/cost-of-living-calculator` both render correctly — confirmed by Playwright MCP `browser_navigate` + `browser_snapshot` steps above.
- [ ] [AI] All calculator UI strings present in both `en` and `id` keys in `apps/ayokoding-www/src/features/i18n/core/translations.ts` — grep for the salary-label key, a category-name key (e.g. `housing`, `childcare`, `school`), a Region/Country/City filter label, the SE-roles caption, and a healthcare-scheme label in both locale branches returns a non-empty string.
- [ ] [AI] "Data last updated" label and "estimates only" disclaimer visible in both locales — confirmed by `browser_snapshot` above.
- [ ] [AI] Zero JS errors on either locale URL — confirmed by `browser_console_messages` above.

> **Pause Safety**: bilingual strings complete, disclaimer visible, a11y/responsive verified,
> Playwright MCP smoke passed in both locales. Safe to stop. To resume: re-run the Playwright
> MCP verification steps above — both locale URLs must render without JS errors.

## Phase 4 — E2E + Local Quality Gates

- [ ] **[AI]** Create the companion Gherkin spec file at
      `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
      (_New file_, _New directory_ `tools/`): copy the 36 Gherkin scenarios from `prd.md
§Acceptance Criteria (Gherkin)` into the feature file with the canonical `Feature:` header
      (`Feature: Cost of Living Calculator`). Mirror the scenario text verbatim from `prd.md`.
      Acceptance:
      `test -f specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature && echo "OK"`.

- [ ] **[AI] RED** Add a failing fe-e2e smoke test in
      `apps/ayokoding-www-fe-e2e/src/cost-of-living-calculator.spec.ts` (_New file_): navigate to
      `/en/tools/cost-of-living-calculator`, assert the **Cost of living** table is populated with at least one
      city row showing both a Country and a City column plus category expenses; apply a Region then
      Country filter and assert the rows narrow; click a city name and assert it deep-links to
      `?tab=cost&city=` showing the single-city detail; switch to the **Savings** tab, enter a gross
      monthly salary of `"8000"`, assert the annual `96,000` is shown and at least one net/savings cell
      and the non-salary-comp + total-comp columns are visible; then switch to the **Minimum role** tab, confirm the
      software-engineering-roles caption, set a savings target of `"2000"`, and assert the ladder is
      reordered (a qualifying group above a divider with one role marked as the minimum, a dimmed
      below-minimum group beneath) and a row shows the best city + its country.
      Command: `npx nx run ayokoding-www-fe-e2e:test:e2e`. Acceptance: test file exists and the test
      fails (page route not yet reached by e2e or an element assertion fails when written before the
      page is fully wired). - _Suggested executor: `swe-e2e-dev`_
- [ ] **[AI] GREEN** Confirm that `npx nx run ayokoding-www-fe-e2e:test:e2e` passes with the calculator
      page fully implemented from Phases 1–3. Acceptance: smoke test passes end-to-end across all three
      tabs with zero errors.
- [ ] **[AI]** Run affected local quality gates: `npx nx affected -t typecheck lint test:quick specs:coverage` — warm the cache first (`npx nx affected -t typecheck lint test:quick specs:coverage --skip-nx-cache` if cache is cold). Fix ALL failures encountered — including preexisting issues not introduced by this plan's changes (root-cause orientation: do not defer or mention-and-skip existing failures). Acceptance: all four targets exit 0.

### Commit Guidelines

- Commit changes thematically: FX + city data layer (`fx.ts` + `cities.ts` + tax/relocation +
  `calc.ts`) in one commit, role data layer (`roles.ts` role × country distribution + `geo-filter.ts`
  - `role-lookup.ts`) in a second, `web-ui` `Table` primitive in a third, UI components (geo-filters,
    cost-of-living, city-detail, savings, min-role, controls) in a fourth, bilingual strings in a fifth,
    e2e in a sixth. Follow Conventional Commits format:
    `feat(ayokoding-www): add cost-of-living calculator`.
- Do NOT bundle unrelated fixes into the same commit. Note: commits happen only on explicit user
  instruction per repo policy.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` — exits 0 (smoke test passes across all three tabs).
- [ ] [AI] `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] `npx nx affected -t lint` — exits 0.
- [ ] [AI] `npx nx affected -t test:quick` — exits 0.
- [ ] [AI] `npx nx affected -t specs:coverage` — exits 0.

> **Pause Safety**: all local quality gates green and e2e smoke passing. Safe to stop before push.
> To resume: `npx nx affected -t typecheck lint test:quick specs:coverage` — all must still exit 0.

## Phase 5 — Post-Push CI Verification

- [ ] **[HUMAN]** Review the diff and approve push to `main` (trunk-based). Observable resume signal:
      user confirms approval; verify with `git log --oneline -1 origin/main` after push shows the new
      commit.
- [ ] **[AI]** Push and trigger/monitor relevant GitHub Actions for `ayokoding-www` (poll every 3 min
      via `gh run list --limit 5` + `gh run view <run-id> --json status,conclusion`; do not use
      `gh run watch`). Acceptance: CI green.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `gh run list --limit 5 --json status,conclusion,name` — all runs triggered by this push show `conclusion: success`.
- [ ] [AI] No open CI failures on `main` related to `ayokoding-www` or `ayokoding-www-fe-e2e`.

> **Pause Safety**: all CI checks green on `main`. Safe to stop. To resume:
> `gh run list --limit 5 --json status,conclusion,name` — all must show `success`.

## Phase 6 — Plan Archival

- [ ] **[AI]** Run
      `git mv plans/in-progress/ayokoding-www-salary-savings-calculator plans/done/$(date +%Y-%m-%d)__ayokoding-www-salary-savings-calculator`
      from repo root. Acceptance: folder appears under `plans/done/` with today's date prefix;
      `plans/in-progress/ayokoding-www-salary-savings-calculator/` no longer exists
      (`test ! -d plans/in-progress/ayokoding-www-salary-savings-calculator && echo "OK"`).
      Also update `plans/in-progress/README.md` (remove this plan's entry) and
      `plans/done/README.md` (add entry with completion date).
- [ ] **[HUMAN]** Remove the worktree once merged:
      `git worktree remove worktrees/ayokoding-www-salary-savings-calculator`. Observable resume
      signal: `git worktree list` no longer shows the worktree path.

### Phase 6 Gate

> All checks below must pass to consider the plan complete.

- [ ] [AI] `test ! -d plans/in-progress/ayokoding-www-salary-savings-calculator && echo "OK"` — plan folder no longer exists under `in-progress/`.
- [ ] [AI] `ls plans/done/ | grep ayokoding-www-salary-savings-calculator` — folder exists under `done/` with a date prefix.
- [ ] [AI] `plans/in-progress/README.md` no longer lists this plan; `plans/done/README.md` lists it with a completion date.

> **Pause Safety**: plan archived, worktree cleaned up. Feature live at
> `/[locale]/tools/cost-of-living-calculator` in `en` + `id`. No further action required.
