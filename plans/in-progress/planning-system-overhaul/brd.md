# Business Requirements Document

## Business Goal and Rationale

Seven workflow gaps cause repeated friction when executing and creating plans:

1. **Worktree termination** forces a manual interrupt in plan execution that should be automated. The human added no value — the orchestrator has all the information it needs to provision the worktree.

2. **Collapsed TDD items** obscure execution progress. When RED/GREEN/REFACTOR appear as one checkbox, the executor cannot reflect partial completion in the task list and the user loses observability.

3. **Ad-hoc grilling** means plan-maker may skip stress-testing design decisions under time pressure, and the user has no guarantee the finished plan was reviewed before handing it to the executor.

4. **No guided prompt-to-plan pipeline** — turning a vague behavioral prompt into a production-ready plan requires coordinating repo exploration, web research, grill sessions, plan-maker, and plan-quality-gate manually. There is no single workflow that orchestrates this lifecycle from prompt to pushed plan.

5. **Stale links flagged in archived content** — the markdown link checker validates internal links inside `plans/done/` and `archived/`. These directories contain frozen historical files whose internal cross-references legitimately rot over time. False failures block pushes and erode trust in the linter.

6. **Plan authoring lacks harness-neutrality awareness** — `plan-establishment` and `plan-maker` do not explicitly remind authors that governance docs must remain vendor-neutral. `plan-quality-gate` already runs the harness-neutrality scan, but authoring-time awareness is missing. Related governance files need an audit pass to surface and fix any current gaps.

7. **No standardized Phase 0** — delivery checklists have an "Environment Setup" section but no standardized Phase 0 that establishes a clean baseline and resolves ALL preexisting failures before plan work begins. No dedicated agent encapsulates this responsibility, so each plan author invents the setup sequence ad hoc.

## Affected Roles

- AI plan orchestrator (calling context following `plan-execution` workflow)
- AI plan-maker agent
- Human maintainer reviewing and executing plans

## Business-Level Success Metrics

1. **Observable fact**: `plan-execution.md` Step 0 contains no "terminate with fail" action for the CWD-mismatch case — the step auto-provisions instead.
2. **Observable fact**: `test-driven-development.md` contains a HARD RULE section that explicitly prohibits combining RED, GREEN, REFACTOR into one checkbox.
3. **Observable fact**: `plan-maker.md` contains a mandatory pre-creation grill step and a mandatory post-creation grill step in its Planning Workflow.
4. **Observable fact**: `repo-governance/workflows/plan/plan-establishment.md` exists, contains YAML frontmatter with `name: plan-establishment`, and defines all eight steps (0 through 7) for the prompt-to-pushed-plan pipeline.
5. **Observable fact**: `.markdownlintignore` and `.markdownlint-cli2.jsonc` exclude `plans/done/` and `archived/` from markdown linting; `repo-governance/development/quality/markdown.md` documents the archive exclusion policy.
6. **Observable fact**: `plan-establishment.md` and `plan-maker.md` contain explicit harness-neutrality reminders; all plan-related governance files audited and updated where gaps are found.
7. **Observable fact**: `.claude/agents/repo-setup-manager.md` exists and is synced to `.opencode/agents/`; `plan-maker.md` delivery checklist template mandates Phase 0 as the first phase; `plan-execution.md` Step 1b references Phase 0.

## Business-Scope Non-Goals

- Changing how `plan-checker` or `plan-fixer` enforce TDD structure (deferred)
- Changing the `grill-me` skill itself
- Broad harness-neutrality audit of all governance files (only plan-related files in scope)

## Business Risks and Mitigations

| Risk                                                                                       | Mitigation                                                                                               |
| ------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| Auto-provisioning worktree pollutes `main` with uncommitted in-flight work                 | The worktree is created from `HEAD` at a clean state; it does not affect `main`                          |
| Mandatory grill steps slow down simple plans                                               | Grilling resolves open branches; simple plans with no open branches complete the grill in one pass       |
| plan-establishment doubles grill sessions (plan-establishment + plan-maker each grill)     | plan-maker's grills become short validation passes when decisions are pre-resolved by plan-establishment |
| Research step returns irrelevant results for internal governance changes                   | Step 2 has an explicit skip condition; user confirms in Step 1 grill whether research is needed          |
| Excluding `plans/done/` from link checker may allow dead links to accumulate undetected    | Archived content is frozen historical record; dead links are expected and acceptable there               |
| Harness-neutrality audit surfaces violations in files outside the plan-related scope       | Scope explicitly limited to plan-related governance files; broader audit is future work                  |
| Phase 0 baseline resolution uncovers preexisting failures outside the current plan's scope | Resolving preexisting failures before plan work is a standing repo principle; Phase 0 enforces it        |
