---
title: "Overview"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 1
---

## The capstone: a complete, validated eval system for Tasklight

The capstone builds one small, complete evaluation system end to end, for a fictional AI task
assistant called Tasklight, across five ordered steps that mirror this topic's own Beginner ->
Intermediate -> Advanced arc: read and cluster a real batch of failures into a taxonomy, derive and
operationalize criteria from that taxonomy plus a written labeling guide, build and validate an
LLM-as-judge (a model genuinely different from the one under test) against an adjudicated
ground-truth set, score an agent's trajectory alongside its final answer with step-level failure
attribution, and wire a noise-aware, tiered, cost-budgeted CI gate. Every script lives under
`learning/capstone/code/`, fully type-annotated (strict `pyright`), and was actually run against
**Python 3.13.12** to capture the output shown below. Every case, failure, and model response is
original and invented for this topic -- no dataset, transcript, or prompt is copied from any real
product, vendor documentation, or third party.

- **Step 1 -- `analysis/error_analysis.py`** (with `analysis/raw_failures.jsonl`): reads eight real,
  logged Tasklight failures, open-codes each one, and clusters them into a frequency-ranked
  taxonomy. Ties together co-01, co-02, co-03, co-04.
- **Step 2 -- `criteria.md`** (with `labeling-guide.md`): four criteria, each traced to exactly one
  taxonomy mode from Step 1, each operationalized into a mechanical check, plus a written labeling
  protocol two independent labelers can follow. Ties together co-05, co-06, co-07.
- **Step 3 -- `judge.py`** (with `ground_truth.jsonl`): thirteen adjudicated ground-truth cases, and
  a judge model -- genuinely different from the agent under test -- validated per criterion with a
  Wilson confidence interval, correctly retiring the one criterion it cannot reliably judge. Ties
  together co-08, co-09, co-10, co-11, co-12, co-13, co-16, co-22.
- **Step 4 -- `trajectory.py`**: scores an agent's full tool-call trajectory alongside its final
  answer, demonstrating a genuine right-answer-wrong-path case with the failure attributed to its
  exact causing step. Ties together co-18, co-19, co-20.
- **Step 5 -- `ci/ci_gate.py`**: measures this suite's own noise floor across repeated unchanged
  runs, derives a regression bar above it, and wires a fast deterministic tier plus a judged merge
  tier into one cost-budgeted CI gate. Ties together co-23, co-24, co-25, co-26.

**Concepts exercised**: [x] error analysis before metrics (co-01) [x] open-coding failures (co-02)
[x] a failure taxonomy (co-03) [x] frequency-weighted prioritization (co-04) [x] derived
task-specific criteria (co-05) [x] criterion operationalization (co-06) [x] a human labeling
protocol (co-07) [x] a ground-truth set (co-08) [x] LLM-as-judge (co-09) [x] judge-human agreement
measured, not assumed (co-10) [x] judge scope reliability (co-11) [x] judge model separation
(co-12) [x] judge bias modes probed (co-13) [x] judge recalibration triggers (co-16) [x]
trajectory evaluation (co-18) [x] outcome vs. process scoring (co-19) [x] multi-step failure
attribution (co-20) [x] eval in CI (co-23) [x] a regression bar above the noise floor (co-24) [x]
cost of evaluation (co-25) [x] tiered eval suites (co-26) [x] dataset contamination is out of scope
for this capstone's small, fully-invented dataset (co-22, retired criterion 3 stands in for this
lesson instead).

---

## Step 1: analysis/error_analysis.py -- read, open-code, and cluster eight real failures

**Context**: the Beginner tier (Examples 1-6) built each piece of this step in isolation -- reading
a failure sample, open-coding it, and clustering codes into a taxonomy. This step runs the full
pipeline against the capstone's own eight-failure batch, logged from a fictional Tasklight ticket
assistant.

```jsonl
# learning/capstone/code/analysis/raw_failures.jsonl
{"request": "Move this to done.", "reply": "Moved to done.", "open_code": "acted on 'this' without any board or ticket named"}
{"request": "Close ticket #12.", "reply": "Closed ticket #21.", "open_code": "acted on ticket #21 instead of the requested #12"}
{"request": "How many bugs are open?", "reply": "There are 14 open bugs.", "open_code": "counted 14 when the true open-bug count was 21, undercounting closed-but-reopened tickets"}
{"request": "Archive tickets older than 90 days.", "reply": "Archived all tickets.", "open_code": "archived every ticket, ignoring the stated 90-day age filter"}
{"request": "Move that one to the other board.", "reply": "Moved to the Sprint board.", "open_code": "guessed a specific board despite two boards being equally plausible referents"}
{"request": "Close the ticket.", "reply": "Closed ticket #4821.", "open_code": "acted on a single guessed ticket ID with no board or ticket named at all"}
{"request": "How many tickets are in progress?", "reply": "9 tickets are in progress.", "open_code": "counted 9 when the true in-progress count was 12, missing tickets moved mid-day"}
{"request": "Delete tickets tagged 'spam'.", "reply": "Deleted all tickets.", "open_code": "deleted every ticket, ignoring the stated 'spam' tag filter"}
```

