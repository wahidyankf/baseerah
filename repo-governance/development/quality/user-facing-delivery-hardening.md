---
title: "User-Facing Delivery Hardening Convention"
description: Fifteen durable rules for planning, executing, verifying, and archiving user-facing feature work so design-parity and behavioral defects cannot ship past green gates
category: explanation
subcategory: development
tags:
  - quality
  - planning
  - ui
  - verification
  - testing
  - deployment
created: 2026-06-19
---

# User-Facing Delivery Hardening Convention

A user-facing feature can be fully implemented, validated to **zero findings**, pass
typecheck + lint + unit + E2E + CI, be **archived to `plans/done/`** — and still ship to
production **bland, off-design, and carrying calculation bugs**. That is not a hypothetical: it
happened, and every automated gate was green while it happened. The defects surfaced only when a
human opened the live site.

This convention distills the fourteen lessons from that incident into durable rules for the whole
**plan → execute → verify → archive** loop, plus a fifteenth rule added afterward to require a
near-end three-tester retest of the live web UI (the `web-ux-test-fixing-planning` workflow —
exploratory correctness, usability, and design fidelity) before archival. Each rule names the gap it closes
and how to apply it, so it can be folded into how we author plans (`plan-maker`), gate them
(`plan-checker`, `plan-execution-checker`), and execute them (the plan-execution workflow).

## Principles Implemented/Respected

- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: The root cause
  of the incident was not weak tests — it was the absence of a human (or Playwright) observing the
  rendered result against the design before declaring the work done. These rules target that root
  cause, not its symptoms.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Every rule converts an implicit assumption ("tests pass, so it must look and work right") into an
  explicit, checkable delivery step.
- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: Visual
  and value-bearing verification forces deliberate observation of actual behavior instead of trust
  in green checkmarks.

## Conventions Implemented/Respected

- **[Manual Behavioral Verification](./manual-behavioral-verification.md)**: This convention
  extends it from "verify before done" to "verify against the design mockups, per breakpoint, per
  locale, before **archival**."
- **[Evidence Capture Convention](./evidence-capture.md)**: The per-breakpoint, per-locale sign-off
  required by Rules 1 and 10 MUST leave a committed evidence trail — screenshots in the plan's
  `evidence/` subfolder, screenshot paths referenced from `delivery.md` implementation notes. A
  sign-off claimed without committed evidence is not a sign-off.
- **[Feature Change Completeness](./feature-change-completeness.md)**: Completeness now includes
  per-breakpoint responsive deliverables and labelled outputs, not just specs+Gherkin parity.
- **[Test-Driven Development](../workflow/test-driven-development.md)**: Sharpened by Rule 12
  (assertions must distinguish correct from buggy) and Rule 5 (value-bearing, not presence-only).
- **[UI Mockups in Plan Docs](../../conventions/formatting/diagrams.md)**: Sharpened by Rules 2
  and 8 (name the design-system primitive; annotate mockup colors as theme tokens).

## Scope

### What This Convention Covers

- Authoring plans for any user-facing change (web UI, rendered output, public-facing CLI text).
- Executing, verifying, and archiving those plans.
- The done/archival criterion for user-facing work.

### What This Convention Does NOT Cover

- Pure library/internal refactors with no observable output (see
  [Manual Behavioral Verification](./manual-behavioral-verification.md) scope).
- Documentation/governance-only changes.
- API-only behavior (covered by the curl path in
  [Manual Behavioral Verification](./manual-behavioral-verification.md)).

## The Fifteen Rules

Rules 1–14 are listed in source order (each maps to the same-numbered lesson from the originating
incident); rule 15 was added afterward and does not correspond to an incident lesson. A **phase
tag** marks where each rule binds, and each states the **gap** it closes and **how to apply** it.

1. **(Authoring) A UI plan MUST carry a manual visual-parity gate, executed before archival.** Gap:
   automated tests asserted DOM/behavior presence; none compared the rendered pixels to the
   approved `assets/` mockups, and the workflow's Playwright visual step was skipped. Apply: any
   plan that ships UI carries an explicit, checked "screenshot vs each mockup, per breakpoint, per
   locale" step; `plan-checker` flags its absence the way it flags a missing design funnel. The
   sign-off step MUST save screenshots to `evidence/` and reference them in `delivery.md`; a
   step without committed evidence is not signed off. See [Evidence Capture Convention](./evidence-capture.md).

