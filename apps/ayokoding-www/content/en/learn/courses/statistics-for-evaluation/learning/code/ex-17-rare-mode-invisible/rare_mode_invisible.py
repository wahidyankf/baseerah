"""Worked Example 17: A Rare Mode Is Invisible at This Sample Size."""  # => co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-08: builds the population and draws both the random and the stratified sample

POPULATION_SIZE = 5000  # => co-08: a realistic-sized pool of logged cases
RARE_PREVALENCE = 0.01  # => co-08: the rare failure mode's TRUE prevalence -- 1 in 100 cases
RANDOM_SAMPLE_SIZE = 50  # => co-08: a typical small eval-set size


if __name__ == "__main__":  # => co-08: entry point -- runs only when this file executes directly, not on import
    rng = random.Random(5)  # => co-08: builds the fixed population every sample below draws from
    population = [rng.random() < RARE_PREVALENCE for _ in range(POPULATION_SIZE)]  # => co-08: True = this case exhibits the rare failure mode
    actual_count = sum(population)  # => co-08: how many rare-mode cases genuinely exist in the population
    print(f"Population: {POPULATION_SIZE} cases, {actual_count} exhibit the rare mode ({actual_count / POPULATION_SIZE:.4f})")  # => co-08

    random_sample_idx = random.Random(80).sample(range(POPULATION_SIZE), RANDOM_SAMPLE_SIZE)  # => co-08: an honest random draw, the OBVIOUS strategy
    rare_found_random = sum(population[i] for i in random_sample_idx)  # => co-08: how many rare-mode cases this random sample happened to include
    print(f"Rare-mode cases found in a random {RANDOM_SAMPLE_SIZE}-case sample: {rare_found_random}")  # => co-08: almost always zero
    assert rare_found_random == 0, "a random sample at this size must miss the rare mode entirely, for this example's own fixed seed"  # => co-08

    theoretical_miss_probability = (1 - RARE_PREVALENCE) ** RANDOM_SAMPLE_SIZE  # => co-08: P(every one of 50 draws misses a 1%-prevalence case)
    print(f"Theoretical probability a random {RANDOM_SAMPLE_SIZE}-case sample misses it entirely: {theoretical_miss_probability:.4f}")  # => co-08
    assert theoretical_miss_probability > 0.5, "missing a 1%-prevalence mode in 50 random draws must be MORE likely than not"  # => co-08: names the structural risk, not just this one draw

    rare_indices = [i for i, is_rare in enumerate(population) if is_rare]  # => co-08: the rare-mode stratum, identified deliberately
    common_indices = [i for i, is_rare in enumerate(population) if not is_rare]  # => co-08: everything else
    stratified_sample_idx = random.Random(90).sample(rare_indices, 10) + random.Random(91).sample(common_indices, 40)  # => co-08: DELIBERATELY draw 10 from the rare stratum
    rare_found_stratified = sum(population[i] for i in stratified_sample_idx)  # => co-08: how many rare-mode cases the stratified draw includes
    print(f"Rare-mode cases found in a stratified {len(stratified_sample_idx)}-case sample (10 from the rare stratum on purpose): {rare_found_stratified}")  # => co-08
    assert rare_found_stratified == 10, "a stratified sample drawing 10 from the rare stratum on purpose must find all 10"  # => co-08
    print("MATCH: random sampling structurally cannot be trusted to find a rare mode; deliberate stratification can")  # => co-08
    # => co-08: 'we didn't see it in the eval set' is not evidence a rare failure mode is gone -- it may just mean the sampling strategy could not have found it
