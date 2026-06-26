# Product Requirements — Instruction-File Size-Budget Gate

## Product Overview

The instruction-file size-budget gate enforces byte-level ceilings on every auto-loaded AI
instruction surface in the repository. Without these ceilings, instruction files grow
unbounded, causing silent truncation for some harnesses and wasted context budget for all. The
gate blocks any `git push` that would push an instruction file over its per-file ceiling,
surfacing a precise, actionable failure message that names progressive disclosure as the
sanctioned remediation. Success means no harness silently truncates its instruction file, the
Claude Code 40k warning never fires again, and enforcement is governed (not tribal) across all
three sibling repos.

## Personas

- **Codex/Copilot/Cursor/Windsurf/Junie agent** — auto-loads an instruction surface at
  session start; must receive the _whole_ file, not a truncated prefix.
- **Claude Code agent** — auto-loads `CLAUDE.md` + `@imports`; must stay under the 40k
  runtime warning.
- **Maintainer (`@wahidyankf`)** — edits governance prose; wants over-budget growth blocked
  at push with a precise, per-file message.
- **Contributor** — adds a rule to an instruction file; wants a fast local failure they can
  act on, not a silent degradation.

## User Stories

1. As a maintainer, I want **every auto-loaded instruction surface** size-checked against a
   per-file budget, so no harness silently truncates.
2. As a maintainer, I want the check to **fail my push** when a changed instruction file is
   over its hard ceiling, so drift never lands on `main`.
3. As a maintainer, I want the thresholds in **one committed config file**, so tuning is a
   reviewed one-line edit.
4. As a Claude Code user, I want the **resolved `CLAUDE.md` tree** checked against the 40k
   warning, so the warning I hit never recurs.
5. As a maintainer, I want the rule **documented as a convention and validated by
   `repo-rules-checker`**, so it is governed, not tribal.
6. As a downstream maintainer, I want the parity gap to `ose-primer`/`ose-infra` **explicitly
   recorded**, so it is not forgotten.

## Functional Requirements

- **FR1** — A `rhino-cli convention instruction-size` command reads
  `instruction-size-budget.yaml`, measures every existing file matching each configured glob,
  classifies each against `target`/`warn`/`fail`, and exits non-zero if any file exceeds its
  `fail` ceiling.
- **FR2** — The command also computes the **Claude resolved tree** (`CLAUDE.md` plus the
  byte size of each recursively `@`-imported file) and classifies it against the tree budget.
- **FR3** — Globs that match no file are **no-ops** (no failure for not-yet-created
  surfaces).
- **FR4** — `text`, `json`, and `markdown` output modes, matching the existing validator's
  envelope conventions (`schema: rhino-cli/instruction-size/v1`).
- **FR5** — `convention agents-md-size` remains callable as a **thin alias** that delegates
  to the generalized path scoped to `AGENTS.md` (back-compat for any existing reference).
- **FR6** — An `instruction-size:validation` Nx target wraps the command.
- **FR7** — `.husky/pre-push` runs the target **only when** the pushed range touches an
  instruction-file glob.
- **FR8** — The validator is also a member of pre-commit (early catch) and runs in the **PR
  quality gate** (`commons-quality-gate.yml`, on `pull_request` + `push:main`).
- **FR9** — Every `fail` message names **progressive disclosure** as the remediation and
  includes the path `repo-governance/principles/content/progressive-disclosure.md`.
- **FR10** — `instruction-size` is emitted as a **category of `repo-governance audit`** so the
  deterministic JSON envelope (`schema rhino-cli/repo-governance-audit/v1`) carries it;
  `repo-rules-checker` Step 0.5 consumes it (no AI byte-counting) and Step 6 defers to it.
- **FR11** — After the `AGENTS.md` trim, the gate passes (`fail` count = 0) in **each** repo.
- **FR12** — The same validator + config + target + gates + governance wiring land in all
  three repos (`ose-public`, `ose-primer`, `ose-infra`).

## Non-Functional Requirements

