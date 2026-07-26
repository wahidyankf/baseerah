"""Worked Example 2: Pass Rate Is an Estimate."""  # => co-02: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-02: draws many independent samples to reveal the estimator's own behavior
import statistics  # => co-02: averages many observed pass rates to approximate the true rate

TRUE_PASS_RATE = 0.73  # => co-02: the system's real, unobservable pass rate -- known here only because this is a simulation
SAMPLE_SIZE = 50  # => co-02: cases per single eval run
NUM_REPEATS = 4000  # => co-02: how many independent runs to average over -- large enough for the law of large numbers to bite


def observed_pass_rate(true_rate: float, n: int, *, seed: int) -> float:  # => co-02: ONE run -- what a team actually sees
    """Draw n Bernoulli(true_rate) trials and return the sample proportion that passed."""  # => co-02: documents observed_pass_rate's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-02: a fresh generator per run
    passes = sum(1 for _ in range(n) if rng.random() < true_rate)  # => co-02: count of Bernoulli successes in this run
    return passes / n  # => co-02: the sample proportion -- a single run's ESTIMATE of the true rate, not the true rate itself


if __name__ == "__main__":  # => co-02: entry point -- runs only when this file executes directly, not on import
    one_run = observed_pass_rate(TRUE_PASS_RATE, SAMPLE_SIZE, seed=0)  # => co-02: what a single team actually observes, once
    print(f"True (unobservable) pass rate: {TRUE_PASS_RATE:.4f}")  # => co-02: the ground truth this simulation controls
    print(f"One run's observed pass rate: {one_run:.4f}")  # => co-02: a single sample proportion -- one team's whole evidence
    many_runs = [observed_pass_rate(TRUE_PASS_RATE, SAMPLE_SIZE, seed=i) for i in range(NUM_REPEATS)]  # => co-02: thousands of INDEPENDENT teams running the same eval
    average_of_runs = statistics.mean(many_runs)  # => co-02: the estimator's own long-run behavior -- what "unbiased" means
    print(f"Average pass rate across {NUM_REPEATS} independent runs: {average_of_runs:.4f}")  # => co-02: converges toward the true rate
    gap_to_truth = abs(average_of_runs - TRUE_PASS_RATE)  # => co-02: how close the LONG-RUN average lands to the true rate
    print(f"Gap between the long-run average and the true rate: {gap_to_truth:.4f}")  # => co-02: should be small -- the estimator is unbiased
    assert gap_to_truth < 0.01, "the average of many independent pass-rate estimates must land close to the true rate"  # => co-02
    print("MATCH: no single run equals the true rate, but the estimator is unbiased across many runs")  # => co-02
    # => co-02: a pass rate IS a sample proportion of an unknown true rate -- treating one run's number as the truth itself is the mistake
