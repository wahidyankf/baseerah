# learning/code/ex-68-statistical-significance-of-the-improvement/statistical_significance.py
"""Worked Example 68: Statistical Significance of the Improvement."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from math import comb  # => co-25: an exact binomial sign-test needs the binomial coefficient, not an approximation


def sign_test_p_value(b: int, c: int) -> float:  # => co-25: the exact two-sided sign-test p-value on the b vs c discordant-pair counts
    """Return the exact two-sided binomial sign-test p-value for `b` wins against `c` losses out of `b + c` discordant pairs."""  # => co-25: documents sign_test_p_value's contract -- no runtime output, just sets its __doc__
    n = b + c  # => co-25: only discordant pairs carry information -- concordant pairs cancel out of a sign test entirely
    if n == 0:  # => co-25: no discordant pairs means no evidence either way
        return 1.0  # => co-25: returns this computed value to the caller
    extreme = min(b, c)  # => co-25: the smaller of the two counts is the "more extreme in the other direction" tail
    one_sided = sum(comb(n, k) for k in range(extreme + 1)) / (2**n)  # => co-25: probability of `extreme` or fewer under a fair 50/50 null
    return min(1.0, 2 * one_sided)  # => co-25: two-sided -- double the one-sided tail, capped at 1.0


SIGNIFICANCE_THRESHOLD = 0.05  # => co-25: the conventional cutoff, imported by name from statistics-for-evaluation's own convention


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    small_b, small_c = 6, 1  # => co-25: ex-35's OWN discordant-pair counts, from its 20-case held-out set
    p_value_small = sign_test_p_value(small_b, small_c)  # => co-25: run the EXACT test on ex-35's own, small scenario
    print(f"ex-35's 20-case set: b={small_b}, c={small_c} -> exact p-value {p_value_small:.4f}")  # => co-25
    assert p_value_small >= SIGNIFICANCE_THRESHOLD, "ex-35's small 20-case set must NOT clear the exact 0.05 threshold, despite its own illustrative b>=5,c<=1 shortcut saying 'supported'"  # => co-25
    print("(ex-35's own 'b>=5, c<=1' shortcut called this 'supported' -- the EXACT test disagrees: 7 discordant pairs is too few to reach p<0.05)")  # => co-25
    large_b, large_c = 24, 4  # => co-25: the SAME 6-to-1 ratio, scaled to 28 discordant pairs instead of ex-35's 7
    p_value_large = sign_test_p_value(large_b, large_c)  # => co-25: run the exact test on the LARGER, better-powered scenario
    print(f"Scaled-up set, 28 discordant pairs (same ratio): b={large_b}, c={large_c} -> exact p-value {p_value_large:.6f}")  # => co-25
    assert p_value_large < SIGNIFICANCE_THRESHOLD, "the same discordant ratio, run on a properly sized held-out set, must clear the significance threshold"  # => co-25
    print(f"MATCH: the same 6-to-1 ratio is NOT significant at 7 discordant pairs (p={p_value_small:.4f}) but IS significant at 28 (p={p_value_large:.6f}) -- sample size, not just ratio, decides it")  # => co-25
    # => co-25: ex-35's illustrative b>=5,c<=1 shortcut was too generous -- the real statistics-for-evaluation machinery shows a small eval set can UNDER-power even a lopsided-looking result
