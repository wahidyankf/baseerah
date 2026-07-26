"""Worked Example 3: Simulate Sampling Error."""  # => co-03: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-03: draws repeated samples at each sample size
import statistics  # => co-03: measures the spread (population standard deviation) of the resulting pass rates

TRUE_PASS_RATE = 0.85  # => co-03: the system's real, unobservable pass rate -- fixed across every sample size tested
SAMPLE_SIZES = (10, 40, 160, 640)  # => co-03: quadrupling n each step -- sampling error should shrink roughly like 1/sqrt(n)
REPEATS_PER_SIZE = 200  # => co-03: independent draws per sample size, enough to estimate the spread itself precisely


def observed_pass_rate(true_rate: float, n: int, *, seed: int) -> float:  # => co-03: ONE simulated eval run at size n
    """Draw n Bernoulli(true_rate) trials and return the observed pass rate."""  # => co-03: documents observed_pass_rate's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-03: a fresh generator per (size, repeat) draw
    passes = sum(1 for _ in range(n) if rng.random() < true_rate)  # => co-03: count of Bernoulli successes
    return passes / n  # => co-03: this run's observed pass rate


if __name__ == "__main__":  # => co-03: entry point -- runs only when this file executes directly, not on import
    spreads: dict[int, float] = {}  # => co-03: sample size -> spread (pstdev) of observed pass rates at that size
    for n in SAMPLE_SIZES:  # => co-03: one spread measurement per candidate sample size
        rates = [observed_pass_rate(TRUE_PASS_RATE, n, seed=1000 * n + i) for i in range(REPEATS_PER_SIZE)]  # => co-03: many independent runs at this n
        spread = statistics.pstdev(rates)  # => co-03: how much the observed rate bounces around, at this n
        spreads[n] = spread  # => co-03: record it for the shrink-rate check below
        bar = "#" * round(spread * 400)  # => co-03: a plain-text bar -- longer bar means more sampling error at this n
        print(f"n={n:>4} | spread={spread:.4f} | {bar}")  # => co-03: prints size, spread, and its ASCII-bar visualization
    assert spreads[SAMPLE_SIZES[0]] > spreads[SAMPLE_SIZES[-1]], "spread must shrink as n grows"  # => co-03: the qualitative claim
    ratio_10_to_640 = spreads[10] / spreads[640]  # => co-03: sqrt(640/10) = 8 is the theoretical shrink factor for a 64x larger n
    print(f"Spread ratio (n=10 / n=640): {ratio_10_to_640:.2f} (theoretical 1/sqrt(n) factor: {(640 / 10) ** 0.5:.2f})")  # => co-03
    assert 6.0 < ratio_10_to_640 < 10.0, "the shrink ratio must be close to the theoretical sqrt(n) factor"  # => co-03: sanity-checks the simulation itself
    print("MATCH: sampling error shrinks roughly like 1/sqrt(n) -- quantifiable, not mysterious")  # => co-03
    # => co-03: this is WHY a confidence interval (ex-04 onward) has a width that depends on n, not just on the observed rate
