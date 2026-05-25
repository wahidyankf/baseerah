# Product Requirements — Planning and Dev Practice Improvement

## Product Overview

This plan specifies a three-component improvement to the planning and development practice layer:

1. **Grill-Me Skill** — A structured interrogation skill (`.claude/skills/grill-me/SKILL.md`) that
   stress-tests plans and designs through one-question-at-a-time multiple-choice questioning before
   implementation begins.
2. **TDD Mandate for Delivery Checklists** — A convention formalizing that all code delivery steps
   in plan checklists must follow the RED-GREEN-REFACTOR three-substep pattern.
3. **Harness-Neutral Plan Quality Gate** — An extension to
   `repo-governance/workflows/plan/plan-quality-gate.md` that adds a conditional harness-neutrality
   check step (Step 5g) for plans that touch agents, skills, rules, or governance docs.

Together these components close three gaps: unresolved design decisions surfacing mid-execution,
inconsistent TDD discipline in delivery checklists, and harness-specific assumptions slipping
through the plan quality gate.

## Personas

| Persona                               | Description                                                                                                                                           |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Developer**                         | Solo-maintainer hat; creates plans, invokes grill-me to stress-test design decisions, writes delivery checklists with RED-GREEN-REFACTOR steps        |
| **User being interrogated**           | Solo-maintainer hat; receives grill-me questions and selects from presented options to resolve design branches                                        |
| **Agent executing planning tasks**    | Consuming agent; invokes grill-me when trigger phrases appear, walks decision tree to completion                                                      |
| **Plan reviewer**                     | Consuming agent (`plan-checker`, `plan-execution-checker`); validates TDD shape in delivery checklists and harness-neutrality compliance              |
| **Contributor adding agent or skill** | Solo-maintainer hat; receives harness-neutrality feedback from the extended plan quality gate when creating plans that touch agents/skills/governance |

## Scope

### In Scope

- Create `.claude/skills/grill-me/SKILL.md` with structured interrogation behavior
- Skill presents questions as multiple-choice options (AskUserQuestion-style)
- Skill explores codebase before asking questions answerable from existing files
- TDD mandate (RED-GREEN-REFACTOR) documented as required shape for code delivery steps
- Update all related governance documents, workflow docs, agent definitions, and rules
- Run `repo-rules-maker` to propagate new convention across governance layer
- Run `repo-rules-quality-gate` to verify coherence
- Extend `repo-governance/workflows/plan/plan-quality-gate.md` with a harness-neutrality check
  step that fires when a plan touches agents, skills, rules, or governance docs

### Out of Scope

- Changes to existing plan files already in `done/` or `backlog/`
- Automated enforcement tooling for TDD (convention-based enforcement only)
- OpenCode skill mirror (OpenCode reads `.claude/skills/{name}/SKILL.md` natively
  [Repo-grounded: `AGENTS.md` → "Skills: NOT mirrored"])
- Changes to Amazon Q or other harness bindings (skill files not harness-specific)

## User Stories

### Grill-Me

**As a developer** starting a plan, **I want** structured interrogation about every design
decision before implementation, **so that** I commit to a fully-resolved design.

**As an agent** executing planning tasks, **I want** to invoke `grill-me` when "grill me" or
equivalent is mentioned, **so that** I walk the decision tree to completion.

**As a user** being interrogated, **I want** each question presented with concrete options and a
recommended answer, **so that** trade-offs are explicit and I can make an informed choice.

### TDD

**As a developer** reading a delivery checklist, **I want** code steps expressed as
RED → GREEN → REFACTOR cycles, **so that** I know to write the failing test first.

**As a plan reviewer**, **I want** delivery checklists to enforce TDD discipline at the plan
level, **so that** test-first is not optional.

### Harness-Neutral Plan Quality Gate

**As a plan checker**, **I want** to detect harness-specific assumptions in plans that touch
agents, skills, or governance docs, **so that** vendor lock-in never slips through the quality
gate unnoticed.

**As a contributor** adding an agent or skill, **I want** the plan quality gate to verify
harness-neutrality, **so that** I get immediate feedback if my plan introduces vendor-specific
content into shared governance.

### Repo Rules Propagation

**As a contributor** reading governance docs, **I want** grill-me and TDD conventions visible in
related documentation, **so that** the six-layer governance hierarchy is coherent.

## Gherkin Acceptance Criteria

### Feature: Grill-Me Skill Activation