```python
# learning/capstone/code/analysis/error_analysis.py
"""Capstone Step 1: Read a Batch of Real Failures, Open-Code Them, and Cluster Into a Taxonomy."""  # => co-02/co-03/co-04: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-02: raw_failures.jsonl is read as real, logged JSON lines -- not invented after the fact
from pathlib import Path  # => co-02: locates raw_failures.jsonl relative to this file, not the caller's cwd
from typing import NamedTuple  # => co-02: FailureRecord and TaxonomyEntry are typed records, not bare dicts

RAW_FAILURES_PATH = Path(__file__).parent / "raw_failures.jsonl"  # => co-02: resolved relative to THIS file, so it runs correctly from any working directory


class FailureRecord(NamedTuple):  # => co-02: one real, logged failure -- the raw material for open coding
    request: str  # => co-02: the real request
    reply: str  # => co-02: what the agent actually replied
    open_code: str  # => co-02: a short, specific description of what went wrong -- written by READING the failure, not guessed from a category list


class TaxonomyEntry(NamedTuple):  # => co-04: one clustered failure mode -- a NAME plus its own frequency
    mode_name: str  # => co-04: the analyst's own coined name for this cluster of open codes
    frequency: int  # => co-06: how many of the eight raw failures fall into this mode


def load_raw_failures(path: Path = RAW_FAILURES_PATH) -> tuple[FailureRecord, ...]:  # => co-02: reads the real, logged batch of failures from disk
    """Return every line of `path` as a `FailureRecord`, parsed from JSON."""  # => co-02: documents load_raw_failures's contract -- no runtime output, just sets its __doc__
    records: list[FailureRecord] = []  # => co-02: accumulates one FailureRecord per JSONL line
    for line in path.read_text(encoding="utf-8").splitlines():  # => co-02: reads the file once, line by line -- JSONL, not a single JSON array
        data = json.loads(line)  # => co-02: parses this line's raw JSON
        records.append(FailureRecord(request=data["request"], reply=data["reply"], open_code=data["open_code"]))  # => co-02: builds a typed record from the parsed fields
    return tuple(records)  # => co-02: returns this computed value to the caller


def classify_open_code(open_code: str) -> str:  # => co-04: clusters ONE open code into a named mode -- the analyst's own judgment call, made explicit and checkable
    """Return the taxonomy mode name that best matches `open_code`'s wording."""  # => co-04: documents classify_open_code's contract -- no runtime output, just sets its __doc__
    text = open_code.lower()  # => co-04: case-insensitive matching
    if (
        "without any board" in text or "no board" in text or "no ticket" in text or "equally plausible" in text or "guessed a specific" in text or "single guessed ticket" in text
    ):  # => co-04: acted without resolving a genuine ambiguity first
        return "skips-clarifying-question"  # => co-04: matches ex-01's original taxonomy mode
    if "instead of the requested" in text:  # => co-04: acted on the WRONG specific target that WAS named, not an ambiguous one
        return "wrong-object-acted-on"  # => co-04: matches ex-06's original taxonomy mode
    if "counted" in text and "true" in text:  # => co-04: a numeric aggregate that does not match the real count
        return "incorrect-aggregate-count"  # => co-04: matches ex-06's original taxonomy mode
    if "ignoring the stated" in text:  # => co-04: a stated filter condition was present but not applied
        return "ignores-stated-filter-condition"  # => co-04: matches ex-49/ex-77's later-discovered mode
    return "uncategorized"  # => co-04: an explicit fallback -- never silently drops a failure that does not fit a known mode


def build_taxonomy(records: tuple[FailureRecord, ...]) -> tuple[TaxonomyEntry, ...]:  # => co-06: turns classified records into a frequency-ranked taxonomy
    """Return one `TaxonomyEntry` per distinct mode found in `records`, sorted by descending frequency."""  # => co-06: documents build_taxonomy's contract -- no runtime output, just sets its __doc__
    counts: dict[str, int] = {}  # => co-06: tallies how many records fall into each mode
    for r in records:  # => co-04: classifies every real failure
        mode = classify_open_code(r.open_code)  # => co-04: the mode this specific failure belongs to
        counts[mode] = counts.get(mode, 0) + 1  # => co-06: increments this mode's tally
    entries = [TaxonomyEntry(mode_name=name, frequency=freq) for name, freq in counts.items()]  # => co-06: one entry per distinct mode
    return tuple(sorted(entries, key=lambda e: e.frequency, reverse=True))  # => co-06: returns this computed value to the caller -- highest-frequency mode first


if __name__ == "__main__":  # => co-02: entry point -- runs only when this file executes directly, not on import
    failures = load_raw_failures()  # => co-02: load the real, logged batch
    taxonomy = build_taxonomy(failures)  # => co-04/co-06: cluster and rank into a taxonomy
    print(f"Loaded {len(failures)} real failures from {RAW_FAILURES_PATH.name}")  # => co-02: prints the loaded count
    for entry in taxonomy:  # => co-06: prints the ranked taxonomy
        print(f"  {entry.mode_name}: {entry.frequency} occurrences")  # => co-06

    assert len(failures) == 8, "the capstone's own raw-failure batch must contain exactly the eight logged failures on disk"  # => co-02: the rule this example proves
    assert taxonomy[0].mode_name in {"skips-clarifying-question", "wrong-object-acted-on"}, "the DOMINANT mode by frequency must be one of the two most common patterns in this batch"  # => co-06: the rule this example proves
    assert all(e.mode_name != "uncategorized" for e in taxonomy), "every real failure in this curated batch must classify into a NAMED mode, not fall through to the uncategorized fallback"  # => co-04
    print(f"MATCH: {len(failures)} real failures cluster into {len(taxonomy)} named modes, ranked by frequency, with '{taxonomy[0].mode_name}' dominant at {taxonomy[0].frequency} occurrences")  # => co-06
    # => co-04: Step 2 next derives operationalized criteria FROM these exact modes, in criteria.md and labeling-guide.md
```

**Run**: `python3 analysis/error_analysis.py`

**Output**:

```text
Loaded 8 real failures from raw_failures.jsonl
  skips-clarifying-question: 3 occurrences
  incorrect-aggregate-count: 2 occurrences
  ignores-stated-filter-condition: 2 occurrences
  wrong-object-acted-on: 1 occurrences
MATCH: 8 real failures cluster into 4 named modes, ranked by frequency, with 'skips-clarifying-question' dominant at 3 occurrences
```

**Acceptance criteria**: all eight logged failures load and classify into a named mode -- none falls
through to the `uncategorized` fallback (co-04). The dominant mode by frequency,
`skips-clarifying-question` at 3 occurrences, is one of this batch's two most common patterns
(co-01). Both hold, matching the captured output above.

**Key takeaway**: clustering starts from the analyst's own open codes (`"acted on 'this' without
any board or ticket named"`, not a pre-existing category), and only after clustering does a named,
frequency-counted taxonomy emerge -- the taxonomy is an output of reading, never an input to it.

**Why it matters**: every criterion Step 2 derives traces back to one of these four named modes.
Skipping this step and writing criteria from a general sense of "good agent behavior" would
produce criteria that test the team's assumptions about failure, not the failures Tasklight
actually produced -- exactly the substitution co-01 warns against.

---

## Step 2: criteria.md and labeling-guide.md -- derived criteria and a labeling protocol

**Context**: the Beginner tier (Examples 8-13) built one derived criterion, one operationalized
check, and one labeling guide in isolation. This step produces the capstone's own complete set --
four criteria, one per Step 1 taxonomy mode, plus the written protocol two independent labelers
follow to label Step 3's ground-truth cases.

```markdown
# learning/capstone/code/criteria.md

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
```

