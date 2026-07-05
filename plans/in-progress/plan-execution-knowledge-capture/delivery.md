# Delivery — Plan-Execution Knowledge Capture

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.
>
> **Delivery mode** — this plan predates the `## Delivery Mode` convention, so it carries no such
> section. It is delivered under the current default: work in a worktree, then **commit and push
> directly to `origin main` (no PR)**. All git-mechanical steps (worktree add/remove, commit, push)
> are `[AI]`. There is NO `[HUMAN]` PR-merge gate. The three-repo sweep uses three worktrees, one per
> repo, each pushed to its own `origin main` by `[AI]`.

## Worktree

Worktree path: `worktrees/plan-execution-knowledge-capture/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree plan-execution-knowledge-capture
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting
the worktree after the plan is archived and pushed. `ose-primer` and `ose-infra` receive their own
worktrees provisioned inside their respective repo roots (see Phase 4 and Phase 5).

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Provision the `ose-public` worktree: `git worktree add worktrees/plan-execution-knowledge-capture origin/main`
      — acceptance: `worktrees/plan-execution-knowledge-capture/` exists and is on a branch tracking `origin/main`
- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the polyglot toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift
- [ ] [AI] Confirm sibling repos are present and clean:
      `git -C /Users/wkf/ose-projects/ose-primer status --short` and
      `git -C /Users/wkf/ose-projects/ose-infra status --short`
      — acceptance: both commands exit 0; any pre-existing WIP is recorded (do NOT `git add -A` in siblings)
- [ ] [AI] Record markdown/governance baseline in `ose-public`:
      `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` and `npm run lint:md:fix`
      — acceptance: baseline pass/fail recorded; all preexisting failures documented
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` baseline recorded and every
      preexisting failure resolved (zero unresolved)
- [ ] [AI] `worktrees/plan-execution-knowledge-capture/` exists; sibling repos reachable and their WIP recorded

> **Pause Safety**: only the toolchain was verified and the baseline recorded — no governance change
> exists yet. Safe to stop indefinitely. To resume: re-run
> `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` and confirm it is still clean.

---

## Phase 1: Author the Source-of-Truth Convention (ose-public)

> _Suggested executor: `repo-rules-maker`_

- [ ] [AI] Create `repo-governance/development/quality/knowledge-capture.md` (sibling of
      `feature-change-completeness.md`, `evidence-capture.md`) defining ALL required elements:
      the transient `learnings.md` running log; the **open-ended, principle-based triage matrix**
      (route to the home that owns the knowledge — including but not limited to `repo-governance/`,
      `docs/`, `.claude/agents/`, `.claude/skills/`, `apps/`/`libs/` code, tests, `post-mortems/`;
      plus explicit discard); the **code-routing downstream rule** (code learnings attach specs/Gherkin
      two-path + regression-test mandate + TDD, are ALWAYS a separate `plans/backlog/` plan and NEVER
      inline, with the Iron Rule 3 carve-out for current-plan blockers); the two SAFETY gates
      (repo-relevance + secret/sensitivity); destination-aware routing timing (inline for small non-code,
      backlog for large or any code); the mandatory + explicit "none"-escape rule; the pure-docs/trivial
      exemption; the anti-theater guardrails (single named owner, lives in a tool already opened,
      fixed-cadence review; guard both under- and over-capture); the "would the system catch this next
      time?" litmus; and the transient-log caveat (`plans/done/*/learnings.md` may be deleted; never the
      system of record; nothing may depend on querying it later)
      — acceptance: file exists; `grep -c "repo-relevance\|secret\|discard\|litmus\|transient\|backlog\|regression" repo-governance/development/quality/knowledge-capture.md` ≥ 6
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Add an index entry linking the new convention in
      `repo-governance/development/quality/README.md` (alongside the existing convention list)
      — acceptance: `grep -c "knowledge-capture.md" repo-governance/development/quality/README.md` ≥ 1
