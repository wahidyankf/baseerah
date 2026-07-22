# Product Requirements — Skills Paths: Accounting

> **Programme decisions** — the `R*` rules and `A*` amendments cited below are defined in
> [ayokoding-learning-path-programme.md](../ayokoding-learning-path-programme.md).

## Product Overview

Two paths, two manifests, twenty-four courses (nineteen shared, five Sharia-specific), served at
`/en/learn/paths/skills/conventional-accounting` and `/en/learn/paths/skills/sharia-accounting`
(A10).

A **skills path** is a subject-scoped reading arc over the shared course library, addressed as
`skills/<subject>` — two URL segments. The arc segment is absent because every skills path **is** the
`immediately-effective` arc (R8). Both accounting paths record `arc: immediately-effective` in their
manifest data.

What ships:

- **Two manifests** — `manifests/skills/conventional-accounting.yaml` (`pathId:
skills/conventional-accounting`, 19-entry `courseOrder`) and `manifests/skills/sharia-accounting.yaml`
  (`pathId: skills/sharia-accounting`, 24-entry `courseOrder` — the **same** 19 IDs as the
  conventional manifest, in the same order, followed by 5 Sharia-specific IDs). **No course body is
  ever duplicated** to serve both manifests — both reference the shared 19 by ID, which the schema
  already supports (see [tech-docs §Two manifests, nineteen shared courses](./tech-docs.md#two-manifests-nineteen-shared-courses-a10--a11)).
- **Two landing contents** — the arc promise, the ramp boundaries relevant to that path, and the
  outbound links to the two linked prerequisites (both paths link the same two: `sql-essentials` and
  `backend-essentials`). _(Visual design belongs to `ayokoding-learning-path-03-navigation-ui`; this
  plan supplies content only.)_
- **Twenty-four syllabus specs**, each carrying a module/topic breakdown (new requirement — see
  [tech-docs §Syllabus layer](./tech-docs.md#syllabus-layer--custody-and-shape)).
- **Twenty-four course bodies** — nineteen shared, five Sharia-specific, none duplicated.

What does not ship: any `_index.md` under `paths/` (plan 01's, A3), any ERP content (plan 07's), any
component (plan 03's), any schema (plan 02's), any edit to an existing library course, and any
building exercise or capstone that constructs a system (A6).

## The silent-failure constraint (the corpus-shaping fact)

**Accounting's characteristic failure mode is silent, and that is the single most important
pedagogical constraint on both paths.**

A trial balance still balances when:

- revenue is recognised in the wrong period;
- a lease is classified as an operating cost when it should be capitalised;
- a foreign-currency balance is translated with the wrong method;
- inventory is costed on a method inconsistent with how it is actually consumed;
- intercompany balances are consolidated without elimination;
- a **murabaha markup is booked as interest income**;
- **Zakah is computed on the wrong base, or folded into income tax**.

**Four product consequences follow directly, unchanged in kind from the original single-path design
and now applying identically to both paths:**

1. **The ramp slows after course #3 rather than accelerating**, in both paths — they share the same
   three foundation courses.
2. **Every course from #4 onward carries an explicit "what still balances while being wrong"
   section.** Required in every shared course from #4 through #19, and in every Sharia-specific
   course — 21 courses total (`ACCT_SILENT`).
3. **The Sharia stage sits at the end of the `sharia-accounting` path, not sprinkled through.**
   Teaching the contrast between a murabaha receivable schedule and a conventional amortising loan
   schedule requires the conventional model to already be solid. This is also why A11's "interleave"
   resolves to shared-then-Sharia ordering — see [tech-docs DD-601](./tech-docs.md#design-decisions).
4. **"Dangerous by here" is stated as much by what a reader _cannot_ do as by what they can**, on
   both landings.

## Personas

- **The systems builder with no accounting background, conventional-only** (north-star for
  `conventional-accounting`). Ships software, has been handed a ledger, an invoicing feature, or a
  finance integration, has no Sharia requirement. Wants to be useful within three courses and correct
  by course #19 — and wants a path that **ends** there rather than continuing into content they will
  never use.
- **The builder of Sharia-compliant financial systems** (north-star for `sharia-accounting`). Arrives
  either cold (no accounting background at all) or after finishing `conventional-accounting` and
  wanting the additional five courses. Needs three jurisdictional models rather than one, needs
  murabaha modelled as a trade, and needs Zakah and Sukuk treated as their own subjects rather than
  folded into tax or bond accounting respectively.
- **The reader who only needs the first three courses.** Unchanged — the arc must genuinely pay off
  early, in either path.
- **The ERP-path reader arriving from plan 07.** Hits ERP's record-to-report stage and is sent to
  `conventional-accounting`'s course #3 (the shared course, reachable from either manifest). Wants
  the shortest correct route in and a clear route back — not enrollment in a second full path, and
  not a detour through Sharia-specific content they may not need.
- **The reader who deep-links a single accounting course** from search or a share, with no `?path=`
  context. Must get a coherent standalone view with prerequisites surfaced, and — because the course
  may now belong to **two** paths — a way to discover both.
- **Maintainer** (content strategist / domain researcher / licensing steward / content author /
  frontend engineer / reviewer).

## User Stories

- As a **conventional-only systems builder**, I want a path that ends at conventional competence
  rather than continuing into Sharia-specific content, so that I am not asked to read material I will
  never use.
- As a **systems builder with no accounting background**, I want the first three courses to leave me
  with a working, correctly balancing ledger, so that I get real capability before I invest in depth.
- As a **systems builder**, I want every course past the foundations to name the mistakes that still
  balance, so that I can recognise a plausible wrong answer instead of trusting the totals.
- As a **builder of Sharia-compliant systems entering cold**, I want `sharia-accounting` to teach me
  everything `conventional-accounting` teaches plus the Sharia-specific depth, so that I never have to
  separately discover and complete the conventional path first.
- As a **builder of Sharia-compliant systems**, I want the standards courses to present AAOIFI, PSAK
  Syariah and MFRS-plus-BNM as three coexisting models, so that I am not confidently wrong in two of
  the three jurisdictions I might build for.
- As a **builder of Sharia-compliant systems**, I want murabaha modelled as a trading transaction with
  a disclosed markup rather than as accrued interest, so that my receivable schedule and my revenue
  recognition are both right.
- As a **builder of Sharia-compliant systems**, I want Zakah computation treated as its own subject
  rather than folded into payroll-and-tax essentials, so that I do not conflate a religious levy with
  an income tax.
- As a **maintainer**, I want the two paths to share course bodies by reference rather than by
  duplication, so that an update to a shared course never desyncs between the two manifests that cite
  it.
- As a **maintainer**, I want no course to teach a standard's text verbatim, a proprietary
  chart-of-accounts structure, or copyleft reference-implementation code, so that this corpus is
  originally authored and defensible against every body's licensing posture.
- As an **ERP-path reader**, I want the accounting courses my ERP course depends on to be reachable
  and finishable on their own, so that a cross-domain prerequisite is a detour and not a second
  curriculum.
- As the **maintainer**, I want no unverified standard number or doctrinal claim to reach a published
  course, so that the corpus is trustworthy in exactly the places where being wrong is expensive.
- As the **maintainer**, I want both manifests published early and grown in recorded stages, so that
  a truncated path cannot pass as complete and plan 07 is unblocked at the earliest safe moment.
- As a **screen-reader or keyboard user**, I want both landings' ramp statements and ordered course
  lists to be fully navigable without a mouse, so that path selection works without pointing.

## Acceptance Criteria (Gherkin)

Fourteen scenarios. Each uses **exactly one** primary `Given`, one `When`, and one `Then`; every extra
precondition, action, or outcome chains with `And`. `Scenario Outline` / `Examples` are used for the
four checks that repeat identically per path, per the Step-Keyword-Cardinality exemption for
outlines.

**How these relate to `delivery.md`'s embedded Gherkin — they are two levels, not two copies.** The
scenarios below are **requirement-level** acceptance criteria: they state what must be true of the
delivered corpus. `delivery.md`'s fenced `Gherkin (binds)` blocks are the **execution-level**
bindings actually copied into `specs/**` at authoring time; they name the same scenario titles but
phrase each step against the concrete artifact under test (a numbered course, a named file, a
specific command). **They are deliberately not verbatim copies of each other, and neither is
generated from the other.** Every scenario below is verified by at least one delivery step — some by
a titled `Gherkin (binds)` block, others by a delivery acceptance clause that asserts the same
property mechanically. When editing either side, re-check the other: a requirement here with no
verifying delivery step, or a delivery binding asserting a property stated nowhere here, is a defect
in this plan.

### The ramp

```gherkin
Scenario: The first ramp boundary is reachable in three courses
  Given both accounting manifests are published with courses 1 through 3 in courseOrder
  When a reader finishes the third course
  Then the reader can build a correctly balancing ledger and produce the three statements for a single entity
  And both landings state that the reader cannot yet safely handle revenue recognition, inventory costing, multi-currency translation, leases, or consolidation
```

```gherkin
Scenario Outline: A path landing states its arc and ramp before the course list
  Given the <path> landing is published
  When a reader opens /en/learn/paths/skills/<path>
  Then the immediately-effective promise and that path's dangerous-by-here boundaries appear before the ordered course list
  And each boundary names both what the reader can do and what the reader cannot yet do
  And the ordered course list is rendered from that path's manifest rather than hand-listed in the landing

  Examples:
    | path                    |
    | conventional-accounting |
    | sharia-accounting       |
```

### Composition

```gherkin
Scenario Outline: A manifest links its software-engineering prerequisites instead of walking them
  Given the <path> manifest is published
  When a reader inspects its courseOrder
  Then neither sql-essentials nor backend-essentials appears in courseOrder
  And the chart-of-accounts course declares sql-essentials in its prerequisites frontmatter
  And the general-ledger-system-architecture course declares backend-essentials in its prerequisites frontmatter
  And the <path> landing links both prerequisite courses at their canonical /en/learn/courses/ URLs

  Examples:
    | path                    |
    | conventional-accounting |
    | sharia-accounting       |
```

```gherkin
Scenario Outline: A two-segment skills path ID resolves end to end
  Given the <path> manifest declares its pathId and arc immediately-effective
  When a reader walks the path from its landing
  Then the landing, the prev and next controls, and the breadcrumb all resolve against that two-segment path ID
  And the ?path=<path> context persists across every course in the walk
  And no resolver assumes a three-segment path ID

  Examples:
    | path                             |
    | skills/conventional-accounting   |
    | skills/sharia-accounting         |
```

```gherkin
Scenario: Shared courses are referenced by both manifests, never duplicated
  Given both manifests are published at their full composition
  When a reader compares the first nineteen entries of each courseOrder
  Then both lists are identical, entry for entry
  And the courses directory contains exactly one body per shared course ID, never two
  And no course file exists at two different paths for the same subject matter
```

```gherkin
Scenario: The conventional-accounting manifest completes and the Sharia-accounting manifest continues past it
  Given both manifests are published with the same three foundation courses
  When the shared authoring stage finishes at nineteen courses
  Then the conventional-accounting manifest holds exactly nineteen IDs and never grows again
  And the sharia-accounting manifest also holds nineteen IDs at that point and later grows to twenty-four
  And every deferred Sharia-specific course ID is recorded as absent at that point and asserted present after its own growth step
```

### Correctness

```gherkin
Scenario: Every post-foundations course names what still balances while being wrong
  Given a shared course from number four onward, or any Sharia-specific course, is authored
  When its overview is inspected
  Then it contains an explicit section naming at least one outcome that still balances while being substantively wrong
  And that section names the observable signal, if any, that would reveal the error
```

```gherkin
Scenario: The Sharia stage presents three jurisdictional models
  Given the Sharia-standards, contract-modelling, and Sharia-ledger-architecture courses are authored
  When a reader compares their treatment of standards
  Then each names AAOIFI, PSAK Syariah, and MFRS with the Bank Negara Malaysia Shariah Governance Policy as three structurally different coexisting models
  And none of them describes AAOIFI as the single Sharia accounting standard
  And each states that Malaysia is not on AAOIFI's mandatory-adoption list
  And each states that Indonesia uses AAOIFI as a basis rather than adopting it
```

```gherkin
Scenario: A murabaha is modelled as a trade rather than as a loan
  Given the Islamic contract modelling course is authored
  When a reader compares a murabaha receivable schedule with a conventional amortising loan schedule
  Then the course shows the two schedules can look numerically similar and must be modelled differently
  And the markup is presented as fixed and disclosed at the point of sale in a trade with an underlying asset
  And the recognition is presented as a receivable and revenue from a sale rather than interest income
```

```gherkin
Scenario: Zakah is computed and reported as its own obligation, not folded into tax
  Given the Zakah computation and reporting course is authored
  When a reader compares its treatment with the conventional payroll-and-tax course
  Then Zakah is presented as a distinct religious levy computed on a defined asset base under AAOIFI FAS 9
  And the course states explicitly that Zakah is not income tax and is not computed on the same base
  And no course folds a Zakah obligation into a payroll-and-tax course's scope
```

```gherkin
Scenario: No unverified claim is published as fact
  Given the research seeding this plan marked items as Unverified or Needs Verification
  When a syllabus spec or a course body states a standard number or a doctrinal position
  Then the claim carries either a primary-source citation or an explicit confidence marker
  And every item still marked Needs Verification when the Sharia stage begins is registered with a reason in verification-log.md
```

```gherkin
Scenario: No standard's text or proprietary structure is reproduced
  Given the corpus is authored under the licensing posture in tech-docs.md
  When any course body cites a standard, a chart of accounts, or a reference implementation
  Then the standard is restated in original words with only its number, title, and official link cited
  And every chart of accounts in the corpus is originally authored rather than copied from any source
  And no copyleft reference-implementation code appears in any course body
```

### Boundaries

```gherkin
Scenario: The accounting corpus never re-teaches a linked library course
  Given the accounting corpus is authored
  When a course's scope is compared with the library course it links as a prerequisite
  Then the course states its scope boundary against that library course explicitly
  And no accounting course teaches relational modelling, query performance, or HTTP service construction as its own subject
```

```gherkin
Scenario: Both accounting paths build and validate green
  Given both manifests are at their full compositions and every body is authored
  When the app build, the affected test tiers, and the link and heading validators run
  Then the build and every affected tier succeed
  And manifest integrity and prerequisite consistency report zero violations for both accounting manifests
  And the manifests directory contains exactly two data files plus their co-located unit tests this plan owns
```

## Product Scope

### In scope

- Two `PathManifest` YAML data files at
  `apps/ayokoding-www/src/features/course-paths/manifests/skills/conventional-accounting.yaml` and
  `…/manifests/skills/sharia-accounting.yaml`.
- **Each manifest's own co-located unit test.**
- One shared Gherkin feature file (Scenario Outline, two Examples rows — one per path) and its one
  step-definition file (see [tech-docs §File Impact](./tech-docs.md#file-impact)).
- Two path landing bundles — `<PATHS>skills/conventional-accounting/_index.md` and
  `<PATHS>skills/sharia-accounting/_index.md`. **No `courseOrder` in either landing.**
- Twenty-four syllabus specs under this plan's own `syllabus/courses/`, each with a module/topic
  breakdown.
- Twenty-four course bodies under `apps/ayokoding-www/content/en/learn/courses/<course-id>/` —
  nineteen shared, five Sharia-specific, never duplicated.
- Population of both paths' cards in the paths-hub / skills-category surfaces plans 01 and 03 own, as
  content only.
- Stage-completion signals recorded in `delivery.md`, expressed at ERP-capability granularity.
- Manifest integrity, prerequisite-consistency, ownership-boundary, licensing, and ramp verification
  at every phase gate.

### Out of scope

- **Every `_index.md` under `paths/`**, including both `paths/skills/_index.md` — plan 01's (A3).
- **All ERP content** — plan 07's.
- **Any rendering component, route wiring, or design asset** — plan 03's.
- **The `PathManifest` schema, the `course-paths` core modules, and the integrity functions** —
  plan 02's.
- **The `careers/` manifests and their landings** — plan 05's.
- **Any edit to an existing library course.**
- **Any edit to plan 02's frozen `syllabus/` corpus.**
- **An Indonesian mirror** of either path or the corpus.
- **A second skills arc**, a second skills subject, or a `skills/<arc>/<subject>` URL grammar.
- **Certification-syllabus coverage**, tax-jurisdiction depth, and corporate finance.
- **Any building exercise, capstone, or scaffolded codebase (A6).**
- **Reproducing any standard's text, proprietary chart-of-accounts structure, or copyleft
  reference-implementation code (A8).**

### UI-design-funnel disposition

**Exempt — and the exemption is recorded, not silently taken.**

This plan adds **no net-new screen and no net-new component**, for either path. Both skills path
landings are rendered by `path-landing.tsx` and its siblings, all owned by
`ayokoding-learning-path-03-navigation-ui`. This plan ships **content and data into** those screens.

What this plan contributes is a **requirement**, not a design: the **ramp affordance**, and now also
the **path-choice affordance** — a reader on the skills category index needs to distinguish
`conventional-accounting` from `sharia-accounting` before entering either, which has no
single-path-per-subject precedent anywhere else in the programme. Both requirements are handed to
plan 03 in [tech-docs §Landing content contract](./tech-docs.md#landing-content-contract--what-each-landing-must-convey).

The exemption is scoped to the **design funnel only**. Because this plan ships two user-visible
surfaces, the **Rule-15 three-tester retest remains mandatory** and runs in
[Phase 7](./delivery.md#phase-7-manual-ui-verification-and-rule-15-three-tester-retest).

## Product-Level Risks

- **A course teaches something plausible and wrong.** Mitigated by the mandatory "what still balances
  while being wrong" section from #4 onward in the shared spine and in every Sharia-specific course,
  the deliberately slow post-#3 ramp, and the fact-checker on every body.
- **A shared course drifts because two manifests reference it and only one path's authoring pass
  reviews it.** Mitigated by authoring each shared course exactly once, gated by the same checkers
  regardless of which manifest(s) reference it, and by the Phase 6 sweep asserting byte-identical
  first-19 entries across both manifests.
- **The `conventional-accounting` manifest is silently extended past its terminus**, eroding the
  "the whole path is done at #19" promise. Mitigated by a falsifiable clause at every later phase
  gate asserting that file is unchanged since Phase 3.
- **"Interleave" (A11) is misread as mid-ramp alternation.** Mitigated by an explicit design decision
  stating the composition is shared-then-Sharia, preserving the silent-failure ordering argument.
- **Licensing exposure (A8)** — a course reproduces standards text or a proprietary chart-of-accounts
  structure. Mitigated by the eleven safe-authoring rules, a Phase 6 reading audit, and the explicit
  flag that IAI has no educational exception and no public-domain chart of accounts exists.
- **Overconfidence at a ramp boundary.**
- **Sharia content flattened to one standard.**
- **Verification laundering.**
- **Silent truncation of either manifest.**
- **Prerequisite drift.**
- **Scope collision with ERP.**
- **Scope collision with the library.**
- **Cross-plan file collision.**
- **Either landing reads as a syllabus.**
- **Prior-art contamination** from `business/accounting.md`.
