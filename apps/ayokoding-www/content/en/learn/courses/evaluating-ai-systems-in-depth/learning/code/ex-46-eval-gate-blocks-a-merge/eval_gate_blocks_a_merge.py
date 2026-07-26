"""Worked Example 46: Wire the Regression Bar Into a CI Gate That Actually Blocks a Merge."""  # => co-23: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-23: GateResult is a typed record -- a CI gate's own machine-checkable verdict


class GateResult(NamedTuple):  # => co-23: what a CI eval gate actually returns -- not just a bool, a reasoned verdict
    merge_allowed: bool  # => co-23: the gate's binary decision
    observed_pass_rate: float  # => co-23: what the suite scored on THIS candidate change
    bar: float  # => co-23: the regression bar this run was checked against
    reason: str  # => co-23: a human-readable explanation, for the CI log


REGRESSION_BAR = 0.796  # => co-23: ex-45's derived bar (86.0% - 2 * 3.2%), reused here as CI's own fixed threshold


def run_eval_gate(candidate_pass_rate: float, *, bar: float = REGRESSION_BAR) -> GateResult:  # => co-23: the CI gate itself -- runs the suite, compares to the bar, decides
    """Return a `GateResult` deciding whether `candidate_pass_rate` clears `bar`."""  # => co-23: documents run_eval_gate's contract -- no runtime output, just sets its __doc__
    if candidate_pass_rate < bar:  # => co-23: below the bar -- a real regression, not noise
        return GateResult(merge_allowed=False, observed_pass_rate=candidate_pass_rate, bar=bar, reason=f"pass rate {candidate_pass_rate:.1%} fell below the regression bar {bar:.1%}")  # => co-23: returns this computed value to the caller
    return GateResult(merge_allowed=True, observed_pass_rate=candidate_pass_rate, bar=bar, reason=f"pass rate {candidate_pass_rate:.1%} cleared the regression bar {bar:.1%}")  # => co-23: returns this computed value to the caller


if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    good_candidate = run_eval_gate(0.88)  # => co-23: a change that IMPROVES the pass rate
    regressed_candidate = run_eval_gate(0.70)  # => co-23: a change that genuinely REGRESSES the pass rate
    print(f"Good candidate: merge_allowed={good_candidate.merge_allowed} ({good_candidate.reason})")  # => co-23: prints the gate's decision
    print(f"Regressed candidate: merge_allowed={regressed_candidate.merge_allowed} ({regressed_candidate.reason})")  # => co-23: prints the gate's decision

    assert good_candidate.merge_allowed is True, "an improving change must be allowed to merge"  # => co-23
    assert regressed_candidate.merge_allowed is False, "a genuinely regressing change must be BLOCKED from merging"  # => co-23: the rule this example proves
    assert "regression bar" in regressed_candidate.reason, "a blocked merge must report a machine-readable, human-legible reason in the CI log"  # => co-23
    print("MATCH: the CI gate allows the improving candidate and BLOCKS the regressed candidate, citing the exact bar in its CI-log reason")  # => co-23
    # => co-23: ex-47 next splits this single gate into TIERED suites -- a fast tier per commit, a slower judged tier on merge
