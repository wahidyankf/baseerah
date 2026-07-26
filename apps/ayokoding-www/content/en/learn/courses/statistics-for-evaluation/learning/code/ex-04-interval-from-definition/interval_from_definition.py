"""Worked Example 4: Interval From Definition."""  # => co-04: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-04: sqrt for the standard error, no third-party dependency needed for the from-definition half

from statsmodels.stats.proportion import proportion_confint  # => co-04: the pinned library's own binomial-interval function

PASSES = 34  # => co-04: cases the system passed
TOTAL = 40  # => co-04: cases attempted -- a typical small eval-set size
Z_95 = 1.959963984540054  # => co-04: the two-sided 95% critical value of the standard normal distribution -- a fixed constant


def normal_interval_from_definition(passes: int, total: int, *, z: float) -> tuple[float, float]:  # => co-04: the textbook formula, in code
    """Return the normal-approximation confidence interval p_hat +/- z * standard_error."""  # => co-04: documents normal_interval_from_definition's contract -- no runtime output, just sets its __doc__
    p_hat = passes / total  # => co-04: the point estimate -- the observed pass rate itself
    standard_error = math.sqrt(p_hat * (1 - p_hat) / total)  # => co-04: how much p_hat is expected to vary run to run, from ex-03's own spread
    margin = z * standard_error  # => co-04: the interval's half-width
    return p_hat - margin, p_hat + margin  # => co-04: returns this computed value to the caller


if __name__ == "__main__":  # => co-04: entry point -- runs only when this file executes directly, not on import
    p_hat = PASSES / TOTAL  # => co-04: the point estimate this interval is centered on
    print(f"Point estimate: {PASSES}/{TOTAL} = {p_hat:.4f}")  # => co-04: the bare number ex-01's overview warns is not yet enough
    lo_def, hi_def = normal_interval_from_definition(PASSES, TOTAL, z=Z_95)  # => co-04: computed from the textbook formula directly
    print(f"From definition: [{lo_def:.4f}, {hi_def:.4f}]")  # => co-04: the interval this file derives itself
    lo_lib, hi_lib = proportion_confint(PASSES, TOTAL, method="normal")  # => co-04: the SAME computation, called from the pinned library
    print(f"From statsmodels (method='normal'): [{lo_lib:.4f}, {hi_lib:.4f}]")  # => co-04: the library's answer to the identical question
    assert math.isclose(lo_def, lo_lib, abs_tol=1e-9), "the from-definition lower bound must match the library's"  # => co-04
    assert math.isclose(hi_def, hi_lib, abs_tol=1e-9), "the from-definition upper bound must match the library's"  # => co-04
    print("MATCH: the hand-derived interval and the library's interval agree to within floating-point precision")  # => co-04
    # => co-04: this interval says the true pass rate is plausibly anywhere from ~70% to ~96% -- NOT that it is exactly 85%
