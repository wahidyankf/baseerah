# learning/code/ex-37-catastrophic-forgetting-measured/forgetting_measured.py
"""Worked Example 37: Catastrophic Forgetting, Measured."""  # => co-22: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-26: a compact regression suite, self-contained for this file, echoing ex-36's shape


class RegressionCase(NamedTuple):  # => co-26: a capability check the fine-tune was NOT meant to change
    case_id: str  # => co-26: unique id
    base_pass: bool  # => co-22: did the UNADAPTED base model pass this case
    adapted_pass: bool  # => co-22: did the ADAPTED model pass the SAME case, after triage fine-tuning


# => co-22: the same ten held-out regression cases, run through both models -- none of them is a triage case
REGRESSION_RESULTS: list[RegressionCase] = [  # => co-22: one row per regression case
    RegressionCase(case_id="reg-01", base_pass=True, adapted_pass=True),  # => co-22: arithmetic -- unaffected
    RegressionCase(case_id="reg-02", base_pass=True, adapted_pass=True),  # => co-22: arithmetic -- unaffected
    RegressionCase(case_id="reg-03", base_pass=True, adapted_pass=False),  # => co-22: arithmetic -- REGRESSED after triage fine-tuning
    RegressionCase(case_id="reg-04", base_pass=True, adapted_pass=True),  # => co-22: general-qa -- unaffected
    RegressionCase(case_id="reg-05", base_pass=True, adapted_pass=False),  # => co-22: general-qa -- REGRESSED
    RegressionCase(case_id="reg-06", base_pass=True, adapted_pass=True),  # => co-22: general-qa -- unaffected
    RegressionCase(case_id="reg-07", base_pass=True, adapted_pass=True),  # => co-22: formatting -- unaffected
    RegressionCase(case_id="reg-08", base_pass=True, adapted_pass=False),  # => co-22: formatting -- REGRESSED
    RegressionCase(case_id="reg-09", base_pass=True, adapted_pass=True),  # => co-22: reasoning -- unaffected
    RegressionCase(case_id="reg-10", base_pass=True, adapted_pass=False),  # => co-22: reasoning -- REGRESSED
]  # => co-22: closes REGRESSION_RESULTS -- base passes ALL 10, adapted passes only 6

TARGET_TASK_PASS_RATE_ADAPTED = 0.94  # => co-25: the triage eval from ex-35's lineage -- looks like a clean win in isolation
REGRESSION_ALERT_THRESHOLD = 0.90  # => co-22: below this regression pass rate, the forgetting is a real operational problem


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    base_regression_pass_rate = sum(1 for c in REGRESSION_RESULTS if c.base_pass) / len(REGRESSION_RESULTS)  # => co-22: base model's regression-suite score
    adapted_regression_pass_rate = sum(1 for c in REGRESSION_RESULTS if c.adapted_pass) / len(REGRESSION_RESULTS)  # => co-22: adapted model's regression-suite score
    print(f"Target-task pass rate (adapted): {TARGET_TASK_PASS_RATE_ADAPTED:.0%}")  # => co-25: this is the number ex-35 alone would report
    print(f"Regression suite: base {base_regression_pass_rate:.0%} | adapted {adapted_regression_pass_rate:.0%}")  # => co-22: what ex-35 alone WOULD NOT show
    assert base_regression_pass_rate == 1.00, "the unadapted base must pass every regression case -- these capabilities were never in question for it"  # => co-22
    assert adapted_regression_pass_rate == 0.60, "the adapted model must show a measurable regression-suite drop in this scenario"  # => co-22
    forgetting_detected = adapted_regression_pass_rate < REGRESSION_ALERT_THRESHOLD  # => co-22: does the drop cross the alert line
    print(f"Catastrophic forgetting detected (threshold {REGRESSION_ALERT_THRESHOLD:.0%}): {forgetting_detected}")  # => co-22
    assert forgetting_detected, "a 40-point regression-suite drop must trigger the forgetting alert"  # => co-22,co-26
    print("MATCH: the target-task eval alone reports a 94% win -- the regression suite reveals a 40-point capability loss that eval never saw")  # => co-22,co-25,co-26
    # => co-22,co-25,co-26: co-25's mandate is BOTH the target-task comparison AND this regression suite -- either alone is an incomplete picture
