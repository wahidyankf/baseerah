# Business Requirements — Plan-Execution Knowledge Capture

## Business Goal

Turn every plan execution into a **compounding** event for the codebase: each plan should leave the
repository's durable knowledge surfaces (`docs/`, governance rules, skills, agents) a little better
than it found them, so the system gets progressively harder to break in the same way twice.

The litmus test — adopted verbatim from the Compound Engineering practice — is:

> A learning is worth capturing if, once routed, **the system would catch this automatically next
> time**.

If routing a learning would make a checker flag it, a convention forbid it, a skill teach it, or a
doc answer it before it recurs, the learning compounds and belongs somewhere durable. If it would
not, it is noise and gets discarded with a one-line reason.

## Business Rationale (WHY)

### The pain: knowledge evaporates plan-over-plan

This is a solo-maintainer repo with a large fleet of AI agents that reload the same instruction
surfaces every session. Today, the useful things learned while executing a plan — a cleaner way to
word a convention, a repeated papercut, a workflow step everyone forgets, a fact worth writing down
— live only in the maintainer's short-term memory and the plan's own prose. When the plan is
archived (and `plans/done/` may be pruned at any future date), that knowledge is gone. The next plan
rediscovers it. `[Judgment call]` — this is the observable pattern behind repeated near-identical
fixes across the plan history (e.g., the recurring worktree-toolchain-init papercut, the recurring
"git-mechanical steps are `[AI]`" clarification).

### The anti-pattern this prevents: the knowledge black-hole

Web research (see [prd.md](./prd.md) sources) converges on a well-known failure mode: retrospective
artifacts — Lessons Learned Logs, retro boards, standalone `learnings.md` files — that get written
and never read again. Three conditions make such an artifact survive rather than vanish into a
black hole:

1. **A single named owner.** In a solo-maintainer repo, that is the maintainer, acting through the
   plan-execution workflow.
2. **It lives in a tool people already open.** The agents reload `AGENTS.md`, governance rules,
   skills, and agent definitions every session; `docs/` is the searched archive. Routing INTO those
   surfaces (rather than into a separate log) satisfies this condition by construction.
3. **A fixed-cadence review forces someone to look again.** The mandatory Knowledge Capture phase,
   run at the end of every substantive plan, is that cadence.

Our design satisfies all three by **routing out** of the transient log into surfaces that are
already loaded and already reviewed — instead of hoping a standalone log gets revisited.

### Why a running log + a final triage phase

The running log keeps capture cheap and in-the-moment (learnings recorded when noticed, not
reconstructed from memory at the end). The final triage phase is the forcing function that prevents
the log from becoming the black hole: it drains the log into durable homes before archival. This
mirrors the PRINCE2 shape (a running Lessons Learned Log plus a closure Lessons Learned Report),
kept intentionally lighter for a solo maintainer — no reports, no ceremonies, just a triage-and-route
pass.

### Why guard both under- and over-capture

Research is equally clear on the opposite failure: **when every idea becomes an action item, focus
disappears.** A capture practice that never discards is as useless as one that never captures. The
explicit `discard — not generalizable` destination, with a required one-line reason, is the
deliberate noise guard. The mandatory-but-with-explicit-"none"-escape rule guards the other side:
substantive plans must run the phase, but may honestly record "no generalizable learnings" with a
reason rather than manufacture busywork.

## Business Impact

| Dimension           | Expected benefit                                                                                     |
| ------------------- | ---------------------------------------------------------------------------------------------------- |
| Compounding quality | Each plan hardens a durable surface; recurring papercuts become checks/rules and stop recurring.     |
| Reduced rework      | Fewer rediscoveries of the same lesson across plans; agents load the improved surface next session.  |
| Governance hygiene  | A single owned rubric decides where a learning goes, instead of ad-hoc, inconsistent placement.      |
| Safety by default   | Two hard gates (repo-relevance, secret/sensitivity) prevent private/infra leakage into public repos. |
| No new toil         | The `none` escape and pure-docs/trivial exemption keep the practice from becoming ceremony.          |

## Affected Roles

Solo-maintainer repo — no sign-off ceremonies, sponsors, or stakeholder committees. The "roles" are
the hats the maintainer wears and the agents that consume the changed files:

- **Plan author (maintainer via `plan-maker` / plan-creating skill)** — emits the Knowledge Capture
  phase and the `learnings.md` scaffold into every authored plan.
- **Plan executor (the plan-execution workflow the maintainer drives)** — appends to `learnings.md`
  during execution and runs the final triage.
- **Quality gate (maintainer via `plan-checker`)** — flags a silently absent Knowledge Capture phase.
- **Completion gate (maintainer via `plan-execution-checker`)** — verifies routing actually happened
  and both safety gates passed before archival.
- **Repair (maintainer via `plan-fixer`)** — scaffolds a missing phase.
- **Every agent that reloads governance/skills/agents** — the ultimate consumer: routed learnings
  reach them the next session.

## Business-Level Success Metrics

Numeric targets would be fabricated here, so they are stated as qualitative/observable signals
(`[Judgment call]` unless marked otherwise):

- **Observable:** After execution, `knowledge-capture.md` exists in all three repos and is linked
  from each repo's `repo-governance/development/quality/README.md` index. `[Repo-grounded]` once done.
- **Observable:** Newly authored plans contain a Knowledge Capture phase and a `learnings.md`
  scaffold (grep-checkable). `[Repo-grounded]` once done.
- **Qualitative reasoning:** The rate of "we fixed this exact thing before" moments trends down as
  recurring learnings become checks/rules that catch them automatically. Supported by the
  compounding logic above; not separately measured.
- **Qualitative reasoning:** Zero private/infra content leaks into the public repos via a learning —
  guaranteed by the repo-relevance gate being a hard, checker-enforced step, not a best-effort note.

## Business-Scope Non-Goals

- Not a metrics/analytics program — no dashboards, no counting of learnings as a KPI.
- Not a replacement for incident post-mortems — those remain the home for the failure case.
- Not a permanent archive of learnings — `learnings.md` is explicitly transient (see the
  transient-log caveat in [tech-docs.md](./tech-docs.md)).
- Not a code-enforcement effort — no new `rhino-cli` validators (enforcement is agent-checker prose).

## Business Risks and Mitigations

| Risk                                                                    | Mitigation                                                                                                    |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| The practice becomes ceremony/theater (over-capture)                    | Explicit `discard` path + `none` escape + pure-docs/trivial exemption; "would the system catch it?" litmus.   |
| Learnings never get routed (black-hole reappears inside `learnings.md`) | Archival is BLOCKED until every entry is routed, filed as backlog, or discarded-with-reason.                  |
| Private `ose-infra` content leaks into public `ose-public`/`ose-primer` | Hard repo-relevance gate as an explicit triage step; `plan-execution-checker` verifies it before archival.    |
| Secrets/credentials committed into world-readable `learnings.md`        | Hard secret/sensitivity gate inheriting the No-Secrets iron rule + post-mortem placeholder rule.              |
| Someone treats `learnings.md` as the system of record                   | Transient-log caveat is stated in the convention; nothing may depend on querying `plans/done/*/learnings.md`. |
| Three-repo drift (change lands in one repo, not the others)             | Delivery has explicit per-repo phases + a parity check; `ose-public`→`ose-primer` rides the parity loop.      |
