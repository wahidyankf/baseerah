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

## Savings Model (v1)

The salary input is **gross monthly salary, before taxes and deductions**. v1 does **not** model
taxes, social contributions, or other deductions — so "savings" here is a simplified
gross-minus-cost-of-living figure that **overstates** real take-home savings. A visible disclaimer
states this; tax modelling is deferred (see Out of Scope).

Per city, with `costLocal` = monthly cost of living in local currency and `fxToUsd` = USD value of 1
local-currency unit:

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

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Salary savings calculator

  Scenario: Compare all cities for a USD salary
    Given I am on "/en/tools/salary-savings"
    And the "Compare all" mode is active
    When I enter a monthly salary of "8000" USD
    Then I see a table of tech-hub cities
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
```

## Functional Requirements

- FR-1: Route `/[locale]/tools/salary-savings` renders in `en` and `id`.
- FR-2: "Compare all" mode: one gross (pre-tax) USD salary input → sortable table of all cities with
  savings % and local-currency savings.
- FR-3: "Single city" mode: city picker + gross (pre-tax) local-currency salary input → cost/savings
  breakdown.
- FR-4: Currency and number formatting respect each city's currency and the active locale.
- FR-5: Negative (deficit) savings are computed and displayed explicitly.
- FR-6: Dataset excludes all Israeli cities and records an FX/cost snapshot date. The exclusion is a
  deliberate country-level choice about the state of Israel and its political stance — **not** about
  any ethnic, racial, or religious group.
- FR-7: Visible disclaimer covering both "estimates only" **and** "gross salary, before tax —
  taxes/deductions are not modelled, so real savings will be lower". Salary input labels read
  "Gross monthly salary (before tax)".
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
non-default currencies, charts, and any Israeli city.
