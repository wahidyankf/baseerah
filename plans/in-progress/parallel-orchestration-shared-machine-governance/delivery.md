# Delivery — Parallel-Orchestration & Shared-Machine Governance

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/parallel-orchestration-shared-machine-governance/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree parallel-orchestration-shared-machine-governance
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting
the worktree after the plan is archived and pushed. Propagation phases (6, 7) provision their own
per-repo worktrees in `ose-primer` and `ose-infra`.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Parallelization Model

This plan uses the **N+1 model it introduces**: `1 main thread + N background agents = N+1 total`,
**default N=3** (4 total, chosen to bound token/compute-budget burn), N adjustable along the way.

**Dependency DAG** (nodes = phases/steps; edges = blocks/blockedBy). Independent nodes fan out up to
N; dependent nodes serialize; **cleanup (Phase 9) is the terminal node depending on all delivery
nodes**: Phases 0-5 form a dependent serial spine (each phase builds the source of truth the next
needs); Phase 5's ose-public merge fans out into two independent parallel branches, Phase 6
(ose-primer) and Phase 7 (ose-infra); both join back at Phase 8 (Knowledge Capture), which feeds
Phase 9 (Cleanup), the terminal node. See the authoritative Mermaid rendering of this flow in
[`tech-docs.md` §Phase / delivery flow (gated progression)](./tech-docs.md#phase--delivery-flow-gated-progression)
— this section is the prose summary only, not a second diagram, to avoid two diverging
representations of the same flow.

Concurrency map:

- **Serial spine**: Phases 0→1→2→3→4→5 are dependency-ordered (each builds the source of truth the
  next needs) — they run one at a time.
- **Parallel branch**: Phase 6 (ose-primer) and Phase 7 (ose-infra) are genuinely independent once
  Phase 5 merges → they fan out as **2 parallel worktrees** (dogfooding N+1).
- **Within a phase**: independent doc edits across distinct files may run in parallel up to N=3
  background agents; dependent edits (e.g. a convention file then its index link) stay serial.

**Background-slot preference**: fill the background slots up to N and keep the **main thread vacant
and responsive** (orchestrator, not long-running worker) — but bounded by the DAG: never force
parallelism onto dependent nodes just to raise utilization.

**Status cadence**: while task-list items are active, the orchestrator updates the user every **3-5
minutes (not faster)** — no update-storming on micro-events.

**Adjust N** down under budget/runner/disk pressure on the shared machine; never silently
self-promote beyond the declared N without cause. Keep the 3-min mtime poll / 30-min stuck /
`TaskStop`+relaunch guidance.

**Per-phase PR + feature-flag structure (Delta 10)**: this plan decomposes delivery so independent
DAG nodes can land as separate PRs — the propagation branch (Phases 6 + 7) is the concrete example:
`ose-primer` and `ose-infra` each get **their own worktree → branch → PR** (strict **1-PR ↔
1-worktree**), reviewed and merged in parallel. Phases 0-5 form one dependent chain (the source of
truth must land first) and therefore stay a single ose-public PR — a genuine DAG dependency is NOT
force-split. Feature flags are not applicable to this docs/governance change (no runtime code to keep
dark), but the plan encodes the flag rule into governance for future code-bearing plans.

## Delivery Mode: worktree-to-pr

Per-repo worktree + draft PR across `ose-public` → `ose-primer` → `ose-infra`. `ose-public` is
authored and merged first as the source of truth, then `ose-primer` and `ose-infra` are propagated in
parallel. Each repo's PR runs the **PR-Review Maker→Fixer Cycle** (default 3 sequential CI-gated
cycles via `pr-review-maker` → `pr-review-fixer`) before the `[HUMAN]` merge. Per the maintainer's
standing preference, `[AI]` may auto-merge once the hardened merge preconditions hold (3 review
cycles + branch up-to-date with latest `origin/main` via non-destructive forward update + all gates
green). Git-mechanical steps (worktree add, commit, push to the PR branch, worktree remove) are `[AI]`.
This `[AI]`-auto-merge deviation from the mode's default `[HUMAN]`-merge requirement is a documented,
authorized exception — see **DD-10** in `tech-docs.md` §Design decisions for the rationale, the
authorizing context, and its explicit non-precedential scope.

> **Plan-doc authoring vs plan execution (distinct delivery paths)**: this `worktree-to-pr` mode
> governs the plan's **execution** (the governance/config edits it applies). The **plan-doc artifacts
> themselves** (this plan's `README.md`/`brd.md`/`prd.md`/`tech-docs.md`/`delivery.md`/`learnings.md`
> and related `.md` edits) are authored on the **primary checkout and committed + pushed directly to
> `origin main`** — docs-only `.md` changes fall in the "known-safe direct push" category, so no
> worktree/PR is needed for the authoring artifacts. This is the working example of the same principle
> behind the pure-schedule main-CI decision (see §Delivery Mode rationale in `tech-docs.md`). This
> split — and its consequence that Plan Archival lands via direct push after all three PRs merge,
> rather than inside any one delivering PR — is a documented, authorized deviation for this specific
> tri-repo-propagation plan: see **DD-11** in `tech-docs.md` §Design decisions for the rationale,
> the authorizing context, and its explicit non-precedential scope.

## Guardrails (this plan obeys its own new rules)

- **Non-destructive git only**: no `git reset --hard`, `git checkout -f`, `git clean -fd`,
  `git branch -D` on shared branches, force-push, shared-branch history rewrite,
  `git worktree remove --force` on worktrees you did not create, work-swallowing `git stash`, or
  shared-object-store pruning. Operate only within this plan's own worktrees.
- **Explicit-path staging**: stage named paths only; never `git add -A`.
- **rhino-cli byte-identity**: do NOT touch `apps/rhino-cli/**` or the rhino gherkin tree
  (`specs/apps/rhino/behavior/rhino-cli/gherkin/**`). If any rhino-cli surface is unavoidably touched,
  it MUST remain byte-identical across all three repos.
- **Self-scoped cleanup**: the final Cleanup gate removes only this plan's own worktrees and
  self-created artifacts, verified-not-in-use; never the shared cargo `target/` or any shared cache.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Provision/enter the worktree `worktrees/parallel-orchestration-shared-machine-governance/`
      from latest `origin/main` — acceptance: `git -C worktrees/parallel-orchestration-shared-machine-governance status` shows a clean tree on a fresh branch off `origin/main`
- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift
- [ ] [AI] Record the pre-change grep baseline of the old cap phrasing:
      `grep -rn "cap at 2\|3 total\|Cap at Three\|stricter cap of 2\|2 concurrent background\|capped at \*\*3 concurrent\*\*" AGENTS.md CLAUDE.md repo-governance/`
      — acceptance: hit list captured in `learnings.md` as the "surfaces to update" baseline
- [ ] [AI] Establish the docs quality baseline: `npm run lint:md:fix` then
      `npx nx affected -t lint` — acceptance: baseline pass/fail recorded; preexisting failures documented
- [ ] [AI] Resolve all preexisting failures before proceeding — acceptance: no preexisting failures remain

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] The old-cap grep baseline is recorded in `learnings.md` and markdown lint baseline is clean

> **Pause Safety**: only the local toolchain and the grep baseline were established — no governance
> edits exist yet. Safe to stop indefinitely. To resume: re-run the grep baseline and confirm it
> still matches.

---

## Phase 1: N+1 Parallel-Orchestration Model (ose-public)

> _Suggested executor: `repo-rules-maker`_

- [ ] [AI] Edit `AGENTS.md` §Agent Workflow Orchestration (lines ~264-266): replace
      "capped at 3 concurrent … background agents cap at 2 (never more), for 3 total including the
      main thread" with the N+1 model — "1 main thread + N background agents = N+1 total; default
      N=3 (4 total); N adjustable per-plan and along the way; never silently self-promote beyond the
      declared N; keep mtime/staleness relaunch guidance" — acceptance: `grep -n "N+1\|N background\|default N=3" AGENTS.md` returns the new text and the old numbers are gone
