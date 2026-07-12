# 47 · Advanced Frontend (By Example, TypeScript †)

**prd row**: Pass 3 · Build for the Real World · By Example · TypeScript † · Learn 147 / Drill 247 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the deep frontend pass — performance (Core Web Vitals), rendering strategies
(CSR/SSR/SSG/streaming), state-at-scale, advanced accessibility (ARIA/focus/live regions), and applied
component + e2e testing from the CLI. The usable slice is the prerequisite
[`14-frontend-essentials`](./14-frontend-essentials.md).

## Why this exists · the big idea

- **The problem before the solution**: a UI that renders correctly can still fail its users — slow to paint,
  janky to interact, inaccessible to a screen reader, and impossible to reason about once state sprawls.
- **Keep-this-if-you-forget-everything**: the frontend's hard problems are state and time — derive UI from
  state, push data-fetching and caching to well-defined edges, and _measure_ performance instead of guessing.
- **Big ideas touched**: `taming-state` (state-at-scale is the central difficulty),
  `consistency-latency-throughput` (Core Web Vitals and rendering strategy are latency/throughput trade-offs).

## Prerequisites

- **Prior topics**: [topic 14 Frontend Essentials](./14-frontend-essentials.md) (components, state,
  accessible forms), [topic 13 Just Enough TypeScript](./13-just-enough-typescript.md), and
  [topic 15 Software Testing](./15-software-testing.md) (Testing-Library + e2e).
- **Tools & environment**: a macOS/Linux terminal; **Node.js** + a pinned CVE-clean UI framework + build
  tool; **Vitest** + Testing-Library; **Playwright** for an e2e smoke; CLI **Lighthouse** for perf
  measurement; a browser.
- **Assumed knowledge**: building a typed component with state + an accessible form (topic 14); running a
  component test from the CLI (topic 15).

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
- Progressive Web Apps: service workers, offline caching, installability, and the app-shell model.
- WebAssembly in the browser: when to drop to Wasm for compute-heavy work, and the JS↔Wasm boundary cost.
- Internationalization (i18n): locale-aware formatting, message catalogs, RTL layout, translation-safe UI.
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

## Read more

**Books**

- **High Performance Browser Networking** — Ilya Grigorik (2013). Free, author-hosted classic on the networking layer (TCP, TLS, HTTP/2) that underlies frontend performance work. <https://hpbn.co/>
- **Micro Frontends in Action** — Michael Geers (2020). Standard reference for scaling frontend architecture and rendering ownership across teams.

**Papers & articles**

- **Rendering on the Web** — Google Chrome / web.dev team (ongoing). Canonical, framework-agnostic explanation of CSR/SSR/SSG/ISR rendering patterns and their tradeoffs. <https://web.dev/articles/rendering-on-the-web>
- **React Documentation** — Meta / React core team (ongoing). The authoritative source on modern React internals (reconciliation, concurrent rendering, hooks) that much of "advanced frontend" curricula builds on. <https://react.dev/learn>
- **The Cost of JavaScript in 2019** — Addy Osmani (2019), V8 team blog. Widely cited, data-driven reframing of frontend performance around JavaScript parse/compile/execute cost, not just download size. <https://v8.dev/blog/cost-of-javascript-2019>

---

← Previous: [46 · Distributed Systems](./46-distributed-systems.md) · Next: [48 · Build Your Own Reactive UI](./48-build-your-own-reactive-ui.md) →
