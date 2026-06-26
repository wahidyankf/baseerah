# Product Requirements — Instruction-File Size-Budget Gate

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
- **FR8** — The validator is also a member of pre-commit and the markdown/convention CI gate.
- **FR9** — After the `AGENTS.md` trim, the gate passes on the current repo (`fail` count = 0).

## Non-Functional Requirements

- **NFR1** — Deterministic, byte-based measurement; identical result in hook and CI.
- **NFR2** — Fast: the full scan is a handful of `stat`s + small reads; target < 1s.
- **NFR3** — Library coverage ≥ 90% lines (rhino-cli `lang:rust` coverage gate).
- **NFR4** — No new shell scripts beyond the existing pre-push pattern (avoid shellcheck
  surface growth).

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
    When "repo-rules-checker" runs Step 6
    Then it reports qualitative bloat concerns across the whole instruction-file class
    And it annotates that the byte ceiling is enforced by the deterministic
      "instruction-size" gate

  Scenario: The quality-gate workflow lists the validator
    When I read "repo-governance/workflows/repo/repo-rules-quality-gate.md"
    Then the "instruction-size" deterministic validator is named among the
      convention-tier gates
```

## Definition of Done

- All Gherkin scenarios pass (or have a consuming test).
- `nx run rhino-cli:instruction-size:validation` exits 0 on the current repo.
- `AGENTS.md` ≤ 30,000 bytes; resolved Claude tree ≤ 38,000 bytes.
- Pre-push blocks an over-budget instruction-file push (verified manually per delivery §V).
- Convention authored + propagated; `repo-rules-checker` Step 6 extended; workflow updated.
- `specs:coverage` passes for `rhino-cli`.
- Phase 6 parity hand-off note recorded.