- [ ] [AI] Edit `repo-governance/development/agents/agent-workflow-orchestration.md` §Parallelism
      Budget (lines ~111-117): rewrite to the N+1 model with default N=3, adjustable up/down; add the
      same-machine assumption sentence — acceptance: section states N+1 + default N=3 + adjustable; no
      standing "two (2) concurrent background operations" fixed-cap assertion remains
- [ ] [AI] Edit `repo-governance/development/agents/subagent-orchestration.md` Standard 1 (lines
      ~73-93) and the anti-pattern examples (lines ~170-196): change the background cap from a fixed 2
      to N (default 3); keep Standards 2-4 (polling, stuck detection, chunk sizing, relaunch)
      unchanged — acceptance: `grep -n "default N\|N background" subagent-orchestration.md` present; the
      "cap is 2 background" standing assertions rewritten to N (default 3)
- [ ] [AI] Edit `repo-governance/development/practice/parallel-by-default.md` Standards 2 & 3 (lines
      ~74-86): unify the "cap at three" tool-batching cap and the "stricter cap of 2" subagent cap
      into a single adjustable N (default 3), with +1 = the main thread — acceptance: single N model
      documented; cross-links to subagent-orchestration + agent-workflow-orchestration updated
- [ ] [AI] Add the **default-N rationale** to `agent-workflow-orchestration.md` + `parallel-by-default.md`:
      N=3 defaults specifically to bound token/compute-budget burn; raising N is deliberate + justified
      (independent work + capacity + budget headroom); lower under budget/runner/disk pressure
      — acceptance: `grep -ni "token\|compute\|budget" agent-workflow-orchestration.md` returns the rationale
- [ ] [AI] Add the **DAG-first orchestration** rule to `agent-workflow-orchestration.md` +
      `parallel-by-default.md`: every non-trivial task list AND delivery checklist declares a dependency
      DAG (nodes=tasks/items, edges=blocks/blockedBy); independent nodes parallelize up to N, dependent
      nodes serialize; the DAG's independent-node width is the fan-out (capped at N); cleanup is the
      terminal node — acceptance: `grep -ni "DAG\|blockedBy\|dependency graph" agent-workflow-orchestration.md` present
- [ ] [AI] Add the **background-slot preference** to `parallel-by-default.md` +
      `subagent-orchestration.md` + `agent-workflow-orchestration.md`: fill background slots up to N,
      keep the main thread vacant/responsive (orchestrator not worker), bounded by the DAG — never force
      parallelism onto dependent nodes — acceptance: `grep -ni "main thread.*vacant\|responsive\|background slot" parallel-by-default.md` present
