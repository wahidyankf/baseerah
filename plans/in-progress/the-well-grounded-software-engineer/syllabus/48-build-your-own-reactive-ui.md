# 48 · Build Your Own Reactive UI (By Example, TypeScript †)

**prd row**: Pass 3 · Build for the Real World · By Example · TypeScript † · Learn 148 / Drill 248 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the build-your-own tier for the frontend band — a minimal reactive UI runtime that
demystifies React/Vue/Solid by building the core two ways: a virtual DOM + diff/patch, and a
signals-based fine-grained reactive graph, each with its own render loop. Interleaved after
[`47-advanced-frontend`](./47-advanced-frontend.md), it turns "the framework re-renders" into a
mechanism you can single-step. `†`: TypeScript, strict-mode typed throughout — components, virtual
nodes, and signals are all fully typed.

## Why this exists · the big idea

- **The problem before the solution**: hand-written DOM code drifts out of sync with your data — you
  change state in one place and forget to update the three spots that display it, and "why didn't the
  screen update?" becomes a daily bug. Frameworks solve this, but as a black box you can't debug a
  stale render or a performance cliff you don't understand.
- **Keep-this-if-you-forget-everything**: a reactive UI is a function of state to view, plus a way to
  re-run only what changed — whether by diffing a virtual tree against the last one, or by tracking
  which computations read which signals and re-running exactly those. Same goal, two mechanisms.
- **Big ideas touched**: `abstraction-and-its-cost` (a framework hides the DOM behind declarative
  render — building it reveals the diff/subscription bookkeeping that convenience costs, and where it
  bites performance), `taming-state` (the entire topic is a strategy for making mutable UI state
  reason-about-able — the virtual DOM and the signal graph are two different disciplines for the same
  enemy).

## Prerequisites

- **Prior topics**: [topic 13 Just Enough TypeScript](./13-just-enough-typescript.md) (generics,
  discriminated unions, strict typing) and [topic 47 Advanced Frontend](./47-advanced-frontend.md)
  (rendering models, reconciliation, the framework you're now rebuilding).
- **Tools & environment**: a macOS/Linux terminal; **Node.js** at a current LTS with **TypeScript**
  in strict mode; a bundler/dev server and a jsdom-or-browser test harness; no UI framework — that's
  the point; Neovim/VSCode with the TypeScript LSP (DD-17).
- **Assumed knowledge**: DOM APIs and events from the outside (topic 14/47); using a component
  framework as a consumer (topic 47); TypeScript generics and unions (topic 13).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the two mechanisms are correctly framed as coexisting, not one superseding
  the other — the **virtual-DOM + reconciler** lineage (React's Fiber) and the **fine-grained signals**
  lineage (Solid, Vue's reactivity, Preact Signals) are both current, mainstream approaches. Left
  version-unpinned since the concepts, not any framework release, are the subject.
- 2026-07-12 — verified (GAP for plan owner): signals have broadly converged across frameworks but the
  exact API surface (e.g. a proposed TC39 Signals standard) is still moving — teach the _mechanism_
  (dependency tracking + a reactive graph) rather than any one library's or proposal's current API,
  and re-check the standardization status at drafting time.

## Items

- The core contract: view as a function of state, and the render loop that keeps the DOM matching it.
- Virtual DOM path: a typed `h()`/hyperscript, a virtual node tree, and a diff/patch that computes and
  applies the minimal DOM mutations between two trees.
- Reconciliation basics: keys, list diffing, and why identity matters for correctness and performance.
- Signals path: a reactive primitive (`signal`/`computed`/`effect`) that tracks which computations
  read it and re-runs exactly those on change — fine-grained updates without a full diff.
- Components and composition: local state, props, and a lifecycle/cleanup hook in each model.
- The trade-off made concrete: measure update cost of the virtual-DOM vs signals approach on the same
  UI and see where each wins.

## Worked examples

Colocated under `build-your-own-reactive-ui/learning/code/`; each runnable in a browser/jsdom harness,
all TypeScript strict-mode typed (DD-20/DD-30).

- **beginner** — a typed `h()` + `render()` that mounts a virtual tree to the DOM, and a `diff/patch`
  that updates only a changed text node between two renders.
- **intermediate** — add keyed list reconciliation; show that reordering a keyed list moves nodes
  instead of recreating them.
- **advanced** — implement `signal`/`computed`/`effect` with dependency tracking and rebuild the same
  counter/list UI on signals; compare the update path and cost against the virtual-DOM version.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a minimal reactive UI runtime twice over the same small app (a to-do/counter list)
  — once as a typed virtual DOM with keyed diffing and a render loop, once as a fine-grained signal
  graph — and produce a short measured comparison of their update behaviour, proving you understand
  both mechanisms the mainstream frameworks use.
- **Concepts exercised**: [ ] hyperscript/virtual node tree [ ] diff/patch to minimal DOM mutations
  [ ] keyed list reconciliation [ ] signal/computed/effect dependency tracking [ ] component
  state + cleanup [ ] a measured virtual-DOM-vs-signals comparison.
- **Ordered steps**:
  1. `.../learning/capstone/code/vdom/` — a typed `h`/`render`/`diff`/`patch` runtime. Verify the same
     app re-renders by mutating only changed nodes (assert DOM node identity is preserved where
     unchanged); TypeScript strict, no `any`.
  2. `.../learning/capstone/code/vdom/reconcile.ts` — add keyed list diffing. Verify a reorder moves
     existing nodes rather than recreating them.
  3. `.../learning/capstone/code/signals/` — a `signal`/`computed`/`effect` runtime with dependency
     tracking. Verify that changing one signal re-runs only its dependents.
  4. `.../learning/capstone/code/compare.md` + a bench script — build the app on both and measure. Verify
     the note reports where each mechanism does more/less work and why.
- **Acceptance criteria**: both runtimes render and update the app correctly; keyed diffing preserves
  node identity; signal updates are fine-grained; the comparison is concrete and measured; all
  TypeScript is strict-mode typed with no `any`.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Papers & articles**

- **Build your own React** — Rodrigo Pombo (2019). The most widely shared, interactive walkthrough of
  building a Fiber-style reconciler and virtual DOM from scratch, based on React 16.8's architecture.
  <https://pomb.us/build-your-own-react/>
- **How to write your own Virtual DOM** — Denis Radin (2017). Compact, widely referenced tutorial
  implementing a virtual DOM's `h`/`render`/`diff`/`patch` functions from first principles.
  <https://medium.com/@deathmood/how-to-write-your-own-virtual-dom-ee74acc13060>
- **Svelte 3: Rethinking Reactivity** — Rich Harris (2019). The post/talk that reframed UI reactivity
  around compile-time signals rather than a virtual DOM, influential across the signals-based framework
  generation (Solid, Vue 3, Preact Signals). <https://svelte.dev/blog/svelte-3-rethinking-reactivity>
- **Building a Reactive Library from Scratch** — Ryan Carniato (2020). Written by SolidJS's creator;
  walks through implementing fine-grained signal-based reactivity from first principles.
  <https://dev.to/ryansolid/building-a-reactive-library-from-scratch-1i0p>

---

← Previous: [47 · Advanced Frontend](./47-advanced-frontend.md) · Next: [49 · Information Architecture & SEO](./49-information-architecture-and-seo.md) →
