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
