"""Worked Example 5: The Normal Approximation Breaks."""  # => co-05: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-05: sqrt for the from-definition standard error

from statsmodels.stats.proportion import proportion_confint  # => co-05: the pinned library's own binomial-interval function

PASSES = 38  # => co-05: near-ceiling performance -- exactly the regime the normal approximation struggles with
TOTAL = 40  # => co-05: still a small n -- eval datasets are almost always this size or smaller
Z_95 = 1.959963984540054  # => co-05: the two-sided 95% critical value of the standard normal distribution


def normal_interval_unclipped(passes: int, total: int, *, z: float) -> tuple[float, float]:  # => co-05: the RAW textbook formula, no safety clip
    """Return p_hat +/- z * standard_error, exactly as the formula gives it -- may exceed [0, 1]."""  # => co-05: documents normal_interval_unclipped's contract -- no runtime output, just sets its __doc__
    p_hat = passes / total  # => co-05: the point estimate -- 0.95 here, right at the edge of the possible range
    standard_error = math.sqrt(p_hat * (1 - p_hat) / total)  # => co-05: shrinks toward zero as p_hat approaches 0 or 1 -- part of the problem
    margin = z * standard_error  # => co-05: the interval's half-width
    return p_hat - margin, p_hat + margin  # => co-05: UNCLIPPED -- may land outside the valid [0, 1] probability range


if __name__ == "__main__":  # => co-05: entry point -- runs only when this file executes directly, not on import
    p_hat = PASSES / TOTAL  # => co-05: 0.95 -- near the boundary where the normal approximation is known to misbehave
    print(f"Point estimate: {PASSES}/{TOTAL} = {p_hat:.4f}")  # => co-05: a rate near the ceiling, on a small n
    lo_def, hi_def = normal_interval_unclipped(PASSES, TOTAL, z=Z_95)  # => co-05: the raw, unclipped formula result
    print(f"From definition (unclipped): [{lo_def:.4f}, {hi_def:.4f}]")  # => co-05: prints the interval as the formula actually computes it
    assert hi_def > 1.0, "the unclipped normal-approximation upper bound must extend past the valid [0, 1] range"  # => co-05: the breakage this example demonstrates
    print(f"Upper bound exceeds 1.0 by {hi_def - 1.0:.4f} -- a probability greater than 100% is nonsense")  # => co-05: names the nonsensical result explicitly

    lo_lib, hi_lib = proportion_confint(PASSES, TOTAL, method="normal")  # => co-05: the SAME computation, called from the pinned library
    print(f"From statsmodels (method='normal'): [{lo_lib:.4f}, {hi_lib:.4f}]")  # => co-05: the library's own answer
    assert hi_lib == 1.0, "statsmodels silently clips the normal-approximation upper bound to 1.0"  # => co-05: the library hides the symptom rather than fixing it
    print("MATCH: the raw formula breaks past 1.0; the library's convenience clip HIDES that breakage rather than fixing it")  # => co-05
    # => co-05: clipping to [0, 1] is not a fix -- it silently discards the information that the method itself misbehaved here; ex-06 is the actual fix
