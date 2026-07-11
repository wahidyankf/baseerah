# 29 · Advanced Frontend (By Example, TypeScript †)

**prd row**: Pass 3 · Build for the Real World · By Example · TypeScript † · Learn 129 / Drill 229 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the deep frontend pass — performance (Core Web Vitals), rendering strategies
(CSR/SSR/SSG/streaming), state-at-scale, advanced accessibility (ARIA/focus/live regions), and applied
component + e2e testing from the CLI. The usable slice is the prerequisite
[`12-frontend-essentials`](./12-frontend-essentials.md).

## Prerequisites

- **Prior topics**: [topic 12 Frontend Essentials](./12-frontend-essentials.md) (components, state,
  accessible forms), [topic 11 Just Enough TypeScript](./11-just-enough-typescript.md), and
  [topic 13 Software Testing](./13-software-testing.md) (Testing-Library + e2e).
- **Tools & environment**: a macOS/Linux terminal; **Node.js** + a pinned CVE-clean UI framework + build
  tool; **Vitest** + Testing-Library; **Playwright** for an e2e smoke; CLI **Lighthouse** for perf
  measurement; a browser.
- **Assumed knowledge**: building a typed component with state + an accessible form (topic 12); running a
  component test from the CLI (topic 13).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: Core Web Vitals thresholds current for 2026 — **LCP** good ≤2.5s (poor >4.0s),
  **INP** good <200ms (poor >500ms), **CLS** good <0.1 (poor >0.25), at the 75th percentile over a rolling
  28-day CrUX window. **INP replaced FID** (March 2024) — the syllabus's LCP/INP/CLS triad (not FID) is
  correctly up to date. Playwright/Lighthouse CLI + ARIA practices actively evolve — spot-check concrete
  CLI invocations at authoring. (developers.google.com/search/docs/appearance/core-web-vitals)

## Items

- Performance: Core Web Vitals (LCP/INP/CLS), bundle size, code-splitting, lazy loading, memoization,
  render cost.
- Rendering strategies: CSR vs SSR vs SSG vs streaming; hydration; islands.
- State management at scale: shared/derived state, data fetching & caching, optimistic updates.
- Advanced accessibility: ARIA patterns, focus management, live regions.
- Forms at scale; error boundaries.
- Applied testing: component + interaction tests (Testing-Library), and an e2e smoke (Playwright) from
  the CLI.

## Worked examples

Colocated under `advanced-frontend/learning/code/`; each runnable + measured from the CLI (DD-20/DD-30).

- **beginner** — a memoization/perf fix on a slow list; measuring with CLI Lighthouse.
- **intermediate** — an SSR/hydration example; a data-fetch cache with invalidation.
- **advanced** — a state-managed feature with derived selectors + optimistic update; an ARIA-correct custom
  widget with an interaction test.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a performant, accessible data-driven feature (e.g. a searchable, paginated dashboard
  panel) with SSR/streaming, cached data fetching + optimistic updates, derived state, an ARIA-correct
  custom widget, and error boundaries — measured with Lighthouse and covered by component + e2e tests.
- **Concepts exercised**: [ ] a measured perf fix (Core Web Vitals) [ ] SSR/streaming + hydration
  [ ] cached fetch + optimistic update [ ] derived/shared state [ ] an ARIA-correct widget + focus/live
  region [ ] Testing-Library + a Playwright e2e smoke.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — the feature with SSR + a slow baseline. Verify it renders server-side
     and record a baseline Lighthouse score.
  2. Apply perf work (code-split, memoize, lazy-load). Verify Lighthouse LCP/INP improve measurably vs
     baseline.
  3. Add cached data fetching + an optimistic update + derived selectors. Verify the optimistic update
     rolls back on a simulated failure.
  4. Add an ARIA-correct custom widget with focus management + a live region; write a Testing-Library
     interaction test and a Playwright e2e smoke. Verify both pass and the widget is keyboard-operable.
- **Acceptance criteria**: measured Core Web Vitals improve over baseline; optimistic update + rollback
  works; the widget is ARIA-correct and keyboard-operable; component + e2e tests green.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [28 · Backend at Scale](./28-backend-at-scale.md) · Next: [30 · Software Architecture](./30-software-architecture.md) →
