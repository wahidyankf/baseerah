# Delivery Checklist — Salary Savings Calculator

Executor tags: `[AI]` (default), `[HUMAN]`, `[AI+HUMAN]`. Each code step uses RED → GREEN → REFACTOR.
Commands assume repo root `/Users/wkf/ose-projects/ose-public` unless noted.

## Phase 0 — Setup & Baseline

- [ ] **[HUMAN]** Create worktree: `git worktree add worktrees/ayokoding-www-salary-savings-calculator -b ayokoding-www-salary-savings-calculator`.
- [ ] **[AI]** In the worktree, install + converge toolchain: `npm install` then `npm run doctor -- --fix`.
- [ ] **[AI]** Establish green baseline for the app: `npx nx run ayokoding-www:test:quick`. Acceptance: passes before any change.
- [ ] **[AI]** Confirm feature-folder convention by inspecting `apps/ayokoding-www/src` (`features/` vs `contexts/`) and the i18n mechanism in `src/contexts/i18n/`. Record the chosen layout in `tech-docs.md` if it differs from the proposed one.

### Phase 0 Gate

All baseline targets green; conventions confirmed. **Pause Safety**: stop if baseline is red — fix or report before proceeding.

## Phase 1 — Data + Calculation Core (TDD)

- [ ] **[AI] RED** Add `cities.test.ts` asserting dataset invariants: every city has all required fields, `currency` is an ISO code, dataset has a `snapshotDate`, and **no Israeli city / `ILS` currency** is present; also assert **at least one city each from ASEAN, Japan, Europe (non-Nordic), and the Nordics** via the `region` field. File: `apps/ayokoding-www/src/features/salary-savings/data/cities.test.ts`. Command: `npx nx run ayokoding-www:test:unit`. Acceptance: test fails (no dataset yet).
- [ ] **[AI] GREEN** Add `cities.ts` static dataset covering **as many tech-hub cities worldwide as we reasonably can** (breadth-first, excl. Israel) with `snapshotDate` and sourced estimate comments. File: `.../data/cities.ts`. Acceptance: `cities.test.ts` passes.
- [ ] **[AI] RED** Add `calc.test.ts` covering `costUsd`, `compareRow`, `singleCity`, `sortBySavings`, including the deficit (cost > salary → negative) and zero/negative-salary edge cases. File: `.../calc.test.ts`. Acceptance: fails (no calc yet).
- [ ] **[AI] GREEN** Implement pure `calc.ts` functions per `tech-docs.md`. File: `.../calc.ts`. Acceptance: `calc.test.ts` passes.
- [ ] **[AI] REFACTOR** Tidy types/naming; ensure calc is React-free and side-effect-free. Acceptance: `npx nx run ayokoding-www:test:unit` green; no lint errors.

### Phase 1 Gate

Dataset + calc core complete and unit-tested. **Pause Safety**: do not start UI until calc tests are green.

## Phase 2 — Interactive Page (TDD)

- [ ] **[AI] RED** Add a component test for "Compare all": render the table from a sample salary, assert a row shows savings % and a local-currency amount. File: `.../components/compare-table.test.tsx`. Command: `npx nx run ayokoding-www:test:unit`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `compare-table.tsx` (sortable table consuming calc core). Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for "Single city": select a city, enter local salary, assert cost/savings breakdown incl. deficit case. File: `.../components/single-city.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `single-city.tsx`. Acceptance: test passes.
- [ ] **[AI] GREEN** Add `page.tsx` (`'use client'`) at `apps/ayokoding-www/src/app/[locale]/tools/salary-savings/page.tsx` with the mode toggle wiring both components + salary state. Acceptance: route renders in dev (`npx nx dev ayokoding-www`, visit `/en/tools/salary-savings`).
- [ ] **[AI] REFACTOR** Extract shared formatting (`Intl.NumberFormat` by currency/locale); de-duplicate. Acceptance: unit tests green.

### Phase 2 Gate

Both modes render and compute correctly with component tests. **Pause Safety**: hold for i18n + a11y before E2E.

## Phase 3 — Bilingual Strings + Polish

- [ ] **[AI]** Add en/id UI strings (headings, labels, mode names, "estimates only" disclaimer) via the existing i18n mechanism; wire into the page/components. Acceptance: `/id/tools/salary-savings` shows Indonesian labels.
- [ ] **[AI]** Label salary inputs "Gross monthly salary (before tax)"; show a prominent, localized **"Data last updated: &lt;date&gt;"** label (formatted from `snapshotDate` via `Intl.DateTimeFormat`) near the results, plus a disclaimer covering both "estimates only" and "gross/pre-tax — taxes not modelled, real savings lower". Acceptance: last-updated date, gross-salary labels, and disclaimer clearly visible in both locales.
- [ ] **[AI]** Accessibility + responsive pass: labeled inputs, keyboard operation, AA contrast, mobile→desktop layout. Acceptance: manual check via Playwright MCP/devtools; no a11y regressions.

### Phase 3 Gate

Full en/id parity, disclaimer visible, a11y/responsive verified. **Pause Safety**: stop if any string is missing in either locale.

## Phase 4 — E2E + Local Quality Gates

- [ ] **[AI] RED→GREEN** Add one fe-e2e smoke test in `apps/ayokoding-www-fe-e2e`: load `/en/tools/salary-savings`, enter a salary, assert a populated table + a savings cell. Command: `npx nx run ayokoding-www-fe-e2e:test:e2e`. Acceptance: passes.
- [ ] **[AI]** Run affected local gates: `npx nx affected -t typecheck lint test:quick specs:coverage`. Acceptance: all green (warm cache first if needed).

### Phase 4 Gate

E2E smoke + all local gates green. **Pause Safety**: do not push until green locally.

## Phase 5 — Post-Push CI Verification

- [ ] **[HUMAN]** Review the diff and approve push to `main` (trunk-based).
- [ ] **[AI]** Push and trigger/monitor relevant GitHub Actions for `ayokoding-www` (poll every 3 min; do not use `gh run watch`). Acceptance: CI green.

### Phase 5 Gate

CI green on `main`. **Pause Safety**: investigate root cause of any failure; never bypass gates.

## Phase 6 — Plan Archival

- [ ] **[AI]** Move plan folder to `plans/done/YYYY-MM-DD__ayokoding-www-salary-savings-calculator/` and tick all gates.
- [ ] **[HUMAN]** Remove the worktree once merged: `git worktree remove worktrees/ayokoding-www-salary-savings-calculator`.

### Phase 6 Gate

Plan archived; worktree cleaned up. Feature live at `/[locale]/tools/salary-savings` in en + id.