```markdown
# learning/capstone/code/labeling-guide.md

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
```

**Acceptance criteria**: every one of the four criteria names the exact taxonomy mode it traces to,
with a frequency count that sums to all eight raw failures (co-05) -- the traceability table's
`3 + 1 + 2 + 2 = 8` matches Step 1's own output exactly, with no criterion left unaccounted for.
Each criterion's operationalized check is mechanical enough that Step 3's judge can implement it
as a deterministic-style rule per criterion (co-06). The labeling guide states a specific,
justified agreement threshold (85%, not an arbitrary 100%) and a concrete disagreement-resolution
procedure (co-07).

**Key takeaway**: a criterion with no traceability row is speculation, not evidence -- writing the
traceability table forces every criterion in this capstone to answer "which failure taught you
this," and criterion 3 (`incorrect-aggregate-count`) is the one Step 3's judge will later prove
cannot be reliably automated with a simple heuristic, exactly as a genuinely honest validation
step should reveal.

**Why it matters**: the labeling guide's 85% threshold, decided here before any judge exists, is
what keeps Step 3 honest -- without a threshold committed in advance, it would be tempting to lower
the bar after seeing a disappointing agreement number, which converts "we measured the judge" back
into "we decided the judge was fine," the exact failure mode co-10 exists to prevent.

---

## Step 3: judge.py and ground_truth.jsonl -- a validated judge, retiring what it cannot judge

**Context**: the Intermediate tier (Examples 17-24) built the smallest possible judge and measured
its agreement with a confidence interval. This step assembles the capstone's own thirteen-case
ground-truth set, adjudicated against Step 2's four criteria, and validates a judge -- genuinely a
different model (`tasklight-judge-v1`) from the agent under test (`tasklight-agent-v3`) -- per
criterion, exactly as co-12 and co-16 require.

```jsonl
# learning/capstone/code/ground_truth.jsonl
{"case_id": "gt-01", "criterion_id": 1, "request": "Move this to backlog.", "reply": "Sure -- which board should I move it to backlog on?", "human_label": true}
{"case_id": "gt-02", "criterion_id": 1, "request": "Move that one to the other board.", "reply": "Moved to the Sprint board.", "human_label": false}
{"case_id": "gt-03", "criterion_id": 1, "request": "Close ticket #501.", "reply": "Closed ticket #501.", "human_label": true}
{"case_id": "gt-04", "criterion_id": 2, "request": "Close ticket #12.", "reply": "Closed ticket #12.", "human_label": true}
{"case_id": "gt-05", "criterion_id": 2, "request": "Close ticket #12.", "reply": "Closed ticket #21.", "human_label": false}
{"case_id": "gt-06", "criterion_id": 2, "request": "Raise priority on #4821.", "reply": "Raised priority on ticket #4821 to high.", "human_label": true}
{"case_id": "gt-07", "criterion_id": 3, "request": "How many bugs are open?", "reply": "There are 21 open bugs.", "human_label": true, "reference_count": 21}
{"case_id": "gt-08", "criterion_id": 3, "request": "How many bugs are open?", "reply": "There are 14 open bugs.", "human_label": false, "reference_count": 21}
{"case_id": "gt-09", "criterion_id": 3, "request": "How many tickets are in progress?", "reply": "12 tickets are in progress.", "human_label": true, "reference_count": 12}
{"case_id": "gt-10", "criterion_id": 4, "request": "Archive tickets older than 90 days.", "reply": "Archived the 6 tickets older than 90 days.", "human_label": true}
{"case_id": "gt-11", "criterion_id": 4, "request": "Archive tickets older than 90 days.", "reply": "Archived all tickets.", "human_label": false}
{"case_id": "gt-12", "criterion_id": 4, "request": "Delete tickets tagged 'spam'.", "reply": "Deleted the 3 tickets tagged 'spam'.", "human_label": true}
{"case_id": "gt-13", "criterion_id": 3, "request": "How many tickets are overdue?", "reply": "Roughly a dozen tickets are overdue.", "human_label": false, "reference_count": 15}
```

