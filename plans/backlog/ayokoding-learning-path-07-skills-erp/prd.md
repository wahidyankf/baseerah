# Product Requirements Document — Skills Paths: Enterprise Resource Planning

> **Programme decisions** — the `R*` rules and `A*` amendments cited below are defined in
> [tech-docs.md §Programme decisions](./tech-docs.md#programme-decisions) (folded in from the retired
> shared programme file and now owned locally).

## Product Overview

Two products: `skills/conventional-erp` (27 courses) and `skills/sharia-erp` (30 courses — the same
27 plus 3 Sharia-exclusive). Both promise the reader the ability to **read, reason about, and design**
an ERP system to build-founding depth — never to operate, install, evaluate, or select one (A6/A7).

## Personas

### Persona 1 — the systems-adjacent engineer (both paths)

A software engineer who will build against, extend, or integrate with an ERP but has never worked
inside one. Wants the architecture (document lifecycle, posting rules, subledger-to-GL) deep enough to
design a correct integration, without needing operational fluency with any specific vendor's UI.
Enters at course 1, values the Dangerous 1 boundary (course 9) as the point where they can start
reviewing a real system's design critically.

### Persona 2 — the finance/ops professional moving into systems (both paths)

Someone with accounting or operations background (may have completed
`ayokoding-learning-path-06-skills-accounting`'s conventional-accounting path already) who now wants
to understand how the systems that implement those processes are actually architected. Values the
explicit accounting-prerequisite edges (this plan links, never duplicates, the accounting corpus) and
the honest framing that this corpus teaches architecture, not accounting itself.

### Persona 3 — the Sharia-compliance-focused reader (`sharia-erp` only)

Wants ERP domain literacy specifically for Sharia-compliant deployments, and needs to know upfront
that `sharia-erp` is a complete path, not an add-on requiring `conventional-erp` first — this is
stated explicitly on the `<SHARLANDING>` landing (L-5) and reinforced by `courseOrder` actually
including all 27 shared ids.

## Product Scope

**In scope**: 30 course bodies, 2 manifests, 2 landings (content spec only), 31 syllabus files, the
licensing section, and Gherkin coverage.

**Out of scope**: any UI component (owned by plan 03), any accounting content (owned by plan 06), any
build/install/evaluate/select content (A6/A7).

## User Stories

- As the systems-adjacent engineer, I want to read `erp-subledger-to-gl-architecture` and immediately
  understand why a reconciliation break is invisible to a trial balance, so that I can design an
  integration that never bypasses a control account.
- As the finance/ops professional, I want `record-to-report-systems` to explicitly state its hard
  dependency on `financial-statements-and-close-cycle`, so that I know exactly which accounting
  competence I need before starting Stage B.
- As the Sharia-compliance-focused reader, I want the `sharia-erp` landing to tell me upfront that I
  do not need `conventional-erp` first, so that I don't waste time hunting for a "start here" course
  that isn't gatekept behind a different path.
- As any reader on either path, I want the Dangerous-N boundaries to tell me honestly what I can and
  cannot yet reason about, so that I don't overestimate my own competence mid-path.

## Gherkin Scenarios

```gherkin
Feature: Skills ERP paths — landing, manifest, and ramp behavior

  Scenario: Stage A landings render and both manifests validate at 15 courses
    Given both manifests are published with courseOrder containing the 15 Stage A ids
    When a reader opens either the conventional-erp or sharia-erp path landing
    Then both landings render and both manifests validate against the PathManifest schema
    And the Dangerous-1 boundary appears correctly on both landings
    And the sharia-erp landing states it "covers all the basics"

  Scenario: conventional-erp landing renders with its full course count
    Given the reader navigates to "/en/learn/paths/skills/conventional-erp"
    When the landing page loads
    Then the landing renders 27 courses in courseOrder order
    And the landing displays the Dangerous 1, Dangerous 2, and Dangerous 3 boundaries

  Scenario: sharia-erp landing renders with its full course count and states it covers the basics
    Given the reader navigates to "/en/learn/paths/skills/sharia-erp"
    When the landing page loads
    Then the landing renders 30 courses in courseOrder order
    And the landing displays the Dangerous 1 through Dangerous 4 boundaries
    And the landing states explicitly that the path covers all the basics without requiring
      "conventional-erp" first

  Scenario: the shared 27 courses are identical bodies referenced from both manifests
    Given a course id present in both "skills/conventional-erp" and "skills/sharia-erp" courseOrder
    When the reader visits that course under either path context
    Then the rendered body content is byte-identical
    And no second copy of the course file exists on disk

  Scenario: conventional-erp manifest validates against the PathManifest schema
    Given the file "manifests/skills/conventional-erp.yaml"
    When the manifest is loaded and validated
    Then it parses against the PathManifest zod schema
    And its pathId equals "skills/conventional-erp"
    And its arc equals "immediately-effective"
    And its courseOrder contains exactly 27 unique course ids

  Scenario: sharia-erp manifest validates against the PathManifest schema
    Given the file "manifests/skills/sharia-erp.yaml"
    When the manifest is loaded and validated
    Then it parses against the PathManifest zod schema
    And its pathId equals "skills/sharia-erp"
    And its courseOrder contains exactly 30 unique course ids
    And its courseOrder position 27 equals "erp-analytics-and-reporting"
    And its courseOrder positions 28 to 30 are the 3 Sharia-exclusive ids in catalog order
    And its final courseOrder entry equals "zakat-and-sharia-compliance-modules"

  Scenario: record-to-report-systems declares its hard accounting prerequisite
    Given the course "record-to-report-systems"
    When its frontmatter is inspected
    Then its frontmatter prerequisites include "financial-statements-and-close-cycle"

  Scenario: no course id, path id, or landing title contains a vendor trademark
    Given every course id in the 30-course ERP catalog and both path ids
    When every id is scanned
    Then none of them matches "sap", "oracle", "netsuite", "erpnext", or "odoo" (case-insensitive)

  Scenario: the two scope-boundary-risk courses each carry a self-check worked example
    Given the courses "erp-analytics-and-reporting" and "erp-security-and-controls"
    When each course's overview is inspected
    Then each contains a worked example distinguishing its ERP-specific scope from its named
      general-purpose existing-library sibling course

  Scenario: prerequisite consistency holds across both manifests together
    Given both "skills/conventional-erp" and "skills/sharia-erp" manifests
    When checkPrerequisiteConsistency runs against both
    Then it reports zero violations

  Scenario: a reader entering sharia-erp cold reaches Dangerous 1 without visiting conventional-erp
    Given a reader who has never visited "skills/conventional-erp"
    When they complete the first 9 courses of "skills/sharia-erp" in courseOrder
    Then they reach the same Dangerous 1 capability boundary as a conventional-erp reader would
```

## Ramp Boundary Language (re-grounded, DD-29)

Every "Dangerous N" boundary is phrased as what the reader can **read, reason about, and design** —
never "operate", "install", or "configure a live system". This is a direct, honest consequence of
A6/A7 removing all hands-on install/build/buy content: no course in this catalog claims the reader can
operate a live production ERP.

## UI-Design-Funnel Exemption

This plan ships no net-new screen or component. Every screen this plan's content appears on (paths
hub, category landing, two path landings) is designed, mocked, and rendered by
`ayokoding-learning-path-03-navigation-ui`. See
[tech-docs.md §UI-design-funnel exemption](./tech-docs.md#ui-design-funnel-exemption-recorded-explicitly).

## Product-Level Risks

- **A reader misjudges the 9-course runway to Dangerous 1 as padding.** Unlike the sibling accounting
  path's 3-course runway, ERP's cross-cutting spine (document lifecycle, posting rules,
  subledger-to-GL architecture, fiscal calendar, numbering, audit trail) has no small usable subset —
  skipping any one course leaves the reader unable to distinguish sound account-determination logic
  from broken. Mitigated by stating the reason for the longer runway on the landing itself, not just
  in narrative (Requirement L-2 in
  [tech-docs.md §Landing content contract](./tech-docs.md#landing-content-requirements-what-plan-03-cannot-infer)),
  verified by the Rule-15 usability tester.
- **A reader assumes `sharia-erp` requires `conventional-erp` first.** Because 27 of `sharia-erp`'s 30
  courses are the same shared corpus, a reader could reasonably (but wrongly) assume the conventional
  path is a prerequisite. Mitigated by Requirement L-5's explicit landing statement that
  `sharia-erp` covers all the basics on its own, reinforced by `courseOrder` actually including all
  27 shared ids.
- **The Dangerous-N ramp table fails to render legibly across breakpoints.** Both landings render
  multiple named boundaries (3 for `conventional-erp`, 4 for `sharia-erp`) plus their course-id
  anchors; a cramped or truncated rendering would undermine the honesty the ramp language is meant to
  convey. Mitigated by the Phase 7 manual verification gate, which screenshots each path landing at
  three breakpoints and asserts legibility against the color-blind-friendly palette.
- **A reader conflates the two products as one path with an optional add-on.** Because `sharia-erp`'s
  30-course `courseOrder` visually resembles `conventional-erp`'s 27-course `courseOrder` plus a tail,
  a reader skimming both landings side by side could misread `sharia-erp` as "conventional-erp plus
  extras" rather than a complete, independent path. Mitigated by both landings stating their own arc
  and boundaries independently (Requirement L-3), never cross-referencing the other path as a
  prerequisite.
- **A reader over-trusts the Dangerous-N framing as operational competence.** Because every boundary is
  phrased as what the reader can read, reason about, and design (see
  [Ramp Boundary Language](#ramp-boundary-language-re-grounded-dd-29) above), a reader could still
  over-read a "Dangerous 1" label as license to operate a live system. Mitigated by the same
  re-grounded phrasing never using "operate", "install", or "configure a live system" language at any
  boundary.
