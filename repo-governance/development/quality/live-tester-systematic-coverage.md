---
title: "Live-Tester Systematic Coverage"
description: The SSOT practice that mandates enumerate-not-sample forcing-functions for the three live-site testers and the web-ux-test-fixing-planning workflow
category: explanation
subcategory: development
tags:
  - testing
  - live-testing
  - usability
  - ux
  - quality
  - systematic
created: 2026-06-22
---

# Live-Tester Systematic Coverage

Dimension-based, charter-driven, and tour-based testing finds representative defects. It reliably
misses an entire defect class: "enumerate every element and assert one property about it." A tester
that samples two tabs out of four will find defects on the two tabs it visited. A tester that
enumerates all four tabs -- and asserts a specific invariant for each -- cannot miss defects on the
other two.

This document defines the forcing-functions that convert sampling into enumeration, making that
defect class impossible to skip. It is the single source of truth (SSOT) that the three live-site
tester agents (`web-exploratory-tester`, `web-usability-tester`, `web-design-tester`) and the
`web-ux-test-fixing-planning` workflow implement operationally.

## Principles Implemented/Respected

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: Sampling
  is implicit; enumeration is deliberate. Forcing-functions require the tester to observe every
  element on every surface rather than stopping when a representative sample has been checked.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Each forcing-function states exactly what must be enumerated and what property must be asserted
  for each item. "I tested the controls" is implicit. "I exercised each of the N shared controls
  on each of the M tabs it appears on and asserted consistent behavior" is explicit.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**:
  Where a systematic check can be automated -- computed-style tuples for visual consistency,
  URL round-trip scripts, declared-invariant scripts -- automation is preferred. Where human
  or agent judgment is required -- jargon scanning, discoverability probing, completeness
  criticism -- the forcing-function names the exact judgment to apply.

## Conventions Implemented/Respected

- **[User-Facing Delivery Hardening Convention](./user-facing-delivery-hardening.md)**: Rule 15
  of that convention requires the three live-site testers to run a near-end retest before plan
  archival. This document defines what "thorough" means for those runs so the retest is
  systematic, not selective.

- **[Manual Behavioral Verification Convention](./manual-behavioral-verification.md)**: That
  convention defines _what_ to verify (page renders, interactions, console errors, network
  requests, all locales, all breakpoints). This document defines _how_ to achieve completeness
  across all elements on all surfaces -- the enumeration discipline that complements the
  per-locale, per-breakpoint discipline.

- **[Evidence Capture Convention](./evidence-capture.md)**: Each forcing-function that produces
  findings must be recorded as evidence: a defect list, a matrix of results, or a completeness
  assertion captured in the plan's findings or delivery notes.

## The Problem: Sampling Misses Whole Defect Classes

The cost-of-living calculator work is the concrete case that motivated this document. The three
testers ran approximately six rounds across that feature and captured more than 40 findings. Yet
after all six rounds, a human found defects the testers had never flagged:

- A shared control (the city/country filter) had no effect on one tab's output while working
  correctly on another. No tester had exercised that control on every tab.
- Inputs were not persisted in the URL, so reload discarded the user's selections. No tester
  had verified the URL-state round-trip for every interactive input.
- An invariant stated in a source-code comment ("URL is the single source of truth") was
  violated by several inputs. No tester had extracted that invariant and checked each input
  against it.
- Multiple controls were styled as raw native elements on some surfaces and as styled components
  on others. No tester had built the full cross-surface consistency matrix.
- Several labels used domain jargon that first-time users could not interpret. No tester had
  scanned every visible label systematically.

None of these gaps resulted from weak testers. They resulted from sampling: each tester exercised
a representative subset of surfaces and controls, which is insufficient for finding cross-surface
inconsistencies and invariant violations. Enumeration closes the gap.

## The Six Forcing-Functions

### 1. Shared-Control x Surface Matrix

**Obligation**: Identify every control that appears on more than one tab, view, or surface.
For each such control, exercise it on every surface it appears on and assert that the behavior
is identical across surfaces. A control that triggers a change on one surface but no-ops on
another is a consistency defect (Nielsen Heuristic 4: Consistency and Standards; WCAG 2.2 SC
3.2.4 Consistent Identification, technique G197).

**How to apply**: Build a matrix before testing begins:

| Control        | Tab A | Tab B | Tab C | Consistent? |
| -------------- | ----- | ----- | ----- | ----------- |
| City filter    | check | check | check | YES / NO    |
| Currency input | check | check | --    | YES / NO    |

