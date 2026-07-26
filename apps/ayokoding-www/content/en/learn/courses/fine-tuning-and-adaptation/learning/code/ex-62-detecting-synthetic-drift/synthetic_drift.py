# learning/code/ex-62-detecting-synthetic-drift/synthetic_drift.py
"""Worked Example 62: Detecting Synthetic Drift."""  # => co-14: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

REAL_PRODUCTION_CATEGORY_SHARE: dict[str, float] = {  # => co-14: the TRUE category distribution, from ex-21's real traffic counts
    "password-reset": 0.70,  # => co-14
    "billing": 0.18,  # => co-14
    "bug": 0.11,  # => co-14
    "feature-request": 0.01,  # => co-14
}  # => co-14: closes REAL_PRODUCTION_CATEGORY_SHARE -- sums to 1.00

SYNTHETIC_CATEGORY_SHARE: dict[str, float] = {  # => co-14: what the teacher model actually generated when asked for "typical support tickets"
    "password-reset": 0.30,  # => co-14: the teacher under-generates this, despite it being the MOST common real category
    "billing": 0.25,  # => co-14
    "bug": 0.30,  # => co-14: the teacher over-generates "interesting" bug scenarios
    "feature-request": 0.15,  # => co-14: the teacher over-generates these too -- they make more "engaging" synthetic examples
}  # => co-14: closes SYNTHETIC_CATEGORY_SHARE -- sums to 1.00, but shaped very differently from reality

DRIFT_ALERT_THRESHOLD_PERCENTAGE_POINTS = 0.15  # => co-14: any category off by more than 15 points triggers a review before training


def max_absolute_drift(real: dict[str, float], synthetic: dict[str, float]) -> tuple[str, float]:  # => co-14: (worst category, its drift)
    """Return the category with the largest absolute percentage-point difference between `real` and `synthetic`, and that difference."""  # => co-14: documents max_absolute_drift's contract -- no runtime output, just sets its __doc__
    drifts = {category: abs(real[category] - synthetic[category]) for category in real}  # => co-14: per-category drift, both directions treated equally
    worst_category = max(drifts, key=lambda c: drifts[c])  # => co-14: which category diverges most
    return worst_category, drifts[worst_category]  # => co-14: returns this computed value to the caller


if __name__ == "__main__":  # => co-14: entry point -- runs only when this file executes directly, not on import
    for category in REAL_PRODUCTION_CATEGORY_SHARE:  # => co-14: show real vs. synthetic, category by category
        real_share = REAL_PRODUCTION_CATEGORY_SHARE[category]  # => co-14: this category's true share
        synthetic_share = SYNTHETIC_CATEGORY_SHARE[category]  # => co-14: this category's synthetic share
        print(f"  {category}: real {real_share:.0%} vs. synthetic {synthetic_share:.0%}")  # => co-14
    worst_category, drift = max_absolute_drift(REAL_PRODUCTION_CATEGORY_SHARE, SYNTHETIC_CATEGORY_SHARE)  # => co-14: run the audit
    print(f"Largest drift: {worst_category} off by {drift:.0%}")  # => co-14
    alert = drift > DRIFT_ALERT_THRESHOLD_PERCENTAGE_POINTS  # => co-14: does this cross the review threshold?
    print(f"Drift alert triggered (threshold {DRIFT_ALERT_THRESHOLD_PERCENTAGE_POINTS:.0%}): {alert}")  # => co-14
    assert worst_category == "password-reset", "the biggest divergence in this scenario must be the most common real category, under-generated synthetically"  # => co-14
    assert alert, "a 40-point drift on the single largest real category must trigger the review threshold"  # => co-14,co-28
    print("MATCH: the synthetic generator's own bias silently reshaped the dataset's category mix -- caught here, before training, not after")  # => co-14,co-28
    # => co-14,co-28: this drift is exactly the teacher's OWN error propagating into the dataset's shape, not just into individual labels (ex-24)
