# Instruction-File Size-Budget Gate

**Status**: In Progress
**Created**: 2026-06-26
**Authored in**: `ose-public` (this repo)
**Type**: Multi-file plan (5 documents)

> Generalizes the existing single-file `rhino-cli convention agents-md-size` gate into a
> **multi-file instruction-file size budget** covering the whole "AGENTS.md-class" of
> auto-loaded instruction surfaces, recalibrates the thresholds to the real per-harness
> limits, **forces the gate at pre-push when those files change**, trims `AGENTS.md` back
> under budget, and formalizes the rule as a governance convention propagated by
> `repo-rules-maker`, checked by `repo-rules-checker`, and listed in the
> `repo-rules-quality-gate` workflow.

## Context

A Claude Code runtime warning fired: the resolved `CLAUDE.md` tree (`CLAUDE.md` +
`@AGENTS.md` import) exceeds **40,000 characters**. Root cause: `AGENTS.md` is **41,108
bytes** — it inline-expands governance content that already lives behind links in
`repo-governance/`.

A deterministic gate already exists — `rhino-cli convention agents-md-size`
([source](../../../apps/rhino-cli/src/application/repo_governance/agents_md_size.rs)) —
but it has three gaps:

1. **Single file only.** It measures `AGENTS.md` and nothing else. The repo is
   multi-harness (Claude Code, OpenCode, Codex CLI, Copilot, Cursor, Windsurf, Junie,
   Amazon Q, Aider) and each harness auto-loads its own instruction surface. Those are
   unguarded.
2. **Thresholds are too loose.** Its hard limit is 40,000 bytes — but OpenAI Codex CLI
   **silently truncates `AGENTS.md` at 32,768 bytes** (`project_doc_max_bytes`), and an
   `AGENTS.md` near 40k pushes the Claude resolved tree well past the 40k warning. Codex
   users are **already losing the bottom ~8k bytes of `AGENTS.md`** with no warning.
3. **Not enforced at pre-push.** It is referenced as a pre-commit/CI gate but is not wired
   into `.husky/pre-push`, so an over-budget instruction file can be pushed.

## Scope

**In scope** (`ose-public`):

- Generalize `convention agents-md-size` → a config-driven, multi-file
  `convention instruction-size` validator (keep `agents-md-size` as a thin alias).
- A committed **per-file size-budget config** (`instruction-size-budget.yaml`) with
  per-surface `target` / `warn` / `fail` byte thresholds — see
  [tech-docs §2](./tech-docs.md#2-per-file-size-budget-the-numbers).
- A **Claude resolved-tree** check (`CLAUDE.md` + recursive `@imports`) against the 40k
  runtime-warning ceiling.
- **Pre-push enforcement**, changed-path-gated to the instruction-file globs (mirrors the
  existing naming/parity gates in `.husky/pre-push`). Keep the existing pre-commit + CI
  placement.
- **Trim `AGENTS.md`** back under the new ceiling (move inline-expanded content to its
  already-linked `repo-governance/` homes) so the gate ships green.
- A new **governance convention** authored by `repo-rules-maker`, propagated across all
  surfaces (AGENTS.md gate list, conventions README index, nx-targets reference).
- Extend **`repo-rules-checker` Step 6** ("AGENTS.md Size Check" → "Instruction-File Size
  Budget") and reference the validator in **`repo-rules-quality-gate.md`**.
- Companion **`specs/apps/rhino`** Gherkin (two-path rule) + `specs:coverage`.

**Out of scope** (flagged, not built here):

- `ose-primer` / `ose-infra` parity propagation — the convention + gate + checker + workflow
  must land downstream too, via the
  [multi-repo parity planning workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md).
  Tracked as Phase 6 (parity hand-off note), not executed in this plan.
- Actually authoring `.cursor/rules`, `.windsurf/rules`, `.github/copilot-instructions.md`,
  `CONVENTIONS.md` — the budget covers them **if/when added**; their globs are no-ops until
  the files exist.

## Approach Summary

1. **Phase 0** — environment baseline (`repo-setup-manager`); capture current sizes.
2. **Phase 1** — author the budget config + generalize the Rust validator (TDD) + the
   resolved-tree check; keep `agents-md-size` alias green.
3. **Phase 2** — wire the `instruction-size:validation` Nx target into pre-push
   (changed-path-gated), pre-commit, and CI.
4. **Phase 3** — trim `AGENTS.md` under the new ceiling so the gate passes.
5. **Phase 4** — author the governance convention (`repo-rules-maker`) and propagate
   references.
6. **Phase 5** — extend `repo-rules-checker` Step 6 + reference in
   `repo-rules-quality-gate.md`; companion specs + `specs:coverage`.
7. **Phase 6** — parity hand-off note for `ose-primer` / `ose-infra` (not executed here).

## Navigation

- [brd.md](./brd.md) — why this matters (business rationale)
- [prd.md](./prd.md) — what "done" looks like (personas, user stories, Gherkin acceptance criteria)
- [tech-docs.md](./tech-docs.md) — the monitored file class, the budget numbers + rationale, validator design, wiring, and diagrams
- [delivery.md](./delivery.md) — the phased, TDD-shaped execution checklist

## Related

- [apps/rhino-cli/src/application/repo_governance/agents_md_size.rs](../../../apps/rhino-cli/src/application/repo_governance/agents_md_size.rs) — the existing single-file gate being generalized
- [.husky/pre-push](../../../.husky/pre-push) — the changed-path-gated hook this plan extends
- [repo-rules-quality-gate workflow](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md) — Step 6 / preflight integration
- [.claude/agents/repo-rules-checker.md](../../../.claude/agents/repo-rules-checker.md) — Step 6 "AGENTS.md Size Check" being extended
- [repo-governance/development/infra/nx-targets.md](../../../repo-governance/development/infra/nx-targets.md) — canonical Nx target names
