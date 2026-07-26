"""Worked Example 14: Assemble the Adjudicated Labels Into a Versioned, Schema-Valid Reference Set."""  # => co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-08: the ground-truth set is a plain, versionable JSONL file
from typing import TypedDict  # => co-08: GroundTruthCase types every field a downstream scorer/judge can rely on


class GroundTruthCase(TypedDict):  # => co-08: the minimal schema every adjudicated case must satisfy
    ticket_id: str  # => co-08: the case's stable identifier
    answer: str  # => co-08: the exact model reply this label was assigned to
    criterion: str  # => co-08: which criterion (co-06) this case was labeled against
    final_verdict: bool  # => co-08: the adjudicated (ex-13), human-agreed pass/fail
    schema_version: int  # => co-08: versioned, so a later scorer knows exactly which schema it is reading


REQUIRED_KEYS = {"ticket_id", "answer", "criterion", "final_verdict", "schema_version"}  # => co-08: the exact fields a valid case must have

# The ground-truth set -- adjudicated labels from ex-12/ex-13, assembled into one reference file.
GROUND_TRUTH_SET: list[GroundTruthCase] = [  # => co-08: every case here traces back to a real, resolved label
    {"ticket_id": "t-401", "answer": "There are 5 open critical bugs.", "criterion": "count-accuracy", "final_verdict": True, "schema_version": 1},  # => co-08
    {"ticket_id": "t-403", "answer": "There are 3 open critical bugs.", "criterion": "count-accuracy", "final_verdict": False, "schema_version": 1},  # => co-08
    {"ticket_id": "t-501", "answer": "Around 5, roughly.", "criterion": "count-accuracy", "final_verdict": False, "schema_version": 1},  # => co-08: from ex-13's adjudication
    {"ticket_id": "t-502", "answer": "Exactly 5 critical bugs.", "criterion": "count-accuracy", "final_verdict": True, "schema_version": 1},  # => co-08: from ex-13's adjudication
]  # => co-08: closes GROUND_TRUTH_SET -- four adjudicated, schema-valid cases


def validate_case(case: GroundTruthCase) -> tuple[bool, str]:  # => co-08: every case is checked against the schema before it's trusted
    """Return (valid, reason) -- valid iff every REQUIRED_KEYS entry is present in `case`."""  # => co-08: documents validate_case's contract -- no runtime output, just sets its __doc__
    present = set(case.keys())  # => co-08: what this case actually declares
    missing = REQUIRED_KEYS - present  # => co-08: what the schema demands but this case omits
    return (len(missing) == 0, "valid" if not missing else f"missing keys: {sorted(missing)}")  # => co-08: returns this computed value to the caller


if __name__ == "__main__":  # => co-08: entry point -- runs only when this file executes directly, not on import
    as_jsonl = "\n".join(json.dumps(case, sort_keys=True) for case in GROUND_TRUTH_SET)  # => co-08: the versionable, diffable serialization
    print(as_jsonl)  # => co-08: prints the ground-truth set exactly as it would be committed
    validations = [validate_case(case) for case in GROUND_TRUTH_SET]  # => co-08: schema-check every case
    all_valid = all(v for v, _ in validations)  # => co-08: the whole set's schema verdict
    print(f"All {len(GROUND_TRUTH_SET)} cases schema-valid: {all_valid}")  # => co-08: True -- every case satisfies REQUIRED_KEYS

    assert all_valid, "every case in the ground-truth set must satisfy the schema"  # => co-08: the rule this example proves
    reparsed = [json.loads(line) for line in as_jsonl.splitlines()]  # => co-08: round-trip through the serialization
    assert reparsed == GROUND_TRUTH_SET, "the serialized JSONL must round-trip back to the identical set -- proves it is genuinely versionable"  # => co-08
    print(f"MATCH: {len(GROUND_TRUTH_SET)} adjudicated cases assembled into a schema-valid, round-trippable ground-truth set")  # => co-08
    # => co-08: this reference set is what EVERY automated scorer -- deterministic or judge -- gets validated against, starting in ex-16
