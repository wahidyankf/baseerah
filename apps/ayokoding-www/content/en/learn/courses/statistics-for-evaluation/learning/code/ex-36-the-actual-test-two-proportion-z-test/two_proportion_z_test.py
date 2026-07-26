"""Worked Example 36: The Actual Test -- Two-Proportion Z-Test."""  # => co-17: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-17: draws independent baseline and candidate samples, at two different sample sizes

from statsmodels.stats.proportion import proportions_ztest  # => co-17: the pinned library's own unpaired two-sample test for comparing two proportions

TRUE_BASELINE_RATE = 0.75  # => co-17: baseline's real, unobservable pass rate -- the SAME systems as ex-35
TRUE_CANDIDATE_RATE = 0.80  # => co-17: candidate's real, unobservable pass rate -- genuinely 5 points better


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-17: one independent sample of one system's outcomes
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-17: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-17: one fixed generator per sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-17: one Bernoulli draw per case


if __name__ == "__main__":  # => co-17: entry point -- runs only when this file executes directly, not on import
    for n in (30, 300):  # => co-17: the SAME true systems, sampled at a typical small size and a properly powered size
        baseline = sample_outcomes(TRUE_BASELINE_RATE, n, seed=1)  # => co-17: baseline's own sample at this n
        candidate = sample_outcomes(TRUE_CANDIDATE_RATE, n, seed=2)  # => co-17: candidate's own sample at this n
        baseline_rate = sum(baseline) / n  # => co-17: baseline's point estimate at this n
        candidate_rate = sum(candidate) / n  # => co-17: candidate's point estimate at this n

        count = [sum(candidate), sum(baseline)]  # => co-17: successes for each group, in the order the test compares them
        nobs = [n, n]  # => co-17: each group's own sample size
        z_stat, p_value = proportions_ztest(count, nobs)  # => co-17: THE actual hypothesis test -- is this gap distinguishable from sampling noise
        print(f"n={n}: baseline={baseline_rate:.4f} candidate={candidate_rate:.4f} gap={candidate_rate - baseline_rate:+.4f} z={z_stat:.4f} p={p_value:.4f}")  # => co-17: the test's own verdict, not just the bare gap

        if n == 30:  # => co-17: the small-sample case -- the SAME sample size ex-35 showed as unstable
            assert p_value > 0.05, "at n=30, this test must fail to reach significance -- the sample is too small to distinguish this gap from noise"  # => co-17: the underpowered claim
        else:  # => co-17: the properly powered case
            assert p_value < 0.05, "at n=300, this test must reach significance -- the same true gap is now detectable"  # => co-17: the properly-powered claim

    print("MATCH: 'is B better than A' has a real answer -- a p-value from an actual test -- and that answer depends on whether the sample is large enough to detect the true gap")  # => co-17
    # => co-17: this is the test ex-35's bare gap was missing -- a formal hypothesis test, not a comparison of two printed numbers, is what actually answers 'is candidate better than baseline'
