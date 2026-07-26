"""Worked Example 40: Not Yet Significant, but Practically Important."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-06: ceil -- a sample size must be a whole number of cases
import random  # => co-19: draws samples of two systems with a genuinely large true gap

from statsmodels.stats.power import NormalIndPower  # => co-06: the pinned library's own power-based sample-size solver
from statsmodels.stats.proportion import proportion_effectsize, proportions_ztest  # => co-06: Cohen's h -- the effect-size input the power solver needs; co-19: the SAME test ex-36/ex-39 used

TRUE_A_RATE = 0.70  # => co-19: system A's real pass rate
TRUE_B_RATE = 0.78  # => co-19: system B's real pass rate -- a genuinely large, practically important 8-point gap
SMALL_N = 25  # => co-19: a typical small eval run -- the size a team might reach for first


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-19: one independent sample of one system's outcomes
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-19: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-19: one fixed generator per sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-19: one Bernoulli draw per case


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    a_small = sample_outcomes(TRUE_A_RATE, SMALL_N, seed=11)  # => co-19: A's sample at the small, typical size
    b_small = sample_outcomes(TRUE_B_RATE, SMALL_N, seed=12)  # => co-19: B's sample at the small, typical size
    count_small = [sum(b_small), sum(a_small)]  # => co-19: successes for each group, small sample
    _z_small, p_small = proportions_ztest(count_small, [SMALL_N, SMALL_N])  # => co-19: the formal test, applied at the small sample size
    print(f"n={SMALL_N}: A={sum(a_small) / SMALL_N:.4f} B={sum(b_small) / SMALL_N:.4f} p={p_small:.4f}")  # => co-19: looks unremarkable -- NOT significant

    effect_size = proportion_effectsize(TRUE_B_RATE, TRUE_A_RATE)  # => co-06: Cohen's h -- how large this gap is in the units the power solver needs
    power_analysis = NormalIndPower()  # => co-06: the pinned library's own two-sample power calculator
    required_n_raw = power_analysis.solve_power(effect_size=effect_size, alpha=0.05, power=0.80, ratio=1.0, alternative="two-sided")  # => co-06: how many cases PER GROUP are needed to detect this exact gap 80% of the time
    required_n = math.ceil(required_n_raw)  # => co-06: round UP -- a fractional case cannot be collected
    print(f"Effect size (Cohen's h): {effect_size:.4f} | Required n per group for 80% power: {required_n}")  # => co-06: the computed, justified target -- not a guess

    a_big = sample_outcomes(TRUE_A_RATE, required_n, seed=11)  # => co-19: A's sample at the COMPUTED required size, same underlying true rate
    b_big = sample_outcomes(TRUE_B_RATE, required_n, seed=12)  # => co-19: B's sample at the COMPUTED required size, same underlying true rate
    count_big = [sum(b_big), sum(a_big)]  # => co-19: successes for each group, at the required sample size
    _z_big, p_big = proportions_ztest(count_big, [required_n, required_n])  # => co-19: the SAME test, now properly powered
    print(f"n={required_n}: A={sum(a_big) / required_n:.4f} B={sum(b_big) / required_n:.4f} p={p_big:.4f}")  # => co-19: the SAME real gap, now clearly detected

    assert p_small > 0.05, "at n=25, this genuinely large gap must still fail to reach significance"  # => co-19: the "not yet significant" half of the claim
    assert p_big < 0.05, "at the computed required n, the SAME true gap must reach significance"  # => co-19: the "practically important, and now detected" half of the claim
    print(f"MATCH: the identical 8-point true gap is invisible at n={SMALL_N} (p={p_small:.4f}) but clearly detected at the computed n={required_n} (p={p_big:.4f})")  # => co-19
    # => co-06,co-19: 'not significant' at a small sample does not mean 'not real' -- it can mean the sample was never large enough to see a gap that a power calculation would have said needed a specific, computable n