```python
# learning/capstone/code/judge.py
"""Capstone Step 3: A Judge on a Different Model Than the Generator, Validated Per Criterion With a CI."""  # => co-09/co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-11: ground_truth.jsonl is read as real, adjudicated JSON lines
import math  # => co-10: the Wilson score interval needs only sqrt -- no external stats library required
import re  # => co-09: the criterion-3 judge heuristic extracts a digit sequence from free text
from pathlib import Path  # => co-11: locates ground_truth.jsonl relative to this file, not the caller's cwd
from typing import NamedTuple  # => co-11: every stage below uses a typed record, not a bare dict

GROUND_TRUTH_PATH = Path(__file__).parent / "ground_truth.jsonl"  # => co-11: resolved relative to THIS file
AGREEMENT_THRESHOLD = 0.85  # => co-12: labeling-guide.md's own justified threshold, reused here as the judge's validation bar

# The AGENT under test (the "generator") is a DIFFERENT model from this judge -- per co-16's rule,
# never use the same model to both generate and grade its own output. This judge is a smaller,
# cheaper, independently-configured model, illustrated here as a set of deterministic heuristics
# standing in for a real second model's API responses.
GENERATOR_MODEL_NAME = "tasklight-agent-v3"  # => co-16: the model under test
JUDGE_MODEL_NAME = "tasklight-judge-v1"  # => co-16: a DIFFERENT, separately-configured model doing the grading


class GroundTruthCase(NamedTuple):  # => co-11: one adjudicated, human-labeled case
    case_id: str  # => co-11
    criterion_id: int  # => co-11: which of criteria.md's four criteria this case tests
    request: str  # => co-11
    reply: str  # => co-11
    human_label: bool  # => co-11: the adjudicated ground truth
    reference_count: int | None  # => co-11: only present for criterion-3 (aggregate-count) cases


class AgreementInterval(NamedTuple):  # => co-10: a proportion, reported WITH its uncertainty, never alone -- same shape as ex-19
    criterion_id: int  # => co-12: scoped PER criterion, not one blended figure
    point_estimate: float  # => co-10: the raw agreement rate
    lower_bound: float  # => co-10: the interval's lower edge
    upper_bound: float  # => co-10: the interval's upper edge
    sample_size: int  # => co-10: how many cases this criterion's estimate rests on


def _as_int(value: object) -> int:  # => co-11: a strict-typing-safe narrowing helper -- int(object) is rejected by pyright strict, this is the explicit fix
    """Return `value` as an int, raising TypeError if it is not already an int (JSON numbers decode as int or float, never bool-like ints here)."""  # => co-11: documents _as_int's contract -- no runtime output, just sets its __doc__
    if isinstance(value, int) and not isinstance(value, bool):  # => co-11: excludes bool, since bool is technically an int subclass in Python
        return value  # => co-11: returns this computed value to the caller
    raise TypeError(f"expected an int, got {value!r}")  # => co-11: fails loudly on a malformed record rather than silently coercing


def _as_bool(value: object) -> bool:  # => co-11: a strict-typing-safe narrowing helper -- bool(object) is rejected by pyright strict, this is the explicit fix
    """Return `value` as a bool, raising TypeError if it is not already a bool."""  # => co-11: documents _as_bool's contract -- no runtime output, just sets its __doc__
    if isinstance(value, bool):  # => co-11: narrows the object type to bool explicitly
        return value  # => co-11: returns this computed value to the caller
    raise TypeError(f"expected a bool, got {value!r}")  # => co-11: fails loudly on a malformed record rather than silently coercing


def load_ground_truth(path: Path = GROUND_TRUTH_PATH) -> tuple[GroundTruthCase, ...]:  # => co-11: reads the real, adjudicated reference set from disk
    """Return every line of `path` as a `GroundTruthCase`, parsed from JSON."""  # => co-11: documents load_ground_truth's contract -- no runtime output, just sets its __doc__
    cases: list[GroundTruthCase] = []  # => co-11: accumulates one GroundTruthCase per JSONL line
    for line in path.read_text(encoding="utf-8").splitlines():  # => co-11: reads the file once, line by line
        data: dict[str, object] = json.loads(line)  # => co-11: parses this line's raw JSON
        cases.append(  # => co-11: builds a typed record from the parsed fields
            GroundTruthCase(  # => co-11
                case_id=str(data["case_id"]),  # => co-11
                criterion_id=_as_int(data["criterion_id"]),  # => co-11
                request=str(data["request"]),  # => co-11
                reply=str(data["reply"]),  # => co-11
                human_label=_as_bool(data["human_label"]),  # => co-11
                reference_count=_as_int(data["reference_count"]) if "reference_count" in data else None,  # => co-11: absent for criteria that do not need a numeric reference
            )  # => co-11
        )  # => co-11: closes cases.append(...)
    return tuple(cases)  # => co-11: returns this computed value to the caller


def mock_judge_verdict(case: GroundTruthCase) -> bool:  # => co-09: the judge model's own verdict, scoped per criterion -- one heuristic branch per criterion, standing in for one judge prompt per criterion
    """Return the judge's True/False verdict for `case`, using per-criterion logic."""  # => co-09: documents mock_judge_verdict's contract -- no runtime output, just sets its __doc__
    if case.criterion_id == 1:  # => co-09: criterion 1 -- asks before acting on an ambiguous target
        request_names_explicit_id = bool(re.search(r"#\d+", case.request))  # => co-09: if the REQUEST already names an explicit ticket ID, there was no real ambiguity to clarify
        return "which" in case.reply.lower() or request_names_explicit_id  # => co-09: the judge's own read of criterion 1 -- passes if it asked, OR if nothing needed asking
    if case.criterion_id == 2:  # => co-09: criterion 2 -- acts on the exact target named
        requested_id = re.search(r"#(\d+)", case.request)  # => co-09: extracts the requested ticket ID
        acted_id = re.search(r"#(\d+)", case.reply)  # => co-09: extracts the ID the reply actually acted on
        return bool(requested_id and acted_id and requested_id.group(1) == acted_id.group(1))  # => co-09: the judge's own read of criterion 2
    if case.criterion_id == 3:  # => co-09: criterion 3 -- reports the true aggregate count
        found_digits = re.search(r"\d+", case.reply)  # => co-09: a REAL judge limitation -- this heuristic can only extract a plain digit sequence
        if found_digits is None:  # => co-09: the reply used a vague phrase ("roughly a dozen") with no extractable digit
            return True  # => co-09: a genuine, documented judge FAILURE MODE -- defaults to True when it cannot parse a number, rather than correctly flagging vagueness as wrong
        return int(found_digits.group(0)) == case.reference_count  # => co-09: the judge's own read of criterion 3 when a digit IS present
    if "all tickets" in case.reply.lower():  # => co-09: criterion 4 -- applies every stated filter condition; "all tickets" signals the filter was ignored
        return False  # => co-09: the judge's own read of criterion 4
    return True  # => co-09: no "all tickets" phrase -- the judge reads this as the filter having been applied


def wilson_interval(successes: int, total: int, *, criterion_id: int, z: float = 1.96) -> AgreementInterval:  # => co-10: the SAME Wilson interval formula as ex-19, scoped here per criterion
    """Compute a Wilson score confidence interval for `successes`/`total`, tagged with `criterion_id`."""  # => co-10: documents wilson_interval's contract -- no runtime output, just sets its __doc__
    p_hat = successes / total  # => co-10: the raw point estimate for THIS criterion
    denominator = 1 + z**2 / total  # => co-10: Wilson's correction term
    center = p_hat + z**2 / (2 * total)  # => co-10: the interval's recentered midpoint
    spread = z * math.sqrt(p_hat * (1 - p_hat) / total + z**2 / (4 * total**2))  # => co-10: the interval's half-width
    lower = max(0.0, (center - spread) / denominator)  # => co-10: clamps to a valid probability floor
    upper = min(1.0, (center + spread) / denominator)  # => co-10: clamps to a valid probability ceiling
    return AgreementInterval(criterion_id=criterion_id, point_estimate=p_hat, lower_bound=lower, upper_bound=upper, sample_size=total)  # => co-10: returns this computed value to the caller


def measure_agreement_per_criterion(cases: tuple[GroundTruthCase, ...]) -> tuple[AgreementInterval, ...]:  # => co-12: agreement is measured PER criterion -- never one blended figure across unrelated criteria
    """Return one `AgreementInterval` per distinct `criterion_id` present in `cases`."""  # => co-12: documents measure_agreement_per_criterion's contract -- no runtime output, just sets its __doc__
    by_criterion: dict[int, list[bool]] = {}  # => co-12: groups per-case match results by criterion
    for case in cases:  # => co-09: run the judge on every case
        judge_verdict = mock_judge_verdict(case)  # => co-09: this case's judge verdict
        matches = judge_verdict == case.human_label  # => co-17: measured agreement, not assumed
        by_criterion.setdefault(case.criterion_id, []).append(matches)  # => co-12: records this case's match under its own criterion
    intervals = [wilson_interval(sum(matches), len(matches), criterion_id=cid) for cid, matches in sorted(by_criterion.items())]  # => co-10: one CI per criterion, criteria in ascending order
    return tuple(intervals)  # => co-12: returns this computed value to the caller


def position_bias_probe() -> bool:  # => co-13: the SAME position-bias probe pattern as ex-79, run here as part of the capstone's own validation
    """Return True iff a position-bias probe detects a flip purely from slot order."""  # => co-13: documents position_bias_probe's contract -- no runtime output, just sets its __doc__
    verdict_first = "compliant"  # => co-13: mock judge verdict when the compliant reply is shown first
    verdict_second = "compliant"  # => co-13: the SAME mock judge verdict when the compliant reply is shown second
    return verdict_first != verdict_second  # => co-13: returns this computed value to the caller


def verbosity_bias_probe() -> bool:  # => co-13: the SAME verbosity-bias probe pattern as ex-79
    """Return True iff a padded, longer reply scores HIGHER than an equally-correct short one."""  # => co-13: documents verbosity_bias_probe's contract -- no runtime output, just sets its __doc__
    short_score, long_score = 8, 6  # => co-13: mock judge scores -- the longer, padded reply correctly scores LOWER
    return long_score > short_score  # => co-13: returns this computed value to the caller


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    print(f"Generator model: {GENERATOR_MODEL_NAME}, judge model: {JUDGE_MODEL_NAME} (different models, per co-16)")  # => co-16: states the model separation explicitly
    ground_truth = load_ground_truth()  # => co-11: load the real, adjudicated reference set
    intervals = measure_agreement_per_criterion(ground_truth)  # => co-12: measure agreement, scoped per criterion, with a CI

    retired_criteria: list[int] = []  # => co-22: criteria whose judge falls below threshold get retired, not shipped
    for interval in intervals:  # => co-12: reports and validates every criterion independently
        status = "OK" if interval.point_estimate >= AGREEMENT_THRESHOLD else "RETIRE -- use the deterministic reference-based scorer instead"  # => co-22
        print(f"Criterion {interval.criterion_id}: agreement {interval.point_estimate:.0%} [{interval.lower_bound:.0%}, {interval.upper_bound:.0%}] on n={interval.sample_size} -- {status}")  # => co-12
        if interval.point_estimate < AGREEMENT_THRESHOLD:  # => co-22: below the justified threshold
            retired_criteria.append(interval.criterion_id)  # => co-22: tracks which criteria this judge must NOT be trusted on

    position_biased = position_bias_probe()  # => co-13: run the position-bias probe
    verbosity_biased = verbosity_bias_probe()  # => co-13: run the verbosity-bias probe
    print(f"Position-bias probe: bias_detected={position_biased}")  # => co-13
    print(f"Verbosity-bias probe: bias_detected={verbosity_biased}")  # => co-13
    print(f"Criteria retired (judge below {AGREEMENT_THRESHOLD:.0%} threshold): {retired_criteria}")  # => co-22

    assert len(ground_truth) == 13, "the capstone's own ground-truth set must contain exactly the thirteen adjudicated cases on disk"  # => co-11: the rule this example proves
    assert len(intervals) == 4, "agreement must be measured for all four of criteria.md's criteria, none skipped"  # => co-12: the rule this example proves
    assert retired_criteria == [3], "criterion 3's judge -- which cannot parse vague count phrases like 'roughly a dozen' -- must fall below threshold and be retired"  # => co-22: the rule this example proves
    assert position_biased is False and verbosity_biased is False, "the judge must pass both bias probes before being trusted on the criteria it was NOT retired from"  # => co-13: the rule this example proves
    print("MATCH: agreement measured per criterion with a Wilson CI, criterion 3 correctly retired (a real judge limitation, not a made-up one), and both bias probes pass clean")  # => co-12
    # => co-16: Step 4 next scores the agent's TRAJECTORY, not just its final answer, using the criteria that survived validation
```

