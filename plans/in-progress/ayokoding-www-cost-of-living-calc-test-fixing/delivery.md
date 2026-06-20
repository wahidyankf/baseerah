# Delivery — Cost-of-Living Calculator Test-Fixing

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

This plan changes only the **shell / layout / i18n / config** of `apps/ayokoding-www` — never the
verified-correct core math. Suggested executor for all TypeScript/TSX shell + layout steps:
`swe-typescript-dev`. Verify each fix locus against `tech-docs.md`.

## Worktree

Worktree path: `worktrees/ayokoding-www-cost-of-living-calc-test-fixing/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree ayokoding-www-cost-of-living-calc-test-fixing
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift
- [ ] [AI] Establish the unit baseline: `npx nx run ayokoding-www:test:unit`
      — acceptance: pass/fail count recorded; preexisting failures documented
- [ ] [AI] Establish the FE-E2E baseline: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: pass/fail count recorded; preexisting failures documented
- [ ] [AI] Establish the specs-coverage baseline: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: exits 0 (or preexisting state recorded)
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved (root-cause orientation)

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` baseline recorded and every
      preexisting failure resolved (zero unresolved)

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature work
> exists yet. Safe to stop indefinitely. To resume: re-run the baseline command and confirm it is
> still clean.

## Phase 1: UWT-001 re-verification (conflict gate — do this FIRST)

> Resolve the conflict-flagged `UWT-001` before any tab change. See README conflict note.

- [ ] [AI] Start the dev server: `npx nx dev ayokoding-www`
      — acceptance: server reachable at `http://localhost:3101`
- [ ] [AI] Re-verify tab behaviour with Playwright MCP: navigate to
      `http://localhost:3101/en/tools/cost-of-living-calculator`, activate "Savings" then
      "Minimum role" via `browser_click`, and read the DOM via `browser_snapshot`
      — acceptance: the active panel content swaps to match each selected tab (matches `prd.md`
      Cluster N scenario)
- [ ] [AI] Record the outcome in this file under a "UWT-001 re-verification result" note: if tabs
      swap correctly, mark the `UWT-001` tab-rewrite and `USS-001` "disable + Coming soon"
      suggestion **VOID** and reduce remediation to the `UWT-012` label fix (Phase 6); if tabs are
      genuinely broken, add a new RED/GREEN/REFACTOR sub-phase here for the panel-swap fix
      — acceptance: a written disposition (VOID or fix-needed) is recorded with evidence

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] A written `UWT-001` disposition (VOID, or a concrete fix sub-phase) is recorded with
      Playwright evidence
- [ ] [AI] `git status` is clean except for this delivery.md note (no source changes yet unless tabs
      were genuinely broken and fixed RED→GREEN→REFACTOR)

> **Pause Safety**: the conflict is resolved on paper (and code only if tabs were truly broken). Safe
> to stop. To resume: re-read the recorded disposition and continue at Phase 2.

## Phase 2: Locale correctness (`html lang` + Indonesian translation gaps)

Clusters A + B. Shared-root-cause `EWT-001`/`UWT-006` fixed once.

### Phase 2, Cycle 1 — Cluster A (lang="id")

- [ ] [AI] **RED**: add failing unit test asserting `<html lang>` equals the locale for `/id/` in
      `apps/ayokoding-www/src/app/layout.test.tsx` (_New test_; sibling pattern: existing shell
      `*.test.tsx`) — command: `npx nx run ayokoding-www:test:unit` — acceptance: test fails
      because `lang` is hardcoded `"en"`

  **Gherkin (binds) →** "Indonesian locale page declares lang="id""

  ```gherkin
  Scenario: Indonesian locale page declares lang="id"
    Given I navigate to "/id/tools/cost-of-living-calculator"
    When the page HTML is rendered
    Then the html element carries lang="id"
  ```

