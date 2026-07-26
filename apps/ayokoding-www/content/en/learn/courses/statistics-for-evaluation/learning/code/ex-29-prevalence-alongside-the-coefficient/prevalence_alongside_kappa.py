"""Worked Example 29: Prevalence Alongside the Coefficient."""  # => co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-12: builds two datasets that share a raw agreement number but differ in prevalence

from sklearn.metrics import cohen_kappa_score  # => co-12: the pinned library's chance-corrected coefficient


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-12: the SAME fixture-builder used in ex-22 through ex-25 -- extreme prevalence, 91.7% "pass"
    """Return two raters' labels over n items, 55/60 skewed toward 'pass'."""  # => co-12: documents the contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-12: one generator seeds both the shuffle and rater_b's noise
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: fixed skew -- 55 pass, 5 fail, out of 60
    rng.shuffle(rater_a)  # => co-12: randomizes the order so rater_b's noise does not correlate with position
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-12: rater_b independently leans "pass" 90% of the time
    return rater_a, rater_b  # => co-12: two label lists, same length, ready for agreement scoring


def build_balanced_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-12: a SECOND fixture -- same raw agreement, but 50/50 prevalence
    """Return two raters' labels over n items, balanced 50/50 between 'pass' and 'fail'."""  # => co-12: documents the contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-12: shuffles rater_a's balanced labels
    rater_a = ["pass"] * (n // 2) + ["fail"] * (n // 2)  # => co-12: exactly half pass, half fail
    rng.shuffle(rater_a)  # => co-12: randomizes order before rater_b reads it
    noise_rng = random.Random(seed + 1)  # => co-12: a second, independent generator for rater_b's per-item noise
    rater_b: list[str] = []  # => co-12: built item by item below
    for label in rater_a:  # => co-12: rater_b agrees with rater_a 85% of the time, disagrees the other 15%
        if noise_rng.random() < 0.85:  # => co-12: the 85% "agree" branch
            rater_b.append(label)  # => co-12: copies rater_a's label exactly
        else:  # => co-12: the 15% "disagree" branch
            rater_b.append("fail" if label == "pass" else "pass")  # => co-12: flips to the opposite label
    return rater_a, rater_b  # => co-12: two label lists, same length, ready for agreement scoring


if __name__ == "__main__":  # => co-12: entry point -- runs only when this file executes directly, not on import
    skewed_a, skewed_b = build_skewed_dataset(60, seed=7)  # => co-12: the extreme-prevalence dataset -- 91.7% "pass"
    balanced_a, balanced_b = build_balanced_dataset(60, seed=3)  # => co-12: the balanced-prevalence dataset -- 50% "pass"

    skewed_raw = sum(1 for x, y in zip(skewed_a, skewed_b) if x == y) / len(skewed_a)  # => co-12: raw percent agreement, skewed dataset
    balanced_raw = sum(1 for x, y in zip(balanced_a, balanced_b) if x == y) / len(balanced_a)  # => co-12: raw percent agreement, balanced dataset
    print(f"Raw agreement: skewed={skewed_raw:.4f} | balanced={balanced_raw:.4f}")  # => co-12: IDENTICAL -- both datasets show 85% raw agreement
    assert skewed_raw == balanced_raw, "both datasets must share the same raw agreement, for contrast"  # => co-12: the setup this example depends on

    skewed_kappa = cohen_kappa_score(skewed_a, skewed_b)  # => co-12: chance-corrected agreement, skewed dataset
    balanced_kappa = cohen_kappa_score(balanced_a, balanced_b)  # => co-12: chance-corrected agreement, balanced dataset
    skewed_prevalence = skewed_a.count("pass") / len(skewed_a)  # => co-12: rater_a's own "pass" rate -- the number that explains the gap below
    balanced_prevalence = balanced_a.count("pass") / len(balanced_a)  # => co-12: rater_a's own "pass" rate, balanced dataset
    print(f"Cohen's kappa: skewed={skewed_kappa:.4f} (prevalence={skewed_prevalence:.4f}) | balanced={balanced_kappa:.4f} (prevalence={balanced_prevalence:.4f})")  # => co-12: WILDLY different, despite identical raw agreement

    assert skewed_kappa < 0 < balanced_kappa, "the skewed dataset's kappa must be negative while the balanced dataset's kappa stays clearly positive"  # => co-12: the claim this example demonstrates
    print(f"MATCH: same raw agreement (0.85) produces kappa={skewed_kappa:.4f} at 91.7% prevalence but kappa={balanced_kappa:.4f} at 50% prevalence")  # => co-12
    # => co-12: a kappa number alone does not tell a reader whether the dataset was skewed or balanced -- report the prevalence next to it, or the coefficient is unreadable on its own
