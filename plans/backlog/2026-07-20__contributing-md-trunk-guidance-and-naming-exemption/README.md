# CONTRIBUTING.md Trunk Guidance Correction + Naming Exemption

> **Status**: Backlog — filed by the Knowledge Capture phase of
> [`parallel-orchestration-shared-machine-governance`](../../done/) (merged as `60d53119b`).
>
> **Delivery Mode**: `worktree-to-pr` (repo default)

`CONTRIBUTING.md` still instructs contributors to work directly on `main`, contradicting the
repo-wide `worktree-to-pr` default delivery mode. The corrected text cannot be committed today
because `md naming validate` rejects the uppercase filename. This plan fixes both.

## Context

Two coupled problems, discovered while executing the parallel-orchestration plan.

### Problem 1 — stale trunk guidance (the actual defect)

`CONTRIBUTING.md` line 132 states: [Repo-grounded]

```markdown
- **Default**: Work directly on the `main` branch
```

This contradicts [AGENTS.md §Git Workflow §Delivery Mode](../../../AGENTS.md), where
`worktree-to-pr` (worktree → draft PR → `[HUMAN]` merge) is **the default**, and direct push to
`main` is an explicit per-plan selection rather than the assumed path. A new contributor reading
`CONTRIBUTING.md` gets the opposite instruction from every other governance surface.

### Problem 2 — the fix is unlandable (the blocker)

`lint-staged` hands every staged `.md` file to `rhino-cli md naming validate`, which enforces
`^[a-z0-9-]+\.md$` on the basename. `CONTRIBUTING.md` fails it. Verified: [Repo-grounded]

```bash
cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- \
  md naming validate --exempt "*__linkedin__*.md" CONTRIBUTING.md
# DOCS NAMING VALIDATION FAILED: 1 violation(s) found
# filename "CONTRIBUTING.md" violates lowercase-kebab-case rule (^[a-z0-9-]+\.md$)
```

Any commit touching `CONTRIBUTING.md` is therefore blocked at pre-commit.

## Design Decision Required

The validator's always-exempt set is hardcoded in
`apps/rhino-cli/src/application/docs/naming.rs` — `README.md`, `SKILL.md`, `AGENTS.md`,
`CLAUDE.md`, `_index.md` — and that file sits inside the
[rhino-cli byte-identity boundary](../../../docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary)
(zero carve-outs across `ose-public`, `ose-primer`, `ose-infra`). [Repo-grounded]

Two viable routes. **Pick one before execution.**

| Route                                                       | Where the change lands                          | Cost                                                                         | Argument for                                                                                                                             |
| ----------------------------------------------------------- | ----------------------------------------------- | ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| **A — `--exempt` flag** (pragmatic)                         | `package.json` lint-staged line, one arg        | One line, one repo, no boundary crossing                                     | The flag already exists and is already used (`--exempt "*__linkedin__*.md"`). Verified to unblock the file.                              |
| **B — add to the hardcoded always-exempt set** (principled) | `apps/rhino-cli/src/application/docs/naming.rs` | Coordinated 3-repo byte-identical change + Gherkin under `specs/apps/rhino/` | `CONTRIBUTING.md` is an ecosystem-standard root file exactly like `README.md`/`AGENTS.md`/`CLAUDE.md`, which are exempt for that reason. |

Route B is the more consistent rule; Route A is the cheaper unblock. The existing exempt-set
doc comment explicitly reasons about "ecosystem-standard root files", which favors B.

**Verification that Route A works** (already run): [Repo-grounded]

```bash
cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- \
  md naming validate --exempt "*__linkedin__*.md" --exempt "CONTRIBUTING.md" CONTRIBUTING.md
# DOCS NAMING VALIDATION PASSED: no naming violations found
```

## Scope

**In scope**:

- Correct `CONTRIBUTING.md`'s git-workflow section to state `worktree-to-pr` as the default,
  with direct-to-`main` as an explicit selection — aligned with `AGENTS.md` and the
  [Trunk Based Development Convention](../../../repo-governance/development/workflow/trunk-based-development.md).
- Land the chosen naming-exemption route so the file becomes editable.
- Sweep `CONTRIBUTING.md` for any other guidance that drifted from current governance while the
  file was effectively frozen by the naming gate.

**Out of scope**: renaming `CONTRIBUTING.md` to kebab-case (GitHub resolves the file by its
conventional uppercase name; renaming would break platform integration).

## Acceptance Criteria

```gherkin
Feature: CONTRIBUTING.md reflects the current default delivery mode

  Scenario: A contributor reads the git workflow section
    Given CONTRIBUTING.md describes the repository git workflow
    When a contributor reads the "Git Workflow" section
    Then it states worktree-to-pr as the default delivery mode
    And it does not instruct the reader to work directly on main by default
    And it agrees with AGENTS.md Delivery Mode

  Scenario: CONTRIBUTING.md can be committed
    Given a staged edit to CONTRIBUTING.md
    When the pre-commit hook runs md naming validate
    Then the command exits 0
    And no naming violation is reported for CONTRIBUTING.md

  Scenario: The naming exemption does not weaken the rule for other files
    Given a staged file named "Some-Doc.md" that is not an ecosystem-standard root file
    When the pre-commit hook runs md naming validate
    Then the command exits 1
    And a naming violation is reported for that file
```

The third scenario is the falsifiability control — an exemption that silently disables the rule
would pass the first two scenarios and fail this one.

## Notes

If Route B is chosen, the change is bound by the rhino-cli byte-identity boundary: identical
source in all three repos, plus companion Gherkin under
`specs/apps/rhino/behavior/rhino-cli/gherkin/**`, per the
[Feature Change Completeness Convention](../../../repo-governance/development/quality/feature-change-completeness.md).