```gherkin
Feature: Grill-Me Skill
  Background:
    Given the grill-me skill file exists at ".claude/skills/grill-me/SKILL.md"

  Scenario: Trigger on explicit request
    Given a user says "grill me on this plan"
    When the agent processes the message
    Then the agent activates the grill-me skill
    And asks the first question as a multiple-choice question
    And marks one option as recommended

  Scenario: One question at a time
    Given grill-me is active
    When the agent asks a question
    Then exactly one question is presented per message
    And the question has 2-4 options with trade-off descriptions
    And one option is marked as the recommended choice

  Scenario: Codebase exploration before asking
    Given grill-me is active
    And the next question can be answered by reading an existing file
    When the agent would normally ask that question
    Then the agent reads the relevant file instead
    And incorporates the finding into its recommendation

  Scenario: Decision tree completion
    Given grill-me is active
    When all branches of the design decision tree are resolved
    Then the agent summarizes all decisions made
    And confirms shared understanding with the user
    And signals readiness to proceed to implementation
```

### Feature: TDD-Shaped Delivery Checklists

```gherkin
Feature: TDD in Delivery Checklists
  Scenario: Code step expressed as TDD cycle
    Given a delivery checklist contains a code implementation step
    When a developer reads that step
    Then the step includes a RED substep to write a failing test
    And a GREEN substep to write minimal code to make the test pass
    And a REFACTOR substep to clean up without breaking tests

  Scenario: Non-code steps unaffected
    Given a delivery checklist contains a documentation-only step
    When a developer reads that step
    Then the step does not require RED-GREEN-REFACTOR
    And is expressed as a direct action with acceptance criteria
```

### Feature: Harness-Neutral Plan Quality Gate

```gherkin
Feature: Harness-Neutrality Check in Plan Quality Gate
  Background:
    Given the plan-quality-gate.md has been updated with a harness-neutrality check step

  Scenario: Plan touching agents triggers harness-neutrality check
    Given a plan includes steps to create or modify an agent definition file
    When plan-checker runs in strict mode
    Then it checks that the agent definition follows multi-harness-binding conventions
    And it checks that agent mirrors are generated via "npm run generate:bindings"
    And it reports CRITICAL if vendor-specific syntax appears in shared governance docs

  Scenario: Plan touching skills triggers harness-neutrality check
    Given a plan includes steps to create or modify a skill file
    When plan-checker runs in strict mode
    Then it checks that the skill body is plain markdown with no harness-specific syntax
    And it checks that no OpenCode mirror is manually created (OpenCode reads natively)

  Scenario: Plan touching governance rules triggers harness-neutrality check
    Given a plan includes steps to modify files under repo-governance/
    When plan-checker runs in strict mode
    Then it checks that changes follow the vendor-independence convention
    And it reports CRITICAL if harness-specific content appears outside a Platform Binding section

  Scenario: Plan with no agent/skill/governance changes skips harness check
    Given a plan only touches application code and tests
    When plan-checker runs
    Then it does not run the harness-neutrality check step
    And no harness-related findings are generated
```

### Feature: Related Files Updated

```gherkin
Feature: Governance Coherence After Adoption
  Scenario: Skill discoverable in governance docs
    Given planning-and-dev-practice adoption is complete
    When a contributor reads planning-related governance docs
    Then the docs reference grill-me where planning skills are mentioned

  Scenario: Repo rules quality gate passes
    Given all related files are updated
    When repo-rules-quality-gate runs
    Then it completes with zero CRITICAL findings
    And zero HIGH findings
```

## Product Risks

| Risk                                                                                               | Mitigation                                                                                                                                        |
| -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Grill-me produces too many questions and fatigues users                                            | Skill rules cap at 2-4 options per question and terminate after all decision branches are resolved; user can stop at any point                    |
| TDD mandate misapplied to non-code steps (doc edits, config changes)                               | Convention and `tech-docs.md §TDD Shape` explicitly state non-code steps use direct action + acceptance criterion, not RED-GREEN-REFACTOR         |
| Harness-neutrality check generates false positives for plans that are legitimately vendor-specific | Check is conditional on scope (agents/skills/governance); plan-checker exemption via `.known-false-positives.md` handles verified false positives |
| grill-me skill body drifts from prd.md Gherkin scenarios after authoring                           | Step 1.3 (REFACTOR) includes manual verification of all four Gherkin scenarios before Phase 1 is considered complete                              |
| Harness-neutrality check step (Step 5g) conflicts with existing plan-quality-gate steps            | Step 5g is additive and conditional; it does not change the behavior of Steps 5a–5f                                                               |

## Constraints

- Skill frontmatter must include `name` and `description` fields
  [Repo-grounded: existing `.claude/skills/*/SKILL.md` files]
- Skill directory name must be `grill-me` (kebab-case)
  [Repo-grounded: `repo-governance/conventions/structure/file-naming.md`]
- No OpenCode mirror needed
  [Repo-grounded: `AGENTS.md` → "Skills: NOT mirrored"]
- Emoji allowed in skill files and plans
  [Repo-grounded: `repo-governance/conventions/formatting/emoji.md`]