- **NFR1** — Deterministic, byte-based measurement; identical result in hook and CI.
- **NFR2** — Fast: the full scan is a handful of `stat`s + small reads; target < 1s.
- **NFR3** — Library coverage ≥ 90% lines (rhino-cli `test:quick` coverage gate).
- **NFR4** — No new shell scripts beyond the existing pre-push pattern (avoid shellcheck
  surface growth).

## Product Scope

### In Scope

- The multi-harness file class: every auto-loaded instruction surface configured in
  `instruction-size-budget.yaml` (AGENTS.md-class files).
- Per-file size budgets enforced at pre-push (local hard gate), pre-commit (early catch), and
  PR quality gate (server-side backstop).
- The Claude resolved tree (`CLAUDE.md` + recursive `@imports`) checked as a composite surface.
- Deterministic preflight category (`instruction-size`) integrated into the `repo-governance
audit` orchestrator and consumed by `repo-rules-checker`.
- Governance: new convention, principle backlink, checker/workflow updates.
- Parity across all three repos (`ose-public`, `ose-primer`, `ose-infra`).

### Out of Scope

- Creating instruction files for harnesses that do not yet have them (pre-budgeted as no-op
  globs; only fail when the file exists and exceeds its ceiling).
- Altering the _content_ of instruction files beyond the mechanical AGENTS.md
  progressive-disclosure trim (no rule deletions, no compression).
- Changing what any instruction file says — only its size governance is in scope.

## Product Risks

| Risk                                                                                | Impact                                                          | Mitigation                                                                                                              |
| ----------------------------------------------------------------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Users game the per-file check by splitting content into another auto-loaded surface | Budget per-file ceiling bypassed; real context cost unchanged   | Convention forbids this anti-fix explicitly; resolved-tree check catches combined size even when split across surfaces  |
| The sanctioned-remediation message is not prominent enough at the point of failure  | Contributor applies a forbidden fix (delete / compress / split) | Remediation pointer embedded directly in the gate's `fail` message — visible at the moment of failure, not just in docs |
| The three anti-fixes (delete, compress, split-to-auto-loaded) are applied anyway    | Rules silently lost or real context unchanged                   | `repo-rules-checker` Step 6 reinforces the same message; `cross-vendor:parity-validation` detects split games           |
| Contributors compress prose to fit below the ceiling                                | Instruction adherence degraded; agents act on ambiguous rules   | Convention explicitly forbids dense-prose compression; Step 6 judges qualitative bloat even below the byte ceiling      |

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Instruction-file size budget

  Background:
    Given a committed "instruction-size-budget.yaml" mapping instruction-file globs to
      target, warn, and fail byte thresholds

  Scenario: A file within target passes silently
    Given "AGENTS.md" is 24000 bytes
    And its target is 24000 and its fail ceiling is 30000
    When I run "rhino-cli convention instruction-size"
    Then the command exits with code 0
    And the file is reported with severity "ok"

  Scenario: A file over target but under the ceiling warns without failing
    Given "AGENTS.md" is 28000 bytes
    And its target is 24000 and its fail ceiling is 30000
    When I run "rhino-cli convention instruction-size"
    Then the command exits with code 0
    And the file is reported with severity "warn"

  Scenario: A file over its hard ceiling fails the command
    Given "AGENTS.md" is 41108 bytes
    And its fail ceiling is 30000
    When I run "rhino-cli convention instruction-size"
    Then the command exits with a non-zero code
    And the file is reported with severity "fail"

  Scenario: A configured glob matching no file is a no-op
    Given no file exists at ".github/copilot-instructions.md"
    When I run "rhino-cli convention instruction-size"
    Then no finding is emitted for ".github/copilot-instructions.md"

  Scenario: The Claude resolved tree is checked against the warning ceiling
    Given "CLAUDE.md" imports "AGENTS.md" via "@AGENTS.md"
    And the sum of "CLAUDE.md" plus the imported files exceeds the 38000-byte tree ceiling
    When I run "rhino-cli convention instruction-size"
    Then a finding with key "resolved-tree" is reported with severity "fail"

  Scenario: The legacy alias still works
    When I run "rhino-cli convention agents-md-size"
    Then only "AGENTS.md" is measured
    And the command behaves as a scoped instruction-size run
