# Idea Briefs (Two-Pagers)

This folder holds **two-pagers**: shortened, promotable idea briefs that are richer than a one-line
todo but deliberately **not** full five-document plans. Each idea is one `<slug>.md` file. `ideas/`
is the first stage of the plan lifecycle:

```text
ideas/ (two-pagers) → backlog/ (full 5-doc plans) → in-progress/ → done/
```

## Two-Pagers

- [acceptance-clause-vacuity](./acceptance-clause-vacuity.md) — acceptance clauses that cannot fail certify nothing; require falsifiability in both directions.
- [agents-md-progressive-disclosure](./agents-md-progressive-disclosure.md) — `AGENTS.md` sits under 20 B beneath its 30,000 B ceiling; restore headroom via progressive disclosure.
- [bare-repo-landing-method-step-count-drift](./bare-repo-landing-method-step-count-drift.md) — the landing method numbers eight steps but is summarized as "seven-step" in nine sites across three repos.
- [beaver-nest-first-deploy](./beaver-nest-first-deploy.md) — provision the first real `prod-beaver-nest-fe`/`stag-beaver-nest-be` deploy targets; the deployer agents and CI callers ship wired but dormant.
- [beaver-nest-first-llm-integration](./beaver-nest-first-llm-integration.md) — give `beaver-nest-be` its first real LLM-backed route; today there is no capture, no notes, no prompt plumbing at all.
- [beaver-nest-be-nullbyte-path-error-envelope](./beaver-nest-be-nullbyte-path-error-envelope.md) — a null-byte path request gets a bodyless Kestrel 400 instead of `beaver-nest-be`'s usual `Error` envelope; fixing it means replacing the server.
- [beaver-nest-persistence-layer](./beaver-nest-persistence-layer.md) — give `beaver-nest-be` a real data store; every route today is stateless and the greeting is a hardcoded constant.
- [behavior-coverage-json-report-wiring](./behavior-coverage-json-report-wiring.md) — wire rhino-cli's JSON-run-report cross-check into project targets + CI.
- [ci-setup-rust-toolchain-retry](./ci-setup-rust-toolchain-retry.md) — `setup-rust` flaked 7× in one phase on the toolchain download; add a retry in all three repos.
- [class-sweep-completeness](./class-sweep-completeness.md) — class sweeps miss producer surfaces, root instruction files, and the block around a cited substring.
- [contributing-md-trunk-guidance-and-naming-exemption](./contributing-md-trunk-guidance-and-naming-exemption.md) — fix stale "work on main" guidance blocked by the filename-naming gate.
- [demo-apps-standards-recheck](./demo-apps-standards-recheck.md) — re-verify the ose-primer demo apps still meet current repo standards.
- [doc-command-existence-validation](./doc-command-existence-validation.md) — a rhino-cli validator catching doc-cited commands that don't exist.
- [harness-binding-catalog-drift](./harness-binding-catalog-drift.md) — triage the 2026-07-20 harness-compatibility external-drift findings.
- [iam-service-module](./iam-service-module.md) — a shared IAM (authn/authz) capability; early placeholder, mostly open questions.
- [mermaid-validator-does-not-check-syntax](./mermaid-validator-does-not-check-syntax.md) — `md mermaid validate` is cited as the Mermaid-correctness gate but never parses syntax; broken diagrams pass clean.
- [mermaid-state-label-render-clipping-warn](./mermaid-state-label-render-clipping-warn.md) — a WARN rule for `stateDiagram-v2` edge labels that clip in GitHub's renderer.
- [nx-affected-cross-worktree-contamination](./nx-affected-cross-worktree-contamination.md) — `nx affected` includes uncommitted working-directory changes, so a concurrent plan's stray WIP blocked an unrelated docs-only push.
- [plan-archival-in-pr-multi-repo-gap](./plan-archival-in-pr-multi-repo-gap.md) — `plan-execution.md` §8's Archival-in-PR rule has no provision for a plan whose delivery spans multiple repositories.
- [plan-quality-gate-convergence](./plan-quality-gate-convergence.md) — make the plan-quality-gate loop converge in a bounded number of iterations without relaxing checks.
- [post-cutoff-dependency-migrations](./post-cutoff-dependency-migrations.md) — track and promote the deferred dependency bumps as their soak windows clear.
- [pr-review-bot-identity](./pr-review-bot-identity.md) — a dedicated bot identity so blocking reviews post as `REQUEST_CHANGES`.
- [propagation-checklist-under-coverage](./propagation-checklist-under-coverage.md) — propagation checklists enumerated by change ID under-cover the merged changeset; derive the file list from the PR diff.
- [refresh-agent-illustrative-example-paths](./refresh-agent-illustrative-example-paths.md) — 4 generic agent definitions still illustrate usage with example paths naming apps this repo deleted.
- [repo-rules-quality-gate-convergence](./repo-rules-quality-gate-convergence.md) — turn the repo-rules sweep into a bounded, count-diff convergence loop.
- [rhino-cli-env-backup-scripts](./rhino-cli-env-backup-scripts.md) — scripted backup/restore of the gitignored rhino-cli `.env*` files.
- [rust-crate-structural-checklist-promotion](./rust-crate-structural-checklist-promotion.md) — promote the Rust crate structural checklist to governance once a 2nd crate exists.
- [web-ui-alert-destructive-dark-contrast](./web-ui-alert-destructive-dark-contrast.md) — shared `Alert variant="destructive"` renders at 1.99:1 in dark mode; the obvious token fix is unsafe.
- [sdlc-gate-standard-property-bound-lag](./sdlc-gate-standard-property-bound-lag.md) — `ose-public`'s SDLC gate standard trails both siblings on two name-bound bareness claims; adopt their wording.
- [sibling-main-ci-never-runs-on-merge](./sibling-main-ci-never-runs-on-merge.md) — `main-ci` is schedule-triggered in both siblings, so a merge to their `main` gets no post-merge CI signal.
- [source-code-credential-scanning](./source-code-credential-scanning.md) — evaluate Betterleaks (gitleaks successor) for pre-commit + CI credential detection in source.
- [specs-checker-phantom-nx-targets](./specs-checker-phantom-nx-targets.md) — `specs-checker.md`'s Drift Detection section names Nx targets that don't exist.
- [standardize-cis](./standardize-cis.md) — audit for any CI-standardization residual left by the toolchain-parity work.
- [syllabus-conformance-validator](./syllabus-conformance-validator.md) — a deterministic `rhino-cli md syllabus validate` for course-file section conformance, deferred until the format settles.
- [tri-repo-rhino-cli-byte-identity-gate](./tri-repo-rhino-cli-byte-identity-gate.md) — a standing diff gate over the `apps/rhino-cli` byte-identity boundary across all three repos.
- [vendor-audit-kiro-term](./vendor-audit-kiro-term.md) — add `Kiro` to the vendor-audit denylist before it leaks into governance prose.

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
