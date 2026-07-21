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