- [ ] [AI] Document the transient `learnings.md` file + the final Knowledge Capture phase as part of
      plan structure in `repo-governance/conventions/structure/plans.md`, cross-referencing the new
      convention
      — acceptance: `grep -c "learnings.md\|Knowledge Capture" repo-governance/conventions/structure/plans.md` ≥ 2
- [ ] [AI] Add a cross-reference in `repo-governance/conventions/structure/post-mortems.md`: failure
      learnings route to a post-mortem via the triage matrix (do not duplicate post-mortem content)
      — acceptance: `grep -c "knowledge-capture" repo-governance/conventions/structure/post-mortems.md` ≥ 1
- [ ] [AI] Add a short pointer to the new convention in `AGENTS.md` (Development Practices / Quality
      area, near the Specs & Gherkin Completeness entry)
      — acceptance: `grep -c "knowledge-capture" AGENTS.md` ≥ 1

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `test -f repo-governance/development/quality/knowledge-capture.md` exits 0
- [ ] [AI] `npm run lint:md:fix` exits 0 and the new + edited markdown files pass link/mermaid/heading
      validation: `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md links validate`,
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md mermaid validate`,
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md heading-hierarchy validate`
      — all exit 0
- [ ] [AI] `npx nx run rhino-cli:instruction-size:validation` exits 0 (AGENTS.md still within budget)

> **Pause Safety**: the convention and its doc cross-references exist and lint clean; no agent/workflow
> yet consumes it, so the repo is coherent. Safe to stop. To resume: re-run `npm run lint:md:fix`.

---

## Phase 2: Wire the Five plan-\* Workflows (ose-public)

> _Suggested executor: `repo-workflow-maker`_

- [ ] [AI] Edit `repo-governance/workflows/plan/plan-execution.md`: add running-log capture in the
      Step 2 execution loop (append sanitized learnings to `learnings.md` while executing) and add the
      Knowledge Capture phase in `### 8. Finalization and Archival` — archival BLOCKED until every
      learning is routed/backlogged/discarded and both safety gates pass
      — acceptance: `grep -c "knowledge-capture\|learnings.md\|Knowledge Capture" repo-governance/workflows/plan/plan-execution.md` ≥ 3
  - _Suggested executor: `repo-workflow-maker`_
- [ ] [AI] Edit `repo-governance/workflows/plan/plan-planning.md`: note in `### 4. Plan Creation` that
      `plan-maker` emits the Knowledge Capture phase + `learnings.md` scaffold
      — acceptance: `grep -c "knowledge-capture\|Knowledge Capture" repo-governance/workflows/plan/plan-planning.md` ≥ 1
- [ ] [AI] Edit `repo-governance/workflows/plan/plan-quality-gate.md`: reference knowledge-capture as
      an attention point
      — acceptance: `grep -c "knowledge-capture" repo-governance/workflows/plan/plan-quality-gate.md` ≥ 1
- [ ] [AI] Edit `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`: reference
      knowledge-capture as an attention point
      — acceptance: `grep -c "knowledge-capture" repo-governance/workflows/plan/plan-multi-repo-parity-planning.md` ≥ 1
- [ ] [AI] Edit `repo-governance/workflows/plan/plan-multi-repo-parity-planning-and-execution.md`:
      reference knowledge-capture as an attention point
      — acceptance: `grep -c "knowledge-capture" repo-governance/workflows/plan/plan-multi-repo-parity-planning-and-execution.md` ≥ 1

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `grep -L knowledge-capture repo-governance/workflows/plan/plan-planning.md
repo-governance/workflows/plan/plan-execution.md repo-governance/workflows/plan/plan-quality-gate.md
repo-governance/workflows/plan/plan-multi-repo-parity-planning.md
repo-governance/workflows/plan/plan-multi-repo-parity-planning-and-execution.md`
      — expected: empty output (every workflow references the convention)
