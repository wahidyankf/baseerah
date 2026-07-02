# Phase 0 Re-Audit Evidence

Re-run of the three-surface audit against the working tree of all 3 repos, 2026-07-02, before
Phase 1 begins. Compares against tech-docs.md \S2 (current-state, verified 2026-07-02 first pass).

## Files

- `01-rhino-cli-src-diff.md` — `diff -rq` of `apps/rhino-cli/src` pairwise (public vs primer, public vs infra)
- `02-target-keys.md` — `project.json` target keys per repo (jq)
- `03-hook-diffs.md` — `.husky/pre-commit` and `.husky/pre-push` diffs, public vs primer/infra
- `04-namedinputs-mandatory-target.md` — `namedInputs.specs` + mandatory-target (deps:audit/compat:min-version) gap counts, all 3 repos
- `05-cucumber-sweep.md` — cucumber version, [[test]] block count, tests/\*.rs count, .feature count+dirs, Cargo.lock isolation, all 3 repos
- `06-coverage-projects-reconciliation.md` — coverage.projects entry count vs nx show projects total, all 3 repos
- `07-drift-finding-primer-coverage-projects.md` — **drift found**: primer's coverage.projects registry is NOT complete (tech-docs said it was); corrected

## Verdict: matches tech-docs \S2, with ONE correction applied

All facts re-verified against the current working tree match tech-docs.md \S2/\S2.3 **except**:
primer's `coverage.projects` registry, previously logged as "complete", is actually missing 6
entries (`clojure-openapi-codegen`, `elixir-cabbage`, `elixir-gherkin`, `elixir-openapi-codegen`,
`ts-ui-tokens`, `crud-contracts`) — see `07-drift-finding-primer-coverage-projects.md`.
tech-docs.md \S2.3 + \S3 corrected; a new Phase 3 delivery.md item added; a matching Task created.

Everything else (rhino-cli src diff counts, target-key sets, hook mechanism divergence, the
16/29-20/26-6/8 namedInputs.specs split, the infra 6-project mandatory-target gap, cucumber
versions 0.23.0/0.22.1/0.23.0, [[test]] counts 0/11/0, tests/\*.rs counts 4/12/4, .feature counts
41/26/22 with the claimed dir-set divergence, public's own 4-project coverage.projects gap, and
infra's coverage.projects being a genuine exact match) is confirmed accurate — zero further drift.
