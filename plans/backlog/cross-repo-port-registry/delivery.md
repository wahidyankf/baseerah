# Delivery Checklist: Cross-Repo Port Registry

**Delivery Mode**: `worktree-to-pr` (the repo default). One delivery unit, one PR.

> **Legend** — `[AI]` = agent-executable step. `[HUMAN]` = requires a human decision or credential
> this repo's agents may not exercise. No `[HUMAN]` step is anticipated for this plan — recorded
> for completeness per the legend convention.

## Worktree

Worktree path: `worktrees/cross-repo-port-registry/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree cross-repo-port-registry
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

## Phase 1: Investigation and Registry Design

- [ ] [AI] Enumerate every currently-allocated port across the four sibling repos
- [ ] [AI] Decide the registry's home and the validator's home (see `tech-docs.md`)
- [ ] [AI] Prototype the registry against current allocations and confirm zero collisions today

### Phase 1 Gate

- [ ] [AI] The registry, run against the current four-repo state, reports zero collisions
- [ ] [AI] The validator, run against a synthetic duplicate-port injection, reports exactly that
      collision

> **Pause Safety**: this plan is Backlog (not started) — no work has begun, so there is nothing to
> resume. Promotion to `in-progress/` re-reads this README from the top.
