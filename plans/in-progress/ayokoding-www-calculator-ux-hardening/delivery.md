# Delivery Checklist — Calculator UX Hardening

TDD-shaped (RED → GREEN → REFACTOR). Each code step names a file path, a verbatim command, and an
acceptance criterion. Steps are tagged `[AI]` (autonomous) or `[HUMAN]` (needs a human decision). Phase 0
runs first. Phases are gated: do not start a phase until the prior phase's gate is green. Every behavioural
fix lands its reproducing test in the same commit (regression-test mandate); specs are folded in Phase 8
(feature-change-completeness).

**App**: `apps/ayokoding-www` · **Feature**: `src/features/cost-of-living-calculator/` + the route
`src/app/[locale]/tools/cost-of-living-calculator/`.

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate`: a must-pass verification
> checklist plus a **Pause Safety** note (the safe-to-stop state after the phase and the
> single command to resume). A phase is **not complete until its gate is green**; do not start
> phase N+1 while any check in phase N's gate is failing.

Common commands:

- Unit: `npx nx run ayokoding-www:test:unit`
- Typecheck: `npx nx run ayokoding-www:typecheck`
- Lint: `npx nx run ayokoding-www:lint`
- Specs coverage: `npx nx run ayokoding-www:specs:coverage`

---

## Worktree

This plan executes **directly on `main`** (no separate worktree), per explicit user directive. All
changes are committed directly to `origin/main` following Trunk Based Development.

> **Deliberate deviation**: the optional manual pre-provisioning path
> (`claude --worktree ayokoding-www-calculator-ux-hardening`) is intentionally **not** used here — the
> user directed in-place execution on `main`. See the
> [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
> [Plans Organization Convention](../../../repo-governance/conventions/structure/plans.md) for the
> standard worktree-based path when a future execution wants isolation.

---

## Phase 0 — Baseline (gate: all green before any fix) `[AI]`

- [ ] 0.1 `[AI]` Confirm clean tree + dev server reachable (`curl -s -o /dev/null -w "%{http_code}" http://localhost:3101/en/tools/cost-of-living-calculator` → 200).
- [ ] 0.2 `[AI]` `npm install` and `npm run doctor -- --fix` (toolchain converged).
- [ ] 0.3 `[AI]` Baseline: `npx nx run ayokoding-www:typecheck && npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:specs:coverage` — record the baseline pass counts. Resolve any pre-existing failure before proceeding.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] `[AI]` `npx nx run ayokoding-www:typecheck` — exits 0.
- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, baseline pass count recorded.
- [ ] `[AI]` `npx nx run ayokoding-www:specs:coverage` — exits 0.

> **Pause Safety**: Baseline recorded. Toolchain confirmed. Safe to stop. To resume: rerun 0.3 and
> compare pass counts.

---

## Phase 1 — Tab descriptions (EWT-001 ≡ DWT-001, Major) `[AI]`

File: `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`
(L263/270/276) + test `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-content.test.tsx`.

- [ ] 1.1 **RED** `[AI]` Add a unit test asserting an inactive tab description carries the `hidden` class
      (and the active one does not).
      File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-content.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: test fails against current `text-muted-foregroundhidden`.
      **Gherkin (binds) →** "Only the active tab description is visible"

      ```gherkin
      Scenario: Only the active tab description is visible
        Given the cost-of-living calculator is open with the "Cost of living" tab active
        When the page is rendered
        Then the "Cost of living" tab description is visible
        And the "Savings" tab description is not visible
        And the "Minimum role" tab description is not visible
      ```

- [ ] 1.2 **GREEN** `[AI]` Insert the missing space in
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`:
      `` `mt-1 text-sm text-muted-foreground ${activeTab === "…" ? "" : "hidden"}` `` on all three lines.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: 1.1 passes; only the active description is visible.
- [ ] 1.3 **REFACTOR** `[AI]` Extract the description className to a small `cn(...)` helper / `clsx` so the
      bug class cannot recur in
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`.
      Cmd: `npx nx run ayokoding-www:typecheck`. AC: typecheck clean, behaviour unchanged.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, new test 1.1 passing.
- [ ] `[AI]` `npx nx run ayokoding-www:typecheck` — exits 0.

> **Pause Safety**: Tab-description regression fixed and guarded. Safe to stop. To resume:
> `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:typecheck`.

---

