# Delivery Checklist — Calculator Test-Fixing

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Execute on the main checkout at `/Users/wkf/ose-projects/ose-public` (no dedicated worktree — per
user directive).

Quality gate command used throughout (run from the repo root):

```bash
npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage
```

E2E suite (for runtime-proof findings): `npx nx run ayokoding-www-fe-e2e:test:e2e`.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] [AI] Install dependencies in the root checkout: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized. _Done 2026-06-21: deps already synchronized._
- [x] [AI] Converge the toolchain: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift. _Done: toolchain converged (prior install)._
- [x] [AI] Record the calculator baseline:
      `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — acceptance: pass/fail counts recorded; any preexisting failure documented. _Baseline 2026-06-21: typecheck ✓, lint ✓ (3 pre-existing jsx-a11y warnings, non-blocking), test:unit 2030 passed, specs:coverage 15 specs/161 scenarios/590 steps all covered._
- [x] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no unresolved preexisting failures remain. _No failures; only 3 non-blocking lint warnings (calculator-content.tsx:216, controls.tsx:32) carried into the relevant phases._

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [x] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      baseline recorded and every preexisting failure resolved (zero unresolved)

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run the baseline gate command and
> confirm it is still clean.

---

## Phase 1: Correctness — Minimum-role zero-target divider (EWT-001)

- [x] [AI] **RED**: add/adjust a failing unit test in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.test.tsx` asserting
      that with baseline source = savings target and target = `0`, the element
      `data-testid="qualifying-divider"` is present (desktop table) and the lowest-clearing role is
      marked minimum — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: the new assertion fails (divider not rendered at target 0)
      _Done 2026-06-21: new test "with savings_target=0 (baseline engaged) renders the qualifying
      divider and marks the minimum" failed RED (divider absent at target 0)._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN**: in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` change the
      divider render condition at ~line 314 from `qualifying.length > 0 && nonQualifying.length > 0`
      to render the divider whenever `baselineReady && qualifying.length > 0`; apply the equivalent
      fix to the mobile-cards section at ~line 330 — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: the RED test passes; no other `min-role.test.tsx` tests break
      _Done: desktop divider uses `showDivider`; mobile cards split into qualifying/divider/
      non-qualifying with a distinct `qualifying-divider-mobile` testid so the desktop divider stays
      unique to existing `getByTestId` assertions._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR**: extract the divider-visibility condition into a named local
      (e.g. `showDivider`) used by both table and mobile blocks in `min-role.tsx`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all `min-role.test.tsx` tests still pass; single source of truth for the condition
      _Done: `const showDivider = baselineReady && qualifying.length > 0;` drives both blocks;
      mobile card markup extracted into a `MobileRoleCard` component._
- [x] [AI] **RED**: extend Gherkin — make the existing scenario "Zero savings target marks the
      lowest role as the minimum" in
      `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
      explicitly assert the divider is rendered (add an `And` line); add the step-def if missing
      — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` fails on the new uncovered step
      _Done: added `And the qualifying divider element is rendered in the role ladder`;
      specs:coverage failed RED on the uncovered step._
  - _Suggested executor: `specs-maker`_
- [x] [AI] **GREEN**: implement/connect the step definition so the scenario consumes the fixed
      behaviour — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` exits 0
      _Done: wired the new step + strengthened the prior stub assertions to real `qualifying-divider`
      / `minimum-marker` / `non-qualifying-row` checks; specs:coverage exits 0 (591 steps)._
- [x] [AI] **RED**: add an e2e assertion in `apps/ayokoding-www-fe-e2e` navigating to the min-role
      tab, setting target to `0`, asserting `[data-testid="qualifying-divider"]` is visible
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: e2e fails before the GREEN source change is deployed to the dev build (or
      passes confirming the fix end-to-end if run after GREEN)
      _Done: implemented the BDD e2e divider steps (was stubbed); the "Zero savings target" scenario
      passes on chromium against the rebuilt standalone bundle._
  - _Suggested executor: `swe-e2e-dev`_

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — expected: exits 0 _Done 2026-06-21: all 4 targets pass (typecheck clean; lint exits 0 with 3
      pre-existing non-blocking jsx-a11y warnings; test:unit 2032 passed; specs:coverage 591 steps)._
- [x] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` (divider spec) — expected: passes
      _Done: "Zero savings target marks the lowest role as the minimum" e2e scenario passes on
      chromium (PASS 1 / FAIL 0)._