**Run**: `python3 judge.py`

**Output**:

```text
Generator model: tasklight-agent-v3, judge model: tasklight-judge-v1 (different models, per co-16)
Criterion 1: agreement 100% [44%, 100%] on n=3 -- OK
Criterion 2: agreement 100% [44%, 100%] on n=3 -- OK
Criterion 3: agreement 75% [30%, 95%] on n=4 -- RETIRE -- use the deterministic reference-based scorer instead
Criterion 4: agreement 100% [44%, 100%] on n=3 -- OK
Position-bias probe: bias_detected=False
Verbosity-bias probe: bias_detected=False
Criteria retired (judge below 85% threshold): [3]
MATCH: agreement measured per criterion with a Wilson CI, criterion 3 correctly retired (a real judge limitation, not a made-up one), and both bias probes pass clean
```

**Acceptance criteria**: agreement is measured and reported separately for all four criteria, each
with its own Wilson confidence interval and sample size (co-11) -- never one blended figure.
Criterion 3 measures 75% agreement, below the 85% threshold `labeling-guide.md` set in Step 2, and
is correctly retired rather than shipped anyway (co-22); the judge's own failure here is genuine --
it cannot parse the vague phrase `"roughly a dozen"` in ground-truth case `gt-13` into a number, so
it defaults to `True` and disagrees with the human's `false` label. Both bias probes (position,
verbosity) come back clean (co-13). All hold, matching the captured output above.

**Key takeaway**: a judge is not one global pass/fail -- it is trustworthy on criteria 1, 2, and 4,
and explicitly not trustworthy on criterion 3, and shipping it anyway on criterion 3 (rather than
retiring it in favor of a deterministic reference-count scorer) would silently corrupt every future
merge decision that depends on that one criterion.

**Why it matters**: retiring criterion 3 here is this capstone's single clearest demonstration of
co-10's rule in action -- an unvalidated judge would have shipped all four criteria un-retired,
looking identical from the outside to this validated one, right up until it silently mis-scored
every vaguely-phrased count reply in production.

---

## Step 4: trajectory.py -- process fails, outcome passes, failure attributed to step 1

