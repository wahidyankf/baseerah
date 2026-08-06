# Idea Briefs (Two-Pagers)

This folder holds **two-pagers**: shortened, promotable idea briefs that are richer than a one-line
todo but deliberately **not** full five-document plans. Each idea is one `<slug>.md` file. `ideas/`
is the first stage of the plan lifecycle:

```text
ideas/ (two-pagers) → backlog/ (full 5-doc plans) → in-progress/ → done/
```

## Two-Pagers

Grouped into Eisenhower quadrants by [`plan-ideas-grooming`](../../repo-governance/workflows/plan/plan-ideas-grooming.md).

### Q2 — Important, Not Urgent

No active plan waits on these and no live defect is running, but each carries a real stake. This is the plan-from-here quadrant.

- [beaver-nest-first-deploy](./q2-not-urgent-important/beaver-nest-first-deploy.md) — provision the first real `prod-beaver-nest-fe`/`stag-beaver-nest-be` deploy targets; the deployer agents and CI callers ship wired but dormant.
- [orphaned-harness-binding-artifacts](./q2-not-urgent-important/orphaned-harness-binding-artifacts.md) — seven `.opencode/skills/` directories and `.opencode/commands/monitor-ci.md` have no `.claude/` source at all, so nothing regenerates or validates them.
- [unvalidated-cross-repo-citations](./q2-not-urgent-important/unvalidated-cross-repo-citations.md) — the link gate stops at this repo's root and skips inline code, so 16 `plans/done/` citations went unchecked and 14 name folders that do not exist here.
- [vitest-include-glob-silent-false-pass](./q2-not-urgent-important/vitest-include-glob-silent-false-pass.md) — all three Vitest configs here use a single `test` block rather than named projects, so the originating multi-project miss cannot recur, but the silent-false-pass class still can.
- [web-ui-reuse-existing-server-residual](./q2-not-urgent-important/web-ui-reuse-existing-server-residual.md) — `reuseExistingServer: true` survives in exactly one config, `libs/web-ui/e2e/playwright.config.ts:19`, behind a target CI never runs, so the proposed `!process.env.CI` gate would be a no-op here.

### Q4 — Neither Urgent nor Important

Parked deliberately. Kept because the need may become real, not because it is real now.

- [beaver-nest-be-nullbyte-path-error-envelope](./q4-not-urgent-not-important/beaver-nest-be-nullbyte-path-error-envelope.md) — a null-byte path request gets a bodyless Kestrel 400 instead of `beaver-nest-be`'s usual `Error` envelope; fixing it means replacing the server.
- [beaver-nest-first-llm-integration](./q4-not-urgent-not-important/beaver-nest-first-llm-integration.md) — give `beaver-nest-be` its first real LLM-backed route; today there is no capture, no notes, no prompt plumbing at all.
- [beaver-nest-persistence-layer](./q4-not-urgent-not-important/beaver-nest-persistence-layer.md) — add the first concrete feature that durably stores and retrieves product data on the SQLite foundation; no product schema exists yet.

## What a Two-Pager Is

A two-pager sits between a throwaway one-liner and a full backlog plan: short enough to write in one
sitting and triage at a glance, yet structured enough to decide whether to promote it. Target ≤ ~2
printed pages, ~8 short sections:

1. **Title + one-line summary** (plus a provenance note when it came from a plan)
2. **Problem / context** — a specific example of why the status quo doesn't work, with concrete data points (counts/sizes/measurements; never fabricated)
3. **Why now** — the urgency, dependency, or opportunity window
4. **Prior art / precedents** — 2-5 named precedents (tool/pattern/standard/prior plan) with links; lightweight at capture, deep `web-researcher` study deferred to promotion
5. **Proposed direction (sketch)** — core elements only; **not** wireframes, file paths, or Gherkin
6. **Rough scope & non-goals** — in-scope bullets + an explicit out-of-scope list
7. **Risks & open questions** — rabbit holes + the unknowns that block promotion
8. **What success looks like + promotion signal**

