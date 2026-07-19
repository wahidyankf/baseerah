<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: doc-command-existence-validation

## Learning: The canonical target doc had drifted further than the reported defect

- **Context**: Authoring this plan; grounding the three reported nonexistent-target citations
  against `npx nx show project rhino-cli --json`.
- **Observation**: `repo-governance/development/infra/nx-targets.md` lists six targets absent from
  the resolved 21-target graph, not the three originally reported —
  `specs:domain:coverage`, `links:validation`, `mermaid:validation`,
  `headings:hierarchy-validation`, `cross-vendor:parity-validation`,
  `harness:bindings-validation`. The table is framed as canonical but functions as a roadmap.
- **Why it might generalize**: When a "canonical" reference doc is the drifting artifact, discipline
  ("check the canonical doc") cannot help — the check must run against the running system. This may
  warrant a broader governance note about which docs are allowed to state aspirational content and
  how such content must be labelled.

## Learning: Six never-implemented target names, preserved after deletion from the canonical doc

- **Context**: The Q7 grill answer (a maintainer override of the recommended split-into-exists/planned
  approach) directs Phase 3 to DELETE the six nonexistent rows from the "Canonical governance and
  validation targets" table in `repo-governance/development/infra/nx-targets.md` outright, with no
  replacement "Planned targets" table.
- **Observation**: Deleting the rows discards whatever roadmap intent they encoded, so the names are
  recorded here instead. These six were listed as canonical but **never implemented** — none resolves
  against the live Nx graph as of plan authoring:
  - `specs:domain:coverage`
  - `links:validation`
  - `mermaid:validation`
  - `headings:hierarchy-validation`
  - `cross-vendor:parity-validation`
  - `harness:bindings-validation`
- **Why it might generalize**: The principle the override encodes is that a canonical reference doc
  asserts only what exists, and roadmap intent must live somewhere that is not load-bearing for
  execution. If any of these six is later genuinely wanted, it should enter through a plan rather
  than through a doc row that quietly asserts it already exists. Whether the repo needs a sanctioned
  home for toolchain-roadmap intent — as opposed to scattering it across plan learnings logs — is an
  open question worth routing during triage.

## Learning: The plan-template `learnings.md` scaffold omits the required H1

- **Context**: The scaffold emitted for this plan (and, per the coordinator, for the
  `parallel-orchestration-shared-machine-governance` plan before it) contained only the two HTML
  comment lines and no `#` heading.
- **Observation**: `md heading-hierarchy validate` requires exactly one H1 per markdown file, so the
  scaffold as templated fails the repo's own validator the moment it is created. The coordinator
  fixed this instance in place by adding `# Learnings: doc-command-existence-validation`.
- **Why it might generalize**: This is a template defect, not a per-plan authoring slip — it has now
  recurred across at least two independent plans, which is the signature of a durable-surface gap
  rather than human error. The scaffold in the `plan-creating-project-plans` skill, and the
  equivalent block in the `plan-maker` agent definition, should carry the H1 so every future plan
  starts validator-clean.
