# 14 · Frontend Essentials (By Example, TypeScript †)

**prd row**: Pass 1 · Core Foundations · By Example · TypeScript † · Learn 114 / Drill 214 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** — the platform (HTML/CSS/DOM), the component model, accessible forms,
and TS-for-UI, with applied component testing. Performance, SSR, and state-at-scale go to
[`47-advanced-frontend`](./47-advanced-frontend.md) (DD-11). WCAG AA intro here (Accessibility First).

## Why this exists · the big idea

- **The problem before the solution**: UIs are stateful and users are unpredictable; hand-mutating the
  DOM on every event becomes an untraceable tangle of who-changed-what.
- **Keep-this-if-you-forget-everything**: the UI is a _function of state_ — you change state and let the
  render derive the DOM, never poke the DOM directly; data flows one way.
- **Big ideas touched**: `taming-state` — unidirectional data flow makes UI state a single source of truth
  instead of scattered mutations; `abstraction-and-its-cost` — the component model buys reuse and charges
  a render/reconciliation layer between you and the DOM.

## Prerequisites

- **Prior topics**: [topic 13 Just Enough TypeScript](./13-just-enough-typescript.md) (all UI code is
  typed TS); applied testing cross-refs [topic 15 Software Testing](./15-software-testing.md).
- **Tools & environment**: a macOS/Linux terminal; **Node.js** + npm/pnpm; a pinned CVE-clean UI
  framework + build tool; **Vitest** + Testing-Library for component tests; a modern **web browser**.
- **Assumed knowledge**: TypeScript basics (types, unions, async) from topic 13; willingness to learn
  HTML/CSS (introduced here, no prior web experience assumed).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring.

- 2026-07-12 — verified: cite **WCAG 2.2 AA** explicitly as the current baseline (not 2.1) — it underpins
  the EU Accessibility Act, EN 301 549, and Section 508 in 2026. Confirm at w3.org/WAI/WCAG22.
  (secondary a11y sources; w3.org for primary)
- 2026-07-12 — verified: **Vitest 4.1.10** (5.0 in beta). `@testing-library/react` current ≈ **v16.3.0**
  (v16+ needs `@testing-library/dom` as explicit peer dep) — npmjs.com returned 403, so re-check at
  github.com/testing-library/react-testing-library/releases before authoring. UI framework/build tool
  deliberately unnamed in the syllabus — pin CVE-clean versions when the maker picks them. (vitest.dev)

## Items

- **The platform**: HTML semantics, CSS layout basics (flow/flexbox/grid intro), the DOM, the browser
  event loop intro.
- **The vanilla tier first**: build one interactive feature with raw DOM APIs (`document.querySelector`,
  `addEventListener`, manual `textContent` updates) before reaching for a component framework — so
  reactivity is understood as a convenience, not a mystery.
- **Component model**: components, props, state, unidirectional data flow.
- **Rendering a list**; handling events; controlled form inputs + basic validation.
- **Accessibility basics**: semantic markup, labels, keyboard nav (WCAG AA intro).
- **TypeScript for UI**: typing props/state; discriminated unions for loading/error/empty states.
- **Applied testing**: component unit tests with Vitest/Testing-Library from the CLI (cross-ref
  `software-testing`).

## Worked examples

Colocated under `frontend-essentials/learning/code/`; each a runnable component + a Testing-Library test
(DD-20/DD-30).

- **beginner** — a typed counter component with props & state; a semantic, accessible form.
- **intermediate** — a data-list component with loading/error/empty states; a controlled, validated form.
- **advanced** — a small feature with shared state + derived values; an accessibility fix of a broken
  widget (with a Testing-Library test).

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small accessible single-feature UI (e.g. a filterable task list with an add form)
  with typed props/state, loading/error/empty states via a discriminated union, keyboard-accessible
  controls, and Testing-Library unit tests — runnable and testable from the CLI.
- **Concepts exercised**: [ ] components + props + state [ ] list rendering + events [ ] controlled
  validated form [ ] discriminated-union UI states [ ] WCAG AA semantics + keyboard nav [ ] Vitest +
  Testing-Library tests.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — the app scaffold + a typed `TaskList` component. Verify
     `npm run test` (Vitest) passes the initial render test.
  2. Add the add-form (controlled, validated) + filter. Verify tests cover valid/invalid submit + filter.
  3. Wire loading/error/empty states via a discriminated union. Verify each state renders + is tested.
  4. Accessibility pass: labels, roles, keyboard nav. Verify a Testing-Library query-by-role test passes.
- **Acceptance criteria**: all Vitest tests green; the feature is keyboard-operable; every UI state is
  reachable and tested; `tsc --noEmit` clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Don't Make Me Think, Revisited** — Steve Krug (3rd ed., 2014). Classic plain-language web-usability guide; the field's most-cited entry point.
- **CSS: The Definitive Guide** — Meyer, Weyl (5th ed., 2023). Canonical deep reference on CSS layout, box model, specificity.
- **Inclusive Components** — Heydon Pickering (2021). Accessibility-first patterns for building common UI components correctly.

**Papers & articles**

- **HTML Living Standard** — WHATWG (continuously updated). Official canonical HTML spec, versionless since 2011. <https://html.spec.whatwg.org/multipage/>
- **Web Content Accessibility Guidelines (WCAG) 2.2** — W3C (2023). Normative accessibility standard behind WCAG AA. <https://www.w3.org/TR/WCAG22/>

---

← Previous: [13 · Just Enough TypeScript](./13-just-enough-typescript.md) · Next: [15 · Software Testing](./15-software-testing.md) →
