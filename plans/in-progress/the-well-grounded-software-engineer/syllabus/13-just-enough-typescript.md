# 13 · Just Enough TypeScript (Primer, TypeScript †)

**prd row**: Pass 1 · Core Foundations · Primer · TypeScript † · Learn 113 / Drill 213 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: just enough TypeScript to be productive in
[`14-frontend-essentials`](./14-frontend-essentials.md) / [`47-advanced-frontend`](./47-advanced-frontend.md)
and the TS side of [`15-software-testing`](./15-software-testing.md). Node/TS are Tier-1 OSS (DD-21).

## Why this exists · the big idea

- **The problem before the solution**: JavaScript runs the web but has no types, so whole classes of bug
  only surface at runtime in front of a user — TypeScript moves those failures to compile time.
- **Keep-this-if-you-forget-everything**: a type is a compile-time proof about a runtime shape, checked
  structurally (by shape, not by name) — you pay in annotations and buy caught errors.
- **Big ideas touched**: `correctness-vs-pragmatism` — TS is _gradual_: `any`/`unknown`/`never` let you
  dial rigor up where correctness matters and stay loose where speed does.

## Prerequisites

- **Prior topics**: [topic 1 Just Enough Nvim](./01-just-enough-nvim.md) (to edit/run); prior
  programming maturity from [topic 4 Just Enough Python](./04-just-enough-python.md) helps but is not
  required.
- **Tools & environment**: a macOS/Linux terminal; **Node.js** (`node --version`) + **npm**/pnpm; the
  **`tsc`** compiler and **`tsx`**/ts-node runner; eslint/prettier CLIs.
- **Assumed knowledge**: basic programming concepts (variables, functions, control flow) from any prior
  language; basic terminal use.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). **TS 7.0 landed days before this
> sweep — re-check devblogs.microsoft.com/typescript immediately before authoring.**

- 2026-07-12 — verified (CORRECTION, time-sensitive): **TypeScript 7.0 became stable 2026-07-08** — a
  native-Go compiler rewrite ("Project Corsa", ~10-12x faster type-checking; TS 6.0 / 2026-03-23 was the
  last JS-based compiler). **Caveat**: full editor/tooling support for Vue, Svelte, Astro, MDX is NOT yet
  on 7.0 (pending a programmatic API in 7.1) — flag this if the topic teaches broad editor tooling.
  (visualstudiomagazine.com; confirm at devblogs.microsoft.com/typescript)
- 2026-07-12 — verified: **Node 24 ("Krypton") is Active LTS**; Node 22 Maintenance LTS; Node 26 Current
  (LTS Oct 2026). **`tsx` (~4.23.0) is the dominant TS-run tool** for new projects; `ts-node` is stale
  (no major since 2021), not recommended for new work. (nodejs.org / npmjs.com)
- 2026-07-12 — verified: minimal `tsconfig.json` — `target: ES2022`, `module: ESNext`
  (or `NodeNext`/`Bundler` resolution), `strict: true`. Whether TS 7.0's Go compiler changes any
  `tsconfig` field semantics (vs. perf only) is unverified — targeted check at authoring. (typescriptlang.org)

## Items

- **Running TS raw**: `node`, `tsc`, `tsx`/`ts-node`, `npm`/`pnpm` from the terminal; a minimal
  `tsconfig.json`.
- **Types**: primitives, arrays, tuples, object types, unions, literals, `type` vs `interface`.
- **Functions**: typing params/returns, optional/default params, arrow functions.
- **Narrowing**; generics intro; `unknown`/`any`/`never`; structural typing.
- **Modules**: `import`/`export` (ESM); Promises + `async`/`await`.
- **Tooling**: eslint/prettier via CLI; running a script with `tsx`.

## Worked examples

Colocated under `just-enough-typescript/learning/code/`; each run via `tsx <file>` (DD-20/DD-30).

- **beginner** — a typed function + union narrowing, run via `tsx`.
- **intermediate** — a generic utility; a discriminated union for a small state.
- **advanced** — an async data-fetch function typed end to end, run from the CLI.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: write one small (~80–150-line) typed TypeScript CLI/module that uses a discriminated union
  for state, a generic utility, and an `async`/`await` data flow — run from the terminal with `tsx`,
  type-checked clean with `tsc --noEmit`.
- **Concepts exercised**: [ ] `tsconfig` + `tsx` run [ ] union + narrowing [ ] a generic function
  [ ] discriminated union state (loading/error/success) [ ] `async`/`await` + typed Promise [ ] ESM
  imports.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — `tsconfig.json` + `src/state.ts` (discriminated union) + a generic
     `src/util.ts`. Verify `npx tsc --noEmit` passes.
  2. `src/main.ts` — an async flow producing loading→success/error states, narrowed at the call-site.
     Verify `npx tsx src/main.ts` prints the expected transitions.
  3. Add a deliberate type error, show `tsc` catching it, then fix. Verify clean type-check.
- **Acceptance criteria**: `tsc --noEmit` clean; `tsx src/main.ts` runs and prints expected output;
  eslint/prettier clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Programming TypeScript** — Boris Cherny (2019). Comprehensive tour of the type system for engineers coming from JavaScript.
- **Effective TypeScript** — Dan Vanderkam (2nd ed., 2024). Item-based best-practices applying the "Effective X" format to TypeScript.

**Papers & articles**

- **TypeScript Handbook** — Microsoft TypeScript team (continuously updated). Authoritative official reference. <https://www.typescriptlang.org/docs/handbook/intro.html>

---

← Previous: [12 · Networking Essentials](./12-networking-essentials.md) · Next: [14 · Frontend Essentials](./14-frontend-essentials.md) →