> **Pause Safety**: EWT-001 is fixed, unit + spec + e2e green. Safe to stop. To resume: re-run the
> gate command.

---

## Phase 2: Breadcrumb consolidation (DWT-B-003, DWT-B-004, UWT-013)

- [x] [AI] **RED**: extend
      `apps/ayokoding-www/src/features/navigation/shell/breadcrumb.tsx` test (or add
      `breadcrumb.test.tsx` sibling) asserting the shared `Breadcrumb` can render the final
      (current-page) segment when given an opt-in prop — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test fails (prop not yet supported)
      _Done 2026-06-21: added sibling `breadcrumb.test.tsx`; the two `showCurrent` tests failed RED
      (final segment dropped, no svg separators) while the default-behaviour test passed._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN**: extend `navigation/shell/breadcrumb.tsx` to optionally render the
      current-page (last) segment as a non-link `aria-current="page"` crumb, controlled by a new
      prop (e.g. `showCurrent`); keep existing callers' behaviour unchanged when the prop is absent
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: new test passes; existing breadcrumb callers' tests unaffected
      _Done: added `showCurrent?: boolean` prop + `hrefFor` (empty slug → `/en`, not `/en/`); all
      2035 unit tests pass including the content-page breadcrumb caller._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **RED**: rewrite the assertions in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.test.tsx`
      to require (a) `ChevronRight` separators (no literal `/`), and (b) the final crumb text equals
      `t(locale, "calcTitle")` in both `en` and `id` — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: tests fail against the current bespoke component
      _Done 2026-06-21: parametrized the `useLocale` mock via a hoisted holder so en+id render in one
      file; 3 new assertions (DWT-B-003 chevrons/no-slash, UWT-013 final crumb = calcTitle en+id)
      failed RED while the 3 legacy structural tests stayed green._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN**: rewrite
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx`
      to delegate to the shared `Breadcrumb` with segments
      `[{label: breadcrumbHome, slug: ""}, {label: toolsPageTitle, slug: "tools"}, {label: calcTitle, slug: "tools/cost-of-living-calculator"}]`
      and `showCurrent` enabled; preserve the id-locale label translation and `flex-wrap`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: the RED tests pass; no literal `/` remains in the rendered breadcrumb
      _Done: component now delegates to shared `Breadcrumb` with `showCurrent`; flex-wrap preserved by
      the shared `<ol className="flex flex-wrap …">`; all 2037 unit tests pass; final crumb = calcTitle
      in both locales._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR**: remove now-dead bespoke markup; confirm the calculator page still imports
      `CalculatorBreadcrumb` from the same path (`calculator-content.tsx` line ~16 unchanged)
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all tests pass; no stale imports
      _Done: bespoke `<ol>`/literal-`/` markup fully replaced by delegation; `calculator-content.tsx`
      line 16 import unchanged; typecheck + lint clean (only 3 pre-existing jsx-a11y warnings).
      `breadcrumbCalculator` key is now unused but left in `translations.ts` to avoid type-shape churn._
- [x] [AI] **RED/GREEN**: add Gherkin scenarios AC-2 (chevron separators / shared primitive) and
      AC-3 (final crumb equals H1 per locale, as a `Scenario Outline`) to the calculator
      `.feature`; wire step defs — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` exits 0
      _Done 2026-06-21: added "The breadcrumb separates crumbs with chevrons, not a literal slash"
      (chevron-svg count + no-slash) and Scenario Outline "The final breadcrumb crumb matches the page
      title in each locale" (en/id Examples) with one primary Given/When/Then each (And for extras).
      RED: 5 missing steps; GREEN: wired step defs, specs:coverage exits 0 (15 specs/163 scenarios/599
      steps); the outline reads `examples` via the `ScenarioOutlineTest` second callback arg._
  - _Suggested executor: `specs-maker`_

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — expected: exits 0 _Done 2026-06-21 via `nx run-many -t typecheck lint test:unit specs:coverage
-p ayokoding-www`: exit 0 (typecheck clean; lint exits 0 with the 3 pre-existing non-blocking
      jsx-a11y warnings; test:unit 2049 passed; specs:coverage 15 specs/163 scenarios/599 steps)._
- [x] [AI] Grep proof: `grep -n '"/"' apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx`
      — expected: no literal `/` separator list items remain _Done: grep exits 1 (no matches); the
      bespoke literal-slash `<li>` items are gone._

> **Pause Safety**: breadcrumb consolidated and locale-correct; gate green. Safe to stop. To
> resume: re-run the gate command.

---

## Phase 3: Touch targets & responsive (UWT-016/DWT-005, UWT-008)

- [x] [AI] **RED**: add an e2e measurement test in `apps/ayokoding-www-fe-e2e` asserting each of
      `#geo-region-select`, `#geo-country-select`, `#geo-city-select` has rendered height ≥ 44 px at
      375 px — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (selects measure ~29 px)
      _Done 2026-06-21: added Gherkin scenario "Geo-filter selects meet the minimum touch-target
      height on mobile" + e2e step defs (`boundingBox().height >= 44` for all three select ids).
      RED measured `#geo-region-select` height = **29 px** (< 44) — failed as expected._
  - _Suggested executor: `swe-e2e-dev`_
