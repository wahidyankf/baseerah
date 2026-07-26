"""Worked Example 9: Forty Cases Cannot See It."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-06: draws each simulated system's cases

from statsmodels.stats.proportion import proportions_ztest  # => co-19: the pinned library's own two-proportion significance test

TRUE_RATE_BASELINE = 0.78  # => co-19: system A's real, unobservable pass rate
TRUE_RATE_CANDIDATE = 0.90  # => co-19: system B's real, unobservable pass rate -- a genuine 12-point improvement
ILLUSTRATION_SEED = 9  # => co-06: a specific, fixed seed pair used for the single-run illustration below


def draw_passes(true_rate: float, n: int, *, seed: int) -> int:  # => co-06: one simulated eval run's pass COUNT
    """Draw n Bernoulli(true_rate) trials and return the number that passed."""  # => co-06: documents draw_passes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-06: a fresh generator per (system, n, seed) draw
    return sum(1 for _ in range(n) if rng.random() < true_rate)  # => co-06: count of Bernoulli successes


def run_significance_test(n: int, *, seed: int) -> float:  # => co-19: one full A/B test at sample size n -- returns its p-value
    """Draw n cases for each system and return the two-proportion z-test p-value."""  # => co-19: documents run_significance_test's contract -- no runtime output, just sets its __doc__
    passes_a = draw_passes(TRUE_RATE_BASELINE, n, seed=seed * 2)  # => co-06: system A's simulated pass count at this n
    passes_b = draw_passes(TRUE_RATE_CANDIDATE, n, seed=seed * 2 + 1)  # => co-06: system B's simulated pass count at this n
    _, p_value = proportions_ztest([passes_a, passes_b], [n, n])  # => co-19: the real, genuine 12-point effect, tested at this n
    return p_value  # => co-19: returns this computed value to the caller


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    p_at_40 = run_significance_test(40, seed=ILLUSTRATION_SEED)  # => co-19: ONE specific illustrative run at the small size
    p_at_400 = run_significance_test(400, seed=ILLUSTRATION_SEED)  # => co-19: the SAME true effect, the SAME seed family, ten times the cases
    print(f"n=40  -> p-value {p_at_40:.4f} ({'significant' if p_at_40 < 0.05 else 'NOT significant'})")  # => co-19: prints the small-n verdict
    print(f"n=400 -> p-value {p_at_400:.4f} ({'significant' if p_at_400 < 0.05 else 'NOT significant'})")  # => co-19: prints the large-n verdict
    assert p_at_40 >= 0.05, "the illustrative n=40 run must fail to detect the real effect"  # => co-19: the invisible-at-n=40 claim
    assert p_at_400 < 0.05, "the illustrative n=400 run must detect the same real effect"  # => co-19: the visible-at-n=400 claim

    trials = 500  # => co-19: repeats for the Monte Carlo detection-rate check below -- one illustrative run could be a fluke either way
    detections_40 = sum(1 for t in range(1000, 1000 + trials) if run_significance_test(40, seed=t) < 0.05)  # => co-19: how OFTEN n=40 catches this real effect
    detections_400 = sum(1 for t in range(1000, 1000 + trials) if run_significance_test(400, seed=t) < 0.05)  # => co-19: how often n=400 catches it
    print(f"Detection rate across {trials} independent trials: n=40 -> {detections_40 / trials:.1%} | n=400 -> {detections_400 / trials:.1%}")  # => co-19
    assert detections_40 / trials < 0.5, "n=40 must detect this real effect LESS than half the time across repeated trials"  # => co-19
    assert detections_400 / trials > 0.95, "n=400 must detect the same real effect in almost every trial"  # => co-19
    print("MATCH: the SAME real 12-point effect is invisible most of the time at n=40, and reliable at n=400")  # => co-19
    # => co-06,co-19: a non-significant result at n=40 is not 'no effect' -- it is 'not enough cases to see this effect', which ex-40 revisits
