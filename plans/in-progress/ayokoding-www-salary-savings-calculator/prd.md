# Product Requirements Document — Salary Savings Calculator

## Product Overview

An interactive, client-side calculator on `ayokoding-www` that estimates monthly savings for a given
salary across tech-hub cities worldwide. It lives at `/[locale]/tools/salary-savings`, works in both
English (`en`) and Indonesian (`id`), and reports savings as a **percentage** of salary and an amount
in each city's **local currency**. Cost-of-living and FX figures come from a static curated dataset.

## Personas

- **Tech worker / visitor** — has a salary in mind, wants to see where and how much they could save.
- **Relocation planner** — compares many cities at once to shortlist destinations.
- **Indonesian visitor** — uses the `id` locale; expects fully localized labels and number/currency
  formatting.

## Modes

The page has two modes selected by a tab toggle:

- **Compare all** — one salary input (USD), a table of all cities with savings figures.
- **Single city** — one city picker + salary input in that city's local currency, with a breakdown.

A **household selector** applies to both modes and sets the cost-of-living basis. Five household
types are supported:

- `single`
- `married` (couple, no children)
- `married_1_kid`
- `married_2_kids`
- `married_3_kids`

When the household has children, a **school-type toggle** (`public` | `private`) also applies. Each
city carries a **median** monthly school cost per child for both school types; the chosen type adds
schooling cost on top of living cost, multiplied by the number of children. The toggle is hidden for
childless households (`single`, `married`).

An **area toggle** (`center` | `rural`) sets where in the city the person lives. The city-center
baseline is the dataset's stored cost; the rural option applies a discount (mainly housing) via a
shared area-multiplier. The area toggle applies to living cost only, not schooling.

## Savings Model (v1)

The salary input is **gross monthly salary, before taxes and deductions**. v1 does **not** model
taxes, social contributions, or other deductions — so "savings" here is a simplified
gross-minus-cost-of-living figure that **overstates** real take-home savings. A visible disclaimer
states this; tax modelling is deferred (see Out of Scope).

Cost of living has two parts: **living cost** (housing, food, transport) and **schooling cost**. The
transport portion of living cost **assumes public transport** (a typical monthly transit pass) — car
ownership, fuel, and parking are not modelled. This is a fixed v1 assumption, not a toggle.

Living cost scales with the selected **household type** and **area**. The dataset stores one curated
single-person monthly living cost per city (`costOfLivingLocal`, a **city-center** baseline); a
shared **household-multiplier table** derives the other four household living costs, and a shared
**area-multiplier** (`center` = 1.0, `rural` < 1.0) discounts for living outside the center.
Schooling cost is added separately: each city stores a **median** monthly cost per child for
`public` and `private` school, multiplied by the number of children. The multiplier, area, and
median-school approximations are part of the "estimates only" disclaimer; per-city household/area
overrides are deferred (see Out of Scope).

Per city, with `singleCostLocal` = monthly single-person city-center living cost in local currency,
`household` = selected household type, `multiplier(household)` from the household-multiplier table,
`area` = selected area (`center` | `rural`), `areaMultiplier(area)` from the area table,
`kids(household)` = number of children (0 for `single`/`married`, 1–3 otherwise), `schoolType` =
selected school type (`public` | `private`), `schoolMedianLocal[schoolType]` = median monthly cost
per child, and `fxToUsd` = USD value of 1 local-currency unit:

- `livingLocal = singleCostLocal * multiplier(household) * areaMultiplier(area)`
- `schoolLocal = kids(household) * schoolMedianLocal[schoolType]`
- `costLocal = livingLocal + schoolLocal`
- `costUsd = costLocal * fxToUsd`
- **Compare all** (salary entered in USD): `savingsUsd = salaryUsd - costUsd`;
  `savingsLocal = savingsUsd / fxToUsd`; `savingsPct = savingsUsd / salaryUsd * 100`.
- **Single city** (salary entered in local currency): `savingsLocal = salaryLocal - costLocal`;
  `savingsPct = savingsLocal / salaryLocal * 100`.
- Deficit case (`cost > salary`) yields a negative savings and percentage, shown explicitly.

Figures are estimates; the UI shows the FX/cost **snapshot date** and an "estimates only" note.

## User Stories

### US-01: Compare savings across cities for one salary

As a tech worker, I want to enter a single salary and see a table of tech-hub cities ranked by how
much I could save, so I can spot the best destinations at a glance.

### US-02: See savings in both percentage and local currency

As a visitor, I want each result to show savings as a percentage of salary and as an amount in the
city's local currency, so the number is meaningful locally.

### US-03: Drill into a single city

As a relocation planner, I want to pick one city and enter a salary in its local currency to see a
clear breakdown (cost vs. savings), so I can sanity-check a specific offer.

### US-04: Use the tool in Indonesian

As an Indonesian visitor, I want all labels, headings, and disclaimers in `id`, so the tool is fully
usable in my language.

### US-05: Understand data limits

As any visitor, I want a visible snapshot date and an "estimates only" disclaimer, so I don't treat
the figures as exact.

### US-06: Adjust savings for household size

As a visitor with a family, I want to pick a household type (single, married, or married with 1–3
kids) so the cost of living — and therefore my savings — reflects my situation.

### US-07: Choose public vs private schooling

As a visitor with children, I want to toggle between public and private school so the schooling
portion of my cost of living uses the median cost for the type I expect to use.

### US-08: Choose city-center vs rural living

