"""Worked Example 24: Chance-Corrected Agreement, From Definition."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-10: isclose -- verifies the from-definition and library kappas agree
import random  # => co-09: rebuilds the SAME skewed two-rater dataset ex-22 introduced

from sklearn.metrics import cohen_kappa_score  # => co-10: the pinned library's own chance-corrected two-rater coefficient


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the identical fixture-building function from ex-22/ex-23
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: the SAME seed reproduces the SAME dataset ex-22/ex-23 built
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass"
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed split into item order
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


def cohen_kappa_from_definition(rater_x: list[str], rater_y: list[str]) -> float:  # => co-10: the textbook formula, in code
    """Return Cohen's kappa: (observed_agreement - chance_agreement) / (1 - chance_agreement)."""  # => co-10: documents cohen_kappa_from_definition's contract -- no runtime output, just sets its __doc__
    n = len(rater_x)  # => co-10: item count
    observed = sum(1 for x, y in zip(rater_x, rater_y) if x == y) / n  # => co-10: the raw agreement, from ex-21's own arithmetic
    p_x_pass = rater_x.count("pass") / n  # => co-10: rater X's own marginal probability of "pass"
    p_y_pass = rater_y.count("pass") / n  # => co-10: rater Y's own marginal probability of "pass"
    chance = p_x_pass * p_y_pass + (1 - p_x_pass) * (1 - p_y_pass)  # => co-10: the pe term ex-23 computed
    return (observed - chance) / (1 - chance)  # => co-10: the chance CORRECTION -- how much better than chance, as a fraction of the room available above chance


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    rater_a, rater_b = build_skewed_dataset(60, seed=7)  # => co-09: reproduces ex-22/ex-23's exact fixture
    kappa_def = cohen_kappa_from_definition(rater_a, rater_b)  # => co-10: computed from the formula directly
    print(f"Cohen's kappa, from definition: {kappa_def:.4f}")  # => co-10: the chance-corrected coefficient this file derives itself

    kappa_lib = cohen_kappa_score(rater_a, rater_b)  # => co-10: the SAME computation, called from the pinned library
    print(f"Cohen's kappa, from scikit-learn: {kappa_lib:.4f}")  # => co-10: the library's answer to the identical question
    assert math.isclose(kappa_def, kappa_lib, abs_tol=1e-9), "the from-definition and library kappa must agree"  # => co-10
    print("MATCH: the hand-derived kappa and the library's kappa agree to within floating-point precision")  # => co-10
    # => co-10: kappa is negative here -- these raters do WORSE than chance-level agreement, despite 85% raw agreement; ex-25 puts both numbers side by side