Keep it a brief, not a plan: one paragraph per section, no fabricated metrics, no secrets, and no
BRD/PRD/tech-docs/delivery split (that is the backlog plan's job).

## Before You Add — Integrate, Don't Duplicate

Before creating a new two-pager, scan the index above for an existing brief on the same problem or
area and **fold the new thought into it** rather than adding a near-duplicate. Two two-pagers about
the same underlying problem should be one. This applies equally to learnings routed here by the
Knowledge Capture phase — check for an existing home first.

## Promoting a Two-Pager to a Plan

Promotion is a **completeness gate, not a perfection gate**: an idea is ripe when every section holds
a real answer — including honest open questions — and the remaining questions genuinely need a full
plan's deeper work to answer. When a two-pager is ripe, create `backlog/<slug>/` as a full plan, carry
the problem/scope/questions forward, then **delete** the two-pager and drop its line above. "Not
promoted yet" is a legitimate state, distinct from "rejected".

## See Also

- [Plans Organization Convention → Ideas Folder (Two-Pagers)](../../repo-governance/conventions/structure/plans.md#ideas-folder-two-pagers)
  — the authoritative convention, template, and discipline.
- [Knowledge Capture Convention](../../repo-governance/development/quality/knowledge-capture.md) —
  routes future-work learnings from plan execution here as two-pagers.

## Grooming Log

### 2026-08-06 — plan-ideas-grooming (all four OSE repos in one run)

Swept 120 two-pagers across `ose-public`, `ose-primer`, `ose-private`, and `beaver-nest`; 79 survive. Every surviving idea carries a residency verdict (R1 secrets-bearing, R2 single-repo-only, R3 generalizable) and an Eisenhower quadrant.

- **Classified**: 8 idea(s) resident here, filed into quadrant folders.
- **Renamed** (4) — each of these four briefs shares a subject with an `ose-public` brief but is an
  independent idea, re-derived against this repo's own measured state; each kept a filename that
  described the sibling's framing rather than its own content, so all four are renamed as one class:
  - `ose-private-opencode-ci-monitor-orphan.md` → `orphaned-harness-binding-artifacts.md` — the named
    `ose-private` artifact does not exist here; the finding is this repo's source-less `.opencode/` surfaces.
  - `audit-e2e-reuse-existing-server-config.md` → `web-ui-reuse-existing-server-residual.md` — one
    residual instance in `libs/web-ui`, not the six-config sweep the old name implied.
  - `cross-repo-governance-link-parity.md` → `unvalidated-cross-repo-citations.md` — this repo stands
    outside the parity loop; the defect is unvalidated inherited citations, not parity drift.
  - `vitest-glob-coverage-guard.md` → `vitest-include-glob-silent-false-pass.md` — no named Vitest
    projects exist here, so the subject is the silent-false-pass class, not a glob guard.
- **Deduplicated out** (31) — the surviving copy is named for each:
  - `acceptance-clause-vacuity.md` → `ose-public/plans/ideas/q1-urgent-important/acceptance-clause-vacuity.md`
  - `agents-md-progressive-disclosure.md` → `ose-public/plans/ideas/q1-urgent-important/agents-md-progressive-disclosure.md`
  - `bare-repo-landing-method-step-count-drift.md` → `ose-public/plans/ideas/q2-not-urgent-important/bare-repo-landing-method-step-count-drift.md`
  - `behavior-coverage-json-report-wiring.md` → `ose-public/plans/ideas/q2-not-urgent-important/behavior-coverage-json-report-wiring.md`
  - `ci-setup-rust-toolchain-retry.md` → `ose-public/plans/ideas/q2-not-urgent-important/ci-setup-rust-toolchain-retry.md`
  - `class-sweep-completeness.md` → `ose-public/plans/ideas/q2-not-urgent-important/class-sweep-completeness.md`
  - `contributing-md-trunk-guidance-and-naming-exemption.md` → `ose-public/plans/ideas/q2-not-urgent-important/contributing-md-trunk-guidance-and-naming-exemption.md`
  - `demo-apps-standards-recheck.md` → `ose-primer/plans/ideas/q2-not-urgent-important/demo-apps-standards-recheck.md`
  - `doc-command-existence-validation.md` → `ose-public/plans/ideas/q2-not-urgent-important/doc-command-existence-validation.md`
  - `harness-binding-catalog-drift.md` → `ose-public/plans/ideas/q2-not-urgent-important/harness-binding-catalog-drift.md`
  - `iam-service-module.md` → `ose-public/plans/ideas/q2-not-urgent-important/iam-service-module.md`
  - `merge-queue-adoption.md` → `ose-public/plans/ideas/q2-not-urgent-important/merge-queue-adoption.md`
  - `mermaid-state-label-render-clipping-warn.md` → `ose-public/plans/ideas/q2-not-urgent-important/mermaid-state-label-render-clipping-warn.md`
  - `mermaid-validator-does-not-check-syntax.md` → `ose-public/plans/ideas/q1-urgent-important/mermaid-validator-does-not-check-syntax.md`
  - `nx-affected-cross-worktree-contamination.md` → `ose-public/plans/ideas/q2-not-urgent-important/nx-affected-cross-worktree-contamination.md`
  - `plan-archival-in-pr-multi-repo-gap.md` → `ose-public/plans/ideas/q2-not-urgent-important/plan-archival-in-pr-multi-repo-gap.md`
  - `plan-quality-gate-convergence.md` → `ose-public/plans/ideas/q2-not-urgent-important/plan-quality-gate-convergence.md`
  - `post-cutoff-dependency-migrations.md` → `ose-public/plans/ideas/q2-not-urgent-important/post-cutoff-dependency-migrations.md`
  - `pr-review-bot-identity.md` → `ose-public/plans/ideas/q2-not-urgent-important/pr-review-bot-identity.md`
  - `propagation-checklist-under-coverage.md` → `ose-public/plans/ideas/q2-not-urgent-important/propagation-checklist-under-coverage.md`
  - `repo-rules-quality-gate-convergence.md` → `ose-public/plans/ideas/q2-not-urgent-important/repo-rules-quality-gate-convergence.md`
  - `rhino-cli-env-backup-scripts.md` → `ose-public/plans/ideas/q2-not-urgent-important/rhino-cli-env-backup-scripts.md`
  - `rust-crate-structural-checklist-promotion.md` → `ose-public/plans/ideas/q2-not-urgent-important/rust-crate-structural-checklist-promotion.md`
  - `sdlc-gate-standard-property-bound-lag.md` → `ose-public/plans/ideas/q2-not-urgent-important/sdlc-gate-standard-property-bound-lag.md`
  - `sibling-main-ci-never-runs-on-merge.md` → `ose-public/plans/ideas/q2-not-urgent-important/sibling-main-ci-never-runs-on-merge.md`
  - `source-code-credential-scanning.md` → `ose-public/plans/ideas/q2-not-urgent-important/source-code-credential-scanning.md`
  - `standardize-cis.md` → `ose-public/plans/ideas/q2-not-urgent-important/standardize-cis.md`
  - `syllabus-conformance-validator.md` → `ose-public/plans/ideas/q2-not-urgent-important/syllabus-conformance-validator.md`
  - `tri-repo-rhino-cli-byte-identity-gate.md` → `ose-public/plans/ideas/q1-urgent-important/tri-repo-rhino-cli-byte-identity-gate.md`
  - `vendor-audit-kiro-term.md` → `ose-public/plans/ideas/q2-not-urgent-important/vendor-audit-kiro-term.md`
  - `web-ui-alert-destructive-dark-contrast.md` → `ose-public/plans/ideas/q2-not-urgent-important/web-ui-alert-destructive-dark-contrast.md`
- **Unresolved follow-ups**: none. No relocation was interrupted, and no rename hit a collision at its
  computed name. A termination-condition audit run after the first pass caught three briefs still
  sharing a filename with a different `ose-public` idea; they are the three extra renames above.

> Last groomed: 2026-08-06
