# Phase 0 Audit — Unbound Feature-Directory Census

## Method

`grep -rn 'join(".*gherkin/' apps/rhino-cli/tests` finds every `feature_dir()`-style binding in the
13 existing cucumber test binaries, giving the set of gherkin subdirectories that **are** bound to a
binary. Diffed against the full on-disk `gherkin/<dir>/` listing.

## Bound directories (13) — one per existing cucumber binary

| Binary                       | Bound `gherkin/<dir>` (from source) |
| ---------------------------- | ----------------------------------- |
| `agent_naming_validator.rs`  | `agent-naming`                      |
| `agents.rs`                  | `agents`                            |
| `contracts.rs`               | `contracts`                         |
| `docs.rs`                    | `docs`                              |
| `doctor.rs`                  | `system`                            |
| `env.rs`                     | `env`                               |
| `env_contract.rs`            | `env-contract`                      |
| `java.rs`                    | `java`                              |
| `repo_config_data_driven.rs` | `repo-config`                       |
| `repo_config_validate.rs`    | `repo-config-validate`              |
| `repo_governance.rs`         | `repo-governance`                   |
| `spec_coverage.rs`           | `spec-coverage`                     |
| `workflows.rs`               | `workflows`                         |

## Full on-disk `gherkin/` directory listing (17 dirs)

```
agent-naming  agents  contracts  ddd  docs  env  env-contract  git  java
repo-config  repo-config-validate  repo-governance  spec-coverage  specs
system  test-coverage  workflows
```

## Unbound directories — 17 on disk − 13 bound = **4 confirmed**

The plan's claim is **confirmed exactly, no correction needed**:

| Unbound dir      | `.feature` files                                                                                                                                                                                                                                                                               | Scenario/Outline headers |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| `ddd/`           | 2 (`ddd-bc.feature`, `ddd-ul.feature`)                                                                                                                                                                                                                                                         | 18                       |
| `git/`           | 1 (`git-pre-commit.feature`)                                                                                                                                                                                                                                                                   | 5                        |
| `specs/`         | 10 (`behavior-coverage.feature`, `domain-coverage.feature`, `env-staged-guard.feature`, `harness-bindings.feature`, `harness-registry-driven.feature`, `validate-adoption.feature`, `validate-counts.feature`, `validate-links.feature`, `validate-tree.feature`, `worktree-agnostic.feature`) | 29                       |
| `test-coverage/` | 3 (`test-coverage-diff.feature`, `test-coverage-merge.feature`, `test-coverage-validate.feature`)                                                                                                                                                                                              | 17                       |
| **TOTAL**        | **16 `.feature` files**                                                                                                                                                                                                                                                                        | **69 scenarios**         |

All four scenario counts (18, 5, 29, 17) match delivery.md §1e's per-dir figures exactly (18+5+29+17
= 69, matching the 16-`.feature`-file / 69-scenario figure tech-docs §1.4 cites as "pure, unexecuted
spec").

Note the naming near-collision the plan itself flags: `test-coverage/` (unbound) and
`spec-coverage/` (bound, to `spec_coverage.rs`) are **different** directories testing different
command groups (`test-coverage validate` vs `specs behavior-coverage validate`) — confirmed distinct
on disk, no confusion found.
