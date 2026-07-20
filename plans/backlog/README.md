# Backlog Plans

Planned projects for future implementation.

## Planned Projects

- [doc-command-existence-validation](./doc-command-existence-validation/README.md) — a rhino-cli
  validator that checks every command a doc claims to run against the commands the repo actually
  exposes (Nx targets, npm scripts, rhino-cli subcommands), closing the drift that left ~35 distinct
  cited `rhino-cli:<target>` names unresolvable and `links:validation` cited 40 times despite never
  having existed.
- [2026-07-20\_\_plan-quality-gate-convergence](./2026-07-20__plan-quality-gate-convergence/README.md)
  — makes the plan-quality-gate loop converge faster without relaxing any check: a defect-class
  registry, a deterministic pre-flight pass, symmetric empirical verification at authoring and fix
  time, class-level remediation, and an in-surface/latent scope split that gives the loop a bounded
  termination target. Mined from a real 17-iteration audit chain.
- [2026-07-20\_\_repo-rules-quality-gate-convergence](./2026-07-20__repo-rules-quality-gate-convergence/README.md)
  — turns the repo-rules-quality-gate sweep into a bounded, measurable convergence loop, replacing
  trust-based AI re-derivation with a deterministic count-diff.
- [2026-07-20\_\_contributing-md-trunk-guidance-and-naming-exemption](./2026-07-20__contributing-md-trunk-guidance-and-naming-exemption/README.md)
  — corrects `CONTRIBUTING.md`, which still tells contributors to work directly on `main` despite
  `worktree-to-pr` being the default delivery mode, and lands the naming exemption that currently
  makes any edit to that file unlandable at pre-commit.
- [2026-07-20\_\_pr-review-bot-identity](./2026-07-20__pr-review-bot-identity/README.md)
  — provisions a dedicated GitHub App / CI-scoped identity for `pr-review-maker`, so blocking
  reviews can post as `REQUEST_CHANGES` instead of landing as `COMMENT` and reading as unblocked.
- [2026-07-20\_\_mermaid-state-label-render-clipping-warn](./2026-07-20__mermaid-state-label-render-clipping-warn/README.md)
  — adds a WARN-level `rhino-cli` rule for `stateDiagram-v2` edge labels that clip in GitHub's
  renderer, with the threshold derived empirically rather than assumed from character count.
- [2026-07-20\_\_agents-md-progressive-disclosure](./2026-07-20__agents-md-progressive-disclosure/README.md)
  — moves detail in `AGENTS.md` behind progressive disclosure. The file sits at 29,995 bytes against
  a 30,000-byte fail threshold, so the next governance addition of any size fails the gate.
- [2026-07-20\_\_vendor-audit-kiro-term](./2026-07-20__vendor-audit-kiro-term/README.md)
  — adds `Kiro` to the vendor-audit term list across all three repos, closing a preventive gap where
  a Kiro mention in governance prose would pass the vendor-neutrality scanner silently.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

**Naming**: Plans in `backlog/` use NO date prefix — just the slug (e.g.,
`doc-command-existence-validation/`). A date prefix is applied only when a plan is archived to
`done/`, where it records the completion date.

When creating a new plan:

1. Create folder: `[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md
3. Add the plan to this list
