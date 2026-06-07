# Product Requirements — Gherkin Step-Keyword Cardinality Rule

## Product Overview

Ship an explicit HARD Gherkin convention rule, propagate it across the governance
surface, enforce it with a deterministic `rhino-cli` audit category, and retrofit the
real `specs/**/*.feature` corpus to conform. The product is a combination of governance
text, a Rust linter, and normalized spec files.

## The HARD Rule (canonical text)

> **HARD rule — one primary keyword each**: Every `Scenario` MUST use exactly **one**
> primary `Given` line, exactly **one** primary `When` line, and exactly **one** primary
> `Then` line. Every additional precondition, action, or outcome MUST be chained with
> `And` or `But` — never a repeated `Given` / `When` / `Then` keyword. This reinforces
> the "one action / one behavior per scenario" norm.
>
> **Exemptions**: `Background` blocks and `Scenario Outline` `Examples` tables are
> exempt from the one-each constraint.

**Conforming example** (dogfooded throughout this plan):

```gherkin
Scenario: Login succeeds
  Given a registered user
  And the login page is open
  When the user submits valid credentials
  Then the dashboard is shown
  And a session token is set
```

**Non-conforming example** (violates — two primary `When` keyword lines):

```gherkin
# NON-CONFORMING EXAMPLE — deliberate illustration of the violation
Scenario: Login succeeds
  Given a registered user
  When the user opens the login page
  When the user submits valid credentials
  Then the dashboard is shown
```

(The fix replaces the second `When` with `And`.)

## Personas

- **Governance author** (maintainer hat / `repo-rules-maker`) — authors and propagates
  the rule.
- **Tooling engineer** (maintainer hat / `swe-rust-dev`) — builds the linter.
- **Spec maintainer** (maintainer hats / `swe-rust-dev`, `swe-golang-dev`,
  `swe-typescript-dev`) — retrofits feature files and step definitions.
- **Consuming agents** — `plan-maker`, `plan-checker`, `repo-rules-checker`, the two
  affected skills, and content makers that emit Gherkin.

## User Stories

- **US-1**: As a governance author, I want the keyword-cardinality rule stated explicitly
  in the canonical convention, so that authors and agents cannot silently violate it.
- **US-2**: As a consuming agent, I want the rule reflected in my prompt and skills, so
  that I emit conforming Gherkin by default.
- **US-3**: As a maintainer, I want a deterministic linter that flags violations, so that
  CI blocks non-conforming scenarios without manual review.
- **US-4**: As a spec maintainer, I want existing offenders fixed per-app with gates, so
  that no project's step bindings break during the retrofit.
- **US-5**: As a maintainer, I want a strict double-zero gate after the sweep, so that
  the rule is provably consistent repo-wide.

## Acceptance Criteria (Gherkin — dogfoods the new HARD rule)

```gherkin
Scenario: Canonical convention states the HARD rule
  Given the acceptance-criteria convention is open
  When a reader searches for the keyword-cardinality rule
  Then exactly one HARD rule line for one-Given-one-When-one-Then is present
  And the Background and Scenario Outline exemptions are documented
  And a conforming example and a non-conforming example are shown
```

```gherkin
Scenario: Governance sweep propagates the rule via repo-rules-maker
  Given repo-rules-maker has authored the rule in the canonical convention
  When the broad governance sweep completes
  Then every Gherkin-referencing repo-governance doc references the rule
  And the plan-maker, plan-checker, and repo-rules-checker prompts reference the rule
```

```gherkin
Scenario: Skill packages propagate the rule without repo-rules-maker
  Given the two Gherkin-referencing skill packages are edited by hand
  When the binding generator is run
  Then plan-writing-gherkin-criteria reflects the rule
  And plan-creating-project-plans reflects the rule
  And the secondary bindings are re-synced with no parity drift
```

```gherkin
Scenario: Deterministic linter flags a multi-When scenario
  Given a feature file with two primary When keyword lines in one scenario
  When the gherkin-keyword-cardinality audit runs
  Then the audit reports a violation for that scenario
  And the audit exits with a non-zero status
```

```gherkin
Scenario: Deterministic linter exempts Background and Scenario Outline
  Given a feature file whose only repeated keywords are in a Background block
  When the gherkin-keyword-cardinality audit runs
  Then the audit reports zero violations
  And the audit exits with a zero status
```

```gherkin
Scenario: Per-app retrofit fixes offenders without breaking bindings
  Given a project owning feature files that violate the rule
  When the offending scenarios are normalized and step definitions updated in lockstep
  Then the gherkin-keyword-cardinality audit reports zero violations for that project
  And that project's test:unit or test:quick passes
  And that project's spec-coverage validate passes
```

```gherkin
Scenario: Project with zero offenders is handled gracefully
  Given a project whose feature files already conform to the rule
  When the retrofit phase for that project runs the linter
  Then the audit reports zero violations
  And no feature file is edited for that project
  And the phase gate still runs and passes
```

```gherkin
Scenario: Strict quality gate confirms repo-wide consistency
  Given the rule is authored, propagated, and enforced
  When the repo-rules-quality-gate workflow runs in strict mode
  Then the gate terminates with a pass status
  And the deterministic preflight reports zero gherkin-keyword-cardinality findings
```

## Product Scope

**In-scope features**:

- Canonical rule text + normalized example snippets.
- Broad governance propagation (with and without `repo-rules-maker`).
- Deterministic `gherkin-keyword-cardinality` audit category + CLI command + orchestrator
  - preflight + CI wiring.
- Per-app/lib `.feature` + step-definition retrofit.
- Strict `repo-rules-quality-gate` double-zero pass.

**Out-of-scope features**:

- BDD-mapping semantic changes beyond cardinality.
- Behavioral rewrites of scenarios.
- New feature files / new coverage.
- Vendor-specific governance content.

## Product Risks

| Risk                                                                  | Mitigation                                                                  |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Linter parser mis-detects keyword lines inside doc-strings / comments | RED tests cover doc-string and comment edge cases before GREEN.             |
| Retrofit changes step text and orphans a step definition              | Lockstep edits within each phase; phase gate runs `spec-coverage validate`. |
| Skill edits drift from agent-prompt edits                             | Strict gate cross-validates after both propagation phases.                  |
