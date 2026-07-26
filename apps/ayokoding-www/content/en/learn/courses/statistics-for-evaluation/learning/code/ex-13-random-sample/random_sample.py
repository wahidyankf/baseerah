"""Worked Example 13: A Random Sample."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-07: draws the random samples this example estimates from
import statistics  # => co-02: averages many independent random-sample estimates

POPULATION_SIZE = 2000  # => co-07: the full pool of real cases a random sample is drawn FROM
TRUE_PASS_RATE = 0.85  # => co-02: the population's real, unobservable pass rate
SAMPLE_SIZE = 60  # => co-07: cases drawn per sample
REPEATS = 300  # => co-02: independent samples, to check the estimator's own long-run behavior


def build_population(true_rate: float, size: int, *, seed: int) -> list[bool]:  # => co-07: a FIXED, finite population of pass/fail outcomes
    """Build a finite population of size `size` cases, each True (pass) with probability true_rate."""  # => co-07: documents build_population's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-07: one fixed generator -- the population itself does not change across samples
    return [rng.random() < true_rate for _ in range(size)]  # => co-07: one Bernoulli draw per case in the population


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    population = build_population(TRUE_PASS_RATE, POPULATION_SIZE, seed=1)  # => co-07: the FIXED population every sample below draws from
    actual_population_rate = sum(population) / POPULATION_SIZE  # => co-07: the population's own realized rate (close to, not exactly, TRUE_PASS_RATE)
    print(f"Population size: {POPULATION_SIZE} | actual population pass rate: {actual_population_rate:.4f}")  # => co-07

    one_sample = random.Random(10).sample(population, SAMPLE_SIZE)  # => co-07: ONE random sample, drawn without replacement
    one_estimate = sum(one_sample) / SAMPLE_SIZE  # => co-02: this sample's own pass-rate estimate
    print(f"One random sample (n={SAMPLE_SIZE}) estimate: {one_estimate:.4f}")  # => co-02: what a single team would observe

    estimates = [sum(random.Random(100 + i).sample(population, SAMPLE_SIZE)) / SAMPLE_SIZE for i in range(REPEATS)]  # => co-02: many INDEPENDENT random samples
    average_estimate = statistics.mean(estimates)  # => co-02: the estimator's long-run average across repeats
    print(f"Average estimate across {REPEATS} independent random samples: {average_estimate:.4f}")  # => co-02
    gap = abs(average_estimate - actual_population_rate)  # => co-02: how close the long-run average lands to the population's own true rate
    print(f"Gap to the population's actual rate: {gap:.4f}")  # => co-02
    assert gap < 0.01, "the average of many independent random-sample estimates must land close to the population's actual rate"  # => co-07
    print("MATCH: random sampling is unbiased -- no systematic pull toward any particular subset of the population")  # => co-07
    # => co-07: this is the sampling strategy every other strategy in this theme is compared against -- unbiased, but ex-17 shows it can still MISS rare structure
