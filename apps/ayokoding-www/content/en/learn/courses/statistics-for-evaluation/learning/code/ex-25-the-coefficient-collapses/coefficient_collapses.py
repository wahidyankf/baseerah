"""Worked Example 25: The Coefficient Collapses."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-09: rebuilds the SAME skewed two-rater dataset ex-22 through ex-24 used

from sklearn.metrics import cohen_kappa_score  # => co-10: the pinned library's own chance-corrected two-rater coefficient


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the identical fixture-building function from ex-22 through ex-24
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: the SAME seed reproduces the SAME dataset ex-22 through ex-24 built
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass"
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed split into item order
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    rater_a, rater_b = build_skewed_dataset(60, seed=7)  # => co-09: the SAME fixture threaded through this whole run of examples
    n = len(rater_a)  # => co-09: item count
    raw_agreement = sum(1 for x, y in zip(rater_a, rater_b) if x == y) / n  # => co-09: ex-21/ex-22's own raw-agreement number
    p_a_pass = rater_a.count("pass") / n  # => co-10: rater A's marginal "pass" probability
    p_b_pass = rater_b.count("pass") / n  # => co-10: rater B's marginal "pass" probability
    chance_agreement = p_a_pass * p_b_pass + (1 - p_a_pass) * (1 - p_b_pass)  # => co-10: ex-23's chance-expected agreement
    kappa = cohen_kappa_score(rater_a, rater_b)  # => co-10: ex-24's chance-corrected coefficient

    print("Same 60-item dataset, four numbers:")  # => co-01: the "collapse" is only visible when every number is printed side by side
    print(f"  Raw agreement:              {raw_agreement:.4f}")  # => co-09: the number that LOOKS reassuring
    print(f"  Chance-expected agreement:  {chance_agreement:.4f}")  # => co-10: what pure chance alone predicts, given the skew
    print(f"  Cohen's kappa (corrected):  {kappa:.4f}")  # => co-10: the number that tells the truth

    assert raw_agreement >= 0.80, "raw agreement must look superficially solid (>= 0.80) for this example's own point to land"  # => co-09
    assert abs(kappa) < 0.10, "the chance-corrected coefficient must collapse to near zero on this same data"  # => co-10: the collapse itself
    print(f"MATCH: {raw_agreement:.0%} raw agreement collapses to a kappa of {kappa:.4f} once corrected for the label skew's own chance agreement")  # => co-10
    # => co-01,co-09,co-10: a number without its chance-corrected companion is not just incomplete, it can be actively misleading -- 85% sounds like a strong result, and it is not one