Fill in the matrix by exercising each cell. A blank or "NO" cell is a finding.

**Ground**: Nielsen Heuristic 4 (Consistency and Standards); WCAG 2.2 SC 3.2.4 Consistent
Identification (technique G197).

### 2. Per-Control URL / State Round-Trip

**Obligation**: For every interactive control (filter, input, toggle, selector, tab),
execute the full round-trip: change the control value, verify the URL updates, reload the
page (or open a new tab with the same URL), and verify the control restores to its changed
state. A control whose value does not survive a reload is a statelessness defect
(MDN History API state contract; Nielsen Heuristics 1: Visibility of System Status and
3: User Control and Freedom).

**How to apply**: For each control:

1. Record the URL before the change.
2. Change the control.
3. Assert the URL has updated to reflect the new value.
4. Reload.
5. Assert the control shows the value it held before reload.

If any step fails, record a finding with the control name, the expected URL parameter, and
the observed behavior.

**Ground**: MDN History API state contract; Nielsen Heuristics 1 and 3.

### 3. Declared-Invariant Conformance

**Obligation**: Before testing, extract all declared invariants from the available sources:
Gherkin scenarios in `specs/**`, source-code comments (e.g., "URL is the single source of
truth"), `AGENTS.md`/`CLAUDE.md` product-requirement statements, and `prd.md` acceptance
criteria in the plan. For each invariant, verify that it holds for EVERY applicable element,
not just a sample. An invariant that holds for three of four inputs is a violated invariant.

**How to apply**:

1. List every declared invariant before testing.
2. For each invariant, enumerate every element it applies to.
3. Check each element against the invariant.
4. A single failure is a HIGH-severity finding; the invariant is violated even if all other
   instances pass.

**Ground**: Derived from the feature's own specification -- a violation is a spec-conformance
defect by definition.

### 4. Raw / Unstyled Native-Element Audit and Cross-Surface Styling Consistency Matrix

**Obligation**: Enumerate every interactive element (buttons, inputs, selects, checkboxes,
toggles, links) on every surface. For each element, record the tuple:
`(computed background-color, computed color, computed border, computed border-radius, computed
font-size)`. Assert two properties:

1. No element is rendered as a raw, unstyled native browser control while its sibling on another
   surface receives full design-system styling (raw native elements fail visual regression
   baselining and signal incomplete component migration).
2. Elements of the same semantic type (e.g., all primary action buttons) share the same computed-
   style tuple across surfaces (Nielsen Heuristic 4 internal consistency; visual-regression
   baselining).

**Ground**: Nielsen Heuristic 4 internal consistency; visual-regression baselining (Chromatic) /
computed-style tuples.

### 5. Usability Probes

Apply all five probes on every run. A probe that applies to only some surfaces must still be
applied on all surfaces where it could apply.

**5a. Conditional / Hidden-Control Discoverability**: Identify every control that is hidden,
collapsed, or conditionally rendered. For each, verify that a first-time user can discover its
existence and purpose without prior knowledge. A control whose existence is not surfaced through
any visible affordance is a discoverability defect (Nielsen Heuristic 6: Recognition rather than
recall; NN/g Progressive Disclosure).

**5b. Per-Label Jargon Scan**: Read every visible label, heading, tooltip, placeholder, and
button text. Flag any term that a first-time user -- unfamiliar with the domain -- could not
interpret from context alone (Nielsen Heuristic 2: Match between system and real world).

**5c. Cross-View Redundancy**: Identify any element that appears on multiple views and conveys
identical information. Flag it as a redundancy defect (Nielsen Heuristic 8: Aesthetic and
minimalist design; Hick's Law: excess choices increase decision time).

**5d. Input Unit and Currency Consistency**: For every input that accepts a numeric value with
a unit (currency, percentage, distance, weight), verify that:

- The unit is stated adjacent to the input or its label (not hidden or absent).
- If a currency symbol is shown, it reflects the user's selected currency, not a hardcoded
  default (Nielsen Heuristic 5: Error prevention; WCAG 2.2 SC 3.3.2 Labels or Instructions;
  Nielsen Heuristic 4 cross-surface consistency).

**Ground**: Nielsen Heuristics 2, 4, 5, 6, 8; NN/g Progressive Disclosure; Hick's Law;
WCAG 2.2 SC 3.3.2 Labels or Instructions.

### 6. Recurrence + Diff Memory + Completeness Critic

**Obligation**: Every test run must carry forward knowledge from prior runs and challenge itself
to achieve completeness.

**6a. Recurrence check**: At the start of each run, list the defect _classes_ found in prior
runs (not just individual findings). For each class, re-verify the same class of element
on every surface. A defect class that was fixed in one location but not in analogous locations
is a partial fix.

**6b. Diff memory**: Note what changed since the last test run (from the plan's delivery
checklist or git log). Re-verify all surfaces adjacent to or dependent on the changed
components -- not only the changed components themselves.

**6c. Completeness critic**: End every run with an explicit self-audit: "What did we NOT
enumerate?" List the surface categories (tabs, breakpoints, locales, control types) and
verify each was covered. A category not enumerated is an open gap, not an implicit pass.

**Ground**: Regression-test mandate (prior defect classes must be rechecked); differential
testing principle (changes create adjacency risk); Deliberate Problem-Solving principle.

## Motivating Example

The cost-of-living calculator work illustrates each forcing-function in the negative:

- The shared-control x surface matrix was not built, so the city filter's no-op on the savings
  tab was never observed (FF1).
- The URL round-trip was not verified for every input, so the reload-discards-state defect was
  not found (FF2).
- The "URL is the single source of truth" source comment was never extracted as an invariant to
  check, so its violations went undetected (FF3).
- The cross-surface styling matrix was not built, so raw native elements coexisting with
  design-system components were not flagged (FF4).
- The jargon scan and hidden-control probe were run selectively rather than exhaustively (FF5).
- Prior defect classes were not carried forward as mandatory re-checks in later rounds (FF6).

Applying all six forcing-functions on the first run would have surfaced all of these findings.

## Relationship to the Three Live-Site Testers

The three tester agents each carry their own operational playbook. This document defines the
shared underlying discipline:

| Agent                    | Primary lens                                       | Applies forcing-functions |
| ------------------------ | -------------------------------------------------- | ------------------------- |
| `web-exploratory-tester` | Spec-aware correctness (EWT findings)              | FF1, FF2, FF3, FF6        |
| `web-usability-tester`   | Spec-blind first-time-user friction (UWT findings) | FF5, FF6                  |
| `web-design-tester`      | Design-aware visual fidelity (DWT findings)        | FF4, FF6                  |
| `api-exploratory-tester` | Spec/contract-aware API correctness (AET findings) | FF1, FF2, FF3, FF6        |

FF6 (recurrence + diff memory + completeness critic) applies to all of them. The `web-ux-test-
fixing-planning` workflow coordinates the three **web** testers sequentially against the same target
URL and integrates their findings into a unified fix-ready plan. `api-exploratory-tester` is the
**API-surface** counterpart — HTTP/curl-driven, never a browser — and applies the same enumerate-don't-
sample forcing functions to API operations (its three mandatory sweeps are the operation × property
matrix, the cross-cutting convention round-trip, and the declared-invariant conformance pass). It runs
as a single specialist (no triad, no dedicated workflow) because the API surface has one exploratory
lens.

## Scope

This practice applies to:

- All runs of `web-exploratory-tester`, `web-usability-tester`, and `web-design-tester` against
  any live web surface in `apps/`.
- All runs of `api-exploratory-tester` against any live REST or GraphQL API in `apps/`.
- All invocations of the `web-ux-test-fixing-planning` workflow.
- The Rule-15 near-end retest required by the
  [User-Facing Delivery Hardening Convention](./user-facing-delivery-hardening.md) before plan
  archival.

It does not apply to:

- Automated Playwright E2E tests (those follow the [Three-Level Testing Standard](./three-level-testing-standard.md)).
- API-only verification (covered by [Manual Behavioral Verification](./manual-behavioral-verification.md)).
- Library-only changes with no UI surface.

## Related Documentation

- [User-Facing Delivery Hardening Convention](./user-facing-delivery-hardening.md) -- Rule 15
  (near-end three-tester retest before archival) that this practice makes thorough
- [Manual Behavioral Verification Convention](./manual-behavioral-verification.md) -- Per-locale,
  per-breakpoint discipline that this practice extends with element-level enumeration
- [Evidence Capture Convention](./evidence-capture.md) -- Where and how to record findings and
  matrices from each forcing-function
- [Regression Test Mandate](./regression-test-mandate.md) -- Every defect found by these testers
  must land with a reproducing test when fixed
- [Three-Level Testing Standard](./three-level-testing-standard.md) -- Automated testing
  architecture that systematic live testing complements (not replaces)
- [web-ux-test-fixing-planning workflow](../../workflows/web/web-ux-test-fixing-planning.md) --
  The orchestration workflow that sequences all three testers against the same target
