"""Worked Example 22: Skewed Labels Inflate Agreement."""  # => co-09: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-09: builds the skewed two-rater dataset this theme reuses through ex-25


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the shared fixture this theme's next several examples all regenerate identically
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: one fixed generator -- this exact dataset is reproduced identically wherever this function is called with the same seed
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass", a heavily skewed distribution
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed 55/5 split into item order -- the COUNT is fixed, the item assignment is randomized
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time, LARGELY WITHOUT regard to what rater A said on that item
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    n = 60  # => co-09: sixty labeled items -- a realistic small eval-agreement study
    rater_a, rater_b = build_skewed_dataset(n, seed=7)  # => co-09: the shared skewed fixture, seed=7, reproduced identically in ex-23 through ex-25, ex-29, ex-30
    a_pass_rate = rater_a.count("pass") / n  # => co-12: rater A's own marginal "pass" prevalence
    b_pass_rate = rater_b.count("pass") / n  # => co-12: rater B's own marginal "pass" prevalence
    print(f"Rater A 'pass' prevalence: {a_pass_rate:.4f} | Rater B 'pass' prevalence: {b_pass_rate:.4f}")  # => co-12: both heavily skewed toward "pass"

    raw_agreement = sum(1 for x, y in zip(rater_a, rater_b) if x == y) / n  # => co-09: the SAME raw-agreement arithmetic ex-21 introduced
    print(f"Raw agreement: {raw_agreement:.4f}")  # => co-09: looks like a solid, respectable agreement number

    majority_baseline = max(a_pass_rate, 1 - a_pass_rate)  # => co-12: what a rater who ALWAYS said the majority label ("pass") would achieve against rater A, by pure chance of the skew
    print(f"'Always say the majority label' baseline agreement with rater A: {majority_baseline:.4f}")  # => co-12: the trivial baseline this raw number should beat
    assert majority_baseline >= raw_agreement - 0.05, "with this skew, a trivial always-majority-label baseline must be roughly competitive with the observed raw agreement"  # => co-12
    print("MATCH: a rater who blindly guessed the majority label every time would score about as well as the observed raw agreement")  # => co-12
    # => co-09,co-12: 85% raw agreement sounds solid until you notice a rater who never looked at the item at all could nearly match it -- the skew, not real agreement, is doing most of the work
