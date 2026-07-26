"""Worked Example 75: Tell a Genuinely Flaky Case Apart From a Real, Reproducible Regression."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-24: RerunResult is a typed record -- one case's own outcomes across repeated re-runs


class RerunResult(NamedTuple):  # => co-24: one case, re-run several times, to distinguish flake from real regression
    case_id: str  # => co-24: which case this is
    outcomes: tuple[bool, ...]  # => co-24: pass/fail across repeated re-runs of THIS ONE case, same code, same input


FLAKY_CASE = RerunResult("case-33", outcomes=(True, False, True, True, False))  # => co-24: sometimes passes, sometimes fails, SAME code -- genuine non-determinism, not a real bug
REGRESSED_CASE = RerunResult("case-41", outcomes=(False, False, False, False, False))  # => co-24: fails EVERY single re-run -- reproducible, a real regression


def classify_rerun_result(result: RerunResult) -> str:  # => co-24: the actual classification rule -- consistency across re-runs is the signal
    """Return "flaky" if `result.outcomes` mixes True and False, "regressed" if all False, or "stable" if all True."""  # => co-24: documents classify_rerun_result's contract -- no runtime output, just sets its __doc__
    if all(result.outcomes):  # => co-24: every re-run passed
        return "stable"  # => co-24: returns this computed value to the caller
    if not any(result.outcomes):  # => co-24: every re-run failed
        return "regressed"  # => co-24: returns this computed value to the caller -- reproducible failure, a REAL regression
    return "flaky"  # => co-24: returns this computed value to the caller -- inconsistent outcomes, non-determinism, not a code bug


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    flaky_verdict = classify_rerun_result(FLAKY_CASE)  # => co-24: classify the inconsistent case
    regressed_verdict = classify_rerun_result(REGRESSED_CASE)  # => co-24: classify the consistently-failing case
    print(f"{FLAKY_CASE.case_id} outcomes {FLAKY_CASE.outcomes} -> classified as: {flaky_verdict}")  # => co-24: prints the flaky case's classification
    print(f"{REGRESSED_CASE.case_id} outcomes {REGRESSED_CASE.outcomes} -> classified as: {regressed_verdict}")  # => co-24: prints the regressed case's classification

    assert flaky_verdict == "flaky", "a case that mixes pass and fail across identical re-runs must be classified as flaky, not blamed on the code change"  # => co-24: the rule this example proves
    assert regressed_verdict == "regressed", "a case that fails EVERY re-run, with no code change between them, must be classified as a real, reproducible regression"  # => co-24: the rule this example proves
    print(  # => co-24: opens the final MATCH print, reached only if both asserts above passed
        f"MATCH: re-running each case multiple times distinguishes '{flaky_verdict}' (inconsistent, non-deterministic) from '{regressed_verdict}' (consistently failing) -- a CI gate must NOT block a merge for a flaky case alone"
    )  # => co-24
    # => co-24: ex-76 next uses a cheap DRY RUN, before the full suite, to catch an obviously broken change earlier and cheaper
