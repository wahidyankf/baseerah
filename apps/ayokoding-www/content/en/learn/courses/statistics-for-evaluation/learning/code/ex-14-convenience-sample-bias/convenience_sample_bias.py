"""Worked Example 14: Convenience Sample Bias."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-07: builds the population and draws both the random and the convenience sample

POPULATION_SIZE = 2000  # => co-07: the full pool of real cases
TRUE_PASS_RATE = 0.85  # => co-07: the population's real, unobservable pass rate
RANDOM_SAMPLE_SIZE = 60  # => co-07: an honest random sample's size, for direct comparison


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    rng = random.Random(3)  # => co-07: builds the fixed population this whole example draws from
    population = [rng.random() < TRUE_PASS_RATE for _ in range(POPULATION_SIZE)]  # => co-07: one Bernoulli draw per case
    actual_rate = sum(population) / POPULATION_SIZE  # => co-07: the population's own realized pass rate
    print(f"Population's actual pass rate: {actual_rate:.4f}")  # => co-07: the ground truth this example compares both samples against

    random_sample = random.Random(50).sample(population, RANDOM_SAMPLE_SIZE)  # => co-07: an honest, unbiased random sample
    random_estimate = sum(random_sample) / RANDOM_SAMPLE_SIZE  # => co-07: its resulting estimate
    print(f"Random sample (n={RANDOM_SAMPLE_SIZE}) estimate: {random_estimate:.4f}")  # => co-07

    fail_indices = [i for i, passed in enumerate(population) if not passed]  # => co-07: "the failures someone happened to notice" pool -- ALL of them, over-represented
    pass_indices = [i for i, passed in enumerate(population) if passed]  # => co-07: passes get noticed only incidentally, almost never on purpose
    noticed_failures = random.Random(51).sample(fail_indices, min(40, len(fail_indices)))  # => co-07: a support team collects failures they happened to see
    noticed_passes = random.Random(51).sample(pass_indices, 5)  # => co-07: a handful of passes noticed only by accident
    convenience_indices = noticed_failures + noticed_passes  # => co-07: the whole "convenience sample" -- NOT drawn at random from the population
    convenience_estimate = sum(population[i] for i in convenience_indices) / len(convenience_indices)  # => co-07: this sample's resulting estimate
    print(f"Convenience sample (n={len(convenience_indices)}, 'cases someone noticed') estimate: {convenience_estimate:.4f}")  # => co-07

    random_error = abs(random_estimate - actual_rate)  # => co-07: how far the honest random sample's estimate is from the truth
    convenience_error = abs(convenience_estimate - actual_rate)  # => co-07: how far the convenience sample's estimate is from the truth
    print(f"Random sample error: {random_error:.4f} | Convenience sample error: {convenience_error:.4f}")  # => co-07
    assert convenience_error > 5 * random_error, "the convenience sample's error must be dramatically larger than the random sample's"  # => co-07: the bias claim itself
    print("MATCH: a convenience sample of failures someone happened to notice estimates almost nothing about the true rate")  # => co-07
    # => co-07: this is why 'we collected the bugs users reported' is not a sampling strategy -- it is a strategy for finding bugs, not for estimating a rate
