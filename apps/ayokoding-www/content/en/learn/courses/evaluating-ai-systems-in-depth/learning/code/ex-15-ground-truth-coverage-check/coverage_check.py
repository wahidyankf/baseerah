"""Worked Example 15: Verify the Reference Set Covers Every Failure Mode in the Taxonomy."""  # => co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-08: GroundTruthCase mirrors ex-14's schema, extended with a mode tag


class GroundTruthCase(TypedDict):  # => co-08: ex-14's schema, plus the failure mode (co-03) each case represents
    ticket_id: str  # => co-08: the case's stable identifier
    failure_mode: str | None  # => co-08: which taxonomy mode this case represents -- None means a genuine PASS case
    final_verdict: bool  # => co-08: the adjudicated pass/fail


# The full taxonomy from ex-05/ex-06 -- every mode a truly COVERING ground-truth set must represent.
TAXONOMY_MODES = {"wrong-object-acted-on", "malformed-structured-output", "incorrect-aggregate-count", "tone-mismatch-for-audience"}  # => co-08

GROUND_TRUTH_SET: list[GroundTruthCase] = [  # => co-08: a small, intentionally INCOMPLETE ground-truth set, to make the gap visible
    {"ticket_id": "t-601", "failure_mode": "wrong-object-acted-on", "final_verdict": False},  # => co-08: covers mode 1
    {"ticket_id": "t-602", "failure_mode": "malformed-structured-output", "final_verdict": False},  # => co-08: covers mode 2
    {"ticket_id": "t-603", "failure_mode": "incorrect-aggregate-count", "final_verdict": False},  # => co-08: covers mode 3
    {"ticket_id": "t-604", "failure_mode": None, "final_verdict": True},  # => co-08: a genuine pass case, no mode -- does not count toward coverage
]  # => co-08: closes GROUND_TRUTH_SET -- deliberately missing a case for "tone-mismatch-for-audience"


def check_coverage(cases: list[GroundTruthCase], modes: set[str]) -> tuple[set[str], set[str]]:  # => co-08: covered vs. missing modes
    """Return (covered_modes, missing_modes) -- which taxonomy modes DO and do NOT appear in `cases`."""  # => co-08: documents check_coverage's contract -- no runtime output, just sets its __doc__
    represented = {case["failure_mode"] for case in cases if case["failure_mode"] is not None}  # => co-08: modes actually represented by at least one case
    return represented & modes, modes - represented  # => co-08: returns this computed value to the caller


if __name__ == "__main__":  # => co-08: entry point -- runs only when this file executes directly, not on import
    covered, missing = check_coverage(GROUND_TRUTH_SET, TAXONOMY_MODES)  # => co-08: run the coverage check against the full taxonomy
    print(f"Taxonomy modes: {sorted(TAXONOMY_MODES)}")  # => co-08: prints the full taxonomy this set is checked against
    print(f"Covered by ground truth: {sorted(covered)}")  # => co-08: prints what IS represented
    print(f"Missing from ground truth: {sorted(missing)}")  # => co-08: prints the honest gap

    assert missing == {"tone-mismatch-for-audience"}, "this deliberately incomplete set must be missing exactly one named mode"  # => co-08: the gap this example demonstrates
    assert len(covered) == 3, "three of the four taxonomy modes must already be represented"  # => co-08
    print("MATCH: the coverage check names EXACTLY which taxonomy mode has zero ground-truth cases")  # => co-08: reached only if both asserts passed
    # => co-08: a coverage gap like this must be closed BEFORE any judge is validated against this set -- an unrepresented mode is a blind spot no judge score can reveal
