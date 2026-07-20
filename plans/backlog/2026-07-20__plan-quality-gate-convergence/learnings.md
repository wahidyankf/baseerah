<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: plan-quality-gate-convergence

## Learning: the "safe" occurrence-unique grep form masks file absence

- **Context**: authoring this plan's own acceptance clauses, while empirically verifying the
  registry's DC-1 safe rewrite form before writing it down.
- **Observation**: `grep -ohE 'a|b' f | sort -u | wc -l` returns `0` both for a file that exists
  with no match and for a file that does not exist at all, because `wc -l` counts empty stdin
  identically. Exit codes differ (1 versus 2) but are discarded by the pipe. The recommended fix for
  DC-1 therefore silently reintroduces the DC-2 blind spot unless paired with `test -f`.
- **Why it might generalize**: this is a second-order trap — a defect introduced by the remedy for a
  first-order defect. It is exactly the fix-site injection pattern this plan exists to stop, and it
  was caught only because authoring-time empirical simulation was performed. It is seeded into the
  registry as DC-2b and is the plan's own proof that mechanism 3 (symmetric verification) works.

## Learning: `plans/backlog/README.md` contradicts the authoritative plans convention on naming

- **Context**: registering this plan in the backlog index during authoring.
- **Observation**: `plans/backlog/README.md` §Instructions states "Plans in `backlog/` use NO date
  prefix — just the slug", while
  `repo-governance/conventions/structure/plans.md` §backlog — creation date prefix states the
  opposite and is the authoritative surface. The existing sibling plan
  `plans/backlog/doc-command-existence-validation/` follows the stale README rather than the
  convention.
- **Why it might generalize**: a non-authoritative index file drifted from its governing convention
  and is actively producing non-conforming artifacts. This is a routing question for
  `repo-rules-fixer` (correct the README) plus a decision on whether to rename the existing
  non-conforming folder. Not fixed inline here — it is outside this plan's declared scope and would
  be an unrequested rename of another plan's folder.

## Learning: neither Prettier nor this repo's markdownlint catches the indented-fence trap

- **Context**: verifying the 09-26 audit's claim that `markdownlint-cli2` MD046 would catch the
  indented-fence class going forward.
- **Observation**: the claim is false under this repo's configuration. `.markdownlint-cli2.jsonc`
  leaves `MD046` unset, so its default `consistent` style is vacuously satisfied by a single-block
  file — the broken form produces 0 errors. Prettier reports the same broken form as correctly
  formatted. Setting `MD046: {style: fenced}` does catch it (1 error).
- **Why it might generalize**: an audit report's forward-looking claim about tooling coverage was
  taken as fact and would have justified skipping a detector. Tooling-coverage claims need the same
  empirical verification as any other factual claim, and the repo currently has a real, unguarded gap
  for a defect class that consumed two full iterations of the archived chain.