- [ ] [AI] **GREEN**: make `<html lang>` locale-aware for the `/id/` route in
      `apps/ayokoding-www/src/app/layout.tsx` / `apps/ayokoding-www/src/app/[locale]/layout.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: the new `lang="id"` test passes; no other unit test breaks
- [ ] [AI] **REFACTOR**: tidy the locale-resolution helper for the id case
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 2, Cycle 2 — Cluster A (lang="en")

- [ ] [AI] **RED**: add failing unit test asserting `<html lang>` equals `"en"` for `/en/` routes in
      `apps/ayokoding-www/src/app/layout.test.tsx` — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (still hardcoded if only id case was wired)

  **Gherkin (binds) →** "English locale page declares lang="en""

  ```gherkin
  Scenario: English locale page declares lang="en"
    Given I navigate to "/en/tools/cost-of-living-calculator"
    When the page HTML is rendered
    Then the html element carries lang="en"
  ```

- [ ] [AI] **GREEN**: complete the locale-aware `lang` attribute for `/en/` routes in the same
      layout files — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: both locale tests pass; no other unit test breaks
- [ ] [AI] **REFACTOR**: consolidate the locale-resolution helper across both layouts (single
      shared utility) — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all unit tests still pass

### Phase 2, Cycle 3 — Cluster B (EWT-008: Indonesian dropdown labels)

- [ ] [AI] **RED**: add failing unit test asserting each Country/City option label uses the
      Indonesian name (falling back to English) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (English labels returned for all options)

  **Gherkin (binds) →** "Filter dropdowns show Indonesian country and city names in the ID locale"

  ```gherkin
  Scenario: Filter dropdowns show Indonesian country and city names in the ID locale
    Given I am on "/id/tools/cost-of-living-calculator"
    When I open the Country and City filter dropdowns
    Then each option label uses the Indonesian name where one exists
    And it falls back to the English name only when no Indonesian name exists
  ```

- [ ] [AI] **GREEN**: apply `EWT-008` locale-aware option labels (`c.name[locale] ?? c.name.en`) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: dropdown-label test passes; no other unit test breaks
- [ ] [AI] **REFACTOR**: extract the locale-name resolver into a shared helper
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 2, Cycle 4 — Cluster B (EWT-009: Indonesian relocation column header)

- [ ] [AI] **RED**: add failing unit test asserting the relocation sunk-cost column header is fully
      translated in the ID locale, in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (English header present)

  **Gherkin (binds) →** "Relocation column header is fully translated in the ID locale"

  ```gherkin
  Scenario: Relocation column header is fully translated in the ID locale
    Given I am on "/id/tools/cost-of-living-calculator"
    When I read the relocation sunk-cost column header
    Then the header is written entirely in Indonesian with no untranslated English word
  ```

- [ ] [AI] **GREEN**: apply `EWT-009` (Indonesian relocation header) by adding the translation key
      in `apps/ayokoding-www/src/features/i18n/core/translations.ts` and consuming it in the
      relevant column/header component — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: relocation-header test passes; no other unit test breaks
- [ ] [AI] **REFACTOR**: ensure the new translation key follows the existing `t(locale, …)` pattern
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 2, Cycle 5 — Cluster B (EWT-010: Indonesian skip link)

- [ ] [AI] **RED**: add failing unit test asserting the skip-to-content link text is the Indonesian
      `"skipToContent"` translation on `/id/` routes, in the layout test file
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (English skip text present)

  **Gherkin (binds) →** "Skip-to-content link is translated in the ID locale"

  ```gherkin
  Scenario: Skip-to-content link is translated in the ID locale
    Given I am on "/id/tools/cost-of-living-calculator"
    When the skip-to-content link is rendered
    Then its visible text is the Indonesian "skipToContent" translation
  ```

- [ ] [AI] **GREEN**: apply `EWT-010` (translated skip link) in
      `apps/ayokoding-www/src/app/[locale]/layout.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: skip-link translation test passes; no other unit test breaks
- [ ] [AI] **REFACTOR**: verify the skip-link uses the same `t(locale, …)` lookup as other layout
      strings; deduplicate if not — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all unit tests still pass

### Phase 2, Cycle 6 — Cluster B (EWT-011: Indonesian clear-region aria-label)

- [ ] [AI] **RED**: add failing unit test asserting the clear-region control `aria-label` uses the
      Indonesian `"clearRegion"` translation on `/id/` routes, in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (English aria-label present)

  **Gherkin (binds) →** "Clear-region control aria-label is translated in the ID locale"

  ```gherkin
  Scenario: Clear-region control aria-label is translated in the ID locale
    Given I am on "/id/tools/cost-of-living-calculator"
    When the clear-region control is rendered
    Then its aria-label uses the Indonesian "clearRegion" translation
  ```

- [ ] [AI] **GREEN**: apply `EWT-011` (translated `aria-label`) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: aria-label translation test passes; no other unit test breaks
- [ ] [AI] **REFACTOR**: deduplicate the `t(locale, …)` lookups in `geo-filters.tsx` now that
      multiple strings are locale-aware — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all unit tests still pass

### Phase 2 Gate

- [ ] [AI] `npx nx run ayokoding-www:test:unit` exits 0
- [ ] [AI] `npx nx affected -t typecheck lint` exits 0

> **Pause Safety**: locale correctness is complete and tested; no half-applied translation. Safe to
> stop. To resume: `npx nx run ayokoding-www:test:unit`.

## Phase 3: Trustworthy numbers (household scaling + negative input)

Clusters C + G. `EWT-006`/`EWT-007` shared root cause; `EWT-005`; folds `SG-001`/`SG-002`/`SG-007`.

### Phase 3, Cycle 1 — Cluster C (EWT-006: comparison-table column scaling)

