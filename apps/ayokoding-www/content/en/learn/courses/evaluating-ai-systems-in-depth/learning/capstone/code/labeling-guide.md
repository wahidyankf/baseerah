# Capstone Step 2 — Labeling Guide

_Traces to: `criteria.md`. Used by `ground_truth.jsonl`'s adjudication process in Step 3._

This is the written protocol two independent labelers follow to label held-out cases against
Step 2's four criteria. The goal is agreement measured on a held-out sample, not agreement assumed.

## Labeling procedure

1. Read the `request` and `reply` for one case, without seeing the other labeler's answer.
2. Identify which of the four criteria in `criteria.md` applies to this case. Exactly one applies
   per case in this capstone's dataset — if more than one seems to apply, flag the case for
   adjudication rather than picking one.
3. Decide `passed: true` or `passed: false` against that criterion's operationalized check, exactly
   as written — not a looser or stricter personal reading of it.
4. Record the label plus a one-sentence reason quoting the specific part of the reply that drove
   the decision.

## Disagreement resolution

When the two labelers' `passed` values differ on the same case:

1. Both labelers re-read the case and their own stated reason.
2. If one labeler's reason reveals a misreading of the criterion's operationalized check (not a
   difference of opinion, an actual misreading), that labeler's label is corrected.
3. If both readings are defensible under the criterion as written, the criterion's wording itself
   is the problem — it is revised in `criteria.md` to remove the ambiguity, and the case is
   re-labeled by both under the revised wording.
4. Every resolved disagreement is logged: case ID, both original labels, the resolution, and
   whether the criterion wording changed.

## Agreement threshold

- **Justified threshold**: 85% raw agreement on a held-out sample of at least ten cases per
  criterion, before a criterion is considered stable enough to build a ground-truth set from.
- **Why 85%, not 100%**: criteria describe real, sometimes genuinely ambiguous agent behavior;
  demanding perfect agreement would either mask real ambiguity in the underlying behavior or push
  labelers toward rote agreement rather than honest, independent judgment.
- **What happens below threshold**: the criterion returns to `criteria.md` for a wording revision,
  following the same disagreement-resolution step above, and is re-measured before being used to
  build ground truth.

## Held-out sample discipline

The held-out sample used to measure labeler agreement is never reused as the ground-truth set
itself in Step 3 — using the same cases for both would inflate the reported agreement statistic by
letting labelers implicitly "practice" on the exact cases being measured.