- [x] [AI] **GREEN**: in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx` replace the
      three selects' `className="rounded border px-2 py-1 text-sm"` with the shared web-ui control
      styling plus `min-h-[44px]` (reuse the `libs/web-ui` `Select` primitive if present; otherwise
      mirror its Tailwind classes) — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: `geo-filters.test.tsx` still passes; selects carry `min-h-[44px]`
      _Done: no `Select` primitive exists in `libs/web-ui` — mirrored the `Input` primitive's
      Tailwind (`rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs
focus-visible:*`) plus `min-h-[44px] w-full min-w-0 max-w-full`. Rebuilt standalone; e2e GREEN
      measured all three selects ≥ 44 px. `geo-filters.test.tsx` (+ all 2055 unit tests) pass._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **RED**: add an e2e test asserting `document.documentElement.scrollWidth <= 320` at a
      320 px viewport on the calculator page — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: test fails (scrollWidth 328 > 320)
      _Done 2026-06-21: added Gherkin scenario "The calculator page has no horizontal overflow at
      320px" + e2e step def (`document.documentElement.scrollWidth <= 320`). RED measured
      scrollWidth = **328 px** (> 320) — failed as expected._
  - _Suggested executor: `swe-e2e-dev`_
- [x] [AI] **GREEN**: fix the overflow source — inspect the geo-filter row
      (`flex flex-wrap items-center gap-3`) and tab list at 320 px; constrain offending widths
      (e.g. allow selects to shrink with `min-w-0`/`max-w-full`, ensure `overflow-x-auto` only where
      intended) in `geo-filters.tsx` and/or `calculator-content.tsx`
      — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: the 320 px overflow e2e test passes
      _Done: diagnostic (boundingBox sweep at 320 px) found the **single** offender was the shared
      app-header theme-toggle trigger at right=328 — not the geo-filter row. Root cause: header
      `gap-4` (16 px × 4 gaps) overflowed at 320 px. Fixed minimally in
      `apps/ayokoding-www/src/features/app-shell/shell/header.tsx`: `gap-4` → `gap-2 px-4 sm:gap-4`.
      Also hardened geo-filter groups (`basis-full sm:basis-auto min-w-0` wrappers + select
      `min-w-0 max-w-full`) so the filter row never forces overflow. Rebuilt standalone; e2e GREEN
      scrollWidth = 320 (no overflow)._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR**: confirm the touch-target and overflow fixes coexist (re-run both e2e
      measurements) — command: `npx nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: both measurement tests pass together
      _Done: ran both measurement scenarios together against the rebuilt standalone — `2 passed`
      (selects ≥ 44 px AND scrollWidth ≤ 320). No regression between the two fixes._
