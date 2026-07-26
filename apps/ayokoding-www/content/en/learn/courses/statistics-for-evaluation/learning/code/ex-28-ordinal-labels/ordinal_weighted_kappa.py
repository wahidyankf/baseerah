"""Worked Example 28: Ordinal Labels."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from sklearn.metrics import cohen_kappa_score  # => co-11: the pinned library's own weighted-kappa support (weights='linear')

RATER_X = [1, 2, 3, 4, 2, 3, 1, 4, 2, 3, 1, 4, 2, 3, 4]  # => co-11: a 1-4 ordinal quality rating -- the SAME sequence used in both scenarios below
RATER_Y_NEAR = [1, 2, 3, 4, 3, 3, 1, 4, 1, 3, 1, 4, 3, 3, 4]  # => co-11: every disagreement with RATER_X is exactly ONE step away
RATER_Y_FAR = [1, 2, 3, 4, 4, 3, 1, 4, 4, 3, 1, 4, 4, 3, 4]  # => co-11: the SAME items disagree, but now THREE steps away (1 vs 4, the scale's extremes)


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    raw_near = sum(1 for x, y in zip(RATER_X, RATER_Y_NEAR) if x == y) / len(RATER_X)  # => co-11: exact-match agreement, near-disagreement scenario
    raw_far = sum(1 for x, y in zip(RATER_X, RATER_Y_FAR) if x == y) / len(RATER_X)  # => co-11: exact-match agreement, far-disagreement scenario
    print(f"Raw agreement: near-disagreements={raw_near:.4f} | far-disagreements={raw_far:.4f}")  # => co-11: IDENTICAL -- raw agreement cannot see the distance at all
    assert raw_near == raw_far, "raw agreement must be identical in both scenarios -- it does not know disagreements even have a distance"  # => co-11

    unweighted_near = cohen_kappa_score(RATER_X, RATER_Y_NEAR)  # => co-11: plain Cohen's kappa treats every mismatch the same, regardless of distance
    unweighted_far = cohen_kappa_score(RATER_X, RATER_Y_FAR)  # => co-11: same formula, far-disagreement scenario
    print(f"Unweighted kappa: near={unweighted_near:.4f} | far={unweighted_far:.4f}")  # => co-11: nearly identical -- unweighted kappa ALSO cannot see the distance

    weighted_near = cohen_kappa_score(RATER_X, RATER_Y_NEAR, weights="linear")  # => co-11: linear-weighted kappa -- a k-step disagreement costs k/(scale_span) of full disagreement
    weighted_far = cohen_kappa_score(RATER_X, RATER_Y_FAR, weights="linear")  # => co-11: same formula, far-disagreement scenario
    print(f"Linear-weighted kappa: near={weighted_near:.4f} | far={weighted_far:.4f}")  # => co-11: now CLEARLY different
    assert weighted_near > weighted_far, "the linear-weighted kappa must score the near-disagreement scenario HIGHER than the far-disagreement one"  # => co-11: the distance-cost claim itself
    assert abs(unweighted_near - unweighted_far) < 0.01, "unweighted kappa must stay nearly the SAME across both scenarios, for contrast"  # => co-11
    print(f"MATCH: unweighted kappa is blind to distance ({unweighted_near:.4f} vs {unweighted_far:.4f}), weighted kappa is not ({weighted_near:.4f} vs {weighted_far:.4f})")  # => co-11
    # => co-11: on an ordinal scale, a rater who is off by one step is making a smaller mistake than one who is off by three -- weighted kappa is the coefficient that actually says so