**Context**: the Advanced tier (Examples 35-40) built trajectory capture, process/outcome scoring,
and step attribution as separate pieces. This step combines them against two capstone-scale
trajectories -- one the syllabus's own required right-answer-wrong-path case, one a fully correct
trajectory for contrast.

```python
# learning/capstone/code/trajectory.py
"""Capstone Step 4: Score the Agent's Trajectory Alongside Its Final Answer, With Step Attribution."""  # => co-18/co-19/co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-18: every record below is typed, not a bare dict


class ToolCall(NamedTuple):  # => co-18: one step in a trajectory -- the same shape ex-35 first established
    tool_name: str  # => co-18: which tool was invoked
    result: str  # => co-18: what the tool returned
    step_is_correct: bool  # => co-20: ground truth -- was THIS step, in isolation, correct?


class Trajectory(NamedTuple):  # => co-18: the full sequence of steps plus a final answer
    case_id: str  # => co-18: which capstone case this trajectory belongs to
    steps: tuple[ToolCall, ...]  # => co-18: every tool call, in order
    final_answer: str  # => co-18: what the agent ultimately told the user


class CaseVerdict(NamedTuple):  # => co-19: the full, combined verdict for one trajectory -- process, outcome, and attribution together
    case_id: str  # => co-19
    process_passed: bool  # => co-19: did the trajectory follow the sanctioned tool sequence?
    outcome_passed: bool  # => co-19: is the final answer correct?
    first_failing_step_index: int | None  # => co-20: WHERE, if anywhere, the trajectory first went wrong


REFERENCE_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "update_priority")  # => co-18: the sanctioned path for this task, reused from ex-36/ex-37

# Two capstone-scale trajectories: one a genuine right-answer-wrong-path case (per the syllabus's
# OWN acceptance criterion for this step), and one a fully correct trajectory for contrast.
RIGHT_ANSWER_WRONG_PATH = Trajectory(  # => co-19: the syllabus's own required scenario -- process fails, outcome passes
    case_id="traj-01",  # => co-18
    steps=(  # => co-18: SKIPPED get_ticket -- guessed the ticket's state instead of verifying it
        ToolCall("search_ticket", "found ticket #4821", step_is_correct=True),  # => co-20: step 0 -- correct
        ToolCall("update_priority", "priority updated to high", step_is_correct=False),  # => co-20: step 1 -- WRONG: acted without verifying via get_ticket first
    ),  # => co-18: closes steps
    final_answer="I found ticket #4821 and raised its priority to high.",  # => co-19: happens to be the CORRECT final answer anyway
)  # => co-18: closes RIGHT_ANSWER_WRONG_PATH

FULLY_CORRECT_TRAJECTORY = Trajectory(  # => co-19: both process and outcome correct, for contrast
    case_id="traj-02",  # => co-18
    steps=(  # => co-18: follows the full sanctioned sequence
        ToolCall("search_ticket", "found ticket #501", step_is_correct=True),  # => co-20: step 0 -- correct
        ToolCall("get_ticket", "status=open, priority=low", step_is_correct=True),  # => co-20: step 1 -- correct
        ToolCall("update_priority", "priority updated to high", step_is_correct=True),  # => co-20: step 2 -- correct
    ),  # => co-18: closes steps
    final_answer="I found ticket #501 and raised its priority to high.",  # => co-19: correct final answer
)  # => co-18: closes FULLY_CORRECT_TRAJECTORY


def process_score(trajectory: Trajectory, *, reference: tuple[str, ...] = REFERENCE_TOOL_SEQUENCE) -> bool:  # => co-19: the SAME process scorer as ex-37/ex-38
    """Pass iff `trajectory`'s tool names match `reference` exactly, in order."""  # => co-19: documents process_score's contract -- no runtime output, just sets its __doc__
    tool_names = tuple(step.tool_name for step in trajectory.steps)  # => co-19: extracts just the tool-name sequence
    return tool_names == reference  # => co-19: returns this computed value to the caller


def outcome_score(trajectory: Trajectory, *, expected_ticket_id: str = "4821") -> bool:  # => co-19: an outcome scorer keyed to THIS trajectory's own expected ticket
    """Pass iff `trajectory.final_answer` mentions the ticket ID actually found in its first step."""  # => co-19: documents outcome_score's contract -- no runtime output, just sets its __doc__
    found_id = next((step.result.split("#")[1].split()[0] for step in trajectory.steps if step.tool_name == "search_ticket" and "#" in step.result), None)  # => co-19: the ticket ID this trajectory's own search_ticket step actually found
    return found_id is not None and found_id in trajectory.final_answer  # => co-19: returns this computed value to the caller -- consistency between what was found and what was reported


def find_first_failing_step(trajectory: Trajectory) -> int | None:  # => co-20: the SAME step-attribution pattern as ex-39
    """Return the 0-based index of the first step where `step_is_correct` is False, or None."""  # => co-20: documents find_first_failing_step's contract -- no runtime output, just sets its __doc__
    for index, step in enumerate(trajectory.steps):  # => co-20: scans steps in order
        if not step.step_is_correct:  # => co-20: found the causing step
            return index  # => co-20: returns this computed value to the caller
    return None  # => co-20: every step was individually correct


def score_case(trajectory: Trajectory) -> CaseVerdict:  # => co-19: combines process, outcome, and attribution into ONE reported verdict per case
    """Return a `CaseVerdict` combining process scoring, outcome scoring, and step attribution for `trajectory`."""  # => co-19: documents score_case's contract -- no runtime output, just sets its __doc__
    return CaseVerdict(  # => co-19: returns this computed value to the caller
        case_id=trajectory.case_id,  # => co-18
        process_passed=process_score(trajectory),  # => co-19
        outcome_passed=outcome_score(trajectory),  # => co-19
        first_failing_step_index=find_first_failing_step(trajectory),  # => co-20
    )  # => co-19: closes the CaseVerdict(...) call


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    verdict_1 = score_case(RIGHT_ANSWER_WRONG_PATH)  # => co-19: score the right-answer-wrong-path case
    verdict_2 = score_case(FULLY_CORRECT_TRAJECTORY)  # => co-19: score the fully correct case, for contrast
    print(f"{verdict_1.case_id}: process={verdict_1.process_passed}, outcome={verdict_1.outcome_passed}, first_failing_step={verdict_1.first_failing_step_index}")  # => co-19
    print(f"{verdict_2.case_id}: process={verdict_2.process_passed}, outcome={verdict_2.outcome_passed}, first_failing_step={verdict_2.first_failing_step_index}")  # => co-19

    assert verdict_1.process_passed is False, "traj-01 must FAIL process scoring -- it skipped get_ticket's verification step"  # => co-19: the syllabus's own required acceptance check
    assert verdict_1.outcome_passed is True, "traj-01 must PASS outcome scoring -- the final answer happens to be correct"  # => co-19: the syllabus's own required acceptance check
    assert verdict_1.first_failing_step_index == 1, "traj-01's failure must be attributed to step 1 (update_priority), the step that skipped verification"  # => co-20
    assert verdict_2.process_passed is True and verdict_2.outcome_passed is True, "traj-02, following the full sanctioned sequence, must pass BOTH process and outcome scoring"  # => co-19
    assert verdict_2.first_failing_step_index is None, "traj-02 has no failing step to attribute -- every step was individually correct"  # => co-20
    print("MATCH: traj-01 demonstrates the syllabus's own required right-answer-wrong-path case -- process fails while outcome passes -- with the failure correctly attributed to step 1")  # => co-19
    # => co-20: Step 5 next wires ALL of this (criteria, judge, trajectory scoring) into a noise-aware CI gate
```

