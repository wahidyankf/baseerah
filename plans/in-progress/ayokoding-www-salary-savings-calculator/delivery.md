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
- [ ] **[AI]** Confirm the functional-core / imperative-shell layout in `apps/ayokoding-www/src/features/<name>/{core,shell}/` and the i18n mechanism in `src/features/i18n/core/`. Confirm whether the new `tools/` route should live under the `(app)` route group (`app/[locale]/(app)/tools/salary-savings/page.tsx`) or directly under `[locale]` (`app/[locale]/tools/salary-savings/page.tsx`). Record both decisions in `tech-docs.md §Risks / Open Questions` if the chosen layout differs from the proposed one.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npx nx run ayokoding-www:test:quick` and `npx nx run web-ui:test:quick` — both exit 0 (green baseline before any change).
- [ ] [AI] `apps/ayokoding-www/src/features/i18n/core/translations.ts` exists — `test -f apps/ayokoding-www/src/features/i18n/core/translations.ts && echo "OK"`.
- [ ] [AI] Feature-folder convention and route-group placement decision recorded in `tech-docs.md`.

> **Pause Safety**: worktree provisioned, toolchain converged, baseline green, conventions confirmed.
> Safe to stop. To resume: `npx nx run ayokoding-www:test:quick` — must still pass before Phase 1.

## Phase 1 — Data + Calculation Core (TDD)

- [ ] **[AI] RED** Add `cities.test.ts` asserting dataset invariants: every city has all required fields (incl. `schoolMedianLocal.public` and `schoolMedianLocal.private`), `currency` is an ISO code, dataset has a `snapshotDate`, and **no Israeli city / `ILS` currency** is present; also assert **at least one city each from ASEAN, Japan, Europe (non-Nordic), and the Nordics** via the `region` field. File: `apps/ayokoding-www/src/features/salary-savings/data/cities.test.ts`. Command: `npx nx run ayokoding-www:test:unit`. Acceptance: test fails (no dataset yet).
- [ ] **[AI] GREEN** Add `cities.ts` static dataset covering **as many tech-hub cities worldwide as we reasonably can** (breadth-first, excl. Israel) with `snapshotDate`, per-city single-person living cost, `{ public, private }` school medians, and sourced estimate comments; plus the shared `HOUSEHOLD_MULTIPLIERS`, `HOUSEHOLD_KIDS`, and `AREA_MULTIPLIERS` tables. File: `.../data/cities.ts`. Acceptance: `cities.test.ts` passes.
- [ ] **[AI] RED** Add `calc.test.ts` covering `livingLocal`, `schoolLocal`, `costLocal`, `costUsd`, `compareRow`, `singleCity`, `sortBySavings`, including: the deficit (cost > salary → negative) and zero/negative-salary edge cases; cost rising monotonically across household types; `rural` < `center`; `private` ≥ `public` school cost; and zero school cost for childless households. File: `.../calc.test.ts`. Acceptance: fails (no calc yet).
- [ ] **[AI] GREEN** Implement pure `calc.ts` functions per `tech-docs.md` (household + area multipliers, per-child school add-on). File: `.../calc.ts`. Acceptance: `calc.test.ts` passes.
- [ ] **[AI] REFACTOR** Tidy types/naming in `apps/ayokoding-www/src/features/salary-savings/calc.ts` and `apps/ayokoding-www/src/features/salary-savings/data/cities.ts` (or equivalent paths confirmed in Phase 0); ensure `calc.ts` is React-free and side-effect-free (no imports from React, no `console.log`, no module-level mutation). Acceptance: `npx nx run ayokoding-www:test:unit` exits 0; `npx nx run ayokoding-www:lint` exits 0 with no errors.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (all `cities.test.ts` and `calc.test.ts` assertions pass).
- [ ] [AI] `npx nx run ayokoding-www:lint` — exits 0 with no errors on the new data/calc files.
- [ ] [AI] Dataset coverage verified: `cities.ts` contains at least one city each from ASEAN, Japan, Europe (non-Nordic), and Nordics regions.
- [ ] [AI] No Israeli city in dataset: grep for `ILS` and `Israel` returns 0 results in `cities.ts`.

> **Pause Safety**: `cities.ts` dataset and `calc.ts` pure functions are complete, unit-tested, and
> lint-clean. No UI code exists yet. Safe to stop. To resume: `npx nx run ayokoding-www:test:unit`
> — must still pass before Phase 1b.

## Phase 1b — Role-Salary Data + Reverse-Lookup Core (TDD)

Adds the second dataset (`roles.ts`) and the pure `role-lookup.ts` search that powers the
minimum-role mode. The role taxonomy + salary matrix are sourced via `web-research-maker`, then
encoded as a static full city×role matrix with per-cell confidence tiers. Still no UI.

- [ ] **[AI]** Source the role data via `web-research-maker`: (a) the canonical 15-rung engineering
      ladder (IC + management, with `rank`/`track`/`label`), and (b) a typical monthly gross salary
      per role per city in `cities.ts`, each with a `confidence` tier (`high` | `moderate` | `proxy`)
      and a source note. Record the cited findings + `snapshotDate` in a research note referenced
      from `roles.ts` comments. Acceptance: a complete role list + a salary value (or documented
      `proxy` derivation) for every city×role pair, no fabricated exact figures.
- [ ] **[AI] RED** Add `roles.test.ts` asserting matrix invariants: `ladder` is the full 15-rung set
      with strictly increasing `rank`; `salaries` keys **exactly match** `cities.ts` city IDs (full
      matrix, no holes); every cell has a positive `monthlyGrossLocal` and a valid `confidence`;
      **no Israeli city / `ILS`** leaks in; a `snapshotDate` is present. File:
      `apps/ayokoding-www/src/features/salary-savings/data/roles.test.ts`. Command:
      `npx nx run ayokoding-www:test:unit`. Acceptance: fails (no `roles.ts` yet).
- [ ] **[AI] GREEN** Add `roles.ts` — the `ladder` metadata + the full `salaries` matrix from the
      research step, with sourced-estimate comments and `snapshotDate`. File:
      `.../data/roles.ts`. Acceptance: `roles.test.ts` passes.
- [ ] **[AI] RED** Add `role-lookup.test.ts` covering `roleSalaryUsd`, `candidateSavingsUsd`,
      `bestCityForRole`, `resolveBaselineUsd` (all three baseline sources), `rankLadder`,
      `minimumRole`, and `toDisplayCurrencies`, including: the no-qualifier case (`minimumRole` →
      `null`); reference-role baseline parity (a reference role clears its own bar); cost-basis
      changes (household/area/school) shifting candidates; and confidence propagation to the chosen
      row. File: `.../role-lookup.test.ts`. Acceptance: fails (no `role-lookup.ts` yet).
- [ ] **[AI] GREEN** Implement pure `role-lookup.ts` per `tech-docs.md` (USD-normalised qualify,
      seniority-ordered display, lowest-rank minimum). File: `.../role-lookup.ts`. Acceptance:
      `role-lookup.test.ts` passes.
- [ ] **[AI] REFACTOR** Tidy types/naming in `role-lookup.ts` and `roles.ts`; ensure `role-lookup.ts`
      is React-free and side-effect-free (no React imports, no `console.log`, no module-level
      mutation) and reuses `calc.ts` rather than duplicating cost math. Acceptance:
      `npx nx run ayokoding-www:test:unit` exits 0; `npx nx run ayokoding-www:lint` exits 0.

### Phase 1b Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (all `roles.test.ts` and
      `role-lookup.test.ts` assertions pass).
- [ ] [AI] `npx nx run ayokoding-www:lint` — exits 0 with no errors on the new role data/lookup files.
- [ ] [AI] Full-matrix check: `roles.ts` `salaries` key set equals `cities.ts` city-ID set (no
      missing or extra cities) — asserted by `roles.test.ts`.
- [ ] [AI] No Israeli city in role matrix: grep for `ILS` and `Israel` returns 0 results in
      `roles.ts`.

> **Pause Safety**: both datasets and both pure cores (`calc.ts` + `role-lookup.ts`) are complete,
> unit-tested, and lint-clean. No UI code exists yet. Safe to stop. To resume:
> `npx nx run ayokoding-www:test:unit` — must still pass before Phase 2.

## Phase 2 — Interactive Page (TDD)

The Compare-all mode needs a `Table` primitive that `libs/web-ui` does not yet ship (existing
inventory — `Tabs`/`TabBar`, `Input`, `Label`, `Toggle`, `DropdownMenuRadioGroup`, `Command`,
`Alert`/`InfoTip`, `Badge`, `Card`/`StatCard` — covers every other control). Build that primitive
in the shared lib first, then consume it from the app. Changes under `libs/web-ui` are picked up by
the `nx affected` quality gates in Phase 4.

- [ ] **[AI] RED** Add a unit test for a new `Table` primitive in `libs/web-ui` following the existing primitive pattern (e.g. `libs/web-ui/src/primitives/table/table.test.tsx`): assert it renders `Table`/`TableHeader`/`TableBody`/`TableRow`/`TableHead`/`TableCell`/`TableCaption` with correct semantic roles. Command: `npx nx run web-ui:test:unit`. Acceptance: fails (no component yet).
- [ ] **[AI] GREEN** Create the `Table` primitive (delegate to `swe-ui-maker`): `libs/web-ui/src/primitives/table/table.tsx` (shadcn `Table` family, CVA variants, semantic `<table>` markup, AA-contrast tokens), barrel-export it from `libs/web-ui/src/index.ts`, and add `libs/web-ui/src/primitives/table/table.stories.tsx`. Acceptance: `npx nx run web-ui:test:unit` exits 0; `npx nx run web-ui:lint` exits 0; `npx nx run web-ui:build-storybook` succeeds.
- [ ] **[AI] RED** Add a component test for "Compare all": render the table from a sample salary, assert a row shows the cost of living in both local currency and USD, plus savings % and a local-currency amount. File: `.../components/compare-table.test.tsx`. Command: `npx nx run ayokoding-www:test:unit`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `compare-table.tsx` (sortable table consuming the calc core and the new `Table` primitive from `@open-sharia-enterprise/web-ui`). Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for "Single city": select a city, enter local salary, assert cost/savings breakdown (cost shown in both local currency and USD) incl. deficit case. File: `.../components/single-city.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `single-city.tsx`. Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for "Minimum role": set a savings-target baseline, assert the role ladder renders ascending by seniority via the `Table`, the lowest qualifying role is marked as the minimum, below-bar roles are de-emphasised, each row shows savings in USD + local + display currency, switching the baseline source (my salary / reference role / savings target) recomputes, and the no-qualifier message shows when the target exceeds every role's best-city savings. File: `.../components/min-role.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement `min-role.tsx` (baseline selector, display-currency picker, ranked ladder table consuming `role-lookup.ts` + the shared `Table`, minimum marker + confidence badges + summary line). Acceptance: test passes.
- [ ] **[AI] RED** Add a component test for the shared controls: household selector, area toggle (`center`/`rural`), and a school-type toggle (`public`/`private`) that is **hidden** for childless households and **shown** once kids are selected; assert cost/savings recompute when each changes. File: `.../components/controls.test.tsx`. Acceptance: fails.
- [ ] **[AI] GREEN** Implement the shared controls (household select, area toggle, conditional school-type toggle). Acceptance: test passes.
- [ ] **[AI] GREEN** Add `page.tsx` (`'use client'`) at `apps/ayokoding-www/src/app/[locale]/tools/salary-savings/page.tsx` with the **three-way** mode toggle wiring `compare-table`, `single-city`, and `min-role` + salary, household, area, school-type, and minimum-role (baseline source, reference city/role, savings target, display currency) state. Acceptance: route renders in dev (`npx nx dev ayokoding-www`, visit `/en/tools/salary-savings`) with all three tabs reachable.
- [ ] **[AI] REFACTOR** Extract shared `Intl.NumberFormat` formatting logic into a shared helper (e.g.
      `formatCurrency(amount, currency, locale)`); de-duplicate formatting calls across
      `apps/ayokoding-www/src/features/salary-savings/components/compare-table.tsx`,
      `apps/ayokoding-www/src/features/salary-savings/components/single-city.tsx`,
      `apps/ayokoding-www/src/features/salary-savings/components/min-role.tsx`, and
      `apps/ayokoding-www/src/app/[locale]/tools/salary-savings/page.tsx` (or equivalent paths
      confirmed in Phase 0). Acceptance: `npx nx run ayokoding-www:test:unit` exits 0; no test
      regressions.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx run web-ui:test:unit` and `npx nx run web-ui:lint` — both exit 0 (new `Table`
      primitive tested and lint-clean); `npx nx run web-ui:build-storybook` succeeds.
- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (all component tests for `compare-table`,
      `single-city`, `min-role`, and `controls` pass).
- [ ] [AI] Dev server check: `npx nx dev ayokoding-www` starts; navigate to `/en/tools/salary-savings`
      — page renders without a crash and all three tabs are reachable.
- [ ] [AI] `npx nx run ayokoding-www:lint` — exits 0 on all new component files.

> **Pause Safety**: all three calculator modes render and compute correctly with full component test
> coverage; dev server verified. Bilingual strings and a11y not yet applied. Safe to stop.
> To resume: `npx nx run ayokoding-www:test:unit` — must still pass before Phase 3.

## Phase 3 — Bilingual Strings + Polish

- [ ] **[AI]** Edit `apps/ayokoding-www/src/features/i18n/core/translations.ts` — add all
      calculator UI strings (headings, labels, mode names incl. "Minimum role", household-type
      labels, area + school-type toggle labels, **baseline-source labels** (my salary / reference
      role / savings target), **display-currency label**, **confidence-tier labels** (estimate /
      lower-confidence), "estimates only" disclaimer, "Gross monthly salary (before tax)" salary
      label, "Data last updated" label) for both `en` and `id` locales, following the existing
      `Record<Locale, Record<string, string>>` shape in that file. Wire the new keys into the
      calculator page and components. Role labels come from `roles.ts` (`ladder[].label.en/id`).
      Acceptance: `/id/tools/salary-savings` shows Indonesian labels for all calculator UI elements
      including the minimum-role tab.
- [ ] **[AI]** Label salary inputs "Gross monthly salary (before tax)"; show a prominent, localized
      **"Data last updated: &lt;date&gt;"** label (formatted from `snapshotDate` via `Intl.DateTimeFormat`)
      near the results, plus a disclaimer covering "estimates only", "gross/pre-tax — taxes not
      modelled, real savings lower", "household/rural costs use shared multipliers and school costs
      are city medians", and "transport assumes public transport — car ownership not modelled".
      Acceptance: last-updated date, gross-salary labels, and disclaimer clearly visible in both
      locales.

### Manual UI Verification (Playwright MCP)

- [ ] [AI] Start dev server: `npx nx dev ayokoding-www` (port 3101).
- [ ] [AI] `browser_navigate` to `http://localhost:3101/en/tools/salary-savings` — acceptance: page
      loads without JS errors.