- [x] [AI] **RED/GREEN**: add Gherkin AC-4 (44 px selects) and AC-5 (no overflow at 320 px) to the
      calculator `.feature`; wire step defs — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` exits 0
      _Done: AC-4/AC-5 scenarios added to `cost-of-living-calculator.feature` (one primary
      Given/When/Then each — cardinality-clean). Wired both e2e step defs and unit-fe bindings
      (unit asserts the `min-h-[44px]` / `min-w-0` styling contract; jsdom has no layout engine, so
      pixel measurement lives at e2e). Also fixed a **preexisting** gap: the Phase-2 breadcrumb
      AC-2/AC-3 scenarios had no e2e step defs (bddgen failed with 5 missing steps), blocking e2e
      generation — added those 5 breadcrumb e2e step defs too (all 3 breadcrumb e2e scenarios now
      pass). `specs:coverage` exits 0 (15 specs / 165 scenarios / 605 steps);
      `specs:gherkin-cardinality-validation` clean._
  - _Suggested executor: `specs-maker`_

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [x] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — expected: exits 0 _Done 2026-06-21 via `nx run-many -t typecheck lint test:unit specs:coverage
-p ayokoding-www`: exit 0 (typecheck clean; lint exits 0 with the 3 pre-existing non-blocking
      jsx-a11y warnings — calculator-content.tsx:216, controls.tsx:32; test:unit 2055 passed;
      specs:coverage 15 specs / 165 scenarios / 605 steps)._
- [x] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` (touch-target + 320 px overflow specs)
      — expected: passes _Done: both measurement scenarios pass on chromium against the rebuilt
      standalone bundle (`2 passed`); the 3 breadcrumb AC-2/AC-3 e2e scenarios (whose step defs were a
      preexisting gap) also pass (`3 passed`)._

> **Pause Safety**: geo selects are tappable and the page no longer overflows at 320 px; gate
> green. Safe to stop. To resume: re-run the gate command.

---

## Phase 4: Tab a11y & descriptions (UWT-011, UWT-003, UWT-012)

