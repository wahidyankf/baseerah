# Business Requirements — Plan Quality Gate Convergence

## Business Goal

Make the [plan-quality-gate workflow](../../../repo-governance/workflows/plan/plan-quality-gate.md)
reach its first zero in materially fewer iterations, with **zero reduction** in the defects it
catches.

The gate is not optional infrastructure — it is the last checkpoint before a plan is handed to an
execution-grade agent that will follow its checkboxes literally. A defect that survives the gate
becomes broken work. So the goal is strictly cost-side: same catch rate, less loop.

## Business Impact

### Pain points (all observed in the archived 2026-07-20 chain)

**The loop is expensive at the wrong end.** Seventeen audit passes ran against a five-document plan.
The audits are thorough — several run 300+ lines with live command probes, fixture construction, and
multi-renderer markdown checks. Spending that lens on a mistyped backtick (iteration 12) or a
six-space indent (iteration 13) is a poor allocation: both are mechanically detectable in
milliseconds.

**Fixes are a defect source, not just a defect sink.** Five separate iterations were consumed by a
defect the _previous_ fix introduced at the site it was repairing. A loop whose repair step has a
non-trivial injection rate cannot converge quickly regardless of how good its detection is.

**The loop had no principled stopping point.** After the change-surface defects were exhausted, the
checker began mining a 1000+ line document for pre-existing latent defects. This is unbounded by
construction — there is no finite N after which a large hand-authored document contains zero
findings that a sufficiently careful semantic reader would flag. The double-zero rule presumes a
stationary finding distribution; none existed.

**Knowledge did not persist across iterations.** The same trap class was rediscovered at fresh sites
in three consecutive iterations because remediation targeted instances. Nothing in the workflow
obliged either agent to generalize.

### Expected benefits

- **Several classes discovered per round instead of one.** This is the largest expected effect and
  the one with the clearest mechanism. The archived chain discovered roughly one defect class per
  round because one lens iterating can only surface what that lens's shape permits; running
  operationally-disjoint lenses in parallel changes the shape per round rather than the count of
  rounds. Perspective-Based Reading is the basis for expecting the lenses' findings to be largely
  non-overlapping. [Web-cited — via the 2026-07-20 research brief]
- **Cheaper mechanical detection.** Classes that are statically detectable get detected statically,
  before the semantic lens runs. [Judgment call] — the archived chain suggests roughly a third of its
  iterations were spent on statically-detectable defects (iterations 3, 4, 9, 10, 11, 12, 13 all
  turned on grep semantics or CommonMark structure), but this is a reading of the reports, not a
  measured figure.
- **Lower fix-site injection rate.** Requiring the fixer to empirically simulate what it writes moves
  detection to the moment of introduction rather than one expensive iteration later.
- **A terminable loop.** Termination rests on a flattened new-class discovery curve across disjoint
  lenses rather than on a round counter. Separating change-surface from latent defects bounds the
  expensive lens's target — a **scope narrowing**, legal only because the narrowed-out region gets its
  own disjoint lens and the narrowing is recorded.
- **Durable trap memory.** A registry means each trap is paid for once, repo-wide, rather than
  rediscovered per plan.

### Non-benefit — explicitly not a goal

Reducing the number of findings the gate reports. If the changes here caused the gate to report fewer
real defects, the change has failed. Success is measured as _same findings, earlier and cheaper_.

## Affected Roles

Solo-maintainer repo — these are hats the maintainer wears and agents that consume the surfaces.

| Role / consumer              | How this change lands                                                                                             |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Plan author (human or agent) | Gains a trap registry and an obligation to simulate acceptance clauses before writing them                        |
| `plan-maker`                 | New authoring-time empirical-simulation requirement; consumes the DCR                                             |
| `plan-checker`               | New in-surface/latent partition; verifies class closure; runs the deterministic pass first                        |
| `plan-fixer`                 | New class-level remediation contract; transcript-backed self-verification; routes latent findings to the registry |
| `plan-execution-checker`     | Inherits the DCR so execution-time findings reference the same class vocabulary                                   |
| Maintainer reviewing a PR    | Fewer, better-shaped review cycles; latent work bound to the unconditional pre-flight, not to a ticket            |

## Business-Level Success Metrics

Stated as observable checks, not fabricated numbers.