2. **(Authoring) Name the design-system primitive in the delivery step — never assume it.** Gap:
   the shared `Tabs`/`Badge`/`Toggle` primitives existed and were exported, yet the build
   hand-rolled bare `<button role="tab">` / `<span>` / `<select>`. Apply: when a mockup shows a
   known primitive (tabs, badge, segmented control, card), the step names the primitive and asserts
   its presence.

3. **(Authoring) Responsive parity is a first-class, per-breakpoint deliverable.** Gap: `*-mobile`
   and `*-tablet` mockups existed in `assets/` but no delivery step bound them; the build shipped
   one wide desktop table. Apply: each responsive mockup gets its own RED/GREEN step plus a
   viewport-specific visual assertion (see Rule 9 for the technique).

4. **(Authoring) Filter/scope coverage MUST be exhaustive over the cascade.** Gap: the city-only
   filter path (city set, country/region null) had no test, so a "filter silently ignored" bug
   shipped. Apply: for any cascading filter, the plan's Gherkin enumerates **each** level
   independently (region-only, country-only, city-only) and the meaningful combinations — not just
   the happy cascade.

5. **(Authoring) Ordering/threshold features need value-bearing tests, not presence tests.** Gap: a
   "a divider exists + some rows are dimmed" assertion held true under both correct and **inverted**
   logic. Apply: assert concrete positions/identities ("Staff SWE is above the minimum, SWE I
   below") and choose fixture inputs that actually produce the split — probe the data when
   authoring. (See Rule 12 for the execution-side sharpening.)

6. **(Authoring) Every displayed number needs a visible label.** Gap: a preview rendered eight bare
   currency chips with no legend. Apply: a plan presenting computed figures requires a label/legend
   for each value in its acceptance criteria.

7. **(Authoring) Green automated gates are necessary, not sufficient, for UI/UX correctness.** Gap:
   four real defects plus a label-clarity issue shipped with unit/E2E/lint/typecheck/CI all green.
   Apply: the maker-checker-fixer loop for UI work needs a human-or-Playwright visual sign-off rung
   the automated gates cannot substitute for.

8. **(Authoring) Mockup colors MUST be specified as theme tokens, then reconciled to the app's
   brand.** Gap: the mockups used a generic palette; the first implementation copied raw colors
   (teal) that were off-brand for the target app and mis-mapped a semantic badge. Apply: plan-doc
   mockups annotate each color with the **theme token** it represents (`active = --color-primary`,
   `covered = hue=sage`), not a raw swatch; the delivery step reconciles to the specific app's brand
   tokens; `plan-checker` flags raw-value colors with no token mapping.

9. **(Execution) Responsive is per-breakpoint work, not a CSS afterthought.** Technique: the
   **dual-render pattern** — one computed dataset, two DOM views (table + cards) toggled by Tailwind
   `md:`/`lg:`; tablet hides granular columns via `hidden lg:table-cell`; mobile renders stacked
   cards. Keep the canonical test-ids on a single view so assertions stay unambiguous. Verify at
   each viewport with Playwright.

10. **(Verification) "Zero findings + CI green" is NOT "done" — and definitely not "archive" — for
    a user-facing change.** Gap: the plan was validated to zero findings and archived to
    `plans/done/` while the UI was bland and off-design. Apply: the done/archival criterion for any
    user-facing change includes a **production visual sign-off against the mockups, per breakpoint,
    per locale**; plan-execution finalization blocks archival until that sign-off is recorded. The
    sign-off MUST cover ALL supported locales (not just the default locale) and MUST be evidenced by
    committed screenshots in `evidence/` with paths referenced in `delivery.md`. Discovering after
    archival that only one locale was tested is a Rule 14 reopen event.

11. **(Verification) Deploy configuration is code — validate it in the plan.** Gap: a production
    deploy failed because `vercel.json`'s `buildCommand` still pointed at a moved file path; nothing
    tested it, so a green local build produced a broken Vercel build. Apply: any plan that
    moves/renames files includes a deploy-config sweep (`vercel.json`, Dockerfiles, CI
    `buildCommand`s) and a real post-deploy smoke test of the live URL — not just local gates.

12. **(Execution) Prefer assertions that distinguish correct from buggy; pick fixtures that exercise
    the branch.** (Sharpens Rule 5.) A presence-only assertion passes under inverted logic; a
    fixture that trivially satisfies the threshold never exercises the split. Author the test to
    fail when the logic inverts, and probe the data to choose an input that genuinely splits the
    set.

13. **(Process) Keep delivery checkboxes in lockstep with execution (Atomic Sync Ritual).** Gap:
    items were implemented but recorded in a separate as-built log instead of ticking the matching
    boxes, so a phase _looked_ unfinished and needed a later reconciliation pass. Apply: tick the
    box the moment the item lands; if you must record as-built, reconcile the boxes in the **same**
    commit — never leave them divergent.

14. **(Process) A feature reopened after archival needs a clean re-entry, not silent edits on
    `main`.** Gap: a post-archival fix round ran directly on `main` (the worktree was already
    removed) under a tight feedback loop. Apply: reopen the plan first (move it back to
    `plans/in-progress/`, re-provision the worktree) so the work has a home and the trunk stays
    clean; plan-execution documents this "reopen" entry path.

15. **(Verification) A web-UI feature-change plan MUST run a near-end round of all three live-site
    testers — `web-exploratory-tester` (correctness), `web-usability-tester` (usability), and
    `web-design-tester` (design fidelity), i.e. the
    [`web-ux-test-fixing-planning`](../../workflows/web/web-ux-test-fixing-planning.md) workflow —
    against the running UI to iron out rough edges and inconsistencies, and fix their findings before
    archival.** Gap: the visual-parity sign-off (Rule 10) confirms the screen matches the mockups but
    does not hunt for functional, behavioural-consistency, responsive, accessibility, URL/IA, or
    passive-security defects (exploratory), first-time-user confusion (usability), or runtime
    design-token / design-system / spacing drift (design) on the live build — exactly the classes of
    defect that ship past green gates. Apply: after the web UI is implemented and the Rule 10 visual
    sign-off is recorded, run the three testers against the plan's running target URL(s) **across all
    supported locales** (e.g., `/en/` and `/id/` paths for a bilingual app — a single-locale retest is
    incomplete). **Record each resulting finding in `delivery.md` as a new unchecked task-list
    checkbox**, source-attributed (`- [ ] EWT-NNN:` / `- [ ] UWT-NNN:` / `- [ ] DWT-NNN: <defect> —
fix before archival`), in a labelled "Rule-15 three-tester retest follow-ups" section, and each
    SG-### spec-gap / USS-### spec-suggestion as its own unchecked checkbox folded into the `specs/**`
    coverage steps per [Feature Change Completeness](./feature-change-completeness.md). During
    plan-execution these checkboxes materialize 1:1 as harness Task items, are fixed within the same
    plan, and are ticked (`- [x]`) via the Atomic Sync Ritual; a finding may stay unchecked only if
    explicitly deferred with written rationale recorded under the checkbox. Archival is blocked until
    every rule-15 checkbox is ticked or deferred. `plan-maker` emits this step (with the follow-ups
    section scaffold and a locale-coverage note); `plan-checker` flags its absence or
    single-locale-only scope on web-UI feature-change plans; `plan-execution-checker` verifies the
    three-tester round ran across all locales and every rule-15 checkbox is
    resolved-or-explicitly-deferred before archival. Applies to web-UI **feature-change** plans
    (browser-rendered apps) only — not CLI/text user-facing output (which the testers cannot exercise)
    and not pure governance/agent-definition or no-behaviour-change plans.

## Examples

### PASS: A user-facing plan that cannot ship bland

```
- Delivery steps name the web-ui primitive per mockup element (Rule 2)
- Each of mobile/tablet/desktop mockups has its own RED/GREEN step (Rules 3, 9)
- Mockup colors annotated as theme tokens; reconciliation step present (Rule 8)
- Cascading-filter Gherkin enumerates region/country/city independently (Rule 4)
- Ordering test asserts which rows land above/below the divider (Rules 5, 12)
- Finalization blocks archival on production Playwright sign-off per breakpoint/locale (Rules 1, 10)
- Screenshots committed to evidence/ and referenced in delivery.md (Rules 1, 10; Evidence Capture Convention)
- Deploy-config sweep + live-URL smoke test included (Rule 11)
- A near-end three-tester round (web-exploratory + web-usability + web-design) runs across ALL locales; its EWT/UWT/DWT findings are fixed before archival (Rule 15)
```

### FAIL: The incident this convention prevents

```
- Tests assert "a tablist exists" and "a divider exists" — pass under bare markup and inverted logic
- One wide table ships; mobile/tablet mockups never bound
- Raw teal copied from the mockup; off-brand and semantically wrong
- Zero findings → archived to done/ → bland, buggy UI live in production
```

## Tools and Automation

- **Playwright MCP**: per-breakpoint, per-locale visual sign-off against `assets/` mockups.
  Screenshots saved to `evidence/` and referenced in `delivery.md` per the
  [Evidence Capture Convention](./evidence-capture.md).
- **`web-exploratory-tester` / `web-usability-tester` / `web-design-tester`** (the
  [`web-ux-test-fixing-planning`](../../workflows/web/web-ux-test-fixing-planning.md) triad): the
  near-end three-tester round against the running web UI (Rule 15); runs across ALL supported locales;
  surfaces EWT-### (correctness) / UWT-### (usability) / DWT-### (design-fidelity) findings plus SG-###
  spec-gap / USS-### spec-suggestion proposals; saves screenshots to the plan's `evidence/` folder.
- **`plan-maker`**: emits the delivery steps for Rules 1–8 and the rule-15 three-tester-retest step
  for web-UI feature-change plans; includes a locale-coverage note and evidence-capture steps.
- **`plan-checker`**: flags missing visual-parity gate, raw-value mockup colors, presence-only
  ordering tests, missing per-breakpoint responsive steps, missing evidence-capture steps, missing
  locale coverage, and a missing rule-15 three-tester-retest step on web-UI feature-change plans.
- **`plan-execution-checker`**: verifies the production visual sign-off and deploy-config smoke
  test were recorded before archival; verifies evidence/ screenshots exist and are referenced in
  delivery.md; verifies the rule-15 three-tester retest round ran across all locales with its
  EWT/UWT/DWT findings resolved-or-explicitly-deferred before archival.

## References

**Related Conventions:**

- [Manual Behavioral Verification](./manual-behavioral-verification.md) — the visual/behavioral verification baseline this hardens.
- [Evidence Capture Convention](./evidence-capture.md) — where and how to store committed verification evidence: screenshots in `evidence/`, curl outputs inline in `delivery.md`, locale and breakpoint coverage requirements.
- [Feature Change Completeness](./feature-change-completeness.md) — completeness for app/lib changes.
- [Test-Driven Development](../workflow/test-driven-development.md) — RED/GREEN/REFACTOR shape and value-bearing tests.
- [UI Mockups in Plan Docs](../../conventions/formatting/diagrams.md) — both-tiers mockups, design funnel, theme-token colors.
- [Plans Organization Convention](../../conventions/structure/plans.md) — plan folder, phases, Atomic Sync Ritual, reopen path.
- [CI Post-Push Verification](../workflow/ci-post-push-verification.md) — post-push CI + live-URL checks.

**Workflows:**

- [Plan Execution](../../workflows/plan/plan-execution.md) — execution, finalization, archival gate.
- [Plan Quality Gate](../../workflows/plan/plan-quality-gate.md) — pre-execution plan validation.
- [Web UX Test-Fixing Planning](../../workflows/web/web-ux-test-fixing-planning.md) — workflow that runs the three-tester near-end retest (Rule 15).

**Agents:**

- `plan-maker`, `plan-checker`, `plan-execution-checker`, `swe-ui-maker`, `swe-ui-checker`,
  `web-exploratory-tester`, `web-usability-tester`, `web-design-tester`.
