# Planning System Overhaul

## Context

Seven gaps in the planning system require fixes:

1. **Worktree gate fails instead of auto-provisioning** — `plan-execution.md` Step 0 terminates when the working directory does not match the declared worktree path. Executors should provision it themselves.

2. **RED/GREEN/REFACTOR not always separate checklist items** — the TDD convention shows a nested "TDD cycle" pattern but does not prohibit collapsing all three phases into one checkbox. Each phase must always be its own `- [ ]` item.

3. **`plan-maker` does not mandate grilling before and after plan creation** — the `grill-me` skill is referenced as optional. Grilling must be mandatory at two points: before writing and after writing.

4. **No guided prompt-to-plan pipeline** — turning a behavioral prompt into a production-ready plan requires coordinating repo exploration, research, grill sessions, plan-maker, and plan-quality-gate manually. A single orchestrating workflow is missing.

5. **Markdown link checker flags stale links in archived content** — `plans/done/` and `archived/` contain frozen historical files whose internal links may be stale. The link checker should not validate archived content.

6. **Plan-related workflows lack explicit harness-neutrality awareness** — the new `plan-establishment` workflow and `plan-maker` agent do not explicitly flag harness-neutrality constraints when plans touch agents, skills, or `repo-governance/` paths. Plan-quality-gate already runs the harness-neutrality scan, but authoring-time awareness is missing. All related governance files need an audit pass.

7. **No standardized Phase 0 (repo setup + baseline)** — delivery checklists have an "Environment Setup" section but no standardized Phase 0 that establishes a clean baseline and resolves ALL preexisting failures before plan work begins. No dedicated agent exists for this responsibility.

## Scope

**In-scope**:

- `repo-governance/workflows/plan/plan-execution.md` — Step 0 auto-provisioning; Step 1b Phase 0 reference
- `repo-governance/development/workflow/test-driven-development.md` — RED/GREEN/REFACTOR hard rule
- `.claude/agents/plan-maker.md` — mandatory grill before + after; Phase 0 mandate in delivery template
- `AGENTS.md` — summaries for plan-maker, plan-establishment, repo-setup-manager
- `repo-governance/workflows/plan/plan-establishment.md` — new workflow (8 steps); harness-neutrality awareness
- `repo-governance/workflows/plan/README.md` — add plan-establishment
- `.markdownlintignore` — add `plans/done/` and `archived/`
- `.markdownlint-cli2.jsonc` — add `plans/done/**` and `archived/**` to ignores
- `repo-governance/development/quality/markdown.md` — document archive exclusion
- `.claude/agents/repo-setup-manager.md` — new agent definition

**Out-of-scope**:

- `plan-checker` / `plan-fixer` enforcement of the new TDD hard rule (future plan)
- `grill-me` SKILL.md — no changes needed

## Approach Summary

All changes are governance documentation plus two config file edits (`.markdownlintignore`, `.markdownlint-cli2.jsonc`) and two new files (`plan-establishment.md`, `repo-setup-manager.md`). The `plan-establishment.md` and `repo-setup-manager.md` content is fully specified in [`tech-docs.md`](./tech-docs.md). `npm run generate:bindings` syncs both updated and new agent definitions to `.opencode/agents/`.

## Worktree

Worktree path: `worktrees/planning-system-overhaul/`

Provision before execution (run from repo root):

```bash
claude --worktree planning-system-overhaul
```

## Plan Files

- [Business Requirements](./brd.md)
- [Product Requirements](./prd.md)
- [Technical Documentation](./tech-docs.md)
- [Delivery Checklist](./delivery.md)