## Phase 2 — Accessibility: touch targets + ARIA state (EWT-002, EWT-005, UWT-008, UWT-011) `[AI]`

Files: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx`,
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`,
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`,
route `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`;
tests alongside.

- [ ] 2.1 **RED** `[AI]` Test: every segmented-radio button + tab trigger has `min-h-[44px]` (extend the
      existing `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.test.tsx`
      radiogroup test to cover tab triggers + salary-currency + 28px buttons).
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (28–29px controls).
      **Gherkin (binds) →** "Interactive controls meet the 44px touch target"

      ```gherkin
      Scenario: Interactive controls meet the 44px touch target
        Given the calculator at 375px
        When the page is rendered
        Then every tab trigger is at least 44px tall
        And every school-type, area, and salary-currency segmented radio is at least 44px tall
      ```

- [ ] 2.2 **GREEN** `[AI]` Add `min-h-[44px]` + centring to the `SegmentedControl` primitive in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx` (so school-type/area/
      baseline/salary-currency inherit) and ensure tab triggers reach 44px at mobile. AC: 2.1 passes.
- [ ] 2.3 **RED** `[AI]` Test: Area toggle options expose active state via ARIA (`aria-checked` if
      radiogroup, else `aria-pressed`) + a non-colour active indicator class in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (`aria-pressed:null`).
      **Gherkin (binds) →** "Area toggle exposes its pressed state"

      ```gherkin
      Scenario: Area toggle exposes its pressed state
        Given "City center" is the active area
        When the page is rendered
        Then the "City center" button has aria-pressed "true"
        And the "Rural" button has aria-pressed "false"
      ```

- [ ] 2.4 **GREEN** `[AI]` Reconcile the segmented control to one ARIA pattern in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx` and emit the state
      attribute + a non-colour active indicator (ring/underline). AC: 2.3 passes. (Implements USS-003.)
- [ ] 2.5 **RED** `[AI]` Test: disabled school-type buttons have `aria-disabled="true"` +
      `aria-describedby="school-type-hint"`, and the hint element has `id="school-type-hint"` in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
      **Gherkin (binds) →** "Disabled school-type buttons announce the prerequisite"

      ```gherkin
      Scenario: Disabled school-type buttons announce the prerequisite
        Given "School-age children" is 0
        When the page is rendered
        Then the "Public" and "Private" buttons are aria-disabled
        And their accessible description names the "add school-age children" prerequisite
      ```

- [ ] 2.6 **GREEN** `[AI]` Wire `id` on the hint + `aria-describedby`/`aria-disabled` on both disabled
      buttons in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx`.
      AC: 2.5 passes. (Implements USS-004.)
- [ ] 2.7 **RED** `[AI]` Test: the sortable savings `<th>` has `aria-sort` reflecting
      none/ascending/descending.
      File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
      **Gherkin (binds) →** "Sortable savings column exposes aria-sort"

      ```gherkin
      Scenario: Sortable savings column exposes aria-sort
        Given the Savings tab table is shown
        When the page is rendered
        Then the sortable "Savings after essentials" column header has an aria-sort value
      ```

- [ ] 2.8 **GREEN** `[AI]` Add `aria-sort` to the sort header `<th>` in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`. AC: 2.7 passes.
- [ ] 2.9 **REFACTOR** `[AI]` Dedupe any repeated 44px/ARIA wiring into the primitive in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx`.
      Cmd: `npx nx run ayokoding-www:lint`. AC: lint clean.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, all new 2.1/2.3/2.5/2.7 tests passing.
- [ ] `[AI]` `npx nx run ayokoding-www:lint` — exits 0.

> **Pause Safety**: All accessibility touch-target and ARIA-state fixes applied and tested. Safe to stop.
> To resume: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:lint`.

---

## Phase 3 — Foreigner public-school flag (EWT-003, UWT-002, DWT-006) `[AI]`

Files: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`,
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx`,
`apps/ayokoding-www/src/features/i18n/core/translations.ts`; tests + steps.
See `assets/ui-foreigner-flag-low-fi.md`.

- [ ] 3.1 **RED** `[AI]` Test: city-detail renders
      `data-testid="school-foreigner-flag-<cityId>"` when the fallback applies (extend
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.test.tsx` + the fe-step
      for Singapore). AC: fails (testid absent).
      **Gherkin (binds) →** "Foreigner-school flag is clear, styled, and present in both views"

      ```gherkin
      Scenario: Foreigner-school flag is clear, styled, and present in both views
        Given a city whose country does not open public school to foreigners
        And school-age children >= 1 and school type "public"
        When the page is rendered
        Then the cost-of-living table school cell shows a clearly-worded private-fallback flag
        And the flag is visually distinct from ordinary caption text
        And the city-detail school row renders the school-foreigner-flag-<cityId> testid
      ```

