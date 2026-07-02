# Feature-Tree Union Manifest — rhino-cli Gherkin

> Records the reconciled **union** of all three repos' `.feature` directory trees
> under `specs/apps/rhino/behavior/rhino-cli/gherkin/` (Phase 1, ose-public).
> The tree is NOT a verbatim copy of any single repo: public keeps its own
> `ddd`/`specs`/`workflows` dirs and gains primer/infra's
> `contracts`/`java`/`test-coverage` dirs.

## Provenance of each directory

| Directory         | Origin                      | Disposition                             |
| ----------------- | --------------------------- | --------------------------------------- |
| `agents`          | public (shared with primer) | KEPT (public's version)                 |
| `contracts`       | primer/infra only           | ADDED (copied verbatim from ose-primer) |
| `ddd`             | public only                 | KEPT (public-unique — not deleted)      |
| `docs`            | public (shared)             | KEPT (public's version)                 |
| `env`             | public (shared)             | KEPT (public's version)                 |
| `git`             | public (shared)             | KEPT (public's version)                 |
| `java`            | primer/infra only           | ADDED (copied verbatim from ose-primer) |
| `repo-governance` | public (shared)             | KEPT (public's version)                 |
| `spec-coverage`   | public (shared)             | KEPT (public's version)                 |
| `specs`           | public only                 | KEPT (public-unique — not deleted)      |
| `system`          | public (shared)             | KEPT (public's version)                 |
| `test-coverage`   | primer/infra only           | ADDED (copied verbatim from ose-primer) |
| `workflows`       | public (shared with primer) | KEPT (public's version)                 |

Union = public's existing set (`agents ddd docs env git repo-governance
spec-coverage specs system workflows`) **plus** primer/infra's `contracts`,
`java`, `test-coverage`.

## Union directory manifest

Reproduce with:

```bash
find specs/apps/rhino/behavior/rhino-cli/gherkin -type d | sort
```

```text
specs/apps/rhino/behavior/rhino-cli/gherkin
specs/apps/rhino/behavior/rhino-cli/gherkin/agent-naming
specs/apps/rhino/behavior/rhino-cli/gherkin/agents
specs/apps/rhino/behavior/rhino-cli/gherkin/contracts
specs/apps/rhino/behavior/rhino-cli/gherkin/ddd
specs/apps/rhino/behavior/rhino-cli/gherkin/docs
specs/apps/rhino/behavior/rhino-cli/gherkin/env
specs/apps/rhino/behavior/rhino-cli/gherkin/env-contract
specs/apps/rhino/behavior/rhino-cli/gherkin/git
specs/apps/rhino/behavior/rhino-cli/gherkin/java
specs/apps/rhino/behavior/rhino-cli/gherkin/repo-config
specs/apps/rhino/behavior/rhino-cli/gherkin/repo-config-validate
specs/apps/rhino/behavior/rhino-cli/gherkin/repo-governance
specs/apps/rhino/behavior/rhino-cli/gherkin/spec-coverage
specs/apps/rhino/behavior/rhino-cli/gherkin/specs
specs/apps/rhino/behavior/rhino-cli/gherkin/system
specs/apps/rhino/behavior/rhino-cli/gherkin/test-coverage
specs/apps/rhino/behavior/rhino-cli/gherkin/workflows
```

**Update (2026-07-02)**: 4 more dirs were added after the initial union reconciliation, by the 4
net-new-behavior TDD cycles later in Phase 1 (`env-contract`, `repo-config`, `repo-config-validate`,
`agent-naming` — one `.feature` file each, for the IaC env-validation dispatch, repo-config
data-driving, the new `repo-config validate` command, and the `.opencode/agent/` naming-bug regression
test respectively). These are net-new scenarios for behavior this synthesis adds, not part of the
primer/infra union proper, but they live in the same domain-subdirectory structure `specs structure
validate` requires (root-level `.feature` files are rejected). 17 dirs total, confirmed via
`specs structure validate` (0 findings across all 6 areas) and the command above.

## Notes

- One pre-existing gherkin-parse defect was fixed while wiring the cucumber
  harness: `repo-governance/repo-governance-instruction-size.feature` had a
  line-wrapped Background step (continuation onto a second indented line), which
  cucumber-rs's gherkin 0.16 parser rejects. The two lines were joined into a
  single valid step line (no scenario/semantic change).
- Union scenarios whose step text targets a command that public renamed (e.g.
  `workflows validate-naming` → `repo-governance workflows naming validate`) are
  skipped by data (undefined step → cucumber skip, not failure), so `cargo test`
  stays green. No `@requires-<toolchain>` tags were needed: the ported
  java/contracts commands are pure-Rust and require no external toolchain.
