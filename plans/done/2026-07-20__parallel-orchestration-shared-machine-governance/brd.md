# Business Requirements — Parallel-Orchestration & Shared-Machine Governance

## Business goal

Make the governance surface honest about how work actually happens — many agents and engineers
operating **concurrently on one shared machine** — and safe under that reality, by (a) raising and
generalizing the concurrency model, (b) forbidding local git operations that destroy others' work,
and (c) mandating safe, self-scoped disk cleanup so the shared machine does not fill up.

## Business rationale

Throughput and safety are currently in tension. The fixed cap ("2 background, 3 total") under-uses a
machine that can host more parallel work, while nothing in governance protects concurrent actors from
a single `git reset --hard`, `git clean -fd`, or shared-cache prune that silently wipes hours of
uncommitted work. As the repos get busier, both the missed throughput and the blast radius of a
destructive command grow. Encoding the N+1 model, the no-destructive-git rule, and the cleanup
discipline turns informal habits (already reflected in maintainer memory) into durable, propagated
governance.

## Business impact

**Pain points addressed**

- **Under-parallelization**: a rigid cap discourages using available machine capacity when many
  independent units are ready.
- **Silent work destruction**: no rule stops an agent from running a locally destructive git command
  that erases a peer's uncommitted changes on the shared disk.
- **Disk exhaustion**: plans that spin up multiple worktrees leave `target/`, `dist/`, `.next/`, and
  build caches behind, filling the shared disk and eventually breaking everyone's builds.
- **Implicit assumption**: the "same machine, concurrent actors" premise is nowhere stated, so
  guidance is not consistently written to be safe under it.

**Expected benefits**

- Higher sustained parallel throughput, tuned to real machine capacity via an adjustable N.
- Near-elimination of accidental cross-actor work loss from local destructive git commands.
- Bounded shared-disk growth via mandatory, safe, self-scoped cleanup at plan end.
- One coherent, propagated rule set across all three OSE repositories.

## Affected roles (hats the maintainer wears; agents that consume the rules)

- **Orchestrator hat / main-thread agent**: reads the N+1 model to decide how many background agents
  to run and when to raise or lower N.
- **Executor hat / background subagents**: bound by the same N+1 accounting and by the
  no-destructive-git and cleanup rules while operating inside their own worktree.
- **Governance-author hat / `repo-rules-maker`**: authors and propagates the rule text across repos.
- **Reviewer hat / `pr-review-maker` + `pr-review-fixer`**: gate each per-repo PR through the review
  cycle before merge.

Solo-maintainer repository — no sign-off ceremonies; "roles" are hats the maintainer wears and the
agents that consume the governance files.

## Business-level success metrics

- **Model replaced, not duplicated**: every surface that stated the old "2"/"3" fixed cap now states
  the N+1 model with default N=3. _Judgment call:_ verified by grep for stale "cap at 2"/"3 total"
  phrasing returning zero unintended hits post-change.
- **Rules exist and are discoverable**: the two new conventions exist, are linked from `AGENTS.md`
  and the workflow index, and pass `repo-rules-checker`. Observable fact at delivery time.
- **Tri-repo parity**: identical rule text lands in `ose-public`, `ose-primer`, and `ose-infra`.
  Observable fact (diff of the governance blocks across repos).
- **Self-consistency**: this plan's own execution uses non-destructive git, explicit-path staging,
  and self-scoped cleanup — i.e. it passes the rules it introduces. _Judgment call._

No fabricated numeric KPIs — the value is qualitative (safety + honesty of the governance surface)
and structurally verifiable (rules present, linked, propagated, gate-green).

## Business-scope non-goals

- Not building automated linters/hooks to enforce the new rules (governance text + human/agent review
  only).
- Not changing product behavior, app code, or CI cadence.
- Not renaming or restructuring existing conventions beyond what the new rules require.
- Not touching `apps/rhino-cli/**` (byte-identity boundary).

## Business risks and mitigations

| Risk                                                                                  | Likelihood | Mitigation                                                                                                                          |
| ------------------------------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Raising N causes vendor rate-limit / runner saturation on the shared machine          | Medium     | N is adjustable **down** under runner/disk pressure; keep the mtime/staleness relaunch guidance; default N=3 stays conservative.    |
| A "no destructive git ops" rule is read as forbidding legitimate self-scoped teardown | Medium     | Rule scopes the ban to operations affecting **shared/others'** state; explicitly permits additive ops within your **own** worktree. |
| Cleanup rule accidentally deletes a shared cache other sessions depend on             | High       | Hard caveat: delete only self-created, verified-not-in-use artifacts; **never** the shared cargo `target/`; when in doubt, leave.   |
| Tri-repo drift (text lands in one repo, diverges in another)                          | Medium     | `ose-public` authored first as source of truth; propagate identical text; per-repo PR + review cycle; parity diff at each gate.     |
| Governance change accidentally touches the rhino-cli byte-identity boundary           | Low        | Explicit guardrail: do not touch `apps/rhino-cli/**`; if unavoidable, keep byte-identical across all three repos.                   |

## References

- [prd.md](./prd.md) — testable acceptance scenarios for each rule delta.
- [tech-docs.md](./tech-docs.md) — the concrete surface inventory and rule text deltas.
- [Git Push Safety Convention](../../../repo-governance/development/workflow/git-push-safety.md) —
  existing remote-destructive-op rule the new local rule complements.