```

```gherkin
Feature: Pre-push enforcement of the size budget

  Scenario: Pushing an over-budget instruction file is blocked
    Given my push range modifies "AGENTS.md"
    And "AGENTS.md" exceeds its fail ceiling
    When the pre-push hook runs
    Then "nx run rhino-cli:instruction-size:validation" runs
    And the push is aborted with a non-zero exit

  Scenario: Pushing changes that do not touch instruction files skips the gate
    Given my push range modifies only "apps/ose-www/src/page.tsx"
    When the pre-push hook runs
    Then the instruction-size validation target is not invoked

  Scenario: Pushing an in-budget instruction-file edit passes
    Given my push range modifies "AGENTS.md"
    And "AGENTS.md" is within its fail ceiling
    When the pre-push hook runs
    Then the instruction-size validation target runs and exits 0
    And the push proceeds
```

```gherkin
Feature: Governance of the size-budget rule

  Scenario: The rule is documented as a convention
    Given the plan is complete
    When I look under "repo-governance/conventions/structure/"
    Then "instruction-file-size-budget.md" exists and lists the monitored file class,
      per-file budgets, and enforcement points

  Scenario: repo-rules-checker validates the budget
    Given the plan is complete
    When "repo-rules-checker" runs Step 6
    Then it reports qualitative bloat concerns across the whole instruction-file class
    And it annotates that the byte ceiling is enforced by the deterministic
      "instruction-size" gate

  Scenario: The quality-gate workflow lists the validator
    Given the plan is complete
    When I read "repo-governance/workflows/repo/repo-rules-quality-gate.md"
    Then the "instruction-size" preflight category is named among the Step 0.5 categories
```

```gherkin
Feature: Deterministic preflight tracking

  Scenario: The preflight envelope carries the instruction-size category
    Given a repo with instruction files within the configured budgets
    When I run "rhino-cli repo-governance audit -o json"
    Then the envelope schema is "rhino-cli/repo-governance-audit/v1"
    And "result.categories" contains a category named "instruction-size"

  Scenario: The AI checker defers to the deterministic finding
    Given a preflight envelope containing an "instruction-size" category
    When "repo-rules-checker" runs Step 0.5
    Then the "instruction-size" step is added to the deterministic skip set
    And Step 6 does not AI-re-derive any byte count

  Scenario: A fail message names progressive disclosure
    Given "AGENTS.md" exceeds its fail ceiling
    When I run "rhino-cli convention instruction-size"
    Then the fail message contains "progressive disclosure"
    And it contains "repo-governance/principles/content/progressive-disclosure.md"
```

```gherkin
Feature: PR quality gate and multi-repo parity

  Scenario: The PR quality gate runs the validator
    Given a pull request that modifies an instruction file over its ceiling
    When "commons-quality-gate.yml" runs
    Then the "instruction-size:validation" step fails the PR check

  Scenario: All three repos converge
    Given the plan is complete
    When I inspect "ose-public", "ose-primer", and "ose-infra"
    Then each carries the "convention instruction-size" validator, the
      "instruction-size-budget.yaml" config, the "instruction-size:validation" target,
      the pre-push glob gate, the PR-gate step, and the deterministic preflight category
    And each repo's "instruction-size:validation" exits 0
```

## Definition of Done

- All Gherkin scenarios pass (or have a consuming test).
- `nx run rhino-cli:instruction-size:validation` exits 0 in **each** repo.
- Each repo's `AGENTS.md` ≤ 30,000 bytes; each resolved Claude tree ≤ 38,000 bytes.
- Pre-push **and** the PR quality gate block an over-budget instruction-file change.
- `instruction-size` appears as a `repo-governance audit` preflight category; checker
  Step 0.5 + Step 6 consume it deterministically.
- Convention authored + propagated; principle backlinked; `repo-rules-checker` +
  `repo-rules-quality-gate.md` updated, in all three repos.
- `specs:coverage` passes for `rhino-cli` in each repo.
- All changes on `origin/main` in all three repos; worktrees removed; `ose-infra` no longer
  bare; plan archived.
