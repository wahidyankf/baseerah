# Delivery Checklist — The Well-Grounded Software Engineer

This checklist is **table-referential**: the canonical topic set, per-topic level, learning format,
primary language, and weights live in [prd.md](./prd.md) — the single source of truth. Every per-topic
step below reads its row from that table. When a topic is added/removed, edit the prd table and add or
drop its per-topic block here; no other doc changes.

## Executor Legend

- **[AI]** — an AI agent performs this step autonomously (authoring, checking, git-mechanical steps,
  committing, and pushing directly to `origin main`).
- **[HUMAN]** — only a human can perform this step (any explicit approval the user chooses to reserve).

Per repo policy, git-mechanical steps (commit, push to `origin main`) are **[AI]** under this delivery
mode. There is no PR and no human-merge gate — work lands directly on `main`.

## Worktree

**Not used.** This plan's delivery mode is `main-to-origin-main`, which works in the **primary
checkout** (no worktree). All work happens on the `main` working tree and is pushed directly to
`origin main`.

See [Plans Organization Convention §Delivery Mode](../../../repo-governance/conventions/structure/plans.md#delivery-mode).

## Delivery Mode: main-to-origin-main

Work in the **primary checkout** on `main` (no worktree, no PR). The AI commits and **pushes directly
to `origin main`** after each phase gate passes. "Done" = the content is on `origin main` with CI
green. Because there is no PR, the finalization phase runs **direct-push + CI post-push verification**
instead of the PR-Review Maker→Fixer Cycle.

**Direct-to-main discipline** (per repo memory/policy): stage **explicit paths only** (the new content
files and the two nav `_index.md` edits) — never `git add -A` in this repo. Do not touch git identity.
Commit per domain/concern with Conventional Commit messages.

## Per-Topic Step Template (applied in every level phase)

For each canonical topic row, `<slug>` = table "Slug", `<Lwt>` = "Learn wt", `<Dwt>` = "Drill wt"
(`= <Lwt> + 100`), `<lang>` = "Primary language", `<fmt>` = "Learning format". Base path
`CONTENT = apps/ayokoding-www/content/en/learn/software-engineering/the-well-grounded-software-engineer`.
Before authoring any topic, **read that topic's section in [syllabus.md](./syllabus.md)** — it lists
the concrete items (subtopics) and named worked examples the topic must cover.

1. **[AI] Author the learning subtree.** Create `CONTENT/learning/<slug>/` with `_index.md`
   (frontmatter `weight: <Lwt>`), `overview.md` (states primary language `<lang>` and prerequisites),
   and example pages covering **every item and worked example in syllabus.md §`<slug>`**. If
   `<fmt>` = By Example → invoke `apps-ayokoding-www-by-example-maker`, subtree shape
   `by-example/{overview,beginner,intermediate,advanced}`, five-part examples in `<lang>`, density
   1.0–2.25. If `<fmt>` = Annotated-concept → invoke `apps-ayokoding-www-general-maker`, annotated
   worked examples in `<lang>` (prose+diagrams where code doesn't fit).
   **Acceptance**: files exist; frontmatter `weight: <Lwt>`; all code in `<lang>` (or the documented
   exception); every syllabus item/example present; covered to mastery depth (DD-8).
2. **[AI] Check the learning subtree.** Invoke the matching checker
   (`apps-ayokoding-www-by-example-checker` for By Example, `apps-ayokoding-www-general-checker` for
   Annotated-concept). **Acceptance**: no unresolved HIGH/CRITICAL findings; density/format floors met.
3. **[AI] Author the drilling page.** Create `CONTENT/drilling/<slug>.md` (frontmatter `weight: <Dwt>`)
   via `apps-ayokoding-www-general-maker`, using the fixed four-section anatomy (Recall Q&A / Applied
   problems / Code katas / Self-check checklist), answers hidden in `<details>`, katas in `<lang>`.
   **Acceptance**: file exists; `weight: <Dwt>` where `<Dwt> = <Lwt> + 100`; four sections present;
   every answer inside a `<details>` block.
4. **[AI] Check the drilling page.** Invoke `apps-ayokoding-www-general-checker` on the page.
   **Acceptance**: no unresolved HIGH/CRITICAL findings; all four sections present and well-formed.
5. **[AI] Fact-spot the topic.** Run `apps-ayokoding-www-facts-checker` on the topic's learning +
   drilling files. **Acceptance**: no unresolved factual findings (verify commands/versions/claims).

---

## Phase 0 — Environment Setup and Baseline

- [ ] **[AI]** Confirm the primary checkout is on `main` and synced: run `git checkout main` then
      `git pull origin main`. **Acceptance**: on branch `main`, up to date with `origin/main`, working tree clean.
- [ ] **[AI]** Initialize toolchain: run `npm install` then `npm run doctor -- --fix` in the primary checkout.
      **Acceptance**: both exit 0; no missing-tool warnings.
- [ ] **[AI]** Baseline the target project: run `npx nx run ayokoding-www:build`. **Acceptance**: exits 0
      (clean baseline before any new content).
- [ ] **[AI]** Baseline markdown lint: run `npm run lint:md` (or `npm run lint:md:fix`). **Acceptance**:
      exits 0 on the existing tree, or only auto-fixable issues that fix cleanly.
- [ ] **[AI]** Scaffold the section root: create
      `apps/ayokoding-www/content/en/learn/software-engineering/the-well-grounded-software-engineer/_index.md`
      (frontmatter `weight: 1750`, title "The Well-Grounded Software Engineer", link list to both tracks +
      journey map) and `overview.md` (weight 1, read-then-drill workflow + the seven-level journey Mermaid map
      from prd.md, accessible WCAG palette). **Acceptance**: both files exist; `nx run ayokoding-www:build` still exits 0.
- [ ] **[AI]** Scaffold the two track landings: create `learning/_index.md` (weight 100) and
      `drilling/_index.md` (weight 200), each an empty-but-valid nav list to be populated per topic.
      **Acceptance**: both files exist with correct weights; build exits 0.

### Phase 0 Gate

`npx nx run ayokoding-www:build` exits 0 with the scaffold in place; `npm run lint:md` passes; the
section root and both track landings render with correct weights.

> **Pause Safety**: The section is additive scaffold only — no topic content yet, nav not wired into the
> parent SE index. Safe to pause here; the site is unaffected because the section is not yet linked.

---

## Phase 1 — L1 Interview Core (learn + drill)

Apply the Per-Topic Step Template to each L1 row of the prd table, in weight order:

- [ ] **[AI]** Topic `data-structures-and-algorithms` — By Example, Python, Learn 101 / Drill 201 (template steps 1–5).
- [ ] **[AI]** Topic `computer-science-foundations` — Annotated-concept, Python\*, Learn 102 / Drill 202 (steps 1–5).
- [ ] **[AI]** Topic `computer-networking` — Annotated-concept, Python\*, Learn 103 / Drill 203 (steps 1–5).
- [ ] **[AI]** Topic `object-oriented-programming` — By Example, Python, Learn 104 / Drill 204 (steps 1–5).
- [ ] **[AI]** Topic `programming-paradigms` — By Example, Python\*\* (survey), Learn 105 / Drill 205 (steps 1–5).
- [ ] **[AI]** Topic `functional-programming` — By Example, Python, Learn 106 / Drill 206 (steps 1–5).
- [ ] **[AI]** Topic `concurrency-and-parallelism` — By Example, Python, Learn 107 / Drill 207 (steps 1–5).

### Phase 1 Gate

All 7 L1 learning subtrees + drill pages exist with correct weights (parity `Dwt = Lwt + 100`); each
clears its format checker + facts-checker; `nx run ayokoding-www:build` exits 0; `npm run lint:md` passes.

> **Pause Safety**: L1 topics are self-contained and not yet nav-wired into the parent SE index. Safe to
> pause; partial section is invisible to readers until Phase 8 wiring.

---

## Phase 2 — L2 Build & Ship (learn + drill)

The "become dangerous fast" phase: software-engineering practice + the be/fe/mobile app domains. The
app domains are independent parallel tracks (cap 3 concurrent per repo policy).

- [ ] **[AI]** Topic `software-engineering-practices` — Annotated-concept, Python\*, Learn 108 / Drill 208 (steps 1–5).
- [ ] **[AI]** Topic `data-storage` — By Example, SQL + Python †, Learn 109 / Drill 209 (steps 1–5).
- [ ] **[AI]** Topic `backend-development` — By Example, Python, Learn 110 / Drill 210 (steps 1–5).
- [ ] **[AI]** Topic `frontend-development` — By Example, TypeScript †, Learn 111 / Drill 211 (steps 1–5).
- [ ] **[AI]** Topic `android-app-development` — By Example, Kotlin †, Learn 112 / Drill 212 (steps 1–5).
- [ ] **[AI]** Topic `ios-app-development` — By Example, Swift †, Learn 113 / Drill 213 (steps 1–5).

### Phase 2 Gate

All 6 L2 topics complete (learn + drill, correct weights/parity); checkers + facts-checker clean;
`nx run ayokoding-www:build` exits 0; `npm run lint:md` passes.

> **Pause Safety**: L2 topics self-contained, not yet nav-wired. Safe to pause.

---

## Phase 3 — L3 Design at Scale (learn + drill)

- [ ] **[AI]** Topic `software-architecture` — Annotated-concept, Python\*, Learn 114 / Drill 214 (steps 1–5).
- [ ] **[AI]** Topic `system-design` — Annotated-concept, Python\*, Learn 115 / Drill 215 (steps 1–5).

### Phase 3 Gate

Both L3 topics complete (learn + drill, correct weights/parity); checkers + facts-checker clean;
build exits 0; markdown lint passes.

> **Pause Safety**: L3 topics self-contained, not yet nav-wired. Safe to pause.

---

## Phase 4 — L4 Broaden Delivery (parallel; learn + drill)

L4 domains are independent parallel tracks — order within the phase does not matter, but each still
follows the template. May be authored in parallel (cap 3 concurrent per repo policy).

- [ ] **[AI]** Topic `windows-app-development` — By Example, C# †, Learn 116 / Drill 216 (steps 1–5).
- [ ] **[AI]** Topic `linux-app-development` — By Example, Python, Learn 117 / Drill 217 (steps 1–5).
- [ ] **[AI]** Topic `cloud-containers-and-iac` — Annotated-concept, YAML/HCL †, Learn 118 / Drill 218 (steps 1–5).
- [ ] **[AI]** Topic `data-engineering` — Annotated-concept, Python, Learn 119 / Drill 219 (steps 1–5).
- [ ] **[AI]** Topic `creating-ai-powered-apps` — By Example, Python, Learn 120 / Drill 220 (steps 1–5).
- [ ] **[AI]** Topic `it-security` — Annotated-concept, Python\*, Learn 121 / Drill 221 (steps 1–5).

### Phase 4 Gate

All 6 L4 topics complete (learn + drill, correct weights/parity); checkers + facts-checker clean;
build exits 0; markdown lint passes.

> **Pause Safety**: L4 topics self-contained, not yet nav-wired. Safe to pause; parallel authoring
> leaves each finished topic independently valid.

---

## Phase 5 — L5 Systems & Language Depth (learn + drill)

The deep-mastery tier (rarely interviewed directly): OS internals, low-level system programming, and
the language-theory topics (Lisp, Hindley–Milner) plus compilers.

- [ ] **[AI]** Topic `linux-os` — By Example, C + shell †, Learn 122 / Drill 222 (steps 1–5).
- [ ] **[AI]** Topic `windows-os` — By Example, C + PowerShell †, Learn 123 / Drill 223 (steps 1–5).
- [ ] **[AI]** Topic `system-programming` — By Example, C †, Learn 124 / Drill 224 (steps 1–5).
- [ ] **[AI]** Topic `lisp` — By Example, Scheme †, Learn 125 / Drill 225 (steps 1–5).
- [ ] **[AI]** Topic `type-systems` — By Example, OCaml/Haskell † (Hindley–Milner), Learn 126 / Drill 226 (steps 1–5).
- [ ] **[AI]** Topic `compilers-parsers-and-transpilers` — By Example, Python, Learn 127 / Drill 227 (steps 1–5).

### Phase 5 Gate

All 6 L5 topics complete (learn + drill, correct weights/parity); checkers + facts-checker clean;
build exits 0; markdown lint passes.

> **Pause Safety**: L5 topics self-contained, not yet nav-wired. Safe to pause.

---

## Phase 6 — L6 Advanced Ops (learn + drill)

- [ ] **[AI]** Topic `site-reliability-engineering` — Annotated-concept, Python\*, Learn 128 / Drill 228 (steps 1–5).

### Phase 6 Gate

The L6 topic complete (learn + drill, correct weights/parity); checkers + facts-checker clean;
build exits 0; markdown lint passes.

> **Pause Safety**: L6 topics self-contained, not yet nav-wired. Safe to pause.

---

## Phase 7 — L7 Leadership & Product (learn + drill)

Leadership/governance topics (`‡`) are minimal-to-no code — the "Code katas" drill section becomes a
short design/decision exercise.

- [ ] **[AI]** Topic `it-governance-grc` — Annotated-concept, — ‡, Learn 129 / Drill 229 (steps 1–5).
- [ ] **[AI]** Topic `project-management` — Annotated-concept, — ‡, Learn 130 / Drill 230 (steps 1–5).
- [ ] **[AI]** Topic `software-product-engineering` — Annotated-concept, — ‡, Learn 131 / Drill 231 (steps 1–5).
- [ ] **[AI]** Topic `engineering-management` — Annotated-concept, — ‡, Learn 132 / Drill 232 (steps 1–5).

### Phase 7 Gate

All 4 L7 topics complete (learn + drill, correct weights/parity); checkers + facts-checker clean;
build exits 0; markdown lint passes. **All 32 topics now authored in both tracks.**

> **Pause Safety**: L7 topics self-contained, not yet nav-wired. Safe to pause; the whole section is
> content-complete but still invisible to readers (nav wiring is Phase 8).

---

## Phase 8 — Nav wiring, quality gates, manual verification

- [ ] **[AI]** Wire the section into the SE nav: edit
      `apps/ayokoding-www/content/en/learn/software-engineering/_index.md`, adding a
      "The Well-Grounded Software Engineer" link (route
      `/en/c/learn/software-engineering/the-well-grounded-software-engineer`) in weight order.
      **Acceptance**: link present; `nx run ayokoding-www:build` exits 0.
- [ ] **[AI]** Wire the sub-entry into the learn index: edit `apps/ayokoding-www/content/en/learn/_index.md`,
      adding the section as a sub-entry under Software Engineering. **Acceptance**: entry present; build exits 0.
- [ ] **[AI]** Parity check: verify every topic has both `learning/<slug>/` and `drilling/<slug>.md`, and
      `Drill wt = Learn wt + 100` for all rows. **Acceptance**: counts match the prd table (32/32); no
      orphaned or mismatched weights.
- [ ] **[AI]** Link check: run `apps-ayokoding-www-link-checker` across the new section. **Acceptance**:
      no broken internal/external links.
- [ ] **[AI]** Full lint + build: `npm run lint:md` and `npx nx run ayokoding-www:build`. **Acceptance**:
      both exit 0.
- [ ] **[AI]** Playwright smoke (per repo manual-behavioral-verification): start the app, render the section
      landing + one learning page + one drilling page; confirm a `<details>` block expands; confirm a nav
      link resolves; check console. **Acceptance**: three pages render; `<details>` toggles; nav resolves;
      zero console errors.

### Phase 8 Gate

Section is nav-reachable in ≤2 clicks from `learn/software-engineering/`; parity 32/32; link-checker,
markdown lint, and build all green; Playwright smoke passes with zero console errors.

> **Pause Safety**: Section is now live in nav but purely additive — no existing content changed. Safe to
> pause; if paused mid-verify, the nav links already resolve to valid pages.

---

## Phase 9 — Direct push to origin main + CI post-push verification (main-to-origin-main)

- [ ] **[AI]** Stage **explicit paths only** (the new section under
      `apps/ayokoding-www/content/en/learn/software-engineering/the-well-grounded-software-engineer/`
      plus the two nav `_index.md` edits) — never `git add -A`. Commit per domain/concern with Conventional
      Commit messages (e.g. `docs(ayokoding-www): add well-grounded-software-engineer section`).
      **Acceptance**: `git status` shows only intended paths staged; commit(s) created.
- [ ] **[AI]** Push directly to `origin main`: `git push origin main`. **Acceptance**: push succeeds; local
      `main` and `origin/main` at the same commit.
- [ ] **[AI]** CI post-push verification: trigger/observe CI and poll every 2 min per ci-monitoring policy
      (`gh run view --json status,conclusion`; never `gh run watch`; on HTTP 403 wait ~35 min).
      **Acceptance**: latest run `conclusion = success`.

### Phase 9 Gate

Content is on `origin main` with CI green on the pushed commit. "Done" = green on `origin main` (no PR,
no human-merge gate under `main-to-origin-main`).

> **Pause Safety**: Changes are additive content only; if paused after a partial push, `main` still builds
> because every pushed commit passed its phase gate before the push.

---

## Phase 10 — Knowledge Capture

- [ ] **[AI]** Triage [learnings.md](./learnings.md): route each captured learning to a durable home
      (convention, agent, skill, or docs) or explicitly discard it. **Acceptance**: every learnings.md entry
      is either linked to its new home or marked discarded with a reason. [Repo-grounded — Knowledge Capture]

### Phase 10 Gate

`learnings.md` fully triaged — no untriaged entries remain.

> **Pause Safety**: Documentation-only step; safe to pause at any point.

---

## Plan Archival

- [ ] **[AI]** After the section is on `origin main` with CI green, move the plan folder to `done/` with
      today's completion date:
      `git mv plans/in-progress/the-well-grounded-software-engineer plans/done/YYYY-MM-DD__the-well-grounded-software-engineer`
      and update `plans/in-progress/README.md` (remove the active entry). **Acceptance**: plan under `done/`
      with date prefix; in-progress index no longer lists it.
- [ ] **[AI]** Commit the archival move (explicit paths) and push directly to `origin main`.
      **Acceptance**: `git mv` committed; `git push origin main` succeeds; `origin/main` reflects the move.

No worktree is used under `main-to-origin-main`, so there is no worktree-removal step.