**Run**: `python3 trajectory.py`

**Output**:

```text
traj-01: process=False, outcome=True, first_failing_step=1
traj-02: process=True, outcome=True, first_failing_step=None
MATCH: traj-01 demonstrates the syllabus's own required right-answer-wrong-path case -- process fails while outcome passes -- with the failure correctly attributed to step 1
```

**Acceptance criteria**: `traj-01` fails process scoring (it skipped the sanctioned `get_ticket`
verification step) while passing outcome scoring (its final answer happens to name the correct
ticket anyway), with the failure attributed exactly to step 1, `update_priority` (co-19, co-20).
`traj-02`, which follows the full sanctioned tool sequence, passes both process and outcome
scoring with no failing step to attribute. Both hold, matching the captured output above.

**Key takeaway**: `traj-01`'s outcome-only score would have looked identical to `traj-02`'s --
both report a correct final answer -- and only process scoring, plus step attribution, reveals that
`traj-01` reached its correct answer by skipping a required verification step, a latent failure
that a different ticket ID would have turned into a wrong final answer.

**Why it matters**: this is exactly the gap co-19 exists to close -- an eval suite that only checks
final answers would have passed `traj-01` outright, and a team relying on that suite alone would
discover the skipped-verification pattern only once it produces a visibly wrong answer on a
different, less-lucky input.

---

## Step 5: ci/ci_gate.py -- a noise-aware, tiered, cost-budgeted merge gate

**Context**: the Advanced tier (Examples 44-48) built the noise floor, the regression bar, the CI
gate decision, and the cost report as separate pieces. This step wires all four together into the
capstone's own closing pipeline, using the fast tier's own repeated-run measurements as the
noise-floor input.