- [ ] [AI] **RED**: add value-bearing test asserting the visible per-category columns sum to
      Essentials for a 2-adult household in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (raw amounts do not sum to the scaled subtotal)

  **Gherkin (binds) →** "Comparison-table category columns sum to the essentials subtotal under a
  multi-adult household"

  ```gherkin
  Scenario: Comparison-table category columns sum to the essentials subtotal under a multi-adult household
    Given I am on the "Cost of living" tab with the household set to 2 adults
    When I read a city row in the comparison table
    Then each per-category column shows the household-adjusted amount
    And the sum of the per-category columns equals the essentials subtotal shown for that row
  ```

- [ ] [AI] **GREEN**: apply `subLinear`/`perCapita` multipliers in the column mapping in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: columns sum to Essentials; test passes
- [ ] [AI] **REFACTOR**: extract the shared scaling helper (stub for reuse in Cycle 2)
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 3, Cycle 2 — Cluster C (EWT-007: city-detail row scaling)

- [ ] [AI] **RED**: add value-bearing test asserting city-detail rows reconcile to the subtotal
      (2 adults) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (raw rows diverge from subtotal)

  **Gherkin (binds) →** "City-detail rows show household-adjusted amounts that reconcile to the
  subtotal"

  ```gherkin
  Scenario: City-detail rows show household-adjusted amounts that reconcile to the subtotal
    Given I am viewing a city detail with the household set to 2 adults
    When I read the per-category rows
    Then each row shows the household-adjusted amount using the same scaling as the essentials subtotal
    And the rows add up to the essentials subtotal shown in the detail
  ```

- [ ] [AI] **GREEN**: apply the same multipliers in the row mapping in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx` using
      the shared scaling helper from Cycle 1 — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: rows reconcile; test passes
- [ ] [AI] **REFACTOR**: finalise the shared scaling helper so both columns and rows call one path
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 3, Cycle 3 — Cluster G (EWT-005: negative salary clamp)

- [ ] [AI] **RED**: add test for negative salary clamp (`-5000` → annual gross not negative) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (negative value passes through)

  **Gherkin (binds) →** "Negative gross salary input is clamped to zero"

  ```gherkin
  Scenario: Negative gross salary input is clamped to zero
    Given I am on the "Savings" tab
    When I type "-5000" into the gross monthly salary field
    Then the field value is clamped so the annual gross is not negative
    And no city row shows a negative gross-derived figure
  ```

- [ ] [AI] **GREEN**: add `min="0"` and `Math.max(0, parseFloat(e.target.value) || 0)` clamp in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: clamp test passes
- [ ] [AI] **REFACTOR**: tidy the input-sanitisation logic
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 3, Cycle 4 — Cluster G / SG-001 (zero/empty salary deficit)

- [ ] [AI] **RED**: add test for zero/empty deficit with suppressed `—` percentage (`SG-001`) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (percentage shown or `—` absent)

  **Gherkin (binds) →** "Zero or empty salary shows deficit with suppressed percentage"

  ```gherkin
  Scenario: Zero or empty salary shows deficit with suppressed percentage
    Given I am on the "Savings" tab
    When the gross monthly salary field is empty or zero
    Then each city row shows a negative essential-savings amount equal to the negation of that city's essential expenses in USD
    And each percentage cell shows an em dash because there is no net income to compute a percentage from
  ```

- [ ] [AI] **GREEN**: implement the `—` suppression when salary is zero/empty in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: suppressed-percentage test passes; no other unit test breaks
- [ ] [AI] **REFACTOR**: consolidate the zero/empty guard with the negative-clamp from Cycle 3
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 3 Gate

- [ ] [AI] `npx nx run ayokoding-www:test:unit` exits 0 with the new value-bearing tests green
- [ ] [AI] `npx nx affected -t typecheck lint` exits 0

> **Pause Safety**: displayed numbers now reconcile and negative input is clamped; the core math is
> untouched. Safe to stop. To resume: `npx nx run ayokoding-www:test:unit`.

## Phase 4: Relocation columns (dual-currency + definitions) and URL ⇄ filter sync

Clusters D + E. `EWT-002`/`UWT-005` shared columns; `EWT-003`/`UWT-003`/`EWT-004` bidirectional sync.

### Phase 4, Cycle 1 — Cluster D (EWT-002: dual-currency relocation rows)

- [ ] [AI] **RED**: add test asserting city-detail relocation + liquidity rows render local **and**
      USD (`relocationSunkUsd`/`liquidityReserveUsd`) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (USD absent)

  **Gherkin (binds) →** "City detail shows relocation and liquidity figures in both local currency
  and USD"

  ```gherkin
  Scenario: City detail shows relocation and liquidity figures in both local currency and USD
    Given I am viewing a city detail
    When I read the relocation sunk-cost and liquidity-reserve rows
    Then each figure is shown in the city's local currency
    And each figure is also shown with its USD equivalent
  ```

- [ ] [AI] **GREEN**: render the USD values alongside local amounts in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: dual-currency test passes
- [ ] [AI] **REFACTOR**: consolidate the dual-currency render helper
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 4, Cycle 2 — Cluster D (UWT-005: definition tooltips on relocation headers)

- [ ] [AI] **RED**: add test asserting definition tooltips on the "Relocation (sunk)" and "Liquidity
      reserve" headers (using the `libs/web-ui` tooltip primitive) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: test fails (no tooltip)

  **Gherkin (binds) →** "Relocation and liquidity column headers carry definition tooltips"

  ```gherkin
  Scenario: Relocation and liquidity column headers carry definition tooltips
    Given I am viewing the comparison table
    When I hover or focus the "Relocation (sunk)" and "Liquidity reserve" column headers
    Then a tooltip explains what each figure includes
    And the tooltip clarifies that each is a one-time figure rather than a monthly figure
  ```

- [ ] [AI] **GREEN**: add the definition tooltips + new tooltip strings in
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: tooltip test passes
- [ ] [AI] **REFACTOR**: ensure tooltip keys follow the existing `t(locale, …)` pattern
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 4, Cycle 3 — Cluster E (EWT-003/UWT-003: URL hydration from deep link)

- [ ] [AI] **RED**: add e2e test asserting deep-link `?country=id` hydrates the Country dropdown to
      "Indonesia" and filters the table to Indonesian cities in `ayokoding-www-fe-e2e`
      (sibling pattern: existing fe-e2e specs)
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (dropdowns read "All …")

  **Gherkin (binds) →** "Filter dropdowns hydrate from URL query params on deep link"

  ```gherkin
  Scenario: Filter dropdowns hydrate from URL query params on deep link
    Given I deep-link to "/en/tools/cost-of-living-calculator?tab=cost&country=id"
    When the page resolves the deep link
    Then the Region filter is pre-selected to "ASEAN" and the Country filter to "Indonesia"
    And the table is filtered to Indonesian cities
  ```

- [ ] [AI] **GREEN**: implement URL-to-filter hydration in
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`
      (read decoded search params; pass as initial state to `GeoFilters`)
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: deep-link hydration test passes
- [ ] [AI] **REFACTOR**: extract a `useUrlSyncedFilters` hook (stub) to keep `calculator-content.tsx`
      lean; hydration test still passes — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test still passes

