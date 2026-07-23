<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: ayokoding-learning-path-02-schema-and-prerequisite-dag

## Learning: `ayokoding-www`'s `test:integration` and `test:e2e` are `echo` no-op stubs

- **Context**: authoring this plan's Testing and Verification Strategy and its per-phase quality
  gates, which originally claimed both tiers were "run to prove no regression".
- **Observation**: `apps/ayokoding-www/project.json` defines `test:integration` as
  `echo 'no-op: integration tier not used for this content app'` and `test:e2e` as
  `echo 'no-op: target not applicable for this project'`. Both always exit 0, so any acceptance
  clause resting on them is vacuous — a false green. This plan's one shipped-code change is a URL
  emitter (`content-url.ts`), and E2E is precisely the tier that would catch a cross-page URL
  regression.
- **Why it might generalize**: any plan citing an Nx target as evidence should read
  `project.json`'s `options.command` first; a target name is not proof that a target does anything.
  The repo-wide half — whether `ayokoding-www` should have real E2E coverage at all — is a code-homed
  change and therefore cannot land inline in this plan.
- **Terminal state**: _pending triage at Phase 6_ — the plan-doc half is already routed inline (the
  "to prove no regression" framing was removed from `tech-docs.md`, `brd.md` and `delivery.md`); the
  code-homed half must be filed as a separate `plans/backlog/<slug>/` plan per the code-routing rule.

<!--
Entry shape — append one block per generalizable learning, the moment it surfaces:

## Learning: <one-line summary>

- **Context**: what was being done when this surfaced
- **Observation**: what was noticed (sanitized — see the secret/sensitivity gate)
- **Why it might generalize**: the litmus reasoning
- **Terminal state**: routed inline to <path> / filed as plans/backlog/<slug>/ / discarded — <reason>

If execution surfaces nothing generalizable, replace "None yet." above with:
`No generalizable learnings — <one-line reason>`
-->
