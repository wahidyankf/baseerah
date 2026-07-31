# Delivery Checklist: Coverage Artifact Relative Paths

**Delivery Mode**: `worktree-to-pr` (the repo default). One delivery unit, one PR.

> **Legend** — `[AI]` = agent-executable step. `[HUMAN]` = requires a human decision or credential
> this repo's agents may not exercise. No `[HUMAN]` step is anticipated for this plan — recorded
> for completeness per the legend convention.

## Worktree

Worktree path: `worktrees/coverage-artifact-relative-paths/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree coverage-artifact-relative-paths
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

## Phase 1: Investigation and Fix

- [ ] [AI] Confirm the mechanism writing absolute paths into
      `libs/fsharp-crane-core/tests/unit/coverage.json`
- [ ] [AI] Search for other tracked artifacts with the same risk across `libs/*`/`apps/*`
- [ ] [AI] Decide gitignore vs. relative-path emission per artifact found and apply it
- [ ] [AI] Confirm no coverage-gate tooling depends on the tracked file's prior committed state

### Phase 1 Gate

- [ ] [AI] Running `test:quick`/`test:coverage` from two different absolute checkout paths
      produces zero diff in any coverage artifact
- [ ] [AI] `nx run fsharp-crane-core:test:coverage` (or equivalent) still enforces its threshold
      correctly after the fix

> **Pause Safety**: this plan is Backlog (not started) — no work has begun, so there is nothing to
> resume. Promotion to `in-progress/` re-reads this README from the top.
