"""Worked Example 45: Noise Floor Decomposition."""  # => co-22: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-22: builds a population where each case has its own true pass PROBABILITY, not a fixed pass/fail
import statistics  # => co-22: pvariance -- population variance, computed over each simulated distribution of pass rates

POPULATION_SIZE = 300  # => co-22: a realistic-sized pool of real cases
N = 30  # => co-22: a typical eval-set size, drawn from that pool
K_REGENERATIONS = 1000  # => co-23: how many times the SAME fixed cases are re-generated, to measure generation noise alone
M_RESAMPLES = 1000  # => co-22: how many DIFFERENT case samples are drawn, to measure case-sampling variance and total variance


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    population_rng = random.Random(7)  # => co-22: builds the fixed population every sample below draws from
    population_p = [population_rng.betavariate(6, 2) for _ in range(POPULATION_SIZE)]  # => co-22: each case's own TRUE pass PROBABILITY -- some cases are inherently easier or harder than others
    print(f"Population: {POPULATION_SIZE} cases, mean true pass probability {statistics.mean(population_p):.4f}")  # => co-22: the population's own per-case difficulty spread, not a single pass rate

    fixed_sample_rng = random.Random(42)  # => co-23: fixes ONE specific eval set -- the same 30 cases used throughout the generation-noise measurement below
    fixed_indices = fixed_sample_rng.sample(range(POPULATION_SIZE), N)  # => co-23: the exact 30 cases this "eval set" contains, held constant below
    fixed_p = [population_p[i] for i in fixed_indices]  # => co-23: these 30 cases' own true pass probabilities, held fixed

    generation_rates: list[float] = []  # => co-23: one pass rate per re-run of the SAME 30 cases
    for k in range(K_REGENERATIONS):  # => co-23: re-generates the SAME fixed cases, over and over, as an unchanged stochastic system would on repeat runs
        gen_rng = random.Random(1000 + k)  # => co-23: a fresh generation draw each time -- cases do NOT change, only the stochastic outcome does
        outcomes = [gen_rng.random() < p for p in fixed_p]  # => co-23: each case's outcome THIS regeneration -- a fresh Bernoulli draw from its own true probability
        generation_rates.append(sum(outcomes) / N)  # => co-23: this regeneration's own pass rate
    generation_variance_empirical = statistics.pvariance(generation_rates)  # => co-23: the SPREAD of pass rates across regenerations, cases held fixed -- pure generation noise
    generation_variance_closed_form = sum(p * (1 - p) for p in fixed_p) / (N**2)  # => co-23: the EXACT closed form -- mean per-case Bernoulli variance, divided by n
    print(
        f"Generation variance (fixed cases, regenerated {K_REGENERATIONS}x): empirical={generation_variance_empirical:.6f} closed-form={generation_variance_closed_form:.6f}"
    )  # => co-23: computed twice, same as every other named quantity in this course

    case_sampling_means: list[float] = []  # => co-22: one mean TRUE probability per DIFFERENT case sample, with NO generation noise mixed in
    for m in range(M_RESAMPLES):  # => co-22: draws a DIFFERENT set of 30 cases each time, from the SAME population
        sample_rng = random.Random(2000 + m)  # => co-22: this resample's own case draw
        sample_idx = sample_rng.sample(range(POPULATION_SIZE), N)  # => co-22: which 30 cases this resample happens to contain
        sample_p = [population_p[i] for i in sample_idx]  # => co-22: those 30 cases' own TRUE probabilities -- using the true value, not a noisy generation, isolates case-sampling variance alone
        case_sampling_means.append(statistics.mean(sample_p))  # => co-22: this resample's own mean true probability
    case_sampling_variance_empirical = statistics.pvariance(case_sampling_means)  # => co-22: the SPREAD across different case samples, with generation noise removed entirely

    total_rates: list[float] = []  # => co-22,co-23: one pass rate per DIFFERENT case sample, EACH ALSO freshly generated -- both noise sources combined
    for m in range(M_RESAMPLES):  # => co-22: a DIFFERENT case sample every time, exactly like real repeated eval runs
        total_sample_rng = random.Random(3000 + m)  # => co-22: this run's own case draw
        total_idx = total_sample_rng.sample(range(POPULATION_SIZE), N)  # => co-22: which 30 cases this run happens to contain
        total_p = [population_p[i] for i in total_idx]  # => co-22: those 30 cases' own true probabilities
        total_gen_rng = random.Random(4000 + m)  # => co-23: this run's own fresh generation
        total_outcomes = [total_gen_rng.random() < p for p in total_p]  # => co-23: each case's actual outcome THIS run -- both which cases AND their generation vary
        total_rates.append(sum(total_outcomes) / N)  # => co-22,co-23: this run's own observed pass rate, exactly like a real repeated eval run
    total_variance_empirical = statistics.pvariance(total_rates)  # => co-22,co-23: the SPREAD a team would actually observe re-running the whole eval, unchanged system
    sum_of_components = generation_variance_empirical + case_sampling_variance_empirical  # => co-22,co-23: the decomposition's own prediction -- total should equal the sum of its two independent sources
    print(f"Case-sampling variance (different cases, true probabilities): {case_sampling_variance_empirical:.6f}")  # => co-22
    print(f"Total variance (different cases AND fresh generation): {total_variance_empirical:.6f}")  # => co-22,co-23
    print(f"Sum of components (generation + case-sampling): {sum_of_components:.6f}")  # => co-22,co-23: the internal-consistency check this decomposition must pass

    assert abs(generation_variance_empirical - generation_variance_closed_form) / generation_variance_closed_form < 0.10  # => co-23: the empirical and closed-form generation variance must agree closely
    assert abs(total_variance_empirical - sum_of_components) / sum_of_components < 0.20  # => co-22,co-23: total variance must be close to the sum of its two independent components
    print("MATCH: total run-to-run variance decomposes into generation noise (same cases, re-generated) plus case-sampling variance (different cases drawn) -- and the two sum back to the observed total")  # => co-22,co-23
    # => co-22,co-23: the noise floor is not one number -- it is (at least) two distinct sources, and neither one alone is 'the' variance of an unchanged, re-run suite
