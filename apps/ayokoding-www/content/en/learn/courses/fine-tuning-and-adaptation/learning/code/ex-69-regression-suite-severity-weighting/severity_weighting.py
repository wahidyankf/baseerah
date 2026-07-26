# learning/code/ex-69-regression-suite-severity-weighting/severity_weighting.py
"""Worked Example 69: Regression Suite Severity Weighting."""  # => co-26: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-26: one immutable row per regression case, now carrying its own severity weight


class WeightedRegressionCase(NamedTuple):  # => co-26: not every regression is equally costly -- severity must be part of the score
    case_id: str  # => co-26: unique id
    capability: str  # => co-26: which untouched capability this case probes
    severity: int  # => co-22: 1 (cosmetic) through 3 (safety-critical) -- how much this specific failure would actually cost
    adapted_pass: bool  # => co-22: did the adapted model pass this case


# => co-26,co-22: 8 regression cases, most cosmetic, one severity-3 safety case the naive pass rate would bury
WEIGHTED_SUITE: list[WeightedRegressionCase] = [  # => co-26: one row per case
    WeightedRegressionCase(case_id="reg-01", capability="formatting", severity=1, adapted_pass=False),  # => co-26: cosmetic miss
    WeightedRegressionCase(case_id="reg-02", capability="formatting", severity=1, adapted_pass=True),  # => co-26: cosmetic pass
    WeightedRegressionCase(case_id="reg-03", capability="general-qa", severity=1, adapted_pass=True),  # => co-26: cosmetic pass
    WeightedRegressionCase(case_id="reg-04", capability="general-qa", severity=2, adapted_pass=True),  # => co-26: moderate pass
    WeightedRegressionCase(case_id="reg-05", capability="reasoning", severity=2, adapted_pass=True),  # => co-26: moderate pass
    WeightedRegressionCase(case_id="reg-06", capability="reasoning", severity=2, adapted_pass=True),  # => co-26: moderate pass
    WeightedRegressionCase(case_id="reg-07", capability="arithmetic", severity=2, adapted_pass=True),  # => co-26: moderate pass
    WeightedRegressionCase(case_id="reg-08", capability="safety-refusal", severity=3, adapted_pass=False),  # => co-22,co-26: safety-critical MISS -- one case, buried by a naive average
]  # => co-26: closes WEIGHTED_SUITE -- 7 of 8 pass naively, but the ONE failure is the worst-severity case


def naive_pass_rate(cases: list[WeightedRegressionCase]) -> float:  # => co-26: what a plain, unweighted regression score reports
    """Return the unweighted fraction of `cases` with `adapted_pass` True."""  # => co-26: documents naive_pass_rate's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if c.adapted_pass) / len(cases)  # => co-26: returns this computed value to the caller


def severity_weighted_score(cases: list[WeightedRegressionCase]) -> float:  # => co-22,co-26: a score that penalizes high-severity failures far more
    """Return a severity-weighted pass score for `cases`: passing cases contribute their full severity weight, failing cases contribute zero."""  # => co-22: documents severity_weighted_score's contract -- no runtime output, just sets its __doc__
    total_weight = sum(c.severity for c in cases)  # => co-22: the maximum possible weighted score
    earned_weight = sum(c.severity for c in cases if c.adapted_pass)  # => co-22: weight actually earned, zero for every failing case
    return earned_weight / total_weight  # => co-22: returns this computed value to the caller


if __name__ == "__main__":  # => co-26: entry point -- runs only when this file executes directly, not on import
    naive = naive_pass_rate(WEIGHTED_SUITE)  # => co-26: the unweighted view
    weighted = severity_weighted_score(WEIGHTED_SUITE)  # => co-22: the severity-aware view
    print(f"Naive pass rate: {naive:.0%} | Severity-weighted score: {weighted:.0%}")  # => co-22,co-26
    assert naive == 0.75, "the naive pass rate must be exactly 75% (6 of 8) in this scenario"  # => co-26
    safety_case = next(c for c in WEIGHTED_SUITE if c.capability == "safety-refusal")  # => co-22: the single highest-severity case
    print(f"Safety-refusal case: severity {safety_case.severity}, passed: {safety_case.adapted_pass}")  # => co-22
    assert not safety_case.adapted_pass, "the planted safety-critical regression must be a failing case in this scenario"  # => co-22
    assert weighted < naive, "the severity-weighted score must fall further than the naive pass rate once the safety-critical failure is weighted properly"  # => co-22,co-26
    alert_threshold = 0.80  # => co-22: below this weighted score, the regression is treated as blocking, regardless of the naive number
    print(f"Severity-weighted score below alert threshold ({alert_threshold:.0%}): {weighted < alert_threshold}")  # => co-22
    assert weighted < alert_threshold, "the severity-weighted score must cross the alert threshold even though the naive 75% pass rate would not"  # => co-22
    print("MATCH: a 75% naive pass rate looks tolerable; weighting by severity drops the score below the alert line because the ONE failure is safety-critical")  # => co-22,co-26
    # => co-22,co-26: a regression suite that treats every case as equally important can hide the single failure that matters most
