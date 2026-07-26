"""Worked Example 42: Honest Small-n Uncertainty."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-20: rebuilds the SAME latency-generating fixture at two very different sample sizes

import numpy as np  # => co-20: scipy's bootstrap operates on numpy arrays
from scipy.stats import bootstrap  # => co-20: the SAME bootstrap machinery ex-41 introduced


def build_latency_sample(n: int, *, seed: int) -> list[float]:  # => co-20: the identical fixture-building function from ex-41
    """Build n simulated request latencies (ms), log-normally distributed -- a realistic right-skewed shape."""  # => co-20: documents build_latency_sample's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-20: one fixed generator -- reproducible latency draws
    return [round(rng.lognormvariate(mu=6.0, sigma=0.5)) for _ in range(n)]  # => co-20: most requests are fast, a long tail is slow


def median_statistic(x: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-20: the identical statistic function from ex-41
    """Compute the median of x, vectorized over resample rows when axis is given."""  # => co-20: documents median_statistic's contract -- no runtime output, just sets its __doc__
    return np.median(x, axis=axis)  # => co-20: numpy's own median


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    widths: dict[int, float] = {}  # => co-20: collects each sample size's own interval width, for the shrinkage check below
    for n in (15, 150):  # => co-20: the SAME latency-generating process, sampled a small vs. a much larger number of times
        sample = build_latency_sample(n, seed=42)  # => co-20: this sample size's own draw
        arr = np.array(sample, dtype=float)  # => co-20: scipy's bootstrap resamples this array with replacement
        median = float(np.median(sample))  # => co-20: this sample's own point estimate
        result = bootstrap((arr,), median_statistic, vectorized=True, confidence_level=0.95, n_resamples=3000, method="percentile", rng=np.random.default_rng(7))  # => co-20: the SAME bootstrap procedure, only n changes
        low, high = result.confidence_interval  # => co-20: this sample size's own interval bounds
        width = high - low  # => co-20: how WIDE the honest uncertainty band is, at this sample size
        widths[n] = width  # => co-20: stored for the comparison below
        print(f"n={n}: median={median:.1f}ms bootstrap 95% CI=[{low:.1f}, {high:.1f}]ms width={width:.1f}ms")  # => co-20: the full, honest report at each sample size

    assert widths[15] > 3 * widths[150], "the n=15 interval must be dramatically wider than the n=150 interval -- small-n uncertainty is not a rounding error"  # => co-20: the claim this example demonstrates
    print(f"MATCH: the n=15 interval ({widths[15]:.1f}ms wide) is over {widths[15] / widths[150]:.1f}x wider than the n=150 interval ({widths[150]:.1f}ms wide), on the SAME underlying latency distribution")  # => co-20
    # => co-20: a bootstrap interval does not manufacture precision a small sample does not have -- it reports, honestly, exactly how little a 15-case sample can actually pin down, which a bare median alone would have hidden entirely
