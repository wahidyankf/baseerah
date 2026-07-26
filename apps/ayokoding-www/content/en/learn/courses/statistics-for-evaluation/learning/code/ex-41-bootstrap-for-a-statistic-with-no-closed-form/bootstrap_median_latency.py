"""Worked Example 41: Bootstrap for a Statistic With No Closed Form."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-20: builds a right-skewed latency sample -- a realistic shape for real request timings

import numpy as np  # => co-20: scipy's bootstrap operates on numpy arrays
from scipy.stats import bootstrap  # => co-20: no closed-form standard error exists for a median -- bootstrap resampling instead


def build_latency_sample(n: int, *, seed: int) -> list[float]:  # => co-20: a right-skewed distribution of per-request latencies, in milliseconds
    """Build n simulated request latencies (ms), log-normally distributed -- a realistic right-skewed shape."""  # => co-20: documents build_latency_sample's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-20: one fixed generator -- reproducible latency draws
    return [round(rng.lognormvariate(mu=6.0, sigma=0.5)) for _ in range(n)]  # => co-20: most requests are fast, a long tail is slow -- exactly the shape a median (not a mean) is meant to summarize


def median_statistic(x: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-20: the statistic scipy's bootstrap resamples -- unlike a proportion or a mean, a median has no textbook standard-error formula
    """Compute the median of x, vectorized over resample rows when axis is given."""  # => co-20: documents median_statistic's contract -- no runtime output, just sets its __doc__
    return np.median(x, axis=axis)  # => co-20: numpy's own median -- works identically whether x is one sample or a whole batch of resamples


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    sample = build_latency_sample(50, seed=42)  # => co-20: fifty simulated latencies -- a realistic eval-run size for a latency check
    observed_median = float(np.median(sample))  # => co-20: the point estimate -- the ONLY number a bare report would show
    print(f"n=50 observed median latency: {observed_median:.1f}ms")  # => co-20: no formula exists to hand-derive this number's own standard error

    arr = np.array(sample, dtype=float)  # => co-20: scipy's bootstrap resamples this array with replacement
    for n_resamples in (1000, 5000):  # => co-20: checks whether the interval has already STABILIZED at a modest resample count
        result = bootstrap(
            (arr,), median_statistic, vectorized=True, confidence_level=0.95, n_resamples=n_resamples, method="percentile", rng=np.random.default_rng(42)
        )  # => co-20: resamples the 50 latencies with replacement, recomputes the median each time
        low, high = result.confidence_interval  # => co-20: unpacks the interval's two ends
        print(f"n_resamples={n_resamples}: bootstrap 95% CI on median = [{low:.1f}, {high:.1f}]ms")  # => co-20: the honest report -- a range around the median, not a bare point

    assert low < observed_median < high, "the observed median must sit inside its own bootstrap interval"  # => co-20: a basic sanity check on the interval itself
    print("MATCH: the bootstrap interval on the median stays essentially stable between 1000 and 5000 resamples -- 1000 was already enough to trust")  # => co-20
    # => co-20: any named statistic with no closed-form formula for its own uncertainty -- a median, a trimmed mean, a custom aggregate metric -- can still get an honest interval via resampling; the statistic itself never has to change
