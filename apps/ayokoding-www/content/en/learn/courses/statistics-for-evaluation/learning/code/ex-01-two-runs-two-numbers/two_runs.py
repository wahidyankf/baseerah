"""Worked Example 1: Two Runs, Two Numbers."""  # => co-03: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-03: stands in for drawing two independent samples of the same unchanged system

TRUE_PASS_RATE = 0.85  # => co-02: the system's real, unobservable pass rate -- fixed and UNCHANGED across both runs
SAMPLE_SIZE = 40  # => co-03: how many cases each run draws -- a typical small eval-set size


def run_eval(true_rate: float, n: int, *, seed: int) -> float:  # => co-02: simulates one eval run against the SAME system
    """Draw n Bernoulli(true_rate) trials and return the observed pass rate."""  # => co-02: documents run_eval's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-03: a fresh generator per run -- seed stands in for "which cases happened to be sampled"
    passes = sum(1 for _ in range(n) if rng.random() < true_rate)  # => co-02: one Bernoulli draw per case, summed
    return passes / n  # => co-02: the OBSERVED pass rate -- a sample proportion, not the true rate itself


if __name__ == "__main__":  # => co-03: entry point -- runs only when this file executes directly, not on import
    rate_run_1 = run_eval(TRUE_PASS_RATE, SAMPLE_SIZE, seed=1)  # => co-03: "run 1" -- same system, same n, different sample
    rate_run_2 = run_eval(TRUE_PASS_RATE, SAMPLE_SIZE, seed=2)  # => co-03: "run 2" -- same system, same n, DIFFERENT sample
    print(f"True (unobservable) pass rate: {TRUE_PASS_RATE:.2%}")  # => co-02: the ground truth this demo controls
    print(f"Run 1 observed pass rate: {rate_run_1:.2%}")  # => co-03: run 1's number
    print(f"Run 2 observed pass rate: {rate_run_2:.2%}")  # => co-03: run 2's DIFFERENT number
    difference = abs(rate_run_1 - rate_run_2)  # => co-03: the spread between two runs of an UNCHANGED system
    print(f"Difference between two runs of the SAME unchanged system: {difference:.2%}")  # => co-03
    assert rate_run_1 != rate_run_2, "two independent samples of the same system must differ for this demo to make its point"  # => co-03
    print("MATCH: the system never changed, yet the two numbers differ -- that spread is sampling error, not a bug")  # => co-03
    # => co-02,co-03: a pass rate is a sample proportion of an unknown true rate -- re-sampling moves the number even when nothing else does
