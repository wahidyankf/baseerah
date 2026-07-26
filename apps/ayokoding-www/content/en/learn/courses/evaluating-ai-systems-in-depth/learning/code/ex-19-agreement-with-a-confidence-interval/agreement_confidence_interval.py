"""Worked Example 19: Report Agreement With a Confidence Interval, Not a Bare Point Estimate."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-10: the Wilson score interval needs only sqrt -- no external stats library required
from typing import NamedTuple  # => co-10: AgreementInterval is a typed record, not a bare tuple


class AgreementInterval(NamedTuple):  # => co-10: a proportion, reported WITH its uncertainty, never alone
    point_estimate: float  # => co-10: the raw agreement rate -- ex-18's single number
    lower_bound: float  # => co-10: the interval's lower edge
    upper_bound: float  # => co-10: the interval's upper edge


def wilson_interval(successes: int, total: int, *, z: float = 1.96) -> AgreementInterval:  # => co-10: a small-sample-safe interval for a proportion
    """Compute a Wilson score confidence interval for `successes`/`total`, at the given z (default 95%)."""  # => co-10: documents wilson_interval's contract -- no runtime output, just sets its __doc__
    p_hat = successes / total  # => co-10: the raw point estimate, same number ex-18 reported
    denominator = 1 + z**2 / total  # => co-10: Wilson's correction term -- widens the interval for small samples
    center = p_hat + z**2 / (2 * total)  # => co-10: the interval's recentered midpoint
    spread = z * math.sqrt(p_hat * (1 - p_hat) / total + z**2 / (4 * total**2))  # => co-10: the interval's half-width
    lower = max(0.0, (center - spread) / denominator)  # => co-10: clamps to a valid probability floor
    upper = min(1.0, (center + spread) / denominator)  # => co-10: clamps to a valid probability ceiling
    return AgreementInterval(p_hat, lower, upper)  # => co-10: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    small_sample = wilson_interval(successes=9, total=10)  # => co-10: ex-18's exact 9/10 result -- a SMALL sample
    larger_sample = wilson_interval(successes=90, total=100)  # => co-10: the SAME 90% point estimate, but on a larger sample
    print(f"9/10 (90%): interval [{small_sample.lower_bound:.2f}, {small_sample.upper_bound:.2f}], width={small_sample.upper_bound - small_sample.lower_bound:.2f}")  # => co-10
    print(f"90/100 (90%): interval [{larger_sample.lower_bound:.2f}, {larger_sample.upper_bound:.2f}], width={larger_sample.upper_bound - larger_sample.lower_bound:.2f}")  # => co-10

    assert small_sample.point_estimate == larger_sample.point_estimate == 0.9, "both samples must share the identical 90% point estimate"  # => co-10
    small_width = small_sample.upper_bound - small_sample.lower_bound  # => co-10: the small sample's interval width
    larger_width = larger_sample.upper_bound - larger_sample.lower_bound  # => co-10: the larger sample's interval width
    assert small_width > larger_width, "the smaller sample's interval must be WIDER, reflecting its greater uncertainty"  # => co-10: the rule this example proves
    print(f"MATCH: identical 90% point estimates, but the 10-case interval ({small_width:.2f}) is far wider than the 100-case one ({larger_width:.2f})")  # => co-10
    # => co-10: "90% agreement" alone hides whether that 90% rests on 10 cases or 100 -- the interval makes that difference visible