As a visitor, I want to toggle between living in the city center and a rural/outer area so my
estimated housing-driven living cost reflects where I'd actually live.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Salary savings calculator

  Scenario: Compare all cities for a USD salary
    Given I am on "/en/tools/salary-savings"
    And the "Compare all" mode is active
    When I enter a monthly salary of "8000" USD
    Then I see a table of tech-hub cities
    And each row shows the estimated cost of living in both local currency and USD
    And each row shows a savings percentage and a savings amount in the city's local currency
    And the table can be sorted by savings

  Scenario: Single city breakdown in local currency
    Given I am on "/en/tools/salary-savings"
    And I switch to "Single city" mode
    And I select the city "Lisbon"
    When I enter a monthly salary of "5000" in local currency
    Then I see the estimated living cost and the resulting savings
    And savings are shown as a percentage and a local-currency amount

  Scenario: Salary below cost of living shows a deficit
    Given I am in "Single city" mode for a high-cost city
    When I enter a salary lower than that city's cost of living
    Then the savings amount and percentage are shown as negative

  Scenario: Indonesian locale is fully translated
    Given I am on "/id/tools/salary-savings"
    Then all labels, headings, and the disclaimer are in Indonesian

  Scenario: No Israeli cities are listed
    Given I am on the calculator in either locale
    Then no Israeli city appears in the dataset or table

  Scenario: Data last-updated date is clearly shown
    Given I am on the calculator
    Then I see a prominent "Data last updated" label with the dataset snapshot date
    And I see an "estimates only" disclaimer

  Scenario: Household type changes the cost of living
    Given I am on "/en/tools/salary-savings"
    And I have entered a salary
    When I change the household type from "single" to "married 2 kids"
    Then the estimated cost of living increases
    And the savings amount and percentage update accordingly

  Scenario: School type toggle is hidden for childless households
    Given I am on "/en/tools/salary-savings"
    When the household type is "single" or "married"
    Then no school-type toggle is shown

  Scenario: School type toggle appears when household has children
    Given I am on "/en/tools/salary-savings"
    When the household type has children
    Then a "public / private" school-type toggle is shown

  Scenario: Private school raises cost more than public
    Given I am on "/en/tools/salary-savings"
    And the household type is "married 2 kids"
    When I switch the school type from "public" to "private"
    Then the estimated cost of living increases
    And the savings amount and percentage update accordingly

  Scenario: Rural area lowers cost versus city center
    Given I am on "/en/tools/salary-savings"
    And I have entered a salary
    When I switch the area from "city center" to "rural"
    Then the estimated living cost decreases
    And the savings amount and percentage update accordingly
```

## Functional Requirements

- FR-1: Route `/[locale]/tools/salary-savings` renders in `en` and `id`.
- FR-2: "Compare all" mode: one gross (pre-tax) USD salary input → sortable table of all cities with
  savings % and local-currency savings.
- FR-3: "Single city" mode: city picker + gross (pre-tax) local-currency salary input → cost/savings
  breakdown.
- FR-3b: A household selector (single, married, married + 1 kid, married + 2 kids, married + 3 kids)
  applies to both modes; the chosen household scales each city's living cost via the
  household-multiplier table, and all savings figures recompute from it.
- FR-3c: When the household has children, a school-type toggle (`public` | `private`) applies; the
  chosen type's **median** per-child school cost is added per child on top of living cost. The
  toggle is hidden for `single` and `married`.
- FR-3d: An area toggle (`center` | `rural`) applies to both modes; the city-center baseline is the
  stored cost, and `rural` discounts living cost via a shared area-multiplier. Area affects living
  cost only, not schooling.
- FR-4: Currency and number formatting respect each city's currency and the active locale.
- FR-4b: Each city's estimated cost of living is shown in **both** the city's local currency **and**
  USD (`costUsd = costLocal * fxToUsd`), in both modes.
- FR-5: Negative (deficit) savings are computed and displayed explicitly.
- FR-6: Dataset excludes all Israeli cities and records an FX/cost snapshot date. The exclusion is a
  deliberate country-level choice about the state of Israel and its political stance — **not** about
  any ethnic, racial, or religious group.
- FR-7: Visible disclaimer covering "estimates only", "gross salary, before tax —
  taxes/deductions are not modelled, so real savings will be lower", "household and rural
  costs are derived from shared multipliers, and school costs are city medians — not exact per-case
  data", **and** "transport assumes public transport (monthly pass); car ownership is not modelled".
  Salary input labels read "Gross monthly salary (before tax)".
- FR-8: The page **clearly and prominently shows when the data was last fetched/updated** — a
  "Data last updated: &lt;date&gt;" label (localized, from the dataset `snapshotDate`) placed near the
  results, not buried in fine print. Since v1 data is static, "last updated" = the dataset snapshot
  date; the same field will reflect real fetch time if a live source is added later.

## Non-Functional Requirements

- NFR-1: **Client-side rendered (CSR)** — a `'use client'` page; all inputs and computation happen
  in the browser. No server-side rendering of results, no backend/tRPC procedure, no runtime network.
- NFR-1b: Dataset lists **as many tech-hub cities worldwide as we reasonably can** (static), excl.
  Israel; breadth over a fixed small set. **ASEAN, Japan, broader Europe, and the Nordics must each
  be represented**, alongside the Americas, Middle East, South/East Asia, Oceania, and Africa.
- NFR-2: WCAG AA — labeled inputs, keyboard-operable, sufficient contrast; responsive (mobile→desktop).
- NFR-3: Calculation core is pure and unit-tested; component has tests; one fe-e2e smoke test.
- NFR-4: No new runtime dependencies beyond those `ayokoding-www` already ships.

## Out of Scope

Live cost-of-living/FX APIs, tax/deductions, savings goals, persistence/share/export, per-city
non-default currencies, per-city household/area-cost overrides (v1 uses shared multiplier tables),
school-cost granularity beyond a city public/private median, charts, and any Israeli city.