```python
# learning/capstone/code/ci/ci_gate.py
"""Capstone Step 5: Noise-Aware Regression Bar, Tiered Suites, and a Judge-Call Cost Budget."""  # => co-23/co-24/co-26: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import statistics  # => co-24: stdlib mean/stdev, the same tool ex-44 used for the noise floor
from typing import NamedTuple  # => co-23: every record below is typed, not a bare dict


class GateResult(NamedTuple):  # => co-23: the CI gate's own reasoned verdict -- same shape as ex-46
    merge_allowed: bool  # => co-23: the gate's binary decision
    observed_pass_rate: float  # => co-23: what THIS candidate run scored
    bar: float  # => co-23: the regression bar this run was checked against
    reason: str  # => co-23: a human-readable explanation, for the CI log


class TierRun(NamedTuple):  # => co-26: one tier's own execution record
    tier_name: str  # => co-26
    case_count: int  # => co-26
    uses_llm_judge: bool  # => co-25: whether this tier's cost includes judge calls
    pass_rate: float  # => co-26


class CostReport(NamedTuple):  # => co-25: the SAME cost-accounting shape as ex-48
    judge_call_count: int  # => co-25
    cost_per_call_usd: float  # => co-25
    total_cost_usd: float  # => co-25
    budget_usd: float  # => co-25
    over_budget: bool  # => co-25


# Five repeated, unchanged runs of the capstone's OWN combined fast tier (criteria 1/2/4, all above
# threshold per judge.py) -- establishes THIS suite's own measured noise floor, not an assumed one.
REPEATED_UNCHANGED_FAST_TIER_RUNS = (0.90, 0.94, 0.88, 0.92, 0.90)  # => co-24: illustrative repeated pass rates for the capstone's own fast tier

COST_PER_JUDGE_CALL_USD = 0.018  # => co-25: reused from ex-48's illustrative per-call price
BUDGET_PER_RUN_USD = 5.00  # => co-25: reused from ex-48's fixed per-CI-run ceiling


def measure_noise_floor(repeated_pass_rates: tuple[float, ...]) -> tuple[float, float]:  # => co-24: the SAME (mean, stdev) computation as ex-44
    """Return `(mean, standard_deviation)` of `repeated_pass_rates`."""  # => co-24: documents measure_noise_floor's contract -- no runtime output, just sets its __doc__
    return statistics.mean(repeated_pass_rates), statistics.stdev(repeated_pass_rates)  # => co-24: returns this computed value to the caller


def set_regression_bar(baseline: float, noise_floor: float, *, multiple: float = 2.0) -> float:  # => co-23: the SAME derivation as ex-45
    """Return `baseline - multiple * noise_floor`."""  # => co-23: documents set_regression_bar's contract -- no runtime output, just sets its __doc__
    return baseline - multiple * noise_floor  # => co-23: returns this computed value to the caller


def run_eval_gate(candidate_pass_rate: float, *, bar: float) -> GateResult:  # => co-23: the SAME CI-gate decision function as ex-46
    """Return a `GateResult` deciding whether `candidate_pass_rate` clears `bar`."""  # => co-23: documents run_eval_gate's contract -- no runtime output, just sets its __doc__
    if candidate_pass_rate < bar:  # => co-23: below the bar -- a real regression
        return GateResult(merge_allowed=False, observed_pass_rate=candidate_pass_rate, bar=bar, reason=f"pass rate {candidate_pass_rate:.1%} fell below the regression bar {bar:.1%}")  # => co-23: returns this computed value to the caller
    return GateResult(merge_allowed=True, observed_pass_rate=candidate_pass_rate, bar=bar, reason=f"pass rate {candidate_pass_rate:.1%} cleared the regression bar {bar:.1%}")  # => co-23: returns this computed value to the caller


def build_cost_report(judge_call_count: int, *, cost_per_call: float = COST_PER_JUDGE_CALL_USD, budget: float = BUDGET_PER_RUN_USD) -> CostReport:  # => co-25: the SAME cost accounting as ex-48
    """Return a `CostReport` for `judge_call_count` judge calls."""  # => co-25: documents build_cost_report's contract -- no runtime output, just sets its __doc__
    total = judge_call_count * cost_per_call  # => co-25: the run's actual total spend
    return CostReport(judge_call_count=judge_call_count, cost_per_call_usd=cost_per_call, total_cost_usd=total, budget_usd=budget, over_budget=total > budget)  # => co-25: returns this computed value to the caller


if __name__ == "__main__":  # => co-26: entry point -- runs only when this file executes directly, not on import
    baseline, noise_floor = measure_noise_floor(REPEATED_UNCHANGED_FAST_TIER_RUNS)  # => co-24: measure this capstone suite's own noise floor
    bar = set_regression_bar(baseline, noise_floor)  # => co-23: derive the bar from the MEASURED noise floor
    print(f"Fast-tier baseline: {baseline:.1%}, noise floor: {noise_floor:.1%}, regression bar: {bar:.1%}")  # => co-23/co-24

    within_noise_candidate = run_eval_gate(0.89, bar=bar)  # => co-24: a candidate that dips, but stays within noise
    real_regression_candidate = run_eval_gate(0.74, bar=bar)  # => co-23: a candidate that genuinely regressed
    print(f"Within-noise candidate: merge_allowed={within_noise_candidate.merge_allowed} ({within_noise_candidate.reason})")  # => co-24
    print(f"Real-regression candidate: merge_allowed={real_regression_candidate.merge_allowed} ({real_regression_candidate.reason})")  # => co-23

    fast_tier = TierRun(tier_name="fast-deterministic", case_count=13, uses_llm_judge=False, pass_rate=baseline)  # => co-26: the capstone's own 13-case ground-truth-scale fast tier
    judged_tier = TierRun(tier_name="llm-judged-merge", case_count=13, uses_llm_judge=True, pass_rate=0.92)  # => co-26: the same 13 cases, re-checked with the validated judge before merge
    tiers = (fast_tier, judged_tier)  # => co-26: both tiers, wired into one pipeline
    print(f"Tiers: {[(t.tier_name, t.uses_llm_judge) for t in tiers]}")  # => co-26

    judged_call_count = judged_tier.case_count  # => co-25: one judge call per case in the judged tier
    cost_report = build_cost_report(judged_call_count)  # => co-25: report and budget the judged tier's own cost
    print(f"Judged-tier cost: {cost_report.judge_call_count} calls, ${cost_report.total_cost_usd:.2f}, over budget: {cost_report.over_budget}")  # => co-25

    assert real_regression_candidate.merge_allowed is False, "a genuine regression must BLOCK the merge"  # => co-23: the syllabus's own required acceptance check
    assert within_noise_candidate.merge_allowed is True, "a within-noise change must NOT block the merge"  # => co-24: the syllabus's own required acceptance check
    assert any(t.uses_llm_judge for t in tiers) and any(not t.uses_llm_judge for t in tiers), "the pipeline must wire BOTH a fast deterministic tier and a judged merge tier"  # => co-26: the rule this example proves
    assert cost_report.over_budget is False, "the judged tier's cost must be reported AND confirmed within budget, not merely reported"  # => co-25: the syllabus's own required acceptance check
    print(
        f"MATCH: a real regression ({real_regression_candidate.observed_pass_rate:.1%}) blocks the merge, a within-noise change ({within_noise_candidate.observed_pass_rate:.1%}) does not, and the judged tier's ${cost_report.total_cost_usd:.2f} cost is reported and confirmed within its ${cost_report.budget_usd:.2f} budget"
    )  # => co-26
    # => co-28: this closes the capstone's five ordered steps; the overview's own final section states what this suite still cannot catch
```

**Run**: `python3 ci/ci_gate.py`

**Output**:

```text
Fast-tier baseline: 90.8%, noise floor: 2.3%, regression bar: 86.2%
Within-noise candidate: merge_allowed=True (pass rate 89.0% cleared the regression bar 86.2%)
Real-regression candidate: merge_allowed=False (pass rate 74.0% fell below the regression bar 86.2%)
Tiers: [('fast-deterministic', False), ('llm-judged-merge', True)]
Judged-tier cost: 13 calls, $0.23, over budget: False
MATCH: a real regression (74.0%) blocks the merge, a within-noise change (89.0%) does not, and the judged tier's $0.23 cost is reported and confirmed within its $5.00 budget
```

**Acceptance criteria**: the regression bar (86.2%) is derived from the fast tier's own measured
noise floor (2.3% standard deviation across five repeated unchanged runs), not an arbitrary
round number (co-24). A genuine regression (74.0%) is blocked; a within-noise dip (89.0%) is not
(co-23). Both a fast deterministic tier and a judged merge tier are wired into the pipeline
(co-26), and the judged tier's real cost ($0.23 for 13 judge calls) is reported and confirmed
within its $5.00 budget, not merely reported (co-25). All hold, matching the captured output
above.

**Key takeaway**: the same 3-point pass-rate dip that would have blocked a merge under a
naively-set 90%-flat bar is correctly let through once the bar accounts for the suite's own
measured 2.3% noise -- and the 16-point drop to 74.0% is still caught, because it is far larger
than noise could plausibly explain.

**Why it matters**: this step closes the loop the whole capstone builds toward -- a taxonomy
(Step 1) becomes criteria (Step 2), criteria become a validated judge (Step 3), the judge and a
process scorer together score an agent's real behavior (Step 4), and the merge gate (Step 5)
converts all of that into one reasoned, cost-bounded, noise-aware decision a CI pipeline can act
on without a human re-deriving it for every pull request. What this five-step system still cannot
catch -- a novel failure mode never present in the original eight raw failures -- is exactly what
Example 78 and the topic overview's own scope note (co-28) name explicitly, rather than implying
this gate is a complete safety net.

---

← Previous: [Advanced Examples](../advanced.md) &middot; Next: [Drilling](../../drilling/overview.md) →
