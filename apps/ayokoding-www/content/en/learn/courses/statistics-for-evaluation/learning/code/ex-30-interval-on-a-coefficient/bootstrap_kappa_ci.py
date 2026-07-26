"""Worked Example 30: Interval on a Coefficient."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-13: builds the same balanced two-rater fixture used in ex-29

import numpy as np  # => co-13: scipy's bootstrap operates on numpy arrays, paired by index
from scipy.stats import bootstrap  # => co-13: no closed-form standard error exists for kappa -- bootstrap resampling instead
from sklearn.metrics import cohen_kappa_score  # => co-13: the coefficient this example puts an interval around


def build_balanced_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-13: reuses ex-29's balanced-prevalence fixture -- kappa=0.70 on 60 items
    """Return two raters' labels over n items, balanced 50/50 between 'pass' and 'fail'."""  # => co-13: documents the contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-13: shuffles rater_a's balanced labels
    rater_a = ["pass"] * (n // 2) + ["fail"] * (n // 2)  # => co-13: exactly half pass, half fail
    rng.shuffle(rater_a)  # => co-13: randomizes order before rater_b reads it
    noise_rng = random.Random(seed + 1)  # => co-13: a second, independent generator for rater_b's per-item noise
    rater_b: list[str] = []  # => co-13: built item by item below
    for label in rater_a:  # => co-13: rater_b agrees with rater_a 85% of the time, disagrees the other 15%
        if noise_rng.random() < 0.85:  # => co-13: the 85% "agree" branch
            rater_b.append(label)  # => co-13: copies rater_a's label exactly
        else:  # => co-13: the 15% "disagree" branch
            rater_b.append("fail" if label == "pass" else "pass")  # => co-13: flips to the opposite label
    return rater_a, rater_b  # => co-13: two label lists, same length, ready for agreement scoring


def kappa_statistic(rater_a: np.ndarray, rater_b: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-13: the statistic scipy's bootstrap resamples -- vectorized over resample rows
    """Compute Cohen's kappa for one pair of label arrays, or one row per resample."""  # => co-13: documents the contract -- no runtime output, just sets its __doc__
    if rater_a.ndim == 1:  # => co-13: the plain, non-vectorized case -- one dataset, one kappa
        return cohen_kappa_score(rater_a, rater_b)  # => co-13: a single float
    out = np.empty(rater_a.shape[0])  # => co-13: one kappa slot per bootstrap resample row
    for i in range(rater_a.shape[0]):  # => co-13: scipy calls this function once per batch, so loop over the batch's rows
        out[i] = cohen_kappa_score(rater_a[i], rater_b[i])  # => co-13: this resample's own kappa
    return out  # => co-13: one kappa value per resample, feeding the percentile interval below


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    rater_a, rater_b = build_balanced_dataset(60, seed=3)  # => co-13: the SAME fixture as ex-29's balanced dataset
    observed_kappa = cohen_kappa_score(rater_a, rater_b)  # => co-13: the point estimate -- one number from the observed 60 items
    print(f"Observed kappa (point estimate): {observed_kappa:.4f}")  # => co-13: this is the ONLY number a bare kappa report would show

    a_arr = np.array(rater_a)  # => co-13: scipy's bootstrap resamples paired arrays by matching indices
    b_arr = np.array(rater_b)  # => co-13: must stay index-aligned with a_arr through every resample
    result = bootstrap(  # => co-13: resamples (item, item) pairs with replacement, recomputes kappa each time
        (a_arr, b_arr),  # => co-13: the paired data -- resampling shuffles WHICH items are drawn, never which rater said what about a given item
        kappa_statistic,  # => co-13: the statistic recomputed on every resample
        paired=True,  # => co-13: keeps rater_a[i] and rater_b[i] together -- this is agreement data, not two independent samples
        vectorized=True,  # => co-13: lets kappa_statistic receive a whole batch of resamples at once, for speed
        confidence_level=0.95,  # => co-13: the standard 95% interval
        n_resamples=2000,  # => co-13: 2000 resamples -- enough for a stable percentile estimate at this sample size
        method="percentile",  # => co-13: the simplest bootstrap interval -- the 2.5th and 97.5th percentiles of the resampled kappas
        rng=np.random.default_rng(3),  # => co-13: fixes the resampling draw, so this script reproduces the same interval every run
    )
    low, high = result.confidence_interval  # => co-13: unpacks the interval's two ends
    print(f"Bootstrap 95% CI on kappa: [{low:.4f}, {high:.4f}]")  # => co-13: THIS is the honest report -- a range, not a bare point

    assert low < observed_kappa < high, "the observed kappa must sit inside its own bootstrap interval"  # => co-13: a basic sanity check on the interval itself
    assert (high - low) > 0.1, "an interval this wide is the point -- 60 items is not enough for a tight kappa estimate"  # => co-13: the claim this example demonstrates
    print(f"MATCH: a single kappa={observed_kappa:.4f} hides a {high - low:.4f}-wide range of plausible values at n=60")  # => co-13
    # => co-13: reporting "kappa=0.70" with no interval implies more precision than 60 items can support -- the bootstrap interval is what makes that uncertainty visible
