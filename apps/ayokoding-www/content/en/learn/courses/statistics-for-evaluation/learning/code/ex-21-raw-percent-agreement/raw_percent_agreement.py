"""Worked Example 21: Raw Percent Agreement."""  # => co-09: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

RATER_A = ["pass", "fail", "pass", "pass", "fail", "pass", "fail", "pass", "pass", "fail", "pass", "pass", "fail", "pass", "pass", "fail", "pass", "fail", "pass", "pass"]  # => co-09: labeler A's 20 verdicts
RATER_B = [
    "pass",
    "fail",
    "pass",
    "fail",
    "fail",
    "pass",
    "pass",
    "pass",
    "pass",
    "fail",
    "fail",
    "pass",
    "fail",
    "pass",
    "pass",
    "fail",
    "pass",
    "fail",
    "fail",
    "pass",
]  # => co-09: labeler B's 20 verdicts on the SAME 20 items, in the same order


def raw_agreement(rater_x: list[str], rater_y: list[str]) -> float:  # => co-09: the plain arithmetic definition -- no library needed for this one
    """Return the fraction of items where rater_x and rater_y assigned the identical label."""  # => co-09: documents raw_agreement's contract -- no runtime output, just sets its __doc__
    assert len(rater_x) == len(rater_y), "both raters must have labeled the exact same number of items"  # => co-09: a basic shape check
    agreements = sum(1 for x, y in zip(rater_x, rater_y) if x == y)  # => co-09: count items where the two labels match, exactly
    return agreements / len(rater_x)  # => co-09: the fraction of items the two raters agreed on


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    n = len(RATER_A)  # => co-09: how many items both raters labeled
    agreement = raw_agreement(RATER_A, RATER_B)  # => co-09: the raw percent agreement itself
    print(f"Items labeled: {n}")  # => co-09: states the sample size up front
    print(f"Raw agreement: {agreement:.4f} ({round(agreement * n)}/{n} items match)")  # => co-09: the number and the arithmetic behind it
    matches = [i for i, (x, y) in enumerate(zip(RATER_A, RATER_B)) if x == y]  # => co-09: WHICH items matched, for the arithmetic check below
    assert len(matches) == round(agreement * n), "the count of matching items must equal agreement times n"  # => co-09: verifies the arithmetic itself
    print(f"MATCH: {len(matches)} of {n} items have identical labels -- raw agreement is exactly that ratio")  # => co-09
    # => co-09: raw percent agreement is nothing more than 'how often did the two labels match' -- ex-22 shows why that number alone can be deeply misleading