- [ ] [AI] `browser_snapshot` — verify calculator UI renders with salary input, mode toggle
      ("Compare all" / "Single city" / "Minimum role"), household selector, and area toggle all
      visible.
- [ ] [AI] `browser_fill_form` salary input with `"8000"`, then `browser_click` "Compare all" /
      sort trigger — acceptance: table populates with city rows showing cost of living in both local
      currency and USD, plus a savings % and savings amount column.
- [ ] [AI] `browser_click` the "Minimum role" tab, set the baseline source to "savings target", and
      `browser_fill_form` the target with `"2000"` — acceptance: the role ladder renders with a row
      marked as the minimum and below-bar rows de-emphasised; savings show in USD + local + display
      currency. Resize narrow (`browser_resize` to ~375 px) — acceptance: the ladder reflows to
      stacked cards (responsive mobile layout) without overflow.
- [ ] [AI] `browser_console_messages` — acceptance: zero JS errors.
- [ ] [AI] `browser_navigate` to `http://localhost:3101/id/tools/salary-savings`, then
      `browser_snapshot` — acceptance: all labels, headings, and the disclaimer are in Indonesian.
- [ ] [AI] `browser_take_screenshot` — save as visual record for this phase.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run ayokoding-www:test:unit` — exits 0 (no regressions from i18n wiring).
- [ ] [AI] `/en/tools/salary-savings` and `/id/tools/salary-savings` both render correctly —
      confirmed by Playwright MCP `browser_navigate` + `browser_snapshot` steps above.
- [ ] [AI] All calculator UI strings present in both `en` and `id` keys in
      `apps/ayokoding-www/src/features/i18n/core/translations.ts` — grep for the salary-label
      key in both locale branches returns a non-empty string.
- [ ] [AI] "Data last updated" label and "estimates only" disclaimer visible in both locales —
      confirmed by `browser_snapshot` above.
- [ ] [AI] Zero JS errors on either locale URL — confirmed by `browser_console_messages` above.

> **Pause Safety**: bilingual strings complete, disclaimer visible, a11y/responsive verified,
> Playwright MCP smoke passed in both locales. Safe to stop. To resume: re-run the Playwright
> MCP verification steps above — both locale URLs must render without JS errors.

## Phase 4 — E2E + Local Quality Gates

- [ ] **[AI] RED** Add a failing fe-e2e smoke test in
      `apps/ayokoding-www-fe-e2e/src/salary-savings.spec.ts` (_New file_): navigate to
      `/en/tools/salary-savings`, enter a salary of `"8000"`, assert the results table is populated
      and at least one savings cell is visible; then switch to the "Minimum role" tab, set a savings
      target of `"2000"`, and assert a role row is marked as the minimum. Command:
      `npx nx run ayokoding-www-fe-e2e:test:e2e`. Acceptance: test file exists and the test fails
      (page route not yet reached by e2e or an element assertion fails when written before the page is
      fully wired).
- [ ] **[AI] GREEN** Confirm that `npx nx run ayokoding-www-fe-e2e:test:e2e` passes with the
      calculator page fully implemented from Phases 1–3. Acceptance: smoke test passes end-to-end
      with zero errors.
- [ ] **[AI]** Run affected local quality gates: `npx nx affected -t typecheck lint test:quick specs:coverage` — warm the cache first (`npx nx affected -t typecheck lint test:quick specs:coverage --skip-nx-cache` if cache is cold). Fix ALL failures encountered — including preexisting issues not introduced by this plan's changes (root-cause orientation: do not defer or mention-and-skip existing failures). Acceptance: all four targets exit 0.

### Commit Guidelines

- Commit changes thematically: city data layer (`cities.ts` + `calc.ts`) in one commit, role data
  layer (`roles.ts` + `role-lookup.ts`) in a second, UI components in a third, bilingual strings in a
  fourth, e2e in a fifth. Follow Conventional Commits format:
  `feat(ayokoding-www): add salary-savings calculator`.
- Do NOT bundle unrelated fixes into the same commit. Note: commits happen only on explicit user
  instruction per repo policy.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` — exits 0 (smoke test passes).