- [ ] 3.2 **GREEN** `[AI]` Add the flag span to the city-detail school row when `schoolForeignerFallback`
      in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx`.
      AC: 3.1 passes (fixes EWT-003).
- [ ] 3.3 **RED** `[AI]` Test: the flag uses the reworded localized label (new i18n keys) and a
      warning-tone class/`Badge` (assert `text-warning`/badge testid, not `text-muted-foreground`);
      both locales in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
- [ ] 3.4 **GREEN** `[AI]` Add new i18n keys to
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`
      (en "Private — public not open to foreigners" / id "Swasta — negeri tak terbuka untuk WNA"),
      render via `Badge`/warning token in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx` table +
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx`.
      AC: 3.3 passes (UWT-002 + DWT-006).
- [ ] 3.5 **REFACTOR** `[AI]` Factor the flag into one shared sub-component used by both
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx` +
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/city-detail.tsx` so they cannot drift.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: green.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, all new 3.1/3.3 tests passing.
- [ ] `[HUMAN]` Manual check: Singapore/Dubai/Jakarta show the badge in both views + both locales; Berlin
      shows none. Observable resume signal: badge visually present in browser; verify with dev server
      at `http://localhost:3101/en/tools/cost-of-living-calculator`.

> **Pause Safety**: Foreigner-school flag parity fixed across table + city-detail, both locales. Safe to
> stop. To resume: `npx nx run ayokoding-www:test:unit`.

---

## Phase 4 — Jargon glosses & i18n labels (EWT-004, UWT-001/003/004/009/010/012/013/014) `[AI]`

Files: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`,
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`,
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx`,
`apps/ayokoding-www/src/features/i18n/core/translations.ts`.

- [ ] 4.1a **RED** `[AI]` Write failing test: the id-locale OOP header `title` attribute equals
      `t(locale, "healthcareOutOfPocket")`, not the literal `"out-of-pocket"`.
      File: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: test fails (title is literal English string).
      **Gherkin (binds) →** "Jargon table headers carry an accessible explanation"

      ```gherkin
      Scenario: Jargon table headers carry an accessible explanation
        Given the calculator is open
        When the page is rendered
        Then the "Healthcare (OOP)" header has a title explaining out-of-pocket (localized)
        And the "Relocation (sunk)" and "Liquidity reserve" headers carry explanatory titles
        And the "P25"/"Median"/"P75" headers carry percentile explanations
        And the "Track" column abbreviations ic/mgmt are expanded or carry abbr titles
      ```

- [ ] 4.1b **GREEN** `[AI]` Fix `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx:131`
      and `:298` — change `title="out-of-pocket"` to `title={t(locale, "healthcareOutOfPocket")}`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: 4.1a passes; id locale shows "bayar sendiri" (EWT-004/UWT-014).
- [ ] 4.2a **RED** `[AI]` Write failing test: the "Baseline source" label in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` uses a
      scent-bearing replacement string (new i18n value); assert it in both locales.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (still shows "Baseline source").