- [ ] [AI] Add the **vendor-neutral, capability-gated** paragraph to `agent-workflow-orchestration.md`
      verbatim from `tech-docs.md §Cross-harness compatibility` (no vendor names, no numeric caps in the
      prose — per the Governance Vendor-Independence Convention): background-capable harnesses fan out to
      N per-worktree; non-capable harnesses walk the same DAG serially; delivery-safety rules apply
      identically in both modes — acceptance: paragraph present; `npx nx run rhino-cli:governance:vendor-audit-validation`
      (real Nx target, wraps `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor validate repo-governance/`, per `.github/workflows/main-ci.yml`'s `governance` job) reports no vendor leakage
- [ ] [AI] Update the **worktree-to-pr as parallelism mechanism** rationale in
      `agent-workflow-orchestration.md`: sharpen that the **PR** (not just the worktree) is the
      independent merge point — N parallel units → N PRs that review/gate/merge independently without
      blocking each other; each DAG leaf producing changes gets its own worktree + PR — cross-link
      [Delivery Mode](../../../repo-governance/conventions/structure/plans.md#delivery-mode)
      — acceptance: rationale names the PR as the enabler; link resolves
- [ ] [AI] Grep-sweep for any remaining stale numbers using the Phase 0 baseline command
      — acceptance: no unintended "cap at 2"/"3 total"/"stricter cap of 2" hits remain in ose-public

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx affected -t lint` and `npm run lint:md:fix` — exit 0, no markdown violations
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done --exclude apps/ayokoding-www/content --exclude apps/ose-www/content`
      (real invocation — mirrors `.husky/pre-push` and `.github/workflows/main-ci.yml`'s `md-links` job; there is no `rhino-cli:links:validation` Nx target) — exit 0 (no broken links from edited files)
- [ ] [AI] Grep sweep confirms the N+1 model replaced the old numbers across the four surfaces

> **Pause Safety**: the concurrency model is internally consistent across the four ose-public
> surfaces; conventions build and lint clean. Safe to stop. To resume: re-run the grep sweep + lint.

---

## Phase 2: No-Destructive-Git-Operations Convention (ose-public, NEW)

> _Suggested executor: `repo-rules-maker`_

- [ ] [AI] Create `repo-governance/development/workflow/no-destructive-git-operations.md` (sibling of
      `git-push-safety.md`) with: frontmatter, purpose, the same-machine assumption, the forbidden-op
      table (reset --hard, checkout -f/--force, clean -fd, branch -D on shared branches, force-push to
      shared branches, history rewrite on shared branches, worktree remove --force on others'
      worktrees, work-swallowing stash, shared-object-store pruning), the additive/own-worktree
      preference, explicit-path staging, principles/conventions cross-links, and a companion link to
      `git-push-safety.md` (remote side) — acceptance: file exists; `grep -c "reset --hard\|clean -fd\|git add -A" no-destructive-git-operations.md` ≥ 3
- [ ] [AI] Cross-link the new convention from the stage-explicit-paths guidance and from
      `git-push-safety.md` (bidirectional "see also") — acceptance: both files link each other; links resolve

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npm run lint:md:fix` and `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done --exclude apps/ayokoding-www/content --exclude apps/ose-www/content`
      (real invocation — mirrors `.husky/pre-push`; no `rhino-cli:links:validation` Nx target exists) — exit 0
- [ ] [AI] New convention exists and lists the full forbidden-op set

> **Pause Safety**: the new convention is a standalone, lint-clean file with resolving links; no index
> depends on it yet (wired in Phase 4). Safe to stop. To resume: re-run link validation.

---

## Phase 3: Worktree-and-Artifact Cleanup Convention (ose-public, NEW)

> _Suggested executor: `repo-rules-maker`_

- [ ] [AI] Create `repo-governance/development/workflow/worktree-and-artifact-cleanup.md` (teardown
      sibling of `worktree-setup.md`) with: frontmatter, purpose, the mandatory plan-end cleanup gate,
      the self-created-only + verify-not-in-use rules, the artifact taxonomy (`target/`, `dist/`,
      `.next/`, build caches), the HARD caveat that shared caches must never be deleted (naming the
      shared cargo `target/` from the `rust-cargo-target-dir-sharing` plan as the canonical example),
      and the "cleanup is itself non-destructive to others" rule — acceptance: file exists;
      `grep -ci "shared cargo\|verify\|not in use\|self-created" worktree-and-artifact-cleanup.md` ≥ 3
- [ ] [AI] Add the **five mandatory pre-removal checks** to `worktree-and-artifact-cleanup.md` (each
      grounded in a live 2026-07-19 incident, per tech-docs §Delta 5): (1) test merge state with
      `gh pr list --head <branch> --state all`, **never** `git merge-base --is-ancestor` — every PR
      here is squash-merged, so ancestry false-negatives on every merged branch; (2) `git status
--porcelain` the worktree and read any dirty diff, recovering content found nowhere else to
      `main` before removal — a merged PR does not imply an empty working tree; (3) check
      `git log origin/<branch>..<branch>` for unpushed commits; (4) always non-force
      `git worktree remove`, never `rm -rf`; (5) never remove a worktree this plan did not create
      without positive evidence it is idle — acceptance:
      `grep -nic "gh pr list\|is-ancestor\|non-force\|did not create" worktree-and-artifact-cleanup.md`
      returns ≥4
- [ ] [AI] Cross-link the cleanup convention to `worktree-setup.md`, `temporary-files.md` (build-artifact
      taxonomy), and `no-destructive-git-operations.md` — acceptance: links present and resolve

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npm run lint:md:fix` and `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done --exclude apps/ayokoding-www/content --exclude apps/ose-www/content`
      (real invocation — mirrors `.husky/pre-push`; no `rhino-cli:links:validation` Nx target exists) — exit 0
- [ ] [AI] Cleanup convention exists with the shared-cargo-target carve-out explicit

> **Pause Safety**: both new conventions exist and lint clean; the concurrency edits are stable. Safe
> to stop. To resume: re-run link validation on the two new files.

---

## Phase 4: Wiring, Config, Cross-Surface Sweep, Bindings & Indexes (ose-public)

> _Suggested executor: `repo-rules-maker`_ (config step: shell/YAML)

### 4a. AGENTS.md / CLAUDE.md / indexes

- [ ] [AI] Add the same-machine, concurrent-actors assumption to `AGENTS.md` §Agent Workflow
      Orchestration (one sentence) and cross-link the two new conventions — acceptance: `grep -n "same machine\|shared machine" AGENTS.md` present; both convention links resolve
- [ ] [AI] Update `AGENTS.md` §Agent Workflow Orchestration + §Git Workflow §Delivery Mode to add the
      DAG rule, background-slot preference, 3-5 min status cadence, PR-as-independent-merge-point, and
      the hardened merge preconditions (3 cycles + up-to-date-with-origin-main + gates green)
      — acceptance: `grep -n "DAG\|up-to-date with .*origin/main\|3-5 min" AGENTS.md` present
- [ ] [AI] Grep `CLAUDE.md` for any Claude-specific concurrency text
      (`grep -n "concurrent\|background agent\|cap" CLAUDE.md`); update to the N+1 model if present,
      else add nothing — acceptance: `CLAUDE.md` states no stale fixed cap; edits (if any) lint clean
- [ ] [AI] Add the two new conventions to `repo-governance/development/workflow/README.md` §Documents
      (link by name; respect Dynamic Collection References — no hardcoded counts) — acceptance: both
      links present in the Documents list and resolve
- [ ] [AI] Grep the agents/practice index READMEs for stale cap references and update if present
      (`grep -rn "cap at 2\|3 total" repo-governance/development/agents/README.md repo-governance/development/practice/README.md`)
      — acceptance: no stale numbers remain in those indexes

### 4b. Convention surfaces for the new orchestration behaviors

- [ ] [AI] Edit `repo-governance/development/practice/task-list-discipline.md`: add the **3-5 minute
      bounded status-update cadence** (while task-list items are active; not faster; no micro-event
      storming) — acceptance: `grep -ni "3-5\|status update\|cadence" task-list-discipline.md` present
- [ ] [AI] Edit `repo-governance/conventions/structure/plans.md`: document that `delivery.md` expresses
      phases/steps as a **DAG** + a `## Parallelization Model` section (which items are concurrent vs
      serial; cleanup = terminal node) — acceptance: `grep -ni "DAG\|Parallelization Model" plans.md` present
- [ ] [AI] Edit `repo-governance/workflows/pr/pr-review-quality-gate.md`: add the **hardened merge
      preconditions** — (a) 3 `pr-review-maker`→`pr-review-fixer` cycles, (b) branch up-to-date with
      latest `origin/main` at merge (non-destructive forward update if behind), (c) all gates green
      — acceptance: `grep -ni "up-to-date\|origin/main\|3 cycles" pr-review-quality-gate.md` present
- [ ] [AI] **Delta 12 — invert the merge default in its definitional home**: edit
      `repo-governance/conventions/structure/plans.md` §Delivery Mode so `[AI]` merge is the default
      once merge preconditions hold, and a `[HUMAN]` merge gate applies **only** where a plan's own
      step states it explicitly. State plainly that the **preconditions are unchanged — only the actor
      is** — acceptance:
      `grep -nic "\[AI\] merges\|only where.*explicitly\|only the actor" plans.md` returns ≥2 (the
      same command returns **0** against the current pre-edit file)
- [ ] [AI] Propagate the inverted default to `repo-governance/workflows/pr/pr-review-quality-gate.md`
      (merge-gate done-definition), `plan/plan-execution.md`, and `plan/plan-planning.md` — acceptance:
      `grep -rnic "\[HUMAN\] merge\|human merge" repo-governance/workflows/pr/pr-review-quality-gate.md repo-governance/workflows/plan/plan-execution.md repo-governance/workflows/plan/plan-planning.md`
      shows every surviving `[HUMAN]`-merge mention rewritten as the explicit opt-in, not the default
- [ ] [AI] Sweep every remaining hardcoded `[HUMAN]`-merge reference across `repo-governance/**`,
      `.claude/agents/**`, and `.claude/skills/**`:
      `grep -rniE "\[HUMAN\][^.]*merge" repo-governance .claude` — rewrite each as opt-in or delete if
      it merely restated the old default — acceptance: every hit is either an explicit per-plan opt-in
      or gone; the count is recorded in `learnings.md`
- [ ] [AI] Mark **DD-10 as dissolved-by-Delta-12** in `tech-docs.md` — retained as historical record of
      how the authorization arrived, no longer a deviation, since the default now matches it —
      acceptance:
      `sed -n '/^### DD-10/,/^### DD-11/p' tech-docs.md | grep -cic "dissolved by Delta 12"` returns ≥1
      (scoped to DD-10's own section, since Delta 12's prose already contains the word "dissolves"
      elsewhere in the file — a whole-file grep would pass vacuously)
- [ ] [AI] Add the **per-phase-PR + feature-flag + strict 1-PR↔1-worktree** planning-granularity rule
      (Delta 10) to `repo-governance/workflows/plan/plan-planning.md` and cross-reference from
      `repo-governance/conventions/structure/plans.md`: each applicable phase / independent DAG node
      lands as its own PR (one worktree → one branch → one PR → one node), feature-flag partial work
      merged-but-dark on `main`, inseparable dependent phases stay one PR (DAG governs) — acceptance:
      `grep -ni "feature flag\|one PR\|per-phase\|1-PR" plan-planning.md` present
- [ ] [AI] State in `plan-planning.md` how the `worktree-to-pr` default binds at each plan path:
      **creating/updating** a plan binds it as a **design obligation** (the authoring edit may push
      direct to `main`, but phases must be authored to be independently PR-able, and a plan that
      cannot be so decomposed records why in its `tech-docs.md`); **executing** a plan binds it as the
      actual delivery route. Introduce the **plan-docs-only** carve-out as a general convention in its
      own right (a change touching only `plans/**`, no `apps/`/`libs/` code, may push direct to
      `main`) — stated on its own footing, **not** derived from DD-11, which disclaims being a general
      precedent — acceptance:
      `grep -nic "design obligation\|independently PR-able\|plan-docs-only" plan-planning.md` returns
      ≥3 (the same command returns **0** against the current pre-edit file — verified live, so the
      clause discriminates a done step from an undone one)
- [ ] [AI] Make **per-phase merging** explicit (not merely per-phase PR _opening_) in
      `plan-planning.md` + `plan-execution.md`: each phase PR is opened **and merged** as that phase
      completes and is **not** held for a batch merge at plan end. State the merge actor correctly —
      `[HUMAN]` by the unchanged Delivery Mode default, `[AI]` only where the plan carries an explicit
      maintainer auto-merge authorization — citing DD-10 as an **instance**, never as authority for a
      general `[AI]`-merge rule — acceptance:
      `grep -nic "batch merge\|merge actor\|auto-merge authorization" plan-planning.md plan-execution.md`
      returns ≥3 (returns **0** against both current pre-edit files)
- [ ] [AI] Encode the **feature-flag default + escape + removal** rule in `plan-planning.md`:
      flagging is the default; a phase lands unflagged **only** when it ships no user-reachable
      behaviour change (pure docs / governance / refactor / test-only) and the step names which
      exemption applies; every flag introduced carries a named **removal step** in the plan's final
      phase — acceptance: `grep -nic "unflagged\|user-reachable\|flag removal step" plan-planning.md`
      returns ≥3 (returns **0** against the current pre-edit file)
- [ ] [AI] Reflect the 1-PR↔1-worktree cleanup tie in `plan-execution.md` (the worktree is the unit
      cleaned up when its PR lands) — acceptance: `grep -ni "one worktree\|per-PR\|feature flag" plan-execution.md` present

### 4c. Cross-surface sweep (agents / skills / workflows)

- [ ] [AI] Grep-discover every agent/skill/workflow referencing the old cap numbers, orchestration,
      worktrees, git-safety, or cleanup:
      `grep -rln "cap at 2\|3 total\|2 background\|stricter cap of 2\|max-concurrency\|background agent\|worktree\|git-safety\|cleanup" .claude/agents .claude/skills repo-governance/workflows`
      — acceptance: candidate file list recorded in `learnings.md` (expect ≥20 workflow hits from
      `max-concurrency` alone, plus all 7 `plan/*` files)

#### 4c-i. ALL SEVEN `repo-governance/workflows/plan/*` files (one checkbox each)

- [ ] [AI] `repo-governance/workflows/plan/README.md` — update the plan-workflow index to reflect the
      N+1/DAG model and link the two new conventions — acceptance: `grep -ni "N+1\|DAG" plan/README.md` present; new convention links resolve
- [ ] [AI] `repo-governance/workflows/plan/plan-execution.md` — N+1 fan-out, DAG ordering, 1-PR↔1-worktree
      cleanup tie, no-destructive-git, self-scoped cleanup — acceptance: `grep -ni "N+1\|DAG\|one worktree" plan-execution.md` present; no stale "cap at 2 / 3 total"
- [ ] [AI] `repo-governance/workflows/plan/plan-planning.md` — per-phase PR + feature flags + strict
      1-PR↔1-worktree (Delta 10) — acceptance: `grep -ni "feature flag\|per-phase\|1-PR" plan-planning.md` present
- [ ] [AI] `repo-governance/workflows/plan/plan-quality-gate.md` — align the `max-concurrency` frontmatter
      default/wording with N+1 **and** add the hardened merge preconditions (3 cycles + up-to-date with
      `origin/main` + all gates green) to its Delivery-Mode done-definition section — acceptance:
      `grep -ni "max-concurrency\|up-to-date\|3 cycles" plan-quality-gate.md` reflects the new model
- [ ] [AI] `repo-governance/workflows/plan/multi-plans-execution.md` (**most affected** — governs running
      multiple plans at once): adopt N+1, background-slot-preference/main-vacant, DAG-first ordering,
      3-5 min status cadence, 1-PR↔1-worktree; **supersede** its "cap 3 concurrent / background cap 2
      never more" language — acceptance: `grep -n "cap 3\|cap at 2\|never more\|3 total" multi-plans-execution.md` returns nothing; N+1/DAG/cadence text present
- [ ] [AI] `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md` — worktree-to-PR default,
      per-phase PR + feature flags, no-destructive-git, self-scoped cleanup, parallel propagation shape
      (ose-public → ose-primer/ose-infra) — acceptance: `grep -ni "worktree-to-pr\|parallel propagation\|cleanup" plan-multi-repo-parity-planning.md` present
- [ ] [AI] `repo-governance/workflows/plan/plan-multi-repo-parity-planning-and-execution.md` — same
      alignment as above for the execution half — acceptance: same grep clean on this file

#### 4c-ii. Repo-wide `max-concurrency` frontmatter (20 files)

- [ ] [AI] Enumerate: `grep -rl "max-concurrency" repo-governance/workflows/ | sort`
      — acceptance: 20 files listed and recorded in `learnings.md`
- [ ] [AI] Align the `max-concurrency` default/wording with the N+1 model across the 19 files carrying
      `default: 2` — including `workflows/README.md` (which documents "Parallel execution limit -
      default: 2") and `meta/workflow-identifier.md` (which defines the input schema) — acceptance:
      each updated file's `max-concurrency` description references the N+1 model, not a bare fixed 2
- [ ] [AI] **PRESERVE** `web/web-ux-test-fixing-planning.md` at `Default 1` — the three testers run
      SEQUENTIALLY by design (a genuine DAG serialization point, NOT a stale cap); document _why_ it
      stays 1, citing "DAG governs — never force parallelism onto dependent nodes" — acceptance:
      file still reads `Default 1` **and** carries the new justification sentence
- [ ] [AI] `repo-governance/workflows/repo/repo-dependency-bump-planning.md` — align its prose-level
      concurrency cap ("one agent per ecosystem batch") + Subagent-Orchestration cross-link with N+1
      — acceptance: `grep -ni "N+1\|cap" repo-dependency-bump-planning.md` reflects the new model
- [ ] [AI] `repo-governance/workflows/pr/pr-review-quality-gate.md` — align its `max-concurrency` with
      N+1 (merge preconditions already added in 4b) — acceptance: no stale fixed-2 assertion remains

- [ ] [AI] Update every discovered `.claude/agents/*.md` and `.claude/skills/*/SKILL.md` that carries
      stale orchestration text to the N+1/DAG/main-vacant model — acceptance: re-run the 4c grep; only
      intentional historical references remain, each justified inline
- [ ] [AI] Completeness gate: invoke `repo-rules-checker` + `repo-harness-compatibility-checker` over
      the swept files — acceptance: no CRITICAL/HIGH stale-reference or vendor-leak findings unresolved

### 4d. main-ci schedule (ose-public) + bindings

> **Why this is safe** (record in the commit body): `main-ci.yml` runs essentially the **same checks**
> as PR CI and the pre-commit/pre-push hooks — only the **scope** differs (`--all` vs `affected`). The
> hooks are auto-installed on every `npm install` (`"prepare": "husky"`), which worktree-setup
> mandates, so every push already cleared the affected-scope gates locally; PR CI re-runs them at
> affected scope before merge; main-ci is the periodic whole-repo `--all` sweep for cross-project
> drift. Three overlapping layers → no per-push trigger needed; up-to-~6h lag on `main` is an accepted
> tradeoff (direct-push modes carry only known-safe docs-only edits).

- [ ] [AI] Edit `.github/workflows/main-ci.yml`: remove `push: branches: [main]` and set the trigger to
      the 4×/day schedule + dispatch:

  ```yaml
  on:
    schedule:
      - cron: "0 5,11,17,23 * * *" # 06:00/12:00/18:00/00:00 (next day) WIB (UTC+7)
    workflow_dispatch:
  ```

  — acceptance: `grep -n "schedule\|workflow_dispatch" .github/workflows/main-ci.yml` present and
  `grep -n "push:" .github/workflows/main-ci.yml` returns nothing; `actionlint .github/workflows/main-ci.yml` exits 0
  - _Suggested executor: `ci-fixer`_

### 4e. Platform-binding catalog: Amazon Q Developer → Kiro CLI succession

- [ ] [AI] Edit `docs/reference/platform-bindings.md`: update the "Amazon Q Developer" entry to record
      the Q-Developer-CLI → Kiro-CLI succession — sunset dates (new-signup block 2026-05-15, models
      Kiro-only 2026-05-29, IDE-plugin EOS 2027-04-30) and Kiro capabilities (native DAG task-graphs,
      up to 4 subagents, worktree isolation, `q`/`q chat` preserved, `~/.aws/amazonq`→`~/.kiro`
      auto-migrated) — acceptance: `grep -ni "Kiro" docs/reference/platform-bindings.md` present; no
      "Amazon Q Developer" mention reads as evergreen
- [ ] [AI] Grep every other "Amazon Q Developer" mention and update consistently
      (`grep -rln "Amazon Q" AGENTS.md docs/reference/`); confine vendor-accurate detail to
      platform-binding surfaces (NOT `repo-governance/` prose) — acceptance: `AGENTS.md` §Platform
      Binding Examples reflects the succession; `repo-governance/` prose remains vendor-neutral
  - _Suggested executor: `repo-harness-compatibility-fixer`_

### 4f. Surface-conditional UI / API tester gates + NEW `workflows/api/` (Delta 11)

> **Verified gap**: `repo-governance/ui/` exists (`README.md` + `ui-quality-gate.md`), but
> `repo-governance/workflows/api/` **does not exist** — while `.claude/agents/api-exploratory-tester.md`
> DOES. This sub-block creates the missing API half and wires the conditional rule.
>
> **Do not conflate the three UI-related gates**: `plan-checker` **Step 5k** gates the UI **design
> funnel** in `prd.md` (pre-build); `ui/ui-quality-gate.md` gates the **built components** via
> `swe-ui-checker`/`swe-ui-fixer` (static, no browser); `web/web-ux-test-fixing-planning.md` gates the
> **running UI** via the EWT/UWT/DWT triad. Complementary, never substitutes.

- [ ] [AI] Create `repo-governance/workflows/api/api-quality-gate.md` _New file_, modelled on
      `repo-governance/workflows/ui/ui-quality-gate.md`: YAML frontmatter with `name: api-quality-gate`,
      `title`, `goal`, `termination`, `inputs` (`scope`, `mode` enum `[lax, normal, strict, ocd]`,
      `min-iterations`, `max-iterations` default 7, **`max-concurrency` aligned with the §4c-ii N+1
      value**), `outputs` (`final-status`, `iterations-completed`, `final-report` pattern
      `generated-reports/api-exploratory-tester__*__audit.md`); body carries an **Execution Mode**
      section (Agent Delegation preferred / Manual Orchestration fallback) and a **tester-driven
      find→fix→re-test loop** — `api-exploratory-tester` emits `AET-###` findings against a live
      REST/GraphQL endpoint with the contract (OpenAPI 3.x / GraphQL SDL) as ground truth, the
      appropriate `swe-*-dev` agent fixes, the tester re-runs until the defect set is empty
      — acceptance: `test -f repo-governance/workflows/api/api-quality-gate.md` exits 0 and
      `grep -c "max-concurrency" repo-governance/workflows/api/api-quality-gate.md` returns ≥ 1
  - _Suggested executor: `repo-workflow-maker`_
  - _Honest shape note_: there is **no** `api-checker`/`api-fixer` agent pair — do NOT author this as a
    checker/fixer clone of `ui-quality-gate.md`; it is a tester-driven loop. Citing a non-existent
    agent is anti-pattern AP-7.
  - _Naming_: follows the
    [Workflow Naming Convention](../../../repo-governance/conventions/structure/workflow-naming.md)
- [ ] [AI] Create `repo-governance/workflows/api/README.md` _New file_ mirroring
      `repo-governance/workflows/ui/README.md`: frontmatter (`title: "API Workflows"`, `description`,
      `category: explanation`, `subcategory: workflows/api`, `tags`, `created`), an "Available
      Workflows" table row for API Quality Gate naming `api-exploratory-tester`, and a "Related
      Documentation" section — acceptance: `test -f repo-governance/workflows/api/README.md` exits 0
      and the table links `./api-quality-gate.md`
  - _Suggested executor: `repo-workflow-maker`_
- [ ] [AI] Register the new category in `repo-governance/workflows/README.md` alongside `ui/`
      — acceptance: `grep -n "workflows/api\|api-quality-gate" repo-governance/workflows/README.md`
      returns ≥ 1 hit; no hardcoded collection counts introduced (Dynamic Collection References)
- [ ] [AI] Validate the two new workflow files with `repo-workflow-checker`
      — acceptance: no CRITICAL/HIGH findings unresolved
  - _Suggested executor: `repo-workflow-checker`_
- [ ] [AI] State the **surface-conditional gate rule** in
      `repo-governance/workflows/plan/plan-execution.md` and
      `repo-governance/workflows/plan/plan-planning.md`: UI-bearing plan → run BOTH UI gates
      (`ui/ui-quality-gate.md` static + `web/web-ux-test-fixing-planning.md` running triad);
      API/BE-bearing plan → run `api/api-quality-gate.md`; both → both; neither → **the plan MUST
      state the exemption explicitly in `tech-docs.md`**, never leave it implicit. Bind at BOTH points:
      during plan creation/update/execution, AND as a merge precondition — acceptance:
      `grep -c "api-quality-gate" repo-governance/workflows/plan/plan-execution.md repo-governance/workflows/plan/plan-planning.md`
      returns ≥ 1 for each file
- [ ] [AI] Add the explicit **three-way distinction** paragraph (5k design funnel / `ui-quality-gate`
      built components / triad running UI — complementary, not contradictory) to the same two plan
      workflow files so nobody treats one gate as substituting for another — acceptance:
      `grep -ni "5k" repo-governance/workflows/plan/plan-execution.md` returns ≥ 1 hit
- [ ] [AI] Add the conditional gate to `repo-governance/workflows/pr/pr-review-quality-gate.md` as
      **merge precondition clause (d)**, alongside the Delta 8 clauses (3 cycles /
      up-to-date-with-`origin/main` / all gates green) — acceptance:
      `grep -ni "api-quality-gate\|surface-conditional" repo-governance/workflows/pr/pr-review-quality-gate.md`
      returns ≥ 1 hit
- [ ] [AI] Cross-link Rule 15 (web triad) and Rule 16 (AET) in
      `repo-governance/development/quality/user-facing-delivery-hardening.md` to the new conditional
      rule and the new `api/` workflow, so the two surfaces agree rather than drift — acceptance:
      `grep -n "workflows/api" repo-governance/development/quality/user-facing-delivery-hardening.md`
      returns ≥ 1 hit
  - _Suggested executor: `repo-rules-maker`_

- [ ] [AI] Regenerate the platform bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/**` and `.amazonq/**` updated to reflect the new text; no hand-edits
- [ ] [AI] Run the harness-binding sync check: `npm run validate:sync` (real npm script, wraps
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- harness sync validate`
      per `package.json:34`; there is no `rhino-cli:validate:sync` Nx target) — acceptance: exits 0
- [ ] [AI] Run the vendor-audit check: `npx nx run rhino-cli:governance:vendor-audit-validation`
      (real Nx target, wraps `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor validate repo-governance/`)
      — acceptance: exits 0; no vendor-specific content leaked into governance files
- [ ] [AI] Invoke `repo-rules-checker` over the changed governance files — acceptance: no CRITICAL/HIGH findings unresolved

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npm run generate:bindings` exited 0 and binding artifacts are in sync (no uncommitted drift beyond intended edits)
- [ ] [AI] `actionlint .github/workflows/main-ci.yml` exits 0; the trigger is schedule + `workflow_dispatch` only (no `push:`)
- [ ] [AI] The 4c completeness grep returns no unjustified stale orchestration reference across agents/skills/workflows
- [ ] [AI] **Repo-wide superseded-cap proof**: `grep -rn "cap at 2\|cap of 2\|cap 3 concurrent\|3 total\|2 background\|stricter cap of 2\|never more" repo-governance/ AGENTS.md CLAUDE.md .claude/agents .claude/skills`
      returns **zero** hits (or only hits explicitly annotated as superseded-historical) — proves no stale
      cap survives in ANY workflow, convention, agent, or skill doc
- [ ] [AI] All SEVEN `repo-governance/workflows/plan/*` files updated:
      `ls repo-governance/workflows/plan/` lists 7 files and each appears in this phase's completed 4c-i checkboxes
- [ ] [AI] `grep -rl "max-concurrency" repo-governance/workflows/ | wc -l` returns 21 (the 20 preexisting + the new `api/api-quality-gate.md`), and `web/web-ux-test-fixing-planning.md` still reads
      `Default 1` with its new justification sentence
- [ ] [AI] **Delta 11 — new `api/` workflow exists and is registered**:
      `test -f repo-governance/workflows/api/api-quality-gate.md && test -f repo-governance/workflows/api/README.md`
      exits 0, and `grep -c "api-quality-gate" repo-governance/workflows/README.md` returns ≥ 1
- [ ] [AI] **Delta 11 — conditional gate rule wired at both binding points**:
      `grep -l "api-quality-gate" repo-governance/workflows/plan/plan-execution.md repo-governance/workflows/plan/plan-planning.md repo-governance/workflows/pr/pr-review-quality-gate.md`
      lists all three files
- [ ] [AI] **Delta 11 — three-way distinction stated, not conflated**: the 5k / `ui-quality-gate` /
      web-triad distinction paragraph is present in `plan-execution.md` — acceptance:
      `grep -ni "ui-quality-gate" repo-governance/workflows/plan/plan-execution.md` returns ≥ 1 hit
- [ ] [AI] `npx nx affected -t lint` + `npm run lint:md:fix` + link validation — exit 0
- [ ] [AI] `repo-rules-checker` + `repo-harness-compatibility-checker` report no unresolved CRITICAL/HIGH findings

> **Pause Safety**: ose-public governance + config are complete, consistent, lint-clean, bindings
> synced, checker-green, and no stale orchestration reference remains. Safe to stop. To resume: re-run
> `generate:bindings` + the 4c grep + `repo-rules-checker`.

---

## Phase 5: ose-public PR Review Cycle & Merge (source of truth)

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected lint: `npx nx affected -t lint`
- [ ] [AI] Run markdown lint: `npm run lint:md:fix`
- [ ] [AI] Run link + mermaid + headings validation (real invocations — no `rhino-cli:links:validation`,
      `rhino-cli:mermaid:validation`, or `rhino-cli:headings:hierarchy-validation` Nx targets exist;
      these are raw `cargo run` subcommands, per `.husky/pre-push` and
      `.github/workflows/main-ci.yml`'s `markdown-per-file`/`md-links` jobs):
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done --exclude apps/ayokoding-www/content --exclude apps/ose-www/content`,
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md mermaid validate --exclude apps/rhino-cli/tests/fixtures --exclude plans/done`,
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md heading-hierarchy validate`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes
- [ ] [AI] Verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (Root Cause Orientation). Commit preexisting fixes separately with appropriate conventional commit
> messages.

### Commit Guidelines

- [ ] [AI] Commit thematically with Conventional Commits (e.g., `docs(governance): adopt N+1
parallel-orchestration model`, `docs(governance): add no-destructive-git-operations convention`,
      `docs(governance): add worktree-and-artifact-cleanup convention`, `chore(bindings): regenerate
opencode + amazonq`) — acceptance: separate cohesive commits; no `git add -A` (explicit paths only)

### PR & Post-Push CI Verification

- [ ] [AI] Commit and push to origin `<pr-branch>` and open a draft PR against `main`
      — acceptance: PR created; CI triggered
- [ ] [AI] Monitor ALL GitHub Actions workflows (poll every 2 min; one `gh run view --json status,conclusion` per wakeup) — acceptance: all checks green
- [ ] [AI] If any CI check fails, fix root cause and push a follow-up commit; repeat until green

### PR-Review Maker→Fixer Cycle (default 3, CI-gated)

- [ ] [AI] Cycle 1: `pr-review-maker` reviews via the GitHub Reviews API → `pr-review-fixer` applies
      fixes and pushes → CI green — acceptance: review comments addressed; CI green
- [ ] [AI] Cycle 2: `pr-review-maker` → `pr-review-fixer` → CI green — acceptance: no new HIGH findings
- [ ] [AI] Cycle 3: `pr-review-maker` → `pr-review-fixer` → CI green — acceptance: clean review; CI green
- [ ] [AI] **Hardened merge preconditions** (Delta 8): before merge confirm ALL — (a) 3 review cycles
      complete, (b) branch **up-to-date with latest `origin/main`** (if behind, bring forward
      non-destructively: `git fetch origin && git merge --ff-only origin/main` or a forward merge — NEVER
      `reset --hard`/force), (c) all PR quality gates green, **(d) the Delta 11 surface-conditional
      tester gates have been run and their defect findings resolved** — for THIS PR the surface is
      neither UI nor API, so record the explicit exemption in the PR description rather than leaving
      it implicit — acceptance: `gh pr view --json mergeStateStatus` shows the branch current and
      mergeable, and the PR body contains the Delta-11 gate line (run-and-resolved, or explicit exempt)
- [ ] [AI] Merge the ose-public PR to `main` once the hardened preconditions hold (maintainer's standing
      `[AI]` auto-merge preference for this plan — documented deviation, see **DD-10** in `tech-docs.md`
      §Design decisions) — acceptance: PR merged
- [ ] [AI] Since `main-ci.yml` is now schedule-only, trigger a confirmation run via
      `gh workflow run main-ci.yml` (or `workflow_dispatch`) and verify green — acceptance: dispatched main-ci run concludes success

### Phase 5 Gate

> All checks below must pass before starting Phases 6 & 7.

- [ ] [AI] ose-public PR merged; the three review cycles completed; branch was up-to-date with `origin/main` at merge
- [ ] [AI] A `workflow_dispatch` main-ci run on `main` concluded green (main-ci no longer auto-runs on push)
- [ ] [AI] Post-merge grep on `main` confirms the N+1 model + two new conventions are present

> **Pause Safety**: ose-public is the merged source of truth; primer/infra not yet touched. Safe to
> stop. To resume: checkout `main`, confirm the governance blocks, then start propagation.

---

## Phase 6: Propagate to ose-primer (parallel with Phase 7)

> Runs in a dedicated `ose-primer` worktree, in parallel with Phase 7 (dogfooding N+1: 2 parallel units).

- [ ] [AI] **Confirm the sibling's repo topology BEFORE anything else** —
      `git -C /Users/wkf/ose-projects/ose-primer rev-parse --is-bare-repository`
      — acceptance: prints `true`. **`ose-primer` is a BARE repo** (verified 2026-07-19): it has no
      top-level working tree, so `git -C /Users/wkf/ose-projects/ose-primer status` fails with
      `fatal: this operation must be run in a work tree`. All file work happens inside a worktree.
      If this prints `false`, the topology changed — STOP and re-derive the commands below rather
      than assuming. See [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md)
      §Sibling-Repo Relative Paths From Inside a Worktree, which records a real prior incident of
      silent stale-content propagation in a structurally identical tri-repo plan.
- [ ] [AI] Fetch and provision the worktree at the repo-local `worktrees/<name>/` path:
      `git -C /Users/wkf/ose-projects/ose-primer fetch origin main` then
      `git -C /Users/wkf/ose-projects/ose-primer worktree add worktrees/parallel-orchestration-shared-machine-governance -b parallel-orchestration-shared-machine-governance origin/main`
      — acceptance: `git -C /Users/wkf/ose-projects/ose-primer worktree list` shows the new worktree
      at `/Users/wkf/ose-projects/ose-primer/worktrees/parallel-orchestration-shared-machine-governance`, and
      `git -C <primer-worktree> rev-parse HEAD` equals `git -C /Users/wkf/ose-projects/ose-primer rev-parse origin/main`
      (proves it is branched from the LATEST origin/main, not a stale local ref)
- [ ] [AI] Set `<primer-worktree>` = `/Users/wkf/ose-projects/ose-primer/worktrees/parallel-orchestration-shared-machine-governance`
      for every subsequent step in this phase; run `npm install && npm run doctor -- --fix` **inside
      that worktree** (`cd` into it — do not rely on the shell's inherited working directory)
      — acceptance: `git -C <primer-worktree> status --porcelain` is empty; toolchain converged
- [ ] [AI] Apply the identical rule text from ose-public: N+1 + DAG + background-slot preference +
      status cadence + PR-merge preconditions concurrency edits, the two new convention files, the
      same-machine assumption, the vendor-neutral capability-gated paragraph, and index/workflow wiring
      — acceptance: `diff` of the governance blocks vs. merged ose-public shows only path-relative
      differences, no substantive divergence
- [ ] [AI] Apply the swept agents/skills/workflows updates to match ose-public — **ALL SEVEN**
      `workflows/plan/*` files (`README.md`, `plan-execution.md`, `plan-planning.md`,
      `plan-quality-gate.md`, `multi-plans-execution.md`, `plan-multi-repo-parity-planning.md`,
      `plan-multi-repo-parity-planning-and-execution.md`) **plus** the repo-wide `max-concurrency` set
      (preserving `web-ux-test-fixing-planning.md` at `Default 1`) — acceptance: the repo-wide
      superseded-cap grep returns zero hits in the ose-primer worktree
- [ ] [AI] Port the Delta 11 surface-conditional gate: create ose-primer
      `repo-governance/workflows/api/api-quality-gate.md` + `api/README.md` (byte-equivalent modulo
      path-relative links), register `api/` in `workflows/README.md`, and wire the rule into
      `plan/plan-execution.md`, `plan/plan-planning.md`, `pr/pr-review-quality-gate.md`, and
      `development/quality/user-facing-delivery-hardening.md` — acceptance:
      `test -f repo-governance/workflows/api/api-quality-gate.md` exits 0 and the three wiring files
      each contain `api-quality-gate`
  - _Suggested executor: `repo-workflow-maker`_
- [ ] [AI] Edit ose-primer `.github/workflows/main-ci.yml`: same schedule-only trigger
      (`cron: "0 5,11,17,23 * * *"` + `workflow_dispatch`; remove `push`) — acceptance:
      `actionlint` exits 0; no `push:` trigger remains
- [ ] [AI] Regenerate bindings: `npm run generate:bindings`; run link/markdown/vendor-audit gates
      — acceptance: exit 0; bindings synced
- [ ] [AI] Confirm no `apps/rhino-cli/**` surface changed (byte-identity guardrail):
      `git -C <primer-worktree> status --porcelain apps/rhino-cli` — acceptance: empty output
- [ ] [AI] Commit with explicit paths (never `git add -A` — the sibling repos carry unrelated WIP):
      `git -C <primer-worktree> add <explicit paths> && git -C <primer-worktree> commit`
      — acceptance: `git -C <primer-worktree> status --porcelain` shows no unintended files staged
- [ ] [AI] Push to the ose-primer PR branch: `git -C <primer-worktree> push origin <branch>`
      — acceptance: push succeeds; pre-push gates exit 0
- [ ] [AI] Open the draft PR: `gh pr create --repo <ose-primer> --draft`
      — acceptance: PR URL returned; PR shows as draft
- [ ] [AI] Drive PR gates green: `gh pr checks <pr> --watch` (poll every 2 min, never `gh run watch`)
      — acceptance: all required checks report success
- [ ] [AI] Run the 3-cycle `pr-review-maker`→`pr-review-fixer` cycle; apply the hardened merge
      preconditions (3 cycles + up-to-date with `origin/main` via non-destructive forward update + gates
      green; **(d) the Delta 11 surface-conditional tester gates have been run and their defect findings
      resolved** — for THIS PR the surface is neither UI nor API, so record the explicit exemption in
      the PR description rather than leaving it implicit); merge (`[AI]` auto-merge — documented
      deviation, see **DD-10** in `tech-docs.md` §Design decisions) — acceptance: PR merged; branch was
      current at merge; the PR body contains the Delta-11 gate line (run-and-resolved, or explicit
      exempt)

### Phase 6 Gate

> All checks below must pass before Knowledge Capture (jointly with Phase 7).

- [ ] [AI] ose-primer PR merged; PR gates were green; governance blocks parity-match ose-public
- [ ] [AI] ose-primer `main-ci.yml` is schedule + dispatch only (`actionlint` green); a dispatched run concluded green

> **Pause Safety**: ose-primer matches the ose-public source of truth and is merged. Safe to stop. To
> resume: re-run the parity `diff` against ose-public `main`.

---

## Phase 7: Propagate to ose-infra (parallel with Phase 6)

> Runs in a dedicated `ose-infra` worktree, in parallel with Phase 6.

- [ ] [AI] **Confirm the sibling's repo topology BEFORE anything else** —
      `git -C /Users/wkf/ose-projects/ose-infra rev-parse --is-bare-repository`
      — acceptance: prints `true`. **`ose-infra` is a BARE repo** (verified 2026-07-19): it has no
      top-level working tree, so `git -C /Users/wkf/ose-projects/ose-infra status` fails with
      `fatal: this operation must be run in a work tree`. All file work happens inside a worktree.
      Note this repo's topology has CHANGED before (it was non-bare on 2026-07-02), so treat the
      check as live state — if it prints `false`, STOP and re-derive the commands below.
      See [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md)
      §Sibling-Repo Relative Paths From Inside a Worktree.
- [ ] [AI] Fetch and provision the worktree at the repo-local `worktrees/<name>/` path:
      `git -C /Users/wkf/ose-projects/ose-infra fetch origin main` then
      `git -C /Users/wkf/ose-projects/ose-infra worktree add worktrees/parallel-orchestration-shared-machine-governance -b parallel-orchestration-shared-machine-governance origin/main`
      — acceptance: `git -C /Users/wkf/ose-projects/ose-infra worktree list` shows the new worktree
      at `/Users/wkf/ose-projects/ose-infra/worktrees/parallel-orchestration-shared-machine-governance`, and
      `git -C <infra-worktree> rev-parse HEAD` equals `git -C /Users/wkf/ose-projects/ose-infra rev-parse origin/main`
- [ ] [AI] Set `<infra-worktree>` = `/Users/wkf/ose-projects/ose-infra/worktrees/parallel-orchestration-shared-machine-governance`
      for every subsequent step in this phase; run `npm install && npm run doctor -- --fix` **inside
      that worktree** (`cd` into it — do not rely on the shell's inherited working directory)
      — acceptance: `git -C <infra-worktree> status --porcelain` is empty; toolchain converged
- [ ] [AI] Apply the identical rule text from ose-public: N+1 + DAG + background-slot preference +
      status cadence + PR-merge preconditions edits, the two new convention files, the same-machine
      assumption, the vendor-neutral capability-gated paragraph, and index/workflow wiring — acceptance:
      `diff` of the governance blocks vs. merged ose-public shows only path-relative differences
- [ ] [AI] Apply the swept agents/skills/workflows updates to match ose-public — **ALL SEVEN**
      `workflows/plan/*` files **plus** the repo-wide `max-concurrency` set (preserving
      `web-ux-test-fixing-planning.md` at `Default 1`) — acceptance: the repo-wide superseded-cap grep
      returns zero hits in the ose-infra worktree
- [ ] [AI] Port the Delta 11 surface-conditional gate: create ose-infra
      `repo-governance/workflows/api/api-quality-gate.md` + `api/README.md`, register `api/` in
      `workflows/README.md`, and wire the rule into `plan/plan-execution.md`, `plan/plan-planning.md`,
      `pr/pr-review-quality-gate.md`, and `development/quality/user-facing-delivery-hardening.md`
      — acceptance: `test -f repo-governance/workflows/api/api-quality-gate.md` exits 0 and the three
      wiring files each contain `api-quality-gate`
  - _Suggested executor: `repo-workflow-maker`_
- [ ] [AI] Edit ose-infra `.github/workflows/main-ci.yml`: same schedule-only trigger
      (`cron: "0 5,11,17,23 * * *"` + `workflow_dispatch`; remove `push`) while KEEPING ose-infra's
      existing `coralpolyp` jobs unchanged — acceptance: `actionlint` exits 0; no `push:` trigger
      remains; `coralpolyp` jobs still present
- [ ] [AI] Regenerate bindings: `npm run generate:bindings`; run link/markdown/vendor-audit gates
      — acceptance: exit 0; bindings synced. **Repo-relevance guardrail**: keep infra-private content in
      ose-infra only; do NOT cross-route it into the public governance text
- [ ] [AI] Confirm no `apps/rhino-cli/**` surface changed (byte-identity guardrail):
      `git -C <infra-worktree> status --porcelain apps/rhino-cli` — acceptance: empty output
- [ ] [AI] Commit with explicit paths (never `git add -A` — the sibling repos carry unrelated WIP):
      `git -C <infra-worktree> add <explicit paths> && git -C <infra-worktree> commit`
      — acceptance: `git -C <infra-worktree> status --porcelain` shows no unintended files staged
- [ ] [AI] Push to the ose-infra PR branch: `git -C <infra-worktree> push origin <branch>`
      — acceptance: push succeeds; pre-push gates exit 0
- [ ] [AI] Open the draft PR: `gh pr create --repo <ose-infra> --draft`
      — acceptance: PR URL returned; PR shows as draft
- [ ] [AI] Drive PR gates green: `gh pr checks <pr> --watch` (poll every 2 min, never `gh run watch`)
      — acceptance: all required checks report success
- [ ] [AI] Run the 3-cycle `pr-review-maker`→`pr-review-fixer` cycle; apply the hardened merge
      preconditions (3 cycles + up-to-date with `origin/main` via non-destructive forward update + gates
      green; **(d) the Delta 11 surface-conditional tester gates have been run and their defect findings
      resolved** — for THIS PR the surface is neither UI nor API, so record the explicit exemption in
      the PR description rather than leaving it implicit); merge (`[AI]` auto-merge — documented
      deviation, see **DD-10** in `tech-docs.md` §Design decisions) — acceptance: PR merged; branch was
      current at merge; the PR body contains the Delta-11 gate line (run-and-resolved, or explicit
      exempt)

### Phase 7 Gate

> All checks below must pass before Knowledge Capture (jointly with Phase 6).

- [ ] [AI] ose-infra PR merged; PR gates were green; governance blocks parity-match ose-public
- [ ] [AI] ose-infra `main-ci.yml` is schedule + dispatch only (`actionlint` green) with `coralpolyp` jobs intact

> **Pause Safety**: ose-infra matches the ose-public source of truth and is merged. Safe to stop. To
> resume: re-run the parity `diff` against ose-public `main`.

---

## Phase 8: Knowledge Capture

> _Triage every surviving `learnings.md` entry before archival. See the
> [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md)._

- [ ] [AI] Apply the litmus test to every `learnings.md` entry — keep only if a durable surface would
      catch this automatically next time; discard the rest with a one-line reason — acceptance: every entry has a route or a discard reason
- [ ] [AI] Apply the **secret/sensitivity gate** — sanitize any secret/credential/private hostname to
      a `<placeholder>` token, or discard if unsanitizable — acceptance: `learnings.md` contains no raw secret
- [ ] [AI] Apply the **repo-relevance gate** — infra-private content stays in `ose-infra` only and is
      never cross-routed into `ose-public`/`ose-primer`; public-governance content may propagate via
      the existing parity loop — acceptance: no infra-private content in this repo's routed output
- [ ] [AI] Route each surviving learning to exactly one durable home per the open-ended rubric;
      **code-homed** learnings (`apps/`, `libs/`, tests) are ALWAYS filed as a separate
      `plans/backlog/<slug>/` plan, NEVER landed inline — acceptance: every entry records its terminal routing state
- [ ] [AI] If no generalizable learning surfaced, record `No generalizable learnings — <reason>` in
      `learnings.md` — acceptance: `learnings.md` is never silently empty

### Phase 8 Gate

> All checks below must pass before the Cleanup gate.

- [ ] [AI] Every `learnings.md` entry is terminal (routed inline, filed as backlog, or discarded with reason), or the explicit "none" escape is present
- [ ] [AI] No code-homed learning landed inline in this plan's own commits/PRs

> **Pause Safety**: `learnings.md` is fully triaged; nothing depends on querying it later. Safe to
> stop. To resume: re-read `learnings.md` and confirm every entry is terminal.

---

## Phase 9: Cleanup Gate (self-scoped, non-destructive to others)

> Dogfoods the new worktree-and-artifact-cleanup convention. Non-destructive to any other actor.

- [ ] [AI] Enumerate the worktrees THIS plan created (ose-public, ose-primer, ose-infra) and confirm
      each is merged and no other session/process is using it (`git worktree list` per repo; check
      for active processes) — acceptance: each target worktree confirmed self-created + idle
- [ ] [AI] Remove only this plan's own worktrees with the non-forced command
      `git worktree remove <path>` (NEVER `--force`, NEVER a worktree you did not create) — acceptance: only this plan's worktrees removed; others intact
- [ ] [AI] Purge only the build artifacts THIS plan created (any `target/`, `dist/`, `.next/`, build
      caches produced inside this plan's worktrees), after verifying non-use — acceptance: self-created artifacts removed
- [ ] [AI] Explicitly SKIP the shared cargo `target/` and any shared cache other sessions depend on —
      acceptance: shared caches confirmed present and untouched; note recorded in `learnings.md`

### Phase 9 Gate

> All checks below must pass before Plan Archival.

- [ ] [AI] Only self-created, verified-idle worktrees/artifacts were removed; the shared cargo `target/` and all shared caches are intact
- [ ] [AI] No destructive git operation and no `git add -A` was used anywhere in this plan

> **Pause Safety**: the shared disk is reclaimed of this plan's own artifacts only; every other
> actor's worktrees, WIP, and shared caches are untouched. Safe to stop. To resume: re-run
> `git worktree list` per repo and confirm state.

---

## Plan Archival

> This archival runs via direct push to `main` after all three repos' PRs (Phase 5/6/7) have merged,
> rather than being folded into any one delivering PR — a documented, authorized deviation for this
> tri-repo-propagation plan. See **DD-11** in `tech-docs.md` §Design decisions for the rationale, the
> authorizing context, and its explicit non-precedential scope.

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify the Knowledge Capture phase is complete — every `learnings.md` entry terminal or the
      explicit "none" escape present; both safety gates applied
- [ ] [AI] Verify ALL quality gates pass (local + CI) across all three repos
- [ ] [AI] Verify the Cleanup gate ran non-destructively (self-scoped only; shared caches intact)
- [ ] [AI] Move: `git mv plans/in-progress/parallel-orchestration-shared-machine-governance/ plans/done/2026-07-19__parallel-orchestration-shared-machine-governance/` (use the completion date at archival time)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan
- [ ] [AI] Commit the archival (explicit paths): `chore(plans): move parallel-orchestration-shared-machine-governance to done`
