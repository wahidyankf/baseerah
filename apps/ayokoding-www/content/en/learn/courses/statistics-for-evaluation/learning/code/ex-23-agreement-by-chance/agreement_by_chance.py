"""Worked Example 23: Agreement by Chance."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-09: rebuilds the SAME skewed two-rater dataset ex-22 introduced


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the identical fixture-building function from ex-22, reproduced here so this file stays independently runnable
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: the SAME seed reproduces the SAME dataset ex-22 built
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass"
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed split into item order
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    n = 60  # => co-10: the SAME sixty items ex-22 used
    rater_a, rater_b = build_skewed_dataset(n, seed=7)  # => co-09: reproduces ex-22's exact fixture
    observed_agreement = sum(1 for x, y in zip(rater_a, rater_b) if x == y) / n  # => co-10: the raw agreement ex-22 already computed -- 0.85
    print(f"Observed raw agreement: {observed_agreement:.4f}")  # => co-10: restated for direct comparison against the chance figure below

    p_a_pass = rater_a.count("pass") / n  # => co-10: rater A's marginal probability of saying "pass"
    p_b_pass = rater_b.count("pass") / n  # => co-10: rater B's marginal probability of saying "pass"
    chance_agreement = p_a_pass * p_b_pass + (1 - p_a_pass) * (1 - p_b_pass)  # => co-10: P(both say pass) + P(both say fail), if the two raters' labels were INDEPENDENT of each other
    print(f"Chance-expected agreement (from the two raters' own marginals): {chance_agreement:.4f}")  # => co-10: what TWO INDEPENDENT RANDOM LABELERS with these exact marginals would achieve, on average

    gap = abs(observed_agreement - chance_agreement)  # => co-10: how far the OBSERVED number is from what CHANCE ALONE predicts
    print(f"Gap between observed and chance-expected agreement: {gap:.4f}")  # => co-10: a small gap means the raters are barely beating pure chance
    assert gap < 0.03, "the observed raw agreement must be close to the chance-expected agreement on this skewed fixture"  # => co-10: the core claim this example demonstrates
    print("MATCH: the raters' 85% raw agreement is nearly identical to what independent random guessing would produce on this label distribution")  # => co-10
    # => co-10: this chance-expected figure IS the 'pe' term every chance-corrected coefficient subtracts -- ex-24 builds the full correction from here
