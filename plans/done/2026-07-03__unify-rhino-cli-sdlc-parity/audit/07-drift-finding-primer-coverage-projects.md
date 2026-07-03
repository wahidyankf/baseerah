# Drift finding: primer coverage.projects registry is NOT complete (tech-docs \S2.3 was wrong)

tech-docs.md \S2.3 claimed primer's `coverage.projects` registry verdict is "complete" (✅).
Verified against the working tree: **false**.

- primer `nx show projects` total: 26
- primer `repo-config.yml` `coverage.projects` entry count: 20 (gap: 6)
- Gap set: `clojure-openapi-codegen`, `elixir-cabbage`, `elixir-gherkin`,
  `elixir-openapi-codegen`, `crud-contracts`, `ts-ui-tokens`
- Each of these 6 projects **does** own a real `specs/libs/<name>/behavior/gherkin` (or, for
  `crud-contracts`, `specs/apps/crud/behavior/{crud-be,crud-web}/gherkin`) tree — confirmed via
  `find`. These are not codegen-only/no-specs exemptions; they are genuine registry omissions,
  identical in nature to public's already-flagged `fsharp-crane-core`/`web-ui-token`/
  `organiclever-contracts`/`ose-contracts` gap.
- Cross-check: infra's `coverage.projects` (8 entries) is a **perfect** 1:1 match against its 8
  `nx show projects` — infra's "complete" verdict IS accurate (verified both directions:
  zero missing, zero stale entries).
- Root cause: the same 6 primer projects already flagged for the `namedInputs.specs` gap
  (Phase 3 delivery item) also lack a `coverage.projects` entry — one underlying omission,
  two symptoms. Public's plan already treats these as two _separate_ delivery items (Phase 2:
  `namedInputs.specs` completion + `coverage.projects` completion); primer's Phase 3 only had
  the `namedInputs.specs` item. Added a matching `coverage.projects` completion item to Phase 3.

**Action taken**: tech-docs.md \S2.3 row + \S3 Phase-3 bullet corrected; a new Phase 3 delivery.md
item added (mirrors the Phase 2 public item); a matching Task created for 1:1 sync.