1. **Trap-class regression check** — replaying the archived chain's actual defect sites through the
   new deterministic pass flags them. Observable: the validator reports a non-zero count against
   fixture files reproducing the historical defects. Falsifiable both ways: it must report zero
   against the corrected forms.
2. **Class-closure evidence** — a fix report for a pattern-instantiating finding contains an
   enumeration of every instance in the plan, not just the flagged one. Observable by reading the
   fix report.
3. **Termination is defined by saturation** — the workflow's termination criteria name the flattened
   new-class discovery curve and the disjoint-lens requirement, and no longer rest on a round count.
   Observable by grep against the workflow file.
4. **No check was removed** — the count of validation steps in `plan-checker.md` does not decrease.
   Observable by comparing the Step 5x inventory before and after.
5. **The discovery curve is recorded** — each chain records its per-round count of **new** defect
   classes discovered, so termination rests on visible flattening rather than on a counter. Observable
   by reading the final report; a chain terminating without a recorded curve is itself a finding.
6. **Every lens declares its artifact set** — the roster is auditable for disjointness, and a lens
   whose declared set is a subset of another's is rejected. Observable by reading the roster.
7. **Verified fix claims carry re-executable transcripts** — a fix report marked APPLIED (verified)
   contains the literal command output. Observable by re-running the recorded command and diffing.
   Falsifiable both ways: a report claiming verification without a transcript is a finding.
8. **The split convention lists the new category** — the one document whose purpose is answering "is
   category X deterministic or AI-judged" is not silently stale after this plan lands. Observable by
   reading its Split table.

Deliberately not claimed: any specific iteration-count reduction. [Judgment call] — the mechanisms
should reduce it substantially, but a single archived chain is one data point and the honest position
is that the next few chains are the measurement.

## Business-Scope Non-Goals

- Lowering any criticality threshold, or moving any existing finding class below threshold.
- Making the gate advisory rather than blocking.
- Reducing audit report depth or removing the progressive-writing requirement.
- Optimizing the gate for token cost at the expense of catch rate.
- Reworking the sibling PR-review quality gate — the sibling convergence plan already lands the two
  termination-rule edits this plan would otherwise duplicate (README DECISION 5).

## Business Risks and Mitigations

| Risk                                                                                            | Mitigation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The latent/in-surface split converts required-fix into reported-and-owned**                   | The precise cost, stated plainly: a finding classified latent is reported but not required-fixed before `pass`. The previous draft's mitigation — a filed backlog plan gating `pass` — was **removed**, because artifact **existence** is not artifact **execution** and this plan's own README conceded such follow-ups evaporate. What replaces it binds without anyone remembering: a latent finding instantiating a registry class is re-detected by the **unconditional** deterministic pre-flight on the next invocation; a new class is appended to the registry at Knowledge Capture; the residue is an explicitly **recorded, reversible accepted risk** rather than a ticket. Plus the four unchanged guards, restated as invariants (tech-docs DD-5): mechanical surface derivation, `git log -L` provenance, CRITICAL never latent-exempt, execution-reachability promotion |
| **Provenance citation is performed by the same fatigued checker the sibling plan is about**     | Not fully closed, and recorded as such. Bounded by the safe failure direction (uncitable defaults to in-surface) and by the pre-flight re-detecting a mis-classified latent finding next invocation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| **Parallel lenses collapse into relabels of one procedure**                                     | The documented PBR-replication failure mode. Each lens declares the artifact set it reads; a lens whose set is a subset of another's is rejected as a relabel rather than admitted (prd AC-22)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| **The DCR ossifies** — entries added once, never revisited, and drift from real `grep` behavior | Every entry carries a runnable proof command; the registry's own gate re-runs them                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| **The deterministic validator produces false positives**, adding noise rather than removing it  | Validator is advisory-then-blocking: it reports, and the checker's existing FALSE_POSITIVE machinery applies                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| **Tri-repo propagation drift** — `ose-primer` / `ose-infra` diverge from `ose-public`           | Byte-identity check for `apps/rhino-cli` per the SDLC Gate Standard; per-repo propagation phases with their own gates                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| **Scope creep** — the plan grows to fix the PR-review gate too                                  | Explicitly out of scope and now **resolved rather than deferred**: the sibling plan owns that file, so there is no race and no duplicate follow-up (README DECISION 5)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

## Related

Product-level requirements, personas, and the testable Gherkin scenarios for each mechanism live in
[prd.md](./prd.md). Architecture and design decisions live in [tech-docs.md](./tech-docs.md).