- [ ] [AI] `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] `npx nx affected -t lint` — exits 0.
- [ ] [AI] `npx nx affected -t test:quick` — exits 0.
- [ ] [AI] `npx nx affected -t specs:coverage` — exits 0.

> **Pause Safety**: all local quality gates green and e2e smoke passing. Safe to stop before push.
> To resume: `npx nx affected -t typecheck lint test:quick specs:coverage` — all must still exit 0.

## Phase 5 — Post-Push CI Verification

- [ ] **[HUMAN]** Review the diff and approve push to `main` (trunk-based). Observable resume
      signal: user confirms approval; verify with `git log --oneline -1 origin/main` after push shows
      the new commit.
- [ ] **[AI]** Push and trigger/monitor relevant GitHub Actions for `ayokoding-www` (poll every
      3 min via `gh run list --limit 5` + `gh run view <run-id> --json status,conclusion`; do not use
      `gh run watch`). Acceptance: CI green.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `gh run list --limit 5 --json status,conclusion,name` — all runs triggered by this
      push show `conclusion: success`.
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

- [ ] [AI] `test ! -d plans/in-progress/ayokoding-www-salary-savings-calculator && echo "OK"` —
      plan folder no longer exists under `in-progress/`.
- [ ] [AI] `ls plans/done/ | grep ayokoding-www-salary-savings-calculator` — folder exists under
      `done/` with a date prefix.
- [ ] [AI] `plans/in-progress/README.md` no longer lists this plan; `plans/done/README.md` lists
      it with a completion date.

> **Pause Safety**: plan archived, worktree cleaned up. Feature live at
> `/[locale]/tools/salary-savings` in `en` + `id`. No further action required.
