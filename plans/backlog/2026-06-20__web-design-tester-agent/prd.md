# Product Requirements — Web Design Tester Agent

## Product Overview

`web-design-tester` is a new live-site **design-team-advocate** tester agent. Given URL(s) and a
design-testing goal, it drives a real browser against the **running** site and judges design fidelity
and design-practice quality against five ground-truth sources, then files severity-rated findings
(`DWT-###`, with steps-to-reproduce) as a new backlog plan — the same filing pattern as its two
sibling testers. It is non-destructive, locale- and evidence-aware, and never modifies the site or
the design system.

**Metadata** (identical shape to the two sibling testers
[Repo-grounded — sibling agent frontmatter]):

- `name: web-design-tester` — scope `web`, qualifier `design`, role `tester`
- `model: sonnet` (execution-grade)
- `color: green`
- `tools: Read, Write, Edit, Glob, Grep, Bash, WebFetch, WebSearch`
- `skills: plan-creating-project-plans, plan-writing-gherkin-criteria, docs-applying-content-quality`

## Personas

Solo-maintainer repo — personas are hats the maintainer wears and the agents that consume the agent:

- **Maintainer-as-design-reviewer** — wants to know whether the shipped, rendered page matches the
  mockups, honours the design tokens at runtime, reuses the shared component library, and is not
  cramped or visually inconsistent — without manually eyeballing every breakpoint and locale.
- **Orchestrating web workflow** — delegates a third (design) pass and folds `DWT-###` findings into
  the one combined fix plan.
- **`plan-maker`** — promotes the filed backlog plan into an executable delivery plan.
- **`swe-ui-*` / `swe-*-dev` developer** — consumes `findings.md` to fix design defects.

## User Stories

- **US-1** — As a maintainer, I want a tester that judges the **live rendered page** against my
  committed mockups, so that post-build design drift is caught before users see it.
- **US-2** — As a maintainer, I want the tester to check the running page against the **design
  tokens/theme** (colors, spacing, typography) at **runtime**, so that token overrides invisible to
  the static source checker are caught.
- **US-3** — As a maintainer, I want the tester to flag **reinvented UI** that the shared
  design-system library (`libs/web-ui` here; `libs/ts-ui` in primer/infra) already provides, so that
  the design language does not fragment.
- **US-4** — As a maintainer, I want to optionally pass an **external design source** (a Figma link
  or mockup URL) at invocation and have the live page compared against it when provided.
- **US-5** — As a maintainer, I want the tester to evaluate **general design best-practice** — visual
  consistency, hierarchy, information density ("not cramped") — grounded by `web-researcher` design
  references, so the judgement is principled rather than a vibe.
- **US-6** — As a maintainer, I want findings filed as a **backlog plan** (`DWT-###`, severity,
  steps-to-reproduce) identical in shape to the other two testers, so the triad is symmetric and
  feeds the same pipeline.
- **US-7** — As a maintainer, I want every design pass to cover **all supported locales** per
  breakpoint (375/768/1280) with **committed evidence screenshots**, so coverage matches the
  repo-wide parity standard.
- **US-8** — As a maintainer, I want the agent's charter to pin the **runtime-vs-static boundary**
  against `swe-ui-checker`, so the two never duplicate each other.
- **US-9** — As a maintainer, I want the capability to land **identically in all three sibling
  repos** with repo-specifics localized, so governance stays in parity.

## Acceptance Criteria (Gherkin)

> Step-keyword cardinality: each scenario uses exactly one primary Given/When/Then; extras chain with
> And/But.

### Agent definition

```gherkin
Feature: web-design-tester agent definition

Scenario: The agent file exists with the correct metadata shape
  Given the web-design-tester plan is executed
  When the agent definition is authored at .claude/agents/web-design-tester.md
  Then its frontmatter sets name "web-design-tester", model "sonnet", and color "green"
  And its tools are "Read, Write, Edit, Glob, Grep, Bash, WebFetch, WebSearch"
  And its skills are plan-creating-project-plans, plan-writing-gherkin-criteria, and docs-applying-content-quality

Scenario: The charter pins the runtime-vs-static boundary against swe-ui-checker
  Given the web-design-tester agent definition
  When a reader reaches its boundary section
  Then it states web-design-tester judges live mockup/token fidelity and design practice on a RUNNING page
  And it states swe-ui-checker performs the STATIC source token and accessibility compliance check with no overlap

Scenario: The agent documents all five ground-truth sources
  Given the web-design-tester agent definition
  When a reader reaches its ground-truth section
  Then it names committed plan-folder mockup assets, design tokens/theme at runtime, design-system primitives, an optional external design source, and general design best-practice grounded by web-researcher
  And it notes the libs/web-ui (ose-public) versus libs/ts-ui (ose-primer, ose-infra) design-system divergence
```

### Filing format and triad symmetry