- [ ] [AI] `npm run lint:md:fix` exits 0 and
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md links validate`,
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md mermaid validate`,
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md heading-hierarchy validate`
      all exit 0

> **Pause Safety**: all five workflows reference the convention; agents/skill not yet updated. The
> convention is documented and referenced but not yet emitted/enforced — coherent, safe to stop. To
> resume: re-run the `grep -L` gate command.

---

## Phase 3: Wire Agents + Skill, Re-Sync Bindings, Push ose-public

> _Suggested executor: `agent-maker` for `.claude/agents/*`; `repo-rules-maker` for the skill_

- [ ] [AI] Edit `.claude/skills/plan-creating-project-plans/SKILL.md`: emit the final Knowledge Capture
      phase into generated `delivery.md` + a `learnings.md` scaffold in the plan folder; describe the
      rubric and both safety gates
      — acceptance: `grep -c "Knowledge Capture\|learnings.md" .claude/skills/plan-creating-project-plans/SKILL.md` ≥ 2
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Edit `.claude/agents/plan-maker.md`: author the Knowledge Capture phase + `learnings.md`;
      describe the open-ended principle-based rubric (incl. the code-routing rule) and both safety gates
      — acceptance: `grep -c "Knowledge Capture\|repo-relevance\|secret" .claude/agents/plan-maker.md` ≥ 2
- [ ] [AI] Edit `.claude/agents/plan-checker.md`: validate Knowledge Capture phase presence — flag
      SILENT absence at MEDIUM criticality; the explicit "none" record passes
      — acceptance: `grep -c "Knowledge Capture\|MEDIUM" .claude/agents/plan-checker.md` ≥ 1
- [ ] [AI] Edit `.claude/agents/plan-execution-checker.md`: validate that routing actually happened
      before archival — each learning is routed-inline (non-code), filed-as-backlog-plan (any home;
      mandatory for code), or discarded-with-reason; no code born from a learning landed inline; both
      safety gates satisfied; block archival otherwise
      — acceptance: `grep -c "learnings\|routed\|backlog\|repo-relevance\|secret" .claude/agents/plan-execution-checker.md` ≥ 3
- [ ] [AI] Edit `.claude/agents/plan-fixer.md`: scaffold a missing Knowledge Capture phase +
      `learnings.md`
      — acceptance: `grep -c "Knowledge Capture" .claude/agents/plan-fixer.md` ≥ 1
- [ ] [AI] Re-sync platform bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/` and `.amazonq/` regenerated
- [ ] [AI] Confirm binding sync is clean: `git status --short .opencode .amazonq`
      — acceptance: only intended regenerated files changed; no stale drift

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` — exits 0
- [ ] [AI] `npm run lint:md:fix` then
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md links validate`,
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md mermaid validate`,
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- md heading-hierarchy validate`,
      `npx nx run rhino-cli:instruction-size:validation` — all exit 0
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by this change

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root-cause orientation). Commit preexisting fixes separately with their own conventional-commit
> messages.

### Commit Guidelines

- [ ] [AI] Commit thematically, Conventional Commits format, split by concern:
      `feat(governance): add knowledge-capture convention`,
      `docs(workflows): reference knowledge-capture in plan-* workflows`,
      `feat(agents): emit + enforce Knowledge Capture phase`,
      `chore(bindings): re-sync .opencode/.amazonq`

### Push and Post-Push CI Verification (ose-public)

- [ ] [AI] Commit and push to `origin main`
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 2 minutes via
      `gh run view --json status,conclusion`; never tight-loop)
- [ ] [AI] Verify ALL CI checks pass — if any fails, fix at root cause and push a follow-up commit;
      repeat until green
- [ ] [AI] Do NOT proceed to Phase 4 until CI is fully green

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `git status --short .opencode .amazonq` shows no stale drift after `npm run generate:bindings`
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` exits 0
- [ ] [AI] `ose-public` CI is fully green on the pushed commit(s)

> **Pause Safety**: `ose-public` is fully wired, bindings synced, pushed, and CI-green — a complete,
> self-consistent single-repo delivery. Safe to stop here indefinitely (the public repo is done; only
> the primer/infra replicas remain). To resume: `git -C /Users/wkf/ose-projects/ose-primer status`.

---

## Phase 4: Propagate to ose-primer (parity replica)

> _Suggested executor: `repo-harness-compatibility-checker` for parity confirmation_
>
> Apply the IDENTICAL public-governance change to `ose-primer`. `ose-primer` carries its own copies of
> every file edited in Phases 1-3. Work in a dedicated worktree inside the primer repo.

- [ ] [AI] Provision the primer worktree:
      `git -C /Users/wkf/ose-projects/ose-primer worktree add worktrees/plan-execution-knowledge-capture origin/main`
      — acceptance: worktree exists tracking `origin/main`
- [ ] [AI] Initialize toolchain: `npm --prefix /Users/wkf/ose-projects/ose-primer install` and
      `npm --prefix /Users/wkf/ose-projects/ose-primer run doctor -- --fix`
      — acceptance: both exit 0
- [ ] [AI] Replicate the Phase 1 convention + doc edits in `ose-primer` (create
      `repo-governance/development/quality/knowledge-capture.md`; update `quality/README.md`,
      `conventions/structure/plans.md`, `conventions/structure/post-mortems.md`, `AGENTS.md`)
      — acceptance: `test -f /Users/wkf/ose-projects/ose-primer/repo-governance/development/quality/knowledge-capture.md`
- [ ] [AI] Replicate the Phase 2 workflow references in `ose-primer` (all five `plan-*` workflows)
      — acceptance: `grep -L knowledge-capture` across the five primer workflow files is empty
- [ ] [AI] Replicate the Phase 3 agent + skill edits in `ose-primer`, then re-sync:
      `npm --prefix /Users/wkf/ose-projects/ose-primer run generate:bindings`
      — acceptance: exits 0; `git -C /Users/wkf/ose-projects/ose-primer status --short .opencode .amazonq` shows no stale drift
- [ ] [AI] Confirm public-governance parity between `ose-public` and `ose-primer` for the changed files
      (diff the `knowledge-capture.md` bodies and the shared agent/skill/workflow sections)
      — acceptance: intended content matches (repo-name-specific lines excepted)

### Local Quality Gates + Push (ose-primer)

- [ ] [AI] In the primer worktree: `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` and
      `npm --prefix /Users/wkf/ose-projects/ose-primer run lint:md:fix` — all exit 0; fix ALL failures
- [ ] [AI] Commit thematically and push to `ose-primer` `origin main`
- [ ] [AI] Monitor `ose-primer` CI (poll every 2 minutes); fix at root cause until green

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `test -f /Users/wkf/ose-projects/ose-primer/repo-governance/development/quality/knowledge-capture.md` exits 0
- [ ] [AI] `ose-primer` CI is fully green on the pushed commit(s)

> **Pause Safety**: both public repos (`ose-public`, `ose-primer`) carry the identical change, pushed
> and CI-green — the parity loop is satisfied. Safe to stop. To resume:
> `git -C /Users/wkf/ose-projects/ose-infra status`.

---

## Phase 5: Propagate to ose-infra (private replica)

> _Private repo, outside the parity loop, own copies of the governance files. Emphasize the two safety
> gates here — this is where private content (Terraform/k3s/Proxmox/coralpolyp/real hosts) lives._

- [ ] [AI] Provision the infra worktree:
      `git -C /Users/wkf/ose-projects/ose-infra worktree add worktrees/plan-execution-knowledge-capture origin/main`
      — acceptance: worktree exists tracking `origin/main`
- [ ] [AI] Initialize toolchain: `npm --prefix /Users/wkf/ose-projects/ose-infra install` and
      `npm --prefix /Users/wkf/ose-projects/ose-infra run doctor -- --fix`
      — acceptance: both exit 0
- [ ] [AI] Replicate the Phase 1-3 edits in `ose-infra` (convention + docs + five workflows + agents +
      skill), then re-sync bindings if `.claude/**` differs:
      `npm --prefix /Users/wkf/ose-projects/ose-infra run generate:bindings`
      — acceptance: `test -f /Users/wkf/ose-projects/ose-infra/repo-governance/development/quality/knowledge-capture.md`; binding status clean
- [ ] [AI] In the infra copy of `knowledge-capture.md`, ensure the repo-relevance gate explicitly
      states that infra-specific learnings stay in `ose-infra` only and NEVER cross-route to the public
      repos
      — acceptance: `grep -c "never\|only in ose-infra\|private" /Users/wkf/ose-projects/ose-infra/repo-governance/development/quality/knowledge-capture.md` ≥ 1
- [ ] [AI] Verify NO private-infra content (real hostnames, inventories, secrets) was introduced into
      any file destined for `ose-public`/`ose-primer` during Phases 1-4
      — acceptance: manual scan recorded; zero cross-routed private content

### Local Quality Gates + Push (ose-infra)

- [ ] [AI] In the infra worktree: `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` and
      `npm --prefix /Users/wkf/ose-projects/ose-infra run lint:md:fix` — all exit 0; fix ALL failures
- [ ] [AI] Commit thematically and push to `ose-infra` `origin main`
- [ ] [AI] Monitor `ose-infra` CI (poll every 2 minutes); fix at root cause until green

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `test -f /Users/wkf/ose-projects/ose-infra/repo-governance/development/quality/knowledge-capture.md` exits 0
- [ ] [AI] `ose-infra` CI is fully green on the pushed commit(s)
- [ ] [AI] Zero private-infra content leaked into public-repo files (repo-relevance gate satisfied)

> **Pause Safety**: all three repos carry the identical change, pushed and CI-green. The governance
> encoding is complete; only the dogfood Knowledge Capture triage and archival remain. Safe to stop.
> To resume: open `plans/in-progress/plan-execution-knowledge-capture/learnings.md`.

---

## Phase 6: Knowledge Capture (dogfood triage + routing)

> This plan bootstraps the very requirement it defines: harvest the learnings from building the
> knowledge-capture system itself, then triage each through the new rubric. `learnings.md` is transient
> scaffolding — everything kept MUST be routed to a durable home before archival.

- [ ] [AI] Confirm `plans/in-progress/plan-execution-knowledge-capture/learnings.md` exists and holds
      the running log accrued across Phases 0-5 (create it now if capture was deferred, reconstructing
      entries from the phase notes)
      — acceptance: `test -f plans/in-progress/plan-execution-knowledge-capture/learnings.md`
- [ ] [AI] For EACH entry, apply the litmus ("would the system catch this next time?"); discard
      non-generalizable entries with a one-line reason
      — acceptance: every discarded entry has a reason recorded in `learnings.md`
- [ ] [AI] For EACH surviving entry, run the **secret/sensitivity gate**: sanitize to `<placeholder>`
      tokens; discard any entry that cannot be sanitized without losing meaning
      — acceptance: `grep -Ei "(api[_-]?key|token|password|secret|BEGIN [A-Z ]*PRIVATE KEY)" plans/in-progress/plan-execution-knowledge-capture/learnings.md` returns no real secret (placeholders only)
- [ ] [AI] For EACH surviving entry, run the **repo-relevance gate**: route infra-only learnings within
      `ose-infra` only; route public-governance learnings in `ose-public` (and to `ose-primer` via
      parity); NEVER cross-route private-infra content into the public repos
      — acceptance: each entry records its target repo(s); zero private→public cross-routes
- [ ] [AI] Route each surviving entry to EXACTLY ONE durable home that owns that kind of knowledge —
      open-ended, including but not limited to `repo-governance/`, `docs/`, `.claude/agents/`,
      `.claude/skills/`, `apps/`/`libs/` code, tests, `post-mortems/`. **Timing (destination-aware):**
      NON-CODE home → small edit lands INLINE in this plan's commits, large work → `plans/backlog/`
      follow-up. CODE home (`apps/`/`libs/`/tests) → ALWAYS a separate `plans/backlog/` follow-up plan,
      NEVER inline (it carries its own specs/Gherkin, regression-test, and TDD gates). Record the
      backlog path in the entry
      — acceptance: every entry is terminal — routed-inline (non-code only), filed-as-backlog-plan (any
      home; mandatory for code), or discarded-with-reason; zero code changes landed inline in this plan;
      zero entries in an open state
- [ ] [AI] If no generalizable learnings survive, record the explicit escape
      `No generalizable learnings — <one-line reason>` in `learnings.md` (never leave it silently empty)
      — acceptance: `learnings.md` is either fully triaged or carries the explicit "none" record
- [ ] [AI] Land any inline routings + commit and push per-repo to each affected `origin main`; monitor
      CI to green
      — acceptance: routed edits are committed and CI-green in every affected repo

### Phase 6 Gate

> All checks below must pass before archival.

- [ ] [AI] Every `learnings.md` entry is routed-inline (non-code), filed-as-backlog-plan, or
      discarded-with-reason (zero open entries)
- [ ] [AI] Zero code changes born from a learning landed inline in this plan's PR — every code-routed
      learning is a separate `plans/backlog/` plan
- [ ] [AI] Both safety gates satisfied: no real secret in `learnings.md`; no private-infra content in
      public-repo routings
- [ ] [AI] All inline routings pushed and CI-green in each affected repo

> **Pause Safety**: every learning has reached a terminal state and durable routings have landed;
> `learnings.md` now holds only staging residue safe to archive/delete. Safe to stop. To resume:
> proceed to archival.

---

## Phase 7: Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI) in all three repos
- [ ] [AI] Verify the Knowledge Capture phase completed: every learning routed/backlogged/discarded;
      both safety gates satisfied; nothing silently dropped
- [ ] [AI] Verify the transient-log caveat is honored: nothing valuable depends on `learnings.md`
      surviving (everything kept was routed to a durable home)
- [ ] [AI] Move plan folder to `plans/done/`:
      `git mv plans/in-progress/plan-execution-knowledge-capture plans/done/2026-07-05__plan-execution-knowledge-capture`
      (use the completion date, not the creation date)
      — acceptance: folder now under `plans/done/2026-07-05__plan-execution-knowledge-capture/`
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date `2026-07-05`
- [ ] [AI] Update any other READMEs that reference this plan (e.g., `plans/README.md`)
- [ ] [AI] Commit the archival (the `learnings.md` scaffold moves with the plan):
      `chore(plans): move plan-execution-knowledge-capture to done` and push to `origin main`
- [ ] [AI] Replicate the archival move in `ose-primer` and `ose-infra` if those repos track this plan
      folder; otherwise note that only `ose-public` carries the plan doc
      — acceptance: each repo that tracks the plan folder has it under `plans/done/`

### Phase 7 Gate

> Terminal gate — the plan is complete when all checks pass.

- [ ] [AI] `test -d plans/done/2026-07-05__plan-execution-knowledge-capture` exits 0
- [ ] [AI] `plans/in-progress/README.md` no longer lists this plan; `plans/done/README.md` lists it
- [ ] [AI] Archival commit pushed and CI-green

> **Pause Safety**: the plan is archived, all three repos carry the change, and every learning reached
> a durable home. Terminal state. To resume: nothing — the plan is done. Prompt the user to delete the
> three worktrees.
