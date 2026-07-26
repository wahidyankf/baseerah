"""Worked Example 6: Wilson and Clopper-Pearson."""  # => co-05: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from statsmodels.stats.proportion import proportion_confint  # => co-05: the pinned library's own binomial-interval function, three methods compared

PASSES = 38  # => co-05: the SAME near-ceiling data ex-05 showed breaking the normal approximation
TOTAL = 40  # => co-05: the SAME small n


if __name__ == "__main__":  # => co-05: entry point -- runs only when this file executes directly, not on import
    p_hat = PASSES / TOTAL  # => co-05: the point estimate this interval is centered on
    print(f"Point estimate: {PASSES}/{TOTAL} = {p_hat:.4f}")  # => co-05: the same 0.95 that broke the normal approximation

    lo_normal, hi_normal = proportion_confint(PASSES, TOTAL, method="normal")  # => co-05: the BROKEN method, for direct comparison
    print(f"normal (broken, clipped): [{lo_normal:.4f}, {hi_normal:.4f}]")  # => co-05: repeats ex-05's clipped result for contrast

    lo_wilson, hi_wilson = proportion_confint(PASSES, TOTAL, method="wilson")  # => co-05: the Wilson score interval -- the standard small-n corrective
    print(f"wilson: [{lo_wilson:.4f}, {hi_wilson:.4f}]")  # => co-05: the Wilson interval's bounds
    assert 0.0 <= lo_wilson and hi_wilson <= 1.0, "the Wilson interval must stay within the valid [0, 1] range"  # => co-05: the property ex-05's normal interval lacked

    lo_beta, hi_beta = proportion_confint(PASSES, TOTAL, method="beta")  # => co-05: the Clopper-Pearson (exact, beta-distribution-based) interval
    print(f"clopper-pearson (beta): [{lo_beta:.4f}, {hi_beta:.4f}]")  # => co-05: the Clopper-Pearson interval's bounds
    assert 0.0 <= lo_beta and hi_beta <= 1.0, "the Clopper-Pearson interval must stay within the valid [0, 1] range"  # => co-05: same property, different method

    assert hi_wilson < 1.0, "Wilson must give a genuinely sub-1.0 upper bound, not a clip"  # => co-05: distinguishes a real fix from a cosmetic clip
    assert hi_beta < 1.0, "Clopper-Pearson must give a genuinely sub-1.0 upper bound, not a clip"  # => co-05: same distinction
    print("MATCH: both Wilson and Clopper-Pearson stay in [0, 1] by CONSTRUCTION, not by clipping a broken result")  # => co-05
    # => co-05: Clopper-Pearson is provably conservative (coverage >= 95%); Wilson is closer to exactly 95% on average -- both are correct where 'normal' is not