```gherkin
Feature: web-design-tester output

Scenario: Findings are filed as a backlog plan matching the sibling testers
  Given a completed web-design-tester run with at least one finding
  When the agent files its output
  Then it creates a backlog plan with README, brd, prd, findings, and spec-gaps documents
  And every finding carries a DWT-### id, a severity rating, and numbered steps-to-reproduce

Scenario: Every design pass covers all locales and breakpoints with committed evidence
  Given a multi-locale target under design test
  When the agent runs its responsive/visual passes
  Then it exercises every supported locale (discovered from the app i18n config) at 375, 768, and 1280 px
  And each cited screenshot is saved into the plan's evidence/ subfolder named by phase, locale, and breakpoint

Scenario: The agent stays non-destructive and never audits source
  Given the web-design-tester agent is running against a live site
  When it evaluates the page
  Then it only navigates, renders, screenshots, and reads the page and ground-truth files
  And it does not modify the site, fix code, or audit component source the way swe-ui-checker does
```

### Registration surfaces

```gherkin
Feature: web-design-tester registration

Scenario: The agent is registered across every governance surface
  Given the web-design-tester agent definition exists
  When the registration steps complete
  Then the agent-naming convention tester row and bullet list web-design-tester alongside the two siblings
  And the .claude/agents/README.md Testing section and role table reference web-design-tester
  And AGENTS.md lists web-design-tester in its agent catalog

Scenario: Bindings are re-synced and validate clean
  Given the agent and registrations are in place
  When npm run generate:bindings is run
  Then .opencode/agents/web-design-tester.md is generated
  And npm run validate:sync and npm run harness:bindings-validation both exit 0
```

### Workflow extension (two testers to three)

```gherkin
Feature: combined web workflow runs three testers

Scenario: The workflow delegates to all three testers with source-attributed findings
  Given the combined web test-fixing-planning workflow
  When it is updated to seat the design lens
  Then its name, intro, and agent list reference web-design-tester alongside the exploratory and usability testers
  And its synthesized plan keeps findings attributed as EWT-### versus UWT-### versus DWT-###
  And the workflows README row and intro reference the three-tester workflow
```

### Triad reciprocal complementarity

```gherkin
Feature: the three tester agents reciprocally complement each other

Scenario: Each tester definition names the other two and pins its boundary
  Given the three live-site tester agent definitions
  When a reader opens any one of web-exploratory-tester, web-usability-tester, or web-design-tester
  Then that definition references the other two testers by name in its relationship section
  And it states its own non-overlapping lens (correctness versus usability versus design)
  And all three distinguish themselves from swe-ui-checker's static-source audit
```

### Web-UI-feature-change 3-tester governance rule

```gherkin
Feature: web-UI feature-change plans run the three testers near the end

Scenario: Rule 15 binds the triad, not a single tester
  Given the User-Facing Delivery Hardening Convention Rule 15
  When it is updated for the triad
  Then it requires a web-UI feature-change plan to run all three live-site testers near the end of delivery
  And it records each EWT-###, UWT-###, and DWT-### finding as an unchecked delivery.md checkbox
  And the findings are fixed within the same plan-execution run before archival

Scenario: The rule is consistent across the planning agents
  Given the three-tester rule
  When plan-maker, plan-checker, and plan-execution-checker are updated
  Then plan-maker emits the three-tester near-end round for web-UI feature-change plans
  And plan-checker flags its absence and plan-execution-checker verifies it ran across all locales
  And the rule excludes CLI/text output and pure governance/agent-definition plans
```

### Three-repo parity

```gherkin
Feature: three-repo parity

Scenario: The capability lands topic-identically with repo-specifics localized
  Given the change is complete in ose-public
  When it is propagated to ose-primer and ose-infra
  Then each repo carries a web-design-tester agent and the same registration surfaces
  And design-system references read libs/ts-ui and the specs target reads spec-coverage in ose-primer and ose-infra
  And each repo passes its own pre-push and CI gates
```

## Product Scope

**In scope**: the agent definition; its charter/methodology/ground-truth/filing sections; all
registration surfaces; **reciprocal relationship updates to the two existing tester agents
(`web-exploratory-tester`, `web-usability-tester`) so all three cross-reference each other and pin
their non-overlapping boundaries** (Phase 1b); the workflow extension + rename to
`web-ux-test-fixing-planning`; the web-UI-feature-change 3-tester governance rule (hardening +
plan-execution + plan-maker + plan-checker); binding re-sync + validation; three-repo topic-identical
propagation with localization; a `repo-rules-maker` consistency sweep per repo.

**Out of scope**: a design checker/fixer pair; changes to app/lib source; running an actual design
test; modifying the two existing testers beyond the required reciprocal cross-reference/relationship
updates.

## Product-Level Risks

- **Boundary blur** with `swe-ui-checker` — mitigated by an explicit boundary section + a Gherkin
  scenario asserting non-audit-of-source (US-8 / boundary scenario above).
- **Filing-format drift** from the sibling testers — mitigated by modelling the output section
  verbatim on the siblings (triad-symmetry scenario above).
- **Localization slip** in primer/infra (`libs/ts-ui`, `spec-coverage`) — mitigated by the
  localization map in `tech-docs.md` and the parity Gherkin scenario.

## Related Documents

- [`brd.md`](./brd.md) — business rationale and success metrics
- [`tech-docs.md`](./tech-docs.md) — architecture, ground-truth sources, registration surfaces, diagrams
- [`delivery.md`](./delivery.md) — phased delivery checklist
- [Acceptance Criteria Convention](../../../repo-governance/development/infra/acceptance-criteria.md)
