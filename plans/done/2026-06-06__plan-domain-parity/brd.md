# Business Requirements — Plan Domain Parity (ose-public)

## Business Goal

Restore and lock in behavioral parity of the plan domain (planning workflows, plan agents,
plan skills, and their conventions) across the three sibling repositories — `ose-public`,
`ose-primer`, `ose-infra` — by landing the merged best-of canon in `ose-public` first, so
that a plan authored or executed in any repo gets the same quality bar, the same grilling
discipline, the same worktree mechanics, and the same harness bindings.

## Business Rationale (WHY)

- **Drift tax is real and growing** `[Repo-grounded]`: the 2026-06-06 survey measured
  pairwise drift of 2–243 changed lines per file across fourteen plan-domain files (full
  numbers in the embedded matrix in [tech-docs.md](./tech-docs.md)). Every improvement made
  in one repo since the last sync silently failed to reach the other two.
- **Inconsistent agent behavior** — `plan-maker` in one repo enforces grilling gates that
  another repo's copy lacks (infra added mandatory grilling gates to the
  plan-creating-project-plans skill; public and primer drifted separately). The same prompt
  produces different plan quality depending on the repo. _Qualitative reasoning_: a single
  maintainer context-switching across three repos pays the inconsistency cost on every plan.
- **Deprecated harness surface** `[Web-cited]`: OpenCode officially deprecated boolean
  `tools` flags in favor of the `permission` object
  (<https://opencode.ai/docs/agents/>, accessed 2026-06-05 via web-researcher), and
  `.codex/agents/` per-agent directories are not an official Codex convention — the official
  path is `config.toml` `agents.<name>` sub-tables
  (<https://developers.openai.com/codex/config-reference>, accessed 2026-06-05). Staying on
  deprecated formats risks silent breakage when the harnesses drop legacy support.
- **Upstream-first economics**: `ose-public` is the documented upstream source of truth for
  scaffolding. Merging here first means the sibling plans copy a settled canon instead of
  re-litigating each file three times.

## Business Impact

| Pain today                                                      | Expected benefit after this plan                                             |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Fixes land in one repo, never propagate                         | One merged canon in ose-public; siblings adopt it via their own parity plans |
| Plan authoring mechanics differ per repo (worktree vs in-place) | One documented default: author in `worktrees/<identifier>/`, push to main    |
| Parity planning itself had a single fuzzy grill step            | Two-grill + conditional-research structure, matching plan-establishment      |
| OpenCode mirrors use a deprecated frontmatter format            | Mirrors emit the official `permission` object; emitter guarded by tests      |
| `.codex/agents/` carries unofficial per-agent config            | Per-agent config consolidated in `.codex/config.toml`; regression-guarded    |
| No plain-language record of why parity decisions were made      | `docs/explanation/plan-domain-parity-decisions.md` explains all 26 decisions |

## Affected Roles (Solo-Maintainer Hats)

- **Planner hat** — authors plans in all three repos; gains identical workflow text, grill
  gates, and worktree mechanics everywhere.
- **Repo-governance hat** — maintains conventions; gains a single merged text to maintain
  and a rationale doc for future archaeology.
- **Tooling hat** — maintains `rhino-cli`; gains a modernized OpenCode emitter and a Codex
  guard, both test-covered.
- **Consuming agents** — `plan-maker`, `plan-checker`, `plan-fixer`,
  `plan-execution-checker`, `repo-setup-manager`, and every harness binding (Claude Code,
  OpenCode, Amazon Q, Codex) consume the merged files directly.

## Success Criteria (Business Level)

No fabricated numeric KPIs; each criterion is an observable fact checkable at delivery:

1. Every in-scope ose-public file contains the merged best-of content (per-file acceptance
   checks in [delivery.md](./delivery.md)) — observable via the Phase 1–3 gates.
2. `npm run validate:sync` and `npm run validate:harness-bindings` exit 0 after
   regeneration — observable command results.
3. `.opencode/agents/*.md` mirrors contain a `permission` object and no boolean `tools`
   map; `.codex/agents/` does not exist — observable via grep/test checks.
4. The rationale doc exists and covers all 26 matrix rows — observable file + content check.
5. CI on `origin main` is fully green after the delivery push (strict double-zero bar) —
   observable via GitHub Actions run conclusions.
6. _Judgment call_: future plan-domain edits feel "edit once upstream, adopt twice" rather
   than "edit three times" — qualitative, validated by the next sync cycle.

## Business-Scope Non-Goals

- **No automated drift guard** (matrix row 26) — the invoker deliberately dropped a
  cross-repo drift checker; this plan records the drop so it is deliberate, not silent.
- **No sibling repo writes** — primer and infra changes ship in their own plans.
- **No new planning features** beyond the matrix resolutions — this is parity restoration
  plus the explicitly grilled additions (worktree default, two-grill parity workflow), not a
  planning-system redesign.
- **No CI/runner changes** — infra's self-hosted runner constraint is an infra-plan concern.

## Business Risks and Mitigations

| Risk                                                                       | Mitigation                                                                                                       |
| -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Best-of merge silently drops a sibling improvement                         | Per-file merge steps require a recorded 3-way diff review with explicit per-file acceptance strings (Phases 1–3) |
| OpenCode `permission` migration breaks the 70 generated mirrors            | TDD on the converter; `validate:sync` byte-parity check; full regeneration in the same phase                     |
| Codex sub-table migration loses the ci-monitor-subagent instructions       | Execution-time verification against the official config reference before the edit; content diffed pre/post       |
| Merged canon lands upstream but siblings never adopt (parity still broken) | Sibling plans exist and are cross-linked; recommended execution order recorded in all three READMEs              |
| Doc sweep misses a stale "boolean flags" / `.codex/agents/` reference      | Repo-wide grep sweeps with zero-hit acceptance criteria in Phase 6                                               |
