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

## Learning: inside a bracket expression, a backslash is not an escape (DC-8)

- **Context**: the 2026-07-20 goal-alignment audit of this plan and its sibling, reproduced live on
  this host against the real `.claude/agents/repo-rules-checker.md`.
- **Observation**: `command grep -ohE '^### Step [0-9.]+[^\n]*'` truncates every match before the
  first literal lowercase `n` (`Initialize`→`I`, `Validation`→`Validatio`, `Governance`→`Gover`),
  because POSIX bracket expressions treat `\` as a literal, so `[^\n]` means "not backslash and not
  `n`". BSD grep enforces this; GNU grep and ripgrep extend `\n` and do not truncate. The corrected
  `'^### Step [0-9.]+.*$'` returns all headings in full on every engine. Critically, the broken form
  **passed by luck**: `sort -u | wc -l` still returned the right count only because no two truncated
  prefixes collided, so a future heading rename could silently undercount and let a
  "no check was removed" invariant pass after a check really was removed.
- **Why it might generalize**: this is the second self-caught entry in this registry (after DC-2b),
  and it was found in a plan authored specifically to install search-tool discipline. It is a direct
  instance of the enumeration-fails-open rule — the clause enumerated what to exclude instead of
  asserting an invariant, and the enumeration failed open silently. Seeded as DC-8 with the invariant
  form "every clause's regex means the same thing under the BSD, GNU and ripgrep engines".

## Learning: a catalogue nobody consults is inert — both plans re-derived an existing convention

- **Context**: the same audit found that `deterministic-vs-ai-validation-split.md` already codified
  the decision tree and implementation contract both plans re-derived independently.
- **Observation**: this plan contained zero references to that convention, and neither plan added its
  new category to the convention's Split table — the one document whose entire purpose is being the
  canonical answer to "is category X deterministic or AI-judged". The omission is simultaneously a
  BS-10 (definition block missed while usage sites were swept) and a BS-2 (generative-source-only
  scope) instance, committed by the pair of plans that catalogue those exact classes.
- **Why it might generalize**: knowing a blind-spot class exists does not prevent instantiating it;
  only a mechanism does. This is the strongest available argument for XD-2's position that shared
  substrate should be _built and consulted mechanically_ rather than _documented and remembered_.
  Candidate rule for the plan-anti-hallucination convention: before authoring a design-decision
  section, grep the conventions tree for an existing convention owning that decision.