### Phase 4, Cycle 4 — Cluster E (EWT-004: filter writes to URL)

- [ ] [AI] **RED**: add e2e test asserting that selecting Region "ASEAN", Country "Indonesia", and
      City "Jakarta" writes those selections into the URL query params in `ayokoding-www-fe-e2e`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (URL never changes on selection)

  **Gherkin (binds) →** "Selecting filters writes the selection to the URL"

  ```gherkin
  Scenario: Selecting filters writes the selection to the URL
    Given I am on "/en/tools/cost-of-living-calculator"
    When I select Region "ASEAN", Country "Indonesia", and City "Jakarta"
    Then the URL updates to include query parameters reflecting those selections
    And opening the updated URL in a new tab restores the same filter state
  ```

- [ ] [AI] **GREEN**: implement filter-to-URL write-back in `calculator-content.tsx` via
      `useRouter`/`useSearchParams` (write selections to the URL; also hydrate initial state in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx`)
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: write-back test and round-trip restore test pass
- [ ] [AI] **REFACTOR**: complete the `useUrlSyncedFilters` hook so both hydration and write-back
      live in one place; re-run both e2e tests — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: all e2e tests pass

### Phase 4, Cycle 5 — Cluster E (city-click pre-selects City filter)

- [ ] [AI] **RED**: add e2e test asserting clicking a city name in the comparison table pre-selects
      the City filter to that city in `ayokoding-www-fe-e2e`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (City filter remains "All cities")

  **Gherkin (binds) →** "Clicking a city name pre-selects the City filter"

  ```gherkin
  Scenario: Clicking a city name pre-selects the City filter
    Given I am on the "Cost of living" tab
    When I click a city name in the comparison table
    Then the single-city detail for that city is shown
    And the City filter is pre-selected to that city
  ```

- [ ] [AI] **GREEN**: push click-derived `cityId` into the URL-synced filter state in
      `calculator-content.tsx` / `geo-filters.tsx`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: city-click pre-select test passes
- [ ] [AI] **REFACTOR**: verify all five Cluster E e2e tests still pass after the hook is complete
      — command: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: all tests still pass

### Phase 4 Gate

- [ ] [AI] `npx nx run ayokoding-www:test:unit` and `npx nx run ayokoding-www-fe-e2e:test:e2e` exit 0
- [ ] [AI] `npx nx affected -t typecheck lint` exits 0

> **Pause Safety**: relocation columns are dual-currency + defined and filter state is URL-synced;
> both are independently testable. Safe to stop. To resume:
> `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www-fe-e2e:test:e2e`.

## Phase 5: Comparison-table summary-first reorder + overflow affordance

Cluster F. `UWT-004` (chosen design-funnel Option A); folds `USS-006`. UI-bearing — match the
mockups in `assets/` and `tech-docs.md §Cluster F`.

### Phase 5, Cycle 1a — Cluster F ("Summary columns appear immediately after the City column")

- [ ] [AI] **RED**: add e2e test asserting column order is
      `Country · City · Total · Essentials · …breakdown… · Relocation · Liquidity` — in
      `apps/ayokoding-www-fe-e2e/` (new spec file alongside existing e2e specs)
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (summary columns currently at the right edge, not after City)

  **Gherkin (binds) →** "Summary columns appear immediately after the City column"

  ```gherkin
  Scenario: Summary columns appear immediately after the City column
    Given I am on the "Cost of living" tab at 1280px viewport width
    When the comparison table renders
    Then the Total and Essentials columns appear immediately after the City column
    And the per-category breakdown columns follow the summary columns
  ```

- [ ] [AI] **GREEN**: reorder the columns (Total + Essentials immediately after City) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`,
      matching `assets/ui-comparison-table-option-a-summary-first.png`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: column-order test passes
- [ ] [AI] **REFACTOR**: factor the column-config array so order is declarative
      — command: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: all tests still pass

### Phase 5, Cycle 1b — Cluster F ("Total column is visible without horizontal scrolling at desktop width")

- [ ] [AI] **RED**: add e2e test asserting the Total column is within the initial 1280px viewport
      (no horizontal scroll required) — in `apps/ayokoding-www-fe-e2e/`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (Total currently off-screen before column reorder is in place)

  **Gherkin (binds) →** "Total column is visible without horizontal scrolling at desktop width"

  ```gherkin
  Scenario: Total column is visible without horizontal scrolling at desktop width
    Given I am viewing the comparison table at 1280px viewport width with no horizontal scrolling
    When the table renders
    Then the Total column is visible within the initial viewport
    And the Essentials column is visible within the initial viewport
  ```

- [ ] [AI] **GREEN**: the column reorder from Cycle 1a brings Total into the viewport; verify no
      additional changes are needed in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: total-visibility test passes alongside column-order test
- [ ] [AI] **REFACTOR**: confirm no layout regression across unit and e2e suites
      — command: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: all tests still pass

### Phase 5, Cycle 1c — Cluster F ("Overflowing table shows a right-edge scroll affordance")

- [ ] [AI] **RED**: add e2e test asserting a visual scroll affordance is rendered at the right edge
      of the table container when the table overflows the viewport — in `apps/ayokoding-www-fe-e2e/`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (no affordance indicator present)

  **Gherkin (binds) →** "Overflowing table shows a right-edge scroll affordance"

  ```gherkin
  Scenario: Overflowing table shows a right-edge scroll affordance
    Given the comparison table extends beyond the viewport width
    When the right edge of the table container is reached visually
    Then a visual indicator signals that additional columns exist to the right
  ```

- [ ] [AI] **GREEN**: add the right-edge scroll affordance indicator in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: affordance test passes alongside all prior Cycle 1a/1b tests
- [ ] [AI] **REFACTOR**: ensure the affordance style is co-located with the column-config so future
      column changes cannot silently break it
      — command: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: all tests still pass

- [ ] [AI] Verify visual parity against the three mockups (desktop/tablet/mobile) with Playwright MCP
      `browser_take_screenshot` at 1280/768/375 px
      — acceptance: layout matches `assets/ui-comparison-table-option-a-summary-first{,-tablet,-mobile}.png`
      per breakpoint; results recorded in this file

### Phase 5 Gate

- [ ] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` exits 0 (column order + total visibility)
- [ ] [AI] Visual-parity screenshots recorded for all three breakpoints
- [ ] [AI] `npx nx affected -t typecheck lint` exits 0

> **Pause Safety**: the primary UI change is complete, tested, and visually signed off. Safe to stop.
> To resume: `npx nx run ayokoding-www-fe-e2e:test:e2e`.

## Phase 6: Naming, metadata, accessibility, comprehension polish, /tools, security

Clusters H, I, J, K, L (+ `UWT-012` label fix if `UWT-001` was VOIDed in Phase 1).

### Phase 6, Cycle 1 — Cluster H (EWT-012: sort aria-pressed)

- [ ] [AI] **RED**: add test for savings sort `aria-pressed` state in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (`aria-pressed` absent)

  **Gherkin (binds) →** "Savings sort control exposes its state to assistive technology"

  ```gherkin
  Scenario: Savings sort control exposes its state to assistive technology
    Given I am on the "Savings" tab
    When I read the sort control in the accessibility tree
    Then the control exposes its current sort direction via aria-pressed or aria-sort
  ```

- [ ] [AI] **GREEN**: add `aria-pressed={sortAsc}` to the sort control in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: aria-pressed test passes
- [ ] [AI] **REFACTOR**: ensure `aria-pressed` value updates reactively with sort state
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 2 — Cluster H (EWT-014: visible mobile sort control)

- [ ] [AI] **RED**: add test for a reachable mobile sort control with no hidden desktop sort button
      in keyboard tab order, in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (hidden desktop button in tab order)

  **Gherkin (binds) →** "A visible sort control is reachable in the mobile savings layout"

  ```gherkin
  Scenario: A visible sort control is reachable in the mobile savings layout
    Given I am on the "Savings" tab at 375px viewport width
    When the mobile card layout renders
    Then a visible, tappable sort control is present in the mobile layout
    And no hidden desktop-only sort button remains in the keyboard tab order
  ```

- [ ] [AI] **GREEN**: add a visible mobile sort toggle and remove the hidden desktop button from
      tab order in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: mobile-sort test passes
- [ ] [AI] **REFACTOR**: consolidate the sort control render paths
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 3 — Cluster I (UWT-002: H1 subtitle)

- [ ] [AI] **RED**: add test asserting the heading area includes an H1 "Salary Savings Calculator"
      and a subtitle describing it as a cost-of-living comparison tool, in the calculator-content
      test file — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (no subtitle)

  **Gherkin (binds) →** "A subtitle ties the H1 to the cost-of-living purpose"

  ```gherkin
  Scenario: A subtitle ties the H1 to the cost-of-living purpose
    Given I am on "/en/tools/cost-of-living-calculator"
    When the page renders its heading area
    Then the H1 still reads "Salary Savings Calculator"
    And a subtitle describes it as a cost-of-living comparison tool
  ```

- [ ] [AI] **GREEN**: add the subtitle in
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: subtitle test passes
- [ ] [AI] **REFACTOR**: ensure the subtitle string is in `translations.ts` and locale-aware
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 4 — Cluster I/J (UWT-007: descriptive page title)

- [ ] [AI] **RED**: add test for `generateMetadata` producing a descriptive `<title>` naming the
      tool in the calculator route test
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (title is "AyoKoding")

  **Gherkin (binds) →** "Page title names the tool on load"

  ```gherkin
  Scenario: Page title names the tool on load
    Given I navigate to the cost-of-living calculator with default filter state
    When the page finishes loading
    Then the browser tab title names the tool rather than only "AyoKoding"
  ```

- [ ] [AI] **GREEN**: add `generateMetadata` to
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/page.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: descriptive-title test passes
- [ ] [AI] **REFACTOR**: verify the metadata title is locale-aware and composes with the
      `"%s | AyoKoding"` template — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all unit tests still pass

### Phase 6, Cycle 5 — Cluster J (UWT-012: predictive tab labels)

- [ ] [AI] **RED**: add test for predictive tab labels ("Savings" / "Minimum role" carry information
      scent) in the calculator-content test file
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (bare ambiguous labels)

  **Gherkin (binds) →** "Tab labels carry predictive information scent"

  ```gherkin
  Scenario: Tab labels carry predictive information scent
    Given I am on the calculator
    When I read the "Savings" and "Minimum role" tab labels
    Then each label or its subtitle predicts the panel content rather than using a bare ambiguous word
  ```

- [ ] [AI] **GREEN**: add predictive tab-label strings in
      `apps/ayokoding-www/src/features/i18n/core/translations.ts` and consume them in
      `calculator-content.tsx` — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: tab-label test passes
- [ ] [AI] **REFACTOR**: confirm tab-label strings are locale-aware
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 6 — Cluster J (UWT-009: 44px controls)

- [ ] [AI] **RED**: add test asserting interactive controls meet the 44px preferred target height at
      viewports narrower than 768px, in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: test fails

  **Gherkin (binds) →** "Mobile interactive controls meet the 44px preferred target height"

  ```gherkin
  Scenario: Mobile interactive controls meet the 44px preferred target height
    Given I am on the calculator at a viewport narrower than 768px
    When an interactive control renders
    Then the control has a minimum height of at least 44px
  ```

- [ ] [AI] **GREEN**: add `min-h-[44px]` to interactive controls in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: 44px test passes
- [ ] [AI] **REFACTOR**: verify no layout regression at desktop widths
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 7 — Cluster J (UWT-010: ID Area label no-wrap)

- [ ] [AI] **RED**: add test asserting the Indonesian Area label fits on one line at 375px, in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: test fails

  **Gherkin (binds) →** "The Indonesian Area label does not reflow the city-center toggle at 375px"

  ```gherkin
  Scenario: The Indonesian Area label does not reflow the city-center toggle at 375px
    Given I am on "/id/tools/cost-of-living-calculator" at 375px viewport width
    When the Area control renders
    Then the Area label fits on one line without wrapping the city-center and rural toggle onto a new row
  ```

- [ ] [AI] **GREEN**: add `whitespace-nowrap` or equivalent to the Area label in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: no-wrap test passes
- [ ] [AI] **REFACTOR**: confirm no unintended wrapping in the English layout
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 8 — Cluster J (UWT-011: sentence-cased badges + taxonomy tooltip)

- [ ] [AI] **RED**: add test asserting healthcare-scheme badges are sentence-cased and the column
      header carries a taxonomy-defining tooltip, in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: test fails

  **Gherkin (binds) →** "Healthcare scheme badges are sentence-cased and defined"

  ```gherkin
  Scenario: Healthcare scheme badges are sentence-cased and defined
    Given I am on the calculator
    When I read a healthcare-scheme badge
    Then the badge text is sentence-cased rather than all-caps
    And a header tooltip defines the healthcare-scheme taxonomy
  ```

- [ ] [AI] **GREEN**: sentence-case badge text and add the taxonomy tooltip in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx` and
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: badge and tooltip tests pass
- [ ] [AI] **REFACTOR**: confirm badge sentence-casing is data-driven, not hardcoded
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 9 — Cluster J (UWT-014: `<abbr>`-wrapped OOP)

- [ ] [AI] **RED**: add test asserting the "OOP" abbreviation in the Healthcare column is wrapped
      in an `<abbr title="out-of-pocket">` element, in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.test.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: test fails

  **Gherkin (binds) →** "The OOP abbreviation is wrapped for assistive tech"

  ```gherkin
  Scenario: The OOP abbreviation is wrapped for assistive tech
    Given I am on a tab that shows the "Healthcare (OOP)" column
    When I read the OOP abbreviation
    Then it is wrapped in an abbr element whose title expands to "out-of-pocket"
  ```

- [ ] [AI] **GREEN**: wrap OOP in `<abbr title="out-of-pocket">` in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: abbr-wrap test passes
- [ ] [AI] **REFACTOR**: confirm no visual regression in the column header
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all unit tests still pass

### Phase 6, Cycle 10 — Cluster K (UWT-013: /tools index route)

- [ ] [AI] **RED**: add e2e test asserting `/en/tools` resolves (not 404) and the page links to
      the cost-of-living calculator in `ayokoding-www-fe-e2e`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance: test fails (404)

  **Gherkin (binds) →** "The parent tools URL resolves instead of returning 404"

  ```gherkin
  Scenario: The parent tools URL resolves instead of returning 404
    Given I navigate to "/en/tools"
    When the page resolves
    Then an index page is shown rather than an HTTP 404
    And it links to the cost-of-living calculator
  ```

- [ ] [AI] **GREEN**: add `apps/ayokoding-www/src/app/[locale]/tools/page.tsx` (_New file_) —
      minimal index linking to the calculator
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: index resolves; e2e test passes
- [ ] [AI] **REFACTOR**: verify the tools index page works for both `/en/tools` and `/id/tools`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance: all e2e tests pass

### Phase 6, Cycle 11 — Cluster L (EWT-013: security headers)

- [ ] [AI] **RED**: add a header-assertion e2e test (security headers present, `X-Powered-By`
      absent) for `EWT-013` in `ayokoding-www-fe-e2e`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: assertion fails (headers absent)

  **Gherkin (binds) →** "Responses carry baseline security headers and omit the framework banner"

  ```gherkin
  Scenario: Responses carry baseline security headers and omit the framework banner
    Given the ayokoding-www app serves a calculator route
    When I inspect the HTTP response headers
    Then the response includes Content-Security-Policy, X-Content-Type-Options, frame-ancestors protection, and Referrer-Policy
    And the response does not include an X-Powered-By header
  ```

- [ ] [AI] **GREEN**: add the `headers()` block + `poweredByHeader: false` in
      `apps/ayokoding-www/next.config.ts` (CSP, `X-Content-Type-Options: nosniff`,
      frame-ancestors, `Referrer-Policy`)
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: header assertion passes
- [ ] [AI] **REFACTOR**: tidy the `next.config.ts` header block; confirm no other config regression
      — command: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: all tests still pass

### Phase 6 Gate

- [ ] [AI] `npx nx run ayokoding-www:test:unit` and `npx nx run ayokoding-www-fe-e2e:test:e2e` exit 0
- [ ] [AI] `curl -sI http://localhost:3101/en/tools/cost-of-living-calculator` shows the four
      security headers and no `X-Powered-By`
- [ ] [AI] `npx nx affected -t typecheck lint` exits 0

> **Pause Safety**: all remaining findings are fixed and tested; the app is coherent. Safe to stop.
> To resume: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www-fe-e2e:test:e2e`.

## Phase 7: Specs & Gherkin Delivery (fold SG + reconciled USS) + EWT-015 reconciliation

Per locked decision 4 and `tech-docs.md §Specs & Gherkin reconciliation`. Suggested executor:
`specs-maker`.

- [ ] [AI] **RED**: add `SG-001..007` scenarios and the reconciled net-new `USS-002`/`USS-003`/
      `USS-005`/`USS-006` scenarios (plus the Cluster A/C net-new scenarios) to
      `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`,
      dropping `USS-001` (void unless tabs broken) and `USS-004` (duplicate of Cluster A)
      — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: new scenarios present; coverage fails (step defs not yet wired)
- [ ] [AI] **GREEN**: implement/extend the step definitions consuming those scenarios
      — command: `npx nx run ayokoding-www:specs:coverage` — acceptance: exits 0
- [ ] [AI] **REFACTOR**: tidy any duplicated step-definition patterns introduced when wiring the
      new scenarios; remove dead or redundant step matchers
      — command: `npx nx run ayokoding-www:specs:coverage` — acceptance: exits 0; step defs cleaner
- [ ] [AI] Reconcile `EWT-015` (confidence-flag): either implement the
      `[data-testid="confidence-flag"]` affordance to match the existing "Low-confidence cells are
      flagged" scenario, OR retire/adjust that scenario with a recorded rationale in this file
      — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: the spec and the live DOM agree; decision recorded
- [ ] [AI] Update any `specs/**` README / C4 inventory if the surface changed
      — acceptance: affected `specs/**` docs reflect the changes (or none needed — recorded)

### Phase 7 Gate

- [ ] [AI] `npx nx run ayokoding-www:specs:coverage` exits 0
- [ ] [AI] Every `SG-###` and reconciled `USS-###` disposition matches
      `tech-docs.md §Specs & Gherkin reconciliation`; `EWT-015` decision recorded

> **Pause Safety**: specs are folded and coverage is green; the feature file and implementation
> agree. Safe to stop. To resume: `npx nx run ayokoding-www:specs:coverage`.

## Phase 8: Quality gates, commit, push, CI

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t specs:coverage`
- [ ] [AI] Run FE-E2E: `npx nx run ayokoding-www-fe-e2e:test:e2e`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes
- [ ] [AI] Re-run failing checks to confirm resolution; verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit thematically (locale, numbers, relocation/sync, table reorder, polish, specs as
      separate commits) — Conventional Commits `<type>(<scope>): <description>`
- [ ] [AI] Keep preexisting fixes in their own commits, separate from plan work

### Push and Post-Push CI Verification

- [ ] [AI] Commit and push to origin main
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 min; do NOT use
      `gh run watch`)
- [ ] [AI] Verify ALL CI checks pass — no exceptions; fix and push follow-ups until green
- [ ] [AI] Do NOT proceed to archival until CI is fully green

### Phase 8 Gate

- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` exits 0 locally
- [ ] [AI] All GitHub Actions for the push are green

> **Pause Safety**: changes are pushed and CI is green; the branch is in a shippable state. Safe to
> stop. To resume: `gh run list --branch main --limit 5`.

## Phase 9: Rule-15 retest follow-ups

> Per [User-Facing Delivery Hardening rule 15](../../../repo-governance/development/quality/user-facing-delivery-hardening.md):
> after the fixes land and visual sign-off is recorded, run one `web-exploratory-tester` round
> against the running URL and resolve every new finding before archival.

- [ ] [AI] Start the dev server: `npx nx dev ayokoding-www`
      — acceptance: reachable at `http://localhost:3101`
- [ ] [AI] Run one `web-exploratory-tester` round against
      `http://localhost:3101/{en,id}/tools/cost-of-living-calculator` at 375/768/1280 px
      — acceptance: a fresh findings list is produced
- [ ] [AI] Append each new finding below as an unchecked `- [ ]` task-list checkbox under
      "Rule-15 retest findings", then fix and tick each (RED→GREEN→REFACTOR for code) before archival
      — acceptance: the section exists and every appended item is fixed and ticked

### Rule-15 retest findings

_Populated during Phase 9 by the `web-exploratory-tester` round. Each finding is appended here as an
unchecked checkbox and must be fixed and ticked before archival._

- [ ] [AI] (placeholder — replace with the first retest finding, or delete if the round is clean)

### Phase 9 Gate

- [ ] [AI] Every Rule-15 retest finding is fixed and ticked (or the round was clean, recorded as such)
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` still exits 0

> **Pause Safety**: the retest round is complete and its findings resolved. Safe to stop. To resume:
> re-read the Rule-15 retest findings section.

## Phase 10: Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked (including Rule-15 follow-ups)
- [ ] [AI] Verify ALL quality gates pass (local + CI) and visual parity is signed off
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/ayokoding-www-cost-of-living-calc-test-fixing/ plans/done/YYYY-MM-DD__ayokoding-www-cost-of-living-calc-test-fixing/`
      using today's date as the completion date
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan
- [ ] [AI] Commit the archival: `chore(plans): move ayokoding-www-cost-of-living-calc-test-fixing to done`

### Phase 10 Gate

- [ ] [AI] The plan folder lives under `plans/done/YYYY-MM-DD__…/` and the README indexes are updated
- [ ] [AI] The archival commit is pushed and CI is green

> **Pause Safety**: the plan is archived and pushed. Work is complete. To resume (if needed): confirm
> the folder is under `plans/done/` and CI is green.
