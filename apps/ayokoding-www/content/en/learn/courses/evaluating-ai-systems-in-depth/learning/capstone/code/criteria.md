# Capstone Step 2 — Derived Criteria

_Traces to: `analysis/error_analysis.py`'s taxonomy output._

Each criterion below is derived from exactly one of the four taxonomy modes Step 1 produced. No
criterion here checks anything the analysis pass did not actually observe — this is co-05's own
rule, kept explicit and auditable.

## Criterion 1 — asks before acting on an ambiguous target

- **Traces to mode**: `skips-clarifying-question` (3 of 8 failures, the dominant mode)
- **Statement**: The reply must ask a clarifying question before acting, whenever the request
  names no specific board or ticket, or names one ambiguously (for example, "the other board" when
  two boards are equally plausible).
- **Operationalized check**: the reply text contains a question requesting the missing board or
  ticket identity, and the reply does not perform any board-move, close, or archive action in the
  same turn.

## Criterion 2 — acts on the exact target named

- **Traces to mode**: `wrong-object-acted-on` (1 of 8 failures)
- **Statement**: When the request names a specific ticket or board, the reply must act on that
  exact identifier, never a different one.
- **Operationalized check**: the ticket/board ID mentioned in the reply's action must string-match
  the ID named in the request.

## Criterion 3 — reports the true aggregate count

- **Traces to mode**: `incorrect-aggregate-count` (2 of 8 failures)
- **Statement**: A reported count (of open bugs, in-progress tickets, or any other aggregate) must
  match the true count in the underlying data at query time, including tickets that changed state
  the same day.
- **Operationalized check**: the number in the reply equals the reference count computed directly
  from the ground-truth ticket-state snapshot for that query.

## Criterion 4 — applies every stated filter condition

- **Traces to mode**: `ignores-stated-filter-condition` (2 of 8 failures)
- **Statement**: When a request states a filter condition (an age threshold, a tag, a status), the
  reply's action must apply exactly that filter, never the unfiltered full set.
- **Operationalized check**: every ticket the reply acted on must satisfy the stated filter
  condition; no ticket outside that filter was touched.

## Traceability table

| Criterion | Taxonomy mode                     | Failures behind it |
| --------- | --------------------------------- | ------------------ |
| 1         | `skips-clarifying-question`       | 3                  |
| 2         | `wrong-object-acted-on`           | 1                  |
| 3         | `incorrect-aggregate-count`       | 2                  |
| 4         | `ignores-stated-filter-condition` | 2                  |

Every one of the eight raw failures in `analysis/raw_failures.jsonl` is accounted for by exactly
one criterion above — no criterion is speculative (co-09's rule, applied here to the whole set,
not just one example).
