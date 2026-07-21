<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: bare-repo-governance-hardening

Append one `## Learning: <one-line summary>` section per generalizable observation, sanitized per
the secret/sensitivity gate before it is ever written. Entry shape:

```markdown
## Learning: <one-line summary>

- **Context**: what was being done when this surfaced
- **Observation**: what was noticed (sanitized)
- **Why it might generalize**: the litmus reasoning
- **Terminal state**: routed inline to `<path>` / filed as `plans/backlog/<slug>/` / discarded — `<reason>`
```

> **Watch for this plan's own highest-yield source**: Phases 4 and 5 execute the very procedure
> `bare-repo-landing-method.md` documents. Any friction between the written steps and what execution
> actually required is a defect in that document — record it here, and Phase 6 routes it back into
> the document in all three repos.
>
> If execution surfaces nothing generalizable, replace this line with the explicit escape:
> `No generalizable learnings — <one-line reason>`. Never leave the file silently empty.

## Learning: the defect reproduced live during this plan's own promotion

- **Context**: promoting the plan from `backlog/` to `in-progress/` on 2026-07-21, re-verifying the
  repo-grounded claims in `tech-docs.md` before running the quality gate.
- **Observation**: both bare siblings read `2 0` on
  `git rev-list --left-right --count origin/main...main` — local `main` two commits behind
  `origin/main` in each. The lagging commits (`c12e1eb7f` + `53d9081b7` in `ose-primer`,
  `474545a69` + `f6ecdcc0b` in `ose-infra`) were landed through side worktrees in an earlier
  session. Nothing failed and nothing warned; the lag is only visible if you ask for it explicitly.
  `tech-docs.md` had recorded `0 0` for both, so the plan's own written state had silently gone
  stale in under a day.
- **Why it might generalize**: this is the plan's motivating failure class, observed without being
  sought, on a repo whose maintainer already knows about it. It is direct evidence for the strength
  of C1's terminal-reconcile step — a rule that is easy to forget is not adequately served by prose
  alone, which is worth weighing against **DD-2**'s no-automation stance at Phase 6 triage.
  It also shows any "verified state" line in a plan needs a re-verification step, not just a date.
- **Terminal state**: pending — triage at Phase 6. Candidate route: a worked example inside `C1`
  showing the non-zero reading and the `git fetch origin main:main` recovery.