- [x] [AI] **RED**: extend
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.test.tsx`
      asserting (a) the cost trigger has `aria-describedby="tab-desc-cost"` and a `#tab-desc-cost`
      element exists, and (b) all three description elements are visibly rendered (not `sr-only`)
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: assertions fail (no `#tab-desc-cost`; descriptions are `sr-only`)
      _Done 2026-06-21: the planned app-dir test path does not exist; the live unit test for this
      component is `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-content.test.tsx`
      (renders `CostOfLivingCalculatorContent`). Added a `Phase4TF` describe with 3 tests: UWT-011
      (cost trigger `aria-describedby="tab-desc-cost"` + `#tab-desc-cost` exists), UWT-003 (all three
      `#tab-desc-*` not `sr-only`/`aria-hidden`), and a no-duplication guard. All 3 failed RED
      (`aria-describedby` was null; `#tab-desc-cost` absent; the savings/min-role spans were `sr-only`)._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN**: add `tabCostDesc` keys (en + id) to
      `apps/ayokoding-www/src/features/i18n/core/translations.ts`; in `calculator-content.tsx` add
      `aria-describedby="tab-desc-cost"` to the cost trigger, add a `#tab-desc-cost` span, render all
      three descriptions visibly (drop `sr-only`), and remove the duplicate `aria-hidden` visible
      paragraph (lines ~174–178) — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: RED assertions pass; no duplicated description text
      _Done: added `tabCostDesc` (en "Compare monthly living costs across cities" / id "Bandingkan
      biaya hidup bulanan di berbagai kota"); added `aria-describedby="tab-desc-cost"` to the cost
      trigger; replaced the two `sr-only` spans + the `aria-hidden` paragraph with three visible `<p>`
      description elements (`#tab-desc-cost`/`-savings`/`-min-role`), each shown for its active tab
      (inactive ones carry `hidden`, never `sr-only`/`aria-hidden`). No description text is duplicated.
      All 3 RED tests pass; 2058 unit tests green._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **RED**: add a unit assertion in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.test.tsx`
      requiring every rendered "OOP" acronym to be inside an `<abbr title="out-of-pocket">` (audit
      all occurrences incl. the mobile card label using `colHealthcareOOP` ~line 283)
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails if any OOP is a bare `<span>`; passes if all are already `<abbr>` (then no
      source change needed per Assumption A-2)
      _Done 2026-06-21: added `UWT-012: every rendered 'OOP' acronym is inside an abbr` — a
      `TreeWalker` audit over the rendered `CostOfLivingTable`, excluding the definitional legend
      ("OOP = out-of-pocket — …"). RED **failed** (Assumption A-2 disproved): the mobile card label
      `colHealthcareOOP` = "Healthcare (OOP)" rendered a **bare** OOP, so a source change was required._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN**: if RED failed, wrap the offending OOP occurrence(s) in
      `cost-of-living.tsx` with `<abbr title="out-of-pocket">OOP</abbr>`
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: OOP audit test passes
      _Done: widened `CardRow`'s `label` prop to `ReactNode` and passed a JSX label for the mobile
      healthcare row: `{colHealthcareOOPPrefix} (<abbr title="out-of-pocket">OOP</abbr>)`, mirroring
      the desktop header. Root-cause sweep also fixed the **identical** bare OOP in
      `controls.tsx:214` (preview pane, same `colHealthcareOOP` string) with the same abbr wrap. The
      now-unused `colHealthcareOOP` translation key is left in place (matching the Phase-2 precedent
      of not churning `translations.ts` for dead keys). OOP audit passes; 2059 unit tests green._
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **RED/GREEN**: add Gherkin AC-6 (tab descriptions visible + associated) and confirm AC-7
      (OOP abbr) — extend the existing "OOP abbreviation is explained" scenario to assert the
      `<abbr>` element — in the calculator `.feature`; wire step defs
      — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` exits 0
      _Done: added Scenario "Each tab has a visible description associated with its trigger"
      (Given/When/Then + 1 And — cardinality-clean) and extended "The OOP abbreviation is explained on
      screen" with `And every "OOP" acronym is wrapped in an abbr element titled "out-of-pocket"`.
      Wired both in `test/unit/fe-steps/cost-of-living-calculator.steps.tsx` (`@amiceli/vitest-cucumber`
      `Scenario` blocks — this repo binds steps via executable step blocks, not separate defs). The
      tab-desc step asserts the three triggers' `aria-describedby`, visibility, and no-duplication; the
      OOP step reuses the TreeWalker audit. `specs:coverage` exits 0 (15 specs / 166 scenarios / 610
      steps); `rhino-cli:specs:gherkin-cardinality-validation` clean._
  - _Suggested executor: `specs-maker`_

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [x] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — expected: exits 0 _Done 2026-06-21 via `nx run-many -t typecheck lint test:unit specs:coverage
-p ayokoding-www`: exit 0. typecheck clean; lint exits 0 with **one** pre-existing non-blocking
      warning (`controls.tsx:32` jsx-a11y `prefer-tag-over-role`, unrelated radio control) — the two
      prior `calculator-content.tsx:216` jsx-a11y warnings (`click-events-have-key-events`,
      `no-static-element-interactions`) were **resolved** opportunistically (see note); test:unit 2064
      passed; specs:coverage 15 specs / 166 scenarios / 610 steps.
      **Opportunistic line-216 fix**: the warnings sat on `<div onClick={handleTableClick}>`, a
      click-delegation surface over the inner interactive `<a>` links. Keyboard activation (Enter on a
      focused anchor) synthesizes a click that bubbles to this div, so no extra key handler/role is
      needed — adding a `role`/keydown to the div would be semantically wrong. Resolved with a scoped
      `eslint-disable-next-line` plus a rationale comment (no behavior change). The `controls.tsx:32`
      warning is a different rule on an unrelated element and was left untouched._

> **Pause Safety**: all tabs have discoverable associated descriptions and OOP semantics are
> correct; gate green. Safe to stop. To resume: re-run the gate command.

---

## Phase 5: Currency & empty-states (UWT-004, UWT-006)

- [ ] [AI] **RED**: extend
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.test.tsx` asserting
      the gross-salary label does NOT contain the literal "USD" and that an active-currency
      indicator (or selector) is rendered — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails (label currently ends "USD")
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: update `grossMonthlySalaryLabel` (en + id) in `translations.ts` to drop the
      trailing "USD"; in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx` surface the
      active currency next to the input (indicator per Assumption A-6, or mirror min-role's
      `displayCurrency` selector if trivial) — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: RED test passes; existing savings tests still pass
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED**: extend
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.test.tsx` asserting
      that with savings-target baseline and a blank/zero target, the role table is hidden and a
      `data-testid="min-role-empty-state"` guidance message is shown
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails (table always renders today)
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: add `minRoleEmptyStateMessage` keys (en + id) to `translations.ts`; in
      `min-role.tsx`, when `baselineSource === "savings_target" && targetAmount === 0`, render the
      `min-role-empty-state` message and skip the table/mobile-cards blocks (mirror the Savings
      empty-state pattern) — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: RED test passes; the Phase-1 divider tests (which set a non-zero/zero target
      with a chosen baseline) remain green
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **REFACTOR**: reconcile the empty-state with the EWT-001 divider — confirm the
      zero-target _divider_ scenario from Phase 1 still expects the table when a baseline is
      explicitly engaged vs the empty-state when the target is blank; document the distinction in a
      code comment — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all min-role tests pass, no contradiction between empty-state and divider tests
- [ ] [AI] **RED/GREEN**: add Gherkin AC-8 (currency) and AC-9 (min-role empty-state) to the
      calculator `.feature`; wire step defs — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` exits 0
  - _Suggested executor: `specs-maker`_

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — expected: exits 0

> **Pause Safety**: Savings active currency is explicit and min-role has an empty-state; gate
> green. Safe to stop. To resume: re-run the gate command.

---

## Phase 6: Region & URL behaviour (UWT-007, UWT-014, UWT-015, UWT-009)

- [ ] [AI] **RED**: add a unit test in
      `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.test.tsx`
      asserting the region selector lists exactly the nine intended regions
      (`africa, americas, asean, asia, europe, japan, mena, nordics, oceania`) per Assumption A-1
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test asserts the complete set; fails only if a region is missing (likely passes,
      locking the verified set)
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED**: extend `geo-filters.test.tsx` asserting that selecting a country whose region
      differs from the current selection renders a visible `data-testid="region-auto-advisory"`
      message — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails (no advisory exists)
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: add `regionAutoAdvisory` keys (en + id) to `translations.ts`; in
      `geo-filters.tsx`, when `applyCountryChange` results in a region change, render the advisory
      below the dropdowns — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: RED advisory test passes
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED**: add a unit test in
      `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.test.tsx`
      asserting that a city-only deep link (`?city=london`) produces a city-detail back link of
      `?tab=cost` (no injected `region`/`country`) per Assumption A-3
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails (current `cityDetailBackHref` injects `region=europe&country=gb`)
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: in `calculator-content.tsx` (and/or `core/url-state.ts` `parentScopeParams`)
      omit region/country from the back link when they were auto-derived solely from a city deep
      link rather than user-selected — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: RED back-link test passes; existing url-state unit tests still pass
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED**: add a unit test for the tools index (new
      `apps/ayokoding-www/src/app/[locale]/tools/page.test.tsx`) asserting the calculator entry has
      a description element distinct from the link text — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails (no description sibling)
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: add `toolsPageCalcDesc` keys (en + id) to `translations.ts`; in
      `apps/ayokoding-www/src/app/[locale]/tools/page.tsx` render a description `<p>` under the link
      — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: RED tools-index test passes
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED/GREEN**: add Gherkin AC-10 (region set), AC-11 (region advisory), AC-12 (back
      link), and AC-13 (tools-index link description — create
      `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/tools-index.feature` if no
      tools-index feature exists) ; wire step defs
      — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` exits 0
  - _Suggested executor: `specs-maker`_

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — expected: exits 0

> **Pause Safety**: region set verified, auto-change advised, back link predictable, tools-index
> link described; gate green. Safe to stop. To resume: re-run the gate command.

---

## Phase 7: Spec coverage sweep & still-relevant proposals

- [ ] [AI] **RED/GREEN**: ensure the calculator `.feature` also protects the still-relevant
      proposals not yet covered — URL-param-per-control (SG-U-001..004: tab, region, country, city
      reflected in the URL), sub-national net indicator, country-narrows-city, area radiogroup,
      baseline `SegmentedControl` — adding scenarios + step defs that consume existing behaviour
      (RED only where a gap exists) — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: `specs:coverage` exits 0 with the new scenarios covered
  - _Suggested executor: `specs-maker`_
- [ ] [AI] Run the Gherkin keyword-cardinality audit:
      `npx nx run rhino-cli:specs:gherkin-cardinality-validation`
      — acceptance: no cardinality violations reported (one primary Given/When/Then per scenario in
      the changed `.feature` files)
  - _Suggested executor: `specs-maker`_

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] `npx nx run ayokoding-www:typecheck ayokoding-www:lint ayokoding-www:test:unit ayokoding-www:specs:coverage`
      — expected: exits 0
- [ ] [AI] `npx nx run rhino-cli:specs:gherkin-cardinality-validation` — expected: clean

> **Pause Safety**: all behaviours are spec-protected and Gherkin is cardinality-clean; gate green.
> Safe to stop. To resume: re-run the gate command.

---

## Phase 8: Manual verification, rule-15 retest, and archival

### Manual UI Verification (Playwright MCP) — all locales × all breakpoints

- [ ] [AI] Confirm supported locales from
      `apps/ayokoding-www/src/features/i18n/core/config.ts` (expected: `en`, `id`)
      — acceptance: locale set recorded in notes
- [ ] [AI] Start dev server: `npx nx run ayokoding-www:dev` (port 3101)
- [ ] [AI] For EACH locale (`en`, `id`) × EACH breakpoint (320 / 375 / 768 / 1280 px): navigate to
      `/{locale}/tools/cost-of-living-calculator` via `browser_navigate` + `browser_resize`
      — acceptance: page renders, `html[lang]` matches locale
- [ ] [AI] Verify fixed behaviours per locale: breadcrumb chevrons + full-title crumb; geo selects
      ≥ 44 px; no horizontal scroll at 320 px; all three tab descriptions visible; min-role
      empty-state at blank target; min-role divider at zero target; Savings active currency;
      region advisory on country change; tools-index link description
      — acceptance: each behaviour observed in both locales
- [ ] [AI] Check `browser_console_messages` — must be zero errors per locale
- [ ] [AI] Capture one screenshot per locale per breakpoint via `browser_take_screenshot` to
      `evidence/phase-8-calculator-{locale}-{breakpoint}px.png`
      — acceptance: files exist in `plans/in-progress/ayokoding-calculator-test-fixing/evidence/`
- [ ] [AI] Reference each screenshot in this checklist (`![alt](./evidence/...)`) and note console
      status per locale

### Rule-15 Three-Tester Retest (before archival)

- [ ] [AI] Run the `web-ux-test-fixing-planning` workflow (`web-exploratory-tester` +
      `web-usability-tester` + `web-design-tester`) against the running calculator URL across both
      locales — acceptance: EWT/UWT/DWT findings + spec-gaps recorded
- [ ] [AI] Append each new finding here as an unchecked, source-attributed checkbox
      (`- [ ] EWT-NNN:` / `- [ ] UWT-NNN:` / `- [ ] DWT-NNN: <defect> — fix before archival`); add
      each SG-### spec-gap to the Phase 7 spec steps
- [ ] [AI] Fix every rule-15 finding (or explicitly defer with rationale) before archival

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck` — exits 0
- [ ] [AI] `npx nx affected -t lint` — exits 0
- [ ] [AI] `npx nx affected -t test:unit` — exits 0
- [ ] [AI] `npx nx affected -t specs:coverage` — exits 0
- [ ] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` — exits 0

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. Commit preexisting fixes separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit thematically — one commit per phase/concern, Conventional Commits format
      (`fix(ayokoding-www): ...`, `test(ayokoding-www): ...`, `feat(specs): ...`)
- [ ] [AI] Do NOT bundle unrelated changes into a single commit

### Post-Push CI Verification

- [ ] [AI] Commit and push to origin main
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (3-minute poll; do not use
      `gh run watch`)
- [ ] [AI] Verify ALL CI checks pass; fix and push follow-ups until green
- [ ] [AI] Do NOT archive until CI is fully green

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Verify manual assertions pass with committed evidence in `evidence/` (both locales,
      320/375/768/1280 px)
- [ ] [AI] Verify every rule-15 finding is fixed or explicitly deferred
- [ ] [AI] Move plan: `git mv plans/in-progress/ayokoding-calculator-test-fixing plans/done/2026-06-21__ayokoding-calculator-test-fixing`
      (use the actual completion date)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the entry
- [ ] [AI] Update `plans/done/README.md` — add the entry with completion date
- [ ] [AI] Commit the archival: `chore(plans): move ayokoding-calculator-test-fixing to done`

### Phase 8 Gate

> Final gate — the plan is complete only when all below pass.

- [ ] [AI] `npx nx affected -t typecheck lint test:unit specs:coverage` and
      `ayokoding-www-fe-e2e:test:e2e` — expected: all exit 0
- [ ] [AI] CI fully green on the pushed commits
- [ ] [AI] Evidence committed for both locales at all breakpoints; rule-15 findings triaged

> **Pause Safety**: all findings fixed, gates + CI green, evidence committed, plan archived. The
> repository is coherent and the plan is done. To resume (if archival is incomplete): re-run the
> final gate command and complete the archival steps.