- [ ] 4.2b **GREEN** `[AI]` Relabel "Baseline source" → scent-bearing label (new i18n value "How to set
      your target" / id equivalent) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` +
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`; test asserts the new label text both
      locales. AC: green (UWT-001).
- [ ] 4.3a `[AI]` Inspect
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx:137,140` —
      confirm the Relocation(sunk)/Liquidity-reserve `<abbr title>` render a usable tooltip
      (keyboard-focusable via `tabindex="0"` or native abbr behaviour).
      AC: record verdict as a comment in this delivery file: "tooltip present + keyboard-accessible" or
      "needs visible affordance".
- [ ] 4.3b **RED** `[AI]` (conditional — only if verdict = "needs visible affordance") Write failing test
      asserting a visible info icon/button affording the tooltip.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
- [ ] 4.3c **GREEN** `[AI]` (conditional) Implement the visible info affordance.
      AC: 4.3b passes; both locales.
- [ ] 4.4a **RED** `[AI]` Write failing test: P25/Median/P75 headers in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` have `title` glosses
      (new i18n keys) present in both locales.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
- [ ] 4.4b **GREEN** `[AI]` Add `title`/`<abbr>` glosses to P25/Median/P75 headers in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` + new i18n keys in
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`; test asserts titles present.
      AC: green (UWT-010).
- [ ] 4.5a **RED** `[AI]` Write failing test: no bare "ic"/"mgmt" Track column value in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
- [ ] 4.5b **GREEN** `[AI]` Expand ic/mgmt Track values (localized full words or `<abbr title>`) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` +
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`; test asserts no bare "ic"/"mgmt".
      AC: green (UWT-013).
- [ ] 4.6a **RED** `[AI]` Write failing test: Non-salary-comp header is shortened + has `title` expansion
      in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
- [ ] 4.6b **GREEN** `[AI]` Shorten Non-salary-comp header + add `title` expansion in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx` +
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`; test asserts header text + title.
      AC: green (UWT-003).
- [ ] 4.7a **RED** `[AI]` Write failing test: every region option in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx` is in Indonesian
      (id locale); MENA/Nordics are expanded or carry a title in both locales. **Assert URL round-trip
      unchanged** (serialized region key stays English).
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (region names not translated).
- [ ] 4.7b **GREEN** `[AI]` Localize region option display names in id + expand MENA/Nordics in both
      locales in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx` +
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`; **keep the serialized region key
      English** (URL stability — assert URL round-trip unchanged). AC: green (UWT-004).
- [ ] 4.8a **RED** `[AI]` Write failing test: no healthcare-scheme badge is rendered in ALL CAPS in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (badge shows ALL-CAPS).
- [ ] 4.8b **GREEN** `[AI]` Normalize healthcare-scheme badge casing to sentence-case in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx`;
      test asserts no ALL-CAPS scheme label. AC: green (UWT-012).

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, all new 4.x tests passing.
- [ ] `[AI]` URL round-trip for region filter unchanged — assert by running the URL-round-trip test.

> **Pause Safety**: Jargon glosses, i18n labels, and badge casing fixed. URL round-trip preserved. Safe
> to stop. To resume: `npx nx run ayokoding-www:test:unit`.

---

## Phase 5 — UX states (UWT-005, UWT-006, UWT-007) `[AI]`

Files: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`,
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`,
route `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`.

- [ ] 5.1a **RED** `[AI]` Write failing test: Savings empty-state shows a bordered prompt panel
      (`data-testid="savings-empty-state"`) + the gross input auto-focuses on Savings-tab activation in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (panel testid absent, no focus).
      **Gherkin (binds) →** "Savings tab guides the user to enter a salary"

      ```gherkin
      Scenario: Savings tab guides the user to enter a salary
        Given the Savings tab is activated with no salary entered
        When the tab activation occurs
        Then a prominent empty-state prompt is shown in the data area
        And the gross salary input receives focus
      ```

- [ ] 5.1b **GREEN** `[AI]` Add bordered prompt panel in the data area + auto-focus the gross input on
      Savings-tab activation in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`.
      AC: 5.1a passes (UWT-005 / USS-001).
- [ ] 5.2a **RED** `[AI]` Write failing test: Min-role pre-target panel is labelled "Example (<city>)"
      (localized) or suppressed until target in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (example caption absent).
      **Gherkin (binds) →** "Minimum-role pre-target panel is labelled as an example"

      ```gherkin
      Scenario: Minimum-role pre-target panel is labelled as an example
        Given the Minimum-role tab is activated with no target entered
        When the page is rendered
        Then any pre-populated city cost panel is labelled as an example (or hidden)
      ```

- [ ] 5.2b **GREEN** `[AI]` Label Min-role pre-target panel "Example (<city>)" (localized) or suppress
      until target in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` +
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`; test asserts the example caption
      present pre-target. AC: green (UWT-006 / USS-002).
- [ ] 5.3a **RED** `[AI]` Write failing test: the gross salary input in Savings tab shows an at-field
      "USD" indicator in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails (no inline currency adornment).
      **Gherkin (binds) →** "Savings salary input shows its currency at the field"

      ```gherkin
      Scenario: Savings salary input shows its currency at the field
        Given the Savings tab is shown
        When the page is rendered
        Then the gross salary input displays its USD currency inline at the field
      ```

- [ ] 5.3b **GREEN** `[AI]` Add at-field "USD" indicator to Savings gross input in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx`; test asserts the
      inline currency adornment. AC: green (UWT-007).

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, all new 5.x tests passing.
- [ ] `[AI]` Debounce + scroll-preservation tests still pass — no regression (grep for relevant test names
      in `npx nx run ayokoding-www:test:unit` output).

> **Pause Safety**: UX states (empty-state, example-panel, at-field currency) fixed. Safe to stop. To
> resume: `npx nx run ayokoding-www:test:unit`.

---

## Phase 6 — Design-system fidelity (DWT-002, DWT-003, DWT-004, DWT-007) `[AI]`

Files: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx`,
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`;
`libs/web-ui` `SelectField` reuse.
See `assets/ui-select-chrome-low-fi.md`, `assets/ui-baseline-source-mobile-low-fi.md`.

- [ ] 6.1a **RED** `[AI]` Write failing test: every `<select>` in the calculator has the styled class /
      no native-arrow class (assert `appearance-none` and custom chevron wrapper) in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.test.tsx` and
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.test.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
      **Gherkin (binds) →** "All selects share the design-system chrome"

      ```gherkin
      Scenario: All selects share the design-system chrome
        Given the calculator at 1280px
        When the page is rendered
        Then every <select> has computed appearance "none" and a custom chevron affordance
        And no <select> shows the browser's native dropdown arrow
      ```

- [ ] 6.1b **GREEN** `[AI]` Wrap all household + min-role currency/ref selects in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx` and
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` in
      `SelectField`/`GEO_SELECT_CLASS`; test asserts every `<select>` has the styled class / no
      native-arrow class. AC: green (DWT-002/003, SG-002).
- [ ] 6.2a **RED** `[AI]` Write failing test: Baseline-source segmented control in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.test.tsx` height does
      not exceed 44px per row at 320/375px (component height ≤ 44 per row).
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
      **Gherkin (binds) →** "Baseline-source control keeps the 44px rhythm at mobile"

      ```gherkin
      Scenario: Baseline-source control keeps the 44px rhythm at mobile
        Given the Minimum-role tab at 320px and 375px
        When the page is rendered
        Then the "Baseline source" segmented control height does not exceed 44px
      ```

- [ ] 6.2b **GREEN** `[AI]` Add `flex-wrap` keeping each option 44px to the Baseline-source segmented
      control in `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`; test
      asserts ≤44px per-row at 320/375. AC: green (DWT-004, SG-003).
- [ ] 6.3a **RED** `[AI]` Write failing test: Salary-currency toggle in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.test.tsx` is bottom-aligned
      (`items-end` class structure) with its sibling input.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
      **Gherkin (binds) →** "Salary-currency toggle bottom-aligns with its sibling input"

      ```gherkin
      Scenario: Salary-currency toggle bottom-aligns with its sibling input
        Given the Minimum-role "My salary" baseline at 1280px
        When the page is rendered
        Then the salary-currency toggle bottom-aligns with the gross salary input
      ```

- [ ] 6.3b **GREEN** `[AI]` Ensure salary-currency toggle `fieldGroup` is a direct `items-end` flex child + label is `<label>` in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`; test asserts the
      alignment class structure. AC: green (DWT-007).
- [ ] 6.4 **REFACTOR** `[AI]` Consolidate select styling on the single `SelectField` primitive across
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/controls.tsx` and
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`.
      Cmd: `npx nx run ayokoding-www:lint`. AC: lint clean.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, all new 6.x tests passing.
- [ ] `[AI]` `npx nx run ayokoding-www:lint` — exits 0.
- [ ] `[HUMAN]` Manual select-chrome check at 320/375/1280 — custom chevron visible, no native arrow.
      Observable resume signal: all selects show custom chevron in browser; verify via dev server.

> **Pause Safety**: Design-system fidelity (select chrome, baseline-source wrap, salary-currency
> alignment) fixed. Safe to stop. To resume: `npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:lint`.

---

## Phase 7 — Security/CSP (EWT-006) `[HUMAN]` decision, then `[AI]`

File: `apps/ayokoding-www/next.config.ts`.

- [ ] 7.1 `[HUMAN]` Decide: **keep GA** (whitelist `googletagmanager.com` + `google-analytics.com` in
      CSP `script-src`/`connect-src`/`img-src`) **or remove** the GA tag if analytics are unwanted.
      Default if no answer: **keep + whitelist** (the tag already ships). Record the decision here.
      Observable resume signal: decision recorded in this checklist line; proceed to 7.2.
- [ ] 7.2a **RED** `[AI]` Per 7.1 decision: write failing test asserting either (a) the CSP directive
      includes the GA origins (`googletagmanager.com`/`google-analytics.com`) or (b) no `gtag` script
      is present in `apps/ayokoding-www/next.config.ts`.
      Cmd: `npx nx run ayokoding-www:test:unit`. AC: fails.
- [ ] 7.2b **GREEN** `[AI]` Per 7.1: either extend the CSP in `apps/ayokoding-www/next.config.ts`
      (assert the directive includes the GA origins) or remove the GA script (assert no `gtag` script).
      AC: no CSP-violation console error on calculator load; other directives unchanged.

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, 7.2a passing.
- [ ] `[HUMAN]` Verify no console CSP error on calculator load in browser.
      Observable resume signal: browser devtools console shows zero CSP errors; verify via dev server.

> **Pause Safety**: CSP decision recorded and implemented. No console CSP error. Safe to stop. To
> resume: start dev server + check browser console on calculator page.

---

## Phase 8 — Specs fold-in + coverage (feature-change-completeness) `[AI]`

File: `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature` +
binding steps in
`apps/ayokoding-www/test/unit/fe-steps/cost-of-living-calculator.steps.tsx`.

- [ ] 8.1 `[AI]` Add accepted scenarios to
      `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`:
      SG-001 (active-tab-description), SG-002 (all selects styled), SG-003 (baseline ≤44px), USS-001
      (Savings auto-focus), USS-002 (Min-role example/empty), USS-003 (Area aria state), USS-004
      (disabled school-type description), plus foreigner-flag city-detail parity + jargon-gloss +
      region-localization scenarios.
- [ ] 8.2 `[AI]` Bind every new scenario (no unbound scenarios — `@amiceli/vitest-cucumber`) in
      `apps/ayokoding-www/test/unit/fe-steps/cost-of-living-calculator.steps.tsx`.
      Cmd: `npx nx run ayokoding-www:test:unit`.
- [ ] 8.3 `[AI]` `npx nx run ayokoding-www:specs:coverage`. AC: green; every changed behaviour has
      companion Gherkin.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [ ] `[AI]` `npx nx run ayokoding-www:specs:coverage` — exits 0, every changed behaviour has companion
      Gherkin.
- [ ] `[AI]` `npx nx run ayokoding-www:test:unit` — exits 0, no unbound scenarios.

> **Pause Safety**: All behavioural changes have companion Gherkin and passing bindings. Feature-change-
> completeness satisfied. Safe to stop. To resume: `npx nx run ayokoding-www:specs:coverage`.

---

## Phase 9 — Full validation + visual sign-off `[AI]` + `[HUMAN]` sign-off

- [ ] 9.1 `[AI]` `npx nx run ayokoding-www:typecheck && npx nx run ayokoding-www:lint && npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:specs:coverage` — all green.
- [ ] 9.2 `[AI]` Manual behavioural verification (Playwright MCP) at 320/375/768/1280 in **en and id**:
  - [ ] 9.2a `[AI]` Navigate to `/en/tools/cost-of-living-calculator` via `browser_navigate`, then repeat
        for `/id/tools/cost-of-living-calculator`.
  - [ ] 9.2b `[AI]` Resize to 320px, 375px, 768px, 1280px via `browser_resize` for each locale.
  - [ ] 9.2c `[AI]` For each locale × breakpoint: run `browser_snapshot` — verify tab descriptions
        single (only active visible), 44px controls, foreigner badge in both views, glosses, empty-states,
        select chrome consistency, salary-currency alignment.
  - [ ] 9.2d `[AI]` `browser_console_messages` — zero JS errors per locale.
  - [ ] 9.2e `[AI]` `browser_network_requests` — verify expected API integration.
  - [ ] 9.2f `[AI]` For each locale × breakpoint: `browser_take_screenshot` → save to
        `plans/in-progress/ayokoding-www-calculator-ux-hardening/evidence/phase-9-<feature>-<locale>-<breakpoint>px.png`.
        Reference each screenshot inline below.
  - [ ] 9.2g `[AI]` Document evidence: list each screenshot path here after capture (e.g.
        `![tab-descriptions en 375px](./evidence/phase-9-tab-descriptions-en-375px.png)`).
- [ ] 9.3 `[HUMAN]`/`[AI]` Visual-parity sign-off against `assets/` + the salary-savings-calculator hi-fi
      mockups, per breakpoint + locale, before archival (user-facing delivery hardening).
      Observable resume signal: sign-off recorded in this checklist item.
- [ ] 9.4 `[AI]` Commit changes thematically following Conventional Commits format (`fix(calculator): …`).
      Split different domains (a11y, i18n, design-fidelity, CSP) into separate commits. Do NOT bundle
      unrelated fixes.
- [ ] 9.5 `[AI]` Push to `origin main` and monitor GitHub Actions for the push. Verify `ci.yml` and
      affected workflow checks pass within 10 minutes. If any check fails, fix immediately before
      declaring the phase done.

### Phase 9 Gate

> All checks below must pass before starting Phase 10.

- [ ] `[AI]` `npx nx run ayokoding-www:typecheck && npx nx run ayokoding-www:lint && npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:specs:coverage` — all exits 0.
- [ ] `[AI]` Evidence screenshots committed to `evidence/` — at least one per locale per breakpoint.
- [ ] `[HUMAN]` Visual-parity sign-off recorded at 9.3.
      Observable resume signal: 9.3 checkbox ticked with sign-off note.
- [ ] `[AI]` GitHub Actions CI green — `gh run list --limit 5` shows most recent run passed.

> **Pause Safety**: All fixes validated locally and in CI. Visual sign-off recorded. Evidence committed.
> Safe to proceed to retest. To resume: re-verify `npx nx run ayokoding-www:specs:coverage` and check
> `gh run list --limit 5`.

---

## Phase 10 — Rule-15 three-tester retest follow-ups `[AI]`

After the fixes land and 9.3 visual sign-off is recorded, run the three live-site testers against the
running target across **both locales** (the `web-ux-test-fixing-planning` round in `delivery` output-mode,
or invoke each tester directly with `output-mode: delivery` + this plan's `plan-path`).

- [ ] 10.1 `[AI]` Run `web-exploratory-tester`, `web-usability-tester`, `web-design-tester` (delivery
      mode) → append each new finding below as an unchecked `EWT-###`/`UWT-###`/`DWT-###` checkbox.
- [ ] 10.2 `[AI]` Fix + tick every appended retest defect finding before archival.
      (SG-### proposals and USS-### suggestions may be triaged or deferred.)

_Retest findings (appended here in Phase 10):_

- _(none yet — populated during the retest round)_

### Phase 10 Gate

> All checks below must pass before archival.

- [ ] `[AI]` Every EWT-###/UWT-###/DWT-### defect finding appended in 10.1 is ticked (fixed).
      (SG-### proposals and USS-### suggestions may be triaged.) — AC: no unchecked defect checkbox.
- [ ] `[AI]` `npx nx run ayokoding-www:specs:coverage` — exits 0.

> **Pause Safety**: All retest defects resolved. Safe to archive. To archive: proceed to Plan Archival.

---

## Plan Archival

- [ ] `[AI]` Verify ALL delivery checklist items above are ticked.
- [ ] `[AI]` Verify ALL quality gates pass (local + CI): `npx nx run ayokoding-www:typecheck && npx nx run ayokoding-www:lint && npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:specs:coverage`.
- [ ] `[AI]` Verify ALL manual assertions pass with committed evidence in `evidence/` (screenshots present
      for each locale × breakpoint).
- [ ] `[AI]` Verify both en and id locales were exercised in UI verification (not just the default).
- [ ] `[AI]` Verify every rule-15 EWT/UWT/DWT defect finding from Phase 10 is fixed (ticked) or has
      explicit user deferral permission.
- [ ] `[AI]` `git mv plans/in-progress/ayokoding-www-calculator-ux-hardening plans/done/$(date +%Y-%m-%d)__ayokoding-www-calculator-ux-hardening`
- [ ] `[AI]` Update `plans/in-progress/README.md` — remove this plan entry.
- [ ] `[AI]` Update `plans/done/README.md` — add this plan entry with completion date.
- [ ] `[AI]` Update any other READMEs that reference this plan.
- [ ] `[AI]` Commit: `chore(plans): move ayokoding-www-calculator-ux-hardening to done`
